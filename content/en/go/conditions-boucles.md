# Conditions

This page shows how to compare values and make choices with `if` and `switch`.

> **Goals:**
>
> - you will know how to compare values and combine conditions with `&&`, `||` and `!`;
> - you will know how to write a condition with `if`, `else if` and `else`;
> - you will know how to choose among several cases with `switch`.

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

**Question 2:** what is the difference between `=` and `==`?

> **Answer**
>
> `=` stores a value in a variable. `==` compares two values and gives `true` or
> `false`.

**Question 3:** what does `fmt.Println(5 > 3 && 2 > 4)` print?

> **Answer**
>
> `false`. `5 > 3` is true, but `2 > 4` is false, and `&&` needs both to be true.

## Summary

- `==`, `!=`, `<`, `>`, `<=` and `>=` compare; `&&`, `||` and `!` combine
  conditions.
- `if`, `else if` and `else` make choices; braces are required.
- `if x := ...; condition {` creates a variable that only lives inside the `if`.
- `switch` chooses among several cases, without `break`; without a value, it tests
  conditions.

Next: [loops](boucles.md).
