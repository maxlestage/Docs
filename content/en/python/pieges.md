# Beginner traps in Python

This page gathers the twelve traps that most often block beginners in Python, with, for each one, what happens, why, and how to avoid it.

> **Goals:**
>
> - you will be able to recognize the twelve most common Python traps;
> - you will understand why Python behaves this way;
> - you will be able to fix each trap in a line or two.

## How to read this page

Each trap follows the same path:

1. what you **think** will happen;
2. the code, and what **really** happens;
3. **why**;
4. **the fix**.

Do not read everything at once. Keep this page at hand. When a program surprises you,
come back here and look for the trap that looks like your problem.

> **Remember:** a trap is not your fault. Everyone falls into them at the beginning.
> What matters is recognizing it the second time.

## Syntax traps

[Syntax](../debuter/glossaire.md#syntax) is the set of writing rules of the language,
like grammar for English.

### Writing = instead of ==

**You think:** "I am testing whether the age is 18".

```python norun
age = 18
if age = 18:
    print("adult")
```

```text
  File "main.py", line 2
    if age = 18:
       ^^^^^^^^
SyntaxError: invalid syntax. Maybe you meant '==' or ':=' instead of '='?
```

**Why:** in Python, the two signs do not mean the same thing.

| Sign | Read as | Role |
|---|---|---|
| `=` | "gets" | stores a value in a variable |
| `==` | "is equal to?" | asks a question, answers `True` or `False` |

Think of a box with a label. `age = 18` puts 18 **into** the box. `age == 18`
**looks** into the box and answers yes or no.

**The fix:**

```python
age = 18
if age == 18:
    print("adult")
```

```sortie
adult
```

Python helps you: the message says `Maybe you meant '=='`.

### Forgetting the indentation

**You think:** "the `print` is part of the `if`".

```python norun
age = 18
if age >= 18:
print("adult")
```

```text
  File "main.py", line 3
    print("adult")
    ^^^^^
IndentationError: expected an indented block after 'if' statement on line 2
```

**Why:** in Python, [indentation](../debuter/glossaire.md#indentation) (the spaces at
the start of the line) is not decoration. It is what tells which lines are **inside**
the `if`.

```text
if age >= 18:          ← the question
    print("adult")     ← 4 spaces: this line is INSIDE the if
print("end")           ← 0 spaces: this line is AFTER the if
```

**The fix:** add 4 spaces in front of each line of the block.

```python
age = 18
if age >= 18:
    print("adult")
print("end")
```

```sortie
adult
end
```

The opposite trap also exists: too many spaces, for no reason. Python then answers
`IndentationError: unexpected indent`.

> **Warning:** never mix tabs and spaces. Set up your editor so that it inserts 4
> spaces when you press Tab.

## Type traps

### Adding text and a number

**You think:** `"12" + 1` gives `13`.

```python norun
age = "12"
print(age + 1)
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(age + 1)
          ~~~~^~~
TypeError: can only concatenate str (not "int") to str
```

**Why:** `"12"` has quotes. It is **text** (a
[string](../debuter/glossaire.md#string)), not a number. For text, `+`
means "stick end to end". Python does not know how to stick a number to text, and it
refuses to guess.

The same `+` gives very different results depending on the type:

```python
print(12 + 1)
print("12" + "1")
print("12" * 3)
```

```sortie
13
121
121212
```

**The fix:** turn the text into a number with `int()`.

```python
age = "12"
print(int(age) + 1)
```

```sortie
13
```

This trap often comes with `input()`: what the user types is **always** text. Write
`int(input("Your age: "))`.

> **Try it:** replace `int(age)` with `str(1)` and `+ 1` with `+ str(1)`. You get
> `121`: this time, everything is text.

### Division that gives a decimal number

**You think:** `10 / 2` gives `5`.

```python
print(10 / 2)
print(7 / 2)
print(7 // 2)
print(7 % 2)
```

```sortie
5.0
3.5
3
1
```

**Why:** in Python, `/` **always** gives a decimal number (a
[float](../debuter/glossaire.md#float)), even when the division is exact.

| Operator | Name | `7 ? 2` |
|---|---|---|
| `/` | division | `3.5` |
| `//` | integer division | `3` |
| `%` | remainder | `1` |

**The fix:** use `//` when you want a whole number, for example to share 7 candies
between 2 children.

### 0.1 + 0.2 is not 0.3

**You think:** `0.1 + 0.2 == 0.3` is true.

```python
print(0.1 + 0.2)
print(0.1 + 0.2 == 0.3)
```

```sortie
0.30000000000000004
False
```

**Why:** the computer stores decimal numbers in binary. `0.1` does not come out exact in
binary, just as `1/3` does not come out exact in decimal (`0.3333…`). A tiny error
remains.

**The fix:** round before comparing.

```python
print(round(0.1 + 0.2, 2) == 0.3)
```

```sortie
True
```

The chapter [numbers](nombres.md) also shows `math.isclose` and `Decimal`.

> **Remember:** always look at the **type** of a value. When in doubt, display
> `type(value)`.

## List traps

### Counting from 0, and range that stops before

**You think:** a list of 3 elements has an element number 3.

```python norun
numbers = [10, 20, 30]
print(numbers[3])
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(numbers[3])
          ~~~~~~~^^^
IndexError: list index out of range
```

**Why:** in Python, positions are counted from **0**. The number of a position is
called an [index](../debuter/glossaire.md#index).

```text
numbers = [ 10 , 20 , 30 ]
index :      0    1    2      ← there is no position 3
```

`range` follows the same logic: `range(1, 5)` stops **before** 5.

```python
print(list(range(1, 5)))
print(list(range(5)))
```

```sortie
[1, 2, 3, 4]
[0, 1, 2, 3, 4]
```

**The fix:** the last element is at index `len(my_list) - 1`, or more simply
`my_list[-1]`. To count from 1 to 5 included, write `range(1, 6)`.

```python
numbers = [10, 20, 30]
print(numbers[2])
print(numbers[-1])
```

```sortie
30
30
```

### Two names for the same list

**You think:** `copy = grades` makes a copy.

```python
grades = [12, 15]
copy = grades
copy.append(8)
print(grades)
```

```sortie
[12, 15, 8]
```

**Why:** `copy = grades` copies nothing. It sticks a **second label** on the **same**
list.

```text
grades ──┐
         ├──► [12, 15, 8]     a single list, two labels
copy ────┘
```

A list is [mutable](../debuter/glossaire.md#mutable): it can be changed. So what you do
through one label shows through the other.

**The fix:** ask for a real copy with `.copy()`.

```python
grades = [12, 15]
copy = grades.copy()
copy.append(8)
print(grades)
print(copy)
```

```sortie
[12, 15]
[12, 15, 8]
```

> **Try it:** replace `.copy()` with `[:]` (`copy = grades[:]`). The result is the
> same: `[:]` takes a slice that contains everything.

### Removing elements while looping over the list

**You think:** this code removes all the negative numbers.

```python
numbers = [-1, -2, 3, 4]
for n in numbers:
    if n < 0:
        numbers.remove(n)
print(numbers)
```

```sortie
[-2, 3, 4]
```

**Why:** the loop moves forward one position at each turn. But `remove` shifts all the
elements to the left. So the loop skips an element. Let's follow it step by step:

| Turn | Position read | List before | `n` | Action | List after |
|---|---|---|---|---|---|
| 1 | 0 | `[-1, -2, 3, 4]` | `-1` | remove | `[-2, 3, 4]` |
| 2 | 1 | `[-2, 3, 4]` | `3` | nothing | `[-2, 3, 4]` |
| 3 | 2 | `[-2, 3, 4]` | `4` | nothing | `[-2, 3, 4]` |

At turn 2, the loop reads position 1. The `-2` has slid to position 0: it is never read.

**The fix:** do not change the list you are looping over. Build a **new** list with
what you keep, for example with a [comprehension](comprehensions.md).

```python
numbers = [-1, -2, 3, 4]
numbers = [n for n in numbers if n >= 0]
print(numbers)
```

```sortie
[3, 4]
```

### sort() returns nothing

**You think:** `words = words.sort()` stores the sorted list in `words`.

```python
words = ["pear", "apricot"]
words = words.sort()
print(words)
```

```sortie
None
```

**Why:** `.sort()` sorts the list **in place**: it changes the list itself, then
returns `None` ("nothing"). So you have stored `None` in `words`.

**The fix:** two ways, your choice.

```python
words = ["pear", "apricot"]
words.sort()
print(words)

fruits = ["pear", "apricot"]
in_order = sorted(fruits)
print(in_order)
print(fruits)
```

```sortie
['apricot', 'pear']
['apricot', 'pear']
['pear', 'apricot']
```

`my_list.sort()` changes the list. `sorted(my_list)` leaves the list alone and returns
a new sorted list.

> **Remember:** a list can have several names. Before changing it, ask yourself "who
> else is looking at this list?".

## Text traps

### Text methods do not change the text

**You think:** `name.upper()` puts `name` in capital letters.

```python
name = "alice"
name.upper()
print(name)
```

```sortie
alice
```

**Why:** a string is [immutable](../debuter/glossaire.md#immutable): it can never be
changed. `upper()` builds a **new** string. If you do not store it anywhere, it is lost.

It is the opposite of the `sort()` trap: for text, the method returns the result and
does not touch the original.

**The fix:** store the result.

```python
name = "alice"
name = name.upper()
print(name)
```

```sortie
ALICE
```

## Function traps

### Displaying instead of returning

**You think:** this function gives the double of a number.

```python
def double(x):
    print(x * 2)

result = double(5)
print(result)
```

```sortie
10
None
```

**Why:** `print` **shows** a value on the screen. `return` **gives** a value to the
rest of the program. A function without `return` returns `None`.

Think of a drinks vending machine. `print` is the drink shown on the screen. `return`
is the drink that falls into the tray: you can take it.

**The fix:**

```python
def double(x):
    return x * 2

result = double(5)
print(result)
print(result + 1)
```

```sortie
10
11
```

### An empty list as a default value

**You think:** each call starts with an empty cart.

```python
def add(item, cart=[]):
    cart.append(item)
    return cart

print(add("bread"))
print(add("milk"))
```

```sortie
['bread']
['bread', 'milk']
```

**Why:** the default value `[]` is created **only once**, when Python reads the `def`
line. All the calls then share **the same** list.

**The fix:** use `None` as the default, and create the list inside the function.

```python
def add(item, cart=None):
    if cart is None:
        cart = []
    cart.append(item)
    return cart

print(add("bread"))
print(add("milk"))
```

```sortie
['bread']
['milk']
```

### Changing a variable from outside

**You think:** the function adds 1 to the counter.

```python norun
counter = 0

def increment():
    counter = counter + 1

increment()
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    increment()
    ~~~~~~~~~^^
  File "main.py", line 4, in increment
    counter = counter + 1
              ^^^^^^^
UnboundLocalError: cannot access local variable 'counter' where it is not associated with a value
```

**Why:** as soon as a function **stores** something in a name (`counter = …`), Python
considers that this name belongs to the function. It is a matter of
[scope](../debuter/glossaire.md#scope). So the function looks for its own `counter`,
which has no value yet.

**The fix:** pass the value as a parameter and return the result.

```python
def increment(counter):
    return counter + 1

counter = 0
counter = increment(counter)
counter = increment(counter)
print(counter)
```

```sortie
2
```

The `global` keyword also exists, but it makes programs hard to follow. Avoid it at
the beginning.

### Calling a variable list or str

**You think:** `list` is a good name for a list.

```python norun
list = [3, 1, 2]
print(list("abc"))
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(list("abc"))
          ~~~~^^^^^^^
TypeError: 'list' object is not callable
```

**Why:** `list`, `str`, `int`, `sum`, `max`… are Python functions. If you store
something else under that name, you **hide** the function. Python finds your list and
tries to "call" it like a function.

**The fix:** choose a name that says what the variable contains: `numbers`,
`first_names`, `text`, `total`.

> **Remember:** a function **returns** with `return`, works with its **parameters**,
> and must not reuse Python's names.

## Common mistakes

Two more errors, which are not traps but happen every day.

### Forgetting the quotes

```python norun
print(hello)
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    print(hello)
          ^^^^^
NameError: name 'hello' is not defined. Did you mean: 'help'?
```

Without quotes, `hello` is a **variable name**, and that variable does not exist.
Write `print("hello")`. The same error happens with a typo in a name: `nmae` instead
of `name`.

### A key missing from the dictionary

```python norun
ages = {"Alice": 30}
print(ages["Bob"])
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(ages["Bob"])
          ~~~~^^^^^^^
KeyError: 'Bob'
```

The key `"Bob"` does not exist. Use `.get`, which returns a fallback value instead of
crashing:

```python
ages = {"Alice": 30}
print(ages.get("Bob", "unknown"))
```

```sortie
unknown
```

To read an error message calmly, see
[reading an error](../debuter/lire-une-erreur.md).

## Test yourself

**Question 1:** what does this program display, and why?

```python norun
a = [1, 2]
b = a
b.append(3)
print(a)
```

> **Answer**
>
> It displays `[1, 2, 3]`. `b = a` does not copy the list: `a` and `b` are two labels
> on the same list. For a real copy, write `b = a.copy()`.

**Question 2:** this program displays `10 / 4 = 2.5`. How do you get `2`?

```python norun
print("10 / 4 =", 10 / 4)
```

> **Answer**
>
> With integer division `//`:
>
> ```python
> print("10 // 4 =", 10 // 4)
> ```
>
> ```sortie
> 10 // 4 = 2
> ```

**Question 3:** why does this program display `None`?

```python norun
def square(x):
    print(x * x)

print(square(3))
```

> **Answer**
>
> The function displays `9`, but returns nothing: without `return`, it returns `None`.
> So the second `print` displays `None`. Replace `print(x * x)` with
> `return x * x`.

## Summary

- `=` stores a value, `==` compares two values.
- Indentation is part of the code: 4 spaces per level.
- `"12"` is text: convert it with `int()` before calculating. `/` always gives a float,
  `//` an integer.
- You count from 0, and `range(a, b)` stops before `b`.
- `b = a` does not copy a list; do not change a list while you loop over it.
- A function returns with `return`; no empty list as a default value.

Next: [exercises to get started](exercices-faciles.md)
