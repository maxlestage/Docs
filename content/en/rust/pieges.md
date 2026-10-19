# Beginner traps in Rust

This page gathers twelve traps that almost every Rust beginner falls into, with, for each one, what happens, why, and how to get out of it.

> **Goals:**
>
> - recognize the most common mistakes of the first weeks;
> - read the compiler's message without panicking;
> - know which small fix to apply each time.

## Before you start

Rust has a very strict [compiler](../debuter/glossaire.md#compiler). It refuses a
lot of programs. That is frustrating at first. But it is a friend: it stops you
**before** the program does something silly.

Each trap follows the same path:

```text
what you expect  →  what happens  →  why  →  the fix
```

> **Remember:** a Rust error message is not a punishment. It is an explanation, often
> with the fix at the bottom, after `help:`.

## Variables and text

### Trap 1: variables do not change

**What you expect**

You create a score at 0, then you add 10. You expect to see `10`.

```rust norun
fn main() {
    let score = 0;
    score = score + 10;
    println!("{score}");
}
```

**What happens**

```text
error[E0384]: cannot assign twice to immutable variable `score`
 --> src/main.rs:3:5
  |
2 |     let score = 0;
  |         ----- first assignment to `score`
3 |     score = score + 10;
  |     ^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut score = 0;
  |         +++
```

**Why**

In Rust, a variable is [immutable](../debuter/glossaire.md#immutable) by default. It no
longer changes once created. It is a label stuck on a closed box.

**The fix**

Add `mut` (for *mutable*, "changeable"):

```rust
fn main() {
    let mut score = 0;
    score = score + 10;
    println!("{score}");
}
```

```sortie
10
```

Try it: remove `mut`. The compiler itself suggests putting it back, with `+++` under
the spot to fill in.

### Trap 2: String and &str, two kinds of text

**What you expect**

You write a text between quotes in a variable of type `String`.

```rust norun
fn main() {
    let name: String = "Zoé";
    println!("{name}");
}
```

**What happens**

```text
error[E0308]: mismatched types
 --> src/main.rs:2:24
  |
2 |     let name: String = "Zoé";
  |               ------   ^^^^^ expected `String`, found `&str`
  |               |
  |               expected due to this
  |
help: try using a conversion method
  |
2 |     let name: String = "Zoé".to_string();
  |                             ++++++++++++
```

**Why**

A text between quotes is a `&str`: a text **written in the program**, that you only
read. A `String` is a text **of your own**, which can grow. It is the difference
between a poster stuck on the wall and a sheet in your notebook.

| Type | Picture | Can it grow? |
|------|---------|--------------|
| `&str` | a poster you look at | no |
| `String` | a sheet of your own | yes |

**The fix**

Turn the poster into a sheet with `String::from` or `.to_string()`:

```rust
fn main() {
    let name: String = String::from("Zoé");
    let city = "Lyon".to_string();
    println!("{name} lives in {city}");
}
```

```sortie
Zoé lives in Lyon
```

Everything is explained in [Working with text](texte.md).

### Trap 3: using a value after giving it away

**What you expect**

You copy a text into another variable, then you display both.

```rust norun
fn main() {
    let a = String::from("hello");
    let b = a;
    println!("{a} {b}");
}
```

**What happens**

```text
error[E0382]: borrow of moved value: `a`
 --> src/main.rs:4:16
  |
2 |     let a = String::from("hello");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
3 |     let b = a;
  |             - value moved here
4 |     println!("{a} {b}");
  |                ^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let b = a.clone();
  |              ++++++++
```

**Why**

`let b = a;` does not copy the `String`. It **gives** it to `b`. It is like giving
away a book: you no longer have it. We say that the value is "moved".

```text
before:  a ──► "hello"
after:   a     (empty)
         b ──► "hello"
```

This is the [ownership](../debuter/glossaire.md#ownership) rule. Don't panic: it has
its own chapter, [Ownership and borrowing](possession.md). For now, just remember the
fix.

**The fix**

If you really want two texts, make a copy with `.clone()`:

```rust
fn main() {
    let a = String::from("hello");
    let b = a.clone();
    println!("{a} {b}");
}
```

```sortie
hello hello
```

> **Note:** numbers (`i32`, `f64`…) and `bool` are not affected. They are small, so
> Rust copies them on its own. `let y = x;` keeps `x` usable.

## Numbers and functions

### Trap 4: a number that overflows

**What you expect**

A `u8` goes from 0 to 255. You start from 250 and add 1, ten times. You expect to see
`260`.

```rust norun
fn main() {
    let mut level: u8 = 250;
    for _ in 0..10 {
        level += 1;
    }
    println!("{level}");
}
```

**What happens**

The program compiles, then stops dead while running:

```text
thread 'main' (1667) panicked at src/main.rs:4:9:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

**Why**

A `u8` variable is a box that cannot hold more than 255. When it reaches 256, the
number "overflows". In development mode, Rust prefers to stop: this is called a
[panic](../debuter/glossaire.md#panic).

| Round | level before | level after |
|-------|--------------|-------------|
| 1 | 250 | 251 |
| … | … | … |
| 5 | 254 | 255 |
| 6 | 255 | overflow: panic |

**The fix**

Choose a type big enough. `i32` goes up to more than two billion:

```rust
fn main() {
    let mut level: i32 = 250;
    for _ in 0..10 {
        level += 1;
    }
    println!("{level}");
}
```

```sortie
260
```

For more details, see [Overflow](nombres.md#overflow).

### Trap 5: 7 divided by 2 gives 3

**What you expect**

You share 7 candies between 2 children. You expect to see `3.5`.

```rust
fn main() {
    let candies = 7;
    let children = 2;
    println!("{}", candies / children);
    println!("{}", candies % children);
}
```

```sortie
3
1
```

**Why**

`7` and `2` are [integers](../debuter/glossaire.md#integer). Between two integers, `/`
does an **integer** division: it keeps the part before the decimal point. `%` gives
the remainder. It is exactly sharing candies: 3 each, and 1 is left over.

**The fix**

For a decimal result, work with `f64` values, by writing `7.0` and `2.0`:

```rust
fn main() {
    let candies = 7.0;
    let children = 2.0;
    println!("{}", candies / children);
}
```

```sortie
3.5
```

Try it: write `7.0 / 2`. Rust refuses to mix an `f64` and an integer. Write
`7.0 / 2.0`.

### Trap 6: one semicolon too many

**What you expect**

Your function computes the double of a number. You expect to see `8`.

```rust norun
fn double(x: i32) -> i32 {
    x * 2;
}

fn main() {
    println!("{}", double(4));
}
```

**What happens**

```text
error[E0308]: mismatched types
 --> src/main.rs:1:22
  |
1 | fn double(x: i32) -> i32 {
  |    ------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     x * 2;
  |          - help: remove this semicolon to return this value
```

**Why**

In Rust, the last line **without** a semicolon is the returned value. With a
semicolon, the line becomes a simple statement: it computes, then throws the result
away. The function then returns "nothing", which is written `()`.

```text
x * 2     →  a value: 8          (returned)
x * 2;    →  a statement         (the 8 is thrown away, () is left)
```

**The fix**

Remove the semicolon from the last line:

```rust
fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    println!("{}", double(4));
}
```

```sortie
8
```

See also [Expressions and statements](fonctions.md#expressions-and-statements).

## Text and types

### Trap 7: reading a letter with [0]

**What you expect**

You want the first letter of "cat". You expect to see `c`.

```rust norun
fn main() {
    let word = String::from("cat");
    let first = word[0];
    println!("{first}");
}
```

**What happens**

```text
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> src/main.rs:3:22
  |
3 |     let first = word[0];
  |                      ^ string indices are ranges of `usize`
  |
  = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
  = note: you can use `.chars().nth()` or `.bytes().nth()`
```

**Why**

Rust stores text in [Unicode](../debuter/glossaire.md#unicode). A letter like `é`
takes 2 slots in memory, `a` takes only one. So slot 0 is not always a whole letter.
Rust prefers to refuse rather than give you half a letter.

**The fix**

Walk through the letters with `.chars()`, then take the one you want:

```rust
fn main() {
    let word = String::from("éclair");
    let first = word.chars().next();
    let third = word.chars().nth(2);
    println!("{:?} {:?}", first, third);
}
```

```sortie
Some('é') Some('l')
```

The result is an [Option](../debuter/glossaire.md#option): the word might be too
short. See [Walking through the characters](texte.md#walking-through-the-characters).

### Trap 8: comparing two kinds of integers

**What you expect**

You compare the length of a list with a number. You expect to see `big`.

```rust norun
fn main() {
    let v = vec![10, 20, 30];
    let limit: i32 = 2;
    if v.len() > limit {
        println!("big");
    }
}
```

**What happens**

```text
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     if v.len() > limit {
  |        -------   ^^^^^ expected `usize`, found `i32`
  |        |
  |        expected because this is `usize`
```

**Why**

`len()` returns a `usize`: the type of sizes and positions. `limit` is an `i32`. For
Rust, these are two different [types](../debuter/glossaire.md#type), even though they
both hold whole numbers. It never mixes them on its own.

**The fix**

The simplest: give `limit` the right type from the start.

```rust
fn main() {
    let v = vec![10, 20, 30];
    let limit: usize = 2;
    if v.len() > limit {
        println!("big");
    }
}
```

```sortie
big
```

> **Tip:** a variable used as a position or a size in a list should be a `usize`.

## Loops, lists and Option

### Trap 9: unwrap on an empty box

**What you expect**

You ask for the first element of a list, and you open it with `unwrap()`.

```rust norun
fn main() {
    let v: Vec<i32> = Vec::new();
    let first = v.first().unwrap();
    println!("{first}");
}
```

**What happens**

```text
thread 'main' (1735) panicked at src/main.rs:3:27:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

**Why**

`first()` returns an `Option`: a box that holds either `Some(value)` or `None`
(nothing). The list is empty, so the box is too. `unwrap()` means "open the box, and
if there is nothing inside, stop everything".

**The fix**

Plan for the empty case, with `match` or with `unwrap_or`:

```rust
fn main() {
    let v: Vec<i32> = Vec::new();
    match v.first() {
        Some(x) => println!("first: {x}"),
        None => println!("the list is empty"),
    }
    let first = v.first().copied().unwrap_or(0);
    println!("{first}");
}
```

```sortie
the list is empty
0
```

> **Remember:** `unwrap()` is handy for trying things out. In a real program, each
> `unwrap()` is a question: "am I **sure** the box is not empty?"

### Trap 10: shadowing instead of changing

**What you expect**

You add 1, 2 and then 3 to the total in a loop. You expect to see `6` at the end.

```rust
fn main() {
    let total = 0;
    for n in 1..=3 {
        let total = total + n;
        println!("in the loop: {total}");
    }
    println!("at the end: {total}");
}
```

```sortie
in the loop: 1
in the loop: 2
in the loop: 3
at the end: 0
```

**Why**

`let total = …` in the loop creates a **new** variable, which has the same name. This
is called **shadowing**. It hides the old one, then disappears at the end of the
round. The old `total` never moved.

| Round | n | loop total | outside total |
|-------|---|------------|---------------|
| 1 | 1 | 0 + 1 = 1 | 0 |
| 2 | 2 | 0 + 2 = 2 | 0 |
| 3 | 3 | 0 + 3 = 3 | 0 |

**The fix**

To change the same variable: `let mut` only once, then **no** `let` in the loop.

```rust
fn main() {
    let mut total = 0;
    for n in 1..=3 {
        total = total + n;
    }
    println!("at the end: {total}");
}
```

```sortie
at the end: 6
```

> **Remember:** `let` creates a new variable. `x = …` (without `let`) changes the one
> that exists.

### Trap 11: a slot that does not exist

**What you expect**

A list has 3 grades. You want the third one, so you write `grades[3]`.

```rust norun
fn main() {
    let grades = vec![12, 15, 9];
    println!("{}", grades[3]);
}
```

**What happens**

```text
thread 'main' (1765) panicked at src/main.rs:3:26:
index out of bounds: the len is 3 but the index is 3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

**Why**

Positions (the [indexes](../debuter/glossaire.md#index)) start at **0**. Three slots
have the numbers 0, 1 and 2. Slot 3 does not exist.

```text
position:   0    1    2
grades  : [ 12 | 15 |  9 ]
```

**The fix**

The third grade is `grades[2]`. If you are not sure, use `get`, which returns an
`Option` instead of stopping:

```rust
fn main() {
    let grades = vec![12, 15, 9];
    println!("{}", grades[2]);
    println!("{:?}", grades.get(3));
}
```

```sortie
9
None
```

### Trap 12: the range that stops too early

**What you expect**

You count from 1 to 5 with `1..5`. You expect to see 5 numbers.

```rust
fn main() {
    for i in 1..5 {
        print!("{i} ");
    }
    println!();
}
```

```sortie
1 2 3 4 
```

**Why**

A [range](../debuter/glossaire.md#range) `1..5` stops **before** 5. This is
handy for positions: `0..v.len()` goes over every slot, without going too far.

**The fix**

To include the end, add `=`: `1..=5`.

```rust
fn main() {
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();
}
```

```sortie
1 2 3 4 5 
```

| Written | Numbers you get |
|---------|-----------------|
| `1..5` | 1, 2, 3, 4 |
| `1..=5` | 1, 2, 3, 4, 5 |

## Common mistakes

This whole page is about mistakes. Here are two more, very short ones.

### Displaying a list with {}

```rust norun
fn main() {
    let v = vec![1, 2, 3];
    println!("{}", v);
}
```

```text
error[E0277]: `Vec<{integer}>` doesn't implement `std::fmt::Display`
 --> src/main.rs:3:20
  |
3 |     println!("{}", v);
  |               --   ^ `Vec<{integer}>` cannot be formatted with the default formatter
  |               |
  |               required by this formatting parameter
  |
  = help: the trait `std::fmt::Display` is not implemented for `Vec<{integer}>`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

`{}` is made for humans, and Rust does not know how to present a list. Use `{:?}`,
the "developer" display: `println!("{:?}", v);` displays `[1, 2, 3]`.

### parse without saying which type you want

```rust norun
fn main() {
    let n = "42".parse().unwrap();
    println!("{}", n + 1);
}
```

```text
error[E0284]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let n = "42".parse().unwrap();
  |         ^        ----- type must be known at this point
  |
  = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `n` an explicit type
  |
2 |     let n: /* Type */ = "42".parse().unwrap();
  |          ++++++++++++
```

`parse` can build an integer, a decimal number, a boolean… Say which one:
`let n: i32 = "42".parse().unwrap();`.

## Test yourself

**1.** What does this program display: `let x = 10; let y = 4; println!("{}", x / y);`?

> **Answer**
>
> `2`. These are two integers: the division keeps only the whole part. To get `2.5`,
> write `10.0 / 4.0`.

**2.** Why does this function not compile?
`fn triple(x: i32) -> i32 { x * 3; }`

> **Answer**
>
> Because of the semicolon after `x * 3`. The line no longer returns anything (`()`),
> while the function promises an `i32`. Write `x * 3` without a semicolon.

**3.** After `let a = String::from("ok"); let b = a;`, can you still display `a`?
How do you keep both?

> **Answer**
>
> No: the `String` was given (moved) to `b`. To keep both, write
> `let b = a.clone();`.

## Summary

- A variable is immutable by default: add `mut` to change it.
- `"text"` is a `&str`; `String::from("text")` gives a `String`.
- `let b = a;` gives a `String` to `b`: use `.clone()` to keep both.
- Between two integers, `/` cuts off the decimals; an overflow stops the program.
- No semicolon after the returned value; no `[0]` on a text.
- Positions start at 0, `1..5` excludes 5, and `unwrap()` on `None` stops everything.

Next: [Exercises to get started](exercices-faciles.md)
