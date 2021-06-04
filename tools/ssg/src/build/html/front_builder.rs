use super::model::{Content, Element};

pub fn a() -> Element {
    Element {
        name: "a",
        attributes: Vec::new(),
        content: Content::new(),
    }
}

pub trait Attributes: Sized {
    fn href(self, value: &'static str) -> Self;
}

impl Attributes for Element {
    fn href(mut self, value: &'static str) -> Self {
        self.attributes.push(("href", value));
        self
    }
}

pub trait With: Sized {
    fn with(self, content: impl Into<Content>) -> Self;
}

impl With for Element {
    fn with(mut self, content: impl Into<Content>) -> Self {
        self.content = content.into();
        self
    }
}
