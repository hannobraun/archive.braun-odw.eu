use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let output = quote! {
        impl ssg::Component for #name {}

        impl From<#name> for Content {
            fn from(component: #name) -> Self {
                let element: Element = component.into();
                element.into()
            }
        }

        impl From<#name> for Node {
            fn from(component: #name) -> Self {
                let element: Element = component.into();
                element.into()
            }
        }
    };

    output.into()
}
