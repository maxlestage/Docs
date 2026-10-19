# Handling errors

This page explains the two kinds of errors in Rust: fatal errors with `panic!`, and expected errors with `Result` and the `?` operator.

> **Goals:**
>
> - tell a fatal error (`panic!`) apart from an expected error (`Result`);
> - return a `Result` and handle it with `match`;
> - pass an error on to the caller with `?`;
> - choose between `unwrap`, `expect` and a default value;
> - create your own error type.

## Two kinds of errors

An [error](../debuter/glossaire.md#error) is when the program cannot do what you ask
it to do. Rust tells two kinds apart, like a car: an engine failure stops you dead, a
traffic jam only makes you change your route.

- An **unrecoverable** error: a bug, an impossible situation. The program stops with
  `panic!`.
- A **recoverable** error: a missing file, a badly written text… The function returns
  a `Result` and the caller decides what to do.

## Stopping the program with panic!

`panic!` stops the program and prints a message. Reading slot 10 of a list of 3
items, for example, causes a `panic!`:

```rust norun
fn main() {
    let v = vec![1, 2, 3];
    println!("{}", v[10]);
}
```

```text
thread 'main' panicked at src/main.rs:3:21:
index out of bounds: the len is 3 but the index is 10
```

You can also trigger `panic!` yourself:

```rust norun
panic!("This should never happen");
```

> **Warning:** keep `panic!` for real bugs. For a normal error, use `Result`.

## The Result type

`Result<T, E>` is an [enumeration](types.md) with two variants. `T` is the type of the
value on success, `E` the type of the error. A `Result` is either:

- `Ok(value)`: everything went well;
- `Err(error)`: something failed.

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("{:?}", divide(10.0, 4.0));
    println!("{:?}", divide(1.0, 0.0));
}
```

```sortie
Ok(2.5)
Err("division by zero")
```

Think of an envelope with your exam results: it holds either the diploma, or a letter
that explains the failure. You have to open it to find out.

## Handling a Result with match

`match` opens the `Result` and handles both cases:

```rust
fn main() {
    let inputs = ["42", "abc"];
    for text in inputs {
        match text.parse::<i32>() {
            Ok(n) => println!("{text} → the number {n}"),
            Err(e) => println!("{text} → error: {e}"),
        }
    }
}
```

```sortie
42 → the number 42
abc → error: invalid digit found in string
```

## Propagating with ?

The `?` operator makes the code much simpler. It **passes** the error on to the
function that called, like an employee who sends a file back to the boss instead of
handling it. Placed after a `Result`:

- if it is `Ok(v)`, it gives `v`;
- if it is `Err(e)`, the function stops and returns `Err(e)`.

```rust
use std::num::ParseIntError;

fn sum(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    println!("{:?}", sum("2", "40"));
    println!("{}", sum("2", "forty").is_err());
}
```

```sortie
Ok(42)
true
```

> **Remember:** `?` can only be used in a function that itself returns a `Result` (or
> an `Option`).

`ParseIntError` is the error type returned by `parse` for an integer. `main` can also
return a `Result`. `Ok(())` means "succeeded, with no value to give":

```rust
fn main() -> Result<(), std::num::ParseIntError> {
    let n: i32 = "7".parse()?;
    println!("{}", n * 6);
    Ok(())
}
```

```sortie
42
```

## unwrap and expect

`unwrap()` gets the value out of an `Ok` (or a `Some`), and causes a `panic!` otherwise.
`expect()` does the same, with your own message:

```rust
let n: i32 = "12".parse().unwrap();
let m: i32 = "30".parse().expect("a number was expected");
println!("{}", n + m);
```

```sortie
42
```

If the value is missing, the program stops with your message:

```rust norun
fn main() {
    let n: i32 = "twelve".parse().expect("a number was expected");
}
```

```text
thread 'main' panicked at src/main.rs:2:35:
a number was expected: ParseIntError { kind: InvalidDigit }
```

> **Tip:** `unwrap` is handy in tests and examples. In a real program, prefer `match`,
> `?` or a default value.

## Default values and combinators

Short methods save you from writing a full `match`. They work on `Option` and on
`Result`:

```rust
let empty: Option<i32> = None;
println!("{}", empty.unwrap_or(0));
println!("{:?}", Some(5).map(|n| n * 2));
println!("{}", "abc".parse::<i32>().unwrap_or(-1));
```

```sortie
0
Some(10)
-1
```

| Method | Effect |
|---------|-------|
| `unwrap_or(v)` | the value, or `v` if missing |
| `map(f)` | transforms the value if it exists |
| `is_some()` / `is_ok()` | tests for presence or success |

## Your own error type

For a bigger program, create an [enumeration](types.md) that lists the possible
errors. `match` then lets you react to each one:

```rust
enum AccountError {
    InsufficientBalance,
    ZeroAmount,
}

