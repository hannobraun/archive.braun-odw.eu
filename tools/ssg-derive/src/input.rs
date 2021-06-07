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
            syn::Fields::Unit => vec![],
            syn::Fields::Unnamed(_) => {
                panic!("`Component` can not be derived for tuple structs")
            }
        };

        let mut mandatory_fields = Vec::new();
        let mut optional_fields = Vec::new();

        for field in fields {
            let fields = if is_optional(&field) {
                &mut optional_fields
            } else {
                &mut mandatory_fields
            };

            // Can't panic, as we already made sure this is a struct with named
            // fields.
            let ident = field.ident.unwrap();

            fields.push(Field { name: ident });
        }

        Self {
            vis: input.vis,
            name: input.ident,
            mandatory_fields,
            optional_fields,
        }
    }
}

fn is_optional(field: &syn::Field) -> bool {
    let path = match &field.ty {
        syn::Type::Path(path) => path,
        _ => {
            // Type is not a path, so it can't be `Option<...>`.
            // Therefore this is not an optional field.
            return false;
        }
    };

    // The path is optional, if it's an `Option`. `Option` could be
    // used in other ways (like a fully qualified path), but this
    // should do for now.
    path.path.segments[0].ident.to_string() == "Option"
}

pub struct Field {
    pub name: syn::Ident,
}
