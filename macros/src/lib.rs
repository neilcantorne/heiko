use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn kernel(_: TokenStream, item: TokenStream) -> TokenStream {
    return item;
}