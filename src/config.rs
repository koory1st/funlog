use crate::log_template::LogTemplate;
use crate::output::Output;
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, token::Comma, Ident};
use syn::{Block, FnArg};

/// Specifies when logging output should occur during function execution.
///
/// # Examples
///
/// ```
/// use funlog::config::OutputPosition;
///
/// let position = OutputPosition::OnStart;
/// // This will log only at the start of function execution
/// ```
#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum OutputPosition {
    /// Log only at the start of function execution
    OnStart,
    /// Log only at the end of function execution
    OnEnd,
    /// Log at both start and end of function execution
    OnStartAndEnd,
}

/// Specifies the type of logging output to use.
///
/// # Examples
///
/// ```
/// use funlog::config::OutputType;
///
/// let output_type = OutputType::Debug;
/// // This will use log::debug! for output
/// ```
#[derive(Debug)]
pub enum OutputType {
    /// Use println! for output
    Print,
    /// Use log::error! for output
    Error,
    /// Use log::warn! for output
    Warn,
    /// Use log::info! for output
    Info,
    /// Use log::debug! for output
    Debug,
    /// Use log::trace! for output
    Trace,
}

/// Configuration struct that holds all the settings for function logging.
///
/// This struct contains all the necessary information to generate the appropriate
/// logging code for a function, including output position, type, parameters, and
/// function metadata.
///
/// # Examples
///
/// ```
/// use funlog::config::{Config, OutputPosition, OutputType};
/// use syn::{parse_quote, Visibility, Block, Ident, ReturnType};
/// use syn::punctuated::Punctuated;
///
/// // This would typically be created by ConfigBuilder
/// ```
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
    /// Converts the configuration into an Output struct containing the generated code.
    ///
    /// This method takes all the configuration settings and generates the appropriate
    /// TokenStream code for the logging functionality.
    ///
    /// # Returns
    ///
    /// Returns an `Output` struct containing all the generated code tokens
    ///
    /// # Examples
    ///
    /// ```
    /// // This is typically called internally by the macro
    /// // let output = config.to_output();
    /// ```
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
        let needs_param_values_for_end = matches!(
            output_position,
            OutputPosition::OnEnd | OutputPosition::OnStartAndEnd
        ) && !func_params_for_output.is_empty();

        let (param_values, param_value_names) = if needs_param_values_for_end {
            let values = func_params_for_output
                .iter()
                .map(|param| {
                    let value_name = format_ident!("__{}_value__", param);
                    quote! {
                        let #value_name = format!("{:?}", #param);
                    }
                })
                .collect::<Vec<_>>();

            let value_names = func_params_for_output
                .iter()
                .map(|param| format_ident!("__{}_value__", param))
                .collect::<Vec<_>>();

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

#[cfg(test)]
mod tests {
    use super::*;
    use quote::format_ident;
    use syn::{parse_quote, ItemFn};

    fn create_test_config() -> Config {
        let func: ItemFn = parse_quote! {
            pub fn test_func(x: i32, y: String) -> i32 {
                x + 1
            }
        };

        Config {
            output_position: OutputPosition::OnStartAndEnd,
            output_type: OutputType::Debug,
            output_ret_value: true,
            func_vis: func.vis,
            func_block: *func.block,
            func_name: func.sig.ident,
            func_params_for_output: vec![format_ident!("x"), format_ident!("y")],
            func_params_for_invoke: vec![format_ident!("x"), format_ident!("y")],
            func_params_for_declare: func.sig.inputs,
            func_return_type: func.sig.output,
        }
    }

    #[test]
    fn test_output_position_debug() {
        let pos = OutputPosition::OnStart;
        assert!(matches!(pos, OutputPosition::OnStart));

        let pos = OutputPosition::OnEnd;
        assert!(matches!(pos, OutputPosition::OnEnd));

        let pos = OutputPosition::OnStartAndEnd;
        assert!(matches!(pos, OutputPosition::OnStartAndEnd));
    }

    #[test]
    fn test_output_type_debug() {
        let output_type = OutputType::Debug;
        assert!(matches!(output_type, OutputType::Debug));

        let output_type = OutputType::Info;
        assert!(matches!(output_type, OutputType::Info));

        let output_type = OutputType::Print;
        assert!(matches!(output_type, OutputType::Print));
    }

    #[test]
    fn test_config_to_output() {
        let config = create_test_config();
        let output = config.to_output();

        // Verify that output contains the expected components
        // We can't easily test the exact token content, but we can verify
        // that the method completes without panicking
        assert!(!output.inner_func.is_empty());
        assert!(!output.func_declare_start.is_empty());
    }

    #[test]
    fn test_config_with_different_positions() {
        let mut config = create_test_config();

        // Test OnStart position
        config.output_position = OutputPosition::OnStart;
        let output = config.to_output();
        assert!(!output.func_output_start.is_empty());

        // Test OnEnd position
        config.output_position = OutputPosition::OnEnd;
        let output = config.to_output();
        assert!(!output.func_output_end.is_empty());

        // Test OnStartAndEnd position
        config.output_position = OutputPosition::OnStartAndEnd;
        let output = config.to_output();
        assert!(!output.func_output_start.is_empty());
        assert!(!output.func_output_end.is_empty());
    }

    #[test]
    fn test_config_with_different_output_types() {
        let mut config = create_test_config();

        // Test different output types
        config.output_type = OutputType::Print;
        let _output = config.to_output();

        config.output_type = OutputType::Error;
        let _output = config.to_output();

        config.output_type = OutputType::Warn;
        let _output = config.to_output();

        config.output_type = OutputType::Info;
        let _output = config.to_output();

        config.output_type = OutputType::Trace;
        let _output = config.to_output();

        // All should complete without panicking
    }

    #[test]
    fn test_config_with_no_parameters() {
        let func: ItemFn = parse_quote! {
            pub fn test_func() -> i32 {
                42
            }
        };

        let config = Config {
            output_position: OutputPosition::OnStartAndEnd,
            output_type: OutputType::Debug,
            output_ret_value: true,
            func_vis: func.vis,
            func_block: *func.block,
            func_name: func.sig.ident,
            func_params_for_output: vec![],
            func_params_for_invoke: vec![],
            func_params_for_declare: func.sig.inputs,
            func_return_type: func.sig.output,
        };

        let output = config.to_output();
        assert!(!output.inner_func.is_empty());
    }

    #[test]
    fn test_config_with_no_return_value() {
        let func: ItemFn = parse_quote! {
            pub fn test_func(x: i32) {
                println!("{}", x);
            }
        };

        let config = Config {
            output_position: OutputPosition::OnStartAndEnd,
            output_type: OutputType::Debug,
            output_ret_value: false,
            func_vis: func.vis,
            func_block: *func.block,
            func_name: func.sig.ident,
            func_params_for_output: vec![format_ident!("x")],
            func_params_for_invoke: vec![format_ident!("x")],
            func_params_for_declare: func.sig.inputs,
            func_return_type: func.sig.output,
        };

        let output = config.to_output();
        assert!(!output.inner_func.is_empty());
    }
}
