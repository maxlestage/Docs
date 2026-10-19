# Exercises to get started

This page offers 15 small, very guided exercises, all easy, to practice the basics of Go: variables, arithmetic, conditions, loops, functions, text, slices and maps.

## How to do these exercises

Take them in order. They go from the simplest to the least simple. For each one:

1. **Read** the instructions and the expected output.
2. **Try it on paper**: write the steps in plain English, like a recipe.
3. **Write** the code in a `main.go` file.
4. **Run it** with `go run main.go`.
5. **Compare** what is shown with the expected output, character by character.
6. Stuck? Read **the first hint**. Then the second one. The solution comes last.

```text
 read → paper → write → run → compare
                  ↑                │
                  └── not the same ┘
```

> **Tip:** every program starts with the same lines. Copy this template and fill in
> `main`:
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	fmt.Println("your turn!")
> }
> ```
>
> ```sortie
> your turn!
> ```

An error message? That's normal. Read [the beginner's traps](pieges.md): it's probably
there.

## Printing and calculating

### Exercise 1: Hello

Level: ★☆☆

Print two lines: `Hello!` then `I'm learning Go.`

Expected output:

```text
Hello!
I'm learning Go.
```

> **Hint:** one `fmt.Println` per line.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	fmt.Println("Hello!")
> 	fmt.Println("I'm learning Go.")
> }
> ```
>
> ```sortie
> Hello!
> I'm learning Go.
> ```
>
> Why it works: each `fmt.Println` prints its text and then moves to the next line.

### Exercise 2: My profile

Level: ★☆☆

Create a variable `name` set to `"Léa"` and a variable `age` set to `11`.
Print `Léa is 11 years old.`

Expected output:

```text
Léa is 11 years old.
```

> **Hint:** create the variables with `:=`.

> **Hint:** `fmt.Println` puts a space between every piece, even where you don't want one
> (try `"years old"` and `"."` as two pieces!). Try `fmt.Printf` instead, with `%s` (a
> text) and `%d` (an integer), without forgetting `\n`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	name := "Léa"
> 	age := 11
> 	fmt.Printf("%s is %d years old.\n", name, age)
> }
> ```
>
> ```sortie
> Léa is 11 years old.
> ```
>
> Why it works: `Printf` replaces `%s` with `name` and `%d` with `age`, in order. `\n`
> moves to the next line.

### Exercise 3: The total price

Level: ★☆☆

A notebook costs 3 euros. You buy 4 of them. Calculate the total in a variable and
print `Total: 12 euros`.

Expected output:

```text
Total: 12 euros
```

> **Hint:** `total := price * quantity`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	price := 3
> 	quantity := 4
> 	total := price * quantity
> 	fmt.Println("Total:", total, "euros")
> }
> ```
>
> ```sortie
> Total: 12 euros
> ```
>
> Why it works: `*` multiplies. `Println` adds a space between the pieces.

### Exercise 4: Sharing candies

Level: ★☆☆

17 candies for 5 children. How many does each child get, and how many are left over?

Expected output:

```text
Each: 3
Left: 2
```

> **Hint:** between two integers, `/` gives the integer division.

> **Hint:** `%` (modulo) gives the remainder of the division.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	candies := 17
> 	children := 5
> 	fmt.Println("Each:", candies/children)
> 	fmt.Println("Left:", candies%children)
> }
> ```
>
> ```sortie
> Each: 3
> Left: 2
> ```
>
> Why it works: 5 × 3 = 15, and 17 − 15 = 2. `/` gives 3, `%` gives 2.

### Exercise 5: The average

Level: ★☆☆

Calculate the average of three grades: 12, 15 and 14. Print it with one decimal place.

Expected output:

```text
Average: 13.7
```

> **Hint:** 12 + 15 + 14 = 41. Divided by 3, that makes 13.66…: you need a
> `float64`, otherwise Go cuts off the decimals.

