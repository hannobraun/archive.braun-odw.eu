pub mod ongoing_work;
pub mod side_projects;
pub mod site_footer;
pub mod util;

use ssg::{
    html,
    html::{front_builder::*, Element},
};

use crate::data;

pub fn build(
    dev: bool,
    ongoing_work: data::OngoingWork,
    side_projects: data::SideProjects,
) -> Element {
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
                    { ongoing_work::ongoing_work(ongoing_work) }
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
