# sync: Once, RWMutex, atomic and errgroup

This page introduces Go's other coordination tools: `sync.Once` to do something only once, `sync.RWMutex` to let many goroutines read at the same time, `sync/atomic` for counters, and `errgroup` to wait for goroutines that can fail.

> **Goals:**
>
> - recognise a race condition and have `go run -race` report it;
> - choose between `sync.Mutex` and `sync.RWMutex`;
> - run an initialisation only once with `sync.Once`;
> - count without a lock with `atomic.Int64`;
> - wait for goroutines and get the first error with `errgroup`.

This chapter assumes you know [goroutines](concurrence.md), `sync.WaitGroup` and
`sync.Mutex`. We go over them again quickly.

## The sync package toolbox

Several [goroutines](../debuter/glossaire.md#goroutine) sharing data is like several
cooks in the same kitchen. You need rules. Each tool is a different rule:

| Tool | The rule, in the kitchen | Used for |
|---|---|---|
| `sync.WaitGroup` | the head chef counts the cooks who are done | waiting for the end |
| `sync.Mutex` | a single key for the cold room | one at a time |
| `sync.RWMutex` | everyone reads the recipe, only one corrects it | many readers, one writer |
| `sync.Once` | the oven is switched on only once in the morning | doing something once |
| `atomic.Int64` | a tally counter at the entrance | counting fast without a lock |
| `errgroup` | the head chef waits for everyone and notes the first problem | waiting and getting an error |

> **Remember:** each tool answers a precise question. Start by asking yourself: "what is
> shared, and who touches it?"

## Reminder: waiting with WaitGroup

`wg.Add(1)` before starting, `defer wg.Done()` in the goroutine, `wg.Wait()` to wait.
Each goroutine writes into **its own slot**: there is nothing to protect.

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	numbers := []int{1, 2, 3, 4}
	squares := make([]int, len(numbers))

	var wg sync.WaitGroup
	for i, n := range numbers {
		wg.Add(1)
		go func() {
			defer wg.Done()
			squares[i] = n * n
		}()
	}
	wg.Wait()
	fmt.Println(squares)
}
```

```sortie
[1 4 9 16]
```

```text
goroutine 0 ──► squares[0]
goroutine 1 ──► squares[1]      each its own slot:
goroutine 2 ──► squares[2]      no fighting
goroutine 3 ──► squares[3]
```

> **Remember:** if each goroutine has its own slot, a `WaitGroup` is enough. Locks are
> only needed when several goroutines touch the **same** data.

## A race condition, step by step

When two goroutines change the same variable without any rule, you get a **race
condition** (a *data race*). Here is the smallest example: two goroutines do
`counter++`.

```go norun
counter := 0
var wg sync.WaitGroup
for i := 0; i < 2; i++ {
	wg.Add(1)
	go func() {
		defer wg.Done()
		counter++
	}()
}
wg.Wait()
fmt.Println(counter)
```

`counter++` looks like a single action. In reality, it is three small steps: **read**,
**add 1**, **write**. Here is one possible order, where one addition gets lost:

| Step | Goroutine A | Goroutine B | counter in memory |
|---|---|---|---|
| 1 | reads 0 | | 0 |
| 2 | | reads 0 | 0 |
| 3 | computes 0 + 1 = 1 | | 0 |
| 4 | | computes 0 + 1 = 1 | 0 |
| 5 | writes 1 | | 1 |
| 6 | | writes 1 | **1** instead of 2 |

It's like two people counting a museum's visitors in the same notebook: they both read
"41" at the same time, both write "42", and one visitor vanishes.

### The -race detector

The bug is rare and changes from one run to the next: you don't always see it. Go has a
detector. Add `-race` to `go run` (or to `go test`):

```sh
go run -race main.go
```

```text
==================
WARNING: DATA RACE
Read at 0x00c000122078 by goroutine 7:
  main.main.func1()
      /home/lea/counter/main.go:15 +0x84

Previous write at 0x00c000122078 by goroutine 8:
  main.main.func1()
      /home/lea/counter/main.go:15 +0x96

Goroutine 7 (running) created at:
  main.main()
      /home/lea/counter/main.go:13 +0x78

Goroutine 8 (finished) created at:
  main.main()
      /home/lea/counter/main.go:13 +0x78
