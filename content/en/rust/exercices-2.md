# Solved exercises: second set

This page offers twelve new Rust exercises, from easiest to hardest, each with a hint, an explained solution and often a table that follows the program step by step.

## How to work

These exercises are **new**: they do not repeat those of
[Exercises to get started](exercices-faciles.md) or of
[Solved exercises](exercices.md). Do them in a project created with
`cargo new exercises2`, and run your code with `cargo run`.

1. Read the task and the expected output.
2. On paper, write the steps in plain English. Make a small trace table, as in
   [Following a program step by step](pas-a-pas.md).
3. Write the code, run it, compare the output character by character.
4. Stuck? Read the hint. Look at the solution last.

The levels:

- ★☆☆: easy, a few lines;
- ★★☆: medium, you need to combine several ideas;
- ★★★: harder, take your time and make the table on paper.

> **Tip:** your solution can be different and just as correct. If it compiles and
> prints the right output, you win.

## Warming up

### Exercise 1: The countdown

Level: ★☆☆

Print a countdown from 5 to 1, then `Liftoff!`.

Expected output:

```text
5
4
3
2
1
Liftoff!
```

> **Hint:** `(1..=5).rev()` gives the numbers from 1 to 5 **backwards**: 5, 4, 3, 2, 1.
> Don't forget the parentheses around `1..=5`.

> **Solution**
>
> ```rust
> fn main() {
>     for n in (1..=5).rev() {
>         println!("{n}");
>     }
>     println!("Liftoff!");
> }
> ```
>
> ```sortie
> 5
> 4
> 3
> 2
> 1
> Liftoff!
> ```
>
> | Round | n | Prints |
> |-------|---|--------|
> | 1 | 5 | 5 |
> | 2 | 4 | 4 |
> | 3 | 3 | 3 |
> | 4 | 2 | 2 |
> | 5 | 1 | 1 |
> | after | | Liftoff! |
>
> Why it works: `.rev()` flips the range, like reading a list starting from the
> bottom. The last `println!` is **after** the loop: it runs only once.

### Exercise 2: The largest of three

Level: ★☆☆

Write a function `largest(a: i32, b: i32, c: i32) -> i32` that returns the largest of
the three numbers, **without** using `max`. Test it with `(4, 9, 2)` and
`(7, 3, 7)`.

Expected output:

```text
9
7
```

> **Hint:** keep a "champion": `let mut big = a;`. If `b` is larger, `b` becomes the
> champion. Then do the same with `c`.

> **Solution**
>
> ```rust
> fn largest(a: i32, b: i32, c: i32) -> i32 {
>     let mut big = a;
>     if b > big {
>         big = b;
>     }
>     if c > big {
>         big = c;
>     }
>     big
> }
>
> fn main() {
>     println!("{}", largest(4, 9, 2));
>     println!("{}", largest(7, 3, 7));
> }
> ```
>
> ```sortie
> 9
> 7
> ```
>
> Why it works: it's like a tournament. The starting champion is `a`, and each
> newcomer challenges it. For `(4, 9, 2)`: `big` is 4, then 9 (9 > 4), then stays 9
> (2 is not larger than 9).

### Exercise 3: Fizz and Buzz

Level: ★☆☆

For each number from 1 to 15, print `Fizz` if it is divisible by 3, `Buzz` if it is
divisible by 5, `FizzBuzz` if it is divisible by both, and otherwise the number. Put
everything on a single line, separated by spaces.

Expected output:

```text
1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
```

> **Hint:** "divisible by 3" is written `n % 3 == 0`. Test the case `n % 15 == 0`
> **first**: otherwise, 15 would be caught by the "divisible by 3" case.

> **Hint:** `print!` (without `ln`) prints without going to a new line. Add an empty
> `println!()` at the end.

