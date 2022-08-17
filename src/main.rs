use ::std::io::Write;
use std::{env, fs, process::exit};

use crate::error::RostError;

mod compiler;
mod error;
mod lexer;
mod nasm;
mod parser;

#[derive(PartialEq)]
enum ShellLevel {
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

fn shell(shell_level: ShellLevel) {
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

                if shell_level == ShellLevel::Lexed {
                    println!("{:#?}", document);
                    continue;
                }

                let parsed = document.and_then(|document| match parser::parse(&document) {
                    Ok(program) => Some(program),
                    Err(err) => {
                        print_error(err.into());
                        None
                    }
                });

                if shell_level == ShellLevel::Parsed {
                    println!("{:#?}", parsed);
                    continue;
                }

                let compiled = parsed.and_then(|parsed| match compiler::compile(&parsed) {
                    Ok(code) => Some(code),
                    Err(err) => {
                        print_error(err.into());
                        None
                    }
                });

                if shell_level == ShellLevel::Compiled {
                    println!("{:#?}", compiled);
                    continue;
                }

                let nasm = compiled.and_then(|compiled| match nasm::generate(&compiled) {
                    Ok(code) => Some(code),
                    Err(err) => {
                        print_error(err.into());
                        None
                    }
                });

                if shell_level == ShellLevel::Nasm {
                    println!("{:#?}", nasm);
                    continue;
                }
            }
        };
    }
}

#[allow(dead_code)]
fn run(file: &str) -> i32 {
    let text = &fs::read_to_string(file).expect("Unable to read file");

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

    let parsed = document.and_then(|document| match parser::parse(&document) {
        Ok(program) => Some(program),
        Err(err) => {
            print_error(err.into());
            None
        }
    });

    let compiled = parsed.and_then(|parsed| match compiler::compile(&parsed) {
        Ok(code) => Some(code),
        Err(err) => {
            print_error(err.into());
            None
        }
    });

    let nasm = compiled.and_then(|compiled| match nasm::generate(&compiled) {
        Ok(code) => Some(code),
        Err(err) => {
            print_error(err.into());
            None
        }
    });

    if let Some(nasm) = nasm {
        fs::write("out.asm", format!("{}", nasm)).expect("Unable to write file");
        return 0;
    }

    return -1;
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    let mut run_shell = false;
    let mut shell_level = ShellLevel::Compiled;
    let mut input_file: Option<String> = None;

    let mut i = 0;
    while let Some(arg) = args.get(i) {
        i += 1;

        match arg.as_str() {
            "-s" => run_shell = true,
            "-sl" => {
                if let Some(level) = &args
                    .get(i)
                    .and_then(|v| v.parse::<usize>().ok())
                    .filter(|&v| v < ShellLevel::End as usize)
                {
                    shell_level = match *level {
                        0 => ShellLevel::Lexed,
                        1 => ShellLevel::Parsed,
                        2 => ShellLevel::Compiled,
                        3 => ShellLevel::Nasm,
                        _ => unreachable!(),
                    };

                    i += 1;
                } else {
                    println!("-sl requires positive numeric level below 3");
                    exit(-1);
                }
            }
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
        shell(shell_level)
    } else if let Some(file) = input_file {
        exit(run(&file));
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
