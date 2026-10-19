# Error handling

This page explains how Go treats errors as ordinary values, with `error`, `if err != nil`, `fmt.Errorf` and `errors.Is`.

> **Goals:**
>
> - you will know how to read and check an error with `if err != nil`;
> - you will know how to create an error with `errors.New` and `fmt.Errorf`;
> - you will know how to wrap an error with `%w` and find it again with `errors.Is` and `errors.As`;
> - you will know how to write your own error types;
> - you will know when to use `panic` and `recover` (rarely).

## An error is a value

Many languages raise an [exception](../debuter/glossaire.md#exception) that
interrupts the program (see [errors in Ruby](../ruby/erreurs.md)). Go does things
differently: a function that can fail **returns** the [error](../debuter/glossaire.md#error)
as one more result, in last position.

Think of a delivery driver. They don't shout in the street when the parcel is damaged:
they hand you the parcel **and** a note saying "parcel damaged". It's up to you to read
the note.

The type of errors is called `error`. A missing error is `nil`.

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("42")
	fmt.Println(n, err)

	n, err = strconv.Atoi("twelve")
	fmt.Println(n, err)
}
```

```sortie
42 <nil>
0 strconv.Atoi: parsing "twelve": invalid syntax
```

`strconv.Atoi` turns a text into an integer. When the text is not a number, it
returns `0` and an error that explains the problem.

## if err != nil

After each call that can fail, you check the error right away. This is the most
famous line in Go:

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	inputs := []string{"12", "abc", "7"}
	for _, s := range inputs {
		n, err := strconv.Atoi(s)
		if err != nil {
			fmt.Println("Skipped:", err)
			continue
		}
		fmt.Println("The double is", n*2)
	}
}
```

```sortie
The double is 24
Skipped: strconv.Atoi: parsing "abc": invalid syntax
The double is 14
```

You handle the error case **first**, then the normal code continues without being
indented. This makes the happy path easy to follow.

## Creating an error

### errors.New

Your own function can return an error. `errors.New` creates one with a fixed message.
By convention, the message starts with a lowercase letter and doesn't end with a
period.

```go
package main

import (
	"errors"
	"fmt"
)

func divide(a, b float64) (float64, error) {
	if b == 0 {
		return 0, errors.New("division by zero")
	}
	return a / b, nil
}

func main() {
	r, err := divide(10, 4)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	fmt.Println("Result:", r)

	_, err = divide(1, 0)
	if err != nil {
		fmt.Println("Error:", err)
	}
}
```

```sortie
Result: 2.5
Error: division by zero
```

When everything goes well, the function returns `nil` as the error.

### fmt.Errorf

`fmt.Errorf` builds an error the way `fmt.Sprintf` builds a text: you can slip values
into it.

```go
package main

import "fmt"

func checkAge(age int) error {
	if age < 0 {
		return fmt.Errorf("invalid age: %d", age)
	}
	return nil
}

func main() {
	fmt.Println(checkAge(12))
	fmt.Println(checkAge(-3))
}
```

```sortie
<nil>
invalid age: -3
```

## Wrapping an error with %w

An error often travels up through several functions. At each level, you add
**context**: what you were trying to do. With the verb `%w` (*wrap*), `fmt.Errorf`
puts the original error inside a new one, like a parcel inside a bigger box.

```go
package main

import (
	"fmt"
	"strconv"
)

func readPrice(text string) (int, error) {
	price, err := strconv.Atoi(text)
	if err != nil {
		return 0, fmt.Errorf("reading price %q: %w", text, err)
	}
	return price, nil
}

func main() {
	_, err := readPrice("ten")
	fmt.Println(err)
}
```

```sortie
reading price "ten": strconv.Atoi: parsing "ten": invalid syntax
```

The message tells the whole story, from the most general to the most precise.

## Sentinel errors and errors.Is

A **sentinel error** is an error declared once, in a variable, so that you can
recognize it. Its name starts with `Err`. The standard library has several, like
`io.EOF` ("end of file").

`errors.Is(err, ErrX)` answers: "is `ErrX` inside `err`, even at the bottom of the
box?". It opens every wrapping made with `%w`.

```go
package main

import (
	"errors"
	"fmt"
)

var ErrNotFound = errors.New("not found")

var stock = map[string]int{"apple": 3}

func quantity(fruit string) (int, error) {
	n, ok := stock[fruit]
	if !ok {
		return 0, fmt.Errorf("fruit %q: %w", fruit, ErrNotFound)
	}
	return n, nil
}

func main() {
	for _, f := range []string{"apple", "kiwi"} {
		n, err := quantity(f)
		if errors.Is(err, ErrNotFound) {
			fmt.Println(err, "-> ordering some", f+"s")
			continue
		}
		fmt.Println(f, ":", n)
	}
}
```

```sortie
apple : 3
fruit "kiwi": not found -> ordering some kiwis
```

> **Warning:** don't compare messages (`err.Error() == "not found"`). A message can
> change; the sentinel variable can't.

## Your own error types

`error` is a simple [interface](interfaces.md):

```go norun
type error interface {
	Error() string
}
```

So any type that has an `Error() string` method is an error. An error type can carry
information on top of the message, like the faulty field.

`errors.As(err, &target)` looks inside `err` for an error of the type of `target`. If
it finds one, it stores it in `target` and returns `true`.

```go
package main

import (
	"errors"
	"fmt"
)

type ValidationError struct {
	Field  string
	Reason string
}

func (e *ValidationError) Error() string {
	return e.Field + ": " + e.Reason
}

func register(name string, age int) error {
	if name == "" {
		return &ValidationError{Field: "name", Reason: "required"}
	}
	if age < 13 {
		return &ValidationError{Field: "age", Reason: "13 years minimum"}
	}
	return nil
}

func main() {
	err := register("Lou", 9)
	err = fmt.Errorf("registration refused: %w", err)
	fmt.Println(err)

	var ve *ValidationError
	if errors.As(err, &ve) {
		fmt.Println("Field to fix:", ve.Field)
	}
}
```

```sortie
registration refused: age: 13 years minimum
Field to fix: age
```

> **Remember:** `errors.Is` looks for **a precise value** (a sentinel).
> `errors.As` looks for **a type** of error and gives you access to its fields.

## panic and recover

`panic` stops the program immediately, with a message and the call trace. This is
a [panic](../debuter/glossaire.md#panic). Go panics on its own in some cases:
integer division by zero, an index outside a slice, a `nil` pointer…

`recover`, called in a `defer` function, catches a panic. `defer` schedules a call for
the end of the function, whatever happens.

```go
package main

import "fmt"

func safeDivide(a, b int) (result int, err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("recovered: %v", r)
		}
	}()
	return a / b, nil
}

func main() {
	fmt.Println(safeDivide(10, 2))
	fmt.Println(safeDivide(1, 0))
	fmt.Println("The program goes on.")
}
```

```sortie
5 <nil>
0 recovered: runtime error: integer divide by zero
The program goes on.
```

### When should you panic?

Almost never. A panic signals a **bug**: a situation that should never happen if the
code were right. For everything that can normally fail (a missing file, a wrong
input, a network outage), return an `error`.

- Invalid input, missing file: `return ..., err`.
- Programmer bug, impossible state: `panic`.
- `recover`: only so that a server, for example, survives a bug in one request. Not
  to replace `if err != nil`.

## Common mistakes

### Forgetting the error value

```go norun
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n := strconv.Atoi("12")
	fmt.Println(n)
}
```

```text
# command-line-arguments
./main.go:9:7: assignment mismatch: 1 variable but strconv.Atoi returns 2 values
```

`strconv.Atoi` returns two values: the number and the error. Write
`n, err := strconv.Atoi("12")`, then check `err`.

### Getting the error without checking it

```go norun
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("12")
	fmt.Println(n)
}
```

```text
# command-line-arguments
./main.go:9:5: declared and not used: err
```

Go refuses an unused variable. Add the test `if err != nil { ... }`.

### Ignoring the error with _

```go norun
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, _ := strconv.Atoi("twelve")
	fmt.Println(100 / n)
}
```

```text
panic: runtime error: integer divide by zero

goroutine 1 [running]:
main.main()
	/path/to/main.go:10 +0x78
exit status 2
```

The `_` throws the error away: `n` is `0` without warning, and the division panics two
lines later. The real problem is far from the message. Keep the error and test it.

### Wrapping with %v instead of %w

```go norun
err := fmt.Errorf("fruit %q: %v", fruit, ErrNotFound)
fmt.Println(errors.Is(err, ErrNotFound))
```

```text
false
```

With `%v`, only the **text** of the error is copied: the original error is lost and
`errors.Is` doesn't find it. Use `%w`.

## Test yourself

**Question 1:** by convention, what does a function `func f() (int, error)` return
when everything goes well?

> **Answer**
>
> The result and `nil`: `return result, nil`. A `nil` error means "no error".

**Question 2:** what is the difference between `errors.Is` and `errors.As`?

> **Answer**
>
> `errors.Is` checks whether a precise error (a sentinel like `io.EOF`) is in the
> chain. `errors.As` looks for an error of a certain **type** and stores it in a
> variable so you can read its fields.

**Question 3:** write a function `withdraw(balance, amount int) (int, error)` that
returns the sentinel error `ErrInsufficientFunds` if the amount is more than the
balance.

> **Answer**
>
> ```go
> package main
>
> import (
> 	"errors"
> 	"fmt"
> )
>
> var ErrInsufficientFunds = errors.New("insufficient funds")
>
> func withdraw(balance, amount int) (int, error) {
> 	if amount > balance {
> 		return balance, ErrInsufficientFunds
> 	}
> 	return balance - amount, nil
> }
>
> func main() {
> 	fmt.Println(withdraw(100, 30))
> 	_, err := withdraw(10, 50)
> 	fmt.Println(errors.Is(err, ErrInsufficientFunds))
> }
> ```
>
> ```sortie
> 70 <nil>
> true
> ```

## Summary

- An error is a value of type `error`, returned last; `nil` means "no error".
- Check each error right away with `if err != nil`.
- `errors.New` and `fmt.Errorf` create errors; `%w` wraps another one.
- `errors.Is` recognizes a sentinel; `errors.As` finds an error type.
- `panic` is reserved for bugs; `recover` in a `defer` catches it.

Next: [packages and modules](modules.md). Compare with
[errors in Rust](../rust/erreurs.md).
