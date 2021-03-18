#![allow(dead_code)]

use darling::*;
use proc_macro2::{Ident, TokenStream as SynTokenStream};
use proc_macro::{TokenStream, Span};
use syn::{DeriveInput, parse_macro_input, Data, DataStruct};
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

#[derive(Default, FromMeta)]
#[darling(default)]
struct Equalia {
    hash: bool,
}

#[derive(FromField)]
#[darling(attributes(equalia), forward_attrs(doc, allow, warn))]
struct Attribute {
    skip: bool,
    only: bool,
    map: Option<Ident>,
}

/// generate actual equalia implementations
fn generate_equalia(_input: &DeriveInput, _data: &DataStruct) -> std::result::Result<SynTokenStream, SynTokenStream> {
    println!("fields: {:?}", _data.fields);
    Ok(SynTokenStream::new())
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
