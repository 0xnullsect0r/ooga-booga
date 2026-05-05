# Built-in Functions

Ooga Booga provides a small set of built-in functions for common operations. These are always available without any declaration.

---

## Type conversion

### `NUMBR(x)`

Converts `x` to a number. Equivalent to `Number(x)` in JavaScript.

```ooga
OOGA s BE "42"
OOGA n BE NUMBR(s)
SAY n PLUS 1    OOF 43
```

Useful after `HEAR`, which always returns a string.

---

### `WORDY(x)`

Converts `x` to a string. Equivalent to `String(x)`.

```ooga
OOGA n BE 100
SAY "The answer is: " PLUS WORDY(n)
```

---

## String / array

### `BIGNESS(x)`

Returns the length of a string (or array, if you construct one via JavaScript interop). Equivalent to `x.length`.

```ooga
OOGA word BE "mammoth"
SAY BIGNESS(word)   OOF 7
```

---

## Math

### `FLOORY(x)`

Returns the floor of `x` (largest integer ≤ x). Equivalent to `Math.floor(x)`.

```ooga
SAY FLOORY(3.9)    OOF 3
SAY FLOORY(-1.2)   OOF -2
```

---

### `ROUNDY(x)`

Returns `x` rounded to the nearest integer. Equivalent to `Math.round(x)`.

```ooga
SAY ROUNDY(3.5)   OOF 4
SAY ROUNDY(3.4)   OOF 3
```

---

### `ROOTY(x)`

Returns the square root of `x`. Equivalent to `Math.sqrt(x)`.

```ooga
SAY ROOTY(144)   OOF 12
SAY ROOTY(2)     OOF 1.4142135623730951
```

---

## Summary table

| Function      | JS equivalent      | Description                        |
|---------------|--------------------|------------------------------------|
| `NUMBR(x)`    | `Number(x)`        | Convert to number                  |
| `WORDY(x)`    | `String(x)`        | Convert to string                  |
| `BIGNESS(x)`  | `x.length`         | Length of string (or array)        |
| `FLOORY(x)`   | `Math.floor(x)`    | Floor of a float                   |
| `ROUNDY(x)`   | `Math.round(x)`    | Round to nearest integer           |
| `ROOTY(x)`    | `Math.sqrt(x)`     | Square root                        |

---

## Calling built-ins in expressions

Built-ins are functions and can be used anywhere a function call expression is valid:

```ooga
OOGA r BE 5
OOGA area BE ROUNDY(3.14159 TIMES r TIMES r)
SAY "Circle area ≈ " PLUS WORDY(area)
```
