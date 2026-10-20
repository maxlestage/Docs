# Advanced functions and decorators

This page shows that Python functions are values like any other, which lets you write closures, decorators and use the tools of the `functools` module.

> **Goals:**
>
> - you will be able to store a function in a variable and pass it to another function;
> - you will know what a closure is;
> - you will be able to write a decorator step by step, with `functools.wraps`;
> - you will be able to write a decorator that takes arguments;
> - you will be able to use `lru_cache`, `partial`, `map`, `filter` and `reduce`.

## Functions are objects

In Python, a [function](../debuter/glossaire.md#function) is a value, like a number or
a string. Without parentheses, `greet` means **the function itself**; with
parentheses, `greet()` **runs it**.

```python
def greet(name):
    return f"Hello {name}"

say = greet
print(say("Alice"))
print(greet.__name__)
print(type(greet))
```

```sortie
Hello Alice
greet
<class 'function'>
```

### Passing a function as an argument

Since a function is a value, you can give it to another function. You have already
done it with `sorted(..., key=len)`.

```python
def apply(function, values):
    return [function(v) for v in values]

def double(x):
    return x * 2

print(apply(double, [1, 2, 3]))
print(apply(str.upper, ["a", "b"]))
print(apply(lambda x: x ** 2, [1, 2, 3]))
```

```sortie
[2, 4, 6]
['A', 'B']
[1, 4, 9]
```

`lambda x: x ** 2` is a [lambda](../debuter/glossaire.md#lambda): a small function
without a name, written on one line, that returns a single expression.

### Returning a function

A function can also **build** and return another function:

```python
def make_multiplier(factor):
    def multiply(x):
        return x * factor
    return multiply

triple = make_multiplier(3)
print(triple(5))
print(make_multiplier(10)(4))
```

```sortie
15
40
```

## Closures

In the previous example, `multiply` uses `factor`, a variable of the function that
created it. Even when `make_multiplier` has finished, `triple` **remembers** that
`factor` is 3. A function that carries variables with it like this is called a
[closure](../debuter/glossaire.md#closure).

It is a backpack: the inner function leaves with the variables it needs.

To **change** a carried variable, you have to say so with `nonlocal`:

```python
def make_counter():
    total = 0
    def count():
        nonlocal total
        total += 1
        return total
    return count

a = make_counter()
b = make_counter()
print(a(), a(), a())
print(b())
```

```sortie
1 2 3
1
```

Each call to `make_counter` creates a new backpack: `a` and `b` each have their own
`total`.

## Writing a decorator, step by step

A **decorator** is a function that takes a function and returns an improved version of
it. It is gift wrap: the gift stays the same, but you add something around it.

### Step 1: a function that wraps

Let's write a decorator that counts calls. It receives `function`, creates a function
`wrapper` that does some extra work then calls `function`, and returns `wrapper`.

```python
def count_calls(function):
    def wrapper(*args, **kwargs):
        wrapper.calls += 1
        return function(*args, **kwargs)
    wrapper.calls = 0
    return wrapper

def add(a, b):
    return a + b

add = count_calls(add)
print(add(2, 3))
print(add(10, 1))
print(add.calls)
```

```sortie
5
11
2
```

`*args, **kwargs` means "all the arguments, whatever they are": the wrapper passes them
on as they are. So it works with any function.

### Step 2: the @ syntax

`add = count_calls(add)` is so common that Python has a shortcut: you write
`@count_calls` just above the definition.

```python
def count_calls(function):
    def wrapper(*args, **kwargs):
        wrapper.calls += 1
        return function(*args, **kwargs)
    wrapper.calls = 0
    return wrapper

@count_calls
def greet(name):
    return f"Hi {name}"

greet("Alice")
greet("Bob")
greet("Chloé")
print(greet.calls)
```

```sortie
3
```

> **Remember:** `@decorator` above `def f` means exactly `f = decorator(f)`. Nothing
> more.

### A logging decorator

Another typical use: displaying each call and its result, to debug.

```python
def log_calls(function):
    def wrapper(*args, **kwargs):
        print(f"→ {function.__name__}{args}")
        result = function(*args, **kwargs)
        print(f"← {result!r}")
        return result
    return wrapper

@log_calls
def multiply(a, b):
    return a * b

multiply(6, 7)
```

```sortie
→ multiply(6, 7)
← 42
```

### A timing decorator

The same pattern can measure how long a function takes with `time.perf_counter()`. The
duration changes at each run, so the output below is only an example:

```python norun
import time

def timed(function):
    def wrapper(*args, **kwargs):
        start = time.perf_counter()
        result = function(*args, **kwargs)
        duration = time.perf_counter() - start
        print(f"{function.__name__}: {duration:.3f} s")
        return result
    return wrapper

@timed
def long_calculation():
    return sum(range(10_000_000))

long_calculation()
```

```text
long_calculation: 0.142 s
```

## functools.wraps

A small flaw: after decoration, the function is called `wrapper`. Its original name and
documentation are lost.

```python
def decorate(function):
    def wrapper(*args, **kwargs):
        return function(*args, **kwargs)
    return wrapper

@decorate
def greet():
    """Says hello."""

print(greet.__name__, greet.__doc__)
```

```sortie
wrapper None
```

`@functools.wraps(function)`, placed on the wrapper, copies the name, the documentation
and the other information of the original function. **Always** use it.

```python
import functools

def decorate(function):
    @functools.wraps(function)
    def wrapper(*args, **kwargs):
        return function(*args, **kwargs)
    return wrapper

@decorate
def greet():
    """Says hello."""

print(greet.__name__, greet.__doc__)
```

```sortie
greet Says hello.
```

## A decorator with arguments

To write `@repeat(3)`, you need one more level: `repeat(3)` must **return a
decorator**. You get three nested functions:

1. `repeat(times)` receives the argument and returns the decorator;
2. `decorator(function)` receives the function and returns the wrapper;
3. `wrapper(*args)` does the work.

```python
import functools

def repeat(times):
    def decorator(function):
        @functools.wraps(function)
        def wrapper(*args, **kwargs):
            for _ in range(times):
                function(*args, **kwargs)
        return wrapper
    return decorator

@repeat(3)
def shout(word):
    print(word.upper())

shout("hip")
```

```sortie
HIP
HIP
HIP
```

`@repeat(3)` is the same as `shout = repeat(3)(shout)`.

## lru_cache: remembering results

`functools.lru_cache` is a ready-made decorator: it keeps in memory the results already
computed. If the function is called again with the same arguments, the result is
given back immediately, without computing it again.

It is very effective for a [recursive](../debuter/glossaire.md#recursion) function
like Fibonacci, which keeps computing the same values over and over:

```python
from functools import lru_cache

calls = 0

@lru_cache(maxsize=None)
def fib(n):
    global calls
    calls += 1
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(80))
print(calls)
```

```sortie
23416728348467685
81
```

Only 81 computations, one per value of `n`. Without the cache, it would take billions
of billions.

> **Warning:** the cache only works if the arguments are *hashable*: numbers, strings,
> tuples… but not lists or dictionaries.

## partial: fixing arguments

`functools.partial` creates a new function where some arguments are **already filled
in**.

```python
from functools import partial

def power(base, exponent):
    return base ** exponent

square = partial(power, exponent=2)
cube = partial(power, exponent=3)
print(square(5), cube(2))

binary_to_int = partial(int, base=2)
print(binary_to_int("1010"))
```

```sortie
25 8
10
```

## map, filter, reduce or comprehensions?

Three classic functions apply a function to a collection:

- `map(f, values)` applies `f` to each value;
- `filter(f, values)` keeps the values for which `f` returns true;
- `reduce(f, values)` (from the `functools` module) combines the values two by two
  until only one is left.

```python
from functools import reduce

numbers = [1, 2, 3, 4, 5]
print(list(map(lambda x: x * 10, numbers)))
print(list(filter(lambda x: x % 2 == 0, numbers)))
print(reduce(lambda acc, x: acc * x, numbers))
```

```sortie
[10, 20, 30, 40, 50]
[2, 4]
120
```

`map` and `filter` return lazy iterators: you turn them into a list with `list`. In
Python, [comprehensions](comprehensions.md) often do the same thing in a more readable
way:

```python
numbers = [1, 2, 3, 4, 5]
print([x * 10 for x in numbers])
print([x for x in numbers if x % 2 == 0])
print(sum(numbers), max(numbers))
```

```sortie
[10, 20, 30, 40, 50]
[2, 4]
15 5
```

| Need | Most readable |
|------|---------------|
| transform with an existing function | `map(str, numbers)` or a comprehension |
| transform with a calculation | comprehension |
| filter | comprehension with `if` |
| add up, max, min | `sum`, `max`, `min` |
| other combination | `reduce`, or a loop |

## Common mistakes

### Forgetting to return the wrapper

```python norun
def announce(function):
    def wrapper():
        print("Calling!")
        function()

@announce
def greet():
    print("Hello")

greet()
```

```text
Traceback (most recent call last):
  File "main.py", line 10, in <module>
    greet()
    ~~~~~^^
TypeError: 'NoneType' object is not callable
```

`announce` returns nothing, so `greet` is `None`. Add `return wrapper` at the end of
the decorator.

### Changing a closure variable without nonlocal

```python norun
def make_counter():
    total = 0
    def add_one():
        total += 1
        return total
    return add_one

count = make_counter()
print(count())
```

```text
Traceback (most recent call last):
  File "main.py", line 9, in <module>
    print(count())
          ~~~~~^^
  File "main.py", line 4, in add_one
    total += 1
    ^^^^^
UnboundLocalError: cannot access local variable 'total' where it is not associated with a value
```

`total += 1` makes `total` a local variable of `add_one`, which has no value yet. Add
`nonlocal total` at the start of `add_one`.

### Forgetting the parentheses of a decorator with arguments

```python norun
def repeat(times):
    def decorator(function):
        def wrapper():
            for _ in range(times):
                function()
        return wrapper
    return decorator

@repeat
def greet():
    print("Hello")

greet()
```

```text
Traceback (most recent call last):
  File "main.py", line 13, in <module>
    greet()
    ~~~~~^^
TypeError: repeat.<locals>.decorator() missing 1 required positional argument: 'function'
```

Without parentheses, `greet` was given as the value of `times`, and `greet` became the
decorator itself. Write `@repeat(2)`.

### Caching a function that receives a list

```python norun
from functools import lru_cache

@lru_cache
def total(numbers):
    return sum(numbers)

print(total([1, 2, 3]))
```

```text
Traceback (most recent call last):
  File "main.py", line 7, in <module>
    print(total([1, 2, 3]))
          ~~~~~^^^^^^^^^^^
TypeError: unhashable type: 'list'
```

The cache stores the arguments as keys of a dictionary: a list cannot be one. Pass a
tuple: `total((1, 2, 3))`.

## Test yourself

**Question 1:** what is the difference between `f` and `f()`?

> **Answer**
>
> `f` is the function itself, a value you can store or pass. `f()` runs it and gives
> its result.

**Question 2:** what does this code do?

```python norun
@log_calls
def square(x):
    return x * x
```

> **Answer**
>
> It defines `square`, then replaces it with `log_calls(square)`: it is exactly
> `square = log_calls(square)`.

**Question 3:** write a decorator `uppercase` that puts the text returned by a function
in uppercase.

> **Answer**
>
> ```python
> import functools
>
> def uppercase(function):
>     @functools.wraps(function)
>     def wrapper(*args, **kwargs):
>         return function(*args, **kwargs).upper()
>     return wrapper
>
> @uppercase
> def greet(name):
>     return f"hello {name}"
>
> print(greet("zoé"))
> ```
>
> ```sortie
> HELLO ZOÉ
> ```

## Summary

- A function is a value: you can store it, pass it, return it.
- A closure remembers the variables of the function that created it; `nonlocal` lets
  you change them.
- A decorator takes a function and returns a wrapper; `@d` means `f = d(f)`.
- Always put `@functools.wraps(function)` on the wrapper.
- `lru_cache` remembers results, `partial` fixes arguments.
- Prefer comprehensions to `map` and `filter` when they are more readable.

Next: [handling errors](erreurs.md)
