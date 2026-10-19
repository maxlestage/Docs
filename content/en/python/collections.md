# Lists and tuples

This page explains how to keep several values together in Python, in a list you can change or in a fixed tuple.

> **Goals:**
>
> - you will be able to create a list, read its items and take a slice of it;
> - you will be able to add, insert, remove and sort items;
> - you will be able to avoid the trap of two names for the same list;
> - you will be able to use a tuple and unpack it into variables;
> - you will be able to turn a `range` into a list.

## Creating a list

A [list](../debuter/glossaire.md#list) keeps several values in a precise order.
Think of a shopping list: a sheet of paper with numbered lines. You write it between
square brackets `[ ]`, with commas between the items.

```python
shopping = ["bread", "milk", "eggs"]
grades = [12, 15, 9, 18]
empty = []
mixed = ["Alice", 30, True]

print(shopping)
print(len(grades))
print(empty, len(empty))
print(mixed)
```

```sortie
['bread', 'milk', 'eggs']
4
[] 0
['Alice', 30, True]
```

Line by line:

- `shopping` gets a list of three texts, kept in this order;
- `empty = []` creates a list with nothing in it, ready to be filled later;
- `print(shopping)` shows the whole list, with its brackets and quotes;
- `len(grades)` counts the items in `grades`: there are 4.

A list can hold values of different types. In practice, you often keep values of the
same kind in it: grades, names, prices.

## Reading an item

A list is no use if you cannot get back what is in it. Just as you read "the third
line" of a shopping list, you ask Python for the item stored at a given position.

As with text, each item has an [index](../debuter/glossaire.md#index), which starts at
**0**. Negative indexes count from the end.

```python
colors = ["red", "green", "blue", "yellow"]
print(colors[0])
print(colors[2])
print(colors[-1])
```

```sortie
red
blue
yellow
```

Line by line:

- `colors[0]` gives the first item, `"red"`: counting starts at 0;
- so `colors[2]` gives the third one, `"blue"`;
- `colors[-1]` gives the last one, `"yellow"`, whatever the length of the list.

Unlike text, a list is [mutable](../debuter/glossaire.md#mutable): you can change an
item.

```python
colors = ["red", "green", "blue"]
colors[1] = "purple"
print(colors)
```

```sortie
['red', 'purple', 'blue']
```

## Slices

Sometimes you do not want a single item, but a piece of the list: the first three, the
last two. It is like cutting a slice of bread without touching the rest of the loaf.

A slice `list[start:end]` returns a **new** list with part of the items. The end is
excluded, exactly as with [text](texte.md).

```python
numbers = [10, 20, 30, 40, 50, 60]
print(numbers[1:4])
print(numbers[:3])
print(numbers[-2:])
print(numbers[::2])
print(numbers[::-1])
```

```sortie
[20, 30, 40]
[10, 20, 30]
[50, 60]
[10, 30, 50]
[60, 50, 40, 30, 20, 10]
```

Line by line:

- `numbers[1:4]` takes indexes 1, 2 and 3: the end, 4, is excluded;
- `numbers[:3]` starts at the beginning, and `numbers[-2:]` goes to the end;
- `numbers[::2]` takes every other item;
- `numbers[::-1]` goes through the list backwards.

**Your turn:** write a program that stores the days of the week in a list `days`, then
prints the first day, the last one, and the slice of the first five.

> **Solution**
>
> ```python
> days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
> print(days[0])
> print(days[-1])
> print(days[:5])
> ```
>
> ```sortie
> Mon
> Sun
> ['Mon', 'Tue', 'Wed', 'Thu', 'Fri']
> ```

## Adding and removing items

A list often changes while a program runs: you add an item to the shopping list, you
cross out the one you just bought. These methods do that job.

| Method | Effect |
| --- | --- |
| `append(x)` | adds `x` at the end |
| `insert(i, x)` | inserts `x` at index `i` |
| `extend(other)` | adds all the items of another list |
| `remove(x)` | removes the first `x` found |
| `pop()` | removes and returns the last item |
| `pop(i)` | removes and returns the item at index `i` |

```python
queue = ["Alice", "Bob"]
queue.append("Chloe")
print(queue)
queue.insert(0, "Zoe")
print(queue)
queue.remove("Bob")
print(queue)
last = queue.pop()
print(last, queue)
first = queue.pop(0)
print(first, queue)
queue.extend(["David", "Emma"])
print(queue)
```

```sortie
['Alice', 'Bob', 'Chloe']
['Zoe', 'Alice', 'Bob', 'Chloe']
['Zoe', 'Alice', 'Chloe']
Chloe ['Zoe', 'Alice']
Zoe ['Alice']
['Alice', 'David', 'Emma']
```

Line by line:

- `queue.append("Chloe")` adds Chloe at the end: the queue now holds three names;
- `queue.insert(0, "Zoe")` puts Zoe at index 0, at the very front; the others move back;
- `queue.remove("Bob")` looks for Bob and removes him;
- `queue.pop()` removes the last name, Chloe, and stores it in `last`;
- `queue.extend(["David", "Emma"])` adds two names at once, at the end.

These methods change the list **in place**: they change the list itself.

**Your turn:** write a program that starts from `shopping = ["bread", "milk"]`, adds
"butter" at the end, removes "bread", then prints the list.

> **Solution**
>
> ```python
> shopping = ["bread", "milk"]
> shopping.append("butter")
> shopping.remove("bread")
> print(shopping)
> ```
>
> ```sortie
> ['milk', 'butter']
> ```

## Searching and measuring

Once the list is filled, you want to ask it questions: "is milk on the list?", "how
many grades are there?", "which one is the best?".

```python
grades = [12, 15, 9, 18, 15]
print(15 in grades)
print(20 in grades)
print(grades.index(18))
print(grades.count(15))
print(len(grades), sum(grades), min(grades), max(grades))
print(sum(grades) / len(grades))
```

```sortie
True
False
3
2
5 69 9 18
13.8
```

- `in` checks whether a value is present;
- `index` gives the position of the first occurrence;
- `count` counts the occurrences;
- `len`, `sum`, `min` and `max` give the size, the sum, the smallest and the largest.
  The average is `sum / len`.

## Sorting: sort and sorted

Sorting means putting things in order: grades from lowest to highest, names in
alphabetical order, like in a phone book.

There are two ways to sort:

- `list.sort()` sorts the list **in place** and returns nothing (`None`);
- `sorted(list)` returns a **new** sorted list and leaves the original untouched.

```python
grades = [12, 15, 9, 18]
in_order = sorted(grades)
print(in_order)
print(grades)

grades.sort()
print(grades)

grades.sort(reverse=True)
print(grades)
```

```sortie
[9, 12, 15, 18]
[12, 15, 9, 18]
[9, 12, 15, 18]
[18, 15, 12, 9]
```

Line by line:

- `sorted(grades)` builds a new sorted list, stored in `in_order`;
- `print(grades)` shows that `grades` has not moved;
- `grades.sort()` this time sorts `grades` itself;
- `grades.sort(reverse=True)` sorts it again, the other way round.

`reverse=True` sorts from largest to smallest.

### Sorting by a criterion with key

`key` takes a function. Python applies it to each item and sorts by the result. It is
like saying "line the students up **by height**" instead of "by name".

```python
words = ["kiwi", "Pineapple", "strawberry", "apple"]
print(sorted(words))
print(sorted(words, key=str.lower))
print(sorted(words, key=len))

students = [("Alice", 15), ("Bob", 12), ("Chloe", 18)]
print(sorted(students, key=lambda s: s[1], reverse=True))
```

```sortie
['Pineapple', 'apple', 'kiwi', 'strawberry']
['apple', 'kiwi', 'Pineapple', 'strawberry']
['kiwi', 'apple', 'Pineapple', 'strawberry']
[('Chloe', 18), ('Alice', 15), ('Bob', 12)]
```

`lambda` functions are explained in [functions](fonctions.md).

> **Warning:** by default, uppercase letters come before lowercase ones (`"Z"` before
> `"a"`). `key=str.lower` sorts without caring about case.

### Reversing the order

> **Note:** this part can wait. Read it once the basics feel solid.

`reverse()` flips the list in place. `reversed()` or `[::-1]` give a backwards version
without touching the original.

```python
steps = [1, 2, 3]
print(steps[::-1])
print(list(reversed(steps)))
steps.reverse()
print(steps)
```

```sortie
[3, 2, 1]
[3, 2, 1]
[3, 2, 1]
```

## Looping over a list

You often want to do the same thing with each item: add up all the prices on a
receipt, print each name in a class.

The `for` loop goes through the items one by one. It is explained in
[conditions and loops](conditions-boucles.md).

```python
prices = [4.5, 12.0, 3.25]
total = 0
for p in prices:
    total += p
print(f"Total: {total:.2f} €")
```

```sortie
Total: 19.75 €
```

Line by line:

- `total = 0` prepares a variable that will build up the sum;
- `for p in prices:` gives `p` the value 4.5, then 12.0, then 3.25;
- on each pass, `total += p` adds the price to `total`: 4.5, then 16.5, then 19.75;
- after the loop, `print` shows the total with two digits after the decimal point.

**Your turn:** write a program that loops over `grades = [12, 15, 9]` and prints each
grade on its own line, preceded by "Grade:".

> **Solution**
>
> ```python
> grades = [12, 15, 9]
> for g in grades:
>     print("Grade:", g)
> ```
>
> ```sortie
> Grade: 12
> Grade: 15
> Grade: 9
> ```

## Nested lists

> **Note:** this part can wait. Read it once the basics feel solid.

A list can hold other lists. This is handy for a two-dimensional table: a grid, a game
board.

```python
grid = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
]
print(grid[1])
print(grid[1][2])
for row in grid:
    print(row[0], row[1], row[2])
```

```sortie
[4, 5, 6]
6
1 2 3
4 5 6
7 8 9
```

`grid[1][2]` reads in two steps: the row at index 1 (`[4, 5, 6]`), then the item at
index 2 of that row (`6`).

## Copy or share? The classic trap

Sometimes you want to keep a list untouched and work on a duplicate, the way you
photocopy a document before writing on it. You still need to really have two lists.

Writing `b = a` does **not** copy the list. It sticks a second label on the **same**
list. If you change it through `b`, `a` changes too: this is an **alias**.

```python
a = [1, 2, 3]
b = a
b.append(4)
print(a)
print(b)
print(a is b)
```

```sortie
[1, 2, 3, 4]
[1, 2, 3, 4]
True
```

Line by line:

- `a = [1, 2, 3]` creates a list and gives it the name `a`;
- `b = a` creates no list at all: `b` becomes a second name for the same one;
- `b.append(4)` adds 4 to that single list;
- so both `print` calls show the same list, with the 4; `a is b` is `True`.

`is` checks whether two names refer to the same object. Here, yes: there is only one
list.

For a real copy, use `list(a)`, `a.copy()` or `a[:]`:

```python
a = [1, 2, 3]
b = a.copy()
b.append(4)
print(a)
print(b)
print(a is b)
```

```sortie
[1, 2, 3]
[1, 2, 3, 4]
False
```

> **Warning:** `copy()` makes a **shallow** copy. For a list of lists, the inner lists
> are still shared. In that case, use `copy.deepcopy` (from the `copy` module).

```python
import copy

grid = [[0, 0], [0, 0]]
shallow = grid.copy()
deep = copy.deepcopy(grid)
grid[0][0] = 9
print(shallow)
print(deep)
```

```sortie
[[9, 0], [0, 0]]
[[0, 0], [0, 0]]
```

## Tuples

A [tuple](../debuter/glossaire.md#tuple) looks like a list, but it is **fixed**: you
cannot add, remove or change an item. You write it with parentheses.

```python
point = (3, 4)
color = (255, 128, 0)
single = (42,)

print(point[0], point[1])
print(len(color))
print(type(single))
```

```sortie
3 4
3
<class 'tuple'>
```

Line by line:

- `point = (3, 4)` creates a tuple of two numbers;
- `single = (42,)` creates a tuple with a single item, thanks to the comma;
- `point[0]` and `point[1]` are read just like in a list;
- `type(single)` confirms that it really is a tuple.

A tuple with a single item needs a comma: `(42,)`. Without it, `(42)` is just the
number 42 in parentheses.

You use a tuple for a group of values that belong together and do not change:
coordinates, a date, an RGB color.

### Unpacking a tuple

**Unpacking** puts each item in its own variable, in a single line.

```python
point = (3, 4)
x, y = point
print(f"x = {x}, y = {y}")

a, b = 1, 2
a, b = b, a
print(a, b)

first, *rest = [10, 20, 30, 40]
print(first, rest)
```

```sortie
x = 3, y = 4
2 1
10 [20, 30, 40]
```

- `a, b = b, a` swaps two variables, without a temporary variable;
- `*rest` collects all the remaining items in a list.

**Your turn:** write a program that unpacks the tuple `date = (14, 7, 1789)` into three
variables `day`, `month` and `year`, then prints "Year: 1789".

> **Solution**
>
> ```python
> date = (14, 7, 1789)
> day, month, year = date
> print("Year:", year)
> ```
>
> ```sortie
> Year: 1789
> ```

## range and lists

Sometimes you need a list of numbers in a row: from 1 to 10, or all the even numbers up
to 20. Typing them by hand would be slow, and you might forget one.

`range` produces a sequence of numbers on demand, without storing it. To get a real
list, pass it to `list()`.

```python
print(list(range(5)))
print(list(range(2, 11, 2)))
print(sum(range(1, 101)))
```

```sortie
[0, 1, 2, 3, 4]
[2, 4, 6, 8, 10]
5050
```

Line by line:

- `range(5)` gives 0, 1, 2, 3, 4: it starts at 0 and stops before 5;
- `range(2, 11, 2)` starts at 2, goes up by 2 and stops before 11;
- `list(...)` turns these numbers into a real list, which `print` can show.

`sum(range(1, 101))` adds up the numbers from 1 to 100 without creating a list.

## Common mistakes

### An index that is too large

```python norun
fruits = ["apple", "pear"]
print(fruits[2])
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 2, in <module>
    print(fruits[2])
          ~~~~~~^^^
IndexError: list index out of range
```

The list has 2 items, at indexes 0 and 1. The last one is `fruits[-1]`.

### Storing the result of sort

```python norun
grades = [12, 8, 15]
in_order = grades.sort()
print(in_order)
```

```text
None
```

`sort()` sorts in place and returns `None`. Write `in_order = sorted(grades)`, or call
`grades.sort()` and then use `grades`.

### Removing a missing item

```python norun
fruits = ["apple", "pear"]
fruits.remove("kiwi")
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 2, in <module>
    fruits.remove("kiwi")
    ~~~~~~~~~~~~~^^^^^^^^
ValueError: list.remove(x): x not in list
```

`"kiwi"` is not in the list. Check first: `if "kiwi" in fruits:`.

### Changing a tuple

```python norun
point = (3, 4)
point[0] = 10
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 2, in <module>
    point[0] = 10
    ~~~~~^^^
TypeError: 'tuple' object does not support item assignment
```

A tuple is fixed. Create a new one (`point = (10, point[1])`), or use a list if the
values need to change.

## Test yourself

**Question 1:** what does this code print?

```python norun
a = ["x"]
b = a
b.append("y")
print(len(a))
```

> **Answer**
>
> `2`. `a` and `b` refer to the same list: adding through `b` also changes `a`.

**Question 2:** what is the difference between `grades.sort()` and `sorted(grades)`?

> **Answer**
>
> `grades.sort()` changes the list and returns `None`. `sorted(grades)` returns a new
> sorted list and leaves `grades` untouched.

**Question 3:** print the three best grades of `[12, 17, 9, 15, 19, 11]`.

> **Answer**
>
> ```python
> grades = [12, 17, 9, 15, 19, 11]
> print(sorted(grades, reverse=True)[:3])
> ```
>
> ```sortie
> [19, 17, 15]
> ```

## Summary

- A list `[...]` keeps values in order; you read it with `list[i]` and `list[a:b]`.
- `append`, `insert`, `remove` and `pop` change it in place.
- `sorted` returns a new sorted list; `sort` sorts in place; `key` picks the criterion.
- `b = a` shares the list; `a.copy()` copies it.
- A tuple `(...)` is a fixed list; `x, y = point` unpacks it.
- `list(range(...))` turns a sequence of numbers into a list.

Next: [dictionaries and sets](dictionnaires.md). Compare with
[collections in Ruby](../ruby/collections.md).
