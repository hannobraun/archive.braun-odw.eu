use ssg::build::html::{front_builder::*, model::Element};

pub fn raw_link(link: &'static str) -> Element {
    ext_link(link, link)
}

pub fn ext_link(link: &'static str, text: &'static str) -> Element {
    a().href(link).target("_blank").with(text)
}

pub fn email(text: &'static str) -> Element {
    a().href("mailto:Hanno%20Braun%20%3Channo@braun-odw.eu%3E")
        .with(text)
}
