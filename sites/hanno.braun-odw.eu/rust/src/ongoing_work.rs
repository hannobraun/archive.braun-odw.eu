use ssg::build::html::{front_builder::*, model::Element};

use crate::util;

pub fn ongoing_work(
    title: &'static str,
    link: &'static str,
    content: Element,
) -> Element {
    li().with((ongoing_work_header(title, link), content))
}

fn ongoing_work_header(title: &'static str, link: &'static str) -> Element {
    header().with((h3().with(title), util::ext_link(link)))
}
