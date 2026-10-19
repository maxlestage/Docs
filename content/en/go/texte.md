# Working with text

This page explains how Go stores text (bytes and runes), and how to split it, transform it and put it together with the `strings`, `fmt` and `strconv` packages.

> **Goals:**
>
> - you will know what the bytes and runes of a string are;
> - you will know how to count characters and go through a text with `range`;
> - you will know how to use the common functions of the `strings` package;
> - you will know how to build text with `strings.Builder` and `fmt.Sprintf`;
> - you will know how to write a raw text between backquotes.

## Strings

Almost every program handles text: a first name, a message, a line read from a file.
In Go, text has its own type, just like numbers have theirs.

A piece of text is called a [string](../debuter/glossaire.md#string), of
type `string`. You write it between double quotes.

```go
package main

import "fmt"

func main() {
	firstName := "Alice"
	greeting := "Hello, " + firstName + "!"
	fmt.Println(greeting)
	fmt.Println(firstName == "Alice")
}
```

```sortie
Hello, Alice!
true
```

Line by line:

- `firstName := "Alice"` stores the text "Alice" in the variable `firstName`;
- the next line sticks three pieces together: `"Hello, "`, the content of
  `firstName`, then `"!"`. So `greeting` holds `"Hello, Alice!"`;
- `fmt.Println(greeting)` prints that text;
- `firstName == "Alice"` asks a question: "are the two texts identical?". The answer
  is yes, hence `true`.

- `+` sticks two strings end to end;
- `==` compares two strings.

A few special characters are written with a backslash `\`:

| Written | Meaning |
|----------|------|
| `\n` | go to a new line |
| `\t` | a tab |
| `\"` | a double quote inside the text |
| `\\` | the backslash `\` itself |

```go
package main

import "fmt"

func main() {
	fmt.Println("Line 1\nLine 2")
	fmt.Println("He said: \"Hi\"")
}
```

```sortie
Line 1
Line 2
He said: "Hi"
```

> **Remember:** a Go string is **immutable**: you can't change one of its letters.
> All the functions that "modify" a text actually create a new one.

**Your turn:** write a program that stores `"Leeds"` in a variable `city`, then prints
"I live in Leeds." by sticking the pieces together with `+`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	city := "Leeds"
> 	fmt.Println("I live in " + city + ".")
> }
> ```
>
> ```sortie
> I live in Leeds.
> ```

## Bytes and runes

Why care? As soon as a word contains an accent, counting its letters or taking its
first letter no longer works the way you expect. Knowing how Go stores text saves you
from these surprises.

A Go string is a sequence of [bytes](../debuter/glossaire.md#byte) (numbers from 0 to
255), encoded in UTF-8. Simple letters (`a`, `Z`, `5`) take 1 byte. Accented letters
take 2, and an emoji takes 4.

A character is called a **rune** in Go. A rune is written between single quotes: `'é'`.

`len` counts **bytes**, not characters. To count characters, use
`utf8.RuneCountInString`.

```go
package main

import (
	"fmt"
	"unicode/utf8"
)

func main() {
	word := "été"
	fmt.Println(len(word))
	fmt.Println(utf8.RuneCountInString(word))
	fmt.Println(len("abc"))
}
```

```sortie
5
3
3
```

Line by line:

- `word := "été"` stores a 3-letter word, two of them accented;
- `len(word)` counts bytes: 2 for each `é` and 1 for the `t`, so 5;
- `utf8.RuneCountInString(word)` counts characters: 3;
- `len("abc")` gives 3: without accents, one character takes a single byte.

`"été"` (French for "summer") has 3 characters, but 5 bytes: each `é` takes 2.

### Reading a byte or a rune

`word[i]` gives byte number `i`. To get the characters, convert the string into a
slice of runes with `[]rune(word)`.

```go
package main

import "fmt"

func main() {
	word := "café"
	fmt.Println(word[0])
	fmt.Printf("%c\n", word[0])

	runes := []rune(word)
	fmt.Println(len(runes))
	fmt.Printf("%c\n", runes[3])
}
```

```sortie
99
c
4
é
```

> **Warning:** `word[3]` doesn't give `é`, but the first of the two bytes of `é`.
> As soon as a text may contain accents, go through `[]rune` or `range`.

### Going through a text with range

`range` on a string moves forward **character by character**. It gives the position
(in bytes) and the rune.

```go
package main

import "fmt"

func main() {
	for i, c := range "Noël" {
		fmt.Printf("%d %c\n", i, c)
	}
}
```

```sortie
0 N
1 o
2 ë
4 l
```

The position jumps from 2 to 4: `ë` takes bytes 2 and 3.

**Your turn:** write a program that goes through the word `"café"` with `range` and
prints each letter on its own line.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for _, c := range "café" {
> 		fmt.Printf("%c\n", c)
> 	}
> }
> ```
>
> ```sortie
> c
> a
> f
> é
> ```

## The strings package

Cutting a sentence into words, putting a text in capitals, checking that an address
contains an `@`: these needs come up all the time. Instead of rewriting everything, you
take the ready-made tool from this toolbox.

The `strings` package holds the basic tools for text.

### Searching

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	sentence := "The cat sleeps on the sofa"
	fmt.Println(strings.Contains(sentence, "cat"))
	fmt.Println(strings.HasPrefix(sentence, "The"))
	fmt.Println(strings.HasSuffix(sentence, "bed"))
	fmt.Println(strings.Index(sentence, "sleeps"))
	fmt.Println(strings.Count(sentence, "a"))
}
```

