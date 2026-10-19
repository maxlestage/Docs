# Solved exercises: second set

This page offers 12 new solved exercises in Go, from easiest to hardest, with tables that follow the variables step by step.

## How to use this page

These exercises are **new**: they repeat neither
[the exercises to get started](exercices-faciles.md) nor
[the first set](exercices.md). Each one has a level:

- ★☆☆: easy (4 exercises);
- ★★☆: medium (5 exercises);
- ★★★: harder (3 exercises).

For each exercise:

1. Read the instructions and the expected output.
2. Write the steps in plain English, on paper, like a recipe.
3. Code in `main.go`, then run `go run main.go`.
4. Stuck? Read the hint. The solution comes last.

```text
 instructions → paper → code → go run → right result?
                         ▲                    │
                         └─────── no ─────────┘
```

> **Tip:** when a result surprises you, make a trace table, as in
> [Follow a program step by step](pas-a-pas.md). The solutions below show
> several of them.

## Warm-up

### Exercise 1: Seconds to hours

Level: ★☆☆

A movie lasts 3725 seconds. Print its length in hours, minutes and seconds:
`1 h 2 min 5 s`.

> **Hint:** an hour has 3600 seconds, a minute has 60. Between two integers, `/`
> gives the integer division and `%` the remainder.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	total := 3725
> 	hours := total / 3600
> 	rest := total % 3600
> 	minutes := rest / 60
> 	seconds := rest % 60
> 	fmt.Printf("%d h %d min %d s\n", hours, minutes, seconds)
> }
> ```
>
> ```sortie
> 1 h 2 min 5 s
> ```
>
> Why it works: it's like sorting coins. You first fill the "one-hour boxes" (1 box,
> 125 s left), then the "one-minute boxes" (2 boxes, 5 s left).
>
> | Step | Calculation | Result |
> |---|---|---|
> | hours | 3725 / 3600 | 1 |
> | rest | 3725 % 3600 | 125 |
> | minutes | 125 / 60 | 2 |
> | seconds | 125 % 60 | 5 |
>
> Try it: with `total := 59`, the program prints `0 h 0 min 59 s`.

### Exercise 2: Count the vowels

Level: ★☆☆

Count the vowels (a, e, i, o, u, y) in the word `"programming"`.

Expected output:

```text
programming has 3 vowels
```

> **Hint:** go through the word with `range`. `strings.ContainsRune("aeiouy", letter)`
> answers `true` if the letter is one of the vowels.

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
> func main() {
> 	word := "programming"
> 	vowels := 0
> 	for _, letter := range word {
> 		if strings.ContainsRune("aeiouy", letter) {
> 			vowels++
> 		}
> 	}
> 	fmt.Println(word, "has", vowels, "vowels")
> }
> ```
>
> ```sortie
> programming has 3 vowels
> ```
>
> Why it works: the string `"aeiouy"` acts as a reference list, like a shopping list
> you check against. Each letter of the word is looked up in it: o, a, i are found,
> so 3.

### Exercise 3: The sum of the digits

Level: ★☆☆

Compute the sum of the digits of `4721`, without turning it into text.

Expected output:

```text
Sum of the digits of 4721 is 14
```

