use ::std::io::Write;
use std::{env, fs, process::exit};

mod compiler;
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
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Could not read user input.");
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

                let parsed = document.and_then(|document| match parser::parse(&document, text) {
                    Ok(program) => Some(program),
                    Err(err) => {
                        println!("{:?}", err);
                        None
                    }
                });

                let compiled = parsed.and_then(|parsed| match compiler::compile(&parsed, text) {
                    Ok(code) => Some(code),
                    Err(err) => {
                        println!("{:?}", err);
                        None
                    }
                });

                if let Some(compiled) = compiled {
                    println!("{}", compiled);
                }
            }
        };
    }
}

#[allow(dead_code)]
fn run(text: &str) -> i32 {
    let document = match lexer::lex(text) {
        Ok(lexed) => Some(lexed),
        Err(err) => {
            println!("{}", err.get_error(text));
            None
        }
    };

    let parsed = document.and_then(|document| match parser::parse(&document, text) {
        Ok(program) => Some(program),
        Err(err) => {
            println!("{:?}", err);
            None
        }
    });

    let compiled = parsed.and_then(|parsed| match compiler::compile(&parsed, text) {
        Ok(code) => Some(code),
        Err(err) => {
            println!("{:?}", err);
            None
        }
    });

    if let Some(compiled) = compiled {
        fs::write("out.asm", format!("{}", compiled)).expect("Unable to write file");
        return 0;
    }

    return -1;
}

fn main() {
    let args = env::args().skip(1);

    let mut run_shell = false;
    let mut input_file: Option<String> = None;

    for arg in args {
        match arg.as_str() {
            "-shell" => run_shell = true,
            arg => {
                if arg.starts_with("-") {
                    println!("Unknown argument: {}", arg);
                    exit(-1);
                } else {
                    input_file = Some(arg.to_string());
                }
            }
        }
    }

    if run_shell && input_file.is_some() {
        println!("Cannot use input file while running shell");
        exit(-1);
    }

    if run_shell {
        shell()
    } else if let Some(file) = input_file {
        let text = fs::read_to_string(file).expect("Unable to read file");
        exit(run(&text));
    } else {
        println!("No input file provided");
        exit(-1);
    }

    // if args.next().map(|arg| arg.eq("shell")).unwrap_or_else(false) {
    //     shell()
    // } else {
    //     run(text)
    // }
}
