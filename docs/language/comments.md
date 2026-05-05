# Comments

Ooga Booga supports single-line comments using the `OOF` keyword.

---

## Syntax

A comment starts with `OOF` and extends to the end of the line. Everything after `OOF` on the same line is ignored by the compiler.

```ooga
OOF This entire line is a comment

OOGA x BE 42    OOF This is an inline comment
SAY x           OOF Print x to the cave wall
```

---

## Placement

Comments can appear:

- On their own line
- At the end of any statement line
- As section dividers in long programs

```ooga
OOF ─────────────────────────────────────────────
OOF  Phase 1: Initialisation
OOF ─────────────────────────────────────────────

OOGA count BE 0    OOF start counting from zero
OOGA limit BE 100  OOF stop at 100
```

---

## Block comments

Ooga Booga does not have multi-line block comment syntax. Use multiple `OOF` lines instead:

```ooga
OOF This function computes the nth triangle number:
OOF   T(n) = 1 + 2 + ... + n = n*(n+1)/2
OOF It uses a simple loop rather than the formula.

MAGIC triangle(n)
    OOGA total BE 0
    OOGA i BE 1
    UGGA WHILE i SMALLR IS n
        total GETS total PLUS i
        i GETS i PLUS 1
    UGHA
    GIVEBACK total
UGHA
```

---

## Comments and the `OOF` identifier

The lexer treats `OOF` as a comment only when it appears as a standalone word (not as part of a longer identifier). The following is a valid variable name:

```ooga
OOGA OOFBALL BE 42   OOF 'OOFBALL' starts with OOF but is an identifier
```

The identifier `OOFBALL` is valid because the lexer checks that `OOF` is not followed immediately by another alphanumeric character.
