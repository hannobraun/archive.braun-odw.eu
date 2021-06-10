use ssg::html::{front_builder::*, model::Element};

use super::util;

pub fn ongoing_work_item(
    title: &'static str,
    link: &'static str,
    content: Element,
) -> Element {
    li((ongoing_work_header(title, link), content))
}

fn ongoing_work_header(title: &'static str, link: &'static str) -> Element {
    header((h3(title), util::ext_link(link)))
}
