use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_quote, Arm, ImplItemFn, LitInt};

use crate::{
    output::{output_path, Output},
    schema::{GetIdent, Schema, ToType, TypeDef},
    Generator,
};

use super::define_generator;

pub struct AstKindGenerator;

define_generator!(AstKindGenerator);

pub const BLACK_LIST: [&str; 1] = ["Span"];

impl Generator for AstKindGenerator {
    fn generate(&mut self, schema: &Schema) -> Output {
        let have_kinds = schema
            .defs
            .iter()
            .filter(|def| {
                let is_enum = matches!(def, TypeDef::Enum(_));
                let is_visitable = def.is_visitable();
                let is_blacklisted = BLACK_LIST.contains(&def.name());
                !is_enum && is_visitable && !is_blacklisted
            })
            .map(|def| {
                let ident = def.ident();
                let typ = def.to_type();
                (ident, typ)
            })
            .collect_vec();

        let (types, kinds): (Vec<_>, Vec<_>) = have_kinds
            .iter()
            .enumerate()
            .map(|(index, (ident, typ))| {
                let index = u8::try_from(index).unwrap();
                let index = LitInt::new(&index.to_string(), Span::call_site());
                let type_variant = quote!( #ident = #index );
                let kind_variant = quote!( #ident(&'a #typ) = AstType::#ident as u8 );
                (type_variant, kind_variant)
            })
            .unzip();

        let span_matches: Vec<Arm> = have_kinds
            .iter()
            .map(|(ident, _)| parse_quote!(Self :: #ident(it) => it.span()))
            .collect_vec();

        let get_children_matches: Vec<Arm> = have_kinds
            .iter()
            .map(|(ident, _)| parse_quote!(Self :: #ident(it) => it.get_children()))
            .collect_vec();

        let get_node_id_matches: Vec<Arm> = have_kinds
            .iter()
            .map(|(ident, _)| parse_quote!(Self :: #ident(it) => it.get_node_id()))
            .collect_vec();

        let as_ast_kind_impls: Vec<ImplItemFn> = have_kinds
            .iter()
            .map(|(ident, typ)| {
                let snake_case_name =
                    format_ident!("as_{}", ident.to_string().to_case(Case::Snake));
                parse_quote!(
                    ///@@line_break
                    #[inline]
                    pub fn #snake_case_name(self) -> Option<&'a #typ> {
                        if let Self::#ident(v) = self {
                            Some(v)
                        } else {
                            None
                        }
                    }
                )
            })
            .collect_vec();

        Output::Rust {
            path: output_path(crate::AST_CRATE, "ast_kind.rs"),
            tokens: quote! {
                #![allow(missing_docs)] ///@ FIXME (in ast_tools/src/generators/ast_kind.rs)

                ///@@line_break
                use std::ptr;

                ///@@line_break
                use oxc_span::{GetSpan, Span};

                ///@@line_break
                use crate::ast::*;
                use super::derive_get_children::GetChildren;

                ///@@line_break
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                #[repr(u8)]
                pub enum AstType {
                    #(#types),*,
                }

                ///@@line_break
                /// Untyped AST Node Kind
                #[derive(Debug, Clone, Copy)]
                #[repr(C, u8)]
                pub enum AstKind<'a> {
                    #(#kinds),*,
                }

                ///@@line_break
                impl AstKind<'_> {
                    /// Get the [`AstType`] of an [`AstKind`].
                    #[inline]
                    pub fn ty(&self) -> AstType {
                        ///@ SAFETY: `AstKind` is `#[repr(C, u8)]`, so discriminant is stored in first byte,
                        ///@ and it's valid to read it.
                        ///@ `AstType` is also `#[repr(u8)]` and `AstKind` and `AstType` both have the same
                        ///@ discriminants, so it's valid to read `AstKind`'s discriminant as `AstType`.
                        unsafe { *ptr::from_ref(self).cast::<AstType>().as_ref().unwrap_unchecked() }
                    }
                }

                ///@@line_break
                impl GetSpan for AstKind<'_> {
                    #[allow(clippy::match_same_arms)]
                    fn span(&self) -> Span {
                        match self {
                            #(#span_matches),*,
                        }
                    }
                }

                ///@@line_break
                impl<'a> AstKind<'a> {
                    pub fn get_children(&self) -> Vec<AstKind<'a>> {
                        match self {
                            #(#get_children_matches),*,
                        }
                    }

                    pub fn get_node_id(&self) -> u32 {
                        match self {
                            #(#get_node_id_matches),*,
                        }
                    }
                }

                ///@@line_break
                impl<'a> AstKind<'a> {
                    #(#as_ast_kind_impls)*
                }
            },
        }
    }
}
