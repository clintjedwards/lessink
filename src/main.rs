mod parser;
mod renderer;

fn main() {
    let unparsed_file = std::fs::read_to_string("test.lessink").unwrap();

    let ast = parser::parse(&unparsed_file).unwrap();
    let html = renderer::HtmlRenderer::new();

    dbg!(html.render(ast));
}
