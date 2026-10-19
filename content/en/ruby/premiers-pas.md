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

## Numbers

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

## Strings

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

## Symbols

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

You will see them again as hash keys in [arrays and hashes](collections.md).

## Booleans and nil

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

> **Warning:** `=` stores a value, `==` compares two values. Don't mix them up.

> **Remember:** in Ruby, only `false` and `nil` are "false". Everything else is true,
> even `0` and `""`.

## Printing with puts, print and p

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
