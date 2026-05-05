//! ooga — the Ooga Booga package manager / build tool (mirrors cargo)

use clap::{Parser as ClapParser, Subcommand};
use oogac::codegen::Codegen;
use oogac::error::OogaError;
use oogac::lexer::Lexer;
use oogac::parser::Parser;
use oogac::semantic::analyse;
use std::path::{Path, PathBuf};
use std::process;

#[derive(ClapParser)]
#[command(
    name = "ooga",
    about = "UGH! OOGA — CAVE BUILD TOOL FOR OOGA BOOGA PROGRAMS",
    long_about = "Build and run Ooga Booga projects.\n\
                  Works like cargo. Uses Ooga.toml. Cave approve."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new Ooga Booga project.
    New {
        /// Project name.
        name: String,
    },
    /// Build the project (transpile + compile).
    Build {
        /// Build in release mode.
        #[arg(long)]
        release: bool,
    },
    /// Build and run the project.
    Run {
        /// Build in release mode.
        #[arg(long)]
        release: bool,
        /// Arguments to pass to the program.
        #[arg(last = true)]
        args: Vec<String>,
    },
    /// Check the project for errors without building.
    Check,
    /// Remove build artifacts.
    Clean,
    /// Run tests.
    Test,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::New { name } => cmd_new(&name),
        Command::Build { release } => cmd_build(release),
        Command::Run { release, args } => cmd_run(release, &args),
        Command::Check => cmd_check(),
        Command::Clean => cmd_clean(),
        Command::Test => cmd_test(),
    }
}

// ── new ───────────────────────────────────────────────────────────────────────

fn cmd_new(name: &str) {
    let dir = PathBuf::from(name);
    if dir.exists() {
        eprintln!("BONK! DIRECTORY '{}' ALREADY EXIST. CAVE CONFUSED.", name);
        process::exit(1);
    }
    std::fs::create_dir_all(dir.join("src")).expect("UGH! CAVE NO MAKE DIR");

    let toml = format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        name
    );
    std::fs::write(dir.join("Ooga.toml"), toml).expect("UGH! CAVE NO WRITE Ooga.toml");

    let main_ooga = r#"OOF Hello World in Ooga Booga!
SAY "Ooga Booga! Cave greet world!"
"#;
    std::fs::write(dir.join("src").join("main.ooga"), main_ooga)
        .expect("UGH! CAVE NO WRITE main.ooga");

    // .gitignore
    std::fs::write(dir.join(".gitignore"), "/target\n/.ooga-gen\n")
        .expect("UGH! CAVE NO WRITE .gitignore");

    println!("CAVE MADE NEW PROJECT: {}", name);
    println!("  {} ooga run", name);
}

// ── build ─────────────────────────────────────────────────────────────────────

fn cmd_build(release: bool) {
    let (project_dir, manifest) = find_project();
    let rust_src = transpile_project(&project_dir);
    write_ooga_gen(&project_dir, &manifest, &rust_src);
    run_cargo_build(&project_dir, release);
    copy_binary(&project_dir, &manifest.name, release);
    println!(
        "CAVE BUILD DONE! BINARY AT: target/{}/{}",
        if release { "release" } else { "debug" },
        manifest.name
    );
}

fn cmd_run(release: bool, args: &[String]) {
    let (project_dir, manifest) = find_project();
    let rust_src = transpile_project(&project_dir);
    write_ooga_gen(&project_dir, &manifest, &rust_src);
    run_cargo_build(&project_dir, release);
    copy_binary(&project_dir, &manifest.name, release);

    let profile = if release { "release" } else { "debug" };
    let binary = project_dir
        .join("target")
        .join(profile)
        .join(&manifest.name);
    let status = process::Command::new(&binary)
        .args(args)
        .status()
        .unwrap_or_else(|e| {
            eprintln!("UGH! CAVE NO RUN '{}': {}", binary.display(), e);
            process::exit(1);
        });
    process::exit(status.code().unwrap_or(0));
}

fn cmd_check() {
    let (project_dir, _) = find_project();
    let src_path = project_dir.join("src").join("main.ooga");
    let src = read_file(&src_path);
    match run_pipeline(&src) {
        Ok(_) => println!("CAVE GRAMMAR GOOD! NO PROBLEMS FOUND."),
        Err(errors) => {
            for e in &errors {
                eprintln!("{}", e);
            }
            process::exit(1);
        }
    }
}

