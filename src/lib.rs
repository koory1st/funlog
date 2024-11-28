use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Meta, Lit, Expr, ExprLit, FnArg};
use syn::punctuated::Punctuated;
use syn::token::Comma;

/// Configuration for the funlog macro
#[derive(Default)]
struct FunlogConfig {
    param: bool,        // Whether to log parameters
    ret: bool,         // Whether to log return value
    param_limit: Option<usize>,  // Parameter output length limit
    ret_limit: Option<usize>,    // Return value output length limit
    gener: bool,       // Whether to show generic type parameters
    log_level: Option<String>, // Log level (info, debug, warn, error, trace)
}

impl FunlogConfig {
    fn from_meta(meta: Punctuated<Meta, Comma>) -> Self {
        let mut config = FunlogConfig::default();
        
        for meta_item in meta {
            match meta_item {
                Meta::Path(path) => {
                    if let Some(ident) = path.get_ident() {
                        let name = ident.to_string();
                        match name.as_str() {
                            "param" => config.param = true,
                            "ret" => config.ret = true,
                            "gener" => config.gener = true,
                            level @ ("info" | "debug" | "warn" | "error" | "trace") => {
                                config.log_level = Some(level.to_string());
                            }
                            _ => {}
                        }
                    }
                }
                Meta::NameValue(nv) => {
                    let ident = nv.path.get_ident().unwrap().to_string();
                    match ident.as_str() {
                        "param" => {
                            if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = nv.value {
                                if let Ok(val) = lit_str.value().parse() {
                                    config.param_limit = Some(val);
                                }
                            }
                        }
                        "ret" => {
                            if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = nv.value {
                                if let Ok(val) = lit_str.value().parse() {
                                    config.ret_limit = Some(val);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        config
    }
}

#[proc_macro_attribute]
pub fn funlog(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_meta = parse_macro_input!(attr with Punctuated::<Meta, Comma>::parse_terminated);
    let input = parse_macro_input!(item as ItemFn);
    
    let config = FunlogConfig::from_meta(attr_meta);
    let fn_name = &input.sig.ident;
    let fn_generics = &input.sig.generics;
    let fn_inputs = &input.sig.inputs;
    let fn_output = &input.sig.output;
    let fn_block = &input.block;
    let vis = &input.vis;
    
    // Generate parameter logging code
    let mut param_strings = Vec::new();
    let mut debug_bounds = Vec::new();
    
    for param in fn_inputs.iter() {
        if let FnArg::Typed(pat_type) = param {
            let param_name = &pat_type.pat;
            let param_ty = &pat_type.ty;
            
            let format_expr = if config.gener {
                quote! { format!("<{}>({:?})", std::any::type_name::<#param_ty>(), #param_name) }
            } else if config.param {
                quote! { format!("{:?}", #param_name) }
            } else {
                continue;
            };
            
            let param_str = if let Some(limit) = config.param_limit {
                let limit_val = limit;
                quote! {
                    {
                        let s = #format_expr;
                        if s.len() <= #limit_val {
                            s
                        } else {
                            format!("{}...", &s[..#limit_val])
                        }
                    }
                }
            } else {
                format_expr
            };
            
            param_strings.push(param_str);
            debug_bounds.push(quote! { #param_ty: std::fmt::Debug });
        }
    }
    
    // Determine which log macro to use
    let log_macro = match config.log_level.as_deref() {
        Some("debug") => quote! { log::debug },
        Some("warn") => quote! { log::warn },
        Some("error") => quote! { log::error },
        Some("trace") => quote! { log::trace },
        _ => quote! { log::info },
    };
    
    let param_logging = if config.param || config.gener {
        quote! {
            let params = vec![#(#param_strings),*].join(", ");
            let param_str = if params.is_empty() { "()".to_string() } else { format!("({})", params) };
        }
    } else {
        quote! {
            let param_str = "()".to_string();
        }
    };
    
    let return_logging = if config.ret {
        if let Some(limit) = config.ret_limit {
            quote! {
                let ret_debug = format!("{:?}", __result);
                let ret_str = if ret_debug.len() <= #limit {
                    format!("->{}", ret_debug)
                } else {
                    format!("->{}...", &ret_debug[..#limit])
                };
            }
        } else {
            quote! {
                let ret_str = format!("->{:?}", __result);
            }
        }
    } else {
        quote! {
            let ret_str = String::new();
        }
    };
    
    let expanded = quote! {
        #vis fn #fn_name #fn_generics(#fn_inputs) #fn_output 
        where
            #(#debug_bounds,)*
        {
            use std::any::type_name;
            
            // Log function entry
            let file = file!();
            let line = line!();
            
            #param_logging
            
            let begin_msg = format!("{}:{} {}::{}{} begin", 
                file, 
                line,
                module_path!(),
                stringify!(#fn_name),
                param_str
            );
            #log_macro!("{}", begin_msg);
            
            // Execute the original function
            let __result = #fn_block;
            
            // Log return value and function end
            #return_logging
            
            let end_msg = format!("{}:{} {}::{}{}{} end", 
                file, 
                line,
                module_path!(),
                stringify!(#fn_name),
                param_str,
                ret_str
            );
            #log_macro!("{}", end_msg);
            
            __result
        }
    };
    
    TokenStream::from(expanded)
}
