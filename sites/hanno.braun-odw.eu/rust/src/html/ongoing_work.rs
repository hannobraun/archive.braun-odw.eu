use ssg::html::{front_builder::*, model::Element};

use crate::data;

use super::util;

pub fn ongoing_work_item(item: data::OngoingWorkItem) -> Element {
    li((
        header((h3(item.title), util::ext_link(item.link))),
        item.description,
    ))
}
