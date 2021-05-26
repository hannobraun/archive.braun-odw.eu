#[macro_use]
pub mod front_macro;
pub mod model;

use std::io::{self, Write};

use self::model::Element;

// TASK: Move this into a separate, site-specific application. Leave
//       infrastructure code in a library that is called from there.
pub fn build(dev: bool, target: &mut impl Write) -> io::Result<()> {
    let html = html! {
        html {
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

                { base(dev) }

                link(
                    "href"="style.css"
                    "rel"="stylesheet"
                    "type"="text/css"
                    "media"="all"
                ) {}
            }
            body {
                main {
                    h1 { "Hanno Braun" }

                    section {
                        h2 { "Who?" }
                        p {
                            "Hanno Braun, self-employed software developer \
                            from the Odenwald region, Germany. The best way to \
                            reach me is via email ("
                            { email("hanno@braun-odw.eu") }
                            ") and Matrix ("
                            a("href"="https://matrix.to/#/@hanno:braun-odw.eu")
                            {
                                "@hanno:braun-odw.eu"
                            }
                            ")."
                        }
                        // TASK: Add picture.
                    }
                    // TASK: Use `ext_link` to generate links.
                    section {
                        h2 { "Ongoing Work" }
                        ul {
                            li {
                                header {
                                    h3 { "Braun Embedded" }
                                    a("href"="https://braun-embedded.com/") {
                                        "https://braun-embedded.com/"
                                    }
                                }
                                p {
                                    "I provide software development and \
                                    consulting services, specifically relating \
                                    to Rust firmware on ARM Cortex-M \
                                    microcontrollers. If you want to write \
                                    your next firmware project in Rust and \
                                    could use some help with the low-level \
                                    stuff, "
                                    { email("let me know") }
                                    "!"
                                }
                            }
                            li {
                                header {
                                    h3 { "Flott" }
                                    a("href"="https://flott-motion.org/") {
                                        "https://flott-motion.org/"
                                    }
                                }
                                p {
                                    "Flott is an open source toolkit for \
                                    motion control software written in Rust. \
                                    I'm its creator and main developer. Please \
                                    consider "
                                    a("href"=
                                        "https://github.com/sponsors/hannobraun"
                                    )
                                    {
                                        "sponsoring me"
                                    }
                                    ", if you want to support this effort."
                                }
                            }
                            li {
                                header {
                                    h3 { "Made by Hanno" }
                                    a("href"="https://madeby.hannobraun.de/") {
                                        "https://madeby.hannobraun.de/"
                                    }
                                }
                                p {
                                    "I have a small workshop where I make \
                                    semi-interesting stuff, mainly 3D-printed."
                                }
                            }
                        }
                    }
                    section {
                        h2 { "Side Projects" }
                        p {
                            "I always have an ongoing side project, and I try \
                            to work on it every day. Here's a list of projects \
                            I've completed recently."
                        }
                        ol {
                            li {
                                header {
                                    h3 { "Fornjot: Spacer" }
                                    p("class"="date") {
                                        "Finished "
                                        span { "2021-05-19" }
                                    }
                                }
                                p {
                                    {
                                        ext_link(
                                            "https://github.com/hannobraun/fornjot",
                                            "Fornjot",
                                        )
                                    }
                                    " is an experimental programmatic CAD \
                                    system. I've been working on multiple \
                                    iterations of it over a few years, and \
                                    this is the first public release."
                                }
                                p {
                                    "The goal for this first release was to \
                                    build enough infrastructure to support "
                                    {
                                        ext_link(
                                            "https://github.com/hannobraun/fornjot/tree/main/models/spacer",
                                            "a simple spacer model",
                                        )
                                    }
                                    ". I plan to extend it and create more \
                                    complex models going forward."
                                }
                            }
                            li {
                                header {
                                    h3 { "braun-odw.eu" }
                                    p("class"="date") {
                                        "Finished "
                                        span { "2021-04-06" }
                                    }
                                }
                                p {
                                    "I revamped my website (the one you're \
                                    looking at right now) and put it on new \
                                    technical footing, using my own "
                                    {
                                        ext_link(
                                            "https://github.com/hannobraun/braun-odw.eu",
                                            "homegrown static site generator",
                                        )
                                    }
                                    ". I also got a new domain and started \
                                    consolidating most of my IT infrastructure \
                                    (including email and Matrix homeserver) \
                                    under it."
                                }
                            }
                            li {
                                header {
                                    h3 { "My Boss: Contacts" }
                                    p("class"="date") {
                                        "Finished "
                                        span { "2021-03-05" }
                                    }
                                }
                                p {
                                    {
                                        ext_link(
                                            "https://github.com/hannobraun/my-boss",
                                            "My Boss",
                                        )
                                    }
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
                                    possibly more."
                                }
                                p {
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
                                    easier (for me) than that."
                                }
                            }
                        }
                        p {
                            "There's a lot more on my GitHub accounts ("
                            {
                                ext_link(
                                    "https://github.com/hannobraun",
                                    "personal",
                                )
                            }
                            " and "
                            {
                                ext_link(
                                    "https://github.com/braun-embedded",
                                    "professional",
                                )
                            }
                            "), if you want to see more."
                        }
                    }
                }

                hr {}

                footer {
                    address {
                        p { "Hanno Braun" }
                        p { "Untere Pfarrgasse 19" }
                        p { "64720 Michelstadt" }
                        p { "Germany" }

                        hr {}

                        p {
                            {{ email("hanno@braun-odw.eu") }}
                        }
                    }

                    div("class"="made-in-odenwald") {
                        a("href"="made-in-odenwald/2.jpg") {
                            "Made in Odenwald ♥"
                        }
                    }
                }
            }
        }
    };

    writeln!(target, "<!DOCTYPE html>")?;
    html.write_to(target)?;

    Ok(())
}

fn base(dev: bool) -> Element {
    if dev {
        html! {

            base("href"="/hanno.braun-odw.eu/") {}
        }
    } else {
        html! {
            base("href"="/") {}
        }
    }
}

fn ext_link(link: &'static str, text: &'static str) -> Element {
    html! {
        a("href"=link "target"="_blank") {
            { text }
        }
    }
}

fn email(text: &'static str) -> Element {
    html! {
        a("href"="mailto:Hanno%20Braun%20%3Channo@braun-odw.eu%3E") {
            { text }
        }
    }
}
