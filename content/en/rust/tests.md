# Testing your code

A test is a small program that checks that your code does what it should: Rust and Cargo make them very easy to write and run.

> **Goals:**
>
> - check a value with `assert!`, `assert_eq!` and `assert_ne!`;
> - write a test with `#[test]` and run it with `cargo test`;
> - put tests in a `#[cfg(test)]` module;
> - test that a function really stops with `#[should_panic]`;
> - write tests in the `tests/` folder and in the documentation.

## Why test?

When you change a program, you risk breaking what used to work. Checking everything by
hand, every time, takes long and you forget some cases.

A test does this check **automatically**, with one command. It is like a checklist
before an airplane takes off: quick, and reassuring.

## The checking macros

Three macros check a condition. If it is false, they cause a `panic!` with a clear
message.

| Macro | Checks that… |
|-------|--------------|
| `assert!(condition)` | the condition is true |
| `assert_eq!(a, b)` | `a` is equal to `b` |
| `assert_ne!(a, b)` | `a` is different from `b` |

```rust
let total = 2 + 2;
assert!(total > 3);
assert_eq!(total, 4);
assert_ne!(total, 5);
println!("All checks pass");
```

```sortie
All checks pass
```

You can add a message, written like in `println!`. It is shown only if the check
fails:

```rust
let age = 15;
assert!(age >= 0, "age cannot be negative: {age}");
println!("ok");
```

```sortie
ok
```

When a check fails, the program stops and shows both values:

```rust norun
assert_eq!(2 + 2, 5);
```

```text
assertion `left == right` failed
  left: 4
 right: 5
```

> **Note:** `left` is the first value, `right` the second. By habit, you put the
> computed result on the left and the expected value on the right.

## Writing a test with #[test]

A test is a function without parameters, preceded by `#[test]`. The test **passes** if
it runs to the end, and **fails** if it causes a `panic!` (for example with an `assert`
macro).

Tests go in a `tests` module, at the end of the file:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn adds_a_negative() {
        assert_eq!(add(2, -5), -3);
    }
}

fn main() {
    println!("{}", add(2, 3));
}
```

```sortie
5
```

Line by line:

- `#[cfg(test)]`: this module is compiled only for tests. It does not make your final
  program heavier;
- `mod tests`: a [module](modules.md) like any other;
- `use super::*;`: imports everything in the parent module (here, `add`);
- `#[test]`: marks each test function.

> **Tip:** give your tests names that describe what they check: `adds_a_negative`
> says more than `test2`.

## Running the tests: cargo test

In the project folder, type:

```sh
cargo test
```

Cargo compiles the project in test mode, runs each `#[test]` function and shows a
summary:

```text
running 2 tests
test tests::adds_two_numbers ... ok
test tests::adds_a_negative ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

If a test fails, Cargo tells you which one, with the two compared values. Here is what
is shown if you write `assert_eq!(add(2, -5), 3);` by mistake:

```text
running 2 tests
test tests::adds_two_numbers ... ok
test tests::adds_a_negative ... FAILED

failures:

---- tests::adds_a_negative stdout ----

thread 'tests::adds_a_negative' panicked at src/main.rs:16:9:
assertion `left == right` failed
  left: -3
 right: 3

failures:
    tests::adds_a_negative

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

To run only some tests, give a piece of their name:

```sh
cargo test negative   # runs only the tests whose name contains "negative"
```

> **Note:** tests run in parallel, so the order in which they are shown can change.
> And what your tests print with `println!` is hidden, except when they fail.

## Testing a stop: #[should_panic]

Sometimes, the right behavior is to **stop**. `#[should_panic]` says that the test must
cause a `panic!` to pass. With `expected`, you also check the message:

```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divides_normally() {
        assert_eq!(divide(10, 2), 5);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn rejects_zero() {
        divide(1, 0);
    }
}

fn main() {
    println!("{}", divide(10, 2));
}
```

```sortie
5
```

In the summary, this test shows up like this:

```text
test tests::rejects_zero - should panic ... ok
```

## What to test?

For each function, think of three kinds of cases:

- a **normal** case: `add(2, 3)`;
- an **edge** case: an empty list, zero, an empty text;
- an **error** case: an invalid input must give an `Err` (or a `panic!`).

```rust
fn average(grades: &[f64]) -> Option<f64> {
    if grades.is_empty() {
        return None;
    }
    Some(grades.iter().sum::<f64>() / grades.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_average() {
        assert_eq!(average(&[10.0, 20.0]), Some(15.0));
    }

    #[test]
    fn average_of_an_empty_list() {
        assert_eq!(average(&[]), None);
    }

    #[test]
    fn parsing_an_invalid_text() {
        assert!("twelve".parse::<i32>().is_err());
    }
}

fn main() {
    println!("{:?} {:?}", average(&[10.0, 20.0]), average(&[]));
}
```

