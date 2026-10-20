# Structs and methods

This page shows how to create your own types in Go with `struct`, give them methods, and combine them through embedding.

> **Goals:**
>
> - you will know how to define a struct and create values of that type;
> - you will know how to read and change the fields of a struct;
> - you will know how to write methods, with a value or pointer receiver;
> - you will know how to write a `NewX` constructor;
> - you will know how to reuse a struct inside another one through embedding.

## Defining a struct

A [struct](../debuter/glossaire.md#structure) groups several values under a single
name. Think of a sign-up form: it has boxes (name, age, city), and each person fills
in their own form. The blank form is the **type**, each filled-in form is a **value**
of that type.

In Go, you write `type`, the name, then `struct` and the list of **fields** between
curly braces. Each field has a name and a [type](../debuter/glossaire.md#type).

```go
package main

import "fmt"

type Person struct {
	Name string
	Age  int
	City string
}

func main() {
	alice := Person{Name: "Alice", Age: 30, City: "Lyon"}
	fmt.Println(alice)
}
```

```sortie
{Alice 30 Lyon}
```

The struct is declared **outside** `main`, at the file level. The type name often
starts with a capital letter: you will see why in
[packages and modules](modules.md).

## Creating values

There are several ways to write a struct value, called a **literal**.

- With the field names: `Person{Name: "Alice", Age: 30}`. This is the recommended
  form: the order doesn't matter and the code reads by itself.
- Without the names: `Person{"Bob", 25, "Paris"}`. You must then give **all** the
  fields, in order.
- Empty: `Person{}`. Each field gets its **zero value**: `""` for text, `0` for a
  number, `false` for a boolean.

```go
package main

import "fmt"

type Person struct {
	Name string
	Age  int
	City string
}

func main() {
	a := Person{Name: "Alice", Age: 30}
	b := Person{"Bob", 25, "Paris"}
	var c Person

	fmt.Println(a)
	fmt.Println(b)
	fmt.Println(c)
	fmt.Printf("%+v\n", a)
	fmt.Printf("%#v\n", b)
}
```

```sortie
{Alice 30 }
{Bob 25 Paris}
{ 0 }
{Name:Alice Age:30 City:}
main.Person{Name:"Bob", Age:25, City:"Paris"}
```

> **Tip:** to display a struct while you hunt for a bug, use `%+v`: it shows the name
> of each field. `%#v` also shows the name of the type.

## Reading and changing fields

A dot `.` gives access to a field: `alice.Age`. You can read it and give it a new
value.

```go
package main

import "fmt"

type Account struct {
	Owner   string
	Balance float64
}

func main() {
	a := Account{Owner: "Alice", Balance: 100}
	a.Balance = a.Balance + 50
	fmt.Println(a.Owner, "has", a.Balance, "euros")
}
```

```sortie
Alice has 150 euros
```

### A copy is independent

When you assign a struct to another variable, Go **copies** all the boxes. Changing
the copy doesn't touch the original: it's like photocopying the form.

```go
package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	a := Point{X: 1, Y: 2}
	b := a
	b.X = 100
	fmt.Println(a, b)
}
```

```sortie
{1 2} {100 2}
```

`X, Y int` declares two fields of the same type on a single line.

## Comparing two structs

Two structs of the same type are compared with `==`: they are equal if **all** their
fields are equal. This works as long as the fields themselves are comparable (numbers,
text, booleans… but not slices or maps).

```go
package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	a := Point{1, 2}
	b := Point{1, 2}
	c := Point{2, 1}
	fmt.Println(a == b)
	fmt.Println(a == c)
}
```

```sortie
true
false
```

## Methods

A [method](../debuter/glossaire.md#methode) is a function attached to a type. You
write it like a function, with an extra **receiver** in parentheses before the name:
`func (r Rectangle) Area()`. The receiver is the value the method is called on. By
habit, it gets a short name: the first letter of the type.

```go
package main

import "fmt"

type Rectangle struct {
	Width, Height float64
}

func (r Rectangle) Area() float64 {
	return r.Width * r.Height
}

func (r Rectangle) Perimeter() float64 {
	return 2 * (r.Width + r.Height)
}

func main() {
	r := Rectangle{Width: 3, Height: 4}
	fmt.Println("Area:", r.Area())
	fmt.Println("Perimeter:", r.Perimeter())
}
```

```sortie
Area: 12
Perimeter: 14
```

Go has no classes: a struct and its methods play the same role. Compare with
[classes in Ruby](../ruby/types.md).

## Value or pointer receiver

With `func (r Rectangle)`, the method receives a **copy** of the value. It can read
the fields, but a change is lost at the end of the method.

To change the original value, write a star: `func (c *Counter)`. The method then
receives the **address** of the value, a [pointer](../debuter/glossaire.md#pointeur),
and works on the original. The chapter [pointers](pointeurs.md) explains this in
detail.

```go
package main

import "fmt"

type Counter struct {
	Value int
}

func (c Counter) IncrementCopy() {
	c.Value++
}

func (c *Counter) Increment() {
	c.Value++
}

func main() {
	c := Counter{}
	c.IncrementCopy()
	fmt.Println("After the copy:", c.Value)
	c.Increment()
	c.Increment()
	fmt.Println("After the pointer:", c.Value)
}
```

```sortie
After the copy: 0
After the pointer: 2
```

You don't need to write `(&c).Increment()`: Go takes the address by itself when you
write `c.Increment()`.

> **Remember:** a method that **changes** the struct takes a pointer receiver `*T`. If
> a single method of the type has a pointer receiver, give one to all of them: it's
> more consistent.

## NewX constructors

Go has no keyword to build an object. The habit is to write a function
`NewTypeName` that prepares a valid value and returns it. It is called a
**constructor**. It often returns a pointer.

```go
package main

import (
	"fmt"
	"strings"
)

type Player struct {
	Name   string
	Lives  int
	Points int
}

func NewPlayer(name string) *Player {
	return &Player{
		Name:  strings.TrimSpace(name),
		Lives: 3,
	}
}

func main() {
	p := NewPlayer("  Zoé ")
	p.Points += 10
	fmt.Printf("%+v\n", *p)
}
```

```sortie
{Name:Zoé Lives:3 Points:10}
```

The constructor guarantees that a player always starts with 3 lives and a clean name.
Notice the comma after the last field: when a literal spreads over several lines, Go
requires it.

## Embedding

Go has no [inheritance](../debuter/glossaire.md#heritage). Instead, a struct can
**embed** another one: you write the type alone, without a field name. The fields and
methods of the embedded type become directly accessible.

It's like a car that contains an engine: you say "the car starts", even though the
engine does the work.

```go
package main

import "fmt"

type Animal struct {
	Name string
}

func (a Animal) Introduce() string {
	return "My name is " + a.Name
}

type Dog struct {
	Animal
	Breed string
}

func main() {
	rex := Dog{Animal: Animal{Name: "Rex"}, Breed: "labrador"}
	fmt.Println(rex.Name)
	fmt.Println(rex.Introduce())
	fmt.Println(rex.Animal.Name)
}
```

```sortie
Rex
My name is Rex
Rex
```

`rex.Name` is a shortcut for `rex.Animal.Name`. The embedded field is named after its
type: `Animal`.

## Field tags

After the type of a field, you can add a **tag** between backquotes. Go ignores it,
but some packages read it. The most common one is `encoding/json`, which uses it to
choose the name of the field in the [JSON](../debuter/glossaire.md#json) (see
[formats](formats.md)).

```go
package main

import (
	"encoding/json"
	"fmt"
)

type Book struct {
	Title  string `json:"title"`
	Pages  int    `json:"pages"`
	Secret string `json:"-"`
}

func main() {
	b := Book{Title: "Dune", Pages: 600, Secret: "hidden"}
	data, _ := json.Marshal(b)
	fmt.Println(string(data))
}
```

```sortie
{"title":"Dune","pages":600}
```

`json:"-"` means: don't write this field.

## Common mistakes

### Mixing named and unnamed fields

```go norun
package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	p := Point{X: 1, 2}
	fmt.Println(p)
}
```

```text
# command-line-arguments
./main.go:10:19: mixture of field:value and value elements in struct literal
```

A literal uses names either for all the fields or for none. Write
`Point{X: 1, Y: 2}`.

### Leaving out fields without the names

```go norun
package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	p := Point{1}
	fmt.Println(p)
}
```

```text
# command-line-arguments
./main.go:10:14: too few values in struct literal of type Point
```

Without the names, you must give **all** the fields. Write `Point{X: 1}`: the missing
field is then `0`.

### Getting the capital letter wrong

```go norun
package main

import "fmt"

type Person struct {
	name string
}

func main() {
	p := Person{name: "Alice"}
	fmt.Println(p.Name)
}
```

```text
# command-line-arguments
./main.go:11:16: p.Name undefined (type Person has no field or method Name, but does have field name)
```

Go tells uppercase and lowercase apart: `Name` and `name` are two different names. The
message even gives you the right spelling: `p.name`.

### Changing a copy in a method

```go norun
func (c Counter) Increment() {
	c.Value++
}
```

```text
0
```

No error message here, but the counter stays at `0`: the method changed a copy. Write
`func (c *Counter) Increment()`.

## Test yourself

**Question 1:** what does `fmt.Printf("%+v\n", Point{X: 3})` print if `Point` has two
fields `X` and `Y` of type `int`?

> **Answer**
>
> `{X:3 Y:0}`. The field `Y` isn't given: it takes the zero value of integers.

**Question 2:** when should you write `func (a *Account)` rather than
`func (a Account)`?

> **Answer**
>
> When the method must change the original account. With `(a Account)`, the method
> works on a copy and the changes are lost.

**Question 3:** write a struct `Circle` with a field `Radius` and a method
`Diameter()` that returns twice the radius.

> **Answer**
>
> ```go
> package main
>
> import "fmt"
>
> type Circle struct {
> 	Radius float64
> }
>
> func (c Circle) Diameter() float64 {
> 	return 2 * c.Radius
> }
>
> func main() {
> 	c := Circle{Radius: 2.5}
> 	fmt.Println(c.Diameter())
> }
> ```
>
> ```sortie
> 5
> ```

## Summary

- `type Name struct { ... }` defines a struct; each of its fields has a type.
- A literal `T{Field: value}` creates a value; missing fields are zero.
- `%+v` displays a struct with the field names; `==` compares all the fields.
- A method has a receiver; use `*T` to change the original value.
- A `NewT` function acts as a constructor; embedding replaces inheritance.

Next: [interfaces](interfaces.md).
