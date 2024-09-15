use clap::Parser;
use rost::{run, shell, CompilationLevel, RunSettings, ShellSettings};
use std::{fs, process::exit};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Run shell
    #[arg(short, long, default_value_t = false)]
    shell: bool,

    // The compilation level
    #[command(subcommand)]
    level: Option<CompilationLevel>,

    // The file to compile
    file: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.shell && args.file.is_some() {
        println!("Cannot use input file while running shell");
        exit(-1);
    }

    let level = match args.level {
        Some(level) => level,
        None => CompilationLevel::Compiled,
    };

    if args.shell {
        shell(ShellSettings { level });
    } else {
        let asm = run(RunSettings {
            file_name: args.file,
            level,
        });
        if let Some(asm) = asm {
            fs::write("out.asm", format!("{asm:?}")).expect("Unable to write file");
            exit(0);
        }

        exit(1);
    }

    Ok(())
}
