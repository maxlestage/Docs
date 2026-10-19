# Conditions

This page shows how to ask a question with a comparison, then choose between several cases with `if`, `unless` and `match`.

> **Goals:**
>
> - you will know how to write a condition with `==`, `<`, `and`, `or` and `not`;
> - you will know how to choose a value with `if ... then ... else`, and why the `else` is required;
> - you will know how to compare a value with several cases using `match`.

## True or false: comparisons

A program often has to decide: do you need an umbrella? is the person an adult? To
decide, it first asks a question, and the answer is yes or no.

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

In daily life, you constantly choose depending on the situation: if it rains, you
take a coat, otherwise a jacket. `if` lets a program make the same kind of choice.

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

Line by line:

- `age = 20` gives the name `age` to the value 20;
- to the right of the `=`, `if` asks the question "is 20 at least 18?". The answer is
  `true`, so the `if` gives the value after `then`: `"adult"`;
- this value gets the name `status`;
- `print(status)` prints `adult`.

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

Often there are more than two cases. A number can be negative, zero or positive, and
you want a different word for each one, like on a school report.

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

Line by line:

- `fn sign(n) =` defines a function: it receives a number, named `n`;
- for `sign(-3)`, the first question `-3 < 0` is true: the function gives
  `"negative"` and does not read the rest;
- for `sign(0)`, `0 < 0` is false, so we move to the `else if`: `0 == 0` is true, so
  `"zero"`;
- for `sign(8)`, both questions are false: we reach the last `else`, which gives
  `"positive"`.

The conditions are tried from top to bottom, and the first true one wins.

The condition must be `true` or `false`. `if 1 then ...` is an error:
nothing is "true" by default, not a number, not a text, not a list.

**Your turn:** write a function `greeting(hour)` that gives "Good morning" if the hour
is before 12, "Good afternoon" if it is before 18, and "Good evening" otherwise. Print
`greeting(15)`.

> **Solution**
>
> ```rythme
> fn greeting(hour) =
>   if hour < 12 then "Good morning"
>   else if hour < 18 then "Good afternoon"
>   else "Good evening"
>
> print(greeting(15))
> ```
>
> ```sortie
> Good afternoon
> ```

## unless, the opposite of if

> **Note:** this part can wait. Read it once the basics feel solid.

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

Line by line:

- `fn day(n) = match n`: the function compares the number `n` with the patterns that
  follow;
- for `day(6)`, the first pattern `6` matches: the result is `"Saturday"`;
- for `day(2)`, neither `6` nor `7` matches. `_` accepts anything: the result is
  `"a weekday"`;
- `print` shows both results, separated by a space.

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

**Your turn:** write a function `season(month)` that gives "winter" for months 12, 1
and 2, "spring" from 3 to 5, "summer" from 6 to 8, and "autumn" otherwise. Print
`season(1)`, `season(7)` and `season(10)`.

> **Solution**
>
> ```rythme
> fn season(month) = match month
>   12 | 1 | 2 -> "winter"
>   3..5 -> "spring"
>   6..8 -> "summer"
>   _ -> "autumn"
> end
>
> print(season(1), season(7), season(10))
> ```
>
> ```sortie
> winter summer autumn
> ```

## Common mistakes

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

**Question 3:** what is `if 4 >= 4 and not (4 == 5) then "yes" else "no"`?

> **Answer**
>
> `"yes"`. `4 >= 4` is true; `4 == 5` is false, so `not (4 == 5)` is true.
> Both sides of the `and` are true: the `if` gives the value after `then`.

## Summary

- `==`, `!=`, `<`, `>`, `<=`, `>=` compare; `and`, `or` and `not`
  combine conditions.
- `if condition then a else b` gives a value, and the `else` is required;
  `unless` is its opposite.
- With `else if`, the conditions are tried from top to bottom: the first true
  one wins.
- `match` compares a value with patterns, in order; `_` catches everything
  else.

Next: [loops](boucles.md). Compare with conditions in
[Ruby](../ruby/conditions-boucles.md) and in [Rust](../rust/conditions-boucles.md).
