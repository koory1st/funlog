mod config;
mod config_builder;
mod generics_item_fn;
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
    let is_debug = if cfg!(debug_assertions) { true } else { false };

    // when not debug, just return the original function
    if !is_debug {
        return item;
    }

    let func = parse_macro_input!(item as ItemFn);
    let func = GenericsFn::from(func);
    let attr_meta: Punctuated<Meta, Comma> =
        parse_macro_input!(args with Punctuated::<Meta, Comma>::parse_terminated);
    match ConfigBuilder::from(attr_meta, func) {
        Ok(config_builder) => {
            let config = config_builder.build();
            let output = config.to_output();
            output.into()
        }
        Err(e) => e.into_compile_error().into(),
    }
}
