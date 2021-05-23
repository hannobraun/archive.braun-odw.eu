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
    // Content parsing directives. Each rule parses a different kind of content.
    (@content
        $name:ident(
            $($attr_name:ident = $attr_value:expr),* $(,)?
        ) {
            $($content:tt)*
        }
    ) => {{
        let mut element = Element {
            name: stringify!($name),
            attributes: std::collections::HashMap::new(),
            content: vec![html!(@content $($content)*).into()],
        };

        $(
            element.attributes.insert(stringify!($attr_name), $attr_value);
        )*

        element
    }};
    (@content
        $text:expr
    ) => {{
        $text
    }};

    // Entry point to the macro.
    (
        $name:ident(
            $($attr_name:ident = $attr_value:expr),* $(,)?
        ) {
            $($content:tt)*
        }
    ) => {
        html!(@content
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
