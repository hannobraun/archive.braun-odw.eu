mod field;

use convert_case::{Case, Casing as _};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::Ident;

use crate::input::Input;

use self::field::Field;

pub fn generate(input: Input) -> TokenStream {
    let vis = input.vis;
    let name = input.name;
    let mandatory_fields = Field::convert(input.mandatory_fields);
    let optional_fields = input.optional_fields;

    let name_lower = name.to_string().to_case(Case::Snake);
    let name_lower = Ident::new(&name_lower, Span::call_site());

    let builder_methods = optional_fields.iter().map(|field| {
        let name = &field.name;
        let ty = &field.ty;

        quote! {
            pub fn #name(mut self, value: impl Into<#ty>) -> Self {
                self.#name = Some(value.into());
                self
            }
        }
    });
    let mandatory_args = mandatory_fields.iter().map(|field| {
        let name = &field.name;
        let ty = &field.ty;

        quote! {
            #name: #ty,
        }
    });
    let mandatory_inits = mandatory_fields.iter().map(|field| {
        let name = &field.name;

        quote! {
            #name,
        }
    });
    let optional_inits = optional_fields.iter().map(|field| {
        let name = &field.name;

        quote! {
            #name: None,
        }
    });

    let output = quote! {
        impl ssg::Component for #name {}

        impl #name {
            #(
                #builder_methods
            )*
        }

        impl From<#name> for ssg::html::Content {
            fn from(component: #name) -> Self {
                let element: Element = component.into();
                element.into()
            }
        }

        impl From<#name> for ssg::html::Node {
            fn from(component: #name) -> Self {
                let element: Element = component.into();
                element.into()
            }
        }

        #vis fn #name_lower(
            #(
                #mandatory_args
            )*
        )
            -> #name
        {
            #name {
                #(
                    #mandatory_inits
                )*
                #(
                    #optional_inits
                )*
            }
        }
    };

    output.into()
}
