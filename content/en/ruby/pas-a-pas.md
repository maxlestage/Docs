# Following a program step by step in Ruby

This page teaches you to run a Ruby program "in your head", line by line, with a table that tracks the value of each variable.

> **Goals:**
>
> - read a program line by line, in the order Ruby runs it;
> - fill in a tracking table for the [variables](../debuter/glossaire.md#variable);
> - guess the output of a program before running it;
> - make the program print its own steps.

## Why follow a program by hand?

A computer guesses nothing. It reads a line, does it, then moves on to the next one.
To understand a program, do the same: **one line at a time**.

It is like following a cooking recipe. You do not read the whole recipe at once and
hope the cake appears. You crack the eggs, then you add the sugar, then you stir. And
at each step, you know what is in the bowl.

Knowing how to do this helps you:

- understand an example that looks like magic;
- find a [bug](../debuter/glossaire.md#bug) without help;
- succeed at the exercises, because you can check your idea on paper.

## The tool: the tracking table

Take a sheet of paper. Make one column per variable, and one row per step.
For each line of code that changes a variable, write down its **new** value.

```text
  | Line | a | b |      <- one column per variable
  |------|---|---|
  |  1   | 5 | – |      <- "–": the variable does not exist yet
  |  2   | 5 | 3 |
```

Picture boxes with a label. A variable is a box. When you write `a = 8`, you empty the
box `a` and put `8` in it. The old value is **lost**.

```text
  before a = 8         after a = 8
  +-------+            +-------+
  |   5   |   a        |   8   |   a
  +-------+            +-------+
```

The method, always the same:

1. **Before running**, guess what gets printed.
2. Fill in the table, line by line.
3. Run the program and compare.

> **Remember:** a variable holds only one value at a time. When you give it a new
> value, the old one disappears.

## Program 1: variables that change

Here is the first program. It has no condition and no loop.

```ruby
a = 5
b = 3
a = a + b
b = a - b
puts a
puts b
```

Before running it, guess what gets printed. Write your answer on a piece of paper.

### The tracking table

| Line | Code | a | b | What happens |
|---|---|---|---|---|
| 1 | `a = 5` | 5 | – | the box `a` gets 5 |
| 2 | `b = 3` | 5 | 3 | the box `b` gets 3 |
| 3 | `a = a + b` | 8 | 3 | Ruby first computes `5 + 3`, then stores 8 in `a` |
| 4 | `b = a - b` | 8 | 5 | `a` is **already** 8: `8 - 3` gives 5 |
| 5 | `puts a` | 8 | 5 | prints 8 |
| 6 | `puts b` | 8 | 5 | prints 5 |

The trap is on line 4. Many people use the **old** value of `a`, that is 5. But on
line 4, `a` has already changed.

### The output

```sortie
8
5
```

On line 3, read the `=` as an arrow pointing left: "compute `a + b`, then put the
result in `a`". It is not an equality from mathematics.

```text
  a = a + b
  ^   \___/
  |     |
  |     +-- 1. compute with the current values: 5 + 3 = 8
  +-------- 2. store the result in a
```

Try it: replace line 4 with `b = b - a`. Fill in the table before running. You should
find `8` then `-5`.

> **Remember:** on the right of the `=`, you use the values **of the moment**. Always
> look at the last row of your table.

## Program 2: a condition

A [condition](../debuter/glossaire.md#condition) makes the program **skip** lines. In
the table, write down which branch Ruby chooses.

```ruby
temperature = 18
if temperature > 25
  advice = "shorts"
elsif temperature > 15
  advice = "a light sweater"
else
  advice = "a coat"
end
puts "It is #{temperature} degrees: take #{advice}."
```

Before running it, guess what gets printed.

### The tracking table

| Line | Code | Test | temperature | advice |
|---|---|---|---|---|
| 1 | `temperature = 18` | | 18 | – |
| 2 | `if temperature > 25` | `18 > 25` → `false` | 18 | – |
| 4 | `elsif temperature > 15` | `18 > 15` → `true` | 18 | – |
| 5 | `advice = "a light sweater"` | | 18 | `"a light sweater"` |
| 9 | `puts ...` | | 18 | `"a light sweater"` |

Look at the "Line" column: 1, 2, 4, 5, 9. Lines 3, 6 and 7 are **never** run. Ruby
found a true condition on line 4: it runs that branch, then jumps straight past the
`end`.

```text
  temperature > 25 ?  --no-->  temperature > 15 ?  --yes-->  "a light sweater"
        |                             |                              |
       yes                            no                             |
        v                             v                              |
    "shorts"                       "a coat"                          |
                                                                     v
                                                              puts (line 9)
```

### The output

```sortie
It is 18 degrees: take a light sweater.
```

Try it: set `temperature = 30`. The first test becomes true, and you get "take
shorts". Then try `temperature = 15`: `15 > 15` is **false**, so it is the coat.

> **Remember:** in an `if ... elsif ... else`, only one branch runs: the first one
> whose test is true. Write down each test and its result in the table.

## Program 3: a loop that accumulates

A [loop](../debuter/glossaire.md#loop) repeats lines. In the table, you write **one
row per pass**.

You put 10 euros in a magic piggy bank. Every week, what it holds doubles. After how
many weeks does it go over 50 euros?

```ruby
money = 10
week = 0
while money < 50
  week += 1
  money = money * 2
end
puts "Week #{week}: #{money} euros"
```

Before running it, guess what gets printed.

### The tracking table

| Step | Test `money < 50` | week | money |
|---|---|---|---|
| before the loop | | 0 | 10 |
| pass 1 | `10 < 50` → `true` | 1 | 20 |
| pass 2 | `20 < 50` → `true` | 2 | 40 |
| pass 3 | `40 < 50` → `true` | 3 | 80 |
| end | `80 < 50` → `false` | 3 | 80 |

The test is done **at the start** of each pass. On pass 3, `money` is still 40 when
it is tested: we go in. Then it becomes 80. The next test is false: the loop stops.

```text
          +-------------------------+
          v                         |
   money < 50 ? --yes--> week += 1
          |              money = money * 2 ------+
          no
          |
          v
        puts
```

### The output

```sortie
Week 3: 80 euros
```

Try it: replace `50` with `80`. On pass 4, the test `80 < 80` is false. The result
does not change: "Week 3: 80 euros". Then replace `<` with `<=`: there is one more
pass, and you get "Week 4: 160 euros".

> **Remember:** for a loop, make one row per pass, and write down the test **before**
> doing the pass. The last row of the table is the test that fails.

## Program 4: a method called twice

A [method](../debuter/glossaire.md#method) is like a coffee machine. You put
something in it (the [parameter](../debuter/glossaire.md#parameter)), it works, and it
gives you back a result. You can use it several times.

```ruby
def price_with_tax(price)
  tax = price * 20 / 100
  price + tax
end

book = price_with_tax(10)
pen = price_with_tax(5)
puts "Book: #{book}"
puts "Pen: #{pen}"
puts "Total: #{book + pen}"
```

Before running it, guess what gets printed.

### Who calls whom?

Lines 1 to 4 do **nothing** right away: they describe the machine. The program really
starts on line 6.

```text
  main program                                method price_with_tax
  ------------                                ---------------------
  line 6 : price_with_tax(10)  ---- 10 ---->  price = 10
                                              tax = 2
           book = 12           <--- 12 -----  returns 12

  line 7 : price_with_tax(5)   ----  5 ---->  price = 5
                                              tax = 1
           pen = 6             <---  6 -----  returns 6
```

### The tracking table

The method's variables (`price`, `tax`) are brand new on **each** call. They disappear
when the method is done.

| Step | Where? | price | tax | book | pen |
|---|---|---|---|---|---|
| line 6: call | method | 10 | – | – | – |
| line 2 | method | 10 | 2 | – | – |
| line 3: returns 12 | method | 10 | 2 | – | – |
| back to line 6 | main | – | – | 12 | – |
| line 7: call | method | 5 | – | 12 | – |
| line 2 | method | 5 | 1 | 12 | – |
| line 3: returns 6 | method | 5 | 1 | 12 | – |
| back to line 7 | main | – | – | 12 | 6 |

On line 2, Ruby computes from left to right: `5 * 20` gives 100, then `100 / 100`
gives 1. The last line of the method (`price + tax`) is the returned value: this is
the implicit return (see [methods](fonctions.md#implicit-return)).

### The output

```sortie
Book: 12
Pen: 6
Total: 18
```

Try it: add `puts tax` at the end of the program. Ruby stops
with a `NameError` ("undefined local variable or method"): `tax` only exists
**inside** the method.
Then remove that line and call `price_with_tax(7)`: `7 * 20 / 100` gives 1 (integer
division), so the price is 8.

> **Remember:** on each call, jump into the method with the new value of the
> parameter, run its lines, then come back with the result to the place of the call.

## Program 5: going through an array with a condition

Last program: a loop over an array, with tests inside. We count the passing grades and
look for the best one.

```ruby
grades = [12, 7, 15, 9, 18]
passed = 0
best = 0
grades.each do |grade|
  if grade >= 10
    passed += 1
  end
  if grade > best
    best = grade
  end
end
puts "Passed: #{passed}"
puts "Best: #{best}"
```

Before running it, guess what gets printed.

### The tracking table

`each` hands the elements one by one to `grade`, from left to right, like a teacher
taking the papers from the pile.

| Pass | grade | `grade >= 10`? | passed | `grade > best`? | best |
|---|---|---|---|---|---|
| before | – | | 0 | | 0 |
| 1 | 12 | yes | 1 | `12 > 0` yes | 12 |
| 2 | 7 | no | 1 | `7 > 12` no | 12 |
| 3 | 15 | yes | 2 | `15 > 12` yes | 15 |
| 4 | 9 | no | 2 | `9 > 15` no | 15 |
| 5 | 18 | yes | 3 | `18 > 15` yes | 18 |

When a test is false, the variable **keeps** its value: you copy the row above.

```text
  grades :  [ 12 ,  7 , 15 ,  9 , 18 ]
              ^     ^    ^    ^    ^
  pass :      1     2    3    4    5
  passed :    ok    -    ok   -    ok    -> 3
  record :    12    -    15   -    18    -> 18
```

### The output

```sortie
Passed: 3
Best: 18
```

Try it: replace the array with `[-4, -2, -9]`. The best grade printed is 0, even
though it is not in the array! Your tracking table shows why: `best` starts at 0 and
no test `grade > best` is true. The fix: start with `best = grades[0]`.

> **Remember:** for a loop over an array, one column for the element of the pass, one
> column per test, one column per variable that changes. When a test is false, copy
> the previous value.

## Making the program print its steps

Filling in a table by hand is the best practice. But you can also ask the program to
print **its own** tracking table. Add a `puts` with labels inside the loop:

```ruby
money = 10
week = 0
while money < 50
  week += 1
  money = money * 2
  puts "pass #{week}: week = #{week}, money = #{money}"
end
puts "End: week #{week}, #{money} euros"
```

```sortie
pass 1: week = 1, money = 20
pass 2: week = 2, money = 40
pass 3: week = 3, money = 80
End: week 3, 80 euros
```

The labels (`week = `, `money = `) matter. Without them, you see a column of numbers
and you no longer know which is which.

`p` is handy to see **exactly** what a variable holds: it shows the quotes of a string
and prints `nil` plainly.

```ruby
words = ["cat", "", "dog"]
words.each do |word|
  print "word = "
  p word
end
```

```sortie
word = "cat"
word = ""
word = "dog"
```

With `puts`, the empty string would have given a line with just `word = `: easy to
miss.

Try it: in program 5, add
`puts "pass: grade = #{grade}, passed = #{passed}, best = #{best}"` just before the
`end` of the loop. Compare with your table.

> **Remember:** a `puts` with labels inside the loop prints the tracking table by
> itself. Remove these lines once the program works.

## The debugger, in two sentences

Ruby can also pause your program: write `binding.irb` on a line, and you can type the
name of each variable to see its value at that moment. The `debug` debugger goes
further and moves forward line by line; it is all explained in
[the chapter on debugging](debogage.md).

> **Remember:** the debugger fills in the tracking table for you, but you need to know
> how to do it by hand to understand what it shows.

## Common mistakes

### Reusing an old value

We want to swap the contents of two variables. We write two lines, and forget that the
first one has already changed `a`.

```ruby
a = "red"
b = "blue"
a = b
b = a
puts a
puts b
```

```sortie
blue
blue
```

The tracking table shows the breakdown: after line 3, `a` is `"blue"`, and `"red"` is
lost. So line 4 copies `"blue"`. The fix: one more box to keep the old value, or
Ruby's one-line swap.

```ruby
a = "red"
b = "blue"
a, b = b, a
puts a
puts b
```

```sortie
blue
red
```

### Forgetting the "before the loop" step

We want to add up numbers, but we forget to create the total before the loop.

```ruby norun
[3, 4].each do |n|
  total += n
end
```

```text
a.rb:2:in `block in <main>': undefined method `+' for nil (NoMethodError)

  total += n
        ^
	from a.rb:1:in `each'
	from a.rb:1:in `<main>'
```

In the table, the "before" row would be empty: `total` has no value, so it is `nil`,
and you cannot add a number to `nil`. The fix: write `total = 0` **before** the loop.

### Resetting to zero on each pass

Another version of the same slip: the total is created **inside** the loop.

```ruby
[3, 4, 5].each do |n|
  total = 0
  total += n
  puts total
end
```

```sortie
3
4
5
```

The tracking table shows that `total` starts again from 0 on each pass. Move
`total = 0` out of the loop: you will get 3, 7, 12.

### One pass too many or too few

`(1..3)` holds 3 numbers, but `(1...3)` holds only 2. People often get it wrong by
just one pass.

```ruby
(1...3).each do |i|
  puts "pass #{i}"
end
```

```sortie
pass 1
pass 2
```

To avoid this, write the **first** and the **last** pass in your table before filling
in the rest. See [ranges](intervalles.md).

## Test yourself

1. What is the value of `x` at the end?

   ```ruby
   x = 2
   x = x * 3
   x = x + x
   puts x
   ```

   ```sortie
   12
   ```

> **Answer**
>
> 12. Line 1: `x` is 2. Line 2: `2 * 3`, so 6. Line 3: `6 + 6`, so 12. On each line,
> you use the value `x` has **at that moment**.

2. How many passes does this loop make, and what does it print?

   ```ruby
   n = 1
   while n < 20
     n = n + 5
   end
   puts n
   ```

   ```sortie
   21
   ```

> **Answer**
>
> 4 passes. `n` goes from 1 to 6, 11, 16, then 21. The test `21 < 20` is false: the
> loop stops and prints 21.
>
> | Pass | Test | n |
> |---|---|---|
> | before | | 1 |
> | 1 | `1 < 20` yes | 6 |
> | 2 | `6 < 20` yes | 11 |
> | 3 | `11 < 20` yes | 16 |
> | 4 | `16 < 20` yes | 21 |
> | end | `21 < 20` no | 21 |

3. Why don't the lines of a method run when Ruby reads the `def`?

> **Answer**
>
> The `def` only describes the method, like a recipe written in a book. It is carried
> out only when you **call** it, for example `price_with_tax(10)`. In the tracking
> table, you therefore skip the `def` and start at the first line outside it.

## Summary

- A computer runs one line at a time: follow it the same way.
- The tracking table has one column per variable and one row per step (or per pass).
- On the right of the `=`, always use the value **of the moment**, the one in the last
  row of the table.
- For a condition, write down the test and its result; for a loop, write down the test
  **before** each pass; for a method, jump into it then come back with the result.
- `puts` with labels, or `p`, prints the steps; the debugger goes further.

Next: [beginner pitfalls in Ruby](pieges.md)
