use oxc_allocator::{Box, Vec};
use oxc_ast::ast::*;
use oxc_diagnostics::Result;
use oxc_ecmascript::PropName;
use oxc_span::{GetSpan, Span};

use crate::{
    diagnostics,
    lexer::Kind,
    modifiers::{ModifierFlags, ModifierKind, Modifiers},
    Context, ParserImpl, StatementContext,
};

type Extends<'a> =
    Vec<'a, (Expression<'a>, Option<Box<'a, TSTypeParameterInstantiation<'a>>>, Span)>;

type Implements<'a> = Vec<'a, TSClassImplements<'a>>;

/// Section 15.7 Class Definitions
impl<'a> ParserImpl<'a> {
    // `start_span` points at the start of all decoractors and `class` keyword.
    pub(crate) fn parse_class_statement(
        &mut self,
        stmt_ctx: StatementContext,
        start_span: Span,
    ) -> Result<Statement<'a>> {
        let modifiers = self.parse_modifiers(
            /* allow_decorators */ true, /* permit_const_as_modifier */ false,
            /* stop_on_start_of_class_static_block */ true,
        );
        let decl = self.parse_class_declaration(start_span, &modifiers)?;

        if stmt_ctx.is_single_statement() {
            self.error(diagnostics::class_declaration(Span::new(
                decl.span.start,
                decl.body.span.start,
            )));
        }

