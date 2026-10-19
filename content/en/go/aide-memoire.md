# Cheat sheet

This page sums up at a glance the Go syntax seen throughout the track, with compact tables and short programs to run.

## Basics

| Element | Example |
| --- | --- |
| Program | `package main` + `func main() { … }` |
| Import | `import "fmt"` or `import ( "fmt"; "os" )` |
| Variable (short declaration) | `age := 30` |
| Variable (explicit type) | `var price float64 = 9.5` |
| Constant | `const VATRate = 0.2` |
| Several at once | `a, b := 1, 2` ; swap: `a, b = b, a` |
| Ignore a value | `_ = value` |
| Comment | `// line` or `/* block */` |
| Conversion | `float64(n)`, `int(x)`, `string(r)` |
| Text ↔ number | `strconv.Itoa(42)`, `strconv.Atoi("42")` |

| Type | Zero value | Example |
| --- | --- | --- |
| `int`, `int64` | `0` | `42` |
| `float64` | `0` | `3.14` |
| `string` | `""` | `"Hello"` |
| `bool` | `false` | `true` |
| `rune` (one character) | `0` | `'é'` |
| pointer, slice, map, channel, interface, function | `nil` | |

```go
package main

import "fmt"

func main() {
	name := "Alice"
	var age int
	fmt.Println("Hello", name, age)
	fmt.Println(7/2, 7.0/2, 7%2)
	fmt.Println(float64(7) / 2)
}
```

```sortie
Hello Alice 0
3 3.5 1
3.5
```

Details: [first steps](premiers-pas.md), [numbers](nombres.md).

## Printing with fmt

| Function | Role |
| --- | --- |
| `fmt.Println(a, b)` | prints, separates with spaces, goes to a new line |
| `fmt.Printf(format, …)` | prints using a format (add `\n`) |
| `fmt.Sprintf(format, …)` | returns the text instead of printing it |
| `fmt.Fprintf(w, format, …)` | writes to an `io.Writer` |
| `fmt.Errorf(format, …)` | creates an error (`%w` to wrap another one) |

| Verb | Prints | Example → result |
| --- | --- | --- |
| `%v` | the value | `{Alice 30}` |
| `%+v` | with the field names | `{Name:Alice Age:30}` |
| `%#v` | like Go code | `main.P{Name:"Alice", Age:30}` |
| `%T` | the type | `main.P` |
| `%d` | an integer | `42` |
| `%5d`, `%-5d`, `%05d` | width 5, left-aligned, with zeros | `   42`, `42   `, `00042` |
| `%f`, `%.2f` | a float, 2 decimals | `3.141593`, `3.14` |
| `%g` | a float without useless zeros | `2.5` |
| `%s`, `%q` | a text, in quotes | `Go`, `"Go"` |
| `%-10s` | a text on 10 characters, left-aligned | `Go        ` |
| `%t` | a boolean | `true` |
| `%c`, `%U` | a character, its Unicode code | `é`, `U+00E9` |
| `%x`, `%b` | hexadecimal, binary | `ff`, `101` |
| `%p` | an address | `0xc000012345` |
| `%%` | the `%` sign | `%` |

```go
package main

import "fmt"

type P struct {
	Name string
	Age  int
}

func main() {
	p := P{"Alice", 30}
	fmt.Printf("%v | %+v | %#v | %T\n", p, p, p, p)
	fmt.Printf("[%5d] [%-5d] [%05d] [%.2f] [%q] [%t]\n", 42, 42, 42, 3.14159, "Go", true)
	s := fmt.Sprintf("%-6s|%6s|", "ab", "cd")
	fmt.Println(s)
}
```

```sortie
{Alice 30} | {Name:Alice Age:30} | main.P{Name:"Alice", Age:30} | main.P
[   42] [42   ] [00042] [3.14] ["Go"] [true]
ab    |    cd|
```

## Conditions and loops

| Need | Syntax |
| --- | --- |
| Condition | `if x > 0 { … } else if … { … } else { … }` |
| With initialization | `if err := f(); err != nil { … }` |
| Several cases | `switch x { case 1, 2: … default: … }` |
| Chained conditions | `switch { case x < 0: … }` |
| Classic loop | `for i := 0; i < 5; i++ { … }` |
| "While" | `for x < 10 { … }` |
| Infinite | `for { … }` |
| Iterate | `for i, v := range slice { … }` |
| From 0 to n-1 | `for i := range 5 { … }` |
| Control | `break`, `continue` |
| Logic | `&&`, `\|\|`, `!` |

