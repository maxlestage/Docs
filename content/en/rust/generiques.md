# Generics and traits

This page shows how to write a single function for several types thanks to generics, and how to describe a shared behavior with traits.

> **Goals:**
>
> - write a generic function and a generic struct with `<T>`;
> - limit a generic type with a bound (`T: Display`);
> - define a trait and implement it for several types;
> - store objects of different types with `Box<dyn Trait>`;
> - use the common traits: `Debug`, `Clone`, `Display`, `From`.

## The problem: duplicated code

Imagine two functions that do the same thing, for two different types:

```rust
fn largest_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn largest_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", largest_i32(3, 8));
    println!("{}", largest_f64(2.5, 1.5));
}
```

```sortie
8
2.5
```

The code is identical, only the type changes. **Generics** avoid this repetition.

## A generic function

A generic function is like a cake mold: the shape is always the same, but you choose
the batter each time. You replace the type with a letter, often `T` (for *type*),
written between `<` and `>` after the name:

```rust
fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", largest(3, 8));
    println!("{}", largest(2.5, 1.5));
    println!("{}", largest("pear", "kiwi"));
}
```

```sortie
8
2.5
pear
```

Read `<T: PartialOrd>` like this: "for any type `T` that can be compared".

> **Remember:** `: PartialOrd` is a **bound**. Without it, Rust refuses `a > b`,
> because it does not know whether `T` can be compared.

## Several bounds

A bound is a **trait**: an ability the type must have. A job offer works the same way:
"any candidate who can drive **and** speaks English". To ask for several traits, join
them with `+`. Here, `Display` allows printing with `{}`:

```rust
use std::fmt::Display;

fn show_twice<T: Display + Clone>(x: T) {
    let copy = x.clone();
    println!("{x} and {copy}");
}

fn main() {
    show_twice(7);
    show_twice("ok");
}
```

```sortie
7 and 7
ok and ok
```

## impl Trait as a parameter

There is a shorter way to write it: `impl Trait` in place of the type. It reads
"any type that has this trait":

```rust
use std::fmt::Display;

fn frame(x: impl Display) -> String {
    format!("[ {x} ]")
}

fn main() {
    println!("{}", frame(42));
    println!("{}", frame("hello"));
    println!("{}", frame(3.5));
}
```

```sortie
[ 42 ]
[ hello ]
[ 3.5 ]
```

> **Tip:** when you are starting out, `impl Trait` is often the most readable way to
> write it.

## A generic struct

A struct can also have a generic type. `Vec<T>` and `Option<T>` are built this way.
Here is a box that holds a value of any type:

```rust
#[derive(Debug)]
struct Container<T> {
    content: T,
}

impl<T> Container<T> {
    fn open(self) -> T {
        self.content
    }
}

fn main() {
    let a = Container { content: 42 };
    let b = Container { content: "secret" };
    println!("{:?}", a);
    println!("{}", b.open());
}
```

```sortie
Container { content: 42 }
secret
```

`impl<T> Container<T>` reads: "for any type `T`, here are the methods of
`Container<T>`".

## Defining your own trait

A trait describes a behavior: a list of methods that a type promises to have. It is a
contract: "every animal can say its name and make its sound". You write it with
`trait`, then each type signs it with `impl Trait for Type`. A method can also have a
**default** behavior:

```rust
trait Animal {
    fn name(&self) -> String;
    fn sound(&self) -> String;

    // Default method: types do not need to write it.
    fn introduce(&self) -> String {
        format!("{} goes {}", self.name(), self.sound())
    }
}

struct Dog;

impl Animal for Dog {
    fn name(&self) -> String {
        String::from("The dog")
    }
    fn sound(&self) -> String {
        String::from("woof")
    }
}

fn main() {
    let rex = Dog;
    println!("{}", rex.introduce());
}
```

```sortie
The dog goes woof
```

`struct Dog;` is a struct with no field: it holds nothing, but it can have methods.

## Implementing a trait for two structs

Each struct writes its own version. A function that receives `&impl Animal` accepts
both:

```rust
trait Animal {
    fn name(&self) -> String;
    fn sound(&self) -> String;
    fn introduce(&self) -> String {
        format!("{} goes {}", self.name(), self.sound())
    }
}

struct Dog;
struct Cat {
    first_name: String,
}

impl Animal for Dog {
    fn name(&self) -> String {
        String::from("The dog")
    }
    fn sound(&self) -> String {
        String::from("woof")
    }
}

impl Animal for Cat {
    fn name(&self) -> String {
        self.first_name.clone()
    }
    fn sound(&self) -> String {
        String::from("meow")
    }
    // The cat replaces the default method.
    fn introduce(&self) -> String {
        format!("{} says {} (yawning)", self.name(), self.sound())
    }
}

fn make_speak(a: &impl Animal) {
    println!("{}", a.introduce());
}

fn main() {
    make_speak(&Dog);
    make_speak(&Cat { first_name: String::from("Kitty") });
}
```

```sortie
The dog goes woof
Kitty says meow (yawning)
```

## A list of different objects

A `Vec` holds only one type. To store a dog **and** a cat in the same list, you use
`Box<dyn Animal>`: "a box that holds an animal, no matter which one".

```rust
trait Animal {
    fn sound(&self) -> String;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn sound(&self) -> String {
        String::from("woof")
    }
}

impl Animal for Cat {
    fn sound(&self) -> String {
        String::from("meow")
    }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat), Box::new(Dog)];
    for a in &animals {
        println!("{}", a.sound());
    }
}
```

