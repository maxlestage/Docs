# Procs, lambdas and methods

This page shows how to store code in a variable with a proc or a lambda, pass it to a method, return it and combine it.

> **Goals:**
>
> - you will know the difference between a block, a proc and a lambda;
> - you will know how to create a lambda with `->(x) { }` and call it;
> - you will know how to go from one to the other with `&` and `method(:name)`;
> - you will know that a lambda keeps the variables around it;
> - you will know how to write a method that returns a function, and how to compose functions.

## Block, proc, lambda: three cousins

You already know [blocks](blocs.md): code given to a method, between `{ }`. A block has
one limit: it only exists for the duration of the call. You cannot store it in a
variable to use it again.

A **proc** and a **lambda** are blocks turned into objects. You can store them, pass
them, return them. Think of a recipe: the block is a recipe said out loud, the proc or
the lambda is the same recipe **written on a card** that you can give to someone.

```ruby
greet_proc = proc { |name| "Hello #{name}" }
greet_lambda = ->(name) { "Hello #{name}" }

puts greet_proc.call("Alice")
puts greet_lambda.call("Bob")
puts greet_proc.class
puts greet_lambda.lambda?
```

```sortie
Hello Alice
Hello Bob
Proc
true
```

A lambda is a kind of proc (its class is also `Proc`), a bit stricter.

