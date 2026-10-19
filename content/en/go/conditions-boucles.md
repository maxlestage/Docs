# Conditions and loops

This page shows how to make choices with `if` and `switch`, and how to repeat actions with `for`, Go's only loop.

> **Goals:**
>
> - you will know how to write a condition with `if`, `else if` and `else`;
> - you will know how to choose among several cases with `switch`;
> - you will know how to write the three forms of the `for` loop;
> - you will know how to go through a list, a string or numbers with `range`;
> - you will know how to stop or skip a round with `break` and `continue`.

## Comparing values

A program often has to decide: open the door or not, show "you win" or "you
lose". To decide, it first asks a question, just like you ask yourself "do I have
enough money for this cake?".

A [condition](../debuter/glossaire.md#condition) is a question whose answer is true
(`true`) or false (`false`). You build it with comparison operators.

| Operator | Meaning | Example | Result |
|-----------|------|---------|----------|
| `==` | equal to | `3 == 3` | `true` |
| `!=` | not equal to | `3 != 4` | `true` |
| `<`, `>` | less than, greater than | `2 > 5` | `false` |
| `<=`, `>=` | less than or equal, greater than or equal | `5 >= 5` | `true` |

You combine conditions with logical operators:

- `&&`: "and" (both must be true);
- `||`: "or" (at least one must be true);
- `!`: "not" (swaps true and false).

```go
package main

import "fmt"

func main() {
	age := 15
	withParent := true
	fmt.Println(age >= 18)
	fmt.Println(age >= 18 || withParent)
	fmt.Println(age > 12 && age < 18)
	fmt.Println(!withParent)
}
```

```sortie
false
true
true
false
```

Line by line:

- `age := 15` stores 15 in `age`, and `withParent := true` stores "true" in
  `withParent`;
- `age >= 18` asks "is 15 greater than or equal to 18?". No: `false`;
- with `||`, one true answer is enough: `withParent` is `true`, so `true`;
- with `&&`, both are needed: 15 is greater than 12 **and** less than 18, so `true`;
- `!withParent` flips `true`: you get `false`.

> **Warning:** `=` stores a value, `==` compares two values. Don't mix them up.

## if and else

In life, you make choices all the time: if it rains, you take an umbrella;
otherwise, you leave it. `if` gives your program this power: do one thing or another
depending on the situation.

`if` runs a block only if the condition is true. `else` gives the block to run
otherwise.

```go
package main

import "fmt"

func main() {
	temperature := 28
	if temperature > 25 {
		fmt.Println("It's hot.")
	} else {
		fmt.Println("It's nice.")
	}
}
```

```sortie
It's hot.
```

Line by line:

- `temperature := 28` stores 28 in the variable `temperature`;
- `if temperature > 25 {` asks the question "is 28 greater than 25?". The answer
  is yes;
- so Go runs the first block and prints "It's hot.";
- the `else` block is skipped: it is only used when the answer is no.

In Go:

- no parentheses around the condition;
- the braces `{ }` are **required**, even for a single line;
- `else` goes on the same line as the closing brace: `} else {`.

### else if

For more than two cases, chain them with `else if`. Go tests the conditions in order
and stops at the first true one.

```go
package main

import "fmt"

func main() {
	grade := 14
	if grade >= 16 {
		fmt.Println("Very good")
	} else if grade >= 12 {
		fmt.Println("Good")
	} else if grade >= 10 {
		fmt.Println("Pass")
	} else {
		fmt.Println("Fail")
	}
}
```

```sortie
Good
```

`grade >= 10` is true as well, but Go doesn't even test it: it has already found
`grade >= 12`.

**Your turn:** write a program that stores `10` in a variable `age`, then prints
"Child" if the age is less than 13, "Teenager" if it is less than 18, and "Adult"
otherwise.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	age := 10
> 	if age < 13 {
> 		fmt.Println("Child")
> 	} else if age < 18 {
> 		fmt.Println("Teenager")
> 	} else {
> 		fmt.Println("Adult")
> 	}
> }
> ```
>
> ```sortie
> Child
> ```

### if with an initialization statement

> **Note:** this part can wait. Read it once the basics feel solid.

Go lets you put a small statement **before** the condition, separated by a `;`. The
variable created there only exists inside the `if` and its `else` blocks.

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	if n, err := strconv.Atoi("42"); err != nil {
		fmt.Println("Not a number:", err)
	} else {
		fmt.Println("Twice that is", n*2)
	}
}
```

