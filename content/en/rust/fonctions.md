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

Line by line:

- `fn greet() { … }` describes the function: its name, `greet`, and what it does.
  Nothing is displayed yet at this point;
- the program always starts with `main`;
- the first `greet();` calls the function: Rust runs its block, displays "Hello!",
  then comes back into `main`;
- the second `greet();` does exactly the same thing again.

The order does not matter: a function can be written before or after `main`.

## Parameters

A function that always does exactly the same thing is of little use. Parameters give
it different information at each call, like a pancake recipe that you make for 2 or
for 6 people.

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

Line by line:

- `fn introduce(name: &str, age: u32)` announces two parameters: a text `name` and a
  positive whole number `age`;
- `introduce("Hugo", 12);` calls the function: `name` is `"Hugo"` and `age` is `12`,
  then `println!` displays the sentence;
- the second call starts again with other values: `"Inès"` and `34`.

Here, `name` and `age` are the parameters; `"Hugo"` and `12` are the arguments.

**Your turn:** write a function `show_price` that receives the name of an item (`&str`)
and its price in euros (`u32`), then displays for example "Bread: 2 euros". Call it for
bread at 2 euros and for cheese at 5 euros.

> **Solution**
>
> ```rust
> fn show_price(item: &str, price: u32) {
>     println!("{item}: {price} euros");
> }
>
> fn main() {
>     show_price("Bread", 2);
>     show_price("Cheese", 5);
> }
> ```
>
> ```sortie
> Bread: 2 euros
> Cheese: 5 euros
> ```

## The return type

A function can also do a calculation and **give you back** the result, like a
calculator. You can then store this result in a variable and use it again.

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

Line by line:

- `fn square(n: i32) -> i32`: the function receives a whole number `n` and promises to
  give back a whole number;
- in `main`, `square(7)` calls the function: `n` is 7;
- the last line of the function, `n * n`, computes 49. It has no `;`: it is the
  returned value;
- `let result = square(7);` therefore stores 49 in `result`, which `println!` displays.

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

**Your turn:** write a function `perimeter` that receives the side of a square (`i32`)
and returns its perimeter, that is 4 times the side. Display the perimeter of a square
with a side of 5.

> **Solution**
>
> ```rust
> fn perimeter(side: i32) -> i32 {
>     4 * side
> }
>
> fn main() {
>     println!("{}", perimeter(5));
> }
> ```
>
> ```sortie
> 20
> ```

## Expressions and statements

This difference explains a rule you have just seen: the last line of a function has no
semicolon when it is the one that gives the returned value.

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

Line by line:

- `let x = {` opens a block whose value will go into `x`;
- `let a = 3;` and `let b = 4;` are statements: they end with `;` and produce no
  value;
- `a * b` has no `;`: it is an expression, worth 12. It is the value of the block;
- so `x` receives 12, and `println!` displays it.

> **Warning:** an extra semicolon turns the expression into a statement. The
> function then no longer returns anything (see [Common mistakes](#common-mistakes)).

## A function that calls itself

> **Note:** this part can wait. Read it once the basics feel solid.

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

Line by line:

- `factorial(5)` computes `5 * factorial(4)`: it first needs the answer for 4;
- in the same way, `factorial(4)` asks for `factorial(3)`, then 2, then 1;
- `factorial(1)` is the base case: it returns 1 without calling itself;
- the answers come back up: 2 × 1 = 2, then 3 × 2 = 6, 4 × 6 = 24 and 5 × 24 = 120.

## Closures

Sometimes you need a very small function, just in one place: for example to tell
another function which calculation to do. Writing it separately with `fn` would be
heavy. A closure fits on one line, right where you need it.

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

Line by line:

- `let double = |x: i32| x * 2;` stores a closure in `double`: it receives `x` and
  returns `x * 2`;
- `let add = |a, b| a + b;` receives two values and returns their sum. Rust guesses
  their type;
- `double(5)` is called like a function and is worth 10; `add(2, 3)` is worth 5.

**Your turn:** write a closure `is_even` that receives an `i32` and returns `true` if
the number is even (the remainder of its division by 2 is 0). Display the result for 4,
then for 7.

> **Solution**
>
> ```rust
> let is_even = |n: i32| n % 2 == 0;
> println!("{}", is_even(4));
> println!("{}", is_even(7));
> ```
>
> ```sortie
> true
> false
> ```

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
