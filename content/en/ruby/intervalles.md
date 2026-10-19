# Ranges

This page shows how to describe a sequence of values, from 1 to 10 or from "a" to "z", with a range, and everything you can do with it.

> **Goals:**
>
> - you will know how to write a range with `..` and with `...`;
> - you will know how to loop over it, turn it into an array and move in steps;
> - you will know how to test whether a value is in a range;
> - you will know how to slice an array or a string with a range;
> - you will know how to use a range with no start or no end.

## Writing a range

A [range](../debuter/glossaire.md#range) (`Range` in Ruby) describes all the
values between a start and an end. It is like saying "pages 1 to 10" instead of
listing them one by one.

- `1..10`: from 1 to 10, **10 included**;
- `1...10`: from 1 to 10, **10 excluded** (so from 1 to 9).

```ruby
p (1..5).to_a
p (1...5).to_a
puts (1..5).class
puts (1..5).size
puts (1...5).size
```

```sortie
[1, 2, 3, 4, 5]
[1, 2, 3, 4]
Range
5
4
```

`to_a` turns the range into an array. The range itself does not store its
values: it only knows its start and its end. So `1..1_000_000` takes almost no
memory.

> **Tip:** to remember the difference, think that the third dot "pushes" the
> end out.

## Looping over a range

`each` goes over each value, like for an array.

```ruby
(1..3).each { |n| puts "Round #{n}" }

(1...4).each do |i|
  puts "#{i} × 5 = #{i * 5}"
end
```

```sortie
Round 1
Round 2
Round 3
1 × 5 = 5
2 × 5 = 10
3 × 5 = 15
```

A range also knows the methods of
[Enumerable](blocs.md): `map`, `select`, `sum`…

```ruby
p (1..5).map { |n| n * n }
p (1..10).select(&:even?)
```

```sortie
[1, 4, 9, 16, 25]
[2, 4, 6, 8, 10]
```

## Moving in steps with step

`step` skips several values at a time. `%` does the same thing, in a shorter way.

```ruby
p (0..20).step(5).to_a
p ((0..20) % 5).to_a
p (0.0..1.0).step(0.25).to_a
10.step(1, -3) { |n| print n, " " }
puts
```

```sortie
[0, 5, 10, 15, 20]
[0, 5, 10, 15, 20]
[0.0, 0.25, 0.5, 0.75, 1.0]
10 7 4 1 
```

To count down, a range like `10..1` is **empty**. Use `step` with a negative step,
`downto`, or `(1..10).reverse_each`.

## Testing membership

`include?` answers the question: "is this value in the range?"

```ruby
teen_ages = 13..19
puts teen_ages.include?(15)
puts teen_ages.include?(20)
puts (1.5..2.5).include?(2.1)
puts (1...10).include?(10)
```

```sortie
true
false
true
false
```

`cover?` does almost the same thing: it only compares with the start and the end. The
difference shows with strings.

```ruby
puts ("a".."z").include?("bb")
puts ("a".."z").cover?("bb")
puts (1..10).cover?(3..5)
```

```sortie
false
true
true
```

`"bb"` is not one of the 26 letters, but it does sit between `"a"` and `"z"` in
alphabetical order. `cover?` also accepts a whole range.

## Ranges of letters

A range works with anything that has a "next" value: letters too.

```ruby
p ("a".."e").to_a
puts ("a".."z").count
p ("A".."C").map { |letter| letter * 2 }
p ("aa".."ad").to_a
```

```sortie
["a", "b", "c", "d", "e"]
26
["AA", "BB", "CC"]
["aa", "ab", "ac", "ad"]
```

## Sum, minimum and maximum

```ruby
puts (1..100).sum
puts (1..10).min
puts (1..10).max
p (1..10).minmax
puts (1..5).reduce(:*)
```

```sortie
5050
1
10
[1, 10]
120
```

`sum` on a range of integers is instant: Ruby uses Gauss's formula instead of
adding the numbers one by one. `reduce(:*)` multiplies everything: 1 × 2 × 3 × 4 × 5.

## Ranges in case and when

With [case and when](conditions-boucles.md#case-and-when), a range puts a value
into a category. `when` uses `cover?` behind the scenes.

```ruby
def grade(score)
  case score
  when 16..20 then "Excellent"
  when 14...16 then "Good"
  when 10...14 then "Pass"
  else "Fail"
  end
end

puts grade(17)
puts grade(14)
puts grade(13.5)
puts grade(4)
```

```sortie
Excellent
Good
Pass
Fail
```

> **Warning:** with decimal scores, `10..13` would let `13.5` slip between two
> boxes. `10...14` covers everything up to 14, excluded.

## Slicing with a range

Inside square brackets, a range takes a piece of an array or a string. Positions
start at 0, and `-1` is the last one.

```ruby
letters = ["a", "b", "c", "d", "e"]
p letters[1..3]
p letters[1...3]
p letters[2..]
p letters[..1]
p letters[-2..]

word = "sunshine"
puts word[0..2]
puts word[3..]
puts word[..-2]
```

```sortie
["b", "c", "d"]
["b", "c"]
["c", "d", "e"]
["a", "b"]
["d", "e"]
sun
shine
sunshin
```

## Endless or beginless

You can leave one end empty:

- `5..`: from 5 to infinity (an **endless** range);
- `..5`: everything less than or equal to 5 (a **beginless** range).

```ruby
p (5..).first(3)
puts (..5).include?(-100)

def category(age)
  case age
  when ...18 then "minor"
  when 18..64 then "adult"
  when 65.. then "senior"
  end
end

puts category(12)
puts category(40)
puts category(80)
```

```sortie
[5, 6, 7]
true
minor
adult
senior
```

`first(3)` takes the first 3 values: do not call `to_a` on `5..`, it would never
finish.

## Common mistakes

### Forgetting the parentheses around the range

```ruby norun
p 1..5.to_a
```

```text
main.rb:1:in `<main>': undefined method `to_a' for an instance of Integer (NoMethodError)

p 1..5.to_a
      ^^^^^
Did you mean?  to_c
               to_r
               to_f
               to_i
               to_s
```

`.to_a` applies to `5`, not to the range. Wrap it: `p (1..5).to_a`.

### A backwards range

```ruby norun
p (10..1).to_a
```

```text
[]
```

No error, but an empty array: a range always goes from the smallest to the
largest. To count down: `10.downto(1).to_a` or `(1..10).to_a.reverse`.

### Turning an endless range into an array

```ruby norun
p (1..).to_a
```

```text
main.rb:1:in `to_a': cannot convert endless range to an array (RangeError)
	from main.rb:1:in `<main>'
```

An endless range has infinitely many values. Take only a few of them
with `first(n)` or `take(n)`.

## Test yourself

**Question 1:** how many values does `0...10` contain?

> **Answer**
>
> 10 values: from 0 to 9.
>
> ```ruby
> puts (0...10).size
> ```
>
> ```sortie
> 10
> ```

**Question 2:** how do you get the last 3 elements of `[1, 2, 3, 4, 5, 6]`?

> **Answer**
>
> ```ruby
> p [1, 2, 3, 4, 5, 6][-3..]
> ```
>
> ```sortie
> [4, 5, 6]
> ```

**Question 3:** how do you compute the sum of the even numbers from 1 to 100?

> **Answer**
>
> ```ruby
> puts (2..100).step(2).sum
> puts (1..100).select(&:even?).sum
> ```
>
> ```sortie
> 2550
> 2550
> ```

## Summary

- `a..b` includes the end, `a...b` excludes it.
- `each`, `map`, `select`, `sum`, `min`, `max` work on a range.
- `step` moves in steps; a backwards range is empty.
- `include?` and `cover?` test membership; `case`/`when` sorts a value.
- `array[1..3]` or `text[2..]` slice out a piece.
- `5..` has no end, `..5` has no start: use `first(n)` to read them.

Next: [common beginner traps in Ruby](pieges.md)
