# Structs, enums and traits

This page shows how to create your own types with `struct` and `enum`, add methods to them with `impl`, and use `Option` for a value that may be missing.

> **Goals:**
>
> - group data together in a structure (`struct`);
> - add methods and associated functions to it with `impl`;
> - describe one value among several choices with an enumeration (`enum`);
> - handle a missing value with `Option`, `match` and `if let`;
> - get common traits with `#[derive(...)]`.

## Structs

A **struct** (`struct`) groups several named values, called **fields**.
It is like a form: each box has a name and expects a certain type of answer.
The name of a struct starts with a capital letter.

```rust
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{} is {} years old", p.name, p.age);
}
```

```sortie
Alice is 30 years old
```

To change a field, the variable must be `mut`:

```rust
struct Counter {
    value: u32,
}

fn main() {
    let mut c = Counter { value: 0 };
    c.value += 5;
    println!("{}", c.value);
}
```

```sortie
5
```

## Methods with impl

An `impl` block (for *implementation*) adds functions to a struct. A
[method](../debuter/glossaire.md#methode) is a function attached to a type, that
you call with a dot: `r.area()`. Its first parameter, `self`, refers to
the object itself:

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // An associated function: it creates a Rectangle.
    fn square(side: f64) -> Rectangle {
        Rectangle { width: side, height: side }
    }

    // A method that reads the object.
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // A method that changes the object.
    fn grow(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let mut r = Rectangle { width: 3.0, height: 2.0 };
    println!("{}", r.area());
    r.grow(2.0);
    println!("{}", r.area());
    let c = Rectangle::square(4.0);
    println!("{}", c.area());
}
```

```sortie
6
24
16
```

| First parameter | Meaning |
|-----------------|---------|
| `&self` | the method reads the object |
| `&mut self` | the method changes the object |
| none | associated function, called with `Type::name()` |

### The new constructor

By habit, the associated function that creates an object is called `new`. In an
`impl` block, `Self` is a shortcut for the name of the type:

```rust
struct Player {
    name: String,
    points: u32,
}

impl Player {
    fn new(name: &str) -> Self {
        Self { name: name.to_string(), points: 0 }
    }

    fn score(&mut self, n: u32) {
        self.points += n;
    }
}

fn main() {
    let mut p = Player::new("Sam");
    p.score(3);
    p.score(2);
    println!("{}: {} points", p.name, p.points);
}
```

```sortie
Sam: 5 points
```

`String::new()` and `Vec::new()`, which you already know, are associated functions
of the same kind.

## Enums

An **enumeration** (`enum`) is a type that holds **one** value from a list of
choices, called **variants**. A traffic light is green, yellow **or** red, never
two at once. You write a variant with `::`: `Light::Yellow`.

```rust
enum Light {
    Green,
    Yellow,
    Red,
}

fn main() {
    let l = Light::Yellow;
    let action = match l {
        Light::Green => "go",
        Light::Yellow => "slow down",
        Light::Red => "stop",
    };
    println!("{action}");
}
```

```sortie
slow down
```

### Variants with data

Each variant can carry its own data: a number for the circle (its
radius), two named fields for the rectangle, nothing for the point.

```rust
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Point,
}
```

## match on an enum

`match` opens each variant and gets its data:

```rust
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Point,
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Point => 0.0,
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle(1.0),
        Shape::Rectangle { width: 2.0, height: 5.0 },
        Shape::Point,
    ];
    for s in &shapes {
        println!("{}", area(s));
    }
}
```

```sortie
3.14
10
0
```

> **Remember:** if you forget a variant in a `match`, the program does not
> compile. Adding a variant shows you every place you need to update.

## The Option type

Many languages have a [null](../debuter/glossaire.md#null) value, which means
"nothing", and which causes errors when you forget about it. Rust has no `null`. A
value that may be missing has the type `Option<T>`: a box that contains a value of
type `T`, or that is empty.

- `Some(value)`: there is a value;
- `None`: there is nothing.

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    match divide(10, 2) {
        Some(n) => println!("Result: {n}"),
        None => println!("Division impossible"),
    }
    if let Some(n) = divide(9, 3) {
        println!("9 / 3 = {n}");
    }
    println!("{:?}", divide(1, 0));
}
```

```sortie
Result: 5
9 / 3 = 3
None
```

> **Tip:** `if let` is a shortcut for a `match` that only cares about one
> case.

### Useful Option methods

For simple cases, methods save you from writing a `match`:

