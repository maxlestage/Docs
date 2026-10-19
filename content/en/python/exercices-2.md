# Exercises with solutions: second series

This page offers 12 new exercises with solutions, from easiest to hardest, with tables that track the variables step by step in the solutions.

## How to use this page

These exercises are **new**: they do not repeat the ones from the
[exercises to get started](exercices-faciles.md) or the
[exercises with solutions](exercices.md). Each one has a level:

- ★☆☆: easy, a single idea;
- ★★☆: medium, two or three ideas together;
- ★★★: harder, you need to think before you write.

For each exercise:

1. Read the instructions and the expected output.
2. Write the steps in plain English on a piece of paper.
3. Write the code in a `.py` file and run it with `python3`.
4. Stuck? Read the hint. Still stuck? Open the solution.
5. Read "Why it works" even if you succeeded.

Many solutions contain a **trace table**: one column per variable, one row per turn
of the loop. If you do not know this tool yet, first read
[Following a program step by step](pas-a-pas.md).

> **Remember:** an exercise you failed and then understood is worth more than one you
> solved by luck. Always compare your solution with the one on the page.

## Level ★☆☆: warming up

### Exercise 1: The receipt

Level: ★☆☆

You have a shopping list: `[("bread", 2), ("milk", 1), ("cheese", 6)]`. Each item is a
tuple (name, price). Print one line per item, then the total.

```text
bread ... 2 €
milk ... 1 €
cheese ... 6 €
TOTAL ... 9 €
```

> **Hint:** `for name, price in items:` splits each tuple into two variables. Keep a
> piggy bank `total = 0` before the loop.

> **Solution**
>
> ```python
> items = [("bread", 2), ("milk", 1), ("cheese", 6)]
> total = 0
> for name, price in items:
>     print(f"{name} ... {price} €")
>     total = total + price
> print(f"TOTAL ... {total} €")
> ```
>
> ```sortie
> bread ... 2 €
> milk ... 1 €
> cheese ... 6 €
> TOTAL ... 9 €
> ```
>
> Why it works: like a cashier, the loop goes through each item, prints it, and adds its
> price to the total.
>
> | Turn | `name` | `price` | `total` after |
> |---|---|---|---|
> | 1 | bread | 2 | 2 |
> | 2 | milk | 1 | 3 |
> | 3 | cheese | 6 | 9 |
>
> The last `print` is not indented: it runs only once, after the loop.

### Exercise 2: From degrees Celsius to degrees Fahrenheit

Level: ★☆☆

Write a function `fahrenheit(celsius)` that returns the temperature in degrees
Fahrenheit. The formula is: `celsius × 9 / 5 + 32`. Print the conversion of 0, 20, 37
and 100.

```text
0 °C = 32.0 °F
20 °C = 68.0 °F
37 °C = 98.6 °F
100 °C = 212.0 °F
```

> **Hint:** the function fits on one line, with `return`. Then call it in a loop over
> the list `[0, 20, 37, 100]`.

> **Solution**
>
> ```python
> def fahrenheit(celsius):
>     return celsius * 9 / 5 + 32
>
> for c in [0, 20, 37, 100]:
>     print(f"{c} °C = {fahrenheit(c)} °F")
> ```
>
> ```sortie
> 0 °C = 32.0 °F
> 20 °C = 68.0 °F
> 37 °C = 98.6 °F
> 100 °C = 212.0 °F
> ```
>
> Why it works: the function is a small converting machine; the loop gives it each
> temperature in turn. The result has a `.0` because `/` always gives a decimal number
> (see [numbers](nombres.md)).
>
> Try it: add `-40` to the list. You get `-40 °C = -40.0 °F`: it is the only
> temperature that is the same in both units.

### Exercise 3: The longest word

Level: ★☆☆

In the sentence `"the cat eats a mouse"`, find the longest word, with a loop, without
using `max`.

```text
mouse
```

