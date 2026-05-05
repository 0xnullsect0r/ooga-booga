# Roadmap

Planned improvements and future directions for Ooga Booga.

---

## Near-term

### Array literals

Add first-class array support:

```ooga
OOF Proposed syntax (not yet implemented)
OOGA items BE [1, 2, 3, 4, 5]
SAY items[0]
items[2] GETS 99
```

### For-each loop

A concise iteration syntax over arrays:

```ooga
OOF Proposed syntax
UGGA EACH item IN items
    SAY item
UGHA
```

### String indexing

Access individual characters:

```ooga
OOF Proposed syntax
OOGA s BE "hello"
SAY s[0]   OOF "h"
```

---

## Medium-term

### Syntax highlighting

- VS Code extension with TextMate grammar for `.ooga` files
- Pygments lexer for use in documentation code blocks (removes `ooga` language class requirement from MkDocs)

### Playground page

An in-browser playground embedded in the documentation site where visitors can write and run Ooga Booga programs without installing anything.

Implementation options:

- Compile `oogac` to WebAssembly and run it client-side
- Use a server-side evaluation endpoint

### Better error recovery

The parser currently stops at the first parse error. Improved error recovery would let the compiler report multiple parse errors per file.

### Source maps

Generate Rust source maps or debug annotations so that panic stack traces and error output point back to `.ooga` line numbers.

---

## Long-term

### Native code generation

Add an LLVM or Cranelift backend for compiling to native binaries. This would make Ooga Booga usable in performance-sensitive or embedded contexts.

### Standard library

A collection of useful functions shipped with the compiler:

- String manipulation (split, trim, uppercase, lowercase)
- Basic math (min, max, abs, pow)
- Simple I/O helpers

### REPL

An interactive read-eval-print loop for quick experimentation:

```bash
oogac repl
> SAY "hello"
hello
> OOGA x BE 40 PLUS 2
> SAY x
42
```

### Package system

A mechanism for splitting programs across multiple files and importing functions from other `.ooga` files.

---

## Contributing

If you want to work on any of these features, open an issue first to discuss the design, then see the [Contributing guide](contributing.md).
