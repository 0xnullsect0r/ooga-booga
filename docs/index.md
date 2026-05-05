# Ooga Booga 🪨🔥

> **"Me make code. Code do thing. UGH!"**

Ooga Booga is a caveman-themed esoteric programming language that transpiles to JavaScript. It is Turing complete, surprisingly expressive, and 100% caveman.

---

## Taste the language

```ooga
OOF Compute factorial — recursion in cave style!

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

Output:

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

---

## Why Ooga Booga?

- **Cave-themed keywords** — `SAY`, `MAGIC`, `UGGA WHILE`, `GIVEBACK`, `IFF`, `NOPE`
- **Turing complete** — loops, recursion, mutable state, conditionals
- **Transpiles to JavaScript** — output runs with Node.js, zero extra runtime
- **Compiler in Rust** — fast, correct, friendly error messages
- **Real documentation** — this site

---

## Quick links

<div class="grid cards" markdown>

- :material-download: **[Installation](getting-started/installation.md)** — build the compiler
- :material-rocket-launch: **[Quick Start](getting-started/quick-start.md)** — write your first program
- :material-book-open: **[Language Reference](language/overview.md)** — full syntax guide
- :material-code-braces: **[Examples](examples.md)** — annotated programs

</div>

---

## Status

Ooga Booga is a complete, working language implementation. The compiler (`oogac`) handles the full pipeline: lexing, parsing, semantic analysis, and JavaScript code generation.

All [example programs](examples.md) compile and run correctly. 41 automated tests cover the lexer, parser, semantic analyser, and code generator.
