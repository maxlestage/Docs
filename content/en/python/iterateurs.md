# Iterators and generators

This page explains what really happens in a `for` loop, how to write your own generators with `yield`, and how the `itertools` module handles sequences of values, even infinite ones.

> **Goals:**
>
> - you will be able to tell an iterable from an iterator;
> - you will be able to use `iter` and `next`, and recognise `StopIteration`;
> - you will be able to write a generator with `yield` and chain several generators;
> - you will be able to use the main tools of `itertools`;
> - you will know what lazy evaluation is.

## Iterable and iterator

These two words look alike, but they do not mean the same thing:

- an **iterable** is anything you can go through with `for`: a list, a string, a
  dictionary, a `range`, a file…;
- an [iterator](../debuter/glossaire.md#iterateur) is the object that does the walking.
  It remembers **where it is** and gives you the next element when you ask for it.

Think of a book: the book is the iterable; the **bookmark** is the iterator. Several
readers can read the same book, each with their own bookmark.

## iter and next

`iter(iterable)` creates an iterator; `next(iterator)` gives the next element.

```python
fruits = ["apple", "pear", "kiwi"]
it = iter(fruits)
print(next(it))
print(next(it))
print(next(it))
```

```sortie
apple
pear
kiwi
```

When there is nothing left, `next` raises the `StopIteration` exception. You can also
give `next` a default value, returned at the end instead of the error:

```python
it = iter(["apple"])
print(next(it))
print(next(it, "done"))
```

```sortie
apple
done
```

### What for really does

A `for` loop is just a shortcut: it calls `iter`, then `next` over and over, and stops
cleanly at the first `StopIteration`. These two pieces of code do the same thing:

```python
for letter in "abc":
    print(letter)

it = iter("abc")
while True:
    try:
        letter = next(it)
    except StopIteration:
        break
    print(letter)
```

```sortie
a
b
c
a
b
c
```

### An iterator gets used up

An iterator never goes backwards. Once it reaches the end, it is empty.

```python
it = iter([1, 2, 3])
print(list(it))
print(list(it))
```

```sortie
[1, 2, 3]
[]
```

A list, on the other hand, can be walked through as many times as you like: each `for`
creates a new iterator.

## Writing a generator with yield

A **generator** is the simplest way to create an iterator. It is a function that
contains `yield` ("produce") instead of `return`.

```python
def countdown(n):
    while n > 0:
        yield n
        n -= 1
    print("Liftoff!")

for x in countdown(3):
    print(x)
```

```sortie
3
2
1
Liftoff!
```

What happens?

1. Calling `countdown(3)` **runs nothing**: it returns a generator, paused at the start
   of the function.
2. Each `next` (here, done by `for`) moves the function forward to the next `yield`.
   The value is given out, then the function **pauses**, keeping its variables.
3. When the function ends, the generator raises `StopIteration`, and the loop stops.

You can see it clearly by calling `next` by hand:

```python
def steps():
    print("start")
    yield 1
    print("middle")
    yield 2

g = steps()
print("generator created")
print(next(g))
print(next(g))
```

```sortie
generator created
start
1
middle
2
```

> **Remember:** `return` gives back a value and **ends** the function. `yield` gives back
> a value and **pauses** the function, which will resume at the same place.

## Generator pipelines

Generators can be plugged into each other, like pipes: each one receives the values
from the previous one, transforms them and passes them on to the next. The values flow
**one at a time** through the whole chain.

```python
lines = [
    "alice,15",
    "",
    "bob,8",
    "# comment",
    "chloé,12",
]

def non_empty(lines):
    for line in lines:
        if line and not line.startswith("#"):
            yield line

def split_fields(lines):
    for line in lines:
        name, grade = line.split(",")
        yield name, int(grade)

def passed(students):
    for name, grade in students:
        if grade >= 10:
            yield name

for name in passed(split_fields(non_empty(lines))):
    print(name)
```

```sortie
alice
chloé
```

Each step is a small function that is easy to test. And if `lines` were a file with a
million lines, only one line would be in memory at a time.

### yield from

`yield from` gives out, one at a time, all the values of another iterable:

```python
def everything(*lists):
    for lst in lists:
        yield from lst

print(list(everything([1, 2], [3], [4, 5])))
```

```sortie
[1, 2, 3, 4, 5]
```

## Lazy evaluation

A generator is **lazy**: it only computes a value when you ask for it. This even lets
you write an **infinite** sequence:

```python
def even_numbers():
    n = 0
    while True:
        yield n
        n += 2

gen = even_numbers()
for _ in range(5):
    print(next(gen), end=" ")
print()
```

```sortie
0 2 4 6 8 
```

`while True` does not block the program: the function stops at each `yield` and waits
for you to ask for more. Be careful with `list(even_numbers())`, though: it would run
forever.

Another example, the Fibonacci sequence, where each number is the sum of the two
previous ones:

```python
def fibonacci():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b

for n in fibonacci():
    if n > 100:
        break
    print(n, end=" ")
print()
```

```sortie
0 1 1 2 3 5 8 13 21 34 55 89 
```

## The itertools module

The `itertools` module of the standard library provides ready-made iterators. They are
all lazy.

### count and islice

`count(start, step)` counts forever. `islice(iterable, n)` takes the first `n`
elements, like `[:n]` on a list, but for any iterator.

```python
from itertools import count, islice

print(list(islice(count(10, 5), 4)))

def squares():
    for n in count(1):
        yield n * n

print(list(islice(squares(), 6)))
print(list(islice("abcdefgh", 2, 6)))
```

```sortie
[10, 15, 20, 25]
[1, 4, 9, 16, 25, 36]
['c', 'd', 'e', 'f']
```

### chain

`chain` puts several iterables end to end, without creating a new list.

```python
from itertools import chain

breakfast = ["coffee", "croissant"]
lunch = ["salad"]
for dish in chain(breakfast, lunch, ["apple"]):
    print(dish)
```

```sortie
coffee
croissant
salad
apple
```

### groupby

`groupby` groups the **consecutive** elements that have the same key. So you must
**sort** by that same key first.

```python
from itertools import groupby

words = ["airplane", "bus", "apple", "boat", "cat"]
sorted_words = sorted(words, key=lambda w: w[0])
for letter, group in groupby(sorted_words, key=lambda w: w[0]):
    print(letter, list(group))
```

```sortie
a ['airplane', 'apple']
b ['bus', 'boat']
c ['cat']
```

### product

`product` gives all the combinations, like nested `for` loops.

```python
from itertools import product

for size, colour in product(["S", "M"], ["red", "blue"]):
    print(size, colour)
print(len(list(product("01", repeat=3))))
```

```sortie
S red
S blue
M red
M blue
8
```

`repeat=3` repeats the same iterable three times: there are 8 binary numbers with three
digits.

### accumulate

`accumulate` gives the running totals: each value is the sum of all the previous ones.

```python
from itertools import accumulate

expenses = [12, 5, 30, 8]
print(list(accumulate(expenses)))
print(list(accumulate(expenses, max)))
```

```sortie
[12, 17, 47, 55]
[12, 12, 30, 30]
```

With `max`, you get the largest number seen so far.

| Tool | Role |
|------|------|
| `count(start, step)` | counts forever |
| `islice(it, n)` / `islice(it, start, stop)` | slices an iterator |
| `chain(a, b, …)` | puts iterables end to end |
| `groupby(it, key)` | groups consecutive elements |
| `product(a, b)` | all the combinations |
| `accumulate(it)` | running totals |

## Common mistakes

### Calling next on a list

```python norun
fruits = ["apple", "pear"]
print(next(fruits))
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(next(fruits))
          ~~~~^^^^^^^^
TypeError: 'list' object is not an iterator
```

A list is iterable, but it is not an iterator. Create the bookmark first:
`next(iter(fruits))`.

### Going past the end

```python norun
it = iter(["apple"])
print(next(it))
print(next(it))
```

```text
apple
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    print(next(it))
          ~~~~^^^^
StopIteration
```

The iterator is empty after `apple`. Use a `for` loop, which stops on its own, or a
default value: `next(it, None)`.

### Asking for the length of a generator

```python norun
def evens(n):
    for i in range(0, n, 2):
        yield i

print(len(evens(10)))
```

```text
Traceback (most recent call last):
  File "main.py", line 5, in <module>
    print(len(evens(10)))
          ~~~^^^^^^^^^^^
TypeError: object of type 'generator' has no len()
```

A generator does not know how many values it will produce before it has produced them.
Turn it into a list: `len(list(evens(10)))`, or count with
`sum(1 for _ in evens(10))`.

## Test yourself

**Question 1:** what does this code print?

```python norun
def letters():
    yield "a"
    yield "b"

g = letters()
print(list(g), list(g))
```

> **Answer**
>
> `['a', 'b'] []`. The first `list(g)` empties the generator; the second one finds
> nothing left.

**Question 2:** why does `groupby` give two `a` groups for
`["airplane", "bus", "apple"]` if you do not sort?

> **Answer**
>
> Because `groupby` only groups elements **that follow each other**. `airplane` and
> `apple` are separated by `bus`: they form two groups. Sort first with the same key.

**Question 3:** write an infinite generator `multiples(k)`, then print the first 4
multiples of 7.

> **Answer**
>
> ```python
> from itertools import islice
>
> def multiples(k):
>     n = k
>     while True:
>         yield n
>         n += k
>
> print(list(islice(multiples(7), 4)))
> ```
>
> ```sortie
> [7, 14, 21, 28]
> ```

## Summary

- An iterable can be walked through; an iterator does the walking and remembers its
  position.
- `iter` creates an iterator, `next` gives the next element, `StopIteration` marks the
  end.
- A function with `yield` is a generator: it pauses at each value.
- Generators can be chained into pipelines and can be infinite: they are lazy.
- `itertools` provides `count`, `islice`, `chain`, `groupby`, `product`, `accumulate`.

Next: [advanced collections](collections-avancees.md)
