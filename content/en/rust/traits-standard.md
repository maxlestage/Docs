# Standard library traits

The Rust standard library defines a handful of traits that you find everywhere: this chapter shows what each one is for, and when to get it with `derive` or write it by hand.

> **Goals:**
>
> - display your types with `Debug` and `Display`;
> - copy, compare and sort your structures with `Clone`, `Copy`, `PartialEq` and `Ord`;
> - use a structure as a `HashMap` key with `Hash`;
> - convert with `From` and `Into`, and create a default value with `Default`;
> - write an iterator and overload the `+` operator.

A [trait](../debuter/glossaire.md#trait) is a list of abilities that a type promises
to have, like a licence: a driving licence says "this person can drive". The
`Display` trait says "this type can display itself". You saw how to define your own
traits in [Generics and traits](generiques.md); here, we look at the ones that Rust
provides.

## derive: traits for free

Many standard traits can be **derived**: you write `#[derive(…)]` above the type, and
the compiler writes the code for you.

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

`derive` only works if **all the fields** already have that trait. An `i32` can clone
itself and compare itself: so `Point` can too.

## Debug and Display: displaying yourself

There are two ways to display a value:

| Trait | Format | For whom? | How to get it |
|-------|--------|-----------|---------------|
| `Debug` | `{:?}` or `{:#?}` | the programmer | `#[derive(Debug)]` |
| `Display` | `{}` | the user | write it by hand |

`Display` cannot be derived: Rust cannot guess how you want to present your type. You
write it with `impl` and an `fmt` method:

```rust
use std::fmt;

#[derive(Debug)]
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
    println!("{}", t);
    println!("{:?}", t);
    let text = t.to_string();
    println!("{}", text.len());
}
```

```sortie
21.5 °C
Temperature { degrees: 21.456 }
8
```

`write!` works like `format!`, but writes into `f`. As a bonus, every type that has
`Display` gets the `to_string()` method.

> **Tip:** `{:#?}` displays a structure on several lines, indented. It is handy for big
> structures.

## Clone and Copy: duplicating

- `Clone` gives the `.clone()` method: an **explicit** copy, possibly expensive
  (a `String` copies all its text).
- `Copy` makes the copy **automatic**: `let b = a;` copies instead of
  [moving](possession.md#the-move). Reserved for small, simple types.

```rust
#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let red = Color { r: 255, g: 0, b: 0 };
    let other = red; // copied, not moved
    println!("{:?}", red);
    println!("{:?}", other);
}
```

```sortie
Color { r: 255, g: 0, b: 0 }
Color { r: 255, g: 0, b: 0 }
```

`Copy` also requires `Clone`. A structure that contains a `String` or a `Vec` cannot
be `Copy`: it will only have `Clone`.

## PartialEq and Eq: comparing with ==

`PartialEq` gives the `==` and `!=` operators. The `derive` compares **all the fields**
one by one.

`Eq` is an extra promise: "every value is equal to itself". That is true for integers
and texts, but not for floats: `f64::NAN == f64::NAN` is false. A type with an `f64`
field can therefore only have `PartialEq`.

```rust
#[derive(Debug, PartialEq, Eq)]
struct Card {
    value: u8,
    suit: char,
}

fn main() {
    let a = Card { value: 10, suit: '♥' };
    let b = Card { value: 10, suit: '♠' };
    println!("{}", a == b);
    println!("{}", a != b);
    println!("{}", f64::NAN == f64::NAN);
}
```

```sortie
false
true
false
```

## PartialOrd and Ord: sorting

`PartialOrd` gives `<`, `>`, `<=`, `>=`. `Ord` gives a **total** order, needed for
`sort()`, `max()` or a `BTreeMap`. It requires `Eq` and `PartialOrd`.

The `derive` compares the fields **in the order they are written**, like a dictionary
compares the first letter, then the second:

```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Student {
    grade: u32,
    name: String,
}

fn main() {
    let mut class = vec![
        Student { grade: 15, name: String::from("Zoé") },
        Student { grade: 12, name: String::from("Léo") },
        Student { grade: 15, name: String::from("Ali") },
    ];
    class.sort();
    for s in &class {
        println!("{} {}", s.grade, s.name);
    }
}
```

```sortie
12 Léo
15 Ali
15 Zoé
```

The students are sorted by grade, then by name when there is a tie, because `grade` is
written first.

### Sorting differently, without touching the type

Often, the order depends on the situation. `sort_by_key` and `sort_by` choose the
criterion at sorting time, without any `derive`:

```rust
struct Student {
    name: String,
    grade: u32,
}

fn main() {
    let mut class = vec![
        Student { name: String::from("Zoé"), grade: 15 },
        Student { name: String::from("Léo"), grade: 12 },
        Student { name: String::from("Ali"), grade: 18 },
    ];
    class.sort_by_key(|s| s.grade);
    println!("{}", class[0].name);

    class.sort_by(|a, b| b.grade.cmp(&a.grade)); // descending
    println!("{}", class[0].name);

    class.sort_by(|a, b| a.name.cmp(&b.name));
    println!("{}", class[0].name);
}
```

```sortie
Léo
Ali
Ali
```

`cmp` compares two values and returns an `Ordering`: `Less`, `Equal` or `Greater`.
Swapping `a` and `b` reverses the order.

## Hash: to serve as a key

A `HashMap` key or a `HashSet` element must have `Hash` (being able to sum itself up
as a number) and `Eq`. You derive them together (see [HashMap](hashmap.md)):

```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Square {
    row: u8,
    column: u8,
}

fn main() {
    let mut board = HashMap::new();
    board.insert(Square { row: 0, column: 0 }, "rook");
    board.insert(Square { row: 0, column: 4 }, "king");
    println!("{}", board[&Square { row: 0, column: 4 }]);
}
```

```sortie
king
```

## Default: a starting value

`Default` gives an "empty" value: `0` for numbers, `""` for texts, `false` for
booleans. With `..Default::default()`, you only fill in the fields you care about:

```rust
#[derive(Debug, Default)]
struct Settings {
    volume: u8,
    full_screen: bool,
    username: String,
}

fn main() {
    let s = Settings::default();
    println!("{:?}", s);
    let s2 = Settings { volume: 7, ..Default::default() };
    println!("{:?}", s2);
}
```

```sortie
Settings { volume: 0, full_screen: false, username: "" }
Settings { volume: 7, full_screen: false, username: "" }
```

For other starting values, write `impl Default` by hand:

```rust
struct Game {
    lives: u32,
    level: u32,
}

impl Default for Game {
    fn default() -> Self {
        Game { lives: 3, level: 1 }
    }
}

fn main() {
    let g = Game::default();
    println!("{} lives, level {}", g.lives, g.level);
}
```

```sortie
3 lives, level 1
```

## From and Into: converting

`From` describes a conversion from a type to yours. By writing it, you get `Into`
**for free**, in the other direction:

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

fn main() {
    let f = Fahrenheit::from(Celsius(100.0));
    println!("{}", f.0);

    let g: Fahrenheit = Celsius(0.0).into();
    println!("{}", g.0);
}
```

```sortie
212
32
```

You already know `String::from("…")`: it is the same trait. `From` is also used a lot
for errors, with `?` (see [Handling errors](erreurs.md)).

> **Remember:** implement `From`, never `Into`: `Into` comes on its own.

## Iterator: a handmade iterator

A type becomes an [iterator](../debuter/glossaire.md#iterateur) by implementing a
single method: `next`, which returns `Some(value)`, or `None` when it is over. All the
other methods (`map`, `filter`, `sum`…) come with it.

```rust
struct Countdown {
    remaining: u32,
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(self.remaining + 1)
        }
    }
}

