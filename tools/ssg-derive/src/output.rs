use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn output(name: Ident) -> TokenStream {
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