```rust
let present: Option<i32> = Some(4);
let missing: Option<i32> = None;
println!("{} {}", present.is_some(), missing.is_none());
println!("{}", missing.unwrap_or(0));
println!("{:?}", present.map(|n| n * 10));
```

```sortie
true true
0
Some(40)
```

| Method | Effect |
|--------|--------|
| `is_some()` / `is_none()` | is there a value? |
| `unwrap_or(v)` | the value, or `v` if the box is empty |
| `map(f)` | transforms the value if it exists |
| `unwrap()` | the value, or stops the program if the box is empty |

## Traits

A **trait** describes a behavior: a list of methods that a type promises
to have. It is like a permit: a type that has the `Clone` trait "is allowed" to be
copied with `.clone()`. The simplest way is to get the common traits with
`#[derive(...)]`, a line placed right before the struct:

| Trait | What it allows |
|-------|----------------|
| `Debug` | display with `{:?}` |
| `Clone` | copy with `.clone()` |
| `PartialEq` | compare with `==` |

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a.clone();
    println!("{:?}", a);
    println!("{}", a == b);
}
```

```sortie
Point { x: 1, y: 2 }
true
```

Writing your own traits is explained in [Generics and traits](generiques.md).

## Common mistakes

### Forgetting a field

```rust norun
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p = Person { name: String::from("Alice") };
}
```

```text
error[E0063]: missing field `age` in initializer of `Person`
 --> src/main.rs:7:13
  |
7 |     let p = Person { name: String::from("Alice") };
  |             ^^^^^^ missing `age`
```

Every field must get a value: `Person { name: …, age: 30 }`.

### Displaying a struct without Debug

```rust norun
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);
}
```

```text
error[E0277]: `Point` doesn't implement `Debug`
 --> src/main.rs:8:22
  |
8 |     println!("{:?}", p);
  |               ----   ^ `Point` cannot be formatted using `{:?}` because it doesn't implement `Debug`
  |               |
  |               required by this formatting parameter
  |
  = note: add `#[derive(Debug)]` to `Point` or manually `impl Debug for Point`
```

Add `#[derive(Debug)]` right before `struct Point`.

### Calling a &mut self method on a non-mut variable

```rust norun
struct Counter {
    value: u32,
}

impl Counter {
    fn increase(&mut self) {
        self.value += 1;
    }
}

fn main() {
    let c = Counter { value: 0 };
    c.increase();
}
```

```text
error[E0596]: cannot borrow `c` as mutable, as it is not declared as mutable
  --> src/main.rs:13:5
   |
13 |     c.increase();
   |     ^ cannot borrow as mutable
```

The method changes the object: the variable must be `mut`. `let mut c = …`.

### Forgetting a variant in a match

```rust norun
enum Light {
    Green,
    Yellow,
    Red,
}

fn main() {
    let l = Light::Red;
    match l {
        Light::Green => println!("go"),
        Light::Yellow => println!("slow down"),
    }
}
```

```text
error[E0004]: non-exhaustive patterns: `Light::Red` not covered
  --> src/main.rs:9:11
   |
 9 |     match l {
   |           ^ pattern `Light::Red` not covered
```

Add the forgotten variant: `Light::Red => println!("stop"),`.

## Test yourself

**1.** What is the difference between `fn area(&self)` and `fn square(side: f64)` in an
`impl Rectangle` block?

> **Answer**
>
> `area` is a method: you call it on an object, `r.area()`. `square` is an
> associated function, without an object: you call it with `Rectangle::square(4.0)`.

**2.** Write an enumeration `Coin` with the variants `Heads` and `Tails`.

> **Answer**
>
> ```rust
> #[derive(Debug)]
> enum Coin {
>     Heads,
>     Tails,
> }
>
> fn main() {
>     let c = Coin::Tails;
>     println!("{:?}", c);
> }
> ```
>
> ```sortie
> Tails
> ```

**3.** What is `None.unwrap_or(7)`? And `Some(3).unwrap_or(7)`?

> **Answer**
>
> `7` then `3`: `unwrap_or` gives the value if it exists, otherwise the fallback
> value.

## Summary

- `struct` groups named fields; `impl` adds methods to it.
- `&self` reads the object, `&mut self` changes it; without `self`, it is an associated
  function, like `Type::new()`.
- `enum` holds one value from a list of variants.
- `match` handles each variant and makes sure you forget none.
- `Option` is `Some(value)` or `None`: Rust has no `null`.
- `#[derive(Debug, Clone, PartialEq)]` gives common traits to a type.

Compare with [types in Ruby](../ruby/types.md) and [in Rythme](../rythme/types.md).

Next: [pattern matching](pattern-matching.md)
