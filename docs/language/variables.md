# Variables & Assignment

Variables are the primary way to store and manipulate data in Ooga Booga.

---

## Declaring a variable

Use `OOGA` to declare a new variable. Declaration creates the variable in the current scope.

```ooga
OOGA name           OOF declares 'name', initial value is VOID
OOGA age BE 30      OOF declares 'age' with initial value 30
OOGA msg BE "hi"    OOF declares 'msg' as a string
```

A variable must be declared with `OOGA` before it can be used or assigned to.

---

## Assigning a value

Use `GETS` to assign a new value to a previously declared variable:

```ooga
OOGA x BE 10
x GETS 20        OOF x is now 20
x GETS x PLUS 5  OOF x is now 25
```

The right-hand side of `GETS` can be any expression.

---

## Scope rules

- **Function scope**: variables declared inside a `MAGIC` block are local to that function.
- **Top-level scope**: variables declared outside any function are global.
- Variables declared inside `IFF` or `UGGA` blocks are visible for the remainder of the enclosing function/program (Ooga Booga uses function-level scoping like JavaScript `var`).

```ooga
MAGIC example()
    OOGA local BE 99
    SAY local   OOF ok — local is in scope
UGHA

OOF SAY local   OOF would be an error — local not declared here
```

---

## Common patterns

**Swap two variables:**

```ooga
OOGA a BE 1
OOGA b BE 2
OOGA temp BE a
a GETS b
b GETS temp
SAY a   OOF 2
SAY b   OOF 1
```

**Accumulator:**

```ooga
OOGA total BE 0
OOGA i BE 1
UGGA WHILE i SMALLR IS 100
    total GETS total PLUS i
    i GETS i PLUS 1
UGHA
SAY total   OOF 5050
```

---

## Errors

If you try to use a variable before declaring it, `oogac` reports a semantic error:

```
OW! CAVE THINKER CONFUSED at line 3, col 5:
NAME "x" NOT KNOWN. CAVE CREATURE NOT DECLARE IT WITH OOGA FIRST.
```

If you try to assign to an undeclared variable:

```
OW! CAVE THINKER CONFUSED at line 5, col 1:
NAME "y" NOT KNOWN. CAVE CREATURE NOT DECLARE IT WITH OOGA FIRST.
```
