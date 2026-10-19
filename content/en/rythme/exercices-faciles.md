# Exercises to get started

Fifteen small, very guided exercises, all easy, to practice the basics of Rythme: names, calculations, text, conditions, functions, lists, records and dictionaries.

## How to do these exercises

All the exercises have the level ★☆☆: a few lines, a single idea.
For each one, follow these steps:

```text
1. Read the task and the expected result.
2. Think it through on paper: which values? which calculation?
3. Write the code in a file, for example exo.ry.
4. Run it:  rythme run exo.ry
5. Compare with the expected result.
6. Stuck? Read the first hint, then the second one.
7. Only then, open the solution.
```

An error is not a failure: read the message, especially the `help:` line.
The page [beginner pitfalls](pieges.md) explains the most common ones.

> **Tip:** there are often several good answers. If your program prints
> the right result, it is correct, even if it does not look like the
> solution.

## Names, calculations and text

### Exercise 1: Introduce yourself

Level: ★☆☆

Give a name to your first name and another one to your city. Then print a
sentence that uses both. Expected result (with your values):

```text
Hello, I am Léa and I live in Lyon.
```

> **Hint:** start with `first_name = "Léa"`, then `city = "Lyon"`.

> **Hint:** inside a text, `{first_name}` is replaced by the value of the name.

> **Solution**
>
> ```rythme
> first_name = "Léa"
> city = "Lyon"
> print("Hello, I am {first_name} and I live in {city}.")
> ```
>
> ```sortie
> Hello, I am Léa and I live in Lyon.
> ```
>
> Why it works: the curly braces insert the value of a name into the text.
> The accent in "Léa" is a character like any other, inside the quotes.

### Exercise 2: The receipt

Level: ★☆☆

A croissant costs 3 euros. You buy 4 of them. Calculate the total and
print:

```text
4 croissants at 3 euros: 12 euros
```

> **Hint:** three names: `price`, `quantity` and `total`.

> **Hint:** `total = price * quantity`. The star `*` means "times".

> **Solution**
>
> ```rythme
> price = 3
> quantity = 4
> total = price * quantity
> print("{quantity} croissants at {price} euros: {total} euros")
> ```
>
> ```sortie
> 4 croissants at 3 euros: 12 euros
> ```
>
> Why it works: `total` receives the result of the calculation, then the text
> inserts it. Change `quantity`: the total follows on its own.

### Exercise 3: Sharing candies

Level: ★☆☆

You have 17 candies for 5 children. Each one gets the same number of
candies. Print how many each child gets, and how many are left:

```text
Each child gets 3 candies.
2 are left.
```

> **Hint:** integer division `//` gives each child's share.

> **Hint:** the remainder `%` gives what is left: 17 = 3 × 5 + **2**.

> **Solution**
>
> ```rythme
> candies = 17
> children = 5
> print("Each child gets {candies // children} candies.")
> print("{candies % children} are left.")
> ```
>
> ```sortie
> Each child gets 3 candies.
> 2 are left.
> ```
>
> Why it works: `17 // 5` is 3 (5 goes into 17 three times) and
> `17 % 5` is 2. With `/`, you would get `3.4`, which makes no sense
> for candies.

### Exercise 4: Shouting

Level: ★☆☆

Write a function `shout(sentence)` that puts the sentence in capital letters and
adds `!` at the end. Expected result for `shout("we won")`:

```text
WE WON!
```

> **Hint:** `upper` puts a text in capital letters.

> **Hint:** `+` sticks two texts together: `upper(sentence) + "!"`.

> **Solution**
>
> ```rythme
> fn shout(sentence) = upper(sentence) + "!"
> print(shout("we won"))
> ```
>
> ```sortie
> WE WON!
> ```
>
> Why it works: `upper` gives a new text in capital letters, accented letters
> included, and `+` sticks the two texts end to end.

### Exercise 5: The word backwards

Level: ★☆☆

Print the word `hello` written backwards:

```text
olleh
```

> **Hint:** look for a function that "flips" a value, like a
> pancake.

> **Hint:** `reverse` works on lists, and on texts too.

> **Solution**
>
> ```rythme
> print(reverse("hello"))
> ```
>
> ```sortie
> olleh
> ```
>
> Why it works: `reverse` reads the characters from the last to the first.
> The word `"hello"` itself does not change: `reverse` makes a
> new one.