> **Hint:** `n % 10` gives the last digit (`4721 % 10` is 1). `n / 10` removes it
> (`4721 / 10` is 472). Repeat as long as `n` is not 0.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	number := 4721
> 	n := number
> 	sum := 0
> 	for n > 0 {
> 		sum += n % 10
> 		n = n / 10
> 	}
> 	fmt.Println("Sum of the digits of", number, "is", sum)
> }
> ```
>
> ```sortie
> Sum of the digits of 4721 is 14
> ```
>
> Why it works: on each round, you "tear off" the last digit, like tearing the last
> page out of a notebook.
>
> | Round | n at start | n % 10 | sum | n at end |
> |---|---|---|---|---|
> | 1 | 4721 | 1 | 1 | 472 |
> | 2 | 472 | 2 | 3 | 47 |
> | 3 | 47 | 7 | 10 | 4 |
> | 4 | 4 | 4 | 14 | 0 |
> | end | 0 | `0 > 0` is false | 14 | — |
>
> We copy `number` into `n` so we can print the starting number at the end.

### Exercise 4: A triangle of stars

Level: ★☆☆

Print a triangle of 4 lines: one star on the first, two on the second, and so on.

Expected output:

```text
*
**
***
****
```

> **Hint:** two loops, one inside the other. The outer loop counts the lines
> (`line` from 1 to 4). The inner one prints `line` stars with `fmt.Print("*")`,
> which does not move to the next line.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for line := 1; line <= 4; line++ {
> 		for star := 1; star <= line; star++ {
> 			fmt.Print("*")
> 		}
> 		fmt.Println()
> 	}
> }
> ```
>
> ```sortie
> *
> **
> ***
> ****
> ```
>
> Why it works: the inner loop does a **full** run for each round of the outer one,
> like the hands of a clock: the big hand goes all the way round while the small one
> moves one notch. `fmt.Println()` with nothing inside ends the line.
>
> Try it: replace `star <= line` with `star <= 5-line`. The triangle is upside down:
> 4 stars, then 3, 2, 1.

> **Remember:** `/` and `%` cut a number into pieces; a loop inside a loop draws lines
> and columns.

## A bit more thinking

### Exercise 5: Prime numbers

Level: ★★☆

A prime number can only be divided by 1 and by itself (7 is prime, 8 is not). Write
`isPrime(n int) bool`, then print the prime numbers from 2 to 30 on one line.

Expected output:

```text
2 3 5 7 11 13 17 19 23 29
```

> **Hint:** try dividing `n` by 2, 3, 4… If a remainder is 0 (`n%d == 0`), `n` is not
> prime: return `false` right away. You only need to test the `d` such that
> `d*d <= n`.

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
> func isPrime(n int) bool {
> 	if n < 2 {
> 		return false
> 	}
> 	for d := 2; d*d <= n; d++ {
> 		if n%d == 0 {
> 			return false
> 		}
> 	}
> 	return true
> }
>
> func main() {
> 	var primes []string
> 	for n := 2; n <= 30; n++ {
> 		if isPrime(n) {
> 			primes = append(primes, strconv.Itoa(n))
> 		}
> 	}
> 	fmt.Println(strings.Join(primes, " "))
> }
> ```
>
> ```sortie
> 2 3 5 7 11 13 17 19 23 29
> ```
>
> Why it works: the function is a bouncer at the door of a club. As soon as it finds
> a divisor, it says "no" and stops. For 25:
>
> | d | d*d <= 25? | 25 % d | Decision |
> |---|---|---|---|
> | 2 | 4: yes | 1 | keep going |
> | 3 | 9: yes | 1 | keep going |
> | 4 | 16: yes | 1 | keep going |
> | 5 | 25: yes | **0** | `return false` |
>
> If `n` had a divisor bigger than its square root, it would also have a smaller one,
> already tested: hence `d*d <= n`.

### Exercise 6: The Fibonacci sequence

Level: ★★☆

In the Fibonacci sequence, each number is the sum of the two before it. It starts
with 0 and 1. Print its first 10 numbers.

Expected output:

```text
0 1 1 2 3 5 8 13 21 34
```

> **Hint:** keep two variables, `a` (the current number) and `b` (the next one). On
> each round, print `a`, then move one step forward with `a, b = b, a+b`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	a, b := 0, 1
> 	for i := 1; i <= 10; i++ {
> 		fmt.Print(a)
> 		if i < 10 {
> 			fmt.Print(" ")
> 		}
> 		a, b = b, a+b
> 	}
> 	fmt.Println()
> }
> ```
>
> ```sortie
> 0 1 1 2 3 5 8 13 21 34
> ```
>
> Why it works: `a` and `b` move forward together, like two feet walking.
> `a, b = b, a+b` computes the **whole** right-hand side with the old values, then
> stores both results.
>
> | Round | prints a | new a | new b |
> |---|---|---|---|
> | 1 | 0 | 1 | 1 |
> | 2 | 1 | 1 | 2 |
> | 3 | 1 | 2 | 3 |
> | 4 | 2 | 3 | 5 |
> | 5 | 3 | 5 | 8 |
> | … | … | … | … |
> | 10 | 34 | 55 | 89 |
>
> The `if i < 10` avoids an extra space at the end of the line.

