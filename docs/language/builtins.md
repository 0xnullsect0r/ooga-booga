# Built-in Functions

Ooga Booga provides a small set of built-in functions for common operations. These are always available without any import or declaration — the compiler injects them as Rust helper functions into every generated file.

---

## Type conversion

### `WORDY(x)` → `WORDS`

Converts any value to a `WORDS` (String). Uses Rust's `Display` trait.

```ooga
OOGA n: ROCK BE 100
SAY "The answer is: " PLUS WORDY(n)
```

---

### `NUMBR(x)` → `ROCK`

Parses a `WORDS` value as a `ROCK` (i32). Panics with a caveman message if the input is not a valid integer.

```ooga
OOGA s: WORDS BE "42"
OOGA n: ROCK BE NUMBR(s)
SAY n PLUS 1    OOF 43
```

Useful after `HEAR`, which always produces `WORDS`.

---

### `NUMBR_BIG(x)` → `BIGROCK`

Parses a `WORDS` value as a `BIGROCK` (i64). Use for large integers that overflow `ROCK`.

```ooga
OOGA big: BIGROCK BE NUMBR_BIG("9999999999")
SAY big
```

---

### `NUMBR_DRIP(x)` → `BIGDRIP`

Parses a `WORDS` value as a `BIGDRIP` (f64).

```ooga
OOGA pi: BIGDRIP BE NUMBR_DRIP("3.14159")
SAY pi
```

---

## String

### `BIGNESS(x)` → `BIGROCK`

Returns the byte length of a `WORDS` string as a `BIGROCK` (i64).

```ooga
OOGA word: WORDS BE "mammoth"
SAY BIGNESS(word)   OOF 7
```

---

## Math

### `FLOORY(x)` → `BIGDRIP`

Returns the floor of a `BIGDRIP` — the largest integer not greater than `x`.

```ooga
SAY FLOORY(3.9)    OOF 3
SAY FLOORY(-1.2)   OOF -2
```

---

### `ROUNDY(x)` → `BIGDRIP`

Returns `x` rounded to the nearest integer (as `BIGDRIP`).

```ooga
SAY ROUNDY(3.5)   OOF 4
SAY ROUNDY(3.4)   OOF 3
```

---

### `ROOTY(x)` → `BIGDRIP`

Returns the square root of a `BIGDRIP`.

```ooga
SAY ROOTY(144.0)   OOF 12
SAY ROOTY(2.0)     OOF 1.4142135623730951
```

---

## Summary table

| Function         | Input type | Output type | Description                    |
|------------------|------------|-------------|--------------------------------|
| `WORDY(x)`       | any        | `WORDS`     | Convert to string              |
| `NUMBR(x)`       | `WORDS`    | `ROCK`      | Parse as i32                   |
| `NUMBR_BIG(x)`   | `WORDS`    | `BIGROCK`   | Parse as i64                   |
| `NUMBR_DRIP(x)`  | `WORDS`    | `BIGDRIP`   | Parse as f64                   |
| `BIGNESS(x)`     | `WORDS`    | `BIGROCK`   | String byte length             |
| `FLOORY(x)`      | `BIGDRIP`  | `BIGDRIP`   | Floor                          |
| `ROUNDY(x)`      | `BIGDRIP`  | `BIGDRIP`   | Round to nearest               |
| `ROOTY(x)`       | `BIGDRIP`  | `BIGDRIP`   | Square root                    |

---

## Using built-ins in expressions

Built-ins are functions and can appear anywhere a function call is valid:

```ooga
OOGA r: BIGDRIP BE 5.0
OOGA area: BIGDRIP BE ROUNDY(3.14159 TIMES r TIMES r)
SAY "Circle area ≈ " PLUS WORDY(area)
```
