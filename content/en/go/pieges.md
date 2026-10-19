# Beginner traps in Go

This page gathers eleven traps that almost every Go beginner falls into, with, for each one, what happens, why, and how to avoid it.

> **Goals:**
>
> - recognize Go's most common error messages;
> - understand why a program that compiles sometimes gives a wrong result;
> - know how to fix each trap in one or two lines.

## How to read this page

Each trap follows the same path:

```text
 What you think  →  What happens  →  Why  →  The fix
```

There are two kinds of traps:

- the ones the [compiler](../debuter/glossaire.md#compiler) refuses: an error
  message appears, the program doesn't start;
- the ones that compile, but give a **wrong result**. These are the sneakiest.

No need to read everything at once. Come back here when a program surprises you.

## Compiler traps

### 1. A variable or an import that is never used

**You think:** "I'm preparing a variable, I'll use it later."

**What happens:** Go refuses to compile.

```go norun
package main

import (
	"fmt"
	"strings"
)

func main() {
	name := "Alice"
	age := 12
	fmt.Println(name)
}
```

```text
# command-line-arguments
./main.go:5:2: "strings" imported and not used
./main.go:10:2: declared and not used: age
```

**Why:** in Go, everything you create must be used. A forgotten
[variable](../debuter/glossaire.md#variable) often hides a typo. Go prefers to warn you
right away.

**The fix:** delete what isn't used. Or use it.

```go
package main

import "fmt"

func main() {
	name := "Alice"
	age := 12
	fmt.Println(name, age)
}
```

```sortie
Alice 12
```

> **Tip:** to keep a value without using it, give it to `_` (the "trash drawer"):
> `_ = age`. It's handy while you try things out, but remove it afterwards.

### 2. Mixing up := and =

**You think:** "`:=` and `=` do the same thing."

**What happens:** not at all.

- `:=` **creates** a new variable. It's like sticking a label on a brand-new box.
- `=` **changes** the contents of a box that already exists.

First case: `:=` twice for the same name.

```go norun
package main

import "fmt"

func main() {
	total := 0
	total := 10
	fmt.Println(total)
}
```

```text
# command-line-arguments
./main.go:7:8: no new variables on left side of :=
```

The message means: "on the left of `:=`, there is nothing new to create".

Second case: `=` for a variable that doesn't exist yet.

```go norun
package main

import "fmt"

func main() {
	score = 5
	fmt.Println(score)
}
```

```text
# command-line-arguments
./main.go:6:2: undefined: score
./main.go:7:14: undefined: score
```

`undefined` means "I don't know this name".

**The fix:** `:=` the first time, `=` the following times.

```go
package main

import "fmt"

func main() {
	score := 5
	score = score + 1
	fmt.Println(score)
}
```

```sortie
6
```

> **Remember:** `:=` = new box. `=` = change what's inside the box.

### 3. Changing one letter of a text

**You think:** "A text is a sequence of letters, I can change one of them."

```go norun
package main

import "fmt"

func main() {
	word := "café"
	word[0] = 'C'
	fmt.Println(word)
}
```

```text
# command-line-arguments
./main.go:7:2: cannot assign to word[0] (neither addressable nor a map index expression)
```

**Why:** in Go, a text (`string`) can never be modified. We say it is
[immutable](../debuter/glossaire.md#immutable).

**The fix:** build a **new** text.

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	word := "café"
	word = "C" + word[1:]
	fmt.Println(word)
	fmt.Println(strings.ToUpper("café"))
}
```

```sortie
Café
CAFÉ
```

## Calculation traps

### 4. Integer division

**You think:** "7 divided by 2 is 3.5."

```go
package main

import "fmt"

func main() {
	fmt.Println(7 / 2)
	grade1, grade2 := 12, 15
	average := (grade1 + grade2) / 2
	fmt.Println(average)
}
```

```sortie
3
13
```

**Why:** between two [integers](../debuter/glossaire.md#integer), `/` gives an integer.
Go **cuts off** the part after the decimal point (it doesn't round). 27 / 2 gives 13,
not 13.5.

**The fix:** convert to `float64` **before** dividing.

```go
package main

import "fmt"

func main() {
	grade1, grade2 := 12, 15
	average := float64(grade1+grade2) / 2
	fmt.Println(average)
}
```

```sortie
13.5
```

Try it: write `float64((grade1 + grade2) / 2)`. You get `13`. The division happened
**before** the conversion: too late, the 0.5 part is already lost.

See [numbers](nombres.md#integer-division-and-the-remainder).

### 5. Ignoring the error

**You think:** "Converting the text to a number will go fine."

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, _ := strconv.Atoi("12a")
	fmt.Println(n * 2)
}
```

```sortie
0
```

**Why:** `"12a"` is not a number. `strconv.Atoi` then returns `0` **and** an
[error](../debuter/glossaire.md#error). With `_`, you threw the error in the trash. The
program goes on with a wrong `0`, without saying anything.

**The fix:** always test `err`.

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("12a")
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	fmt.Println(n * 2)
}
```

```sortie
Error: strconv.Atoi: parsing "12a": invalid syntax
```

> **Remember:** when a function returns `err`, the next line is almost always
> `if err != nil { ... }`.

## Loop and block traps

### 6. A variable that hides another one

**You think:** "I'm changing `message` inside the `if`."

```go
package main

import "fmt"

func main() {
	age := 20
	message := "minor"
	if age >= 18 {
		message := "adult"
		fmt.Println("inside the if:", message)
	}
	fmt.Println("after the if:", message)
}
```

```sortie
inside the if: adult
after the if: minor
```

**Why:** inside the `if`, `message :=` creates a **second** variable, with the same
name. It hides the first one, then disappears at the end of the block `}`. This is
called **shadowing**. The area where a name exists is called its
[scope](../debuter/glossaire.md#scope).

```text
main  ┌─────────────────────────────┐
      │ message = "minor"           │
      │   if  ┌───────────────────┐ │
      │       │ message = "adult" │ │  ← another box
      │       └───────────────────┘ │  ← it disappears here
      │ message is still "minor"
      └─────────────────────────────┘
```

**The fix:** a single character. Replace `:=` with `=` inside the `if`.

```go
package main

import "fmt"

func main() {
	age := 20
	message := "minor"
	if age >= 18 {
		message = "adult"
	}
	fmt.Println(message)
}
```

```sortie
adult
```

### 7. range gives a copy

**You think:** "I'm multiplying each number of the slice by 10."

```go
package main

import "fmt"

func main() {
	numbers := []int{1, 2, 3}
	for _, n := range numbers {
		n = n * 10
	}
	fmt.Println(numbers)
}
```

```sortie
[1 2 3]
```

**Why:** `n` is a **copy** of each slot, like a photocopy. You write on the photocopy,
not on the original.

| Round | n before | n after | numbers |
|---|---|---|---|
| 1 | 1 | 10 | [1 2 3] |
| 2 | 2 | 20 | [1 2 3] |
| 3 | 3 | 30 | [1 2 3] |

**The fix:** go through the position `i` to write into the real slot.

```go
package main

import "fmt"

func main() {
	numbers := []int{1, 2, 3}
	for i := range numbers {
		numbers[i] = numbers[i] * 10
	}
	fmt.Println(numbers)
}
```

```sortie
[10 20 30]
```

## Slice, map and text traps

### 8. Two slices that share the same slots

**You think:** "`start` is a new slice. Adding an element to it doesn't touch `base`."

```go
package main

import "fmt"

func main() {
	base := []int{1, 2, 3, 4}
	start := base[:2]
	start = append(start, 99)
	fmt.Println("start:", start)
	fmt.Println("base: ", base)
}
```

```sortie
start: [1 2 99]
base:  [1 2 99 4]
```

**Why:** a [slice](../debuter/glossaire.md#slice) is a **window** onto a hidden array.
`base[:2]` copies nothing: it's a small window onto the same slots. Since there is room
left behind it, `append` writes 99 into slot 2… which also belongs to `base`.

```text
hidden array : [ 1 | 2 | 3 | 4 ]
base         :  └───────────────┘
start        :  └─────┘
after append : [ 1 | 2 | 99| 4 ]   ← slot 2 is overwritten
```

**The fix:** make a real copy with `slices.Clone`.

```go
package main

import (
	"fmt"
	"slices"
)

func main() {
	base := []int{1, 2, 3, 4}
	start := slices.Clone(base[:2])
	start = append(start, 99)
	fmt.Println("start:", start)
	fmt.Println("base: ", base)
}
```

```sortie
start: [1 2 99]
base:  [1 2 3 4]
```

More details in [the shared-array trap](collections.md#the-shared-array-trap).

### 9. Writing into a nil map

**You think:** "I declared my map, I can fill it."

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
	/path/to/main.go:7 +0x28
exit status 2
```

**Why:** `var stock map[string]int` creates a label, but no box. The map is `nil`
("nothing"). Writing into it causes a [panic](../debuter/glossaire.md#panic): the
program stops dead.

**The fix:** create the map with `make` (or with `{}`).

```go
package main

import "fmt"

func main() {
	stock := make(map[string]int)
	stock["apples"] = 5
	fmt.Println(stock)
}
```

```sortie
map[apples:5]
```

### 10. The order of a map changes every time

**You think:** "The map will be shown in the order I put the keys in."

```go norun
package main

import "fmt"

func main() {
	ages := map[string]int{"Zoé": 8, "Alice": 12, "Marc": 10}
	for name, age := range ages {
		fmt.Println(name, age)
	}
}
```

Run this program several times. The order changes:

```text
1st run           2nd run
Marc 10           Alice 12
Zoé 8             Zoé 8
Alice 12          Marc 10
```

**Why:** Go shuffles the order of `range` over a map **on purpose**. That way, nobody
relies on it by mistake.

**The fix:** sort the keys, then go through them.

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

> **Note:** `fmt.Println(ages)` already shows the keys sorted. The trap only concerns
> `range`.

### 11. A text is measured in bytes, not in letters

**You think:** "`"élan"` has 4 letters, so `len` is 4 and `word[0]` is `é`."

```go
package main

import "fmt"

func main() {
	word := "élan"
	fmt.Println(len(word))
	fmt.Println(word[0])
	fmt.Println(string(word[0]))
}
```

```sortie
5
195
Ã
```

**Why:** Go stores text as [bytes](../debuter/glossaire.md#byte) (in UTF-8, see
[Unicode](../debuter/glossaire.md#unicode)). A plain letter like `l` takes 1 byte. An
accented letter like `é` takes **2**. `word[0]` gives the first **byte**, not the first
letter.

```text
letters :  é       l    a    n
bytes   : [195 169][108][97][110]   → 5 bytes
```

**The fix:** go through **runes** (one rune = one letter).

```go
package main

import "fmt"

func main() {
	word := "élan"
	letters := []rune(word)
	fmt.Println(len(letters))
	fmt.Println(string(letters[0]))
	for _, r := range word {
		fmt.Print(string(r), " ")
	}
	fmt.Println()
}
```

```sortie
4
é
é l a n 
```

`range` over a text does give the letters one by one. See
[bytes and runes](texte.md#bytes-and-runes).

> **Remember:** `len(text)` counts bytes. `len([]rune(text))` counts letters.

## Common mistakes

The whole page is about mistakes. Here are two more messages you will see often.

### Reading a slot that doesn't exist

```go norun
package main

import "fmt"

func main() {
	grades := []int{12, 8, 15}
	fmt.Println(grades[3])
}
```

```text
panic: runtime error: index out of range [3] with length 3

goroutine 1 [running]:
main.main()
	/path/to/main.go:7 +0x15
exit status 2
```

Three slots, numbered 0, 1 and 2. Slot 3 doesn't exist. The last slot is always
`grades[len(grades)-1]`.

### Mixing an int and a float64

```go norun
package main

import "fmt"

func main() {
	sum := 27.0
	n := 2
	fmt.Println(sum / n)
}
```

```text
# command-line-arguments
./main.go:8:14: invalid operation: sum / n (mismatched types float64 and int)
```

Go never mixes two types of numbers on its own. Convert it yourself:
`sum / float64(n)`.

## Test yourself

**Question 1:** what does this program print?

```go norun
package main

import "fmt"

func main() {
	total := 0
	for _, n := range []int{1, 2, 3} {
		total := total + n
		fmt.Print(total, " ")
	}
	fmt.Println(total)
}
```

> **Answer**
>
> `1 2 3 0`. Inside the loop, `total :=` creates a new variable at each round (trap
> 6). It is `0 + n`. The outer `total` never changed: it is still 0.
> Fix: `total = total + n`, and the program prints `1 3 6 6`.

**Question 2:** why does `fmt.Println(9 / 4)` print `2`?

> **Answer**
>
> 9 and 4 are integers: the division is an integer division, the part after the
> decimal point is cut off. To get `2.25`, write `9.0 / 4` or `float64(9) / 4`.

**Question 3:** what is `len("Noël")`?

> **Answer**
>
> `5`. N, o and l take 1 byte each, `ë` takes 2. To count the letters:
> `len([]rune("Noël"))`, which is 4.

## Summary

- Go refuses unused variables and imports: delete them.
- `:=` creates a variable, `=` changes it; a `:=` inside a block can hide a variable.
- Between two integers, `/` cuts off the decimals: convert to `float64` first.
- Always test `err`; `range` gives a copy; a map is created with `make`.
- Two slices can share their slots; the order of a map is random.
- A text is measured in bytes: use `[]rune` to count the letters.

Next: [exercises to get started](exercices-faciles.md).
