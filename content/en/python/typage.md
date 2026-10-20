# Type annotations

This page shows how to state the expected types in your code, how dataclasses write your classes for you, and how the mypy tool finds errors before the program runs.

> **Goals:**
>
> - you will be able to annotate variables and functions (`int`, `list[int]`, `dict[str, int]`);
> - you will be able to express "a value or nothing" with `X | None`;
> - you will be able to write a data class with `@dataclass`;
> - you will be able to describe a dictionary with `TypedDict` and a behavior with `Protocol`;
> - you will be able to check your code with mypy.

## Why annotate?

Python never asks for the [type](../debuter/glossaire.md#type) of a variable: it finds it
out while the program runs. That is flexible, but a type error only shows up when the
line runs, sometimes weeks later.

A **type annotation** (*type hint*) is a label that says which type you expect. It works
as documentation, helps your editor complete your code, and lets a checking tool find
errors **without running the program**.

> **Remember:** Python itself **ignores** annotations when the program runs. They change
> nothing in how the program behaves. Tools like mypy are the ones that check them.

```python
age: int = "ten"
print(age)
```

```sortie
ten
```

No error: Python did not look at the annotation. We will see further down how mypy does
report it.

## Annotating variables and functions

For a variable, you write `: type` after the name. For a function, you annotate each
parameter, then the return type after an arrow `->`.

```python
name: str = "Alice"
age: int = 30
height: float = 1.68
adult: bool = True

def greet(first_name: str, times: int = 1) -> str:
    return ("Hello " + first_name + "! ") * times

def display(message: str) -> None:
    print(message)

display(greet(name, 2))
```

```sortie
Hello Alice! Hello Alice! 
```

`-> None` means: "this function returns nothing useful".

> **Tip:** there is no need to annotate every variable. For `age = 30`, the tool guesses
> `int` on its own. Annotate mostly the **parameters** and **return values** of functions.

## Collections

For a collection, you also give the type of what it contains, in square brackets.

| Annotation | Means |
|------------|-------|
| `list[int]` | a list of integers |
| `dict[str, int]` | a dictionary with `str` keys and `int` values |
| `set[str]` | a set of strings |
| `tuple[str, int]` | a tuple of two items: a `str` then an `int` |
| `tuple[int, ...]` | a tuple of integers, of any length |

```python
def average(grades: list[float]) -> float:
    return sum(grades) / len(grades)

def count_words(text: str) -> dict[str, int]:
    counts: dict[str, int] = {}
    for word in text.split():
        counts[word] = counts.get(word, 0) + 1
    return counts

def min_max(numbers: list[int]) -> tuple[int, int]:
    return min(numbers), max(numbers)

print(average([12, 15.5, 9]))
print(count_words("the cat and the dog"))
print(min_max([4, 1, 9]))
```

```sortie
12.166666666666666
{'the': 2, 'cat': 1, 'and': 1, 'dog': 1}
(1, 9)
```

## A value or nothing: X | None

A function that looks for something may find nothing and return `None`. You write it
`str | None`: "a `str`, or else `None`".

```python
def find(names: list[str], start: str) -> str | None:
    for name in names:
        if name.startswith(start):
            return name
    return None

result = find(["Alice", "Bob"], "B")
if result is not None:
    print(result.upper())
print(find(["Alice", "Bob"], "Z"))
```

```sortie
BOB
None
```

The `if result is not None` matters: inside it, the tool knows that `result` is a `str`,
and allows `.upper()`.

### Optional and Union

In older code, you will see the same ideas written with the `typing` module:

| Old style | Modern style |
|-----------|--------------|
| `Optional[str]` | `str \| None` |
| `Union[int, str]` | `int \| str` |

```python
from typing import Optional, Union

def double(x: Union[int, str]) -> Union[int, str]:
    return x * 2

def initial(name: Optional[str]) -> str:
    return name[0] if name else "?"

print(double(4), double("ab"))
print(initial("Zoé"), initial(None))
```

```sortie
8 abab
Z ?
```

Prefer the style with `|`: it is shorter and needs no import.

## Dataclasses

Writing `__init__`, `__repr__` and `__eq__` for every small class is repetitive (see
[classes and objects](types.md)). The `@dataclass` decorator **generates them for you**
from the annotations.

```python
from dataclasses import dataclass

@dataclass
class Point:
    x: int
    y: int

p = Point(1, 2)
print(p)
print(p.x + p.y)
print(p == Point(1, 2))
```

```sortie
Point(x=1, y=2)
3
True
```

Three lines are enough: each annotation becomes a **field**, that is, a parameter of
`__init__` and an attribute.

### Default values

A field can have a default value. For a list or a dictionary, use
`field(default_factory=list)`: that way each object gets **its own** list.

```python
from dataclasses import dataclass, field

@dataclass
class Student:
    name: str
    class_name: str = "6A"
    grades: list[int] = field(default_factory=list)

    def average(self) -> float:
        return sum(self.grades) / len(self.grades) if self.grades else 0.0

alice = Student("Alice")
alice.grades.append(15)
alice.grades.append(12)
bob = Student("Bob", "7B")
print(alice, alice.average())
print(bob)
```

```sortie
Student(name='Alice', class_name='6A', grades=[15, 12]) 13.5
Student(name='Bob', class_name='7B', grades=[])
```

A dataclass is still a normal class: you can add methods to it.

### frozen and order

Two useful options:

- `frozen=True` makes the object **immutable**: its fields cannot be changed;
- `order=True` generates `<`, `>`, `<=`, `>=`: objects are compared field by field, in
  the order they are written.

```python
from dataclasses import dataclass

@dataclass(frozen=True, order=True)
class Version:
    major: int
    minor: int

versions = [Version(2, 1), Version(1, 9), Version(2, 0)]
print(sorted(versions))
print(max(versions))
print(Version(1, 0) < Version(1, 2))
```

```sortie
[Version(major=1, minor=9), Version(major=2, minor=0), Version(major=2, minor=1)]
Version(major=2, minor=1)
True
```

> **Note:** a `frozen` object can be used as a dictionary key or go into a set, because
> its value will never change.

## TypedDict: describing a dictionary

Sometimes your data stays as dictionaries (for example read from
[JSON](../debuter/glossaire.md#json)). `TypedDict` describes the expected keys and the
type of each value. When the program runs, it is an ordinary dictionary.

```python
from typing import TypedDict

class Movie(TypedDict):
    title: str
    year: int

def summarize(movie: Movie) -> str:
    return f"{movie['title']} ({movie['year']})"

alien: Movie = {"title": "Alien", "year": 1979}
print(summarize(alien))
print(type(alien))
```

```sortie
Alien (1979)
<class 'dict'>
```

## Protocol: describing a behavior

In the chapter [inheritance and polymorphism](heritage.md), *duck typing* accepted any
object that has the right method. `Protocol` writes this rule as a type: "any object that
has a `fly()` method that returns a `str`".

```python
from typing import Protocol

class Flyer(Protocol):
    def fly(self) -> str: ...

class Bird:
    def fly(self) -> str:
        return "I flap my wings"

class Plane:
    def fly(self) -> str:
        return "I make my engines roar"

def take_off(f: Flyer) -> None:
    print(f.fly())

take_off(Bird())
take_off(Plane())
```

```sortie
I flap my wings
I make my engines roar
```

`Bird` and `Plane` do not inherit from `Flyer`: having the method is enough. `...`
(three dots) means "empty body": a protocol describes, it does nothing.

## Checking with mypy

**mypy** is the best-known type checker. It is not part of Python: you install it with
pip (see [modules, packages and pip](modules.md)).

```sh
python3 -m pip install mypy
```

Take this `main.py` file, full of type errors:

```python norun
def average(grades: list[int]) -> float:
    return sum(grades) / len(grades)

def greet(name: str) -> str:
    return "Hello " + name

print(average([12, 15]))
print(average("12, 15"))
age: int = "ten"
greet(42)
```

Run mypy on the file:

```sh
mypy main.py
```

```text
main.py:8: error: Argument 1 to "average" has incompatible type "str"; expected "list[int]"  [arg-type]
main.py:9: error: Incompatible types in assignment (expression has type "str", variable has type "int")  [assignment]
main.py:10: error: Argument 1 to "greet" has incompatible type "int"; expected "str"  [arg-type]
Found 3 errors in 1 file (checked 1 source file)
```

Each line gives the file, the line number and the problem. mypy found all three errors
**without running** the program. Python would have stopped at line 8 with a `TypeError`,
without seeing the next two.

> **Tip:** most editors (VS Code with Pylance, PyCharm) do the same check live and
> underline errors while you type.

## Common mistakes

### Forgetting the None case

```python norun
def find(names: list[str], start: str) -> str | None:
    for name in names:
        if name.startswith(start):
            return name
    return None

result = find(["Alice", "Bob"], "B")
print(result.upper())
```

```text
main.py:8: error: Item "None" of "str | None" has no attribute "upper"  [union-attr]
Found 1 error in 1 file (checked 1 source file)
```

mypy reminds you that `result` can be `None`. Test it first: `if result is not None:`.
Without this test, the program would crash the day nothing is found.

### A list as the default value of a dataclass

```python norun
from dataclasses import dataclass

@dataclass
class Student:
    name: str
    grades: list[int] = []
```

```text
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    @dataclass
     ^^^^^^^^^
  ...
ValueError: mutable default <class 'list'> for field grades is not allowed: use default_factory
```

(The full traceback goes through several lines of the `dataclasses` module, shortened
here.) A single list would be shared by all students. Write
`grades: list[int] = field(default_factory=list)`.

### Changing a frozen dataclass

```python norun
from dataclasses import dataclass

@dataclass(frozen=True)
class Point:
    x: int
    y: int

p = Point(1, 2)
p.x = 5
```

```text
Traceback (most recent call last):
  File "main.py", line 9, in <module>
    p.x = 5
    ^^^
  File "<string>", line 16, in __setattr__
dataclasses.FrozenInstanceError: cannot assign to field 'x'
```

A `frozen` object cannot be changed: create a new one. The `replace` function of the
`dataclasses` module does it for you: `p = replace(p, x=5)`.

## Test yourself

**Question 1:** which annotation fits a dictionary that maps a city name to a list of
decimal temperatures?

> **Answer**
>
> `dict[str, list[float]]`.

**Question 2:** does this program raise an error when you run it with `python3`?

```python norun
def double(x: int) -> int:
    return x * 2

print(double("ha"))
```

> **Answer**
>
> No: it displays `haha`. Python ignores annotations. mypy is the one that would report
> that `"ha"` is not an `int`.

**Question 3:** write a dataclass `Book` with a title, an author and a number of pages
(100 by default).

> **Answer**
>
> ```python
> from dataclasses import dataclass
>
> @dataclass
> class Book:
>     title: str
>     author: str
>     pages: int = 100
>
> print(Book("The Little Prince", "Saint-Exupéry"))
> ```
>
> ```sortie
> Book(title='The Little Prince', author='Saint-Exupéry', pages=100)
> ```

## Summary

- `x: int` and `def f(a: str) -> bool:` annotate types; Python ignores them when the
  program runs.
- `list[int]`, `dict[str, int]`, `tuple[str, int]` state what collections contain.
- `X | None` (or `Optional[X]`): a value that may be missing; test `is not None`.
- `@dataclass` generates `__init__`, `__repr__` and `__eq__`; `frozen` and `order` add
  immutability and sorting.
- `TypedDict` describes a dictionary, `Protocol` a behavior.
- `mypy main.py` finds type errors without running the program.

Next: [iterators and generators](iterateurs.md)