```go
package main

import "fmt"

func main() {
	for i := range 5 {
		if i == 1 {
			continue
		}
		switch {
		case i%2 == 0:
			fmt.Println(i, "even")
		default:
			fmt.Println(i, "odd")
		}
	}
}
```

```sortie
0 even
2 even
3 odd
4 even
```

Details: [conditions and loops](conditions-boucles.md).

## Functions

| Form | Example |
| --- | --- |
| Simple | `func double(x int) int { return x * 2 }` |
| Shared type | `func area(w, h float64) float64` |
| Several results | `func div(a, b int) (int, int)` |
| Variadic | `func sum(n ...int) int` ; call `sum(t...)` |
| Anonymous function | `f := func(x int) int { return x + 1 }` |
| Defer | `defer file.Close()` (when the function exits) |

```go
package main

import "fmt"

func divmod(a, b int) (int, int) {
	return a / b, a % b
}

func sum(numbers ...int) int {
	total := 0
	for _, n := range numbers {
		total += n
	}
	return total
}

func counter() func() int {
	n := 0
	return func() int {
		n++
		return n
	}
}

func main() {
	q, r := divmod(17, 5)
	fmt.Println(q, r)
	fmt.Println(sum(1, 2, 3), sum([]int{4, 5}...))
	next := counter()
	next()
	fmt.Println(next())
}
```

```sortie
3 2
6 9
2
```

Details: [functions](fonctions.md).

## Text

| Need | Example |
| --- | --- |
| Length in bytes / in characters | `len(s)`, `utf8.RuneCountInString(s)` |
| Uppercase, lowercase | `strings.ToUpper(s)`, `strings.ToLower(s)` |
| Remove surrounding spaces | `strings.TrimSpace(s)` |
| Search | `strings.Contains`, `strings.HasPrefix`, `strings.Index` |
| Split, join | `strings.Split(s, ",")`, `strings.Fields(s)`, `strings.Join(t, " ")` |
| Replace | `strings.ReplaceAll(s, "a", "b")` |
| Repeat | `strings.Repeat("-", 10)` |
| Build efficiently | `var sb strings.Builder` ; `sb.WriteString(…)` |
| Iterate over characters | `for i, r := range s` (r is a `rune`) |
| Raw text on several lines | `` `line 1 \n stays as is` `` |

```go
package main

import (
	"fmt"
	"strings"
	"unicode/utf8"
)

func main() {
	s := "  Noël à Paris  "
	t := strings.TrimSpace(s)
	fmt.Println(len(t), utf8.RuneCountInString(t))
	fmt.Println(strings.ToUpper(t), strings.Contains(t, "Paris"))
	fmt.Printf("%q\n", strings.Fields(t))
	fmt.Println(strings.Join([]string{"a", "b", "c"}, "-"))
}
```

```sortie
14 12
NOËL À PARIS true
["Noël" "à" "Paris"]
a-b-c
```

Details: [text](texte.md), [regular expressions](regex.md).

## Slices

| Need | Example |
| --- | --- |
| Create | `t := []int{1, 2, 3}`, `make([]int, 0, 10)` |
| Length, capacity | `len(t)`, `cap(t)` |
| Read, modify | `t[0]`, `t[len(t)-1] = 9` |
| Append | `t = append(t, 4, 5)` |
| Sub-slice | `t[1:3]` (indexes 1 and 2), `t[:2]`, `t[2:]` |
| Copy | `c := slices.Clone(t)` |
| Sort | `slices.Sort(t)`, `slices.SortFunc(t, cmp)` |
| Search | `slices.Contains(t, 3)`, `slices.Index(t, 3)` |
| Min, max | `slices.Min(t)`, `slices.Max(t)` |
| Array (fixed size) | `var a [3]int` |

```go
package main

import (
	"fmt"
	"slices"
)

func main() {
	t := []int{5, 2, 8}
	t = append(t, 1)
	slices.Sort(t)
	fmt.Println(t, len(t), t[1:3])
	fmt.Println(slices.Contains(t, 8), slices.Index(t, 8), slices.Max(t))
}
```

```sortie
[1 2 5 8] 4 [2 5]
true 3 8
```