> **Solution**
>
> ```rust
> fn main() {
>     for n in 1..=15 {
>         if n % 15 == 0 {
>             print!("FizzBuzz");
>         } else if n % 3 == 0 {
>             print!("Fizz");
>         } else if n % 5 == 0 {
>             print!("Buzz");
>         } else {
>             print!("{n}");
>         }
>         if n < 15 {
>             print!(" ");
>         }
>     }
>     println!();
> }
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```
>
> Why it works: an `if … else if` stops at the **first** true test. The most precise
> case (15) must therefore come first. The test `n < 15` avoids an extra space at the
> end of the line.

### Exercise 4: The initials

Level: ★☆☆

Write a function `initials(name: &str) -> String` that returns the first letter of
each word, followed by a dot. Test with `"Ada Lovelace"` and
`"Grace Brewster Hopper"`.

Expected output:

```text
A.L.
G.B.H.
```

> **Hint:** `name.split_whitespace()` splits the text into words.
> `word.chars().next()` gives the first letter, inside an `Option`.

> **Hint:** start from `let mut result = String::new();`, then, for each word,
> `result.push(letter);` and `result.push('.');`.

> **Solution**
>
> ```rust
> fn initials(name: &str) -> String {
>     let mut result = String::new();
>     for word in name.split_whitespace() {
>         if let Some(letter) = word.chars().next() {
>             result.push(letter);
>             result.push('.');
>         }
>     }
>     result
> }
>
> fn main() {
>     println!("{}", initials("Ada Lovelace"));
>     println!("{}", initials("Grace Brewster Hopper"));
> }
> ```
>
> ```sortie
> A.L.
> G.B.H.
> ```
>
> Why it works: `chars().next()` returns `Some(letter)` if there is a letter, `None`
> if the word is empty. `if let Some(letter)` keeps only the first case. `push` adds
> **one** character (between `'…'`), `push_str` adds a text.

## Combining ideas

### Exercise 5: The sum of the digits

Level: ★★☆

Write a function `digit_sum(n: u32) -> u32` that adds up the digits of a number: for
4721, that makes 4 + 7 + 2 + 1 = 14. Don't go through text.

Expected output:

```text
14
9
```

(the second line is for `digit_sum(900)`).

> **Hint:** `n % 10` gives the **last** digit (4721 % 10 = 1). `n / 10` removes that
> last digit (4721 / 10 = 472).

> **Hint:** repeat with `while n > 0 { … }`. The parameter must change: write
> `mut n: u32` in the signature.

> **Solution**
>
> ```rust
> fn digit_sum(mut n: u32) -> u32 {
>     let mut sum = 0;
>     while n > 0 {
>         sum += n % 10;
>         n /= 10;
>     }
>     sum
> }
>
> fn main() {
>     println!("{}", digit_sum(4721));
>     println!("{}", digit_sum(900));
> }
> ```
>
> ```sortie
> 14
> 9
> ```
>
> | Round | n at the start | n % 10 | sum | n at the end |
> |-------|----------------|--------|-----|--------------|
> | 1 | 4721 | 1 | 1 | 472 |
> | 2 | 472 | 2 | 3 | 47 |
> | 3 | 47 | 7 | 10 | 4 |
> | 4 | 4 | 4 | 14 | 0 |
>
> Why it works: we "eat" the number from the right, one digit per round. When `n` is
> 0, there is nothing left to eat: the loop stops.

### Exercise 6: Reversing a list by hand

Level: ★★☆

Start from `let list = vec![1, 2, 3, 4];`. Build a **new** list in reverse order,
without using `reverse` or `rev`. Print both lists.

Expected output:

```text
[1, 2, 3, 4]
[4, 3, 2, 1]
```

> **Hint:** create `let mut reversed = Vec::new();`. For each element, insert it **at
> the start** of `reversed`: `reversed.insert(0, x);`.

