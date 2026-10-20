# Pointers

This page explains what a pointer is, how to use it with `&` and `*`, and when it's best to use one.

> **Goals:**
>
> - you will know what an address and a pointer are;
> - you will know how to use `&` to take an address and `*` to follow it;
> - you will know why a function changes (or doesn't change) a variable;
> - you will know how to avoid the `nil` pointer panic;
> - you will know when to choose a pointer.

## An address instead of a copy

Every variable is stored somewhere in
[memory (RAM)](../debuter/glossaire.md#memoire-vive), at an **address**. A
[pointer](../debuter/glossaire.md#pointeur) is a variable that holds this address.

Think of a house. You can give a friend **a photo** of your house: they can color it in,
and your house doesn't change. Or you can give them **your address**: then they can come
and repaint the real door. The photo is a copy. The address is a pointer.

## & and *

Two symbols are enough:

- `&x` gives the address of `x` ("where does `x` live?");
- `*p` follows the address stored in `p` and gives the value found there ("go and look
  at this address").

The type of a pointer to an `int` is written `*int`.

```go
package main

import "fmt"

func main() {
	x := 10
	p := &x

	fmt.Println("x is", x)
	fmt.Println("p points to", *p)

	*p = 20
	fmt.Println("x is now", x)
	fmt.Printf("p has type %T\n", p)
}
```

```sortie
x is 10
p points to 10
x is now 20
p has type *int
```

Writing `*p = 20` changes the value **at the address** of `x`. So it's `x` that changes.

> **Note:** if you print `p` itself, Go shows an address like `0xc000012345`.
> It changes from one run to the next: never rely on it.

## Passing by value or by pointer

In Go, a function always receives a **copy** of its
[arguments](../debuter/glossaire.md#argument). This is passing **by value**. So the
function can't change the caller's variable.

To let it do so, pass the address: the function receives a copy of the address, but
this address leads to the real variable.

```go
package main

import "fmt"

func doubleCopy(n int) {
	n = n * 2
}

func double(n *int) {
	*n = *n * 2
}

func main() {
	x := 5
	doubleCopy(x)
	fmt.Println("After doubleCopy:", x)

	double(&x)
	fmt.Println("After double:", x)
}
```

```sortie
After doubleCopy: 5
After double: 10
```

### The case of structs

A struct passed by value is copied in full. With a pointer to a struct, Go saves you
from writing `(*p).Field`: `p.Field` is enough.

```go
package main

import "fmt"

type Player struct {
	Name  string
	Lives int
}

func loseALife(p *Player) {
	p.Lives--
}

func main() {
	p := Player{Name: "Léa", Lives: 3}
	loseALife(&p)
	loseALife(&p)
	fmt.Printf("%+v\n", p)
}
```

```sortie
{Name:Léa Lives:1}
```

## Pointer receivers

A method is a function like any other: its receiver follows the same rule. With
`func (a *Account)`, the method changes the original. You saw this in
[structs and methods](types.md).

```go
package main

import "fmt"

type Account struct {
	Balance int
}

func (a *Account) Deposit(amount int) {
	a.Balance += amount
}

func (a Account) Show() {
	fmt.Println("Balance:", a.Balance)
}

func main() {
	a := Account{}
	a.Deposit(50)
	a.Deposit(25)
	a.Show()
}
```

```sortie
Balance: 75
```

`Deposit` changes the account: pointer receiver. `Show` only reads: a value receiver is
enough.

## The nil pointer

A pointer that points to nothing is `nil`. It's the zero value of pointers:
`var p *int` is `nil`. Following a `nil` pointer is impossible: there's no house at
this address.

So test `p != nil` before following a pointer that may be empty.

```go
package main

import "fmt"

type Player struct {
	Name string
}

func nameOf(p *Player) string {
	if p == nil {
		return "(nobody)"
	}
	return p.Name
}

func main() {
	var missing *Player
	present := &Player{Name: "Sam"}

	fmt.Println(missing == nil)
	fmt.Println(nameOf(missing))
	fmt.Println(nameOf(present))
}
```

```sortie
true
(nobody)
Sam
```

## new and &T{}

`new(T)` sets aside a zero value of type `T` and returns its address. In practice, you
more often write `&T{...}`, which lets you fill in the fields right away.

```go
package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	n := new(int)
	*n = 7
	fmt.Println(*n)

	p := new(Point)
	p.X = 3
	fmt.Println(*p)

	q := &Point{X: 1, Y: 2}
	fmt.Println(*q)
}
```

```sortie
7
{3 0}
{1 2}
```

> **Note:** in Go, a function can return the address of a local variable, like
> `return &Point{}`. Go keeps the value alive as long as it's used: there's nothing
> to free by hand.

## When should you use a pointer?

Use a pointer:

- when a function or a method must **change** the value it receives;
- when the struct is **big** and copying it would be expensive;
- when "no value" must be possible: the pointer is then `nil`.

Keep a plain value in other cases, especially for small types (`int`, `string`, a
`Point`). A value can't be `nil`, so it can't cause a panic.

> **Remember:** slices, maps and channels already contain a kind of hidden pointer. A
> function that receives a map can change it without `*` (see [maps](maps.md)).

```go
package main

import "fmt"

func add(stock map[string]int, fruit string) {
	stock[fruit]++
}

func main() {
	stock := map[string]int{}
	add(stock, "apple")
	add(stock, "apple")
	fmt.Println(stock["apple"])
}
```

```sortie
2
```

## Common mistakes

### Following a nil pointer

```go norun
package main

import "fmt"

type Player struct {
	Name string
}

func main() {
	var p *Player
	fmt.Println(p.Name)
}
```

```text
panic: runtime error: invalid memory address or nil pointer dereference
[signal SIGSEGV: segmentation violation code=0x1 addr=0x0 pc=0x491af6]

goroutine 1 [running]:
main.main()
	/path/to/main.go:11 +0x16
exit status 2
```

`p` is `nil`: there's no player at this address. The message "nil pointer dereference"
means "following a nil pointer". Create the value (`p := &Player{Name: "Sam"}`) or test
`p != nil` first.

### Forgetting the &

```go norun
package main

import "fmt"

func double(n *int) {
	*n = *n * 2
}

func main() {
	x := 5
	double(x)
	fmt.Println(x)
}
```

```text
# command-line-arguments
./main.go:11:9: cannot use x (variable of type int) as *int value in argument to double
```

The function expects an address (`*int`), not an `int`. Write `double(&x)`.

### Doing arithmetic on an address

```go norun
package main

import "fmt"

func main() {
	x := 5
	p := &x
	fmt.Println(p + 1)
}
```

```text
# command-line-arguments
./main.go:8:14: invalid operation: p + 1 (mismatched types *int and untyped int)
```

Go forbids arithmetic on addresses. You probably wanted the value pointed to: `*p + 1`.

## Test yourself

**Question 1:** what does this program print?

```go norun
x := 1
p := &x
*p = *p + 10
fmt.Println(x)
```

> **Answer**
>
> `11`. `p` holds the address of `x`, so `*p = ...` changes `x`.

**Question 2:** what is the zero value of a variable `var p *string`?

> **Answer**
>
> `nil`. The pointer points to nothing; writing `*p` would cause a panic.

**Question 3:** write a function `swap(a, b *int)` that swaps the values of two
variables.

> **Answer**
>
> ```go
> package main
>
> import "fmt"
>
> func swap(a, b *int) {
> 	*a, *b = *b, *a
> }
>
> func main() {
> 	x, y := 1, 2
> 	swap(&x, &y)
> 	fmt.Println(x, y)
> }
> ```
>
> ```sortie
> 2 1
> ```

## Summary

- A pointer holds the address of a value; its type is written `*T`.
- `&x` takes the address of `x`; `*p` reads or changes the value pointed to.
- Go passes everything by value; pass a pointer to change the original.
- A `nil` pointer points to nothing: following it makes the program panic.
- `new(T)` and `&T{...}` create a value and return its address.

Next: [generics](generiques.md).
