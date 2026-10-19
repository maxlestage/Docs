# Following a program step by step in Go

This page teaches you to run a Go program in your head, line by line, with a table that tracks the value of each variable.

> **Goals:**
>
> - fill in a tracking table (also called a "trace table") for a small
>   program;
> - predict what a program will print **before** you run it;
> - follow a condition, a loop and a function call without getting lost;
> - make the program print its own steps with `fmt.Printf`.

## Playing the computer

A computer doesn't guess anything. It reads a line, does it, then moves on to the next
one. Always in the same order. You can do the same, with a pencil.

Think of a **score sheet** during a board game. Each time a score changes, you cross
out the old one and write the new one. A
[variable](../debuter/glossaire.md#variable) is one line of that sheet: a label (the
name) and a number (the value), which can change.

The **tracking table** is that sheet, laid out in columns:

- one column per variable;
- one row per step (one line of code executed);
- in each cell, the value **after** the step.

```text
 code                    sheet
 ─────────────           ┌───────┬─────┬─────┐
 a := 5        ───────►  │ Step  │  a  │  b  │
 b := 3        ───────►  ├───────┼─────┼─────┤
 a = a + b     ───────►  │   1   │  5  │  —  │
                         │   2   │  5  │  3  │
                         │   3   │  8  │  3  │
                         └───────┴─────┴─────┘
```

The dash `—` means "the variable doesn't exist yet".

The method, always the same:

1. Read the whole program once.
2. **Guess** what it will print. Write it down.
3. Fill in the table, one row at a time. Don't skip anything.
4. Run the program with `go run main.go`, then compare with your guess.

> **Remember:** one line of code = one row of the table. You read the **current**
> value in the table, never the one you "think" it has.

## Program 1: variables that change

```go
package main

import "fmt"

func main() {
	a := 5
	b := 3
	a = a + b
	b = a * 2
	a = a - 1
	fmt.Println(a, b)
}
```

Before running it, guess what gets printed. Write down your answer.

Here is the trace, line by line:

| Step | Line | Calculation | a | b |
|---|---|---|---|---|
| 1 | `a := 5` | — | 5 | — |
| 2 | `b := 3` | — | 5 | 3 |
| 3 | `a = a + b` | 5 + 3 | **8** | 3 |
| 4 | `b = a * 2` | 8 × 2 | 8 | **16** |
| 5 | `a = a - 1` | 8 − 1 | **7** | 16 |
| 6 | `fmt.Println(a, b)` | prints | 7 | 16 |

```sortie
7 16
```

The important point is step 4. `b = a * 2` uses the value of `a` **at that moment**:
8, not 5. The table shows it to you: you just have to read the cell right above.

Read `a = a + b` from **right to left**:

```text
   a  =  a + b
   ▲     └─┬─┘
   │       └── 1. compute with the current values: 5 + 3 = 8
   └────────── 2. store the result in a (the old 5 is erased)
```

Try it: replace `b := 3` with `b := 10`. Redo the table before running it.
The program prints `14 30`.

> **Remember:** the `=` sign doesn't mean "equal" like in maths. It means
> "compute on the right, then store on the left".

## Program 2: a condition

