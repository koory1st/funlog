use log::Level;
use quote::format_ident;
use syn::parse::Parser;
use syn::{Block, FnArg, ItemFn, MetaList};
use syn::{punctuated::Punctuated, token::Comma, Ident, Meta};

#[derive(Debug)]
pub enum ParameterEnum {
    NoneParameter,
    AllParameters,
    Specified(Vec<String>),
}

#[derive(Debug)]
pub struct Config {
    parameter: ParameterEnum,
    log_level: Level,
    func_vis: syn::Visibility,
    func_block: Box<Block>,
    func_name: syn::Ident,
    func_inputs: Punctuated<FnArg, Comma>,
    func_output: syn::ReturnType,
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    param_config: Option<ParameterEnum>,
    log_level: Option<Level>,
    func_vis: Option<syn::Visibility>,
    func_block: Option<Box<Block>>,
    func_name: Option<syn::Ident>,
    func_inputs: Option<Punctuated<FnArg, Comma>>,
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

    pub fn build(self) -> Config {
        Config {
            parameter: self.param_config.unwrap(),
            log_level: self.log_level.unwrap(),
            func_vis: self.func_vis.unwrap(),
            func_block: self.func_block.unwrap(),
            func_name: self.func_name.unwrap(),
            func_inputs: self.func_inputs.unwrap(),
            func_output: self.func_output.unwrap(),
        }
    }

    pub fn from(meta_list: Punctuated<Meta, Comma>, func: ItemFn) -> Self {
        let mut builder = ConfigBuilder::default();
        builder.parse_meta_list(meta_list);
        builder.set_function_fields(func);
        builder
    }

    fn set_function_fields(&mut self, func: ItemFn) {
        self.func_vis = Some(func.vis);
        self.func_block = Some(func.block);
        let func_decl = func.sig;
        self.func_name = Some(func_decl.ident);
        self.func_inputs = Some(func_decl.inputs);
        self.func_output = Some(func_decl.output);
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
                    } else {
                        panic!("Invalid attribute at path");
                    }
                },
                Meta::List(MetaList{path, tokens, ..}) => {
                    if path.is_ident("params") {
                        let parser = Punctuated::<Ident, Comma>::parse_terminated;
                        let idents = parser.parse2(tokens.clone()).expect("Failed to parse params");
                        let params = idents.into_iter().map(|ident| ident.to_string()).collect();
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
