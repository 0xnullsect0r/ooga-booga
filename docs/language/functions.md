# Functions

Functions in Ooga Booga are defined with `MAGIC` and called by name. They support parameters, return values, and full recursion.

---

## Defining a function

```ooga
MAGIC greet(name)
    SAY "Hello, " PLUS name PLUS "!"
UGHA
```

A function definition starts with `MAGIC`, followed by the function name, a parenthesised parameter list, and the body. `UGHA` closes the body.

**Multiple parameters** are separated by commas:

```ooga
MAGIC add(a, b)
    GIVEBACK a PLUS b
UGHA
```

**No parameters:**

```ooga
MAGIC sayHello()
    SAY "Hello, cave!"
UGHA
```

---

## Calling a function

Function calls use `name(args)` syntax, either as a standalone statement or as part of an expression:

```ooga
greet("Thog")                  OOF statement call

OOGA result BE add(3, 4)       OOF call in expression
SAY result                     OOF 7

SAY add(10, 20) TIMES 2        OOF call inside larger expression
```

---

## Returning a value

Use `GIVEBACK` to return a value from a function:

```ooga
MAGIC square(n)
    GIVEBACK n TIMES n
UGHA

SAY square(6)   OOF 36
```

`GIVEBACK` with no expression returns `VOID`:

```ooga
MAGIC doSomething()
    SAY "doing it"
    GIVEBACK
UGHA
```

If a function reaches the end without a `GIVEBACK`, it returns `VOID` implicitly.

---

## Recursion

Functions can call themselves. This is the primary mechanism for Turing completeness in Ooga Booga.

```ooga
MAGIC factorial(n)
    IFF n SMALLR IS 1
        GIVEBACK 1
    UGHA
    GIVEBACK n TIMES factorial(n MINUS 1)
UGHA

SAY factorial(10)   OOF 3628800
```

Mutual recursion (function A calls function B which calls A) is also supported because `oogac` performs a first-pass scan of all function definitions before checking calls.

---

## Scope

- Variables declared inside a function are **local** to that function.
- Parameters are treated as locally declared variables.
- Functions can read global variables declared outside them.
- A function cannot assign to a global variable unless the global is declared before the function is called (JavaScript closure behaviour applies in generated code).

```ooga
OOGA global_count BE 0

MAGIC increment()
    global_count GETS global_count PLUS 1
UGHA

increment()
increment()
SAY global_count   OOF 2
```

---

## Hoisting

`oogac` emits all function definitions before top-level statements in the generated JavaScript. This means you can call a function before its definition appears in the source file:

```ooga
OOF This call appears before the function definition
SAY double(21)

MAGIC double(n)
    GIVEBACK n TIMES 2
UGHA
```

This works correctly and outputs `42`.

---

## Errors

| Error                                    | Message                                          |
|------------------------------------------|--------------------------------------------------|
| `GIVEBACK` outside a function            | `OW! CAVE THINKER CONFUSED ... GIVEBACK OUTSIDE FUNCTION.` |
| Calling an undefined function            | `OW! ... FUNCTION "x" NOT KNOWN. DEFINE WITH MAGIC FIRST.` |
| Duplicate parameter names               | `OW! ... FUNCTION "f" HAS DUPLICATE PARAMETER "x".`        |
