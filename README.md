# 🪨 Ooga Booga

> *The caveman esoteric programming language. Grunt your way to Turing completeness.*

[![CI](https://github.com/0xnullsect0r/ooga-booga/actions/workflows/ci.yml/badge.svg)](https://github.com/0xnullsect0r/ooga-booga/actions/workflows/ci.yml)
[![Docs](https://github.com/0xnullsect0r/ooga-booga/actions/workflows/docs.yml/badge.svg)](https://0xnullsect0r.github.io/ooga-booga/)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Compiler-Rust-orange)](https://www.rust-lang.org/)

**Ooga Booga** is a [LOLCODE](https://lolcode.org/)-inspired esoteric programming language with a prehistoric twist. Its keywords read like cave inscriptions, its error messages growl at you, and its compiler — `oogac` — is written in Rust and transpiles to JavaScript.

---

## The language in thirty seconds

```ooga
OOF Recursive factorial — cave style!

MAGIC factorial(n)
    IFF n SMALLR IS 1
        GIVEBACK 1
    UGHA
    GIVEBACK n TIMES factorial(n MINUS 1)
UGHA

OOGA i BE 1
UGGA WHILE i SMALLR IS 10
    SAY WORDY(i) PLUS "! = " PLUS WORDY(factorial(i))
    i GETS i PLUS 1
UGHA
```

```
1! = 1
2! = 2
3! = 6
4! = 24
5! = 120
6! = 720
7! = 5040
8! = 40320
9! = 362880
10! = 3628800
```

### Key ideas

| Keyword      | Meaning              | Example                              |
|--------------|----------------------|--------------------------------------|
| `OOGA x BE`  | Declare variable     | `OOGA count BE 0`                    |
| `x GETS`     | Assign               | `count GETS count PLUS 1`            |
| `SAY`        | Print                | `SAY "hello, cave!"`                 |
| `HEAR`       | Read stdin           | `HEAR name`                          |
| `IFF / NOPE` | If / else            | `IFF x IS 0` … `NOPE` … `UGHA`      |
| `UGGA WHILE` | While loop           | `UGGA WHILE i SMALLR 10` … `UGHA`   |
| `MAGIC`      | Define function      | `MAGIC fib(n)` … `UGHA`             |
| `GIVEBACK`   | Return               | `GIVEBACK n TIMES 2`                 |
| `OOF`        | Comment              | `OOF this is ignored`                |

---

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- [Node.js](https://nodejs.org/) 18+ (to run generated JavaScript)

### Build from source

```bash
git clone https://github.com/0xnullsect0r/ooga-booga.git
cd ooga-booga
cargo build --release
```

The binary is at `target/release/oogac`. Optionally add it to your `PATH`:

```bash
export PATH="$PWD/target/release:$PATH"
```

---

## Usage

```bash
# Compile to JavaScript
oogac compile program.ooga              # produces program.js
oogac compile program.ooga -o out.js    # custom output path

# Compile and run immediately
oogac run program.ooga

# Check for errors without producing output
oogac check program.ooga
```

### Running examples

```bash
oogac run examples/hello_world.ooga
oogac run examples/factorial.ooga
oogac run examples/fibonacci.ooga
oogac run examples/loop_demo.ooga
```

---

## Documentation

Full documentation is available at:

**[https://0xnullsect0r.github.io/ooga-booga/](https://0xnullsect0r.github.io/ooga-booga/)**

Topics covered:

- [Installation](https://0xnullsect0r.github.io/ooga-booga/getting-started/installation/)
- [Quick Start](https://0xnullsect0r.github.io/ooga-booga/getting-started/quick-start/)
- [Language Reference](https://0xnullsect0r.github.io/ooga-booga/language/overview/)
- [Compiler Architecture](https://0xnullsect0r.github.io/ooga-booga/architecture/)
- [Turing Completeness](https://0xnullsect0r.github.io/ooga-booga/turing-completeness/)
- [Contributing](https://0xnullsect0r.github.io/ooga-booga/contributing/)

---

## Project structure

```
Cargo.toml
src/
  main.rs        CLI (clap subcommands: compile / run / check)
  error.rs       OogaError types with caveman messages
  ast.rs         AST node definitions
  lexer.rs       Tokeniser
  parser.rs      Recursive descent parser
  semantic.rs    Semantic analyser
  codegen.rs     JavaScript code generator
examples/
  hello_world.ooga
  factorial.ooga
  fibonacci.ooga
  loop_demo.ooga
docs/            MkDocs documentation source
mkdocs.yml       MkDocs + Material theme config
.github/
  workflows/
    ci.yml       Build, test, lint on every push/PR
    docs.yml     Deploy docs to GitHub Pages
```

---

## Running tests

```bash
cargo test
```

41 tests cover the lexer, parser, semantic analyser, and code generator.

---

## Status

Ooga Booga is a **complete, working esolang implementation**:

- ✅ Lexer, parser, AST, semantic analysis, JS code generation
- ✅ All example programs compile and produce correct output
- ✅ 41 automated tests pass
- ✅ GitHub Actions CI (build, test, lint, smoke-test)
- ✅ GitHub Pages documentation (MkDocs + Material)
- ✅ Turing complete (while loops + recursion + mutable state)

---

## Contributing

Contributions are welcome! See [CONTRIBUTING](docs/contributing.md) or the [docs page](https://0xnullsect0r.github.io/ooga-booga/contributing/).

Please run `cargo fmt` and `cargo clippy -- -D warnings` before opening a PR.

---

## License

[GPL-3.0](LICENSE) © 2024 Ooga Booga Contributors
