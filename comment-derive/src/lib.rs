use quote::quote;
use syn::{parse_macro_input, AttrStyle, Attribute, DeriveInput, PathArguments};

fn attr_eq(attr: &Attribute, name: &str) -> bool {
    attr.style == AttrStyle::Outer && attr.path.segments.len() == 1 && {
        let segment = attr.path.segments.first().unwrap();
        segment.arguments == PathArguments::None && *segment.ident.to_string() == *name
    }
}

fn get_title(stream: proc_macro2::TokenStream) -> String {
    let val = stream.to_string();
    let l = val.find('"').unwrap();
    let r = val.rfind('"').unwrap();
    val[(l + 1)..r].trim().to_string()
}

fn gen_ser(field: syn::Field) -> proc_macro2::TokenStream {
    let is_doc = |attr: &Attribute| attr_eq(attr, "doc");

    let name: syn::Ident = field.ident.unwrap();

    let comments: Vec<_> = field
        .attrs
        .into_iter()
        .filter(is_doc)
        .map(|attr| attr.tokens)
        .map(get_title)
        .collect();

    if comments.is_empty() {
        quote! {
            s.add_field(stringify!(#name), &self.#name)?;
        }
    } else {
        let comments = comments.join("\n");

        let comment = syn::LitStr::new(&comments, proc_macro2::Span::call_site());
        quote! {
            s.add_comment(stringify!(#name), #comment)?;
            s.add_field(stringify!(#name), &self.#name)?;
        }
    }
}

#[proc_macro_derive(Comment)]
pub fn comment(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let s = match data {
        syn::Data::Struct(s) => s,
        _ => panic!("The comment macro can only be applied to structs"),
    };

    let fields = match s.fields {
        syn::Fields::Named(named) => named.named,
        _ => panic!("must be named"),
    };

    let _len = fields.len();

    let sers = fields.into_iter().map(gen_ser);

    let output = quote! {

    impl comment::Comment for #ident {
        fn serialize<S>(&self, mut s: S) -> Result<S::Ok, S::Error> where S: comment::CommentSerializer {
            use serde::ser::SerializeStruct;
            #(#sers)*
            s.end()
        }
    }
    };

    output.into()
}
