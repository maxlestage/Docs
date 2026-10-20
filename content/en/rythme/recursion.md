# Recursion

Rythme has no loops: to repeat something, a function calls itself, and this page shows how to do it without getting lost.

> **Goals:**
>
> - you will know what a recursive function is, and why it stops;
> - you will know how to write a base case and a recursive case;
> - you will know how to go through a list with the `[x, ..rest]` pattern;
> - you will know how to use an accumulator to go as far as you want;
> - you will know when to prefer `map`, `filter` or `fold`.

## Why no loops?

In many languages, you repeat with a `for` or `while`
[loop](../debuter/glossaire.md#loop). A loop changes a counter on each
turn: `i` is 0, then 1, then 2...

In Rythme, a value never changes. So a counter that changes does not
exist. Two ways to repeat are left:

- the list functions, `map`, `filter`, `fold`, `each`, seen in
  [thinking functionally](fonctionnel.md);
- [recursion](../debuter/glossaire.md#recursion): a function that calls
  itself.

Recursion is the most general tool: anything a loop does, recursion can
do too.

## A function that calls itself

Think of Russian dolls. To get to the smallest one, you open a doll;
inside, there is a slightly smaller doll, which you open the same way;
and so on, until the doll that does not open.

```rythme
fn open(0) = print("the smallest doll!")
fn open(n)
  print("I open doll {n}")
  open(n - 1)
end

open(3)
```

```sortie
I open doll 3
I open doll 2
I open doll 1
the smallest doll!
```

`open(3)` prints a line, then calls `open(2)`, which calls `open(1)`,
which calls `open(0)`. There, the first definition applies, and
everything stops.

## Base case and recursive case

Every recursive function has two parts:

- the **base case**: the situation so simple that you know the answer
  without calling yourself (the doll that does not open);
- the **recursive case**: you take a small step, then you call yourself
  on a **smaller** problem.

The recursive case must always get closer to the base case. Otherwise,
the function never stops.

```rythme
fn countdown(0) = print("Liftoff!")
fn countdown(n)
  print(n)
  countdown(n - 1)
end

countdown(3)
```

```sortie
3
2
1
Liftoff!
```

Here, `n - 1` gets closer to 0 on each call.

## A definition by cases

Recursion reads like a definition in mathematics. The factorial of `n`
(written `n!`) is the product `1 × 2 × ... × n`:

- `0!` is 1;
- `n!` is `n × (n - 1)!`.

In Rythme, you write exactly that, one line per case:

```rythme
fn factorial(0) = 1
fn factorial(n) = n * factorial(n - 1)

print(factorial(5), factorial(20))
```

```sortie
120 2432902008176640000
```

The definitions are tried **in order** (see
[functions](fonctions.md#defining-by-cases)): so the base case comes
first. The same idea gives the power:

```rythme
fn power(x, 0) = 1
fn power(x, n) = x * power(x, n - 1)

print(power(2, 10), power(3, 4))
```

```sortie
1024 81
```

## Going through a list

A list naturally splits into two pieces: its first element, and **the
rest**, which is itself a list, a shorter one. The `[x, ..rest]` pattern
does this split. The base case is the empty list `[]`.

```rythme
fn length([]) = 0
fn length([_, ..rest]) = 1 + length(rest)

fn add_up([]) = 0
fn add_up([x, ..rest]) = x + add_up(rest)

print(length([4, 8, 15]), add_up([4, 8, 15]))
```

```sortie
3 27
```

Read `add_up` out loud: "the sum of an empty list is 0; the sum of a list
that starts with `x` is `x` plus the sum of the rest".

A recursive function can also build a new list:

```rythme
fn double_all([]) = []
fn double_all([x, ..rest]) = [x * 2, ..double_all(rest)]

fn positives([]) = []
fn positives([x, ..rest]) if x > 0 = [x, ..positives(rest)]
fn positives([_, ..rest]) = positives(rest)

print(double_all([1, 2, 3]), positives([3, -1, 4, -5]))
```

```sortie
[2, 4, 6] [3, 4]
```

`positives` uses a guard, `if x > 0`: you keep `x` only if it is
positive. The `[x, ..rest]` pattern is explained in detail in
[pattern matching](pattern-matching.md).

## Watching the calculation step by step

`rythme steps` shows each step of a calculation (see
[rythme steps](etapes.md)). With this file `f.ry`:

```rythme norun
fn factorial(0) = 1
fn factorial(n) = n * factorial(n - 1)
print(factorial(3))
```

```sh
rythme steps f.ry
```

```text
print(factorial(3))
  → print(3 * factorial(2))
  → print(3 * (2 * factorial(1)))
  → print(3 * (2 * (1 * factorial(0))))
  → print(3 * (2 * (1 * 1)))
  → print(3 * (2 * 1))
  → print(3 * 2)
  → print(6)
6
  → ()
```

You can see the **waiting** multiplications pile up towards the right,
then get computed once the base case is reached. Each call waits for the
answer of the next one: this is the call
[stack](../debuter/glossaire.md#stack-and-heap).

## Accumulators

This stack has a limit. With a list of a million elements, `add_up` would
wait for a million answers at the same time: that is too many (see the
common mistakes below).

The solution: carry the result **with you**, in one more parameter, the
**accumulator**. It is like counting sheep while keeping the total in your
head, instead of writing each sheep down on a pile of paper.

```rythme
fn add_up([], total) = total
fn add_up([x, ..rest], total) = add_up(rest, total + x)

print(add_up([4, 8, 15], 0))
print(add_up(1..1000000, 0))
```

```sortie
27
500000500000
```

The step-by-step calculation shows the difference: nothing waits
anymore, the total moves forward on each call.

```text
add_up([1, 2, 3], 0)
  → add_up([2, 3], 1)
  → add_up([3], 3)
  → add_up([], 6)
  → 6
```

## Tail calls

Why does the accumulator work so far? Because the call
`add_up(rest, total + x)` is the **very last thing** the function does.
It is a **tail call**: Rythme has nothing to remember, and it can repeat
this call millions of times without filling the stack.

| Code | Tail call? | Why |
|------|------------|-----|
| `x + add_up(rest)` | no | `x + ...` is still to be done after the call |
| `n * factorial(n - 1)` | no | the multiplication is still to be done |
| `add_up(rest, total + x)` | yes | nothing happens after the call |
| `if n == 0 then 1 else count_down(n - 1)` | yes | the call is the result of a branch |

So that the user does not have to give the starting 0, hide the
accumulator in a small function:

```rythme
fn add_up_from([], total) = total
fn add_up_from([x, ..rest], total) = add_up_from(rest, total + x)
fn add_up(items) = add_up_from(items, 0)

print(add_up([1, 2, 3]))
```

```sortie
6
```

## When to prefer map, filter and fold

Many recursions follow the same model. Rythme has already written them
for you, with no size limit:

| Recursion that... | Ready-made function |
|-------------------|---------------------|
| transforms each element | `map(list, f)` |
| keeps some elements | `filter(list, f)` |
| combines everything into one value | `fold(list, start, f)` |
| adds up | `sum(list)` |

```rythme
print(map([1, 2, 3], fn(x) = x * 2))
print(filter([3, -1, 4, -5], fn(x) = x > 0))
print(fold(1..1000000, 0, fn(total, x) = total + x), sum(1..1000000))
```

```sortie
[2, 4, 6]
[3, 4]
500000500000 500000500000
```

Prefer these functions when they fit: they are shorter and say right away
what they do. Keep recursion for the rest: a calculation that stops in
the middle, a tree structure, a mathematical definition.

## Common mistakes

### Forgetting the base case

```rythme norun
fn factorial(n) = n * factorial(n - 1)
print(factorial(5))
```

```text
error[E014]: too many nested calls (962627 calls in progress)
 --> main.ry:1:23
  |
1 | fn factorial(n) = n * factorial(n - 1)
  |                       ^^^^^^^^^^^^^^^^ this call goes too deep
  |
  = help: a recursive function that never stops? Check that it always reaches a base case.
      Otherwise, make the recursive call the very last thing the function does
      (tail call), for example with an accumulator:
        fn sum([], acc) = acc
        fn sum([x, ..rest], acc) = sum(rest, acc + x)
      or use `fold`, `map`, `filter`, which have no limit.
  = call trace (most recent first):
      in `factorial` (×40) called at main.ry:1:23
      … and 962587 more calls
  = more about this error: `rythme explain E014`
```

Without a base case, `factorial` goes down to 4, 3, ..., 0, -1, -2...
forever. Add `fn factorial(0) = 1` **before** the general line. (The
number of calls in progress depends on the machine.)

### Putting the base case last

```rythme norun
fn factorial(n) = n * factorial(n - 1)
fn factorial(0) = 1
print(factorial(5))
```

```text
warning[W004]: this definition of `factorial` can never be used
 --> main.ry:2:1
  |
2 | fn factorial(0) = 1
  | ^^^^^^^^^^^^^^^
  |
1 | fn factorial(n) = n * factorial(n - 1)
  | --------------- this definition already accepts every argument
  |
  = help: definitions are tried in order: put the most general one last
```

Then the program stops with the same E014 error. `n` accepts every
number, 0 included: the `0` case is never reached. Swap the two lines.

### Forgetting the two dots before the rest

```rythme norun
fn add_up([]) = 0
fn add_up([x, rest]) = x + add_up(rest)
print(add_up([1, 2, 3]))
```

```text
warning[W002]: `add_up` does not handle every possible argument
 --> main.ry:1:4
  |
1 | fn add_up([]) = 0
  |    ^^^^^^
  |
  = note: not handled: `[_, _, _, ..]`
  = help: add a definition for it, like `fn add_up([_, _, _, ..]) = ...`

error[E006]: no definition of `add_up` accepts these arguments: add_up([1, 2, 3])
 --> main.ry:3:7
  |
3 | print(add_up([1, 2, 3]))
  |       ^^^^^^^^^^^^^^^^^ called here
  |
1 | fn add_up([]) = 0
  | ------------- does not match
  |
2 | fn add_up([x, rest]) = x + add_up(rest)
  | -------------------- does not match
  |
  = help: add a definition that handles this case, for example one that ends with a plain name or `_`
  = more about this error: `rythme explain E006`
```

`[x, rest]` means "a list of exactly two elements". For "the first one,
then all the rest", write `[x, ..rest]`.

## Test yourself

**Question 1:** what are the two parts of every recursive function?

> **Answer**
>
> The base case, which answers without calling itself, and the recursive
> case, which calls itself on a smaller problem.

**Question 2:** is this call a tail call?

```rythme norun
fn count_items([]) = 0
fn count_items([_, ..rest]) = 1 + count_items(rest)
```

> **Answer**
>
> No: after the call `count_items(rest)`, 1 still has to be added. With an
> accumulator, `fn count_items([_, ..rest], n) = count_items(rest, n + 1)`
> is a tail call.

**Question 3:** what is `f([1, 2, 3])` with these definitions?

```rythme norun
fn f([]) = []
fn f([x, ..rest]) = [..f(rest), x]
```

> **Answer**
>
> `[3, 2, 1]`: each element is put back at the end, so the list is
> reversed (like `reverse`).

## Summary

- Rythme repeats with recursion: a function that calls itself.
- You need a base case, written first, and a recursive case that gets
  closer to the base case.
- `[x, ..rest]` splits a list into its first element and the rest; `[]`
  is the base case.
- An accumulator makes the call a tail call: the function can then call
  itself millions of times.
- When `map`, `filter`, `fold` or `sum` fit, prefer them.
- `rythme steps` shows the calculation step by step.

Next: [your own types and match](types.md)
