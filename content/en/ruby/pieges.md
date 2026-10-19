# Beginner pitfalls in Ruby

This page gathers the 12 pitfalls that most often block beginners in Ruby, each with its explanation and the right way to do it.

> **Goals:**
>
> - you will recognize the 12 most common pitfalls of Ruby;
> - you will know why they are surprising;
> - you will know how to fix them, or better, avoid them.

## How to read this page

Each pitfall follows the same path:

1. what you **expect**;
2. the code that **surprises** you;
3. **why** Ruby does that;
4. the **fix**.

No need to read everything at once. Come back here when a program does "crazy
things": the cause is often in this list.

## Printing and calculation pitfalls

### 1. puts, print and p do not show the same thing

**You expect:** the three of them to print the same thing.

**What happens:**

```ruby
puts "1"
p "1"
puts 1
p 1
puts nil
p nil
print "a"
print "b"
puts
```

```sortie
1
"1"
1
1

nil
ab
```

**Why:** each [method](../debuter/glossaire.md#method) has its own job.

| Method | What it does | For whom? |
|---|---|---|
| `puts` | prints the text, then goes to the next line | the user |
| `print` | prints the text, **without** going to the next line | the user |
| `p` | prints the value **as Ruby sees it** | you, when hunting a bug |

With `puts`, the text `"1"` and the number `1` look alike. `nil` even becomes an empty
line! `p` shows the quotes and writes `nil`: you see the truth.

**The fix:** to hunt a bug, use `p`, not `puts`.

> **Try it:** print `p [1, "1", nil]` then `puts [1, "1", nil]`. `puts` writes one
> element per line, and the line for `nil` is empty.

### 2. 7 / 2 gives 3

**You expect:** `3.5`.

**What happens:**

```ruby
puts 7 / 2
puts 7.0 / 2
puts 7.fdiv(2)
puts 7 % 2
```

```sortie
3
3.5
3.5
1
```

**Why:** `7` and `2` are [integers](../debuter/glossaire.md#integer). Between two
integers, `/` does an **integer division**: it keeps the quotient and throws away the
remainder. It is like sharing 7 candies between 2 children: each one gets 3, and 1 is
left over (that is what `%` gives).

**The fix:** put a [float](../debuter/glossaire.md#float) in the calculation
(`7.0`) or use `fdiv`.

```ruby
grades = [12, 15, 14]
average = grades.sum / grades.size
puts average
average = grades.sum.fdiv(grades.size)
puts average.round(2)
```

```sortie
13
13.67
```

> **Remember:** integer / integer = integer. For an average, use `fdiv` or `to_f`.

### 3. A text is not a number

**You expect:** `"5" + 5` to give `10`.

**What happens:**

```ruby norun
puts "5" + 5
```

```text
main.rb:1:in `+': no implicit conversion of Integer into String (TypeError)
	from main.rb:1:in `<main>'
```

And when there is no error, the result is surprising too:

```ruby
puts "5" + "5"
puts "5" * 3
```

```sortie
55
555
```

**Why:** `"5"` is a [string](../debuter/glossaire.md#string), not a
number. For a text, `+` **glues** and `*` **repeats**. Ruby never turns a text into a
number on its own.

**The fix:** convert it yourself.

```ruby
puts "5".to_i + 5
puts "5" + 5.to_s
puts "Total: #{5 + 5}"
puts "abc".to_i
```

```sortie
10
55
Total: 10
0
```

| Code | Result | What happened |
|---|---|---|
| `"5".to_i + 5` | `10` | text → number, then addition |
| `"5" + 5.to_s` | `"55"` | number → text, then gluing |
| `"abc".to_i` | `0` | no digit at the start: `0`, with no error! |

> **Warning:** `to_i` never complains. If the text is not a number, you get `0`. To
> get an error instead, use `Integer("abc")`.

## Condition pitfalls

### 4. nil comes from everywhere

**You expect:** a missing value to give a clear error, or `0`.

**What happens:**

```ruby norun
stock = { "apples" => 3 }
puts stock["pears"] + 1
```

```text
main.rb:2:in `<main>': undefined method `+' for nil (NoMethodError)

puts stock["pears"] + 1
                    ^
```

**Why:** `nil` means "nothing" (see [null](../debuter/glossaire.md#null)). In Ruby,
many things return `nil` **without an error**:

```ruby
stock = { "apples" => 3 }
p stock["pears"]
p [1, 2, 3][10]
p "hello".index("z")
p [].first
```

```sortie
nil
nil
nil
nil
```

The error only comes later, when you **use** this `nil`. The message
`undefined method ... for nil` means: "you asked `nil` for something".

```text
stock["pears"]       →  nil       (no error here)
nil + 1              →  NoMethodError  (the error happens here)
```

**The fix:** find where the `nil` comes from, then give a fallback value.

```ruby
stock = { "apples" => 3 }
puts stock.fetch("pears", 0) + 1
puts (stock["pears"] || 0) + 1
name = nil
p name&.upcase
```

```sortie
1
1
nil
```

`&.` means: "call the method only if this is not `nil`".

> **Remember:** `undefined method ... for nil`: go back up the program to find the line
> that produced `nil`.

### 5. Writing = instead of == in an if

**You expect:** this program to print nothing, since `x` is 5.

**What happens:**

```ruby
x = 5
if x = 3
  puts "x is three"
end
puts x
```

```sortie
x is three
3
```

**Why:** `=` **stores** a value, `==` **compares**. `x = 3` puts 3 in `x`, and its
value is `3`. And `3` counts as true (see the next pitfall). So the `if` goes through,
and `x` is changed! Ruby only prints a warning:
`warning: found '= literal' in conditional, should be ==`.

**The fix:** `if x == 3`.

> **Tip:** read `=` as "takes the value" and `==` as "is it equal to". It avoids the
> confusion.

### 6. 0 and the empty text are true

**You expect:** `0`, `""` or `[]` to count as "false", like in other languages.

**What happens:**

```ruby
[0, "", [], "false", nil, false].each do |value|
  if value
    puts "#{value.inspect} is true"
  else
    puts "#{value.inspect} is false"
  end
end
```

```sortie
0 is true
"" is true
[] is true
"false" is true
nil is false
false is false
```

**Why:** in Ruby, there are **only two false values**: `false` and `nil`. Everything
else is true, even `0`, even the text `"false"`.

**The fix:** ask the real question.

```ruby
number = 0
text = ""
puts "zero" if number.zero?
puts "empty text" if text.empty?
```

```sortie
zero
empty text
```

> **Remember:** only `false` and `nil` are false. To test "empty" or "zero", use
> `empty?` or `zero?`.

### 7. and and or are not && and ||

**You expect:** `ok = true and false` to put `false` in `ok`.

**What happens:**

```ruby
ok = true and false
p ok
ok = true && false
p ok
```

```sortie
true
false
```

**Why:** `and` is **very weak**: Ruby first does `ok = true`, then `... and false`.
With parentheses, you can see what Ruby understands:

```text
ok = true and false    →   (ok = true) and false     →  ok is true
ok = true && false     →   ok = (true && false)      →  ok is false
```

**The fix:** in conditions and calculations, always use `&&`, `||` and `!`.

## Range and loop pitfalls

### 8. Two dots or three dots

**You expect:** `1...5` to contain 5.

**What happens:**

```ruby
p (1..5).to_a
p (1...5).to_a
letters = ["a", "b", "c"]
p (0...letters.size).to_a
```

```sortie
[1, 2, 3, 4, 5]
[1, 2, 3, 4]
[0, 1, 2]
```

**Why:** `..` **includes** the end, `...` **excludes** it. The third dot "pushes" the
end out. `0...size` is handy for the positions of an array: they go from 0 to
size − 1.

**The fix:** reread your [range](../debuter/glossaire.md#range) and ask yourself:
"is the last value inside?". Everything is explained in [ranges](intervalles.md).

### 9. A variable created in a block disappears

**You expect:** to find `last` again after the loop.

**What happens:**

```ruby norun
[1, 2, 3].each { |n| last = n }
puts last
```

```text
main.rb:2:in `<main>': undefined local variable or method `last' for main (NameError)

puts last
     ^^^^
```

**Why:** a [variable](../debuter/glossaire.md#variable) **created** in a block
`{ ... }` or `do ... end` only exists inside it (this is its
[scope](../debuter/glossaire.md#scope)). When the block ends, it is thrown away.

**The fix:** create the variable **before** the block. The block can then change it.

```ruby
last = nil
total = 0
[1, 2, 3].each do |n|
  last = n
  total += n
end
puts last
puts total
```

```sortie
3
6
```

| Step | n | last | total |
|---|---|---|---|
| before | – | nil | 0 |
| round 1 | 1 | 1 | 1 |
| round 2 | 2 | 2 | 3 |
| round 3 | 3 | 3 | 6 |

## Sharing pitfalls

These three pitfalls have the same cause: **two names for a single object**. Picture a
box with two labels. If you change the content through one label, the other one sees
the change.

```text
  a ──┐
      ├──►  [ "cat" ]     a single box
  b ──┘
```

### 10. Changing a text also changes the other variable

**You expect:** `a` to stay `"cat"`.

**What happens:**

```ruby
a = "cat"
b = a
b << "s"
puts a
puts b
```

```sortie
cats
cats
```

**Why:** `b = a` does not copy the text. It sticks a second label on the same box.
`<<` **changes** the box: `a` and `b` both see `"cats"`.

**The fix:** `+` creates a **new** text, and leaves the old one alone.

```ruby
a = "cat"
b = a
b += "s"
puts a
puts b
```

```sortie
cat
cats
```

| Code | What does it do? |
|---|---|
| `b << "s"` | changes the box (the other names see the change) |
| `b += "s"` | creates a new box for `b` (the other names do not change) |
| `b = a.dup` | makes a real copy |

It is the same for methods ending in `!`: `upcase` returns a new text, `upcase!`
changes the text itself.

```ruby
word = "hello"
word.upcase
puts word
word.upcase!
puts word
```

```sortie
hello
HELLO
```

> **Try it:** replace `word.upcase` with `word = word.upcase`. The first line then
> prints `HELLO`.

### 11. Array.new(3, []) shares the same array

**You expect:** a grid of 3 rows, and an `"x"` in the first one only.

**What happens:**

```ruby
grid = Array.new(3, [])
grid[0] << "x"
p grid
```

```sortie
[["x"], ["x"], ["x"]]
```

**Why:** Ruby creates **a single** empty array and puts it in 3 times. The 3 slots are
3 labels on the same box.

```text
grid[0] ──┐
grid[1] ──┼──►  [ ]     a single array!
grid[2] ──┘
```

**The fix:** with a block, Ruby creates a new array for each slot.

```ruby
grid = Array.new(3) { [] }
grid[0] << "x"
p grid
```

```sortie
[["x"], [], []]
```

### 12. Hash.new([]) shares the same default value

**You expect:** `h[:a] << 1` to store `[1]` under the key `:a`.

**What happens:**

```ruby
h = Hash.new([])
h[:a] << 1
p h
p h[:b]
```

```sortie
{}
[1]
```

**Why:** the default value is **a single** array, shared by all the missing keys.
`h[:a] << 1` adds 1 to this shared array, without creating the key `:a`. The
[hash](../debuter/glossaire.md#dictionary) stays empty, and `h[:b]` sees the `1`!

**The fix:** the block creates a new array for each key, and stores it.

```ruby
h = Hash.new { |hash, key| hash[key] = [] }
h[:a] << 1
p h
p h[:b]
p h
```

```sortie
{:a=>[1]}
[]
{:a=>[1], :b=>[]}
```

> **Remember:** `Array.new(n, value)` and `Hash.new(value)` share **a single** object.
> For an array or a text, pass a block `{ ... }`.

## Common mistakes

### Changing a frozen text

```ruby norun
# frozen_string_literal: true
name = "Alice"
name << " Martin"
```

```text
main.rb:3:in `<main>': can't modify frozen String: "Alice" (FrozenError)
```

The magic comment `# frozen_string_literal: true` (common at the top of files)
**freezes** all the texts written between quotes: they become
[immutable](../debuter/glossaire.md#immutable). Use `+`: `name = name + " Martin"`,
or start from a copy: `name = +"Alice"` (a text that is not frozen). See
[frozen texts](texte.md#frozen-texts).

### Calling a method on a slot that does not exist

```ruby norun
fruits = ["apple", "kiwi"]
puts fruits[2].upcase
```

```text
main.rb:2:in `<main>': undefined method `upcase' for nil (NoMethodError)

puts fruits[2].upcase
              ^^^^^^^
```

The positions go from 0 to 1: `fruits[2]` is `nil` (pitfall 4). Check the position,
or use `fruits.last` for the last element.

## Test yourself

**Question 1:** what does this program print?

```ruby norun
puts 10 / 4
puts 10 / 4.0
```

> **Answer**
>
> `2` then `2.5`. Between two integers, `/` only keeps the whole part.
>
> ```ruby
> puts 10 / 4
> puts 10 / 4.0
> ```
>
> ```sortie
> 2
> 2.5
> ```

**Question 2:** does this program print "nothing"?

```ruby norun
answer = 0
puts "nothing" unless answer
```

> **Answer**
>
> No. `0` is true in Ruby, so `unless answer` does not go through. To test for zero,
> write `answer.zero?`.
>
> ```ruby
> answer = 0
> puts "nothing" unless answer
> puts "zero" if answer.zero?
> ```
>
> ```sortie
> zero
> ```

**Question 3:** how do you create an array of 2 **independent** empty arrays?

> **Answer**
>
> With a block: each slot gets its own array.
>
> ```ruby
> pairs = Array.new(2) { [] }
> pairs[1] << "b"
> p pairs
> ```
>
> ```sortie
> [[], ["b"]]
> ```

## Summary

- `p` shows the real value (quotes, `nil`): use it to hunt a bug.
- Integer / integer = integer; a text never becomes a number on its own.
- `nil` arrives silently; only `false` and `nil` are false, `0` is true.
- `=` stores, `==` compares; prefer `&&` and `||` to `and` and `or`.
- `..` includes the end, `...` excludes it; a variable created in a block stays there.
- `<<`, methods ending in `!`, `Array.new(n, [])` and `Hash.new([])` share an object.

Next: [exercises to get started](exercices-faciles.md)
