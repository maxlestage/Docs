# Conditions and loops

This page shows how to make decisions with `if`, `unless` and `case`, and how to repeat actions with loops.

> **Goals:**
>
> - you will know how to run code only if a condition is true;
> - you will know how to combine conditions with `&&`, `||` and `!`;
> - you will know how to choose among several cases with `case`;
> - you will know how to repeat code with `while`, `loop`, `times`, `each` and `step`;
> - you will know how to stop a loop or skip a turn.

## if, elsif and else

A [condition](../debuter/glossaire.md#condition) is a question you answer
with true or false: "is the grade greater than 10?". It is a crossroads: depending on
the answer, the program takes one path or another.

`if` runs code only if a condition is true. The block ends with `end`.

```ruby
grade = 14
if grade >= 16
  puts "Very good"
elsif grade >= 10
  puts "Passed"
else
  puts "Failed"
end
```

```sortie
Passed
```

- `if` asks the first question.
- `elsif` ("else, if") asks another question, only if the previous ones
  answered no. Watch the spelling: `elsif`, with no `e` in the middle.
- `else` runs if no question answered yes.
- Ruby stops at the **first** true branch.

Comparisons (`==`, `!=`, `<`, `>`, `<=`, `>=`) are introduced in
[first steps](premiers-pas.md).

### The short form

For a single instruction, you can put `if` at the end of the line.

```ruby
temperature = 35
puts "It's hot!" if temperature > 30
```

```sortie
It's hot!
```

### if returns a value

In Ruby, `if` gives a value: the value of the chosen branch. You can store it
in a variable.

```ruby
grade = 8
result = if grade >= 10
           "passed"
         else
           "failed"
         end
puts result
```

```sortie
failed
```

## unless

`unless` is the opposite of `if`: the code runs if the condition
is **false**.

```ruby
logged_in = false
puts "Please log in." unless logged_in
```

```sortie
Please log in.
```

> **Tip:** avoid `unless` with `else`: it is hard to read. Use `if` instead.

## Combining conditions

`&&` means "and", `||` means "or", `!` means "not".

- `a && b` is true if `a` **and** `b` are true.
- `a || b` is true if at least one of the two is true.
- `!a` is true if `a` is false.

```ruby
age = 20
member = true
puts "Free entry" if age < 26 && member
puts "Not a member" if !member
puts "Reduced price" if age < 18 || age > 65
puts "Full price" unless age < 18 || age > 65
```

```sortie
Free entry
Full price
```

`&&` comes before `||`, just as `*` comes before `+`. Add parentheses to be
clear:

```ruby
age = 70
student = false
member = false
puts "Discount" if (age > 65 || student) && !member
```

```sortie
Discount
```

> **Note:** Ruby also knows `and`, `or` and `not`. They don't have the same precedence
> as `&&` and `||`: as a beginner, use only `&&`, `||` and `!`.

## The ternary operator

`condition ? value_if_true : value_if_false` chooses a value in one line.

```ruby
age = 15
status = age >= 18 ? "adult" : "minor"
puts status
```

```sortie
minor
```

Read it as a question: "age ≥ 18? if yes adult, otherwise minor".

## case and when

`case` compares a value with several possibilities. It is easier to read than a long
series of `elsif`.

```ruby
day = "saturday"
case day
when "saturday", "sunday"
  puts "Weekend!"
when "monday"
  puts "Hang in there..."
else
  puts "Weekday"
end
```

```sortie
Weekend!
```

- A comma in `when` means "or".
- `else` is used if no `when` matches.

`when` also accepts ranges:

```ruby
grade = 12
mention = case grade
          when 0...10 then "insufficient"
          when 10...14 then "fair"
          else "good"
          end
puts mention
```

```sortie
fair
```

`then` lets you write the answer on the same line as `when`.

## Ranges

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

> **Warning:** if the condition always stays true, the loop never stops
> (an **infinite loop**). Here, forgetting `counter += 1` would make the program run
> forever. Stop it with Ctrl+C.

## The until loop

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

The code between `{ }` is a **block**. Between the bars `|i|`, you name the value that
the block receives at each turn.

> **Remember:** `times` counts from `0`. `each` goes through each element of a
> collection. To learn more, read [blocks](blocs.md).

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

## Common mistakes

### Forgetting end

```ruby norun
grade = 14
if grade >= 10
  puts "Passed"
```

```text
main.rb: --> main.rb
Unmatched keyword, missing `end' ?
> 2  if grade >= 10
main.rb:3: syntax error, unexpected end-of-input, expecting `end' or dummy end (SyntaxError)
  puts "Passed"
               ^
```

Every `if`, `case`, `while`, `until` and `do` must be closed with an `end`. Add
`end` after `puts "Passed"`. Indent your code: a missing `end` shows up right
away.

### Writing = instead of ==

```ruby norun
grade = 14
if grade = 10
  puts "ten"
end
```

```text
main.rb:2: warning: found `= literal' in conditional, should be ==
ten
```

`grade = 10` **stores** 10 in `grade`: the condition is always true. Ruby gives a
*warning*, but the program goes on. Write `if grade == 10`.

### Writing elseif or else if

```ruby norun
grade = 12
if grade >= 10
  puts "Passed"
elseif grade >= 5
  puts "Resit"
end
```

```text
Passed
main.rb:4:in `<main>': undefined method `elseif' for main (NoMethodError)

elseif grade >= 5
^^^^^^
Did you mean?  elsif
```

Ruby reads `elseif` as an unknown method. The keyword is spelled `elsif`.

## Test yourself

**Question 1:** what does this code print?

```ruby norun
x = 5
puts "A" if x > 3 && x < 5
puts "B" if x > 3 || x < 5
```

> **Answer**
>
> Only `B`. `x < 5` is false (5 is not less than 5), so the `&&` is false.
> With `||`, it is enough for `x > 3` to be true.

**Question 2:** how many numbers does `(1...10).to_a` contain?

> **Answer**
>
> 9 numbers, from 1 to 9: with three dots, the last value is excluded.

**Question 3:** how do you print 10, 8, 6, 4, 2, 0?

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

## Summary

- `if`, `elsif` and `else` choose what to do; `unless` is the opposite of `if`.
- `&&` (and), `||` (or), `!` (not) combine conditions.
- `case` and `when` compare a value with several possibilities.
- `1..5` includes 5, `1...5` excludes it.
- `while`, `until` and `loop` repeat; `times`, `each`, `upto` and `step` are more common.
- `break` stops the loop, `next` goes to the next turn.

Next: [methods](fonctions.md). Compare with
[conditions and loops in Rythme](../rythme/conditions-boucles.md).
