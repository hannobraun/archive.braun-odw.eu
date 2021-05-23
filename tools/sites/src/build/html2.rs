#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
#[derive(Debug, Eq, PartialEq)]
pub struct Element {
    pub name: &'static str,
    pub attributes: HashMap<&'static str, &'static str>,
    pub content: Vec<Content>,
}

#[cfg(test)]
#[derive(Debug, Eq, PartialEq)]
pub enum Content {
    Element(Element),
    Text(&'static str),
}

/// Macro to build HTML
///
/// Syntax heavily inspired by [Maple].
///
/// [Maple]: https://github.com/lukechu10/maple
#[cfg(test)]
macro_rules! html {
    // Rule for parsing an element, when it is known that we have all the syntax
    // required for an element. This exists to enable code reuse within the
    // macro as elements are parsed in two places (entry point and content
    // parsing directive).
    //
    // The entry point can't just call the content parsing directive, as the
    // entry point returns an `Element`, while the content parsing directive
    // returns `Content`.
    (@element
        $name:ident(
            $($attr_name:ident = $attr_value:expr),* $(,)?
        ) {
            $($content:tt)*
        }
    ) => {{
        let mut element = Element {
            name: stringify!($name),
            attributes: std::collections::HashMap::new(),
            content: vec![html!(@content $($content)*)],
        };

        $(
            element.attributes.insert(stringify!($attr_name), $attr_value);
        )*

        element
    }};

    // Content parsing directives. Each rule parses a different kind of content.
    (@content
        $name:ident(
            $($attr_name:ident = $attr_value:expr),* $(,)?
        ) {
            $($content:tt)*
        }
    ) => {{
        let element = html!(@element
            $name(
                $($attr_name=$attr_value),*
            ) {
                $($content)*
            }
        );

        Content::Element(element)
    }};
    (@content
        $text:expr
    ) => {{
        Content::Text($text)
    }};

    // Entry point to the macro.
    (
        $name:ident(
            $($attr_name:ident = $attr_value:expr),* $(,)?
        ) {
            $($content:tt)*
        }
    ) => {
        html!(@element
            $name(
                $($attr_name=$attr_value),*
            ) {
                $($content)*
            }
        )
    };
}

#[cfg(test)]
mod tests {
    use common_macros::hash_map;

    use super::{Content, Element};

    #[test]
    fn macro_should_create_element_with_text() {
        let html = html! {
            p(id="id", class="class") {
                "This is a paragraph."
            }
        };

        let expected = Element {
            name: "p",
            attributes: hash_map!("id" => "id", "class" => "class"),
            content: vec![Content::Text("This is a paragraph.")],
        };

        assert_eq!(html, expected);
    }

    #[test]
    fn macro_should_create_element_with_nested_element() {
        let html = html! {
            p(id="id", class="class") {
                a(href="https://site.example/", target="_href") {
                    "This is a link."
                }
            }
        };

        let mut expected = Element {
            name: "p",
            attributes: hash_map!("id" => "id", "class" => "class"),
            content: vec![Content::Element(Element {
                name: "a",
                attributes: hash_map!(
                    "href" => "https://site.example/",
                    "target" => "_href",
                ),
                content: vec![Content::Text("This is a link.")],
            })],
        };
        expected.attributes.insert("id", "id");
        expected.attributes.insert("class", "class");

        assert_eq!(html, expected);
    }
}