> **Hint:** `float64(sum) / 3`, then `fmt.Printf("%.1f\n", …)` for a single
> decimal place (rounded).

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	sum := 12 + 15 + 14
> 	average := float64(sum) / 3
> 	fmt.Printf("Average: %.1f\n", average)
> }
> ```
>
> ```sortie
> Average: 13.7
> ```
>
> Why it works: the conversion happens **before** the division, so the calculation keeps
> the decimals. `%.1f` rounds 13.666… to 13.7.

## Conditions and loops

### Exercise 6: Adult or minor

Level: ★☆☆

With `age := 15`, print `Minor`. With 18 or more, the program must print `Adult`.

Expected output:

```text
Minor
```

> **Hint:** `if age >= 18 { … } else { … }`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	age := 15
> 	if age >= 18 {
> 		fmt.Println("Adult")
> 	} else {
> 		fmt.Println("Minor")
> 	}
> }
> ```
>
> ```sortie
> Minor
> ```
>
> Why it works: 15 >= 18 is false, so Go runs the `else` block. Try with
> `age := 18`.

### Exercise 7: Counting to 5

Level: ★☆☆

Print the numbers from 1 to 5, one per line.

Expected output:

```text
1
2
3
4
5
```

> **Hint:** `for i := 1; i <= 5; i++ { … }`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for i := 1; i <= 5; i++ {
> 		fmt.Println(i)
> 	}
> }
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> ```
>
> Why it works: `i` starts at 1, the loop runs as long as `i <= 5`, and `i++` adds 1
> at each turn.

### Exercise 8: The 7 times table

Level: ★☆☆

Print the multiplication table of 7, from 1 to 5.

Expected output:

```text
7 x 1 = 7
7 x 2 = 14
7 x 3 = 21
7 x 4 = 28
7 x 5 = 35
```

> **Hint:** reuse the loop from exercise 7.

> **Hint:** inside the loop: `fmt.Printf("7 x %d = %d\n", i, 7*i)`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for i := 1; i <= 5; i++ {
> 		fmt.Printf("7 x %d = %d\n", i, 7*i)
> 	}
> }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```
>
> Why it works: the first `%d` receives `i`, the second receives `7*i`.

### Exercise 9: Adding from 1 to 10

Level: ★☆☆

Calculate 1 + 2 + 3 + … + 10 with a loop.

Expected output:

```text
Sum: 55
```

> **Hint:** create `total := 0` **before** the loop. Inside the loop, add `i` to
> `total`.

> **Hint:** follow the values on paper:
>
> | Turn | i | total |
> |---|---|---|
> | 1 | 1 | 1 |
> | 2 | 2 | 3 |
> | 3 | 3 | 6 |
> | … | … | … |
> | 10 | 10 | 55 |

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	total := 0
> 	for i := 1; i <= 10; i++ {
> 		total += i
> 	}
> 	fmt.Println("Sum:", total)
> }
> ```
>
> ```sortie
> Sum: 55
> ```
>
> Why it works: `total += i` means `total = total + i`. Be careful not to write
> `total := …` inside the loop: that would be another variable.

### Exercise 10: FizzBuzz up to 15

Level: ★☆☆

For each number from 1 to 15: print `Fizz` if it is divisible by 3, `Buzz` if it is
divisible by 5, `FizzBuzz` if it is divisible by both, otherwise the number.

Expected output:

```text
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
```

> **Hint:** "divisible by 3" is written `i%3 == 0`.

> **Hint:** test the "both" case first (`i%15 == 0`). Otherwise, 15 would only print
> `Fizz`. A `switch` with no value reads well here.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for i := 1; i <= 15; i++ {
> 		switch {
> 		case i%15 == 0:
> 			fmt.Println("FizzBuzz")
> 		case i%3 == 0:
> 			fmt.Println("Fizz")
> 		case i%5 == 0:
> 			fmt.Println("Buzz")
> 		default:
> 			fmt.Println(i)
> 		}
> 	}
> }
> ```
>
> ```sortie
> 1
> 2
> Fizz
> 4
> Buzz
> Fizz
> 7
> 8
> Fizz
> Buzz
> 11
> Fizz
> 13
> 14
> FizzBuzz
> ```
>
> Why it works: the `switch` takes the **first** true case. The 15 case is placed first,
> so it comes before `Fizz` and `Buzz`.

## Functions and text

### Exercise 11: A square function

Level: ★☆☆

Write a function `square` that receives an integer and returns its square. Print
`square(4)` and `square(9)`.

Expected output:

```text
16
81
```

