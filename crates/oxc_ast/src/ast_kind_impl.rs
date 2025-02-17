#![allow(missing_docs)] // FIXME
use oxc_span::Atom;
use oxc_syntax::scope::ScopeId;

use super::{ast::*, AstKind};

impl<'a> AstKind<'a> {
    #[rustfmt::skip]
    pub fn is_statement(self) -> bool {
        self.is_iteration_statement()
            || matches!(self, Self::BlockStatement(_) | Self::BreakStatement(_) | Self::ContinueStatement(_)
                    | Self::DebuggerStatement(_) | Self::EmptyStatement(_) | Self::ExpressionStatement(_)
                    | Self::LabeledStatement(_) | Self::ReturnStatement(_) | Self::SwitchStatement(_)
                    | Self::ThrowStatement(_) | Self::TryStatement(_) | Self::WithStatement(_)
                    | Self::IfStatement(_) | Self::VariableDeclarationList(_) | Self::ExportDefaultDeclaration(_))
    }

    #[rustfmt::skip]
    pub fn is_declaration(self) -> bool {
        matches!(self, Self::Function(func) if func.is_declaration())
        || matches!(self, Self::Class(class) if class.is_declaration())
        || matches!(self, Self::ImportDeclaration(_) | Self::ExportAllDeclaration(_) | Self::ExportDefaultDeclaration(_)
            | Self::ExportNamedDeclaration(_) | Self::TSExportAssignment(_) | Self::TSNamespaceExportDeclaration(_)
            | Self::TSEnumDeclaration(_) | Self::TSModuleDeclaration(_) | Self::VariableDeclarationList(_)
            | Self::TSInterfaceDeclaration(_) | Self::TSTypeAliasDeclaration(_) | Self::TSImportEqualsDeclaration(_)
            | Self::PropertyDefinition(_)
        )
    }

    #[rustfmt::skip]
    pub fn is_iteration_statement(self) -> bool {
        matches!(self, Self::DoWhileStatement(_) | Self::WhileStatement(_) | Self::ForInStatement(_)
                | Self::ForOfStatement(_) | Self::ForStatement(_))
    }

    #[rustfmt::skip]
    pub fn is_identifier(self) -> bool {
        matches!(self, Self::BindingIdentifier(_)
                | Self::IdentifierReference(_)
                | Self::LabelIdentifier(_))
    }

    #[rustfmt::skip]
    pub fn is_type(self) -> bool {
        matches!(self, Self::TSAnyKeyword(_) | Self::TSBigIntKeyword(_) | Self::TSBooleanKeyword(_) | Self::TSIntrinsicKeyword(_)
                | Self::TSNeverKeyword(_) | Self::TSNullKeyword(_) | Self::TSNumberKeyword(_) | Self::TSObjectKeyword(_)
                | Self::TSStringKeyword(_) | Self::TSSymbolKeyword(_) | Self::TSUndefinedKeyword(_) | Self::TSUnknownKeyword(_)
                | Self::TSVoidKeyword(_) | Self::TSIndexedAccessType(_) | Self::TSInferType(_) | Self::TSIntersectionType(_)
                | Self::TSLiteralType(_) | Self::TSMethodSignature(_) | Self::TSTemplateLiteralType(_) | Self::TSThisType(_)
                | Self::TSTypeLiteral(_) | Self::TSTypeReference(_) | Self::TSUnionType(_))
    }

    pub fn is_literal(self) -> bool {
        matches!(
            self,
            Self::NumericLiteral(_)
                | Self::StringLiteral(_)
                | Self::BooleanLiteral(_)
                | Self::NullLiteral(_)
                | Self::BigIntLiteral(_)
                | Self::RegExpLiteral(_)
                | Self::TemplateLiteral(_)
        )
    }

    pub fn is_function_like(self) -> bool {
        matches!(self, Self::Function(_) | Self::ArrowFunctionExpression(_))
    }

