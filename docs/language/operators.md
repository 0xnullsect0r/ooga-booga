# Operators

Ooga Booga supports arithmetic, comparison, and logical operators — all written as English cave words.

---

## Arithmetic operators

Binary operators are written **infix**: `left OP right`.

| Operator | Meaning        | Example                   | JS equivalent |
|----------|----------------|---------------------------|---------------|
| `PLUS`   | Addition       | `3 PLUS 4` → `7`          | `+`           |
| `MINUS`  | Subtraction    | `10 MINUS 3` → `7`        | `-`           |
| `TIMES`  | Multiplication | `6 TIMES 7` → `42`        | `*`           |
| `DIVVY`  | Division       | `10 DIVVY 4` → `2.5`      | `/`           |
| `MOD`    | Modulo         | `10 MOD 3` → `1`          | `%`           |

`MINUS` can also be used **prefix** for negation:

```ooga
OOGA x BE MINUS 5   OOF x = -5
OOGA y BE MINUS x   OOF y = 5
```

---

## Comparison operators

Comparisons return a boolean (`YEAH` or `NAH`).

| Operator       | Meaning              | JS equivalent |
|----------------|----------------------|---------------|
| `IS`           | Equal                | `===`         |
| `ISNT`         | Not equal            | `!==`         |
| `BIGGR`        | Greater than         | `>`           |
| `SMALLR`       | Less than            | `<`           |
| `BIGGR IS`     | Greater than or equal| `>=`          |
| `SMALLR IS`    | Less than or equal   | `<=`          |

!!! note "Two-word operators"
    `BIGGR IS` and `SMALLR IS` are each treated as a single operator — the parser looks ahead for the second token.

```ooga
OOGA a BE 5
OOGA b BE 10
SAY a SMALLR b        OOF true
SAY a BIGGR IS b      OOF false
SAY a IS 5            OOF true
SAY a ISNT b          OOF true
```

---

## Logical operators

| Operator | Meaning     | JS equivalent |
|----------|-------------|---------------|
| `AND`    | Logical AND | `&&`          |
| `OR`     | Logical OR  | `\|\|`        |
| `NOT`    | Logical NOT | `!`           |

```ooga
OOGA x BE 5
IFF x BIGGR 0 AND x SMALLR 10
    SAY "x is between 0 and 10"
UGHA

IFF NOT (x IS 0)
    SAY "x is nonzero"
UGHA
```

---

## Operator precedence

Higher rows bind more tightly (evaluated first):

| Precedence | Operators                                        |
|------------|--------------------------------------------------|
| 8 (highest)| Literals, identifiers, function calls, `()`      |
| 7          | Unary `MINUS`, `NOT`                             |
| 6          | `TIMES`, `DIVVY`, `MOD`                          |
| 5          | `PLUS`, `MINUS`                                  |
| 4          | `IS`, `ISNT`, `BIGGR`, `SMALLR`, `BIGGR IS`, `SMALLR IS` |
| 3          | `NOT` (prefix)                                   |
| 2          | `AND`                                            |
| 1 (lowest) | `OR`                                             |

Use parentheses `( )` to override precedence:

```ooga
SAY 2 PLUS 3 TIMES 4        OOF 14  (TIMES binds tighter)
SAY (2 PLUS 3) TIMES 4      OOF 20  (parens override)
```

---

## String concatenation

`PLUS` also concatenates strings:

```ooga
OOGA s BE "cave" PLUS " " PLUS "creature"
SAY s   OOF "cave creature"
```

!!! warning
    Mixing a number and a string with `PLUS` follows JavaScript's coercion rules: `1 PLUS "2"` produces `"12"`. Use `NUMBR()` or `WORDY()` to convert explicitly.
