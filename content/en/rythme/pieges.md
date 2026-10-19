# Beginner traps in Rythme

This page gathers the twelve surprises that most often block beginners in Rythme, especially those coming from another language, each with the real message and the right way to write it.

> **Goals:**
>
> - you will know why a value never changes, and what to write instead;
> - you will know how to replace a loop and a `null` value;
> - you will know how to avoid the number traps: `/`, `//` and `0.1 + 0.2`;
> - you will know why a text and a number do not mix;
> - you will know how to read Rythme's messages when you fall into a trap.

## How to read this page

Each trap follows the same order:

```text
1. What you expect           (the habit from another language)
2. What Rythme does          (the code and its real message)
3. Why                       (a sentence or two)
4. The fix                   (the code that works)
```

You do not need to read everything at once. Come back here when a message
surprises you.

> **Remember:** Rythme almost always warns you. Read the `help:` line of the
> message: it often gives the fix.

## Value traps

### 1. You cannot change the value of a name

**You expect:** `total = total + 1` adds 1 to `total`, as in almost every
language.

```rythme norun
total = 1
total = total + 1
print(total)
```

```text
error[E011]: `total` is already defined
 --> main.ry:2:1
  |
2 | total = total + 1
  | ^^^^^ defined again here
  |
1 | total = 1
  | ----- first defined here
  |
  = help: values never change in Rythme: give the new value a new name, like `total2 = total + 1`
  = more about this error: `rythme explain E011`
```

