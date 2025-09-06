use crate::config::{OutputPosition, OutputType};
use quote::quote;
use syn::{Ident, ReturnType};

/// Template for generating log statements with proper formatting.
///
/// This struct handles the creation of log message templates and generates
/// the appropriate logging code based on the configuration.
///
/// # Examples
///
/// ```
/// use funlog::log_template::LogTemplate;
/// use syn::{parse_quote, ReturnType, Ident};
/// use quote::format_ident;
///
/// let params = vec![format_ident!("x"), format_ident!("y")];
/// let return_type: ReturnType = parse_quote! { -> i32 };
/// let template = LogTemplate::new("test_func", &params, &return_type, true);
/// ```
pub struct LogTemplate {
    /// The name of the function being logged
    pub function_name: String,
    /// Template string for parameter formatting
    pub parameters_placeholder: String,
    /// Template string for return value formatting
    pub return_placeholder: String,
    /// Whether the function has parameters to log
    pub has_parameters: bool,
    /// Whether the function has a return value to log
    pub has_return_value: bool,
}

impl LogTemplate {
    /// Creates a new LogTemplate with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `function_name` - The name of the function to log
    /// * `params_for_output` - The parameters to include in logging
    /// * `return_type` - The return type of the function
    /// * `output_ret_value` - Whether to include return value in logs
    ///
    /// # Returns
    ///
    /// Returns a new LogTemplate instance
    ///
    /// # Examples
    ///
    /// ```
    /// use funlog::log_template::LogTemplate;
    /// use syn::{parse_quote, ReturnType};
    /// use quote::format_ident;
    ///
    /// let params = vec![format_ident!("x")];
    /// let return_type: ReturnType = parse_quote! { -> i32 };
    /// let template = LogTemplate::new("test", &params, &return_type, true);
    /// assert_eq!(template.function_name, "test");
    /// assert!(template.has_parameters);
    /// assert!(template.has_return_value);
    /// ```
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

    /// Formats the template string for function start logging.
    ///
    /// # Returns
    ///
    /// Returns a formatted string template for the start of function execution
    ///
    /// # Examples
    ///
    /// ```
    /// use funlog::log_template::LogTemplate;
    /// use syn::{parse_quote, ReturnType};
    /// use quote::format_ident;
    ///
    /// let params = vec![format_ident!("x")];
    /// let return_type: ReturnType = parse_quote! { -> i32 };
    /// let template = LogTemplate::new("test", &params, &return_type, false);
    /// let start_template = template.format_start_template();
    /// assert!(start_template.contains("test [in ]"));
    /// ```
    pub fn format_start_template(&self) -> String {
        if self.has_parameters {
            format!(
                "{} [in ]: {}",
                self.function_name, self.parameters_placeholder
            )
        } else {
            format!("{} [in ]", self.function_name)
        }
    }

    /// Formats the template string for function end logging.
    ///
    /// # Arguments
    ///
    /// * `include_params` - Whether to include parameters in the end template
    ///
    /// # Returns
    ///
    /// Returns a formatted string template for the end of function execution
    ///
    /// # Examples
    ///
    /// ```
    /// use funlog::log_template::LogTemplate;
    /// use syn::{parse_quote, ReturnType};
    /// use quote::format_ident;
    ///
    /// let params = vec![format_ident!("x")];
    /// let return_type: ReturnType = parse_quote! { -> i32 };
    /// let template = LogTemplate::new("test", &params, &return_type, true);
    /// let end_template = template.format_end_template(true);
    /// assert!(end_template.contains("test [out]"));
    /// ```
    pub fn format_end_template(&self, include_params: bool) -> String {
        match (include_params && self.has_parameters, self.has_return_value) {
            (true, true) => format!(
                "{} [out]: {}, {}",
                self.function_name, self.parameters_placeholder, self.return_placeholder
            ),
            (true, false) => format!(
                "{} [out]: {}",
                self.function_name, self.parameters_placeholder
            ),
            (false, true) => format!("{} [out]: {}", self.function_name, self.return_placeholder),
            (false, false) => format!("{} [out]", self.function_name),
        }
    }

