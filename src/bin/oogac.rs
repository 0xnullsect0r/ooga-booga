use clap::{Parser as ClapParser, Subcommand};
use oogac::codegen::Codegen;
use oogac::error::OogaError;
use oogac::lexer::Lexer;
use oogac::parser::Parser;
use oogac::semantic::analyse;
use std::path::PathBuf;
use std::process;

#[derive(ClapParser)]
#[command(
    name = "oogac",
    about = "UGH! OOGAC — THE OOGA BOOGA CAVE COMPILER",
    long_about = "Compile .ooga cave scripts into Rust source code.\n\
                  Use MAGIC to make functions. Use UGGA WHILE to make loops.\n\
                  Cave creature happy when code compiles. Cave creature VERY SAD on error."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Transpile a .ooga source file to Rust.
    Compile {
        /// Source .ooga file to compile.
        file: PathBuf,
        /// Output Rust file (defaults to <input>.rs).
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Transpile and immediately compile+run a .ooga file via rustc.
    Run {
        /// Source .ooga file to run.
        file: PathBuf,
        /// Arguments to pass to the program.
        args: Vec<String>,
    },
    /// Check a .ooga file for errors without emitting output.
    Check {
        /// Source .ooga file to check.
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Compile { file, output } => {
            let rust_src = compile_file(&file);
            let out_path = output.unwrap_or_else(|| file.with_extension("rs"));
            if let Err(e) = std::fs::write(&out_path, &rust_src) {
                eprintln!(
                    "UGH! CAVE BRAIN NO WRITE FILE '{}': {}",
                    out_path.display(),
                    e
                );
                process::exit(1);
            }
            eprintln!("GOOD JOB! RUST SCROLL READY: {}", out_path.display());
        }

        Command::Run { file, args } => {
            let rust_src = compile_file(&file);
            let tmp_rs = std::env::temp_dir().join("__ooga_run__.rs");
            let tmp_bin = std::env::temp_dir().join("__ooga_run__");
            std::fs::write(&tmp_rs, &rust_src).expect("UGH! CAVE NO WRITE TEMP");

            // Compile with rustc
            let status = process::Command::new("rustc")
                .arg(&tmp_rs)
                .arg("-o")
                .arg(&tmp_bin)
                .status()
                .unwrap_or_else(|e| {
                    eprintln!("UGH! CAVE NEED RUSTC. INSTALL AT https://rustup.rs\nERROR: {}", e);
                    process::exit(1);
                });
            if !status.success() {
                eprintln!("BONK! RUSTC SAY NO. CAVE CODE BAD.");
                let _ = std::fs::remove_file(&tmp_rs);
                process::exit(1);
            }

            let exit_status = process::Command::new(&tmp_bin)
                .args(&args)
                .status()
                .unwrap_or_else(|e| {
                    eprintln!("UGH! CAVE NO RUN BINARY: {}", e);
                    process::exit(1);
                });
            let _ = std::fs::remove_file(&tmp_rs);
            let _ = std::fs::remove_file(&tmp_bin);
            process::exit(exit_status.code().unwrap_or(1));
        }

        Command::Check { file } => {
            let src = read_source(&file);
            match run_pipeline(&src) {
                Ok(_) => {
                    eprintln!("CAVE GRAMMAR GOOD! NO PROBLEMS FOUND.");
                }
                Err(errors) => {
                    print_errors(&errors);
                    process::exit(1);
                }
            }
        }
    }
}

fn read_source(path: &PathBuf) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("UGH! CAVE BRAIN NO READ FILE '{}': {}", path.display(), e);
        process::exit(1);
    })
}

fn run_pipeline(src: &str) -> Result<String, Vec<OogaError>> {
    let tokens = Lexer::new(src).tokenise().map_err(|e| vec![e])?;
    let program = Parser::new(tokens).parse_program().map_err(|e| vec![e])?;
    let errors = analyse(&program);
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(Codegen::new().generate(&program))
}

fn compile_file(path: &PathBuf) -> String {
    let src = read_source(path);
    match run_pipeline(&src) {
        Ok(rs) => rs,
        Err(errors) => {
            print_errors(&errors);
            process::exit(1);
        }
    }
}

fn print_errors(errors: &[OogaError]) {
    for e in errors {
        eprintln!("{}", e);
    }
}
