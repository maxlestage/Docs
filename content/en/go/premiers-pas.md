# First steps

This page shows how to store values in variables, what Go's basic types are, and how to print neatly with `fmt`.

> **Goals:**
>
> - you will know how to create a variable with `var` or `:=`;
> - you will know the basic types and their zero value;
> - you will know how to create constants, and series of constants with `iota`;
> - you will know how to print values with `fmt.Println` and `fmt.Printf`;
> - you will know why some names start with a capital letter.

## Variables

A [variable](../debuter/glossaire.md#variable) is a box with a label. You store a value
in it, and you find it again by its name.

In Go, the most complete way to create a variable uses the word `var`:

```go
package main

import "fmt"

func main() {
	var firstName string = "Alice"
	var age int = 12
	fmt.Println(firstName, "is", age, "years old")
}
```

```sortie
Alice is 12 years old
```

Read `var age int = 12` like this: "create a variable `age`, of type `int` (an
integer), whose value is 12".

The [type](../debuter/glossaire.md#type) says what kind of value the box holds: a
number, some text, a true/false… In Go, a variable **always** keeps the same type.

### Changing the value

The `=` sign stores a new value in a variable that already exists.

```go
package main

import "fmt"

func main() {
	var score int = 10
	fmt.Println(score)
	score = 25
	fmt.Println(score)
	score = score + 5
	fmt.Println(score)
}
```

```sortie
10
25
30
```

`score = score + 5` reads from right to left: Go computes `score + 5` (25 + 5), then
stores the result in `score`.

## The short declaration :=

Writing the type every time is long. If you give a value, Go **guesses** the type by
itself. Inside a function, you then use `:=`:

```go
package main

import "fmt"

func main() {
	firstName := "Bob"
	age := 9
	height := 1.32
	fmt.Println(firstName, age, height)
}
```

```sortie
Bob 9 1.32
```

- `firstName := "Bob"` creates a variable of type `string`;
- `age := 9` creates an `int`;
- `height := 1.32` creates a `float64` (a decimal number).

> **Remember:** `:=` **creates** a variable. `=` **changes** a variable that already
> exists. This is the most important difference in this chapter.

`:=` only works inside a function. Outside, at the file level, use `var`.

### Several variables at once

```go
package main

import "fmt"

func main() {
	x, y := 3, 4
	fmt.Println(x, y)
	x, y = y, x
	fmt.Println(x, y)
}
```

```sortie
3 4
4 3
```

The line `x, y = y, x` swaps the two values, without a temporary variable.

## Zero values

If you create a variable with `var` without giving it a value, it is not empty: it
gets the **zero value** of its type.

```go
package main

import "fmt"

func main() {
	var n int
	var price float64
	var active bool
	var name string
	fmt.Println(n, price, active)
	fmt.Printf("[%s]\n", name)
}
```

```sortie
0 0 false
[]
```

| Type | Zero value |
|------|-------------|
| `int`, `float64` | `0` |
| `bool` | `false` |
| `string` | `""` (empty text) |

> **Note:** in Go, a variable always has a value. There is no "uninitialized"
> variable that could hold anything.

## Basic types

| Type | What it holds | Example |
|------|-------------------|---------|
| `int` | an [integer](../debuter/glossaire.md#integer) | `42`, `-7` |
| `float64` | a [decimal number](../debuter/glossaire.md#float) | `3.14` |
| `string` | text | `"hello"` |
| `bool` | a [boolean](../debuter/glossaire.md#boolean): true or false | `true`, `false` |
| `rune` | a single character | `'é'` |
| `byte` | a [byte](../debuter/glossaire.md#byte) (from 0 to 255) | `65` |

```go
package main

import "fmt"

func main() {
	grade := 15
	average := 13.5
	city := "Lyon"
	passed := grade >= 10
	initial := 'L'
	fmt.Println(grade, average, city, passed, initial)
}
```

```sortie
15 13.5 Lyon true 76
```

Surprise: `initial` prints as `76`. A `rune` is actually a number, the
[Unicode](../debuter/glossaire.md#unicode) code of the character. To see the letter,
you need `Printf` and `%c`, as we will see.

## Constants

A [constant](../debuter/glossaire.md#constant) is a value that never changes. You
create it with `const`.

```go
package main

import "fmt"

const VatRate = 0.2

func main() {
	const pi = 3.14159
	price := 50.0
	fmt.Println(price * (1 + VatRate))
	fmt.Println(pi * 2)
}
```

```sortie
60
6.28318
```

A constant can be declared outside `main`, at the file level: it is then visible
everywhere. Trying to change it (`pi = 3`) is a compile error.

### Series of constants with iota

To number constants, Go offers `iota`. In a `const ( ... )` block, `iota` is 0 on the
first line, 1 on the next, and so on.

```go
package main

import "fmt"

const (
	Monday = iota
	Tuesday
	Wednesday
	Thursday
)

func main() {
	fmt.Println(Monday, Tuesday, Wednesday, Thursday)
}
```

```sortie
0 1 2 3
```

Lines without a value repeat the formula of the line above, with `iota` going up. So
you can do a calculation:

```go
package main

import "fmt"

const (
	Bronze = (iota + 1) * 100
	Silver
	Gold
)

func main() {
	fmt.Println(Bronze, Silver, Gold)
}
```

```sortie
100 200 300
```

## Printing with fmt

### Println

`fmt.Println` prints its values separated by spaces, then moves to the next line.
`fmt.Print` does the same without moving to the next line.

### Printf and verbs

`fmt.Printf` (*print formatted*) takes a text **template**. Each `%` followed by a
letter is a hole, called a **verb**, replaced by a value. Add `\n` at the end to move
to the next line.

```go
package main

import "fmt"

func main() {
	name := "Alice"
	age := 12
	fmt.Printf("%s is %d years old.\n", name, age)
}
```

```sortie
Alice is 12 years old.
```

The most useful verbs:

| Verb | Prints | Example result |
|-------|---------|---------------------|
| `%d` | an integer | `42` |
| `%s` | a string | `Alice` |
| `%q` | a string in quotes | `"Alice"` |
| `%v` | any value, in the default format | `[1 2 3]` |
| `%+v` | like `%v`, with the field names of a struct | `{Name:Alice Age:12}` |
| `%T` | the type of the value | `float64` |
| `%.2f` | a decimal number, with 2 decimals | `3.14` |
| `%c` | a character (a rune) | `L` |
| `%%` | the `%` sign itself | `%` |

```go
package main

import "fmt"

type Student struct {
	Name string
	Age  int
}

func main() {
	pi := 3.14159
	fmt.Printf("%.2f\n", pi)
	fmt.Printf("%T and %T\n", pi, 42)
	fmt.Printf("%q\n", "hi")
	fmt.Printf("%c\n", 'L')
	fmt.Printf("%v\n", []int{1, 2, 3})
	s := Student{Name: "Alice", Age: 12}
	fmt.Printf("%v\n", s)
	fmt.Printf("%+v\n", s)
	fmt.Printf("Discount: 20%%\n")
}
```

```sortie
3.14
float64 and int
"hi"
L
[1 2 3]
{Alice 12}
{Name:Alice Age:12}
Discount: 20%
```

> **Tip:** lost? Use `%v`: it works with everything. And `%T` tells you the exact
> type of a value, very handy for understanding what is going on.

`Student` is a **struct**: a value that groups several fields together. Structs are
explained in [types](types.md).

## Comments

A [comment](../debuter/glossaire.md#comment) is a note for humans: Go ignores
it.

```go
package main

import "fmt"

// main is the starting point of the program.
func main() {
	fmt.Println("visible") // this is a comment too
	/* a comment
	   on several lines */
}
```

```sortie
visible
```

- `//` comments until the end of the line;
- `/* ... */` comments everything in between.

> **Tip:** by convention, a comment just above a function starts with its name:
> `// main is...`. Go's documentation tools use it.

## Choosing good names

A variable name starts with a letter, followed by letters, digits or `_`. In Go, you
join words with capital letters: `totalPrice`, `numberOfStudents`. This is
*camelCase* (because of the humps).

```go
package main

import "fmt"

func main() {
	totalPrice := 42
	numStudents := 25
	fmt.Println(totalPrice, numStudents)
}
```

```sortie
42 25
```

### Capital letter = visible from another package

In Go, the **first letter** of a name has a meaning:

- a name that starts with a **capital** letter is **exported**: other packages can
  use it;
- a name that starts with a **lowercase** letter stays **private** to its package.

That's why we write `fmt.Println` with a capital `P`: `Println` is exported by the
`fmt` package. A function `fmt.println` (lowercase), even if it existed, would be
hidden.

> **Remember:** there is no `public` or `private` keyword in Go. Only the capital
> letter counts.

## An unused variable is an error

The Go compiler refuses a variable that is created in a function but never used.
This is on purpose: a forgotten variable often hides a bug.

If you really want to ignore a value, use the **underscore** `_`, Go's "trash can":

```go
package main

import "fmt"

func main() {
	a, _ := 10, 20
	fmt.Println(a)
}
```

```sortie
10
```

The value `20` is thrown away: `_` is not a real variable.

## Common mistakes

### Creating a variable without using it

```go norun
package main

import "fmt"

func main() {
	age := 12
	name := "Alice"
	fmt.Println(name)
}
```

```text
# command-line-arguments
./main.go:6:2: declared and not used: age
```

`age` is declared but never used. Use it, or remove the line.

### Using := twice for the same variable

```go norun
package main

import "fmt"

func main() {
	score := 10
	score := 20
	fmt.Println(score)
}
```

```text
# command-line-arguments
./main.go:7:8: no new variables on left side of :=
```

`score` already exists: `:=` has nothing new to create. To change the value, write
`score = 20`, with `=`.

### Using := outside a function

```go norun
package main

import "fmt"

city := "Paris"

func main() {
	fmt.Println(city)
}
```

```text
# command-line-arguments
./main.go:5:1: syntax error: non-declaration statement outside function body
```

Outside a function, `:=` is not allowed. Write `var city = "Paris"`.

### Storing a value of the wrong type

```go norun
package main

import "fmt"

func main() {
	var age int
	age = "twelve"
	fmt.Println(age)
}
```

```text
# command-line-arguments
./main.go:7:8: cannot use "twelve" (untyped string constant) as int value in assignment
```

`age` is an `int` forever: you cannot store text in it. Write `age = 12`.

## Test yourself

**Question 1:** what does this program print?

```go norun
package main

import "fmt"

func main() {
	var total int
	var message string
	fmt.Printf("%d [%s]\n", total, message)
}
```

> **Answer**
>
> `0 []`. The variables get their zero value: `0` for an `int`, empty text for a
> `string`.

**Question 2:** with `Printf`, print `Price: 12.50 €` from the variable
`price := 12.5`.

> **Answer**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	price := 12.5
> 	fmt.Printf("Price: %.2f €\n", price)
> }
> ```
>
> ```sortie
> Price: 12.50 €
> ```

**Question 3:** what is the difference between `x := 5` and `x = 5`?

> **Answer**
>
> `x := 5` **creates** a new variable `x`. `x = 5` changes the value of a variable
> `x` that already exists.

## Summary

- `var name type = value` creates a variable; `name := value` does the same inside a
  function, guessing the type.
- A variable without a value gets the zero value of its type (`0`, `""`, `false`).
- `const` creates a fixed value; `iota` numbers a series of constants.
- `fmt.Printf` replaces the verbs `%d`, `%s`, `%v`, `%T`, `%.2f`… with values.
- A capitalized name is exported; an unused variable stops the program from
  compiling.

Next: [numbers and calculations](nombres.md).
