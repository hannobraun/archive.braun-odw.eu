pub mod model;

use std::io::{self, Write};

use self::model::Element;

/// Macro to build HTML
///
/// Syntax heavily inspired by [Maple].
///
/// [Maple]: https://github.com/lukechu10/maple
macro_rules! html {
    // Entry point to the macro
    (
        $name:ident $((
            $($attr_name:literal = $attr_value:expr)*
        ))? {
            $($content:tt)*
        }
        $($rest:tt)*
    ) => {{
        let mut v: Vec<Element> = Vec::new();

        html!(@content &mut v,
            $name $((
                $($attr_name = $attr_value)*
            ))? {
                $($content)*
            }
            $($rest)*
        );

        v.remove(0)
    }};

    // Content parsing directive for elements
    (@content $vec:expr,
        $name:ident $((
            $($attr_name:literal = $attr_value:expr)*
        ))? {
            $($content:tt)*
        }
        $($rest:tt)*
    ) => {{
        #[allow(unused_mut)]
        let mut element = Element {
            name: stringify!($name),
            attributes: Vec::new(),
            content: Vec::new(),
        };

        $(
            $(
                element.attributes.push(($attr_name, $attr_value));
            )*
        )?


        html!(@content &mut element.content, $($content)*);

        $vec.push(element.into());
        html!(@content $vec, $($rest)*);
    }};

    // Content parsing directive for text
    (@content $vec:expr,
        $text:literal
        $($rest:tt)*
    ) => {{
        $vec.push($text.into());
        html!(@content $vec, $($rest)*);
    }};

    // Content parsing directive for injected content
    (@content $vec:expr,
        { $injected:ident }
        $($rest:tt)*
    ) => {{
        $vec.push($injected.into());
        html!(@content $vec, $($rest)*);
    }};

    // Content parsing directive to terminate parsing once no content is left
    (@content $vec:expr,) => {};
}

#[cfg(test)]
mod tests {
    use super::model::{Content, Element};

    #[test]
    fn macro_should_create_element_with_text() {
        let html = html! {
            p {
                "This is a paragraph."
            }
        };

        let expected = Element {
            name: "p",
            attributes: Vec::new(),
            content: vec![Content::Text("This is a paragraph.")],
        };

        assert_eq!(html, expected);
    }

    #[test]
    fn macro_should_create_element_with_attributes() {
        let html = html! {
            p("id"="id" "class"="class") {
                "This is a paragraph."
            }
        };

        let expected = Element {
            name: "p",
            attributes: vec![("id", "id"), ("class", "class")],
            content: vec![Content::Text("This is a paragraph.")],
        };

        assert_eq!(html, expected);
    }

    #[test]
    fn macro_should_create_element_with_nested_element() {
        let html = html! {
            p {
                strong { "This is a paragraph." }
            }
        };

        let expected = Element {
            name: "p",
            attributes: Vec::new(),
            content: vec![Content::Element(Element {
                name: "strong",
                attributes: Vec::new(),
                content: vec![Content::Text("This is a paragraph.")],
            })],
        };

        assert_eq!(html, expected);
    }

    #[test]
    fn macro_should_create_element_with_mixed_content() {
        let html = html! {
            p {
                "This is a paragraph with"
                strong { "mixed" }
                "content."
            }
        };

        let expected = Element {
            name: "p",
            attributes: Vec::new(),
            content: vec![
                Content::Text("This is a paragraph with"),
                Content::Element(Element {
                    name: "strong",
                    attributes: Vec::new(),
                    content: vec![Content::Text("mixed")],
                }),
                Content::Text("content."),
            ],
        };

        assert_eq!(html, expected);
    }

    #[test]
    fn macro_should_create_element_with_injected_content() {
        let injected = html! {
            p {
                "This is a paragraph."
            }
        };

        let html = html! {
            div {
                { injected }
            }
        };

        let expected = Element {
            name: "div",
            attributes: Vec::new(),
            content: vec![Content::Element(Element {
                name: "p",
                attributes: Vec::new(),
                content: vec![Content::Text("This is a paragraph.")],
            })],
        };

        assert_eq!(html, expected);
    }

    #[test]
    fn element_should_write_html_code() {
        let html = html! {
            p("class"="class") {
                strong { "This is a paragraph." }
                br {}
            }
        };

        let mut output = Vec::new();
        html.write_to(&mut output).unwrap();

        let expected = "\
            <p class=\"class\">\
                <strong>This is a paragraph.</strong>\
                <br />\
            </p>\
        ";

        println!("expected: {}", expected);
        println!("actual: {}", String::from_utf8(output.clone()).unwrap());

        assert_eq!(output, expected.as_bytes().to_vec());
    }
}

// TASK: Move this into a separate, site-specific application. Leave
//       infrastructure code in a library that is called from there.
pub fn build(dev: bool, target: &mut impl Write) -> io::Result<()> {
    let base = if dev {
        html! {

            base("href"="/hanno.braun-odw.eu/") {}
        }
    } else {
        html! {
            base("href"="/") {}
        }
    };
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

                { base }

                link(
                    "href"="style.css"
                    "rel"="stylesheet"
                    "type"="text/css"
                    "media"="all"
                ) {}
            }
            body {
                // TASK: Add name to `mailto` links.
                main {
                    h1 { "Hanno Braun" }

                    section {
                        h2 { "Who?" }
                        p {
                            "Hanno Braun, self-employed software developer \
                            from the Odenwald region, Germany. The best way to \
                            reach me is via email ("
                            a("href"="mailto:hanno@braun-odw.eu") {
                                "hanno@braun-odw.eu"
                            }
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
                                    a("href"="mailto:hanno@braun-embedded.com")
                                    {
                                        "let me know"
                                    }
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
                    // TASK: Look into removing `class`. I don't think it's
                    //       used.
                    section("class"="side-projects") {
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
                                    a(
                                        "href"=
                                        "https://github.com/hannobraun/fornjot" "target"="_blank"
                                    ) {
                                        "Fornjot"
                                    }
                                    " is an experimental programmatic CAD \
                                    system. I've been working on multiple \
                                    iterations of it over a few years, and \
                                    this is the first public release."
                                }
                                p {
                                    "The goal for this first release was to \
                                    build enough infrastructure to support "
                                    a(
                                        "href"=
                                        "https://github.com/hannobraun/fornjot/tree/main/models/spacer" "target"="_blank"
                                    ) {
                                        "a simple spacer model"
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
                                    a(
                                        "href"=
                                        "https://github.com/hannobraun/braun-odw.eu"
                                        "target"="_blank"
                                    ) {
                                        "homegrown static site generator"
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
                                    a(
                                        "href"=
                                        "https://github.com/hannobraun/my-boss"
                                        "target"="_blank"
                                    ) {
                                        "My Boss"
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
                            a(
                                "href"="https://github.com/hannobraun" "target"="_blank"
                            ) {
                                "personal"
                            }
                            " and "
                            a(
                                "href"="https://github.com/braun-embedded" "target"="_blank"
                            ) {
                                "professional"
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
                            a("href"="mailto:hanno@braun-odw.eu") {
                                "hanno@braun-odw.eu"
                            }
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
