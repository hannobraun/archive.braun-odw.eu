use std::vec;

use ssg::html::{front_builder::*, Content, Element};

use crate::html::util;

pub struct OngoingWork(pub Vec<OngoingWorkItem>);

impl OngoingWork {
    pub fn load() -> Self {
        Self(vec![
            OngoingWorkItem {
                title: "Braun Embedded",
                link: "https://braun-embedded.com/",
                description: p((
                    "I provide software development and consulting services, specifically relating to Rust firmware on ARM Cortex-M microcontrollers. If you want to write your next firmware project in Rust and could use some help with the low-level stuff, ",
                    util::email().text("let me know"),
                    "!",
                )),
            },
            OngoingWorkItem {
                title: "Made by Hanno",
                link: "https://madeby.hannobraun.de/",
                description: p(
                    "I have a small workshop where I make semi-interesting stuff, mainly 3D-printed.",
                ),
            },
        ])
    }
}

impl IntoIterator for OngoingWork {
    type Item = OngoingWorkItem;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct OngoingWorkItem {
    pub title: &'static str,
    pub link: &'static str,
    pub description: Element,
}

pub struct SideProjects(pub Vec<SideProject>);

impl SideProjects {
    // TASK: Load side projects from external data files.
    pub fn load() -> Self {
        Self(vec![
            SideProject {
                title: "Fornjot: Spacer",
                date: "2021-05-19",
                description: (
                    p((
                        util::ext_link("https://github.com/hannobraun/fornjot")
                            .text("Fornjot"),
                        " is an experimental programmatic CAD system. I've been working on multiple iterations of it over a few years, and this is the first public release.",
                    )),
                    p((
                        "The goal for this first release was to build enough infrastructure to support ",
                        util::ext_link(
                            "https://github.com/hannobraun/fornjot/tree/main/models/spacer"
                        )
                            .text("a simple spacer model"),
                        ". I plan to extend it and create more complex models going forward."
                    )),
                ).into(),
            },
            SideProject {
                title: "braun-odw.eu",
                date: "2021-04-06",
                description: p((
                    "I revamped my website (the one you're looking at right now) and put it on new technical footing, using my own ",
                    util::ext_link("https://github.com/hannobraun/braun-odw.eu")
                        .text("homegrown static site generator"),
                    ". I also got a new domain and started consolidating most of my IT infrastructure (including email and Matrix homeserver) under it."
                )).into(),
            },
            SideProject {
                title: "My Boss: Contacts",
                date: "2021-03-05",
                description: (
                    p((
                        util::ext_link("https://github.com/hannobraun/my-boss")
                            .text("My Boss"),
                        " is a command-line application that tells me what to do and when to do it. Kind of like an ERP system, except that I'm the opposite of an enterprise. This was the initial release that included contact management functionality, kind of like a CRM."
                    )),
                    p(
                        "I use it every day to remember to keep in touch with my business contacts (and also some personal ones). I plan to extend it in the future to encompass more areas of my business, like basic bookkeeping and possibly more."
                    ),
                ).into(),
            }
        ])
    }
}

impl IntoIterator for SideProjects {
    type Item = SideProject;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct SideProject {
    pub title: &'static str,
    pub date: &'static str,
    pub description: Content,
}
