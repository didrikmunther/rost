use clap::{Parser, ValueEnum};
use rost::{
    backend::{python, wasm, Backend as RostBackend},
    run, CompilationLevel, RunSettings,
};
use std::{fs, process::exit};

// Enum backend, python and wasm
#[derive(Parser, Debug, ValueEnum, Clone)]
enum Backend {
    Python,
    Wasm,
}

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

    // The output directory
    #[arg(short, long, default_value = "build")]
    dir: String,

    // The output file name, without extension
    #[arg(short, long, default_value = "out")]
    output_filename: String,

    // Backend
    #[arg(short, long, default_value = "python")]
    backend: Backend,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.shell && args.file.is_some() {
        println!("Cannot use input file while running shell");
        exit(-1);
    }

    let level = match args.level {
        Some(level) => level,
        None => CompilationLevel::Generated,
    };

    let (backend, extension): (Box<dyn RostBackend>, &str) = match args.backend {
        Backend::Python => (Box::new(python::PythonBackend), "py"),
        Backend::Wasm => (Box::new(wasm::WasmBackend), "wat"),
    };

    if args.shell {
        // shell(ShellSettings { level });
    } else {
        let generated = run(RunSettings {
            file_name: args.file,
            level,
            backend,
        });
        if let Some(generated) = generated {
            fs::write(
                format!("{}/{}.{}", args.dir, args.output_filename, extension),
                generated,
            )
            .expect("Unable to write file");
        
            exit(0);
        }

        exit(1);
    }

    Ok(())
}
