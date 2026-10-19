# Cheat sheet

This page sums up at a glance the Python syntax seen throughout the track, with compact tables and short examples.

## Basics

| Element | Example |
| --- | --- |
| Variable | `age = 30` |
| Constant (convention) | `RATE = 0.2` |
| Text | `"Hello"`, `'Hello'` |
| f-string | `f"Hello {name}"` |
| Booleans | `True`, `False` |
| Nothing | `None` |
| Comment | `# text` |
| Print | `print(a, b, sep=", ", end="")` |
| Read the keyboard | `name = input("Name: ")` (always text) |
| Type of a value | `type(x)`, `isinstance(x, int)` |

```python
name = "Alice"
age = 30
print(f"Hello {name}, {age} years old")
print(type(age), type(3.5), type(name), type(None))
```

```sortie
Hello Alice, 30 years old
<class 'int'> <class 'float'> <class 'str'> <class 'NoneType'>
```

Details: [first steps](premiers-pas.md).

## Numbers

| Operation | Example | Result |
| --- | --- | --- |
| Division | `7 / 2` | `3.5` |
| Integer division | `7 // 2` | `3` |
| Remainder | `7 % 2` | `1` |
| Power | `2 ** 10` | `1024` |
| Rounding | `round(3.14159, 2)` | `3.14` |
| Conversion | `int("42")`, `float("2.5")`, `str(42)` | `42`, `2.5`, `"42"` |
| Absolute value, min, max | `abs(-3)`, `min(4, 2)`, `max(4, 2)` | `3`, `2`, `4` |

```python
import math

print(7 / 2, 7 // 2, 7 % 2, 2 ** 10)
print(round(math.pi, 2), math.sqrt(16))
print(0.1 + 0.2, math.isclose(0.1 + 0.2, 0.3))
print(f"{1234567.891:,.2f}")
```

```sortie
3.5 3 1 1024
3.14 4.0
0.30000000000000004 True
1,234,567.89
```

Details: [numbers](nombres.md).

## Conditions and loops

| Need | Syntax |
| --- | --- |
| Condition | `if … elif … else` |
| Conditional expression | `"yes" if x > 0 else "no"` |
| Several cases | `match x:` then `case 1:` … `case _:` |
| Loop over a sequence | `for x in items:` |
| Count | `for i in range(1, 6):` |
| Index and value | `for i, x in enumerate(items, start=1):` |
| Two lists together | `for a, b in zip(l1, l2):` |
| While | `while condition:` |
| Control | `break`, `continue`, `pass` |

```python
for n in range(1, 6):
    if n == 2:
        continue
    print(n, "even" if n % 2 == 0 else "odd")

match "add":
    case "add" | "ajouter":
        print("adding")
    case _:
        print("unknown")
```

```sortie
1 odd
3 odd
4 even
5 odd
adding
```

Details: [conditions and loops](conditions-boucles.md).

## Text

| Need | Method |
| --- | --- |
| Uppercase, lowercase | `s.upper()`, `s.lower()`, `s.title()` |
| Remove spaces | `s.strip()` |
| Split, join back | `s.split(",")`, `", ".join(items)` |
| Replace | `s.replace("a", "b")` |
| Search | `"ab" in s`, `s.find("ab")`, `s.startswith("x")` |
| Length, piece | `len(s)`, `s[0]`, `s[-1]`, `s[1:4]`, `s[::-1]` |
| Align | `f"{s:<10}"`, `f"{s:>10}"`, `f"{s:^10}"` |
| Formatted number | `f"{x:.2f}"`, `f"{n:03d}"` |

```python
s = "  Hello big world  "
words = s.strip().split()
print(words)
print("-".join(words).upper())
print(f"[{'Python':^10}] [{3.14159:.2f}] [{7:03d}]")
print("radar"[::-1], len("été"))
```

```sortie
['Hello', 'big', 'world']
HELLO-BIG-WORLD
[  Python  ] [3.14] [007]
radar 3
```

Details: [working with text](texte.md).

## Lists and tuples

| Need | Syntax |
| --- | --- |
| Create | `l = [1, 2, 3]`, `t = (1, 2)` (tuple, cannot be changed) |
| Add | `l.append(x)`, `l.extend([4, 5])`, `l.insert(0, x)` |
| Remove | `l.pop()`, `l.pop(0)`, `l.remove(x)` |
| Sort | `sorted(l)` (new list), `l.sort()` (in place) |
| Sort by a key | `sorted(l, key=len, reverse=True)` |
| Slice | `l[1:3]`, `l[:2]`, `l[-2:]` |
| Aggregate | `sum(l)`, `min(l)`, `max(l)`, `len(l)` |
| Test | `x in l`, `any(...)`, `all(...)` |
| Unpack | `a, b = (1, 2)`, `first, *rest = l` |

