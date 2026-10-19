# Numbers and calculations

Rust offers several number types, integers and decimals: this chapter shows how to choose them, calculate with them, convert them and display them nicely.

> **Goals:**
>
> - choose the right type of integer or decimal number;
> - calculate with operators, integer division and the remainder `%`;
> - avoid overflows with `checked_`, `wrapping_` and `saturating_`;
> - convert a number with `as`, `From`, `TryFrom` and `parse`;
> - display a number with decimals, spaces or zeros.

## Integers

A program counts all the time: an age, a score, the items in a shopping cart. Rust
wants to know in advance how much room to set aside for each number, just as you pick a
box of the right size to store something.

An [integer](../debuter/glossaire.md#integer) is a number without a decimal point: `3`,
`-12`, `2026`. Rust has several integer types. Their name tells you two things:

- the letter: `i` for a **signed** integer (it can be negative), `u` for an
  **unsigned** integer (never negative);
- the number: the size in bits. The more bits, the bigger the number can be.

| Type | Smallest value | Largest value |
|------|----------------|---------------|
| `i8` | -128 | 127 |
| `u8` | 0 | 255 |
| `i16` | -32,768 | 32,767 |
| `u16` | 0 | 65,535 |
| `i32` | -2,147,483,648 | 2,147,483,647 |
| `u32` | 0 | 4,294,967,295 |
| `i64` | about -9.2 × 10¹⁸ | about 9.2 × 10¹⁸ |
| `u64` | 0 | about 1.8 × 10¹⁹ |
| `i128`, `u128` | even bigger | (about 38 digits) |

When you do not write a type, Rust chooses `i32`. It is a good default choice.

```rust
let year = 2026;           // i32, chosen by Rust
let age: u8 = 12;          // an age is never negative nor very big
let population: u64 = 8_100_000_000;
println!("{year} {age} {population}");
```

```sortie
2026 12 8100000000
```

Line by line:

- `let year = 2026;` creates the variable `year`, which holds 2026. No type is written,
  so Rust chooses `i32`;
- `let age: u8 = 12;`: the `: u8` after the name sets the type. 12 easily fits between
  0 and 255;
- `population` is a `u64`: 8 billion would not fit in an `i32`, which stops at about
  2.1 billion;
- `println!` replaces each `{name}` with the value of the variable and prints it all.

> **Tip:** the underscore `_` separates digits to make them easier to read:
> `8_100_000_000` is the same number as `8100000000`.

### usize: the type for positions

`usize` is an unsigned integer whose size depends on the computer (64 bits on a recent
computer). It is the type of **positions** and **lengths**: `len()` returns a `usize`,
and a vector index must be a `usize`.

```rust
let fruits = vec!["apple", "kiwi", "pear"];
let n: usize = fruits.len();
let i: usize = 1;
println!("{n} fruits, the second one is {}", fruits[i]);
```

```sortie
3 fruits, the second one is kiwi
```

## Decimal numbers

Integers are not enough for a price of 19.99, a height of 1.75 m or an average of
13.5. For these values with a part after the decimal point, you need another type.

A [float](../debuter/glossaire.md#float) is a number with a decimal part. In Rust,
you write a **point**: `3.14`. There are two types:

- `f64`: precise (about 15 digits), chosen by default;
- `f32`: less precise (about 7 digits), but half the size in memory.

```rust
let price = 19.99;         // f64
let height: f32 = 1.75;
println!("{price} {height}");
```

```sortie
19.99 1.75
```

### Floats are not exact

The computer stores numbers in binary. Many decimal numbers, like `0.1`, have no exact
binary form: it is like `1/3 = 0.333…` in decimal. A tiny error remains:

```rust
let sum: f64 = 0.1 + 0.2;
println!("{sum}");
println!("{}", sum == 0.3);
println!("{}", (sum - 0.3).abs() < 1e-9);
```

```sortie
0.30000000000000004
false
true
```

> **Remember:** never compare two floats with `==`. Check instead that the difference
> between them is very small. For money, count in cents with an integer.

**Your turn:** write a program that stores a number of days, `7`, and a temperature,
`21.5`, in two variables, then prints "7 days at 21.5 degrees".

> **Solution**
>
> ```rust
> let days = 7;
> let temperature = 21.5;
> println!("{days} days at {temperature} degrees");
> ```
>
> ```sortie
> 7 days at 21.5 degrees
> ```

## Operators

Calculating is a computer's first job: the total of a cart, an average, the time left
before the end of a movie. Operators are the signs that write these calculations.

The usual [operators](../debuter/glossaire.md#operator) work like in math:

| Operator | Meaning | Example | Result |
|----------|---------|---------|--------|
| `+` | addition | `7 + 2` | `9` |
| `-` | subtraction | `7 - 2` | `5` |
| `*` | multiplication | `7 * 2` | `14` |
| `/` | division | `7.0 / 2.0` | `3.5` |
| `%` | remainder of the division | `7 % 2` | `1` |

The short forms `+=`, `-=`, `*=`, `/=` change a `mut` variable:

```rust
let mut score = 10;
score += 5;
score *= 2;
println!("{score}");
```

```sortie
30
```

Line by line:

- `let mut score = 10;` creates `score`, which holds 10. `mut` allows it to change;
- `score += 5;` is a short form of `score = score + 5`: `score` now holds 15;
- `score *= 2;` multiplies by 2: 15 × 2 gives 30;
- `println!` prints the current value of `score`: 30.

### Integer division and remainder

Between two integers, `/` gives an **integer division**: the part after the decimal
point is thrown away. `%` gives the **remainder**. Think of 7 candies shared between 2
children: each one gets 3, and 1 is left over.

```rust
println!("{}", 7 / 2);
println!("{}", 7 % 2);
println!("{}", 7.0 / 2.0);
let minutes = 135;
println!("{} h {} min", minutes / 60, minutes % 60);
```

```sortie
3
1
3.5
2 h 15 min
```

> **Tip:** `n % 2 == 0` is the classic way to test that a number is even.

**Your turn:** write a program that stores `200` seconds in a variable, then prints
this duration in minutes and seconds: "3 min 20 s".

> **Solution**
>
> ```rust
> let seconds = 200;
> println!("{} min {} s", seconds / 60, seconds % 60);
> ```
>
> ```sortie
> 3 min 20 s
> ```

## Overflow

A number type is like a box of fixed size: if you put too much in it, it overflows.
This section shows how to keep a calculation from giving a wrong result or stopping
your program.

A `u8` does not go above 255. What happens if a calculation goes further? This is
called an **overflow**.

- In debug mode (`cargo run`), the program **stops** with a
  [panic](../debuter/glossaire.md#panic): `attempt to add with overflow`.
- In optimized mode (`cargo run --release`), the number **wraps around**: 255 + 1
  gives 0, like a car's odometer.

To avoid depending on the mode, say yourself what you want with three families of
methods:

| Method | If it overflows… | `250u8 + 10` |
|--------|------------------|--------------|
| `checked_add` | returns `None` | `None` |
| `wrapping_add` | wraps around | `4` |
| `saturating_add` | stops at the maximum | `255` |

```rust
let x: u8 = 250;
println!("{:?}", x.checked_add(10));
println!("{:?}", x.checked_add(3));
println!("{}", x.wrapping_add(10));
println!("{}", x.saturating_add(10));
let lives: u8 = 1;
println!("{}", lives.saturating_sub(5));
```

```sortie
None
Some(253)
4
255
0
```

Line by line:

- `x` holds 250, and a `u8` does not go above 255;
- `checked_add(10)`: 260 does not fit, so the method returns `None` ("nothing").
  `checked_add(3)`: 253 fits, so it returns `Some(253)`. `{:?}` is used to print these
  values;
- `wrapping_add(10)` wraps around: after 255 it starts again from 0, and 260 - 256
  gives 4;
- `saturating_add(10)` stays stuck at the maximum, 255;
- `lives` holds 1. Removing 5 would give -4, which is impossible for a `u8`:
  `saturating_sub` stops at 0.

The same methods exist for `sub`, `mul` and `div`: `checked_mul`,
`saturating_sub`… `checked_` is the safest: it returns an
[Option](types.md) that you must handle.

**Your turn:** write a program that stores `3` in a variable `lives` of type `u8`.
Print what is left after losing 5 lives, without going below zero, then the result of
`lives.checked_sub(5)`.

> **Solution**
>
> ```rust
> let lives: u8 = 3;
> println!("{}", lives.saturating_sub(5));
> println!("{:?}", lives.checked_sub(5));
> ```
>
> ```sortie
> 0
> None
> ```

## Converting a number

Rust **never** converts a number on its own. To add an `i32` and an `f64`, you must
convert one of them.

### With as: fast, but no safety net

`as` always converts, even if it loses information: the decimal part is cut off, and a
number that is too big gets distorted.

```rust
let n: i32 = 7;
let x = n as f64 / 2.0;
println!("{x}");
println!("{}", 3.99_f64 as i32);
println!("{}", 300_i32 as u8);
println!("{}", 'A' as u32);
```

```sortie
3.5
3
44
65
```

Line by line:

- `n as f64` turns 7 into 7.0: the division happens between two floats and gives 3.5;
- `3.99_f64` is the number 3.99 of type `f64` (the suffix attaches the type to the
  number). `as i32` cuts off the decimal part, without rounding: 3;
- `300_i32 as u8`: 300 does not fit in a `u8`, and `as` distorts it into 44;
- `'A' as u32` gives the number of the letter A in the character table: 65.

`300 as u8` gives `44` (300 - 256) without warning: only use `as` when you are sure the
number fits.

### With From and TryFrom: no surprises

`From` converts when it is **always possible** (from a small type to a bigger one).
`TryFrom` converts when it is **maybe possible**: it returns a
[Result](../debuter/glossaire.md#result).

```rust
let small: u8 = 200;
let big = i64::from(small);
println!("{big}");

let ok = u8::try_from(100_i32);
let too_big = u8::try_from(300_i32);
println!("{:?}", ok);
println!("{:?}", too_big.is_err());
```

```sortie
200
Ok(100)
true
```

### From text with parse

`parse` turns a text into a number. You must say which type you want, and handle the
possible failure (see [Turning a text into a number](texte.md#turning-a-text-into-a-number)):

```rust
let age: u32 = "42".parse().unwrap();
let height = "1.80".parse::<f64>().unwrap();
println!("{age} {height}");
match "twelve".parse::<i32>() {
    Ok(n) => println!("number: {n}"),
    Err(e) => println!("error: {e}"),
}
```

```sortie
42 1.8
error: invalid digit found in string
```

## Math methods

Rounding a price, computing a power, keeping a grade between 0 and 20: these
calculations come up often. Rust gives them to you ready-made, so you do not have to
write them yourself.

Numbers have methods. You call them with a dot:

```rust
println!("{}", 2_i32.pow(10));
println!("{}", 16.0_f64.sqrt());
println!("{}", (-5_i32).abs());
println!("{}", 3.max(8));
println!("{}", 3.min(8));
println!("{}", 15.clamp(0, 10));
println!("{}", 2.6_f64.round());
println!("{}", 2.6_f64.floor());
println!("{}", 2.1_f64.ceil());
```

```sortie
1024
4
5
8
3
10
3
2
3
```

- `pow`: power (integers); for floats, `powi` or `powf`.
- `sqrt`: square root (floats only).
- `clamp(min, max)`: keeps the number between two limits, like a volume between 0 and 10.
- `round`, `floor`, `ceil`: round to the nearest, down, up.

> **Warning:** write `(-5_i32).abs()` with parentheses. Without them,
> `-5_i32.abs()` reads as `-(5.abs())` and gives `-5`.

## Useful constants

> **Note:** this part can wait. Read it once the basics feel solid.

Rather than memorizing 2147483647 or 3.14159…, you can ask Rust for them. The name is
clearer, and there is no risk of a typo.

Each type knows its limits, and `std::f64::consts` contains the math constants:

```rust
use std::f64::consts::PI;

println!("{}", i32::MAX);
println!("{}", u8::MAX);
println!("{}", i64::MIN);
let radius = 2.0;
println!("{}", PI * radius * radius);
```

```sortie
2147483647
255
-9223372036854775808
12.566370614359172
```

Line by line:

- `use std::f64::consts::PI;` makes the constant `PI` available under this short name;
- `i32::MAX`, `u8::MAX` and `i64::MIN` give the largest or smallest value of each type;
- the last line computes the area of a disk of radius 2: π × 2 × 2.

## Displaying a number nicely

A result like `12.666666666666666` is hard to read. On a receipt, you want `12.67`,
and prices neatly lined up in a column. Formats are made for this.

Inside the curly braces of `println!`, after a colon `:`, you can adjust the display:

| Format | Effect | Example | Result |
|--------|--------|---------|--------|
| `{:.2}` | 2 digits after the decimal point | `3.14159` | `3.14` |
| `{:>8}` | right-aligned over 8 characters | `42` | `      42` |
| `{:<8}` | left-aligned over 8 characters | `42` | `42      ` |
| `{:08.3}` | 8 characters, leading zeros, 3 decimals | `3.14159` | `0003.142` |
| `{:e}` | scientific notation | `1500.0` | `1.5e3` |

```rust
let pi = 3.14159;
println!("[{:.2}]", pi);
println!("[{:>8}]", 42);
println!("[{:<8}]", 42);
println!("[{:08.3}]", pi);
println!("[{:e}]", 1500.0);
println!("[{pi:>10.1}]");
```

```sortie
[3.14]
[      42]
[42      ]
[0003.142]
[1.5e3]
[       3.1]
```

Line by line:

- `{:.2}` keeps 2 digits after the decimal point, rounding: 3.14;
- `{:>8}` reserves 8 spaces and pushes 42 to the right; `{:<8}` pushes it to the left.
  The brackets `[ ]` are there so you can see the spaces;
- `{:08.3}`: 3 decimals, 8 characters in total, filled with zeros in front;
- in the last line, the variable name goes before the colon: `{pi:>10.1}`.

This is handy to align a price list:

```rust
let items = [("bread", 1.2), ("cheese", 4.75), ("apples", 12.0)];
for (name, price) in items {
    println!("{name:<10}{price:>7.2} €");
}
```

```sortie
bread        1.20 €
cheese       4.75 €
apples      12.00 €
```

**Your turn:** write a program that stores a total of points, `38`, and a number of
grades, `3`, in two integers. Print the average with two decimals: "Average: 12.67".

> **Solution**
>
> ```rust
> let total = 38;
> let count = 3;
> let average = total as f64 / count as f64;
> println!("Average: {average:.2}");
> ```
>
> ```sortie
> Average: 12.67
> ```

## Common mistakes

### Mixing an integer and a float

```rust norun
fn main() {
    let quantity: i32 = 3;
    let price = 2.5;
    let total = quantity * price;
    println!("{total}");
}
```

```text
error[E0277]: cannot multiply `i32` by `{float}`
 --> src/main.rs:4:26
  |
4 |     let total = quantity * price;
  |                          ^ no implementation for `i32 * {float}`
  |
  = help: the trait `Mul<{float}>` is not implemented for `i32`
```

Rust does not convert on its own. Write `quantity as f64 * price`.

### A number too big for its type

```rust norun
fn main() {
    let age: u8 = 300;
    println!("{age}");
}
```

```text
error: literal out of range for `u8`
 --> src/main.rs:2:19
  |
2 |     let age: u8 = 300;
  |                   ^^^
  |
  = note: the literal `300` does not fit into the type `u8` whose range is `0..=255`
  = note: `#[deny(overflowing_literals)]` on by default
```

A `u8` stops at 255. Choose a bigger type: `u16` or `i32`.

### A calculation that overflows

```rust norun
fn main() {
    let x: u8 = 255;
    let y = x + 1;
    println!("{y}");
}
```

```text
error: this arithmetic operation will overflow
 --> src/main.rs:3:13
  |
3 |     let y = x + 1;
  |             ^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
  |
  = note: `#[deny(arithmetic_overflow)]` on by default
```

Here, the compiler sees the problem in advance. Choose a bigger type, or say what you
want: `x.checked_add(1)` or `x.saturating_add(1)`.

### parse without a type

```rust norun
fn main() {
    let n = "42".parse().unwrap();
    println!("{n}");
}
```

```text
error[E0284]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let n = "42".parse().unwrap();
  |         ^        ----- type must be known at this point
  |
  = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `n` an explicit type
  |
2 |     let n: /* Type */ = "42".parse().unwrap();
  |          ++++++++++++
```

`parse` can produce an `i32`, a `u8`, an `f64`… Say which one: `let n: i32 = …` or
`"42".parse::<i32>()`.

## Test yourself

**1.** What are `17 / 5` and `17 % 5`?

> **Answer**
>
> `3` and `2`: 17 = 5 × 3 + 2. Between two integers, `/` throws away the decimal part.

**2.** Which type should you choose for a vector index? And for a temperature
with decimals?

> **Answer**
>
> `usize` for an index; `f64` for a temperature.

**3.** How do you display `2.0 / 3.0` with exactly two decimals?

> **Answer**
>
> With the format `{:.2}`:
>
> ```rust
> println!("{:.2}", 2.0 / 3.0);
> ```
>
> ```sortie
> 0.67
> ```

## Summary

- `i32` and `f64` are the default types; `usize` is used for positions and lengths.
- Between integers, `/` is an integer division and `%` gives the remainder.
- Floats are not exact: do not compare them with `==`.
- `checked_`, `wrapping_` and `saturating_` say what to do in case of overflow.
- `as` converts with no safety net; `From`, `TryFrom` and `parse` are safer.
- `{:.2}`, `{:>8}` and `{:08.3}` adjust how a number is displayed.

Next: [Conditions and loops](conditions-boucles.md)
