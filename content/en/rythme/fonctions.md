# Functions

In Rythme, almost everything is a function: this page shows how to write them on one line or as a block, by cases, with guards, without a name, and how to call them.

> **Goals:**
>
> - you will know how to write a function on one line or as a block, and call it;
> - you will know the difference between a parameter and an argument;
> - you will know how to define a function by cases, with patterns and guards;
> - you will know how to write an anonymous function and give it to another function;
> - you will know why Rythme has no `return`.

## A function on one line

A [function](../debuter/glossaire.md#function) is a small machine: you give
it values, it gives back a result. Like a toaster: bread goes in, toast
comes out.

You write it with the `fn` keyword, the name, the parameters between
parentheses, `=`, then the result:

```rythme
fn double(x) = x * 2
fn area(width, height) = width * height

print(double(21), area(3, 4))
```

```sortie
42 12
```

Line by line:

- `fn double(x) = x * 2` creates a function named `double`. Nothing is printed yet:
  you have only written the recipe;
- `fn area(width, height) = …` creates a second function, with two parameters;
- `print(double(21), area(3, 4))` calls both functions: `x` receives 21, so
  `double(21)` is 42; `width` receives 3 and `height` receives 4, so `area(3, 4)`
  is 12;
- `print` shows both results, separated by a space.

There is no `return`: what follows `=` **is** the result.

**Your turn:** write a program that defines a function `perimeter`, which receives the
width and height of a rectangle and gives its perimeter (twice the sum of the two),
then prints `perimeter(3, 4)`.

> **Solution**
>
> ```rythme
> fn perimeter(width, height) = 2 * (width + height)
>
> print(perimeter(3, 4))
> ```
>
> ```sortie
> 14
> ```

## Parameters and arguments

A function lets you redo the same calculation with different values. Think of a form:
the boxes are always the same, but everyone writes their own answers in them.

Two words come up all the time:

- a [parameter](../debuter/glossaire.md#parameter) is the name written in
  the definition: `width` and `height` in `fn area(width, height)`;
- an [argument](../debuter/glossaire.md#argument) is the value given in the
  call: `3` and `4` in `area(3, 4)`.

In the call, `width` receives 3 and `height` receives 4, in order. You must
give exactly one argument per parameter.

**Calling** a function means writing its name followed by the arguments
between parentheses: `double(21)`. You can use the result anywhere a value
is allowed: `double(double(5))` is 20.

## A function as a block

Some calculations take several steps, like a recipe: you prepare, then you put things
together. A block function lets you write these steps one below the other.

When you need several lines, you remove the `=` and close with `end`.
**The last line gives the result**:

```rythme
fn introduce(name, age)
  year = 2024 - age
  "{name} was born around {year}."
end

print(introduce("Ada", 36))
```

```sortie
Ada was born around 1988.
```

Line by line:

- `fn introduce(name, age)` opens the block, and `end` closes it: everything in
  between is part of the function;
- `print(introduce("Ada", 36))` calls the function: `name` receives `"Ada"` and `age`
  receives 36;
- inside the function, `year = 2024 - age` computes 1988 and stores it in `year`;
- the text line is the last line of the block: it is the result, and `print` shows it.

Names defined inside a function, like `year`, only exist inside the
function: that is their [scope](../debuter/glossaire.md#scope). Outside,
`year` is unknown.

A function without parameters keeps its parentheses, in the definition as
well as in the call:

```rythme
fn hello() = "hello!"
print(hello())
```

```sortie
hello!
```

**Your turn:** write a program that defines a block function `price_with_tax(net)`:
it computes a 20 % tax (`net * 20 // 100`), adds it to the price, and gives a text
like "50 € before tax makes 60 € with tax.". Print `price_with_tax(50)`.

> **Solution**
>
> ```rythme
> fn price_with_tax(net)
>   tax = net * 20 // 100
>   total = net + tax
>   "{net} € before tax makes {total} € with tax."
> end
>
> print(price_with_tax(50))
> ```
>
> ```sortie
> 50 € before tax makes 60 € with tax.
> ```

## Defining by cases

Often, the answer depends on what the function receives: an empty list is not
described like a full one. Instead of a single definition full of questions, you
write one definition per case, like the rows of a price table.

A function can have several definitions, written one below the other.
Rythme tries them **in order** and takes the first one that matches:

```rythme
fn fib(0) = 0
fn fib(1) = 1
fn fib(n) = fib(n - 1) + fib(n - 2)

print(fib(10))
```

```sortie
55
```

Here, `fib(0)` takes the first definition, `fib(1)` the second, and all
other numbers the third. It's like a row of service counters: you go to the
first one that accepts your request.

Parameters are **patterns**, like in `match`. For example, you can
recognize an empty list, or a list with a single element:

```rythme
fn describe([]) = "an empty list"
fn describe([x]) = "a single element: {x}"
fn describe(list) = "{len(list)} elements"

print(describe([]), "/", describe([7]), "/", describe([1, 2, 3]))
```

```sortie
an empty list / a single element: 7 / 3 elements
```

## Guards

A pattern recognizes a shape (0, an empty list…), but not a comparison like "under
13 years old". To choose with a comparison, like a price that depends on age, you use
a guard.

A **guard** adds a condition to a definition: `if` after the parameters,
before the `=`. The definition is chosen only if the condition is true.

```rythme
fn category(age) if age < 13 = "child"
fn category(age) if age < 18 = "teenager"
fn category(_) = "adult"

print(category(8), category(15), category(40))
```

```sortie
child teenager adult
```

Line by line:

- `category(8)`: Rythme tries the first case; `8 < 13` is true, so it gives
  `"child"`;
- `category(15)`: `15 < 13` is false, Rythme moves on to the second case; `15 < 18` is
  true, so it gives `"teenager"`;
- `category(40)`: both guards are false; the last case accepts anything, so it gives
  `"adult"`;
- `print` shows the three results on one line.

`_` means "any value, I don't use it". Always put the most general case
last: otherwise, it would catch everything.

**Your turn:** write a program that defines a function `weather`, with guards: it
gives "cold" under 10 degrees, "mild" under 25, and "hot" otherwise. Print
`weather(5)`, `weather(18)` and `weather(30)`.

> **Solution**
>
> ```rythme
> fn weather(degrees) if degrees < 10 = "cold"
> fn weather(degrees) if degrees < 25 = "mild"
> fn weather(_) = "hot"
>
> print(weather(5), weather(18), weather(30))
> ```
>
> ```sortie
> cold mild hot
> ```

## Calls without parentheses

> **Note:** this part can wait. Read it once the basics feel solid.

At the start of a line, you can call a function without parentheses, as in
Ruby:

```rythme
fn greet(name, greeting) = print("#{greeting}, #{name}!")

greet "Ada", "Hello"
greet("Alan", "Hi")
```

```sortie
Hello, Ada!
Hi, Alan!
```

> **Warning:** this only works at the start of a statement (or of a
> result). In the middle of an expression, parentheses are still required:
> `x = len(xs)`, not `x = len xs`.

## Method-style calls

This way of writing reads from left to right, in the order you think: you take a
value, then you apply a function to it.

`value.f(args)` is another way to write `f(value, args)`: the value before
the dot becomes the first argument. Without other arguments, the
parentheses are optional:

```rythme
print("hi".upper(), [3, 1, 2].sort, 10.max(20))
```

```sortie
HI [1, 2, 3] 20
```

It works with your own functions too:

```rythme
fn double(x) = x * 2
print(21.double)
```

```sortie
42
```

## Names ending in ?

> **Note:** this part can wait. Read it once the basics feel solid.

A function that answers yes or no can end in `?`, as in Ruby:

```rythme
fn adult?(person) = person.age >= 18

ada = {name: "Ada", age: 36}
print(adult?(ada), ada.adult?, even?(4))
```

```sortie
true true true
```

`{name: "Ada", age: 36}` is a record, and `person.age` reads its `age`
field (see [collections](collections.md)).

## Anonymous functions

Sometimes a function is only used once, in a single place. Finding a name for it
would be like labeling a sticky note you are about to throw away.

A function without a name is written `fn(parameters) = result`. It's a
value like any other: you can store it in a name or give it to another
function.

```rythme
triple = fn(x) = x * 3
print(triple(5))
print([1, 2, 3] |> map(fn(x) = x + 10))
```

```sortie
15
[11, 12, 13]
```

Line by line:

- `triple = fn(x) = x * 3` stores a function without a name in `triple`; nothing is
  computed yet;
- `triple(5)` calls it like an ordinary function: `x` receives 5, the result is 15;
- `[1, 2, 3] |> map(fn(x) = x + 10)` gives the list to `map`, with a function written
  right there; `map` calls it on 1, then 2, then 3, and puts 11, 12 and 13 in a new
  list.

`map` applies the function to each element of the list. You often write an
anonymous function when it is only used once, right there.

**Your turn:** write a program that adds 5 to each price in the list `[10, 20, 30]`,
with `map` and an anonymous function, then prints the new list.

> **Solution**
>
> ```rythme
> print([10, 20, 30] |> map(fn(price) = price + 5))
> ```
>
> ```sortie
> [15, 25, 35]
> ```

## A function that remembers: the closure

Sometimes a function needs a value that is not one of its parameters: a rate, a
discount. A closure lets it carry that value along.

An anonymous function can use the names written before it:

```rythme
rate = 3
print([1, 2, 3] |> map(fn(x) = x * rate))
```

```sortie
[3, 6, 9]
```

A function can even build another function. The new function keeps the
value it received with it, like a backpack:

```rythme
fn adder(n) = fn(x) = x + n

plus10 = adder(10)
plus1 = adder(1)
print(plus10(5), plus1(5))
```

```sortie
15 6
```

Such a function, which carries values from the place where it was written,
is called a [closure](../debuter/glossaire.md#closure). Since values never
change, what it carries never changes either.

## Functions are values

This lets you write a function that knows how to do "something" without knowing what
in advance: you give it the action to perform, like giving a recipe to a cook.

A named function can also be given to another function, by its name,
**without parentheses**:

```rythme
fn square(x) = x * x
fn apply_twice(f, x) = f(f(x))

print(apply_twice(square, 3), map([1, 2, 3], square))
```

```sortie
81 [1, 4, 9]
```

Line by line:

- `square` gives the square of a number;
- `apply_twice(f, x)` receives a function `f` and calls it twice: `f(f(x))`;
- `apply_twice(square, 3)`: `square(3)` is 9, then `square(9)` is 81;
- `map([1, 2, 3], square)` applies `square` to each element: `[1, 4, 9]`.

`square` is the function itself; `square(3)` is its result, 9.

## Common mistakes

### Writing return

```rythme norun
fn double(x)
  return x * 2
end
print(double(4))
```

```text
error[E001]: `return` is not used in Rythme
 --> main.ry:2:3
  |
2 |   return x * 2
  |   ^^^^^^ comes from another language
  |
  = help: no `return` needed: the last expression of a function is its result.
      To stop early with an error, return `Err(...)` and use `?`.
  = more about this error: `rythme explain E001`
```

Remove `return`: the last line, `x * 2`, is already the result. Or write
everything on one line: `fn double(x) = x * 2`.

### Forgetting an argument

```rythme norun
fn area(width, height) = width * height
print(area(3))
```

```text
error[E004]: `area` takes 2 arguments, but 1 was given
 --> main.ry:2:7
  |
2 | print(area(3))
  |       ^^^^^^^ 1 argument(s) here
  |
1 | fn area(width, height) = width * height
  |    ---- defined here
  |
  = help: usage: `area(width, height)`
  = more about this error: `rythme explain E004`
```

Rythme has no default values: give one argument per parameter,
`area(3, 4)`.

### Forgetting the parentheses of a function without parameters

```rythme norun
fn hello = "hi"
print(hello())
```

```text
error[E001]: expected `(` and the parameters, found `=`
 --> main.ry:1:10
  |
1 | fn hello = "hi"
  |          ^ expected `(` and the parameters here
  |
  = help: even without parameters, write the parentheses: `fn greet() = print("hi")`
  = more about this error: `rythme explain E001`
```

Write `fn hello() = "hi"`.

### Forgetting the end of a block

```rythme norun
fn introduce(name)
  "Hello {name}"

print(introduce("Ada"))
```

```text
error[E020]: missing `end` to close this `fn`
 --> main.ry:1:1
  |
1 | fn introduce(name)
  | ^^ this `fn` is never closed
  |
5 | 
  | - the code ends here
  |
  = help: add `end` on its own line after the last line of the block
  = more about this error: `rythme explain E020`
```

A function without `=` is a block: close it with `end`, below its last
line.

## Test yourself

**Question 1:** in `fn price(quantity, unit_price) = quantity * unit_price`
and the call `price(3, 2)`, what are the parameters and the arguments?

> **Answer**
>
> The parameters are `quantity` and `unit_price`; the arguments are `3` and
> `2`. The result is 6.

**Question 2:** what does this program print?

```rythme norun
fn sign(n) if n < 0 = "negative"
fn sign(0) = "zero"
fn sign(_) = "positive"
print(sign(0), sign(-2), sign(5))
```

> **Answer**
>
> `zero negative positive`. The definitions are tried in order: `0` is not
> negative, so it takes the second one.

**Question 3:** what is `adder(3)(4)`, with
`fn adder(n) = fn(x) = x + n`?

> **Answer**
>
> `7`. `adder(3)` gives a function that adds 3; it is called right away
> with 4.

## Summary

- `fn name(x) = ...` writes a function on one line; `fn name(x) ... end`
  as a block, where the last line is the result.
- There is no `return`; you need one argument for each parameter.
- Several definitions by cases are tried in order; an `if` guard adds a
  condition.
- `value.f(args)` is another way to write `f(value, args)`.
- `fn(x) = ...` is a function without a name, which you can give to `map`;
  it carries the names written before it (a closure).

Next: [working with text](texte.md). Compare with functions in
[Ruby](../ruby/fonctions.md) and in [Rust](../rust/fonctions.md).
