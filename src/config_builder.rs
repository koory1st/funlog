use log::Level;
use syn::parse::Parser;
use syn::{Block, FnArg, ItemFn, MetaList};
use syn::{punctuated::Punctuated, token::Comma, Ident, Meta};
use syn::{Pat, PatIdent, PatType};

use crate::config::{Config, OutputType, ParameterEnum};

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    output_type: Option<OutputType>,
    param_config: Option<ParameterEnum>,
    log_level: Option<Level>,
    func_vis: Option<syn::Visibility>,
    func_block: Option<Box<Block>>,
    func_name: Option<syn::Ident>,
    func_inputs: Vec<String>,
    func_output: Option<syn::ReturnType>,
}

impl ConfigBuilder {
    pub fn param_config(&mut self, param_config: ParameterEnum) {
        if let Some(v) = &self.param_config {
            panic!("Parameter config already set: {:?}", v);
        }
        self.param_config = Some(param_config);
    }

    pub fn log_level(&mut self, log_level: Level) {
        if let Some(v) = &self.log_level {
            panic!("Log level already set: {:?}", v);
        }
        self.log_level = Some(log_level);
    }

    pub fn output_type(&mut self, output_type: OutputType) {
        if let Some(v) = &self.output_type {
            panic!("Output type already set: {:?}", v);
        }
        self.output_type = Some(output_type);
    }
    pub fn build(self) -> Config {
        Config {
            output_type: self.output_type.unwrap_or(OutputType::OnStartAndEnd),
            parameter: self.param_config.unwrap_or(ParameterEnum::AllParameters),
            log_level: self.log_level.unwrap_or(Level::Debug),
            func_vis: self.func_vis.unwrap(),
            func_block: self.func_block.unwrap(),
            func_name: self.func_name.unwrap(),
            func_inputs: self.func_inputs,
            func_output: self.func_output.unwrap(),
        }
    }

    pub fn from(meta_list: Punctuated<Meta, Comma>, func: ItemFn) -> Self {
        let mut builder = ConfigBuilder::default();
        builder.set_function_fields(func);
        builder.parse_meta_list(meta_list);
        builder
    }

    fn set_function_fields(&mut self, func: ItemFn) {
        self.func_vis = Some(func.vis);
        self.func_block = Some(func.block);
        let func_decl = func.sig;
        self.func_name = Some(func_decl.ident);
        self.set_parameters(func_decl.inputs);
        self.func_output = Some(func_decl.output);
    }

    fn set_parameters(&mut self, inputs: Punctuated<FnArg, Comma>) {
        for input in inputs.iter().filter_map(|arg| match arg {
            FnArg::Typed(PatType { pat, .. }) => Some(pat),
            _ => None,
        }) {
            if let Pat::Ident(PatIdent { ident, .. }) = input.as_ref() {
                self.func_inputs.push(ident.to_string());
            }
        }
    }
    fn parse_meta_list(&mut self, meta_list: Punctuated<Meta, Comma>) {
        for meta in meta_list.iter() {
            match meta {
                Meta::Path(path) => {
                    if path.is_ident("all") {
                        self.param_config(ParameterEnum::AllParameters);
                    } else if path.is_ident("none") {
                        self.param_config(ParameterEnum::NoneParameter);
                    } else if path.is_ident("trace") {
                        self.log_level(Level::Trace);
                    } else if path.is_ident("debug") {
                        self.log_level(Level::Debug);
                    } else if path.is_ident("info") {
                        self.log_level(Level::Info);
                    } else if path.is_ident("warn") {
                        self.log_level(Level::Warn);
                    } else if path.is_ident("error") {
                        self.log_level(Level::Error);
                    } else if path.is_ident("onStart") {
                        self.output_type(OutputType::OnStart);
                    } else if path.is_ident("onEnd") {
                        self.output_type(OutputType::OnEnd);
                    } else if path.is_ident("onStartEnd") {
                        self.output_type(OutputType::OnStartAndEnd);
                    } else {
                        panic!("Invalid attribute at path");
                    }
                },
                Meta::List(MetaList{path, tokens, ..}) => {
                    if path.is_ident("params") {
                        let parser = Punctuated::<Ident, Comma>::parse_terminated;
                        let idents = parser.parse2(tokens.clone()).expect("Failed to parse params");
                        let params = idents.into_iter().map(|ident| {
                            if self.func_inputs.contains(&ident.to_string()) {
                                ident.to_string()
                            } else {
                                panic!("Invalid parameter: {}, valid parameters: {:?}", ident, self.func_inputs);
                            }
                        }).collect();
                        self.param_config(ParameterEnum::Specified(params));
                    }
                }
                _ => {
                    panic!("Invalid attribute at meta");
                },
            }
        }
    }

}
