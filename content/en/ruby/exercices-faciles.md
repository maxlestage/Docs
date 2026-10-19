# Exercises to get started

This page offers 15 very guided exercises, all easy, to practice the basics of Ruby: variables, calculations, conditions, loops, methods, text, arrays, hashes and ranges.

## How to do these exercises

Take them in order. For each exercise:

1. **Read** the instructions and the expected output.
2. **Try on paper**: which steps? which variables?
3. **Write** your program in a file `exercise.rb`.
4. **Run it** with `ruby exercise.rb` (see [the introduction](introduction.md#running-a-program)).
5. **Compare** with the expected output, character by character.
6. Stuck? Read the **first hint**, then the second one. Open the **solution** last.

> **Tip:** an error is not a failure. Read the message: it gives the line and the
> cause. See [reading an error message](introduction.md#reading-an-error-message).

## Printing and calculating

### Exercise 1: Say hello

Level: ★☆☆

Store your first name in a variable `first_name`. Then print a welcome sentence that
contains this first name.

Expected output (with `"Léa"`):

```text
Hello Léa, welcome!
```

> **Hint:** create the variable with `first_name = "Léa"`.

> **Hint:** in a text between double quotes, `#{first_name}` is replaced by the value of
> the variable.

> **Solution**
>
> ```ruby
> first_name = "Léa"
> puts "Hello #{first_name}, welcome!"
> ```
>
> ```sortie
> Hello Léa, welcome!
> ```
>
> Why it works: `#{...}` is called interpolation. Ruby computes what is between the
> braces and puts it in the text.

### Exercise 2: In ten years

Level: ★☆☆

Store an age in a variable `age`. Compute the age in 10 years and print it.

Expected output (with `age = 25`):

```text
Today: 25 years old
In 10 years: 35 years old
```

> **Hint:** create a second variable: `later = age + 10`.

> **Solution**
>
> ```ruby
> age = 25
> later = age + 10
> puts "Today: #{age} years old"
> puts "In 10 years: #{later} years old"
> ```
>
> ```sortie
> Today: 25 years old
> In 10 years: 35 years old
> ```
>
> Why it works: `age + 10` computes a new value without changing `age`. You store it
> under another name to print it.

### Exercise 3: The price with VAT

Level: ★☆☆

An item costs 40 euros before tax. VAT is 20 %. Print the price including tax.

Expected output:

```text
Price with tax: 48.0 euros
```

> **Hint:** 20 % more means multiplying by `1.2`.

> **Solution**
>
> ```ruby
> price_before_tax = 40
> price_with_tax = price_before_tax * 1.2
> puts "Price with tax: #{price_with_tax} euros"
> ```
>
> ```sortie
> Price with tax: 48.0 euros
> ```
>
> Why it works: `1.2` is a decimal number, so the result is one too: Ruby prints
> `48.0`. See [numbers](nombres.md).

### Exercise 4: Sharing candies

Level: ★☆☆

17 candies are shared between 5 children. How many does each one get? How many are
left?

Expected output:

```text
Each one: 3
Left: 2
```

> **Hint:** between two integers, `/` gives the quotient without decimals.

> **Hint:** `%` gives the remainder of the division.

> **Solution**
>
> ```ruby
> candies = 17
> children = 5
> puts "Each one: #{candies / children}"
> puts "Left: #{candies % children}"
> ```
>
> ```sortie
> Each one: 3
> Left: 2
> ```
>
> Why it works: 5 × 3 = 15, and 17 − 15 = 2. Here, integer division is exactly what
> you want (see [the pitfalls](pieges.md)).

## Conditions and loops

### Exercise 5: Positive, negative or zero

Level: ★☆☆

For each number in the list `[4, -2, 0]`, print whether it is positive, negative or zero.

Expected output:

```text
4 is positive
-2 is negative
0 is zero
```

> **Hint:** start with a single number, `n = 4`, with `if`, `elsif` and `else`.

> **Hint:** then, wrap your `if` in a loop `[4, -2, 0].each do |n| ... end`.

> **Solution**
>
> ```ruby
> [4, -2, 0].each do |n|
>   if n > 0
>     puts "#{n} is positive"
>   elsif n < 0
>     puts "#{n} is negative"
>   else
>     puts "#{n} is zero"
>   end
> end
> ```
>
> ```sortie
> 4 is positive
> -2 is negative
> 0 is zero
> ```
>
> Why it works: Ruby tests the conditions in order and stops at the first true one. If
> none is true, it takes the `else`.

### Exercise 6: Counting to 5

Level: ★☆☆

Print the numbers from 1 to 5, one per line.

Expected output:

```text
1
2
3
4
5
```

> **Hint:** `(1..5)` describes the numbers from 1 to 5, 5 included.

> **Solution**
>
> ```ruby
> (1..5).each do |n|
>   puts n
> end
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> ```
>
> Why it works: `each` gives each value to `n`, one per turn. `1.upto(5)` works too.

### Exercise 7: The countdown

Level: ★☆☆

Print a countdown from 3 to 1, then `Liftoff!`.

Expected output:

```text
3...
2...
1...
Liftoff!
```

> **Hint:** `(3..1)` is empty! Use `3.downto(1)`.

> **Solution**
>
> ```ruby
> 3.downto(1) do |n|
>   puts "#{n}..."
> end
> puts "Liftoff!"
> ```
>
> ```sortie
> 3...
> 2...
> 1...
> Liftoff!
> ```
>
> Why it works: `downto` goes down from 3 to 1. The last `puts` is after `end`: it
> runs only once, at the end.

### Exercise 8: The sum from 1 to 10

Level: ★☆☆

Add up the numbers from 1 to 10 with a loop, then print the total.

Expected output:

```text
Total: 55
```

> **Hint:** create `total = 0` **before** the loop.

> **Hint:** in the loop, add the number of the turn: `total += n`.

> **Solution**
>
> ```ruby
> total = 0
> (1..10).each do |n|
>   total += n
> end
> puts "Total: #{total}"
> ```
>
> ```sortie
> Total: 55
> ```
>
> Why it works: `total` grows at each turn.
>
> | Step | n | total |
> |---|---|---|
> | before | – | 0 |
> | turn 1 | 1 | 1 |
> | turn 2 | 2 | 3 |
> | turn 3 | 3 | 6 |
> | … | … | … |
> | turn 10 | 10 | 55 |
>
> In Ruby, `(1..10).sum` does the same thing in one go.

## Methods

### Exercise 9: The square

Level: ★☆☆

Write a method `square` that receives a number and returns this number multiplied by
itself. Print `square(4)` and `square(9)`.

Expected output:

```text
16
81
```

> **Hint:** a method starts with `def square(n)` and ends with `end`.

> **Hint:** the last line of the method is the returned value: `n * n`.

> **Solution**
>
> ```ruby
> def square(n)
>   n * n
> end
>
> puts square(4)
> puts square(9)
> ```
>
> ```sortie
> 16
> 81
> ```
>
> Why it works: when you call `square(4)`, `n` is 4 inside the method. It returns
> `4 * 4`, which `puts` prints. See [methods](fonctions.md).

### Exercise 10: The larger of the two

Level: ★☆☆

Write a method `larger(a, b)` that returns the larger of the two numbers, with an `if`.

Expected output:

```text
8
12
```

for `larger(8, 3)` then `larger(5, 12)`.

> **Hint:** if `a > b`, the answer is `a`. Otherwise, it is `b`.

> **Solution**
>
> ```ruby
> def larger(a, b)
>   if a > b
>     a
>   else
>     b
>   end
> end
>
> puts larger(8, 3)
> puts larger(5, 12)
> ```
>
> ```sortie
> 8
> 12
> ```
>
> Why it works: an `if` returns the value of the chosen branch, and it is the last
> thing in the method. Ruby also has `[a, b].max`.

## Text

### Exercise 11: Shouting

Level: ★☆☆

Write a method `shout(text)` that returns the text in capital letters, followed by `!`.

Expected output:

```text
HELLO!
WATCH OUT!
```

> **Hint:** `"hello".upcase` gives `"HELLO"`.

> **Solution**
>
> ```ruby
> def shout(text)
>   "#{text.upcase}!"
> end
>
> puts shout("hello")
> puts shout("watch out")
> ```
>
> ```sortie
> HELLO!
> WATCH OUT!
> ```
>
> Why it works: `upcase` returns a new text in capital letters. You put it in a text
> that adds `!` at the end.

### Exercise 12: The initials

Level: ★☆☆

Starting from `"Marie Curie"`, print the initials and the number of letters in the
first name.

Expected output:

```text
Initials: M.C.
The first name has 5 letters
```

> **Hint:** `"Marie Curie".split` gives `["Marie", "Curie"]`.

> **Hint:** `word[0]` gives the first letter, `word.length` the number of characters.

> **Solution**
>
> ```ruby
> full_name = "Marie Curie"
> first_name, last_name = full_name.split
> puts "Initials: #{first_name[0]}.#{last_name[0]}."
> puts "The first name has #{first_name.length} letters"
> ```
>
> ```sortie
> Initials: M.C.
> The first name has 5 letters
> ```
>
> Why it works: `split` cuts at the spaces. `first_name, last_name = ...` stores the
> first piece in `first_name` and the second one in `last_name`. See
> [text](texte.md).

## Arrays, hashes and ranges

### Exercise 13: The shopping list

Level: ★☆☆

Store `bread`, `milk` and `apples` in an array. Print them numbered from 1, then the
number of items.

Expected output:

```text
1. bread
2. milk
3. apples
3 items
```

> **Hint:** `each_with_index` gives the element **and** its position (which starts at 0).

> **Hint:** print `i + 1` to start at 1.

> **Solution**
>
> ```ruby
> shopping = ["bread", "milk", "apples"]
> shopping.each_with_index do |item, i|
>   puts "#{i + 1}. #{item}"
> end
> puts "#{shopping.size} items"
> ```
>
> ```sortie
> 1. bread
> 2. milk
> 3. apples
> 3 items
> ```
>
> Why it works: the positions go from 0 to 2. By adding 1, you print 1 to 3, like on
> a real list.

### Exercise 14: The average grade

Level: ★☆☆

With the grades `[12, 15, 9, 16]`, print the highest, the lowest and the average.

Expected output:

```text
Highest: 16
Lowest: 9
Average: 13.0
```

> **Hint:** an array knows `max`, `min`, `sum` and `size`.

> **Hint:** `sum / size` would do an integer division. Use `sum.fdiv(size)`.

> **Solution**
>
> ```ruby
> grades = [12, 15, 9, 16]
> puts "Highest: #{grades.max}"
> puts "Lowest: #{grades.min}"
> puts "Average: #{grades.sum.fdiv(grades.size)}"
> ```
>
> ```sortie
> Highest: 16
> Lowest: 9
> Average: 13.0
> ```
>
> Why it works: 12 + 15 + 9 + 16 = 52, and 52 ÷ 4 = 13. `fdiv` keeps the decimals,
> even when the result is a whole number.

### Exercise 15: The price of fruit

Level: ★☆☆

Store the prices in a hash: apples 2, kiwis 3, pears 4. Print each price, then the label
of a 9-euro basket: `small` up to 5, `medium` from 6 to 10, `large` above.

Expected output:

```text
apples: 2 €
kiwis: 3 €
pears: 4 €
Basket of 9 €: medium
```

> **Hint:** `prices.each do |fruit, amount| ... end` gives each key and its value.

> **Hint:** for the label, a `case` with ranges: `when 0..5`, `when 6..10`.

> **Solution**
>
> ```ruby
> prices = { "apples" => 2, "kiwis" => 3, "pears" => 4 }
> prices.each do |fruit, amount|
>   puts "#{fruit}: #{amount} €"
> end
>
> basket = 9
> label = case basket
>         when 0..5 then "small"
>         when 6..10 then "medium"
>         else "large"
>         end
> puts "Basket of #{basket} €: #{label}"
> ```
>
> ```sortie
> apples: 2 €
> kiwis: 3 €
> pears: 4 €
> Basket of 9 €: medium
> ```
>
> Why it works: a hash keeps the order in which you stored the keys. `9` is in
> `6..10`, so `case` picks `"medium"`. See [ranges](intervalles.md).

## What's next?

Well done if you made it here! Other, more varied exercises are waiting for you in
[the solved exercises](exercices.md). But first, discover blocks: they are everywhere
in Ruby.

Next: [blocks, iterators and Enumerable](blocs.md)
