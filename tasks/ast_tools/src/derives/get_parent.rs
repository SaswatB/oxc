use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Ident};

use crate::{
    schema::{EnumDef, GetGenerics, Schema, StructDef, ToType, TypeDef},
    util::{ToIdent, TypeWrapper},
};

use super::{define_derive, Derive};

pub const BLACK_LIST: [&str; 1] = ["Span"];

fn handle_special_case(field_name: &str, field_expr: TokenStream) -> Option<TokenStream> {
    match field_name {
        "Expression" => Some(quote! { AstKind::from_expression(#field_expr) }),
        "Statement" => Some(quote! { AstKind::from_statement(#field_expr) }),
        "Declaration" => Some(quote! { AstKind::from_declaration(#field_expr) }),
        "BindingPatternKind" => Some(quote! { AstKind::from_binding_pattern_kind(#field_expr) }),
        "ChainElement" => Some(quote! { AstKind::from_chain_element(#field_expr) }),
        "ModuleExportName" => Some(quote! { AstKind::from_module_export_name(#field_expr) }),
        "AssignmentTargetMaybeDefault" => {
            Some(quote! { AstKind::from_assignment_target_maybe_default(#field_expr) })
        }
        "AssignmentTargetProperty" => {
            Some(quote! { AstKind::from_assignment_target_property(#field_expr) })
        }
        "ForStatementLeft" => Some(quote! { AstKind::from_for_statement_left(#field_expr) }),
        "ImportAttributeKey" => Some(quote! { AstKind::from_import_attribute_key(#field_expr) }),
        "ExportDefaultDeclarationKind" => {
            Some(quote! { AstKind::from_export_default_declaration_kind(#field_expr) })
        }
        "ImportDeclarationSpecifier" => {
            Some(quote! { AstKind::from_import_declaration_specifier(#field_expr) })
        }
        "ObjectPropertyKind" => Some(quote! { AstKind::from_object_property_kind(#field_expr) }),
        "ClassElement" => Some(quote! { AstKind::from_class_element(#field_expr) }),
        "JSXAttributeName" => Some(quote! { AstKind::from_jsx_attribute_name(#field_expr) }),
        "JSXAttributeValue" => Some(quote! { AstKind::from_jsx_attribute_value(#field_expr) }),
        "JSXExpression" => Some(quote! { AstKind::from_jsx_expression(#field_expr) }),
        "JSXChild" => Some(quote! { AstKind::from_jsx_child(#field_expr) }),
        "TSType" => Some(quote! { AstKind::from_ts_type(#field_expr) }),
        "TSImportAttributeName" => {
            Some(quote! { AstKind::from_ts_import_attribute_name(#field_expr) })
        }
        "TSTypeQueryExprName" => {
            Some(quote! { AstKind::from_ts_type_query_expr_name(#field_expr) })
        }
        "TSModuleDeclarationName" => {
            Some(quote! { AstKind::from_ts_module_declaration_name(#field_expr) })
        }
        "TSTypePredicateName" => Some(quote! { AstKind::from_ts_type_predicate_name(#field_expr) }),
        "TSTupleElement" => Some(quote! { AstKind::from_ts_tuple_element(#field_expr) }),
        "TSLiteral" => Some(quote! { AstKind::from_ts_literal(#field_expr) }),
        "TSEnumMemberName" => Some(quote! { AstKind::from_ts_enum_member_name(#field_expr) }),
        "TSSignature" => Some(quote! { AstKind::from_ts_signature(#field_expr) }),
        "TSModuleDeclarationBody" => {
            Some(quote! { AstKind::from_ts_module_declaration_body(#field_expr) })
        }
        _ => None,
    }
}

pub struct DeriveGetParent;

define_derive!(DeriveGetParent);

impl Derive for DeriveGetParent {
    fn trait_name() -> &'static str {
        "GetParent"
    }

    fn prelude() -> TokenStream {
        quote! {
            #![allow(clippy::match_same_arms)]
            #![allow(unused_variables)]

            ///@@line_break
            use crate::GetParent;
            use crate::AstKind;

            #[allow(dead_code)]
            pub trait GetChildren<'a> {
                fn get_children(&'a self) -> Vec<AstKind<'a>>;
            }
        }
    }

    fn derive(&mut self, def: &TypeDef, schema: &Schema) -> TokenStream {
        let parent_impl = {
            let self_type = quote!(&self);
            let mut_self_type = quote!(&mut self);
            let result_type = quote!(Option<AstKind<'a>>);
            let result_expr = quote!(self.parent);
            let set_new_type = quote!(AstKind<'a>);
            let set_expr = quote!(self.parent = Some(new_parent));
            let unbox = |it| quote!(#it.as_ref());
            let unbox_mut = |it| quote!(#it.as_mut());

            derive(
                Self::trait_name(),
                "get_parent",
                "set_parent",
                &self_type,
                &mut_self_type,
                &result_type,
                &result_expr,
                &set_new_type,
                &set_expr,
                def,
                unbox,
                unbox_mut,
            )
        };

        let children_impl = match def {
            TypeDef::Enum(def) => generate_enum_get_children_fn(def, schema),
            TypeDef::Struct(def) => generate_struct_get_children_fn(def, schema),
        };

        quote! {
            #parent_impl

            #children_impl
        }
    }
}

#[expect(clippy::too_many_arguments)]
fn derive<U, U2>(
    trait_name: &str,
    get_method_name: &str,
    set_method_name: &str,
    self_type: &TokenStream,
    mut_self_type: &TokenStream,
    result_type: &TokenStream,
    result_expr: &TokenStream,
    set_new_type: &TokenStream,
    set_expr: &TokenStream,
    def: &TypeDef,
    unbox: U,
    unbox_mut: U2,
) -> TokenStream
where
    U: Fn(TokenStream) -> TokenStream,
    U2: Fn(TokenStream) -> TokenStream,
{
    let trait_ident = trait_name.to_ident();
    let get_method_ident = get_method_name.to_ident();
    let set_method_ident = set_method_name.to_ident();
    match &def {
        TypeDef::Enum(def) => derive_enum(
            def,
            &trait_ident,
            &get_method_ident,
            &set_method_ident,
            self_type,
            mut_self_type,
            result_type,
            set_new_type,
            unbox,
            unbox_mut,
        ),
        TypeDef::Struct(def) => derive_struct(
            def,
            &trait_ident,
            &get_method_ident,
            &set_method_ident,
            self_type,
            mut_self_type,
            result_type,
            result_expr,
            set_new_type,
            set_expr,
        ),
    }
}

fn derive_enum<U, U2>(
    def: &EnumDef,
    trait_name: &Ident,
    get_method_name: &Ident,
    set_method_name: &Ident,
    self_type: &TokenStream,
    mut_self_type: &TokenStream,
    result_type: &TokenStream,
    set_new_type: &TokenStream,
    unbox: U,
    unbox_mut: U2,
) -> TokenStream
where
    U: Fn(TokenStream) -> TokenStream,
    U2: Fn(TokenStream) -> TokenStream,
{
    let target_type = if def.has_lifetime() {
        def.to_type_with_explicit_generics(parse_quote! {<'a>})
    } else {
        def.to_type_elide()
    };

    let get_matches = def.all_variants().map(|var| {
        let ident = var.ident();
        let mut it = quote!(it);
        if var.fields.first().is_some_and(|it| it.typ.analysis().wrapper == TypeWrapper::Box) {
            it = unbox(it);
        }
        quote!(Self :: #ident(it) => #trait_name :: #get_method_name(#it))
    });

    let set_matches = def.all_variants().map(|var| {
        let ident = var.ident();
        let mut it = quote!(it);
        if var.fields.first().is_some_and(|it| it.typ.analysis().wrapper == TypeWrapper::Box) {
            it = unbox_mut(it);
        }
        quote!(Self :: #ident(it) => #trait_name :: #set_method_name(#it, new_parent))
    });

    quote! {
        impl<'a> #trait_name<'a> for #target_type {
            fn #get_method_name(#self_type) -> #result_type {
                match self {
                    #(#get_matches),*
                }
            }

            fn #set_method_name(#mut_self_type, new_parent: #set_new_type) {
                match self {
                    #(#set_matches),*
                }
            }
        }
    }
}

fn derive_struct(
    def: &StructDef,
    trait_name: &Ident,
    get_method_name: &Ident,
    set_method_name: &Ident,
    self_type: &TokenStream,
    mut_self_type: &TokenStream,
    result_type: &TokenStream,
    result_expr: &TokenStream,
    set_new_type: &TokenStream,
    set_expr: &TokenStream,
) -> TokenStream {
    let target_type = if def.has_lifetime() {
        def.to_type_with_explicit_generics(parse_quote! {<'a>})
    } else {
        def.to_type_elide()
    };

    quote! {
        impl<'a> #trait_name<'a> for #target_type {
            #[inline]
            fn #get_method_name(#self_type) -> #result_type {
                #result_expr
            }

            #[inline]
            fn #set_method_name(#mut_self_type, new_parent: #set_new_type) {
                #set_expr
            }
        }
    }
}

fn generate_enum_get_children_fn(def: &EnumDef, schema: &Schema) -> TokenStream {
    let target_type = if def.has_lifetime() {
        def.to_type_with_explicit_generics(parse_quote! {<'a>})
    } else {
        def.to_type_elide()
    };

    let matches = def.all_variants().map(|variant| {
        let var_ident = variant.ident();
        let field = variant.fields.first().expect("enum variant should have exactly one field");
        let field_type = &field.typ;

        if let Some(type_id) = field_type.type_id() {
            if let Some(field_def) = schema.get(type_id) {
                if field_def.is_visitable() {
                    let field_name = field_def.name();
                    if !BLACK_LIST.iter().any(|&x| x == field_name) {
                        let child_expr = quote!(child);
                        return if let Some(special_case) =
                            handle_special_case(&field_name, child_expr)
                        {
                            quote! {
                                Self::#var_ident(child) => vec![#special_case],
                            }
                        } else {
                            let field_type_ident = format_ident!("{field_name}");
                            quote! {
                                Self::#var_ident(child) => vec![AstKind::#field_type_ident(child)],
                            }
                        };
                    }
                }
            }
        }
        quote! {
            Self::#var_ident(_) => vec![],
        }
    });

    quote! {
        impl<'a> GetChildren<'a> for #target_type {
            #[allow(unused_variables, clippy::match_same_arms)]
            fn get_children(&'a self) -> Vec<AstKind<'a>> {
                match self {
                    #(#matches)*
                }
            }
        }
    }
}

fn generate_struct_get_children_fn(def: &StructDef, schema: &Schema) -> TokenStream {
    let target_type = if def.has_lifetime() {
        def.to_type_with_explicit_generics(parse_quote! {<'a>})
    } else {
        def.to_type_elide()
    };

    let mut child_fields = Vec::new();

    for field in &def.fields {
        if let Some(type_id) = field.typ.transparent_type_id() {
            if let Some(field_def) = schema.get(type_id) {
                if field_def.is_visitable() {
                    let field_name = field_def.name();
                    if !BLACK_LIST.iter().any(|&x| x == field_name) {
                        let field_ident =
                            field.ident().expect("struct field should have an identifier");
                        let analysis = field.typ.analysis();
                        child_fields.push((field_ident, field_def.name(), analysis.wrapper));
                    }
                }
            }
        }
    }

    if child_fields.is_empty() {
        quote! {
            impl<'a> GetChildren<'a> for #target_type {
                fn get_children(&'a self) -> Vec<AstKind<'a>> {
                    vec![]
                }
            }
        }
    } else {
        let field_conversions =
            child_fields.iter().map(|(field, field_type, wrapper)| match wrapper {
                TypeWrapper::Vec | TypeWrapper::VecBox => {
                    let field_expr = quote!(&self.#field);
                    if let Some(special_case) = handle_special_case(field_type, quote!(item)) {
                        quote! {
                            for item in #field_expr {
                                children.push(#special_case);
                            }
                        }
                    } else {
                        let field_type_ident = format_ident!("{field_type}");
                        quote! {
                            for item in #field_expr {
                                children.push(AstKind::#field_type_ident(item));
                            }
                        }
                    }
                }
                TypeWrapper::OptVec => {
                    let field_expr = quote!(&self.#field);
                    if let Some(special_case) = handle_special_case(field_type, quote!(item)) {
                        quote! {
                            if let Some(vec) = #field_expr {
                                for item in vec {
                                    children.push(#special_case);
                                }
                            }
                        }
                    } else {
                        let field_type_ident = format_ident!("{field_type}");
                        quote! {
                            if let Some(vec) = #field_expr {
                                for item in vec {
                                    children.push(AstKind::#field_type_ident(item));
                                }
                            }
                        }
                    }
                }
                TypeWrapper::VecOpt => {
                    let field_expr = quote!(&self.#field);
                    if let Some(special_case) = handle_special_case(field_type, quote!(item)) {
                        quote! {
                            for opt_item in #field_expr {
                                if let Some(item) = opt_item {
                                    children.push(#special_case);
                                }
                            }
                        }
                    } else {
                        let field_type_ident = format_ident!("{field_type}");
                        quote! {
                            for opt_item in #field_expr {
                                if let Some(item) = opt_item {
                                    children.push(AstKind::#field_type_ident(item));
                                }
                            }
                        }
                    }
                }
                TypeWrapper::Opt | TypeWrapper::OptBox => {
                    let field_expr = quote!(&self.#field);
                    if let Some(special_case) = handle_special_case(field_type, quote!(field)) {
                        quote! {
                            if let Some(field) = #field_expr {
                                children.push(#special_case);
                            }
                        }
                    } else {
                        let field_type_ident = format_ident!("{field_type}");
                        quote! {
                            if let Some(field) = #field_expr {
                                children.push(AstKind::#field_type_ident(field));
                            }
                        }
                    }
                }
                TypeWrapper::Box => {
                    let field_expr = quote!(&*self.#field);
                    if let Some(special_case) = handle_special_case(field_type, field_expr.clone())
                    {
                        quote! {
                            children.push(#special_case);
                        }
                    } else {
                        let field_type_ident = format_ident!("{field_type}");
                        quote! {
                            children.push(AstKind::#field_type_ident(#field_expr));
                        }
                    }
                }
                _ => {
                    let field_expr = quote!(&self.#field);
                    if let Some(special_case) = handle_special_case(field_type, field_expr.clone())
                    {
                        quote! {
                            children.push(#special_case);
                        }
                    } else {
                        let field_type_ident = format_ident!("{field_type}");
                        quote! {
                            children.push(AstKind::#field_type_ident(#field_expr));
                        }
                    }
                }
            });

        quote! {
            impl<'a> GetChildren<'a> for #target_type {
                fn get_children(&'a self) -> Vec<AstKind<'a>> {
                    let mut children = Vec::new();
                    #(#field_conversions)*
                    children
                }
            }
        }
    }
}
