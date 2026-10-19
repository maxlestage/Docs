# Going further with functions

This page presents more advanced tools for Go functions: a variable number of arguments, functions stored in variables, closures, `defer` and recursion.

> **Goals:**
>
> - you will know how to write a function that accepts a variable number of arguments;
> - you will know how to store a function in a variable and create a closure;
> - you will know how to push an action back to the end with `defer`.

This chapter follows [functions](fonctions.md): you will meet `func`, parameters and `return` again, this time for more flexible uses.

## A variable number of arguments

You don't always know how many values you will receive: two grades today, ten
tomorrow. Instead of writing one function per case, you write a single one that
accepts any quantity.

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

Line by line:

- `numbers ...int`: all the numbers given in the call arrive in the slice `numbers`;
- `total` starts at 0, then the loop adds each number to it, one by one;
- `sum(1, 2)` gives back 3, `sum(1, 2, 3, 4)` gives back 10;
- `sum()` receives nothing: the loop doesn't run, and `total` stays at 0.

To pass an existing slice, add `...` after it: `sum(grades...)`. `fmt.Println` is
variadic too: that's why it accepts any number of values.

**Your turn:** write a function `greetAll(names ...string)` that prints "Hello, …!"
for each name it receives. Call it with three names.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func greetAll(names ...string) {
> 	for _, name := range names {
> 		fmt.Println("Hello,", name+"!")
> 	}
> }
>
> func main() {
> 	greetAll("Alice", "Bob", "Chloe")
> }
> ```
>
> ```sortie
> Hello, Alice!
> Hello, Bob!
> Hello, Chloe!
> ```

## Functions are values

Sometimes you want to write a general function, then tell it **what to do**. It's like
a drill: the same machine drills or screws, depending on the bit you put in.

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

Line by line:

- `operation := double` stores the function itself in a variable, without calling it
  (no parentheses);
- so `operation(5)` calls `double(5)`, which gives back 10;
- `apply` receives a slice and a function `f`. It calls `f` on each number and stores
  the results in a new slice;
- with `double`, `[1 2 3]` becomes `[2 4 6]`.

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

A function may need to **remember** something between two calls, like the counter at
the entrance of a museum, which adds 1 for each visitor.

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

Line by line:

- `counter()` creates a variable `n` set to 0, then gives back an anonymous function;
- `next` receives that function. It keeps access to `n`, even after `counter` has
  ended;
- each call to `next()` adds 1 to `n` and gives it back: 1, then 2, then 3;
- `other := counter()` runs `counter` again, which creates a **new** `n` at 0: the
  first call gives back 1.

Picture a backpack: the returned function keeps `n` in its backpack. Each call to
`next` increases **its** `n`. `other` has its own backpack, with its own `n`.

## defer: pushing back to the end

When you open something, you have to remember to close it. It's like a "turn off the
light" note stuck on the door: you write it when you arrive, and it serves when you
leave.

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

Line by line:

- `main` calls `work`, which first prints "Opening";
- the `defer` line prints nothing right away: it notes "print Closing" for later;
- "Task 1" then "Task 2" are printed;
- `work` reaches its end: the call put aside runs, and "Closing" is printed last.

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

**Your turn:** write a `main` that prints "Goodbye" last, even though you write that
line first. Then add two prints: "Hello" and "How are you?".

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	defer fmt.Println("Goodbye")
> 	fmt.Println("Hello")
> 	fmt.Println("How are you?")
> }
> ```
>
> ```sortie
> Hello
> How are you?
> Goodbye
> ```

## A function that calls itself

> **Note:** this part can wait. Read it once the basics feel solid.

Some problems contain a smaller version of themselves, like Russian dolls: when you
open one doll, you find another, smaller one inside, down to the last.

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

### Forgetting the ... to pass a slice

```go norun
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
	grades := []int{12, 15, 9}
	fmt.Println(sum(grades))
}
```

```text
# command-line-arguments
./main.go:15:18: cannot use grades (variable of type []int) as int value in argument to sum
```

`sum` expects `int`s one by one, not a slice. Add `...` after the slice:
`sum(grades...)`.

### Calling a function instead of passing it

```go norun
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
	fmt.Println(apply([]int{1, 2, 3}, double()))
}
```

```text
# command-line-arguments
./main.go:18:36: not enough arguments in call to double
	have ()
	want (int)
```

With the parentheses, `double()` **calls** the function, here without an argument.
To pass the function itself, write its name alone: `apply([]int{1, 2, 3}, double)`.

### Putting the variadic parameter before the others

```go norun
package main

import "fmt"

func introduce(grades ...int, name string) {
	fmt.Println(name, grades)
}

func main() {
	introduce(12, 15, "Alice")
}
```

```text
# command-line-arguments
./main.go:5:23: can only use ... with final parameter in list
```

The parameter with `...` must be the last one. Write
`func introduce(name string, grades ...int)`, then `introduce("Alice", 12, 15)`.

## Test yourself

**Question 1:** you have `grades := []int{12, 15, 9}` and a function
`sum(numbers ...int) int`. How do you pass `grades` to `sum`?

> **Answer**
>
> Add `...` after the slice: `sum(grades...)`. Without the `...`, Go refuses: it
> expects `int`s, not a slice.

**Question 2:** what does this program print?

```go norun
package main

import "fmt"

func main() {
	total := 0
	add := func(n int) {
		total += n
	}
	add(3)
	add(4)
	fmt.Println(total)
}
```

> **Answer**
>
> `7`. The anonymous function uses the variable `total` around it: each call really
> changes it.

**Question 3:** what does this program print?

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

## Summary

- `...type` accepts a variable number of arguments, received in a slice.
- A function is a value: you store it in a variable, you pass it as an argument.
- A closure keeps the variables around it.
- `defer` pushes a call back to the end of the function, in reverse order.
- A recursive function calls itself and needs a stopping case.

Next: [working with text](texte.md).
