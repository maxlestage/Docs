# First steps

This page covers the basics of Ruby: variables, numbers, text, symbols, booleans, `nil` and printing.

> **Goals:**
>
> - you will know how to store a value in a variable and change it;
> - you will know how to calculate with whole numbers and decimal numbers;
> - you will know how to write a text and insert a value into it;
> - you will know what `true`, `false` and `nil` are;
> - you will know how to print with `puts`, `print` and `p`.

## Variables

A program needs to remember things: a name, a score, a price. Without variables, every
value would be lost right away, like a phone number you forget because you did not
write it down.

A [variable](../debuter/glossaire.md#variable) is a name that refers to a
[value](../debuter/glossaire.md#value). Think of a label stuck on a box:
the label is the name, what is inside the box is the value. You create it with `=`.

```ruby
age = 30
first_name = "Alice"
puts first_name
puts age
```

```sortie
Alice
30
```

Line by line:

- `age = 30` stores the number 30 in a box named `age`;
- `first_name = "Alice"` stores the text `"Alice"` in another box, named `first_name`;
- the two `puts` read what is in each box and print it, in the order they are
  written: first `first_name`, then `age`.

Here, `=` does not mean "is equal to". It means "store this value under this
name". This is called an **assignment**.

You don't need to give the [type](../debuter/glossaire.md#type) (number,
text…): Ruby guesses it. `class` tells you what it guessed.

```ruby
puts 30.class
puts "Alice".class
puts 2.5.class
```

```sortie
Integer
String
Float
```

### Changing the value of a variable

A variable can change its value. The new value replaces the old one.

```ruby
score = 10
score = score + 5
puts score
score += 1
puts score
score -= 6
puts score
```

```sortie
15
16
10
```

`score += 1` is a shortcut for `score = score + 1`. There are also `-=`, `*=` and
`/=`.

**Your turn:** write a program that stores `5` in a variable `apples`, prints its value,
adds 3 to it with `+=`, then prints the new value.

> **Solution**
>
> ```ruby
> apples = 5
> puts apples
> apples += 3
> puts apples
> ```
>
> ```sortie
> 5
> 8
> ```

## Numbers

Almost every program calculates: the total of a shopping cart, an average of grades,
the time left before a meeting. Ruby calculates like a calculator, with a few rules to
know.

Ruby tells apart whole numbers (`Integer`, no decimal point) and decimal numbers (`Float`,
called **floats**). The decimal point is written with a dot: `2.5`.

### Operators

An **operator** is a sign that does a calculation.

| Operator | Meaning | Example | Result |
|-----------|------|---------|----------|
| `+` | addition | `7 + 3` | `10` |
| `-` | subtraction | `7 - 3` | `4` |
| `*` | multiplication | `7 * 3` | `21` |
| `/` | division | `7 / 2` | `3` |
| `%` | remainder of the division | `7 % 2` | `1` |
| `**` | power | `2 ** 10` | `1024` |

```ruby
puts 7 + 3
puts 7 / 2
puts 7.0 / 2
puts 7 % 2
puts 2 ** 10
```

```sortie
10
3
3.5
1
1024
```

Line by line:

- `puts 7 + 3`: Ruby first does the calculation, then `puts` prints the result, 10;
- `puts 7 / 2` prints 3, not 3.5: both numbers are whole numbers (the next section
  explains why);
- `puts 7.0 / 2`: with a float, the result keeps its part after the decimal point;
- `7 % 2` is 1, because 7 = 2 x 3 + 1: 1 is left over;
- `2 ** 10` multiplies 2 by itself 10 times: 1024.

### Whole division and decimal division

> **Warning:** `7 / 2` gives `3`. Between two whole numbers, the division keeps only the
> whole part. Write `7.0 / 2` to get `3.5`.

It is enough for **just one** of the two numbers to be a float. If your two numbers are
in variables, use `to_f` (*to float*) or `fdiv`.

```ruby
total = 7
parts = 2
puts total / parts
puts total.to_f / parts
puts total.fdiv(parts)
```

```sortie
3
3.5
3.5
```

`%` (the **modulo**) gives the remainder. It is often used to know whether a number is even:
`n % 2` is `0` for an even number.

### The order of calculations

As in math, `*`, `/` and `%` come before `+` and `-`. `**` comes before
everything. This is operator **precedence**. Parentheses change the order.

```ruby
puts 2 + 3 * 4
puts (2 + 3) * 4
puts 2 * 3 ** 2
puts 10 - 4 - 3
```

```sortie
14
20
18
3
```

> **Tip:** when in doubt, add parentheses. They cost nothing and
> make the calculation clear.

### Rounding

```ruby
puts 3.7.round
puts 3.14159.round(2)
puts 3.7.floor
puts 3.2.ceil
puts -5.abs
```

```sortie
4
3.14
3
4
5
```

- `round` rounds to the nearest; `round(2)` keeps 2 digits after the decimal point.
- `floor` rounds down, `ceil` rounds up.
- `abs` gives the value without its sign (the **absolute value**).

**Your turn:** 17 candies are shared between 5 children. Write a program that stores
these two numbers in variables, then prints how many candies each child gets, and how
many are left over.

> **Solution**
>
> ```ruby
> candies = 17
> children = 5
> puts candies / children
> puts candies % children
> ```
>
> ```sortie
> 3
> 2
> ```

## Strings

A program talks with words: it prints a name, a welcome message, an address. Anything
that is text is stored in a string.

A text is written between quotes. It is called a
[string of characters](../debuter/glossaire.md#string), or a **string**
(`String`): the letters are strung one after another, like beads.

```ruby
name = "Ruby"
puts name.length
puts name.upcase
puts name + " is nice"
```

```sortie
4
RUBY
Ruby is nice
```

- `length` gives the number of characters.
- `upcase` turns it into capital letters.
- `+` sticks two texts end to end.

### Interpolation

Between double quotes, `#{...}` inserts a value into the text.

```ruby
first_name = "Alice"
age = 30
puts "#{first_name} is #{age} years old."
puts "In 5 years: #{age + 5}"
```

```sortie
Alice is 30 years old.
In 5 years: 35
```

> **Warning:** interpolation does not work with single quotes:
> `'#{age}'` prints the text as is.

### Going from a text to a number

A text that contains digits is not a number: `"5"` is not `5`. To
go from one to the other:

- `to_i` (*to integer*) turns it into a whole number;
- `to_f` turns it into a float;
- `to_s` (*to string*) turns it into a text.

```ruby
puts "5" + "5"
puts "5".to_i + 5
puts 5.to_s + "5"
```

```sortie
55
10
55
```

The chapter [working with text](texte.md) goes much further.

**Your turn:** write a program that stores a price of `4` euros and a quantity of `3`
in two variables, then prints `3 croissants cost 12 euros.` using interpolation.

> **Solution**
>
> ```ruby
> price = 4
> quantity = 3
> puts "#{quantity} croissants cost #{price * quantity} euros."
> ```
>
> ```sortie
> 3 croissants cost 12 euros.
> ```

## Symbols

> **Note:** this part can wait. Read it once the basics feel solid.

A symbol starts with `:`. It is a fixed name, often used as a label.
Unlike a text, you don't change it: it is only there to name something.

```ruby
color = :red
puts color
p color
```

```sortie
red
:red
```

Line by line:

- `color = :red` stores the symbol `:red` in the variable `color`;
- `puts color` prints it like a word, without the colon;
- `p color` shows it the way Ruby sees it, with its colon: it really is a symbol, not
  a text.

You will see them again as hash keys in [arrays and hashes](collections.md).

## Booleans and nil

A program often has to answer yes or no: is this person an adult? Is the password
right? Booleans are these answers. And sometimes there is nothing to store yet at all:
that is what `nil` is for.

`true` and `false` are the [booleans](../debuter/glossaire.md#boolean).
`nil` means "nothing, no value": it is an empty box.

### Comparing two values

A comparison answers with `true` or `false`.

| Operator | Meaning |
|-----------|------|
| `==` | equal to |
| `!=` | not equal to |
| `<`, `>` | less than, greater than |
| `<=`, `>=` | less than or equal, greater than or equal |

```ruby
puts 3 > 2
puts 3 == 4
puts 3 != 4
puts "a" == "a"
p nil
puts nil.nil?
```

```sortie
true
false
true
true
nil
true
```

Line by line:

- `3 > 2` asks the question "is 3 greater than 2?". Yes: Ruby answers `true`;
- `3 == 4` answers `false`, and `3 != 4` answers `true`: 3 and 4 really are different;
- `"a" == "a"` shows that you can compare texts too;
- `p nil` prints `nil`, and `nil.nil?` asks "is this `nil`?": the answer is `true`.

> **Warning:** `=` stores a value, `==` compares two values. Don't mix them up.

> **Remember:** in Ruby, only `false` and `nil` are "false". Everything else is true,
> even `0` and `""`.

## Printing with puts, print and p

A program works in silence. To see a result, you have to ask for it, like asking
someone to read a result out loud. Ruby gives you three ways to do it.

- `puts` prints then goes to the next line.
- `print` prints without going to the next line.
- `p` prints the value the way Ruby sees it (handy for debugging).

```ruby
print "a"
print "b"
puts
puts "text"
p "text"
p 42
```

```sortie
ab
text
"text"
42
```

Line by line:

- `print "a"` prints `a` and stays on the same line; `print "b"` writes `b` right next
  to it;
- `puts` on its own prints nothing: it only goes to the next line;
- `puts "text"` prints the text without quotes, `p "text"` shows it with them;
- `p 42` prints `42` without quotes: it is a number, not a text.

`p` shows the quotes: you see right away that `"42"` is a text and `42`
a number.

```ruby
p "42"
p 42
puts "42"
puts 42
```

```sortie
"42"
42
42
42
```

**Your turn:** write a program that prints `Hello, Alice` on a single line, with a
`print` for `"Hello, "` then a `puts` for `"Alice"`. Then print the number `7` and the
text `"7"` with `p`.

> **Solution**
>
> ```ruby
> print "Hello, "
> puts "Alice"
> p 7
> p "7"
> ```
>
> ```sortie
> Hello, Alice
> 7
> "7"
> ```

## Comments

A comment starts with `#`. Ruby ignores it. It is there to explain the code to
humans.

```ruby
# This is a comment
puts "visible" # a comment at the end of a line
```

```sortie
visible
```

## Naming conventions

> **Note:** this part can wait. Read it once the basics feel solid.

A program is read far more often than it is written. A good name is like a clear label
on a jar: you know what is inside without opening it.

- Variables and methods: `snake_case` (`total_price`), in lowercase with `_`.
- Classes and modules: `CamelCase` (`BankAccount`).
- Constants: in capital letters (`TAX_RATE`).

A **constant** is a value that must not change. Its name starts with a
capital letter.

```ruby
TAX_RATE = 0.2
price_before_tax = 100
price_with_tax = price_before_tax * (1 + TAX_RATE)
puts price_with_tax
```

```sortie
120.0
```

Line by line:

- `TAX_RATE = 0.2` creates a constant: its name in capital letters says "do not
  change";
- `price_before_tax` is 100;
- Ruby first calculates the parentheses, `1 + 0.2`, which is `1.2`, then `100 * 1.2`.
  It stores `120.0` in `price_with_tax`;
- `puts` prints this value.

The result is `120.0` and not `120`: a calculation that mixes a whole number and a float
gives a float.

> **Tip:** follow these conventions: everyone uses them, and your code will be
> easier to read.

## Common mistakes

### Adding a text and a number

```ruby norun
puts "Age: " + 30
```

```text
main.rb:1:in `+': no implicit conversion of Integer into String (TypeError)

puts "Age: " + 30
               ^^
	from main.rb:1:in `<main>'
```

Ruby refuses to stick a number to a text. Convert the number, or use
interpolation:

```ruby
puts "Age: " + 30.to_s
puts "Age: #{30}"
```

```sortie
Age: 30
Age: 30
```

### A typo in a variable name

```ruby norun
first_name = "Alice"
puts frst_name
```

```text
main.rb:2:in `<main>': undefined local variable or method `frst_name' for main (NameError)

puts frst_name
     ^^^^^^^^^
Did you mean?  first_name
```

`frst_name` does not exist. Ruby suggests the closest name. Fix it to `puts first_name`.

### Dividing by zero

```ruby norun
puts 10 / 0
```

```text
main.rb:1:in `/': divided by 0 (ZeroDivisionError)
	from main.rb:1:in `<main>'
```

A whole division by zero is impossible. Check the divisor before calculating.
With a float, Ruby does not crash but gives `Infinity`:

```ruby
puts 10.0 / 0
```

```sortie
Infinity
```

## Test yourself

**Question 1:** what is `10 / 4`? And `10 / 4.0`?

> **Answer**
>
> `10 / 4` is `2`: between two whole numbers, the division keeps the whole part.
> `10 / 4.0` is `2.5`.

**Question 2:** what does this code print?

```ruby norun
x = 2 + 3 * 2
puts x
```

> **Answer**
>
> `8`: multiplication comes before addition. Write `(2 + 3) * 2` to get
> `10`.

**Question 3:** is `0` true or false in a Ruby condition?

> **Answer**
>
> True. Only `false` and `nil` are false.

## Summary

- A variable is created with `=`; Ruby guesses the type by itself.
- `*` and `/` come before `+` and `-`; parentheses change the order.
- `7 / 2` is `3` between whole numbers; write `7.0 / 2` to get `3.5`.
- A text is written between quotes; `#{...}` inserts a value into it.
- Only `false` and `nil` are false; everything else is true.
- `puts` prints, `print` prints without a new line, `p` shows the exact value.

Next: [Numbers and calculations](nombres.md)
