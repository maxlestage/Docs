# Handling errors

This page explains how to read a Python error message, how to catch errors with `try` and `except`, how to raise them with `raise`, and how `with` guarantees the cleanup.

> **Goals:**
>
> - you will be able to read an error trace line by line;
> - you will be able to catch a specific error with `try` and `except`;
> - you will be able to use `else` and `finally`;
> - you will be able to raise your own errors with `raise` and exception classes;
> - you will be able to use `with` and choose between "asking forgiveness" and "asking
>   permission".

## An error stops the program

When Python runs into a problem, it raises an
[exception](../debuter/glossaire.md#exception): an object that describes the
[error](../debuter/glossaire.md#error). "Raise" means: set off the alarm.
If nobody deals with it, the program stops and displays a **trace** (*traceback*).

```python norun
def average(grades):
    return sum(grades) / len(grades)

def report_card(student, grades):
    m = average(grades)
    return f"{student}: {m:.1f}"

print(report_card("Alice", [12, 15]))
print(report_card("Bob", []))
```

```text
Alice: 13.5
Traceback (most recent call last):
  File "main.py", line 9, in <module>
    print(report_card("Bob", []))
          ~~~~~~~~~~~^^^^^^^^^^^
  File "main.py", line 5, in report_card
    m = average(grades)
  File "main.py", line 2, in average
    return sum(grades) / len(grades)
           ~~~~~~~~~~~~^~~~~~~~~~~~~
ZeroDivisionError: division by zero
```

The first line was displayed: the program stops **at the moment** of the error, not
before.

### Reading the trace line by line

A trace is read **from bottom to top**: the cause is at the end.

1. `ZeroDivisionError: division by zero`: the **class** of the error (its family
   name), then its description. Always start there.
2. `File "main.py", line 2, in average`: the exact place, in the `average` function.
   The `~~~^~~~` underline the faulty operation: the division.
3. `line 5, in report_card`: `average` had been called by `report_card`, line 5.
4. `line 9, in <module>`: and `report_card` by the main program (`<module>`),
   line 9.

`Traceback (most recent call last)` reminds you of it: the most recent call comes last.
Here, it is quick to understand: `report_card` received an empty list, and `len([])`
equals 0.

> **Note:** on your computer, `File` shows the full path, for example
> `File "/home/you/project/main.py"`. It is shortened to `main.py` in this guide.

## try and except

`try` and `except` ("except if") catch an error. The program keeps going instead of
stopping. It is the net under a trapeze artist: the fall is caught, the show goes on.

- The risky code goes in the `try` block.
- The backup code goes in the `except` block, with the expected error class.

```python
try:
    result = 10 / 0
    print("This line is never reached")
except ZeroDivisionError:
    print("You can't divide by zero!")
print("The program keeps going.")
```

```sortie
You can't divide by zero!
The program keeps going.
```

As soon as an error happens in `try`, Python jumps straight to the `except`: the rest of
the `try` block is abandoned.

### Getting the message

With `as e`, you get the exception object. `str(e)` (or `print(e)`) gives its message.

```python
try:
    age = int("twelve")
except ValueError as e:
    print("Error:", e)
```

```sortie
Error: invalid literal for int() with base 10: 'twelve'
```

### Several except blocks

You can write several `except` blocks, one per error class, or group several classes in
a tuple.

```python
def divide(text_a, text_b):
    try:
        return int(text_a) / int(text_b)
    except ValueError:
        return "That is not a number."
    except ZeroDivisionError:
        return "Division by zero."

print(divide("10", "4"))
print(divide("ten", "4"))
print(divide("10", "0"))
```

```sortie
2.5
That is not a number.
Division by zero.
```

> **Warning:** catch **specific** errors. A bare `except:`, or `except Exception:`,
> catches everything, including your typos: a bug then becomes invisible.

## else and finally

Two optional blocks complete `try`:

- `else` runs only if there was **no** error;
- `finally` runs **always**, error or not. It is used for cleaning up.

```python
def read_age(text):
    try:
        age = int(text)
    except ValueError:
        print(f"{text!r}: not a number")
    else:
        print(f"{text!r}: {age} years old")
    finally:
        print("  (check done)")

read_age("42")
read_age("old")
```

```sortie
'42': 42 years old
  (check done)
'old': not a number
  (check done)
```

Why `else` rather than putting the `print` in `try`? To keep **only** the risky line in
`try`: that way, you don't catch by mistake a problem that comes from somewhere else.

## Raising an error with raise

`raise` sets off an error yourself, for example to reject an absurd value. Give the
class of the error and a clear message.

```python
def withdraw(balance, amount):
    if amount <= 0:
        raise ValueError(f"invalid amount: {amount}")
    if amount > balance:
        raise ValueError("insufficient balance")
    return balance - amount

print(withdraw(100, 30))
try:
    withdraw(100, -5)
except ValueError as e:
    print("Refused:", e)
```

```sortie
70
Refused: invalid amount: -5
```

Raising an error rather than returning a strange value (`-1`, `None`…) forces the
caller to deal with it: the error cannot go unnoticed.

### Re-raising an error

In an `except`, a `raise` on its own re-raises the caught error, after doing something
(for example, logging the problem).

```python
try:
    try:
        int("abc")
    except ValueError:
        print("I log the problem, then I pass it on.")
        raise
except ValueError as e:
    print("Caught higher up:", e)
```

```sortie
I log the problem, then I pass it on.
Caught higher up: invalid literal for int() with base 10: 'abc'
```

## Your own exceptions

An exception is an object like any other, of a [class](types.md) that inherits from
`Exception` (see [inheritance](heritage.md)). Creating your own class gives a clear
name to your errors, which the caller can catch precisely.

```python
class InsufficientBalanceError(Exception):
    def __init__(self, balance, amount):
        super().__init__(f"{amount - balance} € missing")
        self.missing = amount - balance

class Account:
    def __init__(self, balance):
        self.balance = balance

    def withdraw(self, amount):
        if amount > self.balance:
            raise InsufficientBalanceError(self.balance, amount)
        self.balance -= amount

account = Account(50)
try:
    account.withdraw(80)
except InsufficientBalanceError as e:
    print("Refused:", e)
    print("Missing:", e.missing)
```

```sortie
Refused: 30 € missing
Missing: 30
```

> **Remember:** inherit from `Exception`, and end the name with `Error`. For a very
> simple exception, `class MyError(Exception): pass` is enough.

## assert: checking an assumption

`assert condition, "message"` raises an `AssertionError` if the condition is false.
It is used to check what **must** be true if your code is correct: it is a safeguard
for you, the developer.

```python
def percentage(part, total):
    assert total > 0, "total must be positive"
    return part / total * 100

print(percentage(1, 4))
try:
    percentage(1, 0)
except AssertionError as e:
    print("Bug detected:", e)
```

```sortie
25.0
Bug detected: total must be positive
```

> **Warning:** don't validate user input with `assert`: assertions can be turned off
> (`python3 -O`). For that, use `raise`.

## Context managers: with

Some resources must be **released** after use: an open file must be closed, even if an
error happens. `with` takes care of it for you.

```python
with open("grades.txt", "w") as f:
    f.write("15\n12\n")

with open("grades.txt") as f:
    print(f.read().split())
print(f.closed)
```

```sortie
['15', '12']
True
```

When the `with` block ends, the file is closed automatically, whether there was an
error or not. It is a ready-made `try … finally`. Files are explained in detail in
[reading and writing files](fichiers.md).

### Writing your own manager

The `contextlib` module lets you write one with a generator (see
[iterators and generators](iterateurs.md)): the code before `yield` runs on entry, the
code after it, in `finally`, on exit.

```python
from contextlib import contextmanager

@contextmanager
def section(title):
    print(f"[start {title}]")
    try:
        yield
    finally:
        print(f"[end {title}]")

with section("calculation"):
    print(2 + 2)

try:
    with section("risky"):
        1 / 0
except ZeroDivisionError:
    print("error caught")
```

```sortie
[start calculation]
4
[end calculation]
[start risky]
[end risky]
error caught
```

Even with the error, `[end risky]` is displayed: the cleanup is guaranteed.

## Asking forgiveness or asking permission?

Two ways to avoid an error:

- **LBYL** (*Look Before You Leap*): you check first with an `if`;
- **EAFP** (*Easier to Ask Forgiveness than Permission*): you try, and you catch the
  error if it fails.

```python
stock = {"apple": 3}

# LBYL: check first
if "pear" in stock:
    print(stock["pear"])
else:
    print("no pear (LBYL)")

# EAFP: just try
try:
    print(stock["pear"])
except KeyError:
    print("no pear (EAFP)")
```

```sortie
no pear (LBYL)
no pear (EAFP)
```

The EAFP style is very common in Python, especially when the check would be complicated
(is a text a valid number?) or when the situation can change between the test and the
action (a file deleted in the meantime). For a simple case like a dictionary key,
`stock.get("pear", 0)` is even shorter.

## Common exceptions

| Class | When? |
|--------|---------|
| `NameError` | unknown variable (often a typo) |
| `TypeError` | wrong type, for example `"a" + 1` |
| `ValueError` | right type, wrong value: `int("abc")` |
| `IndexError` | index outside the list: `[1, 2][5]` |
| `KeyError` | key missing from a dictionary |
| `AttributeError` | unknown attribute or method: `(5).upper()` |
| `ZeroDivisionError` | division by zero |
| `FileNotFoundError` | file not found |
| `ImportError` / `ModuleNotFoundError` | module not found |
| `StopIteration` | iterator exhausted |
| `AssertionError` | `assert` whose condition is false |

They all inherit from `Exception`. Catching a parent class also catches its children:
`except LookupError` catches both `IndexError` and `KeyError`.

## Common mistakes

### Catching the wrong error class

```python norun
try:
    age = int("twelve")
except ZeroDivisionError:
    print("Problem")
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    age = int("twelve")
ValueError: invalid literal for int() with base 10: 'twelve'
```

`except ZeroDivisionError` only catches divisions by zero: the `ValueError` slips
through. Copy the class written at the bottom of the trace: `except ValueError:`.

### Raising a string instead of an exception

```python norun
def withdraw(balance, amount):
    if amount > balance:
        raise "insufficient balance"
    return balance - amount

withdraw(10, 50)
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    withdraw(10, 50)
    ~~~~~~~~^^^^^^^^
  File "main.py", line 3, in withdraw
    raise "insufficient balance"
TypeError: exceptions must derive from BaseException
```

You can only raise exception objects. Write `raise ValueError("insufficient balance")`.

### Opening a file that does not exist

```python norun
with open("grades.txt") as f:
    print(f.read())
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    with open("grades.txt") as f:
         ~~~~^^^^^^^^^^^^^^
FileNotFoundError: [Errno 2] No such file or directory: 'grades.txt'
```

The file is not in the folder where you start the program. Check the name and the
folder, or catch `FileNotFoundError` to display a clear message.

## Test yourself

**Question 1:** what does this code display?

```python norun
try:
    print("A")
    int("x")
    print("B")
except ValueError:
    print("C")
else:
    print("D")
finally:
    print("E")
```

> **Answer**
>
> `A`, `C`, then `E`. `B` is skipped because of the error, and `D` is only displayed if
> there is no error. `finally` always runs.

**Question 2:** in a trace, where do you find the cause of the error?

> **Answer**
>
> On the **last** line (class and message), then just above it: the file and the line
> where the error happened.

**Question 3:** write a function `valid_age(n)` that raises a `ValueError` if `n` is
negative, and returns `n` otherwise.

> **Answer**
>
> ```python
> def valid_age(n):
>     if n < 0:
>         raise ValueError("negative age")
>     return n
>
> print(valid_age(12))
> try:
>     valid_age(-1)
> except ValueError as e:
>     print(e)
> ```
>
> ```sortie
> 12
> negative age
> ```

## Summary

- A trace is read from bottom to top: class and message, then the faulty line.
- `try` / `except Class as e` catches a specific error; `else` runs when there is no
  error, `finally` always.
- `raise` raises an error; your exceptions inherit from `Exception`.
- `assert` checks your assumptions as a developer, not user input.
- `with` releases a resource automatically, even when an error happens.
- In Python, you often try first and catch the error afterwards (EAFP).

Next: [context managers](contextes.md). Compare with
[errors in Ruby](../ruby/erreurs.md).