> **Hint:** `sentence.split()` cuts the sentence into a list of words. Keep in
> `longest` the longest word seen so far; at the start, an empty text `""`.

> **Solution**
>
> ```python
> sentence = "the cat eats a mouse"
> longest = ""
> for word in sentence.split():
>     if len(word) > len(longest):
>         longest = word
> print(longest)
> ```
>
> ```sortie
> mouse
> ```
>
> Why it works: it is a contest. Each word faces the current champion; if it is
> longer, it takes its place.
>
> | `word` | `len(word)` | `len(longest)` before | Longer? | `longest` after |
> |---|---|---|---|---|
> | the | 3 | 0 | yes | the |
> | cat | 3 | 3 | no | the |
> | eats | 4 | 3 | yes | eats |
> | a | 1 | 4 | no | eats |
> | mouse | 5 | 4 | yes | mouse |
>
> Try it: replace `mouse` with `rat`. The program prints `eats`. In case of a tie, the
> first word keeps its place, because `>` is strict: that is why `cat` does not beat
> `the`.

### Exercise 4: Reversing a list

Level: ★☆☆

Start from the list `["a", "b", "c", "d"]`. Build a **new** reversed list, with a loop,
without using `reversed` or `[::-1]`.

```text
['d', 'c', 'b', 'a']
```

> **Hint:** `list.insert(0, x)` adds `x` **at the start** of the list, in front of all
> the others (see [collections](collections.md)).

> **Solution**
>
> ```python
> letters = ["a", "b", "c", "d"]
> backwards = []
> for letter in letters:
>     backwards.insert(0, letter)
> print(backwards)
> ```
>
> ```sortie
> ['d', 'c', 'b', 'a']
> ```
>
> Why it works: think of a stack of plates. Each new plate goes **in front of** the
> others; the last one put down ends up first.
>
> | `letter` | `backwards` after |
> |---|---|
> | a | `['a']` |
> | b | `['b', 'a']` |
> | c | `['c', 'b', 'a']` |
> | d | `['d', 'c', 'b', 'a']` |

## Level ★★☆: combining several ideas

> **Remember:** at this level, break the problem down. First write a small function
> that does one single thing, test it, then put the pieces together.

### Exercise 5: The leap year

Level: ★★☆

A year is a leap year (it has a February 29) if it is divisible by 4, **except** if it
is divisible by 100, **except** if it is divisible by 400. Write `is_leap(year)` and
test 2024, 2023, 1900 and 2000.

```text
2024 True
2023 False
1900 False
2000 True
```

> **Hint:** `year % 4 == 0` tests "divisible by 4". Start with the rarest case: 400,
> then 100, then 4.

> **Solution**
>
> ```python
> def is_leap(year):
>     if year % 400 == 0:
>         return True
>     if year % 100 == 0:
>         return False
>     return year % 4 == 0
>
> for year in [2024, 2023, 1900, 2000]:
>     print(year, is_leap(year))
> ```
>
> ```sortie
> 2024 True
> 2023 False
> 1900 False
> 2000 True
> ```
>
> Why it works: the first `return` reached stops the function. So we test the
> strongest exception first.
>
> | `year` | `% 400 == 0`? | `% 100 == 0`? | `% 4 == 0`? | Result |
> |---|---|---|---|---|
> | 2024 | no | no | yes | True |
> | 2023 | no | no | no | False |
> | 1900 | no | yes | (not tested) | False |
> | 2000 | yes | (not tested) | (not tested) | True |

### Exercise 6: The cash machine

Level: ★★☆

A cash machine has 50, 20 and 10 € notes. It always gives as few notes as possible:
the biggest ones first. Write `dispense(amount)` and test it with 180 €.

```text
3 x 50 €
1 x 20 €
1 x 10 €
```

> **Hint:** for each note, `amount // note` gives how many of them you can give, and
> `amount % note` what is left to pay.

