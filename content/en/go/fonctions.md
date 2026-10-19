# Functions

This page explains how to write functions in Go with `func`, give them parameters, and make them return one or more values.

> **Goals:**
>
> - you will know how to write a function with `func` and call it;
> - you will know how to give typed parameters and return one or more values;
> - you will know how to leave a function early with `return`.

## Defining a function

A [function](../debuter/glossaire.md#function) is a named piece of code that you can
reuse. It's like a recipe: you write it once, then you follow it as many times as you
like. It starts with the word `func`.

```go
package main

import "fmt"

func greet() {
	fmt.Println("Hello!")
}

func main() {
	greet()
	greet()
}
```

```sortie
Hello!
Hello!
```

Line by line:

- `func greet() { ... }` records the recipe under the name `greet`. Nothing is shown
  yet;
- the program always starts with `main`;
- the first `greet()` runs the recipe: "Hello!" is shown;
- the second `greet()` runs it a second time.

Writing `func greet()` doesn't run anything: it records the recipe. Writing `greet()`
**calls** the function, that is, runs it.

> **Note:** functions are defined outside `main`, at the file level. The order
> doesn't matter: `main` can call a function written further down.

## Parameters

A pancake recipe can serve 4 or 8 people: it's the same recipe, with a different
quantity. Parameters let a function work with different values at each call.

[Parameters](../debuter/glossaire.md#parameter) are the values the function receives.
They are empty slots, filled in at call time. The values given in the call are
called [arguments](../debuter/glossaire.md#argument).

In Go, every parameter has a **type**, written **after** its name.

```go
package main

import "fmt"

func greet(firstName string) {
	fmt.Println("Hello,", firstName)
}

func main() {
	greet("Alice")
	greet("Bob")
}
```

```sortie
Hello, Alice
Hello, Bob
```

Line by line:

- `func greet(firstName string)` announces an empty slot named `firstName`, which will
  receive text;
- `greet("Alice")` fills the slot: inside the function, `firstName` is `"Alice"`;
- `greet("Bob")` runs the function again, and this time `firstName` is `"Bob"`.

Several parameters are separated by commas. If they have the same type, you can write
it only once, at the end.

```go
package main

import "fmt"

func introduce(name string, age int) {
	fmt.Printf("%s is %d years old.\n", name, age)
}

func area(width, height int) {
	fmt.Println("Area:", width*height)
}

func main() {
	introduce("Alice", 12)
	area(3, 4)
}
```

```sortie
Alice is 12 years old.
Area: 12
```

`width, height int` means: `width` and `height` are two `int`s.

**Your turn:** write a function `announce(dish string, price int)` that prints
"Today's special: soup, 6 euros.". Call it twice from `main`, with two different
dishes.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func announce(dish string, price int) {
> 	fmt.Printf("Today's special: %s, %d euros.\n", dish, price)
> }
>
> func main() {
> 	announce("soup", 6)
> 	announce("lasagna", 12)
> }
> ```
>
> ```sortie
> Today's special: soup, 6 euros.
> Today's special: lasagna, 12 euros.
> ```

## Returning a value

Showing a result is not always enough. Often, you want to **keep** it to use it
later, like the number you read on a calculator before going on with your sum.

For a function to **give** a result, write its type after the parentheses, and use
`return`.

```go
package main

import "fmt"

func square(x int) int {
	return x * x
}

func main() {
	fmt.Println(square(4))
	result := square(3) + 1
	fmt.Println(result)
}
```

```sortie
16
10
```

Line by line:

- `func square(x int) int`: the function receives an `int` and promises to give back
  an `int`;
- `return x * x` computes the result and gives it back to the caller;
- so `square(4)` is 16, and `fmt.Println` prints it;
- `square(3) + 1` gives `9 + 1`: the variable `result` holds 10.

The returned value replaces the call: `square(3) + 1` becomes `9 + 1`.

`return` also stops the function right away. You use it to leave early:

```go
package main

import "fmt"

func verdict(grade int) string {
	if grade >= 16 {
		return "very good"
	}
	if grade >= 10 {
		return "passed"
	}
	return "failed"
}

func main() {
	fmt.Println(verdict(17))
	fmt.Println(verdict(11))
	fmt.Println(verdict(4))
}
```

```sortie
very good
passed
failed
```

> **Remember:** a function that announces a return type must **always** end with a
> `return`, whatever path is taken. Otherwise, the compiler refuses.

**Your turn:** write a function `perimeter(side int) int` that returns the perimeter of
a square (4 times the side). In `main`, print `perimeter(5)`, then `perimeter(2) + 1`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func perimeter(side int) int {
> 	return 4 * side
> }
>
> func main() {
> 	fmt.Println(perimeter(5))
> 	fmt.Println(perimeter(2) + 1)
> }
> ```
>
> ```sortie
> 20
> 9
> ```

## Several return values

Some questions have two answers. Sharing 17 sweets between 5 children gives a number
of sweets per child, and a remainder. Go can give back both at once.

A Go function can return **several** values. Write their types in parentheses.

```go
package main

import "fmt"

func divide(a, b int) (int, int) {
	return a / b, a % b
}

func main() {
	quotient, remainder := divide(17, 5)
	fmt.Println(quotient, remainder)

	q, _ := divide(20, 3)
	fmt.Println(q)
}
```

```sortie
3 2
6
```

Line by line:

- `(int, int)` announces that `divide` gives back two integers;
- `return a / b, a % b` gives back the quotient then the remainder, separated by a
  comma;
- `quotient, remainder := divide(17, 5)` stores 3 in `quotient` and 2 in `remainder`;
- `q, _ := divide(20, 3)` keeps the quotient, 6, and throws away the remainder.

The `_` throws away the value you don't need.

This is how Go reports errors: the last value returned is often an `error`, which is
`nil` if everything is fine.

```go
package main

import (
	"errors"
	"fmt"
)

func intSqrt(n int) (int, error) {
	if n < 0 {
		return 0, errors.New("negative number")
	}
	r := 0
	for (r+1)*(r+1) <= n {
		r++
	}
	return r, nil
}

func main() {
	r, err := intSqrt(20)
	fmt.Println(r, err)
	r, err = intSqrt(-4)
	fmt.Println(r, err)
}
```

```sortie
4 <nil>
0 negative number
```

Errors have their own chapter: [error handling](erreurs.md).

### Named results

> **Note:** this part can wait. Read it once the basics feel solid.

You can give names to the return values. They are then variables, created with their
zero value. A bare `return` returns their current values.

```go
package main

import "fmt"

func minMax(numbers []int) (small, big int) {
	small, big = numbers[0], numbers[0]
	for _, n := range numbers {
		small = min(small, n)
		big = max(big, n)
	}
	return
}

func main() {
	s, b := minMax([]int{4, 9, 1, 7})
	fmt.Println(s, b)
}
```

```sortie
1 9
```

> **Tip:** names help document what the function returns. But a bare `return` in a
> long function quickly becomes hard to read: keep it for short functions.

## Common mistakes

### Forgetting a return

```go norun
package main

import "fmt"

func sign(n int) string {
	if n >= 0 {
		return "positive"
	}
}

func main() {
	fmt.Println(sign(3))
}
```

```text
# command-line-arguments
./main.go:9:1: missing return
```

If `n` is negative, the function has nothing to return. Add a `return "negative"`
before the final brace.

### Forgetting an argument

```go norun
package main

import "fmt"

func area(width, height int) int {
	return width * height
}

func main() {
	fmt.Println(area(3))
}
```

```text
# command-line-arguments
./main.go:10:19: not enough arguments in call to area
	have (number)
	want (int, int)
```

The function wants two `int`s (*want*), it gets a single number (*have*). Give them
all: `area(3, 4)`.

### Receiving one value instead of two

```go norun
package main

import "fmt"

func divide(a, b int) (int, int) {
	return a / b, a % b
}

func main() {
	q := divide(7, 2)
	fmt.Println(q)
}
```

```text
# command-line-arguments
./main.go:10:7: assignment mismatch: 1 variable but divide returns 2 values
```

You need one variable per returned value: `q, r := divide(7, 2)`, or `q, _ :=` to
ignore the rest.

### Passing an argument of the wrong type

```go norun
package main

import "fmt"

func double(n int) int {
	return n * 2
}

func main() {
	fmt.Println(double("4"))
}
```

```text
# command-line-arguments
./main.go:10:21: cannot use "4" (untyped string constant) as int value in argument to double
```

`"4"` is text, not a number. Write `double(4)`, or convert the text with
`strconv.Atoi`.

## Test yourself

**Question 1:** write a function `average(a, b int) float64` that returns the
average of two integers.

> **Answer**
>
> ```go
> package main
>
> import "fmt"
>
> func average(a, b int) float64 {
> 	return float64(a+b) / 2
> }
>
> func main() {
> 	fmt.Println(average(3, 4))
> }
> ```
>
> ```sortie
> 3.5
> ```
>
> The conversion `float64(...)` avoids integer division.

**Question 2:** what is the difference between a parameter and an argument?

> **Answer**
>
> The parameter is the name written in `func` (`firstName string`). The argument is
> the value given in the call (`"Alice"`).

**Question 3:** what does the `_` do in `q, _ := divide(20, 3)`?

> **Answer**
>
> It throws away the second returned value, the remainder. Go asks for one variable
> per returned value: `_` takes the place of the one you don't need.

## Summary

- A function is written `func name(parameter type) returnType { ... }` and returns
  with `return`.
- A parameter's type is written after its name; `width, height int` gives both the
  same type.
- `return` gives back the value and stops the function right away.
- A function can return several values; the last one is often an `error`.

Next: [going further with functions](fonctions-avancees.md). Compare with
[functions in Rust](../rust/fonctions.md).