```python
fruits = ["pear", "kiwi", "plum"]
fruits.append("apricot")
print(sorted(fruits))
print(sorted(fruits, key=len))
first, *rest = fruits
print(first, rest)
print(any(len(f) > 6 for f in fruits), fruits[-2:])
```

```sortie
['apricot', 'kiwi', 'pear', 'plum']
['pear', 'kiwi', 'plum', 'apricot']
pear ['kiwi', 'plum', 'apricot']
True ['plum', 'apricot']
```

Details: [lists and tuples](collections.md).

## Dictionaries

| Need | Syntax |
| --- | --- |
| Create | `d = {"a": 1, "b": 2}` |
| Read | `d["a"]` (error if missing), `d.get("z", 0)` |
| Write, delete | `d["c"] = 3`, `del d["a"]`, `d.pop("a")` |
| Loop over | `for key, value in d.items():` |
| Keys, values | `d.keys()`, `d.values()` |
| Test a key | `"a" in d` |
| Merge | `d1 \| d2` |
| Count | `collections.Counter(items)` |
| Group | `collections.defaultdict(list)` |

```python
from collections import Counter, defaultdict

stock = {"apple": 3, "kiwi": 0}
stock["pear"] = 5
print(stock.get("banana", 0), "kiwi" in stock)
for fruit, quantity in stock.items():
    print(fruit, quantity)

print(Counter("abracadabra").most_common(2))
by_initial = defaultdict(list)
for word in ["ant", "arch", "bowl"]:
    by_initial[word[0]].append(word)
print(dict(by_initial))
```

```sortie
0 True
apple 3
kiwi 0
pear 5
[('a', 5), ('b', 2)]
{'a': ['ant', 'arch'], 'b': ['bowl']}
```

Details: [dictionaries](dictionnaires.md).

## Sets

| Operation | Syntax |
| --- | --- |
| Create | `{1, 2, 3}`, `set()` (empty), `set(items)` |
| Add, remove | `s.add(x)`, `s.discard(x)` |
| Union, intersection | `a \| b`, `a & b` |
| Difference | `a - b`, `a ^ b` (in only one of the two) |

```python
a = {1, 2, 3, 4}
b = {3, 4, 5}
print(sorted(a | b), sorted(a & b), sorted(a - b))
print(sorted(set([3, 1, 3, 2, 1])))
```

```sortie
[1, 2, 3, 4, 5] [3, 4] [1, 2]
[1, 2, 3]
```

A set has no order: `sorted` gives a stable display.

## Comprehensions and generators

| Form | Example |
| --- | --- |
| List | `[x * 2 for x in l if x > 0]` |
| Dictionary | `{x: x * x for x in l}` |
| Set | `{len(w) for w in words}` |
| Generator (lazy) | `sum(x * x for x in l)` |
| Generator function | `yield` inside a function |

```python
numbers = [3, -1, 4, 0, 5]
print([n * 2 for n in numbers if n > 0])
print({n: n * n for n in numbers if n > 3})
print(sum(n * n for n in numbers))


def countdown(n):
    while n > 0:
        yield n
        n -= 1


print(list(countdown(3)))
```

```sortie
[6, 8, 10]
{4: 16, 5: 25}
51
[3, 2, 1]
```

Details: [comprehensions](comprehensions.md), [iterators](iterateurs.md).

## Functions

| Element | Syntax |
| --- | --- |
| Define | `def name(a, b=0):` then `return …` |
| Keyword arguments | `name(1, b=2)` |
| Variable number | `def f(*args, **kwargs):` |
| Anonymous function | `lambda x: x * 2` |
| Annotations | `def f(x: int) -> str:` |
| Docstring | `"""What the function does."""` |
| Decorator | `@functools.cache` above `def` |

```python
from functools import cache


def greet(name: str, greeting: str = "Hello") -> str:
    """Return a greeting sentence."""
    return f"{greeting} {name}!"


def total(*numbers):
    return sum(numbers)


@cache
def fib(n):
    return n if n < 2 else fib(n - 1) + fib(n - 2)


print(greet("Alice"), greet("Bob", greeting="Hi"))
print(total(1, 2, 3), fib(50))
print(sorted(["bb", "a", "ccc"], key=lambda w: -len(w)))
```

```sortie
Hello Alice! Hi Bob!
6 12586269025
['ccc', 'bb', 'a']
```

Details: [functions](fonctions.md), [decorators](decorateurs.md),
[type hints](typage.md).

## Classes and inheritance

| Element | Syntax |
| --- | --- |
| Class | `class Dog:` |
| Constructor | `def __init__(self, name):` |
| Attribute | `self.name = name` |
| Readable text | `def __str__(self):` |
| Inheritance | `class Dog(Animal):` |
| Call the parent | `super().__init__(name)` |
| Computed property | `@property` |

