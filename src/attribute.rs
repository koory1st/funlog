use log::Level;
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Error, MacroDelimiter, MetaList, Result};
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
    log_level: Option<Level>,
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    param_config: Option<ParameterEnum>,
    log_level: Option<Level>,
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
}

impl ConfigBuilder {
    pub fn from(meta_list: Punctuated<Meta, Comma>) -> Self {
        let mut builder = ConfigBuilder::default();

        for meta in meta_list.iter() {
            match meta {
                Meta::Path(path) => {
                    if path.is_ident("all") {
                        builder.param_config(ParameterEnum::AllParameters);
                    } else if path.is_ident("none") {
                        builder.param_config(ParameterEnum::NoneParameter);
                    } else if path.is_ident("trace") {
                        builder.log_level(Level::Trace);
                    } else if path.is_ident("debug") {
                        builder.log_level(Level::Debug);
                    } else if path.is_ident("info") {
                        builder.log_level(Level::Info);
                    } else if path.is_ident("warn") {
                        builder.log_level(Level::Warn);
                    } else {
                        panic!("Invalid attribute at path");
                    }
                },
                Meta::List(MetaList{path, tokens, ..}) => {
                    if path.is_ident("params") {
                        let parser = Punctuated::<Ident, Comma>::parse_terminated;
                        let idents = parser.parse2(tokens.clone()).expect("Failed to parse params");
                        let params = idents.into_iter().map(|ident| ident.to_string()).collect();
                        builder.param_config(ParameterEnum::Specified(params));
                    }
                }
                _ => {
                    panic!("Invalid attribute at meta");
                },
            }
        }
        builder
    }
}