## Conditions and functions

### Exercise 6: Adult or minor

Level: ★☆☆

Write a function `status(age)` that gives `"adult"` from 18 years old,
and `"minor"` otherwise. Expected result for 20, 12 and 18:

```text
adult minor adult
```

> **Hint:** `if condition then value else other_value`.

> **Hint:** the condition is `age >= 18`: "at least 18".

> **Solution**
>
> ```rythme
> fn status(age) = if age >= 18 then "adult" else "minor"
> print(status(20), status(12), status(18))
> ```
>
> ```sortie
> adult minor adult
> ```
>
> Why it works: `>=` includes 18. With `>`, `status(18)` would give
> `"minor"`. Always remember to test the boundary value.

### Exercise 7: The bigger one

Level: ★☆☆

Write a function `bigger(a, b)` that gives the bigger of the two
numbers. Expected result for `(3, 8)`, `(10, 2)` and `(5, 5)`:

```text
8 10 5
```

> **Hint:** compare `a` and `b` with `>`.

> **Hint:** if `a > b`, the answer is `a`; otherwise, it is `b`.

> **Solution**
>
> ```rythme
> fn bigger(a, b) = if a > b then a else b
> print(bigger(3, 8), bigger(10, 2), bigger(5, 5))
> ```
>
> ```sortie
> 8 10 5
> ```
>
> Why it works: when both are equal, the condition is false and
> you get `b`, which is also 5. Rythme already has a function for this:
> `max([a, b])`.

### Exercise 8: The traffic light

Level: ★☆☆

Write a function `instruction(color)`: `"green"` gives `"go"`,
`"yellow"` gives `"slow down"`, `"red"` gives `"stop"`, and any
other color gives `"unknown color"`.

```text
stop
go
unknown color
```

> **Hint:** several cases: use `match color`, one case per line,
> then `end`.

> **Hint:** each line is written `"green" -> "go"`. The last case,
> `_`, catches everything else.

> **Solution**
>
> ```rythme
> fn instruction(color) = match color
>   "green" -> "go"
>   "yellow" -> "slow down"
>   "red" -> "stop"
>   _ -> "unknown color"
> end
> print(instruction("red"))
> print(instruction("green"))
> print(instruction("blue"))
> ```
>
> ```sortie
> stop
> go
> unknown color
> ```
>
> Why it works: `match` tries the cases from top to bottom. `"blue"` does not
> match any text, so it falls on `_`, always last.

### Exercise 9: The average

Level: ★☆☆

Calculate the average of the grades `[12, 15, 10]`. Print it as it is,
then rounded to one digit after the decimal point:

```text
12.333333333333334
12.3
```

> **Hint:** the average is the sum divided by the number of grades:
> `sum` and `len`.

> **Hint:** `round(x, 1)` keeps one digit after the decimal point.

> **Solution**
>
> ```rythme
> grades = [12, 15, 10]
> average = sum(grades) / len(grades)
> print(average)
> print(round(average, 1))
> ```
>
> ```sortie
> 12.333333333333334
> 12.3
> ```
>
> Why it works: `sum(grades)` is 37 and `len(grades)` is 3. `/`
> gives a decimal number, which `round` shortens.

### Exercise 10: The countdown

Level: ★☆☆

Print a countdown from 3 to 1, then `Go!`:

```text
3
2
1
Go!
```

> **Hint:** there are no loops in Rythme. `reverse(1..3)` gives `[3, 2, 1]`.

> **Hint:** `each` calls a function for each element:
> `each(fn(n) = print(n))`.

