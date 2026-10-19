# Exercises to get started

This page offers fifteen small, very guided exercises, all easy, to practice the basics of Rust: variables, numbers, conditions, loops, functions, text, lists and maps.

## How to do these exercises

Each exercise is short. Take them in order: each one uses a little more than the
previous one.

1. **Read** the instructions and the expected output.
2. **Try on paper**: write the steps in plain English, line by line.
3. **Write the code** in a project created with `cargo new exercises`, then run it
   with `cargo run`.
4. **Compare** your output with the expected output, character by character.
5. Stuck? Read **the first hint**, then the second one. Look at the solution last.

> **Tip:** if the compiler shows an error, read the `help:` line. It often contains
> the fix. The page [Beginner traps](pieges.md) explains the most common errors.

Your solution can be different from ours and just as correct. If it compiles and
displays the right output, you win.

## Display and calculate

### Exercise 1: Say hello

Level: ★☆☆

Write a program that displays two lines: `Hello!` then `I am learning Rust.`

Expected output:

```text
Hello!
I am learning Rust.
```

> **Hint:** a line is displayed with `println!("…");`. Do not forget the `!`.

> **Solution**
>
> ```rust
> fn main() {
>     println!("Hello!");
>     println!("I am learning Rust.");
> }
> ```
>
> ```sortie
> Hello!
> I am learning Rust.
> ```
>
> Why it works: each `println!` displays its text, then goes to the next line.

### Exercise 2: In ten years

Level: ★☆☆

Create a variable `name` set to `"Léa"` and a variable `age` set to `14`. Display her
age in ten years.

Expected output:

```text
Léa will be 24 in ten years.
```

> **Hint:** create the variables with `let name = "Léa";` and `let age = 14;`.

> **Hint:** the calculation can go into another variable:
> `let later = age + 10;`. Then display it with `{later}`.

> **Solution**
>
> ```rust
> fn main() {
>     let name = "Léa";
>     let age = 14;
>     let later = age + 10;
>     println!("{name} will be {later} in ten years.");
> }
> ```
>
> ```sortie
> Léa will be 24 in ten years.
> ```
>
> Why it works: `{name}` and `{later}` are replaced by the value of the variables at
> the moment of display.

### Exercise 3: A counter that goes up

Level: ★☆☆

Create a variable `step` set to 0. Add 100 to it three times, displaying its value
after each addition.

Expected output:

```text
100
200
300
```

> **Hint:** a variable that changes must be declared with `let mut`.

> **Hint:** to add 100, write `step += 100;` (it is the same as
> `step = step + 100;`).

> **Solution**
>
> ```rust
> fn main() {
>     let mut step = 0;
>     step += 100;
>     println!("{step}");
>     step += 100;
>     println!("{step}");
>     step += 100;
>     println!("{step}");
> }
> ```
>
> ```sortie
> 100
> 200
> 300
> ```
>
> Why it works: `mut` allows the variable to change. Without it, Rust refuses the
> second line.

### Exercise 4: Hours and minutes

Level: ★☆☆

A movie lasts 135 minutes. Display its length in hours and minutes.

Expected output:

```text
2 h 15 min
```

> **Hint:** one hour is 60 minutes. How many times does 60 go into 135?

> **Hint:** between two integers, `/` gives the whole number of times (`135 / 60`) and
> `%` gives what is left (`135 % 60`).

> **Solution**
>
> ```rust
> fn main() {
>     let length = 135;
>     let hours = length / 60;
>     let minutes = length % 60;
>     println!("{hours} h {minutes} min");
> }
> ```
>
> ```sortie
> 2 h 15 min
> ```
>
> Why it works: 60 goes 2 times into 135 (that makes 120), and 135 − 120 = 15 minutes
> are left.

### Exercise 5: Celsius to Fahrenheit

Level: ★☆☆

Convert 20 °C to degrees Fahrenheit. The formula is: `F = C × 9 / 5 + 32`.

Expected output:

```text
20 °C = 68 °F
```

> **Hint:** use decimal numbers (`f64`): write `20.0`, `9.0`, `5.0` and `32.0`.

> **Solution**
>
> ```rust
> fn main() {
>     let celsius = 20.0;
>     let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
>     println!("{celsius} °C = {fahrenheit} °F");
> }
> ```
>
> ```sortie
> 20 °C = 68 °F
> ```
>
> Why it works: 20 × 9 = 180, 180 / 5 = 36, 36 + 32 = 68. With `f64` values, the
> division does not cut off the decimals. Rust displays `20.0` as `20`.

## Decide and repeat

### Exercise 6: Even or odd

