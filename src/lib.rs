use log::debug;
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, FnArg, ItemFn, PatType};

#[proc_macro_attribute]
pub fn funlog(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    // dbg!(&func);
    let func_vis = &func.vis; // pub
    let func_block = &func.block; // 函数主体实现部分{}

    let func_decl = &func.sig; // 函数申明
    let func_name = &func_decl.ident; // 函数名
    let func_name_str = func_name.to_string();
    let func_generics = &func_decl.generics; // 函数泛型
    let func_inputs: &Punctuated<FnArg, Comma> = &func_decl.inputs; // 函数输入参数
    let func_output = &func_decl.output; // 函数返回

    get_input(func_inputs);

    quote! {
        #func_vis fn #func_name() {
            std::println!("{} start", #func_name_str);
            #func_block
            std::println!("{} end", #func_name_str);
        }
    }
    .into()
}

fn get_input(func_inputs: &Punctuated<FnArg, Comma>) {
    for input in func_inputs.iter().filter_map(|arg| match arg {
        FnArg::Typed(PatType { pat, ty, .. }) => Some((pat, ty)),
        _ => None,
    }) {
        let (pat, ty) = input;
        if let syn::Pat::Ident(pat_ident) = pat.as_ref() {
            println!("Ident: {}", pat_ident.ident);
        }
        if let syn::Type::Path(type_path) = ty.as_ref() {
            if let Some(segment) = type_path.path.segments.last() {
                println!("Type Ident: {}", segment.ident);
            }
        }
    }
}
