use pest::Parser as PestParser;
use pest::iterators::Pairs;
use pest_derive::Parser as PestParser;

#[derive(PestParser)]
#[grammar = "grammar.pest"]
struct Parser;

pub fn parse(input: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    let mut ast = Parser::parse(Rule::document, input)?;
    Ok(ast.next().unwrap().into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use std::sync::Arc;

    static AST: Lazy<Arc<Pairs<'_, Rule>>> = Lazy::new(|| {
        let unparsed_file = std::fs::read_to_string("example.lessink").unwrap();

        Mutex::new(parse(&unparsed_file).unwrap())
    });

    #[test]
    fn test_heading_1() {}

    #[test]
    fn test_heading_2() {}
}
