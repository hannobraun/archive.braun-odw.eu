use ssg::{
    html,
    html::{front_builder::*, model::Element},
    Component,
};

use crate::data;

use super::util;

#[derive(Component)]
pub struct OngoingWork;

impl From<OngoingWork> for Element {
    fn from(_: OngoingWork) -> Self {
        html! {
            section {
                h2 { "Ongoing Work" }
                ul {
                    {
                        ongoing_work_item(
                            data::OngoingWorkItem {
                                title: "Braun Embedded",
                                link: "https://braun-embedded.com/",
                                description: html! {
                                    p {
                                        "I provide software \
                                        development and consulting \
                                        services, specifically \
                                        relating to Rust firmware on \
                                        ARM Cortex-M microcontrollers. \
                                        If you want to write your next \
                                        firmware project in Rust and \
                                        could use some help with the \
                                        low-level stuff, "
                                        {
                                            util::email()
                                                .text("let me know")
                                        }
                                        "!"
                                    }
                                },
                            }
                        )
                    }
                    {
                        ongoing_work_item(
                            data::OngoingWorkItem {
                                title: "Made by Hanno",
                                link: "https://madeby.hannobraun.de/",
                                description: html! {
                                    p {
                                        "I have a small workshop where \
                                        I make semi-interesting stuff, \
                                        mainly 3D-printed."
                                    }
                                },
                            }
                        )
                    }
                }
            }
        }
    }
}

pub fn ongoing_work_item(item: data::OngoingWorkItem) -> Element {
    li((
        header((h3(item.title), util::ext_link(item.link))),
        item.description,
    ))
}
