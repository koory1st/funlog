use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Meta, Lit, Expr, ExprLit, FnArg, PatType, ReturnType, Type, TypePath, Path, PathSegment};
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

fn is_result_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path: Path { segments, .. }, .. }) = ty {
        if let Some(PathSegment { ident, .. }) = segments.last() {
            return ident == "Result";
        }
    }
    false
}

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path: Path { segments, .. }, .. }) = ty {
        if let Some(PathSegment { ident, .. }) = segments.last() {
            return ident == "Option";
        }
    }
    false
}

fn get_inner_type_name(ty: &Type) -> String {
    if let Type::Path(TypePath { path: Path { segments, .. }, .. }) = ty {
        if let Some(PathSegment { arguments, .. }) = segments.last() {
            if let syn::PathArguments::AngleBracketed(args) = arguments {
                if let Some(arg) = args.args.first() {
                    if let syn::GenericArgument::Type(inner_ty) = arg {
                        if let Type::Path(TypePath { path: Path { segments: inner_segments, .. }, .. }) = inner_ty {
                            if let Some(inner_seg) = inner_segments.last() {
                                return inner_seg.ident.to_string();
                            }
                        }
                    }
                }
            }
        }
    }
    "<unknown>".to_string()
}

#[proc_macro_attribute]
pub fn funlog(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_meta = parse_macro_input!(attr with Punctuated::<Meta, Comma>::parse_terminated);
    let mut input = parse_macro_input!(item as ItemFn);
    
    let config = FunlogConfig::from_meta(attr_meta);
    let fn_name = &input.sig.ident;
    let fn_inputs = &input.sig.inputs;
    let fn_output = &input.sig.output;
    let fn_block = &input.block;
    let vis = &input.vis;
    
    // Generate parameter logging code
    let mut param_names = Vec::new();
    let mut param_types = Vec::new();
    
    for param in fn_inputs.iter() {
        if let FnArg::Typed(PatType { pat, ty, .. }) = param {
            let param_name = &pat;
            param_names.push(quote! { #param_name });
            param_types.push(ty);
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
    
    let param_logging = if config.param {
        let param_logs = param_names.iter().zip(param_types.iter()).map(|(name, _)| {
            quote! {
                format!("{}: {:?}", 
                    stringify!(#name),
                    #name
                )
            }
        });
        
        quote! {
            let params = vec![#(#param_logs),*].join(", ");
            let param_str = if params.is_empty() { "()".to_string() } else { format!("({})", params) };
        }
    } else {
        quote! {
            let param_str = "()".to_string();
        }
    };

    // Get return type info
    let (_return_type, is_result, is_option) = if let ReturnType::Type(_, ty) = fn_output {
        let type_name = get_inner_type_name(ty);
        let is_res = is_result_type(ty);
        let is_opt = is_option_type(ty);
        (type_name, is_res, is_opt)
    } else {
        ("()".to_string(), false, false)
    };
    
    let return_logging = if config.ret {
        if is_result {
            quote! {
                let ret_str = match &__result {
                    Ok(v) => format!("->Ok({:?})", v),
                    Err(e) => format!("->Err({:?})", e),
                };
            }
        } else if is_option {
            quote! {
                let ret_str = match &__result {
                    Some(v) => format!("->Some({:?})", v),
                    None => "->None".to_string(),
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
    
    let mut new_block = quote! {
        {
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

    input.block = syn::parse2(new_block).unwrap();
    TokenStream::from(quote! { #input })
}
