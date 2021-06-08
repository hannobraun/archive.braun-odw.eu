mod ongoing_work;
mod side_projects;
mod site_footer;
mod util;

use ssg::{
    html,
    html::{front_builder::*, Element},
};

use crate::data;

pub fn build(dev: bool) -> Element {
    // TASK: Load side projects from external data files. TOML with a `content`
    //       key that contains Markdown would probably be best.
    //
    //       pulldown-cmark looks like a good option for handling and rendering
    //       the Markdown.
    let side_projects = vec![
        data::SideProject {
            title: "Fornjot: Spacer",
            date: "2021-05-19",
            description: (
                p((
                    util::ext_link(
                        "https://github.com/hannobraun/fornjot",
                    )
                    .text("Fornjot"),
                    " is an experimental programmatic CAD \
                    system. I've been working on multiple \
                    iterations of it over a few years, and \
                    this is the first public release.",
                )),
                p((
                    "The goal for this first release was to \
                    build enough infrastructure to support ",
                    util::ext_link(
                        "https://github.com/hannobraun/fornjot/tree/main/models/spacer",
                    )
                    .text("a simple spacer model"),
                    ". I plan to extend it and create more \
                    complex models going forward.",
                )),
            )
            .into()
        },
        data::SideProject {
            title: "braun-odw.eu",
            date: "2021-04-06",
            description: p((
                "I revamped my website (the one you're \
                    looking at right now) and put it on new \
                    technical footing, using my own ",
                util::ext_link("https://github.com/hannobraun/braun-odw.eu")
                    .text("homegrown static site generator"),
                ". I also got a new domain and started \
                    consolidating most of my IT infrastructure \
                    (including email and Matrix homeserver) \
                    under it.",
            ))
            .into(),
        },
        data::SideProject {
            title: "My Boss: Contacts",
            date: "2021-03-05",
            description: (
                p((
                    util::ext_link("https://github.com/hannobraun/my-boss")
                        .text("My Boss"),
                    " is a command-line application that tells me \
                    what to do and when to do it. Kind of like an \
                    ERP system, except that I'm the opposite of an \
                    enterprise. This was the initial release that \
                    included contact management functionality, \
                    kind of like a CRM.",
                )),
                p("I use it every day to remember to keep in \
                    touch with my business contacts (and also some \
                    personal ones). I plan to extend it in the \
                    future to encompass more areas of my business, \
                    like basic bookkeeping and possibly more."),
            )
                .into(),
        },
    ];

    html! {
        html("lang"="en") {
            head {
                meta(
                    "http-equiv"="Content-Type"
                    "content"="text/html; charset=UTF-8"
                ) {}
                meta(
                    "name"="viewport"
                    "content"="width=device-width, initial-scale=1"
                ) {}

                title {
                    "Hanno Braun"
                }

                { base_if_dev_mode(dev) }

                link(
                    "href"="style.css"
                    "rel"="stylesheet"
                    "type"="text/css"
                    "media"="all"
                ) {}
            }
            body {
                main {
                    h1 { "Hanno Braun" }

                    section {
                        h2 { "Who?" }
                        p {
                            "Hanno Braun, self-employed software developer \
                            from the Odenwald region, Germany. The best way to \
                            reach me is via email ("
                            { util::email() }
                            ") and Matrix ("
                            a("href"="https://matrix.to/#/@hanno:braun-odw.eu")
                            {
                                "@hanno:braun-odw.eu"
                            }
                            ")."
                        }
                        // TASK: Add picture.
                    }
                    section {
                        h2 { "Ongoing Work" }
                        ul {
                            {
                                ongoing_work::ongoing_work(
                                    "Braun Embedded",
                                    "https://braun-embedded.com/",
                                    html! {
                                        p {
                                            "I provide software development \
                                            and consulting services, \
                                            specifically relating to Rust \
                                            firmware on ARM Cortex-M \
                                            microcontrollers. If you want to \
                                            write your next firmware project \
                                            in Rust and could use some help \
                                            with the low-level stuff, "
                                            {
                                                util::email()
                                                    .text("let me know")
                                            }
                                            "!"
                                        }
                                    }
                                )
                            }
                            {
                                ongoing_work::ongoing_work(
                                    "Made by Hanno",
                                    "https://madeby.hannobraun.de/",
                                    html! {
                                        p {
                                            "I have a small workshop where I \
                                            make semi-interesting stuff, \
                                            mainly 3D-printed."
                                        }
                                    }
                                )
                            }
                        }
                    }
                    { side_projects::side_projects(side_projects) }
                }

                hr {}

                { site_footer::site_footer() }
            }
        }
    }
}

// TASK: Return `Option<Element>` instead, only return `Some` if dev mode is
//       active.
fn base_if_dev_mode(dev: bool) -> Element {
    if dev {
        base(()).href("/hanno.braun-odw.eu/")
    } else {
        base(()).href("/")
    }
}
