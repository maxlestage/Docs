# Conditions and loops

This page shows how to choose between several paths with `if` and `match`, and how to repeat code with `loop`, `while` and `for`.

> **Goals:**
>
> - write a [condition](../debuter/glossaire.md#condition) with `if`, `else if` and
>   `else`;
> - choose among several cases with `match` and `matches!`;
> - repeat code with `loop`, `while` and `for`;
> - leave a loop, even a nested one, with `break` and `continue`;
> - read an optional value with `if let` and `while let`.

## The if condition

`if` runs a block only if the condition is true. No need for parentheses,
but the curly braces are required.

```rust
let temperature = 25;
if temperature > 30 {
    println!("It's hot.");
} else if temperature > 15 {
    println!("It's nice.");
} else {
    println!("It's cool.");
}
```

```sortie
It's nice.
```

Rust tests the conditions in order and stops at the first true one. `else`
catches all the other cases.

The condition must be a real [boolean](../debuter/glossaire.md#boolean) (`bool`):
`true` or `false`. A number is not enough (see [Common
mistakes](#common-mistakes)).

### Comparison operators

| Operator | Meaning |
|-----------|------|
| `==` / `!=` | equal / not equal |
| `<` / `<=` | less than / less than or equal |
| `>` / `>=` | greater than / greater than or equal |
| `&&` / `\|\|` / `!` | and / or / not |

```rust
let age = 15;
let with_adult = true;
if age >= 18 || with_adult {
    println!("Entry allowed");
}
if !(age >= 18) && age > 12 {
    println!("Teenager");
}
```

```sortie
Entry allowed
Teenager
```

## if is an expression

In Rust, `if` **returns a value**. You can store it in a variable:

```rust
let age = 20;
let status = if age >= 18 { "adult" } else { "minor" };
println!("{status}");
```

```sortie
adult
```

> **Warning:** both branches must give the same type.

## Choosing with match

`match` compares a value with several **patterns** (possible shapes). The first
one that matches wins. Think of mail sorting: each letter goes into the first box
that suits it.

```rust
let day = 6;
let name = match day {
    1 => "monday",
    2 | 3 | 4 => "middle of the week",
    5 => "friday",
    6 | 7 => "weekend",
    _ => "unknown day",
};
println!("{name}");
```

```sortie
weekend
```

`|` means "or". Patterns can also be ranges:

```rust
let grade = 14;
match grade {
    0..=9 => println!("Insufficient"),
    10..=15 => println!("Good"),
    _ => println!("Very good"),
}
```

```sortie
Good
```

> **Remember:** `match` must cover **all** the cases. The `_` pattern catches everything
> else.

### Guards

A pattern can be followed by `if` and a condition: this is a **guard**. The name `n`
receives the tested value:

```rust
let temperature = -4;
let state = match temperature {
    n if n < 0 => "ice",
    0..=99 => "liquid",
    _ => "steam",
};
println!("{state}");
```

```sortie
ice
```

### Testing a single pattern with matches!

`matches!(value, pattern)` answers `true` or `false`. It is handy inside an `if`:

```rust
let letter = 'e';
let vowel = matches!(letter, 'a' | 'e' | 'i' | 'o' | 'u' | 'y');
println!("{vowel}");
println!("{}", matches!(42, 0..=9));
```

```sortie
true
false
```

`match` becomes even more useful with enums: see
[Structs, enums and traits](types.md).

## The loop loop

A [loop](../debuter/glossaire.md#loop) repeats a block of code. `loop` repeats
forever, until a `break`. The `break` can return a value:

```rust
let mut n = 1;
let result = loop {
    n = n * 2;
    if n > 100 {
        break n;
    }
};
println!("{result}");
```

```sortie
128
```

## The while loop

`while` repeats as long as the condition is true. It is tested before each turn:

```rust
let mut count = 3;
while count > 0 {
    println!("{count}...");
    count -= 1;
}
println!("Go!");
```

```sortie
3...
2...
1...
Go!
```

## The for loop

`for` goes through a range or a collection, element by element. It is the most
used loop.

### Over a range

`1..4` goes from 1 to 3 (the 4 is excluded). `1..=4` goes from 1 to 4 included. `rev()` goes
backwards.

```rust
for i in 1..4 {
    print!("{i} ");
}
println!();
for i in (1..=4).rev() {
    print!("{i} ");
}
println!();
```

```sortie
1 2 3 
4 3 2 1 
```

### Over a collection

```rust
let fruits = ["apple", "pear", "kiwi"];
for fruit in fruits {
    println!("I like {fruit}s");
}
for (i, fruit) in fruits.iter().enumerate() {
    println!("{i}: {fruit}");
}
```

```sortie
I like apples
I like pears
I like kiwis
0: apple
1: pear
2: kiwi
```

`enumerate()` gives, along with each element, its position (starting from 0).

## continue and break

`continue` goes to the next turn, `break` stops the loop:

```rust
for n in 1..10 {
    if n % 2 == 0 {
        continue;
    }
    if n > 7 {
        break;
    }
    print!("{n} ");
}
println!();
```

```sortie
1 3 5 7 
```

### Leaving two loops: labels

In two nested loops, `break` only stops the inner loop. To stop
the outer one, give it a **label**: a name that starts with a single quote,
like `'grid`.

```rust
'grid: for row in 1..=3 {
    for column in 1..=3 {
        if row * column == 4 {
            println!("Found at ({row}, {column})");
            break 'grid;
        }
    }
}
println!("End");
```

```sortie
Found at (2, 2)
End
```

`continue 'grid` works the same way: it goes to the next turn of the labeled
loop.

## if let and while let

Some values are **optional**: they are `Some(x)` if there is something,
or `None` if there is nothing. This is the `Option` type, covered in detail in
[Structs, enums and traits](types.md).

`if let` runs a block only if the value matches the pattern, and gives a name to
what it holds:

```rust
let mut stack = vec![1, 2, 3];
if let Some(top) = stack.pop() {
    println!("Removed: {top}");
}
```

```sortie
Removed: 3
```

`pop()` removes the last element of a list and returns `Some(element)`, or `None` if
the list is empty. `while let` repeats as long as the pattern matches:

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    print!("{top} ");
}
println!();
```

```sortie
3 2 1 
```

> **Tip:** `if let` is a shortened `match`, for when you only care about one case.

## Common mistakes

### A condition that is not a boolean

```rust norun
fn main() {
    let n = 1;
    if n {
        println!("yes");
    }
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if n {
  |        ^ expected `bool`, found integer
```

Rust does not turn a number into a boolean. Write the comparison: `if n != 0`.

### = instead of ==

```rust norun
fn main() {
    let x = 5;
    if x = 5 {
        println!("five");
    }
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if x = 5 {
  |        ^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
```

`=` stores a value, `==` compares. Write `if x == 5`.

### A match that forgets some cases

```rust norun
fn main() {
    let grade = 14;
    match grade {
        0..=9 => println!("Insufficient"),
        10..=20 => println!("Good"),
    }
}
```

```text
error[E0004]: non-exhaustive patterns: `i32::MIN..=-1_i32` and `21_i32..=i32::MAX` not covered
 --> src/main.rs:3:11
  |
3 |     match grade {
  |           ^^^^^ patterns `i32::MIN..=-1_i32` and `21_i32..=i32::MAX` not covered
```

An `i32` can also be `-5` or `300`. Add a last line
`_ => println!("Invalid grade"),`.

### Two branches of different types

```rust norun
fn main() {
    let age = 20;
    let status = if age >= 18 { "adult" } else { 0 };
}
```

```text
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:3:50
  |
3 |     let status = if age >= 18 { "adult" } else { 0 };
  |                                 -------          ^ expected `&str`, found integer
  |                                 |
  |                                 expected because of this
```

A variable has only one type: both branches must give text, for example
`else { "minor" }`.

## Test yourself

**1.** How many times does this code print `beep`: `for _ in 0..3 { println!("beep"); }`?

> **Answer**
>
> Three times: `0..3` gives 0, 1 and 2 (the 3 is excluded).

**2.** Write a `match` that prints `small` for 0 to 9, `medium` for 10 to 99 and
`large` otherwise, for `n = 42`.

> **Answer**
>
> ```rust
> let n = 42;
> match n {
>     0..=9 => println!("small"),
>     10..=99 => println!("medium"),
>     _ => println!("large"),
> }
> ```
>
> ```sortie
> medium
> ```

**3.** In two nested loops, how do you stop both at once?

> **Answer**
>
> Give a label to the outer loop (`'outer: for …`) and write
> `break 'outer;` in the inner loop.

## Summary

- `if … else` chooses a path, and can return a value.
- `match` compares a value with patterns; `_` catches the rest; `matches!` tests a
  single pattern.
- `loop` repeats until `break`, `while` as long as a condition is true.
- `for x in 1..=5` or `for x in list`: the most common loop.
- `continue` goes to the next turn, `break` stops the loop; a label `'name`
  targets a specific loop.
- `if let Some(x) = …` and `while let` read an optional value.

Compare with [Ruby](../ruby/conditions-boucles.md) and
[Rythme](../rythme/conditions-boucles.md).

Next: [Functions](fonctions.md).
