# Arrays and slices

This page presents the two ways to store several values in a row in Go: the array, which has a fixed size, and the slice, which can grow.

> **Goals:**
>
> - you will know how to create an array and a slice, and read their elements;
> - you will know how to add elements with `append`;
> - you will know how to cut a slice with `s[start:end]` and copy it with `copy`;
> - you will know how to avoid the shared-array trap;
> - you will know how to sort and search with the `slices` and `sort` packages.

## Arrays

An array stores a **fixed** number of values of the same type. The size is part of the
type: `[3]int` is "an array of 3 integers".

```go
package main

import "fmt"

func main() {
	var grades [3]int
	grades[0] = 12
	grades[2] = 18
	fmt.Println(grades)

	days := [3]string{"Mon", "Tue", "Wed"}
	fmt.Println(days[1], len(days))
}
```

```sortie
[12 0 18]
Tue 3
```

- The slots are numbered starting from **0**: that's the [index](../debuter/glossaire.md#index).
- A slot that hasn't been filled holds the zero value of its type (`0` here).
- `len` gives the number of slots.

An array can neither grow nor shrink. That's why it is rarely used: you will almost
always prefer the slice.

## Slices

A slice is a [list](../debuter/glossaire.md#list) of values of the same type, whose
size can change. Its type is written without a size: `[]int`.

```go
package main

import "fmt"

func main() {
	fruits := []string{"apple", "kiwi", "pear"}
	fmt.Println(fruits)
	fmt.Println(fruits[0], len(fruits))

	fruits[1] = "mango"
	fmt.Println(fruits)
	fmt.Println(fruits[len(fruits)-1])
}
```

```sortie
[apple kiwi pear]
apple 3
[apple mango pear]
pear
```

`fruits[len(fruits)-1]` is the last element: with 3 elements, the indexes go from 0 to
2.

### Creating a slice with make

`make([]type, length)` creates a slice filled with zero values.

```go
package main

import "fmt"

func main() {
	scores := make([]int, 4)
	scores[2] = 7
	fmt.Println(scores, len(scores))

	var empty []string
	fmt.Println(empty, len(empty), empty == nil)
}
```

```sortie
[0 0 7 0] 4
[] 0 true
```

A slice declared with `var` and no value is `nil`: it is empty, and you can still add
elements to it.

## Adding with append

`append` adds elements at the end of a slice and **returns** the new slice. You must
always keep this result.

```go
package main

import "fmt"

func main() {
	var shopping []string
	shopping = append(shopping, "bread")
	shopping = append(shopping, "milk", "eggs")
	fmt.Println(shopping, len(shopping))

	others := []string{"rice", "tea"}
	shopping = append(shopping, others...)
	fmt.Println(shopping)
}
```

```sortie
[bread milk eggs] 3
[bread milk eggs rice tea]
```

`others...` unpacks the slice `others` into several arguments.

### Capacity

Behind every slice hides an array, which has a certain amount of room: the **capacity**
(`cap`). When there is no room left, `append` creates a bigger array and copies
everything into it. That's why you must keep the result of `append`: it may no longer
be the same array.

```go
package main

import "fmt"

func main() {
	s := make([]int, 0, 3)
	for i := range 5 {
		s = append(s, i)
		fmt.Println(len(s), cap(s))
	}
}
```

```sortie
1 3
2 3
3 3
4 6
5 6
```

- `len`: the number of elements present;
- `cap`: the number of reserved places.

Picture an egg carton: `len` counts the eggs, `cap` counts the cups. When the carton is
full, Go takes a bigger one.

> **Tip:** if you know in advance how many elements you will add, reserve the room
> with `make([]int, 0, n)`. Go won't have to copy along the way.

## Cutting a slice

`s[start:end]` takes the elements from index `start` **included** to index `end`
**excluded**.

```go
package main

import "fmt"

func main() {
	letters := []string{"a", "b", "c", "d", "e"}
	fmt.Println(letters[1:3])
	fmt.Println(letters[:2])
	fmt.Println(letters[3:])
	fmt.Println(letters[:])
}
```

```sortie
[b c]
[a b]
[d e]
[a b c d e]
```

- `[:2]`: from the start up to 2 (excluded);
- `[3:]`: from 3 to the end;
- `[:]`: everything.

> **Remember:** `s[1:3]` contains `3 - 1 = 2` elements. The end is always excluded.

## The shared-array trap

Cutting copies **nothing**: it looks at the same hidden array, through another window.
Changing one changes the other.

```go
package main

import "fmt"

func main() {
	numbers := []int{1, 2, 3, 4}
	start := numbers[:2]
	start[0] = 99
	fmt.Println(numbers)
	fmt.Println(start)
}
```

```sortie
[99 2 3 4]
[99 2]
```

`start` and `numbers` share the same slots: changing `start[0]` changed `numbers[0]`.

The same thing happens when you **assign** a slice to another variable, or pass it to
a function: both names refer to the same slots.

```go
package main

import "fmt"

func resetToZero(s []int) {
	for i := range s {
		s[i] = 0
	}
}

func main() {
	a := []int{5, 6, 7}
	b := a
	b[0] = 100
	fmt.Println(a)

	resetToZero(a)
	fmt.Println(a)
}
```

```sortie
[100 6 7]
[0 0 0]
```

### Making a real copy with copy

To get an independent slice, create a new one and copy the elements into it with
`copy(destination, source)`.

```go
package main

import "fmt"

func main() {
	original := []int{1, 2, 3}
	duplicate := make([]int, len(original))
	copy(duplicate, original)
	duplicate[0] = 99
	fmt.Println(original)
	fmt.Println(duplicate)
}
```

```sortie
[1 2 3]
[99 2 3]
```

> **Tip:** since Go 1.21, `slices.Clone(original)` does the same thing in one line.

## Two-dimensional slices

A slice can contain slices: it's a grid, like a spreadsheet or a game board.

```go
package main

import "fmt"

func main() {
	grid := [][]int{
		{1, 2, 3},
		{4, 5, 6},
	}
	fmt.Println(grid[1][2])

	for i, row := range grid {
		fmt.Println("row", i, ":", row)
	}

	board := make([][]string, 2)
	for i := range board {
		board[i] = make([]string, 3)
	}
	board[0][1] = "X"
	fmt.Printf("%q\n", board)
}
```

```sortie
6
row 0 : [1 2 3]
row 1 : [4 5 6]
[["" "X" ""] ["" "" ""]]
```

`grid[1][2]` reads: row 1, column 2. With `make`, you have to create **each** row, one
by one. The `%q` verb prints text between quotes: this way you can see the empty slots
`""`.

## Sorting and searching

### The slices package

Since Go 1.21, the `slices` package gathers the common operations.

```go
package main

import (
	"fmt"
	"slices"
)

func main() {
	grades := []int{14, 9, 17, 12}
	fmt.Println(slices.Contains(grades, 9))
	fmt.Println(slices.Index(grades, 17))
	fmt.Println(slices.Max(grades), slices.Min(grades))

	slices.Sort(grades)
	fmt.Println(grades)

	names := []string{"Zoe", "Alice", "Mark"}
	slices.Sort(names)
	fmt.Println(names)
}
```

```sortie
true
2
17 9
[9 12 14 17]
[Alice Mark Zoe]
```

| Function | Role |
|----------|------|
| `slices.Contains(s, v)` | does `s` contain `v`? |
| `slices.Index(s, v)` | the position of `v` (`-1` if missing) |
| `slices.Max(s)`, `slices.Min(s)` | the biggest, the smallest |
| `slices.Sort(s)` | sorts `s` in place, in ascending order |
| `slices.Reverse(s)` | reverses the order of `s` |

> **Note:** `slices.Sort` changes the slice itself, and returns nothing.

### Sorting by a rule: sort.Slice

To sort by your own rule, `sort.Slice` takes a function that answers the question:
"should element `i` come before element `j`?".

```go
package main

import (
	"fmt"
	"sort"
)

func main() {
	words := []string{"grapefruit", "kiwi", "pineapple", "fig"}
	sort.Slice(words, func(i, j int) bool {
		return len(words[i]) < len(words[j])
	})
	fmt.Println(words)

	grades := []int{12, 18, 9}
	sort.Slice(grades, func(i, j int) bool {
		return grades[i] > grades[j]
	})
	fmt.Println(grades)
}
```

```sortie
[fig kiwi pineapple grapefruit]
[18 12 9]
```

The first sort orders the words from shortest to longest, the second orders the grades
from highest to lowest.

> **Tip:** `sort.SliceStable` does the same sort, but keeps the starting order of
> equal elements. Use it if two words of the same length must stay in their original
> order.

## Common mistakes

### Reading past the end

```go norun
package main

import "fmt"

func main() {
	grades := []int{12, 15, 9}
	fmt.Println(grades[3])
}
```

```text
panic: runtime error: index out of range [3] with length 3

goroutine 1 [running]:
main.main()
	/tmp/tmp.XlL8TvM6jN/main.go:7 +0x15
exit status 2
```

With 3 elements, the indexes go from 0 to 2. Index 3 is *out of range*. The last
element is `grades[len(grades)-1]`.

### Forgetting to keep the result of append

```go norun
package main

import "fmt"

func main() {
	grades := []int{12, 15}
	append(grades, 9)
	fmt.Println(grades)
}
```

```text
# command-line-arguments
./main.go:7:2: append(grades, 9) (value of type []int) is not used
```

`append` doesn't change `grades`: it returns a new slice. Write
`grades = append(grades, 9)`.

### Comparing two slices with ==

```go norun
package main

import "fmt"

func main() {
	a := []int{1, 2}
	b := []int{1, 2}
	fmt.Println(a == b)
}
```

```text
# command-line-arguments
./main.go:8:14: invalid operation: a == b (slice can only be compared to nil)
```

A slice can only be compared to `nil`. To compare the contents, use
`slices.Equal(a, b)`.

### Too many values for an array

```go norun
package main

import "fmt"

func main() {
	days := [3]string{"Mon", "Tue", "Wed", "Thu"}
	fmt.Println(days)
}
```

```text
# command-line-arguments
./main.go:6:41: index 3 is out of bounds (>= 3)
```

The array has 3 slots, not 4. Make it bigger (`[4]string`), or use a slice
(`[]string{...}`), which adapts by itself.

## Test yourself

**Question 1:** what does this program print?

```go norun
package main

import "fmt"

func main() {
	s := []int{10, 20, 30, 40, 50}
	fmt.Println(s[1:4], len(s[2:]))
}
```

> **Answer**
>
> `[20 30 40] 3`. `s[1:4]` goes from index 1 to 3 included; `s[2:]` contains 30, 40
> and 50.

**Question 2:** starting from `[]int{5, 3, 8, 1}`, create a sorted copy without
changing the original, then print both.

> **Answer**
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
> 	original := []int{5, 3, 8, 1}
> 	sorted := slices.Clone(original)
> 	slices.Sort(sorted)
> 	fmt.Println(original, sorted)
> }
> ```
>
> ```sortie
> [5 3 8 1] [1 3 5 8]
> ```

**Question 3:** after `b := a[:2]` then `b[0] = 0`, has `a` changed?

> **Answer**
>
> Yes. Cutting doesn't copy the elements: `a` and `b` share the same hidden array. For
> an independent copy, use `copy` or `slices.Clone`.

## Summary

- An array `[3]int` has a fixed size; a slice `[]int` can grow.
- `append` adds at the end and returns the slice: write `s = append(s, v)`.
- `s[start:end]` cuts, end excluded, without copying anything: both slices share their
  slots.
- `copy` or `slices.Clone` make a real copy.
- The `slices` package sorts, searches and compares; `sort.Slice` sorts by your rule.

Next: [maps](maps.md). Compare with
[collections in Rust](../rust/collections.md).
