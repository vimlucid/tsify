use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput};

use crate::{container::Container, namespace, parser::Parser, wasm_bindgen};
use crate::namespace::Namespace;

pub fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let cont = Container::from_derive_input(&input)?;

    let parser = Parser::new(&cont);
    let decl = parser.parse();

    let (impl_generics, ty_generics, where_clause) = cont.generics().split_for_impl();

    let ident = cont.ident();
    let decl_str = match &cont.attrs.namespace {
        Some(ns) => namespace::wrap(&decl.to_string(), Namespace(ns)),
        None => decl.to_string()
    };

    let tokens = if cfg!(feature = "wasm-bindgen") {
        // TODO Test
        wasm_bindgen::expand(&cont, decl)
    } else {
        quote! {
            const _: () = {
                use tsify_next::Tsify;
                #[automatically_derived]
                impl #impl_generics Tsify for #ident #ty_generics #where_clause {
                    const DECL: &'static str = #decl_str;
                    const CONFIG: tsify_next::SerializationConfig;
                }
            };
        }
    };

    cont.check()?;

    Ok(tokens)
}

/// Expand an `enum` or `struct` with `#[derive(Tsify)]`.
pub fn expand_by_attr(args: TokenStream, input: DeriveInput) -> syn::Result<TokenStream> {
    let mut cloned_input = input.clone();
    let attr: syn::Attribute = parse_quote!(#[tsify(#args)]);
    cloned_input.attrs.push(attr);

    let derived = expand(cloned_input)?;

    let tokens = quote! {
      #input
      #derived
    };

    Ok(tokens)
}
