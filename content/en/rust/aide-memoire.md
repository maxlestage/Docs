# Cheat sheet

This page sums up, in tables and short examples, everything to remember from the Rust documentation.

## Tools

| Command | Effect |
|----------|-------|
| `rustup update` | updates Rust |
| `cargo new name` | creates a project |
| `cargo run` | compiles and runs |
| `cargo check` | checks without compiling |
| `cargo test` | runs the tests |
| `cargo add crate` | adds a dependency |
| `cargo fmt` / `cargo clippy` | formats / gives advice |

Details: [introduction](introduction.md) and [organizing a project with Cargo](projet.md).

## Variables and types

| Syntax | Meaning |
|----------|------|
| `let x = 5;` | immutable variable |
| `let mut x = 5;` | mutable variable |
| `let x: f64 = 2.5;` | explicit type |
| `const MAX: u32 = 10;` | constant |
| `i32`, `u32`, `f64`, `bool`, `char` | simple types |
| `&str` / `String` | borrowed text / owned text |

```rust
let mut n = 1;
n += 1;
let n = n * 10; // shadowing
let s = format!("n = {n:>4}");
println!("{s} {:?}", "text");
```

```sortie
n =   20 "text"
```

Details: [first steps](premiers-pas.md).

## Conditions and loops

```rust
let x = 7;
let parity = if x % 2 == 0 { "even" } else { "odd" };
let size = match x {
    0 => "zero",
    1..=9 => "small",
    _ => "big",
};
println!("{parity} {size}");
for i in 0..3 {
    print!("{i}");
}
let mut k = 0;
while k < 2 {
    k += 1;
}
let v = loop {
    break k * 10;
};
println!(" {v}");
```

```sortie
odd small
012 20
```

Details: [conditions and loops](conditions-boucles.md).

## Functions and closures

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // no ; : this is the returned value
}

fn main() {
    let triple = |x: i32| x * 3;
    println!("{} {}", add(1, 2), triple(4));
}
```

```sortie
3 12
```

Details: [functions](fonctions.md).

## Text

| Syntax | Meaning |
|----------|------|
| `String::from("a")` / `"a".to_string()` | creates a `String` |
| `s.push_str("…")` / `s.push('c')` | appends text / a character |
| `s.chars().count()` | number of characters (`len()`: bytes) |
| `s.trim()` / `s.to_uppercase()` | cleans up / converts to uppercase |
| `s.contains("x")` / `s.starts_with("x")` | searches |
| `s.replace("a", "b")` | replaces |
| `s.split(',').collect::<Vec<_>>()` | splits |
| `s.parse::<i32>()` | text → number (`Result`) |

```rust
let s = String::from(" Café, tea ");
let words: Vec<&str> = s.trim().split(", ").collect();
println!("{:?} {}", words, s.trim().chars().count());
println!("{}", "21".parse::<i32>().unwrap() * 2);
```

```sortie
["Café", "tea"] 9
42
```

Details: [working with text](texte.md).

## Collections

| Syntax | Meaning |
|----------|------|
| `vec![1, 2]` | creates a `Vec` |
| `v.push(x)` / `v.pop()` | adds / removes at the end |
| `v.get(i)` | `Option` on element `i` |
| `&v[1..3]` | slice |
| `HashMap::new()` | key-value table |
| `m.insert(k, v)` / `m.get(&k)` | adds / reads |
| `*m.entry(k).or_insert(0) += 1` | counts |
| `BTreeMap` | table sorted by key |

```rust
let v = vec![1, 2, 3, 4];
let squares: Vec<i32> = v.iter().map(|x| x * x).collect();
let even_sum: i32 = v.iter().filter(|x| *x % 2 == 0).sum();
println!("{:?} {even_sum} {:?}", squares, &v[..2]);
```

```sortie
[1, 4, 9, 16] 6 [1, 2]
```

Details: [collections](collections.md).

## Ownership and borrowing

| Syntax | Meaning |
|----------|------|
| `let b = a;` | moves `a` (copies for simple types) |
| `a.clone()` | independent copy |
| `&a` | read borrow (several allowed) |
| `&mut a` | write borrow (only one at a time) |
| `fn f<'a>(x: &'a str) -> &'a str` | explicit lifetime |

