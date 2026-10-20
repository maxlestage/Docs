# Your own types and match

This page shows how to create your own types with `type`, and how to examine them with `match` and its patterns: lists, records, guards and symbols.

> **Goals:**
>
> - you will know how to create a type with its variants, with or without fields;
> - you will know how to handle each variant with `match`, or with a definition by cases;
> - you will know how to recognize lists and records with patterns;
> - you will know how to add a guard to a `match` case;
> - you will know when to prefer a symbol or a `type`.

## Creating a type

A [type](../debuter/glossaire.md#type) is a kind of value. Rythme already
has some (`Int`, `Text`...), and you can create your own.

`type` creates a new type, with the list of its **variants**, the possible
shapes of its values, separated by `|` (read "or"). The names of the type
and of the variants start with an uppercase letter:

```rythme
type Light = Red | Orange | Green

light = Orange
print(light, type_of(light))
```

```sortie
Orange Light
```

Read: "a `Light` is `Red`, or `Orange`, or `Green`". A light cannot be
anything else: no typo is possible, as there would be with a text like
`"ornage"`.

A variant can carry **fields**: it is then used like a function that builds
a value. You read a field with a dot, like in a record:

```rythme
type Shape = Circle(radius) | Rect(width, height)

c = Circle(2)
r = Rect(3, 4)
print(c, r, r.width)
```

```sortie
Circle(2) Rect(3, 4) 3
```

Think of a form with several templates: the "circle" template has one
box, `radius`, the "rectangle" template has two boxes, `width` and `height`.

## Examining a value with match

`match` recognizes each variant, and gives a name to its fields:

```rythme
type Shape = Circle(radius) | Rect(width, height)

fn area(shape) = match shape
  Circle(r) -> 3 * r * r
  Rect(w, h) -> w * h
end

print(area(Circle(2)), area(Rect(3, 4)))
```

```sortie
12 12
```

In the case `Rect(w, h)`, `w` receives the width and `h` the height: you
choose the names.

You can also write a definition by cases, without `match`:

```rythme
type Shape = Circle(radius) | Rect(width, height)

fn perimeter(Circle(r)) = 2 * pi * r
fn perimeter(Rect(w, h)) = 2 * (w + h)

print(round(perimeter(Circle(1)), 2), perimeter(Rect(3, 4)))
```

```sortie
6.28 14
```

`pi` is the constant 3.14159..., and `round(x, 2)` keeps two decimals.

## The checker spots forgotten cases

If you forget a variant, Rythme warns you **before** running the program.
It is a warning: the program runs anyway.

```rythme norun
type Light = Red | Orange | Green

fn next(light) = match light
  Red -> Green
  Green -> Orange
end

print(next(Red))
```

`rythme check main.ry`:

```text
warning[W002]: this `match` does not handle every case
 --> main.ry:3:18
  |
3 | fn next(light) = match light
  |                  ^^^^^^^^^^^
  |
  = note: not handled: `Orange`
  = help: add the missing cases, like `Orange -> ...`, or a last case `_ -> ...`

✓ main.ry: no errors, 1 warning(s)
```

All you need is to add the line `Orange -> Red`. This is a big advantage
of a `type`: Rythme knows all its cases, and reminds you of the ones you
forgot.

## List patterns

A **pattern** describes the shape of a value. A pattern can describe a list:
`[]` for the empty list, `[x]` for a single element, `[first, ..rest]`
for "at least one element".

```rythme
fn describe(list) = match list
  [] -> "empty"
  [x] -> "only {x}"
  [a, b] -> "{a} then {b}"
  [first, ..rest] -> "{first}, and {len(rest)} more"
end

print(describe([]))
print(describe([1]))
print(describe([1, 2]))
print(describe([1, 2, 3, 4]))
```

```sortie
empty
only 1
1 then 2
1, and 3 more
```

## Record patterns

`{name}` recognizes a record that has a `name` field, and names it.
`{age: 0}` requires an exact value. Other fields are allowed:

```rythme
fn greet(person) = match person
  {name, age: 0} -> "Welcome to the world, {name}!"
  {name, city} -> "Hello {name}, from {city}."
  {name} -> "Hello {name}."
end

print(greet({name: "Léa", age: 0}))
print(greet({name: "Ada", city: "London", age: 36}))
print(greet({name: "Alan", age: 41}))
```

```sortie
Welcome to the world, Léa!
Hello Ada, from London.
Hello Alan.
```

Alan has no `city` field: the second case does not match him, so he gets
the third one.

## Guards in match

After a pattern, `if condition` adds a condition, a **guard**:

```rythme
fn classify(n) = match n
  0 -> "zero"
  x if x < 0 -> "negative"
  x if x % 2 == 0 -> "even"
  _ -> "odd"
end

print([0, -4, 6, 7] |> map(classify))
```

```sortie
["zero", "negative", "even", "odd"]
```

Several possibilities are written with `|`:

```rythme
type Light = Red | Orange | Green

fn instruction(light) = match light
  Red | Orange -> "careful"
  Green -> "go"
end

print(instruction(Orange), instruction(Green))
```

```sortie
careful go
```

## Symbols

A **symbol**, like in Ruby, is a name used as a value: `:ok`,
`:pending`. It is written with `:` in front. It is handy for a state,
without creating a type:

```rythme
fn label(state) = match state
  :ok -> "done"
  :pending -> "waiting"
  _ -> "unknown"
end

order = {number: 12, state: :pending}
print(order.state, label(order.state), label(:lost))
```

```sortie
:pending waiting unknown
```

> **Note:** a symbol is never equal to a text: `:ok == "ok"` is
> `false`. For a fixed set of cases, a `type` is safer, because the
> checker spots forgotten cases.

## Common mistakes

### A type name in lowercase

```rythme norun
type light = red | green
```

```text
error[E001]: type names start with an uppercase letter
 --> main.ry:1:6
  |
1 | type light = red | green
  |      ^^^^^
  |
  = help: write `type Light = ...`
  = more about this error: `rythme explain E001`
```

The type and its variants take an uppercase letter: `type Light = Red | Green`.

### A forgotten case that really happens

```rythme norun
type Light = Red | Orange | Green

fn next(light) = match light
  Red -> Green
  Green -> Orange
end

print(next(Orange))
```

```text
warning[W002]: this `match` does not handle every case
 --> main.ry:3:18
  |
3 | fn next(light) = match light
  |                  ^^^^^^^^^^^
  |
  = note: not handled: `Orange`
  = help: add the missing cases, like `Orange -> ...`, or a last case `_ -> ...`

error[E006]: no case of this `match` handles Light Orange
 --> main.ry:3:24
  |
3 | fn next(light) = match light
  |                        ^^^^^ this value
  |
  = help: add a case for this value, or a last case `_ -> ...` that handles everything else
  = call trace (most recent first):
      in `next` called at main.ry:8:7
  = more about this error: `rythme explain E006`
```

The warning comes first; then, when `Orange` really shows up, the
program stops. Do not leave a W002 warning: add `Orange -> Red`.

### Forgetting a field of a variant

```rythme norun
type Shape = Circle(radius) | Rect(width, height)
r = Rect(3)
```

```text
error[E004]: `Rect` takes 2 arguments, but 1 was given
 --> main.ry:2:5
  |
2 | r = Rect(3)
  |     ^^^^^^^ 1 argument(s) here
  |
1 | type Shape = Circle(radius) | Rect(width, height)
  |                               ------------------- defined here
  |
  = help: usage: `Rect(width, height)`
  = more about this error: `rythme explain E004`
```

A variant with fields is like a function: give one value per field,
`Rect(3, 4)`.

### Putting _ first

```rythme norun
fn describe(n) = match n
  _ -> "other"
  0 -> "zero"
end
print(describe(0))
```

```text
warning[W004]: this case can never be reached
 --> main.ry:3:3
  |
3 |   0 -> "zero"
  |   ^
  |
2 |   _ -> "other"
  |   - this case already handles everything
  |
  = help: cases are tried in order: put `_` last
```

The program prints `other`: `_` catches everything, even 0. Always put
`_` last.

## Test yourself

**Question 1:** write a type `Coin` that is either `Heads` or `Tails`.

> **Answer**
>
> `type Coin = Heads | Tails`.

**Question 2:** what does this program print?

```rythme norun
fn head(list) = match list
  [] -> "nothing"
  [x, ..] -> "starts with {x}"
end
print(head([7, 8, 9]), head([]))
```

> **Answer**
>
> `starts with 7 nothing`. `[x, ..]` recognizes a list of at least one
> element, and names the first one `x`.

**Question 3:** why is a `type` safer than a symbol to describe the state
of an order?

> **Answer**
>
> Because Rythme knows all the variants of a `type`: it reports forgotten
> cases in a `match` (W002), and a misspelled name is an error. A
> misspelled symbol, like `:pendng`, goes unnoticed.

## Summary

- `type` creates a type with its variants: `type Light = Red | Orange | Green`.
- A variant can carry fields: `Circle(radius)`, read with a dot.
- `match` handles each variant, in order, and names its fields; `_`
  goes last.
- Patterns also recognize lists (`[first, ..rest]`) and records
  (`{name}`); an `if` guard adds a condition.
- `rythme check` reports forgotten cases in a `match`.

Next: [symbols and when to use them](symboles.md)