> **Solution**
>
> ```rust
> fn main() {
>     let list = vec![1, 2, 3, 4];
>     let mut reversed = Vec::new();
>     for x in &list {
>         reversed.insert(0, *x);
>     }
>     println!("{list:?}");
>     println!("{reversed:?}");
> }
> ```
>
> ```sortie
> [1, 2, 3, 4]
> [4, 3, 2, 1]
> ```
>
> | Round | x | reversed after the round |
> |-------|---|--------------------------|
> | 1 | 1 | [1] |
> | 2 | 2 | [2, 1] |
> | 3 | 3 | [3, 2, 1] |
> | 4 | 4 | [4, 3, 2, 1] |
>
> Why it works: it's like stacking plates. Each new plate goes in front of the
> others. `&list` lends the list to the loop: it stays usable for the `println!`.
> `{:?}` prints a whole list.

### Exercise 7: The longest word

Level: ★★☆

Write a function `longest(sentence: &str) -> &str` that returns the longest word of a
sentence. If there is a tie, keep the first one. Test with
`"we ate crêpes at the café"`.

Expected output:

```text
crêpes (6 letters)
```

> **Hint:** keep a champion, as in exercise 2: `let mut best = "";`. Compare the
> lengths with `word.chars().count()`, not `len()`: `len()` counts bytes, and `ê`
> takes 2 of them.

> **Solution**
>
> ```rust
> fn longest(sentence: &str) -> &str {
>     let mut best = "";
>     for word in sentence.split_whitespace() {
>         if word.chars().count() > best.chars().count() {
>             best = word;
>         }
>     }
>     best
> }
>
> fn main() {
>     let word = longest("we ate crêpes at the café");
>     println!("{word} ({} letters)", word.chars().count());
> }
> ```
>
> ```sortie
> crêpes (6 letters)
> ```
>
> Why it works: the test is `>` and not `>=`, so a word of the same length does not
> replace the champion: the first one wins. The returned word is a piece of the
> sentence (a `&str`), with no copy.
>
> Try it: with `>=`, test `"cat dog"`. You get `dog` instead of `cat`.

### Exercise 8: The Fibonacci sequence

Level: ★★☆

In the Fibonacci sequence, each number is the sum of the two before it:
0, 1, 1, 2, 3, 5… Print the first 10 numbers on one line.

Expected output:

```text
0 1 1 2 3 5 8 13 21 34
```

> **Hint:** keep two variables `a` (the current number) and `b` (the next one). Each
> round, print `a`, then move forward: the new `a` is `b`, the new `b` is `a + b`.

> **Hint:** watch the order! Compute `let next = a + b;` **before** changing `a`.

> **Solution**
>
> ```rust
> fn main() {
>     let mut a: u64 = 0;
>     let mut b: u64 = 1;
>     let mut numbers = Vec::new();
>     for _ in 0..10 {
>         numbers.push(a.to_string());
>         let next = a + b;
>         a = b;
>         b = next;
>     }
>     println!("{}", numbers.join(" "));
> }
> ```
>
> ```sortie
> 0 1 1 2 3 5 8 13 21 34
> ```
>
> | Round | a (printed) | b | next = a + b | a after | b after |
> |-------|-------------|---|--------------|---------|---------|
> | 1 | 0 | 1 | 1 | 1 | 1 |
> | 2 | 1 | 1 | 2 | 1 | 2 |
> | 3 | 1 | 2 | 3 | 2 | 3 |
> | 4 | 2 | 3 | 5 | 3 | 5 |
> | 5 | 3 | 5 | 8 | 5 | 8 |
>
> Why it works: the two variables move forward together, like two walking feet.
> `next` keeps the result while we move `a`: without it, we would lose the old value
> of `a`. `join(" ")` glues the texts together with a space between them. The `_` in
> `for _ in` means "I don't use the round number".

### Exercise 9: The grades

Level: ★★☆

Write a function `grade_label(grade: u32) -> &'static str` that returns `"fail"`
(0 to 9), `"fair"` (10 to 13), `"good"` (14 to 15) or `"very good"` (16 to 20), and
`"invalid grade"` above that. Use `match` with ranges. Print the label of 8, 12, 15,
19 and 25.

