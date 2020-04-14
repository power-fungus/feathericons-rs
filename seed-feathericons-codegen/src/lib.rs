extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote_spanned;
use quote::ToTokens;

use regex::Regex;
use std::path::Path;
use std::{env, fs, str};

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
    let span = ident.span();

    let sprite_file = env::var("FEATHERICONS_SPRITE").unwrap();
    let sprite_content = fs::read(Path::new(&sprite_file)).unwrap();
    let sprite_content = str::from_utf8(&sprite_content).unwrap();

    let re = Regex::new(r#"<symbol id="([^"]*)" viewBox="0 0 24 24">(.*?)</symbol>"#).unwrap();
    let icon_fns = re.captures_iter(sprite_content).map(|cap| {
        let fn_ident = syn::Ident::new(&cap[1].replace("-", "_"), span);
        let svg_str = format!("<svg {}>{}</svg>", SVG_ATTRS, &cap[2]);
        quote_spanned! {span=>
            const #fn_ident: &'static str = #svg_str;
        }
    });

    output.extend(quote_spanned! {span=>
        impl #ident {
            #(#icon_fns)*
        }
    });

    output
}