fn main() {
    for n in (Countdown { remaining: 3 }) {
        println!("{n}…");
    }
    let total: u32 = Countdown { remaining: 4 }.sum();
    println!("total: {total}");
}
```

```sortie
3…
2…
1…
total: 10
```

`type Item = u32;` says what type of value the iterator produces. The
[Iterators](iterateurs.md) chapter goes further.

## AsRef<str>: accepting several kinds of text

`AsRef<str>` means "I can be seen as a `&str`". A function that accepts it takes a
`&str` just as well as a `String`:

```rust
fn greet<S: AsRef<str>>(name: S) {
    println!("Hello {}!", name.as_ref());
}

fn main() {
    greet("Zoé");
    greet(String::from("Léo"));
}
```

```sortie
Hello Zoé!
Hello Léo!
```

> **Note:** when you are starting out, a `&str` parameter is almost always enough. You
> will mostly come across `AsRef` in the standard library, for example
> `AsRef<Path>` for [files](fichiers.md).

## Add: overloading the + operator

Operators are traits too, in `std::ops`. Implement `Add` and your type can be added
with `+`:

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector {
    x: i32,
    y: i32,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let a = Vector { x: 1, y: 2 };
    let b = Vector { x: 10, y: 20 };
    println!("{:?}", a + b);
    println!("{}", a + b == Vector { x: 11, y: 22 });
}
```

```sortie
Vector { x: 11, y: 22 }
true
```

`Sub` (`-`), `Mul` (`*`) and `Neg` (the `-` in front) follow the same pattern.

## derive or impl by hand?

