mod ast;
mod codegen;
mod error;
mod lexer;
mod parser;
mod semantic;

use clap::{Parser as ClapParser, Subcommand};
use std::path::PathBuf;
use std::process;

use codegen::Codegen;
use error::OogaError;
use lexer::Lexer;
use parser::Parser;
use semantic::analyse;

#[derive(ClapParser)]
#[command(
    name = "oogac",
    about = "UGH! OOGAC — THE OOGA BOOGA CAVE COMPILER",
    long_about = "Compile .ooga cave scripts into runnable JavaScript.\n\
                  Use MAGIC to make functions. Use UGGA WHILE to make loops.\n\
                  Cave creature happy when code compiles. Cave creature VERY SAD on error."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Transpile a .ooga source file to JavaScript.
    Compile {
        /// Source .ooga file to compile.
        file: PathBuf,
        /// Output JavaScript file (defaults to <input>.js).
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Transpile and immediately run a .ooga file with node.
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
            let js = compile_file(&file);
            let out_path = output.unwrap_or_else(|| file.with_extension("js"));
            if let Err(e) = std::fs::write(&out_path, &js) {
                eprintln!(
                    "UGH! CAVE BRAIN NO WRITE FILE '{}': {}",
                    out_path.display(),
                    e
                );
                process::exit(1);
            }
            eprintln!("GOOD JOB! CAVE SCRIPT READY: {}", out_path.display());
        }

        Command::Run { file, args } => {
            // Write to a temp file, then execute with node.
            let js = compile_file(&file);
            let tmp = file.with_extension("js");
            if let Err(e) = std::fs::write(&tmp, &js) {
                eprintln!("UGH! CAVE BRAIN NO WRITE TEMP FILE: {}", e);
                process::exit(1);
            }
            let mut cmd = process::Command::new("node");
            cmd.arg(&tmp);
            cmd.args(&args);
            let status = cmd.status().unwrap_or_else(|e| {
                eprintln!(
                    "UGH! CAVE NEED NODE.JS TO RUN. INSTALL AT https://nodejs.org\nERROR: {}",
                    e
                );
                process::exit(1);
            });
            // Clean up temp file.
            let _ = std::fs::remove_file(&tmp);
            process::exit(status.code().unwrap_or(1));
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

/// Read source or exit with a friendly message.
fn read_source(path: &PathBuf) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("UGH! CAVE BRAIN NO READ FILE '{}': {}", path.display(), e);
        process::exit(1);
    })
}

/// Lex → parse → semantic analysis → codegen. Returns generated JS or errors.
fn run_pipeline(src: &str) -> Result<String, Vec<OogaError>> {
    // Lex
    let tokens = Lexer::new(src).tokenise().map_err(|e| vec![e])?;

    // Parse
    let program = Parser::new(tokens).parse_program().map_err(|e| vec![e])?;

    // Semantic analysis
    let errors = analyse(&program);
    if !errors.is_empty() {
        return Err(errors);
    }

    // Code generation
    let js = Codegen::new().generate(&program);
    Ok(js)
}

/// Run the full pipeline and print errors, exiting on failure.
fn compile_file(path: &PathBuf) -> String {
    let src = read_source(path);
    match run_pipeline(&src) {
        Ok(js) => js,
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
