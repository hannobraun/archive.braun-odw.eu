use proc_macro2::Span;

use crate::input;

pub struct Field {
    pub arg_name: syn::Ident,
    pub field_name: syn::Ident,
    pub ty: syn::Type,
}

impl Field {
    pub fn convert(
        fields: impl IntoIterator<Item = input::Field>,
    ) -> Vec<Self> {
        let mut converted = Vec::new();

        for (i, field) in fields.into_iter().enumerate() {
            let arg_name =
                syn::Ident::new(&format!("_{}", i), Span::call_site());
            let field_name = field.name;

            converted.push(Self {
                arg_name,
                field_name,
                ty: field.ty,
            });
        }

        converted
    }
}
