# Thinking functionally

This page explains the Rythme way of thinking: values that do not change, chains of functions with `|>` or with dots, Ruby-style blocks, and recursion.

> **Goals:**
>
> - you will know why a Rythme function never changes what it receives;
> - you will know how to chain steps with `|>` or with dots;
> - you will know how to give a block `{ |x| ... }` or `do |x| ... end` to a function;
> - you will know how to replace each kind of loop with `map`, `filter`, `fold`, `find`...;
> - you will know how to write a recursive function with an accumulator.

## Values that do not change

A Rythme function never changes what it receives. It gives a **new**
value:

```rythme
numbers = [3, 1, 2]
in_order = sort(numbers)
print(numbers, in_order)
```

```sortie
[3, 1, 2] [1, 2, 3]
```

This is easier to follow: a value you have named keeps its meaning until
the end of the program. And a function always gives the same result for
the same arguments, like a calculator key: `√9` is 3 today, tomorrow, and
in any order.

This is called **functional programming**: you build a program by putting
functions together, instead of changing values step by step.

## Chaining with |>

`|>` reads "then". `x |> f` means `f(x)`. And `x |> f(a)` means
`f(x, a)`: the value becomes the **first argument**. You then read the
calculation in the order it happens:

```rythme
result = [1, 2, 3, 4, 5, 6]
  |> filter(is_even)
  |> map(fn(x) = x * x)
  |> sum
print(result)
```

```sortie
56
```

Step by step: `[1, 2, 3, 4, 5, 6]`, then we keep the even numbers
`[2, 4, 6]`, then we square them `[4, 16, 36]`, then we add them up: 56.
Without `|>`, you would have to read backwards: `sum(map(filter(...), ...))`.

## Chaining with dots

The same calculation, the Ruby way, with method-style calls. Each line
that starts with `.` continues the previous one:

```rythme
result = [1, 2, 3, 4, 5, 6]
  .select(even?)
  .map(fn(x) = x * x)
  .sum
print(result)
```

```sortie
56
```

`select` and `even?` are the Ruby names of `filter` and `is_even`. Pick
the style you prefer, and keep it throughout your program.

## Ruby-style blocks

Instead of writing `fn(x) = ...`, you can give a **block** at the end of
a call. There are two forms:

- `do |x| ... end`, often on several lines;
- `{ |x| ... }`, often on one line.

Between the bars `|x|`, you name the element received.

```rythme
squares = [1, 2, 3].map { |x| x * x }
print(squares)

[1, 2, 3].each do |x|
  doubled = x * 2
  puts "#{x} times 2 = #{doubled}"
end
```

```sortie
[1, 4, 9]
1 times 2 = 2
2 times 2 = 4
3 times 2 = 6
```

A block is exactly an anonymous function: `xs.map { |x| x * x }` is
`map(xs, fn(x) = x * x)`.

## map, filter and reject

- `map` transforms each element;
- `filter` (or `select`) keeps those for which the function answers `true`;
- `reject` removes those for which the function answers `true`.

```rythme
words = ["bear", "elephant", "cat", "giraffe"]
print(words.map { |w| w.upper })
print(words.filter { |w| len(w) > 3 })
print(words.reject { |w| len(w) > 3 })
```

```sortie
["BEAR", "ELEPHANT", "CAT", "GIRAFFE"]
["bear", "elephant", "giraffe"]
["cat"]
```

Think of a colander: `filter` keeps what stays in the colander, `reject`
keeps what goes through.

## fold and reduce

`fold(list, start, f)` goes through the list while keeping an
**accumulator**, a value that grows with each element. `f` receives the
accumulator and the element, and gives the new accumulator. `reduce` is
its Ruby name:

```rythme
total = fold([1, 2, 3, 4], 0, fn(acc, x) = acc + x)
longest = ["kiwi", "banana", "fig"].reduce("") do |best, w|
  if len(w) > len(best) then w else best
end
print(total, longest)
```

```sortie
10 banana
```

For `total`, the accumulator is 0 at the start, then 0 + 1 = 1, then
1 + 2 = 3, then 3 + 3 = 6, then 6 + 4 = 10. `scan` shows these steps:

```rythme
print(scan([1, 2, 3, 4], 0, fn(acc, x) = acc + x))
```

```sortie
[1, 3, 6, 10]
```

## Replacing loops

| In Python or Ruby | In Rythme |
|-------------------|-----------|
| a loop that prints | `each` |
| a loop that fills a list | `map` |
| a loop with an `if` that keeps | `filter`, `reject` |
| a loop that adds up | `sum`, `fold` |
| a loop that counts | `count` |
| a loop that searches | `find`, `any`, `all` |
| `break` | `take_while` |
| `while` | a recursive function |

```rythme
numbers = [4, 8, 15, 16, 23, 42]
print(find(numbers, is_odd), any(numbers, fn(x) = x > 40), all(numbers, fn(x) = x > 0))
print(take_while(numbers, is_even), count(numbers, is_even))
```

