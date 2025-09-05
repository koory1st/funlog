use syn::parse::Parser;
use syn::{punctuated::Punctuated, token::Comma, Ident, Meta};
use syn::{Block, FnArg, MetaList, ReturnType, Visibility};
use syn::{Pat, PatIdent, PatType};

use crate::config::{Config, OutputPosition, OutputType};
use crate::error::ConfigError;
use crate::generics_item_fn::GenericsFn;

#[derive(Debug)]
pub enum ParameterEnum {
    NoneParameter,
    AllParameters,
    Specified,
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    output_position: Option<OutputPosition>,
    param_config: Option<ParameterEnum>,
    output_ret_value: Option<bool>,
    output_type: Option<OutputType>,
    func_vis: Option<Visibility>,
    func_block: Option<Block>,
    func_name: Option<Ident>,
    func_params_for_output: Vec<Ident>,
    func_params_for_invoke: Vec<Ident>,
    func_params_for_declare: Punctuated<FnArg, Comma>,
    func_return_type: Option<ReturnType>,
}

impl ConfigBuilder {
    /// Sets the parameter configuration for the function logging.
    /// 
    /// # Arguments
    /// 
    /// * `param_config` - The parameter configuration enum specifying how parameters should be handled
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `ConfigError` if there's an issue
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlog::config_builder::{ConfigBuilder, ParameterEnum};
    /// 
    /// let mut builder = ConfigBuilder::default();
    /// assert!(builder.param_config(ParameterEnum::AllParameters).is_ok());
    /// ```
    pub fn param_config(&mut self, param_config: ParameterEnum) -> Result<(), ConfigError> {
        // Don't check conflicts here, check them uniformly in parse_meta_list
        self.param_config = Some(param_config);
        Ok(())
    }

    /// Sets the output type for logging (print, debug, info, etc.).
    /// 
    /// # Arguments
    /// 
    /// * `output_type` - The type of output to use for logging
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `ConfigError` if there's an issue
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlog::config_builder::ConfigBuilder;
    /// use funlog::config::OutputType;
    /// 
    /// let mut builder = ConfigBuilder::default();
    /// assert!(builder.output_type(OutputType::Debug).is_ok());
    /// ```
    pub fn output_type(&mut self, output_type: OutputType) -> Result<(), ConfigError> {
        // Don't check conflicts here, check them uniformly in parse_meta_list
        self.output_type = Some(output_type);
        Ok(())
    }

    /// Sets the position where logging output should occur.
    /// 
    /// # Arguments
    /// 
    /// * `output_position` - The position for logging (OnStart, OnEnd, or OnStartAndEnd)
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `ConfigError` if there's an issue
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlog::config_builder::ConfigBuilder;
    /// use funlog::config::OutputPosition;
    /// 
    /// let mut builder = ConfigBuilder::default();
    /// assert!(builder.output_position(OutputPosition::OnStart).is_ok());
    /// ```
    pub fn output_position(&mut self, output_position: OutputPosition) -> Result<(), ConfigError> {
        // Don't check conflicts here, check them uniformly in parse_meta_list
        self.output_position = Some(output_position);
        Ok(())
    }

