# Functions

This page explains how to write functions with `fn`, give them parameters and a return type, and use closures.

> **Goals:**
>
> - declare a function with `fn` and call it;
> - give it typed parameters and a return type;
> - understand the difference between an expression and a statement;
> - return several values with a tuple;
> - write a closure and pass it to another function.

## Declaring a function

A [function](../debuter/glossaire.md#function) is a piece of code that has a
name. You write it once, then you **call** it as many times as you want,
like a cooking recipe you make again and again.

A function is declared with `fn`, a name in `lowercase_with_underscores`,
parentheses and a block:

```rust
fn greet() {
    println!("Hello!");
}

fn main() {
    greet();
    greet();
}
```

```sortie
Hello!
Hello!
```

The order does not matter: a function can be written before or after `main`.

## Parameters

A [parameter](../debuter/glossaire.md#parameter) is a variable that the function
receives. The value given at the moment of the call is called an
[argument](../debuter/glossaire.md#argument).

Each parameter has a name **and a type**. The type is required.

```rust
fn introduce(name: &str, age: u32) {
    println!("{name} is {age} years old.");
}

fn main() {
    introduce("Hugo", 12);
    introduce("Inès", 34);
}
```

```sortie
Hugo is 12 years old.
Inès is 34 years old.
```

Here, `name` and `age` are the parameters; `"Hugo"` and `12` are the arguments.

## The return type

The returned type is written after an arrow `->`. The **last expression**, without a
semicolon, is the returned value:

```rust
fn square(n: i32) -> i32 {
    n * n
}

fn main() {
    let result = square(7);
    println!("{result}");
}
```

```sortie
49
```

To leave early, use `return`:

```rust
fn sign(n: i32) -> &'static str {
    if n < 0 {
        return "negative";
    }
    if n == 0 {
        return "zero";
    }
    "positive"
}

fn main() {
    println!("{} {} {}", sign(-3), sign(0), sign(8));
}
```

```sortie
negative zero positive
```

> **Note:** `&'static str` means a text written directly in the program. The
> `'static` is a lifetime, covered in
> [Ownership and borrowing](possession.md).

A function without `->` returns `()`, pronounced "unit": an empty value, which means
"nothing".

### Returning several values

A **tuple** groups several values (see [First steps](premiers-pas.md#tuples)):

```rust
fn division(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn main() {
    let (quotient, remainder) = division(17, 5);
    println!("{quotient} remainder {remainder}");
}
```

```sortie
3 remainder 2
```

## Expressions and statements

This is an important idea in Rust:

- an **expression** produces a value: `5`, `a + b`, `if … { … } else { … }`, a
  block `{ … }`;
- a **statement** does an action and produces nothing. It ends with `;`.

A block between curly braces is an expression. Its value is its last line:

```rust
let x = {
    let a = 3;
    let b = 4;
    a * b
};
println!("{x}");
```

```sortie
12
```

> **Warning:** an extra semicolon turns the expression into a statement. The
> function then no longer returns anything (see [Common mistakes](#common-mistakes)).

## A function that calls itself

A function can call itself: this is
[recursion](../debuter/glossaire.md#recursion). There must always be a base case, otherwise
it never stops:

```rust
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!("{}", factorial(5));
}
```

```sortie
120
```

## Closures

A [closure](../debuter/glossaire.md#closure) is a small function without a name, that
you store in a variable. Its parameters are written between bars `|…|`. The types
are often guessed:

```rust
let double = |x: i32| x * 2;
let add = |a, b| a + b;
println!("{}", double(5));
println!("{}", add(2, 3));
```

```sortie
10
5
```

### Capturing the environment

A closure can use the variables around it. It **captures** them, like
a backpack it carries with it:

```rust
let bonus = 10;
let with_bonus = |grade: i32| grade + bonus;
println!("{}", with_bonus(12));
```

```sortie
22
```

A closure that changes a captured variable must be stored in a `mut` variable:

```rust
let mut total = 0;
let mut add = |n: i32| total += n;
add(5);
add(7);
println!("{total}");
```

```sortie
12
```

### Passing a closure to a function

Closures are mostly used to customize the work of another function. Here, the
parameter `f` accepts any function that takes an `i32` and returns an `i32`:

```rust
fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    println!("{}", apply_twice(|n| n + 3, 1));
    println!("{}", apply_twice(|n| n * n, 3));
}
```

```sortie
7
81
```

Read `impl Fn(i32) -> i32` like this: "any function that takes an `i32` and
returns an `i32`". You find them everywhere with iterators: see
[Collections](collections.md).

## Common mistakes

### Forgetting the type of a parameter

```rust norun
fn double(x) -> i32 {
    x * 2
}

fn main() {
    println!("{}", double(4));
}
```

```text
error: expected one of `:`, `@`, or `|`, found `)`
 --> src/main.rs:1:12
  |
1 | fn double(x) -> i32 {
  |            ^ expected one of `:`, `@`, or `|`
help: if this is a parameter name, give it a type
```

Fix: `fn double(x: i32) -> i32`.

### A semicolon after the returned value

```rust norun
fn square(n: i32) -> i32 {
    n * n;
}

fn main() {
    println!("{}", square(3));
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:1:22
  |
1 | fn square(n: i32) -> i32 {
  |    ------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     n * n;
  |          - help: remove this semicolon to return this value
```

Fix: remove the `;` after `n * n`.

### Forgetting the return type

```rust norun
fn square(n: i32) {
    n * n
}

fn main() {
    println!("{:?}", square(3));
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:2:5
  |
1 | fn square(n: i32) {
  |                  - help: try adding a return type: `-> i32`
2 |     n * n
  |     ^^^^^ expected `()`, found `i32`
```

Without `->`, the function returns nothing. Add `-> i32`.

### Forgetting an argument

```rust norun
fn introduce(name: &str, age: u32) {
    println!("{name} is {age} years old.");
}

fn main() {
    introduce("Hugo");
}
```

```text
error[E0061]: this function takes 2 arguments but 1 argument was supplied
 --> src/main.rs:6:5
  |
6 |     introduce("Hugo");
  |     ^^^^^^^^^-------- argument #2 of type `u32` is missing
```

Give one argument per parameter, in order: `introduce("Hugo", 12);`.

## Test yourself

**1.** What does `fn f() -> i32 { 1; 2 }` return?

> **Answer**
>
> `2`: it is the last expression, without a semicolon.

**2.** Write a function `average(a: f64, b: f64) -> f64`.

> **Answer**
>
> ```rust
> fn average(a: f64, b: f64) -> f64 {
>     (a + b) / 2.0
> }
>
> fn main() {
>     println!("{}", average(12.0, 15.0));
> }
> ```
>
> ```sortie
> 13.5
> ```

**3.** Write a closure `triple` that multiplies a number by 3.

> **Answer**
>
> ```rust
> let triple = |x: i32| x * 3;
> println!("{}", triple(4));
> ```
>
> ```sortie
> 12
> ```

## Summary

- `fn name(param: Type) -> Return { … }` declares a function.
- The type of each parameter is required.
- The last expression, without `;`, is the returned value; `return` leaves early.
- A tuple `(a, b)` returns several values.
- `|x| x * 2` is a closure: a small function without a name, which can capture the
  variables around it.

Compare with [functions in Ruby](../ruby/fonctions.md) and
[in Rythme](../rythme/fonctions.md).

Next: [Working with text](texte.md).
