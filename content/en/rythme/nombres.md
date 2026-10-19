# Numbers and calculations

This page shows the two kinds of numbers in Rythme, integers and decimals, and everything you can calculate with them.

> **Goals:**
>
> - you will know the difference between an `Int` and a `Float`;
> - you will know how to use the operators `+`, `-`, `*`, `/`, `//`, `%` and `**`;
> - you will know how to round, convert and use the math functions;
> - you will know how to print a number nicely, with decimals and separated thousands.

## Two kinds of numbers

Rythme knows two kinds of numbers:

- [integers](../debuter/glossaire.md#integer) (`Int`), with no decimal part:
  `42`, `-7`, `0`;
- [floats](../debuter/glossaire.md#float) (`Float`), decimal numbers,
  written with a **dot**: `3.14`, `2.0`, `-0.5`.

`type_of` tells you what kind a number is:

```rythme
print(type_of(42), type_of(3.14), type_of(2.0))
print(1_000_000, 0xFF, 0b1010)
```

```sortie
Int Float Float
1000000 255 10
```

`2.0` is a Float, even though it has no decimals. The `_` separates the
digits to make them readable: `1_000_000` is one million. `0xFF` is written
in hexadecimal and `0b1010` in binary.

> **Warning:** you write `3.14` with a dot, never `3,14`. The comma
> separates values: `print(3,14)` prints `3 14`.

## Integers with no size limit

In many languages, an integer has a maximum size, and a calculation that is
too big gives a wrong result. In Rythme, an integer grows as much as
needed, like a ribbon you can unroll forever:

```rythme
print(2 ** 100)
print(1_000_000 * 1_000_000)
fn factorial(n) = if n == 0 then 1 else n * factorial(n - 1)
print(factorial(30))
```

```sortie
1267650600228229401496703205376
1000000000000
265252859812191058636308480000000
```

All the digits are exact. Floats, on the other hand, are **approximate**
values: they keep about 16 digits.

```rythme
print(0.1 + 0.2, 0.1 + 0.2 == 0.3)
```

```sortie
0.30000000000000004 false
```

This is not a flaw in Rythme: all languages calculate decimal numbers this
way. To compare Floats, round them first (see below).

## The four operations

`+`, `-` and `*` work like at school. Between two Ints, the result is an
Int; as soon as there is a Float, the result is a Float:

```rythme
print(7 + 2, 7 - 2, 7 * 2)
print(7 + 2.0, 3 * 1.0)
```

```sortie
9 5 14
9.0 3.0
```

Division `/` **always gives a Float**, even when it divides evenly:

```rythme
print(7 / 2, 6 / 3, 1 / 3)
```

```sortie
3.5 2.0 0.3333333333333333
```

So `7 / 2` really is `3.5`, and not `3` as in some other languages.

## Integer division, remainder and power

To share whole objects, you often want the **quotient** and the
**remainder**: 17 candies for 5 children is 3 candies each, with 2 left
over.

```rythme
print(17 // 5, 17 % 5)
print(2 ** 10, 2 ** -1, 9 ** 0.5)
```

```sortie
3 2
1024 0.5 3.0
```

| Operator | Name | Example | Result |
|----------|------|---------|--------|
| `+` `-` `*` | addition, subtraction, multiplication | `7 * 2` | `14` |
| `/` | division, always a Float | `7 / 2` | `3.5` |
| `//` | integer division, rounded down | `17 // 5` | `3` |
| `%` | remainder of the division | `17 % 5` | `2` |
| `**` | power | `2 ** 10` | `1024` |

The priorities are the ones from math: `**` first, then `*`, `/`, `//`,
`%`, then `+` and `-`. Parentheses change the order:

```rythme
print(2 + 3 * 4, (2 + 3) * 4, -2 ** 2)
```

```sortie
14 20 -4
```

`-2 ** 2` means `-(2 ** 2)`, so `-4`.

> **Tip:** `n % 2 == 0` tells you whether `n` is even. Rythme also has
> `is_even(n)` and `is_odd(n)`, which are easier to read.

## Even or odd

```rythme
print(is_even(4), is_odd(4), is_odd(7))
print(filter(1..10, is_even))
```

```sortie
true false true
[2, 4, 6, 8, 10]
```

`is_even` and `is_odd` are functions like any other: you can give them to
`filter` or to `count`.

## Math functions

Rythme provides the usual math functions, without importing anything:

```rythme
print(abs(-5), abs(-2.5))
print(min(3, 7), max(3, 7.5), max([2, 9, 4]))
print(sqrt(16), sqrt(2))
print(pow(2, 8), pow(2.0, 0.5))
```

```sortie
5 2.5
3 7.5 9
4.0 1.4142135623730951
256 1.4142135623730951
```

- `abs(x)`: the absolute value, the number without its sign;
- `min` and `max`: the smallest or the largest, of several values or of a
  list;
- `sqrt(x)`: the square root, always a Float;
- `pow(x, n)`: the power, like `x ** n`.

## Rounding

Four functions turn a Float into an Int:

```rythme
print(round(2.5), round(2.4), round(-2.5))
print(floor(2.7), floor(-2.5), ceil(2.1))
print(int(3.99), int(-3.99))
print(round(3.14159, 2), round(0.1 + 0.2, 2) == 0.3)
```

```sortie
3 2 -3
2 -3 3
3 -3
3.14 true
```

| Function | What it does | `2.7` | `-2.5` |
|----------|--------------|-------|--------|
| `round(x)` | to the nearest (a `.5` goes away from zero) | `3` | `-3` |
| `floor(x)` | down, to the "floor" | `2` | `-3` |
| `ceil(x)` | up, to the "ceiling" | `3` | `-2` |
| `int(x)` | cuts off the decimals | `2` | `-2` |

`round(x, digits)` keeps that many decimals and gives a Float.

## Converting

Rythme **never** converts a value silently. `int` and `float` convert when
you ask them to:

```rythme
print(int("42") + 1, float("2.5") * 2, float(3), int(true))
print(parse_int("42"), parse_int("twelve"))
```

```sortie
43 5.0 3.0 1
Ok(42) Err("\"twelve\" is not a whole number")
```

`int("twelve")` stops the program. When a text comes from the user and
might not be a number, prefer `parse_int`: it gives a `Result`, explained
in [handling errors](erreurs.md).

## GCD and LCM

`gcd(a, b)` gives the greatest common divisor (GCD), and `lcm(a, b)` the
least common multiple (LCM):

```rythme
print(gcd(12, 18), lcm(4, 6))
numerator = 18
denominator = 24
d = gcd(numerator, denominator)
print("{numerator}/{denominator} = {numerator // d}/{denominator // d}")
```

```sortie
6 12
18/24 = 3/4
```

The GCD is useful, for example, to simplify a fraction.

## Pi and trigonometry

`pi` is the constant π. `sin`, `cos` and `tan` take an angle in
**radians**. For degrees, multiply by `pi / 180`:

```rythme
print(pi)
angle = 30 * pi / 180
print(sin(angle), round(sin(angle), 2))
print(cos(0), round(cos(60 * pi / 180), 2))
radius = 2
print("area of the disc: {round(pi * radius ** 2, 2)}")
```

```sortie
3.141592653589793
0.49999999999999994 0.5
1.0 0.5
area of the disc: 12.57
```

The sine of 30° is 0.5; the approximate calculation gives
`0.49999999999999994`, and `round` tidies it up.

## Random numbers

`random(a, b)` picks an integer from `a` to `b`, inclusive, like a die.
`random()` gives a Float between 0 and 1, and `random(list)` a random
element. The result changes every time you run the program:

```rythme norun
print(random(1, 6))
print(random(["heads", "tails"]))
```

```text
4
tails
```

What you can check is that the die stays between 1 and 6:

```rythme
die = random(1, 6)
print(die >= 1 and die <= 6, contains(1..6, die))
```

```sortie
true true
```

## Printing a number nicely

`text(x, decimals)` writes a number with **exactly** that many decimals.
It's perfect for prices:

```rythme
print(text(3.14159, 2), text(5, 2), text(1 / 3, 3))
price = 4.5
print("Total: {text(price * 3, 2)} €")
```

```sortie
3.14 5.00 0.333
Total: 13.50 €
```

`format_number(x, decimals, language)` also separates the thousands. With
`"fr"`, you get the French style: a space for thousands, a comma for
decimals.

```rythme
print(format_number(1234567.891, 2, "fr"))
print(format_number(1234567.891, 2))
print(format_number(-42000, 0, "fr"), format_number(0.5, 2, "fr"))
```

```sortie
1 234 567,89
1,234,567.89
-42 000 0,50
```

Without a language, you get the English style: a comma for thousands and a
dot for decimals.

## Common mistakes

### Adding a number to a text

```rythme norun
age = "36"
print(age + 1)
```

```text
error[E005]: cannot use `+` with Text and Int
 --> main.ry:2:11
  |
2 | print(age + 1)
  |           ^ Text + Int 1
  |
  = help: to build a text with a value inside, use braces: "total: {x}"
      or convert the value first: text(x)
  = more about this error: `rythme explain E005`
```

`"36"` is a text, not a number. Convert it: `int(age) + 1` is `37`.

### Using the result of / as a position

```rythme norun
xs = ["a", "b", "c", "d"]
middle = len(xs) / 2
print(xs[middle])
```

```text
error[E005]: an index must be a whole number (Int), but got a value of type Float
 --> main.ry:3:10
  |
3 | print(xs[middle])
  |          ^^^^^^
  |
  = more about this error: `rythme explain E005`
```

`/` always gives a Float: `4 / 2` is `2.0`. For a position, use integer
division: `middle = len(xs) // 2`.

### Dividing by zero

```rythme norun
grades = []
print(sum(grades) / len(grades))
```

```text
error[E008]: division by zero
 --> main.ry:2:19
  |
2 | print(sum(grades) / len(grades))
  |                   ^
  |
  = help: check the divisor first: `if b == 0 then ... else a / b`,
      or return a Result: `if b == 0 then Err("division by zero") else Ok(a / b)`
  = more about this error: `rythme explain E008`
```

An empty list has 0 elements. Test it first:
`if is_empty(grades) then 0 else sum(grades) / len(grades)`.

## Test yourself

**Question 1:** what are `7 / 2`, `7 // 2` and `7 % 2`?

> **Answer**
>
> `3.5`, `3` and `1`. `/` always gives a Float, `//` the integer quotient,
> `%` the remainder.

**Question 2:** what do `round(2.5)`, `floor(2.5)` and `int(-2.5)` give?

> **Answer**
>
> `3`, `2` and `-2`. `round` goes to the nearest (a `.5` goes away from
> zero), `floor` goes down, `int` cuts off the decimals.

**Question 3:** how do you print `1234.5` as `1 234,50`?

> **Answer**
>
> `format_number(1234.5, 2, "fr")`: two decimals, French style.

## Summary

- `Int` for integers, with no size limit; `Float` for decimals, written
  with a dot, and approximate.
- `/` always gives a Float; `//` and `%` give the integer quotient and the
  remainder; `**` the power.
- `round`, `floor`, `ceil` and `int` round; `int`, `float` and `parse_int`
  convert.
- `abs`, `min`, `max`, `sqrt`, `pow`, `gcd`, `lcm`, `pi`, `sin`, `cos` are
  always available.
- `text(x, 2)` and `format_number(x, 2, "fr")` print a number nicely.

Next: [conditions and repetition](conditions-boucles.md)
