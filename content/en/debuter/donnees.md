# Data types: numbers, text, lists

Every value has a type, like a number, a text or a list: this page introduces the basic types, first in Python, then in JavaScript and Ruby.

> **Goals:**
>
> - understand what a type is, and why it matters;
> - use whole numbers and decimal numbers;
> - write text, join it to another text and measure its length;
> - convert a text into a number, and a number into a text;
> - store values in a list and in a dictionary;
> - recognize the "nothing" value.

## What is a type

In a kitchen, you don't put the flour away with the forks. Each thing has
its **kind**: an ingredient, a utensil, a plate.

Programming works the same way. Every [value](glossaire.md#value) has a
kind: it's called its [type](glossaire.md#type).

- `12` is a number;
- `"hello"` is a text;
- `[12, 15, 9]` is a list.

The type tells you **what you can do** with the value. You can add two
numbers. You can't add a word and a number: `"cat" + 3` doesn't mean
anything.

Python gives you the type of a value with `type`:

```python
print(type(12), type("hello"), type([12, 15, 9]))
```

```sortie
<class 'int'> <class 'str'> <class 'list'>
```

`int` means whole number (integer), `str` means text (string), `list` means list.

> **Remember:** every value has a type. The type decides what you can do
> with it.

## Whole numbers and decimals

There are two kinds of numbers:

- **integers**, with no decimal point: `3`, `-12`, `2026` (type `int`, see
  [integer](glossaire.md#integer));
- **decimals**, with a decimal point: `3.5`, `-0.25` (type `float`, see
  [float](glossaire.md#float)).

Careful: in programming, the decimal separator is always a **point**. You
write `3.5`, never `3,5`.

```python
print(7 + 2, 7 - 2, 7 * 2)
print(7 / 2, 7 // 2, 7 % 2)
```

```sortie
9 5 14
3.5 3 1
```

| Sign | Operation | Example | Result |
|---|---|---|---|
| `+` | addition | `7 + 2` | `9` |
| `-` | subtraction | `7 - 2` | `5` |
| `*` | multiplication | `7 * 2` | `14` |
| `/` | division | `7 / 2` | `3.5` |
| `//` | whole division | `7 // 2` | `3` |
| `%` | remainder of the division | `7 % 2` | `1` |

Think of 7 candies for 2 children: each one gets 3 (`7 // 2`), and there is
1 left over (`7 % 2`).

Try it: replace `7` with `10`. The remainder `10 % 2` is `0`: 10 is even.

### The mystery of 0.1 + 0.2

```python
print(0.1 + 0.2)
print(round(0.1 + 0.2, 2))
```

```sortie
0.30000000000000004
0.3
```

This is not a bug in Python. The computer stores decimals in binary (with
0s and 1s). Some numbers, like 0.1, don't come out exactly, just like 1/3
which is written 0.3333… forever. A tiny error is left over. JavaScript and
Ruby give the same result.

For a clean display, you round with `round`: the `2` means "two digits
after the decimal point".

> **Remember:** decimals are sometimes a tiny bit wrong. Round them before
> displaying them, and never count money with them without care.

## Text

A piece of text is called a [string](glossaire.md#string),
`str` in Python. You write it between **quotes**: `"hello"` or `'hello'`.
The quotes tell the computer: "this is a text, not a variable name".

You join two texts with `+`: this is called **concatenating** them. You
measure their length with `len`:

```python
name = "Ada"
greeting = "Hello " + name + "!"
print(greeting)
print(len(name))
print(len(greeting))
```

```sortie
Hello Ada!
3
10
```

`len` counts **all** the characters, including spaces and `!`.

Try it: write `print("name")` instead of `print(name)`. Python displays the
word `name` itself, not `Ada`: with quotes, it's a text.

## Converting from one type to another

`"3"` and `3` look alike, but they are not the same thing. `"3"` is a text,
like a label with 3 written on it. `3` is the number three, which you can
calculate with. With `+`, the difference shows right away:

```python
print("3" + "3", 3 + 3)
```

```sortie
33 6
```

Two texts are joined: `"33"`. Two numbers are added: `6`.

To go from one to the other, you **convert**:

- `int("3")` turns the text into a whole number;
- `float("3.5")` turns the text into a decimal number;
- `str(3)` turns the number into a text.

```python
text = "12"
number = int(text)
print(number + 1)
print(str(number) + " years old")
```

```sortie
13
12 years old
```

This is very useful with `input`, which reads the keyboard: it **always**
gives a text, even if the user types a number.

| Language | Text → number | Number → text |
|---|---|---|
| Python | `int("3")` | `str(3)` |
| JavaScript | `Number("3")` | `String(3)` |
| Ruby | `"3".to_i` | `3.to_s` |

> **Remember:** a number between quotes is a text. Convert it before
> calculating.

## True or false

A yes-or-no question has two answers: yes or no. In programming, it's
`True` or `False`: the [boolean](glossaire.md#boolean) type, `bool` in
Python.

A comparison gives a boolean:

```python
age = 15
print(age >= 18)
print(age == 15)
```

```sortie
False
True
```

In JavaScript and Ruby, you write `true` and `false` in lowercase. Booleans
are mostly used in conditions: see [conditions](logique.md).

## Lists

A [list](glossaire.md#list) stores several values in order, like a
shopping list. You write it between square brackets `[ ]`, with commas.

Each value has a **position**, called an [index](glossaire.md#index).
Careful: you count from **0**.

```text
fruits = ["apple", "pear", "kiwi"]

   ┌─────────┬─────────┬─────────┐
   │ "apple" │ "pear"  │ "kiwi"  │
   └─────────┴─────────┴─────────┘
       0         1         2        ← the positions
```

There are 3 boxes, numbered 0, 1 and 2. Box 3 doesn't exist.

```python
fruits = ["apple", "pear", "kiwi"]
print(fruits[0])
print(fruits[2])
print(len(fruits))
```

```sortie
apple
kiwi
3
```

You add to the end with `append`, and you change a box by giving it a new
value:

```python
fruits = ["apple", "pear", "kiwi"]
fruits.append("strawberry")
fruits[1] = "banana"
print(fruits)
```

```sortie
['apple', 'banana', 'kiwi', 'strawberry']
```

| Step | Instruction | The list after |
|---|---|---|
| 1 | start | `apple, pear, kiwi` |
| 2 | `append("strawberry")` | `apple, pear, kiwi, strawberry` |
| 3 | `fruits[1] = "banana"` | `apple, banana, kiwi, strawberry` |

Try it: display `fruits[-1]`. Python gives the **last** box: `strawberry`.

In JavaScript and Ruby, it's almost the same:

| Action | Python | JavaScript | Ruby |
|---|---|---|---|
| first box | `fruits[0]` | `fruits[0]` | `fruits[0]` |
| add to the end | `fruits.append(x)` | `fruits.push(x)` | `fruits.push(x)` |
| length | `len(fruits)` | `fruits.length` | `fruits.length` |

> **Remember:** the first box of a list is box **0**. The last one is box
> `length - 1`.

## Dictionaries

A phone book links a **name** to a **number**. You don't look for "the
third line", you look for "Ada".

A [dictionary](glossaire.md#dictionary) does the same thing: it links
**keys** to **values**. You write it between curly braces `{ }`, with
`key: value`.

```text
phone_book
   ┌───────┬──────────────────┐
   │ "Ada" │ "06 12 34 56 78" │
   │ "Léo" │ "07 98 76 54 32" │
   └───────┴──────────────────┘
     key         value
```

```python
phone_book = {"Ada": "06 12 34 56 78", "Léo": "07 98 76 54 32"}
print(phone_book["Ada"])
phone_book["Zoé"] = "06 00 00 00 01"
print(len(phone_book))
print("Léo" in phone_book)
```

```sortie
06 12 34 56 78
3
True
```

- `phone_book["Ada"]` reads the value of the key `"Ada"`;
- `phone_book["Zoé"] = …` adds a new entry;
- `"Léo" in phone_book` asks: "does this key exist?".

Reading a missing key, like `phone_book["Max"]`, stops the program with the
error `KeyError` ("unknown key").

In JavaScript, it's called an **object**: `{ Ada: "06 12 34 56 78" }`. In
Ruby, a **hash**: `{ "Ada" => "06 12 34 56 78" }`. You read them the same
way: `phone_book["Ada"]`.

> **Remember:** a list is read by position (0, 1, 2…), a dictionary is
> read by key (`"Ada"`).

## "Nothing": None, null, nil

Sometimes there is **no value**: an empty box, a person missing from the
phone book. Languages have a special value that means "nothing" (see
[null](glossaire.md#null)): `None` in Python, `null` and `undefined` in
JavaScript, `nil` in Ruby.

In Python, `get` looks for a key and gives `None` if it doesn't exist:

```python
phone_book = {"Ada": "06 12 34 56 78"}
print(phone_book.get("Léo"))
print(phone_book.get("Léo") is None)
```

```sortie
None
True
```

You test for "nothing" with `is None`. "Nothing" is neither `0` nor `""`:
it's the absence of a value. You can do almost nothing with it: no
calculations, no capital letters. So first check that a value is not
"nothing".

## Common mistakes

### Adding a text and a number

```python norun
age = "12"
print(age + 1)
```

```text
Traceback (most recent call last):
  File "/home/ada/age.py", line 2, in <module>
    print(age + 1)
          ~~~~^~~
TypeError: can only concatenate str (not "int") to str
```

`age` is a text (it has quotes). Python can't join a number to a text. Fix:
`int(age) + 1`, or `age = 12` without quotes.

### Reading a box that doesn't exist

```python norun
fruits = ["apple", "pear", "kiwi"]
print(fruits[3])
```

```text
Traceback (most recent call last):
  File "/home/ada/fruits.py", line 2, in <module>
    print(fruits[3])
          ~~~~~~^^^
IndexError: list index out of range
```

The list has 3 boxes: 0, 1 and 2. Box 3 is "out of range". Fix:
`fruits[2]`, or `fruits[-1]` for the last one.

## Test yourself

**1.** What is `"5" + "5"` in Python? And `5 + 5`?

> **Answer**
>
> `"5" + "5"` is `"55"`: two texts joined. `5 + 5` is `10`: two numbers
> added.

**2.** In `grades = [12, 15, 9, 18]`, what does `grades[1]` give? And what
is the position of `18`?

> **Answer**
>
> `grades[1]` is `15`, because you count from 0. `18` is at position 3.

**3.** You want to store the age of each student, found by their first
name. List or dictionary?

> **Answer**
>
> A dictionary: the key is the first name, the value is the age.
> `ages = {"Ada": 12, "Léo": 11}`, then `ages["Ada"]`.

## Summary

- Every value has a type: it decides what you can do with it.
- Whole numbers (`3`) and decimals (`3.5`, with a point); decimals are sometimes slightly wrong, so round them.
- Text is written between quotes; `+` joins it, `len` measures it.
- `int("3")` and `str(3)` go from text to number and back.
- A list is read by position, starting at 0; a dictionary is read by key.
- "Nothing" is written `None` in Python, `null` in JavaScript, `nil` in Ruby.

Next: [reading an error message without panicking](lire-une-erreur.md).
