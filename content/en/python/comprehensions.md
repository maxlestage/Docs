# Comprehensions

This page shows how to build a list, a dictionary or a set in a single line with comprehensions, and when it is better to do without them.

> **Goals:**
>
> - you will be able to turn a `for` loop into a list comprehension;
> - you will be able to filter elements with `if`;
> - you will be able to write dictionary and set comprehensions;
> - you will be able to use a generator expression with `sum`, `any` and `all`;
> - you will be able to spot a comprehension that is too complicated.

## From loop to comprehension

Changing every element of a list is a very common need: prices to convert, names to
write in capitals, grades to double. It is like a recipe that says "for each apple, peel
it": the same action, on each element.

You already know how to build a new [list](../debuter/glossaire.md#list) with a `for`
loop: you start from an empty list, then add one element on each pass.

```python
numbers = [1, 2, 3, 4]
squares = []
for n in numbers:
    squares.append(n * n)
print(squares)
```

```sortie
[1, 4, 9, 16]
```

Line by line:

- `squares = []` creates an empty list: it is the bowl where the results will go;
- `for n in numbers:` gives `n` the values 1, then 2, then 3, then 4;
- on each pass, `squares.append(n * n)` adds the square of `n` to the end of `squares`;
- after the last pass, `print(squares)` displays the filled list.

This pattern is so common that Python has a shorter way to write it: the **list
comprehension**. You build it in three steps, starting from the loop:

1. write square brackets `[ ]`: the result will be a list;
2. first put **what you add**: `n * n`;
3. then copy the `for` line without the colon: `for n in numbers`.

```python
numbers = [1, 2, 3, 4]
squares = [n * n for n in numbers]
print(squares)
```

```sortie
[1, 4, 9, 16]
```

Read it out loud: "the list of `n * n`, for each `n` in `numbers`". It is almost an
English sentence.

> **Remember:** a comprehension **always creates a new list**. The original list is not
> changed.

### Any expression

To the left of the `for`, you can put any
[expression](../debuter/glossaire.md#expression): a calculation, a method call, an
f-string…

```python
names = ["alice", "bob", "chloé"]
print([name.capitalize() for name in names])
print([len(name) for name in names])
print([f"Hello {name}!" for name in names])
```

```sortie
['Alice', 'Bob', 'Chloé']
[5, 3, 5]
['Hello alice!', 'Hello bob!', 'Hello chloé!']
```

**Your turn:** write a program that starts from `prices = [2, 5, 8]` (prices in euros)
and builds, with a comprehension, the list of the same prices in cents, then displays it.

> **Solution**
>
> ```python
> prices = [2, 5, 8]
> cents = [p * 100 for p in prices]
> print(cents)
> ```
>
> ```sortie
> [200, 500, 800]
> ```

## Filtering with if

Often, you only keep some of the elements. With a loop, you write an `if` inside the
`for`:

```python
grades = [12, 7, 15, 9, 18]
passed = []
for grade in grades:
    if grade >= 10:
        passed.append(grade)
print(passed)
```

```sortie
[12, 15, 18]
```

Line by line:

- `passed = []` starts from an empty list;
- `for grade in grades:` takes the grades one by one: 12, 7, 15, 9, then 18;
- `if grade >= 10:` only lets through grades of 10 or more;
- so 12, 15 and 18 are added with `append`; 7 and 9 are skipped.

In a comprehension, the `if` goes **at the end**, after the `for`:

```python
grades = [12, 7, 15, 9, 18]
passed = [grade for grade in grades if grade >= 10]
print(passed)
```

```sortie
[12, 15, 18]
```

It is a sieve: only the elements that pass the test end up in the new list.

**Your turn:** write a program that starts from `ages = [12, 25, 17, 40]` and builds,
with a comprehension, the list of ages of 18 or more, then displays it.

> **Solution**
>
> ```python
> ages = [12, 25, 17, 40]
> adults = [age for age in ages if age >= 18]
> print(adults)
> ```
>
> ```sortie
> [25, 40]
> ```

### Transforming based on a condition

To **keep every element** but transform them differently, use the conditional
expression `a if condition else b`, placed **before** the `for`:

```python
grades = [12, 7, 15, 9]
results = ["pass" if grade >= 10 else "fail" for grade in grades]
print(results)
```

```sortie
['pass', 'fail', 'pass', 'fail']
```

> **Warning:** the two positions do not mean the same thing. An `if` **after** the `for`
> filters (the list can get shorter). An `if … else` **before** the `for` chooses a
> value (the list keeps the same length).

## Dictionary comprehensions

A dictionary is like an address book: each name goes with a piece of information. A
dictionary comprehension fills that book in a single line.

The same idea works for a
[dictionary](../debuter/glossaire.md#dictionary). You use curly braces `{ }` and write
`key: value` before the `for`.

```python
names = ["alice", "bob", "chloé"]
lengths = {name: len(name) for name in names}
print(lengths)
```

```sortie
{'alice': 5, 'bob': 3, 'chloé': 5}
```

Line by line:

- `names` holds three names;
- for each name, the comprehension creates a pair: the key `name` and the value
  `len(name)`, the number of letters;
- `"alice"` gives the pair `'alice': 5`, then `"bob"` gives `'bob': 3`, and so on.

To go through an existing dictionary, use `.items()`, which gives the `(key, value)`
pairs:

```python
prices = {"apple": 2.0, "pear": 3.0, "kiwi": 0.5}
on_sale = {fruit: p * 0.8 for fruit, p in prices.items()}
cheap = {fruit: p for fruit, p in prices.items() if p < 2.5}
print(on_sale)
print(cheap)
```

```sortie
{'apple': 1.6, 'pear': 2.4000000000000004, 'kiwi': 0.4}
{'apple': 2.0, 'kiwi': 0.5}
```

> **Note:** `2.4000000000000004` is not a bug in the comprehension: it comes from how
> decimal numbers are rounded (see [numbers](nombres.md)). Use `round(p * 0.8, 2)` for a
> clean display.

**Your turn:** write a program that starts from `prices = {"bread": 1, "milk": 2}` and
builds, with a comprehension, a dictionary of the same prices in cents, then displays
it.

> **Solution**
>
> ```python
> prices = {"bread": 1, "milk": 2}
> cents = {name: p * 100 for name, p in prices.items()}
> print(cents)
> ```
>
> ```sortie
> {'bread': 100, 'milk': 200}
> ```

### Inverting a dictionary

A classic use: swapping keys and values.

```python
codes = {"FR": "France", "BE": "Belgium"}
country_to_code = {name: code for code, name in codes.items()}
print(country_to_code)
```

```sortie
{'France': 'FR', 'Belgium': 'BE'}
```

## Set comprehensions

Think of a guest list where some names were written twice. You want each person only
once: that is exactly what a set does.

With curly braces but **without** a colon, you get a
[set](../debuter/glossaire.md#set): each value appears in it only once.

```python
words = ["Cat", "dog", "CAT", "Fish", "dog"]
unique = {w.lower() for w in words}
print(sorted(unique))
```

```sortie
['cat', 'dog', 'fish']
```

Line by line:

- `words` holds five words, with duplicates written in different ways;
- `w.lower()` puts each word in lowercase: `"Cat"` and `"CAT"` both become `"cat"`;
- the curly braces put these words in a set, which keeps each value only once;
- `sorted(unique)` gives a list sorted in alphabetical order, for the display.

A set has no order: you sort it with `sorted` to get a stable display.

| Syntax | Result |
|----------|----------|
| `[x for x in ...]` | a list |
| `{x for x in ...}` | a set |
| `{k: v for ... in ...}` | a dictionary |
| `(x for x in ...)` | a generator expression (see below) |

## Nested comprehensions

> **Note:** this part can wait. Read it once the basics feel solid.

Sometimes you need to combine two lists: each color with each size, as in a clothes
shop.

A comprehension can contain **several `for`**. They read in the same order as nested
loops: the first `for` is the outer loop.

```python
colors = ["red", "blue"]
sizes = ["S", "M"]
items = [f"{c}-{s}" for c in colors for s in sizes]
print(items)
```

```sortie
['red-S', 'red-M', 'blue-S', 'blue-M']
```

Line by line:

- the first `for` takes `"red"`;
- the second `for` then goes through all the sizes: you get `"red-S"`, then `"red-M"`;
- the first `for` then moves on to `"blue"`, and the second starts again: `"blue-S"`,
  then `"blue-M"`.

It is the same as:

```python
colors = ["red", "blue"]
sizes = ["S", "M"]
items = []
for c in colors:
    for s in sizes:
        items.append(f"{c}-{s}")
print(items)
```

```sortie
['red-S', 'red-M', 'blue-S', 'blue-M']
```

### Flattening a list of lists

Two `for` are often used to "flatten" a grid into a single list:

```python
grid = [[1, 2, 3], [4, 5, 6]]
flat = [n for row in grid for n in row]
print(flat)
```

```sortie
[1, 2, 3, 4, 5, 6]
```

### A comprehension inside a comprehension

The other way around, a comprehension placed **on the left** of another one builds a
list of lists. Here, a multiplication table:

```python
table = [[row * col for col in range(1, 4)] for row in range(1, 4)]
for row in table:
    print(row)
```

```sortie
[1, 2, 3]
[2, 4, 6]
[3, 6, 9]
```

## Generator expressions

With **parentheses** instead of square brackets, you get a **generator expression**. It
does not build the list in memory: it produces the values one by one, when they are
asked for. It is a tap rather than a bucket filled in advance.

It is ideal with functions that consume values: `sum`, `min`, `max`, `any`, `all`. When
it is the only argument, the function's parentheses are enough.

```python
grades = [12, 7, 15, 9, 18]
print(sum(n * 2 for n in grades))
print(max(len(w) for w in ["one", "three", "four"]))
```

```sortie
122
5
```

Line by line:

- `n * 2 for n in grades` produces the doubled grades, one by one: 24, 14, 30, 18, 36;
- `sum` adds them up as they come: the total is 122;
- `len(w) for w in [...]` produces the lengths 3, 5 and 4; `max` keeps the largest.

### any and all

- `any(...)` is `True` if **at least one** value is true;
- `all(...)` is `True` if **all** the values are true.

```python
grades = [12, 7, 15, 9, 18]
print(any(n >= 18 for n in grades))
print(all(n >= 10 for n in grades))
print(all(n >= 0 for n in grades))
```

```sortie
True
False
True
```

These two functions stop as soon as they know the answer: `any` stops at the first
`True`, `all` at the first `False`.

> **Tip:** a generator expression can be gone through **only once**. Once it is empty,
> it gives nothing back. To reuse the values, make a list. The chapter
> [iterators and generators](iterateurs.md) explains why.

```python
squares = (n * n for n in range(4))
print(sum(squares))
print(sum(squares))
```

```sortie
14
0
```

**Your turn:** write a program that starts from `temperatures = [18, 22, 25, 19]`, then
displays whether at least one temperature is above 24 (with `any`), and whether all of
them are at least 15 (with `all`).

> **Solution**
>
> ```python
> temperatures = [18, 22, 25, 19]
> print(any(t > 24 for t in temperatures))
> print(all(t >= 15 for t in temperatures))
> ```
>
> ```sortie
> True
> True
> ```

## Knowing when to stop

A comprehension should stay **readable at a glance**. This one is correct, but hard to
read back:

```python
result = [x * y for x in range(1, 5) if x % 2 == 0 for y in range(1, 5) if y > x]
print(result)
```

```sortie
[6, 8]
```

The same thing with loops is longer, but each step is clear:

```python
result = []
for x in range(1, 5):
    if x % 2 == 0:
        for y in range(1, 5):
            if y > x:
                result.append(x * y)
print(result)
```

```sortie
[6, 8]
```

A few simple rules:

- **one** loop and **one** filter: the comprehension is perfect;
- two `for`: fine if it stays short;
- beyond that, or if the line runs off the screen: go back to a loop;
- if you need a `print` or an action (not a value), use a loop: a comprehension is for
  **building** a collection.

You can also move the calculation into a [function](fonctions.md) with a clear name:

```python
def is_prime(n):
    return n > 1 and all(n % d != 0 for d in range(2, n))

print([n for n in range(20) if is_prime(n)])
```

```sortie
[2, 3, 5, 7, 11, 13, 17, 19]
```

## Common mistakes

### Putting an if … else at the end

```python norun
numbers = [3, -1, 4]
positives = [n for n in numbers if n > 0 else 0]
print(positives)
```

```text
  File "main.py", line 2
    positives = [n for n in numbers if n > 0 else 0]
                                             ^^^^
SyntaxError: invalid syntax
```

An `if` placed at the end is a filter: it does not accept an `else`. To choose a value,
put the expression before the `for`: `[n if n > 0 else 0 for n in numbers]`.

### Forgetting the colon in a dictionary

```python norun
prices = {"apple": 2, "pear": 3}
doubled = {name, p * 2 for name, p in prices.items()}
print(doubled)
```

```text
  File "main.py", line 2
    doubled = {name, p * 2 for name, p in prices.items()}
               ^^^^^^^^^^^
SyntaxError: did you forget parentheses around the comprehension target?
```

A comma between the key and the value does not create a dictionary. Write
`{name: p * 2 for name, p in prices.items()}`.

### Using the loop variable afterwards

```python norun
squares = [n * n for n in range(4)]
print(n)
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(n)
          ^
NameError: name 'n' is not defined
```

Unlike a `for` loop, the variable of a comprehension only exists **inside** the square
brackets. Use the result (`squares`), not the variable.

### Taking an element from a generator expression

```python norun
squares = (n * n for n in range(4))
print(squares[0])
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(squares[0])
          ~~~~~~~^^^
TypeError: 'generator' object is not subscriptable
```

Parentheses give a generator, not a list: it has no index. Use square brackets
`[n * n for n in range(4)]` if you need `squares[0]`.

## Test yourself

**Question 1:** rewrite this loop as a comprehension.

```python norun
uppercase = []
for word in ["yes", "no"]:
    uppercase.append(word.upper())
```

> **Answer**
>
> ```python
> uppercase = [word.upper() for word in ["yes", "no"]]
> print(uppercase)
> ```
>
> ```sortie
> ['YES', 'NO']
> ```

**Question 2:** what is the type of `{x % 3 for x in range(10)}`, and what does it
contain?

> **Answer**
>
> It is a set (`set`), because it has curly braces without a colon. It contains
> `{0, 1, 2}`: the possible remainders, each one only once.

**Question 3:** how can you tell whether a list `ages` contains at least one minor
(under 18), without building a new list?

> **Answer**
>
> With `any` and a generator expression:
>
> ```python
> ages = [34, 17, 52]
> print(any(age < 18 for age in ages))
> ```
>
> ```sortie
> True
> ```

## Summary

- `[expression for x in collection]` builds a new list from another one.
- An `if` at the end filters; an `a if condition else b` at the start chooses a value.
- `{k: v for ...}` builds a dictionary, `{x for ...}` a set.
- `(x for ...)` is a generator expression: perfect with `sum`, `any`, `all`, and usable
  only once.
- If the comprehension gets hard to read, go back to a `for` loop.

Next: [Suivre un programme pas à pas](pas-a-pas.md)
