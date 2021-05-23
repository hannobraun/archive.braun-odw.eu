#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
#[derive(Debug, Eq, PartialEq)]
pub struct Element {
    pub name: &'static str,
    pub attributes: HashMap<&'static str, &'static str>,
    pub content: Content,
}

#[cfg(test)]
#[derive(Debug, Eq, PartialEq)]
pub enum Content {
    Text(&'static str),
}

/// Macro to build HTML
///
/// Syntax heavily inspired by [Maple].
///
/// [Maple]: https://github.com/lukechu10/maple
#[cfg(test)]
macro_rules! html {
    (
        $name:ident(
            $($attr_name:ident = $attr_value:expr),* $(,)?
        ) {
            $content:expr
        }
    ) => {{
        let mut element = Element {
            name: stringify!($name),
            attributes: std::collections::HashMap::new(),
            content: Content::Text($content),
        };

        $(
            element.attributes.insert(stringify!($attr_name), $attr_value);
        )*

        element
    }};
}

#[cfg(test)]
mod tests {
    use common_macros::hash_map;

    use super::{Content, Element};

    #[test]
    fn macro_should_create_element_with_text() {
        let p = html! {
            p(id="id", class="class") {
                "This is a paragraph."
            }
        };

        let expected = Element {
            name: "p",
            attributes: hash_map!("id" => "id", "class" => "class"),
            content: Content::Text("This is a paragraph."),
        };

        assert_eq!(p, expected);
    }
}
