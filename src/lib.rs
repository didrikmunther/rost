use ::std::io::Write;
use std::{fs, process::exit};

use clap::Subcommand;
use compiler::program::Program;

use crate::error::RostError;

pub mod compiler;
pub mod error;
pub mod lexer;
pub mod parser;

#[derive(Subcommand, PartialEq, Clone, Debug)]
pub enum CompilationLevel {
    Lexed,
    Parsed,
    Compiled
}

pub struct ShellSettings {
    pub level: CompilationLevel,
}

pub struct RunSettings {
    pub file_name: Option<String>,
    pub level: CompilationLevel,
}

fn flush() {
    std::io::stdout().flush().expect("Flush failed.");
}

pub fn shell(settings: ShellSettings) {
    let mut code = String::new();

    loop {
        print!("> ");
        flush();

        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Could not read user input.");

        code.push_str(&buf);

        let print_error = |mut err: RostError| {
            println!("{}", err.with_code(Some(buf.clone())));
        };

        match buf.as_ref() {
            "quit\n" => break,
            _ => {
                let document = match lexer::lex(&buf) {
                    Ok(lexed) => Some(lexed),
                    Err(err) => {
                        print_error(err.into());
                        None
                    }
                };

                if settings.level == CompilationLevel::Lexed {
                    println!("{document:#?}");
                    continue;
                }

                let parsed = document.and_then(|document| match parser::parse(&document) {
                    Ok(program) => Some(program),
                    Err(err) => {
                        print_error(err.into());
                        None
                    }
                });

                if settings.level == CompilationLevel::Parsed {
                    println!("{parsed:#?}");
                    continue;
                }

                let compiled = parsed.and_then(|parsed| match compiler::compile(parsed) {
                    Ok(code) => Some(code),
                    Err(err) => {
                        print_error(err.into());
                        None
                    }
                });

                if settings.level == CompilationLevel::Compiled {
                    println!("{compiled:#?}");
                    continue;
                }
            }
        };
    }
}

#[allow(dead_code)]
pub fn run(settings: RunSettings) -> Option<Program> {
    let file = if let Some(file) = settings.file_name {
        file
    } else {
        println!("No input file provided");
        exit(-1);
    };

    let text = &fs::read_to_string(&file).expect("Unable to read file");

    let print_error = |mut err: RostError| {
        println!(
            "{}",
            err.with_code(Some(text.to_string()))
                .with_file(Some(file.to_string()))
        );
    };

    let document = match lexer::lex(text) {
        Ok(lexed) => Some(lexed),
        Err(err) => {
            print_error(err.into());
            None
        }
    };

    if settings.level == CompilationLevel::Lexed {
        println!("{document:#?}");
        return None;
    }

    let parsed = document.and_then(|document| match parser::parse(&document) {
        Ok(program) => Some(program),
        Err(err) => {
            print_error(err.into());
            None
        }
    });

    if settings.level == CompilationLevel::Parsed {
        println!("{parsed:#?}");
        return None;
    }

    let compiled = parsed.and_then(|parsed| match compiler::compile(parsed) {
        Ok(code) => Some(code),
        Err(err) => {
            print_error(err.into());
            None
        }
    });

    if settings.level == CompilationLevel::Compiled {
        println!("{compiled:#?}");
        return None;
    }

    compiled
}
