mod structure;

use std::{
    env,
    fs::{self},
};

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use structure::Config;

#[proc_macro]
pub fn patterns(_item: TokenStream) -> TokenStream
{
    let path = _item.to_string().replace('\"', "");
    let json = fs::read_to_string(format!(
        "{}/{}",
        env::var("CARGO_MANIFEST_DIR").unwrap(),
        path
    ))
    .expect("can't open file");

    let config: Config = serde_json::from_str(&json).expect("can't parse json");

    let mut code = Vec::new();

    for pattern in &config.signatures
    {
        let name = &pattern.name;
        let extra = pattern.extra;
        let relative = pattern.relative;
        let module = &pattern.module;
        let result_type: proc_macro2::TokenStream = if let Some(result_type) = &pattern.result_type
        {
            result_type.parse().unwrap()
        }
        else
        {
            quote! { usize }
        };
        let offsets = if let Some(offsets) = &pattern.offsets
        {
            quote! { Some(&[#(#offsets),*]) }
        }
        else
        {
            quote! { None }
        };

        let signature_bytes = pattern.pattern.split(' ').map(|x| {
            if x != "?"
            {
                let byte = u8::from_str_radix(x, 16).unwrap();
                quote! { Some(#byte) }
            }
            else
            {
                quote! { None }
            }
        });

        let signature = quote! { &[#(#signature_bytes),*] };

        let varname = Ident::new(name, Span::call_site());

        code.push(quote! {
            pub static #varname: Pattern<#result_type> = Pattern::new(#name, #extra, #relative, #module, #offsets, #signature);
        });
    }

    let code = quote! { #(#code)* };
    //panic!("{}", code);
    code.into()
}
