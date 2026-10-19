# Loops

This page shows how to repeat actions with loops, and how to describe a series of values with a range.

> **Goals:**
>
> - you will know how to describe a series of values with a range;
> - you will know how to repeat code with `while`, `loop`, `times`, `each` and `step`;
> - you will know how to stop a loop or skip a turn.

This chapter follows [conditions](conditions-boucles.md): a loop repeats code as long
as a condition asks for it.

## Ranges

Grades from 10 to 14, days from 1 to 31, letters from a to e: we often talk about
values that follow each other. A range names them all at once, without writing them
one by one.

A range (`Range`) goes from one value to another. `1..5` includes 5, `1...5` excludes it.

```ruby
p (1..5).to_a
p (1...5).to_a
puts (1..10).include?(7)
p ("a".."e").to_a
```

```sortie
[1, 2, 3, 4, 5]
[1, 2, 3, 4]
true
["a", "b", "c", "d", "e"]
```

> **Tip:** to remember it: the third dot "pushes" the last value out.

## The while loop

Imagine you are asked to print the numbers from 1 to 100. Writing 100 `puts` would be
long and tedious. A loop does this repetitive work for you.

A [loop](../debuter/glossaire.md#loop) repeats code. Each pass is called a
**turn** (or an [iteration](../debuter/glossaire.md#iteration)).

`while` repeats as long as the condition is true.

```ruby
counter = 1
while counter <= 3
  puts "Turn #{counter}"
  counter += 1
end
```

```sortie
Turn 1
Turn 2
Turn 3
```

Line by line:

- `counter = 1`: the variable is 1 at the start;
- `while counter <= 3`: before each turn, Ruby asks the question. `1 <= 3` is true: it
  enters the loop;
- the block prints "Turn 1", then `counter += 1` adds 1: `counter` is now 2;
- Ruby goes back up to the `while` and asks again, with 2, then with 3;
- when `counter` is 4, `4 <= 3` is false: Ruby leaves the loop, after the `end`.

> **Warning:** if the condition always stays true, the loop never stops
> (an **infinite loop**). Here, forgetting `counter += 1` would make the program run
> forever. Stop it with Ctrl+C.

**Your turn:** write a program that prints the numbers from 1 to 5 with a `while`
loop, then "Done!".

> **Solution**
>
> ```ruby
> n = 1
> while n <= 5
>   puts n
>   n += 1
> end
> puts "Done!"
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> Done!
> ```

## The until loop

> **Note:** this part can wait. Read it once the basics feel solid.

`until` repeats **until** the condition becomes true.

```ruby
n = 3
until n.zero?
  puts n
  n -= 1
end
puts "Go!"
```

```sortie
3
2
1
Go!
```

## The loop loop

`loop` repeats forever. You get out with `break`. It is handy when the stopping
condition is in the middle of the turn.

```ruby
total = 0
loop do
  total += 7
  break if total > 20
  puts "Total: #{total}"
end
puts "Done: #{total}"
```

```sortie
Total: 7
Total: 14
Done: 21
```

Line by line:

- `total` is 0 at the start;
- first turn: `total += 7` gives 7. `7 > 20` is false: no `break`, and the `puts`
  prints 7;
- second turn: `total` is 14, same path;
- third turn: `total` is 21. `21 > 20` is true: `break` leaves the loop before the
  `puts`;
- Ruby carries on after the `end` and prints "Done: 21".

## times and each

In Ruby, people often prefer `times` and `each` to classic loops. You have no
counter to manage: impossible to forget `+= 1`.

```ruby
3.times { |i| puts "i is #{i}" }
(1..3).each { |n| puts n * 10 }
```

```sortie
i is 0
i is 1
i is 2
10
20
30
```

Line by line:

- `3.times` runs the block three times: `i` is 0, then 1, then 2;
- `(1..3).each` takes each number of the range, one by one, and stores it in `n`;
- the block prints `n * 10`: 10, then 20, then 30.

The code between `{ }` is a **block**. Between the bars `|i|`, you name the value that
the block receives at each turn.

> **Remember:** `times` counts from `0`. `each` goes through each element of a
> collection. To learn more, read [blocks](blocs.md).

**Your turn:** write a program that prints the 7 times table, from "7 x 1 = 7" to
"7 x 5 = 35", one line per calculation, with `each` on a range.

> **Solution**
>
> ```ruby
> (1..5).each { |i| puts "7 x #{i} = #{7 * i}" }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

### Counting with upto, downto and step

- `upto` counts up, `downto` counts down;
- `step` moves forward in steps: 2 by 2, 5 by 5…

```ruby
1.upto(3) { |n| puts "upto: #{n}" }
3.downto(2) { |n| puts "downto: #{n}" }
0.step(10, 5) { |n| puts "step: #{n}" }
```

```sortie
upto: 1
upto: 2
upto: 3
downto: 3
downto: 2
step: 0
step: 5
step: 10
```

`0.step(10, 5)` means: "from 0 to 10, 5 by 5".

### Getting the position with each_with_index

`each_with_index` gives the element **and** its position (its
[index](../debuter/glossaire.md#index)), which starts at `0`.

```ruby
["gold", "silver", "bronze"].each_with_index do |medal, i|
  puts "#{i + 1}. #{medal}"
end
```

```sortie
1. gold
2. silver
3. bronze
```

`do ... end` replaces the curly braces when the block has several lines.

## break and next

Sometimes you want to change the course of a loop along the way: stop searching as
soon as you have found something, or ignore some elements. That is the job of these
two words.

`break` stops the loop. `next` goes straight to the next turn.

```ruby
(1..10).each do |n|
  next if n.even?
  break if n > 7
  puts n
end
```

```sortie
1
3
5
7
```

- For `2`, `4`, `6`: `next` skips the rest of the turn.
- For `9`: `break` stops everything.

Line by line:

- `(1..10).each` gives `n` the values 1, 2, 3… up to 10;
- `n.even?` asks "is `n` even?". If so, `next` moves on to the next turn: the `puts`
  is not reached;
- `break if n > 7`: as soon as `n` goes past 7 (here, at 9), the loop stops: 9 and 10
  are never printed;
- otherwise, `puts n` prints the number.

## Common mistakes

### Calling each on a number

```ruby norun
5.each { |n| puts n }
```

```text
main.rb:1:in `<main>': undefined method `each' for an instance of Integer (NoMethodError)

5.each { |n| puts n }
 ^^^^^
```

A number is not a collection: it has no `each` method. To repeat 5 times, write
`5.times`. To go through the numbers from 1 to 5, write `(1..5).each`.

### Using the block variable after the block

```ruby norun
3.times { |i| puts i }
puts "Last: #{i}"
```

```text
0
1
2
main.rb:2:in `<main>': undefined local variable or method `i' for main (NameError)

puts "Last: #{i}"
              ^
```

`i` only exists inside the block, between `{` and `}`. To keep a value after the
loop, create the variable before the block (`last = nil`), then change it inside the
block (`last = i`).

## Test yourself

**Question 1:** how many numbers does `(1...10).to_a` contain?

> **Answer**
>
> 9 numbers, from 1 to 9: with three dots, the last value is excluded.

**Question 2:** how do you print 10, 8, 6, 4, 2, 0?

> **Answer**
>
> With `step` and a negative step:
>
> ```ruby
> 10.step(0, -2) { |n| puts n }
> ```
>
> ```sortie
> 10
> 8
> 6
> 4
> 2
> 0
> ```

**Question 3:** what is the difference between `break` and `next`?

> **Answer**
>
> `break` stops the whole loop: Ruby goes on after the `end`. `next` only skips the
> rest of the current turn and moves to the next turn.

## Summary

- `1..5` includes 5, `1...5` excludes it.
- `while`, `until` and `loop` repeat; `times`, `each`, `upto` and `step` are more common.
- `each_with_index` gives each element and its position, starting at `0`.
- `break` stops the loop, `next` goes to the next turn.

Next: [methods](fonctions.md). Compare with
[conditions and loops in Rythme](../rythme/conditions-boucles.md).
