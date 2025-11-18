#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Proc-macro for auto implementing
/// `jsmoke_utils::printing::print_error::ErrorPrint` trait.
///
/// Easy task since `ErrorPrint` provides default behavior function.
#[proc_macro_derive(ErrorPrint)]
pub fn print_error_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    quote! {
        impl ErrorPrint for #ident {}
    }
    .into()
}

/// Proc-macro for auto implementing
/// `jsmoke_utils::printing::warning_print::WarningPrint` trait.
///
/// Easy task since `WarningPrint` provides default behavior
/// function.
#[proc_macro_derive(WarningPrint)]
pub fn print_warning_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    quote! {
        impl WarningPrint for #ident {}
    }
    .into()
}
