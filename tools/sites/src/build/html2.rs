#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
#[derive(Debug, Eq, PartialEq)]
pub struct Element {
    pub name: &'static str,
    pub attributes: HashMap<&'static str, &'static str>,
    pub content: &'static str,
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
            content: $content,
        };

        $(
            element.attributes.insert(stringify!($attr_name), $attr_value);
        )*

        element
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Element;

    #[test]
    fn macro_should_create_element_with_text() {
        let p = html! {
            p(id="id", class="class") {
                "This is a paragraph."
            }
        };

        let mut expected = Element {
            name: "p",
            attributes: HashMap::new(),
            content: "This is a paragraph.",
        };
        expected.attributes.insert("id", "id");
        expected.attributes.insert("class", "class");

        assert_eq!(p, expected);
    }
}
