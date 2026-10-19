# Going further with functions

This page shows more advanced uses of Python functions: a variable number of arguments, docstrings, functions passed as values, `lambda` and recursion.

> **Goals:**
>
> - you will be able to accept a variable number of arguments with `*args` and
>   `**kwargs`;
> - you will be able to write a `lambda` and give it to `sorted`;
> - you will be able to write a recursive function with a stopping case.

This chapter follows [functions](fonctions.md): you will meet `def`, parameters and `return` again, this time to go a little further.

## A variable number of arguments: *args and **kwargs

Sometimes you do not know in advance how many values you will receive. A restaurant
bill can list two dishes or ten. The stars let a function accept as many arguments as
needed.

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

Line by line:

- `*numbers` stores all the arguments received in a tuple: `(1, 2)`, then
  `(1, 2, 3, 4)`, then `()`;
- `result = 0` starts from zero;
- the `for` loop adds each number to `result`;
- `return result` sends back the sum. With no argument, the loop does not run: 0.

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

> **Note:** this part can wait. Read it once the basics feel solid.

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

## Functions are values

> **Note:** this part can wait. Read it once the basics feel solid.

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

Sometimes a function is used only once, for a tiny task, like telling `sorted` what
to sort by. Giving it a name with `def` would be heavy.

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

Line by line:

- `square = lambda x: x * x` creates a small function and stores it in `square`;
- `square(5)` computes `5 * 5` and returns 25;
- `students` is a list of pairs (name, grade);
- `sorted(..., key=lambda student: student[1])` looks at the grade of each pair, and
  orders the students from the lowest grade to the highest.

`key=lambda student: student[1]` means "sort by the second element". Sorting is
explained in detail in [lists and tuples](collections.md).

> **Tip:** a lambda is mostly used as an argument to another function. For everything
> else, a real `def` with a name is clearer.

**Your turn:** the list `friends = [("Lea", 25), ("Tom", 19), ("Ines", 31)]` holds
names and ages. Display it sorted by age, with `sorted` and a lambda.

> **Solution**
>
> ```python
> friends = [("Lea", 25), ("Tom", 19), ("Ines", 31)]
> print(sorted(friends, key=lambda friend: friend[1]))
> ```
>
> ```sortie
> [('Tom', 19), ('Lea', 25), ('Ines', 31)]
> ```

## A function that calls itself

Some problems contain a smaller version of themselves. Counting down from 3 means
saying "3", then counting down from 2. A function can follow the same idea.

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

Line by line:

- `countdown(3)`: `n` is 3, not 0. The function displays 3, then calls itself
  with 2;
- same thing with 2, then with 1: each call displays its number;
- with 0, the `if` is true: the function displays "Go!" and `return` stops
  everything. This is the stopping case.

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

### Forgetting the stopping case

```python norun
def countdown(n):
    print(n)
    countdown(n - 1)

countdown(3)
```

```text
3
2
1
0
-1
...
Traceback (most recent call last):
  File "/home/you/main.py", line 5, in <module>
    countdown(3)
    ~~~~~~~~~^^^
  File "/home/you/main.py", line 3, in countdown
    countdown(n - 1)
    ~~~~~~~~~^^^^^^^
  [Previous line repeated 996 more times]
RecursionError: maximum recursion depth exceeded
```

Without a stopping case, the function calls itself forever: 3, 2, 1, 0, -1… Python
stops it after about 1000 calls (*maximum recursion depth exceeded*). Add the stopping
case before the call: `if n == 0:` followed by `return`.

### Calling the function instead of passing it

```python norun
words = ["kiwi", "pineapple", "strawberry", "apple"]
print(sorted(words, key=len()))
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 2, in <module>
    print(sorted(words, key=len()))
                            ~~~^^
TypeError: len() takes exactly one argument (0 given)
```

With the parentheses, `len()` **calls** `len` right away, with no argument. To give
the function itself, write its name without parentheses: `key=len`.

### Giving keyword arguments to *args

```python norun
def total(*numbers):
    result = 0
    for n in numbers:
        result += n
    return result

print(total(a=1, b=2))
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 7, in <module>
    print(total(a=1, b=2))
          ~~~~~^^^^^^^^^^
TypeError: total() got an unexpected keyword argument 'a'
```

`*numbers` only receives **positional** arguments (*unexpected keyword argument*).
Write `total(1, 2)`, or add a `**kwargs` parameter to accept keyword arguments.

## Test yourself

**Question 1:** what does `args` receive in `def f(*args)` called with `f(1, 2, 3)`?

> **Answer**
>
> The tuple `(1, 2, 3)`. The star stores all the positional arguments in a tuple.

**Question 2:** rewrite `def triple(x): return x * 3` as a lambda.

> **Answer**
>
> ```python
> triple = lambda x: x * 3
> print(triple(4))
> ```
>
> ```sortie
> 12
> ```

**Question 3:** why does a recursive function need a stopping case?

> **Answer**
>
> Without a stopping case, it calls itself forever. Python ends up stopping it with a
> `RecursionError`.

## Summary

- `*args` stores the extra positional arguments in a tuple; `**kwargs` stores the
  extra keyword arguments in a dictionary.
- A docstring, right below the `def` line, explains what the function does.
- A function is a value: you can pass it to another one, or write it as a `lambda`.
- A recursive function calls itself and always has a stopping case.

Next: [working with text](texte.md).
