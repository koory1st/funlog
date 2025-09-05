use crate::log_template::LogTemplate;
use crate::output::Output;
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, token::Comma, Ident};
use syn::{Block, FnArg};

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum OutputPosition {
    OnStart,
    OnEnd,
    OnStartAndEnd,
}

#[derive(Debug)]
pub enum OutputType {
    Print,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug)]
pub struct Config {
    pub output_position: OutputPosition,
    pub output_type: OutputType,
    pub output_ret_value: bool,
    pub func_vis: syn::Visibility,
    pub func_block: Block,
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
            output_type,
            output_ret_value,
        } = self;

        let inner_func_name = format_ident!("__{}__", func_name);
        let inner_func: proc_macro2::TokenStream = quote! {
            #[allow(clippy::too_many_arguments)]
            fn #inner_func_name(#func_params_for_declare) #func_return_type {
                #func_block
            }
        };
        
        let func_declare_start = quote! {
            #[allow(clippy::too_many_arguments)]
            #func_vis fn #func_name(#func_params_for_declare) #func_return_type
        };

        // Check if we need to save parameter values for later use (onEnd or onStartAndEnd with parameters)
        let needs_param_values_for_end = matches!(output_position, 
            OutputPosition::OnEnd | OutputPosition::OnStartAndEnd
        ) && !func_params_for_output.is_empty();

        let (param_values, param_value_names) = if needs_param_values_for_end {
            let values = func_params_for_output.iter().map(|param| {
                let value_name = format_ident!("__{}_value__", param);
                quote! {
                    let #value_name = format!("{:?}", #param);
                }
            }).collect::<Vec<_>>();

            let value_names = func_params_for_output.iter().map(|param| {
                format_ident!("__{}_value__", param)
            }).collect::<Vec<_>>();

            (values, value_names)
        } else {
            (Vec::new(), Vec::new())
        };

        let func_declare_body = quote! {
            #(#param_values)*
            let output = #inner_func_name(#(#func_params_for_invoke,) *);
        };
        
        let func_declare_end = quote! {
            output
        };

        // Use the new LogTemplate to generate log statements
        let template = LogTemplate::new(
            &func_name.to_string(),
            func_params_for_output,
            func_return_type,
            *output_ret_value,
        );

        let (func_output_start, func_output_end) = template.generate_log_statements_with_context(
            output_position,
            output_type,
            func_params_for_output,
            &param_value_names,
        );

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
