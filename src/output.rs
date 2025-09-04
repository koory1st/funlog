use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
pub struct Output {
    pub inner_func: TokenStream2,
    pub func_declare_start: TokenStream2,
    pub func_output_start: TokenStream2,
    pub func_declare_body: TokenStream2,
    pub func_output_end: TokenStream2,
    pub func_declare_end: TokenStream2,
}

impl From<Output> for TokenStream {
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
