# Exercises to get started

This page offers 15 very guided exercises, all easy, to practice the basics of Python: printing, calculating, choosing, repeating, writing functions and handling lists and dictionaries.

## How to do these exercises

Each exercise is small. It asks for only one new idea. For each one:

1. **Read** the instructions and the expected output. This is what your program must
   print.
2. **Try on paper.** Write the steps in plain English: "first I…, then I…".
3. **Write the code** in a file `exercise.py`, then run `python3 exercise.py`.
4. **Compare** what is printed with the expected output, character by character.
5. **Stuck?** Read the first hint. Still stuck? The second one. The solution comes
   last.

> **Tip:** a red error is not a failure. Read the last line of the message: it often
> says what is wrong. The page [reading an error](../debuter/lire-une-erreur.md) helps
> you.

There are often several good solutions. If your program prints exactly the expected
output, you win.

## Printing and calculating

### Exercise 1: A business card

Level: ★☆☆

Store your first name and your city in two variables. Then print two lines, as shown
below.

```text
First name: Alice
City: London
```

> **Hint:** a [variable](../debuter/glossaire.md#variable) is created with `=`:
> `first_name = "Alice"`. Don't forget the quotes around the text.

> **Hint:** an f-string sticks text and a variable together:
> `f"First name: {first_name}"`.

> **Solution**
>
> ```python
> first_name = "Alice"
> city = "London"
> print(f"First name: {first_name}")
> print(f"City: {city}")
> ```
>
> ```sortie
> First name: Alice
> City: London
> ```
>
> Why it works: in an f-string, Python replaces `{first_name}` with what the box
> `first_name` contains. Each `print` prints one line.

### Exercise 2: The total price

Level: ★☆☆

A notebook costs 4 euros. You buy 3 of them. Store the price and the quantity in two
variables, calculate the total, then print it.

```text
Total: 12 euros
```

> **Hint:** the multiplication sign is `*`.

> **Hint:** create a third variable: `total = price * quantity`.

> **Solution**
>
> ```python
> price = 4
> quantity = 3
> total = price * quantity
> print(f"Total: {total} euros")
> ```
>
> ```sortie
> Total: 12 euros
> ```
>
> Why it works: Python first calculates `price * quantity`, that is `4 * 3`, then
> stores `12` in `total`.

### Exercise 3: From minutes to hours

Level: ★☆☆

A movie lasts 135 minutes. Print its length in hours and minutes.

```text
2 h 15 min
```

> **Hint:** in 135 minutes, how many times 60? That is integer division `//`
> (see [numbers](nombres.md)).

> **Hint:** what is left after removing the hours is the remainder `%`:
> `135 % 60`.

> **Solution**
>
> ```python
> length = 135
> hours = length // 60
> minutes = length % 60
> print(f"{hours} h {minutes} min")
> ```
>
> ```sortie
> 2 h 15 min
> ```
>
> Why it works: `135 // 60` is `2` (two full packs of 60). There are
> `135 - 120 = 15` minutes left, and that is exactly `135 % 60`.

## Choosing

### Exercise 4: Adult or minor

Level: ★☆☆

Store the age `15` in a variable. Print `minor` if the age is less than 18, otherwise
`adult`.

```text
minor
```

> **Hint:** use `if` and `else` (see
> [conditions and loops](conditions-boucles.md)). Don't forget the colons and the
> 4 spaces.

> **Solution**
>
> ```python
> age = 15
> if age < 18:
>     print("minor")
> else:
>     print("adult")
> ```
>
> ```sortie
> minor
> ```
>
> Why it works: `15 < 18` is `True`, so Python runs the `if` block and skips the
> `else` block.
>
> Try it: set `age = 18`. The program prints `adult`, because `18 < 18` is false.

### Exercise 5: The weather

Level: ★☆☆

Store a temperature in a variable. Print `cold` below 10 degrees, `mild` from 10 to 24,
and `hot` from 25 up. Test with `18`.

```text
mild
```

> **Hint:** three cases, so `if`, `elif` and `else`.

> **Hint:** Python tests the conditions in order and stops at the first true one. Start
> with `if temperature < 10:`.

> **Solution**
>
> ```python
> temperature = 18
> if temperature < 10:
>     print("cold")
> elif temperature < 25:
>     print("mild")
> else:
>     print("hot")
> ```
>
> ```sortie
> mild
> ```
>
> Why it works: `18 < 10` is false, so Python moves on to the `elif`. `18 < 25` is
> true: it prints `mild` and ignores the rest.

## Repeating

### Exercise 6: Counting to 5

Level: ★☆☆

Print the numbers from 1 to 5, one per line.

```text
1
2
3
4
5
```

> **Hint:** a `for` loop with `range`.

> **Hint:** `range(1, 6)` gives 1, 2, 3, 4, 5. The 6 is left out.

> **Solution**
>
> ```python
> for i in range(1, 6):
>     print(i)
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> ```
>
> Why it works: at each turn, `i` takes the next value of `range(1, 6)`, and
> `print(i)` prints it.

### Exercise 7: The sum from 1 to 10

Level: ★☆☆

Calculate `1 + 2 + 3 + … + 10` with a loop, then print the result.

```text
55
```

> **Hint:** create a variable `total = 0` **before** the loop. It is an empty piggy
> bank.

> **Hint:** in the loop, add `i` to the piggy bank: `total = total + i`.

> **Solution**
>
> ```python
> total = 0
> for i in range(1, 11):
>     total = total + i
> print(total)
> ```
>
> ```sortie
> 55
> ```
>
> Why it works: here are the first turns.
>
> | Turn | `i` | `total` after the turn |
> |---|---|---|
> | 1 | 1 | 1 |
> | 2 | 2 | 3 |
> | 3 | 3 | 6 |
> | … | … | … |
> | 10 | 10 | 55 |
>
> The `print` is **outside** the loop (no spaces in front): it prints only once, at the
> end.

### Exercise 8: The countdown

Level: ★☆☆

With a `while` loop, print 3, 2, 1, then `Go!`.

```text
3
2
1
Go!
```

> **Hint:** start from `n = 3`. The loop runs as long as `n > 0`.

> **Hint:** in the loop, print `n`, then subtract 1: `n = n - 1`. Without this line,
> the loop never stops.

> **Solution**
>
> ```python
> n = 3
> while n > 0:
>     print(n)
>     n = n - 1
> print("Go!")
> ```
>
> ```sortie
> 3
> 2
> 1
> Go!
> ```
>
> Why it works: `n` goes from 3 to 2, then 1, then 0. When `n` is 0, `n > 0` is false:
> the loop stops and Python moves on to the next line.

## Functions

### Exercise 9: Saying hello

Level: ★☆☆

Write a function `hello(first_name)` that **returns** the sentence `Hello Alice!`.
Print the result for `"Alice"` and for `"Bob"`.

```text
Hello Alice!
Hello Bob!
```

> **Hint:** a [function](../debuter/glossaire.md#function) starts with
> `def hello(first_name):` (see [functions](fonctions.md)).

> **Hint:** the function uses `return`, and you do the `print` outside:
> `print(hello("Alice"))`.

> **Solution**
>
> ```python
> def hello(first_name):
>     return f"Hello {first_name}!"
>
> print(hello("Alice"))
> print(hello("Bob"))
> ```
>
> ```sortie
> Hello Alice!
> Hello Bob!
> ```
>
> Why it works: at each call, the [parameter](../debuter/glossaire.md#parameter)
> `first_name` receives a different value. `return` sends back the sentence, and
> `print` prints it.

### Exercise 10: The bigger of the two

Level: ★☆☆

Write a function `bigger(a, b)` that returns the bigger of the two numbers,
**without** using `max`. Print `bigger(4, 9)` and `bigger(7, 2)`.

```text
9
7
```

> **Hint:** in the function, compare `a` and `b` with an `if`.

> **Hint:** a function can have two `return`. The first one reached stops the
> function.

> **Solution**
>
> ```python
> def bigger(a, b):
>     if a > b:
>         return a
>     return b
>
> print(bigger(4, 9))
> print(bigger(7, 2))
> ```
>
> ```sortie
> 9
> 7
> ```
>
> Why it works: for `(7, 2)`, `7 > 2` is true, so the function returns `7` and stops.
> For `(4, 9)`, the test is false: we reach the second `return b`.

## Text

### Exercise 11: Counting the letters

Level: ★☆☆

Store the word `"python"` in a variable. Print it in capital letters, followed by its
number of letters.

```text
PYTHON has 6 letters
```

> **Hint:** `len(word)` gives the length of a text (see [text](texte.md)).

> **Hint:** `word.upper()` returns the text in capital letters.

> **Solution**
>
> ```python
> word = "python"
> print(f"{word.upper()} has {len(word)} letters")
> ```
>
> ```sortie
> PYTHON has 6 letters
> ```
>
> Why it works: between the braces of an f-string, you can write any calculation,
> even a method call.

## Lists and dictionaries

### Exercise 12: The shopping list

Level: ★☆☆

Start from the list `["bread", "milk"]`. Add `"apples"` at the end. Print each item
preceded by a dash, then the number of items.

```text
- bread
- milk
- apples
3 items
```

> **Hint:** `shopping.append("apples")` adds at the end (see
> [collections](collections.md)).

> **Hint:** `for item in shopping:` goes through the list, `len(shopping)` counts it.

> **Solution**
>
> ```python
> shopping = ["bread", "milk"]
> shopping.append("apples")
> for item in shopping:
>     print(f"- {item}")
> print(f"{len(shopping)} items")
> ```
>
> ```sortie
> - bread
> - milk
> - apples
> 3 items
> ```
>
> Why it works: `append` changes the list itself. So the loop does see all three
> items.

### Exercise 13: The best grade

Level: ★☆☆

Find the highest grade in the list `[12, 17, 9, 15]` with a loop, **without** using
`max`.

```text
17
```

> **Hint:** keep in a variable `best` the highest grade seen so far. At the start,
> take the first one: `best = grades[0]`.

> **Hint:** for each grade, if it is higher than `best`, it takes its place.

> **Solution**
>
> ```python
> grades = [12, 17, 9, 15]
> best = grades[0]
> for grade in grades:
>     if grade > best:
>         best = grade
> print(best)
> ```
>
> ```sortie
> 17
> ```
>
> Why it works:
>
> | `grade` | `grade > best`? | `best` after |
> |---|---|---|
> | 12 | no | 12 |
> | 17 | yes | 17 |
> | 9 | no | 17 |
> | 15 | no | 17 |

### Exercise 14: Fruit prices

Level: ★☆☆

Start from the dictionary `{"apple": 2, "kiwi": 3}`. Add the key `"melon"` with the
price `5`. Print each fruit with its price.

```text
apple: 2 €
kiwi: 3 €
melon: 5 €
```

> **Hint:** you add a key the same way you store a value:
> `prices["melon"] = 5` (see [dictionaries](dictionnaires.md)).

> **Hint:** `for fruit, amount in prices.items():` gives the key and the value at each
> turn.

> **Solution**
>
> ```python
> prices = {"apple": 2, "kiwi": 3}
> prices["melon"] = 5
> for fruit, amount in prices.items():
>     print(f"{fruit}: {amount} €")
> ```
>
> ```sortie
> apple: 2 €
> kiwi: 3 €
> melon: 5 €
> ```
>
> Why it works: a [dictionary](../debuter/glossaire.md#dictionary) stores pairs
> "key → value". It keeps the order in which you added the keys.

### Exercise 15: The squares

Level: ★☆☆

Build the list of the squares of the numbers from 1 to 5 in **a single line**, with a
comprehension, then print it.

```text
[1, 4, 9, 16, 25]
```

> **Hint:** the square of `n` is written `n * n`.

> **Hint:** a comprehension has the form `[what_you_add for n in ...]` (see
> [comprehensions](comprehensions.md)).

> **Solution**
>
> ```python
> squares = [n * n for n in range(1, 6)]
> print(squares)
> ```
>
> ```sortie
> [1, 4, 9, 16, 25]
> ```
>
> Why it works: read it out loud: "the list of `n * n`, for each `n` from 1 to 5". The
> same thing with a loop would take three lines and an `append`.

## What next?

Well done! If these exercises seem easy to you, move on to the
[exercises with solutions](exercices.md), which go up to level ★★★. If one of them
gave you trouble, read [beginner pitfalls](pieges.md) again: the cause is often there.

Next: [classes and objects](types.md)