| Trait | Used for | `derive`? | By hand when… |
|-------|----------|-----------|---------------|
| `Debug` | `{:?}` | yes | rarely |
| `Display` | `{}`, `to_string()` | **no** | always |
| `Clone` | `.clone()` | yes | rarely |
| `Copy` | automatic copy | yes | never |
| `PartialEq`, `Eq` | `==`, `!=` | yes | some fields do not count |
| `PartialOrd`, `Ord` | `<`, `sort()` | yes | the order is not the fields' order |
| `Hash` | `HashMap` key | yes | rarely |
| `Default` | `::default()` | yes | the starting values are not 0 or empty |
| `From` | conversions | **no** | always |
| `Iterator` | `for`, `map`, `sum`… | **no** | always |
| `Add` | `+` | **no** | always |

> **Tip:** a good habit for your simple data structures:
> `#[derive(Debug, Clone, PartialEq)]`. Add the rest when the compiler asks for it.

## Common mistakes

### Sorting a structure without Ord

```rust norun
#[derive(Debug)]
struct Student {
    name: String,
    grade: u32,
}

fn main() {
    let mut class = vec![
        Student { name: String::from("Zoé"), grade: 15 },
        Student { name: String::from("Léo"), grade: 12 },
    ];
    class.sort();
    println!("{:?}", class);
}
```

```text
error[E0277]: the trait bound `Student: Ord` is not satisfied
  --> src/main.rs:12:11
   |
12 |     class.sort();
   |           ^^^^ the trait `Ord` is not implemented for `Student`
   |
help: consider annotating `Student` with `#[derive(Ord)]`
   |
 2 + #[derive(Ord)]
 3 | struct Student {
   |
```

Rust does not know how to put two students in order. Derive
`PartialEq, Eq, PartialOrd, Ord`, or more simply, choose the criterion:
`class.sort_by_key(|s| s.grade);`.

### Copy on a structure that contains a String

```rust norun
#[derive(Debug, Clone, Copy)]
struct Player {
    name: String,
    points: u32,
}

fn main() {
    let p = Player { name: String::from("Zoé"), points: 3 };
    let q = p;
    println!("{:?} {:?}", p, q);
}
```

```text
error[E0204]: the trait `Copy` cannot be implemented for this type
 --> src/main.rs:2:8
  |
1 | #[derive(Debug, Clone, Copy)]
  |                        ---- in this derive macro expansion
2 | struct Player {
  |        ^^^^^^
3 |     name: String,
  |     ------------ this field does not implement `Copy`
```

A `String` cannot be copied automatically. Remove `Copy` and write
`let q = p.clone();`.

### Eq on a structure with an f64

```rust norun
#[derive(Debug, PartialEq, Eq)]
struct Product {
    name: String,
    price: f64,
}

fn main() {
    let p = Product { name: String::from("bread"), price: 1.2 };
    println!("{:?}", p);
}
```

```text
error[E0277]: the trait bound `f64: Eq` is not satisfied
 --> src/main.rs:4:5
  |
1 | #[derive(Debug, PartialEq, Eq)]
  |                            -- in this derive macro expansion
...
4 |     price: f64,
  |     ^^^^^^^^^^ the trait `Eq` is not implemented for `f64`
```

Floats only have `PartialEq`, because of `NAN`. Remove `Eq`, or store the price in
cents in a `u32`.

## Test yourself

**1.** What is the difference between `Debug` and `Display`?

> **Answer**
>
> `Debug` (`{:?}`) is a technical display for the programmer, and can be derived.
> `Display` (`{}`) is the display for the user, and is written by hand.

**2.** You implement `From<Celsius> for Fahrenheit`. Can you write
`let f: Fahrenheit = c.into();`?

> **Answer**
>
> Yes: implementing `From` gives `Into` automatically in the other direction.

**3.** Write `Display` for a structure `Fraction { top: i32, bottom: i32 }`, which is
displayed as `3/4`.

> **Answer**
>
> ```rust
> use std::fmt;
>
> struct Fraction {
>     top: i32,
>     bottom: i32,
> }
>
> impl fmt::Display for Fraction {
>     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
>         write!(f, "{}/{}", self.top, self.bottom)
>     }
> }
>
> fn main() {
>     println!("{}", Fraction { top: 3, bottom: 4 });
> }
> ```
>
> ```sortie
> 3/4
> ```

## Summary

- `#[derive(…)]` writes `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`,
  `Ord`, `Hash` and `Default` for you.
- `Display` is written by hand and gives `{}` and `to_string()`.
- `Ord` allows `sort()`; `sort_by_key` sorts by a criterion without deriving anything.
- A `HashMap` key needs `Eq` and `Hash`.
- Implement `From` and you get `Into`; implement `next` and you get all of
  `Iterator`.
- `Add` and its cousins from `std::ops` give a meaning to `+`, `-` and `*`.

Next: [handling errors](erreurs.md).
