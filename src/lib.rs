use ::std::io::Write;
use std::{fs, process::exit};

use crate::error::RostError;
use nasm::code::Code;

pub mod compiler;
pub mod error;
pub mod lexer;
pub mod nasm;
pub mod parser;

pub struct Settings {
    pub optimize: bool,
    pub remove_comments: bool,
    pub lsp: bool,
    pub file: Option<String>,
    pub shell_level: ShellLevel,
    pub run_shell: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            optimize: true,
            remove_comments: false,
            file: None,
            shell_level: ShellLevel::End,
            run_shell: false,
            lsp: false,
        }
    }
}

#[derive(PartialEq)]
pub enum ShellLevel {
    Lexed = 0,
    Parsed = 1,
    Compiled = 2,
    Nasm = 3,

    // Not used
    End = 4,
}

fn flush() {
    std::io::stdout().flush().expect("Flush failed.");
}

pub fn shell(settings: Settings) {
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

                if settings.shell_level == ShellLevel::Lexed {
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

                if settings.shell_level == ShellLevel::Parsed {
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

                if settings.shell_level == ShellLevel::Compiled {
                    println!("{compiled:#?}");
                    continue;
                }

                let nasm = compiled.and_then(|compiled| {
                    match nasm::generate(&compiled, !settings.remove_comments, settings.optimize) {
                        Ok(code) => Some(code),
                        Err(err) => {
                            print_error(err.into());
                            None
                        }
                    }
                });

                if settings.shell_level == ShellLevel::Nasm {
                    println!("{nasm:#?}");
                    continue;
                }
            }
        };
    }
}

#[allow(dead_code)]
pub fn run(settings: Settings) -> Option<Code> {
    let file = if let Some(file) = settings.file {
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

    if settings.shell_level == ShellLevel::Lexed {
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

    if settings.shell_level == ShellLevel::Parsed {
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

    if settings.shell_level == ShellLevel::Compiled {
        println!("{compiled:#?}");
        return None;
    }

    let nasm = compiled.and_then(|compiled| {
        match nasm::generate(&compiled, !settings.remove_comments, settings.optimize) {
            Ok(code) => Some(code),
            Err(err) => {
                print_error(err.into());
                None
            }
        }
    });

    if settings.shell_level == ShellLevel::Nasm {
        println!("{nasm:#?}");
        return None;
    }

    nasm
}
