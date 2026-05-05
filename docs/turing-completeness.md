# Turing Completeness

Ooga Booga is Turing complete. This page explains what that means, why it matters, and provides a formal argument for why Ooga Booga qualifies.

---

## What is Turing completeness?

A computational system is **Turing complete** if it can simulate any Turing machine — informally, if it can compute anything that is computable given enough time and memory. Every general-purpose programming language you have used (Python, C, JavaScript, etc.) is Turing complete.

The minimum ingredients for Turing completeness are:

1. **Mutable state** — the ability to read and write memory.
2. **Conditional branching** — the ability to take different actions based on data.
3. **Unbounded repetition** — the ability to repeat actions an arbitrary number of times (via loops or recursion).

---

## Ooga Booga's Turing completeness argument

### 1. Mutable state

Variables declared with `OOGA` can be read and written at any point:

```ooga
OOGA x BE 0
x GETS x PLUS 1   OOF mutable state
```

There is no limit on the number of variables a program may declare, and they can hold arbitrarily large integer values (bounded only by the host JavaScript engine, which uses arbitrary-precision arithmetic for very large BigInts — or standard 64-bit floats for practical use).

### 2. Conditional branching

`IFF` / `NOPE IFF` / `NOPE` provides full conditional branching:

```ooga
IFF condition
    OOF one path
NOPE
    OOF another path
UGHA
```

Any computable predicate can be expressed as an Ooga Booga expression.

### 3. Unbounded repetition

Ooga Booga provides two mechanisms:

**a) `UGGA WHILE` — while loop**

The loop condition is checked each iteration, and the body can modify any variable used in the condition. There is no syntactic bound on the number of iterations:

```ooga
UGGA WHILE some_condition
    OOF body may run forever
UGHA
```

**b) Recursion via `MAGIC`**

Functions may call themselves (or call other functions that call back). There is no depth limit imposed by the language (only the practical stack limit of the Node.js runtime):

```ooga
MAGIC recurse(n)
    IFF n IS 0
        GIVEBACK 0
    UGHA
    GIVEBACK recurse(n MINUS 1) PLUS 1
UGHA
```

Either mechanism alone is sufficient for Turing completeness when combined with mutable state and branching.

---

## Proof sketch: simulating a Turing machine

A Turing machine consists of:

- An infinite tape of cells, each holding a symbol
- A finite set of states
- A transition function: `(state, symbol) → (new_state, new_symbol, direction)`

We can simulate this in Ooga Booga:

- Represent the tape as a sufficiently large array (via JavaScript interop) or as a pair of integer stacks encoded as large numbers.
- Represent the state as an integer variable.
- Represent the head position as an integer variable.
- Use a `UGGA WHILE` loop for the machine's step function.
- Use nested `IFF` / `NOPE IFF` chains to implement the transition table.

This simulation is constructive: any Turing machine can be mechanically translated to an Ooga Booga program.

---

## Demonstration programs

The included [examples](examples.md) demonstrate practical unbounded computation:

- **`factorial.ooga`** — computes `n!` via recursion for any non-negative `n`
- **`fibonacci.ooga`** — iterative Fibonacci using `UGGA WHILE`; also recursive `fib(n)`
- **`loop_demo.ooga`** — sums the first 100 integers; shows `UGGA DO` with `STOP`

These programs would not terminate for all inputs if there were an inherent bound on repetition. The fact that they compute unbounded sequences proves the repetition mechanism is genuinely unlimited.

---

## Limitations

Turing completeness is a theoretical property. Practical limits include:

- **Stack depth**: deep recursion is limited by Node.js's call stack (~10,000–15,000 frames by default).
- **Memory**: variables are held in RAM.
- **Integer precision**: JavaScript numbers are IEEE-754 doubles; very large integers lose precision unless `BigInt` is used explicitly.

None of these are language-level restrictions; they are implementation-level constraints shared by all practical computing systems.