> **Solution**
>
> ```python
> def dispense(amount):
>     for note in [50, 20, 10]:
>         count = amount // note
>         amount = amount % note
>         if count > 0:
>             print(f"{count} x {note} €")
>
> dispense(180)
> ```
>
> ```sortie
> 3 x 50 €
> 1 x 20 €
> 1 x 10 €
> ```
>
> Why it works: at each turn, we take as many of the current note as possible, then
> we go on with what is left.
>
> | `note` | `amount` before | `count` | `amount` after |
> |---|---|---|---|
> | 50 | 180 | 3 | 30 |
> | 20 | 30 | 1 | 10 |
> | 10 | 10 | 1 | 0 |
>
> ```text
> 180 €  →  [50][50][50]  left 30  →  [20]  left 10  →  [10]  left 0
> ```
>
> Try it: `dispense(90)`. There is no line for a note given 0 times, thanks to the
> `if count > 0`.

### Exercise 7: Removing duplicates

Level: ★★☆

Write `without_duplicates(elements)` that returns a list without duplicates, **keeping
the order** of first appearance. Test it with `[3, 1, 3, 2, 1, 4]` and
`["red", "green", "red"]`.

```text
[3, 1, 2, 4]
['red', 'green']
```

> **Hint:** keep a [set](../debuter/glossaire.md#ensemble) `seen` of the elements
> already met. A set answers the question `x in seen` very quickly.

> **Solution**
>
> ```python
> def without_duplicates(elements):
>     seen = set()
>     result = []
>     for e in elements:
>         if e not in seen:
>             seen.add(e)
>             result.append(e)
>     return result
>
> print(without_duplicates([3, 1, 3, 2, 1, 4]))
> print(without_duplicates(["red", "green", "red"]))
> ```
>
> ```sortie
> [3, 1, 2, 4]
> ['red', 'green']
> ```
>
> Why it works: it is the bouncer at a party with the list of guests already inside.
> Someone who is already in does not get in a second time.
>
> | `e` | Already seen? | `result` after |
> |---|---|---|
> | 3 | no | `[3]` |
> | 1 | no | `[3, 1]` |
> | 3 | yes | `[3, 1]` |
> | 2 | no | `[3, 1, 2]` |
> | 1 | yes | `[3, 1, 2]` |
> | 4 | no | `[3, 1, 2, 4]` |
>
> `list(set(elements))` also removes duplicates, but does not guarantee the order.

### Exercise 8: The Caesar cipher

Level: ★★☆

Julius Caesar hid his messages by shifting each letter by 3 in the alphabet: `a`
becomes `d`, `b` becomes `e`… and `z` goes back round to `c`. Write
`caesar(text, shift)` for lowercase letters; the other characters stay as they are.
Encode `"hello zoe"`, then decode the result.

```text
khoor crh
hello zoe
```

> **Hint:** `alphabet.index(letter)` gives the position of a letter (0 for `a`). To go
> back to the start after `z`, take the remainder: `(position + shift) % 26`.

> **Solution**
>
> ```python
> def caesar(text, shift):
>     alphabet = "abcdefghijklmnopqrstuvwxyz"
>     result = ""
>     for letter in text:
>         if letter in alphabet:
>             position = alphabet.index(letter)
>             result += alphabet[(position + shift) % 26]
>         else:
>             result += letter
>     return result
>
> secret = caesar("hello zoe", 3)
> print(secret)
> print(caesar(secret, -3))
> ```
>
> ```sortie
> khoor crh
> hello zoe
> ```
>
> Why it works: the alphabet is like the face of a clock. After position 25 (`z`), we
> go back to 0 (`a`). For `z`: `(25 + 3) % 26` is `2`, that is `c`. To decode, we shift
> the other way, by `-3`.
>
> ```text
> a b c d e f ... w x y z
> 0 1 2 3 4 5 ... 22 23 24 25  →  after 25, back to 0
> ```

### Exercise 9: Sorting words by initial

Level: ★★☆

Sort the words `["peach", "pear", "banana", "kiwi", "broccoli", "plum"]` into a
dictionary, by their first letter. Then print each group.

```text
p: peach, pear, plum
b: banana, broccoli
k: kiwi
```

> **Hint:** the key is `word[0]`, the value is a list. If the key does not exist yet,
> first create an empty list (see [dictionaries](dictionnaires.md)).

> **Solution**
>
> ```python
> words = ["peach", "pear", "banana", "kiwi", "broccoli", "plum"]
> groups = {}
> for word in words:
>     initial = word[0]
>     if initial not in groups:
>         groups[initial] = []
>     groups[initial].append(word)
>
> for initial, group in groups.items():
>     print(f"{initial}: {', '.join(group)}")
> ```
>
> ```sortie
> p: peach, pear, plum
> b: banana, broccoli
> k: kiwi
> ```
>
> Why it works: like a binder with A, B, C… dividers. We create a divider the first
> time we need it, then file each word behind it.
>
> `", ".join(group)` glues the words of the list together with a comma and a space
> between them.

## Level ★★★: going further

> **Remember:** for a hard exercise, make the trace table **before** writing the code,
> on a small example. The code then comes on its own.

### Exercise 10: Searching a sorted list

Level: ★★★

To find a word in a paper dictionary, you do not read every page: you open it in the
middle, then keep the right half. Write `search(items, target)` that does the same on
a **sorted** list, and returns the position of `target`, or `None`. Print each step.

```text
low=0 high=7 middle=3 -> 19
low=4 high=7 middle=5 -> 31
position: 5
low=0 high=7 middle=3 -> 19
low=0 high=2 middle=1 -> 8
low=0 high=0 middle=0 -> 3
position: None
```

> **Hint:** keep two bounds, `low` and `high`, around the zone where the target can
> still be. The middle is `(low + high) // 2`. Loop while `low <= high`.

> **Solution**
>
> ```python
> def search(items, target):
>     low = 0
>     high = len(items) - 1
>     while low <= high:
>         middle = (low + high) // 2
>         print(f"low={low} high={high} middle={middle} -> {items[middle]}")
>         if items[middle] == target:
>             return middle
>         if items[middle] < target:
>             low = middle + 1
>         else:
>             high = middle - 1
>     return None
>
> numbers = [3, 8, 12, 19, 25, 31, 40, 47]
> print("position:", search(numbers, 31))
> print("position:", search(numbers, 5))
> ```
>
> ```sortie
> low=0 high=7 middle=3 -> 19
> low=4 high=7 middle=5 -> 31
> position: 5
> low=0 high=7 middle=3 -> 19
> low=0 high=2 middle=1 -> 8
> low=0 high=0 middle=0 -> 3
> position: None
> ```
>
> Why it works: at each turn, the search zone is cut in two. To search for 5:
>
> | Turn | `low` | `high` | `middle` | `items[middle]` | Decision |
> |---|---|---|---|---|---|
> | 1 | 0 | 7 | 3 | 19 | 19 > 5: `high = 2` |
> | 2 | 0 | 2 | 1 | 8 | 8 > 5: `high = 0` |
> | 3 | 0 | 0 | 0 | 3 | 3 < 5: `low = 1` |
> | — | 1 | 0 | | | `1 <= 0` false: end, `None` |
>
> ```text
> [3, 8, 12, 19, 25, 31, 40, 47]
>  ^           ^              ^
> low        middle          high     turn 1: 19 too big, keep the left side
> ```
>
> With 1,000 numbers, it takes at most 10 turns, instead of 1,000 by reading
> everything.

### Exercise 11: Compressing a text

Level: ★★★

A text like `"aaabccdddd"` contains repeated letters. Compress it by writing each
letter followed by the number of repetitions: `"a3b1c2d4"`. Write `compress` and
`decompress`, which does the opposite. We assume a letter is repeated at most 9 times.

```text
a3b1c2d4
aaabccdddd
```

> **Hint:** keep the `current` letter and its `count`. When the letter changes, write
> `current` and `count` into the result, then start again at 1. Do not forget the last
> group after the loop.

> **Hint:** to decompress, read the code two characters at a time:
> `range(0, len(code), 2)`. `"d" * 4` gives `"dddd"`.

> **Solution**
>
> ```python
> def compress(text):
>     if text == "":
>         return ""
>     result = ""
>     current = text[0]
>     count = 0
>     for letter in text:
>         if letter == current:
>             count += 1
>         else:
>             result += f"{current}{count}"
>             current = letter
>             count = 1
>     result += f"{current}{count}"
>     return result
>
> def decompress(code):
>     result = ""
>     for i in range(0, len(code), 2):
>         result += code[i] * int(code[i + 1])
>     return result
>
> c = compress("aaabccdddd")
> print(c)
> print(decompress(c))
> ```
>
> ```sortie
> a3b1c2d4
> aaabccdddd
> ```
>
> Why it works: we count identical letters that follow each other, like counting
> wagons of the same colour in a train.
>
> | `letter` | Same as `current`? | `current` | `count` | `result` |
> |---|---|---|---|---|
> | a | yes | a | 1 | `""` |
> | a | yes | a | 2 | `""` |
> | a | yes | a | 3 | `""` |
> | b | no | b | 1 | `"a3"` |
> | c | no | c | 1 | `"a3b1"` |
> | c | yes | c | 2 | `"a3b1"` |
> | d | no | d | 1 | `"a3b1c2"` |
> | d, d, d | yes | d | 4 | `"a3b1c2"` |
> | after the loop | | | | `"a3b1c2d4"` |
>
> The last row of the table shows why we need the line after the loop: without it, the
> group `d4` would be lost.

### Exercise 12: Balanced brackets

Level: ★★★

Write `balanced(text)` that tells whether the signs `()`, `[]` and `{}` of a text are
properly closed, in the right order. Test four texts.

```text
(a[b]{c})    True
(]           False
((x)         False
f(x) + [y]   True
```

> **Hint:** use a list like a **stack** of plates: `append` puts an opening on top,
> `pop()` takes back the one on top.

> **Hint:** for a closing sign, the stack must not be empty and its top must be the
> matching opening. At the end, the stack must be empty.

> **Solution**
>
> ```python
> def balanced(text):
>     pairs = {")": "(", "]": "[", "}": "{"}
>     stack = []
>     for sign in text:
>         if sign in "([{":
>             stack.append(sign)
>         elif sign in pairs:
>             if not stack or stack.pop() != pairs[sign]:
>                 return False
>     return stack == []
>
> for attempt in ["(a[b]{c})", "(]", "((x)", "f(x) + [y]"]:
>     print(f"{attempt:<12} {balanced(attempt)}")
> ```
>
> ```sortie
> (a[b]{c})    True
> (]           False
> ((x)         False
> f(x) + [y]   True
> ```
>
> Why it works: the last bracket opened must be the first one closed, like boxes
> stored inside each other. For `"(a[b]{c})"`:
>
> | `sign` | Action | `stack` after |
> |---|---|---|
> | ( | put on top | `['(']` |
> | a | nothing | `['(']` |
> | [ | put on top | `['(', '[']` |
> | b | nothing | `['(', '[']` |
> | ] | take back `[`: it matches | `['(']` |
> | { | put on top | `['(', '{']` |
> | c | nothing | `['(', '{']` |
> | } | take back `{`: it matches | `['(']` |
> | ) | take back `(`: it matches | `[]` |
>
> The stack is empty at the end: `True`. For `"((x)"`, a `(` is left in the stack, so
> `False`.

## Summary

- Exercises 1 to 4 review loops, functions and lists.
- Exercises 5 to 9 combine conditions, integer divisions, sets, text and dictionaries.
- Exercises 10 to 12 ask you to make a trace table before writing the code.
- When a solution surprises you, copy its table by hand: that is where you
  understand.

Next: keep [the cheat sheet](aide-memoire.md) at hand.
