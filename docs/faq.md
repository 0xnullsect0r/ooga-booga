# FAQ

Common questions about Ooga Booga, answered.

---

## General

**Q: Is Ooga Booga a joke language?**

It is an *esoteric* language — one designed for exploration, humour, and education rather than production use. The caveman theme is intentional and consistent. That said, the implementation is genuine: a real Rust compiler, a real spec, real tests. You can write real programs in it.

---

**Q: Is the language Turing complete?**

Yes. Ooga Booga has unbounded while loops (`UGGA WHILE`), recursion (`MAGIC`), mutable state (`OOGA` + `GETS`), and conditional branching (`IFF`). A formal argument is on the [Turing Completeness](turing-completeness.md) page.

---

**Q: Why does it transpile to Rust instead of having its own runtime?**

Rust transpilation means:

- The compiler emits readable, inspectable Rust source
- Programs compile to native binaries — no VM, no interpreter overhead
- Leverages Rust's excellent type system and memory safety at runtime
- Easy to distribute: a single binary with no external runtime dependency

---

## Language

**Q: Why is the end-of-block keyword `UGHA`?**

It is a caveman grunt indicating completion — "I am done with this thing." It was chosen to be distinctive, consistent, and easy to search for in code.

---

**Q: Can I use lowercase in identifiers?**

Yes. Variable and function names can use any mix of letters, digits, and underscores, as long as they don't start with a digit and don't clash with a reserved word. Keywords are all-uppercase.

```ooga
OOGA myVariable BE 42       OOF fine
OOGA MY_CONSTANT BE 100     OOF also fine
```

---

**Q: Does Ooga Booga have arrays or objects?**

Not natively. The language is intentionally minimal. There is no first-class array or object type in the current version. This is on the [Roadmap](roadmap.md).

---

**Q: Can I call Rust functions directly?**

Not from Ooga Booga syntax. The generated Rust lives in `.ooga-gen/` — you could add a `build.rs` or a hand-written Rust file there that exposes helper functions, but there is no FFI mechanism in the language itself. This is not currently planned.

---

## Compiler

**Q: Why Rust for the compiler?**

Rust produces fast, correct, memory-safe code. For a compiler, correctness is paramount. Rust's strong type system and exhaustive pattern matching make it excellent for AST traversal and transformation. It also makes `oogac` easy to distribute as a single binary.

---

**Q: Can I embed `oogac` as a library?**

Yes — the core pipeline (lexer, parser, semantic, codegen) is published as a Rust library crate (`oogac` lib). Both the `oogac` and `ooga` binaries use it via `use oogac::...`. You can add `oogac` as a dependency in your own Rust project.

---

**Q: The error messages are funny. Can I turn them off?**

No. The caveman error messages are a feature, not a bug. UGH.

---

## Running programs

**Q: Do I need Node.js?**

No. Ooga Booga compiles to native Rust binaries. You only need Rust installed (via `rustup`). Run `./install-ooga.sh` to set up the toolchain.

---

**Q: `HEAR` doesn't work — what do I do?**

`HEAR` reads a line from stdin using `stdin().read_line()` in the generated Rust. Make sure you are running the compiled binary in a terminal (not piping from /dev/null). See the [Input & Output](language/io.md) page for details.
