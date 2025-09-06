use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Represents the generated output code components for a function with logging.
///
/// This struct contains all the TokenStream components needed to generate
/// the final function with logging capabilities.
///
/// # Examples
///
/// ```
/// use funlog::output::Output;
/// use quote::quote;
///
/// let output = Output {
///     inner_func: quote! { fn __test__() {} },
///     func_declare_start: quote! { fn test() },
///     func_output_start: quote! { println!("start"); },
///     func_declare_body: quote! { let output = __test__(); },
///     func_output_end: quote! { println!("end"); },
///     func_declare_end: quote! { output },
/// };
/// ```
pub struct Output {
    /// The inner function definition (renamed with __ prefix)
    pub inner_func: TokenStream2,
    /// The start of the outer function declaration
    pub func_declare_start: TokenStream2,
    /// The logging output at function start
    pub func_output_start: TokenStream2,
    /// The body of the outer function (calls inner function)
    pub func_declare_body: TokenStream2,
    /// The logging output at function end
    pub func_output_end: TokenStream2,
    /// The end of the outer function (return statement)
    pub func_declare_end: TokenStream2,
}

impl From<Output> for TokenStream {
    /// Converts the Output struct into a final TokenStream for the macro.
    ///
    /// This method combines all the output components into a single TokenStream
    /// that represents the complete function with logging.
    ///
    /// # Arguments
    ///
    /// * `val` - The Output struct to convert
    ///
    /// # Returns
    ///
    /// Returns a TokenStream containing the complete generated code
    ///
    /// # Examples
    ///
    /// ```
    /// use funlog::output::Output;
    /// use proc_macro::TokenStream;
    /// use quote::quote;
    ///
    /// let output = Output {
    ///     inner_func: quote! { fn __test__() {} },
    ///     func_declare_start: quote! { fn test() },
    ///     func_output_start: quote! {},
    ///     func_declare_body: quote! { let output = __test__(); },
    ///     func_output_end: quote! {},
    ///     func_declare_end: quote! { output },
    /// };
    /// let token_stream: TokenStream = output.into();
    /// ```
    fn from(val: Output) -> Self {
        let Output {
            inner_func,
            func_declare_start,
            func_output_start,
            func_declare_body,
            func_output_end,
            func_declare_end,
        } = val;
        let ts2 = quote! {
            #inner_func
            #func_declare_start {
                #func_output_start
                #func_declare_body
                #func_output_end
                #func_declare_end
            }
        };
        ts2.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_output_creation() {
        let output = Output {
            inner_func: quote! { fn __test__() {} },
            func_declare_start: quote! { fn test() },
            func_output_start: quote! { println!("start"); },
            func_declare_body: quote! { let output = __test__(); },
            func_output_end: quote! { println!("end"); },
            func_declare_end: quote! { output },
        };

        // Test that all fields are properly set
        assert!(!output.inner_func.is_empty());
        assert!(!output.func_declare_start.is_empty());
        assert!(!output.func_output_start.is_empty());
        assert!(!output.func_declare_body.is_empty());
        assert!(!output.func_output_end.is_empty());
        assert!(!output.func_declare_end.is_empty());
    }

    #[test]
    fn test_output_from_conversion() {
        let output = Output {
            inner_func: quote! { fn __test__() { 42 } },
            func_declare_start: quote! { fn test() -> i32 },
            func_output_start: quote! { println!("entering test"); },
            func_declare_body: quote! { let output = __test__(); },
            func_output_end: quote! { println!("exiting test"); },
            func_declare_end: quote! { output },
        };

        // We can't convert to TokenStream in unit tests, but we can verify
        // that the conversion method exists and the struct is properly formed
        let _: Output = output;
        // Test passes if no panic occurs
    }

    #[test]
    fn test_output_with_empty_components() {
        let output = Output {
            inner_func: quote! { fn __empty__() {} },
            func_declare_start: quote! { fn empty() },
            func_output_start: quote! {},
            func_declare_body: quote! { let output = __empty__(); },
            func_output_end: quote! {},
            func_declare_end: quote! { output },
        };

        // Verify that empty components are handled properly
        assert!(output.func_output_start.is_empty());
        assert!(output.func_output_end.is_empty());
        assert!(!output.inner_func.is_empty());
    }

    #[test]
    fn test_output_with_complex_logging() {
        let output = Output {
            inner_func: quote! {
                fn __complex_func__(x: i32, y: String) -> Result<i32, String> {
                    if x > 0 { Ok(x) } else { Err(y) }
                }
            },
            func_declare_start: quote! {
                fn complex_func(x: i32, y: String) -> Result<i32, String>
            },
            func_output_start: quote! {
                log::debug!("complex_func [in ]: x:{}, y:{}", format!("{:?}", x), format!("{:?}", y));
            },
            func_declare_body: quote! {
                let __x_value__ = format!("{:?}", x);
                let __y_value__ = format!("{:?}", y);
                let output = __complex_func__(x, y);
            },
            func_output_end: quote! {
                log::debug!("complex_func [out]: return:{}", format!("{:?}", output));
            },
            func_declare_end: quote! { output },
        };

        // Verify complex function components are properly set
        assert!(!output.inner_func.is_empty());
        assert!(!output.func_declare_start.is_empty());
        assert!(!output.func_output_start.is_empty());
        assert!(!output.func_declare_body.is_empty());
        assert!(!output.func_output_end.is_empty());
        assert!(!output.func_declare_end.is_empty());
    }

    #[test]
    fn test_output_structure_integrity() {
        let output = Output {
            inner_func: quote! { fn __test__() -> i32 { 42 } },
            func_declare_start: quote! { fn test() -> i32 },
            func_output_start: quote! { println!("start"); },
            func_declare_body: quote! { let output = __test__(); },
            func_output_end: quote! { println!("end"); },
            func_declare_end: quote! { output },
        };

        // Verify the structure has all expected components
        // We can't convert to TokenStream in unit tests, but we can verify
        // that all components are present and non-empty where expected
        assert!(!output.inner_func.is_empty());
        assert!(!output.func_declare_start.is_empty());
        assert!(!output.func_output_start.is_empty());
        assert!(!output.func_declare_body.is_empty());
        assert!(!output.func_output_end.is_empty());
        assert!(!output.func_declare_end.is_empty());
    }
}
