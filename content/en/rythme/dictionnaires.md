# Dictionaries and sets

This page looks more closely at two collections: the dictionary, which finds a value by its key, and the set, which keeps each value only once.

> **Goals:**
>
> - you will know how to create a dictionary and read, add or remove a key;
> - you will know how to go through a dictionary, its keys and its values;
> - you will know how to count words with a dictionary;
> - you will know how to create a set and combine two sets;
> - you will know how to choose between a record, a dictionary and a set.

## Creating a dictionary

A [dictionary](../debuter/glossaire.md#dictionary) links a **value** to
each **key**. It's like a phone book: you look up a name (the key), you
find a number (the value).

Rythme has no special syntax for dictionaries: you call the `dict`
function with a list of `[key, value]` pairs:

```rythme
capitals = dict([["France", "Paris"], ["Italy", "Rome"], ["Japan", "Tokyo"]])
print(capitals)
print(len(capitals))
```

```sortie
dict("France" => "Paris", "Italy" => "Rome", "Japan" => "Tokyo")
3
```

The display puts an arrow `=>` between each key and its value. This form is
only used for display: in your code, always write
`dict([[key, value], ...])`.

`dict()` is the empty dictionary. When a key appears twice, the last value
wins:

```rythme
print(dict(), dict([[1, "one"], [2, "two"], [1, "ONE"]]))
```

```sortie
dict() dict(1 => "ONE", 2 => "two")
```

## All kinds of keys

Keys are not always texts. They can be numbers,
[symbols](../debuter/glossaire.md#symbol), and even lists. Here, a cell of
a grid is identified by its coordinates `[x, y]`:

```rythme
grid = dict([[[0, 0], "start"], [[2, 1], "finish"]])
print(grid[[2, 1]])
stock = dict([[:apple, 3], [:pear, 0]])
print(stock[:apple])
```

```sortie
finish
3
```

Only functions cannot be keys.

> **Note:** a dictionary is always sorted by its keys (alphabetical order
> for texts), whatever the order in which you built it.

## Reading a value

There are two ways to read, as with lists:

- `dict[key]` gives the value, and stops the program if the key is missing;
- `get(dict, key)` gives an `Option`: `Some(value)` or `None`.

`has(dict, key)` tells whether the key exists:

```rythme
capitals = dict([["France", "Paris"], ["Japan", "Tokyo"]])
print(capitals["Japan"])
print(get(capitals, "Japan"), get(capitals, "Chile"))
print(has(capitals, "France"), has(capitals, "Chile"))
print(unwrap_or(get(capitals, "Chile"), "unknown"))
```

```sortie
Tokyo
Some("Tokyo") None
true false
unknown
```

Use `[...]` when you are sure the key exists, and `get` otherwise. Options
are explained in [handling errors](erreurs.md).

## Adding and removing a key

A dictionary never changes: it is
[immutable](../debuter/glossaire.md#immutable). `put` and `remove` build a
**new** dictionary:

```rythme
capitals = dict([["France", "Paris"], ["Italy", "Rome"]])
more = put(capitals, "Chile", "Santiago")
fewer = remove(capitals, "Italy")
fixed = put(capitals, "France", "PARIS")
print(more)
print(fewer)
print(fixed)
print(capitals)
```

```sortie
dict("Chile" => "Santiago", "France" => "Paris", "Italy" => "Rome")
dict("France" => "Paris")
dict("France" => "PARIS", "Italy" => "Rome")
dict("France" => "Paris", "Italy" => "Rome")
```

`put` adds the key, or replaces its value if it already exists. The
original `capitals` dictionary has not moved.

## Keys, values and pairs

```rythme
capitals = dict([["France", "Paris"], ["Italy", "Rome"]])
print(keys(capitals))
print(values(capitals))
print(entries(capitals))
```

```sortie
["France", "Italy"]
["Paris", "Rome"]
[["France", "Paris"], ["Italy", "Rome"]]
```

| Function | What it gives |
|----------|---------------|
| `keys(d)` | the list of keys |
| `values(d)` | the list of values |
| `entries(d)` | the list of `[key, value]` pairs |

These three functions give **lists**: you can then use `map`, `sort`,
`filter` and all the list functions.

## Going through a dictionary

`each` calls a function for each pair, in key order. The function receives
**two** values: the key and the value.

```rythme
capitals = dict([["France", "Paris"], ["Japan", "Tokyo"]])
each(capitals, fn(country, city) = print("{country}: {city}"))
capitals.each do |country, city|
  print("The capital of {country} is {city}.")
end
```

```sortie
France: Paris
Japan: Tokyo
The capital of France is Paris.
The capital of Japan is Tokyo.
```

The style with `do |country, city|` is Ruby's; both ways of writing it do
the same thing.

`filter` keeps the chosen pairs, and `map_values` transforms each value:

```rythme
prices = dict([["apple", 3], ["pear", 5], ["fig", 9]])
print(filter(prices, fn(fruit, p) = p < 6))
print(map_values(prices, fn(p) = p * 2))
```

```sortie
dict("apple" => 3, "pear" => 5)
dict("apple" => 6, "fig" => 18, "pear" => 10)
```

## Counting words

This is the most classic use of a dictionary: each word is a key, and its
value is the number of times you have seen it.

The idea: you start from an empty dictionary, and for each word, you add 1
to its counter (0 if it doesn't exist yet). `fold` passes the dictionary
from one word to the next (see [thinking functionally](fonctionnel.md)):

```rythme
fn count(counts, word)
  already = unwrap_or(get(counts, word), 0)
  put(counts, word, already + 1)
end

text = "the cat and the dog and the cat"
counts = fold(split(text), dict(), count)
print(counts)
print(counts["the"])
```

```sortie
dict("and" => 2, "cat" => 2, "dog" => 1, "the" => 3)
3
```

To find the most frequent words, you go through the list of pairs:

```rythme
counts = dict([["and", 2], ["cat", 2], ["dog", 1], ["the", 3]])
podium = entries(counts) |> sort_by(fn(pair) = -pair[1]) |> take(2)
print(podium)
```

```sortie
[["the", 3], ["and", 2]]
```

Sorting by `-pair[1]` goes from the largest number to the smallest.

## Creating a set

A **set** (`set`) holds values **without duplicates**, like a stamp album
where each stamp appears only once. You build it with `set` and a list:

```rythme
colors = set(["red", "green", "red", "blue"])
print(colors, len(colors))
print(set(chars("banana")))
print(set(), is_empty(set()))
```

```sortie
set("blue", "green", "red") 3
set("a", "b", "n")
set() true
```

Like a dictionary, a set is sorted by its values.

## Adding, removing, searching

The names are the same as for lists and dictionaries:

```rythme
friends = set(["Ada", "Alan", "Grace"])
print(push(friends, "Linus"))
print(remove(friends, "Ada"))
print(contains(friends, "Ada"), has(friends, "Linus"))
print(values(friends))
```

```sortie
set("Ada", "Alan", "Grace", "Linus")
set("Alan", "Grace")
true false
["Ada", "Alan", "Grace"]
```

- `push(s, x)` gives the set with `x` added;
- `remove(s, x)` gives the set without `x`;
- `contains(s, x)` and `has(s, x)` tell whether `x` is in it;
- `values(s)` gives its elements as a list.

Searching in a set is very fast, even with millions of elements.

## Combining two sets

Think of two overlapping circles: Ada's friends and Linus's friends.

```rythme
a = set(["Alan", "Grace", "Tim"])
b = set(["Alan", "Linus", "Tim"])
print(union(a, b))
print(intersection(a, b))
print(difference(a, b))
print(is_subset(set(["Alan"]), a))
```

```sortie
set("Alan", "Grace", "Linus", "Tim")
set("Alan", "Tim")
set("Grace")
true
```

| Function | What it gives | With the circles |
|----------|---------------|------------------|
| `union(a, b)` | what is in `a` **or** in `b` | both whole circles |
| `intersection(a, b)` | what is in `a` **and** in `b` | the shared part |
| `difference(a, b)` | what is in `a` but **not** in `b` | `a` without the shared part |
| `is_subset(a, b)` | `true` if all of `a` is in `b` | `a` is inside `b` |

`union` also works with two dictionaries: for a key present on both sides,
the value of the second one wins.

## Record, dictionary or set?

All three look alike. One simple question helps you choose: **do you know
the names in advance?**

| Collection | When to choose it | Example |
|------------|-------------------|---------|
| [record](../debuter/glossaire.md#record) | a form whose fields are known when you write the program | `{name: "Ada", age: 36}` |
| dictionary | keys that come with the data: words, first names, coordinates | `dict([["Ada", 36], ["Alan", 41]])` |
| set | only knowing **whether** a value is present, without duplicates | `set(["Ada", "Alan"])` |

A student always has a `name` and a `grade`: that's a record. The words of a
book are not known in advance: to count them, use a dictionary. To know
which words you have already seen, a set is enough.

## Common mistakes

### Reading a key that doesn't exist

```rythme norun
stock = dict([["apple", 3], ["pear", 0]])
print(stock["kiwi"])
```

```text
error[E010]: this dict has no key "kiwi"
 --> main.ry:2:13
  |
2 | print(stock["kiwi"])
  |             ^^^^^^
  |
  = help: use `get(dict, key)` to receive an Option instead, or `has(dict, key)` to check first
  = more about this error: `rythme explain E010`
```

If the key can be missing, write `get(stock, "kiwi")`, or
`unwrap_or(get(stock, "kiwi"), 0)` to get 0 by default.

### Forgetting the brackets around the pairs

```rythme norun
ages = dict(["Ada", 36])
```

```text
error[E023]: `dict` needs a list of [key, value] pairs, but it contains Text "Ada"
 --> main.ry:1:8
  |
1 | ages = dict(["Ada", 36])
  |        ^^^^^^^^^^^^^^^^^
  |
  = help: each pair is a list of two elements, like `dict([["Ada", 36], ["Alan", 41]])`
  = more about this error: `rythme explain E023`
```

You need a list **of pairs**: `dict([["Ada", 36]])`, with two sets of
brackets.

### Looking for an add function

```rythme norun
seen = set([1, 2])
print(add(seen, 3))
```

```text
error[E003]: unknown name `add`
 --> main.ry:2:7
  |
2 | print(add(seen, 3))
  |       ^^^ not defined
  |
  = help: in Rythme, this is called `push`
  = more about this error: `rythme explain E003`
```

For a set as for a list, you "add" with `push`.

### Thinking that put changes the dictionary

```rythme norun
stock = dict([["apple", 3]])
put(stock, "kiwi", 5)
print(stock)
```

```text
warning[W008]: the result of `put` is lost
 --> main.ry:2:1
  |
2 | put(stock, "kiwi", 5)
  | ^^^^^^^^^^^^^^^^^^^^^ this value is lost
  |
  = help: values never change: `put(record, name, value)` gives a new value, and changes nothing.
      To keep it, give it a name: `new = put(record, name, value)`

dict("apple" => 3)
```

The kiwi doesn't appear. Keep the result: `stock2 = put(stock, "kiwi", 5)`.

## Test yourself

**Question 1:** with `d = dict([["a", 1], ["b", 2]])`, what are
`get(d, "b")`, `get(d, "z")` and `keys(d)`?

> **Answer**
>
> `Some(2)`, `None` and `["a", "b"]`.

**Question 2:** what does `print(len(set([1, 2, 2, 3, 3, 3])))` print?

> **Answer**
>
> `3`: a set keeps each value only once, so it contains 1, 2 and 3.

**Question 3:** which students take both the piano class and the dance
class?

```rythme norun
piano = set(["Ada", "Léa", "Tom"])
dance = set(["Léa", "Sam", "Tom"])
```

> **Answer**
>
> `intersection(piano, dance)`, which is `set("Léa", "Tom")`.

## Summary

- `dict([[key, value], ...])` creates a dictionary; keys can be texts,
  numbers, symbols, lists...
- `d[key]` reads a value that is sure to exist, `get(d, key)` one that may
  be missing, `has(d, key)` checks.
- `put` and `remove` give a new dictionary; `keys`, `values` and `entries`
  give lists.
- `set(list)` creates a set without duplicates; `push`, `remove`,
  `contains`, `union`, `intersection` and `difference` work with it.
- Fields known in advance: record; keys that come from the data:
  dictionary; just presence: set.

Next: [following a program step by step](pas-a-pas.md).