    pub fn identifier_name(self) -> Option<Atom<'a>> {
        match self {
            Self::BindingIdentifier(ident) => Some(ident.name),
            Self::IdentifierReference(ident) => Some(ident.name),
            Self::LabelIdentifier(ident) => Some(ident.name),
            Self::IdentifierName(ident) => Some(ident.name),
            _ => None,
        }
    }

    pub fn is_jsx(self) -> bool {
        matches!(
            self,
            Self::JSXElement(_)
                | Self::JSXOpeningElement(_)
                | Self::JSXIdentifier(_)
                | Self::JSXNamespacedName(_)
                | Self::JSXMemberExpression(_)
                | Self::JSXFragment(_)
                | Self::JSXAttribute(_)
                | Self::JSXSpreadAttribute(_)
                | Self::JSXText(_)
                | Self::JSXExpressionContainer(_)
        )
    }

    pub fn is_specific_id_reference(&self, name: &str) -> bool {
        match self {
            Self::IdentifierReference(ident) => ident.name == name,
            _ => false,
        }
    }

    /// If this node is a container, get the [`ScopeId`] it creates.
    ///
    /// Will always be none if semantic analysis has not been run.
    pub fn get_container_scope_id(self) -> Option<ScopeId> {
        match self {
            Self::SourceFile(p) => Some(p.scope_id()),
            Self::BlockStatement(b) => Some(b.scope_id()),
            Self::ForStatement(f) => Some(f.scope_id()),
            Self::ForInStatement(f) => Some(f.scope_id()),
            Self::ForOfStatement(f) => Some(f.scope_id()),
            Self::SwitchStatement(switch) => Some(switch.scope_id()),
            Self::CatchClause(catch) => Some(catch.scope_id()),
            Self::Function(f) => Some(f.scope_id()),
            Self::ArrowFunctionExpression(f) => Some(f.scope_id()),
            Self::Class(class) => Some(class.scope_id()),
            Self::StaticBlock(b) => Some(b.scope_id()),
            Self::TSEnumDeclaration(e) => Some(e.scope_id()),
            Self::TSConditionalType(e) => Some(e.scope_id()),
            Self::TSTypeAliasDeclaration(e) => Some(e.scope_id()),
            Self::TSInterfaceDeclaration(e) => Some(e.scope_id()),
            Self::TSMethodSignature(e) => Some(e.scope_id()),
            Self::TSConstructSignatureDeclaration(e) => Some(e.scope_id()),
            Self::TSModuleDeclaration(e) => Some(e.scope_id()),
            Self::TSMappedType(e) => Some(e.scope_id()),
            _ => None,
        }
    }

    pub fn from_expression(e: &'a Expression<'a>) -> Self {
        match e {
            Expression::BooleanLiteral(e) => Self::BooleanLiteral(e),
            Expression::NullLiteral(e) => Self::NullLiteral(e),
            Expression::NumericLiteral(e) => Self::NumericLiteral(e),
            Expression::BigIntLiteral(e) => Self::BigIntLiteral(e),
            Expression::RegExpLiteral(e) => Self::RegExpLiteral(e),
            Expression::StringLiteral(e) => Self::StringLiteral(e),
            Expression::TemplateLiteral(e) => Self::TemplateLiteral(e),
            Expression::Identifier(e) => Self::IdentifierReference(e),
            Expression::MetaProperty(e) => Self::MetaProperty(e),
            Expression::Super(e) => Self::Super(e),
            Expression::ArrayExpression(e) => Self::ArrayExpression(e),
            Expression::ArrowFunctionExpression(e) => Self::ArrowFunctionExpression(e),
            Expression::AssignmentExpression(e) => Self::AssignmentExpression(e),
            Expression::AwaitExpression(e) => Self::AwaitExpression(e),
            Expression::BinaryExpression(e) => Self::BinaryExpression(e),
            Expression::CallExpression(e) => Self::CallExpression(e),
            Expression::ChainExpression(e) => Self::ChainExpression(e),
            Expression::ClassExpression(e) => Self::Class(e),
            Expression::ConditionalExpression(e) => Self::ConditionalExpression(e),
            Expression::FunctionExpression(e) => Self::Function(e),
            Expression::ImportExpression(e) => Self::ImportExpression(e),
            Expression::LogicalExpression(e) => Self::LogicalExpression(e),
            Expression::ElementAccessExpression(e) => Self::ElementAccessExpression(e),
            Expression::PropertyAccessExpression(e) => Self::PropertyAccessExpression(e),
            Expression::PrivateFieldExpression(e) => Self::PrivateFieldExpression(e),
            Expression::NewExpression(e) => Self::NewExpression(e),
            Expression::ObjectExpression(e) => Self::ObjectExpression(e),
            Expression::ParenthesizedExpression(e) => Self::ParenthesizedExpression(e),
            Expression::SequenceExpression(e) => Self::SequenceExpression(e),
            Expression::TaggedTemplateExpression(e) => Self::TaggedTemplateExpression(e),
            Expression::ThisExpression(e) => Self::ThisExpression(e),
            Expression::UnaryExpression(e) => Self::UnaryExpression(e),
            Expression::UpdateExpression(e) => Self::UpdateExpression(e),
            Expression::YieldExpression(e) => Self::YieldExpression(e),
            Expression::PrivateInExpression(e) => Self::PrivateInExpression(e),
            Expression::JSXElement(e) => Self::JSXElement(e),
            Expression::JSXFragment(e) => Self::JSXFragment(e),
            Expression::TSAsExpression(e) => Self::TSAsExpression(e),
            Expression::TSSatisfiesExpression(e) => Self::TSSatisfiesExpression(e),
            Expression::TSTypeAssertion(e) => Self::TSTypeAssertion(e),
            Expression::TSNonNullExpression(e) => Self::TSNonNullExpression(e),
            Expression::TSInstantiationExpression(e) => Self::TSInstantiationExpression(e),
        }
    }

    pub fn from_statement(s: &'a Statement<'a>) -> Self {
        match s {
            Statement::BlockStatement(s) => Self::BlockStatement(s),
            Statement::BreakStatement(s) => Self::BreakStatement(s),
            Statement::ContinueStatement(s) => Self::ContinueStatement(s),
            Statement::DebuggerStatement(s) => Self::DebuggerStatement(s),
            Statement::DoWhileStatement(s) => Self::DoWhileStatement(s),
            Statement::EmptyStatement(s) => Self::EmptyStatement(s),
            Statement::ExpressionStatement(s) => Self::ExpressionStatement(s),
            Statement::ForInStatement(s) => Self::ForInStatement(s),
            Statement::ForOfStatement(s) => Self::ForOfStatement(s),
            Statement::ForStatement(s) => Self::ForStatement(s),
            Statement::IfStatement(s) => Self::IfStatement(s),
            Statement::LabeledStatement(s) => Self::LabeledStatement(s),
            Statement::ReturnStatement(s) => Self::ReturnStatement(s),
            Statement::SwitchStatement(s) => Self::SwitchStatement(s),
            Statement::ThrowStatement(s) => Self::ThrowStatement(s),
            Statement::TryStatement(s) => Self::TryStatement(s),
            Statement::WhileStatement(s) => Self::WhileStatement(s),
            Statement::WithStatement(s) => Self::WithStatement(s),

            Statement::VariableDeclarationList(s) => Self::VariableDeclarationList(s),
            Statement::FunctionDeclaration(s) => Self::Function(s),
            Statement::ClassDeclaration(s) => Self::Class(s),
            Statement::TSTypeAliasDeclaration(s) => Self::TSTypeAliasDeclaration(s),
            Statement::TSInterfaceDeclaration(s) => Self::TSInterfaceDeclaration(s),
            Statement::TSEnumDeclaration(s) => Self::TSEnumDeclaration(s),
            Statement::TSModuleDeclaration(s) => Self::TSModuleDeclaration(s),
            Statement::TSImportEqualsDeclaration(s) => Self::TSImportEqualsDeclaration(s),

            Statement::ImportDeclaration(s) => Self::ImportDeclaration(s),
            Statement::ExportAllDeclaration(s) => Self::ExportAllDeclaration(s),
            Statement::ExportDefaultDeclaration(s) => Self::ExportDefaultDeclaration(s),
            Statement::ExportNamedDeclaration(s) => Self::ExportNamedDeclaration(s),
            Statement::TSExportAssignment(s) => Self::TSExportAssignment(s),
            Statement::TSNamespaceExportDeclaration(s) => Self::TSNamespaceExportDeclaration(s),
        }
    }

    pub fn from_declaration(d: &'a Declaration<'a>) -> Self {
        match d {
            Declaration::VariableDeclarationList(d) => Self::VariableDeclarationList(d),
            Declaration::FunctionDeclaration(d) => Self::Function(d),
            Declaration::ClassDeclaration(d) => Self::Class(d),
            Declaration::TSTypeAliasDeclaration(d) => Self::TSTypeAliasDeclaration(d),
            Declaration::TSInterfaceDeclaration(d) => Self::TSInterfaceDeclaration(d),
            Declaration::TSEnumDeclaration(d) => Self::TSEnumDeclaration(d),
            Declaration::TSModuleDeclaration(d) => Self::TSModuleDeclaration(d),
            Declaration::TSImportEqualsDeclaration(d) => Self::TSImportEqualsDeclaration(d),
        }
    }

    pub fn from_module_declaration(m: &'a ModuleDeclaration<'a>) -> Self {
        match m {
            ModuleDeclaration::ImportDeclaration(m) => Self::ImportDeclaration(m),
            ModuleDeclaration::ExportAllDeclaration(m) => Self::ExportAllDeclaration(m),
            ModuleDeclaration::ExportDefaultDeclaration(m) => Self::ExportDefaultDeclaration(m),
            ModuleDeclaration::ExportNamedDeclaration(m) => Self::ExportNamedDeclaration(m),
            ModuleDeclaration::TSExportAssignment(m) => Self::TSExportAssignment(m),
            ModuleDeclaration::TSNamespaceExportDeclaration(m) => {
                Self::TSNamespaceExportDeclaration(m)
            }
        }
    }

    pub fn from_destructure_binding_pattern_kind(b: &'a DestructureBindingPatternKind<'a>) -> Self {
        match b {
            DestructureBindingPatternKind::BindingIdentifier(b) => Self::BindingIdentifier(b),
            DestructureBindingPatternKind::ObjectPattern(b) => Self::ObjectPattern(b),
            DestructureBindingPatternKind::ArrayPattern(b) => Self::ArrayPattern(b),
            DestructureBindingPatternKind::AssignmentPattern(b) => Self::AssignmentPattern(b),
        }
    }

    pub fn from_chain_element(c: &'a ChainElement<'a>) -> Self {
        match c {
            ChainElement::CallExpression(c) => Self::CallExpression(c),
            ChainElement::TSNonNullExpression(c) => Self::TSNonNullExpression(c),
            ChainElement::ElementAccessExpression(c) => Self::ElementAccessExpression(c),
            ChainElement::PropertyAccessExpression(c) => Self::PropertyAccessExpression(c),
            ChainElement::PrivateFieldExpression(c) => Self::PrivateFieldExpression(c),
        }
    }

    pub fn from_module_export_name(m: &'a ModuleExportName<'a>) -> Self {
        match m {
            ModuleExportName::IdentifierName(m) => Self::IdentifierName(m),
            ModuleExportName::IdentifierReference(m) => Self::IdentifierReference(m),
            ModuleExportName::StringLiteral(m) => Self::StringLiteral(m),
        }
    }

    pub fn from_assignment_target_maybe_default(a: &'a AssignmentTargetMaybeDefault<'a>) -> Self {
        match a {
            AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(a) => {
                Self::AssignmentTargetWithDefault(a)
            }
            AssignmentTargetMaybeDefault::AssignmentTargetIdentifier(a) => {
                Self::IdentifierReference(a)
            }
            AssignmentTargetMaybeDefault::TSAsExpression(a) => Self::TSAsExpression(a),
            AssignmentTargetMaybeDefault::TSSatisfiesExpression(a) => {
                Self::TSSatisfiesExpression(a)
            }
            AssignmentTargetMaybeDefault::TSNonNullExpression(a) => Self::TSNonNullExpression(a),
            AssignmentTargetMaybeDefault::TSTypeAssertion(a) => Self::TSTypeAssertion(a),
            AssignmentTargetMaybeDefault::TSInstantiationExpression(a) => {
                Self::TSInstantiationExpression(a)
            }
            AssignmentTargetMaybeDefault::ElementAccessExpression(a) => {
                Self::ElementAccessExpression(a)
            }
            AssignmentTargetMaybeDefault::PropertyAccessExpression(a) => {
                Self::PropertyAccessExpression(a)
            }
            AssignmentTargetMaybeDefault::PrivateFieldExpression(a) => {
                Self::PrivateFieldExpression(a)
            }
            AssignmentTargetMaybeDefault::ArrayAssignmentTarget(a) => {
                Self::ArrayAssignmentTarget(a)
            }
            AssignmentTargetMaybeDefault::ObjectAssignmentTarget(a) => {
                Self::ObjectAssignmentTarget(a)
            }
        }
    }

    pub fn from_assignment_target_property(a: &'a AssignmentTargetProperty<'a>) -> Self {
        match a {
            AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(a) => {
                Self::AssignmentTargetPropertyIdentifier(a)
            }
            AssignmentTargetProperty::AssignmentTargetPropertyProperty(a) => {
                Self::AssignmentTargetPropertyProperty(a)
            }
        }
    }

    pub fn from_for_statement_left(f: &'a ForStatementLeft<'a>) -> Self {
        match f {
            ForStatementLeft::VariableDeclarationList(f) => Self::VariableDeclarationList(f),

            ForStatementLeft::AssignmentTargetIdentifier(a) => Self::IdentifierReference(a),
            ForStatementLeft::TSAsExpression(a) => Self::TSAsExpression(a),
            ForStatementLeft::TSSatisfiesExpression(a) => Self::TSSatisfiesExpression(a),
            ForStatementLeft::TSNonNullExpression(a) => Self::TSNonNullExpression(a),
            ForStatementLeft::TSTypeAssertion(a) => Self::TSTypeAssertion(a),
            ForStatementLeft::TSInstantiationExpression(a) => Self::TSInstantiationExpression(a),
            ForStatementLeft::ElementAccessExpression(a) => Self::ElementAccessExpression(a),
            ForStatementLeft::PropertyAccessExpression(a) => Self::PropertyAccessExpression(a),
            ForStatementLeft::PrivateFieldExpression(a) => Self::PrivateFieldExpression(a),
            ForStatementLeft::ArrayAssignmentTarget(a) => Self::ArrayAssignmentTarget(a),
            ForStatementLeft::ObjectAssignmentTarget(a) => Self::ObjectAssignmentTarget(a),
        }
    }

    pub fn from_import_attribute_key(k: &'a ImportAttributeKey<'a>) -> Self {
        match k {
            ImportAttributeKey::Identifier(k) => Self::IdentifierName(k),
            ImportAttributeKey::StringLiteral(k) => Self::StringLiteral(k),
        }
    }

    pub fn from_export_default_declaration_kind(e: &'a ExportDefaultDeclarationKind<'a>) -> Self {
        match e {
            ExportDefaultDeclarationKind::FunctionDeclaration(e) => Self::Function(e),
            ExportDefaultDeclarationKind::ClassDeclaration(e) => Self::Class(e),
            ExportDefaultDeclarationKind::TSInterfaceDeclaration(e) => {
                Self::TSInterfaceDeclaration(e)
            }
            ExportDefaultDeclarationKind::BooleanLiteral(e) => Self::BooleanLiteral(e),
            ExportDefaultDeclarationKind::NullLiteral(e) => Self::NullLiteral(e),
            ExportDefaultDeclarationKind::NumericLiteral(e) => Self::NumericLiteral(e),
            ExportDefaultDeclarationKind::BigIntLiteral(e) => Self::BigIntLiteral(e),
            ExportDefaultDeclarationKind::RegExpLiteral(e) => Self::RegExpLiteral(e),
            ExportDefaultDeclarationKind::StringLiteral(e) => Self::StringLiteral(e),
            ExportDefaultDeclarationKind::TemplateLiteral(e) => Self::TemplateLiteral(e),
            ExportDefaultDeclarationKind::Identifier(e) => Self::IdentifierReference(e),
            ExportDefaultDeclarationKind::MetaProperty(e) => Self::MetaProperty(e),
            ExportDefaultDeclarationKind::Super(e) => Self::Super(e),
            ExportDefaultDeclarationKind::ArrayExpression(e) => Self::ArrayExpression(e),
            ExportDefaultDeclarationKind::ArrowFunctionExpression(e) => {
                Self::ArrowFunctionExpression(e)
            }
            ExportDefaultDeclarationKind::AssignmentExpression(e) => Self::AssignmentExpression(e),
            ExportDefaultDeclarationKind::AwaitExpression(e) => Self::AwaitExpression(e),
            ExportDefaultDeclarationKind::BinaryExpression(e) => Self::BinaryExpression(e),
            ExportDefaultDeclarationKind::CallExpression(e) => Self::CallExpression(e),
            ExportDefaultDeclarationKind::ChainExpression(e) => Self::ChainExpression(e),
            ExportDefaultDeclarationKind::ClassExpression(e) => Self::Class(e),
            ExportDefaultDeclarationKind::ConditionalExpression(e) => {
                Self::ConditionalExpression(e)
            }
            ExportDefaultDeclarationKind::FunctionExpression(e) => Self::Function(e),
            ExportDefaultDeclarationKind::ImportExpression(e) => Self::ImportExpression(e),
            ExportDefaultDeclarationKind::LogicalExpression(e) => Self::LogicalExpression(e),
            ExportDefaultDeclarationKind::NewExpression(e) => Self::NewExpression(e),
            ExportDefaultDeclarationKind::ObjectExpression(e) => Self::ObjectExpression(e),
            ExportDefaultDeclarationKind::ParenthesizedExpression(e) => {
                Self::ParenthesizedExpression(e)
            }
            ExportDefaultDeclarationKind::SequenceExpression(e) => Self::SequenceExpression(e),
            ExportDefaultDeclarationKind::TaggedTemplateExpression(e) => {
                Self::TaggedTemplateExpression(e)
            }
            ExportDefaultDeclarationKind::ThisExpression(e) => Self::ThisExpression(e),
            ExportDefaultDeclarationKind::UnaryExpression(e) => Self::UnaryExpression(e),
            ExportDefaultDeclarationKind::UpdateExpression(e) => Self::UpdateExpression(e),
            ExportDefaultDeclarationKind::YieldExpression(e) => Self::YieldExpression(e),
            ExportDefaultDeclarationKind::PrivateInExpression(e) => Self::PrivateInExpression(e),
            ExportDefaultDeclarationKind::JSXElement(e) => Self::JSXElement(e),
            ExportDefaultDeclarationKind::JSXFragment(e) => Self::JSXFragment(e),
            ExportDefaultDeclarationKind::TSAsExpression(e) => Self::TSAsExpression(e),
            ExportDefaultDeclarationKind::TSSatisfiesExpression(e) => {
                Self::TSSatisfiesExpression(e)
            }
            ExportDefaultDeclarationKind::TSTypeAssertion(e) => Self::TSTypeAssertion(e),
            ExportDefaultDeclarationKind::TSNonNullExpression(e) => Self::TSNonNullExpression(e),
            ExportDefaultDeclarationKind::TSInstantiationExpression(e) => {
                Self::TSInstantiationExpression(e)
            }
            ExportDefaultDeclarationKind::ElementAccessExpression(e) => {
                Self::ElementAccessExpression(e)
            }
            ExportDefaultDeclarationKind::PropertyAccessExpression(e) => {
                Self::PropertyAccessExpression(e)
            }
            ExportDefaultDeclarationKind::PrivateFieldExpression(e) => {
                Self::PrivateFieldExpression(e)
            }
        }
    }

    pub fn from_import_declaration_specifier(i: &'a ImportDeclarationSpecifier<'a>) -> Self {
        match i {
            ImportDeclarationSpecifier::ImportSpecifier(i) => Self::ImportSpecifier(i),
            ImportDeclarationSpecifier::ImportDefaultSpecifier(i) => {
                Self::ImportDefaultSpecifier(i)
            }
            ImportDeclarationSpecifier::ImportNamespaceSpecifier(i) => {
                Self::ImportNamespaceSpecifier(i)
            }
        }
    }

    pub fn from_object_property_kind(o: &'a ObjectPropertyKind<'a>) -> Self {
        match o {
            ObjectPropertyKind::ObjectProperty(o) => Self::ObjectProperty(o),
            ObjectPropertyKind::SpreadProperty(o) => Self::SpreadElement(o),
        }
    }

    pub fn from_class_element(c: &'a ClassElement<'a>) -> Self {
        match c {
            ClassElement::StaticBlock(c) => Self::StaticBlock(c),
            ClassElement::MethodDefinition(c) => Self::MethodDefinition(c),
            ClassElement::PropertyDefinition(c) => Self::PropertyDefinition(c),
            ClassElement::AccessorProperty(c) => Self::AccessorProperty(c),
            ClassElement::TSIndexSignature(c) => Self::TSIndexSignature(c),
        }
    }

    pub fn from_jsx_attribute_name(j: &'a JSXAttributeName<'a>) -> Self {
        match j {
            JSXAttributeName::Identifier(j) => Self::JSXIdentifier(j),
            JSXAttributeName::NamespacedName(j) => Self::JSXNamespacedName(j),
        }
    }

    pub fn from_jsx_attribute_value(j: &'a JSXAttributeValue<'a>) -> Self {
        match j {
            JSXAttributeValue::StringLiteral(j) => Self::StringLiteral(j),
            JSXAttributeValue::ExpressionContainer(j) => Self::JSXExpressionContainer(j),
            JSXAttributeValue::Element(j) => Self::JSXElement(j),
            JSXAttributeValue::Fragment(j) => Self::JSXFragment(j),
        }
    }

    pub fn from_jsx_expression(j: &'a JSXExpression<'a>) -> Self {
        match j {
            JSXExpression::EmptyExpression(j) => Self::JSXEmptyExpression(j),
            JSXExpression::BooleanLiteral(j) => Self::BooleanLiteral(j),
            JSXExpression::NullLiteral(j) => Self::NullLiteral(j),
            JSXExpression::NumericLiteral(j) => Self::NumericLiteral(j),
            JSXExpression::BigIntLiteral(j) => Self::BigIntLiteral(j),
            JSXExpression::RegExpLiteral(j) => Self::RegExpLiteral(j),
            JSXExpression::StringLiteral(j) => Self::StringLiteral(j),
            JSXExpression::TemplateLiteral(j) => Self::TemplateLiteral(j),
            JSXExpression::Identifier(j) => Self::IdentifierReference(j),
            JSXExpression::MetaProperty(j) => Self::MetaProperty(j),
            JSXExpression::Super(j) => Self::Super(j),
            JSXExpression::ArrayExpression(j) => Self::ArrayExpression(j),
            JSXExpression::ArrowFunctionExpression(j) => Self::ArrowFunctionExpression(j),
            JSXExpression::AssignmentExpression(j) => Self::AssignmentExpression(j),
            JSXExpression::AwaitExpression(j) => Self::AwaitExpression(j),
            JSXExpression::BinaryExpression(j) => Self::BinaryExpression(j),
            JSXExpression::CallExpression(j) => Self::CallExpression(j),
            JSXExpression::ChainExpression(j) => Self::ChainExpression(j),
            JSXExpression::ClassExpression(j) => Self::Class(j),
            JSXExpression::ConditionalExpression(j) => Self::ConditionalExpression(j),
            JSXExpression::FunctionExpression(j) => Self::Function(j),
            JSXExpression::ImportExpression(j) => Self::ImportExpression(j),
            JSXExpression::LogicalExpression(j) => Self::LogicalExpression(j),
            JSXExpression::NewExpression(j) => Self::NewExpression(j),
            JSXExpression::ObjectExpression(j) => Self::ObjectExpression(j),
            JSXExpression::ParenthesizedExpression(j) => Self::ParenthesizedExpression(j),
            JSXExpression::SequenceExpression(j) => Self::SequenceExpression(j),
            JSXExpression::TaggedTemplateExpression(j) => Self::TaggedTemplateExpression(j),
            JSXExpression::ThisExpression(j) => Self::ThisExpression(j),
            JSXExpression::UnaryExpression(j) => Self::UnaryExpression(j),
            JSXExpression::UpdateExpression(j) => Self::UpdateExpression(j),
            JSXExpression::YieldExpression(j) => Self::YieldExpression(j),
            JSXExpression::PrivateInExpression(j) => Self::PrivateInExpression(j),
            JSXExpression::JSXElement(j) => Self::JSXElement(j),
            JSXExpression::JSXFragment(j) => Self::JSXFragment(j),
            JSXExpression::TSAsExpression(j) => Self::TSAsExpression(j),
            JSXExpression::TSSatisfiesExpression(j) => Self::TSSatisfiesExpression(j),
            JSXExpression::TSTypeAssertion(j) => Self::TSTypeAssertion(j),
            JSXExpression::TSNonNullExpression(j) => Self::TSNonNullExpression(j),
            JSXExpression::TSInstantiationExpression(j) => Self::TSInstantiationExpression(j),
            JSXExpression::ElementAccessExpression(j) => Self::ElementAccessExpression(j),
            JSXExpression::PropertyAccessExpression(j) => Self::PropertyAccessExpression(j),
            JSXExpression::PrivateFieldExpression(j) => Self::PrivateFieldExpression(j),
        }
    }

    pub fn from_jsx_child(j: &'a JSXChild<'a>) -> Self {
        match j {
            JSXChild::Text(j) => Self::JSXText(j),
            JSXChild::Element(j) => Self::JSXElement(j),
            JSXChild::Fragment(j) => Self::JSXFragment(j),
            JSXChild::ExpressionContainer(j) => Self::JSXExpressionContainer(j),
            JSXChild::Spread(j) => Self::JSXSpreadChild(j),
        }
    }

    pub fn from_jsx_attribute_item(j: &'a JSXAttributeItem<'a>) -> Self {
        match j {
            JSXAttributeItem::Attribute(j) => Self::JSXAttribute(j),
            JSXAttributeItem::SpreadAttribute(j) => Self::JSXSpreadAttribute(j),
        }
    }

    pub fn from_ts_type(t: &'a TSType<'a>) -> Self {
        match t {
            TSType::TSAnyKeyword(t) => Self::TSAnyKeyword(t),
            TSType::TSBigIntKeyword(t) => Self::TSBigIntKeyword(t),
            TSType::TSBooleanKeyword(t) => Self::TSBooleanKeyword(t),
            TSType::TSIntrinsicKeyword(t) => Self::TSIntrinsicKeyword(t),
            TSType::TSNeverKeyword(t) => Self::TSNeverKeyword(t),
            TSType::TSNullKeyword(t) => Self::TSNullKeyword(t),
            TSType::TSNumberKeyword(t) => Self::TSNumberKeyword(t),
            TSType::TSObjectKeyword(t) => Self::TSObjectKeyword(t),
            TSType::TSStringKeyword(t) => Self::TSStringKeyword(t),
            TSType::TSSymbolKeyword(t) => Self::TSSymbolKeyword(t),
            TSType::TSUndefinedKeyword(t) => Self::TSUndefinedKeyword(t),
            TSType::TSUnknownKeyword(t) => Self::TSUnknownKeyword(t),
            TSType::TSVoidKeyword(t) => Self::TSVoidKeyword(t),
            TSType::TSArrayType(t) => Self::TSArrayType(t),
            TSType::TSConditionalType(t) => Self::TSConditionalType(t),
            TSType::TSConstructorType(t) => Self::TSConstructorType(t),
            TSType::TSFunctionType(t) => Self::TSFunctionType(t),
            TSType::TSImportType(t) => Self::TSImportType(t),
            TSType::TSIndexedAccessType(t) => Self::TSIndexedAccessType(t),
            TSType::TSInferType(t) => Self::TSInferType(t),
            TSType::TSIntersectionType(t) => Self::TSIntersectionType(t),
            TSType::TSLiteralType(t) => Self::TSLiteralType(t),
            TSType::TSMappedType(t) => Self::TSMappedType(t),
            TSType::TSNamedTupleMember(t) => Self::TSNamedTupleMember(t),
            TSType::TSQualifiedName(t) => Self::TSQualifiedName(t),
            TSType::TSTemplateLiteralType(t) => Self::TSTemplateLiteralType(t),
            TSType::TSThisType(t) => Self::TSThisType(t),
            TSType::TSTupleType(t) => Self::TSTupleType(t),
            TSType::TSTypeLiteral(t) => Self::TSTypeLiteral(t),
            TSType::TSTypeOperatorType(t) => Self::TSTypeOperator(t),
            TSType::TSTypePredicate(t) => Self::TSTypePredicate(t),
            TSType::TSTypeQuery(t) => Self::TSTypeQuery(t),
            TSType::TSTypeReference(t) => Self::TSTypeReference(t),
            TSType::TSUnionType(t) => Self::TSUnionType(t),
            TSType::TSParenthesizedType(t) => Self::TSParenthesizedType(t),
            TSType::JSDocNullableType(t) => Self::JSDocNullableType(t),
            TSType::JSDocNonNullableType(t) => Self::JSDocNonNullableType(t),
            TSType::JSDocUnknownType(t) => Self::JSDocUnknownType(t),
        }
    }

    pub fn from_ts_import_attribute_name(t: &'a TSImportAttributeName<'a>) -> Self {
        match t {
            TSImportAttributeName::Identifier(t) => Self::IdentifierName(t),
            TSImportAttributeName::StringLiteral(t) => Self::StringLiteral(t),
        }
    }

    pub fn from_ts_type_query_expr_name(t: &'a TSTypeQueryExprName<'a>) -> Self {
        match t {
            TSTypeQueryExprName::TSImportType(t) => Self::TSImportType(t),
            TSTypeQueryExprName::IdentifierReference(t) => Self::IdentifierReference(t),
            TSTypeQueryExprName::QualifiedName(t) => Self::TSQualifiedName(t),
        }
    }

    pub fn from_ts_module_declaration_name(t: &'a TSModuleDeclarationName<'a>) -> Self {
        match t {
            TSModuleDeclarationName::Identifier(t) => Self::BindingIdentifier(t),
            TSModuleDeclarationName::StringLiteral(t) => Self::StringLiteral(t),
        }
    }

    pub fn from_ts_type_predicate_name(t: &'a TSTypePredicateName<'a>) -> Self {
        match t {
            TSTypePredicateName::Identifier(t) => Self::IdentifierName(t),
            TSTypePredicateName::This(t) => Self::TSThisType(t),
        }
    }

    pub fn from_ts_tuple_element(t: &'a TSTupleElement<'a>) -> Self {
        match t {
            TSTupleElement::TSOptionalType(t) => Self::TSOptionalType(t),
            TSTupleElement::TSRestType(t) => Self::TSRestType(t),
            TSTupleElement::TSAnyKeyword(t) => Self::TSAnyKeyword(t),
            TSTupleElement::TSBigIntKeyword(t) => Self::TSBigIntKeyword(t),
            TSTupleElement::TSBooleanKeyword(t) => Self::TSBooleanKeyword(t),
            TSTupleElement::TSIntrinsicKeyword(t) => Self::TSIntrinsicKeyword(t),
            TSTupleElement::TSNeverKeyword(t) => Self::TSNeverKeyword(t),
            TSTupleElement::TSNullKeyword(t) => Self::TSNullKeyword(t),
            TSTupleElement::TSNumberKeyword(t) => Self::TSNumberKeyword(t),
            TSTupleElement::TSObjectKeyword(t) => Self::TSObjectKeyword(t),
            TSTupleElement::TSStringKeyword(t) => Self::TSStringKeyword(t),
            TSTupleElement::TSSymbolKeyword(t) => Self::TSSymbolKeyword(t),
            TSTupleElement::TSUndefinedKeyword(t) => Self::TSUndefinedKeyword(t),
            TSTupleElement::TSUnknownKeyword(t) => Self::TSUnknownKeyword(t),
            TSTupleElement::TSVoidKeyword(t) => Self::TSVoidKeyword(t),
            TSTupleElement::TSArrayType(t) => Self::TSArrayType(t),
            TSTupleElement::TSConditionalType(t) => Self::TSConditionalType(t),
            TSTupleElement::TSConstructorType(t) => Self::TSConstructorType(t),
            TSTupleElement::TSFunctionType(t) => Self::TSFunctionType(t),
            TSTupleElement::TSImportType(t) => Self::TSImportType(t),
            TSTupleElement::TSIndexedAccessType(t) => Self::TSIndexedAccessType(t),
            TSTupleElement::TSInferType(t) => Self::TSInferType(t),
            TSTupleElement::TSIntersectionType(t) => Self::TSIntersectionType(t),
            TSTupleElement::TSLiteralType(t) => Self::TSLiteralType(t),
            TSTupleElement::TSMappedType(t) => Self::TSMappedType(t),
            TSTupleElement::TSNamedTupleMember(t) => Self::TSNamedTupleMember(t),
            TSTupleElement::TSQualifiedName(t) => Self::TSQualifiedName(t),
            TSTupleElement::TSTemplateLiteralType(t) => Self::TSTemplateLiteralType(t),
            TSTupleElement::TSThisType(t) => Self::TSThisType(t),
            TSTupleElement::TSTupleType(t) => Self::TSTupleType(t),
            TSTupleElement::TSTypeLiteral(t) => Self::TSTypeLiteral(t),
            TSTupleElement::TSTypeOperatorType(t) => Self::TSTypeOperator(t),
            TSTupleElement::TSTypePredicate(t) => Self::TSTypePredicate(t),
            TSTupleElement::TSTypeQuery(t) => Self::TSTypeQuery(t),
            TSTupleElement::TSTypeReference(t) => Self::TSTypeReference(t),
            TSTupleElement::TSUnionType(t) => Self::TSUnionType(t),
            TSTupleElement::TSParenthesizedType(t) => Self::TSParenthesizedType(t),
            TSTupleElement::JSDocNullableType(t) => Self::JSDocNullableType(t),
            TSTupleElement::JSDocNonNullableType(t) => Self::JSDocNonNullableType(t),
            TSTupleElement::JSDocUnknownType(t) => Self::JSDocUnknownType(t),
        }
    }

    pub fn from_ts_literal(t: &'a TSLiteral<'a>) -> Self {
        match t {
            TSLiteral::BooleanLiteral(t) => Self::BooleanLiteral(t),
            TSLiteral::NullLiteral(t) => Self::NullLiteral(t),
            TSLiteral::NumericLiteral(t) => Self::NumericLiteral(t),
            TSLiteral::BigIntLiteral(t) => Self::BigIntLiteral(t),
            TSLiteral::RegExpLiteral(t) => Self::RegExpLiteral(t),
            TSLiteral::StringLiteral(t) => Self::StringLiteral(t),
            TSLiteral::TemplateLiteral(t) => Self::TemplateLiteral(t),
            TSLiteral::UnaryExpression(t) => Self::UnaryExpression(t),
        }
    }

    pub fn from_ts_enum_member_name(t: &'a TSEnumMemberName<'a>) -> Self {
        match t {
            TSEnumMemberName::Identifier(t) => Self::IdentifierName(t),
            TSEnumMemberName::String(t) => Self::StringLiteral(t),
        }
    }

    pub fn from_ts_signature(t: &'a TSSignature<'a>) -> Self {
        match t {
            TSSignature::TSIndexSignature(t) => Self::TSIndexSignature(t),
            TSSignature::TSPropertySignature(t) => Self::TSPropertySignature(t),
            TSSignature::TSCallSignatureDeclaration(t) => Self::TSCallSignatureDeclaration(t),
            TSSignature::TSConstructSignatureDeclaration(t) => {
                Self::TSConstructSignatureDeclaration(t)
            }
            TSSignature::TSMethodSignature(t) => Self::TSMethodSignature(t),
        }
    }

    pub fn from_ts_module_declaration_body(t: &'a TSModuleDeclarationBody<'a>) -> Self {
        match t {
            TSModuleDeclarationBody::TSModuleDeclaration(t) => Self::TSModuleDeclaration(t),
            TSModuleDeclarationBody::TSModuleBlock(t) => Self::TSModuleBlock(t),
        }
    }
}

