# Solved exercises

This page offers 12 solved exercises, from easiest to hardest, to practice the whole Ruby track.

## How to use this page

Each exercise has a level:

- ★☆☆: easy;
- ★★☆: medium;
- ★★★: harder.

Read the instructions, then write your solution in a `.rb` file and run it. Open the
solution only after you have tried.

> **Tip:** there are often several good solutions. If your program prints the right
> result, you win!

## The basics

### Exercise 1: Introduce yourself

Level: ★☆☆

Create two variables, `first_name` and `age`. Then print the sentence
`My name is Alice and I am 30 years old.`, using your variables.

> **Hint:** use interpolation `#{...}` (see [first steps](premiers-pas.md)).

> **Solution**
>
> ```ruby
> first_name = "Alice"
> age = 30
> puts "My name is #{first_name} and I am #{age} years old."
> ```
>
> ```sortie
> My name is Alice and I am 30 years old.
> ```

### Exercise 2: Even or odd

Level: ★☆☆

For the number `7`, print `7 is odd`. For `10`, the program must print
`10 is even`.

> **Hint:** `n.even?` is `true` if `n` is even.

> **Solution**
>
> ```ruby
> [7, 10].each do |n|
>   if n.even?
>     puts "#{n} is even"
>   else
>     puts "#{n} is odd"
>   end
> end
> ```
>
> ```sortie
> 7 is odd
> 10 is even
> ```

### Exercise 3: The 7 times table

Level: ★☆☆

Print the multiplication table of 7, from `7 x 1 = 7` to `7 x 5 = 35`.

> **Hint:** a `(1..5).each` loop runs 5 times.

