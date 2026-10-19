# Numbers and calculations

Go offers whole numbers and decimal numbers: this chapter shows how to calculate with them, convert them, read them from text and display them neatly.

> **Goals:**
>
> - you will know how to choose between `int` and `float64`;
> - you will know how to calculate with integer division and the remainder `%`;
> - you will know how to convert a number from one type to another, and from text with `strconv`;
> - you will know how to use the `math` package and draw random numbers;
> - you will know how to display a number with the right number of decimal places.

## Integers

Integers are for counting: students, years, points in a game. Whenever a quantity
can't be cut into pieces, it's an integer.

An [integer](../debuter/glossaire.md#integer) is a number without a decimal point: `3`,
`-12`, `2026`. The most common type is `int`.

```go
package main

import "fmt"

func main() {
	year := 2026
	students := 28
	fmt.Println(year-2000, students*2)
}
```

```sortie
26 56
```

Line by line:

- `year := 2026` creates a variable `year` and stores 2026 in it;
- `students := 28` does the same with 28;
- `fmt.Println` first calculates `year-2000` (26) and `students*2` (56), then displays
  both results, separated by a space.

### Integer sizes

> **Note:** this part can wait. Read it once the basics feel solid.

Go has several integer types. Their name tells you two things:

- the start: `int` for a **signed** integer (it can be negative), `uint` for an
  **unsigned** integer (never negative);
- the number: the size in [bits](../debuter/glossaire.md#bit). The more bits, the
  bigger the number can be.

| Type | Smallest value | Largest value |
|------|----------------|---------------|
| `int8` | -128 | 127 |
| `uint8` (or `byte`) | 0 | 255 |
| `int16` | -32,768 | 32,767 |
| `int32` (or `rune`) | -2,147,483,648 | 2,147,483,647 |
| `int64` | about -9.2 × 10¹⁸ | about 9.2 × 10¹⁸ |
| `uint64` | 0 | about 1.8 × 10¹⁹ |
| `int` | like `int64` on a modern computer | |

> **Remember:** use `int` by default. The other sizes are for specific cases: saving
> memory, reading a binary file…

The underscore `_` can separate digits to make them easier to read:

```go
package main

import "fmt"

func main() {
	population := 8_100_000_000
	fmt.Println(population)
}
```

```sortie
8100000000
```

## Decimal numbers

Not everything is counted in whole numbers: a price (19.99 €), a height (1.75 m), a
temperature (36.6 °C). For these values, you need decimal numbers.

A number with a decimal point is called a [float](../debuter/glossaire.md#float).
In Go, you use `float64`. The decimal separator is written with a **dot**.

```go
package main

import "fmt"

func main() {
	price := 19.99
	quantity := 3.0
	fmt.Println(price * quantity)

	a := 0.1
	b := 0.2
	fmt.Println(a + b)
}
```

```sortie
59.97
0.30000000000000004
```

Line by line:

- `price := 19.99` stores a decimal number in `price`: Go gives it the type `float64`;
- `quantity := 3.0` is also a `float64`, thanks to the `.0`;
- the first `Println` displays the product of the two: 59.97;
- `a` and `b` are 0.1 and 0.2, and their sum gives a little surprise, explained just
  below.

The result of `a + b` is not quite `0.3`. The computer stores decimal numbers in
binary, and some numbers don't fit exactly, like 1/3 in decimal (0.3333…). This isn't
a bug in Go: every language does the same.

> **Note:** `fmt.Println(0.1 + 0.2)` does display `0.3`. Numbers written directly in
> the code are **constants**, which the compiler calculates exactly. The problem only
> shows up with variables.

> **Tip:** for money, count in **cents** with an `int`: `1999` cents instead of
> `19.99` euros. The calculations stay exact.

## Operators

A program calculates all the time: a total to pay, a score, the time left. Operators
are the keys of its calculator.

| Operator | Role | Example | Result |
|----------|------|---------|--------|
| `+` | addition | `7 + 2` | `9` |
| `-` | subtraction | `7 - 2` | `5` |
| `*` | multiplication | `7 * 2` | `14` |
| `/` | division | `7 / 2` | `3` |
| `%` | remainder of the division | `7 % 2` | `1` |

The precedence rules are the same as in maths: `*` and `/` before `+` and `-`.
Parentheses come before everything.

### Integer division and the remainder

Between two integers, `/` gives an **integer division**: the part after the decimal
point is cut off. `%` (pronounced "modulo") gives the remainder.

```go
package main

import "fmt"

func main() {
	fmt.Println(7 / 2)
	fmt.Println(7 % 2)
	fmt.Println(7.0 / 2)

	minutes := 135
	fmt.Println(minutes/60, "h", minutes%60, "min")
}
```

```sortie
3
1
3.5
2 h 15 min
```

Line by line:

- `7 / 2`: two integers, so an integer division. The result is 3;
- `7 % 2`: the remainder of that division, 1;
- `7.0 / 2`: `7.0` is a decimal number, so the division is a normal one: 3.5;
- `minutes/60` gives the whole hours (2), and `minutes%60` the minutes left over (15).

Imagine 7 sweets shared between 2 children: each one gets 3 (`7 / 2`), and 1 is left
over (`7 % 2`).

> **Tip:** `n % 2 == 0` tests whether `n` is even. The remainder is very useful!

**Your turn:** write a program that stores `200` seconds in a variable, then displays
this time in minutes and seconds: "3 min 20 s".

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	seconds := 200
> 	fmt.Println(seconds/60, "min", seconds%60, "s")
> }
> ```
>
> ```sortie
> 3 min 20 s
> ```

### Shortcuts

```go
package main

import "fmt"

func main() {
	points := 10
	points += 5
	points *= 2
	points--
	fmt.Println(points)
}
```

```sortie
29
```

- `points += 5` is a shortcut for `points = points + 5` (likewise `-=`, `*=`, `/=`);
- `points++` adds 1, `points--` subtracts 1.

> **Note:** in Go, `points++` is a statement on its own line. You can't write
> `x := points++`.

## Overflow

> **Note:** this part can wait. Read it once the basics feel solid.

An integer has a limited size. If a calculation goes past the largest value, the
number "wraps around", like a car's odometer going back to zero.

```go
package main

import "fmt"

func main() {
	var n int8 = 127
	n++
	fmt.Println(n)

	var b uint8 = 255
	b++
	fmt.Println(b)
}
```

```sortie
-128
0
```

Go doesn't warn you: the result is just wrong. With an `int` (64 bits), the limit is
so large that the problem is rare, but it exists.

> **Warning:** if the value is written directly in the code, the compiler checks it:
> `var n int8 = 200` is rejected. Wrapping around only happens while the program runs.

## Conversions are explicit

A program often mixes whole quantities (3 items) and decimal prices (2.50 €). Go wants
you to say clearly how to combine them: that way, no calculation goes wrong silently.

Go never mixes two number types on its own. To multiply an `int` by a `float64`, you
must **convert** one of them, by writing the type like a function: `float64(x)`.

```go
package main

import "fmt"

func main() {
	quantity := 3
	price := 2.5
	total := float64(quantity) * price
	fmt.Println(total)

	grade := 15.8
	fmt.Println(int(grade))
}
```

```sortie
7.5
15
```

- `float64(quantity)` turns `3` into `3.0`.
- `int(grade)` **cuts off** the decimal part: `15.8` becomes `15`, not `16`.

### Calculating an average

This is the classic trap: dividing two integers gives an integer.

```go
package main

import "fmt"

func main() {
	sum := 47
	count := 4
	fmt.Println(sum / count)
	fmt.Println(float64(sum) / float64(count))
}
```

```sortie
11
11.75
```

**Your turn:** write a program that calculates the exact average of four whole-number
grades: 12, 15, 16 and 18.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	sum := 12 + 15 + 16 + 18
> 	fmt.Println(float64(sum) / 4)
> }
> ```
>
> ```sortie
> 15.25
> ```

## From text to a number: strconv

A number typed by the user or read from a file arrives as **text**. The `strconv`
package (*string conversion*) does the conversions.

- `strconv.Atoi`: text → `int` (*ASCII to integer*);
- `strconv.Itoa`: `int` → text;
- `strconv.ParseFloat`: text → `float64`.

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("42")
	fmt.Println(n+1, err)

	text := strconv.Itoa(2026)
	fmt.Println("Year " + text)

	f, err := strconv.ParseFloat("3.75", 64)
	fmt.Println(f*2, err)
}
```

```sortie
43 <nil>
Year 2026
7.5 <nil>
```

Line by line:

- `strconv.Atoi("42")` turns the text `"42"` into a number: `n` gets 42, and `err` gets
  `nil`;
- so `n+1` is a real calculation, which gives 43;
- `strconv.Itoa(2026)` goes the other way: `text` holds the text `"2026"`, which `+`
  sticks onto `"Year "`;
- `ParseFloat` reads `"3.75"` as the number 3.75, and `f*2` gives 7.5.

`Atoi` and `ParseFloat` return **two** values: the number, and an error. When all goes
well, the error is `nil` ("nothing"). The `64` in `ParseFloat` asks for a `float64`.

### When the text isn't a number

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("twelve")
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	fmt.Println(n)
}
```

```sortie
Error: strconv.Atoi: parsing "twelve": invalid syntax
```

> **Remember:** always test `err != nil` after a conversion. Errors are covered in
> detail in [error handling](erreurs.md).

**Your turn:** write a program that converts the text `"17"` into a number, then
displays next year's age: "Next year: 18 years old". Don't forget to test `err`.

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"strconv"
> )
>
> func main() {
> 	age, err := strconv.Atoi("17")
> 	if err != nil {
> 		fmt.Println("Error:", err)
> 		return
> 	}
> 	fmt.Println("Next year:", age+1, "years old")
> }
> ```
>
> ```sortie
> Next year: 18 years old
> ```

## The math package

Some calculations take a long time to write yourself: a square root, a power, a
rounding. The `math` package is a toolbox that is already full: you pick up the tool,
without having to build it.

The `math` package contains the common mathematical functions.

```go
package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println(math.Sqrt(16))
	fmt.Println(math.Pow(2, 10))
	fmt.Println(math.Abs(-7.5))
	fmt.Println(math.Round(2.5), math.Floor(2.7), math.Ceil(2.1))
	fmt.Println(math.Max(3, 8), math.Min(3, 8))
	fmt.Printf("%.4f\n", math.Pi)
}
```

```sortie
4
1024
7.5
3 2 3
8 3
3.1416
```

| Function | Role |
|----------|------|
| `math.Sqrt(x)` | square root |
| `math.Pow(x, y)` | x to the power of y |
| `math.Abs(x)` | absolute value (without the minus sign) |
| `math.Round(x)` | round to the nearest |
| `math.Floor(x)`, `math.Ceil(x)` | round down, round up |
| `math.Max(a, b)`, `math.Min(a, b)` | the largest, the smallest |

These functions work with `float64`. For two integers, Go 1.21 added the `max` and
`min` functions, with no import:

```go
package main

