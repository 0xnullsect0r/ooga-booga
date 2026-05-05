# Loops

Ooga Booga provides two loop constructs: `UGGA WHILE` for condition-driven loops, and `UGGA DO` for infinite loops. Both are closed with `UGHA`.

---

## UGGA WHILE — while loop

Executes the body repeatedly as long as the condition is truthy.

```ooga
OOGA i BE 0
UGGA WHILE i SMALLR 5
    SAY i
    i GETS i PLUS 1
UGHA
```

Output:

```
0
1
2
3
4
```

The condition is checked **before** each iteration. If it is falsy on the first check, the body never executes.

---

## UGGA DO — infinite loop

Runs the body forever until a `STOP` statement is reached.

```ooga
OOGA count BE 0
UGGA DO
    count GETS count PLUS 1
    IFF count IS 5
        STOP
    UGHA
UGHA
SAY count   OOF 5
```

Use `UGGA DO` when the exit condition is naturally in the middle or end of the body.

---

## STOP — break

`STOP` immediately exits the innermost enclosing loop. Using `STOP` outside any loop is a semantic error.

```ooga
OOGA i BE 0
UGGA WHILE YEAH
    IFF i IS 10
        STOP
    UGHA
    i GETS i PLUS 1
UGHA
SAY i   OOF 10
```

---

## SKIP — continue

`SKIP` skips the rest of the current loop body and proceeds to the next iteration.

```ooga
OOF Print only odd numbers 1–9
OOGA i BE 0
UGGA WHILE i SMALLR 10
    i GETS i PLUS 1
    IFF i MOD 2 IS 0
        SKIP
    UGHA
    SAY i
UGHA
```

Output:

```
1
3
5
7
9
```

---

## Nested loops

Each loop has its own `UGHA`. `STOP` and `SKIP` only affect the **innermost** enclosing loop.

```ooga
OOGA row BE 1
UGGA WHILE row SMALLR IS 3
    OOGA col BE 1
    UGGA WHILE col SMALLR IS 3
        SAY WORDY(row) PLUS "," PLUS WORDY(col)
        col GETS col PLUS 1
    UGHA
    row GETS row PLUS 1
UGHA
```

Output:

```
1,1
1,2
1,3
2,1
2,2
2,3
3,1
3,2
3,3
```

---

## Tips

- Always update the loop variable inside the body or you will create an infinite loop.
- Use `UGGA DO` with `STOP` for loops where the exit condition depends on user input or complex mid-body logic.
- The Fibonacci example in the [Examples](../examples.md) page shows a practical while loop.
