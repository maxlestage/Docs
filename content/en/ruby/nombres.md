# Numbers and calculations

This page shows how Ruby calculates with whole numbers and decimal numbers, how to round, count money without errors and display a number nicely.

> **Goals:**
>
> - you will know the difference between an `Integer` and a `Float`;
> - you will know how to avoid the integer division trap;
> - you will know how to round, convert and format a number;
> - you will know why `0.1 + 0.2` is not exactly `0.3`, and what to do for money;
> - you will know how to draw random numbers in a reproducible way.

## Two kinds of numbers

A program counts all the time: an age, a price, a score, a temperature. Sometimes you
need a whole number (3 children), sometimes a decimal number (1.5 liters of milk).

Ruby has two main kinds of numbers:

- [integers](../debuter/glossaire.md#integer) (`Integer`): `3`, `-12`, `1_000_000`;
- [floats](../debuter/glossaire.md#float) (`Float`), with a decimal point:
  `3.5`, `-0.25`.

```ruby
puts 42.class
puts 3.14.class
puts 1_000_000
puts 2**100
```

```sortie
Integer
Float
1000000
1267650600228229401496703205376
```

Line by line:

- `42.class` asks Ruby what kind of number 42 is: it is an `Integer`;
- `3.14` contains a point, so it is a `Float`;
- `1_000_000` is displayed without the `_`: they are only there for you, to read it;
- `2**100` multiplies 2 by itself 100 times: the result is huge, and Ruby gives it as a
  whole number, digit by digit.

The `_` only makes the number easier to read, like the comma in "1,000,000".
A Ruby integer has no size limit: `2**100` works without any problem.

## Operators

A computer is first of all a calculating machine. As on a calculator, you write the
calculation, and Ruby gives the result.

[Operators](../debuter/glossaire.md#operator) do the basic calculations.

```ruby
puts 7 + 2
puts 7 - 2
puts 7 * 2
puts 7 / 2
puts 7 % 2
puts 7**2
```

```sortie
9
5
14
3
1
49
```

Line by line:

- the first three lines do an addition, a subtraction and a multiplication, as in
  school;
- `7 / 2` gives `3`, not `3.5`: the next section explains why;
- `7 % 2` gives what is left over from the division, and `7**2` multiplies 7 by itself.

- `%` gives the **remainder** of the division: 7 = 2 × 3 + **1**.
- `**` is the power: `7**2` is 7 × 7.

> **Tip:** `n % 2 == 0` tests whether `n` is even. Ruby also has `n.even?` and `n.odd?`.

### Order of operations

As in school, `**` comes before `*`, `/` and `%`, which come before `+` and `-`.
Parentheses come before everything.

```ruby
puts 2 + 3 * 4
puts (2 + 3) * 4
puts 2 * 3**2
puts -2**2
```

```sortie
14
20
18
-4
```

The last result is surprising: `-2**2` is read as `-(2**2)`. Write `(-2)**2` to get
`4`. When in doubt, add parentheses: they cost nothing.

**Your turn:** you have 25 days of vacation. Write a program that displays the number
of full weeks (7 days), then the number of extra days left over.

> **Solution**
>
> ```ruby
> puts 25 / 7
> puts 25 % 7
> ```
>
> ```sortie
> 3
> 4
> ```

## Integer division and decimal division

If both numbers are integers, `/` gives an integer: the part after the decimal point is
**thrown away**. It is like sharing 7 candies between 2 children: each one gets 3, and
1 is left over.

```ruby
puts 7 / 2
puts 7.0 / 2
puts 7 / 2.0
puts 7.fdiv(2)
p 7.divmod(2)
```

```sortie
3
3.5
3.5
3.5
[3, 1]
```

Line by line:

- `7 / 2`: both numbers are integers, so the decimals are thrown away: `3`;
- `7.0 / 2` and `7 / 2.0`: one of the two numbers is a `Float`, so the result keeps its
  decimals: `3.5`;
- `7.fdiv(2)` asks for a decimal division, even between two integers;
- `7.divmod(2)` gives two numbers at once: the quotient `3` and the remainder `1`.

As soon as one of the two numbers is a `Float`, the result is a `Float`. `fdiv` forces a
decimal division, and `divmod` gives the quotient and the remainder at once.

> **Warning:** `-7 / 2` is `-4`, not `-3`: Ruby always rounds down.

## Rounding

A calculation often gives too many digits after the decimal point. Nobody can pay a
price of $3.756: rounding keeps only the useful digits.

```ruby
x = 3.756
puts x.round
puts x.round(1)
puts x.floor
puts x.ceil
puts x.truncate
puts 1234.round(-2)
```

```sortie
4
3.8
3
4
3
1200
```

Line by line:

- `x = 3.756` stores the number in the variable `x`;
- `x.round` gives the nearest whole number: 3.756 is closer to 4 than to 3;
- `x.round(1)` keeps a single digit after the decimal point: `3.8`;
- `floor`, `ceil` and `truncate` give `3`, `4` and `3` (see the list below);
- `1234.round(-2)` rounds to the nearest hundred: `1200`.

- `round` rounds to the nearest; `round(1)` keeps one digit after the decimal point.
- `floor` rounds down (to the "floor"), `ceil` rounds up (to the "ceiling").
- `truncate` cuts off the part after the decimal point.
- A negative number in `round` rounds to the tens, hundreds…

**Your turn:** a trip costs `12.456` dollars. Write a program that stores this price in
a variable, then displays the price rounded to 2 digits after the decimal point, then
the next whole number up.

> **Solution**
>
> ```ruby
> price = 12.456
> puts price.round(2)
> puts price.ceil
> ```
>
> ```sortie
> 12.46
> 13
> ```

## The float trap

This part explains why a very simple calculation can give a result that is a tiny bit
wrong, and how to avoid it, especially when money is involved.

A `Float` is stored in binary in the computer. Some numbers that are simple for us,
like `0.1`, do not come out exactly in binary, just as `1/3` does not come out exactly in
decimal (`0.3333…`). Hence this surprising result:

```ruby
puts 0.1 + 0.2
puts 0.1 + 0.2 == 0.3
puts (0.1 + 0.2 - 0.3).abs < 1e-9
```

```sortie
0.30000000000000004
false
true
```

Line by line:

- `0.1 + 0.2` gives `0.30000000000000004`: a tiny bit more than `0.3`;
- `0.1 + 0.2 == 0.3` compares the two numbers: they are not exactly equal, so `false`;
- the last line computes the gap between the two, without its sign (`abs`), and checks
  that it is smaller than `1e-9`, that is 0.000000001: it is true.

To compare two floats, check whether their difference is tiny instead of using `==`.

### Rational: exact fractions

A `Rational` is a real fraction. It is written with an `r` at the end.

```ruby
third = 1/3r
puts third
puts third * 3
puts 0.1r + 0.2r == 0.3r
puts (3/4r).to_f
```

```sortie
1/3
1/1
true
0.75
```

### BigDecimal: for money

For prices, use `BigDecimal`. It calculates in decimal, like on a till receipt. You have
to load it with `require`.

```ruby
require "bigdecimal"

price = BigDecimal("19.99")
total = price * 3
puts total.to_s("F")
puts BigDecimal("0.1") + BigDecimal("0.2") == BigDecimal("0.3")
```

```sortie
59.97
true
```

> **Remember:** create a `BigDecimal` from a **string** (`"19.99"`), not from a
> float: the float would already be imprecise. `to_s("F")` displays it without
> scientific notation.

Another simple solution: count in cents, with integers (`1999` instead of `19.99`).

## The Math module

> **Note:** this part can wait. Read it once the basics feel solid.

For more advanced calculations, like a square root, there is no need to write
everything yourself: Ruby already provides them, gathered in one toolbox.

`Math` groups the mathematical functions.

```ruby
puts Math.sqrt(16)
puts Math::PI.round(4)
puts Math.hypot(3, 4)
puts Math.log10(1000)
puts 10.pow(3)
puts -7.abs
```

```sortie
4.0
3.1416
5.0
3.0
1000
7
```

`Math.sqrt` is the square root, `Math.hypot(3, 4)` the hypotenuse of a right triangle.
`abs` gives the value without its sign.

## Randomness with rand

A dice game, a quiz that shuffles its questions, a prize draw: they all need
randomness.

`rand(1..6)` draws a number between 1 and 6, like a die. The result changes every time
you run the program. To always get the same sequence (useful for testing), give a
**seed** with `srand` or create a `Random` generator.

```ruby
die = Random.new(42)
p 5.times.map { die.rand(1..6) }

die = Random.new(42)
p 5.times.map { die.rand(1..6) }
```

```sortie
[4, 5, 3, 5, 5]
[4, 5, 3, 5, 5]
```

Line by line:

- `Random.new(42)` creates a random generator with the seed 42, stored in `die`;
- `5.times.map { die.rand(1..6) }` rolls the die 5 times and gathers the 5 results in a
  list, which `p` displays;
- the last two lines start again with the same seed 42: the list is identical.

Same seed, same sequence. `srand(42)` does the same thing for plain `rand`. Without a
seed, each run gives other numbers.

## Converting

What a person types on the keyboard, or what you read from a file, always arrives as
text. But `"42"` is not a number: you have to convert it before calculating with it.

```ruby
puts "42".to_i + 1
puts "3.5".to_f * 2
puts "12abc".to_i
puts "abc".to_i
puts 7.to_f
puts 7.9.to_i
puts Integer("42")
```

```sortie
43
7.0
12
0
7.0
7
42
```

Line by line:

- `"42".to_i` turns the string `"42"` into the number 42, then `+ 1` gives `43`;
- `"3.5".to_f` gives the float 3.5, and `* 2` gives `7.0`;
- `"12abc".to_i` reads `12` then stops; `"abc".to_i` finds no digit at all: `0`;
- `7.to_f` gives `7.0`, and `7.9.to_i` cuts off the decimals without rounding: `7`.

`to_i` is lenient: it reads the start of the string and returns `0` if it finds nothing.
`Integer("...")` is strict: it raises an error if the string is not a number. It is
better for checking user input.

```ruby
begin
  Integer("12abc")
rescue ArgumentError => e
  puts "Invalid input: #{e.message}"
end
```

```sortie
Invalid input: invalid value for Integer(): "12abc"
```

**Your turn:** write a program that stores the strings `"12"` and `"30"` in two
variables, then displays their sum, calculated as numbers.

> **Solution**
>
> ```ruby
> a = "12"
> b = "30"
> puts a.to_i + b.to_i
> ```
>
> ```sortie
> 42
> ```

### The digits of a number

> **Note:** this part can wait. Read it once the basics feel solid.

`digits` gives the digits, **starting from the end**.

```ruby
p 1234.digits
puts 1234.digits.sum
puts 255.to_s(2)
puts "ff".to_i(16)
```

```sortie
[4, 3, 2, 1]
10
11111111
255
```

`to_s(2)` writes the number in binary, `to_i(16)` reads a number written in hexadecimal.

## Formatting a number

A result like `3.14159` is too long for a receipt or a table. Formatting means choosing
how the number is displayed, without changing its value.

`format` builds a string from a template. `%.2f` means "a float with 2 digits after the
decimal point", `%05d` "an integer on 5 digits, padded with 0s".

```ruby
puts format("%.2f", 3.14159)
puts format("%05d", 42)
puts format("%-6s|", "ab")
puts format("%d%% success", 87)
```

```sortie
3.14
00042
ab    |
87% success
```

Line by line:

- `%.2f` keeps 2 digits after the decimal point: `3.14`;
- `%05d` writes 42 on 5 digits, with 0s in front;
- `%-6s` writes the string `"ab"` on 6 characters, aligned to the left: you can see
  the spaces before the `|`;
- `%%` displays a real `%` sign.

**Your turn:** write a program that displays the price `7.5` with 2 digits after the
decimal point, then the number `7` on 3 digits, padded with 0s.

> **Solution**
>
> ```ruby
> puts format("%.2f", 7.5)
> puts format("%03d", 7)
> ```
>
> ```sortie
> 7.50
> 007
> ```

### Separating thousands

Ruby has no ready-made method to write `1,234,567`. A small regular expression does it:

```ruby
def thousands(n)
  n.to_s.reverse.scan(/\d{1,3}/).join(",").reverse
end

puts thousands(1234567)
puts thousands(999)
puts format("$%.2f", 1234.5)
```

```sortie
1,234,567
999
$1234.50
```

We reverse the string, cut it into groups of 3, glue them back together with commas,
then reverse it again.

## Common mistakes

### The division that loses the decimals

```ruby norun
average = (12 + 15) / 2
puts average
```

```text
13
```

No error, but the result is wrong: the real average is `13.5`. Both numbers are
integers, so the division is an integer division. Write `(12 + 15) / 2.0` or
`(12 + 15).fdiv(2)`.

### Dividing an integer by zero

```ruby norun
puts 10 / 0
```

```text
main.rb:1:in `/': divided by 0 (ZeroDivisionError)
	from main.rb:1:in `<main>'
```

Check the divisor before dividing. Note that `10 / 0.0` does not crash: it gives
`Infinity`, which is often no better.

### Adding a string and a number

```ruby norun
age = "30"
puts age + 1
```

```text
main.rb:2:in `+': no implicit conversion of Integer into String (TypeError)

puts age + 1
           ^
	from main.rb:2:in `<main>'
```

`"30"` is a string. Convert it first: `age.to_i + 1`.

## Test yourself

**Question 1:** what are `17 / 5` and `17 % 5`?

> **Answer**
>
> `3` and `2`: 17 = 5 × 3 + 2.
>
> ```ruby
> p 17.divmod(5)
> ```
>
> ```sortie
> [3, 2]
> ```

**Question 2:** how do you display `2.5` rounded up to the next integer?

> **Answer**
>
> With `ceil`.
>
> ```ruby
> puts 2.5.ceil
> ```
>
> ```sortie
> 3
> ```

**Question 3:** why use `BigDecimal("0.1")` rather than `0.1` for a price?

> **Answer**
>
> Because `0.1` is a `Float`, stored in binary as an approximation. The small errors add
> up (`0.1 + 0.2` gives `0.30000000000000004`). `BigDecimal` calculates in decimal,
> exactly.

## Summary

- `Integer` for whole numbers, `Float` for decimal numbers.
- Between two integers, `/` throws away the decimals: use `2.0` or `fdiv`.
- `round`, `floor`, `ceil` round; `format("%.2f", x)` displays neatly.
- Floats are approximate: `Rational` or `BigDecimal` for exact calculations.
- `Integer("...")` converts and checks; `to_i` converts without complaining.
- `Random.new(seed)` gives reproducible randomness.

Next: [Conditions and loops](conditions-boucles.md)
