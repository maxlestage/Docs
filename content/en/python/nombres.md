# Numbers and calculations

This page explains how Python calculates: operators, integers and floats, conversions, the `math` and `random` modules, and how to display numbers neatly.

> **Goals:**
>
> - you will be able to use the operators, including `/`, `//`, `%` and `**`;
> - you will know why `0.1 + 0.2` is not exactly `0.3`, and what to do about it;
> - you will be able to convert text into a number, and back;
> - you will be able to use the `math` and `random` modules;
> - you will be able to display a number with 2 decimals or with thousands separators.

## The basic operators

An [operator](../debuter/glossaire.md#operator) is a symbol that does a calculation.
Python can do everything a calculator does.

| Operator | Meaning | Example | Result |
| --- | --- | --- | --- |
| `+` | addition | `7 + 2` | `9` |
| `-` | subtraction | `7 - 2` | `5` |
| `*` | multiplication | `7 * 2` | `14` |
| `/` | division | `7 / 2` | `3.5` |
| `//` | integer division | `7 // 2` | `3` |
| `%` | remainder (modulo) | `7 % 2` | `1` |
| `**` | power | `7 ** 2` | `49` |

```python
print(7 + 2, 7 - 2, 7 * 2)
print(7 / 2, 7 // 2, 7 % 2)
print(7 ** 2, 2 ** 10)
```

```sortie
9 5 14
3.5 3 1
49 1024
```

## Three divisions: /, // and %

Imagine you are sharing 17 candies among 5 children.

- `/` is the "exact" division: `17 / 5` is `3.4`. It **always** gives a `float`, even
  when the result is a whole number.
- `//` is **integer** division: each child gets `17 // 5`, that is `3` candies.
- `%` is the **remainder**: `17 % 5`, that is `2` candies, are left in the bag.

```python
candies = 17
children = 5
print(candies / children)
print(candies // children)
print(candies % children)
print(10 / 2)
```

```sortie
3.4
3
2
5.0
```

Notice that `10 / 2` gives `5.0` and not `5`: `/` returns a `float`.

The remainder `%` is very useful to check whether a number is even: an even number has
a remainder of 0 when you divide it by 2.

```python
print(14 % 2 == 0)
print(15 % 2 == 0)
```

```sortie
True
False
```

> **Tip:** to convert seconds into minutes and seconds, combine `//` and `%`:
> `125 // 60` gives `2` minutes, `125 % 60` gives `5` seconds.

## Order of operations

Python follows the rules of math: first `**`, then `*`, `/`, `//`, `%`, then `+` and
`-`. When the priority is equal, it calculates from left to right.

```python
print(2 + 3 * 4)
print((2 + 3) * 4)
print(2 * 3 ** 2)
print(10 - 4 - 3)
```

```sortie
14
20
18
3
```

> **Remember:** when in doubt, use parentheses. They cost nothing and make the
> calculation obvious to the reader.

## Integers without limits

In many languages, an integer has a maximum size. Not in Python: an `int` grows as much
as needed. Only the computer's memory limits it.

```python
print(2 ** 100)
print(1_000_000 * 1_000_000)
```

```sortie
1267650600228229401496703205376
1000000000000
```

The underscore `_` in a number is only there to make it readable: `1_000_000` is the
same number as `1000000`.

## Floats are approximate

A [float](../debuter/glossaire.md#float) is stored in
[binary](../debuter/glossaire.md#binary). Some numbers that are simple in decimal, like
`0.1`, do not come out exactly in binary. It is like `1/3` in decimal: `0.3333...`
never ends.

```python
print(0.1 + 0.2)
print(0.1 + 0.2 == 0.3)
```

```sortie
0.30000000000000004
False
```

This is not a bug in Python: all languages do the same. Two simple solutions:

- **round** for display, with `round(number, decimals)`;
- **compare with a tolerance**, with `math.isclose`.

```python
import math

total = 0.1 + 0.2
print(round(total, 2))
print(math.isclose(total, 0.3))
```

```sortie
0.3
True
```

> **Warning:** `round` rounds "to the nearest even number" when the value is exactly
> halfway: `round(2.5)` gives `2`, and `round(3.5)` gives `4`. This is banker's
> rounding, which avoids always rounding up.

```python
print(round(2.5), round(3.5), round(2.567, 1))
```

```sortie
2 4 2.6
```

## Decimal and Fraction: exact calculations

With money, a small rounding error can be a problem. The `decimal` module calculates in
**decimal**, like on paper. Create the numbers from **text**:

```python
from decimal import Decimal

print(Decimal("0.1") + Decimal("0.2"))
price = Decimal("19.99")
print(price * 3)
```

```sortie
0.3
59.97
```

The `fractions` module calculates with exact fractions, like in math class:

```python
from fractions import Fraction

third = Fraction(1, 3)
print(third + third)
print(third * 3)
print(Fraction(3, 4) + Fraction(1, 6))
```

```sortie
2/3
1
11/12
```

The fraction is always simplified: `1/3 * 3` really gives `1`.

> **Note:** `Decimal` and `Fraction` are slower than `float`. Use them when exactness
> really matters: amounts of money, precise calculations.

## The math module

A [module](../debuter/glossaire.md#module) is a toolbox. The `math` module contains the
mathematical functions. You open it with `import math`, then you write `math.` in front
of each tool.

```python
import math

print(math.sqrt(16))
print(math.pi)
print(math.floor(3.7), math.ceil(3.2))
print(math.gcd(12, 18))
print(abs(-5), max(3, 9, 4), min(3, 9, 4))
```

```sortie
4.0
3.141592653589793
3 4
6
5 9 3
```

- `sqrt`: square root;
- `floor` rounds down (to the floor), `ceil` rounds up (to the ceiling);
- `gcd`: the greatest common divisor;
- `abs`, `max` and `min` do not need an `import`: they are always available.

## The random module

The `random` module picks numbers at random: perfect for a die or a game. But a program
whose output changes every time is hard to test. `random.seed(n)` sets the starting
point of the randomness: with the same seed, you always get the same draws.

```python
import random

random.seed(42)
print(random.randint(1, 6))
print(random.randint(1, 6))
print(random.choice(["rock", "paper", "scissors"]))
```

```sortie
6
1
rock
```

- `randint(1, 6)`: an integer between 1 and 6, **both ends included**;
- `choice(list)`: one element of the list;
- `random()`: a float between 0 and 1.

> **Tip:** remove `random.seed(42)` in a real game, otherwise the die will always give
> the same sequence. Keep it for tests.

## Converting between text and numbers

Three functions convert a value from one type to another:

- `int(x)`: to an integer;
- `float(x)`: to a float;
- `str(x)`: to text.

```python
text = "42"
number = int(text)
print(number + 1)
print(float("3.5") * 2)
print(str(100) + " points")
print(int(7.9))
```

```sortie
43
7.0
100 points
7
```

Be careful with `int(7.9)`: it **cuts off** the decimal part, it does not round. To
round, use `round(7.9)`, which gives `8`.

## Displaying numbers nicely

In an f-string, add `:` then a **format** after the value:

- `:.2f`: 2 digits after the decimal point (*f* for *fixed* point);
- `:,`: a thousands separator;
- `:%`: a percentage;
- `:5`: a width of 5 characters.

```python
price = 3.14159
population = 68373433
rate = 0.256

print(f"{price:.2f}")
print(f"{population:,}")
print(f"{population:_}")
print(f"{rate:.1%}")
print(f"[{42:5}]")
print(f"{1234567.891:,.2f}")
```

```sortie
3.14
68,373,433
68_373_433
25.6%
[   42]
1,234,567.89
```

> **Note:** the `,` format uses a comma. For another separator, such as a space
> (`68 373 433`), replace it: `f"{population:,}".replace(",", " ")`.

## Common mistakes

### Converting text with a decimal point using int

```python norun
n = int("3.5")
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 1, in <module>
    n = int("3.5")
ValueError: invalid literal for int() with base 10: '3.5'
```

`int` only accepts text that contains an integer. For `"3.5"`, use `float("3.5")`. And
to get the integer: `int(float("3.5"))`.

### Dividing by zero

```python norun
print(10 / 0)
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 1, in <module>
    print(10 / 0)
          ~~~^~~
ZeroDivisionError: division by zero
```

Dividing by zero makes no sense. Check the divisor first: `if divisor != 0:`.
Conditions are explained in [conditions and loops](conditions-boucles.md).

### Forgetting import math

```python norun
print(math.sqrt(16))
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 1, in <module>
    print(math.sqrt(16))
          ^^^^
NameError: name 'math' is not defined. Did you forget to import 'math'?
```

Python only knows `math` after `import math`. Add this line at the top of the file.

### Comparing two floats with ==

```python norun
print(0.1 * 3 == 0.3)
```

```text
False
```

No error, but a surprising result: `0.1 * 3` is `0.30000000000000004`. Compare with
`math.isclose(0.1 * 3, 0.3)`, which answers `True`.

## Test yourself

**Question 1:** what are `20 / 4`, `20 // 6` and `20 % 6`?

> **Answer**
>
> `5.0` (a `float`, always with `/`), `3` (integer division) and `2` (the remainder:
> 20 = 3 × 6 + 2).

**Question 2:** convert 200 seconds into minutes and seconds, and display
`3 min 20 s`.

> **Answer**
>
> ```python
> seconds = 200
> print(f"{seconds // 60} min {seconds % 60} s")
> ```
>
> ```sortie
> 3 min 20 s
> ```

**Question 3:** how do you display `1/3` with 3 decimals?

> **Answer**
>
> ```python
> print(f"{1 / 3:.3f}")
> ```
>
> ```sortie
> 0.333
> ```

## Summary

- `/` always gives a `float`; `//` gives integer division and `%` the remainder.
- `**` is the power; parentheses set the order of calculations.
- `int` values have no limit; `float` values are approximate: round with `round` or
  compare with `math.isclose`.
- `Decimal` and `Fraction` calculate exactly.
- `int()`, `float()` and `str()` convert; `import math` and `import random` open
  toolboxes.
- `f"{x:.2f}"` and `f"{n:,}"` display numbers neatly.

Next: [conditions and loops](conditions-boucles.md).
