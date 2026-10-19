# Following a program step by step

This page teaches you to run a Rythme program "in your head", with a table that tracks the values at each step, on six small programs that get richer and richer.

> **Goals:**
>
> - you will know how to fill in a trace table, one row per step;
> - you will know how to follow a condition, a `fold`, a function call and a recursive function;
> - you will know how to guess what a program prints **before** you run it;
> - you will know how to make a program show its steps, with `print`, `debug`, `scan` and `rythme steps`.

## Why follow a program by hand

A computer follows instructions one by one, without ever skipping a
line. To understand a program, do the same: play the computer.

It's like following a cooking recipe with your finger: you read a line,
you do what it says, you note where you are, then you move on to the next one.

Your tool is a **trace table**: one column per name, one row per
step. You need a sheet of paper and a pencil.

```text
 ┌───────┬──────────────────────┬────────┬────────┐
 │ Line  │ What happens         │ name 1 │ name 2 │
 ├───────┼──────────────────────┼────────┼────────┤
 │   1   │ ...                  │  ...   │        │
 │   2   │ ...                  │  ...   │  ...   │
 └───────┴──────────────────────┴────────┴────────┘
```

Every example on this page follows the same method:

1. read the code;
2. **before running it, guess what it prints**, and write it down;
3. fill in the trace table;
4. run the program and compare.

> **Remember:** getting your guess wrong is not a problem. That is
> exactly where you learn: the table shows where your idea and the
> computer part ways.

## Something special about Rythme: values never change

