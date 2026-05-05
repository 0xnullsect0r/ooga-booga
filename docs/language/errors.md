# Error Messages

`oogac` produces caveman-flavored diagnostic messages. All errors are printed to `stderr`; the compiler exits with code `1` when errors are found.

---

## Error categories

### Lex errors

Occur when the source contains a character or token that the lexer cannot recognise.

**Format:**

```
UGH! ME NO UNDERSTAND '<token>' at line N, col C. <hint>
```

| Trigger                              | Message                                        |
|--------------------------------------|------------------------------------------------|
| Unrecognised character               | `CAVE DRAWING NOT RECOGNISED. MAYBE YOU DROP CHISEL?` |
| Unterminated string                  | `CAVE SPEAK NEVER END. PUT CLOSING " ROCK.`   |
| Escape at end of input               | `ESCAPE AT END OF CAVE WALL. ADD MORE CHISELING.` |

**Example:**

```
UGH! ME NO UNDERSTAND '@' at line 4, col 7. CAVE DRAWING NOT RECOGNISED. MAYBE YOU DROP CHISEL?
```

---

### Parse errors

Occur when the token sequence does not match the grammar.

**Format:**

```
BONK! BAD GRAMMAR at line N, col C: <description>
```

| Trigger                              | Message                                         |
|--------------------------------------|-------------------------------------------------|
| Missing `UGHA`                       | `CAVE EXPECTED UGHA BUT FOUND <token>. GRAMMAR ROCK BROKEN.` |
| `UGGA` not followed by `WHILE`/`DO` | `UGGA NEEDS WHILE OR DO AFTER IT. FOUND <token> INSTEAD.` |
| Missing newline after statement      | `CAVE EXPECTED LINE BREAK BUT GOT <token>. ONE THING PER ROCK SLAB.` |
| Unexpected token in expression       | `EXPECTED EXPRESSION BUT GOT <token>. CAVE BRAIN HURT.` |

**Example:**

```
BONK! BAD GRAMMAR at line 8, col 1: CAVE EXPECTED UGHA BUT FOUND Eof. GRAMMAR ROCK BROKEN.
```

---

### Semantic errors

Occur after a successful parse, when the program is structurally valid but logically incorrect.

**Format:**

```
OW! CAVE THINKER CONFUSED at line N, col C: <description>
```

| Trigger                              | Message                                         |
|--------------------------------------|-------------------------------------------------|
| Undeclared variable reference        | `NAME "x" NOT KNOWN. CAVE CREATURE NOT DECLARE IT WITH OOGA FIRST.` |
| Undeclared assignment target         | `NAME "x" NOT KNOWN. CAVE CREATURE NOT DECLARE IT WITH OOGA FIRST.` |
| `HEAR` with undeclared variable      | `HEAR NEEDS DECLARED VARIABLE BUT "x" NOT KNOWN. USE OOGA FIRST.` |
| `GIVEBACK` outside function          | `GIVEBACK OUTSIDE FUNCTION. ONLY USE GIVEBACK INSIDE MAGIC BLOCK.` |
| `STOP` outside loop                  | `STOP OUTSIDE LOOP. CAVE BRAIN CONFUSED.`       |
| `SKIP` outside loop                  | `SKIP OUTSIDE LOOP. CAVE BRAIN CONFUSED.`       |
| Unknown function call                | `FUNCTION "f" NOT KNOWN. DEFINE WITH MAGIC FIRST.` |
| Duplicate parameter name             | `FUNCTION "f" HAS DUPLICATE PARAMETER "x". CAVE NO LIKE TWINS.` |

**Example:**

```
OW! CAVE THINKER CONFUSED at line 3, col 5:
NAME "total" NOT KNOWN. CAVE CREATURE NOT DECLARE IT WITH OOGA FIRST.
```

---

## Multiple errors

Semantic analysis collects **all** errors before reporting them, so you see every problem in a single compiler run instead of fixing one error at a time.

---

## Exit codes

| Code | Meaning                           |
|------|-----------------------------------|
| `0`  | Success                           |
| `1`  | Compile error or runtime failure  |
