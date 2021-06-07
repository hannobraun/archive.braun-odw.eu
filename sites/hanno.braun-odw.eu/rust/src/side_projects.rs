use ssg::{
    html::{front_builder::*, model::Element, Content},
    Component,
};

use crate::util;

// TASK: Download side projects from external data files. TOML with a `content`
//       key that contains Markdown would probably be best.
//
//       pulldown-cmark looks like a good option for handling and rendering the
//       Markdown.
pub fn side_projects() -> Element {
    section((
        h2("Side Projects"),
        p(
            "I always have an ongoing side project, and I try \
            to work on it every day. Here's a list of projects \
            I've completed recently.",
        ),
        ol((
            side_project()
                .title("Fornjot: Spacer")
                .date("2021-05-19")
                .description((
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
                    ))
                )),
            side_project()
                .title("braun-odw.eu")
                .date("2021-04-06")
                .description(
                    p((
                        "I revamped my website (the one you're \
                        looking at right now) and put it on new \
                        technical footing, using my own ",
                        util::ext_link(
                            "https://github.com/hannobraun/braun-odw.eu",
                        )
                        .text("homegrown static site generator"),
                        ". I also got a new domain and started \
                        consolidating most of my IT infrastructure \
                        (including email and Matrix homeserver) \
                        under it.",
                    ))
                ),
            side_project()
                .title("My Boss: Contacts")
                .date("2021-03-05")
                .description((
                    p((
                        util::ext_link(
                            "https://github.com/hannobraun/my-boss",
                        )
                        .text("My Boss"),
                        " is a command-line application that tells me \
                        what to do and when to do it. Kind of like an \
                        ERP system, except that I'm the opposite of an \
                        enterprise. This was the initial release that \
                        included contact management functionality, \
                        kind of like a CRM.",
                    )),
                    p(
                        "I use it every day to remember to keep in \
                        touch with my business contacts (and also some \
                        personal ones). I plan to extend it in the \
                        future to encompass more areas of my business, \
                        like basic bookkeeping and possibly more."
                    )
                )),
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

#[derive(Component)]
struct SideProject {
    title: Option<&'static str>,
    date: Option<&'static str>,
    description: Option<Content>,
}

impl From<SideProject> for Element {
    fn from(side_project: SideProject) -> Self {
        li((
            side_project_header()
                .title(side_project.title.unwrap())
                .date(side_project.date.unwrap()),
            side_project.description.unwrap(),
        ))
    }
}

#[derive(Component)]
struct SideProjectHeader {
    title: Option<&'static str>,
    date: Option<&'static str>,
}

impl From<SideProjectHeader> for Element {
    fn from(side_project_header: SideProjectHeader) -> Self {
        let title = side_project_header.title.unwrap();
        let date = side_project_header.date.unwrap();

        header((h3(title), p(("Finished ", span(date))).class("date")))
    }
}
