# Interfaces

This page explains how to describe a behavior with an interface, so you can write code that accepts several different types.

> **Goals:**
>
> - you will know how to define an interface and satisfy it without declaring anything;
> - you will know how to store values of different types in the same slice;
> - you will know how to make a type printable with `fmt.Stringer`;
> - you will know how to find the type hidden behind `any` with an assertion or a `switch`;
> - you will know why Go prefers small interfaces.

## What is an interface?

An **interface** is a list of methods. It says **what a type can do**, not what it is.

Think of a power socket. The socket does not ask whether you plug in a lamp, a toaster
or a charger. It only asks: "do you have the right plug?". An interface is that socket:
any type that has the right methods can plug into it.

```go
package main

import "fmt"

type Shape interface {
	Area() float64
}

type Rectangle struct {
	Width, Height float64
}

func (r Rectangle) Area() float64 {
	return r.Width * r.Height
}

func printArea(s Shape) {
	fmt.Println("Area:", s.Area())
}

func main() {
	r := Rectangle{Width: 3, Height: 4}
	printArea(r)
}
```

```sortie
Area: 12
```

The function `printArea` accepts any `Shape`. It does not know `Rectangle`: it only
knows that it can call `Area()`.

## An implicit implementation

Notice that `Rectangle` never says "I implement `Shape`". In Go, this is **implicit**: a
type satisfies an interface as soon as it has all of its methods, with the same names
and the same types.

This is very handy: you can create an interface for types written by someone else,
without changing them. In Rust, on the other hand, you have to write
`impl Trait for Type` (see [traits in Rust](../rust/types.md)).

## Polymorphism

Let's add a second type, `Circle`. It also has an `Area()` method: so it is also a
`Shape`. A `[]Shape` slice can hold rectangles **and** circles.

This fancy word, **polymorphism** ("many shapes"), simply means: the same call
`s.Area()` does a different job depending on the real type of `s`.

```go
package main

import (
	"fmt"
	"math"
)

type Shape interface {
	Area() float64
}

type Rectangle struct {
	Width, Height float64
}

func (r Rectangle) Area() float64 {
	return r.Width * r.Height
}

type Circle struct {
	Radius float64
}

func (c Circle) Area() float64 {
	return math.Pi * c.Radius * c.Radius
}

func main() {
	shapes := []Shape{
		Rectangle{Width: 3, Height: 4},
		Circle{Radius: 1},
		Rectangle{Width: 2, Height: 2},
	}

	total := 0.0
	for _, s := range shapes {
		fmt.Printf("%.2f\n", s.Area())
		total += s.Area()
	}
	fmt.Printf("Total: %.2f\n", total)
}
```

```sortie
12.00
3.14
4.00
Total: 19.14
```

## fmt.Stringer: choosing how a value is printed

The `fmt` package defines a tiny interface:

```go norun
type Stringer interface {
	String() string
}
```

If your type has a `String() string` method, `fmt.Println` and `%v` use it to print the
value. It is like handing over your business card.

```go
package main

import "fmt"

type Temperature struct {
	Degrees float64
}

func (t Temperature) String() string {
	return fmt.Sprintf("%.1f °C", t.Degrees)
}

func main() {
	t := Temperature{Degrees: 21.456}
	fmt.Println(t)
	fmt.Printf("It is %v outside.\n", t)
}
```

```sortie
21.5 °C
It is 21.5 °C outside.
```

## The empty interface: any

An interface with no methods at all, `interface{}`, is satisfied by **every** type. Go
gives it a shorter name: `any`.

```go
package main

import "fmt"

func main() {
	var box any
	box = 42
	fmt.Println(box)
	box = "hello"
	fmt.Println(box)

	mix := []any{1, "two", 3.0, true}
	fmt.Println(len(mix), "elements")
}
```

```sortie
42
hello
4 elements
```

> **Warning:** with `any` you lose the compiler's help: it no longer knows which type is
> stored inside. Use it only when you really need it. `fmt.Println` is a good example:
> it accepts `any` values so it can print anything.

## Type assertions

To get back the value stored in an interface, you write a **type assertion**:
`x.(string)`. It means: "I think `x` contains a `string`".

If you are wrong, the program stops. So use the two-result form, called **comma ok**:
`s, ok := x.(string)`. If the type is right, `ok` is `true`; otherwise `ok` is `false`
and `s` is the zero value.

```go
package main

import "fmt"

func main() {
	var x any = "hello"

	s, ok := x.(string)
	fmt.Println(s, ok)

	n, ok := x.(int)
	fmt.Println(n, ok)

	if text, ok := x.(string); ok {
		fmt.Println("Length:", len(text))
	}
}
```

```sortie
hello true
0 false
Length: 5
```

## The type switch

When there are several possible types, a **type switch** is easier to read. You write
`x.(type)`, only inside a `switch`. In each `case`, the variable has the right type.

```go
package main

import "fmt"

func describe(x any) string {
	switch v := x.(type) {
	case int:
		return fmt.Sprintf("an integer, its double is %d", v*2)
	case string:
		return fmt.Sprintf("a text with %d letters", len(v))
	case bool:
		return "a boolean"
	default:
		return fmt.Sprintf("an unknown type: %T", v)
	}
}

func main() {
	fmt.Println(describe(21))
	fmt.Println(describe("Go"))
	fmt.Println(describe(true))
	fmt.Println(describe(3.5))
}
```

