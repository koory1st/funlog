mod config;
mod config_builder;
mod error;
mod generics_item_fn;
mod log_template;
mod output;

use config_builder::ConfigBuilder;
use generics_item_fn::GenericsFn;
use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Meta;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn funlog(args: TokenStream, item: TokenStream) -> TokenStream {
    let is_debug = cfg!(debug_assertions);

    // when not debug, just return the original function
    if !is_debug {
        return item;
    }

    let func = parse_macro_input!(item as ItemFn);
    let func = GenericsFn::from(func);
    let attr_meta: Punctuated<Meta, Comma> =
        parse_macro_input!(args with Punctuated::<Meta, Comma>::parse_terminated);
    match ConfigBuilder::from(attr_meta, func) {
        Ok(config_builder) => match config_builder.build() {
            Ok(config) => {
                let output = config.to_output();
                output.into()
            }
            Err(e) => {
                let syn_error: syn::Error = e.into();
                syn_error.into_compile_error().into()
            }
        },
        Err(e) => {
            let syn_error: syn::Error = e.into();
            syn_error.into_compile_error().into()
        }
    }
}
