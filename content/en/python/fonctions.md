# Functions

This page explains how to write functions in Python with `def`, give them parameters, get their result back with `return`, and know which variables they can see.

> **Goals:**
>
> - you will be able to write a function with `def` and call it;
> - you will be able to return a value with `return`;
> - you will be able to use default values and keyword arguments;
> - you will know which variables a function can see.

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

Line by line:

- `def greet():` creates a function named `greet`. Nothing is displayed yet;
- `print("Hello!")` is indented: it belongs to the function, so it waits;
- the first `greet()` runs the function: Python executes its body and displays
  "Hello!";
- the second `greet()` does it again: the same recipe, followed a second time.

Writing `def` does not run anything: it saves the recipe. Writing `greet()` **calls**
the function, that is, runs it. The parentheses are required, even when empty.

## Parameters

A pancake recipe can serve 4 or 8 people: only the quantities change. In the same
way, a function is more useful when it can work with different values at each call.

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

Line by line:

- `def greet(first_name):` creates the function, with an empty slot named
  `first_name`;
- `greet("Alice")` calls the function: `first_name` receives `"Alice"`, then `print`
  displays "Hello, Alice!";
- `greet("Bob")` calls it again: this time, `first_name` is `"Bob"`.

Here, `first_name` is the parameter; `"Alice"` and `"Bob"` are the arguments. Several
parameters are separated by commas.

**Your turn:** write a function `birthday(first_name)` that displays "Happy birthday,
…!" with the name it receives. Call it for Lea, then for Tom.

> **Solution**
>
> ```python
> def birthday(first_name):
>     print(f"Happy birthday, {first_name}!")
>
> birthday("Lea")
> birthday("Tom")
> ```
>
> ```sortie
> Happy birthday, Lea!
> Happy birthday, Tom!
> ```

## Returning a value with return

Often, you want a function to do a calculation and **give** you the result, like a
calculator. You can then display it, store it in a variable or reuse it in another
calculation.

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

Line by line:

- `def area(width, height):` creates a function that receives two numbers;
- `return width * height` computes the product and sends it back to the caller;
- `print(area(3, 4))`: the call is worth 12, so Python displays 12;
- `total = area(2, 5) + area(1, 1)`: the two calls are worth 10 and 1, and `total`
  receives 11.

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

**Your turn:** write a function `perimeter(width, height)` that **returns** the
perimeter of a rectangle (twice the sum of the sides). Display `perimeter(3, 4)`.

> **Solution**
>
> ```python
> def perimeter(width, height):
>     return 2 * (width + height)
>
> print(perimeter(3, 4))
> ```
>
> ```sortie
> 14
> ```

## Default values

At a café, you get a regular coffee unless you ask for something else. A default value
plays that role: it saves you from repeating the most common value.

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

Line by line:

- `greeting="Hello"` gives the parameter `greeting` a fallback value;
- `greet("Alice")` gives only one argument: `greeting` therefore takes `"Hello"`;
- `greet("Bob", "Hi")` gives both: `"Hi"` replaces the default value.

Parameters with a default value go **after** the others.

**Your turn:** write a function `price(amount, currency="euros")` that returns a text
like "5 euros". Display `price(5)`, then `price(8, "dollars")`.

> **Solution**
>
> ```python
> def price(amount, currency="euros"):
>     return f"{amount} {currency}"
>
> print(price(5))
> print(price(8, "dollars"))
> ```
>
> ```sortie
> 5 euros
> 8 dollars
> ```

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

Line by line:

- `order("coffee")`: only `product` is given; `quantity` is 1 and `urgent` is
  `False`;
- `order("tea", urgent=True)`: `urgent` is given by its name, without touching
  `quantity`;
- `order(quantity=3, product="croissant")`: both arguments are named, so the order
  does not matter.

> **Tip:** name boolean arguments. `order("tea", urgent=True)` is clear right away;
> `order("tea", 1, True)` much less so.

## Variable scope

A function has its own scratch paper. What it writes there while it works does not
spill over into the rest of the program. This way, two functions can use the same
variable name without getting in each other's way.

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

Line by line:

- `result = 42` creates the variable `result` **inside** the function;
- `print(calculate())` calls the function, which returns 42: Python displays it;
- `globals()` gives the variables from outside. `"result" in globals()` asks whether
  `result` is one of them: the answer is `False`.

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

**Question 3:** with `def greet(first_name, greeting="Hello")`, what is
`greeting` in the call `greet("Alice")`?

> **Answer**
>
> `"Hello"`. The argument is missing, so Python uses the default value.

## Summary

- `def name(parameters):` defines a function; `name(arguments)` calls it.
- `return` returns a value and leaves the function; without `return`, it returns
  `None`.
- Parameters can have a default value; arguments can be named.
- A function's variables are local; avoid `global`.

Next: [going further with functions](fonctions-avancees.md). Compare with
[functions in Ruby](../ruby/fonctions.md).
