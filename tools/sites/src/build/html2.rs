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

#[cfg(test)]
impl From<Element> for Content {
    fn from(element: Element) -> Self {
        Self::Element(element)
    }
}

#[cfg(test)]
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
#[cfg(test)]
macro_rules! html {
    // Parsing directive for element content
    (@content $vec:expr,
        $name:ident(
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

    // Parsing directive for text content
    (@content $vec:expr,
        $text:literal
        $($rest:tt)*
    ) => {{
        $vec.push($text.into());
        html!(@content $vec, $($rest)*);
    }};

    // Parsing directive to terminate parsing once no content is left
    (@content $vec:expr,) => {};

    // Entry point to the macro
    ($($html:tt)*) => {{
        let mut v: Vec<Element> = Vec::new();
        html!(@content &mut v, $($html)*);
        v.remove(0)
    }};
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

        let expected = Element {
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

        assert_eq!(html, expected);
    }

    #[test]
    fn macro_should_create_element_with_mixed_content() {
        let html = html! {
            p() {
                "This is a paragraph with"
                strong() {
                    "mixed"
                }
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
}
