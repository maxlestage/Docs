# Ownership and borrowing

Ownership is the central idea of Rust: each value has a single owner, and the other parts of the code can only borrow it.

> **Goals:**
>
> - explain the three rules of ownership;
> - recognize a move, and avoid it with `clone` or a borrow;
> - borrow a value for reading with `&` and for writing with `&mut`;
> - understand the error messages of the borrow checker;
> - read a lifetime such as `'a`.

## Why ownership?

A program stores its data in memory. One day, that memory has to be freed.

- Some languages (C) ask you to free it by hand: you forget, or you free it
  twice.
- Others (Ruby, Python) use a garbage collector that runs while the program runs.

Rust chooses a third way: **simple rules**, checked by the compiler.
Memory is freed automatically, at the right moment, with no hidden cost.

A picture helps you understand everything: a value is an **object**, for example a book.
A variable that **owns** it is its owner. You can give the book away
(move it), make a photocopy of it (clone it) or lend it (borrow it).

## The three rules

> **Remember:**
>
> 1. Each value has **one owner** (a variable).
> 2. There is **only one** owner at a time.
> 3. When the owner goes away (end of its `{ }` block), the value is freed.

A variable only exists in the block where it is created: this is its
[scope](../debuter/glossaire.md#portee).

```rust
{
    let s = String::from("hello");
    println!("{s}");
} // here, s goes away: the memory is freed
println!("end of the block");
```

```sortie
hello
end of the block
```

## The move

When you give a `String` to another variable, it is **moved**. It is like giving
away your book: you no longer have it. The old variable
can no longer be used.

```text
let a = String::from("cat");
let b = a;

   a  ──╳                     (a is no longer valid)
   b  ──────►  [ "cat" ]      (b is the new owner)
```

Using `a` after this move is refused by the compiler. The same move
happens when you pass a value to a function: the function becomes the owner.
Both cases are shown in [Common mistakes](#common-mistakes).

### Copied types

Small simple types (`i32`, `f64`, `bool`, `char`…) are **copied** instead
of being moved: copying them costs almost nothing. We say they have the `Copy` trait.
Both variables remain usable:

```rust
let x = 5;
let y = x;
println!("{x} {y}");
```

```sortie
5 5
```

## Cloning a value

To have **two** independent values, use `clone()`. It is the photocopy:
everyone has their own copy.

```rust
let a = String::from("cat");
let mut b = a.clone();
b.push_str("s");
println!("{a} {b}");
```

```sortie
cat cats
```

```text
   a  ──────►  [ "cat" ]
   b  ──────►  [ "cats" ]     (a complete copy, modified)
```

> **Tip:** `clone()` is handy when you start. But copying takes time:
> borrowing, below, is often better.

## Borrowing with &

A **reference** `&` lets you use a value without taking it. We say you
**borrow** it. It is lending the book: the other person reads it, then gives it back. The
owner stays the same.

```text
   name  ──────►  [ "Zoé" ]
                     ▲
   &name ────────────┘   (we look, we do not take)
```

```rust
fn length(s: &String) -> usize {
    s.len()
}

fn main() {
    let name = String::from("Zoé");
    let n = length(&name);
    println!("{name} has {n} bytes");
}
```

```sortie
Zoé has 4 bytes
```

> **Note:** `é` takes two bytes in memory, hence the `4`.

In practice, for a text parameter, you write `&str` rather than `&String`: it accepts
both kinds of text.

```rust
fn shout(s: &str) -> String {
    s.to_uppercase()
}

fn main() {
    let a = String::from("hi");
    println!("{}", shout(&a));
    println!("{}", shout("hello"));
}
```

```sortie
HI
HELLO
```

## Borrowing to modify with &mut

A `&mut` reference lets you **modify** the borrowed value: you lend the book
with a pencil. The variable itself must be `mut`:

```rust
fn add_period(s: &mut String) {
    s.push('.');
}

fn main() {
    let mut sentence = String::from("End");
    add_period(&mut sentence);
    add_period(&mut sentence);
    println!("{sentence}");
}
```

```sortie
End..
```

## The borrowing rules

At any given moment, you can have **either**:

- as many `&` references as you want (several readers),
- **or** a single `&mut` reference (a single writer).

Never both at the same time. Think of a whiteboard: several people can
read it together, but when someone writes on it, nobody else must read or
write at the same time.

The part of the compiler that checks these rules is called the **borrow checker**.

```rust
let mut v = vec![1, 2, 3];
let a = &v;
let b = &v;
println!("{:?} {:?}", a, b); // several readers: allowed
let c = &mut v;
c.push(4); // a and b are no longer used: a writer is allowed
println!("{:?}", v);
```

```sortie
[1, 2, 3] [1, 2, 3]
[1, 2, 3, 4]
```

A borrow lasts until its last use. Here, `a` and `b` are no longer used after
the first `println!`: so the `&mut` borrow that follows is allowed.

## Lifetimes

A reference must never live longer than the value it borrows. You do not
keep a borrowed book after the library has closed for good. The
compiler checks this with **lifetimes**.

Most of the time, it guesses them on its own. Sometimes, you have to write them with an apostrophe,
like `'a`:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    println!("{}", longest("apple", "kiwi"));
}
```

```sortie
apple
```

Read `'a` like this: "the result lives as long as `x` and `y`". To get started, it
is enough to know that it exists.

A text written between quotes in the code has the `'static` lifetime: it exists
for the whole run of the program.

## Common mistakes

### Using a moved value

```rust norun
fn main() {
    let a = String::from("cat");
    let b = a;
    println!("{a} {b}");
}
```

```text
error[E0382]: borrow of moved value: `a`
 --> src/main.rs:4:16
  |
2 |     let a = String::from("cat");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
3 |     let b = a;
  |             - value moved here
4 |     println!("{a} {b}");
  |                ^ value borrowed here after move
```

After `let b = a;`, only `b` owns the text. To keep both, write
`let b = a.clone();`, or borrow: `let b = &a;`.

### Giving a value to a function, twice

```rust norun
fn show(s: String) {
    println!("{s}");
}

fn main() {
    let name = String::from("Zoé");
    show(name);
    show(name);
}
```

```text
error[E0382]: use of moved value: `name`
 --> src/main.rs:8:10
  |
6 |     let name = String::from("Zoé");
  |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
7 |     show(name);
  |          ---- value moved here
8 |     show(name);
  |          ^^^^ value used here after move
```

The function only needs to read: make it borrow the text.
`fn show(s: &str)`, then `show(&name);`.

### Modifying during a read

```rust norun
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("{first}");
}
```

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let first = &v[0];
  |                  - immutable borrow occurs here
4 |     v.push(4);
  |     ^^^^^^^^^ mutable borrow occurs here
5 |     println!("{first}");
  |                ----- immutable borrow later used here
```

This error protects you: `push` can move the vector in memory, and `first`
would then point to a freed area. Use `first` before the `push`, or copy the
value: `let first = v[0];`.

### Returning a reference without a lifetime

```rust norun
fn longest(x: &str, y: &str) -> &str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    println!("{}", longest("apple", "kiwi"));
}
```

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
```