        Ok(Statement::ClassDeclaration(decl))
    }

    /// Section 15.7 Class Definitions
    pub(crate) fn parse_class_declaration(
        &mut self,
        start_span: Span,
        modifiers: &Modifiers<'a>,
    ) -> Result<Box<'a, Class<'a>>> {
        self.parse_class(start_span, ClassType::ClassDeclaration, modifiers)
    }

    /// Section [Class Definitions](https://tc39.es/ecma262/#prod-ClassExpression)
    /// `ClassExpression`[Yield, Await] :
    ///     class `BindingIdentifier`[?Yield, ?Await]opt `ClassTail`[?Yield, ?Await]
    pub(crate) fn parse_class_expression(&mut self) -> Result<Expression<'a>> {
        let class =
            self.parse_class(self.start_span(), ClassType::ClassExpression, &Modifiers::empty())?;
        Ok(Expression::ClassExpression(class))
    }

    fn parse_class(
        &mut self,
        start_span: Span,
        r#type: ClassType,
        modifiers: &Modifiers<'a>,
    ) -> Result<Box<'a, Class<'a>>> {
        self.bump_any(); // advance `class`

        let decorators = self.consume_decorators();
        let start_span = decorators.iter().next().map_or(start_span, |d| d.span);

        let id = if self.cur_kind().is_binding_identifier() && !self.at(Kind::Implements) {
            Some(self.parse_binding_identifier()?)
        } else {
            None
        };

        let type_parameters = if self.is_ts { self.parse_ts_type_parameters()? } else { None };
        let (extends, implements) = self.parse_heritage_clause()?;
        let mut super_class = None;
        if let Some(mut extends) = extends {
            if !extends.is_empty() {
                let first_extends = extends.remove(0);
                super_class = Some(self.ast.class_extends(
                    self.end_span(start_span),
                    self.ast.expression_with_type_arguments(
                        first_extends.2,
                        first_extends.0,
                        first_extends.1,
                    ),
                ));
            }
        }
        let body = self.parse_class_body()?;

        self.verify_modifiers(
            modifiers,
            ModifierFlags::DECLARE | ModifierFlags::ABSTRACT,
            diagnostics::modifier_cannot_be_used_here,
        );

        Ok(self.ast.alloc_class(
            self.end_span(start_span),
            r#type,
            decorators,
            id,
            type_parameters,
            super_class,
            implements,
            body,
            modifiers.contains_abstract(),
            modifiers.contains_declare(),
        ))
    }

    pub(crate) fn parse_heritage_clause(
        &mut self,
    ) -> Result<(Option<Extends<'a>>, Option<Implements<'a>>)> {
        let mut extends = None;
        let mut implements = None;

        loop {
            match self.cur_kind() {
                Kind::Extends => {
                    extends = Some(self.parse_extends_clause()?);
                }
                Kind::Implements => {
                    implements = Some(self.parse_ts_implements_clause()?);
                }
                _ => break,
            }
        }

        Ok((extends, implements))
    }

    /// `ClassHeritage`
    /// extends `LeftHandSideExpression`[?Yield, ?Await]
    fn parse_extends_clause(&mut self) -> Result<Extends<'a>> {
        self.bump_any(); // bump `extends`
        let mut extends = self.ast.vec();

        let span = self.start_span();
        let mut first_extends = self.parse_lhs_expression_or_higher()?;
        let first_type_argument;
        if let Expression::TSInstantiationExpression(expr) = first_extends {
            let expr = expr.unbox();
            first_extends = expr.expression;
            first_type_argument = Some(expr.type_parameters);
        } else {
            first_type_argument = self.try_parse_type_arguments()?;
        }
        extends.push((first_extends, first_type_argument, self.end_span(span)));

        while self.eat(Kind::Comma) {
            let span = self.start_span();
            let mut extend = self.parse_lhs_expression_or_higher()?;
            let type_argument;
            if let Expression::TSInstantiationExpression(expr) = extend {
                let expr = expr.unbox();
                extend = expr.expression;
                type_argument = Some(expr.type_parameters);
            } else {
                type_argument = self.try_parse_type_arguments()?;
            }

            extends.push((extend, type_argument, self.end_span(span)));
        }

        Ok(extends)
    }

    fn parse_class_body(&mut self) -> Result<Box<'a, ClassBody<'a>>> {
        let span = self.start_span();
        let class_elements =
            self.parse_normal_list(Kind::LCurly, Kind::RCurly, Self::parse_class_element)?;
        Ok(self.ast.alloc_class_body(self.end_span(span), class_elements))
    }

    pub(crate) fn parse_class_element(&mut self) -> Result<Option<ClassElement<'a>>> {
        // skip empty class element `;`
        while self.at(Kind::Semicolon) {
            self.bump_any();
        }
        if self.at(Kind::RCurly) {
            return Ok(None);
        }

        let span = self.start_span();

        let modifiers = self.parse_modifiers(true, true, true);

        let mut kind = MethodDefinitionKind::Method;
        let mut generator = false;

        let mut key_name = None;

        let accessibility = modifiers.accessibility();
        let accessor = modifiers.contains(ModifierKind::Accessor);
        let declare = modifiers.contains(ModifierKind::Declare);
        let readonly = modifiers.contains(ModifierKind::Readonly);
        let r#override = modifiers.contains(ModifierKind::Override);
        let r#abstract = modifiers.contains(ModifierKind::Abstract);
        let mut r#static = modifiers.contains(ModifierKind::Static);
        let mut r#async = modifiers.contains(ModifierKind::Async);

        if self.at(Kind::Static) {
            // static { block }
            if self.peek_at(Kind::LCurly) {
                self.bump(Kind::Static);
                return self.parse_class_static_block(span).map(Some);
            }

            // static ...
            if self.peek_kind().is_class_element_name_start() || self.peek_at(Kind::Star) {
                self.bump(Kind::Static);
                r#static = true;
            } else {
                key_name = Some(self.parse_class_element_name()?);
            }
        }

        // async ...
        if key_name.is_none() && self.at(Kind::Async) && !self.peek_at(Kind::Question) {
            if !self.peek_token().is_on_new_line
                && (self.peek_kind().is_class_element_name_start() || self.peek_at(Kind::Star))
            {
                self.bump(Kind::Async);
                r#async = true;
            } else {
                key_name = Some(self.parse_class_element_name()?);
            }
        }

        if self.is_at_ts_index_signature_member() {
            return self
                .parse_index_signature_declaration(span, &modifiers)
                .map(|sig| Some(ClassElement::TSIndexSignature(self.alloc(sig))));
        }

        // * ...
        if key_name.is_none() && self.eat(Kind::Star) {
            generator = true;
        }

        if key_name.is_none() && !r#async && !generator {
            // get ... / set ...
            let peeked_class_element = self.peek_kind().is_class_element_name_start();
            key_name = match self.cur_kind() {
                Kind::Get if peeked_class_element => {
                    self.bump(Kind::Get);
                    kind = MethodDefinitionKind::Get;
                    Some(self.parse_class_element_name()?)
                }
                Kind::Set if peeked_class_element => {
                    self.bump(Kind::Set);
                    kind = MethodDefinitionKind::Set;
                    Some(self.parse_class_element_name()?)
                }
                kind if kind.is_class_element_name_start() => {
                    Some(self.parse_class_element_name()?)
                }
                _ => return Err(self.unexpected()),
            }
        }

        let (key, computed) =
            if let Some(result) = key_name { result } else { self.parse_class_element_name()? };

        let (optional, optional_span) = if self.at(Kind::Question) {
            let span = self.start_span();
            self.bump_any();
            (true, self.end_span(span))
        } else {
            (false, oxc_span::SPAN)
        };
        let definite = self.eat(Kind::Bang);

        if optional && definite {
            self.error(diagnostics::optional_definite_property(optional_span.expand_right(1)));
        }

        if modifiers.contains(ModifierKind::Const) {
            self.error(diagnostics::const_class_member(key.span()));
        }

        if let PropertyKey::PrivateIdentifier(private_ident) = &key {
            // `private #foo`, etc. is illegal
            if self.is_ts {
                self.verify_modifiers(
                    &modifiers,
                    ModifierFlags::all() - ModifierFlags::ACCESSIBILITY,
                    diagnostics::accessibility_modifier_on_private_property,
                );
            }
            if private_ident.name == "constructor" {
                self.error(diagnostics::private_name_constructor(private_ident.span));
            }
        }

        if accessor {
            if optional {
                self.error(diagnostics::optional_accessor_property(optional_span));
            }
            self.parse_class_accessor_property(
                span,
                key,
                computed,
                r#static,
                r#abstract,
                definite,
                accessibility,
            )
            .map(Some)
        } else if self.at(Kind::LParen) || self.at(Kind::LAngle) || r#async || generator {
            if !computed {
                if let Some((name, span)) = key.prop_name() {
                    if r#static && name == "prototype" && !self.ctx.has_ambient() {
                        self.error(diagnostics::static_prototype(span));
                    }
                    if !r#static && name == "constructor" {
                        if kind == MethodDefinitionKind::Get || kind == MethodDefinitionKind::Set {
                            self.error(diagnostics::constructor_getter_setter(span));
                        }
                        if r#async {
                            self.error(diagnostics::constructor_async(span));
                        }
                        if generator {
                            self.error(diagnostics::constructor_generator(span));
                        }
                    }
                }
            }
            // LAngle for start of type parameters `foo<T>`
            //                                         ^
            let definition = self.parse_class_method_definition(
                span,
                kind,
                key,
                computed,
                r#static,
                r#async,
                generator,
                r#override,
                r#abstract,
                accessibility,
                optional,
            )?;
            Ok(Some(definition))
        } else {
            // getter and setter has no ts type annotation
            if !kind.is_method() {
                return Err(self.unexpected());
            }
            if !computed {
                if let Some((name, span)) = key.prop_name() {
                    if name == "constructor" {
                        self.error(diagnostics::field_constructor(span));
                    }
                    if r#static && name == "prototype" && !self.ctx.has_ambient() {
                        self.error(diagnostics::static_prototype(span));
                    }
                }
            }
            let definition = self.parse_class_property_definition(
                span,
                key,
                computed,
                r#static,
                declare,
                r#override,
                readonly,
                r#abstract,
                accessibility,
                optional,
                definite,
            )?;
            Ok(Some(definition))
        }
    }

    fn parse_class_element_name(&mut self) -> Result<(PropertyKey<'a>, bool)> {
        match self.cur_kind() {
            Kind::PrivateIdentifier => {
                let private_ident = self.parse_private_identifier();
                Ok((PropertyKey::PrivateIdentifier(self.alloc(private_ident)), false))
            }
            _ => self.parse_property_name(),
        }
    }

    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
    fn parse_class_method_definition(
        &mut self,
        span: Span,
        kind: MethodDefinitionKind,
        key: PropertyKey<'a>,
        computed: bool,
        r#static: bool,
        r#async: bool,
        generator: bool,
        r#override: bool,
        r#abstract: bool,
        accessibility: Option<TSAccessibility>,
        optional: bool,
    ) -> Result<ClassElement<'a>> {
        let kind = if !r#static
            && !computed
            && key.prop_name().is_some_and(|(name, _)| name == "constructor")
        {
            MethodDefinitionKind::Constructor
        } else {
            kind
        };

        let decorators = self.consume_decorators();

        let value = self.parse_method(r#async, generator)?;

        if kind == MethodDefinitionKind::Constructor {
            if let Some(this_param) = &value.this_param {
                // class Foo { constructor(this: number) {} }
                self.error(diagnostics::ts_constructor_this_parameter(this_param.span));
            }

            if let Some(type_sig) = &value.type_parameters {
                // class Foo { constructor<T>(param: T ) {} }
                self.error(diagnostics::ts_constructor_type_parameter(type_sig.span));
            }

            if r#static {
                self.error(diagnostics::static_constructor(key.span()));
            }
        }

        let r#type = if r#abstract {
            MethodDefinitionType::TSAbstractMethodDefinition
        } else {
            MethodDefinitionType::MethodDefinition
        };
        Ok(self.ast.class_element_method_definition(
            self.end_span(span),
            r#type,
            decorators,
            key,
            value,
            kind,
            computed,
            r#static,
            r#override,
            optional,
            accessibility,
        ))
    }

    /// `FieldDefinition`[?Yield, ?Await] ;
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
    fn parse_class_property_definition(
        &mut self,
        span: Span,
        key: PropertyKey<'a>,
        computed: bool,
        r#static: bool,
        declare: bool,
        r#override: bool,
        readonly: bool,
        r#abstract: bool,
        accessibility: Option<TSAccessibility>,
        optional: bool,
        definite: bool,
    ) -> Result<ClassElement<'a>> {
        let type_annotation = if self.is_ts { self.parse_ts_type_annotation()? } else { None };
        let decorators = self.consume_decorators();
        let value = if self.eat(Kind::Eq) { Some(self.parse_expr()?) } else { None };
        self.asi()?;

        let r#type = if r#abstract {
            PropertyDefinitionType::TSAbstractPropertyDefinition
        } else {
            PropertyDefinitionType::PropertyDefinition
        };
        Ok(self.ast.class_element_property_definition(
            self.end_span(span),
            r#type,
            decorators,
            key,
            value,
            computed,
            r#static,
            declare,
            r#override,
            optional,
            definite,
            readonly,
            type_annotation,
            accessibility,
        ))
    }

    /// `ClassStaticBlockStatementList` :
    ///    `StatementList`[~Yield, +Await, ~Return]
    fn parse_class_static_block(&mut self, span: Span) -> Result<ClassElement<'a>> {
        let block =
            self.context(Context::Await, Context::Yield | Context::Return, Self::parse_block)?;
        Ok(self.ast.class_element_static_block(self.end_span(span), block.unbox().body))
    }

    /// <https://github.com/tc39/proposal-decorators>
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
    fn parse_class_accessor_property(
        &mut self,
        span: Span,
        key: PropertyKey<'a>,
        computed: bool,
        r#static: bool,
        r#abstract: bool,
        definite: bool,
        accessibility: Option<TSAccessibility>,
    ) -> Result<ClassElement<'a>> {
        let type_annotation = if self.is_ts { self.parse_ts_type_annotation()? } else { None };
        let value =
            self.eat(Kind::Eq).then(|| self.parse_assignment_expression_or_higher()).transpose()?;
        let r#type = if r#abstract {
            AccessorPropertyType::TSAbstractAccessorProperty
        } else {
            AccessorPropertyType::AccessorProperty
        };

        let decorators = self.consume_decorators();
        Ok(self.ast.class_element_accessor_property(
            self.end_span(span),
            r#type,
            decorators,
            key,
            value,
            computed,
            r#static,
            definite,
            type_annotation,
            accessibility,
        ))
    }
}
