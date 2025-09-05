use std::fmt;

#[derive(Debug, Clone)]
pub enum ConfigError {
    AlreadySet(&'static str),
    InvalidParameter(String),
    InvalidAttribute(String),
    ParseError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::AlreadySet(field) => write!(f, "Configuration field '{field}' is already set"),
            ConfigError::InvalidParameter(param) => write!(f, "Invalid parameter: {param}"),
            ConfigError::InvalidAttribute(attr) => write!(f, "Invalid attribute: {attr}"),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {msg}"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<ConfigError> for syn::Error {
    fn from(err: ConfigError) -> Self {
        syn::Error::new(proc_macro2::Span::call_site(), err.to_string())
    }
}