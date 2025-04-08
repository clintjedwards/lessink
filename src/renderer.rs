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
            match node.as_rule() {
                Rule::h1 => render_h1(node, &mut self.output),
                Rule::h2 => render_h2(node, &mut self.output),
                Rule::h3 => render_h3(node, &mut self.output),
                Rule::h4 => render_h4(node, &mut self.output),
                Rule::h5 => render_h5(node, &mut self.output),
                Rule::h6 => render_h6(node, &mut self.output),
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

    write!(out, "<h1>{text}</h1>").unwrap();
}

fn render_h2(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::h2 {
        panic!("incorrect renderer");
    }

    let text = node.into_inner().next().unwrap().as_span().as_str();

    write!(out, "<h2>{text}</h2>").unwrap();
}

fn render_h3(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::h3 {
        panic!("incorrect renderer");
    }

    let text = node.into_inner().next().unwrap().as_span().as_str();

    write!(out, "<h3>{text}</h3>").unwrap();
}

fn render_h4(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::h4 {
        panic!("incorrect renderer");
    }

    let text = node.into_inner().next().unwrap().as_span().as_str();

    write!(out, "<h4>{text}</h4>").unwrap();
}

fn render_h5(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::h5 {
        panic!("incorrect renderer");
    }

    let text = node.into_inner().next().unwrap().as_span().as_str();

    write!(out, "<h5>{text}</h5>").unwrap();
}

fn render_h6(node: Pair<Rule>, out: &mut String) {
    if node.as_rule() != Rule::h6 {
        panic!("incorrect renderer");
    }

    let text = node.into_inner().next().unwrap().as_span().as_str();

    write!(out, "<h6>{text}</h6>").unwrap();
}
