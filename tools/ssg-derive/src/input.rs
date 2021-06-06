use syn::{DeriveInput, Ident};

pub struct Input {
    pub name: Ident,
}

impl From<DeriveInput> for Input {
    fn from(input: DeriveInput) -> Self {
        Self { name: input.ident }
    }
}
