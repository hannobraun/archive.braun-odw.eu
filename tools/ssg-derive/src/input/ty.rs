pub fn is_optional(field: &syn::Field) -> bool {
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
