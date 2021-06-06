use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn hello_macro_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
