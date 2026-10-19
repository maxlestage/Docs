# Solved exercises

This page offers 12 solved exercises, from easiest to hardest, to practice the whole Go track.

## How to use this page

Each exercise has a level:

- ★☆☆: easy;
- ★★☆: medium;
- ★★★: harder.

Read the instructions, then write your solution in a `main.go` file and run it with
`go run main.go`. Open the solution only after you have tried.

> **Tip:** there are often several good solutions. If your program prints the right
> result, you win! Also remember to run `go vet main.go`: it spots small mistakes.

## The basics

### Exercise 1: Introduce yourself

Level: ★☆☆

Create two variables, `name` and `age`. Then print the sentence
`My name is Alice and I am 30 years old.`, using your variables.

> **Hint:** `fmt.Printf` with `%s` for a text and `%d` for an integer (see
> [first steps](premiers-pas.md)). Don't forget `\n` at the end.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	name := "Alice"
> 	age := 30
> 	fmt.Printf("My name is %s and I am %d years old.\n", name, age)
> }
> ```
>
> ```sortie
> My name is Alice and I am 30 years old.
> ```

### Exercise 2: Even or odd

Level: ★☆☆

For each number in the list `7, 10, 0`, print whether it is even or odd:
`7 is odd`, `10 is even`, `0 is even`.

> **Hint:** `n%2 == 0` is true when `n` is even (see
> [numbers](nombres.md)).

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for _, n := range []int{7, 10, 0} {
> 		if n%2 == 0 {
> 			fmt.Println(n, "is even")
> 		} else {
> 			fmt.Println(n, "is odd")
> 		}
> 	}
> }
> ```
>
> ```sortie
> 7 is odd
> 10 is even
> 0 is even
> ```

### Exercise 3: FizzBuzz

Level: ★☆☆

Print the numbers from 1 to 15, but replace multiples of 3 with `Fizz`, multiples of 5
with `Buzz`, and multiples of both 3 and 5 with `FizzBuzz`. Print everything on one
line, separated by spaces.

> **Hint:** test the "multiple of 15" case first, otherwise it will never be
> reached. A `switch` with no value reads well (see
> [conditions and loops](conditions-boucles.md)).

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"strconv"
> 	"strings"
> )
>
> func main() {
> 	var words []string
> 	for i := 1; i <= 15; i++ {
> 		switch {
> 		case i%15 == 0:
> 			words = append(words, "FizzBuzz")
> 		case i%3 == 0:
> 			words = append(words, "Fizz")
> 		case i%5 == 0:
> 			words = append(words, "Buzz")
> 		default:
> 			words = append(words, strconv.Itoa(i))
> 		}
> 	}
> 	fmt.Println(strings.Join(words, " "))
> }
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```

### Exercise 4: Area and perimeter

Level: ★☆☆

Write a function `rectangle(width, height float64)` that returns **two** values: the
area and the perimeter. Print them for a 4 by 2.5 rectangle.

> **Hint:** a Go function can return several results:
> `func f() (float64, float64)` (see [functions](fonctions.md)).

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func rectangle(width, height float64) (float64, float64) {
> 	area := width * height
> 	perimeter := 2 * (width + height)
> 	return area, perimeter
> }
>
> func main() {
> 	area, perimeter := rectangle(4, 2.5)
> 	fmt.Println("Area:", area)
> 	fmt.Println("Perimeter:", perimeter)
> }
> ```
>
> ```sortie
> Area: 10
> Perimeter: 13
> ```

## Text and collections

### Exercise 5: Average, minimum, maximum

Level: ★★☆

With the slice `grades := []int{12, 15, 9, 14}`, print the average (with one decimal),
the lowest grade and the highest grade.

