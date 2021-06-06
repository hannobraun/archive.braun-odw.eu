use ssg::{
    html::{front_builder::*, Element},
    Component,
};

use crate::util;

#[derive(Component)]
pub struct SiteFooter;

impl From<SiteFooter> for Element {
    fn from(_: SiteFooter) -> Self {
        footer().with((site_footer_address(), made_in_odenwald()))
    }
}

#[derive(Component)]
struct SiteFooterAddress;

impl From<SiteFooterAddress> for Element {
    fn from(_: SiteFooterAddress) -> Self {
        address().with((
            p().with("Hanno Braun"),
            p().with("Untere Pfarrgasse 19"),
            p().with("64720 Michelstadt"),
            p().with("Germany"),
            hr(),
            p().with(util::email()),
        ))
    }
}

#[derive(Component)]
struct MadeInOdenwald;

impl From<MadeInOdenwald> for Element {
    fn from(_: MadeInOdenwald) -> Self {
        div().class("made-in-odenwald").with(
            a().href("made-in-odenwald/2.jpg")
                .with("Made in Odenwald ♥"),
        )
    }
}