```sortie
an integer, its double is 42
a text with 2 letters
a boolean
an unknown type: float64
```

`%T` prints the name of a value's type.

## Small interfaces

In Go, good interfaces are **small**: often a single method. Their name then often ends
in `-er`: `Stringer`, `Reader`, `Writer`. A Go proverb says: "the bigger the interface,
the weaker the abstraction".

A small interface is easy to satisfy, so many types can plug into it. The best example
is `io.Writer`, "something you can write into":

```go norun
type Writer interface {
	Write(p []byte) (n int, err error)
}
```

A file, the standard output `os.Stdout`, a network connection and a `strings.Builder`
are all `io.Writer`s. A function that writes into an `io.Writer` therefore works with
each of them.

```go
package main

import (
	"fmt"
	"io"
	"os"
	"strings"
)

func writeList(w io.Writer, groceries []string) {
	for i, g := range groceries {
		fmt.Fprintf(w, "%d. %s\n", i+1, g)
	}
}

func main() {
	groceries := []string{"bread", "milk", "apples"}

	writeList(os.Stdout, groceries)

	var b strings.Builder
	writeList(&b, groceries)
	fmt.Println("The builder contains", len(b.String()), "bytes.")
}
```

```sortie
1. bread
2. milk
3. apples
The builder contains 27 bytes.
```

`fmt.Fprintf` is like `fmt.Printf`, but it writes into the given `io.Writer`. The same
code writes once to the screen, once to memory.

> **Tip:** when you write a function, ask for the smallest interface it needs. Here,
> `writeList` asks for an `io.Writer`, not a file: it is easier to reuse and to
> [test](tests.md).

## Common mistakes

### A method is missing

```go norun
package main

import "fmt"

type Shape interface {
	Area() float64
	Perimeter() float64
}

type Square struct {
	Side float64
}

func (s Square) Area() float64 {
	return s.Side * s.Side
}

func main() {
	var sh Shape = Square{Side: 2}
	fmt.Println(sh.Area())
}
```

```text
# command-line-arguments
./main.go:19:17: cannot use Square{…} (value of struct type Square) as Shape value in variable declaration: Square does not implement Shape (missing method Perimeter)
```

The message gives the name of the missing method: `Perimeter`. Add it to `Square`.

### A method with a pointer receiver

```go norun
package main

import "fmt"

type Shape interface {
	Area() float64
}

type Square struct {
	Side float64
}

func (s *Square) Area() float64 {
	return s.Side * s.Side
}

func main() {
	var sh Shape = Square{Side: 2}
	fmt.Println(sh.Area())
}
```

```text
# command-line-arguments
./main.go:18:17: cannot use Square{…} (value of struct type Square) as Shape value in variable declaration: Square does not implement Shape (method Area has pointer receiver)
```

The method is defined on `*Square`, so only `*Square` is a `Shape`. Give a pointer:
`var sh Shape = &Square{Side: 2}`. Or, if the method changes nothing, use a value
receiver: `func (s Square) Area()`.

### An assertion without comma ok

```go norun
package main

import "fmt"

func main() {
	var x any = "hello"
	n := x.(int)
	fmt.Println(n)
}
```

```text
panic: interface conversion: interface {} is string, not int

goroutine 1 [running]:
main.main()
	/path/to/main.go:7 +0x28
exit status 2
```

`x` contains a `string`, not an `int`: the program
[panics](../debuter/glossaire.md#panique). Use `n, ok := x.(int)` and check `ok`.

## Test yourself

**Question 1:** a `Cat` type has a `String() string` method. What does
`fmt.Println(myCat)` do?

> **Answer**
>
> It calls `myCat.String()` and prints the returned text, because `Cat` satisfies the
> `fmt.Stringer` interface.

**Question 2:** what are `v` and `ok` after `v, ok := x.(int)` if `x` contains `"7"`?

> **Answer**
>
> `v` is `0` (the zero value of integers) and `ok` is `false`: `x` contains a text, not
> an integer. The program does not stop.

**Question 3:** write an `Animal` interface with a `Sound() string` method, two types
`Dog` and `Cat`, then print the sound of each one in a loop.

> **Answer**
>
> ```go
> package main
>
> import "fmt"
>
> type Animal interface {
> 	Sound() string
> }
>
> type Dog struct{}
>
> func (Dog) Sound() string { return "Woof" }
>
> type Cat struct{}
>
> func (Cat) Sound() string { return "Meow" }
>
> func main() {
> 	animals := []Animal{Dog{}, Cat{}}
> 	for _, a := range animals {
> 		fmt.Println(a.Sound())
> 	}
> }
> ```
>
> ```sortie
> Woof
> Meow
> ```

## Summary

- An interface is a list of methods; a type satisfies it without declaring it.
- A slice of interfaces stores different types that share a behavior.
- `String() string` makes a type printable by `fmt` (the `fmt.Stringer` interface).
- `any` accepts everything; `v, ok := x.(T)` and `switch x.(type)` find the real type.
- Prefer small interfaces, like `io.Writer`.

Next: [pointers](pointeurs.md).
