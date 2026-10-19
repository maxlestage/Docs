# Methods

This page explains how to write methods in Ruby with `def`, their parameters and their return value.

> **Goals:**
>
> - you will know how to write a method with `def` and call it;
> - you will know how to give parameters to a method, with or without a default value;
> - you will know how to return a value, with or without `return`;
> - you will know which variables a method can see;
> - you will know how to pass a block to a method with `yield`.

## Defining a method

A [method](../debuter/glossaire.md#method) is a named piece of code that you
can reuse. It is like a recipe: you write it once, then you
follow it as many times as you want. It starts with `def` (*define*)
and ends with `end`.

```ruby
def greet
  puts "Hello!"
end

greet
greet
```

```sortie
Hello!
Hello!
```

Writing `def` does not run anything: it saves the recipe. Writing `greet` **calls** the
method, that is, runs it.

> **Note:** in other languages, this is called a [function](../debuter/glossaire.md#function).
> In Ruby, everything is an object, and every function belongs to an object: so it is
> called a method.

## Parameters

[Parameters](../debuter/glossaire.md#parameter) are the values the method
receives. They are empty slots, filled in at the moment of the call. The values given at
the call are called [arguments](../debuter/glossaire.md#argument).

```ruby
def greet(first_name)
  puts "Hello, #{first_name}!"
end

greet("Alice")
greet "Bob"
```

```sortie
Hello, Alice!
Hello, Bob!
```

Here, `first_name` is the parameter; `"Alice"` and `"Bob"` are the arguments.

> **Note:** parentheses are optional when calling. Keep them when there are
> several arguments: it is easier to read.

Several parameters are separated by commas. The arguments are given in the
same order.

```ruby
def area(width, height)
  width * height
end

puts area(3, 4)
```

```sortie
12
```

## Implicit return

A method automatically returns the value of its **last expression**. The word
`return` is not needed in that case.

```ruby
def square(x)
  x * x
end

puts square(4)
result = square(3) + 1
puts result
```

```sortie
16
10
```

The returned value replaces the call: `square(3) + 1` becomes `9 + 1`.

`return` is mostly used to leave the method early:

```ruby
def divide(a, b)
  return "division impossible" if b.zero?
  a / b
end

puts divide(10, 2)
puts divide(10, 0)
```

```sortie
5
division impossible
```

### Print or return?

`puts` **prints** a value on the screen. Returning a value **gives** it to the code that
called the method, which can use it. A method that ends with `puts` returns `nil`.

```ruby
def print_double(x)
  puts x * 2
end

def double(x)
  x * 2
end

p print_double(5)
p double(5)
```

```sortie
10
nil
10
```

> **Remember:** prefer methods that return a value. You can then choose
> to print it, store it or reuse it.

### The one-line method

For a very short method, write `def name(parameters) = expression`.

```ruby
def triple(x) = x * 3

puts triple(7)
```

```sortie
21
```

### Returning several values

Separate the values with commas. Ruby returns them in an array, which you can
split into several variables.

```ruby
def min_max(numbers)
  return numbers.min, numbers.max
end

smallest, largest = min_max([4, 9, 1])
puts smallest
puts largest
```

```sortie
1
9
```

## Default values

A parameter can have a default value. It is used if the argument is missing.

```ruby
def greet(first_name, greeting = "Hello")
  "#{greeting}, #{first_name}!"
end

puts greet("Alice")
puts greet("Bob", "Hi")
```

```sortie
Hello, Alice!
Hi, Bob!
```

## Keyword arguments

With `name:`, an argument is given by its name. The order no longer matters.

```ruby
def order(product:, quantity: 1)
  "#{quantity} x #{product}"
end

puts order(product: "coffee")
puts order(quantity: 3, product: "tea")
```

```sortie
1 x coffee
3 x tea
```

- `product:` is **required**: it has no default value.
- `quantity: 1` is optional: it is `1` if you don't give it.

> **Tip:** keyword arguments make calls very clear, especially with
> booleans: `send_message(urgent: true)` is understood right away.

## A variable number of arguments

With `*`, a parameter receives all the remaining arguments, stored in an array.

```ruby
def sum(*numbers)
  numbers.sum
end

puts sum(1, 2)
puts sum(1, 2, 3, 4)
puts sum
```

```sortie
3
10
0
```

## Variable scope

The [scope](../debuter/glossaire.md#scope) of a variable is the area of the code where it
exists. A method is a closed room: it does not see the variables created
outside. It only sees its parameters and its own variables.

```ruby
def price_with_tax(price, rate)
  total = price * (1 + rate)
  total.round(2)
end

tax_rate = 0.2
puts price_with_tax(100, tax_rate)
p defined?(total)
```

```sortie
120.0
nil
```

- To give a value to a method, pass it as an argument.
- `total` only exists inside the method: outside, `defined?(total)` answers `nil`.

> **Tip:** a **constant** (`TAX_RATE = 0.2`), on the other hand, is visible everywhere, even
> inside a method.

## Names ending in ? and !

By convention:

- a method that ends with `?` answers true or false;
- a method that ends with `!` changes the object or is "dangerous".

```ruby
def adult?(age)
  age >= 18
end

puts adult?(20)

name = "alice"
name.capitalize!
puts name
```

```sortie
true
Alice
```

> **Warning:** `capitalize` returns a new string. `capitalize!` changes the
> string itself.

## A method that calls itself

A method can call itself: this is
[recursion](../debuter/glossaire.md#recursion). It needs a **base case**, otherwise it
never stops.

```ruby
def countdown(n)
  return puts("Go!") if n.zero?
  puts n
  countdown(n - 1)
end

countdown(3)
```

```sortie
3
2
1
Go!
```

Here, the base case is `n.zero?`. Each call brings `n` closer to zero.

## Passing a block with yield

A method can receive a **block** of code, written between `{ }` after the call.
`yield` runs this block.

```ruby
def twice
  yield
  yield
end

twice { puts "Hip hip hooray!" }
```

```sortie
Hip hip hooray!
Hip hip hooray!
```

`yield` can also pass a value to the block:

```ruby
def with_bonus(score)
  yield(score + 10)
end

with_bonus(5) { |total| puts "Total: #{total}" }
```

```sortie
Total: 15
```

Blocks are explained in detail in [the page about blocks](blocs.md).

## Common mistakes

### Forgetting an argument

```ruby norun
def area(w, h)
  w * h
end
puts area(3)
```

```text
main.rb:1:in `area': wrong number of arguments (given 1, expected 2) (ArgumentError)
	from main.rb:4:in `<main>'
```

The method expects 2 arguments (*expected 2*), it receives 1 (*given 1*). Give them
all: `area(3, 4)`. Or give the parameter a default value.

### Calling the method before defining it

```ruby norun
greet("Alice")
def greet(name)
  puts "Hello, #{name}!"
end
```

```text
main.rb:1:in `<main>': undefined method `greet' for main (NoMethodError)

greet("Alice")
^^^^^
```

Ruby reads the file from top to bottom. On line 1, `greet` does not exist yet.
Put the `def` before the call.

### Using a variable from outside

```ruby norun
rate = 0.2
def price_with_tax(price)
  price * (1 + rate)
end
puts price_with_tax(100)
```

```text
main.rb:3:in `price_with_tax': undefined local variable or method `rate' for main (NameError)

  price * (1 + rate)
               ^^^^
	from main.rb:5:in `<main>'
```

The method does not see `rate`. Pass it as a parameter (`def price_with_tax(price, rate)`),
or make it a constant (`RATE = 0.2`).

### Forgetting a required keyword argument

```ruby norun
def order(product:, quantity: 1)
  "#{quantity} x #{product}"
end
puts order(quantity: 2)
```

```text
main.rb:1:in `order': missing keyword: :product (ArgumentError)
	from main.rb:4:in `<main>'
```

`product:` is missing. Write `order(product: "tea", quantity: 2)`.

## Test yourself

**Question 1:** what does this code print?

```ruby norun
def f(x)
  x + 1
  x * 2
end
puts f(5)
```

> **Answer**
>
> `10`. The method returns its **last** expression, `x * 2`. The line `x + 1` is
> calculated, then forgotten.

**Question 2:** write a method `average(a, b)` that returns the average of two
whole numbers, with decimals if needed.

> **Answer**
>
> ```ruby
> def average(a, b)
>   (a + b) / 2.0
> end
>
> puts average(3, 4)
> ```
>
> ```sortie
> 3.5
> ```
>
> The `2.0` avoids whole division.

**Question 3:** what is the difference between a parameter and an argument?

> **Answer**
>
> The parameter is the name written in `def` (`first_name`). The argument is the value given
> at the call (`"Alice"`).

## Summary

- A method starts with `def` and ends with `end`; you call it by its name.
- It returns the value of its last line; `return` is used to leave early.
- A parameter can have a default value, or be a keyword (`product:`).
- A method does not see variables from outside: pass them as arguments.
- A name ending in `?` answers true or false; a name ending in `!` changes the object.
- `yield` runs the block given to the method.

Next: [working with text](texte.md). Compare with
[functions in Rust](../rust/fonctions.md).