```sortie
true
true
false
8
2
```

Line by line:

- `sentence` holds the text you search in;
- each following line asks a question about `sentence` and prints the answer;
- the sentence doesn't end with "bed", hence `false`;
- "sleeps" starts at position 8, because counting starts at 0: `T` is at 0, `h` at 1,
  `e` at 2, and so on;
- the letter `a` appears 2 times: once in "cat", once in "sofa".

- `Contains`: does the text contain this piece?
- `HasPrefix`, `HasSuffix`: does it start, or end, with this piece?
- `Index`: the position of the piece (`-1` if it isn't there);
- `Count`: how many times the piece appears.

### Transforming

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Println(strings.ToUpper("hello"))
	fmt.Println(strings.ToLower("CAFÉ"))
	fmt.Println(strings.Replace("a-b-c", "-", "+", 1))
	fmt.Println(strings.ReplaceAll("a-b-c", "-", "+"))
	fmt.Println(strings.Repeat("ha", 3))
	fmt.Printf("[%s]\n", strings.TrimSpace("   hello  \n"))
}
```

```sortie
HELLO
café
a+b-c
a+b+c
hahaha
[hello]
```

- `Replace(text, old, new, n)` replaces the first `n` occurrences; `ReplaceAll`
  replaces all of them;
- `TrimSpace` removes the spaces and line breaks at the start and at the end. Very
  useful to clean up what a user typed.

### Splitting and joining

`strings.Split` cuts a text at each separator and returns a slice of pieces.
`strings.Join` does the opposite.

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	line := "apple;kiwi;pear"
	fruits := strings.Split(line, ";")
	fmt.Println(len(fruits), fruits[1])

	fmt.Println(strings.Join(fruits, ", "))

	words := strings.Fields("  one   two\tthree  ")
	fmt.Println(len(words), words)
}
```

```sortie
3 kiwi
apple, kiwi, pear
3 [one two three]
```

`strings.Fields` cuts at spaces, however many there are. It's the best choice to
separate the words of a sentence.

**Your turn:** write a program that cuts the text `"monday,tuesday,wednesday"` at each
comma, prints the number of days, then the days in capitals, separated by ` - `.

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
> 	days := strings.Split("monday,tuesday,wednesday", ",")
> 	fmt.Println(len(days))
> 	fmt.Println(strings.ToUpper(strings.Join(days, " - ")))
> }
> ```
>
> ```sortie
> 3
> MONDAY - TUESDAY - WEDNESDAY
> ```

## Building text

A program often has to make a text out of several pieces: a name, a grade, a date. It's
like filling in a form: the template stays the same, only the boxes change.

### fmt.Sprintf

`fmt.Sprintf` works like `Printf`, but instead of printing, it **returns** the text.

```go
package main

import "fmt"

func main() {
	name := "Alice"
	average := 14.256
	line := fmt.Sprintf("%-8s|%6.1f", name, average)
	fmt.Println(line)
	fmt.Println(len(line))
}
```

```sortie
Alice   |  14.3
15
```

Line by line:

- `name` holds `"Alice"` and `average` holds `14.256`;
- `%-8s` puts the name in a box of 8 characters, aligned to the left: `Alice` is
  followed by 3 spaces. The `|` is copied as is;
- `%6.1f` writes the number with a single digit after the decimal point (`14.3`), in a
  box of 6 characters, aligned to the right;
- nothing is printed at this point: the text is stored in `line`, then printed by
  `Println`. It is 8 + 1 + 6 = 15 characters long.

### strings.Builder

Sticking texts together with `+` in a loop creates many intermediate strings. To
build a long text piece by piece, use a `strings.Builder`: it's like a notebook
where you add lines.

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	var sb strings.Builder
	for i := 1; i <= 3; i++ {
		sb.WriteString("Line ")
		fmt.Fprintf(&sb, "%d\n", i)
	}
	sb.WriteString("End")
	fmt.Println(sb.String())
}
```

```sortie
Line 1
Line 2
Line 3
End
```

- `sb.WriteString(...)` adds a text;
- `fmt.Fprintf(&sb, ...)` adds a formatted text;
- `sb.String()` returns the whole text.

The `&` in front of `sb` is explained in [pointers](pointeurs.md).

