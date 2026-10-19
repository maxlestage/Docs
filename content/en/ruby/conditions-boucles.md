# Conditions

This page shows how to make decisions with `if`, `unless` and `case`, and how to combine several conditions.

> **Goals:**
>
> - you will know how to run code only if a condition is true;
> - you will know how to combine conditions with `&&`, `||` and `!`;
> - you will know how to choose among several cases with `case`.

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

**Question 2:** what does this code print?

```ruby norun
logged_in = true
puts "Welcome" unless logged_in
puts "Goodbye"
```

> **Answer**
>
> Only `Goodbye`. `unless` runs the code when the condition is **false**: here
> `logged_in` is true, so "Welcome" is not printed.

**Question 3:** what is the value of `activity`?

```ruby norun
day = "sunday"
activity = case day
           when "saturday", "sunday" then "rest"
           else "work"
           end
```

> **Answer**
>
> `"rest"`. In a `when`, the comma means "or": `"sunday"` matches the first `when`,
> and `case` returns the value of that branch.

## Summary

- `if`, `elsif` and `else` choose what to do; `unless` is the opposite of `if`.
- `if` returns a value: the value of the chosen branch.
- `&&` (and), `||` (or), `!` (not) combine conditions.
- `case` and `when` compare a value with several possibilities.

Next: [loops](boucles.md).
