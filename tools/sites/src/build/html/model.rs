use std::io::{self, Write};

#[derive(Debug, Eq, PartialEq)]
pub struct Element {
    pub name: &'static str,
    pub attributes: Vec<(&'static str, &'static str)>,
    pub content: Vec<Content>,
}

impl Element {
    pub fn write_to(&self, target: &mut impl Write) -> io::Result<()> {
        write!(target, "<{}", self.name)?;

        for (name, value) in &self.attributes {
            write!(target, " {}=\"{}\"", name, value)?;
        }

        if self.content.is_empty() {
            write!(target, " />")?;
        } else {
            write!(target, ">")?;

            for child in &self.content {
                child.write_to(target)?;
            }

            write!(target, "</{}>", self.name)?;
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Content {
    Element(Element),
    Text(&'static str),
}

impl Content {
    pub fn write_to(&self, target: &mut impl Write) -> io::Result<()> {
        match self {
            Self::Element(element) => element.write_to(target)?,
            Self::Text(text) => write!(target, "{}", text)?,
        }

        Ok(())
    }
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

#[cfg(test)]
mod tests {
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