Expected output:

```text
8: fail
12: fair
15: good
19: very good
25: invalid grade
```

> **Hint:** a `match` accepts inclusive ranges: `0..=9 => "fail",`.

> **Hint:** `&'static str` means "a text written directly in the program", which
> exists the whole time. It is the type of `"good"`.

> **Solution**
>
> ```rust
> fn grade_label(grade: u32) -> &'static str {
>     match grade {
>         0..=9 => "fail",
>         10..=13 => "fair",
>         14..=15 => "good",
>         16..=20 => "very good",
>         _ => "invalid grade",
>     }
> }
>
> fn main() {
>     for grade in [8, 12, 15, 19, 25] {
>         println!("{grade}: {}", grade_label(grade));
>     }
> }
> ```
>
> ```sortie
> 8: fail
> 12: fair
> 15: good
> 19: very good
> 25: invalid grade
> ```
>
> Why it works: `match` tries the cases from top to bottom, like numbered mailboxes.
> `_` catches everything else: Rust requires it, because a `u32` can go far beyond
> 20.

## Going further

### Exercise 10: The Caesar cipher

Level: ★★★

The Caesar cipher shifts each letter by 3 in the alphabet: `a` becomes `d`, `b`
becomes `e`… and `x`, `y`, `z` go back to the start (`a`, `b`, `c`). Write
`caesar(text: &str) -> String` for lowercase letters without accents; the other
characters (spaces…) don't change.

Expected output:

```text
hello zoe -> khoor crh
```

> **Hint:** a character can become a number: `c as u8`. `'a' as u8` is 97. The
> position in the alphabet is therefore `c as u8 - b'a'` (0 for `a`, 25 for `z`).

> **Hint:** shift, then go back to the start with `%`:
> `(position + 3) % 26`. Then turn it back into a letter: `(b'a' + shifted) as char`.

> **Solution**
>
> ```rust
> fn caesar(text: &str) -> String {
>     let mut result = String::new();
>     for c in text.chars() {
>         if c.is_ascii_lowercase() {
>             let position = c as u8 - b'a';
>             let shifted = (position + 3) % 26;
>             result.push((b'a' + shifted) as char);
>         } else {
>             result.push(c);
>         }
>     }
>     result
> }
>
> fn main() {
>     let message = "hello zoe";
>     println!("{message} -> {}", caesar(message));
> }
> ```
>
> ```sortie
> hello zoe -> khoor crh
> ```
>
> | c | position | position + 3 | % 26 | letter |
> |---|----------|--------------|------|--------|
> | h | 7 | 10 | 10 | k |
> | e | 4 | 7 | 7 | h |
> | z | 25 | 28 | 2 | c |
> | (space) | | | | (space) |
>
> Why it works: the alphabet is like a 26-hour clock. After 25, we start again at 0:
> that is the job of `% 26`. `b'a'` is the letter `a` as a byte (`u8`), to compute
> with.
>
> Try it: shift by 23 instead of 3. Applied to the coded text, this decodes it: you
> get back `hello zoe`.

### Exercise 11: Prime numbers

Level: ★★★

A prime number is only divisible by 1 and by itself (2, 3, 5, 7…). Write
`is_prime(n: u32) -> bool`, then print the prime numbers up to 30.

Expected output:

```text
2 3 5 7 11 13 17 19 23 29
```

> **Hint:** 0 and 1 are not prime. For the others, try each divisor `d` from 2 to
> `n - 1`: if `n % d == 0`, it is not prime.

> **Hint:** as soon as a divisor is found, `return false;` leaves the function right
> away. If the loop ends without finding anything, it is prime.

