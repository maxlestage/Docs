# Exercises with solutions

This page offers 12 exercises with solutions, from easiest to hardest, to practice the whole Python track.

## How to use this page

Each exercise has a level:

- ★☆☆: easy;
- ★★☆: medium;
- ★★★: harder.

Read the instructions, then write your solution in a `.py` file and run it with
`python3`. Open the solution only after you have tried.

> **Tip:** there are often several good solutions. If your program prints the right
> result, you win!

## The basics

### Exercise 1: Introduce yourself

Level: ★☆☆

Create two variables, `first_name` and `age`. Then print the sentence
`My name is Alice and I am 30 years old.`, using your variables.

> **Hint:** use an f-string: `f"... {first_name} ..."` (see
> [first steps](premiers-pas.md)).

> **Solution**
>
> ```python
> first_name = "Alice"
> age = 30
> print(f"My name is {first_name} and I am {age} years old.")
> ```
>
> ```sortie
> My name is Alice and I am 30 years old.
> ```

### Exercise 2: Even or odd

Level: ★☆☆

For the number `7`, print `7 is odd`. For `10`, the program must print
`10 is even`.

> **Hint:** `n % 2` gives the remainder of the division by 2: `0` if `n` is even (see
> [numbers](nombres.md)).

> **Solution**
>
> ```python
> for n in [7, 10]:
>     if n % 2 == 0:
>         print(f"{n} is even")
>     else:
>         print(f"{n} is odd")
> ```
>
> ```sortie
> 7 is odd
> 10 is even
> ```

### Exercise 3: The 7 times table

Level: ★☆☆

Print the multiplication table of 7, from `7 x 1 = 7` to `7 x 5 = 35`.

> **Hint:** `range(1, 6)` gives the numbers from 1 to 5 (the 6 is excluded).

