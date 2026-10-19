# Conditions and repetition

This page shows how to choose between several cases with `if`, `unless` and `match`, and how to repeat an action without a loop, with `each`, `map` and recursion.

> **Goals:**
>
> - you will know how to write a condition with `==`, `<`, `and`, `or` and `not`;
> - you will know how to choose a value with `if ... then ... else`, and why the `else` is required;
> - you will know how to compare a value with several cases using `match`;
> - you will know how to repeat an action without a loop, with `each`, `each_with_index` and `map`;
> - you will know how to write a "while" repetition with a recursive function.

## True or false: comparisons

A [condition](../debuter/glossaire.md#condition) is a question whose answer
is `true` or `false`, a [boolean](../debuter/glossaire.md#boolean). You ask
it with a comparison operator:

| Operator | Question | Example | Result |
|-----------|----------|---------|----------|
| `==` | equal? | `3 == 3` | `true` |
| `!=` | different? | `3 != 3` | `false` |
| `<`, `>` | smaller, greater? | `2 < 5` | `true` |
| `<=`, `>=` | smaller or equal, greater or equal? | `5 >= 5` | `true` |

> **Warning:** `=` gives a name, `==` compares. They are two different
> things.

To combine conditions, you use words:

- `a and b`: true if **both** are true;
- `a or b`: true if **at least one** is true;
- `not a`: the opposite of `a`.

```rythme
x = 7
print(x > 0 and x < 10, x < 0 or x == 7, not (x == 7))
```

```sortie
true true false
```

## if gives a value

`if` chooses between two values depending on a condition. In Rythme, `if`
is not a command: it is an **expression**. It gives a value, like a
calculation. So you can store it in a name:

```rythme
age = 20
status = if age >= 18 then "adult" else "minor"
print(status)
```

```sortie
adult
```

The form is always `if condition then value else other_value`. Read it as:
"if the age is at least 18, then "adult", otherwise "minor"".

## The else is required

Since `if` always gives a value, it needs both cases: what would `status`
be if the condition were false? So an `if` without `else` is an error (see
[common mistakes](#common-mistakes)).

When there is nothing to do in one case, give the empty value `()`, which
means "nothing interesting":

```rythme
x = 5
if x > 0 then print("positive") else ()
```

```sortie
positive
```

A branch contains a single expression. To do several things, group them in
a `do ... end` block: its last line gives the value.

```rythme
fn classify(n) =
  if n > 100 then do
    print("a big number!")
    "big"
  end else "small"

print(classify(500), classify(3))
```

```sortie
a big number!
big small
```

## Chaining conditions

`else if` chains several conditions. `then` and `else` can start a new
line:

```rythme
fn sign(n) =
  if n < 0 then "negative"
  else if n == 0 then "zero"
  else "positive"

print(sign(-3), sign(0), sign(8))
```

```sortie
negative zero positive
```

The conditions are tried from top to bottom, and the first true one wins.

The condition must be `true` or `false`. `if 1 then ...` is an error:
nothing is "true" by default, not a number, not a text, not a list.

## unless, the opposite of if

As in Ruby, `unless` is the opposite of `if`: it gives the first value when
the condition is **false**. It also always has an `else`:

```rythme
stock = 0
print(unless stock > 0 then "sold out" else "in stock")
```

```sortie
sold out
```

`unless stock > 0 then a else b` is exactly
`if stock > 0 then b else a`. Use whichever reads better.

## Choosing with match

When there are many cases, `match` is clearer than a series of `else if`.
It compares a value with several **patterns**, in order. The first one that
matches gives the result. `_` matches everything else:

```rythme
fn day(n) = match n
  6 -> "Saturday"
  7 -> "Sunday"
  _ -> "a weekday"
end

print(day(6), day(2))
```

```sortie
Saturday a weekday
```

Each line is written `pattern -> result`, and `end` closes the `match`.
Think of sorting mail: each letter goes into the first slot that fits it.

A pattern can also be a range of numbers (`16..20`, both ends included), or
several possibilities separated by `|`:

```rythme
fn grade(score) = match score
  16..20 -> "excellent"
  12..15 -> "good"
  10 | 11 -> "pass"
  _ -> "needs work"
end

print(grade(17), grade(13), grade(10), grade(8))
```

```sortie
excellent good pass needs work
```

A pattern can even recognize the beginning of a text:

```rythme
fn greet("Dr " + name) = "Hello Doctor {name}"
fn greet(name) = "Hello {name}"

print(greet("Dr Grace"), "/", greet("Ada"))
```

```sortie
Hello Doctor Grace / Hello Ada
```

`match` can do much more: see [your own types and match](types.md).

## No loops

Rythme has neither `for` nor `while`. If you write one, Rythme explains
what to do instead (see [common mistakes](#common-mistakes)).

Instead of saying "for each element, do this", you say "do this to the
whole list". It's like saying "wash all the plates" instead of "take plate
1, wash it, take plate 2...". Here are the tools.

## Doing something for each element with each

`each` calls a function for each element. It replaces the loop that prints.
Here, `1..3` is the **range** of numbers from 1 to 3, that is the list
`[1, 2, 3]`, and `fn(i) = ...` is a small function without a name (see
[functions](fonctions.md#anonymous-functions)):

```rythme
1..3 |> each(fn(i) = print("Round number {i}"))
```

```sortie
Round number 1
Round number 2
Round number 3
```

The same thing, Ruby style, with a `do |name| ... end` **block**: `|name|`
gives a name to the current element.

```rythme
["Ada", "Alan"].each do |name|
  puts "Hello, #{name}!"
end
```

```sortie
Hello, Ada!
Hello, Alan!
```

## Counting by 2, or backwards

`a..b` counts up by 1. `step_by(n)` keeps one element out of every `n`, and
`reverse` turns the list around:

```rythme
print(0..10 |> step_by(2))
print(reverse(1..5))
print(5..1)
```

```sortie
[0, 2, 4, 6, 8, 10]
[5, 4, 3, 2, 1]
[]
```

> **Warning:** `5..1` is empty. To count down, write `reverse(1..5)`.

## Knowing the position with each_with_index

`each_with_index` gives the function the element **and** its position,
starting from 0:

```rythme
["bread", "milk", "eggs"].each_with_index do |item, i|
  print("{i + 1}. {item}")
end
```

```sortie
1. bread
2. milk
3. eggs
```

## Transforming each element with map

`map` builds a **new** list, with the result of the function for each
element. It replaces the loop that fills a list:

```rythme
squares = 1..5 |> map(fn(x) = x * x)
print(squares)
```

```sortie
[1, 4, 9, 16, 25]
```

`each` is for **doing** something (printing); `map` is for **calculating**
a new list.

## Repeating with recursion

For a "while" loop, write a function that calls itself: this is
[recursion](../debuter/glossaire.md#recursion). There must always be a case
where it stops, otherwise it never stops:

```rythme
fn countdown(0) = print("Liftoff!")
fn countdown(n)
  print(n)
  countdown(n - 1)
end

countdown(3)
```

```sortie
3
2
1
Liftoff!
```

Here, the function has two definitions: one for `0` (the stopping case),
one for the other numbers. `countdown(3)` prints 3 then calls
`countdown(2)`, and so on down to 0. See
[functions](fonctions.md#defining-by-cases).

## Common mistakes

### Writing a for loop

```rythme norun
numbers = [1, 2, 3]
for n in numbers
  print(n)
end
```

```text
error[E001]: `for` is not used in Rythme
 --> main.ry:2:1
  |
2 | for n in numbers
  | ^^^ comes from another language
  |
  = help: Rythme has no loops. Work on the whole list instead:
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      or write a recursive function. See `rythme help functional`.
  = more about this error: `rythme explain E001`
```

Write `numbers.each do |n| print(n) end`, or
`numbers |> each(fn(n) = print(n))`. `while` gives the same error.

### Forgetting the else

```rythme norun
age = 20
print(if age >= 18 then "adult")
```

```text
error[E019]: this `if` has no `else`
 --> main.ry:2:7
  |
2 | print(if age >= 18 then "adult")
  |       ^^ every `if` must have an `else`
  |
2 | print(if age >= 18 then "adult")
  |                                - expected `else` here
  |
  = help: in Rythme, `if` always gives a value, so it needs both cases:
        if age >= 18 then "adult" else "minor"
  = more about this error: `rythme explain E019`
```

Add the other case: `else "minor"`, or `else ()` if there is nothing to do.

### Comparing with = instead of ==

```rythme norun
x = 5
if x = 5 then print("five") else ()
```

```text
error[E001]: expected `then`, found `=`
 --> main.ry:2:6
  |
2 | if x = 5 then print("five") else ()
  |      ^ expected `then` here
  |
  = help: to compare two values, use `==`
  = help: an `if` is written: `if condition then value else other_value`
  = more about this error: `rythme explain E001`
```

To compare, use two equal signs: `if x == 5 then ...`.

### Chaining two comparisons

```rythme norun
grade = 15
print(if 10 <= grade <= 20 then "ok" else "no")
```

```text
error[E001]: comparisons cannot be chained
 --> main.ry:2:22
  |
2 | print(if 10 <= grade <= 20 then "ok" else "no")
  |                      ^^
  |
  = help: combine them with `and`: `a < b and b < c`
  = more about this error: `rythme explain E001`
```

Write both questions and join them: `10 <= grade and grade <= 20`.
`&&` and `||` don't exist either: write `and` and `or`.

## Test yourself

**Question 1:** what is `unless 3 > 5 then "a" else "b"`?

> **Answer**
>
> `"a"`. The condition `3 > 5` is false, and `unless` gives the first value
> when the condition is false.

**Question 2:** what does this program print?

```rythme norun
fn size(n) = match n
  0 -> "nothing"
  1..9 -> "small"
  _ -> "big"
end
print(size(0), size(7), size(42))
```

> **Answer**
>
> `nothing small big`. Each number goes to the first pattern that fits it.

**Question 3:** how do you print the even numbers from 2 to 10, without a
loop?

> **Answer**
>
> For example `2..10 |> step_by(2) |> each(fn(n) = print(n))`, or
> `2..10 |> filter(is_even) |> each(fn(n) = print(n))`.

## Summary

- `==`, `!=`, `<`, `>`, `<=`, `>=` compare; `and`, `or` and `not`
  combine conditions.
- `if condition then a else b` gives a value, and the `else` is required;
  `unless` is its opposite.
- `match` compares a value with patterns, in order; `_` catches everything
  else.
- There are no loops: `each` does something for each element,
  `each_with_index` also gives its position, `map` transforms each element.
- `step_by` counts in steps, `reverse` counts backwards.
- For "while", you write a function that calls itself, with a case where it
  stops.

Next: [functions](fonctions.md). Compare with conditions and loops in
[Ruby](../ruby/conditions-boucles.md) and in
[Rust](../rust/conditions-boucles.md).