    /// Sets whether to output the return value in logging.
    /// 
    /// # Arguments
    /// 
    /// * `output_ret_value` - Whether to include return value in the log output
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `ConfigError::AlreadySet` if already configured
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlog::config_builder::ConfigBuilder;
    /// 
    /// let mut builder = ConfigBuilder::default();
    /// assert!(builder.output_ret_value(true).is_ok());
    /// // Setting it again should fail
    /// assert!(builder.output_ret_value(false).is_err());
    /// ```
    pub fn output_ret_value(&mut self, output_ret_value: bool) -> Result<(), ConfigError> {
        if self.output_ret_value.is_some() {
            return Err(ConfigError::AlreadySet("return value configuration"));
        }
        self.output_ret_value = Some(output_ret_value);
        Ok(())
    }
    /// Builds the final configuration from the builder.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(Config)` with the built configuration, or `ConfigError::MissingFunction` 
    /// if required function fields are missing
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlog::config_builder::ConfigBuilder;
    /// use syn::{parse_quote, ItemFn};
    /// use funlog::generics_item_fn::GenericsFn;
    /// 
    /// let func: ItemFn = parse_quote! {
    ///     fn test_func(x: i32) -> i32 { x + 1 }
    /// };
    /// let mut builder = ConfigBuilder::default();
    /// builder.set_function_fields(GenericsFn::from(func));
    /// assert!(builder.build().is_ok());
    /// ```
    pub fn build(self) -> Result<Config, ConfigError> {
        let func_vis = self.func_vis.ok_or(ConfigError::MissingFunction)?;
        let func_block = self.func_block.ok_or(ConfigError::MissingFunction)?;
        let func_name = self.func_name.ok_or(ConfigError::MissingFunction)?;
        let func_return_type = self.func_return_type.ok_or(ConfigError::MissingFunction)?;

        let func_params_for_output = match self.param_config {
            Some(ParameterEnum::AllParameters) => self.func_params_for_invoke.clone(),
            Some(ParameterEnum::NoneParameter) => Vec::new(),
            Some(ParameterEnum::Specified) => self.func_params_for_output,
            None => self.func_params_for_invoke.clone(), // Default to all parameters
        };

        Ok(Config {
            output_position: self.output_position.unwrap_or(OutputPosition::OnStartAndEnd),
            output_type: self.output_type.unwrap_or(OutputType::Print),
            output_ret_value: self.output_ret_value.unwrap_or(false),
            func_vis,
            func_block,
            func_name,
            func_params_for_output,
            func_params_for_invoke: self.func_params_for_invoke,
            func_params_for_declare: self.func_params_for_declare,
            func_return_type,
        })
    }

    /// Creates a ConfigBuilder from metadata and function information.
    /// 
    /// # Arguments
    /// 
    /// * `meta_list` - Punctuated list of metadata from the macro attributes
    /// * `func` - Generic function information extracted from the ItemFn
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(ConfigBuilder)` on success, or `ConfigError` if parsing fails
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlog::config_builder::ConfigBuilder;
    /// use syn::{parse_quote, ItemFn, Meta, punctuated::Punctuated, token::Comma};
    /// use funlog::generics_item_fn::GenericsFn;
    /// 
    /// let func: ItemFn = parse_quote! {
    ///     fn test_func(x: i32) -> i32 { x + 1 }
    /// };
    /// let meta_list: Punctuated<Meta, Comma> = Punctuated::new();
    /// let result = ConfigBuilder::from(meta_list, GenericsFn::from(func));
    /// assert!(result.is_ok());
    /// ```
    pub fn from(meta_list: Punctuated<Meta, Comma>, func: GenericsFn) -> Result<Self, ConfigError> {
        let mut builder = ConfigBuilder::default();
        builder.set_function_fields(func);
        builder.parse_meta_list(meta_list)?;
        Ok(builder)
    }

    /// Sets the function-related fields from the GenericsFn.
    /// 
    /// # Arguments
    /// 
    /// * `func` - The generic function information to extract fields from
    fn set_function_fields(&mut self, func: GenericsFn) {
        self.func_vis = Some(func.vis);
        self.func_block = Some(func.block);
        let func_decl = func.sig;
        self.func_name = Some(func_decl.ident);
        self.set_parameters(&func_decl.inputs);
        self.func_params_for_declare = func_decl.inputs;
        self.func_return_type = Some(func_decl.output);
    }