import "fmt"

func main() {
	fmt.Println(max(4, 9, 2), min(4, 9, 2))
}
```

```sortie
9 2
```

## Random numbers

A game often needs surprises: rolling a die, shuffling cards, placing an enemy
anywhere. For that, you need numbers drawn at random.

The `math/rand` package draws random numbers. `rand.Intn(6)` gives an integer between
0 and 5: you add 1 to simulate a die.

A computer can't really draw at random: it calculates a sequence of numbers that
*seem* random, starting from a starting number, the **seed**. With the same seed, you
always get the same sequence. That's very handy for tests, and for this site: the
output below never changes.

```go
package main

import (
	"fmt"
	"math/rand"
)

func main() {
	r := rand.New(rand.NewSource(1))
	fmt.Println(r.Intn(6)+1, r.Intn(6)+1, r.Intn(6)+1)
	fmt.Println(r.Intn(6)+1, r.Intn(6)+1, r.Intn(6)+1)
	fmt.Printf("%.3f\n", r.Float64())
}
```

```sortie
6 4 6
6 2 1
0.066
```

- `rand.NewSource(1)` creates a source with the seed `1`;
- `rand.New(...)` creates a generator `r` from that source;
- `r.Intn(n)` gives an integer from 0 to n-1, `r.Float64()` a number between 0 and 1.

> **Tip:** in a real game, call `rand.Intn(6)` directly without creating a source: Go
> then picks a different seed each time the program starts.

## Displaying a number

A calculation sometimes gives too many decimal places: `3.14159` euros makes no sense
on a receipt, where a price always has two decimal places. So you choose how the number
is displayed.

`fmt.Printf` controls the display with its verbs:

```go
package main

