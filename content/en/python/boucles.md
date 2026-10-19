# Loops

This page explains how a Python program repeats actions with `while` and `for`, and goes through lists with `range`, `enumerate` and `zip`.

> **Goals:**
>
> - you will be able to repeat code with `while` and `for`;
> - you will be able to count with `range`;
> - you will be able to stop a loop with `break` or skip a round with `continue`;
> - you will be able to go through a list with `enumerate` and `zip`.

This chapter follows [conditions](conditions-boucles.md): you will meet `if` and comparisons again, this time inside loops.

## The while loop

Imagine you are asked to display the numbers from 1 to 100. Writing 100 `print` lines
would be long and tedious. A loop does this repetitive work for you.

A [loop](../debuter/glossaire.md#loop) repeats code. `while` repeats its block **as
long as** the condition is true.

```python
count = 3
while count > 0:
    print(count)
    count = count - 1
print("Liftoff!")
```

```sortie
3
2
1
Liftoff!
```

Line by line:

- `count = 3`: the variable is 3 at the start;
- `while count > 0:`: before each round, Python asks the question. `3 > 0` is true: it
  enters the block;
- the block displays `count`, then takes 1 away from it: it becomes 2, then 1, then 0;
- when `count` is 0, `0 > 0` is false: Python leaves the loop and moves on to the last
  `print`.

On each round, Python checks `count > 0`. When `count` is 0, the condition is false:
the loop stops.

> **Warning:** if the condition never becomes false, the loop runs forever. Make sure a
> variable changes on each round. To stop a stuck program, press Ctrl+C in the
> terminal.

**Your turn:** write a program that displays the numbers from 1 to 5 with a `while`
loop, then "Done!".

> **Solution**
>
> ```python
> n = 1
> while n <= 5:
>     print(n)
>     n = n + 1
> print("Done!")
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> Done!
> ```

## The for loop

Very often, you have a list of things to handle one by one: each item on a shopping
list, each student in a class. `for` is made for this.

`for` goes through the elements of a collection, one by one. It is the most used loop
in Python.

```python
fruits = ["apple", "banana", "cherry"]
for fruit in fruits:
    print(f"I like this {fruit}.")
```

```sortie
I like this apple.
I like this banana.
I like this cherry.
```

Line by line:

- `fruits` holds a list of three texts;
- `for fruit in fruits:` takes the first element, `"apple"`, and stores it in `fruit`;
- the block displays the sentence with `"apple"`. Then Python goes back to the `for`,
  which stores `"banana"` in `fruit`, and so on;
- after `"cherry"`, there are no elements left: the loop stops.

On each round, the variable `fruit` receives the next element of the list. Each pass is
called an [iteration](../debuter/glossaire.md#iteration).

`for` also goes through a text, letter by letter:

```python
for letter in "Yes":
    print(letter)
```

```sortie
Y
e
s
```

### Counting with range

`range` produces a sequence of numbers. It is perfect for repeating an action a precise
number of times.

- `range(5)`: from 0 to 4 (5 numbers, **without** the 5);
- `range(1, 6)`: from 1 to 5;
- `range(0, 10, 2)`: from 0 to 8, in steps of 2.

```python
for i in range(3):
    print("Round", i)

print(list(range(1, 6)))
print(list(range(0, 10, 2)))
print(list(range(10, 0, -3)))
```

```sortie
Round 0
Round 1
Round 2
[1, 2, 3, 4, 5]
[0, 2, 4, 6, 8]
[10, 7, 4, 1]
```

`list(...)` turns the `range` into a list, so you can see all its numbers at once. A
negative step (`-3`) counts down.

> **Remember:** the end of a `range` is always **excluded**. `range(1, 6)` stops at 5.

**Your turn:** write a program that displays the 7 times table, from "7 x 1 = 7" to
"7 x 5 = 35", one line per calculation.

> **Solution**
>
> ```python
> for i in range(1, 6):
>     print("7 x", i, "=", 7 * i)
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

## break and continue

Sometimes you want to change the course of a loop while it runs: stop searching as
soon as you have found something, or ignore some elements. That is what these two words
do.

- `break` leaves the loop immediately;
- `continue` skips the rest of the round and moves on to the next one.

```python
for n in range(1, 10):
    if n % 2 == 0:
        continue
    if n > 7:
        break
    print(n)
```

```sortie
1
3
5
7
```

Line by line:

- `for n in range(1, 10):` gives `n` the values 1, 2, 3… up to 9;
- if `n` is even (the remainder of dividing by 2 is 0), `continue` jumps straight to
  the next round: the `print` is not reached;
- if `n` goes past 7, `break` stops the whole loop: 9 is never displayed;
- otherwise, `print(n)` displays the number.

Even numbers are skipped (`continue`). When `n` goes past 7, the loop stops (`break`).

### else on a loop

A loop can have an `else`. It runs if the loop goes **all the way to the end**, without
`break`. It is useful for a search: "I found nothing".

```python
numbers = [3, 7, 11, 15]
for n in numbers:
    if n % 2 == 0:
        print(f"First even number: {n}")
        break
else:
    print("No even number.")
```

```sortie
No even number.
```

## enumerate: the number and the element

Sometimes you want to number what you go through, like a ranking or the steps of a
recipe.

To get both the position and the element, use `enumerate`. On each round, it gives two
values: the number (the [index](../debuter/glossaire.md#index)) and the element.

```python
podium = ["Alice", "Bob", "Chloe"]
for rank, name in enumerate(podium, start=1):
    print(f"{rank}. {name}")
```

```sortie
1. Alice
2. Bob
3. Chloe
```

Line by line:

- `podium` holds three first names, in order of arrival;
- on each round, `enumerate` gives a pair: first `1` and `"Alice"`, then `2` and
  `"Bob"`, then `3` and `"Chloe"`;
- `rank, name` stores the number in `rank` and the first name in `name`;
- `start=1` makes the numbering start at 1.

Without `start=1`, the numbering starts at 0.

## zip: going through two lists together

Sometimes two lists go together: first names on one side, their grades on the other, in
the same order.

`zip` moves through several lists at the same time, like a zipper joining two sides,
tooth by tooth.

```python
first_names = ["Alice", "Bob", "Chloe"]
grades = [15, 12, 18]
for first_name, grade in zip(first_names, grades):
    print(f"{first_name}: {grade}/20")
```

```sortie
Alice: 15/20
Bob: 12/20
Chloe: 18/20
```

Line by line:

- `first_names` and `grades` are in the same order: the first grade is Alice's;
- `zip` takes the first element of each list, then the second, then the third;
- on each round, `first_name` receives a first name and `grade` the grade that goes
  with it.

> **Note:** `zip` stops at the end of the shortest list.

## Common mistakes

### Looping over a number with for

```python norun
for i in 5:
    print(i)
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 1, in <module>
    for i in 5:
             ^
TypeError: 'int' object is not iterable
```

You cannot go through a number (*not iterable*). To repeat 5 times, write
`for i in range(5):`.

### Forgetting enumerate

```python norun
podium = ["Alice", "Bob", "Chloe"]
for rank, name in podium:
    print(f"{rank}. {name}")
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 2, in <module>
    for rank, name in podium:
        ^^^^^^^^^^
ValueError: too many values to unpack (expected 2)
```

Without `enumerate`, each round gives a single element, the text `"Alice"`. Python
tries to split it into two variables and finds too many letters (*too many values*).
Write `for rank, name in enumerate(podium, start=1):`.

### break outside the loop

```python norun
for n in [3, 8, 11]:
    if n % 2 == 0:
        print("Even number:", n)
break
```

```text
  File "/home/you/main.py", line 4
    break
    ^^^^^
SyntaxError: 'break' outside loop
```

`break` is not indented: it is no longer in the `for` block (*outside loop*). Put it
under the `print`, with the same indentation, so that it belongs to the `if` block.

## Test yourself

**Question 1:** how many times is "hi" displayed with `for i in range(2, 8):`?

> **Answer**
>
> 6 times: `i` takes the values 2, 3, 4, 5, 6 and 7. The end (8) is excluded.

**Question 2:** write a loop that displays the multiples of 3 from 3 to 15, on one
line.

> **Answer**
>
> ```python
> for n in range(3, 16, 3):
>     print(n, end=" ")
> print("end")
> ```
>
> ```sortie
> 3 6 9 12 15 end
> ```

**Question 3:** what does this code display?

```python norun
n = 1
while n < 100:
    n = n * 2
    if n > 10:
        break
print(n)
```

> **Answer**
>
> `16`. `n` becomes 2, then 4, then 8, then 16. At 16, `n > 10` is true: `break` stops
> the loop before the `while` condition becomes false.

## Summary

- `while` repeats as long as a condition is true; a variable must change on each
  round.
- `for` goes through a collection or a `range`.
- The end of a `range` is excluded: `range(1, 6)` goes from 1 to 5.
- `break` leaves, `continue` skips a round.
- `enumerate` and `zip` make loops simpler.

Next: [functions](fonctions.md).