    /// Extracts parameter identifiers from function arguments.
    /// 
    /// # Arguments
    /// 
    /// * `inputs` - The function arguments to extract parameter names from
    fn set_parameters(&mut self, inputs: &Punctuated<FnArg, Comma>) {
        for input in inputs.iter().filter_map(|arg| match arg {
            FnArg::Typed(PatType { pat, .. }) => Some(pat),
            _ => None,
        }) {
            if let Pat::Ident(PatIdent { ident, .. }) = input.as_ref() {
                self.func_params_for_invoke.push(ident.clone());
            }
        }
    }
    /// Parses the metadata list from macro attributes and configures the builder.
    /// 
    /// # Arguments
    /// 
    /// * `meta_list` - The punctuated list of metadata to parse
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or various `ConfigError` types for different parsing failures
    fn parse_meta_list(&mut self, meta_list: Punctuated<Meta, Comma>) -> Result<(), ConfigError> {
        // Check for conflicting parameter configurations
        let mut param_configs = Vec::new();
        let mut log_levels = Vec::new();
        let mut positions = Vec::new();
        
        for meta in meta_list.iter() {
            match meta {
                Meta::Path(path) => {
                    let ident_str = path.get_ident().map_or("unknown".to_string(), |i| i.to_string());
                    
                    match ident_str.as_str() {
                        "all" => {
                            param_configs.push("all");
                            self.param_config(ParameterEnum::AllParameters)?;
                        },
                        "none" => {
                            param_configs.push("none");
                            self.param_config(ParameterEnum::NoneParameter)?;
                        },
                        "print" => {
                            log_levels.push("print");
                            self.output_type(OutputType::Print)?;
                        },
                        "trace" => {
                            log_levels.push("trace");
                            self.output_type(OutputType::Trace)?;
                        },
                        "debug" => {
                            log_levels.push("debug");
                            self.output_type(OutputType::Debug)?;
                        },
                        "info" => {
                            log_levels.push("info");
                            self.output_type(OutputType::Info)?;
                        },
                        "warn" => {
                            log_levels.push("warn");
                            self.output_type(OutputType::Warn)?;
                        },
                        "error" => {
                            log_levels.push("error");
                            self.output_type(OutputType::Error)?;
                        },
                        "onStart" => {
                            positions.push("onStart");
                            self.output_position(OutputPosition::OnStart)?;
                        },
                        "onEnd" => {
                            positions.push("onEnd");
                            self.output_position(OutputPosition::OnEnd)?;
                        },
                        "onStartEnd" => {
                            positions.push("onStartEnd");
                            self.output_position(OutputPosition::OnStartAndEnd)?;
                        },
                        "retVal" => {
                            self.output_ret_value(true)?;
                        },
                        _ => {
                            let suggestion = self.suggest_similar_attribute(&ident_str);
                            return Err(ConfigError::InvalidAttribute { 
                                attr: ident_str, 
                                suggestion 
                            });
                        }
                    }
                }
                Meta::List(MetaList { path, tokens, .. }) => {
                    let list_name = path.get_ident().map_or("unknown".to_string(), |i| i.to_string());
                    
                    if path.is_ident("params") {
                        param_configs.push("params");
                        
                        let parser = Punctuated::<Ident, Comma>::parse_terminated;
                        let idents = parser.parse2(tokens.clone())
                            .map_err(|e| ConfigError::ParseError(format!("Parameter list parsing failed: {e}\n💡 Correct format: params(param1, param2)")))?;
                        
                        let available_params: Vec<String> = self.func_params_for_invoke.iter().map(|i| i.to_string()).collect();
                        
                        let params = idents
                            .into_iter()
                            .map(|ident| {
                                if self.func_params_for_invoke.contains(&ident) {
                                    Ok(ident)
                                } else {
                                    Err(ConfigError::InvalidParameter {
                                        param: ident.to_string(),
                                        available: available_params.clone(),
                                    })
                                }
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        
                        self.param_config(ParameterEnum::Specified)?;
                        self.func_params_for_output = params;
                    } else {
                        let suggestion = if list_name == "param" { Some("params".to_string()) } else { None };
                        return Err(ConfigError::InvalidAttribute { 
                            attr: format!("{}(...)", list_name), 
                            suggestion 
                        });
                    }
                }
                _ => {
                    return Err(ConfigError::InvalidAttribute { 
                        attr: "complex attribute format".to_string(), 
                        suggestion: Some("use simple identifiers or params(parameter_name) format".to_string()) 
                    });
                }
            }
        }
        
        // Check for conflicting configurations
        if param_configs.len() > 1 {
            return Err(ConfigError::ConflictingOptions {
                option1: param_configs[0].to_string(),
                option2: param_configs[1].to_string(),
            });
        }
        
        if log_levels.len() > 1 {
            return Err(ConfigError::ConflictingOptions {
                option1: log_levels[0].to_string(),
                option2: log_levels[1].to_string(),
            });
        }
        
        if positions.len() > 1 {
            return Err(ConfigError::ConflictingOptions {
                option1: positions[0].to_string(),
                option2: positions[1].to_string(),
            });
        }
        
        Ok(())
    }
    
    /// Provides suggestions for misspelled attributes using similarity matching.
    /// 
    /// # Arguments
    /// 
    /// * `input` - The input attribute string to find suggestions for
    /// 
    /// # Returns
    /// 
    /// Returns `Some(String)` with a suggested attribute, or `None` if no similar attribute found
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlog::config_builder::ConfigBuilder;
    /// 
    /// let builder = ConfigBuilder::default();
    /// assert_eq!(builder.suggest_similar_attribute("debg"), Some("debug".to_string()));
    /// assert_eq!(builder.suggest_similar_attribute("xyz"), None);
    /// ```
    fn suggest_similar_attribute(&self, input: &str) -> Option<String> {
        let valid_attributes = [
            "all", "none", "print", "trace", "debug", "info", "warn", "error",
            "onStart", "onEnd", "onStartEnd", "retVal", "params"
        ];
        
        // Simple similarity matching
        for attr in &valid_attributes {
            if self.is_similar(input, attr) {
                return Some(attr.to_string());
            }
        }
        
        None
    }
    
    /// Performs a simple string similarity check using substring matching and edit distance.
    /// 
    /// # Arguments
    /// 
    /// * `a` - First string to compare
    /// * `b` - Second string to compare
    /// 
    /// # Returns
    /// 
    /// Returns `true` if strings are considered similar, `false` otherwise
    fn is_similar(&self, a: &str, b: &str) -> bool {
        let a_lower = a.to_lowercase();
        let b_lower = b.to_lowercase();
        
        // Check if they contain the same substring
        if a_lower.contains(&b_lower) || b_lower.contains(&a_lower) {
            return true;
        }
        
        // Check edit distance
        self.levenshtein_distance(&a_lower, &b_lower) <= 2
    }
    
    /// Calculates the Levenshtein edit distance between two strings.
    /// 
    /// # Arguments
    /// 
    /// * `a` - First string
    /// * `b` - Second string
    /// 
    /// # Returns
    /// 
    /// Returns the minimum number of single-character edits required to transform one string into another
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlog::config_builder::ConfigBuilder;
    /// 
    /// let builder = ConfigBuilder::default();
    /// assert_eq!(builder.levenshtein_distance("debug", "debg"), 1);
    /// assert_eq!(builder.levenshtein_distance("hello", "world"), 4);
    /// ```
    fn levenshtein_distance(&self, a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let a_len = a_chars.len();
        let b_len = b_chars.len();
        
        if a_len == 0 { return b_len; }
        if b_len == 0 { return a_len; }
        
        let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];
        
        for i in 0..=a_len {
            matrix[i][0] = i;
        }
        for j in 0..=b_len {
            matrix[0][j] = j;
        }
        
        for i in 1..=a_len {
            for j in 1..=b_len {
                let cost = if a_chars[i-1] == b_chars[j-1] { 0 } else { 1 };
                matrix[i][j] = std::cmp::min(
                    std::cmp::min(
                        matrix[i-1][j] + 1,      // deletion
                        matrix[i][j-1] + 1       // insertion
                    ),
                    matrix[i-1][j-1] + cost      // substitution
                );
            }
        }
        
        matrix[a_len][b_len]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, ItemFn};

    fn create_test_function() -> GenericsFn {
        let func: ItemFn = parse_quote! {
            pub fn test_func(x: i32, y: String) -> i32 {
                x + 1
            }
        };
        GenericsFn::from(func)
    }

    #[test]
    fn test_param_config() {
        let mut builder = ConfigBuilder::default();
        
        // Test setting parameter config
        assert!(builder.param_config(ParameterEnum::AllParameters).is_ok());
        assert!(matches!(builder.param_config, Some(ParameterEnum::AllParameters)));
        
        // Test overwriting (should be allowed)
        assert!(builder.param_config(ParameterEnum::NoneParameter).is_ok());
        assert!(matches!(builder.param_config, Some(ParameterEnum::NoneParameter)));
    }

    #[test]
    fn test_output_type() {
        let mut builder = ConfigBuilder::default();
        
        // Test setting output type
        assert!(builder.output_type(OutputType::Debug).is_ok());
        assert!(matches!(builder.output_type, Some(OutputType::Debug)));
        
        // Test overwriting (should be allowed)
        assert!(builder.output_type(OutputType::Info).is_ok());
        assert!(matches!(builder.output_type, Some(OutputType::Info)));
    }

    #[test]
    fn test_output_position() {
        let mut builder = ConfigBuilder::default();
        
        // Test setting output position
        assert!(builder.output_position(OutputPosition::OnStart).is_ok());
        assert!(matches!(builder.output_position, Some(OutputPosition::OnStart)));
        
        // Test overwriting (should be allowed)
        assert!(builder.output_position(OutputPosition::OnEnd).is_ok());
        assert!(matches!(builder.output_position, Some(OutputPosition::OnEnd)));
    }

    #[test]
    fn test_output_ret_value() {
        let mut builder = ConfigBuilder::default();
        
        // Test setting return value config
        assert!(builder.output_ret_value(true).is_ok());
        assert_eq!(builder.output_ret_value, Some(true));
        
        // Test setting again should fail
        let result = builder.output_ret_value(false);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::AlreadySet(_)));
    }

    #[test]
    fn test_build_success() {
        let mut builder = ConfigBuilder::default();
        let func = create_test_function();
        builder.set_function_fields(func);
        
        let config = builder.build();
        assert!(config.is_ok());
        
        let config = config.unwrap();
        assert_eq!(config.func_name.to_string(), "test_func");
        assert_eq!(config.func_params_for_invoke.len(), 2);
    }

    #[test]
    fn test_build_missing_function() {
        let builder = ConfigBuilder::default();
        
        let result = builder.build();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::MissingFunction));
    }

    #[test]
    fn test_from_success() {
        let func = create_test_function();
        let meta_list: Punctuated<Meta, Comma> = Punctuated::new();
        
        let result = ConfigBuilder::from(meta_list, func);
        assert!(result.is_ok());
        
        let builder = result.unwrap();
        assert!(builder.func_name.is_some());
        assert_eq!(builder.func_params_for_invoke.len(), 2);
    }

    #[test]
    fn test_set_function_fields() {
        let mut builder = ConfigBuilder::default();
        let func = create_test_function();
        
        builder.set_function_fields(func);
        
        assert!(builder.func_name.is_some());
        assert_eq!(builder.func_name.as_ref().unwrap().to_string(), "test_func");
        assert_eq!(builder.func_params_for_invoke.len(), 2);
        assert_eq!(builder.func_params_for_invoke[0].to_string(), "x");
        assert_eq!(builder.func_params_for_invoke[1].to_string(), "y");
    }

    #[test]
    fn test_set_parameters() {
        let mut builder = ConfigBuilder::default();
        let func: ItemFn = parse_quote! {
            fn test(a: i32, b: String, c: bool) {}
        };
        
        builder.set_parameters(&func.sig.inputs);
        
        assert_eq!(builder.func_params_for_invoke.len(), 3);
        assert_eq!(builder.func_params_for_invoke[0].to_string(), "a");
        assert_eq!(builder.func_params_for_invoke[1].to_string(), "b");
        assert_eq!(builder.func_params_for_invoke[2].to_string(), "c");
    }

    #[test]
    fn test_suggest_similar_attribute() {
        let builder = ConfigBuilder::default();
        
        // Test exact matches
        assert_eq!(builder.suggest_similar_attribute("debug"), Some("debug".to_string()));
        
        // Test similar matches
        assert_eq!(builder.suggest_similar_attribute("debg"), Some("debug".to_string()));
        assert_eq!(builder.suggest_similar_attribute("infoo"), Some("info".to_string()));
        assert_eq!(builder.suggest_similar_attribute("al"), Some("all".to_string()));
        
        // Test no matches
        assert_eq!(builder.suggest_similar_attribute("xyz"), None);
        assert_eq!(builder.suggest_similar_attribute("completely_different"), None);
    }

    #[test]
    fn test_is_similar() {
        let builder = ConfigBuilder::default();
        
        // Test substring matching
        assert!(builder.is_similar("debug", "debug"));
        assert!(builder.is_similar("debug", "deb"));
        assert!(builder.is_similar("info", "inf"));
        
        // Test edit distance
        assert!(builder.is_similar("debug", "debg"));
        assert!(builder.is_similar("trace", "trac"));
        
        // Test case insensitive
        assert!(builder.is_similar("DEBUG", "debug"));
        assert!(builder.is_similar("Info", "info"));
        
        // Test not similar
        assert!(!builder.is_similar("debug", "xyz"));
        assert!(!builder.is_similar("completely", "different"));
    }

    #[test]
    fn test_levenshtein_distance() {
        let builder = ConfigBuilder::default();
        
        // Test identical strings
        assert_eq!(builder.levenshtein_distance("hello", "hello"), 0);
        
        // Test single character differences
        assert_eq!(builder.levenshtein_distance("debug", "debg"), 1);
        assert_eq!(builder.levenshtein_distance("info", "infoo"), 1);
        
        // Test multiple character differences
        assert_eq!(builder.levenshtein_distance("hello", "world"), 4);
        
        // Test empty strings
        assert_eq!(builder.levenshtein_distance("", "hello"), 5);
        assert_eq!(builder.levenshtein_distance("hello", ""), 5);
        assert_eq!(builder.levenshtein_distance("", ""), 0);
    }

    #[test]
    fn test_parse_meta_list_conflicting_params() {
        let mut builder = ConfigBuilder::default();
        let func = create_test_function();
        builder.set_function_fields(func);
        
        // Create conflicting parameter configurations
        let meta_list: Punctuated<Meta, Comma> = parse_quote! { all, none };
        
        let result = builder.parse_meta_list(meta_list);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::ConflictingOptions { .. }));
    }

    #[test]
    fn test_parse_meta_list_conflicting_log_levels() {
        let mut builder = ConfigBuilder::default();
        let func = create_test_function();
        builder.set_function_fields(func);
        
        // Create conflicting log level configurations
        let meta_list: Punctuated<Meta, Comma> = parse_quote! { debug, info };
        
        let result = builder.parse_meta_list(meta_list);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::ConflictingOptions { .. }));
    }

    #[test]
    fn test_parse_meta_list_invalid_attribute() {
        let mut builder = ConfigBuilder::default();
        let func = create_test_function();
        builder.set_function_fields(func);
        
        // Create invalid attribute
        let meta_list: Punctuated<Meta, Comma> = parse_quote! { invalid_attr };
        
        let result = builder.parse_meta_list(meta_list);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::InvalidAttribute { .. }));
    }
}
