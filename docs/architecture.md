# Compiler Architecture

This page describes the internal design of the Ooga Booga toolchain for contributors and curious readers.

---

## Overview

Ooga Booga source files (`.ooga`) are **transpiled to Rust**, then compiled to a native binary by `cargo`. There are two user-facing binaries:

- **`oogac`** — the low-level transpiler CLI (`oogac compile`, `oogac run`, `oogac check`)
- **`ooga`** — the high-level build tool that mirrors `cargo` (`ooga new`, `ooga build`, `ooga run`, `ooga check`, `ooga clean`, `ooga test`)

Both binaries share the same compiler library (`src/lib.rs`).

---

## Pipeline

```
Source file (.ooga)
        │
        ▼
  ┌─────────────┐
  │    Lexer     │   src/lexer.rs
  │  tokenise()  │
  └─────────────┘
        │  Vec<Spanned<Token>>
        ▼
  ┌─────────────┐
  │   Parser     │   src/parser.rs
  │parse_program │
  └─────────────┘
        │  Program (AST)
        ▼
  ┌──────────────────┐
  │ Semantic Analyser │   src/semantic.rs
  │    analyse()      │
  └──────────────────┘
        │  Program (validated)
        ▼
  ┌─────────────┐
  │  Code Gen    │   src/codegen.rs
  │  generate()  │
  └─────────────┘
        │  String (Rust source)
        ▼
  .ooga-gen/src/main.rs
        │
        ▼
  cargo build / rustc
        │
        ▼
  Native binary
```

---

## Module breakdown

### `src/lib.rs`

Re-exports all compiler modules as `pub mod`. Both binary crates (`oogac` and `ooga`) use this library.

---

### `src/error.rs`

Defines `OogaError` and `Span`.

- **`Span`**: source location (line + column).
- **`OogaError`**: four variants — `IoError`, `LexError`, `ParseError`, `SemanticError`. Each variant's `Display` implementation produces a caveman-flavored message.
- **`OogaResult<T>`**: type alias for `Result<T, OogaError>`.

---

### `src/ast.rs`

Pure data structures — no logic.

- **`Program`**: top-level container, holds `Vec<Statement>`.
- **`Statement`**: enum with variants for every statement kind (`VarDecl`, `Assign`, `Say`, `Hear`, `If`, `While`, `Loop`, `Break`, `Continue`, `FuncDef`, `Return`, `ExprStmt`).
- **`Expr`**: expression enum (`Literal`, `Ident`, `BinOp`, `UnaryOp`, `FuncCall`).
- **`TypeAnnotation`**: enum of all 18 Ooga Booga types (`ROCK`, `BIGROCK`, `WORDS`, etc.) with methods `to_rust()`, `default_value()`, `is_words()`.
- All nodes carry a `Span` for accurate error reporting.

---

### `src/lexer.rs`

Converts raw source text into a flat list of `Spanned<Token>` values.

- Recognises all keywords, type keywords, identifiers, integer/float/string/char/boolean literals, operators, and punctuation.
- Emits `Token::Newline` as a statement separator (blank lines are collapsed).
- Type tokens: `TEENYROCK`, `SMALLROCK`, `ROCK`, `BIGROCK`, … `WORDS`, `NOTHING`.
- New tokens vs. v1: `Token::Colon` (`:`), `Token::Arrow` (`->`).

---

### `src/parser.rs`

Recursive-descent parser. Produces an AST `Program` from a token stream.

Key parsing rules:

- `OOGA name: Type` / `OOGA name: Type BE expr`
- `MAGIC name(p1: Type, p2: Type) -> ReturnType`
- `IFF expr` … `NOPE IFF expr` … `NOPE` … `UGHA`
- `UGGA WHILE expr` … `UGHA`
- `UGGA DO` … `UGHA`
- Operator precedence: OR → AND → NOT → comparisons → PLUS/MINUS → TIMES/DIVVY/MOD → unary MINUS → primary

---

### `src/semantic.rs`

