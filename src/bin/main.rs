use std::{env, fs, process::exit};

use rost::{run, shell, Settings, ShellLevel};

fn main() -> std::io::Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let mut settings = Settings::default();

    let mut i = 0;
    while let Some(arg) = args.get(i) {
        i += 1;

        match arg.as_str() {
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
                if arg.starts_with('-') {
                    println!("Unknown argument: {arg}");
                    exit(-1);
                } else {
                    settings.file = Some(arg.to_string());
                }
            }
        }
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
            fs::write("out.asm", format!("{asm}")).expect("Unable to write file");
            exit(0);
        }

        exit(1);
    }

    Ok(())
}
