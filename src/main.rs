use ::std::io::Write;
use std::{env, fs, process::exit};

use language_server::server;
use nasm::code::Code;

use crate::error::RostError;

mod compiler;
mod error;
mod language_server;
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

fn shell(settings: Settings) {
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

                if settings.shell_level == ShellLevel::Parsed {
                    println!("{:#?}", parsed);
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
                    println!("{:#?}", compiled);
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
                    println!("{:#?}", nasm);
                    continue;
                }
            }
        };
    }
}

#[allow(dead_code)]
fn run(settings: Settings) -> Option<Code> {
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
        println!("{:#?}", document);
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
        println!("{:#?}", parsed);
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
        println!("{:#?}", compiled);
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
        println!("{:#?}", nasm);
        return None;
    }

    return nasm;
}

struct Settings {
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

fn main() -> std::io::Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let mut settings = Settings::default();

    let mut i = 0;
    while let Some(arg) = args.get(i) {
        i += 1;

        match arg.as_str() {
            "-lsp" => settings.lsp = true,
            "-no-comments" => settings.remove_comments = true,
            "-no-optimize" => settings.optimize = false,
            "-s" => settings.run_shell = true,
            "-sl" => {
                if let Some(level) = &args
                    .get(i)
                    .and_then(|v| v.parse::<usize>().ok())
                    .filter(|&v| v <= ShellLevel::End as usize)
                {
                    settings.shell_level = match *level {
                        0 => ShellLevel::Lexed,
                        1 => ShellLevel::Parsed,
                        2 => ShellLevel::Compiled,
                        3 => ShellLevel::Nasm,
                        4 => ShellLevel::End,
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
                    settings.file = Some(arg.to_string());
                }
            }
        }
    }

    if settings.lsp {
        return server::run();
    }

    if settings.run_shell && settings.file.is_some() {
        println!("Cannot use input file while running shell");
        exit(-1);
    }

    if settings.run_shell {
        shell(settings);
    } else {
        let asm = run(settings);
        if let Some(asm) = asm {
            fs::write("out.asm", format!("{}", asm)).expect("Unable to write file");
            exit(0);
        }

        exit(1);
    }

    Ok(())
}
