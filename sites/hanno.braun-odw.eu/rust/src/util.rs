use ssg::{
    html::{front_builder::*, model::Element},
    Component,
};

#[derive(Component)]
pub struct ExtLink {
    link: &'static str,
    text: Option<&'static str>,
}

impl From<ExtLink> for Element {
    fn from(ext_link: ExtLink) -> Self {
        a().href(ext_link.link)
            .target("_blank")
            .with(ext_link.text.unwrap_or(ext_link.link))
    }
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
