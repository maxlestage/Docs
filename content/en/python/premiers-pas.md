# First steps

This page introduces the building blocks of Python: variables, simple value types, displaying with `print`, and f-strings.

> **Goals:**
>
> - you will be able to create a variable and change its value;
> - you will be able to recognize the types `int`, `float`, `str`, `bool` and `None`;
> - you will be able to display values with `print` and f-strings;
> - you will be able to write comments and name your variables well;
> - you will be able to read what the user types with `input`.

## Variables

A [variable](../debuter/glossaire.md#variable) is a name given to a
[value](../debuter/glossaire.md#value). Picture a label stuck on a box: the label is
the name; the content of the box is the value.

You create a variable with the `=` sign, which means "receives" (not "is equal to"):

```python
first_name = "Alice"
age = 30
print(first_name)
print(age)
```

```sortie
Alice
30
```

A variable can receive a new value. The old one is forgotten.

```python
score = 10
print(score)
score = 25
print(score)
score = score + 5
print(score)
```

```sortie
10
25
30
```

The line `score = score + 5` is read from right to left: Python calculates `score + 5`
(that is `30`), then stores the result in `score`.

> **Tip:** `score += 5` is a shortcut for `score = score + 5`. There are also `-=`,
> `*=` and `/=`.

## Value types

Every value has a [type](../debuter/glossaire.md#type): its nature. You do not handle a
number the same way as a text. Here are the five basic types.

| Type | Full name | Examples | Used to |
| --- | --- | --- | --- |
| `int` | *integer* | `42`, `-7`, `0` | count |
| `float` | decimal number | `3.14`, `-0.5`, `2.0` | measure |
| `str` | *string*, text | `"cat"`, `'Hello'` | write |
| `bool` | boolean | `True`, `False` | answer yes or no |
| `NoneType` | nothing | `None` | say "no value" |

```python
number = 42
price = 9.99
name = "Python"
is_active = True
result = None

print(number, price, name, is_active, result)
```

```sortie
42 9.99 Python True None
```

A few details:

- An [integer](../debuter/glossaire.md#integer) has no decimal point. A
  [float](../debuter/glossaire.md#float) is written with a **dot**: `3.14`, not
  `3,14`.
- A [string](../debuter/glossaire.md#string) (`str`) goes between double
  quotes `"..."` or single quotes `'...'`. Both work the same.
- A [boolean](../debuter/glossaire.md#boolean) is `True` or `False`, with a capital
  letter.
- `None` means "nothing, no value for now". It is the equivalent of
  [null](../debuter/glossaire.md#null) in other languages.

## Finding the type of a value

The `type()` function gives the type of a value. It is very useful in the console to
understand what you are handling.

```python
print(type(42))
print(type(3.14))
print(type("hello"))
print(type(True))
print(type(None))
print(type("42"))
```

```sortie
<class 'int'>
<class 'float'>
<class 'str'>
<class 'bool'>
<class 'NoneType'>
<class 'str'>
```

Look at the last line: `"42"` is a **text**, not a number. The quotes change
everything.

## Dynamic typing

In Python, a variable does not have a fixed type. It is the **value** that has a type.
The same label can go from a number to a text: we say the typing is **dynamic**.

```python
x = 5
print(type(x))
x = "five"
print(type(x))
```

```sortie
<class 'int'>
<class 'str'>
```

It is handy, but be careful: Python will not warn you if you mix everything up by
mistake. Keep one type per variable, it is clearer.

> **Note:** other languages, like [Rust](../rust/premiers-pas.md), fix the type of a
> variable once and for all. This is called **static** typing.

## Displaying with print

`print` displays one or more values. Separated by commas, they are displayed with a
space between them, followed by a line break.

```python
print("There are", 3, "cats")
print()
print("end")
```

```sortie
There are 3 cats

end
```

`print()` with nothing inside displays an empty line.

### The sep and end options

Two options change this behavior:

- `sep` (*separator*): what goes between the values (a space by default);
- `end`: what is added at the end (a line break by default).

```python
print("2026", "10", "18", sep="-")
print("a", "b", "c", sep="")
print("Loading", end="...")
print("done")
```

```sortie
2026-10-18
abc
Loading...done
```

With `end="..."`, the next `print` continues on the same line.

## f-strings

To put values **inside** a text, use an **f-string**: a text preceded by the letter
`f`. Between curly braces `{ }`, you write a variable or a calculation.

```python
first_name = "Alice"
age = 30
print(f"Hello, {first_name}!")
print(f"{first_name} is {age} years old.")
print(f"In 10 years, she will be {age + 10}.")
```

```sortie
Hello, Alice!
Alice is 30 years old.
In 10 years, she will be 40.
```

It is like a fill-in-the-blanks form: Python fills each `{ }` with the value.

> **Tip:** write `{variable=}` to display the name **and** the value. Very handy for
> debugging: `print(f"{age=}")` displays `age=30`.

```python
age = 30
print(f"{age=}")
```

```sortie
age=30
```

## Comments

A [comment](../debuter/glossaire.md#comment) is a note for humans. Python ignores
it. It starts with `#` and goes to the end of the line.

```python
# Total price calculation
price = 12  # price of one item, in euros
quantity = 3
print(price * quantity)  # displays the total
```

```sortie
36
```

> **Remember:** a good comment explains **why**, not **what**. The code already says
> what it does.

## Naming your variables well

A variable name:

- contains letters, digits and `_`, but does not start with a digit;
- is case-sensitive: `age` and `Age` are two different names;
- cannot be a Python reserved word (`if`, `for`, `def`, `class`, `True`...).

In Python, the convention is **snake_case**: all lowercase, words separated by `_`,
like a snake crawling on the ground.

```python
number_of_players = 4
total_price_incl_tax = 59.90
is_logged_in = False
print(number_of_players, total_price_incl_tax, is_logged_in)
```

```sortie
4 59.9 False
```

Choose names that say what the variable contains. `number_of_players` is much clearer
than `n` or `x`.

> **Note:** accented letters are allowed (`prénom = "Alice"`), but the usual practice is
> to write names without accents, in English.

## Constants

A [constant](../debuter/glossaire.md#constant) is a value that must not change: a
rate, a limit, an application name. Python has no real constants. By **convention**,
they are written in UPPERCASE: it is a message to other programmers, "do not touch this
value".

```python
TAX_RATE = 0.2
MAX_SCORE = 100

price_before_tax = 50
print(price_before_tax * (1 + TAX_RATE))
print(f"Maximum score: {MAX_SCORE}")
```

```sortie
60.0
Maximum score: 100
```

## Reading what the user types

`input()` displays a question, waits for the user to type an answer and press Enter,
and returns that answer.

```python norun
first_name = input("What is your name? ")
print(f"Nice to meet you, {first_name}!")
```

```text
What is your name? Alice
Nice to meet you, Alice!
```

`input` **always** returns a text (`str`), even if the user types a number. To
calculate with it, convert it with `int()`:

```python norun
age = int(input("How old are you? "))
print(f"Next year, you will be {age + 1}.")
```

```text
How old are you? 29
Next year, you will be 30.
```

Conversions are explained in detail in [numbers and calculations](nombres.md).

## Common mistakes

### Joining a text and a number with +

```python norun
age = 30
print("Age: " + age)
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 2, in <module>
    print("Age: " + age)
          ~~~~~~~~^~~~~
TypeError: can only concatenate str (not "int") to str
```

Python refuses to add a text and a number ("you can only join a `str` to a `str`").
Use an f-string: `print(f"Age: {age}")`. Or convert: `"Age: " + str(age)`.

### Using a variable before creating it

```python norun
print(first_name)
first_name = "Alice"
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 1, in <module>
    print(first_name)
          ^^^^^^^^^^
NameError: name 'first_name' is not defined
```

Python reads the file from top to bottom. On line 1, `first_name` does not exist yet.
Create the variable before using it. Also check the spelling: `First_name` or
`firstname` would be other names.

### Calculating with the result of input

```python norun
age = input("Age? ")
print(age + 1)
```

```text
Age? 20
Traceback (most recent call last):
  File "/home/you/main.py", line 2, in <module>
    print(age + 1)
          ~~~~^~~
TypeError: can only concatenate str (not "int") to str
```

`input` returns the text `"20"`, not the number `20`. Convert it:
`age = int(input("Age? "))`.

### Forgetting the f before the string

```python norun
first_name = "Alice"
print("Hello, {first_name}!")
```

```text
Hello, {first_name}!
```

No error, but the curly braces are displayed as they are. Without an `f` in front, it
is an ordinary text. Write `print(f"Hello, {first_name}!")`.

## Test yourself

**Question 1:** what does this code display?

```python norun
a = 5
b = a
a = 8
print(a, b)
```

> **Answer**
>
> `8 5`. The line `b = a` gives `b` the value of `a` at that moment (`5`). Changing `a`
> afterwards does not change `b`.

**Question 2:** what is the type of `"3.5"`? And of `3.5`?

> **Answer**
>
> `"3.5"` is a `str` (a text, because of the quotes). `3.5` is a `float`.

**Question 3:** write a program that stores `"Paris"` in a variable and displays
`I live in Paris.` with an f-string.

> **Answer**
>
> ```python
> city = "Paris"
> print(f"I live in {city}.")
> ```
>
> ```sortie
> I live in Paris.
> ```

## Summary

- `name = value` creates a variable; you can give it a new value later.
- The basic types are `int`, `float`, `str`, `bool` and `None`; `type()` reveals them.
- `print` displays its values separated by `sep` and ended by `end`.
- An f-string (`f"... {x} ..."`) inserts values into a text.
- Names are written in snake_case; constants in UPPERCASE.
- `input()` always returns a text: convert it before calculating.

Next: [numbers and calculations](nombres.md).
