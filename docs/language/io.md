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
```

Output:

```
Hello, cave!
42
3.14
true
```

`SAY` accepts any expression, including function calls and arithmetic:

```ooga
OOGA x: ROCK BE 7
SAY x TIMES x                            OOF 49
SAY "x squared: " PLUS WORDY(x TIMES x)
```

The compiler emits `println!("{}", expr);` for every `SAY`.

---

## HEAR — read input

`HEAR` reads one line from standard input and stores it in a `WORDS` variable. The variable must be declared with `OOGA` before `HEAR` can assign to it.

```ooga
OOGA name: WORDS
SAY "What is your name?"
HEAR name
SAY "Hello, " PLUS name PLUS "!"
```

!!! note "HEAR always produces WORDS"
    `HEAR` always stores a `WORDS` (String) value. To treat input as a number, convert it with a built-in:

    ```ooga
    OOGA raw: WORDS
    HEAR raw
    OOGA n: ROCK BE NUMBR(raw)
    SAY n TIMES 2
    ```

**Generated Rust for `HEAR name`:**

```rust
{
    let mut __ooga_input = String::new();
    std::io::stdin().read_line(&mut __ooga_input).expect("UGH! CAVE EAR BROKEN");
    name = __ooga_input.trim().to_string();
}
```

No external dependencies are needed — `HEAR` uses Rust's standard library.

---

## Full interactive example

```ooga
OOF Number guessing game

OOGA secret: ROCK BE 7
OOGA guess: ROCK BE 0
OOGA raw: WORDS
OOGA won: GRUNT BE NAH

UGGA WHILE NOT won
    SAY "Guess a number between 1 and 10:"
    HEAR raw
    guess GETS NUMBR(raw)
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
