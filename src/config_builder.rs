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
    pub fn param_config(&mut self, param_config: ParameterEnum) -> Result<(), ConfigError> {
        // Don't check conflicts here, check them uniformly in parse_meta_list
        self.param_config = Some(param_config);
        Ok(())
    }

    pub fn output_type(&mut self, output_type: OutputType) -> Result<(), ConfigError> {
        // Don't check conflicts here, check them uniformly in parse_meta_list
        self.output_type = Some(output_type);
        Ok(())
    }

    pub fn output_position(&mut self, output_position: OutputPosition) -> Result<(), ConfigError> {
        // Don't check conflicts here, check them uniformly in parse_meta_list
        self.output_position = Some(output_position);
        Ok(())
    }

    pub fn output_ret_value(&mut self, output_ret_value: bool) -> Result<(), ConfigError> {
        if self.output_ret_value.is_some() {
            return Err(ConfigError::AlreadySet("return value configuration"));
        }
        self.output_ret_value = Some(output_ret_value);
        Ok(())
    }
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

    pub fn from(meta_list: Punctuated<Meta, Comma>, func: GenericsFn) -> Result<Self, ConfigError> {
        let mut builder = ConfigBuilder::default();
        builder.set_function_fields(func);
        builder.parse_meta_list(meta_list)?;
        Ok(builder)
    }

    fn set_function_fields(&mut self, func: GenericsFn) {
        self.func_vis = Some(func.vis);
        self.func_block = Some(func.block);
        let func_decl = func.sig;
        self.func_name = Some(func_decl.ident);
        self.set_parameters(&func_decl.inputs);
        self.func_params_for_declare = func_decl.inputs;
        self.func_return_type = Some(func_decl.output);
    }

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
    
    /// Provide suggestions for misspelled attributes
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
    
    /// Simple string similarity check
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
    
    /// Calculate edit distance
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
