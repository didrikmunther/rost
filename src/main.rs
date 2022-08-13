use ::std::io::{Write};

mod lexer;
mod parser;

fn flush() {
    std::io::stdout().flush().expect("Flush failed.");
}

fn shell() {
    let mut code = String::new();

    loop {
        print!("> ");
        flush();

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("Could not read user input.");
        let text = buf.as_ref();

        code.push_str(&buf);

        match buf.as_ref() {
            "quit\n" => break,
            _ => {
                let document = match lexer::lex(text) {
                    Ok(lexed) => Some(lexed),
                    Err(err) => {
                        println!("{}", err.get_error(text));
                        None
                    }
                };

                let parsed = document.and_then(|document| {
                    match parser::parse(&document, text) {
                        Ok(program) => Some(program),
                        Err(err) => {
                            println!("{:?}", err);
                            None
                        }
                    }
                });

                println!("{:#?}", parsed);
            }
        };
    }
}

#[allow(dead_code)]
fn run(text: &str) -> Result<(), ()> {
    let document = match lexer::lex(text) {
        Ok(lexed) => lexed,
        Err(err) => {
            println!("{}", err.get_error(text));
            return Err(());
        }
    };
    println!("\n{:?}", document);

    let parsed = parser::parse(&document, text);
    println!("\n{:#?}", parsed);

    return Ok(())
}

fn main() {
    shell()
}
