# Examples

Four complete, annotated example programs illustrating the key features of Ooga Booga.

---

## Hello World

**File:** `examples/hello_world.ooga`

The simplest Ooga Booga program: declare a variable, print it, concatenate strings.

```ooga
OOF Hello World in Ooga Booga!
OOF Cave creature greet the world with big voice.

OOGA greeting BE "Hello, World! UGH!"
SAY greeting

OOGA name BE "Thog"
SAY "Me am:"
SAY name

OOF Strings can be joined using PLUS
OOGA msg BE "Greetings from cave creature: "
SAY msg PLUS name
```

**Concepts demonstrated:** variable declaration, string literals, `SAY`, string concatenation.

**Output:**

```
Hello, World! UGH!
Me am:
Thog
Greetings from cave creature: Thog
```

---

## Loop Demo

**File:** `examples/loop_demo.ooga`

Counting, summing, and early exit using `UGGA WHILE` and `UGGA DO`.

```ooga
OOF Count from 1 to 10
OOGA i BE 1
UGGA WHILE i SMALLR IS 10
    SAY i
    i GETS i PLUS 1
UGHA

SAY "---"

OOF Sum numbers 1 through 100 (Gauss would approve)
OOGA total BE 0
OOGA n BE 1
UGGA WHILE n SMALLR IS 100
    total GETS total PLUS n
    n GETS n PLUS 1
UGHA
SAY "Sum 1..100:"
SAY total

SAY "---"

OOF Infinite loop with STOP
OOGA x BE 0
UGGA DO
    x GETS x PLUS 3
    IFF x BIGGR 20
        STOP
    UGHA
UGHA
SAY "Stopped at:"
SAY x
```

**Concepts demonstrated:** `UGGA WHILE`, `UGGA DO`, `STOP`, accumulation pattern.

**Output:**

```
1
2
...
10
---
Sum 1..100:
5050
---
Stopped at:
21
```

---

## Factorial

**File:** `examples/factorial.ooga`

Recursive factorial demonstrating `MAGIC`, `GIVEBACK`, and recursion.

```ooga
OOF Factorial — recursive function demo

MAGIC factorial(n)
    IFF n SMALLR IS 1
        GIVEBACK 1
    UGHA
    GIVEBACK n TIMES factorial(n MINUS 1)
UGHA

OOGA i BE 0
UGGA WHILE i SMALLR IS 12
    SAY WORDY(i) PLUS "! = " PLUS WORDY(factorial(i))
    i GETS i PLUS 1
UGHA
```

**Concepts demonstrated:** recursion, `MAGIC`, `GIVEBACK`, `WORDY` for string formatting.

**Output:**

```
0! = 1
1! = 1
2! = 2
3! = 6
4! = 24
...
12! = 479001600
```

This program proves Ooga Booga is Turing complete — see [Turing Completeness](turing-completeness.md) for details.

---

## Fibonacci

**File:** `examples/fibonacci.ooga`

First 20 Fibonacci numbers using an iterative while loop, plus a recursive implementation.

```ooga
OOF Iterative Fibonacci
OOGA a BE 0
OOGA b BE 1
OOGA count BE 0
SAY "Fibonacci sequence (first 20):"
UGGA WHILE count SMALLR 20
    SAY a
    OOGA temp BE a PLUS b
    a GETS b
    b GETS temp
    count GETS count PLUS 1
UGHA

SAY "---"

OOF Recursive Fibonacci
MAGIC fib(n)
    IFF n SMALLR IS 1
        GIVEBACK n
    UGHA
    GIVEBACK fib(n MINUS 1) PLUS fib(n MINUS 2)
UGHA

SAY "fib(10) ="
SAY fib(10)
SAY "fib(15) ="
SAY fib(15)
```

**Concepts demonstrated:** iterative algorithm with `UGGA WHILE`, variable swap with a temp variable, recursive function, two approaches to the same problem.

**Output:**

```
Fibonacci sequence (first 20):
0
1
1
2
3
5
...
4181
---
fib(10) =
55
fib(15) =
610
```
