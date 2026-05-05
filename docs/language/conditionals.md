# Conditionals

Ooga Booga's `IFF` statement allows programs to take different paths based on a condition.

---

## Basic if

```ooga
IFF condition
    OOF body executes when condition is truthy
    SAY "condition was true"
UGHA
```

The condition can be any expression. `UGHA` closes the block.

---

## If / else

```ooga
IFF x BIGGR 0
    SAY "positive"
NOPE
    SAY "zero or negative"
UGHA
```

`NOPE` introduces the else branch. There is still only one `UGHA` for the whole if/else construct.

---

## If / else-if / else

Chain multiple conditions with `NOPE IFF`:

```ooga
IFF score BIGGR IS 90
    SAY "A"
NOPE IFF score BIGGR IS 80
    SAY "B"
NOPE IFF score BIGGR IS 70
    SAY "C"
NOPE
    SAY "F"
UGHA
```

Any number of `NOPE IFF` branches may be chained before the final `NOPE`.

---

## Truthy and falsy

Ooga Booga is strongly typed — conditions must be `GRUNT` (bool) expressions. Use comparison operators to produce boolean values:

| Expression         | Result |
|--------------------|--------|
| `x BIGGR 0`        | `YEAH` if x > 0, else `NAH` |
| `x IS 0`           | `YEAH` if x equals 0        |
| `YEAH`             | Always `YEAH`               |
| `NAH`              | Always `NAH`                |

Non-boolean expressions used directly as conditions will produce a type error.

---

## Nested conditionals

```ooga
IFF x BIGGR 0
    IFF x SMALLR 100
        SAY "between 1 and 99"
    NOPE
        SAY "100 or more"
    UGHA
NOPE
    SAY "zero or negative"
UGHA
```

Each nested `IFF` has its own `UGHA`.

---

## Compound conditions

Use `AND`, `OR`, and `NOT` to build compound conditions:

```ooga
IFF x BIGGR 0 AND x SMALLR 10
    SAY "single digit positive"
UGHA

IFF x IS 0 OR x IS VOID
    SAY "nothing here"
UGHA

IFF NOT (x IS 0)
    SAY "x has a value"
UGHA
```