```sortie
Some(15.0) None
```

> **Warning:** to compare two computed decimal numbers, exact equality can fail
> (`0.1 + 0.2` is not exactly `0.3`). Check instead that the difference is very
> small: `assert!((x - 0.3).abs() < 1e-9);`.

## Integration tests: the tests/ folder

The tests of the `tests` module check each function from the inside. **Integration
tests** use your code from the outside, like another program would. They need a
library (`src/lib.rs`, see [Modules and crates](modules.md)):

```text
calc/
├── Cargo.toml
├── src/
│   └── lib.rs           ← pub fn add(…)
└── tests/
    └── calculations.rs  ← the integration tests
```

Each file in `tests/` is compiled separately. You import the library by the project
name, and you do not need `#[cfg(test)]`:

```rust norun
// tests/calculations.rs
use calc::add;

#[test]
fn adds_from_outside() {
    assert_eq!(add(10, 32), 42);
}
```

`cargo test` then runs both kinds of tests, one after the other:

```text
     Running unittests src/lib.rs
running 3 tests
...
     Running tests/calculations.rs
running 1 test
test adds_from_outside ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Tests in the documentation

`///` comments document a function. A code example written inside them, between three
backticks, is **also a test**: `cargo test` compiles it and runs it. So your
documentation always stays correct.

````rust norun
/// Adds two numbers.
///
/// ```
/// let total = calc::add(2, 3);
/// assert_eq!(total, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
````

In the `cargo test` summary, these tests show up at the end:

```text
   Doc-tests calc

running 1 test
test src/lib.rs - add (line 3) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

> **Note:** documentation tests only work in a library
> (`src/lib.rs`).

## Common mistakes

### Forgetting use super::*

```rust norun
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    #[test]
    fn adds() {
        assert_eq!(add(2, 3), 5);
    }
}
```

```text
error[E0425]: cannot find function `add` in this scope
help: consider importing this function
  |
7 +     use crate::add;
```

The `tests` module is a separate drawer: it does not see the functions of the parent.
Add `use super::*;` at the start of the module.

### Comparing structs without PartialEq or Debug

```rust norun
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    assert_eq!(a, Point { x: 1, y: 2 });
}
```

```text
error[E0369]: binary operation `==` cannot be applied to type `Point`
help: consider annotating `Point` with `#[derive(PartialEq)]`

error[E0277]: `Point` doesn't implement `Debug`
help: consider annotating `Point` with `#[derive(Debug)]`
```

`assert_eq!` needs to compare (`PartialEq`) and to display (`Debug`) the values. Add
both at once:

```rust
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    assert_eq!(a, Point { x: 1, y: 2 });
    println!("equal");
}
```

```sortie
equal
```

### A should_panic test that does not stop

If the function does not cause a `panic!`, the `#[should_panic]` test fails:

```text
test tests::rejects_zero - should panic ... FAILED

---- tests::rejects_zero stdout ----
note: test did not panic as expected
```

Check that you really call the function with a value that must stop it (here,
`divide(1, 0)` and not `divide(4, 2)`).

## Test yourself

1. What is the difference between `assert!` and `assert_eq!`?

> **Answer**
>
> `assert!` checks that a condition is true. `assert_eq!` checks that two values are
> equal, and shows both of them (`left` and `right`) when it fails.

2. What is `#[cfg(test)]` in front of `mod tests` for?

> **Answer**
>
> The module is compiled only for `cargo test`. It is not included in the final
> program.

3. A test must check that a function stops with `panic!`. What do you need to add?

> **Answer**
>
> The `#[should_panic]` attribute under `#[test]` (with `expected = "…"` to check the
> message).

## Summary

- `assert!`, `assert_eq!` and `assert_ne!` check a condition, otherwise `panic!`.
- A test is a `#[test]` function, placed in a `#[cfg(test)] mod tests` module with
  `use super::*;`.
- `cargo test` runs all the tests and shows a summary.
- `#[should_panic]` tests that a function really stops.
- The `tests/` folder holds the integration tests; the examples in `///` comments are
  tests too.

Compare with [tests in Ruby](../ruby/tests.md) and
[in Rythme](../rythme/tests.md). Next:
[Organizing a project with Cargo](projet.md).

Next: [Debugging in Rust](debogage.md)
