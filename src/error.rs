use std::fmt;

#[derive(Debug, Clone)]
pub enum ConfigError {
    AlreadySet(&'static str),
    InvalidParameter { param: String, available: Vec<String> },
    InvalidAttribute { attr: String, suggestion: Option<String> },
    ParseError(String),
    ConflictingOptions { option1: String, option2: String },
    MissingFunction,
    InvalidParameterSyntax { param: String, expected: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::AlreadySet(field) => {
                write!(f, "funlog configuration error: '{field}' option has already been set\n")?;
                write!(f, "💡 Hint: Each configuration option can only be set once, please check for duplicate configurations")
            },
            ConfigError::InvalidParameter { param, available } => {
                write!(f, "funlog parameter error: parameter '{param}' does not exist\n")?;
                if available.is_empty() {
                    write!(f, "💡 Hint: This function has no parameters, please use 'none' or remove the params() configuration")
                } else {
                    write!(f, "💡 Hint: Available parameters are: {}\n", available.join(", "))?;
                    write!(f, "   Correct usage: #[funlog(params({}))]", available.join(", "))
                }
            },
            ConfigError::InvalidAttribute { attr, suggestion } => {
                write!(f, "funlog configuration error: unknown configuration option '{attr}'\n")?;
                if let Some(suggestion) = suggestion {
                    write!(f, "💡 Hint: Did you mean '{suggestion}'?\n")?;
                }
                write!(f, "📖 Available configuration options:\n")?;
                write!(f, "   Log levels: print, trace, debug, info, warn, error\n")?;
                write!(f, "   Parameter control: all, none, params(parameter_names...)\n")?;
                write!(f, "   Position control: onStart, onEnd, onStartEnd\n")?;
                write!(f, "   Return value: retVal")
            },
            ConfigError::ParseError(msg) => {
                write!(f, "funlog parse error: {msg}\n")?;
                write!(f, "💡 Hint: Please check if the macro syntax is correct, example: #[funlog(debug, all)]")
            },
            ConfigError::ConflictingOptions { option1, option2 } => {
                write!(f, "funlog configuration conflict: '{option1}' and '{option2}' cannot be used together\n")?;
                write!(f, "💡 Hint: Please choose one of the options")
            },
            ConfigError::MissingFunction => {
                write!(f, "funlog error: can only be used on functions\n")?;
                write!(f, "💡 Hint: funlog macro can only be applied to function definitions, not other items")
            },
            ConfigError::InvalidParameterSyntax { param, expected } => {
                write!(f, "funlog parameter syntax error: '{param}' format is incorrect\n")?;
                write!(f, "💡 Hint: Expected format is {expected}")
            },
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<ConfigError> for syn::Error {
    fn from(err: ConfigError) -> Self {
        syn::Error::new(proc_macro2::Span::call_site(), err.to_string())
    }
}