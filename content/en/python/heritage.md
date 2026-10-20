# Inheritance and polymorphism

This page shows how a class can inherit from another one, how different objects can
respond to the same call, and how to make `==`, `<` or `len` work on your own classes.

> **Goals:**
>
> - you will be able to write a subclass and call the parent with `super()`;
> - you will be able to override a method and check a type with `isinstance`;
> - you will know what polymorphism and *duck typing* are;
> - you will be able to write an abstract class with `abc`;
> - you will be able to choose between inheritance and composition, and write `__eq__`,
>   `__lt__` and `__len__`.

## A subclass

[Inheritance](../debuter/glossaire.md#inheritance) lets you create a class from another
one. The new class, the **subclass** (or child class), receives everything the
**parent class** has: its attributes and its methods. You write it by putting the
parent in parentheses.

```python
class Animal:
    def __init__(self, name):
        self.name = name

    def eat(self):
        print(f"{self.name} is eating.")

class Dog(Animal):
    def bark(self):
        print(f"{self.name} barks: Woof!")

rex = Dog("Rex")
rex.eat()
rex.bark()
```

```sortie
Rex is eating.
Rex barks: Woof!
```

`Dog` did not write `__init__` or `eat`: it inherited them from `Animal`. It only adds
what is specific to it.

> **Remember:** inheritance expresses an "**is a**" relationship. A dog **is an**
> animal. If the sentence sounds wrong, inheritance is probably not the right tool.

## super(): calling the parent

If the subclass needs extra attributes, it writes its own `__init__`. To avoid copying
the parent's work, it calls it with `super()`.

```python
class Animal:
    def __init__(self, name):
        self.name = name

class Cat(Animal):
    def __init__(self, name, color):
        super().__init__(name)
        self.color = color

kitty = Cat("Kitty", "gray")
print(kitty.name, kitty.color)
```

```sortie
Kitty gray
```

`super().__init__(name)` means: "first do what the parent would do with `name`".
Then `Cat` adds `color`.

## Overriding a method

A subclass can **override** a method of the parent: it writes a method with the same
name, which replaces the parent's one for its objects.

```python
class Animal:
    def __init__(self, name):
        self.name = name

    def speak(self):
        return "..."

class Dog(Animal):
    def speak(self):
        return "Woof"

class Cat(Animal):
    def speak(self):
        return "Meow"

print(Animal("Beast").speak())
print(Dog("Rex").speak())
print(Cat("Kitty").speak())
```

```sortie
...
Woof
Meow
```

### Extending instead of replacing

With `super()`, an overridden method can also **reuse** the parent's one and add to
it.

```python
class Employee:
    def __init__(self, name, salary):
        self.name = name
        self.salary = salary

    def description(self):
        return f"{self.name}, {self.salary} €"

class Manager(Employee):
    def __init__(self, name, salary, team):
        super().__init__(name, salary)
        self.team = team

    def description(self):
        return super().description() + f", team of {len(self.team)}"

print(Manager("Alice", 4000, ["Bob", "Chloé"]).description())
```

```sortie
Alice, 4000 €, team of 2
```

## isinstance and issubclass

`isinstance(obj, Class)` answers "is this object of this class, **or of one of its
subclasses**?". `issubclass` asks the same question for two classes.

```python
class Animal:
    pass

class Dog(Animal):
    pass

rex = Dog()
print(isinstance(rex, Dog))
print(isinstance(rex, Animal))
print(isinstance(rex, str))
print(issubclass(Dog, Animal))
print(type(rex) == Animal)
```

```sortie
True
True
False
True
False
```

`type(rex) == Animal` is false: `type` gives the exact class, without looking at the
parents. Prefer `isinstance`.

> **Note:** all classes inherit from `object`, Python's base class. It is the one that
> provides the default `__repr__` and `==`.

## Polymorphism

"Polymorphism" means "many forms". The same call, `shape.area()`, gives a different
result depending on the object. The calling code does not need to know which class it
is dealing with.

```python
import math

class Circle:
    def __init__(self, radius):
        self.radius = radius

    def area(self):
        return math.pi * self.radius ** 2

class Rectangle:
    def __init__(self, width, height):
        self.width = width
        self.height = height

    def area(self):
        return self.width * self.height

shapes = [Circle(1), Rectangle(2, 3), Circle(2)]
for shape in shapes:
    print(type(shape).__name__, round(shape.area(), 2))
```

```sortie
Circle 3.14
Rectangle 6
Circle 12.57
```

It is like a remote control: the "on" button turns on the television, the radio or the
fan. Each device knows what "on" means for it.

### Duck typing

In the previous example, `Circle` and `Rectangle` have **no common parent**: they
just both have an `area` method. For Python, that is enough.

This is *duck typing*: "if it walks like a duck and it quacks like a duck, then it is a
duck". Python does not check the type of an object, only that it can do what you ask
it to do.

```python
class Duck:
    def quack(self):
        return "Quack quack"

class Robot:
    def quack(self):
        return "BEEP quack BEEP"

def make_quack(thing):
    print(thing.quack())

make_quack(Duck())
make_quack(Robot())
```

```sortie
Quack quack
BEEP quack BEEP
```

## Abstract classes

An **abstract class** is an incomplete model: it announces methods that each subclass
**must** write, but you cannot create an object directly from it. You write it with
the `abc` module (*abstract base classes*).

```python
from abc import ABC, abstractmethod

class Shape(ABC):
    @abstractmethod
    def area(self):
        pass

    def describe(self):
        return f"{type(self).__name__} with area {self.area()}"

class Square(Shape):
    def __init__(self, side):
        self.side = side

    def area(self):
        return self.side ** 2

print(Square(3).describe())
try:
    Shape()
except TypeError as e:
    print(e)
```

```sortie
Square with area 9
Can't instantiate abstract class Shape without an implementation for abstract method 'area'
```

`describe` is written only once, in `Shape`, and it calls `area`, which each subclass
provides. The abstract class guarantees that no subclass forgets `area`.

## Composition or inheritance?

Inheritance is not the only way to reuse code. **Composition** means **putting one
object inside another**. It expresses a "**has a**" relationship: a car **has an**
engine; it **is not** an engine.

```python
class Engine:
    def __init__(self, power):
        self.power = power

    def start(self):
        return f"Vroom ({self.power} hp)"

class Car:
    def __init__(self, brand, power):
        self.brand = brand
        self.engine = Engine(power)

    def start(self):
        return f"{self.brand}: {self.engine.start()}"

print(Car("Zoé", 90).start())
```

```sortie
Zoé: Vroom (90 hp)
```

| Question | Choice |
|----------|--------|
| Is "X is a Y" true? | inheritance |
| Is "X has a Y" true? | composition |
| Not sure? | composition, more flexible |

Composition lets you change one part without touching the rest: you can give the car an
electric engine without creating a new car class.

## Special methods for your classes

Methods surrounded by double underscores, like `__init__`, are called
**special methods** (or *dunder*, for *double underscore*). Python calls them on its
own when you use an operator or a built-in function.

| You write | Python calls |
|-----------|--------------|
| `a == b` | `a.__eq__(b)` |
| `a < b` | `a.__lt__(b)` |
| `len(a)` | `a.__len__()` |
| `str(a)`, `print(a)` | `a.__str__()` |
| `x in a` | `a.__contains__(x)` |

### Comparing with __eq__

By default, `==` between two objects checks whether they are the **same** object. With
`__eq__`, you decide what "equal" means.

```python
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __eq__(self, other):
        if not isinstance(other, Point):
            return NotImplemented
        return self.x == other.x and self.y == other.y

print(Point(1, 2) == Point(1, 2))
print(Point(1, 2) == Point(3, 4))
print(Point(1, 2) == "text")
```

```sortie
True
False
False
```

`NotImplemented` tells Python: "I don't know how to compare with that". Python then
tries something else, and ends up answering `False`.

### Sorting with __lt__

`sorted`, `min` and `max` use `<`. You only need to write `__lt__` (*less than*).

```python
class Card:
    def __init__(self, value, suit):
        self.value = value
        self.suit = suit

    def __lt__(self, other):
        return self.value < other.value

    def __repr__(self):
        return f"{self.value}{self.suit}"

hand = [Card(10, "♥"), Card(3, "♠"), Card(7, "♦")]
print(sorted(hand))
print(max(hand))
```

```sortie
[3♠, 7♦, 10♥]
10♥
```

### Measuring with __len__

`__len__` lets you use `len()`. As a bonus, an object of length 0 counts as false in an
`if`.

```python
class Basket:
    def __init__(self):
        self.items = []

    def add(self, item):
        self.items.append(item)

    def __len__(self):
        return len(self.items)

    def __contains__(self, item):
        return item in self.items

basket = Basket()
if not basket:
    print("Empty basket")
basket.add("bread")
basket.add("milk")
print(len(basket))
print("bread" in basket)
```

```sortie
Empty basket
2
True
```

> **Tip:** writing all these methods by hand quickly gets repetitive. *Dataclasses*
> generate them for you: see [type annotations](typage.md).

## Common mistakes

### Forgetting super().__init__

```python norun
class Animal:
    def __init__(self, name):
        self.name = name

class Cat(Animal):
    def __init__(self, name, color):
        self.color = color

kitty = Cat("Kitty", "gray")
print(kitty.name)
```

```text
Traceback (most recent call last):
  File "main.py", line 10, in <module>
    print(kitty.name)
          ^^^^^^^^^^
AttributeError: 'Cat' object has no attribute 'name'
```

The `__init__` of `Cat` replaces the one of `Animal`, which is therefore never called.
Add `super().__init__(name)` at the start of `Cat`'s `__init__`.

### Forgetting an abstract method

```python norun
from abc import ABC, abstractmethod

class Shape(ABC):
    @abstractmethod
    def area(self):
        pass

class Square(Shape):
    def __init__(self, side):
        self.side = side

s = Square(3)
```

```text
Traceback (most recent call last):
  File "main.py", line 12, in <module>
    s = Square(3)
TypeError: Can't instantiate abstract class Square without an implementation for abstract method 'area'
```

`Square` did not write `area`: it stays abstract. Add the method to `Square`.

### Sorting objects without __lt__

```python norun
class Card:
    def __init__(self, value):
        self.value = value

cards = [Card(7), Card(2)]
print(sorted(cards))
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    print(sorted(cards))
          ~~~~~~^^^^^^^
TypeError: '<' not supported between instances of 'Card' and 'Card'
```

Python does not know which card is the smallest. Write `__lt__`, or give a sort key:
`sorted(cards, key=lambda c: c.value)`.

### Calling len without __len__

```python norun
class Basket:
    def __init__(self):
        self.items = ["bread", "milk"]

print(len(Basket()))
```

```text
Traceback (most recent call last):
  File "main.py", line 5, in <module>
    print(len(Basket()))
          ~~~^^^^^^^^^^
TypeError: object of type 'Basket' has no len()
```

Add a `__len__` method that returns `len(self.items)`.

## Test yourself

**Question 1:** a `Bike` has two `Wheel`s. Inheritance or composition?

> **Answer**
>
> Composition: a bike **has** wheels, it **is** not a wheel. You would write
> `self.wheels = [Wheel(), Wheel()]` in `Bike`.

**Question 2:** what does `isinstance(True, int)` return? (Hint: `bool` is a subclass
of `int`.)

> **Answer**
>
> `True`. `isinstance` also accepts subclasses, and `bool` inherits from `int`.

**Question 3:** write a `Grade` class with a `value` attribute, that can be sorted with
`sorted`.

> **Answer**
>
> ```python
> class Grade:
>     def __init__(self, value):
>         self.value = value
>
>     def __lt__(self, other):
>         return self.value < other.value
>
>     def __repr__(self):
>         return f"Grade({self.value})"
>
> print(sorted([Grade(15), Grade(8), Grade(12)]))
> ```
>
> ```sortie
> [Grade(8), Grade(12), Grade(15)]
> ```

## Summary

- `class Child(Parent):` inherits the attributes and methods of the parent.
- `super()` calls the parent's version, especially in `__init__`.
- Polymorphism: the same call gives a behavior suited to each object; with *duck
  typing*, only the presence of the method matters.
- `ABC` and `@abstractmethod` force subclasses to write certain methods.
- Prefer composition ("has a") when "is a" does not sound right.
- `__eq__`, `__lt__` and `__len__` make `==`, `sorted` and `len` work on your objects.

Next: [type annotations](typage.md). Compare with
[inheritance in Ruby](../ruby/heritage.md).