The compiler does not know whether the result borrows `x` or `y`. Add a
lifetime, as in [Lifetimes](#lifetimes).

## Test yourself

**1.** After `let a = 5; let b = a;`, can you still use `a`? And with
`String::from("5")`?

> **Answer**
>
> With `5`, yes: an `i32` is copied. With a `String`, no: it is moved into
> `b`.

**2.** Which signature should you choose for a function that counts the vowels of a text
without modifying it?

> **Answer**
>
> A read borrow: `fn vowels(text: &str) -> usize`.

**3.** Fix this code so that it adds `!` to the text:

```rust norun
fn shout(s: String) {
    s.push('!');
}
```

> **Answer**
>
> Borrow for writing:
>
> ```rust
> fn shout(s: &mut String) {
>     s.push('!');
> }
>
> fn main() {
>     let mut word = String::from("Stop");
>     shout(&mut word);
>     println!("{word}");
> }
> ```
>
> ```sortie
> Stop!
> ```

## Summary

- Each value has a single owner, freed at the end of its block.
- `let b = a;` moves `a` into `b` (simple types are copied).
- `a.clone()` makes an independent copy.
- `&a` borrows for reading; several readers are allowed.
- `&mut a` borrows for writing; only one at a time.
- A reference never lives longer than its value: this is its lifetime.

Next: [lifetimes](durees-de-vie.md)
