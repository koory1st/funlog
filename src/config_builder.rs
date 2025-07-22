use syn::parse::Parser;
use syn::{punctuated::Punctuated, token::Comma, Ident, Meta};
use syn::{Block, FnArg, MetaList, ReturnType, Visibility};
use syn::{Pat, PatIdent, PatType};

use crate::config::{Config, OutputPosition, OutputType};
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
    pub fn param_config(&mut self, param_config: ParameterEnum) {
        if let Some(v) = &self.param_config {
            panic!("Parameter config already set: {:?}", v);
        }
        self.param_config = Some(param_config);
    }

    pub fn output_type(&mut self, output_type: OutputType) {
        if let Some(v) = &self.output_type {
            panic!("Output type already set: {:?}", v);
        }
        self.output_type = Some(output_type);
    }

    pub fn output_position(&mut self, output_position: OutputPosition) {
        if let Some(v) = &self.output_position {
            panic!("Output position already set: {:?}", v);
        }
        self.output_position = Some(output_position);
    }
    pub fn output_ret_value(&mut self, output_ret_value: bool) {
        if let Some(v) = &self.output_ret_value {
            panic!("Output return value already set: {:?}", v);
        }
        self.output_ret_value = Some(output_ret_value);
    }
    pub fn build(self) -> Config {
        if let Some(ParameterEnum::AllParameters) = self.param_config {
        }
        Config {
            output_position: self
                .output_position
                .unwrap_or(OutputPosition::OnStartAndEnd),
            output_type: self.output_type.unwrap_or(OutputType::Print),
            output_ret_value: self.output_ret_value.unwrap_or(false),
            func_vis: self.func_vis.unwrap(),
            func_block: self.func_block.unwrap(),
            func_name: self.func_name.unwrap(),
            func_params_for_output: match self.param_config {
                Some(ParameterEnum::AllParameters) => self.func_params_for_invoke.clone(),
                _ => self.func_params_for_output,
            },
            func_params_for_invoke: self.func_params_for_invoke,
            func_params_for_declare: self.func_params_for_declare,
            func_return_type: self.func_return_type.unwrap(),
        }
    }

    pub fn from(meta_list: Punctuated<Meta, Comma>, func: GenericsFn) -> Result<Self, syn::Error> {
        let mut builder = ConfigBuilder::default();
        builder.set_function_fields(func);
        if let Err(e) = builder.parse_meta_list(meta_list) {
            return Err(e);
        }
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
    fn parse_meta_list(&mut self, meta_list: Punctuated<Meta, Comma>) -> Result<(), syn::Error> {
        for meta in meta_list.iter() {
            match meta {
                Meta::Path(path) => {
                    if path.is_ident("all") {
                        self.param_config(ParameterEnum::AllParameters);
                    } else if path.is_ident("none") {
                        self.param_config(ParameterEnum::NoneParameter);
                        self.func_params_for_invoke.clear();
                    } else if path.is_ident("print") {
                        self.output_type(OutputType::Print);
                    } else if path.is_ident("trace") {
                        self.output_type(OutputType::Trace);
                    } else if path.is_ident("debug") {
                        self.output_type(OutputType::Debug);
                    } else if path.is_ident("info") {
                        self.output_type(OutputType::Info);
                    } else if path.is_ident("warn") {
                        self.output_type(OutputType::Warn);
                    } else if path.is_ident("error") {
                        self.output_type(OutputType::Error);
                    } else if path.is_ident("onStart") {
                        self.output_position(OutputPosition::OnStart);
                    } else if path.is_ident("onEnd") {
                        self.output_position(OutputPosition::OnEnd);
                    } else if path.is_ident("onStartEnd") {
                        self.output_position(OutputPosition::OnStartAndEnd);
                    } else if path.is_ident("retVal") {
                        self.output_ret_value(true);
                    } else {
                        return Err(syn::Error::new_spanned(path, "Invalid attribute at path"));
                    }
                }
                Meta::List(MetaList { path, tokens, .. }) => {
                    if path.is_ident("params") {
                        let parser = Punctuated::<Ident, Comma>::parse_terminated;
                        let idents = parser.parse2(tokens.clone()).map_err(|e| {
                            syn::Error::new_spanned(
                                tokens,
                                format!("Failed to parse params: {}", e),
                            )
                        })?;
                        let params = idents
                            .into_iter()
                            .map(|ident| {
                                if self.func_params_for_invoke.contains(&ident) {
                                    Ok(ident)
                                } else {
                                    Err(syn::Error::new_spanned(
                                        &ident,
                                        format!(
                                            "Invalid parameter: {}, valid parameters: {:?}",
                                            ident, self.func_params_for_invoke
                                        ),
                                    ))
                                }
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        self.param_config(ParameterEnum::Specified);
                        self.func_params_for_output = params;
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(meta, "Invalid attribute at meta"));
                }
            }
        }
        Ok(())
    }
}
