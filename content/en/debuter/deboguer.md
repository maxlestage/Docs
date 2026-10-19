# Finding and fixing your mistakes

Every program has bugs at first: this page teaches you how to read an error message, display values to understand what's going on, and ask for help effectively.

> **Goals:**
>
> - recognize the three kinds of errors: syntax, runtime, logic;
> - read an error message line by line;
> - display values to understand what the program does;
> - shrink a problem down to a few lines;
> - search for an error message online and ask for help.

## Three kinds of errors

### Syntax errors

The code is **badly written**: a forgotten parenthesis, a missing `end`.
The program doesn't even start. Here, the function has no `end`:

```ruby norun
def hello(name)
  puts "Hello, #{name}!"

hello("Ada")
```

```text
syntax.rb: --> syntax.rb
Unmatched keyword, missing `end' ?
> 1  def hello(name)
> 4  hello("Ada")
syntax.rb:4: syntax error, unexpected end-of-input, expecting `end' or dummy end (SyntaxError)
hello("Ada")
            ^
```

Ruby guessed right: "missing `end' ?", an `end` is missing.

### Runtime errors

The code is well written, it starts, but it **stops along the way**
because an operation is impossible. Here, we read item number 3 of a list
that only has three (numbered 0, 1 and 2):

```rust norun
fn main() {
    let grades = vec![12, 15, 9];
    println!("{}", grades[3]);
}
```

```text
thread 'main' (6567) panicked at index.rs:3:26:
index out of bounds: the len is 3 but the index is 3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

In Rust, we say the program **panics**.

### Logic errors

The program works, **but the result is wrong**. There's no message at all:
it's the hardest error to find. Here, we want the average of 10 and 20:

```ruby
puts 10 + 20 / 2
```

```sortie
20
```

We expected 15. Division comes before addition, as in math: the computer
calculated `10 + 10`. You had to write `(10 + 20) / 2`.

> **Remember:** a syntax error prevents the start, a runtime error stops
> along the way, a logic error gives a wrong result without saying
> anything.

## Reading an error message

An error message looks scary, but it always follows the same plan. It
answers three questions: **what**, **where** and often **how to fix it**.

### In Ruby

```ruby norun
prices = [12, 5, 8]
total = prices.sum
puts "Total: " + total
```

```text
prices.rb:3:in `+': no implicit conversion of Integer into String (TypeError)

puts "Total: " + total
                 ^^^^^
	from prices.rb:3:in `<main>'
```

Line by line:

1. `prices.rb:3`: the file and the **line** of the problem;
2. ``in `+'``: during the `+` operation;
3. `no implicit conversion of Integer into String`: **what** — Ruby
   doesn't turn a number (*Integer*) into text (*String*) on its own;
4. `(TypeError)`: the **name** of the error, useful for searching online;
5. the line of code, with `^^^^^` under the faulty part;
6. `from …`: the path the program followed up to the error.

Fix: `puts "Total: #{total}"`.

### In Rust

```rust norun
fn main() {
    let age: i32 = "36";
    println!("{age}");
}
```

```text
error[E0308]: mismatched types
 --> age.rs:2:20
  |
2 |     let age: i32 = "36";
  |              ---   ^^^^ expected `i32`, found `&str`
  |              |
  |              expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

1. `error[E0308]: mismatched types`: **what** — the types don't match;
   `E0308` is the error code;
2. `--> age.rs:2:20`: **where** — file, line 2, column 20;
3. the line of code, with `^^^^` under the problem: a number (`i32`) was
   expected, a text (`&str`) was found;
4. `---` shows the cause: it's `i32` that asks for a number;
5. the last line gives the command for a full explanation.

Fix: `let age: i32 = 36;`, without quotes.

### In Python

```python norun
prices = [12, 5, 8]
total = sum(prices)
print("Total: " + total)
```

```text
Traceback (most recent call last):
  File "prices.py", line 3, in <module>
    print("Total: " + total)
          ~~~~~~~~~~^~~~~~~
TypeError: can only concatenate str (not "int") to str
```

Unlike Ruby and Rust, Python puts the essentials **at the bottom**: read
the last line first.

1. `Traceback`: the path the program followed, from top to bottom;
2. `File "prices.py", line 3`: **where** — the file and the line;
3. the line of code, with `^` under the faulty operation;
4. `TypeError: can only concatenate str (not "int") to str`: **what** —
   you can only glue (*concatenate*) a text (`str`) to a text, not a
   number (`int`).

Fix: `print(f"Total: {total}")`.

### In JavaScript

```javascript norun
const student = { name: "Ada", grades: [12, 15] };
console.log(student.grade.length);
```

```text
file:///home/ada/grades.mjs:2
console.log(student.grade.length);
                          ^