```sortie
Some(15) true true
[4, 8] 4
```

- `find` gives the first element that matches, in an `Option`;
- `any` answers `true` if **at least one** element matches, `all` if
  **all** of them match;
- `take_while` takes the elements as long as they match, then stops;
- `count` counts those that match.

## Repeating with an accumulator

A function can call itself
([recursion](../debuter/glossaire.md#recursion)). For it to repeat
**millions of times**, the call to itself must be the very last thing it
does. The trick: you keep the partial result in an extra parameter, the
**accumulator**.

```rythme
fn add_up([], acc) = acc
fn add_up([x, ..rest], acc) = add_up(rest, acc + x)

print(add_up(1..1000000, 0))
```

```sortie
500000500000
```

Here, `add_up` receives the list and the total computed so far.
`[x, ..rest]` separates the first element from the rest. When the list is
empty, the total is the answer.

> **Tip:** when a list function exists (`sum`, `fold`, `map`), prefer it:
> it has no limit and it reads better.

## A while loop

Here is the Collatz sequence: as long as `n` is not 1, you divide it by 2
if it is even, otherwise you compute `3 * n + 1`. You count the steps:

```rythme
fn collatz(1, steps) = steps
fn collatz(n, steps) =
  if n % 2 == 0 then collatz(n // 2, steps + 1)
  else collatz(3 * n + 1, steps + 1)

print(collatz(27, 0))
```

```sortie
111
```

The "while" becomes: "if you have not arrived, start again with the next
value". The case `collatz(1, steps)` is the stopping condition.

## Common mistakes

### Forgetting fn in map

```rythme norun
print([1, 2, 3] |> map(x * 2))
```

```text
error[E003]: unknown name `x`
 --> main.ry:1:24
  |
1 | print([1, 2, 3] |> map(x * 2))
  |                        ^ not defined
  |
  = help: define it before using it: `x = ...` or `fn x(...) = ...`
  = more about this error: `rythme explain E003`
```

`map` expects a **function**, not a calculation. Write
`map(fn(x) = x * 2)`, or `[1, 2, 3].map { |x| x * 2 }`.

### Putting the arguments of fold in the wrong order

```rythme norun
total = fold([1, 2, 3], fn(acc, x) = acc + x, 0)
print(total)
```

```text
error[E005]: `fold` expects a function as argument 3, but got Int 0
 --> main.ry:1:47
  |
1 | total = fold([1, 2, 3], fn(acc, x) = acc + x, 0)
  |                                               ^ wrong kind of value
  |
  = help: usage: `fold(list, start, f)`
  = more about this error: `rythme explain E005`
```

The order is: the list, the starting value, then the function.
`fold([1, 2, 3], 0, fn(acc, x) = acc + x)`.

### A filter that does not answer true or false

```rythme norun
print([1, 2, 3].filter { |x| x % 2 })
```

```text
error[E005]: the function given to `filter` must return true or false, but it returned a value of type Int
 --> main.ry:1:24
  |
1 | print([1, 2, 3].filter { |x| x % 2 })
  |                        ^^^^^^^^^^^^^ returns a value of type Int
  |
  = help: usage: `filter(list, f)`
  = more about this error: `rythme explain E005`
```

In Python, `1` counts as true. Not in Rythme: ask a real question,
`x % 2 == 1`.

### Calculating right after a chain

```rythme norun
print([1, 2, 3] |> sum + 1)
```

```text
error[E001]: `+` cannot follow a `|>` step directly
 --> main.ry:1:24
  |
1 | print([1, 2, 3] |> sum + 1)
  |                        ^
  |
  = help: `|>` comes after the calculations: put the pipeline in parentheses,
      like `(x |> f) + 1`, or continue it with a function: `x |> f |> fn(v) = v + 1`
  = more about this error: `rythme explain E001`
```

Put the chain in parentheses: `([1, 2, 3] |> sum) + 1`.

## Test yourself

**Question 1:** rewrite `upper(trim("  hello "))` with `|>`.

> **Answer**
>
> `"  hello " |> trim |> upper`. You read it in order: first `trim`, then
> `upper`.

**Question 2:** what is `fold([2, 3, 4], 1, fn(acc, x) = acc * x)`?

> **Answer**
>
> `24`: 1 × 2 = 2, then 2 × 3 = 6, then 6 × 4 = 24.

**Question 3:** which function replaces a loop that looks for the first
negative number in a list?

> **Answer**
>
> `find(numbers, fn(x) = x < 0)`. It gives `Some(number)`, or `None` if
> there is none.

## Summary

- A function never changes what it receives: it gives a new value.
- `x |> f(a)` means `f(x, a)`: you read the calculation in order.
- A block `{ |x| ... }` or `do |x| ... end` is a function without a name.
- `map`, `filter`, `reject`, `fold`, `find` and `count` replace loops.
- To repeat "while", a recursive function, with an accumulator if
  possible.

Next: [recursion](recursion.md)
