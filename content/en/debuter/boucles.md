# Loops, step by step

A loop makes the computer repeat an action without copying the code: this page shows, turn by turn, how it moves forward.

> **Goals:**
>
> - understand why we use a loop;
> - follow a "for each" loop turn by turn;
> - count with `range` and know where it stops;
> - calculate a sum, count things, find the largest;
> - write a "while" loop that doesn't run forever;
> - stop a loop before the end with `break`.

## Repeating without copy-pasting

To display "Hello" three times, you can write `print("Hello")` three
times. But what about 1,000 times? Copying isn't a solution.

A [loop](glossaire.md#loop) tells the computer: "do this again several
times". You write the action **only once**.

```python
for turn in range(3):
    print("Hello")
```

```sortie
Hello
Hello
Hello
```

For 1,000 times, you only change `3` to `1000`.

Think of a washing machine: you don't wash each sock by hand. The machine
repeats the same motion for you.

> **Remember:** a loop saves you from copying the same code. You write
> the action once, the computer repeats it.

## The "for each" loop

The simplest loop takes a [list](glossaire.md#list) and goes through
**each item**, one by one. In Python, that's `for`.

```python
fruits = ["apples", "pears", "bananas"]
for fruit in fruits:
    print("I like", fruit)
```

```sortie
I like apples
I like pears
I like bananas
```

Read it out loud: "**for each** fruit **in** fruits, display…".

### What happens, turn by turn

The variable `fruit` is a box. On each turn, the loop puts **a new item**
in it, then runs the indented code.

| Turn | fruit | What is displayed |
|------|-------|-------------------|
| 1 | `"apples"` | I like apples |
| 2 | `"pears"` | I like pears |
| 3 | `"bananas"` | I like bananas |
| end | – | no more items: the loop stops |

A turn is also called an [iteration](glossaire.md#iteration). The word
sounds scary, but it just means "one turn".

### Indentation matters

In Python, the **indented** lines under the `for` are inside the loop. The
other lines come after it.

```python
for fruit in ["apples", "pears"]:
    print("Inside the loop:", fruit)
print("After the loop")
```

```sortie
Inside the loop: apples
Inside the loop: pears
After the loop
```

"After the loop" is displayed **only once**: the line isn't indented. This
shift to the right is called [indentation](glossaire.md#indentation).

Try it: indent the last line by 4 spaces. It is then displayed twice, once
per turn.

> **Remember:** "for each" goes through all the items, one by one, and
> stops on its own at the end of the list.

### The same loop in other languages

The idea is the same everywhere. Only the words change.

```javascript
for (const fruit of ["apples", "pears"]) {
  console.log("I like", fruit);
}
```

```sortie
I like apples
I like pears
```

```ruby
["apples", "pears"].each do |fruit|
  puts "I like #{fruit}"
end
```

```sortie
I like apples
I like pears
```

```go
package main

import "fmt"

func main() {
	for _, fruit := range []string{"apples", "pears"} {
		fmt.Println("I like", fruit)
	}
}
```

```sortie
I like apples
I like pears
```

In Go, `range` gives two things: the position and the item. The `_` means
"the position, I don't need it".

## Counting with range

Often, you don't have a list: you just want to count. `range` builds a
sequence of numbers, a [range](glossaire.md#range).

```python
for i in range(4):
    print(i)
```

```sortie
0
1
2
3
```

Surprise: you get 4 numbers, but from **0 to 3**. Not from 1 to 4.

### Why it stops one before the end

`range(4)` means "4 numbers, starting at 0". The 4 is the **limit**: you
stop **before** it.

Think of a ruler. From 0 to 4, there are 4 centimeters. The "4" mark
shows the end: it isn't one more centimeter.

```text
 0    1    2    3    4
 |----|----|----|----|
  1st  2nd  3rd  4th     ← 4 spaces; the limit 4 isn't one
```

You can also give a start: `range(start, limit)`.

```python
for i in range(1, 4):
    print(i)
```

```sortie
1
2
3
```

So to go **up to** 3, you write `4` as the limit. A third number gives the
step: `range(0, 10, 2)` gives 0, 2, 4, 6, 8.

Try it: write `range(1, 3)`. You'll only get 1 and 2.

> **Remember:** `range(1, 4)` gives 1, 2, 3. The limit is never
> included.

### Counting in other languages

In JavaScript and in Go, you write three pieces: the start, the condition
to keep going, and the step. `for (let i = 1; i < 4; i++)` also gives 1, 2,
3 (`i++` means "add 1 to `i`").

In Ruby, `(1..3)` goes from 1 to 3, end **included** (two dots). With three
dots, `(1...3)`, it stops before, like Python.

## Accumulating: the sum

Here is the most common use of a loop: **adding up** numbers. You keep a
box `total`, and you add each number to it.

It's like a piggy bank. It's empty at the start. With each coin, what's
inside grows.

```python
prices = [3, 5, 2]
total = 0
for p in prices:
    total = total + p
print(total)
```

```sortie
10
```

`total = total + p` reads: "take the old total, add `p`, put the result
in `total`".

| Step | p | total before | total after |
|------|---|--------------|-------------|
| start | – | – | 0 |
| turn 1 | 3 | 0 | 3 |
| turn 2 | 5 | 3 | 8 |
| turn 3 | 2 | 8 | 10 |

People often write `total += p`: it's the same thing, only shorter.

Try it: add `print(total)` **inside** the loop (indented). You see the
piggy bank grow: 3, then 8, then 10.

> **Remember:** for a sum, you put `total = 0` **before** the loop, you
> add **inside** the loop, you display **after**.

## Accumulating: the counter and the largest

A **counter** is a sum where you always add 1. Let's count the grades
above 10.

```python
grades = [12, 8, 15]
count = 0
for grade in grades:
    if grade > 10:
        count += 1
print(count)
```

```sortie
2
```

| Step | grade | grade > 10? | count |
|------|-------|-------------|-------|
| start | – | – | 0 |
| turn 1 | 12 | yes | 1 |
| turn 2 | 8 | no | 1 |
| turn 3 | 15 | yes | 2 |

The `if` is **inside** the loop: the question is asked on every turn.
Conditions are explained in [logic](logique.md).

For the **largest**, you keep "the largest seen so far". You start from
the **first** item, not from 0.

```python
grades = [12, 8, 15]
largest = grades[0]
for grade in grades:
    if grade > largest:
        largest = grade
print(largest)
```

```sortie
15
```

| Step | grade | grade > largest? | largest |
|------|-------|------------------|---------|
| start | – | – | 12 |
| turn 1 | 12 | no | 12 |
| turn 2 | 8 | no | 12 |
| turn 3 | 15 | yes | 15 |

This example is explained in detail, in five languages, in [thinking like
a programmer](algorithmes.md).

## The "while" loop

Sometimes, you don't know **how many** turns to do. You only know **when
to stop**. That's the "while" loop: `while`.

"**While** the water isn't hot, wait." You don't know how many minutes:
you check each time.

```python
counter = 1
while counter <= 3:
    print(counter)
    counter = counter + 1
print("Done")
```

```sortie
1
2
3
Done
```

Before each turn, Python asks the question `counter <= 3`. Yes: it does a
turn. No: it leaves.

| Step | counter | counter <= 3? | Action |
|------|---------|---------------|--------|
| 1 | 1 | yes | displays 1, counter becomes 2 |
| 2 | 2 | yes | displays 2, counter becomes 3 |
| 3 | 3 | yes | displays 3, counter becomes 4 |
| 4 | 4 | no | leaves the loop |

### The infinite loop

If nothing changes in the loop, the question always gets the same answer.
The loop **never** stops: it's an **infinite loop**.

Don't panic: press **Ctrl + C** in the [terminal](terminal.md), and the
program stops. There's an example in the common mistakes, further down.

> **Remember:** in a `while` loop, something must change on each turn,
> otherwise it never stops.

### "While" in other languages

```javascript
let counter = 1;
while (counter <= 2) {
  console.log(counter);
  counter++;
}
```

```sortie
1
2
```

Go has no `while` word: it writes `for counter <= 2 { … }`.

## Stopping early with break

You're looking for your keys in some drawers. As soon as you find them,
you stop. No need to open the next drawers.

That's what `break` does: it **leaves the loop right away**.

```python
drawers = ["pens", "keys", "batteries"]
for drawer in drawers:
    print("Opening:", drawer)
    if drawer == "keys":
        print("Found them!")
        break
print("I'm leaving.")
```

```sortie
Opening: pens
Opening: keys
Found them!
I'm leaving.
```

The "batteries" drawer is never opened. `break` is written the same way in
JavaScript, Ruby and Go.

Try it: replace `"keys"` with `"batteries"` in the `if`. All three drawers
are opened.

> **Remember:** `break` stops the loop immediately. The program carries
> on after the loop.

## Common mistakes

### The infinite loop

You forget to move the counter forward.

```python norun
counter = 1
while counter <= 3:
    print(counter)
```

The program displays `1` endlessly. Press **Ctrl + C**. Python then
displays:

```text
1
1
Traceback (most recent call last):
  File "loop.py", line 3, in <module>
    print(counter)
    ~~~~~^^^^^^^^^
KeyboardInterrupt
```

`KeyboardInterrupt` means "stopped from the keyboard": that's you. To fix
it, add `counter = counter + 1` inside the loop (indented).

### One turn short

You want the numbers from 1 to 3, and you write `range(1, 3)`.

```python
for i in range(1, 3):
    print(i)
```

```sortie
1
2
```

No error message, but the 3 is missing: the limit is never included. Fix:
`range(1, 4)`. This one-turn mistake is so common that it has a name:
*off-by-one*.

### Resetting the total inside the loop

The `total = 0` is indented: it's **inside** the loop.

```python
prices = [3, 5, 2]
for p in prices:
    total = 0
    total = total + p
print(total)
```

```sortie
2
```

You expected 10. On each turn, the piggy bank is emptied, then a single
coin is added. At the end, only the last price is left.

| Turn | p | after `total = 0` | after `total + p` |
|------|---|-------------------|-------------------|
| 1 | 3 | 0 | 3 |
| 2 | 5 | 0 | 5 |
| 3 | 2 | 0 | 2 |

Fix: put `total = 0` **before** the `for`, without indentation.

## Test yourself

**1.** What does `for i in range(3): print(i)` display?

> **Answer**
>
> `0`, then `1`, then `2`, each on its own line. `range(3)` starts at 0
> and stops before 3.

**2.** When should you prefer `while` to `for`?

> **Answer**
>
> When you don't know in advance how many turns to do, but you know when
> to stop. For example: "while the number is smaller than 100".

**3.** With `numbers = [4, 1, 6]`, `total = 0` before the loop and
`total += n` inside it, what values does `total` take?

> **Answer**
>
> 0 at the start, then 4, then 5, then 11. The result is 11.

## Summary

- A loop repeats an action without copying the code.
- `for` goes through each item of a list, one by one.
- `range(1, 4)` gives 1, 2, 3: the limit is never included.
- For a sum: `total = 0` before, `total += x` inside, `print` after.
- `while` repeats as long as a condition is true: something must change, otherwise Ctrl + C.
- `break` leaves the loop right away.

Next: [functions, without the jargon](fonctions.md).
