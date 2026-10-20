# Enumerators and laziness

This page shows what an enumerator is, how to walk through a collection at your own pace, in batches or in pairs, and how to work with infinite sequences thanks to `lazy`.

> **Goals:**
>
> - you will know what `each` returns when you give it no block;
> - you will know how to move forward step by step with `next`;
> - you will know how to split a collection with `each_slice`, `each_cons` and `chunk_while`;
> - you will know how to handle an infinite sequence with `lazy`;
> - you will know how to create your own enumerator with `Enumerator.new`.

## each without a block

What happens if you call `each` or `map` **without** a block? Ruby does not crash:
it returns an **enumerator** (`Enumerator`).

```ruby
e = [10, 20, 30].each
p e
puts e.class
```

```sortie
#<Enumerator: [10, 20, 30]:each>
Enumerator
```

An enumerator is a walk-through **on hold**. Think of a bookmark in a book: it knows
where you are and can give you the next page when you ask for it. It is an
[iterator](../debuter/glossaire.md#iterateur) that you can store in a variable.

It knows all the methods of [Enumerable](blocs.md):

```ruby
e = [10, 20, 30].each
p e.map { |x| x + 1 }
p e.to_a
```

```sortie
[11, 21, 31]
[10, 20, 30]
```

## Moving step by step with next

`next` gives the next element, one at a time. When there are none left, it raises
`StopIteration`. `loop` catches this error and stops on its own.

```ruby
light = ["green", "orange", "red"].each
puts light.next
puts light.next
puts light.next

light.rewind
loop do
  puts "Color: #{light.next}"
end
```

```sortie
green
orange
red
Color: green
Color: orange
Color: red
```

`rewind` puts the bookmark back at the start. `peek` looks at the next element without
moving forward.

## Adding the position with with_index

Since an enumerator is an object, you can chain methods onto it. `with_index` adds the
position to any walk-through, and can start at 1.

```ruby
fruits = ["apple", "kiwi", "pear"]

fruits.each.with_index(1) do |fruit, i|
  puts "#{i}. #{fruit}"
end

p fruits.map.with_index { |fruit, i| "#{fruit}#{i}" }
```

```sortie
1. apple
2. kiwi
3. pear
["apple0", "kiwi1", "pear2"]
```

`map.with_index` is not a separate method: it is `map` without a block (an enumerator)
that you ask for `with_index`.

## In batches with each_slice

`each_slice(n)` cuts the collection into batches of `n` elements. The last batch can be
smaller.

```ruby
students = ["Ana", "Ben", "Cleo", "Dan", "Eva"]
students.each_slice(2) { |group| p group }
```

```sortie
["Ana", "Ben"]
["Cleo", "Dan"]
["Eva"]
```

## In windows with each_cons

`each_cons(n)` slides a window of `n` elements, one notch at a time. Handy for comparing
each value with the next one.

```ruby
temperatures = [12, 15, 14, 18]
temperatures.each_cons(2) do |before, after|
  puts "#{before} -> #{after}: #{after - before}"
end
```

```sortie
12 -> 15: 3
15 -> 14: -1
14 -> 18: 4
```

## Grouping what follows with chunk_while

`chunk_while` groups neighboring elements as long as the block answers `true` for each
pair. Here, we group numbers that follow each other.

```ruby
days = [1, 2, 3, 7, 8, 12]
groups = days.chunk_while { |a, b| b == a + 1 }
p groups.to_a
p groups.map { |g| g.size > 1 ? "#{g.first}-#{g.last}" : g.first.to_s }
```

```sortie
[[1, 2, 3], [7, 8], [12]]
["1-3", "7-8", "12"]
```

Its opposite, `slice_when`, cuts when the block answers `true`.

## Repeating with cycle and take

`cycle` repeats a collection endlessly. `take(n)` takes the first `n` elements, which
lets you stop.

```ruby
teams = ["red", "blue"]
players = ["Ana", "Ben", "Cleo", "Dan", "Eva"]

players.zip(teams.cycle.take(players.size)).each do |player, team|
  puts "#{player}: #{team}"
end
```

```sortie
Ana: red
Ben: blue
Cleo: red
Dan: blue
Eva: red
```

## Laziness with lazy

Normally, each step of a chain processes the **whole** collection before moving on to
the next one. With an infinite sequence, that is impossible: `map` would never finish.

`lazy` makes the chain **lazy**: each element goes through all the steps one after the
other, and only when it is needed. It is like a kitchen that cooks a dish to order,
instead of cooking everything in advance.

```ruby
even_squares = (1..)
  .lazy
  .map { |n| n * n }
  .select(&:even?)
  .first(5)
p even_squares
```

```sortie
[4, 16, 36, 64, 100]
```

`(1..)` is infinite, but only the first 10 numbers are computed. `first(5)` (or
`take(5).to_a`) triggers the work and says when to stop.

```ruby
steps = (1..3).lazy.map { |n| puts "computing #{n}"; n * 10 }
puts "Nothing is computed yet"
p steps.first(2)
```

```sortie
Nothing is computed yet
computing 1
computing 2
[10, 20]
```

> **Remember:** a `lazy` chain does nothing until you ask for a result with `first`,
> `to_a`, `each` or `force`.

## Creating your own enumerator with Enumerator.new

`Enumerator.new` builds a sequence on demand. In the block, `y << value` (the
"yielder") **gives** a value, then waits until you ask for the next one.

```ruby
fibonacci = Enumerator.new do |y|
  a, b = 0, 1
  loop do
    y << a
    a, b = b, a + b
  end
end

p fibonacci.take(10)
p fibonacci.lazy.select(&:odd?).first(5)
puts fibonacci.next
puts fibonacci.next
```

```sortie
[0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
[1, 1, 3, 5, 13]
0
1
```

The `loop` is infinite, but that is fine: the block pauses at each `y <<` and only
resumes if you ask for another value.

## Common mistakes

### Calling next after the end

```ruby norun
e = [1, 2].each
e.next
e.next
e.next
```

```text
main.rb:4:in `next': iteration reached an end (StopIteration)
	from main.rb:4:in `<main>'
```

There are only two elements. Use `loop do ... end`, which stops cleanly at the end, or
`rewind` to go back to the start.

### Forgetting lazy on an infinite sequence

```ruby norun
p (1..).map { |n| n * 2 }.first(3)
```

```text
(the program never stops)
```

`map` wants to process **all** the numbers before moving on to `first`. Add `lazy`:
`(1..).lazy.map { |n| n * 2 }.first(3)`.

### Printing a lazy chain instead of its result

```ruby norun
p (1..).lazy.map { |n| n * 2 }.select(&:even?)
```

```text
#<Enumerator::Lazy: #<Enumerator::Lazy: #<Enumerator::Lazy: 1..>:map>:select>
```

No error, but no numbers either: the chain is waiting for you to ask it for something.
End it with `first(n)`.

## Test yourself

**Question 1:** what does `[1, 2, 3].map` return without a block?

> **Answer**
>
> An enumerator, waiting for a block or another method.
>
> ```ruby
> p [1, 2, 3].map
> ```
>
> ```sortie
> #<Enumerator: [1, 2, 3]:map>
> ```

**Question 2:** how do you print `[1, 2, 3, 4, 5, 6]` in rows of 3?

> **Answer**
>
> ```ruby
> [1, 2, 3, 4, 5, 6].each_slice(3) { |row| puts row.join(" ") }
> ```
>
> ```sortie
> 1 2 3
> 4 5 6
> ```

**Question 3:** how do you get the first 4 multiples of 7 greater than 100?

> **Answer**
>
> ```ruby
> p (101..).lazy.select { |n| n % 7 == 0 }.first(4)
> ```
>
> ```sortie
> [105, 112, 119, 126]
> ```

## Summary

- `each`, `map` and their cousins return an `Enumerator` when you give no block.
- `next` moves step by step; `with_index(1)` adds a position.
- `each_slice` cuts into batches, `each_cons` slides a window, `chunk_while` groups what
  follows.
- `lazy` processes elements one by one, on demand: ideal for infinite sequences.
- `Enumerator.new { |y| y << ... }` creates a custom sequence.

Next: [classes and objects](types.md)
