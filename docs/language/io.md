# Input & Output

Ooga Booga programs communicate with the user through two built-in statements: `SAY` (output) and `HEAR` (input).

---

## SAY — print output

`SAY` evaluates an expression and prints it to standard output, followed by a newline.

```ooga
SAY "Hello, cave!"
SAY 42
SAY 3.14
SAY YEAH
SAY VOID
```

Output:

```
Hello, cave!
42
3.14
true
null
```

`SAY` accepts any expression, including function calls and arithmetic:

```ooga
OOGA x BE 7
SAY x TIMES x         OOF 49
SAY "x squared: " PLUS WORDY(x TIMES x)
```

---

## HEAR — read input

`HEAR` reads one line of text from standard input and stores it as a string in the named variable. The variable must be declared with `OOGA` before `HEAR` can assign to it.

```ooga
OOGA name BE VOID
SAY "What is your name?"
HEAR name
SAY "Hello, " PLUS name PLUS "!"
```

!!! note "Input is always a string"
    `HEAR` always returns a string. To treat the input as a number, convert it with `NUMBR()`:

    ```ooga
    OOGA raw BE VOID
    HEAR raw
    OOGA n BE NUMBR(raw)
    SAY n TIMES 2
    ```

---

## readline-sync requirement

`HEAR` relies on the [`readline-sync`](https://www.npmjs.com/package/readline-sync) Node.js package for synchronous stdin reading. Install it before running programs that use `HEAR`:

```bash
npm install readline-sync
```

If `readline-sync` is not installed, the generated JavaScript will throw an error at runtime when `HEAR` is reached:

```
UGH! CAVE NEEDS readline-sync TO HEAR. RUN: npm install readline-sync
```

---

## Full interactive example

```ooga
OOF Simple number guessing game

OOGA secret BE 7
OOGA guess BE VOID
OOGA won BE NAH

UGGA WHILE NOT won
    SAY "Guess a number between 1 and 10:"
    HEAR guess
    guess GETS NUMBR(guess)
    IFF guess IS secret
        SAY "YES! CAVE BRAIN CORRECT! UGH UGH!"
        won GETS YEAH
    NOPE IFF guess SMALLR secret
        SAY "Too small! Think bigger, cave creature."
    NOPE
        SAY "Too big! Think smaller, cave creature."
    UGHA
UGHA
```