> **Hint:** convert to `float64` before dividing. The `slices` package provides
> `slices.Min` and `slices.Max` (see [collections](collections.md)).

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"slices"
> )
>
> func main() {
> 	grades := []int{12, 15, 9, 14}
> 	total := 0
> 	for _, g := range grades {
> 		total += g
> 	}
> 	average := float64(total) / float64(len(grades))
> 	fmt.Printf("Average: %.1f\n", average)
> 	fmt.Println("Minimum:", slices.Min(grades))
> 	fmt.Println("Maximum:", slices.Max(grades))
> }
> ```
>
> ```sortie
> Average: 12.5
> Minimum: 9
> Maximum: 15
> ```

### Exercise 6: Counting words

Level: ★★☆

Count how many times each word appears in the sentence
`"the cat and the dog and the rabbit"`. Print the words **in alphabetical order**,
with their count.

> **Hint:** `strings.Fields` splits on spaces; a `map[string]int` counts. The
> iteration order of a map changes from one run to the next: sort the keys (see
> [maps](maps.md)).

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"maps"
> 	"slices"
> 	"strings"
> )
>
> func main() {
> 	sentence := "the cat and the dog and the rabbit"
> 	count := map[string]int{}
> 	for _, word := range strings.Fields(sentence) {
> 		count[word]++
> 	}
> 	for _, word := range slices.Sorted(maps.Keys(count)) {
> 		fmt.Printf("%s: %d\n", word, count[word])
> 	}
> }
> ```
>
> ```sortie
> and: 2
> cat: 1
> dog: 1
> rabbit: 1
> the: 3
> ```

### Exercise 7: Reversing a text

Level: ★★☆

Write a function `reverse(s string) string` that returns the text backwards. It must
work with accents: `"Noël"` gives `"lëoN"`.

> **Hint:** a Go string is a sequence of **bytes**, and `ë` takes up two of them.
> Convert to `[]rune` to work character by character (see
> [working with text](texte.md)).

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func reverse(s string) string {
> 	r := []rune(s)
> 	for i, j := 0, len(r)-1; i < j; i, j = i+1, j-1 {
> 		r[i], r[j] = r[j], r[i]
> 	}
> 	return string(r)
> }
>
> func main() {
> 	fmt.Println(reverse("Hello"))
> 	fmt.Println(reverse("Noël"))
> }
> ```
>
> ```sortie
> olleH
> lëoN
> ```

### Exercise 8: Palindrome

Level: ★★☆

A palindrome reads the same in both directions, ignoring spaces and capital letters.
Write `isPalindrome(s string) bool` and test it on `"Kayak"`, `"Never odd or even"`
and `"Hello"`.

> **Hint:** put the text in lowercase with `strings.ToLower`, remove the spaces with
> `strings.ReplaceAll`, then compare the runes in pairs.

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"strings"
> )
>
> func isPalindrome(s string) bool {
> 	r := []rune(strings.ReplaceAll(strings.ToLower(s), " ", ""))
> 	for i := 0; i < len(r)/2; i++ {
> 		if r[i] != r[len(r)-1-i] {
> 			return false
> 		}
> 	}
> 	return true
> }
>
> func main() {
> 	for _, s := range []string{"Kayak", "Never odd or even", "Hello"} {
> 		fmt.Printf("%q: %t\n", s, isPalindrome(s))
> 	}
> }
> ```
>
> ```sortie
> "Kayak": true
> "Never odd or even": true
> "Hello": false
> ```

## Types and errors

### Exercise 9: Shapes and an interface

Level: ★★☆

Create two types, `Rectangle` (width, height) and `Circle` (radius), that each have an
`Area() float64` method. Define a `Shape` interface, then print the area of each shape
in a `[]Shape` slice, with two decimals, and the total area.

