mod ongoing_work;
mod side_projects;
mod site_footer;
mod util;

use ssg::{
    html,
    html::{front_builder::*, Element},
};

use crate::data;

// TASK: Pass side projects in as an argument.
pub fn build(dev: bool) -> Element {
    let side_projects = data::SideProjects::load();

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
