mod input;
mod output;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use self::input::Input;

#[proc_macro_derive(Component)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input = Input::from(input);

    output::generate(input.name)
}