fn withdraw(balance: u32, amount: u32) -> Result<u32, AccountError> {
    if amount == 0 {
        return Err(AccountError::ZeroAmount);
    }
    if amount > balance {
        return Err(AccountError::InsufficientBalance);
    }
    Ok(balance - amount)
}

fn main() {
    for amount in [30, 0, 80] {
        match withdraw(50, amount) {
            Ok(left) => println!("Withdrawal OK, {left} € left"),
            Err(AccountError::ZeroAmount) => println!("Error: the amount must be positive"),
            Err(AccountError::InsufficientBalance) => println!("Error: insufficient balance"),
        }
    }
}
```

```sortie
Withdrawal OK, 20 € left
Error: the amount must be positive
Error: insufficient balance
```

## Common mistakes

### Using ? in a function that does not return a Result

```rust norun
fn main() {
    let n: i32 = "7".parse()?;
    println!("{n}");
}
```

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:2:29
  |
1 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
2 |     let n: i32 = "7".parse()?;
  |                             ^ cannot use the `?` operator in a function that returns `()`
```

`?` must be able to return the error. Make the function return a `Result`:
`fn main() -> Result<(), std::num::ParseIntError>`, with `Ok(())` at the end.

### Taking a Result for its value

```rust norun
fn main() {
    let n: i32 = "42".parse::<i32>();
    println!("{n}");
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:2:18
  |
2 |     let n: i32 = "42".parse::<i32>();
  |            ---   ^^^^^^^^^^^^^^^^^^^ expected `i32`, found `Result<i32, ParseIntError>`
  |            |
  |            expected due to this
```

`parse` returns the envelope, not the number. Open it: `match`, `?`, `unwrap_or(0)` or
`expect("…")`.

### Forgetting the Err case

```rust norun
fn main() {
    match "42".parse::<i32>() {
        Ok(n) => println!("{n}"),
    }
}
```

```text
error[E0004]: non-exhaustive patterns: `Err(_)` not covered
 --> src/main.rs:2:11
  |
2 |     match "42".parse::<i32>() {
  |           ^^^^^^^^^^^^^^^^^^^ pattern `Err(_)` not covered
```

Rust forces you to plan for failure. Add `Err(e) => println!("error: {e}"),`.

## Test yourself

**1.** When should you use `panic!` rather than `Result`?

> **Answer**
>
> For a bug, a situation that should never happen. An error you can foresee (missing
> file, invalid input) is handled with `Result`.

**2.** What does `?` do after an `Err(e)`?

> **Answer**
>
> It stops the function right away and returns `Err(e)` to the caller.

**3.** Read an age from the text `"abc"`, with `0` if reading fails.

> **Answer**
>
> ```rust
> let age: u32 = "abc".parse().unwrap_or(0);
> println!("{age}");
> ```
>
> ```sortie
> 0
> ```

## Summary

- `panic!` stops the program: keep it for real bugs.
- `Result` is either `Ok(value)` or `Err(error)`.
- `match` handles both cases; `unwrap_or` gives a default value.
- `?` returns the error to the caller, in a function that returns a `Result`.
- `unwrap` and `expect` are handy for tests, risky elsewhere.

Compare with [errors in Ruby](../ruby/erreurs.md) and
[in Rythme](../rythme/erreurs.md).

Next: [Closures in detail](closures.md)