Single-pass semantic analysis. Checks:

- Every identifier referenced must be declared (`OOGA`) or be a parameter.
- Every assignment target must be declared.
- `GIVEBACK` must appear inside a `MAGIC` body.
- `STOP` / `SKIP` must appear inside a loop body.
- Function calls must reference a declared or built-in function.
- Duplicate parameter names in `MAGIC` definitions.

Errors are collected and returned as a `Vec<OogaError>` so all problems are reported at once.

---

### `src/codegen.rs`

Emits valid Rust source from a validated AST.

Key behaviours:

- Injects a preamble with `#![allow(...)]` and helper functions (`WORDY`, `NUMBR`, `NUMBR_BIG`, `NUMBR_DRIP`, `BIGNESS`, `FLOORY`, `ROUNDY`, `ROOTY`, `__ooga_concat`).
- Emits `fn name(p: Type) -> ReturnType { ... }` for each `MAGIC` definition.
- Wraps all non-function top-level statements in `fn main() { ... }`.
- `SAY expr` → `println!("{}", expr);`
- `HEAR name` → `std::io::stdin().read_line(...)` block.
- `OOGA x: Type BE expr` → `let mut x: RustType = expr;`
- `PLUS` on `WORDS` values → `__ooga_concat(a, b)` to avoid Rust's asymmetric string `+`.
- Tracks a `type_env: HashMap<String, TypeAnnotation>` to detect string context for PLUS.

---

### `src/bin/oogac.rs`

The low-level transpiler CLI. Subcommands:

| Command               | Action                                          |
|-----------------------|-------------------------------------------------|
| `oogac compile <file>` | Transpile `.ooga` → `.rs` file                 |
| `oogac run <file>`     | Transpile → `rustc` → run binary (temp files)  |
| `oogac check <file>`   | Lex + parse + semantic check, no output         |

---

### `src/bin/ooga.rs`

The high-level build tool. Mirrors `cargo` subcommands:

| Command              | Action                                                      |
|----------------------|-------------------------------------------------------------|
| `ooga new <name>`    | Scaffold project dir with `Ooga.toml` + `src/main.ooga`    |
| `ooga build`         | Transpile → `.ooga-gen/` → `cargo build`                   |
| `ooga build --release` | Release build                                             |
| `ooga run`           | Build + execute                                             |
| `ooga check`         | Transpile + semantic check (no binary)                      |
| `ooga clean`         | Remove `.ooga-gen/` and `target/`                           |
| `ooga test`          | Transpile + `cargo test`                                    |

The `ooga` tool walks up the directory tree to find `Ooga.toml`, writes generated Rust to `.ooga-gen/src/main.rs`, invokes `cargo build`, and copies the binary to `target/debug/` or `target/release/`.

---

## Project layout

```
ooga-booga/
├── Cargo.toml              # lib + two [[bin]] entries
├── src/
│   ├── lib.rs              # re-exports all modules
│   ├── error.rs
│   ├── lexer.rs
│   ├── ast.rs
│   ├── parser.rs
│   ├── semantic.rs
│   ├── codegen.rs
│   └── bin/
│       ├── oogac.rs        # transpiler CLI
│       └── ooga.rs         # build tool CLI
├── tests/
│   └── integration_tests.rs
├── examples/
│   ├── hello_world.ooga
│   ├── fibonacci.ooga
│   ├── factorial.ooga
│   └── loop_demo.ooga
├── install-ooga.sh
├── mkdocs.yml
└── docs/
```

---

## Adding a new language feature

1. Add token(s) to `Token` enum in `src/lexer.rs` and update `keyword_or_ident()`.
2. Add AST node(s) to `Statement` or `Expr` in `src/ast.rs`.
3. Add parsing logic in `src/parser.rs`.
4. Add semantic checks in `src/semantic.rs`.
5. Add Rust code emission in `src/codegen.rs`.
6. Add unit tests in each module and integration tests in `tests/`.
7. Update the relevant documentation pages.