TypeError: Cannot read properties of undefined (reading 'length')
    at file:///home/ada/grades.mjs:2:27
```

1. `grades.mjs:2`: **where** — the file and the line;
2. the line of code, with `^` under the spot of the problem;
3. `TypeError: Cannot read properties of undefined (reading 'length')`:
   **what** — you're asking for `length` from something that is
   `undefined` ("nothing");
4. `at …`: the path the program followed.

`student.grade` doesn't exist: the property is called `grades`.
JavaScript doesn't report it right away, it gives `undefined`, and the
error shows up one step later.

### In Rythme

```rythme norun
grades = [12, 15, 9]
print(len(grade))
```

```text
error[E003]: unknown name `grade`
 --> name.ry:2:11
  |
2 | print(len(grade))
  |           ^^^^^ not defined
  |
  = help: did you mean `grades`?
  = more about this error: `rythme explain E003`
```

1. `error[E003]: unknown name`: **what**, in plain English;
2. `--> name.ry:2:11`: **where**;
3. the line, with `^^^^^` under the unknown name;
4. `help`: **how to fix it** — here, a simple typo;
5. `rythme explain E003`: a detailed explanation, with examples.

> **Tip:** when there are several errors, fix **the first one**, then run
> again. The next ones are often consequences of the first.

## Displaying values to understand

When the result is wrong, look at what your variables contain. Add
displays in the middle of the program.

### In Ruby: puts and p

`puts` displays a value simply. `p` displays it **as it is in the code**:
you see the quotes of a text and the brackets of a list. For debugging,
prefer `p`.

```ruby
prices = [12, 5, 8]
p prices
total = prices.sum
p total
puts "Average: #{total / prices.size}"
```

```sortie
[12, 5, 8]
25
Average: 8
```

```ruby
puts "42"
p "42"
puts nil
p nil
```

```sortie
42
"42"

nil
```

With `puts`, you can't see whether `42` is a number or a text, or whether
a value is `nil`. With `p`, you can.

### In Rust: println! and dbg!

`println!("{:?}", x)` displays a value as it is. `dbg!` does better: it
displays the file, the line, the calculation and its result, then
**returns the value**. So you can slip it into the middle of a
calculation.

```rust
fn main() {
    let prices = [12, 5, 8];
    let total: i32 = prices.iter().sum();
    println!("{:?} {total}", prices);
    let average = dbg!(total / 3);
    println!("Average: {average}");
}
```

In the terminal:

```text
[12, 5, 8] 25
[main.rs:5:19] total / 3 = 8
Average: 8
```

`dbg!` writes to the **error output**, separately: it doesn't mix with
the program's real results if you save them to a file.

### In Rythme: print and debug

`print` displays. `debug` displays the value with the file and the line,
then returns it, like `dbg!`:

```rythme
prices = [12, 5, 8]
total = debug(sum(prices))
print("Average: {total / len(prices)}")
```

In the terminal:

```text
[debug main.ry:2:9] 25
Average: 8.333333333333334
```

### In Python: print and the = sign

In an `f"…"` text, writing `{total=}` displays the variable's **name** and
its value. `repr` shows a value as it is in the code, with the quotes of a
text:

```python
prices = [12, 5, 8]
total = sum(prices)
print(f"{prices=} {total=}")
print("42", repr("42"))
```

```sortie
prices=[12, 5, 8] total=25
42 '42'
```

### In JavaScript: console.log

Put the variables between braces: `console.log` then displays each name
with its value. `JSON.stringify` shows the quotes of a text:

```javascript
const prices = [12, 5, 8];
const total = prices.reduce((sum, p) => sum + p, 0);
console.log({ prices, total });
console.log("42", JSON.stringify("42"));
```

```sortie
{ prices: [ 12, 5, 8 ], total: 25 }
42 "42"
```

> **Warning:** once the bug is found, remove your debugging `p`, `dbg!`,
> `debug`, `print` and `console.log`. They must not stay in the final
> program.

## The rubber duck

A very serious method, despite its name: put a rubber duck (or any
object) next to the screen, and **explain your code to it, line by line,
out loud**.

"Here, I put the first price in `total`. Then, for each price… oh, but I
start at 1, not at 0!"

While explaining, you're forced to say what the code **really does**, not
what you **think** it does. Very often, you find the bug before you're
done. It works with a friend too, even one who doesn't program.

## Shrinking the problem

A bug hidden in 200 lines is hard to find. In 5 lines, it jumps out at
you.

1. **Copy** your program into a new file.
2. **Remove** a piece that doesn't seem related to the problem.
3. **Run it again**: is the bug still there?
   - yes: keep removing;
   - no: the bug was in the removed piece. Put it back, and look inside
     it.
4. Stop when only **a few lines** are left.

Also replace data read from a file or from the keyboard with values
written in the code: `grades = [12, 15, 9]`.

> **Tip:** change **only one thing at a time**, then run again. If you
> change three things, you won't know which one mattered.

## Searching for the message online

You're surely not the first person to run into this message. Copy it into
a search engine:

- keep the **general** part: `no implicit conversion of Integer into
  String`;
- remove what is specific to **your** program: file names, line numbers,
  variable names;
- add the **name of the language**: `python`, `javascript`, `go`, `ruby`,
  `rust`…

Error codes are very easy to search for: `rust E0308`. And they can be
explained without the Internet:

```sh
rustc --explain E0308
rythme explain E003
```

## Asking for help

If you've been stuck for a long time, ask: on a forum, a friend, an
assistant. A good question quickly gets a good answer. It contains:

1. **what you want to do**: "calculate the average of my grades";
2. **a minimal example**: the few lines that show the problem, as text
   (not a photo of the screen);
3. **what you expected**: "12";
4. **what happens**: the result you got, or the **complete** error
   message, copy-pasted;
5. **what you've already tried**.

For example:

```text
I want the average of 10 and 20 in Ruby. My code:

    puts 10 + 20 / 2

