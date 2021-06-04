use std::io::{self, Write};

#[derive(Debug, Eq, PartialEq)]
pub struct Element {
    pub name: &'static str,
    pub attributes: Vec<(&'static str, &'static str)>,
    pub content: Content,
}

impl Element {
    pub fn write_to(&self, target: &mut impl Write) -> io::Result<()> {
        write!(target, "<{}", self.name)?;

        for (name, value) in &self.attributes {
            write!(target, " {}=\"{}\"", name, value)?;
        }

        if self.content.0.is_empty() {
            write!(target, " />")?;
        } else {
            write!(target, ">")?;

            for child in &self.content.0 {
                child.write_to(target)?;
            }

            write!(target, "</{}>", self.name)?;
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Content(pub Vec<Node>);

impl Content {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Node {
    Element(Element),
    Text(&'static str),
}

impl Node {
    pub fn write_to(&self, target: &mut impl Write) -> io::Result<()> {
        match self {
            Self::Element(element) => element.write_to(target)?,
            Self::Text(text) => write!(target, "{}", text)?,
        }

        Ok(())
    }
}

impl From<Element> for Node {
    fn from(element: Element) -> Self {
        Self::Element(element)
    }
}

impl From<&'static str> for Node {
    fn from(text: &'static str) -> Self {
        Self::Text(text)
    }
}

#[cfg(test)]
mod tests {
    use super::{Content, Element, Node};

    #[test]
    fn element_should_write_html_code() {
        let element = Element {
            name: "p",
            attributes: vec![("class", "class")],
            content: Content(vec![
                Node::Element(Element {
                    name: "strong",
                    attributes: Vec::new(),
                    content: Content(vec![Node::Text("This is a paragraph.")]),
                }),
                Node::Element(Element {
                    name: "br",
                    attributes: Vec::new(),
                    content: Content(vec![]),
                }),
            ]),
        };

        let mut output = Vec::new();
        element.write_to(&mut output).unwrap();

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
