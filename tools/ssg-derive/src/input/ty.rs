pub enum Type {
    Mandatory(syn::Type),
    Optional(syn::Type),
}

impl From<syn::Field> for Type {
    fn from(field: syn::Field) -> Self {
        let ty = match field.ty {
            syn::Type::Path(path) => path,
            ty => {
                // Type is not a path, so it can't be `Option<...>`.
                // Therefore this is not an optional field.
                return Type::Mandatory(ty);
            }
        };

        // The path is optional, if it's an `Option`. `Option` could be
        // used in other ways (like a fully qualified path), but this
        // should do for now.
        let segment = ty.path.segments[0].clone();
        if segment.ident.to_string() == "Option" {
            let arguments = match segment.arguments {
                syn::PathArguments::AngleBracketed(arguments) => arguments,
                _ => panic!("Expected `Option` to have generic arguments"),
            };
            let arg = arguments
                .args
                .into_iter()
                .next()
                .expect("Expected `Option` to have a generic argument");
            let ty = match arg {
                syn::GenericArgument::Type(ty) => ty,
                _ => panic!(
                    "Generic argument of `Option` is not a type parameter"
                ),
            };

            Type::Optional(ty)
        } else {
            Type::Mandatory(syn::Type::Path(ty))
        }
    }
}
