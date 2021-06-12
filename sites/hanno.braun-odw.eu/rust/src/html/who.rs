use ssg::{
    html::{front_builder::*, Element},
    Component,
};

use super::util;

#[derive(Component)]
pub struct Who;

impl From<Who> for Element {
    fn from(_: Who) -> Self {
        // TASK: Add picture.
        section((
            h2("Who?"),
            p((
                "Hanno Braun, self-employed software developer \
                from the Odenwald region, Germany. The best way to \
                reach me is via email (",
                util::email(),
                ") and Matrix (",
                a("@hanno:braun-odw.eu")
                    .href("https://matrix.to/#/@hanno:braun-odw.eu"),
                ").",
            )),
        ))
    }
}
