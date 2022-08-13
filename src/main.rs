mod lexer;
mod parser;

fn main() {
    let text = "5+ 5;";

    let document = lexer::lex(text).unwrap();
    println!("\n{:?}", document);

    let parsed = parser::parse(document, text);
    println!("\n{:?}", parsed);
}