    /// Generates the actual log statements as TokenStreams.
    ///
    /// # Arguments
    ///
    /// * `output_position` - When to output logs (start, end, or both)
    /// * `output_type` - What type of logging to use (print, debug, etc.)
    /// * `original_params` - The original parameter identifiers
    /// * `saved_param_values` - The saved parameter value identifiers
    ///
    /// # Returns
    ///
    /// Returns a tuple of (start_statement, end_statement) TokenStreams
    ///
    /// # Examples
    ///
    /// ```
    /// use funlog::log_template::LogTemplate;
    /// use funlog::config::{OutputPosition, OutputType};
    /// use syn::{parse_quote, ReturnType};
    /// use quote::format_ident;
    ///
    /// let params = vec![format_ident!("x")];
    /// let return_type: ReturnType = parse_quote! { -> i32 };
    /// let template = LogTemplate::new("test", &params, &return_type, false);
    /// let (start, end) = template.generate_log_statements_with_context(
    ///     &OutputPosition::OnStart,
    ///     &OutputType::Print,
    ///     &params,
    ///     &[]
    /// );
    /// ```
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

    /// Gets the appropriate logging method TokenStream for the output type.
    ///
    /// # Arguments
    ///
    /// * `output_type` - The type of output to generate method for
    ///
    /// # Returns
    ///
    /// Returns a TokenStream representing the logging method call
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

#[cfg(test)]
mod tests {
    use super::*;
    use quote::format_ident;
    use syn::{parse_quote, ReturnType};

    #[test]
    fn test_new_with_parameters_and_return() {
        let params = vec![format_ident!("x"), format_ident!("y")];
        let return_type: ReturnType = parse_quote! { -> i32 };

        let template = LogTemplate::new("test_func", &params, &return_type, true);

        assert_eq!(template.function_name, "test_func");
        assert_eq!(template.parameters_placeholder, "x:{}, y:{}");
        assert_eq!(template.return_placeholder, "return:{}");
        assert!(template.has_parameters);
        assert!(template.has_return_value);
    }

    #[test]
    fn test_new_no_parameters_no_return() {
        let params = vec![];
        let return_type: ReturnType = parse_quote! {};

        let template = LogTemplate::new("test_func", &params, &return_type, false);

        assert_eq!(template.function_name, "test_func");
        assert_eq!(template.parameters_placeholder, "");
        assert_eq!(template.return_placeholder, "");
        assert!(!template.has_parameters);
        assert!(!template.has_return_value);
    }

    #[test]
    fn test_new_with_parameters_no_return() {
        let params = vec![format_ident!("x")];
        let return_type: ReturnType = parse_quote! {};

        let template = LogTemplate::new("test_func", &params, &return_type, false);

        assert_eq!(template.parameters_placeholder, "x:{}");
        assert!(template.has_parameters);
        assert!(!template.has_return_value);
    }

    #[test]
    fn test_new_no_parameters_with_return() {
        let params = vec![];
        let return_type: ReturnType = parse_quote! { -> String };

        let template = LogTemplate::new("test_func", &params, &return_type, true);

        assert!(!template.has_parameters);
        assert!(template.has_return_value);
        assert_eq!(template.return_placeholder, "return:{}");
    }

    #[test]
    fn test_format_start_template_with_params() {
        let params = vec![format_ident!("x"), format_ident!("y")];
        let return_type: ReturnType = parse_quote! { -> i32 };

        let template = LogTemplate::new("test_func", &params, &return_type, true);
        let start_template = template.format_start_template();

        assert_eq!(start_template, "test_func [in ]: x:{}, y:{}");
    }

    #[test]
    fn test_format_start_template_no_params() {
        let params = vec![];
        let return_type: ReturnType = parse_quote! { -> i32 };

        let template = LogTemplate::new("test_func", &params, &return_type, true);
        let start_template = template.format_start_template();

        assert_eq!(start_template, "test_func [in ]");
    }

    #[test]
    fn test_format_end_template_with_params_and_return() {
        let params = vec![format_ident!("x")];
        let return_type: ReturnType = parse_quote! { -> i32 };

        let template = LogTemplate::new("test_func", &params, &return_type, true);
        let end_template = template.format_end_template(true);

        assert_eq!(end_template, "test_func [out]: x:{}, return:{}");
    }

