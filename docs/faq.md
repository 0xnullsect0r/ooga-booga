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

**Q: Why does it transpile to JavaScript instead of having its own runtime?**

JavaScript transpilation means:

- Zero extra dependencies to run programs (just `node`)
- Easy to inspect and debug the generated code
- Simple implementation — no bytecode, no VM
- The language can be demoed instantly in any browser (potential future playground)

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

Not natively. The language is intentionally minimal. Because the backend is JavaScript, you can store comma-separated strings and parse them manually, but there is no first-class array or object type in the current version. This is on the [Roadmap](roadmap.md).

---

**Q: Can I call JavaScript functions directly?**

Not from Ooga Booga syntax. You would need to extend `oogac` to add an FFI or inline-JS escape hatch. This is not currently planned.

---

## Compiler

**Q: Why Rust for the compiler?**

Rust produces fast, correct, memory-safe code. For a compiler, correctness is paramount. Rust's strong type system and exhaustive pattern matching make it excellent for AST traversal and transformation. It also makes `oogac` easy to distribute as a single binary.

---

**Q: Can I embed `oogac` as a library?**

The core pipeline (lexer, parser, semantic, codegen) is structured as separate modules. It would be straightforward to publish them as a `lib` crate. This is not currently packaged that way, but contributions welcome.

---

**Q: The error messages are funny. Can I turn them off?**

No. The caveman error messages are a feature, not a bug. UGH.

---

## Running programs

**Q: Do I need Node.js?**

You need Node.js only to *run* the generated JavaScript. You can use `oogac compile` without Node.js to produce a `.js` file and run it with any JavaScript runtime (Deno, Bun, etc.).

---

**Q: `HEAR` doesn't work — what do I do?**

Install `readline-sync`:

```bash
npm install readline-sync
```

See the [Input & Output](language/io.md) page for details.
