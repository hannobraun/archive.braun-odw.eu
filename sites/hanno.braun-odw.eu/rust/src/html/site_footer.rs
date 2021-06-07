use ssg::{
    html::{front_builder::*, Element},
    Component,
};

use crate::html::util;

#[derive(Component)]
pub struct SiteFooter;

impl From<SiteFooter> for Element {
    fn from(_: SiteFooter) -> Self {
        footer((site_footer_address(), made_in_odenwald()))
    }
}

#[derive(Component)]
struct SiteFooterAddress;

impl From<SiteFooterAddress> for Element {
    fn from(_: SiteFooterAddress) -> Self {
        address((
            p("Hanno Braun"),
            p("Untere Pfarrgasse 19"),
            p("64720 Michelstadt"),
            p("Germany"),
            hr(()),
            p(util::email()),
        ))
    }
}

#[derive(Component)]
struct MadeInOdenwald;

impl From<MadeInOdenwald> for Element {
    fn from(_: MadeInOdenwald) -> Self {
        let link = a("Made in Odenwald ♥").href("made-in-odenwald/2.jpg");
        div(link).class("made-in-odenwald")
    }
}
