# Language Overview

Ooga Booga is a **strongly typed**, imperative esoteric programming language with a caveman theme. Programs are written in `.ooga` files and transpiled to Rust by `oogac`, which then compiles them to native binaries.

---

## Design philosophy

- **One statement per line.** Each statement occupies exactly one line (no semicolons, no braces).
- **Blocks are delimited by `UGHA`.** All block-ending constructs (`IFF`, `UGGA`, `MAGIC`) close with `UGHA` on its own line.
- **Strongly typed.** Every variable and parameter carries an explicit type annotation using caveman-named Rust types (`ROCK`=i32, `WORDS`=String, `GRUNT`=bool, etc.).
- **Caveman keywords.** Every reserved word sounds like it came from a prehistoric cave.

---

## Program structure

A program is a sequence of statements, one per line. Blank lines are ignored. Comments begin with `OOF` and cover the rest of the line.

```ooga
OOF This is a comment — ignored by the compiler

OOGA x ROCK BE 42          OOF declare a ROCK (i32) variable
SAY x                      OOF print it
```

Functions may be defined anywhere in the file; `oogac` emits all function definitions before `fn main()` in the generated Rust, so they are callable from anywhere in the same file.

---

## Execution model

`oogac` compiles `.ooga` → `.rs` in a single pass. The generated Rust file is then compiled with `rustc` (or `cargo` when using the `ooga` build tool) to produce a native binary. The generated Rust includes:

1. A preamble with runtime helper functions (`__ooga_concat`, etc.).
2. All function definitions (emitted before `fn main()`).
3. A `fn main()` containing top-level statements in source order.

---

## Reserved words

| Keyword    | Purpose                          |
|------------|----------------------------------|
| `OOF`      | Line comment                     |
| `OOGA`     | Variable declaration             |
| `BE`       | Initialiser (used with `OOGA`)   |
| `GETS`     | Assignment                       |
| `SAY`      | Print to stdout                  |
| `HEAR`     | Read line from stdin             |
| `IFF`      | If statement                     |
| `NOPE`     | Else / else-if                   |
| `UGHA`     | End block                        |
| `UGGA`     | Loop prefix                      |
| `WHILE`    | While-loop (after `UGGA`)        |
| `DO`       | Infinite loop (after `UGGA`)     |
| `STOP`     | Break out of loop                |
| `SKIP`     | Continue to next iteration       |
| `MAGIC`    | Function definition              |
| `GIVEBACK` | Return from function             |
| `PLUS`     | Addition                         |
| `MINUS`    | Subtraction / unary negation     |
| `TIMES`    | Multiplication                   |
| `DIVVY`    | Division                         |
| `MOD`      | Modulo                           |
| `IS`       | Equality comparison (`===`)      |
| `ISNT`     | Inequality comparison (`!==`)    |
| `BIGGR`    | Greater-than comparison (`>`)    |
| `SMALLR`   | Less-than comparison (`<`)       |
| `AND`      | Logical AND                      |
| `OR`       | Logical OR                       |
| `NOT`      | Logical NOT                      |
| `YEAH`     | Boolean `true`                   |
| `NAH`      | Boolean `false`                  |
| `NOTHING`  | Unit / nothing (return type)     |
| Type names | `ROCK`, `WORDS`, `GRUNT`, etc.   |

Identifiers may use letters, digits, and underscores, but must not start with a digit and must not clash with a reserved word.

---

## A complete example

```ooga
OOF FizzBuzz — cave edition

OOGA i ROCK BE 1
UGGA WHILE i SMALLR IS 30
    IFF i MOD 15 IS 0
        SAY "FizzBuzz"
    NOPE IFF i MOD 3 IS 0
        SAY "Fizz"
    NOPE IFF i MOD 5 IS 0
        SAY "Buzz"
    NOPE
        SAY i
    UGHA
    i GETS i PLUS 1
UGHA
```