import "fmt"

func main() {
	price := 3.14159
	fmt.Printf("%.2f\n", price)
	fmt.Printf("[%8.2f]\n", price)
	fmt.Printf("[%-6d]\n", 42)
	fmt.Printf("%03d\n", 7)
	fmt.Printf("%e\n", 1234567.0)
	fmt.Printf("%b %x\n", 10, 255)
}
```

```sortie
3.14
[    3.14]
[42    ]
007
1.234567e+06
1010 ff
```

| Format | Effect |
|--------|--------|
| `%.2f` | 2 decimal places |
| `%8.2f` | 2 decimal places, 8 characters wide (right-aligned) |
| `%-6d` | integer 6 characters wide, left-aligned |
| `%03d` | integer on 3 digits, padded with zeros |
| `%e` | scientific notation |
| `%b`, `%x` | binary, hexadecimal |

To get the text without displaying it, use `fmt.Sprintf` with the same verbs:
`s := fmt.Sprintf("%.2f", price)`.

**Your turn:** write a program that calculates the price of 3 notebooks at 4.5 euros
each, then displays "Total: 13.50 euros", with two decimal places.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	quantity := 3
> 	price := 4.5
> 	total := float64(quantity) * price
> 	fmt.Printf("Total: %.2f euros\n", total)
> }
> ```
>
> ```sortie
> Total: 13.50 euros
> ```

