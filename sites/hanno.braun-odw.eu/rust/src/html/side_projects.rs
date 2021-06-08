use ssg::{
    html::{front_builder::*, Content, Element},
    Component,
};

use crate::data;

use super::util;

pub fn side_projects(
    side_projects: impl IntoIterator<Item = data::SideProject>,
) -> Element {
    let side_projects = side_projects
        .into_iter()
        .map(|project| side_project(project));

    section((
        h2("Side Projects"),
        p("I always have an ongoing side project, and I try \
            to work on it every day. Here's a list of projects \
            I've completed recently."),
        ol(Content::from_iter(side_projects)),
        p((
            "There's a lot more on my GitHub accounts (",
            util::ext_link("https://github.com/hannobraun").text("personal"),
            " and ",
            util::ext_link("https://github.com/braun-embedded")
                .text("professional"),
            "), if you want to see more.",
        )),
    ))
}

#[derive(Component)]
struct SideProject(data::SideProject);

impl From<SideProject> for Element {
    fn from(side_project: SideProject) -> Self {
        let title = side_project.0.title;
        let date = side_project.0.date;

        li((
            header((h3(title), p(("Finished ", span(date))).class("date"))),
            side_project.0.description,
        ))
    }
}
