use super::model::{Content, Element};

macro_rules! elements {
    ($($name:ident,)*) => {
        $(
            pub fn $name() -> Element {
                Element {
                    name: stringify!($name),
                    attributes: Vec::new(),
                    content: Content::new(),
                }
            }
        )*
    };
}

elements!(a,);

pub trait Attributes: Sized {
    fn href(self, value: &'static str) -> Self;
}

macro_rules! attributes {
    ($($name:ident,)*) => {
        impl Attributes for Element {
            $(
                fn $name(mut self, value: &'static str) -> Self {
                    self.attributes.push((stringify!($name), value));
                    self
                }
            )*
        }
    };
}

attributes!(href,);

pub trait With: Sized {
    fn with(self, content: impl Into<Content>) -> Self;
}

impl With for Element {
    fn with(mut self, content: impl Into<Content>) -> Self {
        self.content = content.into();
        self
    }
}