==================
2
Found 1 data race(s)
exit status 66
```

How to read it:

- `WARNING: DATA RACE`: a race was seen;
- `Read at … main.go:15` then `Previous write at … main.go:15`: a read and a write of
  the same memory slot, on line 15 (`counter++`), by two different goroutines;
- `created at … main.go:13`: the line where these goroutines are started (`go func()`);
- the `2` printed proves nothing: this time, we were lucky.

Try it: run the program without `-race`, then with it. Without it, it prints `2` and
says nothing more. With it, it prints the report and ends with exit code 66.

> **Remember:** run your programs and your [tests](tests.md) with `-race` as soon as
> there are goroutines. The detector finds the bugs your eyes don't see.

## Reminder: one at a time with Mutex

A `sync.Mutex` is a lock. `mu.Lock()` takes the key; if someone already has it, you
wait. `mu.Unlock()` gives the key back.

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	counter := 0
	var mu sync.Mutex
	var wg sync.WaitGroup

	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			mu.Lock()
			counter++
			mu.Unlock()
		}()
	}
	wg.Wait()
	fmt.Println(counter)
}
```

```sortie
1000
```

With the lock, the table from before becomes:

| Step | Goroutine A | Goroutine B | counter |
|---|---|---|---|
| 1 | `Lock()`: takes the key | | 0 |
| 2 | reads 0, writes 1 | `Lock()`: waits… | 1 |
| 3 | `Unlock()` | …still waiting | 1 |
| 4 | | takes the key, reads 1, writes 2 | 2 |
| 5 | | `Unlock()` | 2 |

> **Remember:** everything between `Lock()` and `Unlock()` is done by only one goroutine
> at a time.

## Reading together: RWMutex

A `Mutex` makes everyone wait, even those who only want to **read**. That's a shame:
reading doesn't break anything.

Think of a school notice board. Twenty pupils can read it at the same time. But when the
head teacher changes a poster, he asks everyone to step aside.

So a `sync.RWMutex` has two kinds of locks:

| Method | For whom | How many at a time |
|---|---|---|
| `RLock()` / `RUnlock()` | a reader | as many as you like |
| `Lock()` / `Unlock()` | a writer | only one, and no reader meanwhile |

```go
package main

import (
	"fmt"
	"sync"
)

type Directory struct {
	mu      sync.RWMutex
	numbers map[string]string
}

func (d *Directory) Find(name string) string {
	d.mu.RLock()
	defer d.mu.RUnlock()
	return d.numbers[name]
}

func (d *Directory) Add(name, number string) {
	d.mu.Lock()
	defer d.mu.Unlock()
	d.numbers[name] = number
}

func main() {
	d := &Directory{numbers: map[string]string{}}
	d.Add("Léa", "06 11")
	d.Add("Tom", "06 22")

	names := []string{"Léa", "Tom", "Léa", "Zoé"}
	found := make([]string, len(names))

	var wg sync.WaitGroup
	for i, name := range names {
		wg.Add(1)
		go func() {
			defer wg.Done()
			found[i] = name + " → " + d.Find(name)
		}()
	}
	wg.Wait()

	for _, f := range found {
		fmt.Println(f)
	}
}
```

```sortie
Léa → 06 11
Tom → 06 22
Léa → 06 11
Zoé → 
```

The four lookups can happen **at the same time**: they all take an `RLock`. `Zoé` is not
in the [map](maps.md): you get the empty string.

```text
 readers:  Léa ─┐
           Tom ─┼──► RLock  RLock  RLock  RLock    (all together)
           Léa ─┤
           Zoé ─┘
 writer:   Add ──► Lock   (waits for the readers to finish,
                           then goes alone)
```

`defer d.mu.RUnlock()` gives the lock back when the method exits, even with a `return`
in the middle (see [defer](fonctions-avancees.md#defer-pushing-back-to-the-end)).

Try it: replace `RLock`/`RUnlock` with `Lock`/`Unlock` in `Find`. The result is the
same, but the lookups go through one by one.

> **Remember:** use an `RWMutex` when you read **much more** than you write (a cache, a
> configuration). Otherwise, a plain `Mutex` is enough and stays simpler.

## Only once: sync.Once

Some things must be done only once: reading a configuration file, opening a connection.
It's like switching on the oven in the morning: the first cook switches it on, the next
ones find it already hot.

`once.Do(f)` calls `f` the first time, and **never again** after that.

```go
package main

import (
	"fmt"
	"sync"
)

var (
	once   sync.Once
	config map[string]string
)

func load() {
	fmt.Println("loading the configuration…")
	config = map[string]string{"language": "en", "theme": "dark"}
}

func get(key string) string {
	once.Do(load)
	return config[key]
}

func main() {
	fmt.Println(get("language"))
	fmt.Println(get("theme"))
	fmt.Println(get("language"))
}
```

```sortie
loading the configuration…
en
dark
en
```

| Call | once already done? | load() called? | Prints |
|---|---|---|---|
| `get("language")` | no | yes | the message, then `en` |
| `get("theme")` | yes | no | `dark` |
| `get("language")` | yes | no | `en` |

### With several goroutines

`sync.Once` also works when ten goroutines arrive **at the same time**. Only one runs the
function; the others wait for it to finish, then carry on.

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	var once sync.Once
	var wg sync.WaitGroup

	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			once.Do(func() { fmt.Println("the oven is on") })
		}()
	}
	wg.Wait()
	fmt.Println("10 cooks served")
}
```

```sortie
the oven is on
10 cooks served
```

### sync.OnceValue: once, with a result

Since Go 1.21, `sync.OnceValue(f)` returns a function. The first call computes the
value; the next ones give it back without computing it again.

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	price := sync.OnceValue(func() int {
		fmt.Println("computing the price…")
		return 12 * 3
	})

	fmt.Println(price())
	fmt.Println(price())
}
```

