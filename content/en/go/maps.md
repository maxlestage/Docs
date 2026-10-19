# Maps

This page introduces the map, the Go structure that links keys to values, the way a dictionary links words to their definitions.

> **Goals:**
>
> - you will know how to create a map with `make` or with starting values;
> - you will know how to read, add, change and delete an entry;
> - you will know how to test whether a key exists with the "comma ok" form;
> - you will know how to go through a map in a fixed order, by sorting the keys;
> - you will know how to store slices or structs in a map.

## What is a map?

A slice stores values by **position**: 0, 1, 2… A map stores them by **key**: a name
you choose. It is a [dictionary](../debuter/glossaire.md#dictionary): you look up a
word (the key) to find its definition (the value).

The type of a map is written `map[KeyType]ValueType`. For example, `map[string]int`
links texts to integers.

```go
package main

import "fmt"

func main() {
	ages := map[string]int{
		"Alice": 12,
		"Bob":   9,
	}
	fmt.Println(ages["Alice"])
	fmt.Println(len(ages))
}
```

```sortie
12
2
```

- Each entry is written `key: value`, followed by a comma (even the last one).
- `ages["Alice"]` reads the value linked to the key `"Alice"`.
- `len` gives the number of entries.

## Creating an empty map with make

`make(map[string]int)` creates an empty map, ready to receive entries.

```go
package main

import "fmt"

func main() {
	stock := make(map[string]int)
	stock["apples"] = 5
	stock["pears"] = 3
	fmt.Println(stock["apples"], len(stock))
}
```

```sortie
5 2
```

> **Warning:** a map declared with `var stock map[string]int` is `nil`. You can read
> it, but writing to it crashes the program. Always create it with `make` or with
> `{}`.

## Adding, changing, deleting

The same syntax `m[key] = value` adds an entry if the key is new, and **replaces** it
if it already exists. `delete(m, key)` deletes an entry.

```go
package main

import "fmt"

func main() {
	prices := map[string]float64{"bread": 1.2}
	prices["milk"] = 0.95
	prices["bread"] = 1.3
	fmt.Println(prices["bread"], len(prices))

	delete(prices, "milk")
	fmt.Println(len(prices))
	delete(prices, "chocolate")
	fmt.Println(len(prices))
}
```

```sortie
1.3 2
1
1
```

Deleting a missing key (`"chocolate"`) is not an error: nothing happens.

## A missing key

Reading a key that doesn't exist causes no error: Go returns the **zero value** of the
value type.

```go
package main

import "fmt"

func main() {
	grades := map[string]int{"Alice": 15, "Bob": 0}
	fmt.Println(grades["Chloe"])
	fmt.Println(grades["Bob"])
}
```

```sortie
0
0
```

Problem: `0` for Chloe (missing) and `0` for Bob (who really got 0) look the same. How
can you tell them apart?

### The "comma ok" form

Read the map with **two** variables. The second one, a boolean, tells whether the key
exists.

```go
package main

import "fmt"

func main() {
	grades := map[string]int{"Alice": 15, "Bob": 0}

	grade, ok := grades["Bob"]
	fmt.Println(grade, ok)

	grade, ok = grades["Chloe"]
	fmt.Println(grade, ok)

	if g, found := grades["Alice"]; found {
		fmt.Println("Alice has", g)
	}
}
```

```sortie
0 true
0 false
Alice has 15
```

By habit, this variable is called `ok`. The last form, with an `if` that creates the
variable, is very common.

> **Remember:** `v := m[k]` gives the value (or zero). `v, ok := m[k]` also tells you
> whether the key exists.

## Going through a map

`range` goes through all the entries of a map, giving the key and the value.

```go norun
for name, age := range ages {
	fmt.Println(name, age)
}
```

But be careful: **the order is random**. Go changes it on purpose from one run to the
next, so that nobody relies on it. The same program can print Alice then Bob, then Bob
then Alice on the next run.

### Sorting the keys before printing

For a stable display, collect the keys, sort them, then go through them in order.

```go
package main

import (
	"fmt"
	"slices"
)

func main() {
	ages := map[string]int{"Zoé": 8, "Alice": 12, "Marc": 10}

	keys := make([]string, 0, len(ages))
	for name := range ages {
		keys = append(keys, name)
	}
	slices.Sort(keys)

	for _, name := range keys {
		fmt.Println(name, ages[name])
	}
}
```

```sortie
Alice 12
Marc 10
Zoé 8
```

### Shorter: maps.Keys and slices.Sorted

Since Go 1.23, the `maps` package provides `maps.Keys`, which gives the keys one by
one. `slices.Sorted` gathers them into a sorted slice. The three steps fit on one
line.

```go
package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	ages := map[string]int{"Zoé": 8, "Alice": 12, "Marc": 10}
	for _, name := range slices.Sorted(maps.Keys(ages)) {
		fmt.Println(name, ages[name])
	}
}
```

```sortie
Alice 12
Marc 10
Zoé 8
```

> **Tip:** make it a habit: as soon as you **print** a map, sort its keys. Your
> results and your tests will always be the same.

## Counting words

Counting is the most classic use of a map. Since a missing key is worth 0,
`count[word]++` works even the first time.

```go
package main

import (
	"fmt"
	"maps"
	"slices"
	"strings"
)

func main() {
	text := "the cat and the dog and the fish"
	count := make(map[string]int)
	for _, word := range strings.Fields(text) {
		count[word]++
	}

	for _, word := range slices.Sorted(maps.Keys(count)) {
		fmt.Printf("%-8s %d\n", word, count[word])
	}
}
```

```sortie
and      2
cat      1
dog      1
fish     1
the      3
```

## Richer values

A map value can be of any type: a slice, another map, a struct…

### A map of slices

To store several values under the same key, use a slice as the value. `append` on a
missing key starts from the empty slice (`nil`): no need to create it.

```go
package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	students := []string{"Alice", "Bob", "Chloe", "David", "Emma"}
	classes := []string{"6A", "6B", "6A", "6B", "6A"}

	byClass := make(map[string][]string)
	for i, name := range students {
		byClass[classes[i]] = append(byClass[classes[i]], name)
	}

	for _, class := range slices.Sorted(maps.Keys(byClass)) {
		fmt.Println(class, byClass[class])
	}
}
```

```sortie
6A [Alice Chloe Emma]
6B [Bob David]
```

### A map of structs

A struct groups several fields (see [types](types.md)). It makes a good map value,
like a card in a card file.

```go
package main

import "fmt"

type Product struct {
	Price float64
	Stock int
}

func main() {
	catalog := map[string]Product{
		"pen":      {Price: 1.5, Stock: 40},
		"notebook": {Price: 3.2, Stock: 12},
	}
	fmt.Printf("%+v\n", catalog["notebook"])

	p := catalog["pen"]
	p.Stock -= 5
	catalog["pen"] = p
	fmt.Println(catalog["pen"].Stock)
}
```

```sortie
{Price:3.2 Stock:12}
35
```

> **Warning:** `catalog["pen"].Stock -= 5` is refused by the compiler. You have to
> take the struct out, change it, then put it back in the map.

## Common mistakes

### Writing to a nil map

```go norun
package main

import "fmt"

func main() {
	var stock map[string]int
	stock["apples"] = 5
	fmt.Println(stock)
}
```

```text
panic: assignment to entry in nil map

goroutine 1 [running]:
main.main()
	/tmp/tmp.TQHY4cxvk4/main.go:7 +0x28
exit status 2
```

`var stock map[string]int` doesn't create a map, only a `nil` variable. Write
`stock := make(map[string]int)`.

### Declaring ok without using it

```go norun
package main

import "fmt"

func main() {
	ages := map[string]int{"Alice": 30}
	age, ok := ages["Bob"]
	fmt.Println(age)
}
```

```text
# command-line-arguments
./main.go:7:7: declared and not used: ok
```

If you ask for `ok`, test it: `if !ok { ... }`. Otherwise, simply write
`age := ages["Bob"]`.

### Using a slice as a key

```go norun
package main

import "fmt"

func main() {
	m := map[[]int]string{}
	fmt.Println(m)
}
```

```text
# command-line-arguments
./main.go:6:11: invalid map key type []int
```

A key must be comparable with `==`: text, number, boolean, array or simple struct. A
slice can't be. Use an array (`[2]int`) or a text.

## Test yourself

**Question 1:** what does this program print?

```go norun
package main

import "fmt"

func main() {
	m := map[string]int{"a": 1}
	m["b"] += 5
	m["a"] = 10
	fmt.Println(m["a"], m["b"], len(m))
}
```

> **Answer**
>
> `10 5 2`. `m["b"]` was 0 (missing key), then 5; `m["a"]` is replaced by 10. The map
> has two keys.

**Question 2:** count the letters of the word `"banana"` and print them in
alphabetical order.

> **Answer**
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
> 	count := make(map[rune]int)
> 	for _, c := range "banana" {
> 		count[c]++
> 	}
> 	for _, c := range slices.Sorted(maps.Keys(count)) {
> 		fmt.Printf("%c: %d\n", c, count[c])
> 	}
> }
> ```
>
> ```sortie
> a: 3
> b: 1
> n: 2
> ```

**Question 3:** why should you sort the keys before printing a map?

> **Answer**
>
> Because Go goes through a map in a random order, different on each run. Sorting the
> keys gives a display that is always the same.

## Summary

- A map links keys to values: `map[string]int`.
- Create it with `make(map[K]V)` or `map[K]V{...}`; a `nil` map accepts nothing.
- `m[k] = v` adds or replaces, `delete(m, k)` deletes, `len(m)` counts.
- A missing key gives the zero value; `v, ok := m[k]` tells whether it exists.
- The order of `range` is random: sort with `slices.Sorted(maps.Keys(m))`.

Next: [common Go pitfalls for beginners](pieges.md)
