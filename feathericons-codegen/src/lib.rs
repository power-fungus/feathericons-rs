#![feature(proc_macro_raw_ident)]

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

use regex::Regex;

const SVG_ATTRS: &'static str = std::concat!(
    r#"width="24""#,
    r#"height="24""#,
    r#"fill="none""#,
    r#"stroke="currentColor""#,
    r#"stroke-width="2""#,
    r#"stroke-linecap="round""#,
    r#"stroke-linejoin="round"#,
);

#[proc_macro_attribute]
pub fn feathericons(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    foo(attr.into(), input.into()).into()
}

fn foo(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = match syn::parse2::<syn::ItemStruct>(input) {
        Ok(ast) => ast,
        Err(e) => return TokenStream::from(e.to_compile_error()),
    };
    let mut output = TokenStream::new();
    output.extend(input.to_token_stream());
    let ident = &input.ident;
    // panic!("{:#?}", TokenStream::from_str("r#match").unwrap());
    let sprite = std::include_str!("../feather-sprite-4.28.0.svg");

    let re = Regex::new(r#"<symbol id="([^"]*)" viewBox="0 0 24 24">(.*?)</symbol>"#).unwrap();
    let icon_consts = re.captures_iter(sprite).map(|cap| {
        let icon_ident = icon_ident(&cap[1]);
        let svg_str = format!("<svg {}>{}</svg>", SVG_ATTRS, &cap[2]);
        quote! {
            pub const #icon_ident: &'static str = #svg_str;
        }
    });

    output.extend(quote! {
        impl #ident {
            #(#icon_consts)*
        }
    });

    output
}

fn icon_ident(name: &str) -> syn::Ident {
    syn::parse2(TokenStream::from(proc_macro::TokenStream::from(
        proc_macro::TokenTree::from(proc_macro::Ident::new_raw(
            &name.replace("-", "_"),
            proc_macro::Span::call_site(),
        )),
    )))
    .unwrap()
}
