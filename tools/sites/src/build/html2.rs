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
    Text(&'static str),
}

/// Macro to build HTML
///
/// Syntax heavily inspired by [Maple].
///
/// [Maple]: https://github.com/lukechu10/maple
#[cfg(test)]
macro_rules! html {
    // Private parsing rule for element content.
    (@element
        $name:ident(
            $($attr_name:ident = $attr_value:expr),* $(,)?
        ) {
            $content:expr
        }
    ) => {{
        let mut element = Element {
            name: stringify!($name),
            attributes: std::collections::HashMap::new(),
            content: vec![Content::Text($content)],
        };

        $(
            element.attributes.insert(stringify!($attr_name), $attr_value);
        )*

        element
    }};

    // Public entry point to the macro.
    ($($html:tt)*) => {
        html!(@element $($html)*)
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
}
