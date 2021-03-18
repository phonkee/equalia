#![allow(dead_code)]

use darling::*;
use proc_macro2::{Ident, TokenStream as SynTokenStream};
use proc_macro::{TokenStream, Span};
use syn::{DeriveInput, parse_macro_input, Data, DataStruct, Type};
use quote::*;

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

/// generate actual equalia implementations
fn generate_equalia(_input: &DeriveInput, _data: &DataStruct) -> std::result::Result<SynTokenStream, SynTokenStream> {
    let attrs: Equalia = match FromDeriveInput::from_derive_input(_input) {
        Ok(v) => v,
        Err(e) => return Err(e.write_errors()),
    };

    let _has_only_field = attrs.has_only_field();

    println!("this is attrs: {:?}", attrs);

    let i = attrs.ident;

    Ok(quote! {
        impl PartialEq for #i {
            fn eq(&self, other: &Self) -> bool {
                todo!()
            }
        }
        impl Eq for #i{}
    })
}


#[proc_macro_derive(Equalia, attributes(equalia))]
pub fn derive_equalia(input: TokenStream) -> TokenStream {
    let mut toks = SynTokenStream::new();
    let input: DeriveInput = parse_macro_input!(input);
    if let Data::Struct(_data) = &input.data {
        toks.extend(match generate_equalia(&input, _data) {
            Ok(t) => t,
            Err(t) => t,
        })
    };

    toks.into()
}
