// https://github.com/cksac/fake-rs/blob/master/dummy_derive/src/lib.rs

use syn::{Ident};

use darling::{FromMeta, FromDeriveInput};

#[derive(Default, FromMeta)]
#[darling(default)]
struct Lorem {
    #[darling(rename = "sit")]
    ipsum: bool,
    dolor: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(from_ident, attributes(my_crate), forward_attrs(allow, doc, cfg))]
struct MyTraitOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    // lorem: Lorem,
}

impl From<Ident> for MyTraitOpts {
    fn from(ident: Ident) -> Self {
        MyTraitOpts {
            ident,
            attrs: Self::attrs,
            // lorem,
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
