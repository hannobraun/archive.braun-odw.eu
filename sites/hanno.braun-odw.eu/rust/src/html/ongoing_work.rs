use ssg::{
    html::{front_builder::*, model::Element, Content},
    Component,
};

use crate::data;

use super::util;

#[derive(Component)]
pub struct OngoingWork;

impl From<OngoingWork> for Element {
    fn from(_: OngoingWork) -> Self {
        let ongoing_work = data::OngoingWork::load();

        section((
            h2("Ongoing Work"),
            ul(Content::from_iter(
                ongoing_work.into_iter().map(|item| OngoingWorkItem(item)),
            )),
        ))
    }
}

#[derive(Component)]
struct OngoingWorkItem(data::OngoingWorkItem);

impl From<OngoingWorkItem> for Element {
    fn from(item: OngoingWorkItem) -> Self {
        li((
            header((h3(item.0.title), util::ext_link(item.0.link))),
            item.0.description,
        ))
    }
}