> **Remember:** when you start, use lambdas. They behave like
> [methods](../debuter/glossaire.md#method), so there are no surprises.

## Writing and calling a lambda

The usual form is the "arrow": `->(parameters) { code }`. The result is the last value
computed, like for a method.

```ruby
square = ->(x) { x * x }
area = ->(width, height) { width * height }
hello = -> { "Hello!" }

puts square.call(4)
puts square.(5)
puts square[6]
puts area.call(3, 4)
puts hello.call
```

```sortie
16
25
36
12
Hello!
```

The three forms `call(…)`, `.(…)` and `[…]` do the same thing. `call` is the most
readable.

On several lines, write `do ... end` or braces over several lines:

```ruby
price_with_tax = lambda do |net|
  tax = net * 0.2
  net + tax
end

puts price_with_tax.call(50)
```

```sortie
60.0
```

## The differences between proc and lambda

### The number of arguments

A lambda checks the number of arguments, like a method. A proc is lenient: it fills in
with `nil` or ignores the extra ones. `arity` gives the expected number.

```ruby
l = ->(a, b) { "#{a} and #{b}" }
pr = proc { |a, b| "#{a.inspect} and #{b.inspect}" }

puts l.arity
puts pr.call(1)
puts pr.call(1, 2, 3)

begin
  l.call(1)
rescue ArgumentError => e
  puts "Error: #{e.message}"
end
```

```sortie
2
1 and nil
1 and 2
Error: wrong number of arguments (given 1, expected 2)
```

### What return means

In a lambda, `return` only leaves the lambda. In a proc, `return` leaves **the method
around it**: this is often a surprise.

```ruby
def with_lambda
  l = -> { return 1 }
  l.call
  "end of the method"
end

def with_proc
  pr = proc { return 1 }
  pr.call
  "end of the method"
end

puts with_lambda
puts with_proc
```

```sortie
end of the method
1
```

The proc made `with_proc` stop right away: the last line was never read.

| | Block | Proc | Lambda |
| --- | --- | --- | --- |
| Stored in a variable | no | yes | yes |
| Checks the number of arguments | no | no | yes |
| `return` leaves… | the method around | the method around | the lambda |

## Going from one to the other with &

The `&` converts in both directions:

- in front of an argument, `&my_lambda` gives the lambda to the method **as a block**;
- in front of a parameter, `&block` **gets** the block received, as a proc.

```ruby
double = ->(x) { x * 2 }
p [1, 2, 3].map(&double)

def keep(list, &rule)
  puts "The rule is a #{rule.class}"
  list.select(&rule)
end

p keep([5, 12, 8, 20]) { |n| n > 10 }
```

```sortie
[2, 4, 6]
The rule is a Proc
[12, 20]
```

You already know `map(&:upcase)`: `&` turns the
[symbol](../debuter/glossaire.md#symbol) `:upcase` into a proc that calls `upcase`.

## Turning a method into an object

`method(:name)` grabs an existing method and makes it an object that you can pass like
a lambda.

```ruby
def shout(text)
  text.upcase + "!"
end

shouter = method(:shout)
puts shouter.call("hi")
p ["yes", "no"].map(&shouter)
p ["3", "14"].map(&method(:Integer))
```

```sortie
HI!
["YES!", "NO!"]
[3, 14]
```

## A lambda keeps its variables

A lambda, like a block, is a [closure](../debuter/glossaire.md#closure): it takes along
the variables visible where it is created. It even sees their changes later.

```ruby
counter = 0
increment = -> { counter += 1 }

increment.call
increment.call
increment.call
puts counter
```

```sortie
3
```

The lambda changes the real variable `counter`, not a copy.

## A method that returns a function

A method can build and return a lambda. Each lambda keeps **its own** value of the
parameter: it is a small function factory.

```ruby
def multiplier(factor)
  ->(x) { x * factor }
end

double = multiplier(2)
triple = multiplier(3)
puts double.call(10)
puts triple.call(10)
p [1, 2, 3].map(&triple)
```

```sortie
20
30
[3, 6, 9]
```

Another useful example: a counter whose variable is hidden, so nobody else can touch
it.

```ruby
def new_counter
  total = 0
  -> { total += 1 }
end

a = new_counter
b = new_counter
a.call
a.call
puts a.call
puts b.call
```

```sortie
3
1
```

## Composing functions with >> and <<

`f >> g` creates a new function: "apply `f`, then `g` to the result". `f << g` does the
opposite: "`g` first, then `f`". It is an assembly line.

```ruby
clean = ->(s) { s.strip }
capitals = ->(s) { s.upcase }
frame = ->(s) { "[#{s}]" }

process = clean >> capitals >> frame
puts process.call("  hello  ")

add_one = ->(x) { x + 1 }
double = ->(x) { x * 2 }
puts (add_one >> double).call(5)
puts (add_one << double).call(5)
```

```sortie
[HELLO]
12
11
```

`(add_one >> double).call(5)` computes (5 + 1) × 2. With `<<`, it is 5 × 2 + 1.

## Common mistakes

### Calling a lambda like a method

```ruby norun
square = ->(x) { x * x }
puts square(3)
```

```text
main.rb:2:in `<main>': undefined method `square' for main (NoMethodError)

puts square(3)
     ^^^^^^
```

`square` is a variable that holds a lambda, not a method. Call it with
`square.call(3)` or `square.(3)`.

### Forgetting the & to pass a lambda

```ruby norun
double = ->(x) { x * 2 }
p [1, 2].map(double)
```

```text
main.rb:2:in `map': wrong number of arguments (given 1, expected 0) (ArgumentError)

p [1, 2].map(double)
             ^^^^^^
	from main.rb:2:in `<main>'
```

`map` expects a block, not an argument. The `&` turns the lambda into a block:
`[1, 2].map(&double)`.

### A return in a proc returned by a method

```ruby norun
def make
  proc { |x| return x * 2 }
end

double = make
puts double.call(5)
```

```text
main.rb:2:in `block in make': unexpected return (LocalJumpError)
	from main.rb:6:in `<main>'
```

The `return` of a proc wants to leave the method `make`, but that method has already
finished. Use a lambda (`->(x) { x * 2 }`): its `return` stays inside it.

## Test yourself

**Question 1:** write a lambda `plus` that adds two numbers, then call it with 2 and 3.

> **Answer**
>
> ```ruby
> plus = ->(a, b) { a + b }
> puts plus.call(2, 3)
> ```
>
> ```sortie
> 5
> ```

**Question 2:** what happens if you call `->(a, b) { a + b }` with a single argument?
And `proc { |a, b| a }`?

> **Answer**
>
> The lambda raises an `ArgumentError` (wrong number of arguments). The proc says
> nothing: `b` is simply `nil`.

**Question 3:** with `add = ->(n) { ->(x) { x + n } }`, what is
`add.call(10).call(5)`?

> **Answer**
>
> `15`: `add.call(10)` returns a lambda that adds 10, then called with 5.
>
> ```ruby
> add = ->(n) { ->(x) { x + n } }
> puts add.call(10).call(5)
> ```
>
> ```sortie
> 15
> ```

## Summary

- A block goes with a call; a proc or a lambda is a block stored in an object.
- `->(x) { ... }` creates a lambda; `call`, `.()` or `[]` run it.
- A lambda checks its arguments and its `return` stays local: prefer it.
- `&` passes a lambda as a block; `method(:name)` turns a method into an object.
- A lambda keeps the variables around it: you can build functions.
- `f >> g` and `f << g` chain functions.

Next: [enumerators and laziness](enumerateurs.md).