> **Hint:** a type satisfies an interface as soon as it has its methods, without
> declaring anything (see [interfaces](interfaces.md)). `math.Pi` gives π.

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"math"
> )
>
> type Shape interface {
> 	Area() float64
> }
>
> type Rectangle struct {
> 	Width, Height float64
> }
>
> func (r Rectangle) Area() float64 { return r.Width * r.Height }
>
> type Circle struct {
> 	Radius float64
> }
>
> func (c Circle) Area() float64 { return math.Pi * c.Radius * c.Radius }
>
> func main() {
> 	shapes := []Shape{Rectangle{4, 3}, Circle{1}, Rectangle{2, 2}}
> 	total := 0.0
> 	for _, s := range shapes {
> 		fmt.Printf("%T: %.2f\n", s, s.Area())
> 		total += s.Area()
> 	}
> 	fmt.Printf("Total: %.2f\n", total)
> }
> ```
>
> ```sortie
> main.Rectangle: 12.00
> main.Circle: 3.14
> main.Rectangle: 4.00
> Total: 19.14
> ```

### Exercise 10: A safe division

Level: ★★☆

Write `divide(a, b float64) (float64, error)`. If `b` is 0, it returns the error
`ErrDivisionByZero`, declared once and for all. In `main`, try `10 / 4` and `1 / 0`,
and recognize this specific error with `errors.Is`.

> **Hint:** `var ErrDivisionByZero = errors.New("division by zero")` creates a sentinel
> error (see [errors](erreurs.md)).

> **Solution**
>
> ```go
> package main
>
> import (
> 	"errors"
> 	"fmt"
> )
>
> var ErrDivisionByZero = errors.New("division by zero")
>
> func divide(a, b float64) (float64, error) {
> 	if b == 0 {
> 		return 0, ErrDivisionByZero
> 	}
> 	return a / b, nil
> }
>
> func main() {
> 	pairs := [][2]float64{{10, 4}, {1, 0}}
> 	for _, p := range pairs {
> 		result, err := divide(p[0], p[1])
> 		if errors.Is(err, ErrDivisionByZero) {
> 			fmt.Printf("%g / %g: impossible (%v)\n", p[0], p[1], err)
> 			continue
> 		}
> 		fmt.Printf("%g / %g = %g\n", p[0], p[1], result)
> 	}
> }
> ```
>
> ```sortie
> 10 / 4 = 2.5
> 1 / 0: impossible (division by zero)
> ```

## Going further

### Exercise 11: A generic filter

Level: ★★★

Write **a single** generic function `Filter` that keeps the elements of a slice for
which a function returns `true`. Use it to keep the even numbers of a slice of
integers, then the words longer than 4 letters of a slice of strings.

> **Hint:** `func Filter[T any](s []T, keep func(T) bool) []T` (see
> [generics](generiques.md)).

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func Filter[T any](s []T, keep func(T) bool) []T {
> 	var result []T
> 	for _, v := range s {
> 		if keep(v) {
> 			result = append(result, v)
> 		}
> 	}
> 	return result
> }
>
> func main() {
> 	numbers := []int{1, 2, 3, 4, 5, 6}
> 	evens := Filter(numbers, func(n int) bool { return n%2 == 0 })
> 	fmt.Println(evens)
>
> 	words := []string{"go", "gopher", "code", "channel"}
> 	long := Filter(words, func(w string) bool { return len(w) > 4 })
> 	fmt.Println(long)
> }
> ```
>
> ```sortie
> [2 4 6]
> [gopher channel]
> ```

### Exercise 12: A parallel sum

Level: ★★★

Split the numbers from 1 to 100 into 4 chunks of 25. Start one goroutine per chunk:
each one computes the sum of its chunk and sends it into a channel. `main` receives the
4 partial sums and prints the total.

> **Hint:** a channel `make(chan int)`; each goroutine does `ch <- sum`, and `main`
> does `<-ch` four times (see [channels](canaux.md)). The arrival order changes from
> one run to the next, but the total does not.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func sum(numbers []int, ch chan<- int) {
> 	total := 0
> 	for _, n := range numbers {
> 		total += n
> 	}
> 	ch <- total
> }
>
> func main() {
> 	var numbers []int
> 	for i := 1; i <= 100; i++ {
> 		numbers = append(numbers, i)
> 	}
>
> 	ch := make(chan int)
> 	for start := 0; start < len(numbers); start += 25 {
> 		go sum(numbers[start:start+25], ch)
> 	}
>
> 	total := 0
> 	for range 4 {
> 		total += <-ch
> 	}
> 	fmt.Println("Total:", total)
> }
> ```
>
> ```sortie
> Total: 5050
> ```
>
> `chan<- int` means the function can only **send** into the channel. The total doesn't
> depend on the arrival order of the partial sums: the result is always 5050.

## Summary

- Start with the ★☆☆ exercises, then move up in difficulty.
- Always try on your own before opening the solution.
- Exercises 1 to 4 go over the basics: variables, conditions, loops, functions.
- Exercises 5 to 8 work on text, slices and maps.
- Exercises 9 and 10 use interfaces and errors; 11 and 12, generics and
  goroutines.

Next: [solved exercises: second set](exercices-2.md)
