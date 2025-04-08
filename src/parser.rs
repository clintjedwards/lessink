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
    use once_cell::unsync::Lazy;
    use std::cell::RefCell;

    thread_local! {
        static AST: Lazy<RefCell<Pairs<'static, Rule>>> = Lazy::new(|| {
            let unparsed_file = std::fs::read_to_string("test.lessink").unwrap();
            let unparsed_file = Box::leak(Box::new(unparsed_file));

            RefCell::new(parse(unparsed_file).unwrap())
        });
    }

    #[test]
    fn test_h1() {
        AST.with(|ast| {
            let next = ast.borrow_mut().next().unwrap();
            assert_eq!(next.as_rule(), Rule::h1);
        });
    }
}
