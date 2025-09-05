use crate::config::{OutputPosition, OutputType};
use quote::quote;
use syn::{Ident, ReturnType};

pub struct LogTemplate {
    pub function_name: String,
    pub parameters_placeholder: String,
    pub return_placeholder: String,
    pub has_parameters: bool,
    pub has_return_value: bool,
}

impl LogTemplate {
    pub fn new(
        function_name: &str,
        params_for_output: &[Ident],
        return_type: &ReturnType,
        output_ret_value: bool,
    ) -> Self {
        let parameters_placeholder = params_for_output
            .iter()
            .map(|p| format!("{p}:{{}}"))
            .collect::<Vec<String>>()
            .join(", ");

        let return_placeholder = match return_type {
            ReturnType::Default => String::new(),
            _ => "return:{}".to_string(),
        };

        Self {
            function_name: function_name.to_string(),
            parameters_placeholder,
            return_placeholder,
            has_parameters: !params_for_output.is_empty(),
            has_return_value: !matches!(return_type, ReturnType::Default) && output_ret_value,
        }
    }

    pub fn format_start_template(&self) -> String {
        if self.has_parameters {
            format!("{} [in ]: {}", self.function_name, self.parameters_placeholder)
        } else {
            format!("{} [in ]", self.function_name)
        }
    }

    pub fn format_end_template(&self, include_params: bool) -> String {
        match (include_params && self.has_parameters, self.has_return_value) {
            (true, true) => format!(
                "{} [out]: {}, {}",
                self.function_name, self.parameters_placeholder, self.return_placeholder
            ),
            (true, false) => format!("{} [out]: {}", self.function_name, self.parameters_placeholder),
            (false, true) => format!("{} [out]: {}", self.function_name, self.return_placeholder),
            (false, false) => format!("{} [out]", self.function_name),
        }
    }

    pub fn generate_log_statements_with_context(
        &self,
        output_position: &OutputPosition,
        output_type: &OutputType,
        original_params: &[Ident],
        saved_param_values: &[Ident],
    ) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        let log_method = self.get_log_method(output_type);

        let start_statement = match output_position {
            OutputPosition::OnStart | OutputPosition::OnStartAndEnd => {
                let template = self.format_start_template();
                if self.has_parameters {
                    // For start logging, use original parameters with format!
                    let format_args = original_params.iter().map(|p| {
                        quote! { format!("{:?}", #p) }
                    });
                    quote! {
                        #log_method(#template, #(#format_args,)*);
                    }
                } else {
                    quote! {
                        #log_method(#template);
                    }
                }
            }
            _ => quote! {},
        };

        let end_statement = match output_position {
            OutputPosition::OnEnd => {
                // For OnEnd, include parameters in the end log
                let template = self.format_end_template(true);
                match (self.has_parameters, self.has_return_value) {
                    (true, true) => quote! {
                        #log_method(#template, #(#saved_param_values,)* format!("{:?}", output));
                    },
                    (true, false) => quote! {
                        #log_method(#template, #(#saved_param_values,)*);
                    },
                    (false, true) => quote! {
                        #log_method(#template, format!("{:?}", output));
                    },
                    (false, false) => quote! {
                        #log_method(#template);
                    },
                }
            }
            OutputPosition::OnStartAndEnd => {
                // For OnStartAndEnd, only include return value in the end log (parameters already logged at start)
                let template = self.format_end_template(false);
                if self.has_return_value {
                    quote! {
                        #log_method(#template, format!("{:?}", output));
                    }
                } else {
                    quote! {
                        #log_method(#template);
                    }
                }
            }
            _ => quote! {},
        };

        (start_statement, end_statement)
    }

    fn get_log_method(&self, output_type: &OutputType) -> proc_macro2::TokenStream {
        match output_type {
            OutputType::Debug => quote! { log::debug! },
            OutputType::Info => quote! { log::info! },
            OutputType::Warn => quote! { log::warn! },
            OutputType::Error => quote! { log::error! },
            OutputType::Trace => quote! { log::trace! },
            OutputType::Print => quote! { println! },
        }
    }
}