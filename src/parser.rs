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

    #[test]
    fn test_h1() {
        let input = "# Heading One";

        let mut ast = parse(input).unwrap();

        let output = ast.next().unwrap();

        assert_eq!(output.as_rule(), Rule::h1);
        assert_eq!(output.as_str(), "# Heading One");
    }

    #[test]
    fn test_list_simple() {
        let input = r#"
- Bullet Point One
  - Bullet Point Inner
- Bullet Point Two
"#;

        let mut ast = parse(input).unwrap();
        dbg!(&ast);

        let output = ast.next().unwrap();

        assert_eq!(output.as_rule(), Rule::list);
    }
}
