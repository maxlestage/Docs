# Trait objects: dyn Trait

This chapter explains how to handle values of different types that share the same trait, with `dyn Trait`, and when to prefer generics.

> **Goals:**
>
> - understand the difference between a generic (static) and `dyn Trait` (dynamic);
> - use `&dyn Trait` and `Box<dyn Trait>`;
> - store values of different types in a `Vec<Box<dyn Trait>>`;
> - choose between a generic and `dyn`;
> - recognize a trait that cannot become `dyn`;
> - return a closure with `Box<dyn Fn>`.

This chapter follows [Generics and traits](generiques.md). Read it again if the word
[trait](../debuter/glossaire.md#trait) feels fuzzy to you.

## The starting point: shapes and a trait

Let's take three shapes: a square, a rectangle and a circle. They are all different,
but they all know two things: how to give their **name** and their **area** (their
surface). This common point becomes a trait, `Shape`:

```rust
use std::f64::consts::PI;

trait Shape {
    fn name(&self) -> String;
    fn area(&self) -> f64;
}

struct Square {
    side: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Square {
    fn name(&self) -> String {
        String::from("square")
    }
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Rectangle {
    fn name(&self) -> String {
        String::from("rectangle")
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn name(&self) -> String {
        String::from("circle")
    }
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

fn main() {
    let s = Square { side: 2.0 };
    let r = Rectangle { width: 2.0, height: 3.0 };
    let c = Circle { radius: 1.0 };
    println!("{}: {:.2}", s.name(), s.area());
    println!("{}: {:.2}", r.name(), r.area());
    println!("{}: {:.2}", c.name(), c.area());
}
```

```sortie
square: 4.00
rectangle: 6.00
circle: 3.14
```

`{:.2}` displays two digits after the decimal point. To keep the examples short, the
rest of the page only uses the square and the circle.

## First way: the generic (static)

You already know `impl Shape` as a parameter. It means "any type that has the
`Shape` trait":

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Square(f64);
struct Circle(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.0 * self.0 * self.0
    }
}

fn describe(s: &impl Shape) {
    println!("area: {}", s.area());
}

fn main() {
    describe(&Square(2.0));
    describe(&Circle(1.0));
}
```

```sortie
area: 4
area: 3
```

`struct Square(f64);` is a structure with a single field that has no name: you read it
with `self.0`. Here, the circle uses 3 for π, to get round numbers.

What does Rust do behind the scenes? During compilation, it **copies** the function for
each type used. It's like a printer who prepares one stamp per shape:

```text
your code:       describe(s: &impl Shape)

what Rust makes of it, at compile time:
                 describe_for_Square(s: &Square)  → calls Square::area
                 describe_for_Circle(s: &Circle)  → calls Circle::area
```

This is called **static**: everything is decided before the program runs.

> **Remember:** a generic produces one copy of the function per type. It's very fast,
> but each copy knows only **one** type.

## Second way: dyn Trait (dynamic)

Write `&dyn Shape` instead of `&impl Shape`. The program displays the same thing:

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Square(f64);
struct Circle(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.0 * self.0 * self.0
    }
}

fn describe(s: &dyn Shape) {
    println!("area: {}", s.area());
}

fn main() {
    describe(&Square(2.0));
    describe(&Circle(1.0));
}
```

```sortie
area: 4
area: 3
```

But this time, there is **only one** `describe` function. It doesn't know whether it
receives a square or a circle. The word `dyn` (for *dynamic*) means: "the real type
will be discovered while the program runs".

A `&dyn Shape` value is called a **trait object**.

## How it works: the method table

How does `describe` find the right `area` method? A `&dyn Shape` is not a simple
address. It is **two** pieces of information stuck together:

1. where the value is (the square itself);
2. where the **method table** of its type is, called the *vtable*.

```text
  &dyn Shape
  ┌────────────┬────────────┐
  │  data    ──┼──► Square(2.0)
  │  table   ──┼──► vtable of Square
  └────────────┴────────────┘          ┌──────────────────────┐
                                       │ area → Square::area  │
                                       └──────────────────────┘
```

Think of a suitcase with an instruction label attached. Whoever receives it doesn't
open the suitcase to guess what's inside: they read the label, which says which method
to call.

A call `s.area()` happens in two steps:

| Step | What the program does |
|------|-----------------------|
| 1 | it reads the table attached to `s` |
| 2 | it finds there the address of the right `area`, then calls it |

You can check that the trait object is twice as big as a simple reference:

