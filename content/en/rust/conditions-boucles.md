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

A program often has to decide. If it is hot, it suggests a drink; if it is cold, a
coat. `if` is how you make that choice.

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

Line by line:

- `let temperature = 25;` stores 25 in the variable `temperature`;
- first test: "is 25 greater than 30?". No: Rust skips that block;
- second test, with `else if`: "is 25 greater than 15?". Yes: Rust prints
  "It's nice.";
- the `else` block does not run, because a test already succeeded.

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

**Your turn:** write a program that stores `hour = 15` in a variable, then prints
"Good morning" if the hour is before 12, "Good afternoon" if it is before 18, and
"Good evening" otherwise.

> **Solution**
>
> ```rust
> let hour = 15;
> if hour < 12 {
>     println!("Good morning");
> } else if hour < 18 {
>     println!("Good afternoon");
> } else {
>     println!("Good evening");
> }
> ```
>
> ```sortie
> Good afternoon
> ```

## if is an expression

> **Note:** this part can wait. Read it once the basics feel solid.

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

When a value can take many shapes, a long chain of `else if` becomes hard to read.
`match` lays out all these cases neatly, one under the other.

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

Line by line:

- `day` is 6;
- `match day` compares 6 with each pattern, from top to bottom: 1? no. 2, 3 or 4? no.
  5? no;
- `6 | 7` matches: the `match` gives the text `"weekend"`, stored in `name`;
- the `_` pattern is not used here: it only counts when no other pattern matches.

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

**Your turn:** write a program that stores `coins = 3` in a variable, then uses
`match` to print "one" for 1, "two" for 2, "three" for 3, and "many" for
everything else.

> **Solution**
>
> ```rust
> let coins = 3;
> match coins {
>     1 => println!("one"),
>     2 => println!("two"),
>     3 => println!("three"),
>     _ => println!("many"),
> }
> ```
>
> ```sortie
> three
> ```

### Guards

> **Note:** this part can wait. Read it once the basics feel solid.

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

Imagine you are asked to print the numbers from 1 to 100. Writing 100 `println!`
would be long and tedious. A loop does this repetitive work for you.

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

Line by line:

- `n` starts at 1; `mut` lets you change it later;
- on each turn, `n` is multiplied by 2: it becomes 2, then 4, 8, 16, 32, 64, 128;
- as long as `n` is not above 100, the `if` does nothing and the loop starts again;
- when `n` is 128, `break n;` stops the loop and gives 128, stored in `result`.

## The while loop

Sometimes you do not know in advance how many turns you will need. You repeat as
long as a situation lasts, like stirring a soup as long as it is not hot.

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

Line by line:

- `count` starts at 3;
- before each turn, Rust asks `count > 0`. `3 > 0` is true: it enters the block;
- the block prints `count`, then takes 1 away from it (`-= 1`): it becomes 2, then
  1, then 0;
- when `count` is 0, `0 > 0` is false: Rust leaves the loop and prints "Go!".

**Your turn:** write a program that prints the numbers from 1 to 5 with a `while`
loop, then "Done!".

> **Solution**
>
> ```rust
> let mut n = 1;
> while n <= 5 {
>     println!("{n}");
>     n += 1;
> }
> println!("Done!");
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> Done!
> ```

## The for loop

Very often, you have a series of things to handle one by one: each item on a
shopping list, each number from 1 to 10. `for` is made for this.

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

Line by line:

- `for i in 1..4` gives `i` the value 1, then 2, then 3;
- `print!` prints without going to a new line: the numbers stay side by side;
- `println!()`, empty, just goes to a new line;
- the second loop does the same from 1 to 4, but backwards thanks to `rev()`.

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

**Your turn:** write a program that prints the 7 times table, from "7 x 1 = 7" to
"7 x 5 = 35", one line per calculation.

> **Solution**
>
> ```rust
> for i in 1..=5 {
>     let result = 7 * i;
>     println!("7 x {i} = {result}");
> }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

## continue and break

Sometimes you want to change the course of a loop along the way: stop searching as
soon as you have found something, or skip some elements. That is what these two
words are for.

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

Line by line:

- `n` takes the values 1, 2, 3… up to 9;
- if `n` is even (the remainder of the division by 2 is 0), `continue` goes straight
  to the next turn: the `print!` is not reached;
- if `n` is above 7, `break` stops the whole loop: 9 is never printed;
- otherwise, `print!` prints the number.

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

Think of a mailbox: sometimes there is a letter, sometimes nothing. Before reading
the letter, you have to check that it is there. `if let` and `while let` do this
check for you.

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
