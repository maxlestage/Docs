# First steps

This page covers the basics of Rythme: giving a name to a value, the kinds of values, calculations, texts, printing and comments.

> **Goals:**
>
> - you will know how to give a name to a value, and why that name never changes;
> - you will recognize the simple kinds of values: `Int`, `Float`, `Text`, `Bool`;
> - you will know how to calculate with `+`, `-`, `*`, `/`, `//`, `%` and `**`;
> - you will know how to insert a value into a text and print it;
> - you will know how to write a comment.

## Giving a name to a value

A [value](../debuter/glossaire.md#value) is a piece of data: a number, a
text... To find it again later, you give it a **name**. You write the
name, `=`, then the value:

```rythme
name = "Ada"
birth_year = 1815
print(name, birth_year)
```

```sortie
Ada 1815
```

Picture a label stuck on a box: `name` is the label, `"Ada"` is what's
inside the box. In other languages, this is called a
[variable](../debuter/glossaire.md#variable).

`print` displays its values separated by a space, then goes to a new line.

## Names never change

In Rythme, a value never changes: it is
[immutable](../debuter/glossaire.md#immutable). A name refers to a single
value, forever. If you want another value, give it a new name:

```rythme
total = 10
total2 = total + 5
print(total, total2)
```

```sortie
10 15
```

The label is stuck for good: you don't peel it off, you stick a new one on
a new box. This makes programs easier to follow: when you read `total`, you
know it is 10 everywhere in the program.

> **Remember:** no `x = x + 1`, no `x += 1`. A new value gets a new name.
> Rythme refuses both (see [common mistakes](#common-mistakes)).

## Kinds of values

Every value has a kind, its [type](../debuter/glossaire.md#type). Here are
the four simplest ones:

| Kind | Examples | What for |
|-------|----------|-----------|
| `Int` | `42`, `-7`, `1_000_000` | whole numbers, or [integers](../debuter/glossaire.md#integer) |
| `Float` | `3.14`, `2.0` | numbers with a decimal point, or [floats](../debuter/glossaire.md#float) |
| `Text` | `"hello"` | texts, or [strings](../debuter/glossaire.md#string) |
| `Bool` | `true`, `false` | true or false, the [booleans](../debuter/glossaire.md#boolean) |

The `_` in `1_000_000` is only there to make it easier to read. A decimal
number is written with a **point**: `3.14`, not `3,14`.

`type_of` gives the kind of a value:

```rythme
print(type_of(42), type_of(3.14), type_of("hi"), type_of(true))
```

```sortie
Int Float Text Bool
```

A `Float` is always displayed with a point, even when it is a whole number:
`2.0`. So you never mistake it for the `Int` `2`.

## Calculating

Here are the calculation **operators**:

| Operator | Means | Example | Result |
|-----------|-----------|---------|----------|
| `+` | plus | `7 + 2` | `9` |
| `-` | minus | `7 - 2` | `5` |
| `*` | times | `7 * 2` | `14` |
| `/` | divided by | `7 / 2` | `3.5` |
| `//` | whole-number division | `7 // 2` | `3` |
| `%` | remainder of the division | `7 % 2` | `1` |
| `**` | power | `7 ** 2` | `49` |

```rythme
print(2 + 3 * 4, (2 + 3) * 4)
print(7 / 2, 6 / 3, 7 // 2, 7 % 3, 2 ** 10)
```

```sortie
14 20
3.5 2.0 3 1 1024
```

A few rules to know:

- `*`, `/`, `//` and `%` come before `+` and `-`, like at school.
  Parentheses change the order.
- `/` **always** gives a `Float`, even when the division is exact:
  `6 / 3` is `2.0`.
- `//` gives the **whole-number division**: how many times does 2 fit in 7?
  3 times. `%` gives what is left over: 7 = 3 × 2 + **1**.
- `%` is often used to find out whether a number is even: `n % 2 == 0`.

Think of candy: 7 candies for 2 children, each one gets `7 // 2`, so 3,
and `7 % 2` is left over, so 1.

Whole numbers have no size limit:

```rythme
print(2 ** 100)
```

```sortie
1267650600228229401496703205376
```

To round or convert a number:

```rythme
print(round(3.7), round(3.14159, 2), int(3.9), float(2))
```

```sortie
4 3.14 3 2.0
```

`round(x)` rounds to the nearest whole number, `round(x, 2)` keeps two
decimals, `int` cuts off the decimals, `float` gives a decimal number.

## Texts and insertion

A text is written between straight double quotes `"`. To put a value inside
a text, write it between braces `{...}`: this is **insertion**.

```rythme
name = "Ada"
age = 36
print("{name} is {age} years old.")
print("In ten years: {age + 10} years old.")
```

```sortie
Ada is 36 years old.
In ten years: 46 years old.
```

Between the braces, you can write a calculation, not just a name. As in
Ruby, `#{...}` works too:

```rythme
name = "Grace"
print("Hello, #{name}!")
```

```sortie
Hello, Grace!
```

`+` puts two texts end to end. But Rythme never mixes text and numbers
silently: `"age: " + 36` is an error. Write `"age: {36}"`, or first convert
the number into a text with `text(36)`.

```rythme
print("Hel" + "lo", "age: " + text(36))
```

```sortie
Hello age: 36
```

A few useful functions on texts (the page
[working with text](texte.md) shows many more):

```rythme
print(upper("hi"), len("été"), trim("  ok  "), "a,b,c" |> split(","))
```

```sortie
HI 3 ok ["a", "b", "c"]
```

`upper` turns letters into capitals, `len` counts the characters, `trim`
removes the spaces around, `split` cuts into pieces.

## Printing with print and puts

`print` and `puts` are the same function. `puts` is the Ruby name. At the
start of a line, you can even leave out the parentheses:

```rythme
print("with parentheses")
puts "without parentheses"
puts "two values:", 1 + 1
```

```sortie
with parentheses
without parentheses
two values: 2
```

The values given to `print` are separated by commas. They are displayed
separated by a space.

## Comments

`#` starts a **comment**, up to the end of the line. Rythme ignores it: it's
a note for humans, including you three months from now.

```rythme
# An ordinary comment.
price = 12  # a comment after some code
print(price * 2)
```

```sortie
24
```

You will also come across `##`, two hash signs: it's a comment that
describes the function written just below it (see
[documenting with rythme doc](projet.md)).

## Accents in names

A name starts with a letter or `_`, then contains letters, digits and `_`.
Accents are allowed:

```rythme
prénom = "Élise"
âge = 28
print("{prénom} is {âge} years old.")
```

```sortie
Élise is 28 years old.
```

Uppercase and lowercase matter: `age` and `Age` are two different names. A
name that starts with a capital letter is reserved for types and their
variants (see [your own types](types.md)).

## Common mistakes

### Changing the value of a name

In many languages, you write `x = x + 1`. Not in Rythme:

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

The error is found before the program even runs. Give it a new name:
`total2 = total + 1`.

### Using += to add

```rythme norun
score = 0
score += 10
```

```text
error[E011]: values cannot be changed in Rythme
 --> main.ry:2:7
  |
2 | score += 10
  |       ^^
  |
  = help: values never change in Rythme: give the new value a new name, like `total2 = total + 1`
  = more about this error: `rythme explain E011`
```

`+=` would change `score`. Write `score2 = score + 10` instead.

### Adding a number to a text

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

Rythme doesn't guess whether you want a text or a number. Write
`print("age: {age}")`.

### Writing let or var before a name

```rythme norun
let age = 36
print(age)
```

```text
error[E001]: `let` is not used in Rythme
 --> main.ry:1:1
  |
1 | let age = 36
  | ^^^ comes from another language
  |
  = help: just write `name = value` (values never change, so no keyword is needed)
  = more about this error: `rythme explain E001`
```

No keyword is needed: `age = 36`.

## Test yourself

**Question 1:** what are `9 / 2`, `9 // 2` and `9 % 2`?

> **Answer**
>
> `4.5`, `4` and `1`. `/` always gives a decimal number, `//` the
> whole-number division, `%` the remainder.

**Question 2:** what does this program print?

```rythme norun
price = 5
quantity = 3
print("Total: {price * quantity} dollars")
```

> **Answer**
>
> `Total: 15 dollars`. The calculation between braces is done, then
> inserted into the text.

**Question 3:** you have `points = 10` and you want 5 more points. What
should you write?

> **Answer**
>
> A new name: `points2 = points + 5`. `points = points + 5` is error E011,
> because a name never changes.

## Summary

- `name = value` gives a name to a value, forever: a new value gets a new
  name.
- The simple kinds of values are `Int`, `Float`, `Text` and `Bool`;
  `type_of` gives the kind of a value.
- `/` always gives a decimal number, `//` the whole-number division, `%` the
  remainder, `**` the power.
- `"Hello {name}"` inserts a value into a text; text and numbers never mix
  with `+`.
- `print` (or `puts`) prints, and `#` starts a comment.

Next: [Numbers and calculations](nombres.md)
