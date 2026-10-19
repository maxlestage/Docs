# Loops

This page shows how to repeat actions with `for`, Go's only loop, and how to go through a list, a string or numbers with `range`.

> **Goals:**
>
> - you will know how to write the three forms of the `for` loop;
> - you will know how to go through a list, a string or numbers with `range`;
> - you will know how to stop or skip a round with `break` and `continue`.

This chapter follows [conditions](conditions-boucles.md): there you learned to ask a question with `if`; here, you will repeat actions as long as a condition is true.

## The for loop

Imagine someone asks you to print the numbers from 1 to 100. Writing 100
`fmt.Println` would be long and boring. A loop does this repetitive work for you,
like the chorus of a song that you sing several times.

A [loop](../debuter/glossaire.md#loop) repeats a block of code. Go has **only
one** loop: `for`. No `while`, no `do`. But `for` can be written in three ways.

### Form 1: the counter

The classic form has three parts, separated by `;`:

```go
package main

import "fmt"

func main() {
	for i := 1; i <= 3; i++ {
		fmt.Println("Round", i)
	}
}
```

```sortie
Round 1
Round 2
Round 3
```

1. `i := 1`: the **start**, run only once;
2. `i <= 3`: the **condition**, tested before each round; if it is false, the loop
   stops;
3. `i++`: the **step**, run at the end of each round.

### Form 2: as long as

With only a condition, `for` plays the role of "as long as" (`while` in other
languages).

```go
package main

import "fmt"

func main() {
	money := 100
	years := 0
	for money < 150 {
		money = money * 110 / 100
		years++
	}
	fmt.Println(years, "years, there are", money, "euros")
}
```

```sortie
5 years, there are 160 euros
```

Line by line:

- at the start, `money` is 100 and `years` is 0;
- `for money < 150 {` asks the question before each round. 100 is less than 150: Go
  enters the block;
- `money * 110 / 100` adds 10%: 100 becomes 110, then 121, 133, 146 and 160 (Go
  computes with whole numbers and drops the decimals);
- `years++` adds 1 to `years` in each round: it counts the rounds;
- when `money` is 160, the answer is no: the loop stops and the last line prints
  the result.

We don't know in advance how many rounds it will take: the loop runs as long as
`money < 150`.

**Your turn:** write a program that starts from `n := 1` and, as long as `n` is less
than 100, prints `n` and then doubles it.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	n := 1
> 	for n < 100 {
> 		fmt.Println(n)
> 		n = n * 2
> 	}
> }
> ```
>
> ```sortie
> 1
> 2
> 4
> 8
> 16
> 32
> 64
> ```

### Form 3: the infinite loop

With nothing at all, `for` runs forever. You get out with `break`.

```go
package main

import "fmt"

func main() {
	n := 1
	for {
		n = n * 2
		if n > 50 {
			break
		}
	}
	fmt.Println(n)
}
```

```sortie
64
```

> **Warning:** if the condition never becomes false (or if `break` never happens),
> the program runs forever. Press `Ctrl+C` in the terminal to stop it.

## Going through with range

Very often, you have a list of things to handle one by one: each item on a shopping
list, each grade on a report card. `range` hands them to you in order, without you
having to count yourself.

`range` goes through the items of a collection, one by one. In each round, it gives
the **index** (the position) and the value.

### A slice

```go
package main

import "fmt"

func main() {
	fruits := []string{"apple", "kiwi", "pear"}
	for i, fruit := range fruits {
		fmt.Println(i, fruit)
	}
}
```

```sortie
0 apple
1 kiwi
2 pear
```

Line by line:

- `fruits` holds a list of three strings;
- in the first round, `range` stores position 0 in `i` and `"apple"` in `fruit`;
- the block prints both. Then Go goes back to the `for`, which gives 1 and
  `"kiwi"`, then 2 and `"pear"`;
- after `"pear"`, there are no items left: the loop stops.

`[]string{...}` is a **slice**, a list of values: you will discover it in
[arrays and slices](collections.md). Positions start at 0.

If you don't need the index, replace it with `_`:

```go
package main

import "fmt"

func main() {
	grades := []int{12, 15, 9}
	total := 0
	for _, grade := range grades {
		total += grade
	}
	fmt.Println("Total:", total)
}
```

```sortie
Total: 36
```

**Your turn:** write a program that goes through the shopping list
`[]string{"bread", "milk", "eggs"}` and prints each item after a dash.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	shopping := []string{"bread", "milk", "eggs"}
> 	for _, item := range shopping {
> 		fmt.Println("-", item)
> 	}
> }
> ```
>
> ```sortie
> - bread
> - milk
> - eggs
> ```

