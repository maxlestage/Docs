# First steps

This page covers variables, basic types, printing with `println!`, comments and constants in Rust.

> **Goals:**
>
> - create variables with `let`, and make them changeable with `mut`;
> - choose the right type of number and convert with `as`;
> - group values in a tuple;
> - print values with `println!` and `format!`;
> - write comments and constants.

## Variables with let

A program often needs to remember a value to use it again later: a name, a score, a
price. That is what variables are for.

A [variable](../debuter/glossaire.md#variable) is a label put on a
value. It is created with `let`. By default, it can **not** change: it is
[immutable](../debuter/glossaire.md#immutable).

```rust
let city = "Lyon";
println!("{city}");
```

```sortie
Lyon
```

If you try to change it, the compiler refuses (see
[Common mistakes](#common-mistakes)).

## Changeable variables with mut

Some values have to change while the program runs: a score that goes up with each
point, the money in a wallet. For those, Rust asks for one more word.

Add `mut` to make the variable [mutable](../debuter/glossaire.md#mutable),
that is, changeable:

```rust
let mut counter = 0;
counter = counter + 1;
counter += 1;
println!("{counter}");
```

```sortie
2
```

Line by line:

- `let mut counter = 0;` creates a changeable variable that holds 0;
- `counter = counter + 1;` reads the current value (0), adds 1, and stores the
  result, 1, in `counter`;
- `counter += 1;` does the same thing, shorter: `counter` now holds 2;
- `println!` prints the final value, 2.

`counter += 1` is a shortcut for `counter = counter + 1`. There are also `-=`,
`*=` and `/=`.

> **Remember:** in Rust, everything is constant by default. Write `mut` only when
> the value has to change.

**Your turn:** write a program that creates a changeable variable `money` holding 10,
adds 5 to it with `+=`, then prints its value.

> **Solution**
>
> ```rust
> let mut money = 10;
> money += 5;
> println!("{money}");
> ```
>
> ```sortie
> 15
> ```

## Shadowing

> **Note:** this part can wait. Read it once the basics feel solid.

You can declare a new variable with the same name. The old one is **shadowed**:
it still exists, but the name now refers to the
new one. The type can even change:

```rust
let text = "42";
let text: i32 = text.parse().unwrap();
let text = text * 2;
println!("{text}");
```

```sortie
84
```

Here, `text` is first a text, then a number. `parse` reads a number from a text
(details in [Working with text](texte.md)).

> **Note:** shadowing is not changing. With `let`, you create a **new**
> variable; with `mut`, you change the value of the same variable.

## Simple types

A number and a text are not stored the same way, and you do not do the same things
with them: you add up prices, not first names. The type lets Rust know what it is
holding, and warn you if you mix things up.

A [type](../debuter/glossaire.md#type) says what kind of value a
variable holds. Rust often guesses the type by itself. You can also write it after `:`.

| Type | Content | Example |
|------|---------|---------|
| `i32` | [integer](../debuter/glossaire.md#integer) (positive or negative) | `-7` |
| `u32` | positive integer | `42` |
| `f64` | decimal number ([float](../debuter/glossaire.md#float)) | `3.14` |
| `bool` | true or false ([boolean](../debuter/glossaire.md#boolean)) | `true` |
| `char` | one character | `'é'` |

```rust
let integer: i32 = -7;
let positive: u32 = 42;
let price: f64 = 9.99;
let active: bool = true;
let letter: char = 'é';
println!("{integer} {positive} {price} {active} {letter}");
println!("{}", 7 / 2);
println!("{}", 7.0 / 2.0);
println!("{}", 7 % 2);
```

```sortie
-7 42 9.99 true é
3
3.5
1
```

Line by line:

- the five `let` lines create one variable of each type; the type is written after `:`;
- `7 / 2` divides two integers: the result is an integer too, `3`. What comes after
  the decimal point is lost;
- `7.0 / 2.0` divides two decimal numbers: the result is `3.5`;
- `7 % 2` gives the remainder of 7 divided by 2: `1`.

> **Warning:** a division between integers gives an integer: `7 / 2` is `3`. The
> `%` gives the remainder of the division: `7 % 2` is `1`.

### Integer sizes

An integer takes a fixed amount of room in memory. The letter says whether it can be negative (`i`
for *integer*, signed) or not (`u` for *unsigned*). The number gives its
size, in bits:

| Type | Smallest value | Largest value |
|------|--------------------|--------------------|
| `u8` | 0 | 255 |
| `i32` | −2,147,483,648 | 2,147,483,647 |
| `u32` | 0 | 4,294,967,295 |
| `i64` | about −9 × 10¹⁸ | about 9 × 10¹⁸ |
| `usize` | 0 | depends on the machine (positions in a list) |

```rust
println!("{} {}", u8::MIN, u8::MAX);
println!("{}", i32::MAX);
```

```sortie
0 255
2147483647
```

> **Tip:** without a particular reason, pick `i32` for an integer and `f64` for a
> decimal number. These are Rust's default choices.

### Overflow

> **Note:** this part can wait. Read it once the basics feel solid.

What happens if a calculation goes past the limits? This is an **overflow**.
During development, the program stops with an error.
To choose the behavior yourself, there are methods:

```rust
let x: u8 = 250;
println!("{}", x.saturating_add(10));
println!("{}", x.wrapping_add(10));
```

```sortie
255
4
```

- `saturating_add` stops at the maximum, like a full gauge;
- `wrapping_add` starts again from zero, like a car's odometer going all the way around.

### Converting with as

Rust never mixes two types of numbers by itself. To convert, write `as`
followed by the type you want:

```rust
let apples: i32 = 7;
let half = apples as f64 / 2.0;
println!("{half}");
let price = 3.99;
println!("{}", price as i32);
let big: i32 = 300;
println!("{}", big as u8);
```

```sortie
3.5
3
44
```

> **Warning:** `as` checks nothing. From `f64` to `i32`, it cuts off the decimals
> (`3.99` becomes `3`, without rounding). To a type that is too small, the result is
> surprising: `300` converted to `u8` is `44`.

**Your turn:** a bag holds `candies: i32 = 9`. Write a program that shares these
candies between 2 children and prints each one's share, with the decimal part.

> **Solution**
>
> ```rust
> let candies: i32 = 9;
> let share = candies as f64 / 2.0;
> println!("{share}");
> ```
>
> ```sortie
> 4.5
> ```

## Tuples

Some values always go together: the two coordinates of a point on a map, or the name
and age of a person. A tuple puts them in a single package, like an index card.

A **tuple** groups a few values, of types that can be different, between
parentheses. You read each value with `.0`, `.1`, `.2`…:

```rust
let point = (3, 4);
let student = ("Léa", 17, true);
println!("{} {}", point.0, point.1);
println!("{:?}", student);
let (name, age, _) = student;
println!("{name} is {age} years old");
```

```sortie
3 4
("Léa", 17, true)
Léa is 17 years old
```

Line by line:

- `point` groups two numbers; `student` groups a text, a number and a boolean;
- `point.0` reads the first value (3) and `point.1` the second one (4): counting
  starts at 0;
- `{:?}` prints the whole tuple, with its parentheses;
- the last `println!` uses `name` and `age`, taken out of the tuple as explained
  just below.

The line `let (name, age, _) = student;` **destructures** the tuple: each value goes into
its own variable. The `_` ignores the third one.

## Text: String and &str

A program handles a lot of text: first names, messages, answers to print. Rust stores
this text in two ways, depending on whether it must be able to change or not.

Rust has two types of text (of [strings of
characters](../debuter/glossaire.md#string)):

- `&str`: a fixed text, often written between quotes in the code.
- `String`: a text that belongs to you and that you can make bigger.

```rust
let hello: &str = "Hello";
let mut message = String::from(hello);
message.push_str(", world");
message.push('!');
println!("{message}");
println!("{}", message.len());
```

```sortie
Hello, world!
13
```

Line by line:

- `hello` is a `&str`: a fixed text, written directly in the code;
- `String::from(hello)` turns it into a `String`, stored in `message`. The `mut` lets
  you change it afterwards;
- `push_str` adds a piece of text at the end; `push` adds a single character, written
  between single quotes;
- `message.len()` gives the length of the text: 13.

Text has its own page: [Working with text](texte.md).

## Printing with println! and format!

Printing is how your program talks to you: it shows a result, a message, a value to
check. Often, you want to slip values into the middle of a sentence.

`println!` prints, `format!` builds a `String` without printing it. The `{}` are
placeholders, replaced by the values:

```rust
let name = "Léa";
let grade = 17.456;
println!("{} got {}", name, grade);
println!("{name} got {grade:.1}");
println!("[{:>5}]", 42);
println!("[{:<5}]", 42);
println!("{:?}", "with quotes");
let sentence = format!("{name}!");
println!("{sentence}");
```

```sortie
Léa got 17.456
Léa got 17.5
[   42]
[42   ]
"with quotes"
Léa!
```

Line by line:

- the first `println!` fills the two `{}` in order: first `name`, then `grade`;
- the second one writes the variable names between the braces, and `:.1` rounds
  `grade` to one digit after the decimal point;
- `{:>5}` and `{:<5}` reserve 5 places and push `42` to the right or to the left; the
  brackets show the spaces;
- `format!` builds the text `"Léa!"` and stores it in `sentence`, without printing
  anything: the last `println!` is what prints it.

| Placeholder | Effect |
|-------------|-------|
| `{}` | normal display |
| `{name}` | prints the variable `name` |
| `{:.2}` | two digits after the decimal point |
| `{:>5}` | right-aligned on 5 characters |
| `{:?}` | debug display |

`print!` (without `ln`) prints without going to the next line.

**Your turn:** write a program that stores `"bread"` in `product` and `1.456` in
`price`, then prints "The bread costs 1.46 euros", with two digits after the decimal
point.

> **Solution**
>
> ```rust
> let product = "bread";
> let price = 1.456;
> println!("The {product} costs {price:.2} euros");
> ```
>
> ```sortie
> The bread costs 1.46 euros
> ```

## Comments

A program is often read again: by you in a month, or by someone else. A comment
explains what the code does, like a note in the margin of a recipe.

A [comment](../debuter/glossaire.md#comment) is a note for humans:
the compiler ignores it.

```rust
// A comment on one line.
/* A comment
   on several lines. */
/// A documentation comment (before a function, a struct…).
fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    println!("{}", double(4)); // prints 8
}
```

```sortie
8
```

Line by line:

- the lines that start with `//`, and everything between `/*` and `*/`, are ignored;
- `///` describes the function `double` that follows: tools use it to build the
  documentation;
- in `main`, the comment `// prints 8` is at the end of the line: the code before it
  runs normally.

## Constants

Some values never change and are used in several places, like a tax rate. With a
name, the code reads better, and you fix the value in one place only.

A [constant](../debuter/glossaire.md#constant) is a value set once and
for all. It is written with `const`, in CAPITAL LETTERS, and its type is required:

```rust
const TAX_RATE: f64 = 0.2;
const DAYS_PER_WEEK: u32 = 7;

fn main() {
    println!("{}", 100.0 * TAX_RATE);
    println!("{}", DAYS_PER_WEEK * 4);
}
```

```sortie
20
28
```

Line by line:

- the two `const` lines are written at the very top, outside `main`: the whole program
  can use them;
- `100.0 * TAX_RATE` computes 100 × 0.2, which is `20`;
- `DAYS_PER_WEEK * 4` gives the number of days in 4 weeks: `28`.

**Your turn:** write a program with a constant `TICKET_PRICE`, of type `f64`, that
holds `2.5`. Print the price of 4 tickets.

> **Solution**
>
> ```rust
> const TICKET_PRICE: f64 = 2.5;
>
> fn main() {
>     println!("{}", TICKET_PRICE * 4.0);
> }
> ```
>
> ```sortie
> 10
> ```

## Common mistakes

### Changing a variable without mut

```rust norun
fn main() {
    let score = 10;
    score = 20;
    println!("{score}");
}
```

```text
error[E0384]: cannot assign twice to immutable variable `score`
 --> src/main.rs:3:5
  |
2 |     let score = 10;
  |         ----- first assignment to `score`
3 |     score = 20;
  |     ^^^^^^^^^^ cannot assign twice to immutable variable
help: consider making this binding mutable
```

Fix: `let mut score = 10;`.

### Mixing integers and decimal numbers

```rust norun
fn main() {
    let total = 5 + 2.5;
    println!("{total}");
}
```

```text
error[E0277]: cannot add a float to an integer
 --> src/main.rs:2:19
  |
2 |     let total = 5 + 2.5;
  |                   ^ no implementation for `{integer} + {float}`
```

Fix: write `5.0 + 2.5`, or convert a variable with `as f64`.

### A number too big for its type

```rust norun
fn main() {
    let small: u8 = 300;
    println!("{small}");
}
```

```text
error: literal out of range for `u8`
 --> src/main.rs:2:21
  |
2 |     let small: u8 = 300;
  |                     ^^^
  |
  = note: the literal `300` does not fit into the type `u8` whose range is `0..=255`
```

Fix: pick a bigger type, like `u32` or `i32`.

### A constant without a type

```rust norun
const MAX = 10;

fn main() {
    println!("{MAX}");
}
```

```text
error: missing type for `const` item
 --> src/main.rs:1:10
  |
1 | const MAX = 10;
  |          ^ help: provide a type for the constant: `: i32`
```

Fix: `const MAX: i32 = 10;`.

## Test yourself

**1.** Does this code compile? If not, how do you fix it?

```rust norun
let points = 0;
points += 5;
```

> **Answer**
>
> No: `points` is not changeable. Write `let mut points = 0;`.

**2.** What are `9 / 4` and `9.0 / 4.0`?

> **Answer**
>
> `9 / 4` is `2` (integer division) and `9.0 / 4.0` is `2.25`.

**3.** How do you print the average of `total: i32 = 7` over `2` students, with decimals?

> **Answer**
>
> Convert with `as` before dividing:
>
> ```rust
> let total: i32 = 7;
> println!("{}", total as f64 / 2.0);
> ```
>
> ```sortie
> 3.5
> ```

## Summary

- `let` creates a variable that cannot change; `let mut` makes it changeable.
- The simple types: `i32`, `u32`, `f64`, `bool`, `char`; `as` converts a number.
- Each integer has limits (`u8`: 0 to 255); going past them is an error.
- A tuple `(a, b)` groups a few values, read with `.0`, `.1`.
- `println!("{x}")` prints, `format!` builds a `String`.
- `const` creates a constant, in CAPITAL LETTERS, with its type.

The same chapter exists for [Ruby](../ruby/premiers-pas.md) and
[Rythme](../rythme/premiers-pas.md).

Next: [Numbers and calculations](nombres.md)
