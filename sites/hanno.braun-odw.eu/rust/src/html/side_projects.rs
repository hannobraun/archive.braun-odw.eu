use ssg::{
    html::{front_builder::*, Content, Element},
    Component,
};

use crate::data;

use super::util;

#[derive(Component)]
pub struct SideProjects(data::SideProjects);

impl From<SideProjects> for Element {
    fn from(side_projects: SideProjects) -> Self {
        section((
            h2("Side Projects"),
            p("I always have an ongoing side project, and I try to work on it \
                every day. Here's a list of projects I've completed recently."),
            ol(Content::from_iter(
                side_projects
                    .0
                    .into_iter()
                    .map(|project| side_project(project)),
            )),
            p((
                "There's a lot more on my GitHub accounts (",
                util::ext_link("https://github.com/hannobraun")
                    .text("personal"),
                " and ",
                util::ext_link("https://github.com/braun-embedded")
                    .text("professional"),
                "), if you want to see more.",
            )),
        ))
    }
}

#[derive(Component)]
struct SideProject(data::SideProject);

impl From<SideProject> for Element {
    fn from(side_project: SideProject) -> Self {
        li((
            header((
                h3(side_project.0.title),
                p(("Finished ", span(side_project.0.date))).class("date"),
            )),
            side_project.0.description.render(),
        ))
    }
}