**Your turn:** write a program that stores `"Alice"` in `name` and `12` in `age`, builds
the text "Alice is 12 years old." with `fmt.Sprintf`, stores it in a variable
`sentence`, then prints it.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	name := "Alice"
> 	age := 12
> 	sentence := fmt.Sprintf("%s is %d years old.", name, age)
> 	fmt.Println(sentence)
> }
> ```
>
> ```sortie
> Alice is 12 years old.
> ```

## Text and numbers: strconv

> **Note:** this part can wait. Read it once the basics feel solid.

To stick a number to a text, you first have to convert it. The `strconv` package
does that (see also [numbers and calculations](nombres.md)).

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	age := 12
	fmt.Println("I am " + strconv.Itoa(age) + " years old")

	n, err := strconv.Atoi("2026")
	fmt.Println(n+1, err)

	ok, _ := strconv.ParseBool("true")
	fmt.Println(ok)

	fmt.Println(strconv.Quote("said \"yes\""))
}
```

```sortie
I am 12 years old
2027 <nil>
true
"said \"yes\""
```

Line by line:

- `strconv.Itoa(age)` turns the number 12 into the text `"12"`, which `+` can stick;
- `strconv.Atoi("2026")` does the opposite: it reads the text and gives the number
  2026. It also returns `err`, which is `nil` ("nothing") when all went well;
- `strconv.ParseBool("true")` reads the text `"true"` and gives the boolean `true`. The
  `_` ignores the error;
- `strconv.Quote` wraps the text in quotes and adds the `\` that are needed.

> **Warning:** `string(65)` doesn't give `"65"`, but `"A"` (the letter with code 65).
> To turn a number into text, always use `strconv.Itoa` or `fmt.Sprint`.

## Raw strings

In a normal string, each `\` must be doubled: a Windows path quickly becomes hard to
read, like `"C:\\Users\\alice"`. Raw strings avoid this problem.

Between backquotes `` ` ``, a string is **raw**: the `\` are not interpreted, and the
text can span several lines.

```go
package main

import "fmt"

func main() {
	path := `C:\Users\alice\notes.txt`
	fmt.Println(path)

	poem := `The sky is blue,
	and so is the sea.`
	fmt.Println(poem)
}
```

```sortie
C:\Users\alice\notes.txt
The sky is blue,
	and so is the sea.
```

Line by line:

- `path` holds `\` characters copied as is: the `\n` in `\notes` doesn't start a new
  line;
- `poem` spans two lines: the line break is part of the text.

Everything is kept as is, even the tab at the start of the second line. It's handy
for a Windows path, a text template, JSON or a
[regular expression](regex.md).

## Common mistakes

### Changing a letter of a string

```go norun
package main

import "fmt"

func main() {
	word := "hello"
	word[0] = 'H'
	fmt.Println(word)
}
```

```text
# command-line-arguments
./main.go:7:2: cannot assign to word[0] (neither addressable nor a map index expression)
```

A string is immutable. Create a new one: `word = "H" + word[1:]`, or go through
`[]rune(word)` then `string(runes)`.

### Using single quotes for a text

```go norun
package main

import "fmt"

func main() {
	fmt.Println('Hello')
}
```

```text
# command-line-arguments
./main.go:6:14: more than one character in rune literal
```

Single quotes `'...'` are reserved for **a single** character (a rune). For a text,
use double quotes: `"Hello"`.

### Sticking a number to a text with +

```go norun
package main

import "fmt"

func main() {
	age := 12
	fmt.Println("I am " + age + " years old")
}
```

```text
# command-line-arguments
./main.go:7:14: invalid operation: "I am " + age (mismatched types untyped string and int)
```

You can't add a text and a number. Convert with `strconv.Itoa(age)`, or write
`fmt.Println("I am", age, "years old")`.

## Test yourself

**Question 1:** what are `len("añ")` and `utf8.RuneCountInString("añ")`?

> **Answer**
>
> `3` and `2`. `a` takes 1 byte, `ñ` takes 2: 3 bytes, but 2 characters.

**Question 2:** starting from `"  alice,BOB , Chloé "`, print the first names in
uppercase, separated by ` / `.

> **Answer**
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
> 	pieces := strings.Split("  alice,BOB , Chloé ", ",")
> 	for i, p := range pieces {
> 		pieces[i] = strings.ToUpper(strings.TrimSpace(p))
> 	}
> 	fmt.Println(strings.Join(pieces, " / "))
> }
> ```
>
> ```sortie
> ALICE / BOB / CHLOÉ
> ```

**Question 3:** why prefer `strings.Builder` to `+` in a big loop?

> **Answer**
>
> Each `+` creates a whole new string, which becomes slow as the text grows.
> `strings.Builder` adds the pieces in the same place, without copying everything.

## Summary

- A string is a sequence of bytes in UTF-8, and it is immutable.
- A character is a `rune` (`'é'`); `len` counts bytes, `utf8.RuneCountInString`
  counts characters.
- `range` on a string gives the runes one by one.
- `strings` searches (`Contains`, `HasPrefix`), transforms (`ToUpper`, `Replace`,
  `TrimSpace`) and splits (`Split`, `Fields`, `Join`).
- `fmt.Sprintf` and `strings.Builder` build text; `strconv` converts numbers.
- Backquotes `` `...` `` make a raw string, over several lines.

Next: [arrays and slices](collections.md). Compare with
[text in Ruby](../ruby/texte.md).
