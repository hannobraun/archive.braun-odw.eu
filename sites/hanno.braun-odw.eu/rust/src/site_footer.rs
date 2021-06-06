use ssg::html::{front_builder::*, Element};

use crate::util;

pub fn site_footer() -> Element {
    footer().with((site_footer_address(), made_in_odenwald()))
}

fn site_footer_address() -> Element {
    address().with((
        p().with("Hanno Braun"),
        p().with("Untere Pfarrgasse 19"),
        p().with("64720 Michelstadt"),
        p().with("Germany"),
        hr(),
        p().with(util::email()),
    ))
}

fn made_in_odenwald() -> Element {
    div().class("made-in-odenwald").with(
        a().href("made-in-odenwald/2.jpg")
            .with("Made in Odenwald ♥"),
    )
}
