# Solved exercises: second set

Twelve new exercises to keep practising Rythme, from the easiest to the hardest, each with a hint, a checked solution and a step-by-step explanation.

## How to do these exercises

This set follows [the exercises to get started](exercices-faciles.md)
and [the first set of solved exercises](exercices.md). No exercise
is repeated. Each exercise has a level:

- ★☆☆ easy: a few lines, a single idea;
- ★★☆ medium: you need to combine two or three ideas;
- ★★★ harder: take your time, and a pencil.

For each exercise, follow the same method:

```text
1. Read the task and the expected result.
2. On paper: which values go in? which value must come out?
3. Write the code in exo.ry, then:  rythme run exo.ry
4. Compare with the expected result.
5. Stuck? Read the hint. Still stuck? Open the solution.
6. Read the step-by-step table of the solution, even if you succeeded.
```

The step-by-step tables follow the value of each name, step by step.
It is like watching a film frame by frame: you see exactly when things
change. The page [rythme etapes](etapes.md) shows how Rythme can do this
work for you.

> **Remember:** try first, even for five minutes. A solution you read
> without searching is quickly forgotten; a solution you find after
> searching stays.

## Level ★☆☆: a single idea

### Exercise 1: The American thermometer

Level: ★☆☆

In the United States, temperature is given in degrees Fahrenheit. To go
from degrees Celsius to degrees Fahrenheit, you multiply by 9, divide
by 5, then add 32. Write `fahrenheit(c)`, then convert the whole list
`[0, 20, 37, 100]` in one go:

```text
[32.0, 68.0, 98.6, 212.0]
```

> **Hint:** start with the function for **a single** number:
> `fn fahrenheit(c) = ...`. Then `map` applies it to each element of the
> list. See [thinking functionally](fonctionnel.md).

> **Solution**
>
> ```rythme
> fn fahrenheit(c) = c * 9 / 5 + 32
>
> print([0, 20, 37, 100] |> map(fahrenheit))
> ```
>
> ```sortie
> [32.0, 68.0, 98.6, 212.0]
> ```
>
> Why it works: `map` is like a stamping machine. Each number goes
> under it, comes out transformed, and the new list keeps the same
> order. `/` always gives a decimal number, hence the `.0`.
>
> | Step | c | c * 9 | / 5 | + 32 |
> |------|---|-------|-----|------|
> | 1 | 0 | 0 | 0.0 | 32.0 |
> | 2 | 20 | 180 | 36.0 | 68.0 |
> | 3 | 37 | 333 | 66.6 | 98.6 |
> | 4 | 100 | 900 | 180.0 | 212.0 |

Try it: replace `/` with `//` (whole-number division). You get
`[32, 68, 98, 212]`: the `.6` of 98.6 has disappeared, because `333 // 5` is 66.

### Exercise 2: Initials

Level: ★☆☆

Write `initials(name)` which gives the initials of a name, in capitals,
each followed by a full stop:

```text
A. L.
J. P. S.
```

for `"ada lovelace"` and `"Jean Paul Sartre"`.

> **Hint:** three steps to chain with `|>`: `split` cuts the name
> into words; `map` turns each word into `"A."`; `join(" ")` glues the
> pieces back together. The first letter of a word `w` is written `w[0]`.

> **Solution**
>
> ```rythme
> fn initial(word) = upper(word[0]) + "."
>
> fn initials(name) = split(name) |> map(initial) |> join(" ")
>
> print(initials("ada lovelace"))
> print(initials("Jean Paul Sartre"))
> ```
>
> ```sortie
> A. L.
> J. P. S.
> ```
>
> Why it works: each `|>` passes the result to the next step,
> like on an assembly line.
>
> ```text
> "ada lovelace"
>    │ split
>    ▼
> ["ada", "lovelace"]
>    │ map(initial)
>    ▼
> ["A.", "L."]
>    │ join(" ")
>    ▼
> "A. L."
> ```

Try it: change `join(" ")` to `join("")`. You get `A.L.`, without a
space.