```rust
use std::mem::size_of;

trait Shape {
    fn area(&self) -> f64;
}

struct Square(f64);

fn main() {
    println!("{}", size_of::<&dyn Shape>() == 2 * size_of::<&Square>());
}
```

```sortie
true
```

> **Remember:** `dyn Trait` = a single function, and a small detour through the table
> at each call. It's a tiny bit slower, but much more flexible.

## Box<dyn Trait>: owning a shape

`&dyn Shape` **borrows** a shape that lives somewhere else. To **own** a shape whose
exact type you don't know, you put it in a `Box`:

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Square(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

fn main() {
    let s: Box<dyn Shape> = Box::new(Square(3.0));
    println!("{}", s.area());
}
```

```sortie
9
```

`Box::new(x)` puts `x` in a box, stored in an area of memory called the heap. The
`Box` itself always has the same size, whatever shape it contains. That's why Rust
accepts it. Box is explained in detail in [Box, Rc and RefCell](pointeurs.md).

Why not just `let s: dyn Shape = …`? Because a square and a circle don't have the same
size. Rust must know the size of each variable before running the program. A box, on
the other hand, always has the same size.

Try it: replace `Square(3.0)` with another shape that has the `Shape` trait. The type
of `s` doesn't change: it's still `Box<dyn Shape>`.

## Vec<Box<dyn Trait>>: a list of mixed shapes

Here, at last, is the answer to the starting problem: a list that contains squares
**and** circles.

```rust
trait Shape {
    fn name(&self) -> String;
    fn area(&self) -> f64;
}

struct Square(f64);
struct Circle(f64);

impl Shape for Square {
    fn name(&self) -> String {
        String::from("square")
    }
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

impl Shape for Circle {
    fn name(&self) -> String {
        String::from("circle")
    }
    fn area(&self) -> f64 {
        3.0 * self.0 * self.0
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Square(2.0)),
        Box::new(Circle(1.0)),
        Box::new(Square(1.0)),
    ];

    let mut total = 0.0;
    for s in &shapes {
        println!("{}: {}", s.name(), s.area());
        total += s.area();
    }
    println!("total: {total}");
}
```

```sortie
square: 4
circle: 3
square: 1
total: 8
```

Let's follow the loop step by step:

| Step | s | real type | area | total |
|------|---|-----------|------|-------|
| start | – | – | – | 0 |
| 1 | `shapes[0]` | `Square(2.0)` | 4 | 4 |
| 2 | `shapes[1]` | `Circle(1.0)` | 3 | 7 |
| 3 | `shapes[2]` | `Square(1.0)` | 1 | 8 |

In memory, the `Vec` contains three boxes of the same size. Each one points to its
shape and its table:

```text
shapes: [  box   |  box   |  box   ]
            │        │        │
            ▼        ▼        ▼
          Square   Circle   Square
          (table   (table   (table
          Square)  Circle)  Square)
```

Try it: add `Box::new(Circle(2.0))` to the list. A line `circle: 12` appears and the
total goes up to 20.

### Building a shape from a choice

A function can return a `Box<dyn Shape>` whose type depends on a value known only while
the program runs:

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Square(f64);
struct Circle(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.0 * self.0 * self.0
    }
}

fn make(letter: char, size: f64) -> Box<dyn Shape> {
    if letter == 's' {
        Box::new(Square(size))
    } else {
        Box::new(Circle(size))
    }
}

fn main() {
    let order = "s2 c1 s3";
    let mut shapes = Vec::new();
    for word in order.split(' ') {
        let letter = word.chars().next().unwrap();
        let size: f64 = word[1..].parse().unwrap();
        shapes.push(make(letter, size));
    }
    for s in &shapes {
        println!("{}", s.area());
    }
}
```

```sortie
4
3
9
```

With a generic, this would be impossible: both branches of the `if` must have the same
type. Here, that common type is `Box<dyn Shape>`.

## Generic or dyn: which one to choose?

| Question | Generic (`impl Trait`, `<T: Trait>`) | `dyn Trait` |
|----------|--------------------------------------|-------------|
| When is the type chosen? | at compile time | while the program runs |
| Several types in the same `Vec`? | no | yes, with `Box` |
| Call speed | the fastest | a small detour through the table |
| Program size | one copy per type | a single function |
| All possible traits? | yes | no, see below |

A simple rule to start with:

- **one type at a time** (a function that receives "a shape"): the generic;
- **several mixed types** (a list of shapes, a choice made while the program runs):
  `dyn`.

> **Remember:** start with `impl Trait`. Switch to `dyn Trait` when you need to **mix**
> different types.

## Returning a closure with `Box<dyn Fn>`

A [closure](fonctions.md#closures) is a small function without a name. Each closure
has its own type, which nobody can write. For a function to return **one or the
other** of two closures, you use the `Fn` trait in its `dyn` version:

```rust
fn setting(n: i32, double: bool) -> Box<dyn Fn(i32) -> i32> {
    if double {
        Box::new(move |x| x * n)
    } else {
        Box::new(move |x| x + n)
    }
}

fn main() {
    let f = setting(3, true);
    let g = setting(3, false);
    println!("{}", f(5));
    println!("{}", g(5));
}
```

```sortie
15
8
```

Read `Box<dyn Fn(i32) -> i32>` like this: "a box that contains a function, any
function, that takes an `i32` and returns an `i32`". `move` makes the closure take `n`
with it.

You can also store different closures in a `Vec`:

```rust
fn main() {
    let steps: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 10),
        Box::new(|x| x - 3),
    ];
    let mut value = 2;
    for step in &steps {
        value = step(value);
        println!("{value}");
    }
}
```

```sortie
3
30
27
```

| Step | calculation | value |
|------|-------------|-------|
| start | – | 2 |
| 1 | 2 + 1 | 3 |
| 2 | 3 × 10 | 30 |
| 3 | 30 − 3 | 27 |

Closures have their own full chapter: [Closures in detail](closures.md).

## When a trait cannot become dyn

Some traits cannot be used as a trait object. Rust then says that the trait is not
*dyn compatible* (it used to be called *object safe*).

The reason is simple. A trait object only has its **method table**. If a method needs
to know the real type in order to exist, it cannot go into the table. Here are the two
most common cases:

| The trait contains… | Why it's a problem |
|---------------------|--------------------|
| a function without `self` that returns `Self` (`fn new() -> Self`) | without a value, there is no table to read; and `Self` has no known size |
| a generic method (`fn compare<T>(&self, x: T)`) | the table would need a slot for **every** possible type `T` |

Here is the real message:

```rust norun
trait Shape {
    fn new() -> Self;
    fn area(&self) -> f64;
}

fn show(s: &dyn Shape) {
    println!("{}", s.area());
}

fn main() {}
```

```text
error[E0038]: the trait `Shape` is not dyn compatible
 --> src/main.rs:6:13
  |
6 | fn show(s: &dyn Shape) {
  |             ^^^^^^^^^ `Shape` is not dyn compatible
  |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
 --> src/main.rs:2:8
  |
1 | trait Shape {
  |       ----- this trait is not dyn compatible...
2 |     fn new() -> Self;
  |        ^^^ ...because associated function `new` has no `self` parameter
help: consider turning `new` into a method by giving it a `&self` argument
  |
2 |     fn new(&self) -> Self;
  |            +++++
help: alternatively, consider constraining `new` so it does not apply to trait objects
  |
2 |     fn new() -> Self where Self: Sized;
  |                      +++++++++++++++++
```

The message does say "it needs to allow building a vtable": it must be possible to
build the table. The second help is the most practical: `where Self: Sized` removes
that function from the table. The rest of the trait becomes usable with `dyn`:

```rust
trait Shape {
    fn new() -> Self
    where
        Self: Sized;
    fn area(&self) -> f64;
}

struct Square(f64);

impl Shape for Square {
    fn new() -> Self {
        Square(1.0)
    }
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

fn show(s: &dyn Shape) {
    println!("{}", s.area());
}

fn main() {
    let sq = Square::new();
    show(&sq);
}
```

```sortie
1
```

> **Note:** the `Clone` trait is in this situation: `clone` returns `Self`. So a trait
> that requires `Clone` (`trait Shape: Clone`) cannot become `dyn Shape`.

## Common mistakes

### Forgetting the dyn keyword

```rust norun
trait Shape {
    fn area(&self) -> f64;
}

struct Square(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

fn main() {
    let s: Box<Shape> = Box::new(Square(2.0));
}
```

```text
error[E0782]: expected a type, found a trait
  --> src/main.rs:14:16
   |
14 |     let s: Box<Shape> = Box::new(Square(2.0));
   |                ^^^^^
   |
help: you can add the `dyn` keyword if you want a trait object
   |
14 |     let s: Box<dyn Shape> = Box::new(Square(2.0));
   |                +++
```

A trait is not a type. Write `Box<dyn Shape>`.

### A Vec<dyn Trait> without Box

```rust norun
trait Shape {
    fn area(&self) -> f64;
}

fn main() {
    let shapes: Vec<dyn Shape> = Vec::new();
}
```

```text
error[E0277]: the size for values of type `dyn Shape` cannot be known at compilation time
 --> src/main.rs:6:17
  |
6 |     let shapes: Vec<dyn Shape> = Vec::new();
  |                 ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `dyn Shape`
```

Shapes don't all have the same size. Put each one in a box: `Vec<Box<dyn Shape>>`.

### A type that doesn't have the trait

```rust norun
trait Shape {
    fn area(&self) -> f64;
}

struct Square(f64);
struct Circle(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Square(2.0)), Box::new(Circle(1.0))];
    println!("{}", shapes.len());
}
```

```text
error[E0277]: the trait bound `Circle: Shape` is not satisfied
  --> src/main.rs:15:67
   |
15 |     let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Square(2.0)), Box::new(Circle(1.0))];
   |                                                                   ^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
   |
help: the trait `Shape` is not implemented for `Circle`
```

Only the types that have signed the contract can get in. Add
`impl Shape for Circle { … }`.

### Returning two closures with impl Fn

```rust norun
fn setting(n: i32, double: bool) -> impl Fn(i32) -> i32 {
    if double {
        move |x| x * n
    } else {
        move |x| x + n
    }
}

fn main() {
    println!("{}", setting(3, true)(5));
}
```

```text
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:5:9
  |
2 | /     if double {
3 | |         move |x| x * n
  | |         --------------
  | |         |
  | |         the expected closure
  | |         expected because of this
4 | |     } else {
5 | |         move |x| x + n
  | |         ^^^^^^^^^^^^^^ expected closure, found a different closure
6 | |     }
  | |_____- `if` and `else` have incompatible types
  |
  = note: expected closure `{closure@src/main.rs:3:9: 3:17}`
             found closure `{closure@src/main.rs:5:9: 5:17}`
  = note: no two closures, even if identical, have the same type
  = help: consider boxing your closure and/or using it as a trait object
```

`impl Fn` stands for **a single** hidden type. Two closures have two different types.
Return `Box<dyn Fn(i32) -> i32>` and put each closure in `Box::new(…)`, as in
[Returning a closure with Box<dyn Fn>](#returning-a-closure-with-box-dyn-fn).

## Test yourself

**1.** What is the difference between `fn f(x: &impl Shape)` and `fn f(x: &dyn Shape)`?

> **Answer**
>
> With `impl Shape`, Rust builds a copy of `f` for each type, at compile time
> (static). With `dyn Shape`, there is only one function; it finds the right method
> while the program runs, thanks to the method table (dynamic).

**2.** Why do we write `Vec<Box<dyn Shape>>` and not `Vec<dyn Shape>`?

> **Answer**
>
> A square and a circle don't have the same size. A `Vec` requires elements of known
> size. A `Box` always has the same size, whatever shape it contains.

**3.** Write a trait `Greet` with `fn hello(&self) -> String`, two structures `Fr` and
`En`, and a loop over a `Vec<Box<dyn Greet>>` that displays `bonjour` then `hello`.

> **Answer**
>
> ```rust
> trait Greet {
>     fn hello(&self) -> String;
> }
>
> struct Fr;
> struct En;
>
> impl Greet for Fr {
>     fn hello(&self) -> String {
>         String::from("bonjour")
>     }
> }
>
> impl Greet for En {
>     fn hello(&self) -> String {
>         String::from("hello")
>     }
> }
>
> fn main() {
>     let list: Vec<Box<dyn Greet>> = vec![Box::new(Fr), Box::new(En)];
>     for g in &list {
>         println!("{}", g.hello());
>     }
> }
> ```
>
> ```sortie
> bonjour
> hello
> ```

## Summary

- A generic (`impl Trait`, `<T: Trait>`) is static: one copy per type, chosen at
  compile time.
- `dyn Trait` is dynamic: a single function, which reads the method table (vtable)
  while the program runs.
- `&dyn Trait` borrows, `Box<dyn Trait>` owns; `Vec<Box<dyn Trait>>` mixes types.
- Start with generics; switch to `dyn` to mix types.
- A trait with a generic method or a `-> Self` without `self` is not dyn compatible
  (except with `where Self: Sized`).
- `Box<dyn Fn(…) -> …>` returns or stores different closures.

Next: [Standard library traits](traits-standard.md)
