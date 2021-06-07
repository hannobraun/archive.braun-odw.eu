use ssg::{
    html::{front_builder::*, Element},
    Component,
};

#[derive(Component)]
pub struct ExtLink {
    link: &'static str,
    text: Option<&'static str>,
}

impl From<ExtLink> for Element {
    fn from(ext_link: ExtLink) -> Self {
        a(ext_link.text.unwrap_or(ext_link.link))
            .href(ext_link.link)
            .target("_blank")
    }
}

#[derive(Component)]
pub struct Email {
    text: Option<&'static str>,
}

impl From<Email> for Element {
    fn from(email: Email) -> Self {
        a(email.text.unwrap_or("hanno@braun-odw.eu"))
            .href("mailto:Hanno%20Braun%20%3Channo@braun-odw.eu%3E")
    }
}