```python
class Animal:
    def __init__(self, name):
        self.name = name

    def speak(self):
        return "..."

    def __str__(self):
        return f"{self.name} says {self.speak()}"


class Dog(Animal):
    def speak(self):
        return "Woof"


for animal in [Animal("Thingy"), Dog("Rex")]:
    print(animal)
print(isinstance(Dog("Rex"), Animal))
```

```sortie
Thingy says ...
Rex says Woof
True
```

Details: [classes](types.md), [inheritance](heritage.md).

## Dataclasses

`@dataclass` writes `__init__`, `__repr__` and `__eq__` for you.

```python
from dataclasses import dataclass, field


@dataclass
class Point:
    x: float
    y: float = 0.0


@dataclass(frozen=True)
class Color:
    name: str


@dataclass
class Basket:
    items: list[str] = field(default_factory=list)


p = Point(1, 2)
print(p, p == Point(1, 2), Point(3))
print(Color("red"))
basket = Basket()
basket.items.append("bread")
print(basket)
```

```sortie
Point(x=1, y=2) True Point(x=3, y=0.0)
Color(name='red')
Basket(items=['bread'])
```

## Exceptions

| Element | Syntax |
| --- | --- |
| Catch | `try:` … `except ValueError as e:` |
| Several types | `except (KeyError, IndexError):` |
| If all goes well | `else:` |
| Always | `finally:` |
| Raise | `raise ValueError("message")` |
| Your own error | `class InsufficientFunds(Exception): pass` |

```python
class InsufficientFunds(Exception):
    pass


def withdraw(balance, amount):
    if amount > balance:
        raise InsufficientFunds(f"{amount - balance} € short")
    return balance - amount


for text in ["30", "abc", "500"]:
    try:
        print("left", withdraw(100, int(text)))
    except ValueError:
        print("not a number:", text)
    except InsufficientFunds as error:
        print("refused:", error)
    finally:
        print("--")
```

```sortie
left 70
--
not a number: abc
--
refused: 400 € short
--
```

Details: [errors](erreurs.md), [debugging](debogage.md).

## Files

| Need | Syntax |
| --- | --- |
| Write | `with open("f.txt", "w", encoding="utf-8") as f: f.write(text)` |
| Append at the end | mode `"a"` |
| Read everything | `Path("f.txt").read_text(encoding="utf-8")` |
| Read line by line | `for line in f:` |
| Test, list | `Path("f.txt").exists()`, `Path(".").glob("*.txt")` |
| Paths | `Path("folder") / "f.txt"`, `p.name`, `p.suffix` |

```python
from pathlib import Path

path = Path("grades.txt")
with open(path, "w", encoding="utf-8") as f:
    f.write("Alice 15\nBob 12\n")

with open(path, encoding="utf-8") as f:
    for line in f:
        name, grade = line.split()
        print(name, int(grade) + 1)

print(path.exists(), path.suffix, path.read_text(encoding="utf-8").count("\n"))
```

```sortie
Alice 16
Bob 13
True .txt 2
```

Details: [files](fichiers.md).

## JSON and CSV

| Need | Function |
| --- | --- |
| Python object to JSON text | `json.dumps(obj, indent=2, ensure_ascii=False)` |
| JSON text to Python object | `json.loads(text)` |
| With a file | `json.dump(obj, f)`, `json.load(f)` |
| Read a CSV | `csv.reader(f)`, `csv.DictReader(f)` |

```python
import csv
import io
import json

data = {"name": "Léa", "grades": [15, 12], "passed": True}
text = json.dumps(data, ensure_ascii=False)
print(text)
print(json.loads(text)["grades"])

for row in csv.DictReader(io.StringIO("name,grade\nAlice,15\nBob,12\n")):
    print(row["name"], int(row["grade"]))
```

```sortie
{"name": "Léa", "grades": [15, 12], "passed": true}
[15, 12]
Alice 15
Bob 12
```

Details: [data formats](formats.md).

## Regular expressions

| Pattern | Meaning |
| --- | --- |
| `\d`, `\w`, `\s` | a digit, a letter or digit, a space |
| `.` | any character |
| `+`, `*`, `?`, `{2,4}` | 1 or more, 0 or more, 0 or 1, between 2 and 4 |
| `^`, `$` | start, end |
| `[a-z]`, `[^0-9]` | a character from the list, outside the list |
| `( )`, `(?P<name> )` | a group, a named group |

```python
import re

text = "Order 42 on 2026-03-15, order 7 on 2026-04-01"
print(re.findall(r"\d+", text))

m = re.search(r"(?P<year>\d{4})-(?P<month>\d{2})", text)
print(m.group("year"), m.group("month"))

print(re.sub(r"\s+", " ", "too   many   spaces"))
print(bool(re.fullmatch(r"[\w.]+@[\w.]+\.\w+", "alice@example.com")))
```

