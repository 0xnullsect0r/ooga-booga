//! Integration tests for the Ooga Booga compiler pipeline.
//!
//! Tests drive the full lex → parse → semantic → codegen → rustc pipeline
//! and verify that generated Rust contains expected constructs or that
//! invalid programs are correctly rejected.

use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

// ─── helpers ──────────────────────────────────────────────────────────────────

fn uid() -> u64 {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Compile .ooga source → Rust source. Returns the generated Rust string.
/// Panics if compilation fails.
fn compile(src: &str) -> String {
    let dir = std::env::temp_dir();
    let id = uid();
    let src_path = dir.join(format!("oogac_test_{id}.ooga"));
    let out_path = dir.join(format!("oogac_test_{id}.rs"));
    std::fs::write(&src_path, src).unwrap();
    let status = Command::new(env!("CARGO_BIN_EXE_oogac"))
        .args([
            "compile",
            src_path.to_str().unwrap(),
            "-o",
            out_path.to_str().unwrap(),
        ])
        .status()
        .expect("oogac binary not found");
    assert!(status.success(), "oogac compile failed for: {}", src);
    let rs = std::fs::read_to_string(&out_path).unwrap();
    let _ = std::fs::remove_file(&src_path);
    let _ = std::fs::remove_file(&out_path);
    rs
}

/// Compile .ooga and expect failure (semantic or parse error).
fn compile_expect_error(src: &str) {
    let dir = std::env::temp_dir();
    let src_path = dir.join(format!("oogac_err_{}.ooga", uid()));
    std::fs::write(&src_path, src).unwrap();
    let status = Command::new(env!("CARGO_BIN_EXE_oogac"))
        .args(["check", src_path.to_str().unwrap()])
        .status()
        .expect("oogac binary not found");
    let _ = std::fs::remove_file(&src_path);
    assert!(!status.success(), "expected oogac to fail but it succeeded for: {}", src);
}

/// Compile .ooga → Rust, then compile Rust with rustc, then run and capture stdout.
fn run_rs(src: &str) -> String {
    let dir = std::env::temp_dir();
    let id = uid();
    let src_path = dir.join(format!("oogac_run_{id}.ooga"));
    let rs_path = dir.join(format!("oogac_run_{id}.rs"));
    let bin_path = dir.join(format!("oogac_run_{id}"));

    std::fs::write(&src_path, src).unwrap();

    // Step 1: transpile
    let status = Command::new(env!("CARGO_BIN_EXE_oogac"))
        .args([
            "compile",
            src_path.to_str().unwrap(),
            "-o",
            rs_path.to_str().unwrap(),
        ])
        .status()
        .expect("oogac binary not found");
    assert!(status.success(), "transpile failed for: {}", src);

    // Step 2: compile with rustc
    let status = Command::new("rustc")
        .arg(&rs_path)
        .arg("-o")
        .arg(&bin_path)
        .status()
        .expect("rustc not found — required for end-to-end tests");
    assert!(status.success(), "rustc compile failed for src: {}", src);

    // Step 3: run
    let out = Command::new(&bin_path)
        .output()
        .expect("failed to run compiled binary");

    let _ = std::fs::remove_file(&src_path);
    let _ = std::fs::remove_file(&rs_path);
    let _ = std::fs::remove_file(&bin_path);

    String::from_utf8_lossy(&out.stdout).into_owned()
}

// ─── preamble / structural ────────────────────────────────────────────────────

#[test]
fn compile_empty_program() {
    let rs = compile("");
    // Should have the preamble helper functions
    assert!(rs.contains("fn WORDY"), "got: {}", rs);
}

#[test]
fn compile_comment_only() {
    let rs = compile("OOF this is a comment\n");
    assert!(rs.contains("fn WORDY"), "got: {}", rs);
}

// ─── variable declaration and assignment ──────────────────────────────────────

#[test]
fn compile_variable_declaration() {
    let rs = compile("OOGA x: ROCK BE 42\n");
    assert!(rs.contains("let mut x: i32"), "got: {}", rs);
    assert!(rs.contains("42"), "got: {}", rs);
}

#[test]
fn compile_string_variable() {
    let rs = compile("OOGA greeting: WORDS BE \"hello cave\"\n");
    assert!(rs.contains("hello cave"), "got: {}", rs);
}

#[test]
fn compile_reassignment() {
    let rs = compile("OOGA n: ROCK BE 1\nn GETS 99\n");
    assert!(rs.contains("let mut n: i32"), "got: {}", rs);
    assert!(rs.contains("n = 99"), "got: {}", rs);
}

// ─── output ───────────────────────────────────────────────────────────────────

#[test]
fn compile_say_statement() {
    let rs = compile("SAY \"UGH\"\n");
    assert!(rs.contains("println!"), "got: {}", rs);
    assert!(rs.contains("UGH"), "got: {}", rs);
}

#[test]
fn e2e_hello_world() {
    let output = run_rs("SAY \"Ooga Booga!\"\n");
    assert_eq!(output.trim(), "Ooga Booga!");
}

// ─── arithmetic ───────────────────────────────────────────────────────────────

#[test]
fn e2e_arithmetic() {
    let output = run_rs("OOGA result: ROCK BE 3 PLUS 4\nSAY result\n");
    assert_eq!(output.trim(), "7");
}

#[test]
fn e2e_multiply() {
    let output = run_rs("OOGA x: ROCK BE 6 TIMES 7\nSAY x\n");
    assert_eq!(output.trim(), "42");
}

#[test]
fn e2e_modulo() {
    let output = run_rs("OOGA r: ROCK BE 10 MOD 3\nSAY r\n");
    assert_eq!(output.trim(), "1");
}

// ─── conditionals ─────────────────────────────────────────────────────────────

#[test]
fn compile_if_statement() {
    let rs = compile("OOGA x: ROCK BE 1\nIFF x IS 1\nSAY \"yes\"\nUGHA\n");
    assert!(rs.contains("if "), "got: {}", rs);
    assert!(rs.contains("yes"), "got: {}", rs);
}

#[test]
fn e2e_if_true_branch() {
    let src = "OOGA x: ROCK BE 5\nIFF x BIGGR IS 3\nSAY \"big\"\nUGHA\n";
    let output = run_rs(src);
    assert_eq!(output.trim(), "big");
}

#[test]
fn e2e_if_else() {
    let src = "OOGA x: ROCK BE 1\nIFF x IS 2\nSAY \"equal\"\nNOPE\nSAY \"not equal\"\nUGHA\n";
    let output = run_rs(src);
    assert_eq!(output.trim(), "not equal");
}

// ─── loops ────────────────────────────────────────────────────────────────────

#[test]
fn compile_while_loop() {
    let rs = compile("OOGA i: ROCK BE 0\nUGGA WHILE i SMALLR IS 3\ni GETS i PLUS 1\nUGHA\n");
    assert!(rs.contains("while "), "got: {}", rs);
}

#[test]
fn e2e_while_loop_count() {
    let src = "OOGA i: ROCK BE 0\nOOGA s: ROCK BE 0\nUGGA WHILE i SMALLR 5\ns GETS s PLUS i\ni GETS i PLUS 1\nUGHA\nSAY s\n";
    let output = run_rs(src);
    // sum 0+1+2+3+4 = 10
    assert_eq!(output.trim(), "10");
}

#[test]
fn compile_do_loop() {
    let rs = compile("OOGA i: ROCK BE 0\nUGGA DO\ni GETS i PLUS 1\nIFF i IS 3\nSTOP\nUGHA\nUGHA\n");
    assert!(rs.contains("loop {"), "got: {}", rs);
    assert!(rs.contains("break;"), "got: {}", rs);
}

// ─── functions ────────────────────────────────────────────────────────────────

#[test]
fn compile_function_definition() {
    let rs = compile("MAGIC greet(n: WORDS) -> NOTHING\nSAY n\nUGHA\n");
    assert!(rs.contains("fn greet"), "got: {}", rs);
}

#[test]
fn e2e_function_call() {
    let src = "MAGIC double(x: ROCK) -> ROCK\nGIVEBACK x TIMES 2\nUGHA\nSAY double(21)\n";
    let output = run_rs(src);
    assert_eq!(output.trim(), "42");
}

#[test]
fn e2e_recursive_factorial() {
    let src = "\
MAGIC fact(n: BIGROCK) -> BIGROCK
IFF n IS 0
GIVEBACK 1
UGHA
GIVEBACK n TIMES fact(n MINUS 1)
UGHA
SAY fact(5)
";
    let output = run_rs(src);
    assert_eq!(output.trim(), "120");
}

#[test]
fn e2e_recursive_fibonacci() {
    let src = "\
MAGIC fib(n: ROCK) -> ROCK
IFF n SMALLR 2
GIVEBACK n
UGHA
GIVEBACK fib(n MINUS 1) PLUS fib(n MINUS 2)
UGHA
SAY fib(10)
";
    let output = run_rs(src);
    assert_eq!(output.trim(), "55");
}

// ─── built-ins ────────────────────────────────────────────────────────────────

#[test]
fn e2e_builtin_numbr() {
    let src = "OOGA n: ROCK BE NUMBR(\"42\")\nSAY n\n";
    let output = run_rs(src);
    assert_eq!(output.trim(), "42");
}

#[test]
fn e2e_builtin_wordy() {
    let src = "OOGA s: WORDS BE WORDY(99)\nSAY s\n";
    let output = run_rs(src);
    assert_eq!(output.trim(), "99");
}

#[test]
fn e2e_builtin_bigness() {
    let src = "OOGA n: BIGROCK BE BIGNESS(\"hello\")\nSAY n\n";
    let output = run_rs(src);
    assert_eq!(output.trim(), "5");
}

// ─── error cases ──────────────────────────────────────────────────────────────

#[test]
fn error_undefined_variable() {
    compile_expect_error("SAY undefined_var\n");
}

#[test]
fn error_return_outside_function() {
    compile_expect_error("GIVEBACK 1\n");
}

#[test]
fn error_break_outside_loop() {
    compile_expect_error("STOP\n");
}

// ─── example programs ─────────────────────────────────────────────────────────

#[test]
fn e2e_example_hello_world() {
    let src = std::fs::read_to_string("examples/hello_world.ooga")
        .expect("examples/hello_world.ooga missing");
    let output = run_rs(&src);
    assert!(output.contains("Ooga Booga") || !output.is_empty(), "output: {}", output);
}

#[test]
fn e2e_example_factorial() {
    let src = std::fs::read_to_string("examples/factorial.ooga")
        .expect("examples/factorial.ooga missing");
    let output = run_rs(&src);
    assert!(output.contains("120"), "output: {}", output);
}

#[test]
fn e2e_example_fibonacci() {
    let src = std::fs::read_to_string("examples/fibonacci.ooga")
        .expect("examples/fibonacci.ooga missing");
    let output = run_rs(&src);
    // fib(10) = 55
    assert!(output.contains("55"), "output: {}", output);
}

#[test]
fn e2e_example_loop_demo() {
    let src = std::fs::read_to_string("examples/loop_demo.ooga")
        .expect("examples/loop_demo.ooga missing");
    let output = run_rs(&src);
    // sum 1..100 = 5050
    assert!(output.contains("5050"), "output: {}", output);
}
