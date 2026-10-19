# Solved exercises

This page offers twelve exercises, from easiest to hardest, to practice the whole Rust track, each with its solution.

## How to work

Try each exercise on your own, in a project created with `cargo new exercises`. Look at
the hint if you get stuck, then at the solution to compare.

- ★☆☆: easy, a few lines;
- ★★☆: medium, you need to combine several ideas;
- ★★★: harder, take your time.

> **Tip:** your solution can be different and just as correct. If it compiles and
> prints the right result, you win.

## The basics

### Exercise 1: Introduce yourself

Level: ★☆☆

Create two variables, `first_name` and `age`, then print the sentence
`My name is Zoé and I will be 26 next year.` (compute `26` from `age`).

> **Solution**
>
> ```rust
> fn main() {
>     let first_name = "Zoé";
>     let age = 25;
>     println!("My name is {first_name} and I will be {} next year.", age + 1);
> }
> ```
>
> ```sortie
> My name is Zoé and I will be 26 next year.
> ```

### Exercise 2: Even or odd

Level: ★☆☆

For each number from 1 to 5, print whether it is even or odd.

> **Hint:** a number is even if the remainder of its division by 2 is 0: `n % 2 == 0`.

> **Solution**
>
> ```rust
> fn main() {
>     for n in 1..=5 {
>         if n % 2 == 0 {
>             println!("{n} is even");
>         } else {
>             println!("{n} is odd");
>         }
>     }
> }
> ```
>
> ```sortie
> 1 is odd
> 2 is even
> 3 is odd
> 4 is even
> 5 is odd
> ```

### Exercise 3: The multiplication table

Level: ★☆☆

Print the table of 7, from `7 x 1 = 7` to `7 x 5 = 35`.

> **Solution**
>
> ```rust
> fn main() {
>     let table = 7;
>     for i in 1..=5 {
>         println!("{table} x {i} = {}", table * i);
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

### Exercise 4: A conversion function

Level: ★☆☆

Write a function `celsius_to_fahrenheit(c: f64) -> f64`. The formula is
`c * 9 / 5 + 32`. Print the result for 0, 20 and 100 degrees.

> **Hint:** with `f64` values, write `9.0` and `5.0`, not `9` and `5`.

> **Solution**
>
> ```rust
> fn celsius_to_fahrenheit(c: f64) -> f64 {
>     c * 9.0 / 5.0 + 32.0
> }
>
> fn main() {
>     for c in [0.0, 20.0, 100.0] {
>         println!("{c} °C = {} °F", celsius_to_fahrenheit(c));
>     }
> }
> ```
>
> ```sortie
> 0 °C = 32 °F
> 20 °C = 68 °F
> 100 °C = 212 °F
> ```

## Lists and text

### Exercise 5: The average grade

Level: ★★☆

In `let grades = vec![12, 15, 9, 18];`, compute the highest grade and the average
(an `f64`).

> **Hint:** `iter().max()` returns an `Option`. For the average, convert with
> `as f64`.

> **Solution**
>
> ```rust
> fn main() {
>     let grades = vec![12, 15, 9, 18];
>     let best = grades.iter().max().unwrap();
>     let sum: i32 = grades.iter().sum();
>     let average = sum as f64 / grades.len() as f64;
>     println!("Best grade: {best}");
>     println!("Average: {average}");
> }
> ```
>
> ```sortie
> Best grade: 18
> Average: 13.5
> ```

### Exercise 6: Count the vowels

Level: ★★☆

Write a function `vowels(text: &str) -> usize` that counts the vowels in a text
(without accents, lowercase or uppercase).

> **Hint:** go through `text.to_lowercase().chars()` and test whether `"aeiou"`
> contains the character.

> **Solution**
>
> ```rust
> fn vowels(text: &str) -> usize {
>     let mut total = 0;
>     for c in text.to_lowercase().chars() {
>         if "aeiou".contains(c) {
>             total += 1;
>         }
>     }
>     total
> }
>
> fn main() {
>     println!("{}", vowels("Hello"));
>     println!("{}", vowels("RUST"));
> }
> ```
>
> ```sortie
> 2
> 1
> ```

### Exercise 7: The palindrome

Level: ★★☆

Write a function `is_palindrome(word: &str) -> bool`: a palindrome reads the same in
both directions, like "kayak". Ignore uppercase.

> **Hint:** reverse the text with `chars().rev().collect::<String>()`.

> **Solution**
>
> ```rust
> fn is_palindrome(word: &str) -> bool {
>     let word = word.to_lowercase();
>     let reversed: String = word.chars().rev().collect();
>     word == reversed
> }
>
> fn main() {
>     for word in ["Kayak", "Rust", "noon"] {
>         println!("{word}: {}", is_palindrome(word));
>     }
> }
> ```
>
> ```sortie
> Kayak: true
> Rust: false
> noon: true
> ```

### Exercise 8: Count the words

Level: ★★☆

Count how many times each word appears in `"one two one three two one"`.
Print the words in alphabetical order.

> **Hint:** use a `HashMap` and `entry(word).or_insert(0)`. The order of a
> `HashMap` is random: put the pairs in a `Vec` and sort it.

> **Solution**
>
> ```rust
> use std::collections::HashMap;
>
> fn main() {
>     let text = "one two one three two one";
>     let mut counts = HashMap::new();
>     for word in text.split_whitespace() {
>         *counts.entry(word).or_insert(0) += 1;
>     }
>     let mut pairs: Vec<_> = counts.into_iter().collect();
>     pairs.sort();
>     for (word, n) in pairs {
>         println!("{word}: {n}");
>     }
> }
> ```
>
> ```sortie
> one: 3
> three: 1
> two: 2
> ```

## Ownership, types and errors

### Exercise 9: Borrow instead of take

Level: ★★☆

This program does not compile, because `name` is moved into the function:

```rust norun
fn greet(name: String) {
    println!("Hello {name}");
}