### Exercise 3: Counting vowels

Level: ★☆☆

How many vowels (a, e, i, o, u, y) are there in the word
`anticonstitutionally`? Expected result:

```text
9
```

> **Hint:** `chars(word)` gives the list of letters. `count(list, f)`
> counts the elements for which `f` answers `true`. To know whether a
> letter is a vowel, look for it in the text `"aeiouy"` with
> `contains`.

> **Solution**
>
> ```rythme
> fn vowel?(letter) = contains("aeiouy", letter)
>
> word = "anticonstitutionally"
> print(count(chars(word), vowel?))
> ```
>
> ```sortie
> 9
> ```
>
> Why it works: `vowel?` answers a question with yes or no,
> hence the `?` at the end of its name. `count` asks the question of each
> letter and counts the "yes" answers. It is like a bouncer counting the
> people with a badge.
>
> | Letters | a | n | t | i | c | o | n | s | t | i | ... |
> |---------|---|---|---|---|---|---|---|---|---|---|-----|
> | vowel? | yes | no | no | yes | no | yes | no | no | no | yes | ... |
> | count | 1 | 1 | 1 | 2 | 2 | 3 | 3 | 3 | 3 | 4 | ... |

Try it: replace the word with `"Rythme"`. You get `2`: the `y` and
the `e`. With `"PLANE"`, you get `0`, because `"aeiouy"` only contains
lowercase letters. Add `lower` to fix it: `chars(lower(word))`.

### Exercise 4: The basket

Level: ★☆☆

A shopping basket contains items, each stored in a record with
a name and a price. Print the number of items and the total to pay:

```text
3 items, total: 12 €
```

> **Hint:** `len` counts the items. `sum(list, f)` adds up the
> values that `f` gives for each element: here, the `price` field.

> **Solution**
>
> ```rythme
> basket = [
>   {item: "bread", price: 2},
>   {item: "cheese", price: 7},
>   {item: "juice", price: 3}
> ]
> total = sum(basket, fn(a) = a.price)
> print("{len(basket)} items, total: {total} €")
> ```
>
> ```sortie
> 3 items, total: 12 €
> ```
>
> Why it works: `fn(a) = a.price` is a small function without a name
> that "opens" each record and takes out the price. `sum` adds up
> these prices one by one.
>
> | Step | a.item | a.price | running total |
> |------|--------|---------|---------------|
> | start | | | 0 |
> | 1 | bread | 2 | 2 |
> | 2 | cheese | 7 | 9 |
> | 3 | juice | 3 | 12 |

Try it: add `{item: "apples", price: 4}` to the basket. The message
becomes `4 items, total: 16 €`, without changing anything else.

> **Remember:** for a list, think first "what do I do to **one**
> element?". Write that small function, then hand the list to
> `map`, `count` or `sum`.

## Level ★★☆: combining two or three ideas

### Exercise 5: FizzBuzz

Level: ★★☆

It is a school game: you count out loud, but you say "Fizz" for a
multiple of 3, "Buzz" for a multiple of 5, and "FizzBuzz" for a
multiple of both. Write `fizzbuzz(n)` with guards, then print the
answers from 1 to 15 on one line:

```text
1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
```