```rust
fn add_s(word: &mut String) {
    word.push('s');
}

fn main() {
    let mut cat = String::from("cat");
    let copy = cat.clone();
    add_s(&mut cat);
    println!("{copy} {cat}");
}
```

```sortie
cat cats
```

Details: [ownership and borrowing](possession.md).

## Structs, enums and traits

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn sum(&self) -> i32 {
        self.x + self.y
    }
}

enum Command {
    Forward(i32),
    Stop,
}

trait Describe {
    fn describe(&self) -> String;
}

impl Describe for Point {
    fn describe(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 2, y: 3 };
    println!("{:?} {} {}", p, p.sum(), p.describe());
    for c in [Command::Forward(5), Command::Stop] {
        match c {
            Command::Forward(n) => println!("forward by {n}"),
            Command::Stop => println!("stop"),
        }
    }
}
```

```sortie
Point { x: 2, y: 3 } 5 (2, 3)
forward by 5
stop
```

Details: [structs, enums and traits](types.md).

## Generics and traits

| Syntax | Meaning |
|----------|------|
| `fn f<T: Display>(x: T)` | generic function with a bound |
| `fn f(x: impl Display)` | the same thing, shorter |
| `trait T { fn m(&self); }` | defines a trait |
| `impl T for Type { … }` | implements a trait |
| `Vec<Box<dyn T>>` | list of objects of different types |
| `#[derive(Debug, Clone, PartialEq, Default)]` | traits written for you |

```rust
use std::fmt::Display;

fn frame(x: impl Display) -> String {
    format!("[{x}]")
}

fn main() {
    println!("{} {}", frame(7), frame("ok"));
}
```

```sortie
[7] [ok]
```

Details: [generics and traits](generiques.md).

## Option and Result

| Syntax | Meaning |
|----------|------|
| `Some(v)` / `None` | value present / absent |
| `Ok(v)` / `Err(e)` | success / failure |
| `x?` | returns the error to the caller |
| `x.unwrap()` / `x.expect("…")` | the value, or `panic!` |
| `x.unwrap_or(v)` | the value, or `v` |
| `if let Some(v) = x { … }` | acts only if present |

```rust
fn read(t: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = t.parse()?;
    Ok(n * 2)
}

fn main() {
    println!("{:?}", read("21"));
    println!("{}", read("x").unwrap_or(0));
    if let Some(c) = "abc".chars().next() {
        println!("{c}");
    }
}
```

```sortie
Ok(42)
0
a
```

Details: [handling errors](erreurs.md).

## Iterators

| Syntax | Meaning |
|----------|------|
| `v.iter()` / `v.iter_mut()` / `v.into_iter()` | read / modify / consume |
| `map(f)` / `filter(f)` | transforms / keeps if true |
| `enumerate()` / `zip(other)` | `(position, x)` / pairs |
| `rev()` / `take(n)` / `skip(n)` / `chain(other)` | reversed / start / end / one after the other |
| `sum()` / `count()` / `min()` / `max()` | sums up |
| `any(f)` / `all(f)` / `find(f)` / `position(f)` | asks a question |
| `collect::<Vec<_>>()` | gathers (also `String`, `HashMap`) |
| `fold(start, \|acc, x\| …)` | accumulates |
| `impl Iterator for T { type Item = …; fn next(&mut self) … }` | your own iterator |

```rust
let v = vec![3, 1, 4, 1, 5];
let big: Vec<i32> = v.iter().filter(|x| **x > 2).map(|x| x * 10).collect();
let total: i32 = v.iter().sum();
let product = v.iter().fold(1, |acc, x| acc * x);
println!("{:?} {total} {product}", big);
println!("{:?} {}", v.iter().position(|x| *x == 4), v.iter().any(|x| *x > 4));
```

```sortie
[30, 40, 50] 14 60
Some(2) true
```

> **Warning:** iterators are lazy: `map` alone does nothing without
> `collect`, `sum`, `for`…

