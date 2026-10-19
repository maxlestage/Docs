# Following a program step by step

This page teaches you to run a Python program in your head, line by line, with a table that tracks the value of each variable, on five small programs that get richer and richer.

> **Goals:**
>
> - read a program in the order Python runs it;
> - fill in a trace table (one column per variable, one row per step);
> - guess the output of a program **before** you run it;
> - follow a function call: who calls whom, and what comes back;
> - make the program print its own steps with `print`.

## Why follow a program by hand?

A computer guesses nothing. It reads your lines **one by one**, from top to bottom, and
does exactly what is written. If you can do the same in your head, you understand any
program, and you find your [bugs](../debuter/glossaire.md#bug) much faster.

Think of a cooking recipe. A good cook does not read the recipe "roughly". They follow
the steps in order, and they look at what is in each bowl after each step. Here, the
bowls are the [variables](../debuter/glossaire.md#variable).

```text
   the recipe (the code)         the bowls (the variables)
   ┌────────────────────┐        ┌─────┐  ┌─────┐
   │ 1. a = 5           │  ───►  │ a 5 │  │ b ? │
   │ 2. b = 3           │        └─────┘  └─────┘
   │ 3. a = a + b       │
   └────────────────────┘
        ▲
        └── the finger: the line being run right now
```

> **Remember:** following a program means moving a finger along the code, one line at
> a time, and writing down what each variable holds after each line.

## The trace table

The basic tool is a table. It is often called a **trace table** (or simply a
*trace*).

- One **column per variable**, plus a "Line" column and a "Prints" column.
- One **row of the table per step**: each time Python runs a line of the code.
- In each cell, the value **after** the step. An empty cell means "unchanged".

A sheet of paper and a pencil are enough. Writing by hand forces you not to skip any
step.

To make things easy to follow, we will write each program twice: first with line
numbers (`L1`, `L2`…), for the table; then for real, to run it.

> **Remember:** one column per variable, one row per step, and always the value
> **after** the step.

## Program 1: variables that change

The first program has no condition and no loop. Only variables that change.

```text
L1  a = 5
L2  b = 3
L3  a = a + b
L4  b = a - b
L5  print(a, b)
```

**Before running it, guess what is printed.** Write your answer on paper.

Here is the trace. On line 3, Python first computes the part **on the right** of the
`=` with the current values, then stores the result in the box on the left.

| Step | Line | Computation | `a` | `b` | Prints |
|---|---|---|---|---|---|
| 1 | L1 | | 5 | | |
| 2 | L2 | | 5 | 3 | |
| 3 | L3 | `5 + 3` | 8 | 3 | |
| 4 | L4 | `8 - 3` | 8 | 5 | |
| 5 | L5 | | 8 | 5 | `8 5` |

The trap is at step 4: `a` is no longer 5, but 8. You must use the value **of the
moment**, not the one from the start.

Run it to check:

```python
a = 5
b = 3
a = a + b
b = a - b
print(a, b)
```

```sortie
8 5
```

Try it: add a line `a = a - b` just before the `print`. Follow it by hand: `a` becomes
`8 - 5`, so `3`. The program prints `3 5`: the starting values have been swapped!

> **Remember:** in `a = a + b`, the right side is computed first, with the values of
> the moment. Only then does `a` change.

## Program 2: a condition

Let's add a [condition](../debuter/glossaire.md#condition). With an `if`, some lines are
**skipped**. The table must show it.

```text
L1  temperature = 12
L2  rain = True
L3  if temperature > 20:
L4      advice = "t-shirt"
L5  elif rain:
L6      advice = "umbrella"
L7  else:
L8      advice = "sweater"
L9  print(advice)
```

**Before running it, guess what is printed.**

For a condition, add a "Test" column: the question asked and its answer, `True` or
`False`.

| Step | Line | Test | `temperature` | `rain` | `advice` | Prints |
|---|---|---|---|---|---|---|
| 1 | L1 | | 12 | | | |
| 2 | L2 | | 12 | True | | |
| 3 | L3 | `12 > 20` → False | | | | |
| 4 | L5 | `rain` → True | | | | |
| 5 | L6 | | | | `"umbrella"` | |
| 6 | L9 | | | | | `umbrella` |

Notice the lines **missing** from the table: L4, L7 and L8. Python does not run them.
It is like a railway switch: only one track is taken.

```text
            temperature > 20 ?
              /          \
           yes            no
            │               │
        "t-shirt"        rain ?
                        /     \
                      yes      no
                       │        │
                 "umbrella"  "sweater"
```

```python
temperature = 12
rain = True
if temperature > 20:
    advice = "t-shirt"
elif rain:
    advice = "umbrella"
else:
    advice = "sweater"
print(advice)
```

```sortie
umbrella
```

Try it: set `temperature = 25`. The first test becomes true: Python takes the first
track, prints `t-shirt`, and does not even look at `rain`.

> **Remember:** in an `if` / `elif` / `else`, Python tests in order and takes **the
> first** true track. The other lines do not appear in the table.

## Program 3: a loop that accumulates

With a [loop](../debuter/glossaire.md#boucle), the same lines come back several times.
In the table, they therefore appear several times, once per round.

```text
L1  total = 0
L2  for n in range(1, 5):
L3      total = total + n * 2
L4  print(total)
```

**Before running it, guess what is printed.** Reminder: `range(1, 5)` gives 1, 2, 3, 4
(the 5 is excluded).

`total` is a piggy bank. It starts empty (0), and each round slips a coin into it.

| Step | Line | `n` | Computation | `total` | Prints |
|---|---|---|---|---|---|
| 1 | L1 | | | 0 | |
| 2 | L2 | 1 | | 0 | |
| 3 | L3 | 1 | `0 + 1 * 2` | 2 | |
| 4 | L2 | 2 | | 2 | |
| 5 | L3 | 2 | `2 + 2 * 2` | 6 | |
| 6 | L2 | 3 | | 6 | |
| 7 | L3 | 3 | `6 + 3 * 2` | 12 | |
| 8 | L2 | 4 | | 12 | |
| 9 | L3 | 4 | `12 + 4 * 2` | 20 | |
| 10 | L2 | end | | 20 | |
| 11 | L4 | | | 20 | `20` |

At step 10, there are no more numbers in `range(1, 5)`: the loop ends, and Python moves
on to the line after the indented block.

```python
total = 0
for n in range(1, 5):
    total = total + n * 2
print(total)
```

```sortie
20
```

Try it: indent `print(total)` by 4 spaces, to put it **inside** the loop. It then runs
on every round: the program prints `2`, `6`, `12` then `20`.

### The same with while

A `while` loop tests its condition **before each round**. Put this test in the table.

```python
money = 10
days = 0
while money > 0:
    money = money - 3
    days = days + 1
print(days, money)
```

```sortie
4 -2
```

| Round | Test `money > 0` | `money` after | `days` after |
|---|---|---|---|
| 1 | `10 > 0` → True | 7 | 1 |
| 2 | `7 > 0` → True | 4 | 2 |
| 3 | `4 > 0` → True | 1 | 3 |
| 4 | `1 > 0` → True | -2 | 4 |
| — | `-2 > 0` → False | end | |

Many people guess `3 1`. The table shows why that is wrong: when 1 is left, the test
`1 > 0` is still true, so a fourth round happens.

> **Remember:** in a loop, you write one row per round. The "after" value of a round
> becomes the "before" value of the next round.

## Program 4: a function called twice

A [function](../debuter/glossaire.md#fonction) is a small machine: you give it
something, it works on its own, then it **returns** a result. When Python meets a call,
it **jumps** into the function, then **comes back** to the place of the call.

```text
L1  def price_with_tax(price):
L2      tax = price * 20 // 100
L3      return price + tax
L4
L5  a = price_with_tax(50)
L6  b = price_with_tax(a)
L7  print(a, b)
```

**Before running it, guess what is printed.**

First thing: lines L1 to L3 **define** the function, they do not run it. It is like
writing a recipe in a notebook: you are not cooking yet. The real work starts at L5.

Here is who calls whom, and what comes back:

```text
  main program                         price_with_tax
  ────────────                         ──────────────
  L5  a = price_with_tax(50)  ── 50 ──►  price = 50
                                         tax = 10
      a = 60                  ◄── 60 ──  return 60

  L6  b = price_with_tax(a)   ── 60 ──►  price = 60
                                         tax = 12
      b = 72                  ◄── 72 ──  return 72

  L7  print(a, b)  →  60 72
```

The table adds a "Where?" column to know whether you are inside the function or
outside.

| Step | Where? | Line | `price` | `tax` | `a` | `b` | Prints |
|---|---|---|---|---|---|---|---|
| 1 | outside | L5 | | | ? | | |
| 2 | function | L1 | 50 | | | | |
| 3 | function | L2 | 50 | 10 | | | |
| 4 | function | L3 | 50 | 10 | | | returns 60 |
| 5 | outside | L5 | | | 60 | | |
| 6 | outside | L6 | | | 60 | ? | |
| 7 | function | L1 | 60 | | | | |
| 8 | function | L2 | 60 | 12 | | | |
| 9 | function | L3 | 60 | 12 | | | returns 72 |
| 10 | outside | L6 | | | 60 | 72 | |
| 11 | outside | L7 | | | 60 | 72 | `60 72` |

Two important details:

- `price` and `tax` only exist **during** the call. At each new call, they start from
  scratch. They are called local variables (see
  [scope](../debuter/glossaire.md#portee)).
- On the second call, the [parameter](../debuter/glossaire.md#parametre) `price`
  receives the value of `a`, that is, 60.

```python
def price_with_tax(price):
    tax = price * 20 // 100
    return price + tax

a = price_with_tax(50)
b = price_with_tax(a)
print(a, b)
```

```sortie
60 72
```

Try it: replace `return price + tax` with `print(price + tax)`. The function prints
`60`, then returns nothing: `a` is `None`, and the second call crashes. See
[the common mistakes](#common-mistakes) below.

> **Remember:** on a call, the finger jumps into the function; at the `return`, it
> comes back to the line of the call with the returned value.

## Program 5: a list and a condition in the loop

Last program: a loop over a [list](../debuter/glossaire.md#liste), with an `if` inside.
Everything you have seen comes together.

```text
L1  grades = [12, 7, 15, 9, 18]
L2  passed = []
L3  for grade in grades:
L4      if grade >= 10:
L5          passed.append(grade)
L6      else:
L7          print(f"{grade}: needs more work")
L8  print(passed)
```

**Before running it, guess what is printed.** Warning: there are `print` calls
**inside** the loop and one `print` **after** it.

| Round | `grade` | Test `grade >= 10` | Action | `passed` after | Prints |
|---|---|---|---|---|---|
| 1 | 12 | True | `append(12)` | `[12]` | |
| 2 | 7 | False | `print` | `[12]` | `7: needs more work` |
| 3 | 15 | True | `append(15)` | `[12, 15]` | |
| 4 | 9 | False | `print` | `[12, 15]` | `9: needs more work` |
| 5 | 18 | True | `append(18)` | `[12, 15, 18]` | |
| after | | | L8 | `[12, 15, 18]` | `[12, 15, 18]` |

Imagine sorting mail: each letter passes in front of you, one at a time. The good ones
go into the `passed` basket, the others get a note right away.

```text
  grades:  12   7   15   9   18
           │    │   │    │   │
           ▼    ▼   ▼    ▼   ▼
         >=10 ? ── yes ──►  basket: [12, 15, 18]
                └─ no ───►  print "needs more work"
```

```python
grades = [12, 7, 15, 9, 18]
passed = []
for grade in grades:
    if grade >= 10:
        passed.append(grade)
    else:
        print(f"{grade}: needs more work")
print(passed)
```

```sortie
7: needs more work
9: needs more work
[12, 15, 18]
```

The messages about 7 and 9 come out **before** the list: they are printed during the
loop, whereas the list is only printed at the end.

Try it: change the test to `grade > 12`. Follow it by hand before running it: 12 no
longer passes, so the program also prints `12: needs more work`, and the list becomes
`[15, 18]`.

> **Remember:** in a loop with an `if`, each round chooses its track. For each round,
> write down the test, its answer, and what changes.

## Making the program print its steps

Sometimes the program is too long for a table on paper. Then ask Python to fill in the
table itself: add a `print` in the loop, with **labels** to know which is which.

```python
total = 0
for n in range(1, 5):
    total = total + n * 2
    print(f"round n={n}: total={total}")
print("end:", total)
```

```sortie
round n=1: total=2
round n=2: total=6
round n=3: total=12
round n=4: total=20
end: 20
```

This is exactly the table of program 3, written by Python.

Python has a handy shortcut: in an f-string, `{x=}` prints the name **and** the value.

```python
def price_with_tax(price):
    tax = price * 20 // 100
    print(f"  in price_with_tax: {price=} {tax=}")
    return price + tax

a = price_with_tax(50)
print(f"{a=}")
b = price_with_tax(a)
print(f"{b=}")
```

```sortie
  in price_with_tax: price=50 tax=10
a=60
  in price_with_tax: price=60 tax=12
b=72
```

The two spaces at the start of the "in price_with_tax" lines show that you are
**inside** the function. You can see the round trip from the drawing of program 4.

Try it: add `print(f"{grade=}")` as the first line of the loop in program 5. You will
see each grade go by, one per line, before the other messages of that round.

> **Tip:** once the bug is found, remove these `print` calls. They are there to
> investigate, not to stay in the final program.

> **Remember:** a `print` with labels, placed in the loop or in the function, turns the
> program into an automatic trace table.

## What about the debugger?

Python also has a [debugger](../debuter/glossaire.md#debogage), `pdb`: write
`breakpoint()` in your code, and the program stops there to let you look at the
variables (`p total`) then move forward one line (`n`). It is your trace table, but
live; everything is explained in [Debugging in Python](debogage.md).

> **Remember:** paper, `print` or debugger: it is always the same idea, looking at the
> variables one step after another.

## Common mistakes

These mistakes often come from a step followed wrongly. The trace table helps you see
them **before** running.

### Doing one round too many

While following, you forget that `range(n)` stops **before** `n`. Here, there is one
round too many:

```python norun
grades = [12, 7, 15]
for i in range(len(grades) + 1):
    print(grades[i])
```

```text
12
7
15
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    print(grades[i])
          ~~~~~~^^^
IndexError: list index out of range
```

The table shows the problem: on round 4, `i` is 3, but the list only has the slots 0,
1 and 2.

| Round | `i` | `grades[i]` |
|---|---|---|
| 1 | 0 | 12 |
| 2 | 1 | 7 |
| 3 | 2 | 15 |
| 4 | 3 | there is no slot 3! |

The fix: go through the list directly, without counting.

```python
grades = [12, 7, 15]
for grade in grades:
    print(grade)
```

```sortie
12
7
15
```

### Using a variable that has no value yet

On the first round, `total` is read. But no line above has given it a value: the first
cell of the `total` column is empty.

```python norun
for n in [3, 4, 5]:
    total = total + n
print(total)
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    total = total + n
            ^^^^^
NameError: name 'total' is not defined
```

The fix: fill the piggy bank **before** the loop.

```python
total = 0
for n in [3, 4, 5]:
    total = total + n
print(total)
```

```sortie
12
```

### Believing a function returns what it prints

While following, you write "`result` is 6" because the function prints 6. But the
function has no `return`: it returns `None`.

```python norun
def double(x):
    print(x * 2)

result = double(3)
print(result + 1)
```

```text
6
Traceback (most recent call last):
  File "main.py", line 5, in <module>
    print(result + 1)
          ~~~~~~~^~~
TypeError: unsupported operand type(s) for +: 'NoneType' and 'int'
```

In the call drawing, the return arrow carries `None`, not 6. The fix:

```python
def double(x):
    return x * 2

result = double(3)
print(result + 1)
```

```sortie
7
```

## Test yourself

**Question 1:** what is the value of `x` at the end?

```python norun
x = 2
x = x * 3
x = x - 1
print(x)
```

> **Answer**
>
> | Step | `x` |
> |---|---|
> | `x = 2` | 2 |
> | `x = x * 3` | 6 |
> | `x = x - 1` | 5 |
>
> The program prints `5`.

**Question 2:** how many times is the word `round` printed, and what is the value of
`n` at the end?

```python norun
n = 1
while n < 20:
    print("round")
    n = n * 3
print(n)
```

> **Answer**
>
> | Test `n < 20` | `n` after the round |
> |---|---|
> | `1 < 20` → True | 3 |
> | `3 < 20` → True | 9 |
> | `9 < 20` → True | 27 |
> | `27 < 20` → False | end |
>
> Three rounds, so `round` three times, then `27`.

**Question 3:** what does this program print?

```python norun
def plus_one(n):
    return n + 1

a = plus_one(1)
b = plus_one(plus_one(a))
print(a, b)
```

> **Answer**
>
> First call: `plus_one(1)` returns 2, so `a = 2`. For `b`, Python first computes the
> **inner** call: `plus_one(2)` returns 3. Then the outer call: `plus_one(3)` returns
> 4. The program prints `2 4`.

## Summary

- Python runs the lines one by one, from top to bottom; follow them with your finger.
- A trace table has one column per variable and one row per step (or per loop round).
- In `x = x + 1`, the right side is computed first, with the value **of the moment**.
- A condition skips lines; a loop repeats lines; a function call jumps into the
  function then comes back with the value of the `return`.
- Guess the output before running, then compare: it is the best training.
- `print(f"{x=}")` makes Python write the table; `breakpoint()` pauses the program.

Next: [Python beginner pitfalls](pieges.md)