> **Solution**
>
> ```python
> for i in range(1, 6):
>     print(f"7 x {i} = {7 * i}")
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

### Exercise 4: The area of a rectangle

Level: ★☆☆

Write a function `area(width, height)` that returns the area of a rectangle. If the
height is not given, the rectangle is a square: the height equals the width.
Print `area(3, 4)` and `area(5)`.

> **Hint:** give `height` the default value `None`, then test it (see
> [functions](fonctions.md)).

> **Solution**
>
> ```python
> def area(width, height=None):
>     if height is None:
>         height = width
>     return width * height
>
>
> print(area(3, 4))
> print(area(5))
> ```
>
> ```sortie
> 12
> 25
> ```

## Text and collections

### Exercise 5: The average

Level: ★☆☆

Write a function `average(grades)` that returns the average of a list, rounded to one
decimal place, or `None` if the list is empty.

> **Hint:** `sum`, `len` and `round` do all the work (see
> [collections](collections.md)).

> **Solution**
>
> ```python
> def average(grades):
>     if not grades:
>         return None
>     return round(sum(grades) / len(grades), 1)
>
>
> print(average([12, 15, 17]))
> print(average([]))
> ```
>
> ```sortie
> 14.7
> None
> ```

### Exercise 6: Counting vowels

Level: ★★☆

Write a function `vowels(text)` that counts the vowels (`a e i o u`) in a text,
ignoring uppercase.

> **Hint:** `text.lower()` puts everything in lowercase, and `letter in "aeiou"` tests
> whether a letter is a vowel (see [working with text](texte.md)).

> **Solution**
>
> ```python
> def vowels(text):
>     return sum(1 for letter in text.lower() if letter in "aeiou")
>
>
> print(vowels("Hello"))
> print(vowels("PYTHON"))
> ```
>
> ```sortie
> 2
> 1
> ```

### Exercise 7: The palindrome

Level: ★★☆

A palindrome reads the same both ways, like `"Radar"` or `"Never odd or even"`. Write
`is_palindrome(text)`, which ignores uppercase and spaces.

> **Hint:** `text[::-1]` returns the text backwards; `replace(" ", "")` removes the
> spaces.

> **Solution**
>
> ```python
> def is_palindrome(text):
>     clean = text.lower().replace(" ", "")
>     return clean == clean[::-1]
>
>
> print(is_palindrome("Radar"))
> print(is_palindrome("Never odd or even"))
> print(is_palindrome("Python"))
> ```
>
> ```sortie
> True
> True
> False
> ```

### Exercise 8: Counting words

Level: ★★☆

Count how many times each word appears in the sentence
`"the cat and the dog and the rabbit"`, then print the words from most frequent to
least frequent.

> **Hint:** a [dictionary](dictionnaires.md) maps each word to its counter;
> `counts.get(word, 0)` is 0 for a new word. `collections.Counter` does the same thing
> in one line.

> **Solution**
>
> ```python
> from collections import Counter
>
> sentence = "the cat and the dog and the rabbit"
>
> counts = {}
> for word in sentence.split():
>     counts[word] = counts.get(word, 0) + 1
>
> for word, number in sorted(counts.items(), key=lambda pair: -pair[1]):
>     print(f"{word}: {number}")
>
> print(Counter(sentence.split()).most_common(2))
> ```
>
> ```sortie
> the: 3
> and: 2
> cat: 1
> dog: 1
> rabbit: 1
> [('the', 3), ('and', 2)]
> ```

### Exercise 9: Keep and double

Level: ★★☆

From the list `[3, -1, 4, 0, -5, 9]`, build in a single line the list of positive
numbers, doubled. Also build a dictionary that maps each positive number to its square.

> **Hint:** a [comprehension](comprehensions.md) filters with `if` and transforms at the
> same time.

> **Solution**
>
> ```python
> numbers = [3, -1, 4, 0, -5, 9]
>
> print([n * 2 for n in numbers if n > 0])
> print({n: n * n for n in numbers if n > 0})
> ```
>
> ```sortie
> [6, 8, 18]
> {3: 9, 4: 16, 9: 81}
> ```

## Going further

### Exercise 10: FizzBuzz

Level: ★★☆

For the numbers from 1 to 15, print `Fizz` if the number is divisible by 3, `Buzz` if
it is divisible by 5, `FizzBuzz` if it is divisible by both, and the number otherwise.
Print everything on one line, separated by spaces.

> **Hint:** test "divisible by 15" first, otherwise this case never happens.

> **Solution**
>
> ```python
> def fizzbuzz(n):
>     if n % 15 == 0:
>         return "FizzBuzz"
>     if n % 3 == 0:
>         return "Fizz"
>     if n % 5 == 0:
>         return "Buzz"
>     return str(n)
>
>
> print(" ".join(fizzbuzz(n) for n in range(1, 16)))
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```

### Exercise 11: The bank account

Level: ★★★

Write a class `Account` with an `owner` and a `balance` (0 at the start), and two
methods: `deposit(amount)` and `withdraw(amount)`. A negative or zero amount raises a
`ValueError`, and so does a withdrawal bigger than the balance. Test with `try` /
`except`.

> **Hint:** see [classes](types.md) and [errors](erreurs.md). `raise
> ValueError("message")` raises the error.

> **Solution**
>
> ```python
> class Account:
>     def __init__(self, owner):
>         self.owner = owner
>         self.balance = 0
>
>     def deposit(self, amount):
>         if amount <= 0:
>             raise ValueError("the amount must be positive")
>         self.balance += amount
>
>     def withdraw(self, amount):
>         if amount <= 0:
>             raise ValueError("the amount must be positive")
>         if amount > self.balance:
>             raise ValueError("insufficient funds")
>         self.balance -= amount
>
>     def __str__(self):
>         return f"{self.owner}: {self.balance} €"
>
>
> account = Account("Alice")
> account.deposit(100)
> account.withdraw(30)
> print(account)
>
> for amount in [-5, 500]:
>     try:
>         account.withdraw(amount)
>     except ValueError as error:
>         print(f"Refused ({amount}): {error}")
>
> print(account)
> ```
>
> ```sortie
> Alice: 70 €
> Refused (-5): the amount must be positive
> Refused (500): insufficient funds
> Alice: 70 €
> ```

### Exercise 12: An inventory in JSON

Level: ★★★

A shop describes its stock in JSON:
`[{"name": "apple", "price": 0.5, "quantity": 40}, {"name": "kiwi", "price": 0.8,
"quantity": 0}, {"name": "pear", "price": 0.7, "quantity": 12}]`.

Read this text, turn each item into an `Item` `dataclass`, then print the items in
stock, sorted by name, with their value (`price * quantity`). End with the total value
of the stock.

> **Hint:** `json.loads` turns the text into a list of dictionaries (see
> [data formats](formats.md)); `Item(**d)` creates an object from a dictionary whose
> keys are the names of the fields.

> **Solution**
>
> ```python
> import json
> from dataclasses import dataclass
>
> TEXT = """[
>   {"name": "apple", "price": 0.5, "quantity": 40},
>   {"name": "kiwi", "price": 0.8, "quantity": 0},
>   {"name": "pear", "price": 0.7, "quantity": 12}
> ]"""
>
>
> @dataclass
> class Item:
>     name: str
>     price: float
>     quantity: int
>
>     def value(self) -> float:
>         return self.price * self.quantity
>
>
> items = [Item(**d) for d in json.loads(TEXT)]
> in_stock = sorted((i for i in items if i.quantity > 0), key=lambda i: i.name)
>
> for item in in_stock:
>     print(f"{item.name:<6} {item.quantity:>3} x {item.price:.2f} = {item.value():6.2f}")
>
> total = sum(item.value() for item in items)
> print(f"Stock value: {total:.2f} €")
> ```
>
> ```sortie
> apple   40 x 0.50 =  20.00
> pear    12 x 0.70 =   8.40
> Stock value: 28.40 €
> ```

## Summary

- Start with the ★☆☆ exercises, then move up in difficulty.
- Always try on your own before opening the solution.
- Exercises 1 to 4 go over the basics: variables, conditions, loops, functions.
- Exercises 5 to 9 work on text, lists, dictionaries and comprehensions.
- Exercises 10 to 12 combine everything, with classes, errors, dataclasses and JSON.

Next: [exercises with solutions: second series](exercices-2.md)
