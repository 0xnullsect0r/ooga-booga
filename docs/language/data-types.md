# Data Types

Ooga Booga is **strongly typed**. Every variable, parameter, and return value must carry an explicit type annotation. Types map directly to Rust primitives — the compiler emits Rust, which `cargo` compiles to a native binary.

---

## Integer types

Signed integers (can hold negative values):

| Ooga Booga   | Rust type | Width       | Range                                 |
|--------------|-----------|-------------|---------------------------------------|
| `TEENYROCK`  | `i8`      | 8-bit       | −128 … 127                            |
| `SMALLROCK`  | `i16`     | 16-bit      | −32 768 … 32 767                      |
| `ROCK`       | `i32`     | 32-bit      | −2 147 483 648 … 2 147 483 647        |
| `BIGROCK`    | `i64`     | 64-bit      | −9.2 × 10¹⁸ … 9.2 × 10¹⁸            |
| `HUGEROCK`   | `i128`    | 128-bit     | massive                               |
| `CLIFFROCK`  | `isize`   | pointer     | platform-dependent signed             |

Unsigned integers (no negative values):

| Ooga Booga    | Rust type | Width       | Range                          |
|---------------|-----------|-------------|--------------------------------|
| `TEENYPEBBLE` | `u8`      | 8-bit       | 0 … 255                        |
| `SMALLPEBBLE` | `u16`     | 16-bit      | 0 … 65 535                     |
| `PEBBLE`      | `u32`     | 32-bit      | 0 … 4 294 967 295              |
| `BIGPEBBLE`   | `u64`     | 64-bit      | 0 … 1.8 × 10¹⁹               |
| `HUGEPEBBLE`  | `u128`    | 128-bit     | 0 … massive                   |
| `CLIFFPEBBLE` | `usize`   | pointer     | platform-dependent unsigned    |

**`ROCK` (i32)** is the default integer type for most programs.

```ooga
OOGA age: ROCK BE 25
OOGA count: BIGROCK BE 10000000000
OOGA byte_val: TEENYPEBBLE BE 255
```

---

## Floating-point types

| Ooga Booga | Rust type | Width   | Notes                  |
|------------|-----------|---------|------------------------|
| `DRIP`     | `f32`     | 32-bit  | single precision       |
| `BIGDRIP`  | `f64`     | 64-bit  | double precision       |

**`BIGDRIP` (f64)** is the default float type.

```ooga
OOGA pi: BIGDRIP BE 3.14159265358979
OOGA temp: DRIP BE 98.6
```

---

## Boolean

| Ooga Booga | Rust type | Literals        |
|------------|-----------|-----------------|
| `GRUNT`    | `bool`    | `YEAH` / `NAH`  |

```ooga
OOGA alive: GRUNT BE YEAH
OOGA done: GRUNT BE NAH

IFF alive
    SAY "cave creature live!"
UGHA
```

---

## Character

| Ooga Booga | Rust type | Notes                |
|------------|-----------|----------------------|
| `SCRATCH`  | `char`    | single Unicode scalar |

```ooga
OOGA letter: SCRATCH BE 'A'
```

---

## String

| Ooga Booga | Rust type | Notes                     |
|------------|-----------|---------------------------|
| `WORDS`    | `String`  | owned UTF-8 string        |

```ooga
OOGA name: WORDS BE "Thog"
SAY name
```

String concatenation uses `PLUS` — the compiler emits `format!("{}{}", a, b)` automatically:

```ooga
OOGA first: WORDS BE "Ooga"
OOGA last: WORDS BE "Booga"
SAY first PLUS " " PLUS last
```

---

## Unit / no return

| Ooga Booga | Rust type | Notes                         |
|------------|-----------|-------------------------------|
| `NOTHING`  | `()`      | return type for void functions |

```ooga
MAGIC greet(name: WORDS) -> NOTHING
    SAY "Hello, " PLUS name PLUS "!"
UGHA
```

When `-> NOTHING` is used, the compiler omits the return type in the emitted Rust (`fn greet(...) { ... }` without `-> ()`).

---

## Type conversion

Use the built-in conversion functions to move values between types:

| Function        | Converts             | Example                          |
|-----------------|----------------------|----------------------------------|
| `WORDY(x)`      | any → `WORDS`        | `WORDY(42)` → `"42"`            |
| `NUMBR(x)`      | `WORDS` → `ROCK`     | `NUMBR("7")` → `7`              |
| `NUMBR_BIG(x)`  | `WORDS` → `BIGROCK`  | `NUMBR_BIG("999999999999")`      |
| `NUMBR_DRIP(x)` | `WORDS` → `BIGDRIP`  | `NUMBR_DRIP("3.14")`             |
