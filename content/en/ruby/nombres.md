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

The `_` only makes the number easier to read, like the comma in "1,000,000".
A Ruby integer has no size limit: `2**100` works without any problem.

## Operators

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

As soon as one of the two numbers is a `Float`, the result is a `Float`. `fdiv` forces a
decimal division, and `divmod` gives the quotient and the remainder at once.

> **Warning:** `-7 / 2` is `-4`, not `-3`: Ruby always rounds down.

## Rounding

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

- `round` rounds to the nearest; `round(1)` keeps one digit after the decimal point.
- `floor` rounds down (to the "floor"), `ceil` rounds up (to the "ceiling").
- `truncate` cuts off the part after the decimal point.
- A negative number in `round` rounds to the tens, hundreds…

## The float trap

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

Same seed, same sequence. `srand(42)` does the same thing for plain `rand`. Without a
seed, each run gives other numbers.

## Converting

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

### The digits of a number

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
