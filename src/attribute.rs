use log::Level;
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Error, MacroDelimiter, MetaList, Result};
use syn::{punctuated::Punctuated, token::Comma, Ident, Meta};

#[derive(Debug)]
pub struct Attri {
    parameter: ParameterEnum,
    log_level: Option<Level>,
}

#[derive(Debug)]
pub enum ParameterEnum {
    NoneParameter,
    AllParameters,
    Specified(Vec<String>),
}

impl Attri {
    pub fn from(meta_list: Punctuated<Meta, Comma>) -> Self {
        let mut parameter = ParameterEnum::NoneParameter;
        let mut log_level = None;

        // add a state machine to handle the state of the attribute
        // 1. check if the attribute is a Parameters
        // 2. check if the attribute is a delimiter of parenthesis
        // 3. check if the attribute is tokenstream

        for meta in meta_list.iter() {
            match meta {
                Meta::Path(path) => {
                    if path.is_ident("all") {
                        parameter = ParameterEnum::AllParameters;
                    } else if path.is_ident("none") {
                        parameter = ParameterEnum::NoneParameter;
                    } else if path.is_ident("params") {

                        
                    } else {
                        panic!("Invalid attribute at path {:?}", path.span());
                    }
                },
                Meta::List(MetaList{path, tokens, ..}) => {
                    if path.is_ident("params") {
                        let parser = Punctuated::<Ident, Comma>::parse_terminated;
                        let idents = parser.parse2(tokens.clone()).expect("Failed to parse params");
                        let params = idents.into_iter().map(|ident| ident.to_string()).collect();
                        dbg!(&params);
                        parameter = ParameterEnum::Specified(params);
                    }
                }
                _ => {
                    panic!("Invalid attribute at meta {:?}", meta.span());
                },
            }
        }

        Attri {
            parameter,
            log_level,
        }
    }
}
