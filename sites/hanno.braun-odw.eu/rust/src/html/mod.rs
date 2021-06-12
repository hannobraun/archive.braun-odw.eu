pub mod ongoing_work;
pub mod side_projects;
pub mod site_footer;
pub mod util;
pub mod who;

use ssg::{
    html,
    html::{front_builder::*, Element},
    Component,
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
            { site_body(ongoing_work, side_projects) }
        }
    }
}

#[derive(Component)]
struct BaseIfDevMode {
    dev: bool,
}

impl From<BaseIfDevMode> for Element {
    fn from(base_if_dev_mode: BaseIfDevMode) -> Self {
        if base_if_dev_mode.dev {
            base(()).href("/hanno.braun-odw.eu/")
        } else {
            base(()).href("/")
        }
    }
}

#[derive(Component)]
struct SiteBody {
    ongoing_work: data::OngoingWork,
    side_projects: data::SideProjects,
}

impl From<SiteBody> for Element {
    fn from(site_body: SiteBody) -> Self {
        body((
            main((
                h1("Hanno Braun"),
                who::who(),
                ongoing_work::ongoing_work(site_body.ongoing_work),
                side_projects::side_projects(site_body.side_projects),
            )),
            hr(()),
            site_footer::site_footer(),
        ))
    }
}