> **Solution**
>
> ```rythme
> reverse(1..3) |> each(fn(n) = print(n))
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
> Why it works: `1..3` is the list `[1, 2, 3]`, `reverse` flips it, and
> `each` prints each number. Warning: `3..1` would be an empty
> list.
>
> | Step | n | Printed |
> |------|---|---------|
> | 1 | 3 | `3` |
> | 2 | 2 | `2` |
> | 3 | 1 | `1` |
> | end | | `Go!` |

## Lists, records and dictionaries

### Exercise 11: The squares

Level: ★☆☆

Build the list of the squares of the numbers from 1 to 6 (a square is a
number multiplied by itself):

```text
[1, 4, 9, 16, 25, 36]
```

> **Hint:** `1..6` gives the numbers from 1 to 6.

> **Hint:** `map` builds a new list: `map(fn(x) = x * x)`.

> **Solution**
>
> ```rythme
> print(1..6 |> map(fn(x) = x * x))
> ```
>
> ```sortie
> [1, 4, 9, 16, 25, 36]
> ```
>
> Why it works: `map` applies `x * x` to each number and puts the
> results in a new list, in the same order.

### Exercise 12: The shopping list

Level: ★☆☆

Start from `["bread", "milk"]`. Add `"eggs"`, then print the number
of items and the list numbered from 1:

```text
3 items
1. bread
2. milk
3. eggs
```

> **Hint:** `push(shopping, "eggs")` gives a **new** list:
> give it a new name.

> **Hint:** `each_with_index` gives the item and its position, which
> starts at 0: print `{i + 1}`.

> **Solution**
>
> ```rythme
> shopping = ["bread", "milk"]
> shopping2 = push(shopping, "eggs")
> print("{len(shopping2)} items")
> shopping2.each_with_index do |item, i|
>   print("{i + 1}. {item}")
> end
> ```
>
> ```sortie
> 3 items
> 1. bread
> 2. milk
> 3. eggs
> ```
>
> Why it works: `shopping` has not changed, `shopping2` has three
> items. The positions go from 0 to 2, hence the `+ 1` for a human.

### Exercise 13: A book's card

Level: ★☆☆

Store a book in a record with a title, an author and a
number of pages. Print a sentence, then build a new card
with 120 pages, and print both page counts:

```text
The Little Prince, by Saint-Exupéry (96 pages)
96 120
```

> **Hint:** a record is written `{title: "...", author: "...", pages: 96}`
> and you read a field with a dot: `book.title`.

> **Hint:** `{..book, pages: 120}` copies the card while changing a single
> field.

> **Solution**
>
> ```rythme
> book = {title: "The Little Prince", author: "Saint-Exupéry", pages: 96}
> print("{book.title}, by {book.author} ({book.pages} pages)")
> book2 = {..book, pages: 120}
> print(book.pages, book2.pages)
> ```
>
> ```sortie
> The Little Prince, by Saint-Exupéry (96 pages)
> 96 120
> ```
>
> Why it works: `book.pages = 120` would be an error, because nothing can be
> changed. `{..book, pages: 120}` builds a copy, and the original keeps
> its 96 pages.

### Exercise 14: The capitals

Level: ★☆☆

Store three countries and their capital in a dictionary. Print the
capital of Italy, then look for the one of Peru, which is not there:

```text
Rome
unknown
```

> **Hint:** `dict([["France", "Paris"], ["Italy", "Rome"], ...])`, then
> `capitals["Italy"]`.

> **Hint:** `capitals["Peru"]` would stop the program. `get` gives
> an option, and `unwrap_or("unknown")` a fallback value.

> **Solution**
>
> ```rythme
> capitals = dict([["France", "Paris"], ["Italy", "Rome"], ["Japan", "Tokyo"]])
> print(capitals["Italy"])
> print(get(capitals, "Peru") |> unwrap_or("unknown"))
> ```
>
> ```sortie
> Rome
> unknown
> ```
>
> Why it works: `get` gives `None` for a missing key, and
> `unwrap_or` replaces `None` with `"unknown"`.

### Exercise 15: The different letters

Level: ★☆☆

How many different letters are there in `abracadabra`? Print them,
then how many there are:

```text
set("a", "b", "c", "d", "r") 5
```

> **Hint:** `chars(word)` gives the list of the characters.

> **Hint:** a set (`set`) keeps each value only once.

> **Solution**
>
> ```rythme
> word = "abracadabra"
> letters = set(chars(word))
> print(letters, len(letters))
> ```
>
> ```sortie
> set("a", "b", "c", "d", "r") 5
> ```
>
> Why it works: `chars` gives 11 characters, with repeats.
> `set` removes the duplicates and puts the letters in order.

## What next?

Well done if you made it this far! More varied exercises, from easy
to hard, are waiting for you in [the solved exercises](exercices.md).
`rythme exercise` also offers 30 exercises in the terminal.

Next: [thinking functionally](fonctionnel.md)
