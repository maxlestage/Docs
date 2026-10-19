# Collections

This page introduces the most useful collections in Rust: `Vec` vectors, `HashMap` tables, iterators and slices.

> **Goals:**
>
> - choose between a fixed-size array and a `Vec`;
> - add, read and remove elements of a `Vec` safely;
> - map keys to values with `HashMap` and `BTreeMap`;
> - transform a collection with `map`, `filter` and `collect`;
> - take part of a list with a slice.

A **collection** is a value that contains several other values, like a storage
box.

## Fixed-size arrays

An array `[T; N]` always contains the same number `N` of elements, all of the same type
`T`. Positions (the **indices**) start at 0:

```rust
let days = ["mon", "tue", "wed"];
println!("{}", days[0]);
println!("{}", days.len());
println!("{:?}", days);
```

```sortie
mon
3
["mon", "tue", "wed"]
```

An array can also be filled with the same value: `[0; 5]` gives five zeros.

```rust
let zeros = [0; 5];
println!("{:?}", zeros);
```

```sortie
[0, 0, 0, 0, 0]
```

## Vectors: Vec

A `Vec` (a "vector") is a [list](../debuter/glossaire.md#list) that can grow or
shrink. It is the most common collection. The `vec![…]` macro creates one; `push`
adds to the end, `pop` removes the last element.

```rust
let mut grades = vec![12, 15, 9];
grades.push(18);
println!("{:?}", grades);
println!("First: {}", grades[0]);
println!("Size: {}", grades.len());
let last = grades.pop();
println!("{:?} {:?}", last, grades);
```

```sortie
[12, 15, 9, 18]
First: 12
Size: 4
Some(18) [12, 15, 9]
```

`pop` returns `Some(18)`: the removed value, inside an `Option`, because the list could
have been empty.

### Array or Vec?

| | Array `[T; N]` | `Vec<T>` |
|-|----------------|----------|
| Size | fixed when you write the code | changes while the program runs |
| Add / remove | impossible | `push`, `pop`, `insert`, `remove` |
| Typical use | the 7 days, the 12 months | a shopping list, grades |

> **Tip:** when in doubt, pick a `Vec`.

### Reading safely with get

`grades[10]` stops the program if the element does not exist. `get` returns an `Option`
(see [Option](types.md)):

```rust
let grades = vec![12, 15, 9];
println!("{:?}", grades.get(1));
println!("{:?}", grades.get(10));
```

```sortie
Some(15)
None
```

### Looping over a vector

```rust
let mut prices = vec![10, 20, 30];
for p in &prices {
    print!("{p} ");
}
println!();
for p in &mut prices {
    *p += 1;
}
println!("{:?}", prices);
```

```sortie
10 20 30 
[11, 21, 31]
```

> **Note:** the `&` in front of `prices` borrows the vector instead of consuming it.
> Everything is explained in [Ownership and borrowing](possession.md).

## Tables: HashMap

A `HashMap` is a [dictionary](../debuter/glossaire.md#dictionary): it maps
**keys** to **values**, like a phone book maps a name to a number. It is not one of
the types loaded automatically: you must import it with `use`.

```rust
use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::new();
    ages.insert("Alice", 30);
    ages.insert("Bob", 25);
    ages.insert("Alice", 31);
    println!("{:?}", ages.get("Alice"));
    println!("{}", ages.len());
    println!("{}", ages.contains_key("Zoe"));
}
```

```sortie
Some(31)
2
false
```

### Counting with entry

`entry(...).or_insert(...)` creates the value if it is missing, then gives access to
the value. The star `*` lets you change what it points to:

```rust
use std::collections::HashMap;

fn main() {
    let text = "the cat and the dog and the rabbit";
    let mut count = HashMap::new();
    for word in text.split_whitespace() {
        *count.entry(word).or_insert(0) += 1;
    }
    let mut pairs: Vec<_> = count.into_iter().collect();
    pairs.sort();
    println!("{:?}", pairs);
}
```

```sortie
[("and", 2), ("cat", 1), ("dog", 1), ("rabbit", 1), ("the", 3)]
```

> **Warning:** the order in which a `HashMap` is traversed is random. For a sorted
> order, sort the result as above, or use `BTreeMap`.

### BTreeMap: a sorted table

`BTreeMap` is used like `HashMap`, but keeps the keys in order:

```rust
use std::collections::BTreeMap;

fn main() {
    let mut stock = BTreeMap::new();
    stock.insert("pears", 4);
    stock.insert("apricots", 10);
    stock.insert("kiwis", 7);
    for (fruit, quantity) in &stock {
        println!("{fruit}: {quantity}");
    }
}
```

```sortie
apricots: 10
kiwis: 7
pears: 4
```

## Iterators

`iter()` creates an **iterator**: an object that gives the elements one by one, like a
checkout conveyor belt. You then chain operations with
[closures](fonctions.md#closures). Iterators have their own chapter:
[Iterators](iterateurs.md).

| Method | Effect |
|--------|--------|
| `map(f)` | transforms each element |
| `filter(f)` | keeps the elements that satisfy `f` |
| `collect()` | gathers the result (often into a `Vec`) |
| `sum()` | adds up |
| `count()` | counts |

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];
let doubles: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
let evens: Vec<&i32> = numbers.iter().filter(|n| *n % 2 == 0).collect();
let total: i32 = numbers.iter().sum();
let big = numbers.iter().filter(|n| **n > 3).count();
println!("{:?}", doubles);
println!("{:?}", evens);
println!("{total} {big}");
```

```sortie
[2, 4, 6, 8, 10, 12]
[2, 4, 6]
21 3
```

You can chain as many steps as you want:

```rust
let words = vec!["rust", "is", "fast"];
let result: Vec<String> = words
    .iter()
    .filter(|w| w.len() > 3)
    .map(|w| w.to_uppercase())
    .collect();
println!("{:?}", result);
```

```sortie
["RUST", "FAST"]
```

> **Tip:** an iterator does nothing until you consume it (with `collect`, `sum`,
> `for`…). We say it is **lazy**.

## Slices

A **slice** (`&[T]`) is a view on part of an array or a vector, without any
copy:

```rust
let letters = vec!['a', 'b', 'c', 'd', 'e'];
let middle = &letters[1..4];
let start = &letters[..2];
println!("{:?} {:?}", middle, start);
```

```sortie
['b', 'c', 'd'] ['a', 'b']
```

A function that accepts `&[i32]` works with an array as well as with a `Vec`:

```rust
fn average(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

fn main() {
    let v = vec![10.0, 12.0, 17.0];
    println!("{}", average(&v));
    println!("{}", average(&[4.0, 6.0]));
}
```

```sortie
13
5
```

## Common mistakes

### Forgetting to import HashMap

```rust norun
fn main() {
    let mut ages = HashMap::new();
    ages.insert("Alice", 30);
}
```

```text
error[E0433]: cannot find type `HashMap` in this scope
 --> src/main.rs:2:20
  |
2 |     let mut ages = HashMap::new();
  |                    ^^^^^^^ use of undeclared type `HashMap`
help: consider importing this struct
  |
1 + use std::collections::HashMap;
```

Add `use std::collections::HashMap;` at the top of the file.

### Mixing types in a Vec

```rust norun
fn main() {
    let mixed = vec![1, "two", 3];
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:2:25
  |
2 |     let mixed = vec![1, "two", 3];
  |                         ^^^^^ expected integer, found `&str`
```

All the elements of a `Vec` have the same type. Write `vec![1, 2, 3]`, or store
different values in an [enum](types.md).

### Reusing a Vec after a for

```rust norun
fn main() {
    let grades = vec![12, 15, 9];
    for g in grades {
        println!("{g}");
    }
    println!("{}", grades.len());
}
```

```text
error[E0382]: borrow of moved value: `grades`
 --> src/main.rs:6:20
  |
2 |     let grades = vec![12, 15, 9];
  |         ------ move occurs because `grades` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     for g in grades {
  |              ------ `grades` moved due to this implicit call to `.into_iter()`
...
6 |     println!("{}", grades.len());
  |                    ^^^^^^ value borrowed here after move
```

`for g in grades` consumes the vector. To keep it, loop over it by borrowing:
`for g in &grades`. The reason is explained in [Ownership and borrowing](possession.md).

## Test yourself

**1.** What does `vec![3, 1, 2].get(5)` give?

> **Answer**
>
> `None`: there is no element at position 5. With `[5]`, the program would
> stop.

**2.** How do you count how many times each letter appears in `"papa"`?

> **Answer**
>
> ```rust
> use std::collections::BTreeMap;
>
> fn main() {
>     let mut count = BTreeMap::new();
>     for c in "papa".chars() {
>         *count.entry(c).or_insert(0) += 1;
>     }
>     println!("{:?}", count);
> }
> ```
>
> ```sortie
> {'a': 2, 'p': 2}
> ```
>
> A `HashMap` works too, but `BTreeMap` shows the letters in order.

**3.** With `let v = vec![1, 2, 3, 4];`, how do you get the sum of the even numbers?

> **Answer**
>
> ```rust
> let v = vec![1, 2, 3, 4];
> let sum: i32 = v.iter().filter(|n| *n % 2 == 0).sum();
> println!("{sum}");
> ```
>
> ```sortie
> 6
> ```

## Summary

- An array `[T; N]` has a fixed size; a `Vec` grows: `vec![…]`, `push`, `pop`,
  `get`.
- `HashMap` maps keys to values; its order is random.
- `BTreeMap` keeps the keys sorted.
- `iter()`, `map`, `filter` and `collect` transform a collection.
- `&v[1..3]` is a slice: a view on part of a list.

Compare with [Ruby](../ruby/collections.md) and [Rythme](../rythme/collections.md).

Next: [HashMap, HashSet and BTreeMap](hashmap.md)