Details: [collections](collections.md).

## Maps

| Need | Example |
| --- | --- |
| Create | `m := map[string]int{"a": 1}`, `make(map[string]int)` |
| Read, write | `m["a"]`, `m["b"] = 2` |
| Test a key | `v, ok := m["c"]` |
| Delete | `delete(m, "a")` |
| Size | `len(m)` |
| Sorted keys | `slices.Sorted(maps.Keys(m))` |
| Warning | the order of `range` over a map is random; writing to a `nil` map panics |

```go
package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	stock := map[string]int{"apples": 3, "pears": 0}
	stock["kiwis"] = 7
	v, ok := stock["bananas"]
	fmt.Println(v, ok)
	delete(stock, "pears")
	for _, k := range slices.Sorted(maps.Keys(stock)) {
		fmt.Println(k, stock[k])
	}
}
```

```sortie
0 false
apples 3
kiwis 7
```

Details: [maps](maps.md).

## Structs and methods

| Need | Example |
| --- | --- |
| Define | `type Point struct { X, Y int }` |
| Create | `p := Point{X: 1, Y: 2}`, `Point{1, 2}` |
| Field | `p.X` |
| Method (read) | `func (p Point) Norm() float64` |
| Method (modifies) | `func (p *Point) Move(dx int)` |
| Pointer | `pp := &p` ; `*pp` ; `pp.X` works directly |
| Constructor (by convention) | `func NewPoint(x, y int) *Point` |
| Exported / private | `Uppercase` / `lowercase` |
| Embed a type | `type Cat struct { Animal }` |

```go
package main

import "fmt"

type Account struct {
	Owner   string
	Balance int
}

func (a Account) String() string {
	return fmt.Sprintf("%s: €%d", a.Owner, a.Balance)
}

func (a *Account) Deposit(amount int) {
	a.Balance += amount
}

func main() {
	a := Account{Owner: "Alice"}
	a.Deposit(50)
	a.Deposit(20)
	fmt.Println(a)
}
```

```sortie
Alice: €70
```

Details: [types](types.md), [pointers](pointeurs.md).

## Interfaces

| Need | Example |
| --- | --- |
| Define | `type Shape interface { Area() float64 }` |
| Satisfy | have the methods, without declaring anything |
| Any type | `any` (alias of `interface{}`) |
| Type assertion | `c, ok := s.(Circle)` |
| Type switch | `switch v := x.(type) { case int: … }` |
| Common ones | `fmt.Stringer`, `error`, `io.Reader`, `io.Writer` |

```go
package main

import "fmt"

func describe(x any) string {
	switch v := x.(type) {
	case int:
		return fmt.Sprintf("integer %d", v)
	case string:
		return fmt.Sprintf("text %q", v)
	default:
		return fmt.Sprintf("other (%T)", v)
	}
}

func main() {
	for _, x := range []any{42, "Go", 3.5} {
		fmt.Println(describe(x))
	}
}
```

```sortie
integer 42
text "Go"
other (float64)
```

Details: [interfaces](interfaces.md).

## Errors

| Need | Example |
| --- | --- |
| Return an error | `return 0, errors.New("message")` |
| With values | `fmt.Errorf("grade %d invalid", n)` |
| Wrap | `fmt.Errorf("reading: %w", err)` |
| Test | `if err != nil { return err }` |
| Specific error | `errors.Is(err, os.ErrNotExist)` |
| Error type | `var e *MyError; errors.As(err, &e)` |
| Serious bug | `panic("…")` ; `recover()` in a `defer` |

```go
package main

import (
	"errors"
	"fmt"
	"strconv"
)

var ErrNegative = errors.New("negative number")

func read(s string) (int, error) {
	n, err := strconv.Atoi(s)
	if err != nil {
		return 0, fmt.Errorf("read %q: %w", s, err)
	}
	if n < 0 {
		return 0, ErrNegative
	}
	return n, nil
}

func main() {
	for _, s := range []string{"12", "x", "-3"} {
		n, err := read(s)
		switch {
		case errors.Is(err, ErrNegative):
			fmt.Println(s, "->", "rejected:", err)
		case err != nil:
			fmt.Println(s, "->", err)
		default:
			fmt.Println(s, "->", n)
		}
	}
}
```

```sortie
12 -> 12
x -> read "x": strconv.Atoi: parsing "x": invalid syntax
-3 -> rejected: negative number
```

