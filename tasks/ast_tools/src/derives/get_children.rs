use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse_quote;

use crate::{
    schema::{EnumDef, GetGenerics, GetIdent, Schema, StructDef, ToType, TypeDef},
    util::TypeWrapper,
};

use super::{define_derive, Derive};

pub const BLACK_LIST: [&str; 1] = ["Span"];

pub struct DeriveGetChildren;

define_derive!(DeriveGetChildren);

impl Derive for DeriveGetChildren {
    fn trait_name() -> &'static str {
        "GetChildren"
    }

    fn prelude() -> TokenStream {
        quote! {
            #![allow(clippy::match_same_arms)]
            #![allow(unused_variables)]

            ///@@line_break
            pub use crate::GetChildren;
            use crate::AstKind;
        }
    }

    fn derive(&mut self, def: &TypeDef, schema: &Schema) -> TokenStream {
        let children_impl = match def {
            TypeDef::Enum(def) => generate_enum_get_children_fn(def, schema),
            TypeDef::Struct(def) => generate_struct_get_children_fn(def, schema),
        };

        quote! {
            #children_impl
        }
    }
}

fn generate_ast_kind_conversion(
    field_type: &str,
    field_expr: TokenStream,
    field_def: Option<&TypeDef>,
) -> TokenStream {
    if let Some(field_def) = field_def {
        if matches!(field_def, TypeDef::Enum(_)) {
            quote! { (*#field_expr).to_ast_kind() }
        } else {
            let field_type_ident = format_ident!("{field_type}");
            quote! { AstKind::#field_type_ident(#field_expr) }
        }
    } else {
        let field_type_ident = format_ident!("{field_type}");
        quote! { AstKind::#field_type_ident(#field_expr) }
    }
}

fn generate_wrapper_conversion(
    wrapper: &TypeWrapper,
    field_expr: TokenStream,
    field_type: &str,
    field_def: Option<&TypeDef>,
) -> TokenStream {
    match wrapper {
        TypeWrapper::Vec | TypeWrapper::VecBox => {
            let conversion = generate_ast_kind_conversion(field_type, quote!(item), field_def);
            quote! {
                for item in #field_expr {
                    children.push(#conversion);
                }
            }
        }
        TypeWrapper::OptVec => {
            let conversion = generate_ast_kind_conversion(field_type, quote!(item), field_def);
            quote! {
                if let Some(vec) = #field_expr {
                    for item in vec {
                        children.push(#conversion);
                    }
                }
            }
        }
        TypeWrapper::VecOpt => {
            let conversion = generate_ast_kind_conversion(field_type, quote!(item), field_def);
            quote! {
                for opt_item in #field_expr {
                    if let Some(item) = opt_item {
                        children.push(#conversion);
                    }
                }
            }
        }
        TypeWrapper::Opt | TypeWrapper::OptBox => {
            let conversion = generate_ast_kind_conversion(field_type, quote!(field), field_def);
            quote! {
                if let Some(field) = #field_expr {
                    children.push(#conversion);
                }
            }
        }
        TypeWrapper::Box => {
            let conversion =
                generate_ast_kind_conversion(field_type, field_expr.clone(), field_def);
            quote! {
                children.push(#conversion);
            }
        }
        _ => {
            let conversion =
                generate_ast_kind_conversion(field_type, field_expr.clone(), field_def);
            quote! {
                children.push(#conversion);
            }
        }
    }
}

fn generate_enum_get_children_fn(def: &EnumDef, _schema: &Schema) -> TokenStream {
    let target_type = if def.has_lifetime() {
        def.to_type_with_explicit_generics(parse_quote! {<'a>})
    } else {
        def.to_type_elide()
    };

    let matches = def.all_variants().map(|variant| {
        let var_ident = variant.ident();
        quote! {
            Self::#var_ident(n) => n.get_children(),
        }
    });

    let matches2 = def.all_variants().map(|variant| {
        let var_ident = variant.ident();
        let field = variant.fields.first().expect("enum variant should have exactly one field");
        let field_type = &field.typ;
        let field_type_name = field_type
            .name()
            .to_string()
            .trim_start_matches("Box<")
            .trim_end_matches('>')
            .to_string();
        let field_type_ident = format_ident!("{field_type_name}");
        quote! {
            Self::#var_ident(e) => AstKind::#field_type_ident(e),
        }
    });

    let matches3 = def.all_variants().map(|variant| {
        let var_ident = variant.ident();
        quote! {
            Self::#var_ident(e) => e.node_id,
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

            fn to_ast_kind(&'a self) -> AstKind<'a> {
                match self {
                    #(#matches2)*
                }
            }

            fn get_node_id(&'a self) -> u32 {
                match self {
                    #(#matches3)*
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
                        child_fields.push((field_ident, field_name, analysis.wrapper, type_id));
                    }
                }
            }
        }
    }

    let type_ident = format_ident!("{}", def.ident());

    if child_fields.is_empty() {
        quote! {
            impl<'a> GetChildren<'a> for #target_type {
                fn get_children(&'a self) -> Vec<AstKind<'a>> {
                    vec![]
                }
                fn to_ast_kind(&'a self) -> AstKind<'a> {
                    AstKind::#type_ident(self)
                }
                fn get_node_id(&'a self) -> u32 {
                    self.node_id
                }
            }
        }
    } else {
        let field_conversions = child_fields.iter().map(|(field, field_type, wrapper, type_id)| {
            let field_expr = match wrapper {
                TypeWrapper::Box => quote!(&*self.#field),
                _ => quote!(&self.#field),
            };
            generate_wrapper_conversion(wrapper, field_expr, field_type, schema.get(*type_id))
        });

        quote! {
            impl<'a> GetChildren<'a> for #target_type {
                fn get_children(&'a self) -> Vec<AstKind<'a>> {
                    let mut children = Vec::new();
                    #(#field_conversions)*
                    children
                }
                fn to_ast_kind(&'a self) -> AstKind<'a> {
                    AstKind::#type_ident(self)
                }
                fn get_node_id(&'a self) -> u32 {
                    self.node_id
                }
            }
        }
    }
}
