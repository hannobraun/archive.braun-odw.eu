use crate::input;

pub struct Field {
    pub name: syn::Ident,
    pub ty: syn::Type,
}

impl Field {
    pub fn convert(
        fields: impl IntoIterator<Item = input::Field>,
    ) -> Vec<Self> {
        let mut converted = Vec::new();

        for field in fields {
            converted.push(Self {
                name: field.name,
                ty: field.ty,
            });
        }

        converted
    }
}