Details: [errors](erreurs.md).

## Generics

| Need | Example |
| --- | --- |
| Generic function | `func First[T any](t []T) T` |
| "comparable" constraint | `func Contains[T comparable](t []T, v T) bool` |
| "ordered" constraint | `func Max[T cmp.Ordered](a, b T) T` |
| Union of types | `type Number interface { ~int \| ~float64 }` |
| Generic type | `type Stack[T any] struct { elements []T }` |

```go
package main

import "fmt"

type Number interface {
	~int | ~float64
}

func Sum[T Number](values []T) T {
	var total T
	for _, v := range values {
		total += v
	}
	return total
}

type Stack[T any] struct {
	elements []T
}

func (s *Stack[T]) Push(v T) { s.elements = append(s.elements, v) }

func main() {
	fmt.Println(Sum([]int{1, 2, 3}), Sum([]float64{0.5, 1.25}))
	var s Stack[string]
	s.Push("a")
	s.Push("b")
	fmt.Println(s.elements)
}
```

```sortie
6 1.75
[a b]
```

Details: [generics](generiques.md).

## Goroutines and channels

| Need | Example |
| --- | --- |
| Start a goroutine | `go work()` |
| Wait for a group | `var wg sync.WaitGroup` ; `wg.Add(1)` ; `defer wg.Done()` ; `wg.Wait()` |
| Protect a variable | `var mu sync.Mutex` ; `mu.Lock()` ; `mu.Unlock()` |
| Create a channel | `ch := make(chan int)`, buffered: `make(chan int, 10)` |
| Send, receive | `ch <- v`, `v := <-ch`, `v, ok := <-ch` |
| Close, iterate | `close(ch)` ; `for v := range ch` |
| Wait on several channels | `select { case v := <-a: … case <-time.After(d): … }` |
| Detect races | `go run -race`, `go test -race` |

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	results := make(chan int)
	var wg sync.WaitGroup
	for i := 1; i <= 3; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			results <- i * i
		}()
	}
	go func() {
		wg.Wait()
		close(results)
	}()

	total := 0
	for r := range results {
		total += r
	}
	fmt.Println("Total:", total)
}
```

```sortie
Total: 14
```

Details: [concurrency](concurrence.md), [channels](canaux.md).

## Files

| Need | Example |
| --- | --- |
| Read everything | `data, err := os.ReadFile("a.txt")` |
| Write everything | `os.WriteFile("a.txt", data, 0o644)` |
| Open, create | `os.Open(name)`, `os.Create(name)` ; then `defer f.Close()` |
| Append at the end | `os.OpenFile(name, os.O_APPEND\|os.O_CREATE\|os.O_WRONLY, 0o644)` |
| Read line by line | `sc := bufio.NewScanner(f)` ; `for sc.Scan() { sc.Text() }` |
| Missing file? | `errors.Is(err, os.ErrNotExist)` |
| Paths | `filepath.Join("folder", "a.txt")`, `filepath.Ext(name)` |
| Program arguments | `os.Args`, or the `flag` package |

```go
package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	err := os.WriteFile("grades.txt", []byte("Alice 15\nBob 9\n"), 0o644)
	if err != nil {
		fmt.Println(err)
		return
	}
	f, err := os.Open("grades.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()
	sc := bufio.NewScanner(f)
	for n := 1; sc.Scan(); n++ {
		fmt.Printf("%d: %s\n", n, sc.Text())
	}
	_, err = os.ReadFile("missing.txt")
	fmt.Println(err)
}
```

```sortie
1: Alice 15
2: Bob 9
open missing.txt: no such file or directory
```

Details: [files](fichiers.md), [the command line](ligne-de-commande.md).

## JSON

| Need | Example |
| --- | --- |
| Struct → JSON | `json.Marshal(v)`, `json.MarshalIndent(v, "", "  ")` |
| JSON → struct | `json.Unmarshal(data, &v)` |
| Field name | `` Title string `json:"title"` `` |
| Omit if empty | `` `json:"grade,omitempty"` `` |
| Ignore a field | `` `json:"-"` `` |
| Rule | only **exported** fields (uppercase) are read and written |

```go
package main

import (
	"encoding/json"
	"fmt"
)

type Task struct {
	Title string   `json:"title"`
	Done  bool     `json:"done"`
	Tags  []string `json:"tags,omitempty"`
}

