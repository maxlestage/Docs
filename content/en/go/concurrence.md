# Goroutines

This page shows how to run several tasks at the same time with goroutines, wait for them with `sync.WaitGroup` and protect shared data with `sync.Mutex`.

> **Goals:**
>
> - you will know how to start a goroutine with the `go` keyword;
> - you will know how to wait for several goroutines to finish with `sync.WaitGroup`;
> - you will know how to spot a race condition and fix it with `sync.Mutex`;
> - you will know how to get output that is always the same despite the disorder;
> - you will know how to organize the work with a pool of workers.

## Doing several things at once

Until now, your programs did one thing after another.
[Concurrency](../debuter/glossaire.md#concurrency) lets you make progress on several
tasks at the same time: download ten files without waiting for each one to finish,
answer several visitors of a website…

Think of a kitchen. A single cook makes the pasta, **then** the sauce, **then** the
dessert. A team of cooks does everything at the same time, and the meal is ready much
sooner. But they need to get organized: know when everyone is done, and not fight over
the same pan.

## The go keyword

A **goroutine** is a function that runs alongside the rest of the program. To start
one, write `go` in front of a function call. That's all. A goroutine costs very little:
a program can start thousands of them. It looks like a
[thread](../debuter/glossaire.md#thread), but much lighter.

```go norun
go cook("pasta")
```

`go` starts the function and moves on to the next line **right away**, without waiting.
Problem: when `main` ends, the program stops, even if goroutines are still working. So
you have to wait for them.

## Waiting with sync.WaitGroup

A `sync.WaitGroup` is a counter of running tasks, like the head chef counting the
cooks:

- `wg.Add(1)`: "one more task" (before starting the goroutine);
- `wg.Done()`: "one task finished" (at the end of the goroutine);
- `wg.Wait()`: "I wait until the counter is back to zero".

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	dishes := []string{"pasta", "sauce", "dessert"}
	ready := make([]string, len(dishes))

	var wg sync.WaitGroup
	for i, dish := range dishes {
		wg.Add(1)
		go func() {
			defer wg.Done()
			ready[i] = dish + ": ready"
		}()
	}
	wg.Wait()

	for _, r := range ready {
		fmt.Println(r)
	}
}
```

```sortie
pasta: ready
sauce: ready
dessert: ready
```

`go func() { ... }()` starts an anonymous function written on the spot. `defer
wg.Done()` guarantees that `Done` is called at the end, even if the function stops
early.

### Why store the results?

Goroutines finish in an **unpredictable** order: the first one started is not
necessarily the first one to finish. If each one printed its result, the order would
change from one run to the next.

Here, each goroutine writes into **its own slot** `ready[i]`. The program then prints
the slots in order, after `wg.Wait()`. The output is always the same.

> **Note:** since Go 1.22, each loop iteration has its own `i` and `dish` variables.
> In older code, you will see `go func(i int) { ... }(i)`: it used to be necessary.

## The problem of simultaneous access

What happens if several goroutines modify **the same** variable? Take a counter that
1000 goroutines increase by 1.

```go norun
counter := 0
var wg sync.WaitGroup
for i := 0; i < 1000; i++ {
	wg.Add(1)
	go func() {
		defer wg.Done()
		counter++
	}()
}
wg.Wait()
fmt.Println(counter)
```

You expect `1000`, but the result can be smaller, and different every time.
`counter++` happens in three steps: read the value, add 1, write. Two goroutines can
read `41` at the same time, then both write `42`: one addition is lost.

This is a **race condition** (*data race*): two cooks salting the same pan without
talking to each other.

### The race detector

Go can find these bugs. Add `-race`:

```sh
go run -race main.go
```

The program runs more slowly, but prints `WARNING: DATA RACE` with the lines involved
as soon as a race happens. Remember it when you write concurrent code, and in your
[tests](tests.md) (`go test -race`).

## Protecting with sync.Mutex

A `sync.Mutex` ("mutual exclusion") is a lock. It's the single key to a café's
restroom: one person at a time, the others wait.

- `mu.Lock()` takes the lock (or waits until it's free);
- `mu.Unlock()` gives it back.

The code between the two is run by only one goroutine at a time.

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

The result is now always `1000`.

### A protected type

The cleanest way is to keep the lock **with** the data it protects, in a
[struct](types.md). The methods take a pointer receiver: a lock must never be copied.

```go
package main

import (
	"fmt"
	"sort"
	"sync"
)

type Votes struct {
	mu    sync.Mutex
	count map[string]int
}

func (v *Votes) Vote(choice string) {
	v.mu.Lock()
	defer v.mu.Unlock()
	v.count[choice]++
}

func main() {
	v := &Votes{count: map[string]int{}}
	ballots := []string{"cat", "dog", "cat", "rabbit", "cat", "dog"}

	var wg sync.WaitGroup
	for _, b := range ballots {
		wg.Add(1)
		go func() {
			defer wg.Done()
			v.Vote(b)
		}()
	}
	wg.Wait()

	names := make([]string, 0, len(v.count))
	for name := range v.count {
		names = append(names, name)
	}
	sort.Strings(names)
	for _, name := range names {
		fmt.Println(name, v.count[name])
	}
}
```

```sortie
cat 3
dog 2
rabbit 1
```

The order of a [map](maps.md) changes at each traversal: we sort the keys before
printing.

## A pool of workers

Starting one goroutine per task works well for a few tasks. For thousands of files to
process, you'd rather use a **pool of workers** (*worker pool*): a fixed number of
goroutines that share the list. It's a team of three cashiers for a whole line of
customers.

Here, each worker takes the next task thanks to a shared index, protected by a lock.

```go
package main

import (
	"fmt"
	"sort"
	"sync"
)

func main() {
	numbers := []int{3, 1, 4, 1, 5, 9, 2, 6}
	var results []string

	var mu sync.Mutex
	next := 0

	var wg sync.WaitGroup
	for w := 1; w <= 3; w++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				mu.Lock()
				if next >= len(numbers) {
					mu.Unlock()
					return
				}
				n := numbers[next]
				next++
				mu.Unlock()

				square := fmt.Sprintf("%d² = %d", n, n*n)

				mu.Lock()
				results = append(results, square)
				mu.Unlock()
			}
		}()
	}
	wg.Wait()

	sort.Strings(results)
	fmt.Println(len(results), "results")
	for _, r := range results {
		fmt.Println(r)
	}
}
```

```sortie
8 results
1² = 1
1² = 1
2² = 4
3² = 9
4² = 16
5² = 25
6² = 36
9² = 81
```

The computation happens **outside** the lock: you hold the lock as briefly as possible,
otherwise the workers wait for each other. The next chapter,
[channels](canaux.md), shows a more natural way to hand out tasks.

## Common mistakes

### Not waiting for the goroutines

```go norun
package main