> **Hint:** `func square(n int) int { … }`. The `int` after the parenthesis is the type
> of what is returned.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func square(n int) int {
> 	return n * n
> }
>
> func main() {
> 	fmt.Println(square(4))
> 	fmt.Println(square(9))
> }
> ```
>
> ```sortie
> 16
> 81
> ```
>
> Why it works: `return` sends `n * n` back to the place of the call. `square(4)` is
> replaced by 16.

### Exercise 12: Shouting a name

Level: ★☆☆

Write a function `shout` that receives a text and returns that text in capital letters,
followed by `!!!`. Test it with `"hello chloé"`.

Expected output:

```text
HELLO CHLOÉ!!!
```

> **Hint:** import `"strings"` as well as `"fmt"`. `strings.ToUpper` turns text into
> capital letters.

> **Hint:** join two texts with `+`.

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
> func shout(text string) string {
> 	return strings.ToUpper(text) + "!!!"
> }
>
> func main() {
> 	fmt.Println(shout("hello chloé"))
> }
> ```
>
> ```sortie
> HELLO CHLOÉ!!!
> ```
>
> Why it works: `ToUpper` makes a new text in capital letters (even the `é`), then `+`
> sticks `!!!` onto it.

### Exercise 13: Counting letters

Level: ★☆☆

Count the letters of the word `"café"` and print `café: 4 letters`.

Expected output:

```text
café: 4 letters
```

> **Hint:** `len("café")` gives 5, not 4: it counts the bytes, and the `é` takes two
> of them.

> **Hint:** turn the text into runes (one rune = one letter): `[]rune(word)`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	word := "café"
> 	fmt.Printf("%s: %d letters\n", word, len([]rune(word)))
> }
> ```
>
> ```sortie
> café: 4 letters
> ```
>
> Why it works: `[]rune(word)` splits the text into 4 letters, and `len` counts them.

## Slices and maps

### Exercise 14: The biggest

Level: ★☆☆

Find the biggest number in the slice `[]int{4, 17, 9, 23, 8}`, with a loop.

Expected output:

```text
The biggest: 23
```

> **Hint:** keep the "champion" in a variable. At the start, it's the first slot:
> `biggest := numbers[0]`.

> **Hint:** go through the slice with `range`. If a number is bigger than the champion,
> it becomes the new champion.
>
> | Number read | Bigger? | biggest |
> |---|---|---|
> | 4 | no | 4 |
> | 17 | yes | 17 |
> | 9 | no | 17 |
> | 23 | yes | 23 |
> | 8 | no | 23 |

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	numbers := []int{4, 17, 9, 23, 8}
> 	biggest := numbers[0]
> 	for _, n := range numbers {
> 		if n > biggest {
> 			biggest = n
> 		}
> 	}
> 	fmt.Println("The biggest:", biggest)
> }
> ```
>
> ```sortie
> The biggest: 23
> ```
>
> Why it works: at each turn, the champion is compared with the number read. At the
> end, only the biggest is left. (The name `biggest` avoids hiding Go's `max`
> function.)

### Exercise 15: Counting fruit

Level: ★☆☆

Here is a basket: `[]string{"apple", "pear", "apple", "kiwi", "apple", "kiwi"}`.
Count each fruit with a map, then print the fruits in alphabetical order.

Expected output:

```text
apple: 3
kiwi: 2
pear: 1
```

> **Hint:** `count := make(map[string]int)`, then `count[fruit]++` for each
> fruit. A missing key is worth 0: the first `++` gives 1.

> **Hint:** the order of a map is random. For a fixed order, go through
> `slices.Sorted(maps.Keys(count))` (import `"maps"` and `"slices"`).

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
> 	basket := []string{"apple", "pear", "apple", "kiwi", "apple", "kiwi"}
> 	count := make(map[string]int)
> 	for _, fruit := range basket {
> 		count[fruit]++
> 	}
> 	for _, fruit := range slices.Sorted(maps.Keys(count)) {
> 		fmt.Printf("%s: %d\n", fruit, count[fruit])
> 	}
> }
> ```
>
> ```sortie
> apple: 3
> kiwi: 2
> pear: 1
> ```
>
> Why it works: the map keeps one counter per fruit. Sorting the keys always gives the
> same output. Review [maps](maps.md#sorting-the-keys-before-printing) if needed.

## What's next?

Well done! You have practiced all the basics. For slightly harder exercises, see
[the solved exercises](exercices.md).

Next: [types](types.md).
