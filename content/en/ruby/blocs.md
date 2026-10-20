# Blocks, iterators and Enumerable

This page explains blocks, iterators such as `map` and `select`, and gives a quick introduction to procs and lambdas.

> **Goals:**
>
> - you will know how to write a block with `{ }` or `do ... end`;
> - you will know how to transform, filter and combine a collection with `map`, `select` and `reduce`;
> - you will know how to ask a collection questions (`any?`, `count`, `max_by`…);
> - you will know that a block sees the variables around it;
> - you will know how to store code in a variable with a lambda.

## Blocks

A [block](../debuter/glossaire.md#bloc) is a piece of code that you give to a method.
The method decides when to run it, and how many times. It is like giving an instruction
to a waiter: "for each plate, add salt". The waiter (the method) goes from plate to
plate; you just wrote the instruction (the block).

```ruby
[1, 2, 3].each { |n| puts n * 2 }
```

```sortie
2
4
6
```

Between the bars `|n|`, you name the value the block receives. This name is a
**block parameter**: it changes on each pass (each
[iteration](../debuter/glossaire.md#iteration)). Choose a meaningful name: `|fruit|`,
`|grade|`…

A method that goes through a collection by calling a block is called an
**iterator**. `each`, `times` and `map` are iterators.

## do...end or braces

There are two ways to write a block:

- `{ ... }` for a short block, on one line;
- `do ... end` for a block of several lines.

```ruby
[1, 2].each { |n| puts n }

[1, 2].each do |n|
  square = n * n
  puts "#{n} squared: #{square}"
end
```

```sortie
1
2
1 squared: 1
2 squared: 4
```

> **Remember:** both forms do the same thing. The choice is a matter of
> readability.

## The Enumerable methods

Arrays, hashes and ranges share the same methods, which come from the `Enumerable`
module ("enumerable": something you can go through one by one). A module is a box of
shared methods: see [modules](modules.md).

> **Remember:** these methods **return** a new result. They do not change the original
> collection.

### map: transform

`map` creates a new array with the result of the block for each element.

```ruby
prices = [10, 20, 30]
p prices.map { |x| x * 2 }
p ["a", "b"].map(&:upcase)
```

```sortie
[20, 40, 60]
["A", "B"]
```

> **Tip:** `map(&:upcase)` is a shortcut for `map { |s| s.upcase }`. It works when the
> block just calls a method with no argument on each element.

### select and reject: filter

`select` keeps the elements for which the block is true. `reject` removes them.

```ruby
numbers = (1..10).to_a
p numbers.select(&:even?)
p numbers.reject { |n| n > 3 }
p numbers.find { |n| n > 4 }
```

```sortie
[2, 4, 6, 8, 10]
[1, 2, 3]
5
```

`find` returns the **first** element for which the block is true (or `nil`).

### reduce: combine

`reduce` combines all the elements into a single value.

```ruby
numbers = [1, 2, 3, 4]
total = numbers.reduce(0) { |sum, n| sum + n }
puts total
```

```sortie
10
```

On each pass, `sum` holds the total computed so far, and `n` the next element. The
`0` is the starting value.

### Asking a collection questions

- `any?`: does at least one element satisfy the block?
- `all?`: do all the elements satisfy it?
- `count`: how many elements satisfy it?
- `min_by` and `max_by`: which element gives the smallest or the largest value?
- `sum`: the sum, possibly of what the block returns.
- `tally`: how many times each value appears.

```ruby
grades = [12, 7, 15, 9]
puts grades.any? { |g| g < 8 }
puts grades.all? { |g| g >= 10 }
puts grades.count { |g| g >= 10 }

words = ["kiwi", "pineapple", "apple"]
puts words.max_by { |w| w.length }
puts words.sum { |w| w.length }
p ["a", "b", "a"].tally
```

```sortie
true
false
2
pineapple
18
{"a"=>2, "b"=>1}
```

### each_with_index

```ruby
%w[monday tuesday].each_with_index do |day, i|
  puts "#{i}: #{day}"
end
```

```sortie
0: monday
1: tuesday
```

> **Note:** `%w[monday tuesday]` is a shortcut for `["monday", "tuesday"]`.

### sort_by: sort

`sort_by` sorts according to the value given by the block.

```ruby
words = ["kiwi", "banana", "raspberry", "apple"]
p words.sort_by { |w| w.length }
p words.sort_by { |w| w.length }.last
```

```sortie
["kiwi", "apple", "banana", "raspberry"]
"raspberry"
```

### group_by: group

`group_by` puts the elements into a hash, according to the value of the block.

```ruby
words = ["ant", "cat", "cow", "ape"]
p words.group_by { |w| w[0] }
```

```sortie
{"a"=>["ant", "ape"], "c"=>["cat", "cow"]}
```

### Chaining methods

Each method returns a value: you can chain them.

```ruby
result = (1..10)
  .select(&:odd?)
  .map { |n| n * n }
  .sum
puts result
```

```sortie
165
```

## A block sees the variables around it

A method does not see the variables from outside (see
[scope](fonctions.md#variable-scope)). A block, however, sees them and can change
them. We say that the block is a [closure](../debuter/glossaire.md#closure): it takes
its surroundings with it.

```ruby
total = 0
[5, 10, 20].each { |price| total += price }
puts total
```

```sortie
35
```

On the other hand, a variable created **inside** the block disappears at the end of the
block.

### Writing a method that takes a block

`yield` runs the given block (see [methods](fonctions.md#passing-a-block-with-yield)).
`block_given?` answers `true` if a block was given.

```ruby
def announce(text)
  if block_given?
    puts yield(text)
  else
    puts text
  end
end

announce("hello")
announce("hello") { |t| t.upcase + "!" }
```

```sortie
hello
HELLO!
```

## Procs and lambdas

A block cannot be stored in a variable. For that, Ruby offers **procs** and
**lambdas**. As a beginner, you only need to be able to recognize them.

```ruby
double = ->(x) { x * 2 }
puts double.call(5)
puts double.call(7)
```

```sortie
10
14
```

`call` runs the lambda with the given value.

A proc is written with `proc { ... }`:

```ruby
say_hi = proc { |name| puts "Hi #{name}" }
say_hi.call("Alice")
```

```sortie
Hi Alice
```

> **Warning:** a lambda checks the number of arguments, a proc does not. When in doubt,
> prefer the lambda.

## Common mistakes

### Forgetting to name the block parameter

```ruby norun
[1, 2, 3].each { puts n * 2 }
```

```text
main.rb:1:in `block in <main>': undefined local variable or method `n' for main (NameError)

[1, 2, 3].each { puts n * 2 }
                      ^
	from main.rb:1:in `each'
	from main.rb:1:in `<main>'
```

The block does not know what `n` is. Declare it between bars: `{ |n| puts n * 2 }`.

### do ... end after p or puts

```ruby norun
p [1, 2].map do |x|
  x * 2
end
```

```text
#<Enumerator: [1, 2]:map>
```

No error, but not the right result! With `do ... end`, the block is given to `p`, not
to `map`. `map` without a block returns a "waiting" object (an `Enumerator`). Store the
result in a variable first, or use braces:

```ruby
p [1, 2].map { |x| x * 2 }
```

```sortie
[2, 4]
```

### Calling a lambda with the wrong number of arguments

```ruby norun
add = ->(a, b) { a + b }
puts add.call(1)
```

```text
main.rb:1:in `block in <main>': wrong number of arguments (given 1, expected 2) (ArgumentError)
	from main.rb:2:in `<main>'
```

A lambda checks its arguments, like a method. Give all of them: `add.call(1, 2)`.

## Test yourself

**Question 1:** what is the difference between `each` and `map`?

> **Answer**
>
> `each` runs the block for each element and returns the original collection.
> `map` returns a **new** array filled with the results of the block.
>
> ```ruby
> p [1, 2].each { |x| x * 10 }
> p [1, 2].map { |x| x * 10 }
> ```
>
> ```sortie
> [1, 2]
> [10, 20]
> ```

**Question 2:** how do you keep only the words with more than 4 letters from
`["cat", "horse", "owl", "giraffe"]`?

> **Answer**
>
> ```ruby
> p ["cat", "horse", "owl", "giraffe"].select { |w| w.length > 4 }
> ```
>
> ```sortie
> ["horse", "giraffe"]
> ```

**Question 3:** what is the value of `[1, 2, 3].reduce(10) { |acc, n| acc + n }`?

> **Answer**
>
> `16`: you start from 10, then add 1, 2 and 3.

## Summary

- A block is code given to a method, between `{ }` or between `do` and `end`.
- `map` transforms, `select` and `reject` filter, `find` looks for the first one.
- `reduce` combines all the elements into a single value.
- `sort_by` sorts and `group_by` groups according to the result of the block.
- A block sees the variables around it; a lambda (`->(x) { ... }`) is a block stored in
  a variable.

Next: [procs, lambdas and methods](procs-lambdas.md)
