mod config;
mod config_builder;
mod output;

use config_builder::ConfigBuilder;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Meta;
use syn::{parse_macro_input, Attribute, FnArg, ItemFn, Pat, PatIdent, PatType};

#[proc_macro_attribute]
pub fn funlog(args: TokenStream, item: TokenStream) -> TokenStream {
    let is_debug = if cfg!(debug_assertions) { true } else { false };

    // when not debug, just return the original function
    if !is_debug {
        return item;
    }

    let func = parse_macro_input!(item as ItemFn);
    let attr_meta: Punctuated<Meta, Comma> = parse_macro_input!(args with Punctuated::<Meta, Comma>::parse_terminated);
    let config_builder = ConfigBuilder::from(attr_meta, func);
    let config = config_builder.build();
    let output = config.to_output();
    // dbg!(&config);

    // // dbg!(&func);
    // let func_vis = &func.vis; // pub
    // let func_block = &func.block; // 函数主体实现部分{}

    // let func_decl = &func.sig; // 函数申明
    // let func_name = &func_decl.ident; // 函数名
    // let func_name_str = func_name.to_string();
    // let inner_func_name = format_ident!("__{}__", func_name);
    // let func_generics = &func_decl.generics; // 函数泛型
    // let func_inputs: &Punctuated<FnArg, Comma> = &func_decl.inputs; // 函数输入参数
    // let func_output = &func_decl.output; // 函数返回

    // let args = get_args(func_inputs);
    // let end_token = output_end(func_output, &func_name_str);
    // let start_token = output_start(&args, &func_name_str);

    // quote! {
    //     fn #inner_func_name(#func_inputs) #func_output {
    //         #func_block
    //     }
    //     #func_vis fn #func_name(#func_inputs) #func_output {
    //         #start_token
    //         let output = #inner_func_name(#(#args,) *);
    //         #end_token
    //         output
    //     }
    // }
    // .into()
    output.into()
}

fn output_start(args: &Vec<Ident>, func_name_str: &String) -> TokenStream2 {
    let aaa = args
        .iter()
        .map(|arg| {
            let arg_str = arg.to_string();
            format!("{}:{{{}}}", arg_str, arg_str)
        })
        .collect::<Vec<String>>()
        .join(", ");
    quote! {
        log::debug!("{}({}) start", #func_name_str, #aaa);
    }
}

fn output_end(func_output: &syn::ReturnType, func_name_str: &str) -> TokenStream2 {
    match func_output {
        syn::ReturnType::Default => {
            quote! {
                log::debug!("{}() end", #func_name_str);
            }
        }
        _ => {
            quote! {
                log::debug!("{}() end -> {}", #func_name_str, output);
            }
        }
    }
}
