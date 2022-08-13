mod lexer;
mod parser;

fn run(text: &str) -> Result<(), ()> {
    let document = match lexer::lex(text) {
        Ok(lexed) => lexed,
        Err(err) => {
            println!("{}", err.get_error(text));
            return Err(());
        }
    };
    println!("\n{:?}", document);

    let parsed = parser::parse(document, text);
    println!("\n{:?}", parsed);

    return Ok(())
}

fn main() {
    let text = "
        let a = \"5\";
        let b = \"1;
        let c = 2;
    ";

    match run(text) {
        Ok(_) => {},
        Err(_) => println!("Program exited with error")
    };
}
