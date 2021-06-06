use convert_case::{Case, Casing as _};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::Ident;

use crate::input::Input;

pub fn generate(input: Input) -> TokenStream {
    let name = input.name;
    let mandatory_fields = input.mandatory_fields;
    let optional_fields = input.optional_fields;

    let name_lower = name.to_string().to_case(Case::Snake);
    let name_lower = Ident::new(&name_lower, Span::call_site());

    let builder_methods = optional_fields.iter().map(|name| {
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

        pub fn #name_lower(
            #(
                #mandatory_fields: &'static str,
            )*
        )
            -> #name
        {
            #name {
                #(
                    #mandatory_fields,
                )*
                #(
                    #optional_fields: None,
                )*
            }
        }
    };

    output.into()
}
