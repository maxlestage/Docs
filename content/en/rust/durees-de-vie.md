# Lifetimes

A lifetime is the span during which a reference stays valid: this chapter explains why Rust checks them, and how to write them when the compiler asks you to.

> **Goals:**
>
> - understand what a dangling reference is, and why Rust rejects it;
> - read and write a `'a` annotation on a function;
> - know when you can skip the annotation;
> - store a reference in a struct;
> - know when to simply use a `String` instead.

This chapter follows on from [Ownership and borrowing](possession.md). Read it again if
the words "borrow" and "reference" are not familiar to you yet.

## The problem: the dangling reference

A [reference](../debuter/glossaire.md#borrowing) `&x` points to a value that belongs to
someone else. What happens if that value disappears while the reference still exists?

Picture a cloakroom ticket. The ticket (the reference) tells where your coat (the value)
is. If the cloakroom closes and throws away all the coats, your ticket points to an
empty hook, or worse, to someone else's coat. This is a **dangling reference**.

```rust norun
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    } // x disappears here
    println!("{r}"); // r would point to freed memory
}
```

```text
error[E0597]: `x` does not live long enough
 --> src/main.rs:5:13
  |
4 |         let x = 5;
  |             - binding `x` declared here
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     } // x disappears here
  |     - `x` dropped here while still borrowed
7 |     println!("{r}"); // r would point to freed memory
  |                - borrow later used here
```

In C, this program would compile and print garbage. Rust rejects it **before** it runs.
The message reads: "`x` does not live long enough; it is freed on line 6 while it is
still borrowed".

The fix is simple: the value must live at least as long as the reference.

```rust
let x = 5;
let r = &x;
println!("{r}");
```

```sortie
5
```

## The borrow checker

The **borrow checker** is the part of the
[compiler](../debuter/glossaire.md#compiler) that does this check. Think of a very
strict librarian: they write down the return date of every book they lend, and they
refuse any loan that would last longer than the book itself.

For each reference, it works out two zones of the code:

- the zone where the **value** exists (from its creation to the end of its block);
- the zone where the **reference** is used (from its creation to its last use).

The second one must fit entirely inside the first one. This zone is called the
**lifetime** of the reference.

```text
let x = 5;           ──┐ x exists
let r = &x;          ──┼─┐ r is used
println!("{r}");     ──┼─┘
                       │
}                    ──┘ x disappears: all good
```

> **Remember:** a lifetime is not a duration in seconds. It is a portion of the source
> code, checked at compile time. It costs nothing at run time.

## Annotating a function with 'a

Most of the time, the compiler tracks lifetimes on its own. But a function that
**returns a reference** raises a question: which parameter does it come from?

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("apricot");
    let b = String::from("kiwi");
    println!("{}", longest(&a, &b));
}
```

```sortie
apricot
```

Let's break down the signature:

- `<'a>` declares a lifetime named `'a` (an apostrophe, then a short name);
- `x: &'a str` and `y: &'a str`: both texts live at least during `'a`;
- `-> &'a str`: the result is valid during `'a`, no longer.

In plain words: "the result lives as long as **the shorter** of the two parameters".
The annotation does not change how long the values live. It **describes** a link, so
that the compiler can check every call.

### The link is checked at the call

Thanks to this link, the compiler knows that `result` depends on `a` **and** on `b`.
Here, everything is used inside the block of `b`: this is allowed.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("apricot");
    {
        let b = String::from("kiwi");
        let result = longest(&a, &b);
        println!("{result}");
    }
}
```

```sortie
apricot
```

Using `result` after the end of the block of `b` would be rejected, even though the
result is actually `a`. The compiler only looks at the signature (see
[Common mistakes](#common-mistakes)).

### A lifetime for a single parameter

If the result depends on only one parameter, link only that one:

```rust
fn first_word<'a>(text: &'a str, _separator: &str) -> &'a str {
    text.split(' ').next().unwrap_or("")
}

fn main() {
    println!("{}", first_word("hello everyone", " "));
}
```

```sortie
hello
```

## When can you write nothing?

Writing `'a` everywhere would be tiresome. So the compiler applies three rules, called
**elision rules** (to elide = to leave out). In a simple version:

1. Each reference parameter gets its own lifetime.
2. If there is **only one** reference parameter, the result takes its lifetime.
3. In a method with `&self`, the result takes the lifetime of `self`.

If these rules are enough, you write nothing. Otherwise, the compiler asks you for an
annotation.

```rust
// Rule 2: only one reference parameter, nothing to write.
fn trim_spaces(s: &str) -> &str {
    s.trim()
}

fn main() {
    println!("[{}]", trim_spaces("  cat  "));
}
```

```sortie
[cat]
```

The compiler reads this function as `fn trim_spaces<'a>(s: &'a str) -> &'a str`. With
two reference parameters, like `longest`, no rule can choose: you have to write `'a`.

## A reference in a struct

A struct can hold a reference, for example a `&str` that borrows an existing text. The
struct then needs a lifetime:

