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

Line by line:

- `def greet` starts the recipe and gives it a name: `greet`;
- `puts "Hello!"` is the content of the recipe. For now, nothing is printed;
- `end` closes the recipe;
- the first `greet` runs the method: "Hello!" is printed. The second one does exactly
  the same thing again.

Writing `def` does not run anything: it saves the recipe. Writing `greet` **calls** the
method, that is, runs it.

> **Note:** in other languages, this is called a [function](../debuter/glossaire.md#function).
> In Ruby, everything is an object, and every function belongs to an object: so it is
> called a method.

## Parameters

A method that always does exactly the same thing is not very useful. To greet Alice and
then Bob, you don't want to write two methods: you want a single recipe, with one
ingredient that changes each time.

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

Line by line:

- `def greet(first_name)` creates a method with one empty slot, called `first_name`;
- `greet("Alice")` calls the method: `first_name` receives `"Alice"`, and the sentence
  is printed with that name;
- `greet "Bob"` calls it a second time. This time, `first_name` is `"Bob"`.

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

**Your turn:** write a method `introduce(first_name, age)` that prints a sentence like
"My name is Léa and I am 12 years old.". Call it for Léa (12 years old), then for Tom
(9 years old).

> **Solution**
>
> ```ruby
> def introduce(first_name, age)
>   puts "My name is #{first_name} and I am #{age} years old."
> end
>
> introduce("Léa", 12)
> introduce("Tom", 9)
> ```
>
> ```sortie
> My name is Léa and I am 12 years old.
> My name is Tom and I am 9 years old.
> ```

## Implicit return

Often, a method computes a result that you need afterwards, like a calculator that gives
you a total. **Returning** a value means handing that result back to the code that
called the method.

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

Line by line:

- `def square(x)` saves the method. Its last line, `x * x`, gives the result;
- `puts square(4)`: `x` is 4, the method returns 16, and `puts` prints it;
- `result = square(3) + 1`: the method returns 9, Ruby adds 1 and stores 10 in
  `result`;
- `puts result` prints 10.

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

**Your turn:** write a method `perimeter(width, height)` that returns the perimeter of a
rectangle (twice the width plus twice the height), without writing `return`. Print
`perimeter(3, 4)`.

> **Solution**
>
> ```ruby
> def perimeter(width, height)
>   2 * width + 2 * height
> end
>
> puts perimeter(3, 4)
> ```
>
> ```sortie
> 14
> ```

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

Some settings rarely change. At a café, if you don't say anything else, you get a
regular coffee. A default value does the same: it saves you from repeating everything
at each call.

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

Line by line:

- `greeting = "Hello"`: without a second argument, `greeting` will be `"Hello"`;
- `greet("Alice")` gives only one argument: `first_name` is `"Alice"`, and `greeting`
  takes its default value;
- `greet("Bob", "Hi")` gives both: `"Hi"` replaces the default value;
- the method returns the sentence, and `puts` prints it.

**Your turn:** write a method `total_price(price, quantity = 1)` that returns the price
multiplied by the quantity. Print `total_price(5)`, then `total_price(5, 3)`.

> **Solution**
>
> ```ruby
> def total_price(price, quantity = 1)
>   price * quantity
> end
>
> puts total_price(5)
> puts total_price(5, 3)
> ```
>
> ```sortie
> 5
> 15
> ```

## Keyword arguments

When a method has several parameters, you quickly forget which one comes first. Giving
a name to each argument is like filling in a form: each box has a label.

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

> **Note:** this part can wait. Read it once the basics feel solid.

Sometimes, you don't know in advance how many values you will receive, like the items
on a shopping receipt: two one day, ten the next.

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

A program quickly holds dozens of variables. If every method could touch all of them, a
mistake in one place would break something somewhere else. Scope prevents this mix-up.

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

Line by line:

- Ruby saves the method, then stores 0.2 in `tax_rate`;
- `price_with_tax(100, tax_rate)` gives 100 to `price` and 0.2 to `rate`;
- inside the method, `total` is `100 * 1.2`, that is 120.0. `total.round(2)` rounds it
  to two digits after the decimal point: this is the returned value, which `puts`
  prints;
- after the call, `total` is gone: `defined?(total)` answers `nil`.

> **Tip:** a **constant** (`TAX_RATE = 0.2`), on the other hand, is visible everywhere, even
> inside a method.

## Names ending in ? and !

These two signs warn the person reading the code, like a label on a jar: you know what
is inside before you open it.

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

> **Note:** this part can wait. Read it once the basics feel solid.

Some tasks repeat themselves in a smaller size. Counting down from 3 means saying 3,
then counting down from 2.

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

Sometimes, a method knows **when** to act, but not **what** to do. Imagine someone who
says "I'll do it twice": you are the one who chooses what they do.

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

Line by line:

- `def twice` defines a method with no parameter, which contains two `yield`;
- `twice { puts "Hip hip hooray!" }` calls the method and gives it the block written
  between `{ }`;
- the first `yield` runs the block: the sentence is printed. The second `yield` runs it
  a second time.

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

**Your turn:** write a method `three_times` that runs the block it receives three times.
Call it with a block that prints "Well done!".

> **Solution**
>
> ```ruby
> def three_times
>   yield
>   yield
>   yield
> end
>
> three_times { puts "Well done!" }
> ```
>
> ```sortie
> Well done!
> Well done!
> Well done!
> ```

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
