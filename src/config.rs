use log::Level;
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::{Block, FnArg, ItemFn, MetaList};
use syn::{punctuated::Punctuated, token::Comma, Ident, Meta};
use syn::{Pat, PatIdent, PatType};

use crate::output::Output;

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
    pub func_params_for_output: Vec<Ident>,
    pub func_params_for_invoke: Vec<Ident>,
    pub func_params_for_declare: Punctuated<FnArg, Comma>,
    pub func_output: syn::ReturnType,
}

impl Config {
    pub(crate) fn to_output(&self) -> Output {
        let Config {
            func_vis,
            func_block,
            func_name,
            func_params_for_output,
            func_params_for_invoke,
            func_params_for_declare,
            func_output,
            ..
        } = self;
        let inner_func_name = format_ident!("__{}__", func_name);
        let inner_func: proc_macro2::TokenStream = quote! {
            fn #inner_func_name(#func_params_for_declare) #func_output {
                #func_block
            }
        };
        let func_declare_start = quote! {
            #func_vis fn #func_name(#func_params_for_declare) #func_output
        };
        let func_declare_body = quote! {
            let output = #inner_func_name(#(#func_params_for_invoke,) *);
        };
        let func_declare_end = quote! {
            output
        };
        Output {
            inner_func,
            func_declare_start,
            func_declare_body,
            func_declare_end,
            func_output_start: quote! {},
            func_output_end: quote! {},
        }
    }
}