use std::process::Command;
/// Integration tests for the Ooga Booga compiler pipeline.
///
/// These tests drive the full lex → parse → semantic → codegen pipeline
/// and verify that generated JavaScript contains expected constructs or
/// that invalid programs are correctly rejected.
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

// ─── helpers ──────────────────────────────────────────────────────────────────

/// Unique suffix per invocation to avoid file conflicts when tests run in parallel.
fn uid() -> u64 {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Run the full pipeline on a source string and return the generated JS.
/// Panics if compilation fails.
fn compile(src: &str) -> String {
    let dir = std::env::temp_dir();
    let id = uid();
    let src_path = dir.join(format!("oogac_test_{id}.ooga"));
    let out_path = dir.join(format!("oogac_test_{id}.js"));
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
    assert!(status.success(), "oogac compile failed");
    let js = std::fs::read_to_string(&out_path).unwrap();
    let _ = std::fs::remove_file(&src_path);
    let _ = std::fs::remove_file(&out_path);
    js
}

/// Run the full pipeline and expect failure (semantic or parse error).
fn compile_expect_error(src: &str) {
    let dir = std::env::temp_dir();
    let src_path = dir.join(format!("oogac_err_{}.ooga", uid()));
    std::fs::write(&src_path, src).unwrap();
    let status = Command::new(env!("CARGO_BIN_EXE_oogac"))
        .args(["check", src_path.to_str().unwrap()])
        .status()
        .expect("oogac binary not found");
    let _ = std::fs::remove_file(&src_path);
    assert!(!status.success(), "expected oogac to fail but it succeeded");
}

/// Run generated JS with node and capture stdout.
fn run_js(js: &str) -> String {
    let dir = std::env::temp_dir();
    let js_path = dir.join(format!("oogac_run_{}.js", uid()));
    std::fs::write(&js_path, js).unwrap();
    let out = Command::new("node")
        .arg(&js_path)
        .output()
        .expect("node not found — install Node.js to run end-to-end tests");
    let _ = std::fs::remove_file(&js_path);
    String::from_utf8_lossy(&out.stdout).into_owned()
}

// ─── lexer-level integration ──────────────────────────────────────────────────

#[test]
fn compile_empty_program() {
    let js = compile("");
    // Should produce the preamble but nothing extra.
    assert!(js.contains("\"use strict\""));
}

#[test]
fn compile_comment_only() {
    let js = compile("OOF this is a comment\n");
    assert!(js.contains("\"use strict\""));
}

// ─── variable declaration and assignment ──────────────────────────────────────

#[test]
fn compile_variable_declaration() {
    let js = compile("OOGA x BE 42\n");
    assert!(js.contains("let x =") || js.contains("var x ="));
    assert!(js.contains("42"));
}

#[test]
fn compile_string_variable() {
    let js = compile("OOGA greeting BE \"hello cave\"\n");
    assert!(js.contains("hello cave"));
}

#[test]
fn compile_reassignment() {
    let js = compile("OOGA n BE 1\nn GETS 99\n");
    assert!(js.contains("let n =") || js.contains("var n ="));
    assert!(js.contains("99"));
    assert!(js.contains("n ="));
}

// ─── output ───────────────────────────────────────────────────────────────────

#[test]
fn compile_say_statement() {
    let js = compile("SAY \"UGH\"\n");
    assert!(js.contains("console.log"));
    assert!(js.contains("UGH"));
}

#[test]
fn e2e_hello_world() {
    let js = compile("SAY \"Ooga Booga!\"\n");
    let output = run_js(&js);
    assert_eq!(output.trim(), "Ooga Booga!");
}

// ─── arithmetic ───────────────────────────────────────────────────────────────

#[test]
fn e2e_arithmetic() {
    let js = compile("OOGA result BE 3 PLUS 4\nSAY result\n");
    let output = run_js(&js);
    assert_eq!(output.trim(), "7");
}

#[test]
fn e2e_multiply() {
    let js = compile("OOGA x BE 6 TIMES 7\nSAY x\n");
    let output = run_js(&js);
    assert_eq!(output.trim(), "42");
}

#[test]
fn e2e_modulo() {
    let js = compile("OOGA r BE 10 MOD 3\nSAY r\n");
    let output = run_js(&js);
    assert_eq!(output.trim(), "1");
}

// ─── conditionals ─────────────────────────────────────────────────────────────

#[test]
fn compile_if_statement() {
    let js = compile("OOGA x BE 1\nIFF x IS 1\nSAY \"yes\"\nUGHA\n");
    assert!(js.contains("if"));
    assert!(js.contains("yes"));
}

#[test]
fn e2e_if_true_branch() {
    let src = "OOGA x BE 5\nIFF x BIGGR IS 3\nSAY \"big\"\nUGHA\n";
    let js = compile(src);
    let output = run_js(&js);
    assert_eq!(output.trim(), "big");
}

#[test]
fn e2e_if_else() {
    let src = "OOGA x BE 1\nIFF x IS 2\nSAY \"equal\"\nNOPE\nSAY \"not equal\"\nUGHA\n";
    let js = compile(src);
    let output = run_js(&js);
    assert_eq!(output.trim(), "not equal");
}

// ─── loops ────────────────────────────────────────────────────────────────────

#[test]
fn compile_while_loop() {
    let js = compile("OOGA i BE 0\nUGGA WHILE i SMALLR IS 3\ni GETS i PLUS 1\nUGHA\n");
    assert!(js.contains("while"));
}

#[test]
fn e2e_while_loop_count() {
    let src =
        "OOGA i BE 0\nOOGA s BE 0\nUGGA WHILE i SMALLR 5\ns GETS s PLUS i\ni GETS i PLUS 1\nUGHA\nSAY s\n";
    let js = compile(src);
    let output = run_js(&js);
    // sum 0+1+2+3+4 = 10
    assert_eq!(output.trim(), "10");
}

#[test]
fn compile_do_loop() {
    let js = compile("OOGA i BE 0\nUGGA DO\ni GETS i PLUS 1\nIFF i IS 3\nSTOP\nUGHA\nUGHA\n");
    assert!(js.contains("do") || js.contains("while"));
}

// ─── functions ────────────────────────────────────────────────────────────────

#[test]
fn compile_function_definition() {
    let js = compile("MAGIC greet(n)\nSAY n\nUGHA\n");
    assert!(js.contains("function greet"));
}

#[test]
fn e2e_function_call() {
    let src = "MAGIC double(x)\nGIVEBACK x TIMES 2\nUGHA\nSAY double(21)\n";
    let js = compile(src);
    let output = run_js(&js);
    assert_eq!(output.trim(), "42");
}

#[test]
fn e2e_recursive_factorial() {
    let src = "\
MAGIC fact(n)
IFF n IS 0
GIVEBACK 1
UGHA
GIVEBACK n TIMES fact(n MINUS 1)
UGHA
SAY fact(5)
";
    let js = compile(src);
    let output = run_js(&js);
    assert_eq!(output.trim(), "120");
}

#[test]
fn e2e_recursive_fibonacci() {
    let src = "\
MAGIC fib(n)
IFF n SMALLR 2
GIVEBACK n
UGHA
GIVEBACK fib(n MINUS 1) PLUS fib(n MINUS 2)
UGHA
SAY fib(10)
";
    let js = compile(src);
    let output = run_js(&js);
    assert_eq!(output.trim(), "55");
}

// ─── built-ins ────────────────────────────────────────────────────────────────

#[test]
fn e2e_builtin_numbr() {
    let src = "OOGA n BE NUMBR(\"42\")\nSAY n\n";
    let js = compile(src);
    let output = run_js(&js);
    assert_eq!(output.trim(), "42");
}

#[test]
fn e2e_builtin_wordy() {
    let src = "OOGA s BE WORDY(99)\nSAY s\n";
    let js = compile(src);
    let output = run_js(&js);
    assert_eq!(output.trim(), "99");
}

#[test]
fn e2e_builtin_bigness() {
    let src = "OOGA n BE BIGNESS(\"hello\")\nSAY n\n";
    let js = compile(src);
    let output = run_js(&js);
    assert_eq!(output.trim(), "5");
}

// ─── error cases ──────────────────────────────────────────────────────────────

#[test]
fn error_undefined_variable() {
    compile_expect_error("SAY undefined_var\n");
}

#[test]
fn error_unknown_function() {
    compile_expect_error("SAY no_such_func 1\n");
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
    let js = compile(&src);
    let output = run_js(&js);
    assert!(output.contains("Ooga Booga") || !output.is_empty());
}

#[test]
fn e2e_example_factorial() {
    let src = std::fs::read_to_string("examples/factorial.ooga")
        .expect("examples/factorial.ooga missing");
    let js = compile(&src);
    let output = run_js(&js);
    // factorial.ooga prints factorials; 5! = 120 should appear
    assert!(output.contains("120"));
}

#[test]
fn e2e_example_fibonacci() {
    let src = std::fs::read_to_string("examples/fibonacci.ooga")
        .expect("examples/fibonacci.ooga missing");
    let js = compile(&src);
    let output = run_js(&js);
    // fibonacci.ooga prints fib sequence; 55 = fib(10)
    assert!(output.contains("55"));
}

#[test]
fn e2e_example_loop_demo() {
    let src = std::fs::read_to_string("examples/loop_demo.ooga")
        .expect("examples/loop_demo.ooga missing");
    let js = compile(&src);
    let output = run_js(&js);
    // loop_demo sums 1..100 = 5050
    assert!(output.contains("5050"));
}
