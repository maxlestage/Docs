# Generics

This page shows how to write a single function or a single type that works with several types, thanks to type parameters.

> **Goals:**
>
> - you will know how to write a generic function with `[T any]`;
> - you will know how to limit the accepted types with a constraint (`comparable`, `cmp.Ordered`);
> - you will know how to write your own constraint with `~int | ~float64`;
> - you will know how to create a generic type, such as a stack;
> - you will know when generics are not useful.

## The problem: writing the same thing twice

Imagine a function that returns the larger of two integers. Then you need the same thing
for decimal numbers, then for strings. Without generics, you have to write three almost
identical functions:

```go norun
func MaxInt(a, b int) int { ... }
func MaxFloat(a, b float64) float64 { ... }
func MaxString(a, b string) string { ... }
```

A [generic](../debuter/glossaire.md#generics) solves this problem: you write the code
once, with a **type parameter** in place of the real type. It's like a cake mold: the
shape is the same, whether you pour in chocolate or vanilla.

## A first generic function

The type parameter is written between square brackets, right after the function name:
`[T any]`. `T` is a placeholder type name; `any` says that `T` can be any type.

```go
package main

import "fmt"

func First[T any](items []T) T {
	return items[0]
}

func main() {
	fmt.Println(First([]int{4, 8, 15}))
	fmt.Println(First([]string{"apple", "pear"}))
	fmt.Println(First[float64]([]float64{1.5, 2.5}))
}
```

```sortie
4
apple
1.5
```

Go **infers** `T` from the arguments: with `[]int`, `T` is `int`. You can also give it
yourself, as in `First[float64](...)`.

## Constraints

With `any`, Go knows nothing about `T`. So you cannot compare two `T` values with `>`:
some types cannot be compared. A **constraint** limits the accepted types, and in
exchange it allows operations.

### comparable

`comparable` accepts the types that support `==` and `!=`: numbers, strings,
booleans, simple structs…

```go
package main

import "fmt"

func Contains[T comparable](items []T, target T) bool {
	for _, e := range items {
		if e == target {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println(Contains([]int{1, 2, 3}, 2))
	fmt.Println(Contains([]string{"red", "green"}, "blue"))
}
```

```sortie
true
false
```

### cmp.Ordered

The `cmp` package provides `cmp.Ordered`: the types that support `<`, `<=`, `>` and
`>=`. These are integers, floats and strings.

```go
package main

import (
	"cmp"
	"fmt"
)

func Max[T cmp.Ordered](a, b T) T {
	if a > b {
		return a
	}
	return b
}

func main() {
	fmt.Println(Max(3, 7))
	fmt.Println(Max(2.5, 1.5))
	fmt.Println(Max("apricot", "banana"))
}
```

```sortie
7
2.5
banana
```

Strings are compared in alphabetical order: `"banana"` comes after `"apricot"`.

> **Note:** since Go 1.21, `max` and `min` already exist in the language, and
> `slices.Max` finds the largest element of a slice. `Max` is an exercise here.

## Writing your own constraint

A constraint is an interface. It can list types separated by `|`
("or"). The `~` in front of a type means: "this type, or any type built on it".
So `~int` accepts `int`, but also a type `type Age int`.

```go
package main

import "fmt"

type Number interface {
	~int | ~float64
}

func Sum[T Number](values []T) T {
	var total T
	for _, v := range values {
		total += v
	}
	return total
}

type Euros float64

func main() {
	fmt.Println(Sum([]int{1, 2, 3}))
	fmt.Println(Sum([]float64{0.5, 0.25}))
	fmt.Println(Sum([]Euros{9.99, 0.01}))
}
```

```sortie
6
0.75
10
```

`var total T` creates the zero value of `T`: `0` for a number. Without the `~`, the line
with `Euros` would be rejected.

## Map and Filter

Generics shine for functions that work on [collections](collections.md). `Map`
transforms each element; `Filter` keeps those that pass a test. They receive a function
as a parameter (see [functions](fonctions.md)).

```go
package main

import (
	"fmt"
	"strings"
)

func Map[T, U any](items []T, f func(T) U) []U {
	result := make([]U, 0, len(items))
	for _, e := range items {
		result = append(result, f(e))
	}
	return result
}

func Filter[T any](items []T, keep func(T) bool) []T {
	var result []T
	for _, e := range items {
		if keep(e) {
			result = append(result, e)
		}
	}
	return result
}

func main() {
	numbers := []int{1, 2, 3, 4, 5, 6}

	squares := Map(numbers, func(n int) int { return n * n })
	fmt.Println(squares)

	evens := Filter(numbers, func(n int) bool { return n%2 == 0 })
	fmt.Println(evens)

	words := Map([]string{"go", "is", "simple"}, strings.ToUpper)
	fmt.Println(words)

	lengths := Map(words, func(s string) int { return len(s) })
	fmt.Println(lengths)
}
```

```sortie
[1 4 9 16 25 36]
[2 4 6]
[GO IS SIMPLE]
[2 2 6]
```

`Map` has **two** type parameters: `T` for the input and `U` for the output. They can be
different, as in the last call (`string` to `int`).

## Generic types

A type can also have a type parameter. Here is a **stack**: a list where you always add
and remove from the top, like a stack of plates. The last element put down is the first
one taken back.

```go
package main

import "fmt"

type Stack[T any] struct {
	items []T
}

func (s *Stack[T]) Push(v T) {
	s.items = append(s.items, v)
}

func (s *Stack[T]) Pop() (T, bool) {
	var zero T
	if len(s.items) == 0 {
		return zero, false
	}
	last := s.items[len(s.items)-1]
	s.items = s.items[:len(s.items)-1]
	return last, true
}

func (s *Stack[T]) Size() int {
	return len(s.items)
}

func main() {
	var plates Stack[string]
	plates.Push("blue")
	plates.Push("red")
	plates.Push("green")
	fmt.Println("Size:", plates.Size())

	for plates.Size() > 0 {
		p, _ := plates.Pop()
		fmt.Println(p)
	}

	_, ok := plates.Pop()
	fmt.Println("One more?", ok)

	var numbers Stack[int]
	numbers.Push(42)
	n, _ := numbers.Pop()
	fmt.Println(n + 1)
}
```

```sortie
Size: 3
green
red
blue
One more? false
43
```

In the methods, you write `Stack[T]` for the receiver. `Pop` also returns a boolean: the
stack may be empty, and the caller must be told.

## When not to use generics

Generics are useful, but they make code more abstract. Avoid them:

- when your function is only used for **one** type: just write that type;
- when an **interface** is enough: if you only call methods, such as `Area()`, take an
  interface (see [interfaces](interfaces.md));
- when the code for each type is **different**: a generic is for writing the same code
  for all of them.

> **Remember:** start with simple code for one type. Switch to generics when you catch
> yourself copying the same function for a second type.

## Common mistakes

### Comparing with the any constraint

```go norun
package main

import "fmt"

func Max[T any](a, b T) T {
	if a > b {
		return a
	}
	return b
}

func main() {
	fmt.Println(Max(3, 7))
}
```

```text
# command-line-arguments
./main.go:6:5: invalid operation: a > b (type parameter T cannot use operator >)
```

`any` accepts every type, including those that cannot be compared. Use the
`cmp.Ordered` constraint and import `cmp`.

### Mixing two types for the same T

```go norun
package main

import (
	"cmp"
	"fmt"
)

func Max[T cmp.Ordered](a, b T) T {
	if a > b {
		return a
	}
	return b
}

func main() {
	x := 3
	y := 2.5
	fmt.Println(Max(x, y))
}
```

```text
# command-line-arguments
./main.go:18:21: in call to Max, type float64 of y does not match inferred type int for T
```

`a` and `b` have the same type `T`. Go chose `int` from `x`, then finds a `float64`.
Convert: `Max(float64(x), y)`.

### Go cannot infer T

```go norun
package main

import "fmt"

func Empty[T any]() []T {
	return []T{}
}

func main() {
	s := Empty()
	fmt.Println(s)
}
```

```text
# command-line-arguments
./main.go:10:12: in call to Empty, cannot infer T (declared at ./main.go:5:12)
```

No argument lets Go infer `T`. Give it between square brackets: `Empty[int]()`.

## Test yourself

**Question 1:** which constraint should you choose for a function that uses `==` on its
elements?

> **Answer**
>
> `comparable`. It accepts every type that supports `==` and `!=`.

**Question 2:** what does the `~` in `~int` mean?

> **Answer**
>
> "`int` or any type whose underlying type is `int`". For example `type Age int`
> satisfies `~int`, but not plain `int`.

**Question 3:** write a generic function `Count[T any](s []T, f func(T) bool) int`
that counts the elements for which `f` returns `true`.

> **Answer**
>
> ```go
> package main
>
> import "fmt"
>
> func Count[T any](s []T, f func(T) bool) int {
> 	n := 0
> 	for _, e := range s {
> 		if f(e) {
> 			n++
> 		}
> 	}
> 	return n
> }
>
> func main() {
> 	grades := []int{12, 8, 15, 9, 18}
> 	fmt.Println(Count(grades, func(n int) bool { return n >= 10 }))
> }
> ```
>
> ```sortie
> 3
> ```

## Summary

- `[T any]` declares a type parameter; Go often infers it from the arguments.
- A constraint limits the types: `comparable` for `==`, `cmp.Ordered` for `<` and `>`.
- A custom constraint is written like an interface: `~int | ~float64`.
- Types can be generic too: `Stack[T]`, with methods on `*Stack[T]`.
- Only use generics to avoid real repetition.

Next: [handling errors](erreurs.md).