### Exercise 7: Shared hobbies

Level: ★★☆

Lea likes `reading`, `football`, `piano` and `chess`. Tom likes `piano`, `swimming`
and `reading`. Print the hobbies they share, in the order of Lea's list.

Expected output:

```text
In common: [reading piano]
```

> **Hint:** put Tom's hobbies in a `map[string]bool`. Then go through Lea's:
> `likesTom[hobby]` is `true` only if Tom likes it too.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	lea := []string{"reading", "football", "piano", "chess"}
> 	tom := []string{"piano", "swimming", "reading"}
>
> 	likesTom := make(map[string]bool)
> 	for _, hobby := range tom {
> 		likesTom[hobby] = true
> 	}
>
> 	var shared []string
> 	for _, hobby := range lea {
> 		if likesTom[hobby] {
> 			shared = append(shared, hobby)
> 		}
> 	}
> 	fmt.Println("In common:", shared)
> }
> ```
>
> ```sortie
> In common: [reading piano]
> ```
>
> Why it works: the map is a guest list at the door. A missing key returns `false`
> (the zero value of a `bool`), so `likesTom["football"]` is false without any error.
> Since we go through Lea's **slice**, the order is always the same.

### Exercise 8: Compress a text

Level: ★★☆

Write `compress(s string) string`, which replaces each run of identical letters with
the letter followed by the number of repetitions: `"aaabccdddd"` becomes `"a3b1c2d4"`.

> **Hint:** keep the current letter and a counter. When the letter changes, write the
> letter and its counter, then start again at 1. Don't forget the last run, after the
> loop.

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
> func compress(s string) string {
> 	letters := []rune(s)
> 	if len(letters) == 0 {
> 		return ""
> 	}
> 	result := ""
> 	current := letters[0]
> 	count := 1
> 	for _, l := range letters[1:] {
> 		if l == current {
> 			count++
> 		} else {
> 			result += string(current) + strconv.Itoa(count)
> 			current = l
> 			count = 1
> 		}
> 	}
> 	result += string(current) + strconv.Itoa(count)
> 	return result
> }
>
> func main() {
> 	fmt.Println(compress("aaabccdddd"))
> 	fmt.Println(compress("éé"))
> }
> ```
>
> ```sortie
> a3b1c2d4
> é2
> ```
>
> Why it works: it's like counting cars of the same color in a traffic jam. When the
> color changes, you write down the group and start again.
>
> | Letter read | Same? | current | count | result |
> |---|---|---|---|---|
> | (start) | — | a | 1 | "" |
> | a | yes | a | 2 | "" |
> | a | yes | a | 3 | "" |
> | b | no | b | 1 | "a3" |
> | c | no | c | 1 | "a3b1" |
> | c | yes | c | 2 | "a3b1" |
> | d | no | d | 1 | "a3b1c2" |
> | d, d, d | yes | d | 4 | "a3b1c2" |
> | (after the loop) | — | d | 4 | "a3b1c2d4" |
>
> The `[]rune` makes accented letters work, like `é`.

### Exercise 9: Group by first letter

Level: ★★☆

Group the fruits `banana`, `avocado`, `blueberry`, `apricot`, `cherry` by their first
letter. Print the letters in alphabetical order.

Expected output:

```text
a: [avocado apricot]
b: [banana blueberry]
c: [cherry]
```

