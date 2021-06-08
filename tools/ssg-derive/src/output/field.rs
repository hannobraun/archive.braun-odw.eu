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

        for field in fields {
            let arg_name = field.name.clone();
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
