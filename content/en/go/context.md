# Cancelling and setting time limits with context

This page shows how the `context` package lets you tell a goroutine "stop, nobody is waiting for your work anymore", right away or after a certain time.

> **Goals:**
>
> - understand what a `context` is for;
> - cancel a piece of work with `context.WithCancel`;
> - set a maximum time with `context.WithTimeout`;
> - listen to `ctx.Done()` in a `select` and read `ctx.Err()`;
> - pass `ctx` as the first parameter of your functions.

This chapter assumes you know [goroutines](concurrence.md) and
[channels](canaux.md), especially `select`.

## Why cancel a piece of work?

Picture a pizzeria. You order a pizza by phone, then you hang up: you've changed your
mind. If nobody tells the pizza maker, he keeps baking a pizza that nobody will eat.

A program has the same problem. A [goroutine](../debuter/glossaire.md#goroutine)
started for some work keeps going, even when nobody is waiting for the result anymore:

- the user closed the web page;
- the answer arrives too late;
- another goroutine has already found the answer.

The **context** is the phone that lets you warn the pizza maker: "stop, forget it".

```text
 main ──── ctx ────► goroutine
   │                    │ working…
 cancel() ─────────►  ctx.Done() closes
                        │ "I was told to stop"
                        └─ stops
```

> **Remember:** a `context` carries a "stop" signal. Whoever starts the work sends it;
> whoever does the work listens to it.

## The starting point: context.Background

Every context starts from a root: `context.Background()`. It's an empty context. It is
never cancelled.

```go
package main

import (
	"context"
	"fmt"
)

func main() {
	ctx := context.Background()
	fmt.Println(ctx)
	fmt.Println(ctx.Err())
}
```

```sortie
context.Background
<nil>
```

`ctx.Err()` tells you whether the context is finished. `<nil>` means "no, all is well".

> **Note:** by habit, the variable is always called `ctx`.

## Cancelling by hand: WithCancel

`context.WithCancel(parent)` returns **two** things:

- a new context `ctx`, a child of `parent`;
- a function `cancel`. Calling it cancels `ctx`.

```go
package main

import (
	"context"
	"fmt"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	fmt.Println("before:", ctx.Err())
	cancel()
	fmt.Println("after:", ctx.Err())
}
```

```sortie
before: <nil>
after: context canceled
```

| Step | What happens | ctx.Err() |
|---|---|---|
| 1 | `WithCancel` creates `ctx` | `<nil>` |
| 2 | `cancel()` | `context canceled` |

Calling `cancel` a second time does nothing more. It isn't an error.

## Listening to ctx.Done() in a select

`ctx.Done()` is a [channel](../debuter/glossaire.md#canal). It **closes** when the
context is cancelled. Receiving from a closed channel succeeds right away: that's the
signal.

Here is a worker that waits. `main` cancels it, then waits for it to finish.

```go
package main

import (
	"context"
	"fmt"
)

func worker(ctx context.Context, done chan<- bool) {
	<-ctx.Done()
	fmt.Println("worker: I was told to stop:", ctx.Err())
	done <- true
}

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	done := make(chan bool)

	go worker(ctx, done)
	fmt.Println("main: cancelling")
	cancel()
	<-done
	fmt.Println("main: finished")
}
```

```sortie
main: cancelling
worker: I was told to stop: context canceled
main: finished
```

The worker can't print anything before `cancel()`: it is blocked on `<-ctx.Done()`.
So the order is always the same.

### A producer we stop

In real life, a worker does more than wait. It then puts `ctx.Done()` in a `select`,
next to its work. Here, a producer sends 1, 2, 3, 4… forever. `main` reads three of
them, then cancels.

```go
package main

import (
	"context"
	"fmt"
)

func producer(ctx context.Context, out chan<- int, done chan<- bool) {
	for n := 1; ; n++ {
		select {
		case out <- n:
			// sent, move on to the next one
		case <-ctx.Done():
			fmt.Println("producer: stopped")
			done <- true
			return
		}
	}
}

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	numbers := make(chan int)
	done := make(chan bool)

	go producer(ctx, numbers, done)
	for i := 0; i < 3; i++ {
		fmt.Println("received:", <-numbers)
	}
	cancel()
	<-done
}
```

```sortie
received: 1
received: 2
received: 3
producer: stopped
```

Step by step:

| Step | main | producer |
|---|---|---|
| 1 | receives 1 | sends 1 |
| 2 | receives 2 | sends 2 |
| 3 | receives 3 | sends 3 |
| 4 | calls `cancel()` | waits in `select` to send 4 |
| 5 | waits for `done` | `ctx.Done()` is ready: it stops |

The 4 is never sent: nobody reads `numbers` anymore. Only the `ctx.Done()` case can
win.

Try it: replace `i < 3` with `i < 5`. You receive 1 to 5, then `producer: stopped`.

> **Remember:** a worker that can be cancelled almost always contains
> `case <-ctx.Done():` in its `select`.

## Always call cancel with defer

A cancellable context holds on to a bit of
[memory](../debuter/glossaire.md#memoire-vive) as long as it isn't cancelled. The rule
is simple: right after creating the context, write `defer cancel()`.

```go norun
ctx, cancel := context.WithCancel(context.Background())
defer cancel() // cancelled at the end of the function, no matter what
```

`defer` pushes the call back to the end of the function (see
[defer](fonctions-avancees.md#defer-pushing-back-to-the-end)). Even if the function stops
early, the context will be cleaned up.

## A maximum time: WithTimeout

`context.WithTimeout(parent, duration)` creates a context that cancels **itself** after
the duration. It's a kitchen timer: when it rings, you stop.

The worker below waits for a result on a channel that **never** receives anything. So
the time limit always wins.

```go
package main

import (
	"context"
	"fmt"
	"time"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancel()

	never := make(chan int)
	select {
	case r := <-never:
		fmt.Println("result:", r)
	case <-ctx.Done():
		fmt.Println("giving up:", ctx.Err())
	}
}
```

```sortie
giving up: context deadline exceeded
```

`deadline exceeded` means "the time limit has passed". It's the second possible reason
for `ctx.Err()`.

On the other hand, if the result is already there, it comes before the time limit:

```go
package main

import (
	"context"
	"fmt"
	"time"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	result := make(chan int, 1)
	result <- 42

	select {
	case r := <-result:
		fmt.Println("result:", r)
	case <-ctx.Done():
		fmt.Println("giving up:", ctx.Err())
	}
	fmt.Println("ctx.Err():", ctx.Err())
}
```

```sortie
result: 42
ctx.Err(): <nil>
```

The one-second limit hasn't passed: `ctx.Err()` is still `<nil>`.

> **Note:** `WithTimeout` looks like `time.After` (see
> [a maximum time with time.After](canaux.md)). The difference: the context is
> **passed on** to other functions, which all stop at the same time.

## Reading the reason: ctx.Err()

`ctx.Err()` has three possible values:

| Situation | ctx.Err() |
|---|---|
| the context is still running | `nil` |
| someone called `cancel()` | `context.Canceled` |
| the time limit has passed | `context.DeadlineExceeded` |

To test the reason, use `errors.Is`.

```go
package main

import (
	"context"
	"errors"
	"fmt"
	"time"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel()
	<-ctx.Done()

	err := ctx.Err()
	if errors.Is(err, context.DeadlineExceeded) {
		fmt.Println("too slow")
	} else if errors.Is(err, context.Canceled) {
		fmt.Println("cancelled")
	}
}
```

```sortie
too slow
```

## Passing ctx as the first parameter

A function that may take time receives the context as its **first** parameter, named
`ctx`. This convention is followed throughout the Go library.

```go
package main

import (
	"context"
	"fmt"
	"time"
)

// search waits for an answer that never comes: only ctx can stop it.
func search(ctx context.Context, word string) (string, error) {
	answer := make(chan string)
	select {
	case r := <-answer:
		return r, nil
	case <-ctx.Done():
		return "", ctx.Err()
	}
}

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 20*time.Millisecond)
	defer cancel()

	_, err := search(ctx, "gopher")
	if err != nil {
		fmt.Println("search failed:", err)
	}
}
```

```sortie
search failed: context deadline exceeded
```

The context goes down from function to function, the way the phone passes from hand to
hand. If `search` called other functions, it would give them the same `ctx`.

> **Remember:** `func name(ctx context.Context, …)`. Don't store a context in a
> struct: pass it to each call.

## Children follow their parent

A context created from another one is its **child**. Cancelling the parent cancels all
its children. The reverse isn't true.

```go
package main

import (
	"context"
	"fmt"
)

func main() {
	parent, cancelParent := context.WithCancel(context.Background())
	child, cancelChild := context.WithCancel(parent)
	defer cancelChild()

	cancelParent()
	<-child.Done()
	fmt.Println("child:", child.Err())
}
```

```sortie
child: context canceled
```

```text
Background
   └── parent   ← cancel here…
         └── child   ← …also cancels this one
```

## Carrying a value: WithValue (in moderation)

`context.WithValue(parent, key, value)` attaches a small label to the context. You use
it for a piece of information that follows a request, like a tracking number.

```go
package main

import (
	"context"
	"fmt"
)

type key string

const trackingKey key = "tracking"

func handle(ctx context.Context) {
	number, ok := ctx.Value(trackingKey).(string)
	if !ok {
		number = "unknown"
	}
	fmt.Println("handling request", number)
}

func main() {
	ctx := context.WithValue(context.Background(), trackingKey, "A-42")
	handle(ctx)
	handle(context.Background())
}
```

```sortie
handling request A-42
handling request unknown
```

- The key has its own type (`key`). That way, it can't get mixed up with another
  package's key.
- `ctx.Value` returns a value of type `any`. The `.(string)` checks that it is a piece
  of text (see [interfaces](interfaces.md)).

> **Warning:** don't use `WithValue` to pass a function's normal parameters. A
> parameter written in the signature is much easier to read.

## Common mistakes

### Throwing away the cancel function

```go norun
package main

import (
	"context"
	"fmt"
	"time"
)

func main() {
	ctx, _ := context.WithTimeout(context.Background(), time.Second)
	fmt.Println(ctx.Err())
}
```

The program compiles, but `go vet main.go` warns you:

```text
# command-line-arguments
# [command-line-arguments]
./main.go:10:7: the cancel function returned by context.WithTimeout should be called, not discarded, to avoid a context leak
```

Keep `cancel` and write `defer cancel()` right below.

### Passing a nil context

```go norun
package main

import (
	"context"
	"fmt"
	"time"
)

func main() {
	var ctx context.Context
	ctx, cancel := context.WithTimeout(ctx, time.Second)
	defer cancel()
	fmt.Println(ctx.Err())
}
```

```text
panic: cannot create context from nil parent

goroutine 1 [running]:
context.WithDeadlineCause({0x0?, 0x0?}, {0x57dbe0?, 0x3b9aca00?, 0x57dbe0?}, {0x0?, 0x0?})
	/usr/local/go/src/context/context.go:625 +0x39e
…
exit status 2
```

A context declared with `var` is `nil`. Always start from `context.Background()`.

### Forgetting to listen to ctx.Done()

```go norun
func producer(ctx context.Context, out chan<- int) {
	for n := 1; ; n++ {
		out <- n // ctx is never looked at
	}
}
```

Here, no error is shown. But cancelling `ctx` is useless: the goroutine stays stuck on
`out <- n` forever. Receiving `ctx` isn't enough, you have to **listen** to it in a
`select`, as in [the producer](#a-producer-we-stop).

## Test yourself

**Question 1:** what is `ctx.Err()` right after `context.WithCancel(...)`, then after
`cancel()`?

> **Answer**
>
> First `nil` (the context is running). After `cancel()`, `context canceled`.

**Question 2:** what does this program print?

```go norun
ctx, cancel := context.WithTimeout(context.Background(), 30*time.Millisecond)
defer cancel()
never := make(chan string)
select {
case m := <-never:
	fmt.Println(m)
case <-ctx.Done():
	fmt.Println(ctx.Err())
}
```

> **Answer**
>
> `context deadline exceeded`. The `never` channel receives nothing: after 30 ms, the
> context cancels itself and the second case wins.

**Question 3:** where does the context go in a function's parameter list?

> **Answer**
>
> First, under the name `ctx`: `func download(ctx context.Context, url string)`.

## Summary

- A `context` carries the "stop" signal to goroutines.
- Everything starts from `context.Background()`.
- `WithCancel` gives you a `cancel` function; `WithTimeout` cancels itself. Always write
  `defer cancel()`.
- The worker listens to `case <-ctx.Done():` in a `select`; `ctx.Err()` gives the
  reason (`Canceled` or `DeadlineExceeded`).
- `ctx` is passed as the first parameter; `WithValue` is kept for small pieces of
  information.

Next: [sync: Once, RWMutex, atomic and errgroup](sync.md)
