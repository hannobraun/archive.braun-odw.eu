use ssg::{
    args::Args,
    build::{
        build_once,
        html::{front_builder::*, model::Element},
    },
    html,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let html = Some(html(args.dev));
    build_once(&args.source, &args.target, html).await?;

    Ok(())
}

// TASK: Consider using a different, macro-less approach to generating HTML
//       code, maybe using typed-builder:
//       https://crates.io/crates/typed-builder
pub fn html(dev: bool) -> Element {
    html! {
        html("lang"="en") {
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
                    section {
                        h2 { "Ongoing Work" }
                        ul {
                            {
                                ongoing_work(
                                    "Braun Embedded",
                                    "https://braun-embedded.com/",
                                    html! {
                                        p {
                                            "I provide software development \
                                            and consulting services, \
                                            specifically relating to Rust \
                                            firmware on ARM Cortex-M \
                                            microcontrollers. If you want to \
                                            write your next firmware project \
                                            in Rust and could use some help \
                                            with the low-level stuff, "
                                            { email("let me know") }
                                            "!"
                                        }
                                    }
                                )
                            }
                            {
                                ongoing_work(
                                    "Made by Hanno",
                                    "https://madeby.hannobraun.de/",
                                    html! {
                                        p {
                                            "I have a small workshop where I \
                                            make semi-interesting stuff, \
                                            mainly 3D-printed."
                                        }
                                    }
                                )
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
                                {
                                    side_project_header(
                                        "Fornjot: Spacer",
                                        "2021-05-19",
                                    )
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
                                {
                                    side_project_header(
                                        "braun-odw.eu",
                                        "2021-04-06",
                                    )
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
                                {
                                    side_project_header(
                                        "My Boss: Contacts",
                                        "2021-03-05",
                                    )
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
    }
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

fn ongoing_work(
    title: &'static str,
    link: &'static str,
    content: Element,
) -> Element {
    html! {
        li {
            { ongoing_work_header(title, link) }
            { content }
        }
    }
}

fn ongoing_work_header(title: &'static str, link: &'static str) -> Element {
    header().with((h3().with(title), raw_link(link)))
}

fn side_project_header(title: &'static str, date: &'static str) -> Element {
    header().with((
        h3().with(title),
        p().class("date").with(("Finished ", span().with(date))),
    ))
}

fn raw_link(link: &'static str) -> Element {
    ext_link(link, link)
}

fn ext_link(link: &'static str, text: &'static str) -> Element {
    a().href(link).target("_blank").with(text)
}

fn email(text: &'static str) -> Element {
    a().href("mailto:Hanno%20Braun%20%3Channo@braun-odw.eu%3E")
        .with(text)
}
