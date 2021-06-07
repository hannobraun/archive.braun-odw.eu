use ssg::html::{front_builder::*, model::Element};

use crate::util;

pub fn ongoing_work(
    title: &'static str,
    link: &'static str,
    content: Element,
) -> Element {
    li((ongoing_work_header(title, link), content))
}

fn ongoing_work_header(title: &'static str, link: &'static str) -> Element {
    header((h3(title), util::ext_link(link)))
}
