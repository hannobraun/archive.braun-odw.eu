use ssg::{
    build::html::{front_builder::*, model::Element},
    html,
};

use crate::util;

pub fn site_footer() -> Element {
    html! {
        footer {
            { site_footer_address() }
            { made_in_odenwald() }
        }
    }
}

fn site_footer_address() -> Element {
    address().with((
        p().with("Hanno Braun"),
        p().with("Untere Pfarrgasse 19"),
        p().with("64720 Michelstadt"),
        p().with("Germany"),
        hr(),
        p().with(util::email("hanno@braun-odw.eu")),
    ))
}

fn made_in_odenwald() -> Element {
    div().class("made-in-odenwald").with(
        a().href("made-in-odenwald/2.jpg")
            .with("Made in Odenwald ♥"),
    )
}
