use syn::{Data, DeriveInput, Field, Fields, Ident, Type};

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

        let mut optional_fields = Vec::new();

        for field in fields.named {
            if is_optional(&field) {
                // Can't panic, as we already made sure this is a struct with
                // named fields.
                let ident = field.ident.unwrap();

                optional_fields.push(ident);
            }
        }

        Self {
            name: input.ident,
            optional_fields,
        }
    }
}

fn is_optional(field: &Field) -> bool {
    let path = match &field.ty {
        Type::Path(path) => path,
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
