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
        let items = data::OngoingWork(vec![
            data::OngoingWorkItem {
                title: "Braun Embedded",
                link: "https://braun-embedded.com/",
                description: p((
                    "I provide software development and consulting services, \
                    specifically relating to Rust firmware on ARM Cortex-M \
                    microcontrollers. If you want to write your next firmware \
                    project in Rust and could use some help with the low-level \
                    stuff, ",
                    util::email().text("let me know"),
                    "!",
                )),
            },
            data::OngoingWorkItem {
                title: "Made by Hanno",
                link: "https://madeby.hannobraun.de/",
                description: p(
                    "I have a small workshop where I make semi-interesting \
                    stuff, mainly 3D-printed.",
                ),
            },
        ]);

        section((
            h2("Ongoing Work"),
            ul(Content::from_iter(
                items.into_iter().map(|item| OngoingWorkItem(item)),
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
