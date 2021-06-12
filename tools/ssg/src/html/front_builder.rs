use super::model::{Content, Element};

macro_rules! elements {
    ($($name:ident,)*) => {
        $(
            pub fn $name(content: impl Into<Content>) -> Element {
                Element {
                    name: stringify!($name),
                    attributes: Vec::new(),
                    content: content.into(),
                }
            }
        )*
    };
}

elements!(
    a, address, base, body, div, footer, h1, h2, h3, head, header, hr, li,
    link, main, meta, ol, p, section, span, title, ul,
);

macro_rules! attributes {
    ($($name:ident,)*) => {
        pub trait Attributes: Sized {
            $(
                fn $name(self, value: &'static str) -> Self;
            )*
        }

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

attributes!(class, href, target,);
