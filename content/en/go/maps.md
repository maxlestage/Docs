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

Often, you don't know in advance what the map will hold: you fill it as the program
runs. It is like opening an empty drawer to put things in later.

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

Line by line:

- `make(map[string]int)` creates an empty map: it has no entries yet;
- `stock["apples"] = 5` adds an entry: the key `"apples"`, with the value 5;
- the next line adds the key `"pears"`, with the value 3;
- `Println` prints the value of `"apples"` (5), then the number of entries (2).

> **Warning:** a map declared with `var stock map[string]int` is `nil`. You can read
> it, but writing to it crashes the program. Always create it with `make` or with
> `{}`.

## Adding, changing, deleting

A map changes while the program runs, like a bakery's price list: a product
arrives, a price goes up, another product disappears.

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

Line by line:

- at the start, `prices` holds a single entry: `"bread"` is 1.2;
- `prices["milk"] = 0.95` uses a new key: the map now has 2 entries;
- `prices["bread"] = 1.3` creates nothing: the key already exists, its value goes from
  1.2 to 1.3;
- `delete(prices, "milk")` removes the milk: 1 entry is left.

Deleting a missing key (`"chocolate"`) is not an error: nothing happens.

**Your turn:** write a program that creates an empty map `colors` with `make`,
adds `"red"` (3) and `"blue"` (5), replaces the value of `"red"` with 4, deletes
`"blue"`, then prints the value of `"red"` and the number of entries.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	colors := make(map[string]int)
> 	colors["red"] = 3
> 	colors["blue"] = 5
> 	colors["red"] = 4
> 	delete(colors, "blue")
> 	fmt.Println(colors["red"], len(colors))
> }
> ```
>
> ```sortie
> 4 1
> ```

## A missing key

A program often asks for a key that may not be there: a name someone typed, a
product that is out of stock. You need to know what Go answers then.

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

Line by line:

- `grades` holds two entries: Alice has 15, Bob has 0;
- `grades["Chloe"]` looks for a key that doesn't exist: Go returns 0, the zero value
  of an `int`;
- `grades["Bob"]` finds the key: its value really is 0.

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

**Your turn:** write a program that creates a phone book
`map[string]string{"Alice": "555 0134"}`, looks up `"Bob"` with the "comma ok" form,
then prints his number if it exists, or "Bob is not in the phone book" otherwise.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	phoneBook := map[string]string{"Alice": "555 0134"}
> 	number, ok := phoneBook["Bob"]
> 	if ok {
> 		fmt.Println("Bob's number:", number)
> 	} else {
> 		fmt.Println("Bob is not in the phone book")
> 	}
> }
> ```
>
> ```sortie
> Bob is not in the phone book
> ```

## Going through a map

You often need to look at everything: print the whole stock, add up all the grades.
You don't want to write every key by hand: that is what `range` is for.

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

Line by line:

- `keys` is an empty slice of texts, with enough room for all the keys;
- the first loop goes over each name in the map and adds it to `keys`, in any order;
- `slices.Sort(keys)` puts the names in alphabetical order: Alice, Marc, Zoé;
- the second loop goes through `keys` in that order and reads each person's age from
  the map.

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

**Your turn:** write a program that stores a fruit seller's stock in
`map[string]int{"pears": 3, "kiwis": 7, "apples": 5}`, then prints each fruit and its
quantity, in alphabetical order.

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
> 	stock := map[string]int{"pears": 3, "kiwis": 7, "apples": 5}
> 	for _, fruit := range slices.Sorted(maps.Keys(stock)) {
> 		fmt.Println(fruit, stock[fruit])
> 	}
> }
> ```
>
> ```sortie
> apples 5
> kiwis 7
> pears 3
> ```

## Counting words

> **Note:** this part can wait. Read it once the basics feel solid.

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

Line by line:

- `strings.Fields(text)` cuts the text into words, at each space;
- for each word, `count[word]++` adds 1. The first time "the" shows up, its value goes
  from 0 to 1; at the end, it is 3;
- the second loop prints the sorted words. `%-8s` pads each word with spaces, to line
  up the numbers.

## Richer values

Sometimes one value per key is not enough: a class has several students, a product
has a price **and** a stock.

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

Line by line:

- `students` and `classes` go together: Alice is in 6A, Bob in 6B, Chloe in 6A…;
- `byClass` is an empty map: for each class, it will hold a slice of names;
- the loop takes each student and adds them (`append`) to the slice of their class;
- at the end, `"6A"` holds three names and `"6B"` holds two.

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

Next: [following a program step by step in Go](pas-a-pas.md).
