extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn json_pretty_str(input: TokenStream) -> TokenStream {
    let j = json::parse(&input.to_string()).unwrap();
    let s = json::stringify_pretty(j, 4);
    quote!(#s).into()
}

#[proc_macro]
pub fn json_str(input: TokenStream) -> TokenStream {
    let s = json::stringify(json::parse(&input.to_string()).unwrap());
    quote!(#s).into()
}

#[proc_macro]
pub fn replace_underscore_with_dash(input: TokenStream) -> TokenStream {
    let s = input.to_string().replace('_', "-");
    quote!(#s).into()
}

// #[proc_macro]
// pub fn newlines_to_carriage_returns(input: TokenStream) -> TokenStream {
//     let s = input.to_string();
//     let s = s.replace('\n', "\r");
//     quote::quote! {
//         #s
//     }
//     .into()
// }