I expect 15, but I get 20. I checked that 10 + 20 does give 30.
Why?
```

Often, while preparing the minimal example, you'll find the answer
yourself.

## Common mistakes

### Reading only the last line of the message

In Ruby, the message starts at the top and the `from …` path follows. In
Rust, there are often several errors in a row:

```rust norun
fn main() {
    let total = 0;
    total = total + 5;
    println!("{total}");
}
```

```text
error[E0384]: cannot assign twice to immutable variable `total`
 --> mu.rs:3:5
  |
2 |     let total = 0;
  |         ----- first assignment to `total`
3 |     total = total + 5;
  |     ^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut total = 0;
  |         +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0384`.
```

The last line says almost nothing. Everything is **at the top**: the
problem (`cannot assign twice`) and even the fix (`help: … let mut
total`).

### Looking in the wrong place

The line shown isn't always the one to fix. For a forgotten `end`, Ruby
complains at the **end** of the file:

```text
syntax.rb:4: syntax error, unexpected end-of-input, expecting `end' or dummy end (SyntaxError)
```

The problem is higher up, on line 1: the function that is never closed.
If the line shown looks right, look at the lines **above**.

### Not seeing the difference between a number and a text

`puts` displays `42` for the number as well as for the text `"42"`:

```ruby norun
age = "36"
puts age + 1
```

```text
main.rb:2:in `+': no implicit conversion of Integer into String (TypeError)

puts age + 1
           ^
	from main.rb:2:in `<main>'
```

A `p age` would have displayed `"36"`, with the quotes: it's a text.
Convert it with `age.to_i`, or write `age = 36`.

## Test yourself

**1.** Your program displays `20` instead of `15`, with no message. What
kind of error is it?

> **Answer**
>
> A logic error: the program works, but the result is wrong. Display the
> intermediate values (`print` in Python, `console.log` in JavaScript,
> `p` in Ruby, `dbg!` in Rust, `debug` in Rythme) to find the step that
> goes wrong.

**2.** In the message `--> age.rs:2:20`, what do `2` and `20` mean?

> **Answer**
>
> The error is in the file `age.rs`, on line 2, column 20 (the twentieth
> character of the line).

**3.** What should a good request for help contain?

> **Answer**
>
> What you want to do, a minimal example as text, the expected result,
> the result you got (or the complete error message) and what you've
> already tried.

## Summary

- Three kinds of errors: syntax (doesn't start), runtime (stops along the way), logic (wrong result).
- An error message says what, where (file, line, column) and often how to fix it: read it in full.
- Display values: `print(f"{x=}")` in Python, `console.log({ x })` in JavaScript, `p` in Ruby, `dbg!` in Rust, `debug` in Rythme.
- Explain your code to the duck, and shrink the problem down to a few lines.
- Search for the general part of the message online; to ask for help, give a minimal example.

Next: [saving your work with Git](git.md).
