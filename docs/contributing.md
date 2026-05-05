# Contributing

Thank you for your interest in contributing to Ooga Booga! Whether you're fixing a bug, adding a feature, or improving documentation, contributions are welcome.

---

## Development setup

```bash
# Clone the repository
git clone https://github.com/0xnullsect0r/ooga-booga.git
cd ooga-booga

# Build
cargo build

# Run all tests
cargo test

# Run a specific example
cargo run -- run examples/fibonacci.ooga
```

### Code style

- Run `cargo fmt` before submitting a PR.
- Run `cargo clippy -- -D warnings` and address all warnings.
- Keep functions small and focused.
- All new language features need tests in the relevant `#[cfg(test)]` block.

---

## Project structure

```
src/
  main.rs       CLI entry point
  error.rs      OogaError types
  ast.rs        AST data structures
  lexer.rs      Tokeniser
  parser.rs     Recursive descent parser
  semantic.rs   Semantic analyser
  codegen.rs    Rust code generator
examples/       .ooga example programs
docs/           MkDocs documentation site
tests/          (integration test stubs)
```

See [Compiler Architecture](architecture.md) for a detailed explanation of each module.

---

## Types of contributions

### Bug fixes

1. Open an issue describing the bug and how to reproduce it.
2. Fork the repository and create a branch: `fix/describe-the-bug`.
3. Add a failing test that reproduces the bug.
4. Fix the bug so the test passes.
5. Open a pull request.

### New language features

1. Open an issue proposing the feature and its syntax.
2. Wait for discussion — breaking changes to the grammar need consensus.
3. Implement the feature across all relevant modules (see [architecture guide](architecture.md)).
4. Add tests and documentation.
5. Update [examples](examples.md) if the feature is significant.

### Documentation

Documentation lives in `docs/`. Each page is a Markdown file. To preview locally:

```bash
pip install mkdocs-material
mkdocs serve
```

Then open `http://127.0.0.1:8000` in your browser.

### New example programs

Add `.ooga` files to `examples/` and a corresponding section in `docs/examples.md`.

---

## Pull request checklist

Before opening a PR, verify:

- [ ] `cargo build` succeeds with no warnings
- [ ] `cargo test` passes (all 41+ tests green)
- [ ] `cargo clippy -- -D warnings` is clean
- [ ] `cargo fmt --check` passes
- [ ] New features have tests
- [ ] Documentation is updated if the user-facing behaviour changed
- [ ] Commit message is clear and descriptive

---

## Commit message convention

```
<type>: <short description>

<optional body>
```

Types: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`.

Example:

```
feat: add SKIP (continue) statement

Implements SKIP as a loop continuation. Adds:
- Token::Skip in lexer
- Statement::Continue in AST
- parse_statement dispatch
- semantic check for in_loop
- codegen emits `continue;`
- tests in all four modules
```

---

## Licence

By contributing, you agree that your contributions will be licensed under the [GPL-3.0 Licence](https://www.gnu.org/licenses/gpl-3.0.html).