```sortie
woof
meow
woof
```

> **Note:** `Box::new(x)` puts `x` in a box. The word `dyn` means that the real type
> will only be known while the program runs.

## Common traits from the library

Rust provides ready-made traits. Many of them come for free with `#[derive(...)]`:

| Trait | What it allows | derive? |
|-------|----------------|---------|
| `Debug` | print with `{:?}` | yes |
| `Clone` | copy with `.clone()` | yes |
| `PartialEq` | compare with `==` | yes |
| `Default` | create a default value | yes |
| `Display` | print with `{}` | no, write it yourself |
| `From` | convert from another type | no, write it yourself |

### Debug, Clone, PartialEq and Default

```rust
#[derive(Debug, Clone, PartialEq, Default)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a.clone();
    let zero = Point::default();
    println!("{:?}", a);
    println!("{}", a == b);
    println!("{:?}", zero);
}
```

```sortie
Point { x: 1, y: 2 }
true
Point { x: 0, y: 0 }
```

### Display: custom printing

To print a type with `{}`, write the `Display` trait yourself. Copy this template,
only the `write!` line changes:

```rust
use std::fmt;

struct Temperature {
    degrees: f64,
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} °C", self.degrees)
    }
}

fn main() {
    let t = Temperature { degrees: 21.456 };
    println!("It is {t}");
}
```

```sortie
It is 21.5 °C
```

### From: converting one type into another

`From` explains how to build your type from another one. You then get `.into()` as a
bonus:

```rust
#[derive(Debug)]
struct Euros(f64);

impl From<f64> for Euros {
    fn from(amount: f64) -> Euros {
        Euros(amount)
    }
}

fn main() {
    let a = Euros::from(12.5);
    let price: f64 = 3.0;
    let b: Euros = price.into();
    println!("{:?} {:?}", a, b);
}
```

```sortie
Euros(12.5) Euros(3.0)
```

> **Tip:** you have been using `From` since the beginning without knowing it:
> `String::from("text")`.

## Common mistakes

### Forgetting the bound

```rust norun
fn largest<T>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", largest(3, 8));
}
```

```text
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:2:10
  |
2 |     if a > b { a } else { b }
  |        - ^ - T
  |        |
  |        T
help: consider restricting type parameter `T` with trait `PartialOrd`
```

Rust does not know whether any `T` can be compared. Write `<T: PartialOrd>`.

### Forgetting a method of the trait

```rust norun
trait Animal {
    fn name(&self) -> String;
    fn sound(&self) -> String;
}

struct Dog;

impl Animal for Dog {
    fn name(&self) -> String {
        String::from("Rex")
    }
}

fn main() {}
```

```text
error[E0046]: not all trait items implemented, missing: `sound`
 --> src/main.rs:8:1
  |
3 |     fn sound(&self) -> String;
  |     -------------------------- `sound` from trait
...
8 | impl Animal for Dog {
  | ^^^^^^^^^^^^^^^^^^^ missing `sound` in implementation
```

Signing the contract forces you to write every method that has no default version.
Add `fn sound(&self) -> String { … }`.

### Putting two types in the same Vec

```rust norun
struct Dog;
struct Cat;

fn main() {
    let animals = vec![Dog, Cat];
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let animals = vec![Dog, Cat];
  |                             ^^^ expected `Dog`, found `Cat`
```

Give them a shared trait, then store boxes: `Vec<Box<dyn Animal>>` (see
[A list of different objects](#a-list-of-different-objects)).

### Printing with {} without Display

```rust norun
struct Euros(f64);

fn main() {
    let price = Euros(12.5);
    println!("{price}");
}
```

```text
error[E0277]: `Euros` doesn't implement `std::fmt::Display`
 --> src/main.rs:5:15
  |
5 |     println!("{price}");
  |               ^^^^^^^ `Euros` cannot be formatted with the default formatter
  |
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

Write `impl fmt::Display for Euros` (see
[Display](#display-custom-printing)), or add `#[derive(Debug)]` and print with `{:?}`.

## Test yourself

**1.** What does `fn f<T: Clone>(x: T)` mean?

> **Answer**
>
> `f` accepts a value of any type `T`, as long as this type can be copied with
> `.clone()`.

**2.** Write a trait `Area` with a method `area(&self) -> f64`, and implement it for a
struct `Square { side: f64 }`.

> **Answer**
>
> ```rust
> trait Area {
>     fn area(&self) -> f64;
> }
>
> struct Square {
>     side: f64,
> }
>
> impl Area for Square {
>     fn area(&self) -> f64 {
>         self.side * self.side
>     }
> }
>
> fn main() {
>     println!("{}", Square { side: 3.0 }.area());
> }
> ```
>
> ```sortie
> 9
> ```

**3.** Why write `Box<dyn Animal>` rather than `Dog` in a `Vec`?

> **Answer**
>
> To store animals of different types in the same list. `dyn Animal` means "any type
> that has the `Animal` trait".

## Summary

- `<T>` makes a function or a struct generic: it works for several types.
- A bound like `T: Display` requires an ability from the type.
- `impl Trait` as a parameter is a short and readable way to write it.
- A trait lists methods; it can provide default methods.
- `Vec<Box<dyn Trait>>` stores objects of different types.
- `#[derive(Debug, Clone, PartialEq, Default)]` writes traits for you.

The basic types are in [Structs, enums and traits](types.md).

Next: [trait objects: dyn Trait](traits-objets.md)
