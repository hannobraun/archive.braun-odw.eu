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
    ($($id:ident, $name:expr;)*) => {
        pub trait Attributes: Sized {
            $(
                fn $id(self, value: &'static str) -> Self;
            )*
        }

        impl Attributes for Element {
            $(
                fn $id(mut self, value: &'static str) -> Self {
                    self.attributes.push(($name, value));
                    self
                }
            )*
        }
    };
}

attributes!(
    class, "class";
    content, "content";
    href, "href";
    media, "media";
    name, "name";
    rel, "rel";
    target, "target";
    type_, "type";
);
