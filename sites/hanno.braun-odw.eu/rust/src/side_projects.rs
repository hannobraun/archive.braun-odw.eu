use ssg::{
    html,
    html::{front_builder::*, model::Element, Content},
    Component,
};

use crate::util;

pub fn side_projects() -> Element {
    html! {
        section {
            h2 { "Side Projects" }
            p {
                "I always have an ongoing side project, and I try \
                to work on it every day. Here's a list of projects \
                I've completed recently."
            }
            ol {
                {
                    side_project()
                        .title("Fornjot: Spacer")
                        .date("2021-05-19")
                        .content((
                            p().with((
                                util::ext_link(
                                    "https://github.com/hannobraun/fornjot",
                                )
                                .text("Fornjot"),
                                " is an experimental programmatic CAD \
                                system. I've been working on multiple \
                                iterations of it over a few years, and \
                                this is the first public release.",
                            )),
                            p().with((
                                "The goal for this first release was to \
                                build enough infrastructure to support ",
                                util::ext_link(
                                    "https://github.com/hannobraun/fornjot/tree/main/models/spacer",
                                )
                                .text("a simple spacer model"),
                                ". I plan to extend it and create more \
                                complex models going forward.",
                            ))
                        ))
                }
                {
                    side_project()
                        .title("braun-odw.eu")
                        .date("2021-04-06")
                        .content(
                            p().with((
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
                        )
                }
                {
                    side_project()
                        .title("My Boss: Contacts")
                        .date("2021-03-05")
                        .content((
                            p().with((
                                util::ext_link(
                                    "https://github.com/hannobraun/my-boss",
                                )
                                .text("My Boss"),
                                " is software that tells me what to do and \
                                when to do it. Kind of like an ERP system, \
                                except that I'm the opposite of an \
                                enterprise. This was the initial release \
                                that included contact management \
                                functionality, kind of like a CRM. I use \
                                it every day to remember to keep in touch \
                                with my business contacts (and actually \
                                also some personal ones). I plan to extend \
                                it in the future to encompass more areas \
                                of my business, like basic bookkeeping and \
                                possibly more.",
                            )),
                            p().with(
                                "I still don't know how I feel about \
                                re-inventing the wheel like this, but as \
                                long as the alternatives are really \
                                complicated, hard to maintain, proprietary \
                                web software that I could lose access to \
                                any moment, or otherwise don't suit my \
                                needs, I guess I'll keep doing it. My Boss \
                                is a command-line application and stores \
                                its data in TOML files that I keep in a \
                                Git repository, and it doesn't get much \
                                easier (for me) than that.",
                            )
                        ))
                }
            }
            p {
                "There's a lot more on my GitHub accounts ("
                {
                    util::ext_link("https://github.com/hannobraun")
                        .text("personal")
                }
                " and "
                {
                    util::ext_link("https://github.com/braun-embedded")
                        .text("professional")
                }
                "), if you want to see more."
            }
        }
    }
}

#[derive(Component)]
struct SideProject {
    title: Option<&'static str>,
    date: Option<&'static str>,
    content: Option<Content>,
}

impl From<SideProject> for Element {
    fn from(side_project: SideProject) -> Self {
        li().with((
            side_project_header()
                .title(side_project.title.unwrap())
                .date(side_project.date.unwrap()),
            side_project.content.unwrap(),
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

        header().with((
            h3().with(title),
            p().class("date").with(("Finished ", span().with(date))),
        ))
    }
}
