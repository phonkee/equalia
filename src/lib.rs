#![allow(dead_code)]

use darling::*;
use proc_macro2::{Ident, TokenStream as SynTokenStream};
use proc_macro::{TokenStream, Span};
use syn::{DeriveInput, parse_macro_input, Data, Type};
use quote::*;
use quote::ToTokens;

#[cfg(feature = "nightly")]
fn error(span: Span, data: &str) -> SynTokenStream {
    span.unstable().error(data).emit();
    SynTokenStream::new()
}

#[cfg(not(feature = "nightly"))]
fn error(_: Span, data: &str) -> SynTokenStream {
    quote! { compile_error!(#data); }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(equalia), supports(struct_named))]
struct Equalia {
    ident: Ident,
    data: ast::Data<util::Ignored, EqualiaField>,
    #[darling(default)]
    hash: bool,
}

/// Equalia implementation
impl Equalia {
    // return if we have only fields, so we can ignore all other fields
    pub fn has_only_field(&self) -> bool {
        self.data.as_ref().take_struct().unwrap().fields.iter().any(|f| f.only)
    }
}

/// write tokens to stream
impl ToTokens for Equalia {
    fn to_tokens(&self, tokens: &mut SynTokenStream) {
        let i = &self.ident;
        let has_only_field = self.has_only_field();
        let mut eq_stream = SynTokenStream::new();

        for field in self.data.as_ref().take_struct().unwrap().fields {
            if has_only_field {
                if field.only {
                    field.write_eq(&mut eq_stream);
                }
            } else {
                // no need to use field
                if field.skip {
                    continue;
                }
                field.write_eq(&mut eq_stream);
            }
        }

        tokens.extend(quote! {
            impl PartialEq for #i {
                fn eq(&self, other: &Self) -> bool {
                    #eq_stream
                    true
                }
            }
        });
    }
}


#[derive(Debug, FromField)]
#[darling(attributes(equalia), forward_attrs(doc, allow, warn))]
struct EqualiaField {
    // field name
    ident: Option<Ident>,

    // field type
    ty: Type,

    #[darling(default)]
    skip: bool,

    #[darling(default)]
    only: bool,

    #[darling(default)]
    map: Option<Ident>,
}

impl EqualiaField {
    fn write_eq(&self, tokens: &mut SynTokenStream) {
        let f_ident = &self.ident;
        let _f_ty = &self.ty;

        if let Some(ref x) = self.map {
            tokens.extend(quote! {
                // TODO: give better error when types don't match
                if #x(&self.#f_ident) != #x(&other.#f_ident) {
                    return false;
                };
            });
        } else {
            tokens.extend(quote! {
                if self.#f_ident != other.#f_ident {
                    return false;
                };
            });
        }
    }
}


// Ok(quote! {#attrs})
// }
#[proc_macro_derive(Equalia, attributes(equalia))]
pub fn derive_equalia(input: TokenStream) -> TokenStream {
    let mut toks = SynTokenStream::new();
    let input: DeriveInput = parse_macro_input!(input);
    if let Data::Struct(_data) = &input.data {
        let attrs: Equalia = match FromDeriveInput::from_derive_input(&input) {
            Ok(v) => v,
            Err(e) => {
                let e = e.to_string();
                toks.extend(quote! { compile_error!(#e); });
                return toks.into();
            }
        };

        toks.extend(quote! { #attrs });
    } else {
        let e = "equalia only supports structs";
        toks.extend(quote! { compile_error!(#e); });
    };

    toks.into()
}