import "fmt"

func main() {
	go fmt.Println("Hello from a goroutine")
	fmt.Println("End")
}
```

```text
End
```

`main` ends before the goroutine has had time to print: its message is lost. Wait for
it with a `sync.WaitGroup`.

### Calling Done more often than Add

```go norun
package main

import (
	"fmt"
	"sync"
)

func main() {
	var wg sync.WaitGroup
	wg.Done()
	fmt.Println("end")
}
```

```text
panic: sync: negative WaitGroup counter

goroutine 1 [running]:
sync.(*WaitGroup).Add(0x0?, 0x68?)
	/usr/local/go1.24.7/src/sync/waitgroup.go:64 +0xcd
sync.(*WaitGroup).Done(...)
	/usr/local/go1.24.7/src/sync/waitgroup.go:89
main.main()
	/path/to/main.go:10 +0x27
exit status 2
```

The counter went below zero. Each `wg.Done()` must match a `wg.Add(1)` done **before**
starting the goroutine.

### Copying a WaitGroup

```go norun
package main

import (
	"fmt"
	"sync"
)

func work(id int, wg sync.WaitGroup) {
	defer wg.Done()
	fmt.Println("work", id)
}

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	go work(1, wg)
	wg.Wait()
}
```

```text
work 1
fatal error: all goroutines are asleep - deadlock!

goroutine 1 [sync.WaitGroup.Wait]:
sync.runtime_SemacquireWaitGroup(0xc00001c240?)
	/usr/local/go1.24.7/src/runtime/sema.go:110 +0x25
sync.(*WaitGroup).Wait(0x0?)
	/usr/local/go1.24.7/src/sync/waitgroup.go:118 +0x48
main.main()
	/path/to/main.go:17 +0x72
exit status 2
```

The function received a **copy** of the `WaitGroup`: its `Done` decrements the copy,
and the original waits forever. Pass a pointer: `wg *sync.WaitGroup` and
`work(1, &wg)`. The `go vet` command reports this trap.

## Test yourself

**Question 1:** what does `go f()` do?

> **Answer**
>
> It starts `f` in a new goroutine and immediately moves on to the next line, without
> waiting for `f` to finish.

**Question 2:** why doesn't `counter++` in 1000 goroutines always give 1000 without a
lock?

> **Answer**
>
> Because `counter++` reads, adds, then writes. Two goroutines can read the same value
> at the same time: one addition is lost. It's a race condition; a `sync.Mutex`
> prevents it.

**Question 3:** compute the length of each word of `[]string{"go", "fast", "and",
"simple"}` in goroutines, and print the results in the order of the words.

> **Answer**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"sync"
> )
>
> func main() {
> 	words := []string{"go", "fast", "and", "simple"}
> 	lengths := make([]int, len(words))
> 	var wg sync.WaitGroup
> 	for i, w := range words {
> 		wg.Add(1)
> 		go func() {
> 			defer wg.Done()
> 			lengths[i] = len(w)
> 		}()
> 	}
> 	wg.Wait()
> 	for i, w := range words {
> 		fmt.Println(w, lengths[i])
> 	}
> }
> ```
>
> ```sortie
> go 2
> fast 4
> and 3
> simple 6
> ```
>
> Each goroutine writes into its own slot: no lock needed.

## Summary

- `go f()` starts `f` in a goroutine, without waiting.
- `sync.WaitGroup` waits for goroutines to finish: `Add` before, `Done` at the end,
  `Wait`.
- Several goroutines modifying the same data create a race; `sync.Mutex` protects it,
  `go run -race` detects it.
- Store results in separate slots or sort them for stable output.
- A pool of workers limits the number of goroutines.

Next: [channels](canaux.md).