```sortie
['42', '2026', '03', '15', '7', '2026', '04', '01']
2026 03
too many spaces
True
```

Details: [regular expressions](regex.md).

## Dates

| Need | Syntax |
| --- | --- |
| A date, a date and time | `date(2026, 3, 15)`, `datetime(2026, 3, 15, 14, 30)` |
| Today, now | `date.today()`, `datetime.now()` |
| Duration | `timedelta(days=7, hours=2)` |
| Format | `d.strftime("%d/%m/%Y")` |
| Read a text | `datetime.strptime("15/03/2026", "%d/%m/%Y")`, `date.fromisoformat("2026-03-15")` |

```python
from datetime import date, datetime, timedelta

school_start = date(2026, 9, 1)
holidays = date(2026, 10, 17)
print((holidays - school_start).days, "days")
print(school_start + timedelta(weeks=2))
print(school_start.strftime("%d/%m/%Y"), school_start.isoweekday())

meeting = datetime.strptime("15/03/2026 14:30", "%d/%m/%Y %H:%M")
print(meeting.isoformat())
```

```sortie
46 days
2026-09-15
01/09/2026 2
2026-03-15T14:30:00
```

Details: [dates](dates.md).

## Command line with argparse

```python
import argparse

parser = argparse.ArgumentParser(description="Computes an average.")
parser.add_argument("grades", type=float, nargs="+", help="the grades")
parser.add_argument("--digits", type=int, default=1)
parser.add_argument("-v", "--verbose", action="store_true")

args = parser.parse_args(["12", "15", "17", "--digits", "2", "-v"])
average = sum(args.grades) / len(args.grades)
if args.verbose:
    print("Grades:", args.grades)
print(round(average, args.digits))
```

```sortie
Grades: [12.0, 15.0, 17.0]
14.67
```

In a real program, write `parser.parse_args()` without a list: it reads `sys.argv`.
`--help` is created for you. Details: [the command line](ligne-de-commande.md).

## Concurrency and asyncio

| Element | Syntax |
| --- | --- |
| Asynchronous function | `async def f():` |
| Wait | `await other()` |
| Start the program | `asyncio.run(main())` |
| In parallel | `await asyncio.gather(f(), g())` |
| Pause without blocking | `await asyncio.sleep(1)` |
| Threads | `concurrent.futures.ThreadPoolExecutor` |

```python
import asyncio


async def download(name, duration):
    await asyncio.sleep(duration)
    return f"{name} done"


async def main():
    results = await asyncio.gather(
        download("image", 0.02),
        download("text", 0.01),
    )
    print(results)


asyncio.run(main())
```

```sortie
['image done', 'text done']
```

`gather` gives back the results in the order of the calls, even if `text` finishes first.
Details: [concurrency](concurrence.md), [async](async.md).

## Tests with unittest

| Element | Syntax |
| --- | --- |
| Test class | `class TestX(unittest.TestCase):` |
| One test | method `def test_…(self):` |
| Equality | `self.assertEqual(actual, expected)` |
| True, in, floats | `assertTrue`, `assertIn`, `assertAlmostEqual` |
| Expected error | `with self.assertRaises(ValueError):` |
| Before each test | `def setUp(self):` |
| Run | `unittest.main()` or `python3 -m unittest` |

```python
import unittest


def double(x):
    return x * 2


class TestDouble(unittest.TestCase):
    def test_double(self):
        self.assertEqual(double(4), 8)

    def test_error(self):
        with self.assertRaises(ZeroDivisionError):
            1 / 0


program = unittest.main(argv=["x"], exit=False)
print(program.result.testsRun, program.result.wasSuccessful())
```

```sortie
2 True
```

Details: [testing your code](tests.md).

## Project and tools

| Command | Role |
| --- | --- |
| `python3 main.py` | run a program |
| `python3` | interactive console (REPL) |
| `python3 -m venv .venv` | create a virtual environment |
| `source .venv/bin/activate` | activate it (Linux, macOS) |
| `python -m pip install name` | install a package |
| `python -m pip install -r requirements.txt` | install a project's list |
| `python -m pip install -e .` | install your own project |
| `python3 -m pdb main.py` | debug |
| `pytest`, `ruff check`, `black .` | test, check, format |
| `uv run main.py` | all in one with uv |

Details: [organizing a project](projet.md), [writing good Python](style.md). To
practice: [step-by-step projects](projets.md) and [exercises with solutions](exercices.md).
Compare with [the Ruby cheat sheet](../ruby/aide-memoire.md) and
[the Rust cheat sheet](../rust/aide-memoire.md).