In many languages, a [variable](../debuter/glossaire.md#variable)
is a box whose contents you change. In Rythme, a
[value](../debuter/glossaire.md#value) is
[immutable](../debuter/glossaire.md#immutable): a name, once given, keeps
its value until the end (see [the traps](pieges.md)).

Good news for tracing: in your table, **each cell is filled in only
once**. You never erase anything. The table goes down like a staircase.

And since there is no loop, repetitions are done with `map`,
`fold` or a recursive function. You follow them with one row per **round**.

## Program 1: names one after another

```rythme
price = 8
quantity = 3
total = price * quantity
discount = total // 4
to_pay = total - discount
print("To pay: €{to_pay}")
```

**Before running it, guess what it prints.** Reminder: `//` is
integer division, it keeps only the whole part.

The trace table, line by line:

| Line | What happens | price | quantity | total | discount | to_pay |
|------|--------------|-------|----------|-------|----------|--------|
| 1 | `price` receives 8 | 8 | | | | |
| 2 | `quantity` receives 3 | 8 | 3 | | | |
| 3 | `8 * 3` | 8 | 3 | 24 | | |
| 4 | `24 // 4` | 8 | 3 | 24 | 6 | |
| 5 | `24 - 6` | 8 | 3 | 24 | 6 | 18 |
| 6 | prints the text | 8 | 3 | 24 | 6 | 18 |

```sortie
To pay: €18
```

On line 6, Rythme replaces `{to_pay}` with the value in the `to_pay`
column: 18. Look at the staircase: each row adds **one** cell, and
the cells already filled in don't move anymore.

Try it: set `quantity = 5`. Fill in the table first: `total`
is 40, `discount` 10, and the program prints `To pay: €30`.

> **Remember:** one line of the program, one row of the table. To
> calculate, look for the value of a name in **its column**, higher up.

## Program 2: a condition

```rythme
temperature = 18
rain = true
outfit = if temperature >= 25 then "shorts"
  else if temperature >= 15 then "a sweater"
  else "a coat"
extra = if rain then " and an umbrella" else ""
print("Today: {outfit}{extra}")
```

**Before running it, guess what it prints.**

For a [condition](../debuter/glossaire.md#condition), add two
columns: the **question** asked, and the **answer**, `true` or `false`.

| Line | Question | Answer | What happens | outfit | extra |
|------|----------|--------|--------------|--------|-------|
| 1-2 | | | `temperature` = 18, `rain` = true | | |
| 3 | `18 >= 25`? | `false` | go to the `else` | | |
| 4 | `18 >= 15`? | `true` | take `"a sweater"` | `"a sweater"` | |
| 6 | `rain`? | `true` | take the text of the `then` | `"a sweater"` | `" and an umbrella"` |
| 7 | | | glue the two texts together | `"a sweater"` | `" and an umbrella"` |

```sortie
Today: a sweater and an umbrella
```

The questions are asked **from top to bottom**, and the first `true`
answer wins: line 5 (`"a coat"`) is never read.

```text
 18 >= 25 ? ── no ──► 18 >= 15 ? ── yes ──► "a sweater"   (stop here)
     │
    yes
     ▼
 "shorts"
```

Try it: set `temperature = 30` and `rain = false`. The program
prints `Today: shorts`: the first question answers `true`, and
`extra` is the empty text `""`.

> **Remember:** for a condition, write the question and its answer
> in the table. Only the chosen branch is calculated.

## Program 3: adding up with fold

In Rythme, a "loop that adds up" is written with
[fold](fonctionnel.md). `fold` carries a value from one round to
the next, the **accumulator** (`acc`), like a basket you fill as you
walk through a shop.

```rythme
purchases = [4, 10, 3, 7]
total = fold(purchases, 0, fn(acc, x) = acc + x)
print("Total: {total}")
```

**Before running it, guess what it prints.**

`fold` starts from `0`, then calls the small function once **per
element**. Each round receives the accumulator of the previous round:

| Round | acc (received) | x | acc + x (given to the next round) |
|-------|----------------|---|-----------------------------------|
| start | | | 0 |
| 1 | 0 | 4 | 4 |
| 2 | 4 | 10 | 14 |
| 3 | 14 | 3 | 17 |
| 4 | 17 | 7 | 24 |
| end | | | `total` = 24 |

```sortie
Total: 24
```

The secret of a good table for `fold`: the **acc (received)** column of a
round is the last column of the round **before**. Follow the arrow:

```text
 0 ──(+4)──► 4 ──(+10)──► 14 ──(+3)──► 17 ──(+7)──► 24
```

Rythme can check your table. `scan` does the same calculation as `fold`,
but gives **all** the intermediate results, in other words your
last column:

```rythme
purchases = [4, 10, 3, 7]
print(scan(purchases, 0, fn(acc, x) = acc + x))
```

```sortie
[4, 14, 17, 24]
```

Try it: replace `0` with `100` in the `fold`, like a wallet
that already holds €100. The table starts at 100, and the program
prints `Total: 124`.

> **Remember:** for `fold`, one row per element; the accumulator received
> is always the result of the row above.

## Program 4: a function called twice

```rythme
fn with_tax(net) = net + net * 20 // 100

book = with_tax(15)
pen = with_tax(5)
print(book, pen, book + pen)
```

**Before running it, guess what it prints.** `with_tax` adds 20% to a price.

A [function](../debuter/glossaire.md#function) is a small machine:
you put a value in, another one comes out. At each call, its
[parameter](../debuter/glossaire.md#parameter) `net` takes the value given,
only for the duration of the call.

```text
 program                              function with_tax
 ───────                              ─────────────────
 book = with_tax(15)  ── net = 15 ──►   15 + 15 * 20 // 100
                                        = 15 + 300 // 100
                                        = 15 + 3
                      ◄── 18 ────────   = 18
 pen = with_tax(5)    ── net = 5 ───►   5 + 5 * 20 // 100
                                        = 5 + 100 // 100
                                        = 5 + 1
                      ◄── 6 ─────────   = 6
```

The trace table has a column for `net`. It only has a value
**during** a call:

| Step | Who is working | net | Calculation | book | pen |
|------|----------------|-----|-------------|------|-----|
| 1 | the program calls `with_tax(15)` | 15 | | | |
| 2 | `with_tax` | 15 | `15 + 300 // 100` = 18 | | |
| 3 | return: `book` receives 18 | | | 18 | |
| 4 | the program calls `with_tax(5)` | 5 | | 18 | |
| 5 | `with_tax` | 5 | `5 + 100 // 100` = 6 | 18 | |
| 6 | return: `pen` receives 6 | | | 18 | 6 |
| 7 | `print`, with `18 + 6` = 24 | | | 18 | 6 |

```sortie
18 6 24
```

The two calls are independent: the `net` of the first call is forgotten
before the second one. It's like a coffee machine: each cup is
made from scratch.

Try it: add `notebook = with_tax(with_tax(10))`. Follow the **inner**
call first: `with_tax(10)` gives 12, then `with_tax(12)` gives `12 + 240 // 100`,
that is 14.

> **Remember:** at each call, write down the value of the parameter, calculate,
> then bring the result back to where the call was written.

## Program 5: a list, with a condition inside

We count the students who passed (a grade of 10 or more):

```rythme
grades = [12, 7, 15, 9, 10]

fn add_if_passed(tally, grade) =
  if grade >= 10 then tally + 1 else tally

passed = fold(grades, 0, add_if_passed)
print("{passed} passed out of {len(grades)}")
```

**Before running it, guess what it prints.** Watch out for the grade 10.

It's a `fold` (program 3) with a condition (program 2) at each
round. The table combines the two:

| Round | tally (received) | grade | `grade >= 10`? | tally (given) |
|-------|------------------|-------|----------------|---------------|
| start | | | | 0 |
| 1 | 0 | 12 | `true` | 1 |
| 2 | 1 | 7 | `false` | 1 |
| 3 | 1 | 15 | `true` | 2 |
| 4 | 2 | 9 | `false` | 2 |
| 5 | 2 | 10 | `true` | 3 |

```sortie
3 passed out of 5
```

In round 5, `10 >= 10` is true: `>=` means "greater than **or
equal to**". With `>`, the student who got 10 would not be counted.

Once you understand the table, here is the short version. `count` does
the same job in one line:

```rythme
grades = [12, 7, 15, 9, 10]
print(count(grades, fn(g) = g >= 10))
```

```sortie
3
```

Try it: replace `>=` with `>` in `add_if_passed`. Only the row of
round 5 changes in the table (`false`, tally stays 2), and the program
prints `2 passed out of 5`.

> **Remember:** a condition inside a `fold` adds a "question?" column
> at each round. When the answer is `false`, the accumulator
> goes through unchanged.

## Program 6: a recursive function

A [recursive](../debuter/glossaire.md#recursion) function calls
itself, on a shorter list, until the empty list (see
[recursion](recursion.md)). Here is an addition done by hand:

```rythme
fn add_up([], total) = total
fn add_up([x, ..rest], total) = add_up(rest, total + x)

print(add_up([4, 10, 3, 7], 0))
```

**Before running it, guess what it prints.**

For each call, write down **which definition** is chosen, and the values
of the parameters:

| Call | List received | total received | Chosen definition | x | rest | Next call |
|------|---------------|----------------|-------------------|---|------|-----------|
| 1 | `[4, 10, 3, 7]` | 0 | `[x, ..rest]` | 4 | `[10, 3, 7]` | `add_up([10, 3, 7], 4)` |
| 2 | `[10, 3, 7]` | 4 | `[x, ..rest]` | 10 | `[3, 7]` | `add_up([3, 7], 14)` |
| 3 | `[3, 7]` | 14 | `[x, ..rest]` | 3 | `[7]` | `add_up([7], 17)` |
| 4 | `[7]` | 17 | `[x, ..rest]` | 7 | `[]` | `add_up([], 24)` |
| 5 | `[]` | 24 | `[]`: we stop | | | result: 24 |

```sortie
24
```

Compare with program 3: the `total` column is exactly the `acc` column
of the `fold`. A `fold` is this recursion, already written for you.

```text
 add_up([4, 10, 3, 7], 0)
   └─► add_up([10, 3, 7], 4)
         └─► add_up([3, 7], 14)
               └─► add_up([7], 17)
                     └─► add_up([], 24)  ──►  24
```

Try it: call `add_up([], 0)`. The table has only one row: the
first definition is chosen right away, and the program prints `0`.

> **Remember:** for a recursion, one row per call. Write down which
> definition is chosen: that is what decides whether you continue.

## Making the program show its steps

Your table and the program don't agree? Ask the program
to show its steps.

### print, with labels

Add a `print` **with a label**: the name of what you are
printing. A number on its own, among others, says nothing. Inside a `fold`,
a `do ... end` block lets you print, then give the new
accumulator (the last line of the block):

```rythme
grades = [12, 7, 15]
passed = fold(grades, 0, fn(tally, grade) = do
  updated = if grade >= 10 then tally + 1 else tally
  print("tally = {tally}, grade = {grade} -> {updated}")
  updated
end)
print("passed = {passed}")
```

```sortie
tally = 0, grade = 12 -> 1
tally = 1, grade = 7 -> 1
tally = 1, grade = 15 -> 2
passed = 2
```

Each printed line is a row of your table. Compare them one by
one: the first one that differs shows your mistake.

> **Tip:** once you have found it, remove these `print`s. They are for
> searching, not for staying.

### debug: printing without changing anything

`debug(value)` prints a value **and gives it back unchanged**. So you
can add it in the middle of a calculation without breaking it:

```rythme
price = 8
total = debug(price * 3) - 6
print("total = {total}")
```

```sortie
total = 18
```

The program also prints, separately, the position of the `debug` in the file
(line 2, column 9) and the value:

```text
[debug main.ry:2:9] 24
```

This message goes to the **error output**, a channel separate from the normal
output: it doesn't get mixed up with your program's results.

### scan: all the rounds of a fold

You saw it in program 3: replace `fold` with `scan`, and you
get the last column of your table, all at once.

```rythme
grades = [12, 7, 15, 9, 10]
print(scan(grades, 0, fn(t, g) = if g >= 10 then t + 1 else t))
```

```sortie
[1, 1, 2, 2, 3]
```

### rythme steps: the full calculation

`rythme steps file.ry` shows each step of each line, like
your table. With program 1 saved in `shopping.ry`:

```text
$ rythme steps shopping.ry
price = 8

quantity = 3

total = price * quantity
  → 8 * quantity
  → 8 * 3
  → 24

discount = total // 4
  → 24 // 4
  → 6

to_pay = total - discount
  → 24 - discount
  → 24 - 6
  → 18

print("To pay: €{to_pay}")
  → print("To pay: €18")
To pay: €18
  → ()
```

Each `→` is a step: a name replaced by its value, or an
operation done. For a single expression, give it between quotes.
With program 6 in `add_up.ry`:

```text
$ rythme steps add_up.ry "add_up([4, 10, 3, 7], 0)"
add_up([4, 10, 3, 7], 0)
  → add_up([10, 3, 7], 4)
  → add_up([3, 7], 14)
  → add_up([7], 17)
  → add_up([], 24)
  → 24
```

It's exactly the "Next call" column of your table.

> **Remember:** `print` with labels for a few values, `debug`
> in the middle of a calculation, `scan` for a `fold`, `rythme steps` to see
> everything.

### What about a debugger?

Rythme has no debugger that stops the program line by line:
since values never change, `rythme steps` (or the
"Steps" button of the playground, or `:steps` in interactive mode) shows the whole
calculation instead. The chapter [understanding a calculation step by
step](etapes.md) explains all its options, and
[debugging](../debuter/deboguer.md) gives a method to hunt for a bug.

## Common mistakes

These mistakes happen **while filling in the table**. The programs
below show what happens when you write the code with the same
wrong idea.

### Thinking a name changes at each round

You think "at each round, `total` goes up", like a box you
fill:

```rythme norun
purchases = [4, 10, 3]
total = 0
purchases.each do |x|
  total = total + x
end
print(total)
```

```text
warning[W001]: `total` is never used
 --> main.ry:4:3
  |
4 |   total = total + x
  |   ^^^^^
  |
  = help: remove it, or name it `_total` if this is on purpose

0
```

The program prints `0`. Inside the block, `total = total + x` creates a
**new** `total`, which disappears at the end of the block. The outside `total`
is still 0. In the table, the `total` cell of line 2 never
changes. To carry a value from one round to the next, use `fold`:
`total = fold(purchases, 0, fn(acc, x) = acc + x)`.

### Going back to the starting value instead of the last one

In a `fold`, people often pick the wrong column: they start again from the
starting value instead of the one from the round before.

```rythme
purchases = [4, 10, 3, 7]
print(fold(purchases, 0, fn(acc, x) = 0 + x))
```

```sortie
7
```

We wanted 24. Here, each round ignores `acc` and starts again from `0`: the
result is the last element. The table shows it right away: the
"acc (received)" column is never used. Write `acc + x`.

### Forgetting a round, or counting one too many

`1..5` contains **5** numbers: 1 and 5 included. But `slice(list, 1, 3)`
stops **before** position 3, and positions start at 0.

```rythme
print(len(1..5), 1..5)
print(slice([10, 20, 30, 40], 1, 3))
```

```sortie
5 [1, 2, 3, 4, 5]
[20, 30]
```

Before filling in the table, write out the whole list, and number its
positions starting from 0. Count the rows at the end: there must be
one per element.

### Mixing up what is printed and what is returned

`print` **prints** a text, but what it **returns** is `()`, the value
"nothing interesting".

```rythme
x = print("hello")
print(x)
```

```sortie
hello
()
```

In the table, keep two things separate: the column of the value, and
what appears on the screen.

## Test yourself

**Question 1:** fill in the table for this program. What does it print?

```rythme norun
a = 5
b = a * 2
c = b - a
print(a + b + c)
```

> **Answer**
>
> | Line | a | b | c |
> |------|---|---|---|
> | 1 | 5 | | |
> | 2 | 5 | 10 | |
> | 3 | 5 | 10 | 5 |
>
> It prints `20`: 5 + 10 + 5.

**Question 2:** what is `fold([2, 3, 4], 1, fn(acc, x) = acc * x)`?
Make the table of the rounds.

> **Answer**
>
> | Round | acc (received) | x | acc * x |
> |-------|----------------|---|---------|
> | 1 | 1 | 2 | 2 |
> | 2 | 2 | 3 | 6 |
> | 3 | 6 | 4 | 24 |
>
> The result is `24`. We start from 1, not 0: with 0, everything would be
> multiplied by zero.

**Question 3:** with `fn f(n) = if n > 3 then n - 3 else n + 3`, what is
`f(f(2))`?

> **Answer**
>
> Start with the inner call. `f(2)`: the question `2 > 3` answers
> `false`, so 2 + 3 = 5. Then `f(5)`: `5 > 3` answers `true`, so
> 5 - 3 = 2. The result is `2`.

## Summary

- To understand a program, play the computer: one line of code, one
  row of the trace table.
- Guess the output **before** running it, then compare: the gap shows what
  you still need to understand.
- In Rythme, each cell is filled in only once: values never
  change.
- For `fold` and recursion, one row per round or per call; the value
  received is the one from the row above.
- `print` with labels, `debug`, `scan` and `rythme steps` make the
  program show its steps.

Next: [beginner traps in Rythme](pieges.md).
