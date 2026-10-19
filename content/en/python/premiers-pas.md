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

A program needs to remember things: a first name, a score, a price. Without
variables, each value would be lost right away, like a phone number you forget because
you did not write it down.

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

Line by line:

- `first_name = "Alice"` stores the text `"Alice"` in a box named `first_name`;
- `age = 30` stores the number 30 in another box, named `age`;
- the two `print` calls read the content of each box and display it, in order.

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

**Your turn:** write a program that stores `5` in a variable `apples`, displays its
value, adds 3 to it, then displays the new value.

> **Solution**
>
> ```python
> apples = 5
> print(apples)
> apples = apples + 3
> print(apples)
> ```
>
> ```sortie
> 5
> 8
> ```

## Value types

Python must know what each box contains: you can multiply two prices, but not two
first names. The type tells Python what it is allowed to do with a value.

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

Line by line:

- the first five lines create one variable of each type: an integer, a float, a text,
  a boolean and `None`;
- the empty line changes nothing: it just gives the code some air;
- `print` receives the five variables, separated by commas, and displays their values
  on a single line;
- the quotes around `"Python"` are not displayed: they only tell Python that it is a
  text.

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

**Your turn:** write a program that displays the type of `7`, then the type of `7.0`,
then the type of `"7"`.

> **Solution**
>
> ```python
> print(type(7))
> print(type(7.0))
> print(type("7"))
> ```
>
> ```sortie
> <class 'int'>
> <class 'float'>
> <class 'str'>
> ```

## Dynamic typing

> **Note:** this part can wait. Read it once the basics feel solid.

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

A program works in silence. To see a result, you have to ask for it: `print` writes
on the screen, like reading a result out loud.

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

Line by line:

- `print("There are", 3, "cats")` receives three values: two texts and a number. It
  displays them with a space between them;
- `print()` has nothing to display: it only moves to the next line;
- `print("end")` displays its text on the following line.

`print()` with nothing inside displays an empty line.

### The sep and end options

Sometimes the space and the line break are not what you want: to write a date with
dashes, or to finish a sentence on the same line.

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

**Your turn:** write a program that displays the time `12:45` with a single `print`
that receives the two texts `"12"` and `"45"`.

> **Solution**
>
> ```python
> print("12", "45", sep=":")
> ```
>
> ```sortie
> 12:45
> ```

## f-strings

You often want a sentence that contains values: "Alice is 30 years old.". Gluing
pieces of text and numbers one by one is tedious. An f-string does it all in one line.

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

Line by line:

- the first two lines store `"Alice"` in `first_name` and `30` in `age`;
- in the first `print`, Python replaces `{first_name}` with `Alice`;
- in the third one, it first calculates `age + 10`, that is 40, then puts the result
  in the text;
- `age` is still 30: a calculation between braces does not change the variable.

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

**Your turn:** write a program that stores a price of `4` euros and a quantity of `3`
in two variables, then displays `3 croissants cost 12 euros.` with an f-string.

> **Solution**
>
> ```python
> price = 4
> quantity = 3
> print(f"{quantity} croissants cost {price * quantity} euros.")
> ```
>
> ```sortie
> 3 croissants cost 12 euros.
> ```

## Comments

In a month, you will have forgotten why you wrote a given line. A comment is a
reminder left in the code, for you and for others.

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

Line by line:

- the first line is entirely a comment: Python skips it;
- `price = 12` runs; the end of the line, after `#`, is ignored;
- the last `print` calculates `12 * 3` and displays 36. Its comment is not displayed.

> **Remember:** a good comment explains **why**, not **what**. The code already says
> what it does.

## Naming your variables well

A program is read far more often than it is written. A good name is like a clear
label on a jar: you know what is inside without opening it.

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

> **Note:** this part can wait. Read it once the basics feel solid.

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

Line by line:

- `TAX_RATE` and `MAX_SCORE` are created at the top of the program, in uppercase: you
  spot them right away;
- `price_before_tax * (1 + TAX_RATE)` first calculates the parentheses, `1.2`, then
  `50 * 1.2`, that is `60.0`;
- the f-string displays the value of `MAX_SCORE`.

## Reading what the user types

Until now, all the values were written in the code. For a program to talk with the
person using it, like a counter clerk asking for your name, it must read what they
type.

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
