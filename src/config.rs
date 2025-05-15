use log::Level;
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::{punctuated::Punctuated, token::Comma, Ident, Meta};
use syn::{Block, FnArg, ItemFn, MetaList, ReturnType};
use syn::{Pat, PatIdent, PatType};

use crate::output::Output;

#[derive(Debug)]
pub enum OutputType {
    OnStart,
    OnEnd,
    OnStartAndEnd,
}

#[derive(Debug)]
pub struct Config {
    pub output_type: OutputType,
    pub log_level: Level,
    pub func_vis: syn::Visibility,
    pub func_block: Box<Block>,
    pub func_name: syn::Ident,
    pub func_params_for_output: Vec<Ident>,
    pub func_params_for_invoke: Vec<Ident>,
    pub func_params_for_declare: Punctuated<FnArg, Comma>,
    pub func_return_type: syn::ReturnType,
}

impl Config {
    pub(crate) fn to_output(&self) -> Output {
        let Config {
            func_vis,
            func_block,
            func_name,
            func_params_for_output,
            func_params_for_invoke,
            func_params_for_declare,
            func_return_type,
            output_type,
            log_level,
        } = self;
        let inner_func_name = format_ident!("__{}__", func_name);
        let inner_func: proc_macro2::TokenStream = quote! {
            fn #inner_func_name(#func_params_for_declare) #func_return_type {
                #func_block
            }
        };
        let func_declare_start = quote! {
            #func_vis fn #func_name(#func_params_for_declare) #func_return_type
        };
        let func_declare_body = quote! {
            let output = #inner_func_name(#(#func_params_for_invoke,) *);
        };
        let func_declare_end = quote! {
            output
        };

        let log_method = match log_level {
            Level::Debug => quote! {
                log::debug!
            },
            Level::Info => quote! {
                log::info!
            },
            Level::Warn => quote! {
                log::warn!
            },
            Level::Error => quote! {
                log::error!
            },
            Level::Trace => quote! {
                log::trace!
            },
        };

        let function_name_str = format!("{}", func_name);

        let parameters_placeholder_for_output = func_params_for_output
            .iter()
            .map(|p| format!("{}:{{}}", p))
            .collect::<Vec<String>>()
            .join(", ");

        let return_placeholder = match func_return_type {
            syn::ReturnType::Default => String::new(),
            _ => format!("return:{{}}"),
        };

        // output_type, func_output, func_params_for_output
        // start mode
        // test() [in]: a:1, b:2
        // test() [in]

        // startEnd mode
        // test() [in]:  a:1, b:2
        // test() [out]: return:3
        // test() [in]
        // test() [out]

        // end mode
        // test() [out]: a:1, b:2, return:3
        // test() [out]: a:1, b:2
        // test() [out]: return:3
        // test() [out]
        let (start, end) = match (output_type, func_return_type, func_params_for_output.len()) {
            (OutputType::OnStart, _, 0) => {
                // test() [in]
                let template = format!("{} [in]", function_name_str);
                (
                    quote! {
                        #log_method(template, #function_name_str);
                    },
                    quote! {},
                )
            }
            (OutputType::OnStart, _, _) => {
                // test() [in]: a:1, b:2
                let template = format!(
                    "{} [in]: {}",
                    function_name_str, parameters_placeholder_for_output
                );
                (
                    quote! {
                        #log_method(template, #function_name_str, #(#func_params_for_output,)*);
                    },
                    quote! {},
                )
            }
            (OutputType::OnEnd, ReturnType::Default, 0) => {
                // test() [out]
                let template = format!("{} [out]", function_name_str);
                (
                    quote! {
                        #log_method(template, #function_name_str);
                    },
                    quote! {},
                )
            }
        };

        let log_template = format!(
            "{} [in]: {}",
            function_name_str, parameters_placeholder_for_output
        );
        dbg!(&log_template);
        let func_output_start = quote! {
            #log_method(#log_template, #(#func_params_for_output,)*);
        };
        let func_output_end = quote! {
            #log_method("{} [out]", #function_name_str);
        };
        Output {
            inner_func,
            func_declare_start,
            func_declare_body,
            func_declare_end,
            func_output_start,
            func_output_end,
        }
    }
}
