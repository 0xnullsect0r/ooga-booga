# Data Types

Ooga Booga is dynamically typed. Every variable can hold any of the following types at runtime.

---

## Integer

Whole numbers, stored as 64-bit integers. Negative literals are written with the `MINUS` unary operator.

```ooga
OOGA count BE 42
OOGA negative BE MINUS 7
```

---

## Float

Decimal numbers. The decimal point must be followed by at least one digit.

```ooga
OOGA pi BE 3.14159
OOGA half BE 0.5
```

!!! note
    Arithmetic between integers and floats works naturally — JavaScript handles mixed-type math.

---

## String

Sequences of characters enclosed in double quotes. The following escape sequences are supported:

| Escape | Meaning        |
|--------|----------------|
| `\n`   | Newline        |
| `\t`   | Tab            |
| `\"`   | Literal `"`    |
| `\\`   | Literal `\`    |

```ooga
OOGA greeting BE "Hello, cave!\nHow are you?"
SAY greeting
```

Strings can be concatenated with `PLUS`:

```ooga
OOGA first BE "Ooga"
OOGA last BE "Booga"
SAY first PLUS " " PLUS last
```

---

## Boolean

`YEAH` represents `true`; `NAH` represents `false`.

```ooga
OOGA flag BE YEAH
OOGA done BE NAH
IFF flag
    SAY "Flag is set!"
UGHA
```

---

## VOID

`VOID` represents the absence of a value (equivalent to `null` in JavaScript). Variables declared without an initializer default to `VOID`.

```ooga
OOGA x           OOF x is VOID
OOGA y BE VOID   OOF also VOID

IFF x IS VOID
    SAY "x has no value yet"
UGHA
```

---

## Type coercion

Because the output is JavaScript, Ooga Booga inherits JavaScript's runtime type coercion. Use the built-in functions to convert explicitly:

| Function      | Converts to |
|---------------|-------------|
| `NUMBR(x)`    | Number      |
| `WORDY(x)`    | String      |

```ooga
OOF Read a number from the user
OOGA raw BE VOID
HEAR raw
OOGA n BE NUMBR(raw)
SAY n TIMES 2
```
