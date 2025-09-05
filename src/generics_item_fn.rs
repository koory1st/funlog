use syn::{Attribute, Block, ItemFn, Signature, Visibility};

/// A wrapper struct for function information extracted from syn::ItemFn.
/// 
/// This struct provides a convenient way to work with function components
/// needed for the funlog macro processing.
/// 
/// # Examples
/// 
/// ```
/// use syn::{parse_quote, ItemFn};
/// use funlog::generics_item_fn::GenericsFn;
/// 
/// let func: ItemFn = parse_quote! {
///     pub fn example(x: i32) -> i32 { x + 1 }
/// };
/// let generics_fn = GenericsFn::from(func);
/// ```
pub struct GenericsFn {
    /// Function attributes (currently unused but preserved for future use)
    #[allow(dead_code)]
    pub attrs: Vec<Attribute>,
    /// Function visibility (pub, pub(crate), etc.)
    pub vis: Visibility,
    /// Function signature (name, parameters, return type)
    pub sig: Signature,
    /// Function body block
    pub block: Block,
}

impl From<ItemFn> for GenericsFn {
    /// Converts a syn::ItemFn into a GenericsFn.
    /// 
    /// # Arguments
    /// 
    /// * `item` - The ItemFn to convert
    /// 
    /// # Returns
    /// 
    /// Returns a GenericsFn with all the function components extracted
    /// 
    /// # Examples
    /// 
    /// ```
    /// use syn::{parse_quote, ItemFn};
    /// use funlog::generics_item_fn::GenericsFn;
    /// 
    /// let func: ItemFn = parse_quote! {
    ///     pub fn test_func(x: i32) -> i32 { x + 1 }
    /// };
    /// let generics_fn = GenericsFn::from(func);
    /// assert_eq!(generics_fn.sig.ident.to_string(), "test_func");
    /// ```
    fn from(item: ItemFn) -> Self {
        GenericsFn {
            attrs: item.attrs,
            vis: item.vis,
            sig: item.sig,
            block: *item.block,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, ItemFn};

    #[test]
    fn test_from_item_fn() {
        let func: ItemFn = parse_quote! {
            pub fn test_func(x: i32, y: String) -> i32 {
                x + 1
            }
        };
        
        let generics_fn = GenericsFn::from(func);
        
        // Test that all fields are properly extracted
        assert_eq!(generics_fn.sig.ident.to_string(), "test_func");
        assert_eq!(generics_fn.sig.inputs.len(), 2);
        assert!(matches!(generics_fn.vis, Visibility::Public(_)));
    }

    #[test]
    fn test_from_private_function() {
        let func: ItemFn = parse_quote! {
            fn private_func() {
                println!("private");
            }
        };
        
        let generics_fn = GenericsFn::from(func);
        
        assert_eq!(generics_fn.sig.ident.to_string(), "private_func");
        assert_eq!(generics_fn.sig.inputs.len(), 0);
        assert!(matches!(generics_fn.vis, Visibility::Inherited));
    }

    #[test]
    fn test_from_function_with_attributes() {
        let func: ItemFn = parse_quote! {
            #[inline]
            #[allow(dead_code)]
            pub fn attributed_func(x: i32) -> i32 {
                x * 2
            }
        };
        
        let generics_fn = GenericsFn::from(func);
        
        assert_eq!(generics_fn.sig.ident.to_string(), "attributed_func");
        assert_eq!(generics_fn.attrs.len(), 2);
    }

    #[test]
    fn test_from_function_with_complex_signature() {
        let func: ItemFn = parse_quote! {
            pub async fn complex_func<T: Clone>(
                x: T,
                y: &str,
                z: Option<i32>
            ) -> Result<T, String> 
            where 
                T: std::fmt::Debug 
            {
                Ok(x)
            }
        };
        
        let generics_fn = GenericsFn::from(func);
        
        assert_eq!(generics_fn.sig.ident.to_string(), "complex_func");
        assert_eq!(generics_fn.sig.inputs.len(), 3);
        assert!(generics_fn.sig.asyncness.is_some());
        assert!(!generics_fn.sig.generics.params.is_empty());
    }

    #[test]
    fn test_from_function_no_return_type() {
        let func: ItemFn = parse_quote! {
            fn void_func(x: i32) {
                println!("{}", x);
            }
        };
        
        let generics_fn = GenericsFn::from(func);
        
        assert_eq!(generics_fn.sig.ident.to_string(), "void_func");
        assert!(matches!(generics_fn.sig.output, syn::ReturnType::Default));
    }

    #[test]
    fn test_from_function_with_return_type() {
        let func: ItemFn = parse_quote! {
            fn returning_func() -> String {
                "hello".to_string()
            }
        };
        
        let generics_fn = GenericsFn::from(func);
        
        assert_eq!(generics_fn.sig.ident.to_string(), "returning_func");
        assert!(matches!(generics_fn.sig.output, syn::ReturnType::Type(_, _)));
    }
}
