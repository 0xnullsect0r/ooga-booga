# Compiler Architecture

This page describes the internal design of `oogac`, the Ooga Booga compiler, for contributors and curious readers.

---

## Pipeline overview

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
  ┌─────────────────┐
  │ Semantic Analyser│  src/semantic.rs
  │   analyse()      │
  └─────────────────┘
        │  Program (validated)
        ▼
  ┌─────────────┐
  │  Code Gen    │   src/codegen.rs
  │  generate()  │
  └─────────────┘
        │  String (JavaScript)
        ▼
  Output file (.js)
```

The CLI (`src/main.rs`) wires these stages together and handles file I/O.

---

## Module breakdown

### `src/error.rs`

Defines the `OogaError` enum and the `Span` struct (line + column).

- **`Span`**: Source location, included with every token and most AST nodes.
- **`OogaError`**: Four variants — `IoError`, `LexError`, `ParseError`, `SemanticError`. Each variant's `Display` implementation produces a caveman-flavored message.
- **`OogaResult<T>`**: Type alias for `Result<T, OogaError>`.

---

### `src/ast.rs`

Pure data structures — no logic.

- **`Program`**: top-level container, holds `Vec<Statement>`.
- **`Statement`**: enum with variants for every statement kind (`VarDecl`, `Assign`, `Say`, `Hear`, `If`, `While`, `Loop`, `Break`, `Continue`, `FuncDef`, `Return`, `ExprStmt`).
- **`Expr`**: enum with variants for every expression kind (`Literal`, `Ident`, `BinOp`, `UnaryOp`, `FuncCall`).
- **`Literal`**, **`BinOp`**, **`UnaryOp`**: leaf enums.

All nodes carry a `Span` for error reporting.

---

### `src/lexer.rs`

Converts raw source text into a flat list of tokens.

- **`Token`**: enum of all syntactic tokens (keywords, literals, punctuation, `Eof`).
- **`Spanned`**: pairs a `Token` with its `Span`.
- **`Lexer`**: byte-by-byte scanner. Key behaviours:
  - Multiple consecutive newlines are collapsed into a single `Token::Newline`.
  - `OOF` (when followed by a non-identifier character) skips the rest of the line.
  - String literals handle `\n`, `\t`, `\"`, `\\` escape sequences.
  - Two-character compound operators (`BIGGR IS`, `SMALLR IS`) are recognised at the **parser** level, not the lexer.

---

### `src/parser.rs`

Recursive descent parser. Consumes `Vec<Spanned>` and produces `Program`.

**Statement parsing**: `parse_statement()` dispatches on the first token of the current line to one of the dedicated statement parsers.

**Expression parsing**: uses standard precedence climbing via mutually-recursive functions:

```
parse_expr → parse_or → parse_and → parse_not
          → parse_comparison → parse_additive
          → parse_multiplicative → parse_unary
          → parse_primary
```

`BIGGR IS` / `SMALLR IS` are handled in `parse_comparison()` by peeking one token ahead after seeing `BIGGR` or `SMALLR`.

**Block parsing**: `parse_block()` reads statements until it sees `UGHA`, `NOPE`, or `Eof` — it does **not** consume the terminator.

---

### `src/semantic.rs`

Single-pass semantic analyser that collects all errors before returning.

- Uses a **`Context`** struct containing:
  - `declared`: set of declared variable names in the current scope.
  - `in_function`: tracks whether we are inside a `MAGIC` body.
  - `in_loop`: tracks whether we are inside a `UGGA` body.
  - `functions`: set of known function names (populated in a first pass over the program).

- **First pass**: scans for all top-level `FuncDef` statements and adds their names to `ctx.functions`, enabling mutual recursion and forward calls.
- **Second pass**: walks every statement and expression recursively, accumulating `OogaError::SemanticError` entries.

Built-in function names (`NUMBR`, `WORDY`, etc.) are always considered declared.

---

### `src/codegen.rs`

Traverses the validated AST and produces a JavaScript string.

- Emits a **preamble** containing `"use strict"`, built-in function definitions, and the `_hear()` helper.
- **Function hoisting**: iterates statements twice — first emitting `FuncDef` nodes, then all other statements.
- **Indentation**: maintains an `indent: usize` counter; `indent()` outputs `2 * indent` spaces.
- **Expression emission**: wraps every `BinOp` in parentheses to guarantee correct precedence in the output regardless of JavaScript's own rules.

---

### `src/main.rs`

CLI entry point built with [clap](https://crates.io/crates/clap).

Three subcommands:

| Subcommand | Action |
|------------|--------|
| `compile`  | Lex → Parse → Analyse → Codegen → write `.js` file |
| `run`      | Same as `compile`, then execute with `node` |
| `check`    | Lex → Parse → Analyse only, no output |

---

## Adding a new language feature

To add a new statement or expression type:

1. **`ast.rs`**: Add a new variant to `Statement` or `Expr`.
2. **`lexer.rs`**: Add any new keywords to `Token` and `keyword_or_ident()`.
3. **`parser.rs`**: Add a parsing function and dispatch from `parse_statement()` or `parse_primary()`.
4. **`semantic.rs`**: Add checks in `check_statement()` or `check_expr()`.
5. **`codegen.rs`**: Add code emission in `emit_statement()` or `emit_expr()`.
6. **Tests**: Add unit tests in each module's `#[cfg(test)]` block.
7. **Docs**: Update the relevant language reference page(s).

---

## Running the test suite

```bash
cargo test
```

Tests are co-located with each module in `#[cfg(test)]` blocks. The test suite covers:

- **Lexer**: keyword recognition, string escapes, numeric literals, comments.
- **Parser**: all statement forms, expression precedence, compound operators.
- **Semantic**: undefined variables, out-of-context `GIVEBACK`/`STOP`/`SKIP`, built-ins.
- **Codegen**: generated JS contains expected patterns, function hoisting, correct operator output.