    #[test]
    fn test_format_end_template_with_params_no_return() {
        let params = vec![format_ident!("x")];
        let return_type: ReturnType = parse_quote! {};

        let template = LogTemplate::new("test_func", &params, &return_type, false);
        let end_template = template.format_end_template(true);

        assert_eq!(end_template, "test_func [out]: x:{}");
    }

    #[test]
    fn test_format_end_template_no_params_with_return() {
        let params = vec![];
        let return_type: ReturnType = parse_quote! { -> i32 };

        let template = LogTemplate::new("test_func", &params, &return_type, true);
        let end_template = template.format_end_template(false);

        assert_eq!(end_template, "test_func [out]: return:{}");
    }

    #[test]
    fn test_format_end_template_no_params_no_return() {
        let params = vec![];
        let return_type: ReturnType = parse_quote! {};

        let template = LogTemplate::new("test_func", &params, &return_type, false);
        let end_template = template.format_end_template(false);

        assert_eq!(end_template, "test_func [out]");
    }

    #[test]
    fn test_get_log_method() {
        let template = LogTemplate::new("test", &[], &parse_quote! {}, false);

        // Test all output types
        let debug_method = template.get_log_method(&OutputType::Debug);
        let info_method = template.get_log_method(&OutputType::Info);
        let warn_method = template.get_log_method(&OutputType::Warn);
        let error_method = template.get_log_method(&OutputType::Error);
        let trace_method = template.get_log_method(&OutputType::Trace);
        let print_method = template.get_log_method(&OutputType::Print);

        // We can't easily test the exact token content, but we can verify
        // that the methods return non-empty token streams
        assert!(!debug_method.is_empty());
        assert!(!info_method.is_empty());
        assert!(!warn_method.is_empty());
        assert!(!error_method.is_empty());
        assert!(!trace_method.is_empty());
        assert!(!print_method.is_empty());
    }

    #[test]
    fn test_generate_log_statements_on_start() {
        let params = vec![format_ident!("x")];
        let return_type: ReturnType = parse_quote! { -> i32 };
        let template = LogTemplate::new("test_func", &params, &return_type, true);

        let (start, end) = template.generate_log_statements_with_context(
            &OutputPosition::OnStart,
            &OutputType::Print,
            &params,
            &[],
        );

        assert!(!start.is_empty());
        assert!(end.is_empty());
    }

    #[test]
    fn test_generate_log_statements_on_end() {
        let params = vec![format_ident!("x")];
        let saved_params = vec![format_ident!("__x_value__")];
        let return_type: ReturnType = parse_quote! { -> i32 };
        let template = LogTemplate::new("test_func", &params, &return_type, true);

        let (start, end) = template.generate_log_statements_with_context(
            &OutputPosition::OnEnd,
            &OutputType::Print,
            &params,
            &saved_params,
        );

        assert!(start.is_empty());
        assert!(!end.is_empty());
    }

    #[test]
    fn test_generate_log_statements_on_start_and_end() {
        let params = vec![format_ident!("x")];
        let saved_params = vec![format_ident!("__x_value__")];
        let return_type: ReturnType = parse_quote! { -> i32 };
        let template = LogTemplate::new("test_func", &params, &return_type, true);

        let (start, end) = template.generate_log_statements_with_context(
            &OutputPosition::OnStartAndEnd,
            &OutputType::Print,
            &params,
            &saved_params,
        );

        assert!(!start.is_empty());
        assert!(!end.is_empty());
    }

    #[test]
    fn test_generate_log_statements_no_params_no_return() {
        let params = vec![];
        let return_type: ReturnType = parse_quote! {};
        let template = LogTemplate::new("test_func", &params, &return_type, false);

        let (start, end) = template.generate_log_statements_with_context(
            &OutputPosition::OnStartAndEnd,
            &OutputType::Debug,
            &params,
            &[],
        );

        assert!(!start.is_empty());
        assert!(!end.is_empty());
    }
}