A [condition](../debuter/glossaire.md#condition) is a railway switch: the program
takes only one path. In the table, you write down the answer to the question (true or
false), then you jump straight to the right block.

```go
package main

import "fmt"

func main() {
	temperature := 12
	advice := "Stay warm"
	if temperature > 20 {
		advice = "Go out in a t-shirt"
	} else if temperature > 10 {
		advice = "Take a sweater"
	}
	fmt.Println(temperature, "degrees:", advice)
}
```

Before running it, guess what gets printed.

| Step | Line | Question | Answer | temperature | advice |
|---|---|---|---|---|---|
| 1 | `temperature := 12` | — | — | 12 | — |
| 2 | `advice := ...` | — | — | 12 | Stay warm |
| 3 | `if temperature > 20` | 12 > 20? | false | 12 | Stay warm |
| 4 | `else if temperature > 10` | 12 > 10? | **true** | 12 | Stay warm |
| 5 | `advice = "Take a sweater"` | — | — | 12 | **Take a sweater** |
| 6 | `fmt.Println(...)` | — | — | 12 | Take a sweater |

```sortie
12 degrees: Take a sweater
```

At step 3, the answer is "false": the first block is **skipped**. The line
`advice = "Go out in a t-shirt"` never appears in the table. That's normal: a skipped
line gets no row in the table.

```text
             temperature > 20?
              /            \
           true            false
            │                │
       "t-shirt"     temperature > 10?
                        /          \
                     true          false
                      │              │
                  "sweater"   (nothing: we keep
                               "Stay warm")
```

Try it: set `temperature := 20`. The program prints `20 degrees: Take a sweater`,
because 20 is not **greater** than 20. With `temperature := 5`, it prints
`5 degrees: Stay warm`: no block is taken.

> **Remember:** for a condition, write down the question **and** its answer in the
> table. Then skip the blocks that aren't taken.

## Program 3: a loop that adds up

A [loop](../debuter/glossaire.md#loop) repeats lines. Think of a piggy bank: on each
round, you slip in a coin, and the total grows. The variable `total` is the piggy
bank.

```go
package main

import "fmt"

func main() {
	total := 0
	for i := 1; i <= 5; i++ {
		total = total + i
	}
	fmt.Println("Total:", total)
}
```

Before running it, guess what gets printed.

In the table, one row per **round**. You also write down the question `i <= 5?`,
asked **before** each round.

| Round | i | i <= 5? | Calculation | total |
|---|---|---|---|---|
| start | — | — | — | 0 |
| 1 | 1 | true | 0 + 1 | 1 |
| 2 | 2 | true | 1 + 2 | 3 |
| 3 | 3 | true | 3 + 3 | 6 |
| 4 | 4 | true | 6 + 4 | 10 |
| 5 | 5 | true | 10 + 5 | 15 |
| end | 6 | **false** | exit | 15 |

```sortie
Total: 15
```

Look at the last row: `i` is 6, the question answers "false", and the loop stops
**without** doing any calculation. Many beginners forget this row. It explains why
the loop stops.

```text
        ┌────────────────────────────┐
        ▼                            │
  i <= 5? ── true ──► total = total + i ──► i++
        │
      false
        ▼
  fmt.Println(...)
```

Try it: replace `i <= 5` with `i < 5`. Round 5 disappears. The program prints
`Total: 10`.

> **Remember:** for a loop, one row per round, plus a last row where the question
> becomes false. That row is what stops everything.

## Program 4: a function called twice

A [function](../debuter/glossaire.md#function) is like a coffee machine. You put
something in (the [parameter](../debuter/glossaire.md#parameter)), it works, and it
gives you back a result (`return`). While it works, you wait.

```go
package main

import "fmt"

func square(n int) int {
	fmt.Println("  square receives", n)
	result := n * n
	return result
}

func main() {
	fmt.Println("start")
	x := square(3)
	y := square(x - 5)
	fmt.Println("x =", x, "y =", y)
}
```

Before running it, guess what gets printed. Careful: there are four lines.

For a function, add a "Where?" column. It says which function you're in. The
variables of `square` (`n`, `result`) only exist during the call.

| Step | Where? | Line | n | result | x | y | Prints |
|---|---|---|---|---|---|---|---|
| 1 | main | `fmt.Println("start")` | — | — | — | — | start |
| 2 | main | `x := square(3)`: call | — | — | — | — | |
| 3 | square | receive `n` | 3 | — | — | — | |
| 4 | square | `fmt.Println(...)` | 3 | — | — | — | square receives 3 |
| 5 | square | `result := n * n` | 3 | 9 | — | — | |
| 6 | square | `return result` | 3 | 9 | — | — | |
| 7 | main | store 9 in `x` | — | — | **9** | — | |
| 8 | main | `y := square(x - 5)`: 9 − 5 = 4 | — | — | 9 | — | |
| 9 | square | receive `n` | 4 | — | 9 | — | |
| 10 | square | `fmt.Println(...)` | 4 | — | 9 | — | square receives 4 |
| 11 | square | `result := n * n` | 4 | 16 | 9 | — | |
| 12 | square | `return result` | 4 | 16 | 9 | — | |
| 13 | main | store 16 in `y` | — | — | 9 | **16** | |
| 14 | main | `fmt.Println(...)` | — | — | 9 | 16 | x = 9 y = 16 |

```sortie
start
  square receives 3
  square receives 4
x = 9 y = 16
```

At steps 7 and 13, `n` and `result` go back to `—`: the call is over, its variables
disappear. On the second call, `square` starts from scratch, with a new `n`.

Here is the same path, drawn. The arrows to the right are the calls, the ones to the
left are the returned values:

```text
 main                          square
 ────                          ──────
 Println("start")
 x := square(3)   ── n = 3 ──►  Println(...)
                                result = 9
                  ◄── 9 ─────   return
 y := square(4)   ── n = 4 ──►  Println(...)
                                result = 16
                  ◄── 16 ────   return
 Println(x, y)
```

Try it: replace `square(x - 5)` with `square(x)`. The second call receives 9. The
program prints `square receives 9`, then `x = 9 y = 81`.

> **Remember:** a function call is a round trip. The program goes into the function,
> runs all of it, then comes back **exactly** to the line it left from, with the
> returned value.

## Program 5: a slice and a condition in the loop

Last program: we combine everything. A [slice](../debuter/glossaire.md#slice) of
grades, a `range` loop that reads them one by one, and an `if` inside. Picture a
teacher flipping through exam papers: they look at each grade, and only count the ones
worth at least 10.

```go
package main

import "fmt"

func main() {
	grades := []int{12, 7, 15, 9, 18}
	passed := 0
	sum := 0
	for _, grade := range grades {
		if grade >= 10 {
			passed++
			sum += grade
		}
	}
	fmt.Println("Grades >= 10:", passed)
	fmt.Println("Their sum:", sum)
}
```

Before running it, guess the two lines printed.

| Round | grade | grade >= 10? | passed | sum |
|---|---|---|---|---|
| start | — | — | 0 | 0 |
| 1 | 12 | true | 1 | 12 |
| 2 | 7 | false | 1 | 12 |
| 3 | 15 | true | 2 | 27 |
| 4 | 9 | false | 2 | 27 |
| 5 | 18 | true | 3 | 45 |
| end | — | no more grades | 3 | 45 |

```sortie
Grades >= 10: 3
Their sum: 45
```

In rounds 2 and 4, the answer is "false": `passed` and `sum` **copy** the row above.
Nothing changes. This is often where people go wrong: they add 7 "out of habit".

The `range` loop moves through the slice on its own, like a finger following the
boxes:

```text
 grades:   [ 12 |  7 | 15 |  9 | 18 ]
 round 1:     ▲
 round 2:          ▲
 round 3:               ▲
 round 4:                    ▲
 round 5:                         ▲
```

Try it: change `grade >= 10` to `grade > 12`. Only 15 and 18 get through. The program
prints `Grades >= 10: 2` and `Their sum: 33` (the printed text, however, doesn't
change: remember to fix it too).

> **Remember:** when the condition is false, simply copy the row above. A cell that
> doesn't change is information, not something you forgot.

## Making the program print its steps

You can also have the table written **by the program**. Add an `fmt.Printf` in the
loop, with a label in front of each value:

```go
package main

import "fmt"

func main() {
	grades := []int{12, 7, 15, 9, 18}
	passed := 0
	sum := 0
	for i, grade := range grades {
		if grade >= 10 {
			passed++
			sum += grade
		}
		fmt.Printf("round %d: grade=%d passed=%d sum=%d\n", i+1, grade, passed, sum)
	}
	fmt.Println("Their sum:", sum)
}
```

```sortie
round 1: grade=12 passed=1 sum=12
round 2: grade=7 passed=1 sum=12
round 3: grade=15 passed=2 sum=27
round 4: grade=9 passed=2 sum=27
round 5: grade=18 passed=3 sum=45
Their sum: 45
```

That's exactly the table from the previous section! Three tips:

- **Use labels** (`grade=`): bare numbers (`12 1 12`) are unreadable.
- **Put the `Printf` at the end of the round**, to see the values after the
  calculations.
- `range` counts from 0: we print `i+1` to say "round 1".

Try it: move the `fmt.Printf` **before** the `if`. Each line then shows the values of
the previous round: `round 1: grade=12 passed=0 sum=0`.

> **Remember:** an `fmt.Printf` with labels turns the program into an automatic
> tracking table. Remove it once the bug is found.

## The debugger in two sentences

A debugger, like Delve for Go, pauses the program on a line you choose and shows you
all the variables, then moves forward one line with each command. It's the tracking
table, filled in by the computer: you'll discover it in
[the chapter on debugging](debogage.md).

## Common mistakes

### Reusing an old value

You want to swap `a` and `b`. You think `b` receives the old `a`.

```go
package main

import "fmt"

func main() {
	a := 1
	b := 2
	a = b
	b = a
	fmt.Println(a, b)
}
```

```sortie
2 2
```

The table shows the mistake: after `a = b`, `a` is 2. The 1 is **erased**.
Then `b = a` copies that 2.

| Step | Line | a | b |
|---|---|---|---|
| 1 | `a := 1` | 1 | — |
| 2 | `b := 2` | 1 | 2 |
| 3 | `a = b` | **2** | 2 |
| 4 | `b = a` | 2 | **2** |

The fix: in Go, the swap is done in a single line, `a, b = b, a`. The program then
prints `2 1`.

### Being off by one round

You want to add up 1, 2, 3, 4 and 5. You write `<` instead of `<=`.

```go
package main

import "fmt"

func main() {
	total := 0
	for i := 1; i < 5; i++ {
		total += i
	}
	fmt.Println("Total:", total)
}
```

```sortie
Total: 10
```

In the table, the row where `i` is 5 answers "false" to `5 < 5?`: the 5 is never
added. The fix: `i <= 5`. Always check the **first** and the **last** round.

### Thinking a variable still exists after the loop

`i` is created in the `for` line. It disappears when the loop ends.

```go norun
package main

import "fmt"

func main() {
	total := 0
	for i := 1; i <= 3; i++ {
		total += i
	}
	fmt.Println("Last i:", i)
	fmt.Println("Total:", total)
}
```

```text
# command-line-arguments
./main.go:10:25: undefined: i
```

In the table, put a `—` in the `i` column as soon as you leave the loop. The fix:
delete the line, or create `i` **before** the loop if you need it afterwards.

### Resetting the piggy bank on every round

You want to print the totals 1, 3, 6. But `total := 0` is placed **inside** the loop.

```go
package main

import "fmt"

func main() {
	for i := 1; i <= 3; i++ {
		total := 0
		total += i
		fmt.Println(total)
	}
}
```

```sortie
1
2
3
```

On each round, the line `total := 0` is executed again: the table sets `total` back
to 0 on every round. The fix: put `total := 0` **before** the `for`.

## Test yourself

**1.** What is `x` after these four lines: `x := 4`, `y := x + 1`, `x = y * 2`,
`y = 0`?

> **Answer**
>
> `x` is 10. Step by step: `x` = 4, then `y` = 5, then `x` = 5 × 2 = 10. Setting `y`
> to 0 afterwards doesn't change `x`: the value 10 has already been stored.

**2.** How many times is `Hello` printed with `for i := 0; i < 3; i++`?
What is the value of `i` when the loop stops?

> **Answer**
>
> Three times: for `i` = 0, 1 and 2. The loop stops when `i` is 3, because
> `3 < 3` is false.

**3.** What does this program print?

```go
package main

import "fmt"

func add(a int, b int) int {
	return a + b
}

func main() {
	r := add(2, 3)
	r = add(r, r)
	fmt.Println(r)
}
```

> **Answer**
>
> It prints 10. The first call returns 2 + 3 = 5, stored in `r`. The second call
> receives `a` = 5 and `b` = 5, and returns 10, which replaces the old 5.

## Summary

- A tracking table has one column per variable and one row per step.
- `=` computes on the right with the **current** values, then stores on the left.
- For a condition, write down the question and its answer, then skip the blocks not
  taken.
- For a loop, one row per round, plus the row where the question becomes false.
- A function call is a round trip: its variables are born and disappear with it.
- `fmt.Printf` with labels makes the program write the table.

Next: [beginner traps in Go](pieges.md)
