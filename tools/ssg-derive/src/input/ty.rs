pub enum Type {
    Mandatory,
    Optional,
}

impl<'a> From<&'a syn::Field> for Type {
    fn from(field: &syn::Field) -> Self {
        let path = match &field.ty {
            syn::Type::Path(path) => path,
            _ => {
                // Type is not a path, so it can't be `Option<...>`.
                // Therefore this is not an optional field.
                return Type::Mandatory;
            }
        };

        // The path is optional, if it's an `Option`. `Option` could be
        // used in other ways (like a fully qualified path), but this
        // should do for now.
        if path.path.segments[0].ident.to_string() == "Option" {
            Type::Optional
        } else {
            Type::Mandatory
        }
    }
}
