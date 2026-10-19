# Lists, records and dictionaries

This page shows the ways to group values together: lists, ranges, records, dictionaries and sets.

> **Goals:**
>
> - you will know how to create a list, read an element by its position, and "add" an element;
> - you will know how to use the common list functions: `sort`, `sum`, `filter`, `map`...;
> - you will know how to describe a card with a record, and build a changed version of it;
> - you will know how to find a value by its key in a dictionary;
> - you will know how to choose the right collection for your data.

## Lists

A [list](../debuter/glossaire.md#list) is a sequence of values, in a precise
order, like a shopping list. You write it between square brackets, with commas
between the elements.

Each element has a **position**, its
[index](../debuter/glossaire.md#index). Positions start at 0, and `-1` means
the last element:

```rythme
fruits = ["apple", "pear", "fig"]
print(fruits[0], fruits[-1], len(fruits))
```

```sortie
apple fig 3
```

`fruits[0]` is the first fruit, `fruits[2]` the third. `len` gives the number
of elements.

## Adding an element without changing anything

A list never changes. To "add" an element, you build a **new** list, and the
old one stays as it is:

```rythme
fruits = ["apple", "pear"]
more = push(fruits, "kiwi")
print(fruits)
print(more)
print(fruits + ["melon"], [..fruits, "plum"], ["cherry", ..fruits])
```

```sortie
["apple", "pear"]
["apple", "pear", "kiwi"]
["apple", "pear", "melon"] ["apple", "pear", "plum"] ["cherry", "apple", "pear"]
```

- `push(list, x)` gives the list with `x` at the end;
- `+` puts two lists end to end;
- `..fruits` between square brackets inserts all the elements of `fruits`, at
  the start, in the middle or at the end.

It's like photocopying the shopping list and writing on the copy: the
original doesn't move.

## Reading safely with get

`fruits[10]` stops the program: the element doesn't exist. `get` and `first`
give an `Option` instead: `Some(value)` if there is a value, `None`
otherwise.

```rythme
fruits = ["apple", "pear"]
print(get(fruits, 1), get(fruits, 10), first(fruits), first([]))
```

```sortie
Some("pear") None Some("apple") None
```

To get the value, give a default value with `unwrap_or`:

```rythme
fruits = ["apple", "pear"]
print(get(fruits, 10) |> unwrap_or("nothing"))
```

```sortie
nothing
```

Options are explained in [handling errors](erreurs.md).

## Ranges

`a..b` is the list of whole numbers from `a` to `b`, both included:

```rythme
print(1..5, len(1..100), 5..1)
print(0..20 |> step_by(5), reverse(1..3))
```

```sortie
[1, 2, 3, 4, 5] 100 []
[0, 5, 10, 15, 20] [3, 2, 1]
```

`5..1` is empty. To count in steps, use `step_by`; to count backwards,
`reverse`.

## Common list functions

```rythme
numbers = [3, 1, 4, 1, 5, 9, 2, 6]
print(sort(numbers), reverse(numbers), unique(numbers))
print(sum(numbers), max(numbers), min(numbers))
print(take(numbers, 3), drop(numbers, 5), contains(numbers, 9))
print(filter(numbers, is_even), map(numbers, fn(x) = x * 10))
```

```sortie
[1, 1, 2, 3, 4, 5, 6, 9] [6, 2, 9, 5, 1, 4, 1, 3] [3, 1, 4, 5, 9, 2, 6]
31 9 1
[3, 1, 4] [9, 2, 6] true
[4, 2, 6] [30, 10, 40, 10, 50, 90, 20, 60]
```

| Function | What it gives |
|----------|---------------|
| `sort`, `reverse`, `unique` | the list sorted, reversed, without duplicates |
| `sum`, `max`, `min` | the sum, the largest, the smallest |
| `take(xs, n)`, `drop(xs, n)` | the first `n`, or everything except the first `n` |
| `contains(xs, x)` | `true` if `x` is in the list |
| `filter(xs, f)` | the elements for which `f` answers `true` |
| `map(xs, f)` | the result of `f` for each element |
| `is_empty(xs)` | `true` if the list is empty |

> **Tip:** `rythme help lists` lists all the list functions, and
> `rythme help sort_by` explains one of them.

## Records

A **record** groups named values, its **fields**. It's a card: a "name" box,
a "born" box... You write it between curly braces, and you read a field with a
dot:

```rythme
ada = {name: "Ada", born: 1815, city: "London"}
print(ada.name, ada.born)
print(ada)
```

```sortie
Ada 1815
{name: "Ada", born: 1815, city: "London"}
```

Rythme has no classes: a record is enough to describe a person, a product or a
point. `keys` gives the names of the fields, `has` tells whether a field
exists, and `get` reads a field that may be missing:

```rythme
ada = {name: "Ada", age: 36}
print(keys(ada), has(ada, "age"), get(ada, "city"))
```

```sortie
["name", "age"] true None
```

## Updating a record

You don't change a record: you build a new one. `{..ada, born: 1816}` copies
all the fields of `ada`, then replaces `born`:

```rythme
ada = {name: "Ada", born: 1815}
fixed = {..ada, born: 1816}
with_city = {..ada, city: "London"}
print(ada)
print(fixed)
print(with_city)
```

```sortie
{name: "Ada", born: 1815}
{name: "Ada", born: 1816}
{name: "Ada", born: 1815, city: "London"}
```

`ada` has not changed.

## Taking a value apart

You can name the pieces of a list or a record all at once:

```rythme
[first_one, ..rest] = [10, 20, 30]
{name, born} = {name: "Ada", born: 1815}
print(first_one, rest, name, born)
```

```sortie
10 [20, 30] Ada 1815
```

`[first_one, ..rest]` means "the first element, then all the rest".
`{name, born}` takes the fields with those names. The same notation works as a
pattern in `match` (see [your own types](types.md)).

## A list of records

This is the most common shape of data, like the rows of a table:

```rythme
students = [
  {name: "Ada", grade: 17},
  {name: "Alan", grade: 12},
  {name: "Grace", grade: 19},
]
best = max_by(students, fn(s) = s.grade)
print(best.name)
print(students |> map(fn(s) = s.name))
print(sort_by(students, fn(s) = s.grade) |> map(fn(s) = s.grade))
```

```sortie
Grace
["Ada", "Alan", "Grace"]
[12, 17, 19]
```

`max_by` and `sort_by` receive a function that says **what** to compare:
here, each student's grade.

## Dictionaries

A [dictionary](../debuter/glossaire.md#dictionary) (`dict`) links a value to
each **key**, just as a real dictionary links a definition to each word. Keys
can be of any kind: text, numbers, symbols... You build it from a list of
`[key, value]` pairs:

```rythme
ages = dict([["Bob", 25], ["Ada", 36]])
print(ages["Ada"], get(ages, "Eve"), len(ages))
older = put(ages, "Bob", 26)
print(older)
print(ages)
```

```sortie
36 None 2
dict("Ada" => 36, "Bob" => 26)
dict("Ada" => 36, "Bob" => 25)
```

`put` gives a **new** dictionary, and `ages` has not changed. A dictionary is
always kept in the order of its keys.

Keys are often **symbols**, names preceded by `:`. `each` gives the block the
key and the value of each pair:

```rythme
stock = dict([[:apple, 3], [:pear, 0]])
print(stock[:apple], has(stock, :kiwi), keys(stock))
stock.each do |fruit, n|
  print(fruit, n)
end
```

```sortie
3 false [:apple, :pear]
:apple 3
:pear 0
```

> **Note:** when the names are known in advance (`name`, `age`), use a
> record. When the keys come with the data (first names, words), use a
> dictionary.

## Sets

A **set** (`set`) keeps each value only once, like a stamp collection without
duplicates:

```rythme
seen = set([3, 1, 3, 2, 1])
print(seen, len(seen), contains(seen, 2))
```

```sortie
set(1, 2, 3) 3 true
```

## Which collection to choose

| Need | Choice | Example |
|------|--------|---------|
| values in an order | list | `[1, 2, 3]` |
| consecutive numbers | range | `1..10` |
| a "card" with fixed fields | record | `{name: "Ada", age: 36}` |
| finding a value by its key | dictionary | `dict([["Ada", 36]])` |
| values without duplicates | set | `set([1, 2])` |

## Common mistakes

### Reading past the last element

```rythme norun
fruits = ["apple", "pear", "fig"]
print(fruits[3])
```

```text
error[E007]: index 3 is out of range: the list has 3 element(s)
 --> main.ry:2:14
  |
2 | print(fruits[3])
  |              ^ no element here
  |
  = help: valid indices go from 0 to 2 (or -1 for the last element)
  = help: to get an Option instead of an error, use `get(x, 3)`
  = more about this error: `rythme explain E007`
```

Positions start at 0: with 3 elements, the last one is 2. Write `fruits[2]`,
`fruits[-1]`, or `get(fruits, 3)` if the element may be missing.

### Thinking that push changes the list

```rythme norun
fruits = ["apple", "pear"]
push(fruits, "kiwi")
print(fruits)
```

```text
warning[W008]: the result of `push` is lost
 --> main.ry:2:1
  |
2 | push(fruits, "kiwi")
  | ^^^^^^^^^^^^^^^^^^^^ this value is lost
  |
  = help: values never change: `push(list, x)` gives a new value, and changes nothing.
      To keep it, give it a name: `new = push(list, x)`
```

The program still prints `["apple", "pear"]`. Keep the new list:
`fruits2 = push(fruits, "kiwi")`.

### Changing a field with =

```rythme norun
ada = {name: "Ada", age: 36}
ada.age = 37
```

```text
error[E001]: cannot give a value to this
 --> main.ry:2:1
  |
2 | ada.age = 37
  | ^^^^^^^ this is not a name
  |
2 | ada.age = 37
  |         - `=` gives a name to a value
  |
  = help: records cannot be changed; build a new one: `new = {..old, age: value}`
  = more about this error: `rythme explain E001`
```

Build a new record: `ada2 = {..ada, age: 37}`.

### Reading a field that doesn't exist

```rythme norun
ada = {name: "Ada", age: 36}
print(ada.city)
```

```text
error[E010]: this record has no field `city`
 --> main.ry:2:11
  |
2 | print(ada.city)
  |           ^^^^ unknown field
  |
  = note: its fields are: `name`, `age`
  = help: to get an Option instead of an error, use `get(record, "city")`
  = more about this error: `rythme explain E010`
```

Check the spelling of the field, or use `get(ada, "city")` if the field may be
missing.

## Test yourself

**Question 1:** with `xs = [4, 8, 15, 16]`, what are `xs[1]`, `xs[-1]` and
`len(xs)`?

> **Answer**
>
> `8`, `16` and `4`. Position 1 is the second element, `-1` the last one.

**Question 2:** what does this program print?

```rythme norun
p = {x: 1, y: 2}
q = {..p, y: 5}
print(p.y, q.y)
```

> **Answer**
>
> `2 5`. `q` is a new record; `p` has not changed.

**Question 3:** you want to count how many times each first name appears.
Which collection should you choose?

> **Answer**
>
> A dictionary: each first name is a key, and its value is the number of
> times it appears.

## Summary

- A list is written `[1, 2, 3]`; `xs[0]` is the first element, `xs[-1]` the
  last one.
- `get` and `first` give an `Option`, with no risk of stopping the program.
- A record `{name: "Ada", age: 36}` groups fields, read with a dot:
  `ada.name`.
- A dictionary finds a value by its key; a set keeps each value only once.
- No collection ever changes: `push`, `put` and `{..r, field: value}` build a
  new value.

Next: [dictionaries and sets](dictionnaires.md)