fn cmd_clean() {
    let (project_dir, _) = find_project();
    let gen_dir = project_dir.join(".ooga-gen");
    let target_dir = project_dir.join("target");
    if gen_dir.exists() {
        std::fs::remove_dir_all(&gen_dir).expect("UGH! CAVE NO CLEAN .ooga-gen");
    }
    if target_dir.exists() {
        std::fs::remove_dir_all(&target_dir).expect("UGH! CAVE NO CLEAN target");
    }
    println!("CAVE CLEAN! ALL ROCK DUST GONE.");
}

fn cmd_test() {
    let (project_dir, manifest) = find_project();
    let rust_src = transpile_project(&project_dir);
    write_ooga_gen(&project_dir, &manifest, &rust_src);
    let gen_dir = project_dir.join(".ooga-gen");
    let status = process::Command::new("cargo")
        .arg("test")
        .current_dir(&gen_dir)
        .status()
        .unwrap_or_else(|e| {
            eprintln!("UGH! CAVE NEED CARGO: {}", e);
            process::exit(1);
        });
    process::exit(status.code().unwrap_or(1));
}

// ── helpers ───────────────────────────────────────────────────────────────────

struct Manifest {
    name: String,
    version: String,
}

fn find_project() -> (PathBuf, Manifest) {
    let mut dir = std::env::current_dir().expect("UGH! CAVE NO CWD");
    loop {
        let toml_path = dir.join("Ooga.toml");
        if toml_path.exists() {
            let manifest = parse_manifest(&toml_path);
            return (dir, manifest);
        }
        if !dir.pop() {
            eprintln!("BONK! NO Ooga.toml FOUND. CAVE LOST. RUN 'ooga new <name>' FIRST.");
            process::exit(1);
        }
    }
}

fn parse_manifest(path: &Path) -> Manifest {
    let content = read_file(path);
    let name = extract_toml_value(&content, "name").unwrap_or_else(|| "cave-project".to_string());
    let version = extract_toml_value(&content, "version").unwrap_or_else(|| "0.1.0".to_string());
    Manifest { name, version }
}

fn extract_toml_value(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with(key) {
            if let Some(rest) = line.split_once('=') {
                return Some(rest.1.trim().trim_matches('"').to_string());
            }
        }
    }
    None
}

fn transpile_project(project_dir: &Path) -> String {
    let src_path = project_dir.join("src").join("main.ooga");
    let src = read_file(&src_path);
    match run_pipeline(&src) {
        Ok(rs) => rs,
        Err(errors) => {
            for e in &errors {
                eprintln!("{}", e);
            }
            process::exit(1);
        }
    }
}

fn write_ooga_gen(project_dir: &Path, manifest: &Manifest, rust_src: &str) {
    let gen_dir = project_dir.join(".ooga-gen");
    std::fs::create_dir_all(gen_dir.join("src")).expect("UGH! CAVE NO MAKE .ooga-gen");

    let cargo_toml = format!(
        "[package]\nname = \"{}\"\nversion = \"{}\"\nedition = \"2021\"\n",
        manifest.name, manifest.version
    );
    std::fs::write(gen_dir.join("Cargo.toml"), cargo_toml)
        .expect("UGH! CAVE NO WRITE .ooga-gen/Cargo.toml");

    std::fs::write(gen_dir.join("src").join("main.rs"), rust_src)
        .expect("UGH! CAVE NO WRITE .ooga-gen/src/main.rs");
}

fn run_cargo_build(project_dir: &Path, release: bool) {
    let gen_dir = project_dir.join(".ooga-gen");
    let mut cmd = process::Command::new("cargo");
    cmd.arg("build");
    if release {
        cmd.arg("--release");
    }
    cmd.current_dir(&gen_dir);
    let status = cmd.status().unwrap_or_else(|e| {
        eprintln!("UGH! CAVE NEED CARGO: {}", e);
        process::exit(1);
    });
    if !status.success() {
        eprintln!("BONK! CARGO BUILD FAIL. CAVE CODE BAD.");
        process::exit(1);
    }
}

fn copy_binary(project_dir: &Path, name: &str, release: bool) {
    let profile = if release { "release" } else { "debug" };
    let src = project_dir
        .join(".ooga-gen")
        .join("target")
        .join(profile)
        .join(name);
    let dst_dir = project_dir.join("target").join(profile);
    std::fs::create_dir_all(&dst_dir).expect("UGH! CAVE NO MAKE target dir");
    let dst = dst_dir.join(name);
    if src.exists() {
        std::fs::copy(&src, &dst).unwrap_or_else(|e| {
            eprintln!("UGH! CAVE NO COPY BINARY: {}", e);
            process::exit(1);
        });
    }
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

fn read_file(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("UGH! CAVE BRAIN NO READ '{}': {}", path.display(), e);
        process::exit(1);
    })
}
