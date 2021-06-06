use syn::{Data, DeriveInput, Fields, Ident};

pub struct Input {
    pub name: Ident,
    pub optional_fields: Vec<Ident>,
}

impl From<DeriveInput> for Input {
    fn from(input: DeriveInput) -> Self {
        let data = match input.data {
            Data::Struct(data) => data,
            _ => panic!("`Component` can only be derived for structs"),
        };
        let fields = match data.fields {
            Fields::Named(fields) => fields,
            _ => panic!(
                "`Component` can only be derived for structs with named fields"
            ),
        };

        let optional_fields = fields
            .named
            .into_iter()
            .map(|field| {
                // Can't panic, as we already made sure this is a struct with
                // named fields.
                field.ident.unwrap()
            })
            .collect();

        Self {
            name: input.ident,
            optional_fields,
        }
    }
}
