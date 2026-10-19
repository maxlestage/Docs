# Loops

This page shows how to repeat code with `loop`, `while` and `for`, and how to change the course of a loop with `break` and `continue`.

> **Goals:**
>
> - repeat code with `loop`, `while` and `for`;
> - go through a range or a collection with `for`;
> - leave a loop, even a nested one, with `break` and `continue`.

This chapter follows [conditions](conditions-boucles.md): a loop uses a condition to know when to stop.

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
## Common mistakes

### Forgetting mut in a loop

```rust norun
fn main() {
    let count = 3;
    while count > 0 {
        println!("{count}");
        count -= 1;
    }
}
```

```text
error[E0384]: cannot assign twice to immutable variable `count`
 --> src/main.rs:5:9
  |
2 |     let count = 3;
  |         ----- first assignment to `count`
...
5 |         count -= 1;
  |         ^^^^^^^^^^ cannot assign twice to immutable variable
help: consider making this binding mutable
```

The loop changes `count` on every turn. Declare it as changeable:
`let mut count = 3;`.

### A break with a value in a for

```rust norun
fn main() {
    let numbers = [4, 8, 15];
    let found = for n in numbers {
        if n > 5 {
            break n;
        }
    };
}
```

```text
error[E0571]: `break` with value from a `for` loop
 --> src/main.rs:5:13
  |
3 |     let found = for n in numbers {
  |                 ---------------- you can't `break` with a value in a `for` loop
4 |         if n > 5 {
5 |             break n;
  |             ^^^^^^^ can only break with a value inside `loop` or breakable block
```

Only `loop` can give a value with `break`: a `for` loop can end without ever
reaching the `break`. Create the variable before the loop (`let mut found = 0;`),
then write `found = n; break;` inside the loop.

## Test yourself

**1.** How many times does this code print `beep`: `for _ in 0..3 { println!("beep"); }`?

> **Answer**
>
> Three times: `0..3` gives 0, 1 and 2 (the 3 is excluded).

**2.** What does `for i in 1..=4 { if i == 2 { continue; } print!("{i} "); }` print?

> **Answer**
>
> `1 3 4`: when `i` is 2, `continue` goes to the next turn without printing anything.

**3.** In two nested loops, how do you stop both at once?

> **Answer**
>
> Give a label to the outer loop (`'outer: for …`) and write
> `break 'outer;` in the inner loop.

## Summary

- `loop` repeats until `break`, `while` as long as a condition is true.
- `for x in 1..=5` or `for x in list`: the most common loop.
- `continue` goes to the next turn, `break` stops the loop; a label `'name`
  targets a specific loop.
- A variable changed inside a loop is declared with `let mut`.

Next: [Functions](fonctions.md).