```rust
struct Excerpt<'a> {
    text: &'a str,
    line: usize,
}

impl<'a> Excerpt<'a> {
    fn show(&self) {
        println!("line {}: {}", self.line, self.text);
    }
}

fn main() {
    let book = String::from("Once upon a time.\nA little cat.");
    let second = book.lines().nth(1).unwrap();
    let e = Excerpt { text: second, line: 2 };
    e.show();
}
```

```sortie
line 2: A little cat.
```

Read `Excerpt<'a>` like this: "an `Excerpt` cannot live longer than the text it
borrows". Here, `e` does not live longer than `book`: all good.

> **Note:** `impl<'a> Excerpt<'a>` repeats the lifetime, just like you repeat a
> [generic](generiques.md) parameter.

## The 'static lifetime

`'static` is a special lifetime: "valid for the **whole** run of the program". Texts
written between quotes in the code are `&'static str`: they are stored inside the
program itself.

```rust
fn season(month: u32) -> &'static str {
    match month {
        12 | 1 | 2 => "winter",
        3..=5 => "spring",
        6..=8 => "summer",
        _ => "autumn",
    }
}

fn main() {
    println!("{}", season(7));
    println!("{}", season(1));
}
```

```sortie
summer
winter
```

Here, `'static` is required: the function has no reference parameter, so no elision
rule applies.

> **Warning:** if the compiler suggests `'static` to silence an error, be careful. It
> is almost never the right fix for a value computed while the program runs.

## Or else: use String

Lifetimes let you **borrow** without copying. But you don't have to borrow. A struct
that **owns** its text, with a `String`, needs no annotation at all:

```rust
struct Book {
    title: String,
    author: String,
}

fn make_book(title: &str) -> Book {
    Book { title: title.to_string(), author: String::from("unknown") }
}

fn main() {
    let b = make_book("Dune");
    println!("{} ({})", b.title, b.author);
}
```

```sortie
Dune (unknown)
```

| Situation | Recommended choice |
|-----------|--------------------|
| a struct that keeps data for a long time | `String` (it owns) |
| a function that creates a new text | return a `String` |
| a parameter that you only read | `&str` (borrow) |
| a small temporary view on an existing text | `&'a str` in a struct |

> **Tip:** as a beginner, put `String`s in your structs. Move on to references when you
> feel comfortable, or when copying really costs too much.

## Common mistakes

### Returning a reference to a local variable

```rust norun
fn greet(name: &str) -> &str {
    let sentence = format!("Hello {name}");
    &sentence
}

fn main() {
    println!("{}", greet("Zoé"));
}
```

```text
error[E0515]: cannot return reference to local variable `sentence`
 --> src/main.rs:3:5
  |
3 |     &sentence
  |     ^^^^^^^^^ returns a reference to data owned by the current function
```

`sentence` disappears at the end of the function: its reference would be dangling.
Return the `String` itself: `fn greet(name: &str) -> String`, then `sentence` without
`&`.

### A reference in a struct, without a lifetime

```rust norun
struct Book {
    title: &str,
}

fn main() {
    let b = Book { title: "Dune" };
    println!("{}", b.title);
}
```

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:12
  |
2 |     title: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct Book<'a> {
2 ~     title: &'a str,
  |
```

Follow the help (`struct Book<'a> { title: &'a str }`), or more simply, use
`title: String`.

### Using the result after one of the parameters is gone

```rust norun
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("apricot");
    let result;
    {
        let b = String::from("kiwi");
        result = longest(&a, &b);
    }
    println!("{result}");
}
```

```text
error[E0597]: `b` does not live long enough
  --> src/main.rs:10:30
   |
 9 |         let b = String::from("kiwi");
   |             - binding `b` declared here
10 |         result = longest(&a, &b);
   |                              ^^ borrowed value does not live long enough
11 |     }
   |     - `b` dropped here while still borrowed
12 |     println!("{result}");
   |                ------ borrow later used here
```

The signature says that the result may come from `b`. So it must not outlive `b`. Use
`result` inside the block, or declare `b` outside the block.

## Test yourself

**1.** What is a dangling reference?

> **Answer**
>
> A reference to a value that has already been freed. Rust rejects it at compile time.

**2.** Why doesn't `fn uppercase(s: &str) -> &str` need `'a`?

> **Answer**
>
> It has only one reference parameter: the elision rule gives its lifetime to the
> result.

**3.** Which lifetime must you write for a function with no parameter that returns
`"hello"`?

> **Answer**
>
> `'static`, because the text is written in the program:
>
> ```rust
> fn welcome() -> &'static str {
>     "hello"
> }
>
> fn main() {
>     println!("{}", welcome());
> }
> ```
>
> ```sortie
> hello
> ```

## Summary

- A reference must never live longer than the value it borrows.
- The borrow checker checks this at compile time, with no cost at run time.
- `'a` describes a link between the parameters and the result; it changes nothing about
  the values.
- With a single reference parameter, or `&self`, you write nothing: this is elision.
- `'static` lasts for the whole program, like texts between quotes.
- When in doubt, a `String` in the struct avoids any annotation.

Next: [structs, enums and traits](types.md)
