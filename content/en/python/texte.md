# Working with text

This page explains how to handle text in Python: cut it up, search it, replace parts of it, format it and understand Unicode characters.

> **Goals:**
>
> - you will be able to read one letter or a piece of text with indexes and slices;
> - you will be able to transform a text with `upper`, `strip`, `replace`...;
> - you will be able to split a text with `split` and glue it back with `join`;
> - you will be able to search a text with `in`, `find` and `count`;
> - you will be able to align text in an f-string and write special characters.

## The str type

A text in Python is a [string](../debuter/glossaire.md#string), of type
`str`. Picture a pearl necklace: each pearl is a character (a letter, a digit, a space,
a symbol), and the pearls are in a precise order.

```python
message = "Hello"
other = 'Hi'
quote = "It's time"
print(message, other, quote)
print(len(message))
```

```sortie
Hello Hi It's time
5
```

- Double or single quotes, as you like. Use double quotes if the text contains an
  apostrophe.
- `len` (*length*) gives the number of characters.

You join two texts with `+` and repeat a text with `*`:

```python
print("Hel" + "lo")
print("ha" * 3)
print("-" * 20)
```

```sortie
Hello
hahaha
--------------------
```

## Reading one character: indexes

Each character has a position, its [index](../debuter/glossaire.md#index). The first
one is at index **0**. Negative indexes count from the end: `-1` is the last one.

```text
 P   y   t   h   o   n
 0   1   2   3   4   5
-6  -5  -4  -3  -2  -1
```

```python
word = "Python"
print(word[0])
print(word[3])
print(word[-1])
print(word[-2])
```

```sortie
P
h
n
o
```

## Cutting: slices

A **slice** takes a piece of the text: `text[start:end]`. The start is included, the
end is **excluded**, just like with `range`.

```python
word = "Python"
print(word[0:3])
print(word[2:])
print(word[:4])
print(word[-3:])
print(word[::2])
print(word[::-1])
```

```sortie
Pyt
thon
Pyth
hon
Pto
nohtyP
```

- without a start, you begin at the start; without an end, you go to the end;
- a third number gives the **step**: `[::2]` takes every other letter;
- `[::-1]` goes backwards: it is the classic trick to reverse a text.

## A text cannot be changed

A `str` is [immutable](../debuter/glossaire.md#immutable): once created, it never
changes. Every operation on a text returns a **new** text. The original stays intact.

```python
name = "alice"
uppercase = name.upper()
print(name)
print(uppercase)
name = name.capitalize()
print(name)
```

```sortie
alice
ALICE
Alice
```

To "change" a text variable, store the new text in the variable:
`name = name.capitalize()`.

## Changing case and cleaning up

A [method](../debuter/glossaire.md#method) is a function attached to a value. You call
it with a dot: `text.method()`. Here are the most useful ones:

```python
title = "the little prince"
print(title.upper())
print(title.title())
print(title.capitalize())
print("HELLO".lower())

entry = "   alice@example.com  \n"
print(f"[{entry.strip()}]")
print(f"[{entry.lstrip()}]".replace("\n", "↵"))
print("---title---".strip("-"))
```

```sortie
THE LITTLE PRINCE
The Little Prince
The little prince
hello
[alice@example.com]
[alice@example.com  ↵]
title
```

- `upper` and `lower`: all uppercase, all lowercase;
- `title`: a capital letter on each word; `capitalize`: only on the first letter;
- `strip` removes spaces and line breaks at both ends; `lstrip` and `rstrip` on one
  side only. With an argument, it removes those characters instead.

> **Tip:** think of `strip()` as soon as you read a text typed by a user or read from a
> file: there are often extra spaces.

## Splitting and joining: split and join

`split` cuts a text into pieces and returns a [list](../debuter/glossaire.md#list).
Without an argument, it cuts at spaces.

```python
sentence = "the cat sleeps on the sofa"
words = sentence.split()
print(words)
print(len(words))

line = "Alice;30;Boston"
print(line.split(";"))
```

```sortie
['the', 'cat', 'sleeps', 'on', 'the', 'sofa']
6
['Alice', '30', 'Boston']
```

`join` does the opposite: it glues the pieces of a list together, with a separator. You
call it **on the separator**.

```python
words = ["one", "two", "three"]
print(" ".join(words))
print(", ".join(words))
print("-".join("abc"))
```

```sortie
one two three
one, two, three
a-b-c
```

> **Remember:** `text.split(sep)` gives a list; `sep.join(list)` gives a text back.

## Replacing

`replace(old, new)` replaces every occurrence of a piece of text.

```python
sentence = "I like tea. The tea is hot."
print(sentence.replace("tea", "coffee"))
print(sentence.replace("tea", "coffee", 1))
print("1,000,000".replace(",", ""))
```

```sortie
I like coffee. The coffee is hot.
I like coffee. The tea is hot.
1000000
```

The third argument limits the number of replacements.

## Searching a text

```python
email = "alice@example.com"
print("@" in email)
print("bob" in email)
print(email.startswith("alice"))
print(email.endswith(".com"))
print(email.find("@"))
print(email.find("#"))
print("banana".count("a"))
```

```sortie
True
False
True
True
5
-1
3
```

- `in` answers `True` or `False`: "is this piece in there?";
- `startswith` and `endswith` test the start and the end;
- `find` gives the index of the first occurrence, or `-1` if it is missing;
- `count` counts the occurrences.

All these searches tell uppercase and lowercase apart. To ignore case, turn everything
to lowercase first:

```python
answer = "YES"
print(answer.lower() == "yes")
```

```sortie
True
```

A few methods test what the text is made of: `isdigit()` (only digits), `isalpha()`
(only letters), `isspace()` (only spaces).

```python
print("2026".isdigit(), "12a".isdigit(), "Café".isalpha())
```

```sortie
True False True
```

## Aligning text

In an f-string, `:` followed by an alignment and a width places the text in a column:

- `<` on the left, `>` on the right, `^` in the center;
- a character before the sign is used as filler.

```python
products = [("Bread", 1.2), ("Cheese", 4.5), ("Marmalade", 3.75)]
for name, price in products:
    print(f"{name:<12}{price:>6.2f} €")
print(f"{'END':*^18}")
```

```sortie
Bread         1.20 €
Cheese        4.50 €
Marmalade     3.75 €
*******END********
```

`{name:<12}` reserves 12 characters, text on the left. `{price:>6.2f}` reserves 6
characters, number on the right with 2 decimals. Number formats are explained in
[numbers and calculations](nombres.md).

## Texts on several lines

Between **triple** quotes `"""..."""`, a text can span several lines. Line breaks are
kept.

```python
poem = """The sky above the roof,
So blue, so calm!"""
print(poem)
print(len(poem.splitlines()))
```

```sortie
The sky above the roof,
So blue, so calm!
2
```

`splitlines()` cuts the text into a list of lines.

## Special characters

Some characters are written with a backslash `\`, followed by a letter: this is an
**escape sequence**.

| Sequence | Meaning |
| --- | --- |
| `\n` | line break |
| `\t` | tab |
| `\"` and `\'` | a quote inside the text |
| `\\` | a backslash |

```python
print("Line 1\nLine 2")
print("Name\tAge")
print("He said: \"Hello\".")
print("C:\\folder\\file.txt")
print(r"C:\folder\new")
```

```sortie
Line 1
Line 2
Name	Age
He said: "Hello".
C:\folder\file.txt
C:\folder\new
```

The `r` in front of the last text makes it a **raw** string: the `\` stay as they are.
Handy for Windows paths and [regular expressions](regex.md).

## Unicode: ord and chr

A computer only stores numbers. [Unicode](../debuter/glossaire.md#unicode) gives a
number to every character in the world: accented letters, alphabets, emojis. `ord`
gives the number of a character, `chr` does the opposite.

```python
print(ord("A"), ord("a"), ord("é"), ord("€"))
print(chr(65), chr(233), chr(8364))
print(len("café"), len("😀"))
```

```sortie
65 97 233 8364
A é €
4 1
```

In Python, a character counts as 1, even an emoji. Python texts are Unicode from the
start: you can write in French, Greek or Japanese without configuring anything.

With `ord` and `chr`, you can shift letters. Here is the Caesar cipher, which shifts
each letter by 3 places:

```python
def caesar(text, shift):
    result = ""
    for letter in text:
        if "a" <= letter <= "z":
            position = (ord(letter) - ord("a") + shift) % 26
            result += chr(ord("a") + position)
        else:
            result += letter
    return result

secret = caesar("hello zoe", 3)
print(secret)
print(caesar(secret, -3))
```

```sortie
khoor crh
hello zoe
```

## Common mistakes

### Changing one letter of a text

```python norun
word = "cat"
word[0] = "C"
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 2, in <module>
    word[0] = "C"
    ~~~~^^^
TypeError: 'str' object does not support item assignment
```

A text is immutable. Build a new text: `word = "C" + word[1:]`, or
`word = word.capitalize()`.

### An index that is too big

```python norun
word = "cat"
print(word[10])
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 2, in <module>
    print(word[10])
          ~~~~^^^^
IndexError: string index out of range
```

`"cat"` has 3 letters: the indexes go from 0 to 2 (*out of range*: outside the limits).
The last character is `word[-1]` or `word[len(word) - 1]`.

### Calling a text method on a list

```python norun
text = "a,b"
print(text.split(",").upper())
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 2, in <module>
    print(text.split(",").upper())
          ^^^^^^^^^^^^^^^^^^^^^
AttributeError: 'list' object has no attribute 'upper'
```

`split` returns a **list**, and a list has no `upper` method. Convert to uppercase
before splitting (`text.upper().split(",")`), or convert each piece.

### Thinking a method changes the text

```python norun
name = "  alice  "
name.strip()
print(f"[{name}]")
```

```text
[  alice  ]
```

No error, but nothing changed: `strip()` returns a new text, which is lost. Write
`name = name.strip()`.

## Test yourself

**Question 1:** what is `"Hello"[1:4]`?

> **Answer**
>
> `"ell"`: the characters at indexes 1, 2 and 3. Index 4 is excluded.

**Question 2:** turn `"apple,pear,kiwi"` into `"apple / pear / kiwi"`.

> **Answer**
>
> ```python
> fruits = "apple,pear,kiwi"
> print(" / ".join(fruits.split(",")))
> ```
>
> ```sortie
> apple / pear / kiwi
> ```
>
> `fruits.replace(",", " / ")` works too.

**Question 3:** how can you tell whether a word is a palindrome (it reads the same both
ways)?

> **Answer**
>
> ```python
> word = "radar"
> print(word == word[::-1])
> ```
>
> ```sortie
> True
> ```

## Summary

- A `str` is a sequence of characters, immutable: every operation returns a new text.
- `text[i]` reads one character (0 is the first, -1 the last); `text[a:b]` reads a
  slice, end excluded.
- `upper`, `lower`, `title`, `strip` and `replace` transform; `split` and `join` cut
  and glue back.
- `in`, `find`, `startswith` and `count` search.
- `f"{x:<10}"` aligns; `\n` and `\t` are special characters; `ord` and `chr` go from a
  character to its Unicode number and back.

Next: [lists and tuples](collections.md).
