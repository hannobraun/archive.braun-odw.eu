use ssg::{
    build::html::{
        front_builder::*,
        model::{Content, Element, Node},
    },
    Component,
};

pub fn raw_link(link: &'static str) -> Element {
    ext_link(link, link)
}

pub fn ext_link(link: &'static str, text: &'static str) -> Element {
    a().href(link).target("_blank").with(text)
}

pub fn email() -> Email {
    Email { text: None }
}

#[derive(Component)]
pub struct Email {
    text: Option<&'static str>,
}

impl From<Email> for Element {
    fn from(email: Email) -> Self {
        a().href("mailto:Hanno%20Braun%20%3Channo@braun-odw.eu%3E")
            .with(email.text.unwrap_or("hanno@braun-odw.eu"))
    }
}