### A string

On a string, `range` gives the position (in bytes) and each character, as a `rune`.

```go
package main

import "fmt"

func main() {
	for i, c := range "Go!" {
		fmt.Printf("%d:%c ", i, c)
	}
	fmt.Println()
}
```

```sortie
0:G 1:o 2:! 
```

### Integers

Since Go 1.22, `range` accepts an integer: `range 3` gives 0, 1, then 2. It is the
shortest way to repeat an action.

```go
package main

import "fmt"

func main() {
	for i := range 3 {
		fmt.Println("Beep", i)
	}
	for range 2 {
		fmt.Println("Hooray!")
	}
}
```

```sortie
Beep 0
Beep 1
Beep 2
Hooray!
Hooray!
```

## break and continue

Sometimes you want to change the course of a loop along the way: stop searching as
soon as you have found your keys, or ignore some items. That is what these two words
are for.

- `break` leaves the loop right away;
- `continue` skips the rest of the round and moves on to the next one.

```go
package main

import "fmt"

func main() {
	for i := range 10 {
		if i%2 == 0 {
			continue
		}
		if i > 7 {
			break
		}
		fmt.Println(i)
	}
}
```

```sortie
1
3
5
7
```

Line by line:

- `for i := range 10 {` gives `i` the values 0, 1, 2… up to 9;
- if `i` is even (the remainder of dividing by 2, `i%2`, is 0), `continue` jumps
  straight to the next round: `fmt.Println` is not reached;
- if `i` is greater than 7, `break` stops the whole loop;
- otherwise, `fmt.Println(i)` prints the number.

Even numbers are skipped by `continue`. When `i` is 9, `break` stops everything.

### Labels

> **Note:** this part can wait. Read it once the basics feel solid.

With two loops one inside the other, `break` only leaves the inner loop. To leave
both, give a name (a **label**) to the outer loop.

```go
package main

import "fmt"

func main() {
search:
	for row := 1; row <= 3; row++ {
		for col := 1; col <= 3; col++ {
			if row*col == 6 {
				fmt.Println("Found:", row, "x", col)
				break search
			}
		}
	}
	fmt.Println("End")
}
```

```sortie
Found: 2 x 3
End
```

`break search` leaves the loop named `search`. `continue search` exists too: it moves
on to the next round of the outer loop.

## Common mistakes

### Writing while

```go norun
package main

import "fmt"

func main() {
	i := 0
	while i < 3 {
		fmt.Println(i)
		i++
	}
}
```

```text
# command-line-arguments
./main.go:7:8: syntax error: unexpected name i at end of statement
./main.go:11:1: syntax error: non-declaration statement outside function body
```

`while` doesn't exist in Go. Write `for i < 3 {`.

### Using the counter after the loop

```go norun
package main

import "fmt"

func main() {
	for i := 0; i < 3; i++ {
		fmt.Println(i)
	}
	fmt.Println(i)
}
```

```text
# command-line-arguments
./main.go:9:14: undefined: i
```

`i` only exists inside the loop. To keep it, create it before:
`i := 0` then `for ; i < 3; i++ {`.

## Test yourself

**Question 1:** print the 7 times table, from `7 x 1 = 7` to `7 x 5 = 35`.

> **Answer**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for i := 1; i <= 5; i++ {
> 		fmt.Printf("7 x %d = %d\n", i, 7*i)
> 	}
> }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

**Question 2:** how do you write a loop "as long as `n` is greater than 1" in Go?

> **Answer**
>
> `for n > 1 { ... }`. The word `while` doesn't exist: `for` with a single condition
> replaces it.

**Question 3:** what does this program print?

```go norun
package main

import "fmt"

func main() {
	for i := range 3 {
		fmt.Println(i)
	}
}
```

> **Answer**
>
> `0`, `1` then `2`, each on its own line. `range 3` starts at 0 and stops before 3.

## Summary

- `for` is the only loop: with a counter, with a condition, or infinite.
- `range` goes through a slice, a string or the integers from 0 to n-1.
- `break` leaves the loop, `continue` moves on to the next round; a label targets an
  outer loop.

Next: [functions](fonctions.md).
