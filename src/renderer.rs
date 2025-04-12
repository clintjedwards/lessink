use crate::parser::Rule;
use pest::iterators::{Pair, Pairs};
use std::fmt::Write;

pub struct Html {
    pub output: String,
}

impl Html {
    pub fn render(ast: Pairs<'_, Rule>) -> String {
        let mut html_renderer = Html {
            output: String::new(),
        };

        html_renderer.render_html(ast);

        html_renderer.output
    }

    fn render_html(&mut self, nodes: Pairs<'_, Rule>) {
        for node in nodes {
            match node.as_rule() {
                Rule::h1 => self.render_h1(node),
                Rule::h2 => self.render_h2(node),
                Rule::h3 => self.render_h3(node),
                Rule::h4 => self.render_h4(node),
                Rule::h5 => self.render_h5(node),
                Rule::h6 => self.render_h6(node),
                Rule::paragraph => self.render_paragraph(node),
                Rule::list => self.render_list(node),
                Rule::numbered_list => self.render_numbered_list(node),
                Rule::codeblock => self.render_code_block(node),
                Rule::admonition => self.render_admonition(node),
                Rule::footnote_def => self.render_footnote(node),
                _ => {}
            }

            writeln!(self.output).unwrap();
        }
    }

    fn render_h1(&mut self, node: Pair<Rule>) {
        if node.as_rule() != Rule::h1 {
            panic!("incorrect renderer");
        }

        let text = node.into_inner().next().unwrap().as_span().as_str();

        writeln!(self.output, "<h1>{text}</h1>").unwrap();
    }

    fn render_h2(&mut self, node: Pair<Rule>) {
        if node.as_rule() != Rule::h2 {
            panic!("incorrect renderer");
        }

        let text = node.into_inner().next().unwrap().as_span().as_str();

        writeln!(self.output, "<h2>{text}</h2>").unwrap();
    }

    fn render_h3(&mut self, node: Pair<Rule>) {
        if node.as_rule() != Rule::h3 {
            panic!("incorrect renderer");
        }

        let text = node.into_inner().next().unwrap().as_span().as_str();

        writeln!(self.output, "<h3>{text}</h3>").unwrap();
    }

    fn render_h4(&mut self, node: Pair<Rule>) {
        if node.as_rule() != Rule::h4 {
            panic!("incorrect renderer");
        }

        let text = node.into_inner().next().unwrap().as_span().as_str();

        writeln!(self.output, "<h4>{text}</h4>").unwrap();
    }

    fn render_h5(&mut self, node: Pair<Rule>) {
        if node.as_rule() != Rule::h5 {
            panic!("incorrect renderer");
        }

        let text = node.into_inner().next().unwrap().as_span().as_str();

        writeln!(self.output, "<h5>{text}</h5>").unwrap();
    }

    fn render_h6(&mut self, node: Pair<Rule>) {
        if node.as_rule() != Rule::h6 {
            panic!("incorrect renderer");
        }

        let text = node.into_inner().next().unwrap().as_span().as_str();

        writeln!(self.output, "<h6>{text}</h6>").unwrap();
    }

    fn render_paragraph(&mut self, node: Pair<Rule>) {
        if node.as_rule() != Rule::paragraph {
            panic!("incorrect renderer");
        }

        let text = node.into_inner().next().unwrap().as_span().as_str();

        writeln!(self.output, "<p>{text}</p>").unwrap();
    }

    fn render_list(&mut self, node: Pair<Rule>) {
        if node.as_rule() != Rule::list {
            panic!("incorrect renderer");
        }

        let mut list = node.into_inner();
        let _list_level = list.next().unwrap();
        let list_items = list.next().unwrap().into_inner();

        writeln!(self.output, "<ul>").unwrap();

        for list_item in list_items {
            let list_item = list_item.into_inner().next().unwrap().as_span().as_str();
            writeln!(self.output, "  <li>{list_item}</li>").unwrap();
        }

        writeln!(self.output, "</ul>").unwrap();
    }

    fn render_numbered_list(&mut self, node: Pair<Rule>) {
        if node.as_rule() != Rule::numbered_list {
            panic!("incorrect renderer");
        }

        let list_items = node.into_inner();

        writeln!(self.output, "<ol>").unwrap();

        for list_item in list_items {
            let mut list_item = list_item.into_inner();
            let _list_item_number = list_item.next().unwrap();
            let list_item_text = list_item.next().unwrap().as_span().as_str();
            writeln!(self.output, "  <li>{list_item_text}</li>").unwrap();
        }

        writeln!(self.output, "</ol>").unwrap();
    }

    fn render_code_block(&mut self, node: Pair<Rule>) {
        if node.as_rule() != Rule::codeblock {
            panic!("incorrect renderer");
        }

        let mut block = node.into_inner();
        let language = block.next().unwrap().as_span().as_str();
        let code = block.next().unwrap().as_span().as_str();

        writeln!(self.output, "<pre><code class=\"language-{language}\">").unwrap();
        writeln!(self.output, "{code}").unwrap();
        writeln!(self.output, "</code></pre>").unwrap();
    }

    fn render_admonition(&mut self, node: Pair<Rule>) {
        if node.as_rule() != Rule::admonition {
            panic!("incorrect renderer");
        }

        let mut block = node.into_inner();
        let kind = block
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap()
            .as_span()
            .as_str();
        let content = block.next().unwrap().as_span().as_str();

        writeln!(self.output, "<div>").unwrap();
        writeln!(self.output, "  <div>{kind}</div>").unwrap();
        writeln!(self.output, "  <p>{content}</p>").unwrap();
        writeln!(self.output, "</div>").unwrap();
    }

    fn render_footnote(&mut self, node: Pair<Rule>) {
        if node.as_rule() != Rule::footnote_def {
            panic!("incorrect renderer");
        }

        let mut block = node.into_inner();
        let number = block.next().unwrap().as_span().as_str();
        let content = block.next().unwrap().as_span().as_str();

        writeln!(self.output, "<p>{number}: {content}</p>").unwrap();
    }
}
