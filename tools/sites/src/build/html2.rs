use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Element {
    pub name: &'static str,
    pub attributes: HashMap<&'static str, &'static str>,
    pub content: Vec<Content>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Content {
    Element(Element),
    Text(&'static str),
}

impl From<Element> for Content {
    fn from(element: Element) -> Self {
        Self::Element(element)
    }
}

impl From<&'static str> for Content {
    fn from(text: &'static str) -> Self {
        Self::Text(text)
    }
}

/// Macro to build HTML
///
/// Syntax heavily inspired by [Maple].
///
/// [Maple]: https://github.com/lukechu10/maple
macro_rules! html {
    // Content parsing directive for elements without attributes
    (@content $vec:expr,
        $name:ident {
            $($content:tt)*
        }
        $($rest:tt)*
    ) => {{
        // Just call the regular element parsing directive with an empty
        // attribute list.
        html!(@content $vec, $name() { $($content)* } $($rest)*);
    }};

    // Content parsing directive for elements
    (@content $vec:expr,
        $name:ident(
            // TASK: `$attr_name` needs to be a string, otherwise attributes
            //       that contain `-` cannot be represented.
            // TASK: Remove the comma, if possible. HTML doesn't have any here.
            $($attr_name:ident = $attr_value:expr),* $(,)?
        ) {
            $($content:tt)*
        }
        $($rest:tt)*
    ) => {{
        let mut element = Element {
            name: stringify!($name),
            attributes: std::collections::HashMap::new(),
            content: std::vec::Vec::new(),
        };

        $(
            element.attributes.insert(stringify!($attr_name), $attr_value);
        )*


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

    // Entry point to the macro
    // TASK: Make parameters more specific to make error message more clear.
    //       Currently any syntax error will result in an error message about
    //       the recursion limit.
    ($($html:tt)*) => {{
        let mut v: Vec<Element> = Vec::new();
        html!(@content &mut v, $($html)*);
        v.remove(0)
    }};
}

mod tests {
    use common_macros::hash_map;

    use super::{Content, Element};

    #[test]
    fn macro_should_create_element_with_text() {
        let html = html! {
            p {
                "This is a paragraph."
            }
        };

        let expected = Element {
            name: "p",
            attributes: hash_map!(),
            content: vec![Content::Text("This is a paragraph.")],
        };

        assert_eq!(html, expected);
    }

    #[test]
    fn macro_should_create_element_with_attributes() {
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
            p {
                strong { "This is a paragraph." }
            }
        };

        let expected = Element {
            name: "p",
            attributes: hash_map!(),
            content: vec![Content::Element(Element {
                name: "strong",
                attributes: hash_map!(),
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
            attributes: hash_map!(),
            content: vec![
                Content::Text("This is a paragraph with"),
                Content::Element(Element {
                    name: "strong",
                    attributes: hash_map!(),
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
            attributes: hash_map!(),
            content: vec![Content::Element(Element {
                name: "p",
                attributes: hash_map!(),
                content: vec![Content::Text("This is a paragraph.")],
            })],
        };

        assert_eq!(html, expected);
    }
}
