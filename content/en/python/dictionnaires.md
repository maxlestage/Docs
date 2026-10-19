# Dictionaries and sets

This page explains how to link keys to values with dictionaries, and how to work with collections without duplicates using sets.

> **Goals:**
>
> - you will be able to create a dictionary, read its values and change them;
> - you will be able to loop over its keys, its values or both;
> - you will be able to count words with a dictionary, then with `Counter`;
> - you will be able to use a set and its operations (union, intersection...);
> - you will be able to choose between list, tuple, dictionary and set.

## Creating a dictionary

A [dictionary](../debuter/glossaire.md#dictionary) (`dict`) links **keys** to
**values**. Think of a real dictionary: you look up a word (the key) to find its
definition (the value). Or think of a phone book: a name gives a number.

You write it between curly braces `{ }`, with `key: value` pairs separated by commas.

```python
ages = {"Alice": 30, "Bob": 25, "Chloe": 35}
capitals = {"France": "Paris", "Italy": "Rome"}
empty = {}

print(ages)
print(len(ages))
print(capitals["Italy"])
print(empty)
```

```sortie
{'Alice': 30, 'Bob': 25, 'Chloe': 35}
3
Rome
{}
```

- Keys are often strings or numbers. Each key is **unique**.
- Values can be anything, even lists or other dictionaries.
- The dictionary keeps the order in which the keys were added.

You can also create one with `dict(...)` and keyword arguments:

```python
person = dict(name="Alice", city="London")
print(person)
```

```sortie
{'name': 'Alice', 'city': 'London'}
```

## Reading a value: [] or get

There are two ways to read a value:

- `d[key]`: if the key does not exist, you get an error (`KeyError`);
- `d.get(key)`: if the key does not exist, it returns `None`, or the default value
  you give.

```python
stock = {"apple": 12, "pear": 0}
print(stock["apple"])
print(stock.get("apple"))
print(stock.get("kiwi"))
print(stock.get("kiwi", 0))
print("kiwi" in stock)
```

```sortie
12
12
None
0
False
```

`in` checks whether a **key** exists (not a value).

> **Remember:** use `d[key]` when the key must exist (if it is missing, that is a
> bug). Use `d.get(key, default)` when it may be missing.

## Adding, changing, removing

To add or change an entry, assign a value to a key. If the key exists, its value is
replaced; otherwise, it is created.

```python
ages = {"Alice": 30}
ages["Bob"] = 25
ages["Alice"] = 31
print(ages)

ages.update({"Chloe": 35, "Bob": 26})
print(ages)

del ages["Bob"]
print(ages)

chloe_age = ages.pop("Chloe")
print(chloe_age, ages)
```

```sortie
{'Alice': 31, 'Bob': 25}
{'Alice': 31, 'Bob': 26, 'Chloe': 35}
{'Alice': 31, 'Chloe': 35}
35 {'Alice': 31}
```

- `d[key] = value` adds or replaces;
- `update` adds or replaces several keys at once;
- `del d[key]` removes; `pop(key)` removes and returns the value.

## Keys, values and pairs

Three methods give a view of the contents:

- `keys()`: the keys;
- `values()`: the values;
- `items()`: the `(key, value)` pairs.

```python
grades = {"Alice": 15, "Bob": 12, "Chloe": 18}
print(list(grades.keys()))
print(list(grades.values()))
print(list(grades.items()))
print(sum(grades.values()) / len(grades))
```

```sortie
['Alice', 'Bob', 'Chloe']
[15, 12, 18]
[('Alice', 15), ('Bob', 12), ('Chloe', 18)]
15.0
```

## Looping over a dictionary

A `for` loop over a dictionary goes through its **keys**. To get the key and the value
together, loop over `items()` and unpack each pair.

```python
grades = {"Alice": 15, "Bob": 12, "Chloe": 18}

for name in grades:
    print(name)

for name, grade in grades.items():
    print(f"{name}: {grade}/20")
```

```sortie
Alice
Bob
Chloe
Alice: 15/20
Bob: 12/20
Chloe: 18/20
```

To loop in a different order, sort. `sorted(grades.items(), key=...)` sorts the pairs,
for example by grade, highest first:

```python
grades = {"Alice": 15, "Bob": 12, "Chloe": 18}
for name, grade in sorted(grades.items(), key=lambda pair: pair[1], reverse=True):
    print(name, grade)
```

```sortie
Chloe 18
Alice 15
Bob 12
```

## Complex values

A value can be a list or another dictionary. This is how you build rich data, like a
record card.

```python
student = {
    "name": "Alice",
    "grades": [15, 12, 17],
    "address": {"city": "London", "code": "SW1A 1AA"},
}
print(student["grades"][0])
print(student["address"]["city"])
student["grades"].append(19)
print(student["grades"])
```

```sortie
15
London
[15, 12, 17, 19]
```

Read `student["address"]["city"]` from left to right: the record, then its address,
then the city of that address.

## Counting words

Counting is the most classic use of a dictionary: the key is the word, the value is the
number of times it appears.

```python
text = "the cat and the dog and the rabbit"
counter = {}
for word in text.split():
    counter[word] = counter.get(word, 0) + 1
print(counter)
```

```sortie
{'the': 3, 'cat': 1, 'and': 2, 'dog': 1, 'rabbit': 1}
```

`counter.get(word, 0)` returns the current count, or 0 if the word is new. You add 1,
then store the result.

### With collections.Counter

The `collections` module provides `Counter`, a dictionary made for counting. It does
all the work in one line, and can give you the most frequent items.

```python
from collections import Counter

text = "the cat and the dog and the rabbit"
counter = Counter(text.split())
print(counter["the"])
print(counter["tiger"])
print(counter.most_common(2))
print(Counter("abracadabra").most_common(3))
```

```sortie
3
0
[('the', 3), ('and', 2)]
[('a', 5), ('b', 2), ('r', 2)]
```

A missing word counts as 0, with no error. `most_common(n)` gives the `n` most
frequent items.

### With defaultdict

`defaultdict` automatically creates a starting value for a missing key. You give it the
type of that value: `list` for an empty list, `int` for 0.

```python
from collections import defaultdict

words = ["apple", "boat", "avocado", "banana", "cherry"]
by_letter = defaultdict(list)
for word in words:
    by_letter[word[0]].append(word)

print(dict(by_letter))
```

```sortie
{'a': ['apple', 'avocado'], 'b': ['boat', 'banana'], 'c': ['cherry']}
```

Without `defaultdict`, you would have to check each time whether the key exists before
adding to its list.

## Sets

A [set](../debuter/glossaire.md#set) (`set`) is a collection **without duplicates**
and **without order**. Think of a bag of marbles: each marble is in it only once, and
they are not arranged in any order. You write it between curly braces, without `:`.

```python
colors = {"red", "green", "blue", "red"}
print(len(colors))
print("green" in colors)

colors.add("yellow")
colors.discard("red")
print(sorted(colors))

grades = [12, 15, 12, 18, 15]
print(sorted(set(grades)))
```

```sortie
3
True
['blue', 'green', 'yellow']
[12, 15, 18]
```

- the duplicate `"red"` is kept only once;
- `add` adds, `discard` removes (with no error if the item is missing);
- `set(list)` removes the duplicates from a list.

Since a set has no order, you sort it with `sorted` before printing it.

> **Warning:** `{}` creates an empty dictionary, not a set. For an empty set, write
> `set()`.

### Set operations

Sets can do the operations you know from math:

| Operation | Method | Contains |
| --- | --- | --- |
| union | `a.union(b)` | the items in `a` **or** in `b` |
| intersection | `a.intersection(b)` | the items in `a` **and** in `b` |
| difference | `a.difference(b)` | the items in `a` that are not in `b` |
| symmetric difference | `a.symmetric_difference(b)` | the items in only one of the two |

Each operation also has a shorter symbol: `a | b` for the union, `a & b` for the
intersection, `a - b` for the difference and `a ^ b` for the symmetric difference.

```python
alice = {"piano", "tennis", "reading"}
bob = {"tennis", "cooking", "reading", "cycling"}

print(sorted(alice | bob))
print(sorted(alice & bob))
print(sorted(alice - bob))
print(sorted(alice ^ bob))
print({"tennis"} <= alice)
```

```sortie
['cooking', 'cycling', 'piano', 'reading', 'tennis']
['reading', 'tennis']
['piano']
['cooking', 'cycling', 'piano']
True
```

`<=` checks that one set is **contained** in another.

> **Tip:** `in` is very fast on a set, even with a million items. On a list, Python has
> to go through everything. For many lookups, prefer a set.

### frozenset: a frozen set

A `frozenset` is an [immutable](../debuter/glossaire.md#immutable) set: you can no
longer add or remove anything. So it can be used as a dictionary key, which is not
allowed for an ordinary `set`.

```python
vowels = frozenset("aeiouy")
print("e" in vowels)

trips = {frozenset({"Paris", "Lyon"}): 465}
print(trips[frozenset({"Lyon", "Paris"})])
```

```sortie
True
465
```

Order does not matter in a set: `{"Paris", "Lyon"}` and `{"Lyon", "Paris"}` mean the
same trip.

## Which collection should you choose?

| Collection | Syntax | Ordered | Mutable | Duplicates | Typical use |
| --- | --- | --- | --- | --- | --- |
| `list` | `[1, 2, 3]` | yes | yes | yes | a sequence of items: shopping, grades |
| `tuple` | `(1, 2)` | yes | no | yes | a fixed group: coordinates, date |
| `dict` | `{"a": 1}` | yes (insertion order) | yes | unique keys | find a value by its key |
| `set` | `{1, 2}` | no | yes | no | remove duplicates, test membership |
| `frozenset` | `frozenset({1, 2})` | no | no | no | a set used as a key |

Ask yourself two questions:

- "Am I looking things up by **position** or by **name**?" Position: list or tuple.
  Name: dictionary.
- "Do duplicates make sense?" No: set.

## Common mistakes

### Reading a missing key

```python norun
ages = {"Alice": 30}
print(ages["Bob"])
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 2, in <module>
    print(ages["Bob"])
          ~~~~^^^^^^^
KeyError: 'Bob'
```

The key `"Bob"` does not exist. Use `ages.get("Bob")`, or check first with
`if "Bob" in ages:`.

### A list as a key

```python norun
d = {[1, 2]: "a"}
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 1, in <module>
    d = {[1, 2]: "a"}
        ^^^^^^^^^^^^^
TypeError: unhashable type: 'list'
```

A key must be immutable: a list can change, so it is refused (*unhashable*). Use a
tuple: `{(1, 2): "a"}`.

### Removing while looping

```python norun
stock = {"apple": 0, "pear": 3}
for fruit in stock:
    if stock[fruit] == 0:
        del stock[fruit]
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 2, in <module>
    for fruit in stock:
                 ^^^^^
RuntimeError: dictionary changed size during iteration
```

You cannot change the size of a dictionary while you loop over it. Loop over a copy of
the keys: `for fruit in list(stock):`.

### Thinking {} is an empty set

```python norun
s = {}
s.add(1)
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 2, in <module>
    s.add(1)
    ^^^^^
AttributeError: 'dict' object has no attribute 'add'
```

`{}` is an empty dictionary. For an empty set, write `s = set()`.

## Test yourself

**Question 1:** what does `{"a": 1}.get("b", 5)` return?

> **Answer**
>
> `5`. The key `"b"` does not exist, so `get` returns the default value.

**Question 2:** how do you count the letters of `"mississippi"` and print the most
frequent one?

> **Answer**
>
> ```python
> from collections import Counter
>
> print(Counter("mississippi").most_common(1))
> ```
>
> ```sortie
> [('i', 4)]
> ```
>
> `i` and `s` both appear 4 times; in a tie, `Counter` keeps the order in which they
> first appeared, so `i` comes first.

**Question 3:** which students are members of both clubs?

```python norun
chess = {"Alice", "Bob", "Chloe"}
theater = {"Bob", "David", "Chloe"}
```

> **Answer**
>
> The intersection:
>
> ```python
> chess = {"Alice", "Bob", "Chloe"}
> theater = {"Bob", "David", "Chloe"}
> print(sorted(chess & theater))
> ```
>
> ```sortie
> ['Bob', 'Chloe']
> ```

## Summary

- A dictionary `{key: value}` finds a value by its key; keys are unique and
  immutable.
- `d[key]` fails if the key is missing; `d.get(key, default)` does not.
- `keys()`, `values()` and `items()` give the keys, the values and the pairs;
  `for key, value in d.items():` goes through everything.
- `Counter` counts, `defaultdict` creates starting values on its own.
- A set `{...}` has no order and no duplicates; `|`, `&`, `-` and `^` combine sets;
  `set()` is the empty set.

Next: [comprehensions](comprehensions.md).
