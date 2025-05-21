use log::Level;
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::{punctuated::Punctuated, token::Comma, Ident, Meta};
use syn::{Block, FnArg, ItemFn, MetaList, ReturnType};
use syn::{Pat, PatIdent, PatType};

use crate::output::Output;

#[derive(Debug)]
pub enum OutputPosition {
    OnStart,
    OnEnd,
    OnStartAndEnd,
}

#[derive(Debug)]
pub struct Config {
    pub output_position: OutputPosition,
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
            output_position,
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

        let (func_output_start, func_output_end) = match (
            output_position,
            func_return_type,
            func_params_for_output.len(),
        ) {
            (OutputPosition::OnStart, _, 0) => {
                // test() [in ]
                let template = format!("{} [in ]", function_name_str);
                (
                    quote! {
                        #log_method(#template);
                    },
                    quote! {},
                )
            }
            (OutputPosition::OnStart, _, _) => {
                // test() [in]: a:1, b:2
                let template = format!(
                    "{} [in ]: {}",
                    function_name_str, parameters_placeholder_for_output
                );
                (
                    quote! {
                        #log_method(#template, #(#func_params_for_output,)*);
                    },
                    quote! {},
                )
            }
            (OutputPosition::OnEnd, ReturnType::Default, 0) => {
                // test() [out]
                let template = format!("{} [out]", function_name_str);
                (
                    quote! {},
                    quote! {
                        #log_method(#template);
                    },
                )
            }
            (OutputPosition::OnEnd, ReturnType::Default, _) => {
                // test() [out]: a:1, b:2
                let template = format!(
                    "{} [out] {}",
                    function_name_str, parameters_placeholder_for_output
                );
                (
                    quote! {},
                    quote! {
                        #log_method(#template, #(#func_params_for_output,)*);
                    },
                )
            }
            (OutputPosition::OnEnd, _, 0) => {
                // test() [out]: return:3
                let template = format!("{} [out]: {}", function_name_str, return_placeholder);
                (
                    quote! {},
                    quote! {
                        #log_method(#template, output);
                    },
                )
            }
            (OutputPosition::OnEnd, _, _) => {
                // test() [out]: a:1, b:2, return:3
                let template = format!(
                    "{} [out]: {}, {}",
                    function_name_str, function_name_str, return_placeholder
                );
                (
                    quote! {},
                    quote! {
                        #log_method(#template, #(#func_params_for_output,)* output);
                    },
                )
            }
            (OutputPosition::OnStartAndEnd, ReturnType::Default, 0) => {
                // test() [in ]
                // test() [out]
                let template_in = format!("{} [in ]", function_name_str);
                let template_out = format!("{} [out]", function_name_str);
                (
                    quote! {
                        #log_method(#template_in);
                    },
                    quote! {
                        #log_method(#template_out);
                    },
                )
            }
            (OutputPosition::OnStartAndEnd, ReturnType::Default, _) => {
                // test() [in ]: a:1, b:2
                // test() [out]
                let template_in = format!(
                    "{} [in ]: {}",
                    function_name_str, parameters_placeholder_for_output
                );
                let template_out = format!("{} [out]", function_name_str);
                (
                    quote! {
                        #log_method(#template_in, #(#func_params_for_output,)*);
                    },
                    quote! {
                        #log_method(#template_out);
                    },
                )
            }
            (OutputPosition::OnStartAndEnd, _, 0) => {
                // test() [in ]
                // test() [out]: return:3
                let template_in = format!("{} [in ]", function_name_str);
                let template_out = format!("{} [out]: {}", function_name_str, return_placeholder);
                (
                    quote! {
                        #log_method(#template_in);
                    },
                    quote! {
                        #log_method(#template_out, output);
                    },
                )
            }
            (OutputPosition::OnStartAndEnd, _, _) => {
                // test() [in ]: a:1, b:2
                // test() [out]: return:3
                let template_in = format!(
                    "{} [in ]: {}",
                    function_name_str, parameters_placeholder_for_output
                );
                let template_out = format!("{} [out]: {}", function_name_str, return_placeholder);
                (
                    quote! {
                        #log_method(#template_in, #(#func_params_for_output,)*);
                    },
                    quote! {
                        #log_method(#template_out, output);
                    },
                )
            }
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
