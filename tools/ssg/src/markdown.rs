use pulldown_cmark::{html::push_html, Parser};

use crate::html::Node;

pub struct Markdown(pub &'static str);

impl Markdown {
    // TASK: Accept argument to control whether links are rendered with
    //       `target="_blank"`.
    pub fn render(&self) -> Node {
        let parser = Parser::new(self.0);

        let mut html = String::new();
        push_html(&mut html, parser);

        Node::Raw(html)
    }
}
