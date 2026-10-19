# Functions

This page explains how to write functions in Go with `func`, their parameters, their return values, and more advanced tools like closures and `defer`.

> **Goals:**
>
> - you will know how to write a function with `func` and call it;
> - you will know how to give typed parameters and return one or more values;
> - you will know how to write a function that accepts a variable number of arguments;
> - you will know how to store a function in a variable and create a closure;
> - you will know how to push an action back to the end with `defer`.

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

Writing `func greet()` doesn't run anything: it records the recipe. Writing `greet()`
**calls** the function, that is, runs it.

> **Note:** functions are defined outside `main`, at the file level. The order
> doesn't matter: `main` can call a function written further down.

## Parameters

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

## Returning a value

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

## Several return values

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

## A variable number of arguments

With `...` in front of the type, the last parameter accepts as many arguments as you
like. Inside the function, it becomes a slice. This is called a **variadic**
function.

```go
package main

import "fmt"

func sum(numbers ...int) int {
	total := 0
	for _, n := range numbers {
		total += n
	}
	return total
}

func main() {
	fmt.Println(sum(1, 2))
	fmt.Println(sum(1, 2, 3, 4))
	fmt.Println(sum())

	grades := []int{12, 15, 9}
	fmt.Println(sum(grades...))
}
```

```sortie
3
10
0
36
```

To pass an existing slice, add `...` after it: `sum(grades...)`. `fmt.Println` is
variadic too: that's why it accepts any number of values.

## Functions are values

In Go, a function is a value like any other. You can store it in a variable, or pass
it as an argument to another function.

```go
package main

import "fmt"

func double(x int) int {
	return x * 2
}

func apply(numbers []int, f func(int) int) []int {
	result := []int{}
	for _, n := range numbers {
		result = append(result, f(n))
	}
	return result
}

func main() {
	operation := double
	fmt.Println(operation(5))
	fmt.Println(apply([]int{1, 2, 3}, double))
}
```

```sortie
10
[2 4 6]
```

The type `func(int) int` reads: "a function that takes an `int` and returns an
`int`".

### Anonymous functions

A function can be written without a name, right where you need it: it is an
**anonymous** function.

```go
package main

import "fmt"

func main() {
	triple := func(x int) int {
		return x * 3
	}
	fmt.Println(triple(7))

	func() {
		fmt.Println("Run right away!")
	}()
}
```

```sortie
21
Run right away!
```

The parentheses `()` at the end of the second function call it immediately.

## Closures

An anonymous function can use the variables around it. It **takes them along**, even
when it leaves the place where it was born. It is then called a
[closure](../debuter/glossaire.md#closure).

```go
package main

import "fmt"

func counter() func() int {
	n := 0
	return func() int {
		n++
		return n
	}
}

func main() {
	next := counter()
	fmt.Println(next())
	fmt.Println(next())
	fmt.Println(next())

	other := counter()
	fmt.Println(other())
}
```

```sortie
1
2
3
1
```

Picture a backpack: the returned function keeps `n` in its backpack. Each call to
`next` increases **its** `n`. `other` has its own backpack, with its own `n`.

## defer: pushing back to the end

`defer` pushes a function call back to the moment the current function ends.

```go
package main

import "fmt"

func work() {
	fmt.Println("Opening")
	defer fmt.Println("Closing")
	fmt.Println("Task 1")
	fmt.Println("Task 2")
}

func main() {
	work()
}
```

```sortie
Opening
Task 1
Task 2
Closing
```

This is very useful for "tidying up": you write the closing of a file right next to
its opening, and you are sure it will happen, even if the function leaves early.

Several `defer`s run in **reverse** order, like a stack of plates: the last one put
down is the first one picked up.

```go
package main

import "fmt"

func main() {
	for i := range 3 {
		defer fmt.Println("defer", i)
	}
	fmt.Println("end of main")
}
```

```sortie
end of main
defer 2
defer 1
defer 0
```

## A function that calls itself

A function can call itself: this is
[recursion](../debuter/glossaire.md#recursion). It needs a **stopping case**,
otherwise it never stops.

```go
package main

import "fmt"

func factorial(n int) int {
	if n <= 1 {
		return 1
	}
	return n * factorial(n-1)
}

func main() {
	fmt.Println(factorial(5))
}
```

```sortie
120
```

`factorial(5)` is `5 * factorial(4)`, which is `4 * factorial(3)`… down to
`factorial(1)`, the stopping case, which is 1. The result is 5 × 4 × 3 × 2 × 1 = 120.

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

**Question 1:** what does this program print?

```go norun
package main

import "fmt"

func main() {
	defer fmt.Println("A")
	fmt.Println("B")
	defer fmt.Println("C")
}
```

> **Answer**
>
> `B`, then `C`, then `A`. `B` is printed right away; the `defer`s wait for the end
> of `main` and run in reverse order.

**Question 2:** write a function `average(a, b int) float64` that returns the
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

**Question 3:** what is the difference between a parameter and an argument?

> **Answer**
>
> The parameter is the name written in `func` (`firstName string`). The argument is
> the value given in the call (`"Alice"`).

## Summary

- A function is written `func name(parameter type) returnType { ... }` and returns
  with `return`.
- A function can return several values; the last one is often an `error`.
- `...type` accepts a variable number of arguments, received in a slice.
- A function is a value: you store it in a variable, you pass it as an argument.
- A closure keeps the variables around it.
- `defer` pushes a call back to the end of the function, in reverse order.

Next: [working with text](texte.md). Compare with
[functions in Rust](../rust/fonctions.md).
