# Functions

Functions in Ooga Booga are defined with `MAGIC` and called by name. They support typed parameters, typed return values, and full recursion.

---

## Defining a function

```ooga
MAGIC greet(name: WORDS) -> NOTHING
    SAY "Hello, " PLUS name PLUS "!"
UGHA
```

A function definition starts with `MAGIC`, followed by:

- the function name
- a parenthesised, comma-separated list of `param: Type` pairs
- `-> ReturnType` (use `-> NOTHING` for functions that return nothing)
- the body, closed by `UGHA`

**Multiple parameters:**

```ooga
MAGIC add(a: ROCK, b: ROCK) -> ROCK
    GIVEBACK a PLUS b
UGHA
```

**No parameters:**

```ooga
MAGIC sayHello() -> NOTHING
    SAY "Hello, cave!"
UGHA
```

**All parameter and return types are required.** The compiler emits an error if any are missing.

---

## Calling a function

Function calls use `name(args)` syntax, either as a standalone statement or inside an expression:

```ooga
greet("Thog")                           OOF statement call

OOGA result: ROCK BE add(3, 4)          OOF call in expression
SAY result                              OOF 7

SAY add(10, 20) TIMES 2                 OOF call inside larger expression
```

---

## Returning a value

Use `GIVEBACK` to return a value from a function:

```ooga
MAGIC square(n: ROCK) -> ROCK
    GIVEBACK n TIMES n
UGHA

SAY square(6)   OOF 36
```

`GIVEBACK` with no expression returns from a `-> NOTHING` function:

```ooga
MAGIC doSomething() -> NOTHING
    SAY "doing it"
    GIVEBACK
UGHA
```

---

## Recursion

Functions can call themselves. This is a primary mechanism for Turing completeness.

```ooga
MAGIC factorial(n: BIGROCK) -> BIGROCK
    IFF n SMALLR IS 1
        GIVEBACK 1
    UGHA
    GIVEBACK n TIMES factorial(n MINUS 1)
UGHA

SAY factorial(10)   OOF 3628800
```

Mutual recursion (A calls B which calls A) is supported because `oogac` does a first-pass scan of all function names before checking calls.

---

## Generated Rust

The compiler emits each function as a top-level Rust `fn`. Functions are placed **before** `fn main()` in the output:

```rust
fn factorial(n: i64) -> i64 {
    if (n <= 1) {
        return 1;
    }
    return (n * factorial((n - 1)));
}

fn main() {
    println!("{}", factorial(10));
}
```

---

## Scope

- Variables declared inside a function are **local** to that function.
- Parameters are treated as locally declared variables.
- Functions can read and modify variables declared in the same or outer scope (Rust's ownership rules apply to generated code).

---

## Errors

| Error                                    | Message                                                     |
|------------------------------------------|-------------------------------------------------------------|
| `GIVEBACK` outside a function            | `OW! CAVE THINKER CONFUSED ... GIVEBACK OUTSIDE FUNCTION.` |
| Calling an undefined function            | `OW! ... FUNCTION "x" NOT KNOWN. DEFINE WITH MAGIC FIRST.` |
| Duplicate parameter names                | `OW! ... FUNCTION "f" HAS DUPLICATE PARAMETER "x".`        |
