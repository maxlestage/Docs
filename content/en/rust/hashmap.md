# HashMap, HashSet and BTreeMap

This chapter introduces the collections that store values by key or without duplicates: `HashMap`, `BTreeMap`, `HashSet`, and also the `VecDeque` queue.

> **Goals:**
>
> - add, read and remove key-value pairs in a `HashMap`;
> - count items with `entry().or_insert()`;
> - display a map in a stable order, with a sort or a `BTreeMap`;
> - compare two sets with `HashSet`;
> - choose the right collection for each problem.

All these collections live in `std::collections`. Unlike `Vec`, you have to import
them with `use`.

## What is a HashMap?

A `HashMap` links **keys** to **values**, like a phone book links a name to a phone
number. It is also called a
[dictionary](../debuter/glossaire.md#dictionary). You do not look for "the third
line": you look for "Zoé" directly.

```rust
use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::new();
    ages.insert("Zoé", 12);
    ages.insert("Léo", 9);
    println!("{}", ages.len());
    println!("{}", ages["Zoé"]);
}
```

```sortie
2
12
```

All the keys have the same type (here `&str`), and so do all the values (here `i32`).
A key appears only once: inserting an existing key **replaces** its value.

```rust
use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::new();
    ages.insert("Zoé", 12);
    let old = ages.insert("Zoé", 13);
    println!("{:?} {}", old, ages["Zoé"]);
}
```

```sortie
Some(12) 13
```

`insert` returns the old value in an [Option](types.md): `None` if the key was
new.

## Reading, checking and removing

`ages["Zoé"]` stops the program if the key does not exist. Prefer `get`, which returns
an `Option`:

```rust
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("apples", 3);
    stock.insert("pears", 0);

    println!("{:?}", stock.get("apples"));
    println!("{:?}", stock.get("kiwis"));
    println!("{}", stock.contains_key("pears"));

    let removed = stock.remove("pears");
    println!("{:?} {}", removed, stock.len());

    match stock.get("kiwis") {
        Some(n) => println!("{n} kiwis"),
        None => println!("no kiwis"),
    }
    let n = stock.get("kiwis").copied().unwrap_or(0);
    println!("{n}");
}
```

```sortie
Some(3)
None
true
Some(0) 1
no kiwis
0
```

> **Note:** `get` returns a **reference** (`Some(&3)`, displayed as `Some(3)`).
> `.copied()` makes a copy of it, and `unwrap_or(0)` gives a default value.

To change a value in place, `get_mut` returns a `&mut` reference:

```rust
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("apples", 3);
    if let Some(n) = stock.get_mut("apples") {
        *n += 10;
    }
    println!("{}", stock["apples"]);
}
```

```sortie
13
```

## Counting with entry

`entry(key)` gives access to the slot of a key, whether it exists or not.
`or_insert(0)` creates it with `0` if it is missing, then returns a reference to the
value. It is the ideal way to **count**:

```rust
use std::collections::HashMap;

fn main() {
    let text = "the cat and the dog and the rabbit";
    let mut count = HashMap::new();
    for word in text.split_whitespace() {
        *count.entry(word).or_insert(0) += 1;
    }
    println!("the: {}", count["the"]);
    println!("and: {}", count["and"]);
    println!("cat: {}", count["cat"]);
}
```

```sortie
the: 3
and: 2
cat: 1
```

Read the important line like this: "take the slot of `word`, put 0 in it if it is
empty, then add 1". The star `*` changes the value the reference points to.

The same tool is used to **group**: here, a list of first names by initial.

```rust
use std::collections::HashMap;

fn main() {
    let names = ["Ali", "Bea", "Anna", "Bob", "Chloé"];
    let mut groups: HashMap<char, Vec<&str>> = HashMap::new();
    for n in names {
        let initial = n.chars().next().unwrap();
        groups.entry(initial).or_insert_with(Vec::new).push(n);
    }
    println!("{:?}", groups[&'A']);
    println!("{:?}", groups[&'B']);
}
```

```sortie
["Ali", "Anna"]
["Bea", "Bob"]
```

## Looping over a HashMap

A `for` loop over `&map` gives `(key, value)` pairs:

```rust
use std::collections::HashMap;

fn main() {
    let mut grades = HashMap::new();
    grades.insert("maths", 15);
    grades.insert("french", 12);
    grades.insert("english", 17);

    let mut total = 0;
    for (_subject, grade) in &grades {
        total += grade;
    }
    println!("total: {total}");
}
```

```sortie
total: 44
```

> **Warning:** a `HashMap` keeps **no order**. Two runs of the same program can
> display the pairs in a different order. This is on purpose: it protects against
> some attacks.

For a stable display, gather the pairs in a `Vec`, then sort it:

```rust
use std::collections::HashMap;

fn main() {
    let mut grades = HashMap::new();
    grades.insert("maths", 15);
    grades.insert("french", 12);
    grades.insert("english", 17);

    let mut pairs: Vec<_> = grades.iter().collect();
    pairs.sort();
    for (subject, grade) in pairs {
        println!("{subject}: {grade}");
    }
}
```

```sortie
english: 17
french: 12
maths: 15
```

`keys()` and `values()` loop over only the keys or only the values.

## BTreeMap: keys always sorted

A `BTreeMap` is used exactly like a `HashMap`, but it keeps its keys **sorted**. No
need to sort before displaying:

```rust
use std::collections::BTreeMap;

fn main() {
    let mut scores = BTreeMap::new();
    scores.insert("Zoé", 30);
    scores.insert("Ali", 25);
    scores.insert("Léo", 40);
    for (name, score) in &scores {
        println!("{name}: {score}");
    }
    println!("{:?}", scores.first_key_value());
}
```

```sortie
Ali: 25
Léo: 40
Zoé: 30
Some(("Ali", 25))
```

> **Tip:** a `HashMap` is a bit faster; a `BTreeMap` always gives the same result.
> When you are starting out, and as soon as you display a map, the `BTreeMap` is often
> the simplest choice.

## HashSet: a set without duplicates

A `HashSet` is a bag where each value appears **only once**. There is no associated
value: only items. `insert` returns `false` if the item was already there.

```rust
use std::collections::HashSet;

fn main() {
    let mut seen = HashSet::new();
    println!("{}", seen.insert("red"));
    println!("{}", seen.insert("blue"));
    println!("{}", seen.insert("red"));
    println!("{}", seen.len());
    println!("{}", seen.contains("blue"));
}
```

```sortie
true
true
false
2
true
```

### Removing duplicates from a list

```rust
use std::collections::HashSet;

fn main() {
    let cities = vec!["Lyon", "Paris", "Lyon", "Nice", "Paris"];
    let unique: HashSet<&str> = cities.into_iter().collect();
    let mut list: Vec<_> = unique.into_iter().collect();
    list.sort();
    println!("{:?}", list);
}
```

```sortie
["Lyon", "Nice", "Paris"]
```

### Union, intersection and difference

Two sets are compared like in mathematics. Picture two clubs and their members:

| Operation | Meaning | Method |
|-----------|---------|--------|
| union | members of at least one club | `a.union(&b)` |
| intersection | members of both clubs | `a.intersection(&b)` |
| difference | members of `a` but not of `b` | `a.difference(&b)` |

```rust
use std::collections::{BTreeSet, HashSet};

fn main() {
    let football: HashSet<&str> = ["Ali", "Bea", "Chloé"].into_iter().collect();
    let chess: HashSet<&str> = ["Bea", "Dan"].into_iter().collect();

    // We store each result in a BTreeSet to display it sorted.
    let union: BTreeSet<_> = football.union(&chess).collect();
    let both: BTreeSet<_> = football.intersection(&chess).collect();
    let only_football: BTreeSet<_> = football.difference(&chess).collect();
    println!("{:?}", union);
    println!("{:?}", both);
    println!("{:?}", only_football);
}
```

```sortie
{"Ali", "Bea", "Chloé", "Dan"}
{"Bea"}
{"Ali", "Chloé"}
```

A `BTreeSet` is the sorted set: it is to `HashSet` what `BTreeMap` is to `HashMap`.
The braces `{ }` in the output remind you that this is a set.

## VecDeque: a queue

A `Vec` adds and removes quickly **at the end**. Removing **at the start** is slow:
all the items have to shift. A `VecDeque` (say "vec-deck") is fast at both ends. It is
the line at the bakery: you join at the back, you are served at the front.

```rust
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
    queue.push_back("Ali");
    queue.push_back("Bea");
    queue.push_back("Dan");
    queue.push_front("Grandma"); // goes first

    while let Some(customer) = queue.pop_front() {
        println!("Serving {customer}");
    }
    println!("{}", queue.is_empty());
}
```

```sortie
Serving Grandma
Serving Ali
Serving Bea
Serving Dan
true
```

## Which collection should you choose?

| Need | Collection |
|------|------------|
| an ordered list, access by position | `Vec<T>` |
| add or remove at both ends (queue) | `VecDeque<T>` |
| find a value by its key, fast | `HashMap<K, V>` |
| the same thing, with sorted keys | `BTreeMap<K, V>` |
| unique values, membership tests | `HashSet<T>` |
| unique and sorted values | `BTreeSet<T>` |

> **Remember:** start with `Vec`. Switch to a map when you search "by name", and to a
> set when duplicates get in your way.

## Common mistakes

### Taking the result of get for the value

```rust norun
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("apples", 3);
    let n: i32 = stock.get("apples");
    println!("{n}");
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:6:18
  |
6 |     let n: i32 = stock.get("apples");
  |            ---   ^^^^^^^^^^^^^^^^^^^ expected `i32`, found `Option<&{integer}>`
  |            |
  |            expected due to this
  |
  = note: expected type `i32`
             found enum `Option<&{integer}>`
```

`get` returns an `Option`: the key may be missing. Write
`stock.get("apples").copied().unwrap_or(0)`, or handle both cases with `match`.

### Forgetting mut

```rust norun
use std::collections::HashMap;

fn main() {
    let stock = HashMap::new();
    stock.insert("apples", 3);
    println!("{:?}", stock);
}
```

```text
error[E0596]: cannot borrow `stock` as mutable, as it is not declared as mutable
 --> src/main.rs:5:5
  |
5 |     stock.insert("apples", 3);
  |     ^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
4 |     let mut stock = HashMap::new();
  |         +++
```

Adding a pair changes the map: declare it with `let mut`.

### A struct as a key, without Hash or Eq

```rust norun
use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut names = HashMap::new();
    names.insert(Point { x: 0, y: 0 }, "origin");
    println!("{:?}", names);
}
```

```text
error[E0277]: the trait bound `Point: Eq` is not satisfied
  --> src/main.rs:11:11
   |
11 |     names.insert(Point { x: 0, y: 0 }, "origin");
   |           ^^^^^^ the trait `Eq` is not implemented for `Point`
   |
help: consider annotating `Point` with `#[derive(Eq)]`
   |
 4 + #[derive(Eq)]
 5 | struct Point {
   |
```

A key must be comparable (`Eq`) and "hashable" (`Hash`). Add
`#[derive(Debug, PartialEq, Eq, Hash)]` above the struct. See
[The standard library traits](traits-standard.md).

## Test yourself

**1.** Why can two runs of the same program display a `HashMap` in a different
order? How do you get a stable order?

> **Answer**
>
> A `HashMap` keeps no order. Sort the pairs in a `Vec` before displaying them, or use
> a `BTreeMap`.

**2.** Which line adds 1 to the counter of `word`, creating it at 0 if it does not
exist?

> **Answer**
>
> `*count.entry(word).or_insert(0) += 1;`

**3.** Which letters do "cheese" and "chess" have in common? Answer with two
`HashSet`s and sort the result.

> **Answer**
>
> ```rust
> use std::collections::HashSet;
>
> fn main() {
>     let a: HashSet<char> = "cheese".chars().collect();
>     let b: HashSet<char> = "chess".chars().collect();
>     let mut common: Vec<char> = a.intersection(&b).copied().collect();
>     common.sort();
>     println!("{:?}", common);
> }
> ```
>
> ```sortie
> ['c', 'e', 'h', 's']
> ```

## Summary

- `HashMap` links keys to values: `insert`, `get`, `remove`, `contains_key`.
- `*map.entry(key).or_insert(0) += 1` counts in one line.
- A `HashMap` has no order: sort before displaying, or use a `BTreeMap`.
- `HashSet` keeps unique values and offers `union`, `intersection`, `difference`.
- `VecDeque` is a queue: `push_back` at the back, `pop_front` at the front.

Next: [following a program step by step](pas-a-pas.md).