impl AstKind<'_> {
    #[allow(clippy::match_same_arms)]
    /// Get the AST kind name with minimal details. Particularly useful for
    /// when debugging an iteration over an AST.
    ///
    /// Note that this method does not exist in release builds. Do not include
    /// usage of this method within your code.
    pub fn debug_name(&self) -> std::borrow::Cow<str> {
        use std::borrow::Cow;

        const COMPUTED: Cow<'static, str> = Cow::Borrowed("<computed>");
        #[allow(dead_code)]
        const UNKNOWN: Cow<'static, str> = Cow::Borrowed("<unknown>");
        const ANONYMOUS: Cow<'static, str> = Cow::Borrowed("<anonymous>");
        const DESTRUCTURE: Cow<'static, str> = Cow::Borrowed("<destructure>");

        #[inline]
        fn or_anonymous<'a>(id: Option<&BindingIdentifier<'a>>) -> Cow<'a, str> {
            id.map_or_else(|| ANONYMOUS.as_ref(), |id| id.name.as_str()).into()
        }

        match self {
            Self::SourceFile(_) => "SourceFile".into(),
            Self::Directive(d) => d.directive.as_ref().into(),
            Self::Hashbang(_) => "Hashbang".into(),

            Self::BlockStatement(_) => "BlockStatement".into(),
            Self::BreakStatement(_) => "BreakStatement".into(),
            Self::ContinueStatement(_) => "ContinueStatement".into(),
            Self::DebuggerStatement(_) => "DebuggerStatement".into(),
            Self::DestructureBindingPattern(_) => "DestructureBindingPattern".into(),
            Self::DoWhileStatement(_) => "DoWhileStatement".into(),
            Self::EmptyStatement(_) => "EmptyStatement".into(),
            Self::ExpressionStatement(_) => "ExpressionStatement".into(),
            Self::ForInStatement(_) => "ForInStatement".into(),
            Self::ForOfStatement(_) => "ForOfStatement".into(),
            Self::ForStatement(_) => "ForStatement".into(),
            Self::IfStatement(_) => "IfStatement".into(),
            Self::LabeledStatement(l) => format!("LabeledStatement({})", l.label.name).into(),
            Self::ReturnStatement(_) => "ReturnStatement".into(),
            Self::SwitchStatement(_) => "SwitchStatement".into(),
            Self::ThrowStatement(_) => "ThrowStatement".into(),
            Self::TryStatement(_) => "TryStatement".into(),
            Self::WhileStatement(_) => "WhileStatement".into(),
            Self::WithStatement(_) => "WithStatement".into(),

            Self::SwitchCase(_) => "SwitchCase".into(),
            Self::CatchClause(_) => "CatchClause".into(),

            Self::VariableDeclarationList(_) => "VariableDeclarationList".into(),
            Self::VariableDeclarator(v) => format!(
                "VariableDeclarator({})",
                v.id.get_identifier().unwrap_or(Atom::from(DESTRUCTURE.as_ref()))
            )
            .into(),

            Self::IdentifierName(x) => format!("IdentifierName({})", x.name).into(),
            Self::IdentifierReference(x) => format!("IdentifierReference({})", x.name).into(),
            Self::BindingIdentifier(x) => format!("BindingIdentifier({})", x.name).into(),
            Self::LabelIdentifier(x) => format!("LabelIdentifier({})", x.name).into(),
            Self::PrivateIdentifier(x) => format!("PrivateIdentifier({})", x.name).into(),

            Self::NumericLiteral(n) => format!("NumericLiteral({})", n.value).into(),
            Self::StringLiteral(s) => format!("StringLiteral({})", s.value).into(),
            Self::BooleanLiteral(b) => format!("BooleanLiteral({})", b.value).into(),
            Self::NullLiteral(_) => "NullLiteral".into(),
            Self::BigIntLiteral(b) => format!("BigIntLiteral({})", b.raw).into(),
            Self::RegExpLiteral(r) => format!("RegExpLiteral({})", r.regex).into(),
            Self::TemplateLiteral(t) => format!(
                "TemplateLiteral({})",
                t.quasi().map_or_else(|| "None".into(), |q| format!("Some({q})"))
            )
            .into(),

            Self::MetaProperty(_) => "MetaProperty".into(),
            Self::Super(_) => "Super".into(),

            Self::ArrayExpression(_) => "ArrayExpression".into(),
            Self::ArrowFunctionExpression(_) => "ArrowFunctionExpression".into(),
            Self::AssignmentExpression(_) => "AssignmentExpression".into(),
            Self::AwaitExpression(_) => "AwaitExpression".into(),
            Self::BinaryExpression(b) => {
                format!("BinaryExpression({})", b.operator.as_str()).into()
            }
            Self::CallExpression(c) => {
                format!("CallExpression({})", c.callee_name().unwrap_or(&COMPUTED)).into()
            }
            Self::ChainExpression(_) => "ChainExpression".into(),
            Self::ConditionalExpression(_) => "ConditionalExpression".into(),
            Self::LogicalExpression(_) => "LogicalExpression".into(),
            Self::NewExpression(n) => {
                let callee = match &n.callee {
                    Expression::Identifier(id) => Some(id.name.as_str()),
                    match_member_expression!(Expression) => {
                        n.callee.to_member_expression().static_property_name()
                    }
                    _ => None,
                };
                format!("NewExpression({})", callee.unwrap_or(&COMPUTED)).into()
            }
            Self::ObjectExpression(_) => "ObjectExpression".into(),
            Self::ParenthesizedExpression(_) => "ParenthesizedExpression".into(),
            Self::SequenceExpression(_) => "SequenceExpression".into(),
            Self::TaggedTemplateExpression(_) => "TaggedTemplateExpression".into(),
            Self::ThisExpression(_) => "ThisExpression".into(),
            Self::UnaryExpression(expr) => format!("UnaryExpression({:?})", expr.operator).into(),
            Self::UpdateExpression(_) => "UpdateExpression".into(),
            Self::YieldExpression(_) => "YieldExpression".into(),
            Self::ImportExpression(_) => "ImportExpression".into(),
            Self::PrivateInExpression(_) => "PrivateInExpression".into(),

            Self::ObjectProperty(p) => {
                format!("ObjectProperty({})", p.key.name().unwrap_or(COMPUTED)).into()
            }
            Self::ElementAccessExpression(_) => "ElementAccessExpression".into(),
            Self::PropertyAccessExpression(_) => "PropertyAccessExpression".into(),
            Self::PrivateFieldExpression(_) => "PrivateFieldExpression".into(),
            Self::ArrayAssignmentTarget(_) => "ArrayAssignmentTarget".into(),
            Self::ObjectAssignmentTarget(_) => "ObjectAssignmentTarget".into(),
            Self::AssignmentTargetWithDefault(_) => "AssignmentTargetWithDefault".into(),
            Self::SpreadElement(_) => "SpreadElement".into(),
            Self::Elision(_) => "Elision".into(),
            Self::BindingRestElement(_) => "BindingRestElement".into(),

            Self::Function(x) => format!("Function({})", or_anonymous(x.id.as_ref())).into(),
            Self::FunctionBody(_) => "FunctionBody".into(),
            Self::FormalParameters(_) => "FormalParameters".into(),
            Self::FormalParameter(p) => format!(
                "FormalParameter({})",
                p.pattern.get_identifier().unwrap_or(Atom::from(DESTRUCTURE.as_ref()))
            )
            .into(),
            Self::CatchParameter(_) => "CatchParameter".into(),

            Self::Class(c) => format!("Class({})", or_anonymous(c.id.as_ref())).into(),
            Self::TSClassImplements(_) => "TSClassImplements".into(),
            Self::ClassBody(_) => "ClassBody".into(),
            Self::StaticBlock(_) => "StaticBlock".into(),
            Self::PropertyDefinition(_) => "PropertyDefinition".into(),
            Self::MethodDefinition(_) => "MethodDefinition".into(),

            Self::ArrayPattern(_) => "ArrayPattern".into(),
            Self::ObjectPattern(_) => "ObjectPattern".into(),
            Self::AssignmentPattern(_) => "AssignmentPattern".into(),

            Self::Decorator(_) => "Decorator".into(),

            Self::ImportDeclaration(_) => "ImportDeclaration".into(),
            Self::ImportSpecifier(i) => format!("ImportSpecifier({})", i.local.name).into(),
            Self::ExportSpecifier(e) => format!("ExportSpecifier({})", e.local.name()).into(),
            Self::ImportDefaultSpecifier(_) => "ImportDefaultSpecifier".into(),
            Self::ImportNamespaceSpecifier(_) => "ImportNamespaceSpecifier".into(),
            Self::ExportDefaultDeclaration(_) => "ExportDefaultDeclaration".into(),
            Self::ExportNamedDeclaration(_) => "ExportNamedDeclaration".into(),
            Self::ExportAllDeclaration(_) => "ExportAllDeclaration".into(),
            Self::JSXOpeningElement(_) => "JSXOpeningElement".into(),
            Self::JSXClosingElement(_) => "JSXClosingElement".into(),
            Self::JSXElement(_) => "JSXElement".into(),
            Self::JSXOpeningFragment(_) => "JSXOpeningFragment".into(),
            Self::JSXClosingFragment(_) => "JSXClosingFragment".into(),
            Self::JSXFragment(_) => "JSXFragment".into(),
            Self::JSXSpreadAttribute(_) => "JSXSpreadAttribute".into(),
            Self::JSXText(_) => "JSXText".into(),
            Self::JSXExpressionContainer(_) => "JSXExpressionContainer".into(),
            Self::JSXEmptyExpression(_) => "JSXEmptyExpression".into(),
            Self::JSXIdentifier(id) => format!("JSXIdentifier({id})").into(),
            Self::JSXMemberExpression(_) => "JSXMemberExpression".into(),
            Self::JSXNamespacedName(_) => "JSXNamespacedName".into(),

            Self::TSModuleBlock(_) => "TSModuleBlock".into(),

            Self::TSAnyKeyword(_) => "TSAnyKeyword".into(),
            Self::TSIntersectionType(_) => "TSIntersectionType".into(),
            Self::TSLiteralType(_) => "TSLiteralType".into(),
            Self::TSMethodSignature(_) => "TSMethodSignature".into(),
            Self::TSNullKeyword(_) => "TSNullKeyword".into(),
            Self::TSTypeLiteral(_) => "TSTypeLiteral".into(),
            Self::TSTypeOperator(_) => "TSTypeOperator".into(),
            Self::TSTypeReference(t) => format!("TSTypeReference({})", t.type_name).into(),
            Self::TSUnionType(_) => "TSUnionType".into(),
            Self::TSParenthesizedType(_) => "TSParenthesizedType".into(),
            Self::TSVoidKeyword(_) => "TSVoidKeyword".into(),
            Self::TSArrayType(_) => "TSArrayType".into(),
            Self::TSConstructorType(_) => "TSConstructorType".into(),
            Self::TSFunctionType(_) => "TSFunctionType".into(),
            Self::TSBigIntKeyword(_) => "TSBigIntKeyword".into(),
            Self::TSBooleanKeyword(_) => "TSBooleanKeyword".into(),
            Self::TSIntrinsicKeyword(_) => "TSIntrinsicKeyword".into(),
            Self::TSNeverKeyword(_) => "TSNeverKeyword".into(),
            Self::TSNumberKeyword(_) => "TSNumberKeyword".into(),
            Self::TSObjectKeyword(_) => "TSObjectKeyword".into(),
            Self::TSStringKeyword(_) => "TSStringKeyword".into(),
            Self::TSSymbolKeyword(_) => "TSSymbolKeyword".into(),
            Self::TSThisType(_) => "TSThisType".into(),
            Self::TSTupleType(_) => "TSTupleType".into(),
            Self::TSOptionalType(_) => "TSOptionalType".into(),
            Self::TSRestType(_) => "TSRestType".into(),
            Self::TSUndefinedKeyword(_) => "TSUndefinedKeyword".into(),
            Self::TSUnknownKeyword(_) => "TSUnknownKeyword".into(),
            Self::TSInferType(_) => "TSInferType".into(),
            Self::TSTemplateLiteralType(_) => "TSTemplateLiteralType".into(),

            Self::TSIndexedAccessType(_) => "TSIndexedAccessType".into(),

            Self::TSAsExpression(_) => "TSAsExpression".into(),
            Self::TSSatisfiesExpression(_) => "TSSatisfiesExpression".into(),
            Self::TSNonNullExpression(_) => "TSNonNullExpression".into(),
            Self::TSNamespaceExportDeclaration(_) => "TSNamespaceExportDeclaration".into(),
            Self::TSInstantiationExpression(_) => "TSInstantiationExpression".into(),

            Self::TSEnumDeclaration(decl) => format!("TSEnumDeclaration({})", &decl.id.name).into(),

            Self::TSEnumMember(_) => "TSEnumMember".into(),

            Self::TSImportEqualsDeclaration(_) => "TSImportEqualsDeclaration".into(),
            Self::TSExternalModuleReference(_) => "TSExternalModuleReference".into(),
            Self::TSQualifiedName(n) => format!("TSQualifiedName({n})").into(),
            Self::TSInterfaceDeclaration(_) => "TSInterfaceDeclaration".into(),
            Self::TSInterfaceHeritage(_) => "TSInterfaceHeritage".into(),
            Self::TSModuleDeclaration(m) => format!("TSModuleDeclaration({})", m.id).into(),
            Self::TSTypeAliasDeclaration(_) => "TSTypeAliasDeclaration".into(),
            Self::TSTypeAnnotation(_) => "TSTypeAnnotation".into(),
            Self::TSTypePredicate(_) => "TSTypePredicate".into(),
            Self::TSTypeQuery(_) => "TSTypeQuery".into(),
            Self::TSTypeAssertion(_) => "TSTypeAssertion".into(),
            Self::TSThisParameter(_) => "TSThisParameter".into(),
            Self::TSTypeParameter(t) => format!("TSTypeParameter({})", t.name).into(),
            Self::TSTypeParameterDeclaration(_) => "TSTypeParameterDeclaration".into(),
            Self::TSTypeParameterInstantiation(_) => "TSTypeParameterInstantiation".into(),
            Self::TSImportType(_) => "TSImportType".into(),
            Self::TSNamedTupleMember(_) => "TSNamedTupleMember".into(),

            Self::TSPropertySignature(_) => "TSPropertySignature".into(),
            Self::TSConditionalType(_) => "TSConditionalType".into(),
            Self::TSMappedType(_) => "TSMappedType".into(),
            Self::TSConstructSignatureDeclaration(_) => "TSConstructSignatureDeclaration".into(),
            Self::TSExportAssignment(_) => "TSExportAssignment".into(),
            Self::JSDocNullableType(_) => "JSDocNullableType".into(),
            Self::JSDocNonNullableType(_) => "JSDocNonNullableType".into(),
            Self::JSDocUnknownType(_) => "JSDocUnknownType".into(),

            Self::TemplateElement(_) => "TemplateElement".into(),
            Self::AssignmentTargetRest(_) => "AssignmentTargetRest".into(),
            Self::AssignmentTargetPropertyIdentifier(_) => {
                "AssignmentTargetPropertyIdentifier".into()
            }
            Self::AssignmentTargetPropertyProperty(_) => "AssignmentTargetPropertyProperty".into(),
            Self::AccessorProperty(_) => "AccessorProperty".into(),
            Self::WithClause(_) => "WithClause".into(),
            Self::ImportAttribute(_) => "ImportAttribute".into(),
            Self::JSXAttribute(_) => "JSXAttribute".into(),
            Self::JSXSpreadChild(_) => "JSXSpreadChild".into(),
            Self::BindingProperty(_) => "BindingProperty".into(),
            Self::TSInterfaceBody(_) => "TSInterfaceBody".into(),
            Self::TSIndexSignature(_) => "TSIndexSignature".into(),
            Self::TSCallSignatureDeclaration(_) => "TSCallSignatureDeclaration".into(),
            Self::TSIndexSignatureName(_) => "TSIndexSignatureName".into(),
            Self::TSImportAttributes(_) => "TSImportAttributes".into(),
            Self::TSImportAttribute(_) => "TSImportAttribute".into(),
        }
    }
}