```sortie
Twice that is 84
```

This form is very common in Go, especially to test an error. It keeps temporary
variables where they are used.

## switch

Think of a drinks machine: depending on the button you press, it serves a different
drink. `switch` works the same way: depending on the value, it picks a block.

When the same value can take several forms, `switch` is easier to read than a long
series of `else if`.

```go
package main

import "fmt"

func main() {
	day := "saturday"
	switch day {
	case "saturday", "sunday":
		fmt.Println("It's the weekend!")
	case "wednesday":
		fmt.Println("No school in the afternoon.")
	default:
		fmt.Println("Off to work.")
	}
}
```

```sortie
It's the weekend!
```

Line by line:

- `day` holds the text `"saturday"`;
- `switch day {` compares the value of `day` with each `case`, from top to bottom;
- the first `case` lists `"saturday"` and `"sunday"`: `"saturday"` matches, so Go
  prints "It's the weekend!";
- the other cases, and `default`, are ignored.

Remember three things about `switch`:

- Each `case` gives one or more values, separated by commas.
- `default` runs if no case matches.
- Go stops at the **first** matching case, then leaves the `switch`.

> **Note:** unlike C or Java, there's no need for `break` at the end of each case:
> Go doesn't "fall" into the next case. To force it, there is the word
> `fallthrough`, but it is almost never used.

**Your turn:** write a program that stores `"lemon"` in a variable `fruit`, then,
with a `switch`, prints "Orchard fruit" for an apple or a pear, "Citrus" for a
lemon or an orange, and "Unknown fruit" otherwise.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	fruit := "lemon"
> 	switch fruit {
> 	case "apple", "pear":
> 		fmt.Println("Orchard fruit")
> 	case "lemon", "orange":
> 		fmt.Println("Citrus")
> 	default:
> 		fmt.Println("Unknown fruit")
> 	}
> }
> ```
>
> ```sortie
> Citrus
> ```

### switch without a value

A `switch` without a value tests conditions, like a `switch true`. It is a nice way
to write a series of `else if`.

```go
package main

import "fmt"

func main() {
	hour := 14
	switch {
	case hour < 12:
		fmt.Println("Good morning")
	case hour < 18:
		fmt.Println("Good afternoon")
	default:
		fmt.Println("Good evening")
	}
}
```

```sortie
Good afternoon
```

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

### Using a number as a condition

```go norun
package main

import "fmt"

func main() {
	n := 3
	if n {
		fmt.Println("true")
	}
}
```

```text
# command-line-arguments
./main.go:7:5: non-boolean condition in if statement
```

In Go, a condition must be a real boolean. `3` is neither true nor false. Write the
comparison: `if n != 0 {`.

### Putting else on a new line

```go norun
package main

import "fmt"

func main() {
	age := 20
	if age >= 18 {
		fmt.Println("adult")
	}
	else {
		fmt.Println("minor")
	}
}
```

```text
# command-line-arguments
./main.go:10:2: syntax error: unexpected keyword else, expected }
```

`else` must come right after the closing brace: `} else {`.

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

**Question 1:** what does this program print?

```go norun
package main

import "fmt"

func main() {
	x := 7
	switch {
	case x > 5:
		fmt.Println("big")
	case x > 2:
		fmt.Println("medium")
	}
}
```

> **Answer**
>
> `big`, and nothing else. Go stops at the first true case and doesn't go on to the
> next one.

**Question 2:** print the 7 times table, from `7 x 1 = 7` to `7 x 5 = 35`.

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

**Question 3:** how do you write a loop "as long as `n` is greater than 1" in Go?

> **Answer**
>
> `for n > 1 { ... }`. The word `while` doesn't exist: `for` with a single condition
> replaces it.

## Summary

- `if`, `else if` and `else` make choices; braces are required.
- `if x := ...; condition {` creates a variable that only lives inside the `if`.
- `switch` chooses among several cases, without `break`; without a value, it tests
  conditions.
- `for` is the only loop: with a counter, with a condition, or infinite.
- `range` goes through a slice, a string or the integers from 0 to n-1.
- `break` leaves the loop, `continue` moves on to the next round; a label targets an
  outer loop.

Next: [functions](fonctions.md).
