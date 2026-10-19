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

Line by line:

- `grade = 14` stores 14 in the variable `grade`;
- `if grade >= 16` asks: "is 14 greater than or equal to 16?". No: Ruby skips this
  block;
- `elsif grade >= 10` asks: "is 14 greater than or equal to 10?". Yes: Ruby prints
  "Passed";
- one branch answered yes: Ruby skips the `else` and carries on after the `end`.

Comparisons (`==`, `!=`, `<`, `>`, `<=`, `>=`) are introduced in
[first steps](premiers-pas.md).

**Your turn:** write a program that stores `hour = 15` in a variable, then prints
"Good morning" if the hour is before 12, "Good afternoon" if it is before 18, and
"Good evening" otherwise.

> **Solution**
>
> ```ruby
> hour = 15
> if hour < 12
>   puts "Good morning"
> elsif hour < 18
>   puts "Good afternoon"
> else
>   puts "Good evening"
> end
> ```
>
> ```sortie
> Good afternoon
> ```

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

Sometimes the question comes more naturally the other way round: "if they are **not**
logged in, ask them to log in". `unless` lets you write it the way you would say it.

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

A decision often depends on several things at once. To get into the museum for free,
you must be young **and** a member. These three signs join several tests into one.

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

Line by line:

- `age` is 20 and `member` is `true`;
- line 3: `age < 26` is true **and** `member` is true: "Free entry" is printed;
- line 4: `!member` turns `true` into `false`: nothing is printed;
- line 5: 20 is neither less than 18 nor greater than 65: the `||` is false;
- line 6: the same condition is false, and `unless` acts precisely when it is false.

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

**Your turn:** write a program that stores `age = 70` in a variable, then prints
"Reduced price" if the age is less than 18 or greater than or equal to 65, and
"Full price" otherwise.

> **Solution**
>
> ```ruby
> age = 70
> if age < 18 || age >= 65
>   puts "Reduced price"
> else
>   puts "Full price"
> end
> ```
>
> ```sortie
> Reduced price
> ```

## The ternary operator

> **Note:** this part can wait. Read it once the basics feel solid.

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

Think of a drinks machine: depending on the button you press, it serves a different
drink. When one value can take many forms (a day, a choice in a menu), you want a
reaction for each one.

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

Line by line:

- `day` is `"saturday"`;
- `case day` announces the value to compare;
- first `when`: is `day` equal to `"saturday"` or `"sunday"`? Yes: Ruby prints
  "Weekend!";
- the other `when`s and the `else` are skipped, just like with `elsif`.

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
