use log::debug;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, FnArg, ItemFn, Pat, PatIdent, PatType};

#[proc_macro_attribute]
pub fn funlog(attr: TokenStream, item: TokenStream) -> TokenStream {
    let is_debug = if cfg!(debug_assertions) {
        true
    } else {
        false
    };

    // when not debug, just return the original function
    if !is_debug {
        return item;
    }

    let func = parse_macro_input!(item as ItemFn);
    // dbg!(&func);
    let func_vis = &func.vis; // pub
    let func_block = &func.block; // 函数主体实现部分{}

    let func_decl = &func.sig; // 函数申明
    let func_name = &func_decl.ident; // 函数名
    let func_name_str = func_name.to_string();
    let inner_func_name = format_ident!("__{}__", func_name);
    let func_generics = &func_decl.generics; // 函数泛型
    let func_inputs: &Punctuated<FnArg, Comma> = &func_decl.inputs; // 函数输入参数
    let func_output = &func_decl.output; // 函数返回

    let args = get_args(func_inputs);

    quote! {
        fn #inner_func_name(#func_inputs) #func_output {
            #func_block
        }
        #func_vis fn #func_name(#func_inputs) #func_output {
            std::println!("{} start", #func_name_str);
            let output = #inner_func_name(#(#args,) *);
            std::println!("{} end", #func_name_str);
        }
    }
    .into()
}

fn get_args(func_inputs: &Punctuated<FnArg, Comma>) -> Vec<Ident> {
    let mut args = Vec::new();
    for input in func_inputs.iter().filter_map(|arg| match arg {
        FnArg::Typed(PatType { pat, ty, .. }) => Some((pat, ty)),
        _ => None,
    }) {
        let (pat, ty) = input;
        args.push(match pat.as_ref() {
            Pat::Ident(PatIdent { ident, .. }) => ident.clone(),
            _ => panic!("no ident"),
        });
    }
    args
}