```sortie
computing the price…
36
36
```

Try it: add a third `fmt.Println(price())`. The message `computing the price…` still
appears only once.

> **Remember:** `once.Do(f)` = "do `f` the first time, ignore the following times". It's
> safe even with many goroutines.

## Counting without a lock: sync/atomic

For a simple counter, a `Mutex` is a bit heavy. The `sync/atomic` package offers numbers
whose operations are **indivisible** ("atomic"): reading, adding and writing happen in a
single step that nobody can interrupt.

It's the nightclub bouncer's tally counter: click, +1. Two bouncers clicking at the same
time never lose a click.

```go
package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

func main() {
	var visits atomic.Int64
	var wg sync.WaitGroup

	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			visits.Add(1)
		}()
	}
	wg.Wait()
	fmt.Println("visits:", visits.Load())
}
```

```sortie
visits: 1000
```

The main methods:

| Method | What it does |
|---|---|
| `v.Add(n)` | adds `n` and returns the new value |
| `v.Load()` | reads the value |
| `v.Store(n)` | replaces the value |
| `v.CompareAndSwap(old, new)` | replaces only if the value is still `old` |

Let's follow a few calls, without goroutines, to see the values returned:

```go
package main

import (
	"fmt"
	"sync/atomic"
)

func main() {
	var n atomic.Int64
	fmt.Println(n.Add(5))
	fmt.Println(n.Add(-2))
	n.Store(10)
	fmt.Println(n.Load())
	fmt.Println(n.CompareAndSwap(10, 20), n.Load())
	fmt.Println(n.CompareAndSwap(10, 99), n.Load())
}
```

```sortie
5
3
10
true 20
false 20
```

| Step | Call | n after | Returns |
|---|---|---|---|
| 1 | `Add(5)` | 5 | 5 |
| 2 | `Add(-2)` | 3 | 3 |
| 3 | `Store(10)` | 10 | nothing |
| 4 | `CompareAndSwap(10, 20)` | 20 | `true`: n was indeed 10 |
| 5 | `CompareAndSwap(10, 99)` | 20 | `false`: n is 20, not 10 |

There are also `atomic.Int32`, `atomic.Uint64` and `atomic.Bool`. Their starting value
is zero (or `false`): no need to create them.

Try it: replace `visits.Add(1)` with `visits.Add(2)`. The program prints
`visits: 2000`.

> **Remember:** for **one** shared number, `atomic.Int64` is enough. As soon as you need
> to change several things together (a map and a counter), go back to a `Mutex`.

## Waiting and getting an error: errgroup

A `WaitGroup` waits, but knows nothing about errors. If a goroutine fails, how do you
find out? The `errgroup` package answers this question.

It is not part of the standard library: it lives in the `golang.org/x/sync` module,
maintained by the Go team. You need to add it to your project (see
[modules](modules.md)):

```sh
go get golang.org/x/sync/errgroup
```

The idea:

- `g.Go(f)` runs `f` in a goroutine; `f` returns an `error`;
- `g.Wait()` waits for all the goroutines and returns the **first** error (or `nil`).

```go norun
package main

import (
	"errors"
	"fmt"

	"golang.org/x/sync/errgroup"
)

func download(name string) (int, error) {
	if name == "c.txt" {
		return 0, errors.New("c.txt: not found")
	}
	return len(name) * 100, nil
}

func main() {
	files := []string{"a.txt", "b.txt", "c.txt"}
	sizes := make([]int, len(files))

	var g errgroup.Group
	for i, name := range files {
		g.Go(func() error {
			s, err := download(name)
			if err != nil {
				return err
			}
			sizes[i] = s
			return nil
		})
	}
	if err := g.Wait(); err != nil {
		fmt.Println("error:", err)
		return
	}
	fmt.Println("sizes:", sizes)
}
```

```text
error: c.txt: not found
```

| Goroutine | File | Returns |
|---|---|---|
| 1 | `a.txt` | `nil` (size 500) |
| 2 | `b.txt` | `nil` (size 500) |
| 3 | `c.txt` | `c.txt: not found` |
| `g.Wait()` | | the first error: `c.txt: not found` |