func main() {
	data, _ := json.Marshal(Task{Title: "Bread"})
	fmt.Println(string(data))

	var t Task
	err := json.Unmarshal([]byte(`{"title":"Go","done":true,"tags":["study"]}`), &t)
	fmt.Printf("%+v %v\n", t, err)
}
```

```sortie
{"title":"Bread","done":false}
{Title:Go Done:true Tags:[study]} <nil>
```

Details: [formats](formats.md).

## Dates and durations

Go describes date formats with a **reference date**: Monday, January 2, 2006, at
15:04:05. Write this date the way you want to see yours.

| Element | Layout code | Example |
| --- | --- | --- |
| Year | `2006` | `2026` |
| Month (number, name) | `01`, `Jan`, `January` | `03`, `Mar` |
| Day | `02` | `14` |
| Hour (24 h), minutes, seconds | `15`, `04`, `05` | `09`, `30`, `00` |
| Day of the week | `Mon`, `Monday` | `Sat` |
| Ready-made ISO format | `time.RFC3339` | `2026-03-14T09:30:00Z` |

| Need | Example |
| --- | --- |
| Now | `time.Now()` |
| Specific date | `time.Date(2026, 3, 14, 9, 30, 0, 0, time.UTC)` |
| Format, parse | `t.Format("01/02/2006")`, `time.Parse("01/02/2006", s)` |
| Durations | `90 * time.Minute`, `t.Add(d)`, `t2.Sub(t1)` |
| Add days | `t.AddDate(0, 0, 7)` |
| Wait | `time.Sleep(500 * time.Millisecond)` |

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	t := time.Date(2026, 3, 14, 9, 30, 0, 0, time.UTC)
	fmt.Println(t.Format("01/02/2006 3:04 PM"))
	fmt.Println(t.Format(time.RFC3339), t.Weekday())
	end := t.Add(90 * time.Minute)
	fmt.Println(end.Format("15:04"), end.Sub(t))
	d, err := time.Parse("01/02/2006", "12/25/2026")
	fmt.Println(d.Sub(t).Hours()/24, err)
}
```

```sortie
03/14/2026 9:30 AM
2026-03-14T09:30:00Z Saturday
11:00 1h30m0s
285.6041666666667 <nil>
```

Details: [dates](dates.md).

## Tests

| Element | Rule |
| --- | --- |
| File | `xxx_test.go`, in the same folder as the code |
| Function | `func TestName(t *testing.T)` |
| Report a failure | `t.Errorf("got %v, want %v", got, want)` ; `t.Fatalf` stops the test |
| Subtests | `t.Run("case", func(t *testing.T) { … })` |
| Run | `go test ./...`, `go test -v`, `go test -run TestName` |
| Coverage, races | `go test -cover`, `go test -race` |
| Benchmark | `func BenchmarkName(b *testing.B)` ; `go test -bench .` |

```go norun
// calc_test.go
package calc

import "testing"

func TestDouble(t *testing.T) {
	cases := []struct {
		input, want int
	}{
		{2, 4},
		{0, 0},
		{-3, -6},
	}
	for _, c := range cases {
		if got := Double(c.input); got != c.want {
			t.Errorf("Double(%d) = %d, want %d", c.input, got, c.want)
		}
	}
}
```

Details: [tests](tests.md).

## Project and tools

| Command | Role |
| --- | --- |
| `go mod init example.com/app` | create a module (`go.mod`) |
| `go run .` / `go run main.go` | compile and run |
| `go build -o app ./cmd/app` | create an executable |
| `go install ./cmd/app` | install into `~/go/bin` |
| `GOOS=windows GOARCH=amd64 go build` | compile for another system |
| `go get module@v1.2.3` | add a dependency |
| `go mod tidy` | add what's missing, remove what's unused |
| `go fmt ./...` / `gofmt -w .` | format the code |
| `go vet ./...` | spot suspicious code |
| `go doc strings.Split` | read the documentation |
| `dlv debug` | start the Delve debugger |

| Folder | Contents |
| --- | --- |
| `cmd/<name>/` | a program (`package main`) |
| `internal/` | packages private to the module |
| `*_test.go` | the tests, next to the code |

Details: [debugging](debogage.md), [writing good Go](style.md),
[organizing a project](projet.md), [modules](modules.md).