## Common mistakes

### Mixing int and float64

```go norun
package main

import "fmt"

func main() {
	quantity := 3
	price := 2.5
	fmt.Println(quantity * price)
}
```

```text
# command-line-arguments
./main.go:8:14: invalid operation: quantity * price (mismatched types int and float64)
```

The types don't match (*mismatched types*). Convert: `float64(quantity) * price`.

### Forgetting the second value of strconv.Atoi

```go norun
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n := strconv.Atoi("42")
	fmt.Println(n)
}
```

```text
# command-line-arguments
./main.go:9:7: assignment mismatch: 1 variable but strconv.Atoi returns 2 values
```

`Atoi` returns 2 values: you need 2 variables. Write `n, err := strconv.Atoi("42")`,
then test `err`.

### Dividing by zero

```go norun
package main

import "fmt"

func main() {
	a := 10
	b := 0
	fmt.Println(a / b)
}
```

```text
panic: runtime error: integer divide by zero

goroutine 1 [running]:
main.main()
	/tmp/tmp.nGOUtnybpF/main.go:8 +0x9
exit status 2
```

The program stops dead: this is a **panic**. Check the divisor first:
`if b != 0 { ... }`.

### A value too large for its type

```go norun
package main

import "fmt"

func main() {
	var small int8 = 200
	fmt.Println(small)
}
```

```text
# command-line-arguments
./main.go:6:19: cannot use 200 (untyped int constant) as int8 value in variable declaration (overflows)
```

An `int8` goes up to 127: 200 is too big (*overflows*). Pick a larger type, such as
`int`.

## Test yourself

**Question 1:** what are `17 / 5` and `17 % 5`?

> **Answer**
>
> `3` and `2`. 17 = 5 × 3 + 2: the quotient is 3, the remainder is 2.

**Question 2:** convert the text `"250"` to a number, then display half of it with one
decimal place.

> **Answer**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"strconv"
> )
>
> func main() {
> 	n, err := strconv.Atoi("250")
> 	if err != nil {
> 		fmt.Println(err)
> 		return
> 	}
> 	fmt.Printf("%.1f\n", float64(n)/2)
> }
> ```
>
> ```sortie
> 125.0
> ```

**Question 3:** why is `int(9.99)` equal to `9`?

> **Answer**
>
> Converting to `int` cuts off the decimal part, without rounding. To round, use
> `int(math.Round(9.99))`, which gives `10`.

## Summary

- `int` for whole numbers, `float64` for decimal numbers.
- Between two integers, `/` is an integer division; `%` gives the remainder.
- Go never converts on its own: write `float64(x)` or `int(x)`.
- `strconv.Atoi`, `strconv.Itoa` and `strconv.ParseFloat` go from text to numbers,
  and return an error to check.
- `math` provides roots, powers and rounding; `math/rand` draws at random.
- `%.2f`, `%03d` or `%8.2f` control the display.

Next: [conditions and loops](conditions-boucles.md).