Try it: replace `"c.txt"` with `"d.txt"` in the list. Nothing fails any more, and the
program prints `sizes: [500 500 500]`.

Compare with a `WaitGroup`: no more `Add`, no more `Done`. `errgroup` does them for you.

### Stopping the others with WithContext

`errgroup.WithContext(ctx)` gives a group **and** a [context](context.md). As soon as a
goroutine returns an error, this context is cancelled: the others can stop.
`g.SetLimit(n)` limits how many goroutines run at the same time.

```go norun
g, ctx := errgroup.WithContext(context.Background())
g.SetLimit(3) // at most 3 downloads at a time
for _, url := range urls {
	g.Go(func() error {
		return downloadWith(ctx, url) // stops if ctx is cancelled
	})
}
err := g.Wait()
```

> **Remember:** `errgroup` = a `WaitGroup` that reports the first error. With
> `WithContext`, an error also warns all the other goroutines.

## Which tool to choose?

```text
Do goroutines share some data?
├── no ─► WaitGroup (or errgroup if they can fail)
└── yes
    ├── is it a simple number? ─► atomic.Int64
    ├── lots of reads, few writes? ─► RWMutex
    ├── initialised only once? ─► sync.Once
    └── otherwise ─► Mutex (or a channel, see the channels page)
```

Don't forget [channels](canaux.md): instead of sharing data, a goroutine can **send** it
to another one. The Go proverb says: "don't communicate by sharing memory; share memory
by communicating".

> **Remember:** use the simplest tool that does the job. And check with `-race`.

## Common mistakes

### Copying a struct that contains a Mutex

```go norun
package main

import (
	"fmt"
	"sync"
)

type Counter struct {
	mu sync.Mutex
	n  int
}

func show(c Counter) {
	fmt.Println(c.n)
}

func main() {
	var c Counter
	show(c)
}
```

The program compiles, but `go vet main.go` complains:

```text
# command-line-arguments
# [command-line-arguments]
./main.go:13:13: show passes lock by value: command-line-arguments.Counter contains sync.Mutex
./main.go:19:7: call of show copies lock value: command-line-arguments.Counter contains sync.Mutex
```

A copy of the lock is **another** lock: it no longer protects anything. Pass a pointer:
`func show(c *Counter)` and `show(&c)` (see [pointers](pointeurs.md)).

### Giving back a key you didn't take

```go norun
var mu sync.Mutex
mu.Lock()
fmt.Println("inside the protected section")
mu.Unlock()
mu.Unlock()
```

```text
inside the protected section
fatal error: sync: unlock of unlocked mutex

goroutine 1 [running]:
…
exit status 2
```

One `Unlock` too many stops the program. Write `defer mu.Unlock()` right after
`mu.Lock()`: there will always be exactly one `Unlock`.

### Taking the same lock twice

```go norun
var mu sync.RWMutex
mu.RLock()
fmt.Println("reading in progress")
mu.Lock()
fmt.Println("never printed")
```

```text
reading in progress
fatal error: all goroutines are asleep - deadlock!

goroutine 1 [sync.RWMutex.Lock]:
…
exit status 2
```

`Lock()` waits for all the readers to finish… but the reader is you. Nobody can move
forward: this is a **deadlock**. Go's locks are not "reentrant": a goroutine that
already holds the lock cannot take it again. Give back the `RLock` first, then take the
`Lock`.

## Test yourself

**Question 1:** why can `counter++` in 1000 goroutines give less than 1000? How can you
see it?

> **Answer**
>
> `counter++` happens in three steps (read, add, write). Two goroutines can read the
> same value and write the same result: one addition is lost. `go run -race` then
> prints `WARNING: DATA RACE`.

**Question 2:** what does this program print?

```go norun
var once sync.Once
for i := 0; i < 3; i++ {
	once.Do(func() { fmt.Println("round", i) })
}
```

> **Answer**
>
> Only `round 0`. The function passed to `once.Do` is called only on the first round;
> the next two ignore it.

**Question 3:** a price list is read thousands of times per second and updated once an
hour. Which lock should you choose?

> **Answer**
>
> A `sync.RWMutex`: `RLock` for the reads, which all go through together, and `Lock` for
> the rare update.

## Summary

- A race condition loses data without warning: use `-race`.
- `Mutex`: one at a time. `RWMutex`: several readers **or** a single writer.
- `sync.Once` (and `sync.OnceValue`) runs an initialisation only once.
- `atomic.Int64` counts without a lock with `Add` and `Load`.
- `errgroup` (module `golang.org/x/sync`) waits for goroutines and returns the first
  error.
- Never copy a lock; give it back with `defer`.

Next: [reading and writing files](fichiers.md).
