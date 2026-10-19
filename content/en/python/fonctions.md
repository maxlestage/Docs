# Functions

This page explains how to write functions in Python with `def`, give them parameters, and get their result back with `return`.

> **Goals:**
>
> - you will be able to write a function with `def` and call it;
> - you will be able to return a value with `return`;
> - you will be able to use default values, keyword arguments, `*args` and
>   `**kwargs`;
> - you will know which variables a function can see;
> - you will be able to write a `lambda` and a recursive function.

## Defining a function

A [function](../debuter/glossaire.md#function) is a named piece of code that you can
reuse. It is like a recipe: you write it once, then you follow it as many times as you
want. It starts with `def` (*define*), followed by the name, parentheses and a colon.
The body is indented.

```python
def greet():
    print("Hello!")

greet()
greet()
```

```sortie
Hello!
Hello!
```

Writing `def` does not run anything: it saves the recipe. Writing `greet()` **calls**
the function, that is, runs it. The parentheses are required, even when empty.

## Parameters

[Parameters](../debuter/glossaire.md#parameter) are the values the function receives.
They are empty slots, filled in at the time of the call. The values given in the call
are called [arguments](../debuter/glossaire.md#argument).

```python
def greet(first_name):
    print(f"Hello, {first_name}!")

greet("Alice")
greet("Bob")
```

```sortie
Hello, Alice!
Hello, Bob!
```

Here, `first_name` is the parameter; `"Alice"` and `"Bob"` are the arguments. Several
parameters are separated by commas.

## Returning a value with return

`return` sends a result back to the code that called the function, and leaves the
function.

```python
def area(width, height):
    return width * height

print(area(3, 4))
total = area(2, 5) + area(1, 1)
print(total)
```

```sortie
12
11
```

The returned value **replaces** the call: `area(2, 5) + area(1, 1)` becomes `10 + 1`.

`return` can also be used to leave early:

```python
def divide(a, b):
    if b == 0:
        return "division impossible"
    return a / b

print(divide(10, 4))
print(divide(10, 0))
```

```sortie
2.5
division impossible
```

### Display or return?

`print` **displays** a value on the screen. `return` **gives** it to the calling code,
which can use it. A function without `return` returns `None`.

```python
def print_double(x):
    print(x * 2)

def double(x):
    return x * 2

result1 = print_double(5)
result2 = double(5)
print(result1, result2)
```

```sortie
10
None 10
```

> **Remember:** prefer functions that return a value. You can then choose to display
> it, store it or reuse it.

### Returning several values

Separate the values with commas. Python groups them into a
[tuple](../debuter/glossaire.md#tuple), which you can split into several variables.

```python
def min_max(numbers):
    return min(numbers), max(numbers)

small, big = min_max([4, 9, 1])
print(small, big)
```

```sortie
1 9
```

## Default values

A parameter can have a default value. It is used if the argument is missing.

```python
def greet(first_name, greeting="Hello"):
    return f"{greeting}, {first_name}!"

print(greet("Alice"))
print(greet("Bob", "Hi"))
```

```sortie
Hello, Alice!
Hi, Bob!
```

Parameters with a default value go **after** the others.

## Keyword arguments

When calling, you can give an argument by its **name**: `name=value`. The order then no
longer matters, and the call explains itself.

```python
def order(product, quantity=1, urgent=False):
    text = f"{quantity} x {product}"
    if urgent:
        text += " (urgent)"
    return text

print(order("coffee"))
print(order("tea", urgent=True))
print(order(quantity=3, product="croissant"))
```

```sortie
1 x coffee
1 x tea (urgent)
3 x croissant
```

> **Tip:** name boolean arguments. `order("tea", urgent=True)` is clear right away;
> `order("tea", 1, True)` much less so.

## A variable number of arguments: *args and **kwargs

With one star, `*args` receives all the extra **positional** arguments, stored in a
tuple.

```python
def total(*numbers):
    result = 0
    for n in numbers:
        result += n
    return result

print(total(1, 2))
print(total(1, 2, 3, 4))
print(total())
```

```sortie
3
10
0
```

With two stars, `**kwargs` (*keyword arguments*) receives all the extra **named**
arguments, stored in a [dictionary](../debuter/glossaire.md#dictionary).

```python
def profile(name, **info):
    print(f"Profile of {name}")
    for key, value in info.items():
        print(f"  {key}: {value}")

profile("Alice", age=30, city="Lyon")
```

```sortie
Profile of Alice
  age: 30
  city: Lyon
```

The names `args` and `kwargs` are a convention: only the stars matter. Dictionaries are
explained in [dictionaries and sets](dictionnaires.md).

## Documenting with a docstring

A **docstring** is a text between triple quotes, placed right below the `def` line. It
explains what the function does. Editors show it when you hover over the name, and
`help()` displays it.

```python
def price_with_tax(price, rate=0.2):
    """Returns the price including tax, rounded to the cent."""
    return round(price * (1 + rate), 2)

print(price_with_tax(10))
print(price_with_tax.__doc__)
```

```sortie
12.0
Returns the price including tax, rounded to the cent.
```

## Variable scope

The [scope](../debuter/glossaire.md#scope) of a variable is the area of the code where
it exists. A variable created **inside** a function is **local**: it is born when the
function is called and disappears at the end.

```python
def calculate():
    result = 42
    return result

print(calculate())
print("result" in globals())
```

```sortie
42
False
```

After the call, `result` no longer exists outside the function.

A function can **read** a variable from outside (a **global** variable). But if it gives
it a value, Python creates a new local variable. To really change the global variable,
you need the word `global`:

```python
counter = 0

def increment():
    global counter
    counter += 1

increment()
increment()
print(counter)
```

```sortie
2
```

> **Warning:** `global` makes code hard to follow. Prefer passing the value as an
> argument and returning the result: `counter = increment(counter)`.

## Functions are values

In Python, a function is a value like any other. Without parentheses, its name refers
to the function itself: you can store it in a variable or pass it to another function.

```python
def shout(text):
    return text.upper() + "!"

def whisper(text):
    return text.lower() + "..."

def speak(text, style):
    return style(text)

print(speak("Hello", shout))
print(speak("Hello", whisper))
```

```sortie
HELLO!
hello...
```

`speak` receives a function (`style`) and calls it. This is very useful for sorting,
for example: `sorted` accepts a function that says what to sort by.

```python
words = ["kiwi", "pineapple", "strawberry", "apple"]
print(sorted(words, key=len))
```

```sortie
['kiwi', 'apple', 'pineapple', 'strawberry']
```

## Lambdas

A [lambda](../debuter/glossaire.md#lambda) is a small function without a name, written
on one line: `lambda parameters: expression`. It automatically returns the value of the
expression.

```python
square = lambda x: x * x
print(square(5))

students = [("Alice", 15), ("Bob", 12), ("Chloe", 18)]
by_grade = sorted(students, key=lambda student: student[1])
print(by_grade)
```

```sortie
25
[('Bob', 12), ('Alice', 15), ('Chloe', 18)]
```

`key=lambda student: student[1]` means "sort by the second element". Sorting is
explained in detail in [lists and tuples](collections.md).

> **Tip:** a lambda is mostly used as an argument to another function. For everything
> else, a real `def` with a name is clearer.

## A function that calls itself

A function can call itself: this is
[recursion](../debuter/glossaire.md#recursion). It needs a **stopping case**, otherwise
it never stops.

```python
def countdown(n):
    if n == 0:
        print("Go!")
        return
    print(n)
    countdown(n - 1)

countdown(3)
```

```sortie
3
2
1
Go!
```

A classic example is the factorial: `5!` equals `5 × 4 × 3 × 2 × 1`. In other words,
`n!` equals `n × (n - 1)!`, and `1!` equals 1.

```python
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

print(factorial(5))
```

```sortie
120
```

> **Note:** Python limits how deep calls can go (about 1000). For very long
> repetitions, a loop is better.

## Common mistakes

### Forgetting an argument

```python norun
def area(width, height):
    return width * height

print(area(3))
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 4, in <module>
    print(area(3))
          ~~~~^^^
TypeError: area() missing 1 required positional argument: 'height'
```

The `height` argument is missing (*missing 1 required argument*). Give all of them:
`area(3, 4)`. Or give the parameter a default value.

### Calling the function before defining it

```python norun
greet("Alice")

def greet(name):
    print(f"Hello, {name}!")
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 1, in <module>
    greet("Alice")
    ^^^^^
NameError: name 'greet' is not defined
```

Python reads the file from top to bottom. On line 1, `greet` does not exist yet. Put
the `def` before the call.

### Changing a global variable without global

```python norun
counter = 0

def increment():
    counter = counter + 1

increment()
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 6, in <module>
    increment()
    ~~~~~~~~~^^
  File "/home/you/main.py", line 4, in increment
    counter = counter + 1
              ^^^^^^^
UnboundLocalError: cannot access local variable 'counter' where it is not associated with a value
```

Since the function gives `counter` a value, Python treats it as local. But it does not
have a value yet when calculating `counter + 1`. Pass the value as a parameter and
return the result, or add `global counter`.

### A list as a default value

```python norun
def add(item, cart=[]):
    cart.append(item)
    return cart

print(add("bread"))
print(add("milk"))
```

```text
['bread']
['bread', 'milk']
```

No error, but a surprise: the second cart still contains the bread. The default value
is created **only once**, when Python reads the `def`, then shared between all calls.
Use `None`:

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

## Test yourself

**Question 1:** what does this code display?

```python norun
def f(x):
    x + 1

print(f(5))
```

> **Answer**
>
> `None`. The function calculates `x + 1` but does not return it: `return` is missing.

**Question 2:** write a function `average(a, b)` that returns the average of two
numbers.

> **Answer**
>
> ```python
> def average(a, b):
>     return (a + b) / 2
>
> print(average(3, 4))
> ```
>
> ```sortie
> 3.5
> ```

**Question 3:** what does `args` receive in `def f(*args)` called with `f(1, 2, 3)`?

> **Answer**
>
> The tuple `(1, 2, 3)`. The star stores all the positional arguments in a tuple.

## Summary

- `def name(parameters):` defines a function; `name(arguments)` calls it.
- `return` returns a value and leaves the function; without `return`, it returns
  `None`.
- Parameters can have a default value; arguments can be named; `*args` and `**kwargs`
  accept a variable number of arguments.
- A function's variables are local; avoid `global`.
- A function is a value: you can pass it to another one, or write it as a `lambda`.
- A recursive function calls itself and always has a stopping case.

Next: [working with text](texte.md). Compare with
[functions in Ruby](../ruby/fonctions.md).
