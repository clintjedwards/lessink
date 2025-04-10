use crate::parser::Rule;
use pest::iterators::{Pair, Pairs};
use std::fmt::Write;

pub struct HtmlRenderer {
    pub output: String,
}

impl HtmlRenderer {
    pub fn new() -> Self {
        HtmlRenderer {
            output: String::new(),
        }
    }

    pub fn render(mut self, ast: Pairs<'_, Rule>) -> String {
        for node in ast {
            dbg!(&node);

            match node.as_rule() {
                Rule::h1 => render_h1(node, &mut self.output),
                Rule::h2 => render_h2(node, &mut self.output),
                Rule::h3 => render_h3(node, &mut self.output),
                Rule::h4 => render_h4(node, &mut self.output),
                Rule::h5 => render_h5(node, &mut self.output),
                Rule::h6 => render_h6(node, &mut self.output),
                Rule::paragraph => render_paragraph(node, &mut self.output),
                Rule::list => render_list(node, &mut self.output),
                Rule::numbered_list => render_numbered_list(node, &mut self.output),
                Rule::codeblock => render_code_block(node, &mut self.output),
                Rule::admonition => render_admonition(node, &mut self.output),
                Rule::footnote_def => render_footnote(node, &mut self.output),
                _ => {}
            }
            writeln!(self.output).unwrap();
        }

        self.output
    }
}

fn render_h1(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::h1 {
        panic!("incorrect renderer");
    }

    let text = node.into_inner().next().unwrap().as_span().as_str();

    writeln!(out, "<h1>{text}</h1>").unwrap();
}

fn render_h2(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::h2 {
        panic!("incorrect renderer");
    }

    let text = node.into_inner().next().unwrap().as_span().as_str();

    writeln!(out, "<h2>{text}</h2>").unwrap();
}

fn render_h3(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::h3 {
        panic!("incorrect renderer");
    }

    let text = node.into_inner().next().unwrap().as_span().as_str();

    writeln!(out, "<h3>{text}</h3>").unwrap();
}

fn render_h4(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::h4 {
        panic!("incorrect renderer");
    }

    let text = node.into_inner().next().unwrap().as_span().as_str();

    writeln!(out, "<h4>{text}</h4>").unwrap();
}

fn render_h5(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::h5 {
        panic!("incorrect renderer");
    }

    let text = node.into_inner().next().unwrap().as_span().as_str();

    writeln!(out, "<h5>{text}</h5>").unwrap();
}

fn render_h6(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::h6 {
        panic!("incorrect renderer");
    }

    let text = node.into_inner().next().unwrap().as_span().as_str();

    writeln!(out, "<h6>{text}</h6>").unwrap();
}

fn render_paragraph(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::paragraph {
        panic!("incorrect renderer");
    }

    let text = node.into_inner().next().unwrap().as_span().as_str();

    writeln!(out, "<p>{text}</p>").unwrap();
}

fn render_list(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::list {
        panic!("incorrect renderer");
    }

    let mut list = node.into_inner();
    let _list_level = list.next().unwrap();
    let list_items = list.next().unwrap().into_inner();

    writeln!(out, "<ul>").unwrap();

    for list_item in list_items {
        let list_item = list_item.into_inner().next().unwrap().as_span().as_str();
        writeln!(out, "  <li>{list_item}</li>").unwrap();
    }

    writeln!(out, "</ul>").unwrap();
}

fn render_numbered_list(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::numbered_list {
        panic!("incorrect renderer");
    }

    let list_items = node.into_inner();

    writeln!(out, "<ol>").unwrap();

    for list_item in list_items {
        let mut list_item = list_item.into_inner();
        let _list_item_number = list_item.next().unwrap();
        let list_item_text = list_item.next().unwrap().as_span().as_str();
        writeln!(out, "  <li>{list_item_text}</li>").unwrap();
    }

    writeln!(out, "</ol>").unwrap();
}

fn render_code_block(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::codeblock {
        panic!("incorrect renderer");
    }

    let mut block = node.into_inner();
    let language = block.next().unwrap().as_span().as_str();
    let code = block.next().unwrap().as_span().as_str();

    writeln!(out, "<pre><code class=\"language-{language}\">").unwrap();
    writeln!(out, "{code}").unwrap();
    writeln!(out, "</code></pre>").unwrap();
}

fn render_admonition(node: Pair<Rule>, out: &mut String) {
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

    writeln!(out, "<div>").unwrap();
    writeln!(out, "  <div>{kind}</div>").unwrap();
    writeln!(out, "  <p>{content}</p>").unwrap();
    writeln!(out, "</div>").unwrap();
}

fn render_footnote(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::footnote_def {
        panic!("incorrect renderer");
    }

    let mut block = node.into_inner();
    let number = block.next().unwrap().as_span().as_str();
    let content = block.next().unwrap().as_span().as_str();

    writeln!(out, "<p>{number}: {content}</p>").unwrap();
}
