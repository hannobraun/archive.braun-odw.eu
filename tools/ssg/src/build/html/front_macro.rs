/// Macro to build HTML
///
/// Syntax heavily inspired by [Maple].
///
/// [Maple]: https://github.com/lukechu10/maple
#[macro_export]
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
        let mut v: Vec<$crate::build::html::model::Element> = Vec::new();

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
        let mut element = $crate::build::html::model::Element {
            name: stringify!($name),
            attributes: Vec::new(),
            content: $crate::build::html::model::Content::new(),
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
        { $injected:expr }
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
    use crate::build::html::model::{Element, Node};

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
            content: vec![Node::Text("This is a paragraph.")],
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
            content: vec![Node::Text("This is a paragraph.")],
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
            content: vec![Node::Element(Element {
                name: "strong",
                attributes: Vec::new(),
                content: vec![Node::Text("This is a paragraph.")],
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
                Node::Text("This is a paragraph with"),
                Node::Element(Element {
                    name: "strong",
                    attributes: Vec::new(),
                    content: vec![Node::Text("mixed")],
                }),
                Node::Text("content."),
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
            content: vec![Node::Element(Element {
                name: "p",
                attributes: Vec::new(),
                content: vec![Node::Text("This is a paragraph.")],
            })],
        };

        assert_eq!(html, expected);
    }
}