**Why:** in Rythme, a [value](../debuter/glossaire.md#value) is
[immutable](../debuter/glossaire.md#immutable): it never changes. A name is
a label stuck for good on a box.

```text
 total ──► [ 1 ]        total2 ──► [ 2 ]
 (label stuck on)       (new label, new box)
```

**The fix:** give the new value a new name.

```rythme
total = 1
total2 = total + 1
print(total, total2)
```

```sortie
1 2
```

`+=` and `++` do not exist either, for the same reason.

Try it: replace `total2` with `total` on the second line. Rythme refuses the
program before it even runs it.

### 2. push and put change nothing

**You expect:** `push(shopping, "eggs")` adds the eggs to the list.

```rythme
shopping = ["bread", "milk"]
push(shopping, "eggs")
print(shopping)
```

```sortie
["bread", "milk"]
```

The eggs are gone! Yet Rythme warned you, with a warning:

```text
warning[W008]: the result of `push` is lost
 --> main.ry:2:1
  |
2 | push(shopping, "eggs")
  | ^^^^^^^^^^^^^^^^^^^^^^ this value is lost
  |
  = help: values never change: `push(list, x)` gives a new value, and changes nothing.
      To keep it, give it a name: `new = push(list, x)`
```

**Why:** `push` makes a **new** list. Since nobody gives it a name, it is
lost. The same goes for `put` on a
[dictionary](../debuter/glossaire.md#dictionary).

**The fix:** keep the result under a new name.

```rythme
shopping = ["bread", "milk"]
shopping2 = push(shopping, "eggs")
print(shopping)
print(shopping2)
```

```sortie
["bread", "milk"]
["bread", "milk", "eggs"]
```

It is like a photocopy of the shopping list: you write on the copy, the
original does not move.

A field of a [record](../debuter/glossaire.md#record) cannot be
changed either:

```rythme norun
ada = {name: "Ada", age: 36}
ada.age = 37
```

```text
error[E001]: cannot give a value to this
 --> main.ry:2:1
  |
2 | ada.age = 37
  | ^^^^^^^ this is not a name
  |
2 | ada.age = 37
  |         - `=` gives a name to a value
  |
  = help: records cannot be changed; build a new one: `new = {..old, age: value}`
  = more about this error: `rythme explain E001`
```

You build a copy with one field changed:

```rythme
ada = {name: "Ada", age: 36}
ada2 = {..ada, age: 37}
print(ada, ada2)
```

```sortie
{name: "Ada", age: 36} {name: "Ada", age: 37}
```

> **Remember:** in Rythme, nothing changes. You make a new value, and you
> give it a new name.

### 3. There is no for or while loop

**You expect:** a `for` [loop](../debuter/glossaire.md#loop) to
repeat.

```rythme norun
for x in [1, 2]
  print(x)
end
```

```text
error[E001]: `for` is not used in Rythme
 --> main.ry:1:1
  |
1 | for x in [1, 2]
  | ^^^ comes from another language
  |
  = help: Rythme has no loops. Work on the whole list instead:
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      or write a recursive function. See `rythme help functional`.
  = more about this error: `rythme explain E001`
```

**Why:** a loop changes a counter at each turn. But nothing changes in
Rythme. Instead, you say "do this for the whole list".

**The fix:** `each` to **do** something with each element, `map` to
**compute** a new list.

```rythme
[1, 2, 3] |> each(fn(x) = print("element {x}"))
doubles = [1, 2, 3] |> map(fn(x) = x * 2)
print(doubles)
```

```sortie
element 1
element 2
element 3
[2, 4, 6]
```

Here is what `map` does, element by element:

| Step | x | x * 2 | List built |
|------|---|-------|------------|
| 1 | 1 | 2 | `[2]` |
| 2 | 2 | 4 | `[2, 4]` |
| 3 | 3 | 6 | `[2, 4, 6]` |

For a "while" loop, write a function that calls itself
(see [loops](boucles.md#repeating-with-recursion)).

Try it: replace `x * 2` with `x * x`. You get `[1, 4, 9]`.

### 4. There is no null

**You expect:** `null` (or `nil`, `None` in Python) to say "no
value".

```rythme norun
answer = null
```

```text
error[E001]: `null` is not used in Rythme
 --> main.ry:1:10
  |
1 | answer = null
  |          ^^^^ comes from another language
  |
  = help: Rythme has no null: use `None` for "no value" and `Some(x)` for a value
  = more about this error: `rythme explain E001`
```

**Why:** a forgotten [null](../debuter/glossaire.md#null) is the cause of
many crashes. Instead, Rythme puts the value in a box, an
[option](../debuter/glossaire.md#option): `Some(value)` when there is
something, `None` when the box is empty.

```rythme
print(first(["a", "b"]), first([]))
```

```sortie
Some("a") None
```

You do not compute directly with the box: `first([]) + 1` is an error. You
open it with `unwrap_or`, giving a fallback value:

```rythme
nothing = first([]) |> unwrap_or(0)
something = first([5, 6]) |> unwrap_or(0)
print(nothing + 1, something + 1)
```

```sortie
1 6
```

Options are explained in detail in [handling errors](erreurs.md).

> **Remember:** no `for`, no `null`. Rythme tells you what to write instead,
> in the `help:` line.

## Number and text traps

### 5. Division / always gives a decimal number

**You expect:** `6 / 3` is `2`.

```rythme
print(6 / 3, 7 / 2, 7 // 2)
```

```sortie
2.0 3.5 3
```

**Why:** `/` **always** gives a decimal number, a
[float](../debuter/glossaire.md#float) (`Float`), even when the division
comes out even. For an [integer](../debuter/glossaire.md#integer) (`Int`),
use integer division `//`.

The trap appears when you use the result as a position:

```rythme norun
grades = [12, 15, 9, 18]
middle = len(grades) / 2
print(grades[middle])
```

```text
error[E005]: an index must be a whole number (Int), but got a value of type Float
 --> main.ry:3:14
  |
3 | print(grades[middle])
  |              ^^^^^^
  |
  = more about this error: `rythme explain E005`
```

**The fix:** `//` gives an integer.

```rythme
grades = [12, 15, 9, 18]
middle = len(grades) // 2
print(middle, grades[middle])
```

```sortie
2 9
```

Try it: print `type_of(4 / 2)` then `type_of(4 // 2)`. You read `Float` then
`Int`.

### 6. 0.1 + 0.2 is not quite 0.3

**You expect:** `0.1 + 0.2 == 0.3` is `true`.

```rythme
print(0.1 + 0.2, 0.1 + 0.2 == 0.3)
```

```sortie
0.30000000000000004 false
```

**Why:** the computer stores decimal numbers in binary, with a limited
number of digits. Some numbers, like 0.1, do not come out exact, just like
1/3 which is written 0.3333... in decimal. This is not specific to Rythme:
Python, JavaScript and Ruby do the same.

**The fix:** round before comparing, or count in integers (cents instead of
dollars).

```rythme
print(round(0.1 + 0.2, 2) == 0.3, 10 + 20 == 30)
```

```sortie
true true
```

### 7. A text and a number do not add up

**You expect:** `"age: " + 36` gives `"age: 36"`.

```rythme norun
age = 36
print("age: " + age)
```

```text
error[E005]: cannot use `+` with Text and Int
 --> main.ry:2:15
  |
2 | print("age: " + age)
  |               ^ Text "age: " + Int
  |
  = help: to build a text with a value inside, use braces: "total: {x}"
      or convert the value first: text(x)
  = more about this error: `rythme explain E005`
```

**Why:** Rythme never guesses. `"3" + 4`, is it `"34"` or `7`? Rather than
choosing silently, it refuses. For the same reason, a text is never equal to
a number:

```rythme
print("1" == 1, 1 == 1.0)
```

```sortie
false true
```

**The fix:** put the value between braces, or convert it with `text`.

```rythme
age = 36
print("age: {age}", "age: " + text(age))
```

```sortie
age: 36 age: 36
```

The other way round, `parse_int` reads a number from a text. But it gives a
box `Ok(42)`, not `42`, because the text may not be a number:

```rythme
print(parse_int("42"), parse_int("hello"))
n = parse_int("42") |> unwrap_or(0)
print(n + 1)
```

```sortie
Ok(42) Err("\"hello\" is not a whole number")
43
```

> **Remember:** a text stays a text, a number stays a number. To go from one
> to the other: `"{x}"` or `text(x)` one way, `parse_int` the other way.

## Writing traps

### 8. An if always has an else

**You expect:** an `if` without `else`, which does nothing when the
condition is false.

```rythme norun
x = 5
if x > 0 then print("positive")
```

```text
error[E019]: this `if` has no `else`
 --> main.ry:2:1
  |
2 | if x > 0 then print("positive")
  | ^^ every `if` must have an `else`
  |
3 | 
  | - expected `else` here
  |
  = help: in Rythme, `if` always gives a value, so it needs both cases:
        if age >= 18 then "adult" else "minor"
  = more about this error: `rythme explain E019`
```

**Why:** in Rythme, `if` gives a value, like a calculation. It needs a value
for both cases.

**The fix:** when there is nothing to do, give the empty value `()`.

```rythme
x = 5
if x > 0 then print("positive") else ()
```

```sortie
positive
```

Another surprise: the [condition](../debuter/glossaire.md#condition) must
be `true` or `false`. An empty list or the number 0 do not mean
"false":

```rythme norun
basket = []
print(if basket then "full" else "empty")
```

```text
error[E005]: a condition must be `true` or `false`, but this is a value of type List
 --> main.ry:2:10
  |
2 | print(if basket then "full" else "empty")
  |          ^^^^^^ not a Bool
  |
  = help: to test if it is empty, use `is_empty(x)` or `not is_empty(x)`
  = more about this error: `rythme explain E005`
```

Write the whole question: `if is_empty(basket) then "empty" else "full"`.

### 9. end closes a block, not indentation

**You expect (in Python):** going back to the left is enough to end the
function.

```rythme norun
fn double(x)
  x * 2
print(double(4))
```

```text
error[E020]: missing `end` to close this `fn`
 --> main.ry:1:1
  |
1 | fn double(x)
  | ^^ this `fn` is never closed
  |
4 | 
  | - the code ends here
  |
  = help: add `end` on its own line after the last line of the block
  = more about this error: `rythme explain E020`
```

**Why:** in Rythme, [indentation](../debuter/glossaire.md#indentation)
(the spaces at the start of lines) is only there for reading. The word
`end` closes a [block](../debuter/glossaire.md#block).

**The fix:**

```rythme
fn double(x)
  x * 2
end
print(double(4))
```

```sortie
8
```

```text
fn double(x)      ◄── opens the block
  x * 2           ◄── the content (two spaces: for your eyes)
end               ◄── closes the block
```

A one-line function does not need `end`:
`fn double(x) = x * 2`.

> **Tip:** `rythme fmt main.ry` puts the spaces in the right place for
> you.

### 10. Accents and capital letters matter

**You expect:** `café` and `cafe` are the same.

```rythme norun
café = "espresso"
print(cafe)
```

```text
error[E003]: unknown name `cafe`
 --> main.ry:2:7
  |
2 | print(cafe)
  |       ^^^^ not defined
  |
  = help: did you mean `café`?
  = more about this error: `rythme explain E003`
```

**Why:** accents are allowed in names. But `é` and `e` are two different
letters, so `café` and `cafe` are two names.

**The fix:** pick one spelling and keep it everywhere. Many people write
their names without accents, so they never hesitate.

A capital letter at the start of a name causes another problem:

```rythme norun
Total = 3
```

```text
error[E003]: unknown constructor `Total`
 --> main.ry:1:1
  |
1 | Total = 3
  | ^^^^^ not defined
  |
  = help: constructors are defined with `type`: `type Name = Total(field) | Other`
  = more about this error: `rythme explain E003`
```

A name that starts with a capital letter is reserved for types and their
variants (see [your own types](types.md)). Write `total = 3`.

### 11. Positions are counted from 0

**You expect:** the first element is at position 1.

```rythme
grades = [12, 15, 9]
print(grades[0], grades[2], grades[-1])
```

```sortie
12 9 9
```

```text
 position:    0    1    2
 grades:    [ 12 , 15 , 9 ]
                        -1  (the last one, counting from the end)
```

So the last element of a list of 3 is at position 2, not 3:

```rythme norun
fruits = ["apple", "pear", "kiwi"]
print(fruits[3])
```

```text
error[E007]: index 3 is out of range: the list has 3 element(s)
 --> main.ry:2:14
  |
2 | print(fruits[3])
  |              ^ no element here
  |
  = help: valid indices go from 0 to 2 (or -1 for the last element)
  = help: to get an Option instead of an error, use `get(x, 3)`
  = more about this error: `rythme explain E007`
```

**The fix:** `fruits[-1]` for the last one, or `get(fruits, 3)` which gives
`None` instead of stopping.

### 12. A line that starts with - is a new line of code

**You expect:** a long calculation can continue on the next line.

```rythme norun
price = 10
discount = 3
total = price
  - discount
print(total)
```

```text
error[E024]: this line computes a value but does nothing with it
 --> main.ry:4:3
  |
4 |   - discount
  |   ^^^^^^^^^^ this value is lost
  |
  = help: a line that starts with `-` is a new statement. To continue the line above,
      end that line with `-` instead:
        total = price -
          discount
  = more about this error: `rythme explain E024`
```

**Why:** `-discount` on its own is already a valid calculation ("minus
discount"). So Rythme does not know it should attach it to the line
above.

**The fix:** leave the operator at the end of the first line.

```rythme
price = 10
discount = 3
total = price -
  discount
print(total)
```

```sortie
7
```

## Common mistakes

The whole page is about errors; here are three more habits from other
languages.

### Single quotes around a text

```rythme norun
name = 'Ada'
```

```text
error[E018]: unexpected character `'`
 --> main.ry:1:8
  |
1 | name = 'Ada'
  |        ^
  |
  = help: texts use double quotes: "hello"
  = more about this error: `rythme explain E018`
```

A text is always written between straight double quotes: `name = "Ada"`.

### Writing return

```rythme norun
fn double(x)
  return x * 2
end
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

The last line of a function is its result: simply write `x * 2`.

### Comparing with = instead of ==

```rythme norun
x = 3
print(if x = 3 then "three" else "other")
```

```text
error[E001]: expected `then`, found `=`
 --> main.ry:2:12
  |
2 | print(if x = 3 then "three" else "other")
  |            ^ expected `then` here
  |
  = help: to compare two values, use `==`
  = help: an `if` is written: `if condition then value else other_value`
  = more about this error: `rythme explain E001`
```

`=` gives a name, `==` compares: `if x == 3 then ...`.

## Test yourself

**Question 1:** this program prints `[1, 2]`. Why is the 3 not there, and
how do you keep it?

```rythme norun
xs = [1, 2]
push(xs, 3)
print(xs)
```

> **Answer**
>
> `push` does not change `xs`: it makes a new list, lost here because it has
> no name. Write `xs2 = push(xs, 3)` then `print(xs2)`.

**Question 2:** what are `9 / 3` and `9 // 3`?

> **Answer**
>
> `3.0` and `3`. `/` always gives a decimal number; `//` gives an integer.

**Question 3:** how do you print `Score: 12` when `score = 12`?

> **Answer**
>
> `print("Score: {score}")`, or `print("Score: " + text(score))`.
> `"Score: " + score` is error E005: a text and a number do not add up.

## Summary

- Nothing changes: `push`, `put` and `{..r, field: v}` give a new value, to
  store under a new name.
- No loops and no `null`: `each`, `map` and recursion on one side, `Some(x)`
  and `None` on the other.
- `/` gives a decimal number, `//` an integer; round before comparing
  decimal numbers.
- A text and a number do not mix: `"{x}"`, `text(x)` and `parse_int`.
- Every `if` has an `else`, and `end` closes a block.
- Names care about accents and capital letters, and positions start at 0.

Next: [exercises to get started](exercices-faciles.md)
