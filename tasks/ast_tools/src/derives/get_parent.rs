use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Ident};

use crate::{
    schema::{EnumDef, GetGenerics, Schema, StructDef, ToType, TypeDef},
    util::{ToIdent, TypeWrapper},
};

use super::{define_derive, Derive};

pub struct DeriveGetParent;

define_derive!(DeriveGetParent);

impl Derive for DeriveGetParent {
    fn trait_name() -> &'static str {
        "GetParent"
    }

    fn prelude() -> TokenStream {
        quote! {
            #![allow(clippy::match_same_arms)]

            ///@@line_break
            use crate::GetParent;
            use crate::AstKind;
        }
    }

    fn derive(&mut self, def: &TypeDef, _: &Schema) -> TokenStream {
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