> **Hint:** a `map[string][]string`: the key is the letter, the value a slice of
> fruits. `fruit[:1]` gives the first letter (here, without an accent). For the order,
> sort the keys with `slices.Sorted(maps.Keys(...))`.

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"maps"
> 	"slices"
> )
>
> func main() {
> 	fruits := []string{"banana", "avocado", "blueberry", "apricot", "cherry"}
> 	byLetter := make(map[string][]string)
> 	for _, fruit := range fruits {
> 		letter := fruit[:1]
> 		byLetter[letter] = append(byLetter[letter], fruit)
> 	}
> 	for _, letter := range slices.Sorted(maps.Keys(byLetter)) {
> 		fmt.Println(letter+":", byLetter[letter])
> 	}
> }
> ```
>
> ```sortie
> a: [avocado apricot]
> b: [banana blueberry]
> c: [cherry]
> ```
>
> Why it works: the map is a chest of drawers, one drawer per letter. For a new
> letter, `byLetter[letter]` is an empty slice (`nil`), and `append` on `nil` works:
> the drawer is created with the first fruit.
>
> ```text
>  ┌──── a ────┐ ┌──── b ────┐ ┌──── c ────┐
>  │ avocado   │ │ banana    │ │ cherry    │
>  │ apricot   │ │ blueberry │ │           │
>  └───────────┘ └───────────┘ └───────────┘
> ```

> **Remember:** a map is for **finding things fast** (exercise 7) and for **grouping**
> (exercise 9); sort its keys for a stable display.

## For the brave

### Exercise 10: Properly closed brackets

Level: ★★★

Write `balanced(s string) bool`. It checks that each `(`, `[` or `{` is closed by the
right sign, in the right order. Test `"(a[b]{c})"`, `"(]"` and `"(("`.

Expected output:

```text
"(a[b]{c})": true
"(]": false
"((": false
```

> **Hint:** use a **stack**: a slice where you always add and remove at the end. An
> opening sign is pushed. A closing sign must match the last pushed sign, which you
> then remove. At the end, the stack must be empty.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func balanced(s string) bool {
> 	pairs := map[rune]rune{')': '(', ']': '[', '}': '{'}
> 	var stack []rune
> 	for _, c := range s {
> 		switch c {
> 		case '(', '[', '{':
> 			stack = append(stack, c)
> 		case ')', ']', '}':
> 			if len(stack) == 0 || stack[len(stack)-1] != pairs[c] {
> 				return false
> 			}
> 			stack = stack[:len(stack)-1]
> 		}
> 	}
> 	return len(stack) == 0
> }
>
> func main() {
> 	for _, s := range []string{"(a[b]{c})", "(]", "(("} {
> 		fmt.Printf("%q: %t\n", s, balanced(s))
> 	}
> }
> ```
>
> ```sortie
> "(a[b]{c})": true
> "(]": false
> "((": false
> ```
>
> Why it works: the stack is a stack of plates. You can only take the one on top. For
> `"(a[b]{c})"` (the letters are ignored):
>
> | Sign read | Action | Stack after |
> |---|---|---|
> | `(` | push | `(` |
> | `[` | push | `( [` |
> | `]` | the top is `[`: remove it | `(` |
> | `{` | push | `( {` |
> | `}` | the top is `{`: remove it | `(` |
> | `)` | the top is `(`: remove it | (empty) |
>
> Empty stack at the end: `true`. For `"(("`, two `(` are left: `false`.

### Exercise 11: Searching by halves

Level: ★★★

The slice `[]int{3, 8, 15, 21, 34, 42, 57, 63}` is sorted. Write
`search(s []int, target int) (int, int)`, which returns the position of `target` (or
-1) and the number of tries. On each try, look at the middle and throw away the
useless half. Search for 42, then 10.

Expected output:

```text
42: position 5 (2 tries)
10: not found (3 tries)
```

> **Hint:** keep two bounds, `low` and `high`. The middle is `(low + high) / 2`. If
> the middle value is too small, `low = middle + 1`; too big, `high = middle - 1`.
> Stop when `low > high`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func search(s []int, target int) (int, int) {
> 	low, high := 0, len(s)-1
> 	tries := 0
> 	for low <= high {
> 		tries++
> 		middle := (low + high) / 2
> 		if s[middle] == target {
> 			return middle, tries
> 		} else if s[middle] < target {
> 			low = middle + 1
> 		} else {
> 			high = middle - 1
> 		}
> 	}
> 	return -1, tries
> }
>
> func main() {
> 	numbers := []int{3, 8, 15, 21, 34, 42, 57, 63}
> 	for _, target := range []int{42, 10} {
> 		pos, tries := search(numbers, target)
> 		if pos == -1 {
> 			fmt.Printf("%d: not found (%d tries)\n", target, tries)
> 		} else {
> 			fmt.Printf("%d: position %d (%d tries)\n", target, pos, tries)
> 		}
> 	}
> }
> ```
>
> ```sortie
> 42: position 5 (2 tries)
> 10: not found (3 tries)
> ```
>
> Why it works: it's the "higher or lower" game. You open a dictionary in the middle,
> and keep only the half where the word can be. For 10:
>
> | Try | low | high | middle | s[middle] | Decision |
> |---|---|---|---|---|---|
> | 1 | 0 | 7 | 3 | 21 | too big: high = 2 |
> | 2 | 0 | 2 | 1 | 8 | too small: low = 2 |
> | 3 | 2 | 2 | 2 | 15 | too big: high = 1 |
> | end | 2 | 1 | — | — | low > high: not found |
>
> ```text
>  [ 3 | 8 | 15 | 21 | 34 | 42 | 57 | 63 ]   try 1: middle = 21
>  [ 3 | 8 | 15 ]                             try 2: middle = 8
>          [ 15 ]                             try 3: middle = 15
> ```
>
> With 1000 numbers, you need at most 10 tries, instead of 1000.

### Exercise 12: Roman numerals

Level: ★★★

Write `roman(n int) string`, which writes a number in Roman numerals. Test 1994, 2026
and 49.

Expected output:

```text
1994 = MCMXCIV
2026 = MMXXVI
49 = XLIX
```

> **Hint:** prepare two slices side by side, from biggest to smallest, **with** the
> special cases: `1000 M, 900 CM, 500 D, 400 CD, 100 C, 90 XC, 50 L, 40 XL, 10 X,
> 9 IX, 5 V, 4 IV, 1 I`. As long as `n` is big enough for the value, write its symbol
> and subtract the value.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func roman(n int) string {
> 	values := []int{1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1}
> 	symbols := []string{"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"}
> 	result := ""
> 	for i, v := range values {
> 		for n >= v {
> 			result += symbols[i]
> 			n -= v
> 		}
> 	}
> 	return result
> }
>
> func main() {
> 	for _, n := range []int{1994, 2026, 49} {
> 		fmt.Println(n, "=", roman(n))
> 	}
> }
> ```
>
> ```sortie
> 1994 = MCMXCIV
> 2026 = MMXXVI
> 49 = XLIX
> ```
>
> Why it works: it's like giving change with the biggest coins first. For 1994 (the
> values that are too big are skipped):
>
> | Value | n before | Symbol added | n after | result |
> |---|---|---|---|---|
> | 1000 | 1994 | M | 994 | M |
> | 900 | 994 | CM | 94 | MCM |
> | 90 | 94 | XC | 4 | MCMXC |
> | 4 | 4 | IV | 0 | MCMXCIV |
>
> Putting 900, 400, 90… in the list avoids writing `DCCCC` instead of `CM`.

> **Remember:** for a harder problem, find the right picture (a stack of plates, a
> dictionary, a cash register), then check it with a trace table on a small example.

## Summary

- Exercises 1 to 4 practice `/`, `%`, `range` over a text, and nested loops.
- Exercises 5 to 9 use functions, the swap `a, b = b, a+b`, maps and slices.
- Exercises 10 to 12 call for a method: a stack, searching by halves, greedy change.
- A trace table on a small example beats reading the code ten times.

Next: [the cheat sheet](aide-memoire.md)