fn main() {
    let name = String::from("Inès");
    greet(name);
    greet(name); // error: use of moved value
}
```

Fix it by **borrowing** the text, without using `clone`.

> **Hint:** the parameter becomes `&str`, and the call `greet(&name)`.

> **Solution**
>
> ```rust
> fn greet(name: &str) {
>     println!("Hello {name}");
> }
>
> fn main() {
>     let name = String::from("Inès");
>     greet(&name);
>     greet(&name);
> }
> ```
>
> ```sortie
> Hello Inès
> Hello Inès
> ```

### Exercise 10: A bank account

Level: ★★☆

Create a struct `Account` with a field `balance: i32`, and two methods:
`deposit(&mut self, amount: i32)` and `show(&self)`. Deposit 50 then 20 into an
account that starts at 0.

> **Solution**
>
> ```rust
> struct Account {
>     balance: i32,
> }
>
> impl Account {
>     fn deposit(&mut self, amount: i32) {
>         self.balance += amount;
>     }
>
>     fn show(&self) {
>         println!("Balance: {} €", self.balance);
>     }
> }
>
> fn main() {
>     let mut account = Account { balance: 0 };
>     account.deposit(50);
>     account.deposit(20);
>     account.show();
> }
> ```
>
> ```sortie
> Balance: 70 €
> ```

### Exercise 11: A safe division

Level: ★★★

Write `divide(a: i32, b: i32) -> Option<i32>` that returns `None` if `b` is 0.
Then write `read_number(text: &str) -> Result<i32, String>` that returns a clear error
message if the text is not a number. Test both.

> **Hint:** for `read_number`, use `match text.parse::<i32>()` with an `Ok(n)` case
> and an `Err(_)` case.

> **Solution**
>
> ```rust
> fn divide(a: i32, b: i32) -> Option<i32> {
>     if b == 0 { None } else { Some(a / b) }
> }
>
> fn read_number(text: &str) -> Result<i32, String> {
>     match text.parse::<i32>() {
>         Ok(n) => Ok(n),
>         Err(_) => Err(format!("\"{text}\" is not a number")),
>     }
> }
>
> fn main() {
>     println!("{:?}", divide(10, 2));
>     println!("{:?}", divide(1, 0));
>     match read_number("12") {
>         Ok(n) => println!("Read: {n}"),
>         Err(e) => println!("Error: {e}"),
>     }
>     match read_number("twelve") {
>         Ok(n) => println!("Read: {n}"),
>         Err(e) => println!("Error: {e}"),
>     }
> }
> ```
>
> ```sortie
> Some(5)
> None
> Read: 12
> Error: "twelve" is not a number
> ```

### Exercise 12: Shapes with a trait

Level: ★★★

Create a trait `Shape` with a method `area(&self) -> f64`. Implement it for
`Square { side: f64 }` and `Rectangle { width: f64, height: f64 }`. Put a square
with side 3 and a 2 by 5 rectangle in a `Vec<Box<dyn Shape>>`, then print the sum of
the areas.

> **Hint:** read [A list of different objects](generiques.md) again.

> **Solution**
>
> ```rust
> trait Shape {
>     fn area(&self) -> f64;
> }
>
> struct Square {
>     side: f64,
> }
>
> struct Rectangle {
>     width: f64,
>     height: f64,
> }
>
> impl Shape for Square {
>     fn area(&self) -> f64 {
>         self.side * self.side
>     }
> }
>
> impl Shape for Rectangle {
>     fn area(&self) -> f64 {
>         self.width * self.height
>     }
> }
>
> fn main() {
>     let shapes: Vec<Box<dyn Shape>> = vec![
>         Box::new(Square { side: 3.0 }),
>         Box::new(Rectangle { width: 2.0, height: 5.0 }),
>     ];
>     let mut total = 0.0;
>     for s in &shapes {
>         println!("Area: {}", s.area());
>         total += s.area();
>     }
>     println!("Total: {total}");
> }
> ```
>
> ```sortie
> Area: 9
> Area: 10
> Total: 19
> ```

## Summary

- Start with the ★☆☆ exercises, then move up in difficulty.
- Read the compiler messages: they often show the fix.
- Test your code with several values, including edge cases (0, empty text).
- To review an idea, the [cheat sheet](aide-memoire.md) sums up the whole track.

To keep going, try the same exercises in [Ruby](../ruby/exercices.md) or in
[Rythme](../rythme/exercices.md).

Next: [solved exercises: second set](exercices-2.md)