Details: [iterators](iterateurs.md).

## Modules and crates

| Syntax | Meaning |
|----------|------|
| `mod name { … }` | module in the same file |
| `mod name;` | module in `name.rs` |
| `pub fn` / `pub struct` / `pub field` | makes public (private by default) |
| `use a::b;` / `use a::{b, c};` | shortcut |
| `crate::…` / `super::…` | from the root / the parent |
| `src/main.rs` / `src/lib.rs` | program / library |
| `cargo add name` | adds a crate from crates.io |

```rust
mod tools {
    pub fn double(n: i32) -> i32 {
        n * 2
    }

    pub mod text {
        pub fn shout(s: &str) -> String {
            format!("{}!", s.to_uppercase())
        }
    }
}

use tools::{double, text};

fn main() {
    println!("{} {}", double(4), text::shout("ok"));
}
```

```sortie
8 OK!
```

Details: [modules and crates](modules.md).

## Keyboard and files

| Syntax | Meaning |
|----------|------|
| `io::stdin().read_line(&mut s)` | reads a line from the keyboard |
| `env::args().collect::<Vec<String>>()` | command-line arguments |
| `fs::write("f.txt", text)?` | writes (overwrites) a file |
| `fs::read_to_string("f.txt")?` | reads a whole file |
| `OpenOptions::new().append(true).open(…)?` | appends at the end |
| `BufReader::new(file).lines()` | reads line by line |

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    fs::write("a.txt", "one\ntwo\n")?;
    for line in fs::read_to_string("a.txt")?.lines() {
        println!("- {line}");
    }
    Ok(())
}
```

```sortie
- one
- two
```

Details: [reading and writing: keyboard and files](fichiers.md).

## Box, Rc and RefCell

| Syntax | Meaning |
|----------|------|
| `Box::new(x)` | value on the heap, one owner (recursive types) |
| `Rc::new(x)` / `Rc::clone(&r)` | several owners, read-only |
| `RefCell::new(x)` | `.borrow()` / `.borrow_mut()` checked at run time |
| `Rc<RefCell<T>>` | shared and mutable (single thread) |

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let boxed = Box::new(5);
    let shared = Rc::new(RefCell::new(vec![1]));
    let other = Rc::clone(&shared);
    other.borrow_mut().push(2);
    println!("{} {:?} {}", *boxed + 1, shared.borrow(), Rc::strong_count(&shared));
}
```

```sortie
6 [1, 2] 2
```

Details: [Box, Rc and RefCell](pointeurs.md).

## Threads

| Syntax | Meaning |
|----------|------|
| `thread::spawn(move \|\| …)` | starts a thread |
| `handle.join().unwrap()` | waits for it to finish, gets its result |
| `let (tx, rx) = mpsc::channel();` | channel: `tx.send(x)`, `rx.recv()` |
| `Arc<Mutex<T>>` | value shared between threads: `*m.lock().unwrap() += 1` |
| `thread::scope(\|s\| { s.spawn(…); })` | threads that borrow |

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || *c.lock().unwrap() += 1)
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!("{}", *counter.lock().unwrap());
}
```

```sortie
4
```

Details: [doing several things at once](concurrence.md).

## Tests

| Syntax | Meaning |
|----------|------|
| `assert!(cond)` / `assert_eq!(a, b)` / `assert_ne!(a, b)` | checks, otherwise `panic!` |
| `#[test]` | marks a test function |
| `#[cfg(test)] mod tests { use super::*; … }` | test module |
| `#[should_panic]` | the test must cause a `panic!` |
| `cargo test` / `cargo test name` | runs all the tests / some of them |
| `tests/*.rs` | integration tests |

```rust
fn double(n: i32) -> i32 {
    n * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_four() {
        assert_eq!(double(4), 8);
    }
}

fn main() {
    assert_eq!(double(4), 8);
    println!("ok");
}
```

```sortie
ok
```

Details: [testing your code](tests.md).

> **Tip:** the cheat sheets of the other languages: [Ruby](../ruby/aide-memoire.md) and
> [Rythme](../rythme/aide-memoire.md).
