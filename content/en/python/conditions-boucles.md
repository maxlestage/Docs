# Conditions and loops

This page explains how a Python program makes decisions with `if` and `match`, and repeats actions with `while` and `for`.

> **Goals:**
>
> - you will be able to write a condition with `if`, `elif` and `else`;
> - you will be able to combine tests with `and`, `or` and `not`;
> - you will be able to choose between several cases with `match`;
> - you will be able to repeat code with `while` and `for`;
> - you will be able to go through a list with `enumerate` and `zip`.

## The if condition

A [condition](../debuter/glossaire.md#condition) lets you run code only if something is
true. It is like a road sign: "if the road is wet, slow down".

```python
temperature = 30
if temperature > 25:
    print("It is hot.")
    print("Remember to drink.")
print("Have a nice day!")
```

```sortie
It is hot.
Remember to drink.
Have a nice day!
```

Line by line:

- `temperature = 30` stores 30 in the variable `temperature`;
- `if temperature > 25:` asks a question: "is 30 greater than 25?". The answer is yes;
- so the two indented `print` lines run, one after the other;
- the last `print` is not indented: it runs in every case, even if it were cold.

Three rules to remember:

- the `if` line ends with a **colon** `:`;
- the lines that depend on the `if` are **indented** (shifted by 4 spaces): they form a
  **block**;
- the first line that is not shifted (`print("Have a nice day!")`) is outside the
  block: it always runs.

> **Remember:** in Python, indentation is not decoration. It is what says which lines
> are part of the block. Always use 4 spaces.

## else and elif

Often there is more than one case. A grade can be very good, average or bad, and you
want a different message for each, like on a school report.

`else` gives the path to follow when the condition is false. `elif` (short for
*else if*) adds more tests.

```python
grade = 13
if grade >= 16:
    print("Very good")
elif grade >= 12:
    print("Good")
elif grade >= 10:
    print("Pass")
else:
    print("Fail")
```

```sortie
Good
```

Python tests the conditions **in order** and stops at the first true one. Here,
`grade >= 16` is false, `grade >= 12` is true: it displays "Good" and skips the rest.

**Your turn:** write a program that stores `hour = 15` in a variable, then displays
"Good morning" if the hour is before 12, "Good afternoon" if it is before 18, and
"Good evening" otherwise.

> **Solution**
>
> ```python
> hour = 15
> if hour < 12:
>     print("Good morning")
> elif hour < 18:
>     print("Good afternoon")
> else:
>     print("Good evening")
> ```
>
> ```sortie
> Good afternoon
> ```

## Comparing values

To decide, a program first asks a question: "is the age 18?", "is the temperature
above 25?". Comparison operators are how you write these questions.

| Operator | Meaning |
| --- | --- |
| `==` | equal to |
| `!=` | not equal to |
| `<`, `>` | less than, greater than |
| `<=`, `>=` | less than or equal, greater than or equal |

A comparison gives a [boolean](../debuter/glossaire.md#boolean): `True` or `False`.

```python
age = 17
print(age == 18)
print(age != 18)
print(age < 18)
print("cat" == "cat")
print(10 <= age < 20)
```

```sortie
False
True
True
True
True
```

The last line shows a Python convenience: `10 <= age < 20` means "`age` is between 10
(included) and 20 (not included)", like in math.

> **Warning:** `=` stores a value in a variable. `==` compares two values. Do not mix
> them up.

## Combining: and, or, not

A decision often depends on several things at once. To get into the cinema, you need a
ticket **and** to be old enough. These three words join several tests into one.

- `a and b` is true if `a` **and** `b` are true;
- `a or b` is true if **at least one** of the two is true;
- `not a` flips it: true becomes false, and false becomes true.

```python
age = 25
has_ticket = True
is_vip = False

if age >= 18 and has_ticket:
    print("Entry allowed")
if has_ticket or is_vip:
    print("Access to the hall")
if not is_vip:
    print("No access to the VIP lounge")
```

```sortie
Entry allowed
Access to the hall
No access to the VIP lounge
```

Line by line:

- the first three lines prepare the data: an age and two booleans;
- first `if`: `age >= 18` is true **and** `has_ticket` is true, so entry is allowed;
- second `if`: `has_ticket` is true, and with `or` that is enough;
- third `if`: `is_vip` is `False`, and `not` turns it into `True`.

**Your turn:** write a program that stores `age = 70` in a variable, then displays
"Reduced price" if the age is under 18 or 65 and over, and "Full price"
otherwise.

> **Solution**
>
> ```python
> age = 70
> if age < 18 or age >= 65:
>     print("Reduced price")
> else:
>     print("Full price")
> ```
>
> ```sortie
> Reduced price
> ```

## True or false: "truthiness"

> **Note:** this part can wait. Read it once the basics feel solid.

In an `if`, Python accepts any value, not just `True` or `False`. Some values count as
**false**:

- `False` and `None`;
- zero: `0` and `0.0`;
- anything empty: `""`, `[]`, `{}`.

Everything else counts as **true**.

```python
cart = []
if cart:
    print("You have items.")
else:
    print("Your cart is empty.")

name = "Alice"
if name:
    print(f"Hello, {name}!")
```

```sortie
Your cart is empty.
Hello, Alice!
```

`if cart:` reads "if the cart contains something". It is shorter than
`if len(cart) > 0:`.

## The conditional expression

> **Note:** this part can wait. Read it once the basics feel solid.

To choose between two values on a single line, write
`value_if_true if condition else value_if_false`.

```python
age = 15
status = "adult" if age >= 18 else "minor"
print(status)

n = 7
print("even" if n % 2 == 0 else "odd")
```

```sortie
minor
odd
```

> **Tip:** keep this form for simple choices. As soon as it gets longer, a real `if`
> over several lines is more readable.

## Choosing among several cases: match

Since Python 3.10, `match` compares a value against several **patterns**. It is like a
railway switch: the value goes down the first track that matches.

```python
def describe(command):
    match command:
        case "north" | "south":
            return "vertical"
        case "east" | "west":
            return "horizontal"
        case "quit":
            return "goodbye"
        case _:
            return "unknown command"

print(describe("north"))
print(describe("west"))
print(describe("jump"))
```

```sortie
vertical
horizontal
unknown command
```

- `|` means "or": `"north" | "south"` accepts either one;
- `case _:` catches everything else: it is the default case.

`match` can also take a value apart. Here, it recognizes a list of two elements and
stores each piece in a variable:

```python
for action in [["go", "north"], ["take", "key"], ["sleep"]]:
    match action:
        case ["go", direction]:
            print(f"You head {direction}.")
        case ["take", item]:
            print(f"You take: {item}.")
        case _:
            print("I do not understand.")
```

```sortie
You head north.
You take: key.
I do not understand.
```

Functions (`def`) are explained in [functions](fonctions.md).

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

### Forgetting the colon

```python norun
age = 20
if age >= 18
    print("adult")
```

```text
  File "/home/you/main.py", line 2
    if age >= 18
                ^
SyntaxError: expected ':'
```

Python expected `:` (*expected ':'*). Every `if`, `elif`, `else`, `while`, `for` and
`match` line ends with a colon.

### Writing = instead of ==

```python norun
age = 20
if age = 18:
    print("exactly")
```

```text
  File "/home/you/main.py", line 2
    if age = 18:
       ^^^^^^^^
SyntaxError: invalid syntax. Maybe you meant '==' or ':=' instead of '='?
```

`=` stores a value; to compare, you need `==`. Python guesses it and suggests it to
you. Write `if age == 18:`.

### Forgetting the indentation

```python norun
age = 20
if age >= 18:
print("adult")
```

```text
  File "/home/you/main.py", line 3
    print("adult")
    ^^^^^
IndentationError: expected an indented block after 'if' statement on line 2
```

After `if ... :`, Python expects an indented block. Add 4 spaces before
`print("adult")`.

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

## Test yourself

**Question 1:** what does this code display?

```python norun
x = 0
if x:
    print("A")
elif x == 0:
    print("B")
else:
    print("C")
```

> **Answer**
>
> `B`. `0` counts as false, so the first test fails. `x == 0` is true: Python displays
> `B` and ignores the `else`.

**Question 2:** how many times is "hi" displayed with `for i in range(2, 8):`?

> **Answer**
>
> 6 times: `i` takes the values 2, 3, 4, 5, 6 and 7. The end (8) is excluded.

**Question 3:** write a loop that displays the multiples of 3 from 3 to 15, on one
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

## Summary

- `if`, `elif` and `else` choose a path; each line ends with `:` and the block is
  indented by 4 spaces.
- `==` compares, `=` stores; `and`, `or` and `not` combine tests.
- `0`, `None` and empty collections count as false.
- `match` chooses among several cases; `case _:` catches the rest.
- `while` repeats as long as a condition is true; `for` goes through a collection or a
  `range`.
- `break` leaves, `continue` skips a round; `enumerate` and `zip` make loops simpler.

Next: [functions](fonctions.md).
