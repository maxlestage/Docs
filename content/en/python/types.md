# Classes and objects

This page explains how to create your own types with `class`: their attributes, their methods, how they are displayed and their properties.

> **Goals:**
>
> - you will be able to write a class with `__init__` and create objects;
> - you will know what `self` stands for;
> - you will be able to add methods and a display with `__str__` and `__repr__`;
> - you will be able to use `@property`, `@classmethod` and `@staticmethod`;
> - you will know that a variable holds a **reference** to an object.

## Why classes?

So far, you have stored your data in lists and dictionaries. For a bank account, that
gives:

```python
account = {"owner": "Alice", "balance": 100}
account["balance"] += 50
print(account)
```

```sortie
{'owner': 'Alice', 'balance': 150}
```

It works, but nothing prevents a typo (`"balanse"`), and the actions (depositing,
withdrawing) are scattered elsewhere in the code.

A [class](../debuter/glossaire.md#class) groups **the data and the actions** in the
same place. It is a blueprint: the `House` class describes houses; each house built from
the blueprint is an [object](../debuter/glossaire.md#object), also called an
**instance**.

> **Note:** in Python, you have been using objects from the start: `"abc"` is an object
> of the `str` class, `[1, 2]` an object of the `list` class. `type("abc")` displays
> `<class 'str'>`.

## A first class

You write `class`, a name in `CamelCase` (a capital letter for each word), then a colon.
You create an object by **calling** the class like a function.

```python
class Dog:
    pass

rex = Dog()
buddy = Dog()
print(type(rex))
print(rex is buddy)
```

```sortie
<class '__main__.Dog'>
False
```

`pass` means "nothing for now". `rex` and `buddy` are two different objects, made from
the same blueprint. `__main__` is the name of the main file.

## __init__ and self

The special method `__init__` (*initialize*) runs automatically when the object is
created. It is used to fill in its **attributes**: the variables that belong to each
object.

```python
class Dog:
    def __init__(self, name, age):
        self.name = name
        self.age = age

rex = Dog("Rex", 3)
buddy = Dog("Buddy", 5)
print(rex.name, rex.age)
print(buddy.name, buddy.age)
```

```sortie
Rex 3
Buddy 5
```

Who is `self`? It is **the object being built**. When you write `Dog("Rex", 3)`,
Python:

1. creates an empty object;
2. calls `__init__(object, "Rex", 3)`: `self` is that object;
3. gives you back the filled-in object.

So `self.name = name` means: "store the value `name` in the `name` attribute of this
object". Without `self.`, the value would stay a simple local variable, lost at the end
of `__init__`.

> **Remember:** `self` is always the **first parameter** of methods, but you never pass
> it yourself: Python does it for you.

## Methods

A [method](../debuter/glossaire.md#method) is a function written inside the class. It
receives `self`, so it can read and change the object's attributes.

```python
class Account:
    def __init__(self, owner, balance=0):
        self.owner = owner
        self.balance = balance

    def deposit(self, amount):
        self.balance += amount

    def withdraw(self, amount):
        if amount > self.balance:
            print("Insufficient funds")
            return
        self.balance -= amount

account = Account("Alice", 100)
account.deposit(50)
account.withdraw(500)
account.withdraw(30)
print(account.balance)
```

```sortie
Insufficient funds
120
```

`account.deposit(50)` is a shortcut for `Account.deposit(account, 50)`: the object
placed before the dot becomes `self`.

## Displaying an object: __str__ and __repr__

By default, `print` displays an object in a not very useful way, with a memory address
that changes on every run (for example `<__main__.Point object at 0x7f3a…>`). Two
special methods solve this problem:

- `__str__`: the text for a **human**, used by `print` and `str()`;
- `__repr__`: the text for a **developer**, used in lists, by `repr()` and in the
  console. Ideally, it looks like the code that recreates the object.

```python
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __str__(self):
        return f"({self.x}, {self.y})"

    def __repr__(self):
        return f"Point({self.x}, {self.y})"

p = Point(2, 5)
print(p)
print(repr(p))
print([p, Point(0, 0)])
```

```sortie
(2, 5)
Point(2, 5)
[Point(2, 5), Point(0, 0)]
```

> **Tip:** if you write only one of the two, write `__repr__`: `print` uses it too when
> `__str__` is missing.

## Class attributes

An attribute written **directly in the class** (not in `__init__`) is shared by all
objects. It is used for a common constant, or a counter.

```python
class Dog:
    species = "Canis familiaris"
    count = 0

    def __init__(self, name):
        self.name = name
        Dog.count += 1

a = Dog("Rex")
b = Dog("Buddy")
print(a.species, b.species)
print(Dog.count)
```

```sortie
Canis familiaris Canis familiaris
2
```

> **Warning:** never use a list or a dictionary as a class attribute if each object must
> have its own: all the objects would share **the same** list. Create it in `__init__`
> with `self.tricks = []`.

## Properties with @property

Sometimes, a value is **computed** from other attributes. `@property` turns a method into
a read-only attribute: you use it without parentheses.

```python
class Rectangle:
    def __init__(self, width, height):
        self.width = width
        self.height = height

    @property
    def area(self):
        return self.width * self.height

r = Rectangle(3, 4)
print(r.area)
r.width = 10
print(r.area)
```

```sortie
12
40
```

The area is always up to date, because it is computed again on each read.

### Checking a change

With `@name.setter`, you can check a value before accepting it. By convention, the real
value is stored in an attribute that starts with `_`: it means "internal, do not touch
directly".

```python
class Thermostat:
    def __init__(self, temperature):
        self.temperature = temperature

    @property
    def temperature(self):
        return self._temperature

    @temperature.setter
    def temperature(self, value):
        if value < 5 or value > 30:
            raise ValueError(f"temperature out of range: {value}")
        self._temperature = value

t = Thermostat(20)
t.temperature = 22
print(t.temperature)
try:
    t.temperature = 80
except ValueError as e:
    print(e)
```

```sortie
22
temperature out of range: 80
```

`raise` and `try` are explained in [handling errors](erreurs.md).

## @classmethod and @staticmethod

Two other decorators change what a method receives:

| Decorator | First parameter | Use |
|-----------|-----------------|-----|
| none | `self`, the object | normal method |
| `@classmethod` | `cls`, the class | another way to **create** an object |
| `@staticmethod` | nothing | utility function stored in the class |

```python
class Date:
    def __init__(self, day, month, year):
        self.day = day
        self.month = month
        self.year = year

    @classmethod
    def from_text(cls, text):
        day, month, year = text.split("/")
        return cls(int(day), int(month), int(year))

    @staticmethod
    def is_leap(year):
        return year % 4 == 0 and (year % 100 != 0 or year % 400 == 0)

    def __repr__(self):
        return f"Date({self.day}, {self.month}, {self.year})"

d = Date.from_text("14/07/2024")
print(d)
print(Date.is_leap(2024))
```

```sortie
Date(14, 7, 2024)
True
```

`from_text` is an "alternative constructor": it prepares the values then calls
`cls(...)`, that is `Date(...)`.

## Objects are references

A variable does not **contain** the object: it holds a **reference** to it, like a label
stuck on it. Two variables can be two labels on the same object.

```python
class Account:
    def __init__(self, balance):
        self.balance = balance

a = Account(100)
b = a
b.balance = 0
print(a.balance)
print(a is b)
```

```sortie
0
True
```

`b = a` does not copy the account: `a` and `b` refer to the **same** object. It is the
same when you pass an object to a function: the function can change it.

```python
class Account:
    def __init__(self, balance):
        self.balance = balance

def empty(account):
    account.balance = 0

c = Account(100)
empty(c)
print(c.balance)
```

```sortie
0
```

For a real independent copy, use the `copy` module:

```python
import copy

class Account:
    def __init__(self, balance):
        self.balance = balance

a = Account(100)
b = copy.copy(a)
b.balance = 0
print(a.balance, b.balance)
```

```sortie
100 0
```

> **Note:** `is` answers "is it the same object?", whereas `==` answers "do they have the
> same value?". For your classes, `==` compares identity as long as you have not written
> `__eq__` (see [inheritance and polymorphism](heritage.md)).

## Common mistakes

### Forgetting self in a method

```python norun
class Dog:
    def bark():
        print("Woof!")

rex = Dog()
rex.bark()
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    rex.bark()
    ~~~~~~~~^^
TypeError: Dog.bark() takes 0 positional arguments but 1 was given
```

Python always passes the object as the first argument: "1 was given" is `rex`. Add
`self`: `def bark(self):`.

### Forgetting self. before an attribute

```python norun
class Dog:
    def __init__(self, name):
        name = name

rex = Dog("Rex")
print(rex.name)
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    print(rex.name)
          ^^^^^^^^
AttributeError: 'Dog' object has no attribute 'name'
```

`name = name` stores nothing in the object. Write `self.name = name`.

### Forgetting a constructor argument

```python norun
class Dog:
    def __init__(self, name):
        self.name = name

rex = Dog()
```

```text
Traceback (most recent call last):
  File "main.py", line 5, in <module>
    rex = Dog()
TypeError: Dog.__init__() missing 1 required positional argument: 'name'
```

The parameters of `__init__` (except `self`) are the ones you give to the class:
`Dog("Rex")`. Or give a default value: `def __init__(self, name="Unknown")`.

### Changing a property without a setter

```python norun
class Circle:
    def __init__(self, radius):
        self.radius = radius

    @property
    def diameter(self):
        return self.radius * 2

c = Circle(3)
c.diameter = 10
```

```text
Traceback (most recent call last):
  File "main.py", line 10, in <module>
    c.diameter = 10
    ^^^^^^^^^^
AttributeError: property 'diameter' of 'Circle' object has no setter
```

A property without `@diameter.setter` is read-only. Change `c.radius`, or add a setter
that computes `self.radius = value / 2`.

## Test yourself

**Question 1:** what is the difference between `self.name` and `Dog.count` in the `Dog`
class seen above?

> **Answer**
>
> `self.name` is an **object** attribute: each dog has its own. `Dog.count` is a
> **class** attribute: a single value, shared by all the dogs.

**Question 2:** what does this code display?

```python norun
class Box:
    def __init__(self):
        self.contents = []

a = Box()
b = a
b.contents.append("key")
print(a.contents)
```

> **Answer**
>
> `['key']`. `b = a` does not create a new box: `a` and `b` refer to the same object.

**Question 3:** write a `Square` class with a `side` attribute and a `perimeter`
property.

> **Answer**
>
> ```python
> class Square:
>     def __init__(self, side):
>         self.side = side
>
>     @property
>     def perimeter(self):
>         return 4 * self.side
>
> print(Square(5).perimeter)
> ```
>
> ```sortie
> 20
> ```

## Summary

- `class Name:` defines a type; `Name(...)` creates an object and calls `__init__`.
- `self` is the current object; `self.x = ...` creates an attribute that belongs to the
  object.
- `__str__` gives the display for `print`, `__repr__` the display for the developer.
- `@property` computes an attribute; `@classmethod` creates objects another way;
  `@staticmethod` stores a utility function.
- A variable is a reference: `b = a` does not copy the object.

Next: [inheritance and polymorphism](heritage.md). Compare with
[classes in Ruby](../ruby/types.md).
