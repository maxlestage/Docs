# Pattern matching

Pattern matching compares a value with possible shapes and pulls out its pieces: it is one of the most powerful tools in Rust, with `match`, `if let`, `let else` and `while let`.

> **Goals:**
>
> - write a complete `match` with values, ranges and alternatives;
> - pull out the pieces of a tuple, a struct, an enum or a slice;
> - add a condition to a pattern with an `if` guard;
> - choose between `match`, `if let`, `let else` and `while let`;
> - test a shape quickly with `matches!`.

A **pattern** is a shape. Think of the shape-sorter toy for children: the cube goes
through the square hole, the cylinder through the round hole. Rust tries each hole in
order, and the value goes through the first one that fits it.

## match: every case, in order

You have already seen [match on an enum](types.md#match-on-an-enum). `match` also
works with numbers, characters or strings. Each line is an **arm**: a pattern, an arrow
`=>`, then the code to run.

```rust
let die = 3;
let text = match die {
    1 => "one",
    2 => "two",
    3 => "three",
    _ => "many",
};
println!("{text}");
```

```sortie
three
```

Two rules to remember:

- the arms are tried **from top to bottom**; the first one that matches wins;
- the `match` must be **exhaustive**: every possible value must be covered. The `_`
  pattern ("anything else") acts as a safety net.

> **Remember:** exhaustiveness is a safety feature. If you add a variant to an enum,
> the compiler shows you every `match` you need to complete.

`match` is an [expression](../debuter/glossaire.md#expression): it produces a value,
stored here in `text`. All the arms must then produce the same type.

## Several values: | and ranges

The bar `|` means "or". A range `a..=b` covers every value from `a` to `b` included
(see [ranges](../debuter/glossaire.md#intervalle)).

```rust
fn assessment(grade: u32) -> &'static str {
    match grade {
        0..=7 => "fail",
        8 | 9 => "almost",
        10..=15 => "good",
        16..=20 => "very good",
        _ => "invalid grade",
    }
}

fn main() {
    for n in [5, 9, 12, 18, 25] {
        println!("{n}: {}", assessment(n));
    }
}
```

```sortie
5: fail
9: almost
12: good
18: very good
25: invalid grade
```

Ranges also work with characters:

```rust
for c in ['a', 'Z', '7', '?'] {
    let kind = match c {
        'a'..='z' => "lowercase",
        'A'..='Z' => "uppercase",
        '0'..='9' => "digit",
        _ => "other",
    };
    println!("{c}: {kind}");
}
```

```sortie
a: lowercase
Z: uppercase
7: digit
?: other
```

## Giving a name with @

A pattern can **test** a value and **name** it at the same time, with `name @ pattern`:

```rust
let age = 15;
match age {
    0 => println!("newborn"),
    n @ 1..=17 => println!("minor, {n} years old"),
    n => println!("adult, {n} years old"),
}
```

```sortie
minor, 15 years old
```

The last arm, `n`, is a name on its own: it accepts any value and stores it in `n`.
It is like `_`, but it keeps the value.

## Destructuring a tuple

A pattern can have the **shape** of the value, and store each piece in a variable.
This is called **destructuring**.

```rust
let point = (3, -2);
match point {
    (0, 0) => println!("origin"),
    (x, 0) => println!("on the horizontal axis, x = {x}"),
    (0, y) => println!("on the vertical axis, y = {y}"),
    (x, y) => println!("elsewhere: ({x}, {y})"),
}
```

```sortie
elsewhere: (3, -2)
```

Destructuring also works outside a `match`, with `let` or in a `for`:

```rust
let (name, age) = ("Zoe", 12);
println!("{name} is {age} years old");
for (i, fruit) in ["apple", "kiwi"].iter().enumerate() {
    println!("{i}: {fruit}");
}
```

```sortie
Zoe is 12 years old
0: apple
1: kiwi
```

## Destructuring a struct

For a [struct](types.md#structs), you write the names of the fields. `..` ignores the
remaining fields.

```rust
struct Player {
    name: String,
    points: u32,
    level: u32,
}

fn main() {
    let p = Player { name: String::from("Leo"), points: 120, level: 3 };

    let Player { name, points, .. } = &p;
    println!("{name}: {points} points");

    match p {
        Player { level: 1, .. } => println!("beginner"),
        Player { level: n, .. } => println!("level {n}"),
    }
}
```

```sortie
Leo: 120 points
level 3
```

`level: 1` tests the value of the field. `level: n` stores the field in `n`.

## Destructuring an enum

The variants of an enum can carry data. The pattern pulls it out:

```rust
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Point,
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle { width, height } => width * height,
        Shape::Point => 0.0,
    }
}

fn main() {
    let shapes = [
        Shape::Circle(1.0),
        Shape::Rectangle { width: 2.0, height: 3.0 },
        Shape::Point,
    ];
    for s in &shapes {
        println!("{}", area(s));
    }
}
```

```sortie
3.14
6
0
```

## Destructuring a slice

A pattern in square brackets describes the **contents** of a list. `..` stands for
"all the elements in the middle", however many there are.

```rust
fn describe(v: &[i32]) -> String {
    match v {
        [] => String::from("empty"),
        [single] => format!("just one: {single}"),
        [a, b] => format!("two: {a} and {b}"),
        [first, .., last] => format!("from {first} to {last}"),
    }
}

fn main() {
    println!("{}", describe(&[]));
    println!("{}", describe(&[7]));
    println!("{}", describe(&[1, 2]));
    println!("{}", describe(&[1, 2, 3, 4, 5]));
}
```

```sortie
empty
just one: 7
two: 1 and 2
from 1 to 5
```

With `rest @ ..`, you can even name the middle: `[first, rest @ ..]` stores
everything except the first element in `rest`.

## Adding a condition: the guard

A **guard** is an `if` placed after the pattern. The arm is chosen only if the
condition is true:

```rust
let temperatures = [-5, 0, 18, 35];
for t in temperatures {
    let opinion = match t {
        t if t < 0 => "freezing",
        0 => "exactly zero",
        t if t > 30 => "heat wave",
        _ => "normal",
    };
    println!("{t}: {opinion}");
}
```

```sortie
-5: freezing
0: exactly zero
18: normal
35: heat wave
```

> **Warning:** the compiler does not look inside guards to check exhaustiveness. A
> last arm without a guard (`_`) is therefore often needed.

## Nested patterns

Patterns fit inside each other like Russian dolls. Here, an `Option` contains a tuple:

```rust
let orders = [Some(("coffee", 2)), None, Some(("tea", 0)), Some(("juice", 1))];
for o in orders {
    match o {
        None => println!("no order"),
        Some((drink, 0)) => println!("{drink}: cancelled"),
        Some((drink, 1)) => println!("one {drink}"),
        Some((drink, n)) => println!("{n} × {drink}"),
    }
}
```

```sortie
2 × coffee
no order
tea: cancelled
one juice
```

## if let: a single interesting case

When only one pattern interests you, `match` is a bit long. `if let` tests a pattern
and runs the block if it matches. An `else` is possible:

```rust
let input = "42";
if let Ok(n) = input.parse::<i32>() {
    println!("the double is {}", n * 2);
} else {
    println!("not a number");
}

let config: Option<&str> = None;
if let Some(theme) = config {
    println!("theme {theme}");
}
println!("end");
```

```sortie
the double is 84
end
```

## let else: leaving early

`let … else` does the opposite: it **destructures** the value, and if the pattern does
not match, it runs the `else` block, which must **leave** the function (`return`,
`break`, `continue` or `panic!`). Afterwards, the variable is available without
nesting.

```rust
fn read_age(text: &str) -> String {
    let Ok(age) = text.parse::<u32>() else {
        return String::from("invalid age");
    };
    format!("you are {age} years old")
}

fn main() {
    println!("{}", read_age("30"));
    println!("{}", read_age("thirty"));
}
```

```sortie
you are 30 years old
invalid age
```

> **Tip:** `let else` keeps the normal case on the far left of the code, and the error
> cases dealt with at the top. The code stays flat and readable.

## while let: looping as long as it matches

`while let` repeats a loop as long as the pattern matches. It is perfect for emptying
a stack:

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("I remove {top}");
}
println!("stack empty");
```

```sortie
I remove 3
I remove 2
I remove 1
stack empty
```

`pop` returns `Some(…)` as long as elements remain, then `None`: the loop stops.

## matches!: a yes or no question

The [macro](../debuter/glossaire.md#macro) `matches!` answers `true` or `false`: "does
this value have this shape?". It accepts `|`, ranges and guards.

```rust
let c = 'e';
println!("{}", matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y'));

let grades = [12, 4, 18, 9];
let good = grades.iter().filter(|n| matches!(n, 10..=20)).count();
println!("{good} good grades");

let result: Option<i32> = Some(7);
println!("{}", matches!(result, Some(x) if x > 5));
```

```sortie
true
2 good grades
true
```

## Common mistakes

### A match that forgets values

```rust norun
fn main() {
    let grade = 15;
    match grade {
        0..=9 => println!("fail"),
        10..=20 => println!("good"),
    }
}
```

```text
error[E0004]: non-exhaustive patterns: `i32::MIN..=-1_i32` and `21_i32..=i32::MAX` not covered
 --> src/main.rs:3:11
  |
3 |     match grade {
  |           ^^^^^ patterns `i32::MIN..=-1_i32` and `21_i32..=i32::MAX` not covered
  |
  = note: the matched value is of type `i32`
```

You know that a grade is between 0 and 20, but the compiler does not: an `i32` can be
-3 or 1000. Add a last arm `_ => println!("invalid"),`.

### Thinking that a name compares with a variable

```rust norun
fn main() {
    let target = 5;
    let guess = 3;
    match guess {
        target => println!("you win!"),
        _ => println!("you lose"),
    }
}
```

```text
warning: unreachable pattern
 --> src/main.rs:6:9
  |
5 |         target => println!("you win!"),
  |         ------ matches any value
6 |         _ => println!("you lose"),
  |         ^ no value can reach this
  |
note: there is a binding of the same name; if you meant to pattern match against the value of that binding, that is a feature of constants that is not available for `let` bindings
```

It is only a warning, but the program displays "you win!". In a pattern, a name on its
own **creates** a new variable that accepts anything. To compare, use a guard:
`n if n == target => …`, or simply `if guess == target`.

### A let else that does not leave

```rust norun
fn main() {
    let text = "abc";
    let Ok(n) = text.parse::<i32>() else {
        println!("not a number");
    };
    println!("{n}");
}
```

```text
error[E0308]: `else` clause of `let...else` does not diverge
 --> src/main.rs:3:42
  |
3 |       let Ok(n) = text.parse::<i32>() else {
  |  __________________________________________^
4 | |         println!("not a number");
5 | |     };
  | |_____^ expected `!`, found `()`
  |
  = note:   expected type `!`
          found unit type `()`
  = help: try adding a diverging expression, such as `return` or `panic!(..)`
  = help: ...or use `match` instead of `let...else`
```

After the `let else`, `n` must exist. So the `else` block must leave: add `return;`
after the `println!`.

## Test yourself

**1.** What does the `_` pattern do? And the `x` pattern on its own?

> **Answer**
>
> Both accept any value. `_` ignores it; `x` stores it in a new variable `x`.

**2.** Which pattern accepts a slice of at least two elements, naming the first and
the last?

> **Answer**
>
> `[first, .., last]`. It accepts two elements or more.

**3.** Write a test with `matches!` that says whether a character is a digit from 0
to 9.

> **Answer**
>
> ```rust
> let c = '4';
> println!("{}", matches!(c, '0'..='9'));
> println!("{}", matches!('x', '0'..='9'));
> ```
>
> ```sortie
> true
> false
> ```

## Summary

- `match` tries the arms in order and must cover every case; `_` acts as a safety net.
- `|` combines patterns, `a..=b` covers a range, `name @ pattern` keeps the value.
- A pattern can destructure a tuple, a struct, an enum or a slice.
- An `if` guard adds a condition to an arm.
- `if let` for a single case, `let else` to leave early, `while let` to loop.
- `matches!` answers `true` or `false` to "does this value have this shape?".

Next: [generics and traits](generiques.md)
