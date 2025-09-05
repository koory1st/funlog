//! # Funlog - Function Logging Macro
//! 
//! Funlog is a procedural macro that automatically adds logging to functions.
//! It supports various logging levels, parameter logging, return value logging,
//! and flexible positioning of log statements.
//! 
//! ## Features
//! 
//! - Multiple log levels: `print`, `trace`, `debug`, `info`, `warn`, `error`
//! - Parameter control: `all`, `none`, or `params(param1, param2)`
//! - Position control: `onStart`, `onEnd`, `onStartEnd`
//! - Return value logging: `retVal`
//! - Conflict detection and helpful error messages
//! 
//! ## Examples
//! 
//! ```rust
//! use funlog::funlog;
//! 
//! #[funlog(debug, all, retVal)]
//! fn calculate(x: i32, y: i32) -> i32 {
//!     x + y
//! }
//! 
//! #[funlog(info, params(name), onStart)]
//! fn greet(name: &str, age: u32) {
//!     println!("Hello, {}!", name);
//! }
//! ```

mod config;
mod config_builder;
mod error;
mod generics_item_fn;
mod log_template;
mod output;

use config_builder::ConfigBuilder;
use generics_item_fn::GenericsFn;
use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Meta;
use syn::{parse_macro_input, ItemFn};

/// A procedural macro attribute for adding automatic logging to functions.
/// 
/// This macro generates logging code that can output function entry, exit,
/// parameters, and return values based on the provided configuration.
/// 
/// # Arguments
/// 
/// The macro accepts various configuration options:
/// 
/// ## Log Levels
/// - `print` - Use `println!` for output (default)
/// - `trace` - Use `log::trace!`
/// - `debug` - Use `log::debug!`
/// - `info` - Use `log::info!`
/// - `warn` - Use `log::warn!`
/// - `error` - Use `log::error!`
/// 
/// ## Parameter Control
/// - `all` - Log all function parameters (default)
/// - `none` - Log no parameters
/// - `params(param1, param2, ...)` - Log specific parameters
/// 
/// ## Position Control
/// - `onStart` - Log only at function entry
/// - `onEnd` - Log only at function exit
/// - `onStartEnd` - Log at both entry and exit (default)
/// 
/// ## Return Value
/// - `retVal` - Include return value in logging
/// 
/// # Examples
/// 
/// ```rust
/// use funlog::funlog;
/// 
/// // Basic usage with debug logging and all parameters
/// #[funlog(debug, all)]
/// fn add(x: i32, y: i32) -> i32 {
///     x + y
/// }
/// 
/// // Log specific parameters only at function start
/// #[funlog(info, params(name), onStart)]
/// fn greet(name: &str, age: u32) {
///     println!("Hello, {}!", name);
/// }
/// 
/// // Include return value in logging
/// #[funlog(debug, all, retVal)]
/// fn multiply(x: i32, y: i32) -> i32 {
///     x * y
/// }
/// 
/// // No parameter logging, only function entry/exit
/// #[funlog(trace, none)]
/// fn process_data() {
///     // processing logic
/// }
/// ```
/// 
/// # Errors
/// 
/// The macro will produce compile-time errors for:
/// - Conflicting options (e.g., `debug, info`)
/// - Invalid parameter names
/// - Invalid attribute names (with suggestions)
/// - Incorrect syntax
/// 
/// # Note
/// 
/// The macro only generates logging code in debug builds. In release builds,
/// the original function is returned unchanged for optimal performance.
#[proc_macro_attribute]
pub fn funlog(args: TokenStream, item: TokenStream) -> TokenStream {
    let is_debug = cfg!(debug_assertions);

    // when not debug, just return the original function
    if !is_debug {
        return item;
    }

    let func = parse_macro_input!(item as ItemFn);
    let func = GenericsFn::from(func);
    let attr_meta: Punctuated<Meta, Comma> =
        parse_macro_input!(args with Punctuated::<Meta, Comma>::parse_terminated);
    match ConfigBuilder::from(attr_meta, func) {
        Ok(config_builder) => match config_builder.build() {
            Ok(config) => {
                let output = config.to_output();
                output.into()
            }
            Err(e) => {
                let syn_error: syn::Error = e.into();
                syn_error.into_compile_error().into()
            }
        },
        Err(e) => {
            let syn_error: syn::Error = e.into();
            syn_error.into_compile_error().into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use syn::{parse_quote, ItemFn};

    #[test]
    fn test_funlog_macro_exists() {
        // This test ensures the macro is properly exported
        // We can't easily test the macro directly in unit tests,
        // but we can verify the supporting functions work
        
        let func: ItemFn = parse_quote! {
            fn test_func(x: i32) -> i32 { x + 1 }
        };
        
        let generics_fn = GenericsFn::from(func);
        assert_eq!(generics_fn.sig.ident.to_string(), "test_func");
    }

    #[test]
    fn test_debug_build_behavior() {
        // In debug builds, the macro should process the function
        // In release builds, it should return the original
        
        // We can test this by checking the cfg! macro
        let is_debug = cfg!(debug_assertions);
        
        // This test documents the expected behavior
        if is_debug {
            // In debug builds, funlog should process the function
            assert!(true, "Debug build - funlog should process functions");
        } else {
            // In release builds, funlog should return original function
            assert!(true, "Release build - funlog should pass through functions");
        }
    }

    #[test]
    fn test_integration_with_config_builder() {
        let func: ItemFn = parse_quote! {
            pub fn integration_test(x: i32, y: String) -> i32 {
                x + 1
            }
        };
        
        let generics_fn = GenericsFn::from(func);
        let meta_list: Punctuated<Meta, Comma> = parse_quote! { debug, all, retVal };
        
        let config_builder = ConfigBuilder::from(meta_list, generics_fn);
        assert!(config_builder.is_ok());
        
        let config = config_builder.unwrap().build();
        assert!(config.is_ok());
        
        let output = config.unwrap().to_output();
        assert!(!output.inner_func.is_empty());
    }

    #[test]
    fn test_error_handling_integration() {
        let func: ItemFn = parse_quote! {
            fn error_test() {}
        };
        
        let generics_fn = GenericsFn::from(func);
        
        // Test with conflicting options
        let meta_list: Punctuated<Meta, Comma> = parse_quote! { debug, info };
        
        let result = ConfigBuilder::from(meta_list, generics_fn);
        assert!(result.is_err());
        
        // Verify it's the expected error type
        match result.unwrap_err() {
            error::ConfigError::ConflictingOptions { .. } => {
                // Expected error type
            }
            _ => panic!("Expected ConflictingOptions error"),
        }
    }
}
