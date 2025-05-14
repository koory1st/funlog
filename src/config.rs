use log::Level;
use quote::format_ident;
use syn::parse::Parser;
use syn::{Block, FnArg, ItemFn, MetaList};
use syn::{punctuated::Punctuated, token::Comma, Ident, Meta};
use syn::{Pat, PatIdent, PatType};

#[derive(Debug)]
pub enum OutputType {
    OnStart,
    OnEnd,
    OnStartAndEnd,
}

#[derive(Debug)]
pub struct Config {
    pub output_type: OutputType,
    pub log_level: Level,
    pub func_vis: syn::Visibility,
    pub func_block: Box<Block>,
    pub func_name: syn::Ident,
    pub func_inputs: Vec<Ident>,
    pub func_inputs_ident: Punctuated<FnArg, Comma>,
    pub func_output: syn::ReturnType,
}