Level: ★☆☆

Create a variable `n` set to 7. Display `7 is odd` (or `even` if you change the
number).

Expected output:

```text
7 is odd
```

> **Hint:** a number is even when the remainder of its division by 2 is 0:
> `n % 2 == 0`.

> **Hint:** use `if … { … } else { … }`.

> **Solution**
>
> ```rust
> fn main() {
>     let n = 7;
>     if n % 2 == 0 {
>         println!("{n} is even");
>     } else {
>         println!("{n} is odd");
>     }
> }
> ```
>
> ```sortie
> 7 is odd
> ```
>
> Why it works: `7 % 2` is 1, so the condition is false and the `else` block is the
> one that runs.

### Exercise 7: The 7 times table

Level: ★☆☆

Display the first five lines of the 7 times table.

Expected output:

```text
7 x 1 = 7
7 x 2 = 14
7 x 3 = 21
7 x 4 = 28
7 x 5 = 35
```

> **Hint:** a loop `for i in 1..=5` makes `i` take the values 1, 2, 3, 4 and 5.

> **Solution**
>
> ```rust
> fn main() {
>     for i in 1..=5 {
>         println!("7 x {i} = {}", 7 * i);
>     }
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
>
> Why it works: the `=` in `1..=5` includes the 5. The empty `{}` are replaced by the
> calculation written after the comma.

### Exercise 8: The sum from 1 to 100

Level: ★☆☆

Calculate 1 + 2 + 3 + … + 100 with a loop, and display the result.

Expected output:

```text
5050
```

> **Hint:** you need a variable `total`, which starts at 0 and changes: `let mut`.

> **Hint:** in the loop `for n in 1..=100`, add `n` to the total. Here are the first
> rounds:
>
> | Round | n | total after |
> |-------|---|-------------|
> | 1 | 1 | 1 |
> | 2 | 2 | 3 |
> | 3 | 3 | 6 |

> **Solution**
>
> ```rust
> fn main() {
>     let mut total = 0;
>     for n in 1..=100 {
>         total += n;
>     }
>     println!("{total}");
> }
> ```
>
> ```sortie
> 5050
> ```
>
> Why it works: `total` is created **before** the loop, only once. Each round adds
> the next number to it.

### Exercise 9: The day of the week

Level: ★☆☆

Create a variable `day` set to 3. With `match`, display the name of the day: 1 for
Monday, 2 for Tuesday, 3 for Wednesday. For any other number, display `other day`.

Expected output:

```text
Wednesday
```

> **Hint:** a `match` lists cases: `1 => println!("Monday"),`.

> **Hint:** the `_ =>` case catches "everything else". Rust requires it here, because
> `day` could be any number.

> **Solution**
>
> ```rust
> fn main() {
>     let day = 3;
>     match day {
>         1 => println!("Monday"),
>         2 => println!("Tuesday"),
>         3 => println!("Wednesday"),
>         _ => println!("other day"),
>     }
> }
> ```
>
> ```sortie
> Wednesday
> ```
>
> Why it works: Rust tries the cases from top to bottom and runs the first one that
> matches.

## Write functions

### Exercise 10: The square

Level: ★☆☆

Write a function `square` that receives an `i32` and returns that number multiplied
by itself. Display `square(4)` and `square(9)`.

Expected output:

```text
16
81
```

> **Hint:** the first line is written `fn square(x: i32) -> i32 {`.

> **Hint:** the last line of the function is the returned value, **without** a
> semicolon: `x * x`.

> **Solution**
>
> ```rust
> fn square(x: i32) -> i32 {
>     x * x
> }
>
> fn main() {
>     println!("{}", square(4));
>     println!("{}", square(9));
> }
> ```
>
> ```sortie
> 16
> 81
> ```
>
> Why it works: `-> i32` announces what the function returns, and `x * x` without a
> semicolon is that value.

### Exercise 11: Adult or not

Level: ★☆☆

Write a function `is_adult` that receives an age (`u32`) and returns `true` if the
age is 18 or more. Test it with 15 and 18.

Expected output:

```text
age 15: false
age 18: true
```

> **Hint:** the function returns a `bool`: `fn is_adult(age: u32) -> bool`.

> **Hint:** a comparison already gives `true` or `false`. The body of the function
> fits on one line: `age >= 18`.

> **Solution**
>
> ```rust
> fn is_adult(age: u32) -> bool {
>     age >= 18
> }
>
> fn main() {
>     println!("age 15: {}", is_adult(15));
>     println!("age 18: {}", is_adult(18));
> }
> ```
>
> ```sortie
> age 15: false
> age 18: true
> ```
>
> Why it works: `>=` means "greater than or equal to". No need for `if`: the
> comparison is already the answer.

## Play with text

### Exercise 12: Shout a word

Level: ★☆☆

For the word `"hello"`, display its number of letters, then the word in capital
letters.

Expected output:

```text
5 letters
HELLO
```

> **Hint:** `.chars().count()` counts the letters of a text.

> **Hint:** `.to_uppercase()` returns the text in capital letters.

> **Solution**
>
> ```rust
> fn main() {
>     let word = "hello";
>     println!("{} letters", word.chars().count());
>     println!("{}", word.to_uppercase());
> }
> ```
>
> ```sortie
> 5 letters
> HELLO
> ```
>
> Why it works: `.chars()` goes through the letters one by one, and `.count()` counts
> them. With `"café"`, `.chars().count()` correctly gives 4.

### Exercise 13: Build a sentence

Level: ★☆☆

Start from an empty `String`. Add `"Rust"` to it, then `" is"`, then `" fast"`.
Display the sentence and its length.

Expected output:

```text
Rust is fast
12
```

> **Hint:** an empty `String` is created with `String::new()`. It is going to change,
> so: `let mut sentence = String::new();`.

> **Hint:** `sentence.push_str("Rust");` adds text at the end.

> **Solution**
>
> ```rust
> fn main() {
>     let mut sentence = String::new();
>     sentence.push_str("Rust");
>     sentence.push_str(" is");
>     sentence.push_str(" fast");
>     println!("{sentence}");
>     println!("{}", sentence.len());
> }
> ```
>
> ```sortie
> Rust is fast
> 12
> ```
>
> Why it works: a `String` is a text that belongs to you, and it can grow. The spaces
> are part of the added texts: they count in the length.

## Lists and maps

### Exercise 14: The best grade

Level: ★☆☆

Here are some grades: `vec![12, 17, 9, 15]`. Display their sum, then the highest one.

Expected output:

```text
sum: 53
best: 17
```

> **Hint:** go through the list with `for n in &grades`. Keep two `mut` variables:
> `sum` (at 0) and `best` (at 0).

> **Hint:** in the loop: `sum += n;`, then `if *n > best { best = *n; }`. The star `*`
> reads the number through the `&`.
>
> | Round | n | sum | best |
> |-------|---|-----|------|
> | 1 | 12 | 12 | 12 |
> | 2 | 17 | 29 | 17 |
> | 3 | 9 | 38 | 17 |
> | 4 | 15 | 53 | 17 |

> **Solution**
>
> ```rust
> fn main() {
>     let grades = vec![12, 17, 9, 15];
>     let mut sum = 0;
>     let mut best = 0;
>     for n in &grades {
>         sum += n;
>         if *n > best {
>             best = *n;
>         }
>     }
>     println!("sum: {sum}");
>     println!("best: {best}");
> }
> ```
>
> ```sortie
> sum: 53
> best: 17
> ```
>
> Why it works: `best` remembers the highest grade seen so far. It only changes if
> the grade of the current round is higher. Rust can also do it on its own:
> `grades.iter().sum::<i32>()` and `grades.iter().max()`.

### Exercise 15: Count the fruit

Level: ★☆☆

Here is a basket: `["apple", "kiwi", "apple", "pear", "apple"]`. Count each fruit
with a `HashMap`, then display the number of apples and kiwis.

Expected output:

```text
apples: 3
kiwis: 1
```

> **Hint:** start with `use std::collections::HashMap;` and
> `let mut count = HashMap::new();`.

> **Hint:** in the loop, this line adds 1 to the fruit (and creates it at 0 if it
> does not exist): `*count.entry(fruit).or_insert(0) += 1;`.

> **Solution**
>
> ```rust
> use std::collections::HashMap;
>
> fn main() {
>     let basket = ["apple", "kiwi", "apple", "pear", "apple"];
>     let mut count = HashMap::new();
>     for fruit in basket {
>         *count.entry(fruit).or_insert(0) += 1;
>     }
>     println!("apples: {}", count["apple"]);
>     println!("kiwis: {}", count["kiwi"]);
> }
> ```
>
> ```sortie
> apples: 3
> kiwis: 1
> ```
>
> Why it works: the map links each fruit to a counter. We only display specific keys:
> so the (unpredictable) order of a `HashMap` is not a problem. More details in
> [HashMap, HashSet and BTreeMap](hashmap.md).

For harder exercises, covering the whole track, see the
[Solved exercises](exercices.md).

Next: [Ownership and borrowing](possession.md)
