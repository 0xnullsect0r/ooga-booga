# Variables & Assignment

Variables are the primary way to store and manipulate data in Ooga Booga. All variables require a **type annotation** — the language is strongly typed.

---

## Declaring a variable

Use `OOGA` to declare a new variable. The type annotation (`: Type`) is **required**.

```ooga
OOGA age: ROCK                    OOF declare, no initial value (defaults to 0)
OOGA name: WORDS BE "Thog"        OOF declare with initial value
OOGA score: BIGDRIP BE 9.5        OOF float
OOGA done: GRUNT BE NAH           OOF boolean
```

**Syntax:**

```
OOGA <name>: <Type>
OOGA <name>: <Type> BE <expr>
```

A variable declared without `BE` receives the **default value** for its type:

| Type              | Default value |
|-------------------|---------------|
| Integer types     | `0`           |
| Float types       | `0.0`         |
| `GRUNT` (bool)    | `NAH` (false) |
| `SCRATCH` (char)  | `'\0'`        |
| `WORDS` (String)  | `""` (empty)  |
| `NOTHING`         | `()`          |

---

## Assigning a value

Use `GETS` to assign a new value to a previously declared variable:

```ooga
OOGA x: ROCK BE 10
x GETS 20          OOF x is now 20
x GETS x PLUS 5    OOF x is now 25
```

The right-hand side of `GETS` can be any expression whose type matches the variable's declared type.

---

## Scope rules

- **Function scope**: variables declared inside a `MAGIC` block are local to that function.
- **Top-level scope**: variables declared outside any function are in the main function scope.
- Variables declared inside `IFF` or `UGGA` blocks are visible within the enclosing scope (Ooga Booga uses Rust-style block scoping in the generated output).

```ooga
MAGIC example() -> NOTHING
    OOGA local: ROCK BE 99
    SAY local          OOF ok — in scope
UGHA

OOF SAY local          OOF would be an error — local not declared here
```

---

## Common patterns

**Swap two variables:**

```ooga
OOGA a: ROCK BE 1
OOGA b: ROCK BE 2
OOGA temp: ROCK BE a
a GETS b
b GETS temp
SAY a              OOF 2
SAY b              OOF 1
```

**Accumulator:**

```ooga
OOGA total: ROCK BE 0
OOGA i: ROCK BE 1
UGGA WHILE i SMALLR IS 100
    total GETS total PLUS i
    i GETS i PLUS 1
UGHA
SAY total          OOF 5050
```

**String building:**

```ooga
OOGA msg: WORDS BE "Cave says: "
OOGA name: WORDS BE "Thog"
SAY msg PLUS name  OOF "Cave says: Thog"
```

---

## Errors

If you use a variable before declaring it, `oogac` reports a semantic error:

```
OW! CAVE THINKER CONFUSED at line 3, col 5:
NAME "x" NOT KNOWN. CAVE CREATURE NOT DECLARE IT WITH OOGA FIRST.
```

If you try to assign to an undeclared variable:

```
OW! CAVE THINKER CONFUSED at line 5, col 1:
NAME "y" NOT KNOWN. CAVE CREATURE NOT DECLARE IT WITH OOGA FIRST.
```
