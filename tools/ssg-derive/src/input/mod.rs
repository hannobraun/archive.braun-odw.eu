mod ty;

use self::ty::Type;

pub struct Input {
    pub vis: syn::Visibility,
    pub name: syn::Ident,
    pub mandatory_fields: Vec<Field>,
    pub optional_fields: Vec<Field>,
}

impl From<syn::DeriveInput> for Input {
    fn from(input: syn::DeriveInput) -> Self {
        let data = match input.data {
            syn::Data::Struct(data) => data,
            _ => panic!("`Component` can only be derived for structs"),
        };
        let fields = match data.fields {
            syn::Fields::Named(fields) => fields.named.into_iter().collect(),
            syn::Fields::Unnamed(fields) => {
                fields.unnamed.into_iter().collect()
            }
            syn::Fields::Unit => vec![],
        };

        let mut mandatory_fields = Vec::new();
        let mut optional_fields = Vec::new();

        for field in fields {
            let (ty, fields) = match Type::from(field.clone()) {
                Type::Mandatory(ty) => (ty, &mut mandatory_fields),
                Type::Optional(ty) => (ty, &mut optional_fields),
            };

            let name = field.ident;
            fields.push(Field { name, ty });
        }

        Self {
            vis: input.vis,
            name: input.ident,
            mandatory_fields,
            optional_fields,
        }
    }
}

pub struct Field {
    pub name: Option<syn::Ident>,
    pub ty: syn::Type,
}
