#![allow(dead_code)]

use darling::*;
use proc_macro2::{Ident, TokenStream as SynTokenStream};
use proc_macro::{TokenStream};
use syn::{DeriveInput, parse_macro_input, Data, Type};
use quote::*;
use quote::ToTokens;

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
        let mut map_stream = SynTokenStream::new();
        let mut eq_stream = SynTokenStream::new();
        let mut hash_stream = SynTokenStream::new();
        for field in self.data.as_ref().take_struct().unwrap().fields {
            if has_only_field {
                if field.only {
                    field.write_eq(&mut eq_stream);
                    if self.hash {
                        field.write_hash(&mut hash_stream);
                    }
                    field.write_map_constraint(&mut map_stream);
                }
            } else {
                // no need to use field
                if field.skip {
                    continue;
                }
                field.write_eq(&mut eq_stream);
                if self.hash {
                    field.write_hash(&mut hash_stream);
                }
                field.write_map_constraint(&mut map_stream);
            }
        }

        tokens.extend(quote! {#map_stream});

        tokens.extend(quote! {
            impl PartialEq for #i {
                fn eq(&self, other: &Self) -> bool {
                    #eq_stream
                    true
                }
            }
        });

        // provide Hash implementation if needed
        if self.hash {
            tokens.extend(quote! {
                impl ::std::hash::Hash for #i {
                    #[allow(unused_variables)]
                    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                        #hash_stream
                    }
                }
            });
        }
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
    fn write_map_constraint(&self, tokens: &mut SynTokenStream) {}
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

    fn write_hash(&self, tokens: &mut SynTokenStream) {
        let f_ident = &self.ident;
        if let Some(ref x) = self.map {
            tokens.extend(quote! {
                #x(&self.#f_ident).hash(state);
            });
        } else {
            tokens.extend(quote! {
                self.#f_ident.hash(state);
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

    println!("toks: {}", toks.to_string());

    toks.into()
}
