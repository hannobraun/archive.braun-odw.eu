use ssg::{
    html::{syntax::*, Element},
    Component,
};

#[derive(Component)]
pub struct ExtLink {
    link: &'static str,
    text: Option<&'static str>,
}

impl From<ExtLink> for Element {
    fn from(ext_link: ExtLink) -> Self {
        let text = ext_link.text.unwrap_or(ext_link.link);

        a(text).href(ext_link.link).target("_blank")
    }
}

#[derive(Component)]
pub struct Email {
    text: Option<&'static str>,
}

impl From<Email> for Element {
    fn from(email: Email) -> Self {
        let text = email.text.unwrap_or("hanno@braun-odw.eu");

        a(text).href("mailto:Hanno%20Braun%20%3Channo@braun-odw.eu%3E")
    }
}
