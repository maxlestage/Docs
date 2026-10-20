# Pattern matching

A pattern describes the shape of a value: this page goes through every kind of pattern in Rythme, in `match` as well as in function parameters.

> **Goals:**
>
> - you will know how to recognize a number, a range, a text or the start of a text;
> - you will know how to take apart a list or a record with a pattern;
> - you will know how to combine patterns with `|`, guards and nested patterns;
> - you will know how to read the checker's W002 and W004 warnings;
> - you will know how to use patterns in the parameters of a function.

## Reminder: match

[`match`](../debuter/glossaire.md#match) compares a value with a series of
**patterns**, in order. The first one that fits gives the result. It is
like a coin sorter: each coin falls into the first hole that fits its
size.

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

Each case is written `pattern -> result`, and `end` closes the `match`. A
pattern computes nothing: it describes a **shape**, and gives names to the
pieces.

## Exact values

The simplest pattern is a value written as is: a number, a text, `true` or
`false`, `()`, a symbol. `_` fits anything, without giving a name.

```rythme
fn answer(x) = match x
  0 -> "zero"
  2.5 -> "two and a half"
  "yes" -> "all right"
  true -> "true"
  () -> "nothing"
  :ok -> "the symbol ok"
  _ -> "something else"
end

print([0, 0.0, 2.5, "yes", true, (), :ok, 99] |> map(answer))
```

```sortie
["zero", "zero", "two and a half", "all right", "true", "nothing", "the symbol ok", "something else"]
```

The pattern `0` also accepts `0.0`: equal numbers are worth the same.

A **name** also fits anything, but it keeps the value:

```rythme
fn describe(x) = match x
  0 -> "nothing"
  n -> "the number {n}"
end

print(describe(0), "/", describe(42))
```

```sortie
nothing / the number 42
```

## Ranges

`a..b` fits a number from `a` to `b`, **both ends included**. The bounds
are numbers written in the code.

```rythme
fn grade(score) = match score
  16..20 -> "very good"
  12..15 -> "good"
  10..11 -> "pass"
  0..9 -> "fail"
  _ -> "not a score"
end

print([17, 12, 10, 3, 25] |> map(grade))
```

```sortie
["very good", "good", "pass", "fail", "not a score"]
```

A range also accepts Floats: `15.5` is in none of these ranges, so it
would go to `_`. The last case `_` is essential, because no range covers
every number.

## The start of a text

`"Mr " + name` fits a text that **starts** with `"Mr "`, and names the
rest `name`. It is very handy to read commands:

```rythme
fn command("/help") = "here is the help"
fn command("/say " + message) = "you say: {message}"
fn command("/" + _) = "unknown command"
fn command(text) = "message: {text}"

print(command("/help"))
print(command("/say hello"))
print(command("/dance"))
print(command("hi"))
```

```sortie
here is the help
you say: hello
unknown command
message: hi
```

Only the **start** of a text can be used as a pattern. For the end, use a
guard (see below) with `ends_with`.

## Lists

A list pattern describes its length and its elements:

| Pattern | Fits |
|---------|------|
| `[]` | the empty list |
| `[x]` | a list of exactly one element |
| `[x, y]` | a list of exactly two elements |
| `[first, ..rest]` | a list of at least one element; `rest` is what follows |
| `[a, b, ..]` | at least two elements; what follows has no name |

```rythme
fn describe(list) = match list
  [] -> "empty"
  [x] -> "only one: {x}"
  [x, y] -> "two: {x} and {y}"
  [first, ..rest] -> "{first} then {len(rest)} others"
end

print(describe([]), "/", describe([7]), "/", describe([7, 8]), "/", describe([1, 2, 3, 4]))
```

```sortie
empty / only one: 7 / two: 7 and 8 / 1 then 3 others
```

`..rest` must come last. This pattern is what makes
[recursion](recursion.md) work on lists.

## Records

`{name}` fits a record that has **at least** a `name` field, and gives
that field under the name `name`. `{age: 0}` requires the `age` field to
be 0. The record may have other fields.

```rythme
fn card(p) = match p
  {name, age: 0} -> "{name} was just born"
  {name, city: "Paris"} -> "{name} lives in Paris"
  {name, age} -> "{name} is {age} years old"
  {} -> "not a person"
end

print(card({name: "Léo", age: 0}))
print(card({name: "Ada", age: 36, city: "Paris"}))
print(card({name: "Alan", age: 41}))
print(card({title: "Book"}))
```

```sortie
Léo was just born
Ada lives in Paris
Alan is 41 years old
not a person
```

`{}` fits any record.

## Constructors

The variants of a type (see [your own types](types.md)), and `Some`,
`None`, `Ok`, `Err`, are recognized by their name. Their fields are
patterns in turn:

```rythme
type Shape = Circle(radius) | Rect(width, height) | Point

fn area(s) = match s
  Circle(r) -> round(pi * r ** 2, 2)
  Rect(w, h) -> w * h
  Point -> 0
end

print(map([Circle(1), Rect(2, 3), Point], area))
```

```sortie
[3.14, 6, 0]
```

## Several possibilities with |

`p1 | p2` fits if either of the two patterns fits:

```rythme
fn kind(day) = match day
  "Saturday" | "Sunday" -> "weekend"
  _ -> "week"
end

print(kind("Saturday"), kind("Monday"))
```

```sortie
weekend week
```

Patterns joined by `|` cannot give names (see the common mistakes).

## Guards

A **guard** is a condition added after the pattern with `if`. The case is
only chosen if the pattern fits **and** the condition is `true`. The names
from the pattern can be used in the guard.

```rythme
type Shape = Circle(radius) | Rect(width, height) | Point

fn nature(s) = match s
  Rect(w, h) if w == h -> "a square with side {w}"
  Rect(w, h) -> "a {w} × {h} rectangle"
  Circle(_) | Point -> "round or point"
end

print(nature(Rect(2, 2)), "/", nature(Rect(2, 5)), "/", nature(Point))
```

```sortie
a square with side 2 / a 2 × 5 rectangle / round or point
```

A guard is also useful for the end of a text, or to compare with a value
that is not written in the code:

```rythme
fn tone(sentence) = match sentence
  t if ends_with(t, "?") -> "question"
  t if ends_with(t, "!") -> "exclamation"
  _ -> "statement"
end

print(tone("Really?"), tone("Well done!"), tone("It is raining."))
```

```sortie
question exclamation statement
```

## Nested patterns

Patterns fit inside each other like boxes in boxes: a record that contains
a record and a list, a list of `Result`...

```rythme
fn summary(order) = match order
  {customer: {name}, items: [first, ..]} -> "{name} first orders {first}"
  {customer: {name}, items: []} -> "{name} ordered nothing"
  _ -> "unreadable order"
end

print(summary({customer: {name: "Ada", city: "London"}, items: ["tea", "scones"]}))
print(summary({customer: {name: "Alan"}, items: []}))

fn first_result([Ok(v), ..]) = "first result: {v}"
fn first_result([Err(e), ..]) = "error: {e}"
fn first_result([]) = "nothing"
print(first_result([Ok(3), Err("x")]), "/", first_result([Err("oops")]))
```

```sortie
Ada first orders tea
Alan ordered nothing
first result: 3 / error: oops
```

## Patterns in parameters

The parameters of a function are patterns too. A function defined by
cases is therefore a hidden `match`:

```rythme
fn distance({x: x1, y: y1}, {x: x2, y: y2}) = sqrt((x2 - x1) ** 2 + (y2 - y1) ** 2)
print(distance({x: 0, y: 0}, {x: 3, y: 4}))

fn head([x, ..]) = Some(x)
fn head([]) = None
print(head([5, 6]), head([]))
```

```sortie
5.0
Some(5) None
```

Finally, patterns let you name several pieces at once:

```rythme
[a, b, ..] = [10, 20, 30]
{name} = {name: "Ada", age: 36}
print(a, b, name)
```

```sortie
10 20 Ada
```

## The checker helps you

Before running the program, Rythme reads your `match` expressions again.
It points out two common oversights.

**W002: a forgotten case.** Here, the orange light is not handled:

```rythme norun
type Light = Red | Orange | Green
fn action(light) = match light
  Red -> "stop"
  Green -> "go"
end
print(action(Green))
```

```text
warning[W002]: this `match` does not handle every case
 --> main.ry:2:20
  |
2 | fn action(light) = match light
  |                    ^^^^^^^^^^^
  |
  = note: not handled: `Orange`
  = help: add the missing cases, like `Orange -> ...`, or a last case `_ -> ...`

go
```

The program still runs, but `action(Orange)` would stop it with error
E006. Add the case `Orange -> "slow down"`.

**W004: a case never reached.** Cases are tried in order: a `_` placed
first takes everything.

```rythme norun
fn size(n) = match n
  _ -> "unknown"
  0 -> "zero"
end
print(size(0))
```

```text
warning[W004]: this case can never be reached
 --> main.ry:3:3
  |
3 |   0 -> "zero"
  |   ^
  |
2 |   _ -> "unknown"
  |   - this case already handles everything
  |
  = help: cases are tried in order: put `_` last

unknown
```

Put the most precise cases at the top, and `_` at the very bottom.

> **Remember:** a warning does not stop the program, but it almost always
> points to a real problem. Read them.

## Common mistakes

### Using the same name twice

```rythme norun
fn equal([a, a]) = true
fn equal(_) = false
print(equal([1, 1]))
```

```text
error[E011]: `a` appears twice in this pattern
 --> main.ry:1:14
  |
1 | fn equal([a, a]) = true
  |              ^
  |
1 | fn equal([a, a]) = true
  |           - first here
  |
  = help: use two different names, then compare them with `==` in a condition: `[a, b] if a == b -> ...`
  = more about this error: `rythme explain E011`
```

A pattern does not compare two pieces with each other. Write
`fn equal([a, b]) if a == b = true`.

### Recognizing the end of a text

```rythme norun
fn tone(t) = match t
  _ + "!" -> "exclamation"
  _ -> "other"
end
print(tone("oh!"))
```

```text
error[E001]: only a text can come before `+` in a pattern
 --> main.ry:2:3
  |
2 |   _ + "!" -> "exclamation"
  |   ^^^
  |
  = help: a text that starts with a text is written `"Mr " + name`: `name` is the rest of the text
  = help: to look at the end of a text, use a guard: `s if ends_with(s, "!") -> ...`
  = more about this error: `rythme explain E001`
```

Only the start of a text can be recognized. For the end: `t if ends_with(t, "!") -> ...`.

### Giving a name in a pattern with |

```rythme norun
fn start(list) = match list
  [x] | [x, _] -> x
  _ -> 0
end
print(start([4, 5]))
```

```text
error[E001]: patterns joined with `|` cannot give names (here `x`)
 --> main.ry:2:4
  |
2 |   [x] | [x, _] -> x
  |    ^
  |
  = help: write separate cases instead
  = more about this error: `rythme explain E001`
```

Write two cases: `[x] -> x` then `[x, _] -> x`.

## Test yourself

**Question 1:** which values do the patterns `[x]`, `[x, ..]` and
`[x, ..rest]` accept?

> **Answer**
>
> `[x]`: a list of exactly one element. `[x, ..]` and `[x, ..rest]`: a
> list of at least one element; the second one also names what follows.

**Question 2:** what does `classify(15)` give?

```rythme norun
fn classify(n) = match n
  0..9 -> "small"
  10..99 if n % 5 == 0 -> "round"
  10..99 -> "medium"
  _ -> "large"
end
```

> **Answer**
>
> `"round"`: 15 is in `10..99`, and the guard `15 % 5 == 0` is `true`.

**Question 3:** what does the W004 warning point out?

> **Answer**
>
> A case that can never be chosen, because a case placed before it already
> accepts the same values (often a `_` put too early).

## Summary

- A pattern describes a shape: exact value, range `a..b`, start of a text
  `"Mr " + name`, list, record, constructor.
- `[x, ..rest]` takes a list apart; `{name}` takes a field; `_` accepts
  anything without a name.
- `|` offers several possibilities (without names); `if` adds a guard.
- Patterns nest, and also work in function parameters and in
  `[a, b] = list`.
- W002 points out a forgotten case, W004 a case never reached.

Next: [handling errors](erreurs.md)
