# Conditions

This page explains how a Python program makes decisions with `if`, `elif`, `else` and `match`.

> **Goals:**
>
> - you will be able to compare two values with `==`, `!=`, `<` and `>`;
> - you will be able to write a condition with `if`, `elif` and `else`;
> - you will be able to combine tests with `and`, `or` and `not`;
> - you will be able to choose between several cases with `match`.

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

**Question 2:** what does `print(5 > 3 and not 2 > 4)` display?

> **Answer**
>
> `True`. `5 > 3` is true; `2 > 4` is false, and `not` makes it true. True `and` true
> gives `True`.

**Question 3:** in a `match`, what is `case _:` for?

> **Answer**
>
> It is the default case: it catches every value that no earlier `case` recognized.
> Put it last.

## Summary

- `==` compares, `=` stores; a comparison gives `True` or `False`.
- `if`, `elif` and `else` choose a path; each line ends with `:` and the block is
  indented by 4 spaces.
- `and`, `or` and `not` combine tests.
- `0`, `None` and empty collections count as false.
- `match` chooses among several cases; `case _:` catches the rest.

Next: [loops](boucles.md).
