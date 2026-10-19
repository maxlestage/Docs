# Handling errors

This page explains how to catch errors with `begin` and `rescue`, how to raise them with `raise` and how to avoid the traps of `nil`.

> **Goals:**
>
> - you will know how to read an error message and find the faulty line;
> - you will know how to catch an error with `begin` and `rescue`;
> - you will know how to run code in every case with `ensure`;
> - you will know how to raise your own errors with `raise`;
> - you will know how to avoid errors on `nil` with `&.`.

## An error stops the program

When Ruby runs into a problem, it raises an **exception**: an object that describes the
[error](../debuter/glossaire.md#error). "Raise" means: set off the alarm.
Without handling, the program stops.

```ruby norun
puts "Start"
puts 10 / 0
puts "End"
```

```text
Start
main.rb:2:in `/': divided by 0 (ZeroDivisionError)
	from main.rb:2:in `<main>'
```

`End` is never printed: the program stops on line 2.

### Reading the message

The message is read in three pieces:

- `main.rb:2`: the file and the line;
- `divided by 0`: the description;
- `(ZeroDivisionError)`: the **class** of the error, its family name.

The `from ...` lines form the **backtrace**: the path the program followed up to the
error, from the deepest method up to the main program (`<main>`). Look for the first
line that mentions **your** file.

### The most common errors

| Class | When? |
|--------|---------|
| `NoMethodError` | unknown method, often called on `nil` |
| `NameError` | unknown variable or constant (typo) |
| `ArgumentError` | wrong number or wrong value of arguments |
| `TypeError` | wrong type, for example `"a" + 1` |
| `ZeroDivisionError` | whole-number division by zero |
| `KeyError` | missing key with `fetch` |
| `Errno::ENOENT` | file not found |

## begin and rescue

`rescue` catches the error. The program carries on instead of stopping. It is a safety
net under a trapeze artist: the fall is caught, the show goes on.

- The risky code goes between `begin` and `rescue`.
- The backup code goes between `rescue` and `end`.

```ruby
begin
  puts 10 / 0
rescue ZeroDivisionError
  puts "You can't divide by zero!"
end
puts "The program carries on."
```

```sortie
You can't divide by zero!
The program carries on.
```

### Reading the message

With `=> e`, you get the exception and its message.

```ruby
begin
  Integer("abc")
rescue ArgumentError => e
  puts "Error: #{e.message}"
end
```

```sortie
Error: invalid value for Integer(): "abc"
```

> **Warning:** catch specific errors (`ArgumentError`, `ZeroDivisionError`…).
> A `rescue` that is too broad can hide real bugs.

### Several rescue and else

You can write several `rescue`, one per error class. The code in `else` runs only if
there was **no** error.

```ruby
def divide(text_a, text_b)
  result = Integer(text_a) / Integer(text_b)
rescue ArgumentError
  puts "That is not a number."
rescue ZeroDivisionError
  puts "Division by zero."
else
  puts "Result: #{result}"
end

divide("10", "2")
divide("ten", "2")
divide("10", "0")
```

```sortie
Result: 5
That is not a number.
Division by zero.
```

> **Note:** `Integer("ten")` raises an error, whereas `"ten".to_i` returns `0` without
> a word. Use `Integer(...)` when invalid input must be spotted.

## ensure

The code in `ensure` runs **always**, whether there is an error or not. It is used to
clean up (closing a file, for example).

```ruby
begin
  puts "Opening"
  raise "oops"
rescue => e
  puts "Problem: #{e.message}"
ensure
  puts "Closing"
end
```

```sortie
Opening
Problem: oops
Closing
```

## rescue in a method

In a method, `begin` is not needed: `rescue` goes directly before `end`. It protects the
whole body of the method.

```ruby
def read_number(text)
  Integer(text)
rescue ArgumentError
  0
end

puts read_number("42")
puts read_number("forty")
```

```sortie
42
0
```

## Raising an error with raise

`raise` sets off an error yourself, for example to refuse a value. Give the class of the
error, then a clear message. Without a class, `raise "message"` raises a
`RuntimeError`.

```ruby
def withdraw(balance, amount)
  raise ArgumentError, "negative amount" if amount < 0
  balance - amount
end

begin
  withdraw(100, -5)
rescue ArgumentError => e
  puts e.message
end
```

```sortie
negative amount
```

## Your own exceptions

An error is an object like any other: its class inherits from an error class
(see [inheritance](types.md)). Create a class that inherits from `StandardError`. The
name often ends in `Error`.

```ruby
class InsufficientFundsError < StandardError
end

begin
  raise InsufficientFundsError, "insufficient funds"
rescue InsufficientFundsError => e
  puts "Refused: #{e.message}"
end
```

```sortie
Refused: insufficient funds
```

> **Remember:** inherit from `StandardError`, not from `Exception`. A plain `rescue`
> only catches `StandardError`s.

## Trying again with retry

`retry` restarts the `begin` block from the beginning. Always limit the number of tries.

```ruby
tries = 0
begin
  tries += 1
  raise "network unavailable" if tries < 3
  puts "Connected after #{tries} tries"
rescue
  retry if tries < 3
end
```

```sortie
Connected after 3 tries
```

## Protecting yourself from nil

`nil` ("nothing") often shows up without warning: an array slot that does not exist, a
missing key… Calling a method on `nil` causes a `NoMethodError`.

The `&.` operator, called **safe navigation**, avoids this trap: if the object is `nil`,
the method is not called and the result is `nil`. It is like knocking on a door only if
there is a house.

```ruby
user = nil
p user&.upcase

name = "alice"
p name&.upcase
```

```sortie
nil
"ALICE"
```

Combine it with `||` to give a default value:

```ruby
city = nil
puts city&.capitalize || "Unknown"
```

```sortie
Unknown
```

> **Tip:** on a hash, `fetch` with a default value also avoids `nil`s
> (see [hashes](collections.md#hashes)).

## Common mistakes

### Catching the wrong error class

```ruby norun
begin
  Integer("abc")
rescue ZeroDivisionError
  puts "Problem"
end
```

```text
<internal:kernel>:307:in `Integer': invalid value for Integer(): "abc" (ArgumentError)
	from main.rb:2:in `<main>'
```

`rescue ZeroDivisionError` only catches divisions by zero. Here, the error is an
`ArgumentError`: it slips through. Give the right class, the one written in parentheses
in the message: `rescue ArgumentError`.

### Inheriting from Exception instead of StandardError

```ruby norun
class MyError < Exception
end
begin
  raise MyError, "oops"
rescue => e
  puts "Caught"
end
```

```text
main.rb:4:in `<main>': oops (MyError)
```

A `rescue` without a class only catches `StandardError`s. Write
`class MyError < StandardError`.

### Calling a method on nil

```ruby norun
users = { alice: "Alice" }
puts users[:bob].upcase
```

```text
main.rb:2:in `<main>': undefined method `upcase' for nil (NoMethodError)

puts users[:bob].upcase
                ^^^^^^^
```

`users[:bob]` is `nil`. Plan for this case: `users[:bob]&.upcase`, or
`users.fetch(:bob, "unknown").upcase`.

## Test yourself

**Question 1:** what does this code print?

```ruby norun
begin
  puts "A"
  raise "failure"
  puts "B"
rescue
  puts "C"
ensure
  puts "D"
end
```

> **Answer**
>
> `A`, `C`, then `D`. `B` is never printed: after `raise`, Ruby jumps straight to the
> `rescue`. `ensure` always runs.

**Question 2:** what is the value of `nil&.length`?

> **Answer**
>
> `nil`. With `&.`, the method is not called on `nil`, and there is no error.

**Question 3:** write a method `valid_age(n)` that raises an `ArgumentError` if `n` is
negative, and returns `n` otherwise.

> **Answer**
>
> ```ruby
> def valid_age(n)
>   raise ArgumentError, "negative age" if n < 0
>   n
> end
>
> puts valid_age(12)
> begin
>   valid_age(-1)
> rescue ArgumentError => e
>   puts e.message
> end
> ```
>
> ```sortie
> 12
> negative age
> ```

## Summary

- An unhandled error stops the program; its message gives the line and the class.
- `begin` and `rescue` catch an error; `=> e` gives its message.
- `ensure` always runs, with or without an error.
- `raise` raises an error; your own errors inherit from `StandardError`.
- `&.` avoids the error when a value is `nil`.

Next: [reading and writing files](fichiers.md). Compare with
[errors in Rust](../rust/erreurs.md).