> **Hint:** a number is a multiple of 3 if `n % 3 == 0`. Write a
> definition by cases, with one guard per case (see
> [guards](fonctions.md#guards)). The last case gives the number as
> text: `text(n)`.

> **Hint:** the order of the cases matters. Rythme tries the definitions from
> top to bottom: the "multiple of 15" case must come first.

> **Solution**
>
> ```rythme
> fn fizzbuzz(n) if n % 15 == 0 = "FizzBuzz"
> fn fizzbuzz(n) if n % 3 == 0 = "Fizz"
> fn fizzbuzz(n) if n % 5 == 0 = "Buzz"
> fn fizzbuzz(n) = text(n)
>
> print(1..15 |> map(fizzbuzz) |> join(" "))
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```
>
> Why it works: a number that is a multiple of 3 **and** of 5 is a multiple
> of 15. So we test 15 first. For 15, here are the cases tried:
>
> | Case tried | Guard | Result |
> |------------|-------|--------|
> | 1 | `15 % 15 == 0` → `true` | `"FizzBuzz"`, we stop |
>
> For 9: `9 % 15` is 9 (false), then `9 % 3` is 0 (true): `"Fizz"`.
> For 7, no guard is true: the last case gives `"7"`.

Try it: put the `n % 15` case last, just before `text(n)`. For
15, you get `Fizz`: the case for 3 answered first.

### Exercise 6: The bank statement

Level: ★★☆

A bank account holds 100 €. Here are the month's operations, each
with a symbol `:deposit` or `:withdrawal` (see [symbols](symboles.md)):

```text
deposit 50, withdrawal 20, deposit 15, withdrawal 30
```

Print the balance after each operation, then the final balance:

```text
[150, 130, 145, 115]
115
```

> **Hint:** write `apply(balance, operation)` which gives the new
> balance, with a `match` on the record. Then `fold` passes the
> balance from one operation to the next, and `scan` shows each step.

> **Solution**
>
> ```rythme
> fn apply(balance, operation) = match operation
>   {type: :deposit, amount: m} -> balance + m
>   {type: :withdrawal, amount: m} -> balance - m
> end
>
> operations = [
>   {type: :deposit, amount: 50},
>   {type: :withdrawal, amount: 20},
>   {type: :deposit, amount: 15},
>   {type: :withdrawal, amount: 30}
> ]
> print(scan(operations, 100, apply))
> print(fold(operations, 100, apply))
> ```
>
> ```sortie
> [150, 130, 145, 115]
> 115
> ```
>
> Why it works: `fold` is like a cashier's notebook. It starts
> from 100, and each line of the notebook uses the balance of the previous
> line. Nothing is modified: each step makes a new balance.
>
> | Step | balance before | type | amount | balance after |
> |------|----------------|------|--------|---------------|
> | 1 | 100 | `:deposit` | 50 | 150 |
> | 2 | 150 | `:withdrawal` | 20 | 130 |
> | 3 | 130 | `:deposit` | 15 | 145 |
> | 4 | 145 | `:withdrawal` | 30 | 115 |

Try it: replace the starting 100 with 0. The final balance becomes `15`,
and the second element of `scan` becomes `30`.

### Exercise 7: The Caesar cipher

Level: ★★☆

Julius Caesar hid his messages by shifting each letter 3 places along
the alphabet: `a` becomes `d`, `b` becomes `e`… and `z` goes back to the start to
become `c`. Write `caesar(message, shift)` for a message in lowercase
without accents. Spaces do not change. Check that a second shift
of 23 (3 + 23 = 26, a full turn) gives back the original message:

```text
khoor crh
hello zoe
```

> **Hint:** each character has a number: `char_code("a")` is 97,
> and `char(97)` gives back `"a"`. The position of a letter in the alphabet is
> therefore `char_code(c) - 97` (from 0 for `a` to 25 for `z`).

> **Hint:** to go back to the start after `z`, take the remainder of the
> division by 26: `(position + shift) % 26`.

> **Solution**
>
> ```rythme
> alphabet = "abcdefghijklmnopqrstuvwxyz"
>
> fn shift(c, k) =
>   if contains(alphabet, c) then char((char_code(c) - 97 + k) % 26 + 97) else c
>
> fn caesar(message, k) = chars(message) |> map(fn(c) = shift(c, k)) |> join("")
>
> secret = caesar("hello zoe", 3)
> print(secret)
> print(caesar(secret, 23))
> ```
>
> ```sortie
> khoor crh
> hello zoe
> ```
>
> Why it works: the alphabet is arranged in a circle, like the hours
> on a clock. The `% 26` goes round the dial.
>
> ```text
>   ... w  x  y  z  a  b  c  d ...
>                └──────────►c   z moves 3 on: 25 + 3 = 28, 28 % 26 = 2 → c
> ```
>
> | c | char_code(c) - 97 | + 3 | % 26 | letter |
> |---|-------------------|-----|------|--------|
> | `h` | 7 | 10 | 10 | `k` |
> | `o` | 14 | 17 | 17 | `r` |
> | `z` | 25 | 28 | 2 | `c` |
> | space | | | | space (unchanged) |

Try it: `caesar("abc", 1)` gives `bcd`, and `caesar("abc", 26)` gives back
`abc`: a full turn changes nothing.

### Exercise 8: The sum of the digits

Level: ★★☆

Write a **recursive** function `digit_sum(n)` which adds up the
digits of a positive whole number: for 1234, it is 1 + 2 + 3 + 4 = 10.
Do not use text, only calculations:

```text
10 9 18
```

for 1234, 9 and 99.

> **Hint:** `n % 10` gives the last digit (1234 % 10 is 4), and
> `n // 10` removes that last digit (1234 // 10 is 123).

> **Hint:** the base case is 0: there are no digits left, the sum
> is 0. See [recursion](recursion.md).

> **Solution**
>
> ```rythme
> fn digit_sum(0) = 0
> fn digit_sum(n) = n % 10 + digit_sum(n // 10)
>
> print(digit_sum(1234), digit_sum(9), digit_sum(99))
> ```
>
> ```sortie
> 10 9 18
> ```
>
> Why it works: on each call, we "tear off" the last digit,
> like tearing tickets out of a book, until nothing is left.
>
> | Call | n | n % 10 | n // 10 | waiting |
> |------|---|--------|---------|---------|
> | 1 | 1234 | 4 | 123 | `4 + ...` |
> | 2 | 123 | 3 | 12 | `4 + 3 + ...` |
> | 3 | 12 | 2 | 1 | `4 + 3 + 2 + ...` |
> | 4 | 1 | 1 | 0 | `4 + 3 + 2 + 1 + ...` |
> | 5 | 0 | base case | | `4 + 3 + 2 + 1 + 0` = 10 |

Try it: remove the line `fn digit_sum(0) = 0`. The function never
stops, because `0 // 10` is still 0. Rythme ends up
stopping with error `E014`: "too many nested calls".

### Exercise 9: Sorting by initial

Level: ★★☆

Here is a list of fruits. Sort them by first letter, then print
the fruits that start with `p`:

```text
{p: ["peach", "pear", "plum"], b: ["banana", "blueberry"], a: ["apricot"]}
["peach", "pear", "plum"]
```

> **Hint:** `group_by(list, f)` puts the elements into drawers:
> the name of the drawer is what `f` answers for each element. Here, `f`
> gives the first letter.

> **Solution**
>
> ```rythme
> fruits = ["peach", "pear", "banana", "apricot", "blueberry", "plum"]
> by_letter = group_by(fruits, fn(f) = f[0])
> print(by_letter)
> print(by_letter.p)
> ```
>
> ```sortie
> {p: ["peach", "pear", "plum"], b: ["banana", "blueberry"], a: ["apricot"]}
> ["peach", "pear", "plum"]
> ```
>
> Why it works: `group_by` acts like someone sorting the
> mail. For each letter, they read the label and put it in the right
> pigeonhole. A new pigeonhole opens the first time a label
> appears, hence the order `p`, `b`, `a`.
>
> ```text
> ┌───────────────────────────┐ ┌───────────────────┐ ┌───────────┐
> │ p                         │ │ b                 │ │ a         │
> │ peach  pear  plum         │ │ banana  blueberry │ │ apricot   │
> └───────────────────────────┘ └───────────────────┘ └───────────┘
> ```

Try it: sort by length with `fn(f) = len(f)`. The drawers
are then called `5`, `4`, `6`, `7` and `9`.

> **Remember:** a medium exercise breaks down into small one-line
> functions (`apply`, `shift`, `initial`…). Test each one on its own before
> putting them together.

## Level ★★★: take your time

### Exercise 10: Compressing a text

Level: ★★★

To save space, you can write `aaabcc` as `a3b1c2`:
each letter followed by the number of times it repeats **in a row**.
Write `compress(word)`:

```text
[["a", 3], ["b", 1], ["c", 2]]
a3b1c2 z10
```

The first line shows the groups of `"aaabcc"`, the second the compressed
texts of `"aaabcc"` and `"zzzzzzzzzz"`.

> **Hint:** first write a recursive function `groups(letters)`
> on the list of letters. For the list `[c, ..rest]`, `take_while`
> takes from `rest` the letters equal to `c`, and `drop` removes those
> letters.

> **Hint:** the base case is the empty list, which gives `[]`. Each
> group is a small list `[letter, number]`.

> **Solution**
>
> ```rythme
> fn groups([]) = []
> fn groups([c, ..rest])
>   same = take_while(rest, fn(x) = x == c)
>   next = drop(rest, len(same))
>   [[c, 1 + len(same)], ..groups(next)]
> end
>
> fn compress(word) =
>   groups(chars(word)) |> map(fn(g) = "{g[0]}{g[1]}") |> join("")
>
> print(groups(chars("aaabcc")))
> print(compress("aaabcc"), compress("zzzzzzzzzz"))
> ```
>
> ```sortie
> [["a", 3], ["b", 1], ["c", 2]]
> a3b1c2 z10
> ```
>
> Why it works: it is like sorting coloured beads. You take
> the first bead, grab all the following ones of the same colour,
> write down "colour, number", then start again with what is
> left.
>
> | Call | c | rest | same | next | group |
> |------|---|------|------|------|-------|
> | 1 | `a` | `a a b c c` | `a a` | `b c c` | `["a", 3]` |
> | 2 | `b` | `c c` | (empty) | `c c` | `["b", 1]` |
> | 3 | `c` | `c` | `c` | (empty) | `["c", 2]` |
> | 4 | empty list | | | | base case: `[]` |

Try it: `compress("abab")` gives `a1b1a1b1`. The letters must follow
each other to form a group: this text does not compress well at all.

### Exercise 11: Guess the number

Level: ★★★

A secret number is between 1 and 100. The best strategy: suggest the
number in the **middle**. If it is too small, search in the upper half;
if it is too big, in the lower half. Write
`guess(secret, low, high, tries)` which counts the tries needed.
For the secrets 37, 50, 1 and 100:

```text
3 1 6 7
```

> **Hint:** the middle is calculated with whole-number division:
> `(low + high) // 2`. Three cases: found, too small, too big.

> **Hint:** "too small" means the secret is above the
> middle: start again with `middle + 1` as the new `low`, and one more
> try.

> **Solution**
>
> ```rythme
> fn guess(secret, low, high, tries)
>   middle = (low + high) // 2
>   if middle == secret then tries
>   else if middle < secret then guess(secret, middle + 1, high, tries + 1)
>   else guess(secret, low, middle - 1, tries + 1)
> end
>
> print(guess(37, 1, 100, 1), guess(50, 1, 100, 1), guess(1, 1, 100, 1), guess(100, 1, 100, 1))
> ```
>
> ```sortie
> 3 1 6 7
> ```
>
> Why it works: on each try, the search zone is cut in
> two, like when you open a dictionary in the middle to find a word.
> Seven tries are always enough for 100 numbers. Here is the game for the
> secret 37:
>
> | Try | low | high | middle | answer |
> |-----|-----|------|--------|--------|
> | 1 | 1 | 100 | 50 | too big: `high` becomes 49 |
> | 2 | 1 | 49 | 25 | too small: `low` becomes 26 |
> | 3 | 26 | 49 | 37 | found: the function gives 3 |
>
> ```text
> 1 ─────────────────────────────── 100     try 1: 50?
> 1 ─────────────── 49                       try 2: 25?
>             26 ── 49                       try 3: 37!
> ```

Try it: add `print(low, high, middle)` just after the line
`middle = ...`. The program then prints each try, and you can
compare with your table.

### Exercise 12: The Polish calculator

Level: ★★★

In "reverse Polish" notation, you write the numbers **before**
the operation: `3 4 +` means 3 + 4, and `3 4 + 2 *` means
(3 + 4) × 2. Some engineers' calculators work this way.
Write `rpn(input)` which gives a `Result`: `Ok(result)`, or `Err(...)`
if the calculation is impossible.

```text
Ok(7)
Ok(14)
Ok(4)
Err("a number is missing before +")
Err("several numbers are left")
```

for `"3 4 +"`, `"3 4 + 2 *"`, `"10 2 3 * -"`, `"3 +"` and `"3 4"`.

> **Hint:** use a **stack**, like a stack of plates: a
> number goes on top; an operation takes the two plates from the
> top, calculates, and puts the result back. With a list, the top is the
> first element: `[n, ..stack]` puts `n` on top.

> **Hint:** write `step(stack, word)` which gives `Ok(new_stack)` or
> `Err(message)`, then a recursive function `evaluate(words, stack)` which
> uses `?` to stop at the first error (see
> [handling errors](erreurs.md)).

> **Solution**
>
> ```rythme
> operations = ["+", "-", "*"]
>
> fn compute(op, a, b) = match op
>   "+" -> a + b
>   "-" -> a - b
>   _ -> a * b
> end
>
> fn step([b, a, ..rest], op) if contains(operations, op) =
>   Ok([compute(op, a, b), ..rest])
> fn step(_, op) if contains(operations, op) = Err("a number is missing before {op}")
> fn step(stack, word)
>   n = parse_int(word)?
>   Ok([n, ..stack])
> end
>
> fn evaluate([], [result]) = Ok(result)
> fn evaluate([], _) = Err("several numbers are left")
> fn evaluate([word, ..rest], stack)
>   new_stack = step(stack, word)?
>   evaluate(rest, new_stack)
> end
>
> fn rpn(input) = evaluate(split(input), [])
>
> print(rpn("3 4 +"))
> print(rpn("3 4 + 2 *"))
> print(rpn("10 2 3 * -"))
> print(rpn("3 +"))
> print(rpn("3 4"))
> ```
>
> ```sortie
> Ok(7)
> Ok(14)
> Ok(4)
> Err("a number is missing before +")
> Err("several numbers are left")
> ```
>
> Why it works: the stack holds the numbers that are waiting. Watch
> the order: the top of the stack is the **second** number (`b`), hence
> `compute(op, a, b)` and not `(op, b, a)`. For `10 2 3 * -`:
>
> | Step | word | stack before | stack after |
> |------|------|--------------|-------------|
> | 1 | `10` | `[]` | `[10]` |
> | 2 | `2` | `[10]` | `[2, 10]` |
> | 3 | `3` | `[2, 10]` | `[3, 2, 10]` |
> | 4 | `*` | `[3, 2, 10]` | `[6, 10]` (2 × 3) |
> | 5 | `-` | `[6, 10]` | `[4]` (10 − 6) |
> | end | | `[4]` | `Ok(4)` |
>
> ```text
>         ┌───┐
>         │ 3 │          ┌───┐
>         │ 2 │   *      │ 6 │   -      ┌───┐
>         │10 │  ───►    │10 │  ───►    │ 4 │
>         └───┘          └───┘          └───┘
> ```

Try it: `print(rpn("3 x +"))`. The word `x` is not a number:
`parse_int` gives an error, and `?` passes it all the way up.

> **Remember:** for a hard exercise, make the step-by-step table
> **before** writing the code. If you can do it by hand, you know
> what to ask Rythme.

## What next?

Well done! You have finished the three sets of exercises of this track. To
keep going:

- `rythme exercise list` offers 30 more exercises, checked by
  tests;
- [the guided projects](projets.md) put all this together in real
  programs;
- [the cheat sheet](aide-memoire.md) sums up all the syntax on one page.

The other tracks also have their second set, for example
[in Ruby](../ruby/exercices-2.md) and [in Python](../python/exercices-2.md).

Next: [cheat sheet](aide-memoire.md)
