# Reading an error message without panicking

An error message isn't a punishment, it's help: this page teaches you how to find three pieces of information in it, in six languages, and what to do next.

> **Goals:**
>
> - see an error message as help;
> - find in a message: the place, the kind of error and the sentence that explains;
> - read a traceback from bottom to top;
> - tell apart a syntax error, a runtime error and a wrong result;
> - know what to do after reading the message.

## An error is help

A red message appears. Your heart beats faster. That's normal.

But look at what really happens: the computer stopped, and it
**explains why**. It's like a GPS saying "make a U-turn when possible":
it doesn't scold you, it helps you get back on the road.

Experienced programmers see errors **all day long**. The
difference is that they read them calmly. An
[error](glossaire.md#error) says nothing about you. It says something about
**one line** of your code.

> **Remember:** an error message is a free clue. Read it
> before touching the code.

## The three things to find

Every message, in every language, contains the same three
pieces of information. Look for them the way you look for the name, the date and the
place on an invitation:

```text
  1. WHERE?          the file and the line    →  hello.py, line 2
  2. WHAT KIND?      the name of the error    →  NameError
  3. WHY?            the sentence explaining  →  name 'nane' is not defined
```

The rest (the `~~~`, the `^^^`, the paths) helps you be more precise. Start
with these three.

Here are the same three things in six languages. The files are in the
`/home/ada` folder.

### In Python

```python norun
name = "Ada"
print("Hello " + nane)
```

```text
Traceback (most recent call last):
  File "/home/ada/hello.py", line 2, in <module>
    print("Hello " + nane)
                     ^^^^
NameError: name 'nane' is not defined. Did you mean: 'name'?
```

| Question | Answer in the message |
|---|---|
| Where? | `hello.py`, `line 2`; the `^^^^` underline `nane` |
| What kind? | `NameError`: an unknown name |
| Why? | `name 'nane' is not defined`: this name doesn't exist |

Python even suggests the fix: `Did you mean: 'name'?`. Just a typo,
nothing more.

### In JavaScript

```javascript norun
const name = "Ada";
console.log("Hello " + nane);
```

```text
file:///home/ada/name.mjs:2
console.log("Hello " + nane);
                       ^

ReferenceError: nane is not defined
    at file:///home/ada/name.mjs:2:24
```

- **Where?** `name.mjs:2`, line 2. The `^` shows the exact spot.
- **What kind?** `ReferenceError`: a reference (a name) that can't be found.
- **Why?** `nane is not defined`: `nane` isn't defined.

Node often adds other `at …` lines that are about Node itself
(`node:internal/…`). You can ignore them.

### In Ruby

```ruby norun
stock = { "apple" => 3, "pear" => 5 }
quantity = stock["banana"]
puts quantity + 1
```

```text
basket.rb:3:in `<main>': undefined method `+' for nil (NoMethodError)

puts quantity + 1
              ^
```

- **Where?** `basket.rb:3`, line 3.
- **What kind?** `NoMethodError`: an operation that doesn't exist.
- **Why?** `undefined method '+' for nil`: we do `+` on `nil`,
  that is, on "nothing".

Line 3 is correct. The real problem is on line 2: there's no
banana in the stock, so `quantity` is `nil`.

### In Rust

```rust norun
fn main() {
    let price = 12;
    println!("{}", price + quantity);
}
```

```text
error[E0425]: cannot find value `quantity` in this scope
 --> total.rs:3:28
  |
3 |     println!("{}", price + quantity);
  |                            ^^^^^^^^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

- **Where?** `total.rs:3:28`: line 3, column 28 (the 28th character).
- **What kind?** `E0425`, a code to look up with `rustc --explain E0425`.
- **Why?** `cannot find value 'quantity'`: the value `quantity`
  can't be found. You first need to write `let quantity = 3;`.

### In Go

```go norun
package main

import "fmt"

func main() {
	name := "Ada"
	age := 36
	fmt.Println(name)
}
```

```text
# command-line-arguments
./main.go:7:2: declared and not used: age
```

- **Where?** `main.go:7:2`: line 7, column 2.
- **What kind?** No name here: it's a compilation error.
- **Why?** `declared and not used: age`: the variable `age` is created
  but never used. Go refuses. Remove it, or print it.

### In Rythme

```rythme norun
fn double(x) = x * 2

print(double("ten"))
```

```text
error[E005]: cannot use `*` with Text and Int
 --> double.ry:3:7
  |
3 | print(double("ten"))
  |       ^^^^^^^^^^^^^ `double` stops with this error, every time
  |
1 | fn double(x) = x * 2
  |                  - in `double`: Text "ten" * Int 2
  |
  = note: from how `double` uses `x`, it needs: `fn double(x: Int|Float) -> Int|Float`
  = help: to repeat a text or a list, use `repeat(x, n)`
  = more about this error: `rythme explain E005`
```

- **Where?** `double.ry:3:7`, then line 1, inside the function.
- **What kind?** `E005`, explained by `rythme explain E005`.
- **Why?** We multiply a text (`"ten"`) by a number. You need to
  write `double(10)`.

> **Remember:** where, what kind, why. Find these three things
> before looking any further.

## Reading a traceback from bottom to top

When a function calls another function, the message shows **the whole
path**. It's called a **traceback** (or *stack trace*).

```python norun
def average(grades):
    return sum(grades) / len(grades)

def report(name, grades):
    print(name, ":", average(grades))

report("Ada", [12, 15])
report("Léo", [])
```

```text
Ada : 13.5
Traceback (most recent call last):
  File "/home/ada/average.py", line 8, in <module>
    report("Léo", [])
    ~~~~~~^^^^^^^^^^^
  File "/home/ada/average.py", line 5, in report
    print(name, ":", average(grades))
                     ~~~~~~~^^^^^^^^
  File "/home/ada/average.py", line 2, in average
    return sum(grades) / len(grades)
           ~~~~~~~~~~~~^~~~~~~~~~~~~
ZeroDivisionError: division by zero
```

`most recent call last` means the most recent call comes last. So the
most important line is **at the very bottom**. Then work your way up:

| Reading order | Line | What it says |
|---|---|---|
| 1 | `ZeroDivisionError: division by zero` | we divided by zero |
| 2 | `line 2, in average` | in `average`, `len(grades)` was 0 |
| 3 | `line 5, in report` | `average` was called by `report` |
| 4 | `line 8, in <module>` | it all started with `report("Léo", [])` |

Léo has no grades: the list is empty, its length is 0. Like a stack
of plates, the last one put down (at the bottom of the message) is the first one to
look at.

Rust, Go and JavaScript do the opposite: the error is **at the top**. When in
doubt, look for the line that contains `Error` or `error`.

## Three moments when things break

### Before starting: the syntax error

[Syntax](glossaire.md#syntax) is the grammar of the language. If a
sign is missing, nothing starts:

```python norun
age = 17
if age >= 18
    print("adult")
```

```text
  File "/home/ada/age.py", line 2
    if age >= 18
                ^
SyntaxError: expected ':'
```

`expected ':'`: Python was expecting a colon at the end of the `if`.

### During: the runtime error

The program starts, then runs into an impossible operation. That's the
example of Léo's average: `Ada : 13.5` is printed, **then** the error
arrives.

### No message: the wrong result

The worst case: everything works, but the result is wrong. We want the total of
three prices:

```python
total = 0
for price in [3, 4, 5]:
    total = price
print(total)
```

```sortie
5
```

We expected 12. There's no message at all: you have to print the values to
understand. Here, you needed `total = total + price`.

> **Remember:** syntax = doesn't start; runtime = stops along the way;
> wrong result = no message, it's up to you to search.

## What to do after reading the message

1. **Reread the line shown, and the one before.** The computer points to
   the place where it got lost, not always the place of the mistake.
2. **Search for the message online.** Copy the general part, without your
   own names: `ZeroDivisionError: division by zero`, not `average.py`.
3. **Print the values.** A `print(grades)` right before the faulty line
   shows `[]`, and everything becomes clear.
4. **Fix the first error**, run again, then move on to the next one.

The techniques for hunting down a [bug](glossaire.md#bug) are in [Finding
and fixing your mistakes](deboguer.md).

## Common mistakes

### Looking only at the line shown

```javascript norun
const grades = [12, 15
console.log(grades);
```

```text
file:///home/ada/grades.mjs:2
console.log(grades);
^^^^^^^

SyntaxError: Unexpected identifier 'console'
```

The message shows line 2, which is correct. It's line 1 that forgets
the `]`. JavaScript only notices when it sees `console`, which it
wasn't expecting (*unexpected*). Fix: `const grades = [12, 15];`.

### Reading only the top of a Python message

```python norun
def find(name):
    print("searching for", name)

result = find("Ada")
print(result.upper())
```

```text
searching for Ada
Traceback (most recent call last):
  File "/home/ada/find.py", line 5, in <module>
    print(result.upper())
          ^^^^^^^^^^^^
AttributeError: 'NoneType' object has no attribute 'upper'
```

`Traceback` at the top says nothing. The explanation is **at the bottom**: `result`
is `None` ("nothing"), because `find` doesn't return any value. Fix:
add `return name` at the end of the function.

## Test yourself

**1.** What are the three things to look for in an error message?

> **Answer**
>
> Where (the file and the line), what kind (the name of the error, like
> `TypeError`), and why (the sentence that explains).

**2.** In a Python traceback with several `File …` lines, where do you start?

> **Answer**
>
> At the bottom: the last line gives the error, the `File` line just
> above gives the place where it happened. Then you work your way up to
> see what called what.

**3.** The message points to line 12, but it looks perfect to you. What
do you do?

> **Answer**
>
> You look at line 11, and the ones before: a parenthesis, a
> bracket or a colon forgotten higher up is often the real cause.

## Summary

- An error message is help: it tells you where, what kind and why.
- In Python, read the traceback from bottom to top; in Rust, Go and JavaScript, the error is at the top.
- Syntax error: doesn't start; runtime error: stops along the way; wrong result: no message.
- Reread the line shown and the one before.
- Search for the general part of the message online, and print the values.

Next: [When you're stuck: a step-by-step method](bloque.md).
