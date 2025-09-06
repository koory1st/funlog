use std::fmt;

/// Represents various configuration errors that can occur during macro processing.
///
/// This enum covers all possible error conditions that can arise when parsing
/// and validating the funlog macro attributes and function information.
///
/// # Examples
///
/// ```
/// use funlog::error::ConfigError;
///
/// let error = ConfigError::AlreadySet("return value configuration");
/// println!("{}", error);
/// ```
#[derive(Debug, Clone)]
pub enum ConfigError {
    /// A configuration option has already been set
    AlreadySet(&'static str),
    /// An invalid parameter was specified
    InvalidParameter {
        param: String,
        available: Vec<String>,
    },
    /// An invalid attribute was used
    InvalidAttribute {
        attr: String,
        suggestion: Option<String>,
    },
    /// A parsing error occurred
    ParseError(String),
    /// Conflicting configuration options were specified
    ConflictingOptions { option1: String, option2: String },
    /// The macro was not applied to a function
    MissingFunction,
    /// Invalid parameter syntax was used
    InvalidParameterSyntax { param: String, expected: String },
}

impl fmt::Display for ConfigError {
    /// Formats the error with helpful messages and suggestions.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to
    ///
    /// # Returns
    ///
    /// Returns `fmt::Result` indicating success or failure of formatting
    ///
    /// # Examples
    ///
    /// ```
    /// use funlog::error::ConfigError;
    ///
    /// let error = ConfigError::AlreadySet("test");
    /// let formatted = format!("{}", error);
    /// assert!(formatted.contains("already been set"));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::AlreadySet(field) => {
                writeln!(
                    f,
                    "funlog configuration error: '{field}' option has already been set"
                )?;
                write!(f, "💡 Hint: Each configuration option can only be set once, please check for duplicate configurations")
            }
            ConfigError::InvalidParameter { param, available } => {
                writeln!(
                    f,
                    "funlog parameter error: parameter '{param}' does not exist"
                )?;
                if available.is_empty() {
                    write!(f, "💡 Hint: This function has no parameters, please use 'none' or remove the params() configuration")
                } else {
                    writeln!(
                        f,
                        "💡 Hint: Available parameters are: {}",
                        available.join(", ")
                    )?;
                    write!(
                        f,
                        "   Correct usage: #[funlog(params({}))]",
                        available.join(", ")
                    )
                }
            }
            ConfigError::InvalidAttribute { attr, suggestion } => {
                writeln!(
                    f,
                    "funlog configuration error: unknown configuration option '{attr}'"
                )?;
                if let Some(suggestion) = suggestion {
                    writeln!(f, "💡 Hint: Did you mean '{suggestion}'?")?;
                }
                writeln!(f, "📖 Available configuration options:")?;
                writeln!(f, "   Log levels: print, trace, debug, info, warn, error")?;
                writeln!(
                    f,
                    "   Parameter control: all, none, params(parameter_names...)"
                )?;
                writeln!(f, "   Position control: onStart, onEnd, onStartEnd")?;
                write!(f, "   Return value: retVal")
            }
            ConfigError::ParseError(msg) => {
                writeln!(f, "funlog parse error: {msg}")?;
                write!(f, "💡 Hint: Please check if the macro syntax is correct, example: #[funlog(debug, all)]")
            }
            ConfigError::ConflictingOptions { option1, option2 } => {
                writeln!(f, "funlog configuration conflict: '{option1}' and '{option2}' cannot be used together")?;
                write!(f, "💡 Hint: Please choose one of the options")
            }
            ConfigError::MissingFunction => {
                writeln!(f, "funlog error: can only be used on functions")?;
                write!(f, "💡 Hint: funlog macro can only be applied to function definitions, not other items")
            }
            ConfigError::InvalidParameterSyntax { param, expected } => {
                writeln!(
                    f,
                    "funlog parameter syntax error: '{param}' format is incorrect"
                )?;
                write!(f, "💡 Hint: Expected format is {expected}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<ConfigError> for syn::Error {
    /// Converts a ConfigError into a syn::Error for use in procedural macros.
    ///
    /// # Arguments
    ///
    /// * `err` - The ConfigError to convert
    ///
    /// # Returns
    ///
    /// Returns a syn::Error with the error message and call site span
    ///
    /// # Examples
    ///
    /// ```
    /// use funlog::error::ConfigError;
    /// use syn::Error;
    ///
    /// let config_error = ConfigError::MissingFunction;
    /// let syn_error: Error = config_error.into();
    /// ```
    fn from(err: ConfigError) -> Self {
        syn::Error::new(proc_macro2::Span::call_site(), err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_already_set_error() {
        let error = ConfigError::AlreadySet("test field");
        let message = format!("{error}");
        assert!(message.contains("test field"));
        assert!(message.contains("already been set"));
        assert!(message.contains("💡 Hint"));
    }

    #[test]
    fn test_invalid_parameter_error_with_available() {
        let error = ConfigError::InvalidParameter {
            param: "invalid_param".to_string(),
            available: vec!["x".to_string(), "y".to_string()],
        };
        let message = format!("{error}");
        assert!(message.contains("invalid_param"));
        assert!(message.contains("x, y"));
        assert!(message.contains("💡 Hint"));
    }

    #[test]
    fn test_invalid_parameter_error_no_available() {
        let error = ConfigError::InvalidParameter {
            param: "invalid_param".to_string(),
            available: vec![],
        };
        let message = format!("{error}");
        assert!(message.contains("invalid_param"));
        assert!(message.contains("no parameters"));
        assert!(message.contains("💡 Hint"));
    }

    #[test]
    fn test_invalid_attribute_error_with_suggestion() {
        let error = ConfigError::InvalidAttribute {
            attr: "debg".to_string(),
            suggestion: Some("debug".to_string()),
        };
        let message = format!("{error}");
        assert!(message.contains("debg"));
        assert!(message.contains("debug"));
        assert!(message.contains("💡 Hint"));
        assert!(message.contains("📖 Available"));
    }

    #[test]
    fn test_invalid_attribute_error_no_suggestion() {
        let error = ConfigError::InvalidAttribute {
            attr: "unknown".to_string(),
            suggestion: None,
        };
        let message = format!("{error}");
        assert!(message.contains("unknown"));
        assert!(message.contains("📖 Available"));
    }

    #[test]
    fn test_parse_error() {
        let error = ConfigError::ParseError("test parse error".to_string());
        let message = format!("{error}");
        assert!(message.contains("test parse error"));
        assert!(message.contains("💡 Hint"));
    }

    #[test]
    fn test_conflicting_options_error() {
        let error = ConfigError::ConflictingOptions {
            option1: "debug".to_string(),
            option2: "info".to_string(),
        };
        let message = format!("{error}");
        assert!(message.contains("debug"));
        assert!(message.contains("info"));
        assert!(message.contains("cannot be used together"));
        assert!(message.contains("💡 Hint"));
    }

    #[test]
    fn test_missing_function_error() {
        let error = ConfigError::MissingFunction;
        let message = format!("{error}");
        assert!(message.contains("can only be used on functions"));
        assert!(message.contains("💡 Hint"));
    }

    #[test]
    fn test_invalid_parameter_syntax_error() {
        let error = ConfigError::InvalidParameterSyntax {
            param: "bad_syntax".to_string(),
            expected: "param(name)".to_string(),
        };
        let message = format!("{error}");
        assert!(message.contains("bad_syntax"));
        assert!(message.contains("param(name)"));
        assert!(message.contains("💡 Hint"));
    }

    #[test]
    fn test_error_trait_implementation() {
        let error = ConfigError::MissingFunction;
        let _: &dyn std::error::Error = &error;
        // This test ensures that ConfigError implements std::error::Error
    }

    #[test]
    fn test_from_config_error_to_syn_error() {
        let config_error = ConfigError::MissingFunction;
        let syn_error: syn::Error = config_error.into();
        let message = syn_error.to_string();
        assert!(message.contains("can only be used on functions"));
    }

    #[test]
    fn test_clone_and_debug() {
        let error = ConfigError::AlreadySet("test");
        let cloned = error.clone();

        // Test Debug implementation
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("AlreadySet"));

        // Test that clone works
        assert!(matches!(cloned, ConfigError::AlreadySet("test")));
    }
}
