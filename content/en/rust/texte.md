# Working with text

This page shows how to create, transform, split and walk through text in Rust, with the two types `String` and `&str`.

> **Goals:**
>
> - choose between `String` and `&str`;
> - build a text with `push_str`, `format!` and `join`;
> - search, replace and split a text;
> - convert a text into a number, and back;
> - walk through the characters without cutting an accented letter in half.

## Two types of text: String and &str

Almost every program handles text: a first name, a welcome message, a shopping
list. Choosing the right type will save you many compiler errors.

A text, or [string](../debuter/glossaire.md#string), is a sequence of
letters. Rust has two types of text. To get started, remember this:

- `&str`: a text that you **look at**. Texts between quotes are `&str`.
- `String`: a text that you **own** and that you can change.

Think of a book: `&str` is a library book, which you read without writing in it;
`String` is your own notebook, where you can add pages.

```rust
let borrowed: &str = "hello";
let owned: String = String::from("hello");
let other: String = "hi".to_string();
println!("{borrowed} {owned} {other}");
```

```sortie
hello hello hi
```

Line by line:

- `borrowed` is a `&str`: it looks at the text `"hello"` written in the program;
- `owned` is a `String` made with `String::from`: a copy that belongs to it;
- `other` is also a `String`, made this time with `.to_string()`;
- `println!` prints all three: on the screen, nothing tells them apart.

`String::from("…")` and `.to_string()` make a `String` from a `&str`. The other way
round, to go from a `String` to a `&str`, put `&` in front:

```rust
fn show(t: &str) {
    println!("[{t}]");
}

fn main() {
    let name = String::from("Léa");
    show(&name);
    show("Tom");
}
```

```sortie
[Léa]
[Tom]
```

> **Remember:** a function parameter of type `&str` accepts both kinds of text. It is
> the most convenient choice.

## Growing a String

A text is often built bit by bit, like a shopping list you add to during the week.
For that, you need a `String` that can change.

A `String` declared with `mut` can grow. `push_str` adds a text, `push` adds a single
character:

```rust
let mut sentence = String::from("Hello");
sentence.push_str(" everyone");
sentence.push('!');
println!("{sentence}");
```

```sortie
Hello everyone!
```

Line by line:

- `let mut sentence` creates a `String` that holds "Hello". `mut` allows it to change;
- `push_str(" everyone")` sticks this text at the end: `sentence` is now
  "Hello everyone";
- `push('!')` adds a single character. A single character is written between single
  quotes;
- `println!` prints the final result.

The `+` operator also sticks two texts together, but the left side must be a `String`:

```rust
let start = String::from("Hello");
let words = start + " world";
println!("{words}");
```

```sortie
Hello world
```

**Your turn:** write a program that starts from the `String` "It is", adds
"sunny" with `push_str`, then a full stop with `push`, and prints the result.

> **Solution**
>
> ```rust
> let mut weather = String::from("It is");
> weather.push_str(" sunny");
> weather.push('.');
> println!("{weather}");
> ```
>
> ```sortie
> It is sunny.
> ```

## Building with format!

A program often needs to build a sentence from values: a label, a welcome message,
a line on a bill. `format!` prepares it without printing it right away.

`format!` works like `println!`, but it **returns** a `String` instead of printing it:

```rust
let first_name = "Alice";
let age = 30;
let card = format!("{first_name} ({age} years old)");
println!("{card}");
```

```sortie
Alice (30 years old)
```

Line by line:

- `first_name` holds the text "Alice", and `age` the number 30;
- `format!` replaces `{first_name}` and `{age}` with their values, and returns the
  `String` "Alice (30 years old)";
- this `String` is stored in `card`: nothing is printed yet;
- `println!` is what finally prints it on the screen.

> **Tip:** `format!` is the simplest way to stick several pieces of text and numbers
> together.

## The length of a text

Counting letters is often useful: checking that a password is long enough, or that
a username is not too long. In Rust, there is a small trap.

`len()` counts **bytes**, not letters. A byte is the smallest box in memory. A plain
letter takes one byte, an accented letter takes two.
To count the characters, use `chars().count()`:

```rust
let word = "café";
println!("{}", word.len());
println!("{}", word.chars().count());
```

```sortie
5
4
```

Line by line:

- `word` holds "café": four letters, one of them accented;
- `word.len()` counts the bytes: 1 each for `c`, `a` and `f`, 2 for `é`, so 5;
- `word.chars().count()` counts the characters: 4.

> **Warning:** for a text that may contain accents, prefer `chars().count()`.

## Changing case and cleaning up

What a person types is rarely clean: extra spaces, random capital letters. Before
comparing or storing a text, you clean it up.

```rust
let raw = "  Hello Rust  ";
println!("[{}]", raw.trim());
println!("{}", raw.trim().to_uppercase());
println!("{}", raw.trim().to_lowercase());
```

```sortie
[Hello Rust]
HELLO RUST
hello rust
```

- `trim()` removes the spaces at the start and at the end;
- `to_uppercase()` puts the text in capital letters;
- `to_lowercase()` puts the text in small letters.

**Your turn:** write a program that starts from the text `"   alice "`, removes
the spaces around it, puts it in capital letters and prints it between square brackets.

> **Solution**
>
> ```rust
> let input = "   alice ";
> println!("[{}]", input.trim().to_uppercase());
> ```
>
> ```sortie
> [ALICE]
> ```

## Searching in a text

A program often needs to check a text: does an email address contain an `@`? Does
a file name end with `.txt`? It is like the search box of your word processor.

These methods answer with `true` or `false`:

```rust
let sentence = "The cat sleeps";
println!("{}", sentence.contains("cat"));
println!("{}", sentence.starts_with("The"));
println!("{}", sentence.ends_with("!"));
```

```sortie
true
true
false
```

## Replacing

Replacing one word with another is how you fix a typo everywhere, or fill in a
letter template with the right name.

`replace` returns a **new** `String`. The original text does not change:

```rust
let before = "I like tea";
let after = before.replace("tea", "coffee");
println!("{before}");
println!("{after}");
```

```sortie
I like tea
I like coffee
```

## Splitting with split

A text often holds several pieces of information stuck together: fruits separated
by commas, a sentence made of words. To handle them one by one, you first separate
them, like breaking a chocolate bar into squares.

`split` cuts the text at each separator. `collect` puts the pieces into a `Vec`:

```rust
let list = "apple,pear,kiwi";
let fruits: Vec<&str> = list.split(',').collect();
println!("{:?}", fruits);
println!("{}", fruits.len());
```

```sortie
["apple", "pear", "kiwi"]
3
```

Line by line:

- `list` holds three fruits separated by commas;
- `split(',')` cuts at each comma: you get "apple", "pear" and "kiwi";
- `collect()` puts these pieces into `fruits`, a `Vec<&str>` (a list of texts);
- `{:?}` prints the list with its brackets, and `len()` gives the number of pieces.

To cut on spaces, `split_whitespace` is more convenient:

```rust
for word in "one  two three".split_whitespace() {
    println!("{word}");
}
```

```sortie
one
two
three
```

To do the opposite, `join` glues the pieces back together:

```rust
let words = vec!["a", "b", "c"];
println!("{}", words.join("-"));
```

```sortie
a-b-c
```

**Your turn:** write a program that splits the text `"Monday Tuesday Wednesday"`
into words, then glues them back with `" / "` between them.

> **Solution**
>
> ```rust
> let days: Vec<&str> = "Monday Tuesday Wednesday".split_whitespace().collect();
> println!("{}", days.join(" / "));
> ```
>
> ```sortie
> Monday / Tuesday / Wednesday
> ```

## Turning a text into a number

For Rust, `"42"` is not a number: it is a text of two characters, and you cannot
do maths with it. Yet what you type on the keyboard always arrives as text. So you
have to convert it.

`parse` reads a number in a text. Give the type you want with `::<i32>`. The result is
a `Result`, because the text may not be a number:

```rust
let good = "42".parse::<i32>();
let bad = "abc".parse::<i32>();
println!("{:?}", good);
println!("{}", bad.is_err());
let n = "  7 ".trim().parse::<i32>().unwrap();
println!("{}", n * 2);
```

```sortie
Ok(42)
true
14
```

Line by line:

- `"42".parse::<i32>()` succeeds: the result is `Ok(42)`, the number inside a
  "success" box;
- `"abc".parse::<i32>()` fails: `is_err()` answers `true`, "yes, it is an error";
- the line before last removes the spaces with `trim()`, reads 7, and `unwrap()` takes
  the number out of its `Ok` box;
- so `n` is the number 7, and `n * 2` gives 14.

> **Tip:** remember to call `trim()` before `parse`: a space or a line break makes the
> reading fail. Errors are explained in [Handling errors](erreurs.md).

The other way round, `to_string()` turns a number into a text:

```rust
let text = 2026.to_string();
println!("{}", text.len());
```

```sortie
4
```

**Your turn:** write a program that reads the numbers in the texts `"12"` and
`"30"` with `parse`, then prints their sum.

> **Solution**
>
> ```rust
> let a = "12".parse::<i32>().unwrap();
> let b = "30".parse::<i32>().unwrap();
> println!("{}", a + b);
> ```
>
> ```sortie
> 42
> ```

## Walking through the characters

Sometimes you need to look at a text letter by letter: count the vowels, check
that a password contains a digit, or reverse a word.

`chars()` gives the characters one by one (values of type `char`):

```rust
let word = "naïve";
for c in word.chars() {
    print!("{c}.");
}
println!();
let vowels = word.chars().filter(|c| "aeiouyï".contains(*c)).count();
println!("{vowels} vowels");
```

```sortie
n.a.ï.v.e.
3 vowels
```

Line by line:

- `for c in word.chars()` gives `c` the letters "n", "a", "ï", "v", then "e";
- `print!` (without `ln`) prints each letter followed by a dot, on the same line;
- `println!()`, empty, simply moves to the next line;
- `filter` keeps the letters found in `"aeiouyï"`, and `count` counts them: "a", "ï"
  and "e", so 3. The `|c| …` part is a small function without a name, explained in
  [closures](closures.md).

To reverse a text, combine `chars`, `rev` and `collect`:

```rust
let reversed: String = "radar".chars().rev().collect();
println!("{reversed}");
```

```sortie
radar
```

## Taking a piece carefully

Taking a piece of a text lets you keep only part of it: the day in "Monday
morning", the year in a date. You have to do it carefully, because of accents.

You can take part of a text with `&text[start..end]`. The numbers are **bytes**:

```rust
let day = "Monday morning";
println!("{}", &day[0..6]);
println!("{}", &day[7..]);
```

```sortie
Monday
morning
```

> **Warning:** if the cut falls in the middle of an accented letter, the program stops
> with a `panic!`.

```rust norun
let word = "résumé";
println!("{}", &word[0..2]); // panic: end byte index 2 is not a char boundary
```

For a text with accents, go through the characters:

```rust
let word = "résumé";
let start: String = word.chars().take(2).collect();
println!("{start}");
```

```sortie
ré
```

## Raw strings

> **Note:** this part can wait. Read it once the basics feel solid.

Some texts contain many `\` or quotes: a Windows file path, a quotation. A raw
string keeps them as they are, with nothing to escape.

A raw string is written `r"…"`. The backslashes `\` are kept as they are. With
`r#"…"#`, it can even contain quotes:

```rust
let path = r"C:\folder\new";
let quote = r#"He said "hello""#;
println!("{path}");
println!("{quote}");
```

```sortie
C:\folder\new
He said "hello"
```

> **Note:** in a normal text, `\n` is a line break. In a raw string, it is just a
> backslash and an `n`.

## Common mistakes

### Putting a &str into a String

```rust norun
fn main() {
    let name: String = "Léa";
    println!("{name}");
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:2:24
  |
2 |     let name: String = "Léa";
  |               ------   ^^^^^ expected `String`, found `&str`
  |               |
  |               expected due to this
help: try using a conversion method
```

A text between quotes is a `&str`. Convert it: `"Léa".to_string()` or
`String::from("Léa")`.

### Reading a letter with [0]

```rust norun
fn main() {
    let word = "hello";
    let first = word[0];
    println!("{first}");
}
```

```text
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> src/main.rs:3:22
  |
3 |     let first = word[0];
  |                      ^ string indices are ranges of `usize`
  |
  = note: you can use `.chars().nth()` or `.bytes().nth()`
```

A letter can take several bytes: Rust refuses to guess. Write `word.chars().next()` for
the first letter, or `word.chars().nth(2)` for the third one. Both return an `Option`
(see [the Option type](types.md)).

### Adding two &str

```rust norun
fn main() {
    let sentence = "Hello" + " world";
    println!("{sentence}");
}
```

```text
error[E0369]: cannot add `&str` to `&str`
 --> src/main.rs:2:28
  |
2 |     let sentence = "Hello" + " world";
  |                    ------- ^ -------- &str
  |                    |       |
  |                    |       `+` cannot be used to concatenate two `&str` strings
  |                    &str
  |
  = note: string concatenation requires an owned `String` on the left
```

The left side of `+` must be a `String`: `String::from("Hello") + " world"`. Simpler:
`format!("{}{}", "Hello", " world")`.

### Growing a String without mut

```rust norun
fn main() {
    let sentence = String::from("Hello");
    sentence.push_str("!");
    println!("{sentence}");
}
```

```text
error[E0596]: cannot borrow `sentence` as mutable, as it is not declared as mutable
 --> src/main.rs:3:5
  |
3 |     sentence.push_str("!");
  |     ^^^^^^^^ cannot borrow as mutable
```

Fix: `let mut sentence = String::from("Hello");`.

## Test yourself

**1.** What are `"naïve".len()` and `"naïve".chars().count()`?

> **Answer**
>
> `6` and `5`: the `ï` takes two bytes, but it is a single character.

**2.** How do you get `["a", "b", "c"]` from `"a;b;c"`?

> **Answer**
>
> ```rust
> let pieces: Vec<&str> = "a;b;c".split(';').collect();
> println!("{:?}", pieces);
> ```
>
> ```sortie
> ["a", "b", "c"]
> ```

**3.** Why write `.trim()` before `.parse()` on keyboard input?

> **Answer**
>
> The input ends with a line break. `parse` fails on this character; `trim()` removes
> it.

## Summary

- `&str` to look at a text, `String` to own it and change it.
- `push_str`, `format!` and `join` build text.
- `len()` counts bytes, `chars().count()` counts characters.
- `trim`, `to_uppercase`, `replace`, `split` and `contains` transform and search.
- `parse::<i32>()` reads a number and returns a `Result`.

Compare with [text in Ruby](../ruby/texte.md) and [in Rythme](../rythme/texte.md).

Next: [Collections](collections.md).