> **Solution**
>
> ```rust
> fn is_prime(n: u32) -> bool {
>     if n < 2 {
>         return false;
>     }
>     let mut d = 2;
>     while d * d <= n {
>         if n % d == 0 {
>             return false;
>         }
>         d += 1;
>     }
>     true
> }
>
> fn main() {
>     let mut primes = Vec::new();
>     for n in 1..=30 {
>         if is_prime(n) {
>             primes.push(n.to_string());
>         }
>     }
>     println!("{}", primes.join(" "));
> }
> ```
>
> ```sortie
> 2 3 5 7 11 13 17 19 23 29
> ```
>
> Trace of `is_prime(9)` then of `is_prime(11)`:
>
> | Call | d | d × d <= n ? | n % d | Result |
> |------|---|--------------|-------|--------|
> | 9 | 2 | 4 <= 9: yes | 1 | keep going |
> | 9 | 3 | 9 <= 9: yes | **0** | `false` |
> | 11 | 2 | 4 <= 11: yes | 1 | keep going |
> | 11 | 3 | 9 <= 11: yes | 2 | keep going |
> | 11 | 4 | 16 <= 11: **no** | | `true` |
>
> Why it works: the solution stops when `d × d` goes past `n`. If `n` had a larger
> divisor, it would also have a smaller "partner", already tested (for 30: 10 × 3,
> and 3 was seen before 10). Your version up to `n - 1` is correct too, just slower.

### Exercise 12: Compressing a text

Level: ★★★

Write `compress(text: &str) -> String` that replaces each run of identical letters
with the letter followed by the number of repetitions: `"aaabccdddd"` becomes
`"a3b1c2d4"`.

Expected output:

```text
aaabccdddd -> a3b1c2d4
zzz -> z3
```

> **Hint:** keep the current letter (`current`) and its counter (`count`). If the next
> letter is the same, `count += 1`. Otherwise, write the letter and its count, then
> start again with the new letter.

> **Hint:** after the loop, the **last** run is still left to write. It is the most
> common slip!

> **Solution**
>
> ```rust
> fn compress(text: &str) -> String {
>     let mut result = String::new();
>     let mut letters = text.chars();
>     let Some(mut current) = letters.next() else {
>         return result;
>     };
>     let mut count = 1;
>     for c in letters {
>         if c == current {
>             count += 1;
>         } else {
>             result.push_str(&format!("{current}{count}"));
>             current = c;
>             count = 1;
>         }
>     }
>     result.push_str(&format!("{current}{count}"));
>     result
> }
>
> fn main() {
>     for text in ["aaabccdddd", "zzz"] {
>         println!("{text} -> {}", compress(text));
>     }
> }
> ```
>
> ```sortie
> aaabccdddd -> a3b1c2d4
> zzz -> z3
> ```
>
> | c | c == current ? | current | count | result |
> |---|----------------|---------|-------|--------|
> | (start) | | a | 1 | |
> | a | yes | a | 2 | |
> | a | yes | a | 3 | |
> | b | no | b | 1 | a3 |
> | c | no | c | 1 | a3b1 |
> | c | yes | c | 2 | a3b1 |
> | d | no | d | 1 | a3b1c2 |
> | d | yes | d | 2 | a3b1c2 |
> | d | yes | d | 3 | a3b1c2 |
> | d | yes | d | 4 | a3b1c2 |
> | (after the loop) | | | | a3b1c2d4 |
>
> Why it works: the first letter is read separately with `letters.next()`. If the
> text is empty, `let … else` returns an empty text right away. After that, a run is
> written only when a **different** letter arrives: the last run, which is followed
> by nothing, is written after the loop.
>
> Try it: delete the `result.push_str(…)` line placed after the loop. You get
> `a3b1c2` and an empty text for `zzz`: the last run is lost.

## Summary

- Make a trace table on paper before writing the code: it shows the steps.
- The "champion" (exercises 2 and 7) and the accumulator (exercises 5 and 8) are two
  ideas that come back everywhere.
- Think about edge cases: empty text, ties, the last run, the number 0 or 1.
- To review an idea, the [cheat sheet](aide-memoire.md) sums up the whole track.

Next: [cheat sheet](aide-memoire.md)