> **Solution**
>
> ```ruby
> (1..5).each do |i|
>   puts "7 x #{i} = #{7 * i}"
> end
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

### Exercise 4: The area of a rectangle

Level: ★☆☆

Write a method `area(width, height)` that returns the area of a rectangle.
Print the area of a 4 by 3 rectangle.

> **Solution**
>
> ```ruby
> def area(width, height)
>   width * height
> end
>
> puts area(4, 3)
> ```
>
> ```sortie
> 12
> ```

## Text and collections

### Exercise 5: The average

Level: ★★☆

With the array `grades = [12, 15, 9, 14]`, compute and print the average.

> **Hint:** `sum` gives the total, `length` the number of grades. Remember to divide
> with a decimal number (`to_f`).

> **Solution**
>
> ```ruby
> grades = [12, 15, 9, 14]
> average = grades.sum.to_f / grades.length
> puts "Average: #{average}"
> ```
>
> ```sortie
> Average: 12.5
> ```

### Exercise 6: Counting vowels

Level: ★★☆

Write a method `vowels(text)` that counts the vowels (a, e, i, o, u, y) in a
text. Test it with `"Hello Ruby"`.

> **Hint:** convert the text to lowercase, then go through `chars`.

> **Solution**
>
> ```ruby
> def vowels(text)
>   count = 0
>   text.downcase.chars.each do |letter|
>     count += 1 if "aeiouy".include?(letter)
>   end
>   count
> end
>
> puts vowels("Hello Ruby")
> ```
>
> ```sortie
> 4
> ```

### Exercise 7: The palindrome

Level: ★★☆

A palindrome reads the same both ways, like "kayak". Write a method
`palindrome?(word)` that answers `true` or `false`. Ignore uppercase letters.

> **Hint:** compare the word with `word.reverse` (see [text](texte.md)).

> **Solution**
>
> ```ruby
> def palindrome?(word)
>   word = word.downcase
>   word == word.reverse
> end
>
> puts palindrome?("Kayak")
> puts palindrome?("Ruby")
> ```
>
> ```sortie
> true
> false
> ```

### Exercise 8: Counting words

Level: ★★☆

With the sentence `"the cat and the dog and the rabbit"`, count how many times each
word appears. Print one line per word.

> **Hint:** use a hash created with `Hash.new(0)`: a missing key is then worth `0`.

> **Solution**
>
> ```ruby
> sentence = "the cat and the dog and the rabbit"
> count = Hash.new(0)
> sentence.split(" ").each do |word|
>   count[word] += 1
> end
> count.each do |word, number|
>   puts "#{word}: #{number}"
> end
> ```
>
> ```sortie
> the: 3
> cat: 1
> and: 2
> dog: 1
> rabbit: 1
> ```

### Exercise 9: Keep and double

Level: ★★☆

With the numbers from 1 to 10, keep only the even numbers, then double them.
Print the resulting array.

> **Hint:** `select` filters, `map` transforms (see [blocks](blocs.md)).

> **Solution**
>
> ```ruby
> result = (1..10).select { |n| n.even? }.map { |n| n * 2 }
> p result
> ```
>
> ```sortie
> [4, 8, 12, 16, 20]
> ```

## Going further

### Exercise 10: FizzBuzz

Level: ★★☆

Print the numbers from 1 to 15. But:

- instead of multiples of 3, print `Fizz`;
- instead of multiples of 5, print `Buzz`;
- instead of multiples of 3 **and** 5, print `FizzBuzz`.

> **Hint:** `n % 3 == 0` is true if `n` is a multiple of 3. Test the "3 and 5" case
> first.

> **Solution**
>
> ```ruby
> (1..15).each do |n|
>   if n % 3 == 0 && n % 5 == 0
>     puts "FizzBuzz"
>   elsif n % 3 == 0
>     puts "Fizz"
>   elsif n % 5 == 0
>     puts "Buzz"
>   else
>     puts n
>   end
> end
> ```
>
> ```sortie
> 1
> 2
> Fizz
> 4
> Buzz
> Fizz
> 7
> 8
> Fizz
> Buzz
> 11
> Fizz
> 13
> 14
> FizzBuzz
> ```

### Exercise 11: The bank account

Level: ★★★

Create an `Account` class with a balance that starts at `0`. Add:

- a `deposit(amount)` method that adds money;
- a `withdraw(amount)` method that takes money out, only if the balance is enough;
- a way to read the balance with `attr_reader`.

> **Hint:** store the balance in `@balance`, in `initialize` (see
> [classes](types.md)).

> **Solution**
>
> ```ruby
> class Account
>   attr_reader :balance
>
>   def initialize
>     @balance = 0
>   end
>
>   def deposit(amount)
>     @balance += amount
>   end
>
>   def withdraw(amount)
>     if amount > @balance
>       puts "Insufficient balance"
>     else
>       @balance -= amount
>     end
>   end
> end
>
> account = Account.new
> account.deposit(100)
> account.withdraw(30)
> account.withdraw(500)
> puts "Balance: #{account.balance}"
> ```
>
> ```sortie
> Insufficient balance
> Balance: 70
> ```

### Exercise 12: A valid age

Level: ★★★

Write a method `read_age(text)` that turns a text into an age:

- if the text is not a number, it returns `nil`;
- if the age is negative, it raises an `ArgumentError`.

Test it with `"25"`, `"abc"` and `"-3"`, without the program stopping.

> **Hint:** `Integer("abc")` raises an `ArgumentError`. Use `rescue` (see
> [handling errors](erreurs.md)).

> **Solution**
>
> ```ruby
> def read_age(text)
>   begin
>     age = Integer(text)
>   rescue ArgumentError
>     return nil
>   end
>   raise ArgumentError, "negative age" if age < 0
>   age
> end
>
> ["25", "abc", "-3"].each do |text|
>   begin
>     age = read_age(text)
>     if age.nil?
>       puts "#{text}: not a number"
>     else
>       puts "#{text}: #{age} years old"
>     end
>   rescue ArgumentError => e
>     puts "#{text}: error, #{e.message}"
>   end
> end
> ```
>
> ```sortie
> 25: 25 years old
> abc: not a number
> -3: error, negative age
> ```

## Summary

- Start with the ★☆☆ exercises, then move up in difficulty.
- Always try on your own before opening the solution.
- Exercises 1 to 4 review the basics: variables, conditions, loops, methods.
- Exercises 5 to 9 work on text, arrays, hashes and blocks.
- Exercises 10 to 12 combine everything, with classes and errors.

Next: [solved exercises: second set](exercices-2.md)
