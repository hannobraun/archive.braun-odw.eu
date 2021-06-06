use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn generate(name: Ident, fields: Vec<Ident>) -> TokenStream {
    let builder_methods = fields.into_iter().map(|name| {
        quote! {
            pub fn #name(mut self, value: &'static str) -> Self {
                self.#name = Some(value);
                self
            }
        }
    });

    let output = quote! {
        impl ssg::Component for #name {}

        impl #name {
            #(
                #builder_methods
            )*
        }

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
