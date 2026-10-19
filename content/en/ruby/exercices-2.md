# Solved exercises: second set

This page offers 12 new solved exercises in Ruby, from easiest to hardest, with tables that follow the variables step by step.

## How to use this page

These exercises are **different** from the ones in
[exercises to get started](exercices-faciles.md) and in
[solved exercises](exercices.md). Each exercise has a level:

- ★☆☆: easy;
- ★★☆: medium;
- ★★★: harder.

For each exercise:

1. Read the instructions and the expected output.
2. Work it out on paper, with a tracing table (see
   [following a program step by step](pas-a-pas.md)).
3. Write your program in a `.rb` file and run it.
4. Stuck? Read the hint. Open the solution last.

> **Tip:** if your program prints a wrong result, add a `puts` with labels inside the
> loop. You will see on which pass things go wrong.

## Warm-up

### Exercise 1: Celsius to Fahrenheit

Level: ★☆☆

For each temperature in the array `[0, 20, 37, 100]` (in degrees Celsius), print its
value in degrees Fahrenheit. The formula: `f = c * 9 / 5 + 32`.

Expected output:

```text
0 °C = 32.0 °F
20 °C = 68.0 °F
37 °C = 98.6 °F
100 °C = 212.0 °F
```

> **Hint:** `37 * 9 / 5` gives a whole number (66), with no decimal part. Write `9.0`
> instead of `9` to get a decimal number.

> **Solution**
>
> ```ruby
> [0, 20, 37, 100].each do |c|
>   f = c * 9.0 / 5 + 32
>   puts "#{c} °C = #{f} °F"
> end
> ```
>
> ```sortie
> 0 °C = 32.0 °F
> 20 °C = 68.0 °F
> 37 °C = 98.6 °F
> 100 °C = 212.0 °F
> ```
>
> Why it works: as soon as one of the numbers is a decimal (`9.0`), the whole
> calculation is done with decimals. With `9`, Ruby would do an integer division and
> print `98` for 37 °C (see [numbers](nombres.md)).

### Exercise 2: The smallest, by hand

Level: ★☆☆

Find the smallest number in the array `[8, 3, 11, 5]` **without** using `min`.
Use a loop.

Expected output:

```text
Smallest: 3
```

> **Hint:** keep the "record" in a variable `smallest`. Start with the first element,
> `numbers[0]`.

> **Hint:** on each pass, if the number is smaller than the record, it becomes the new
> record.

> **Solution**
>
> ```ruby
> numbers = [8, 3, 11, 5]
> smallest = numbers[0]
> numbers.each do |n|
>   if n < smallest
>     smallest = n
>   end
> end
> puts "Smallest: #{smallest}"
> ```
>
> ```sortie
> Smallest: 3
> ```
>
> Why it works: like a champion who keeps the trophy as long as nobody does better.
>
> | Pass | n | `n < smallest`? | smallest |
> |---|---|---|---|
> | before | – | | 8 |
> | 1 | 8 | `8 < 8` no | 8 |
> | 2 | 3 | `3 < 8` yes | 3 |
> | 3 | 11 | `11 < 3` no | 3 |
> | 4 | 5 | `5 < 3` no | 3 |
>
> We start from `numbers[0]` and not from `0`: with `0`, no positive number would beat
> the record, and the program would print 0.

### Exercise 3: The staircase

Level: ★☆☆

Print a staircase of 4 steps with stars.

Expected output:

```text
*
**
***
****
```

> **Hint:** in Ruby, `"*" * 3` gives `"***"`: you can multiply a string by a number.

> **Solution**
>
> ```ruby
> (1..4).each do |step|
>   puts "*" * step
> end
> ```
>
> ```sortie
> *
> **
> ***
> ****
> ```
>
> Why it works: on each pass, `step` grows by 1, so the line has one more star.
>
> | Pass | step | `"*" * step` |
> |---|---|---|
> | 1 | 1 | `"*"` |
> | 2 | 2 | `"**"` |
> | 3 | 3 | `"***"` |
> | 4 | 4 | `"****"` |

### Exercise 4: Seconds to hours

Level: ★☆☆

A movie lasts `3725` seconds. Print its length in hours, minutes and seconds.

Expected output:

```text
3725 s = 1 h 2 min 5 s
```

> **Hint:** an hour is 3600 seconds. `/` gives the number of whole hours, `%` gives
> what is left.

> **Hint:** do the same with the remainder and 60 for the minutes.

> **Solution**
>
> ```ruby
> total = 3725
> hours = total / 3600
> rest = total % 3600
> minutes = rest / 60
> seconds = rest % 60
> puts "#{total} s = #{hours} h #{minutes} min #{seconds} s"
> ```
>
> ```sortie
> 3725 s = 1 h 2 min 5 s
> ```
>
> Why it works: it is like giving change with the big bills first.
>
> | Line | Calculation | hours | rest | minutes | seconds |
> |---|---|---|---|---|---|
> | 2 | `3725 / 3600` | 1 | – | – | – |
> | 3 | `3725 % 3600` | 1 | 125 | – | – |
> | 4 | `125 / 60` | 1 | 125 | 2 | – |
> | 5 | `125 % 60` | 1 | 125 | 2 | 5 |

## Strings and collections

### Exercise 5: Reverse a word by hand

Level: ★★☆

Write a method `reverse_word(word)` that returns the word written backwards,
**without** using `reverse`. Test it with `"Ruby"`.

Expected output:

```text
ybuR
```

> **Hint:** start from an empty string. Go through the letters with `word.chars.each`.
> Add each letter **in front of** the result, not after it.

> **Solution**
>
> ```ruby
> def reverse_word(word)
>   result = ""
>   word.chars.each do |letter|
>     result = letter + result
>   end
>   result
> end
>
> puts reverse_word("Ruby")
> ```
>
> ```sortie
> ybuR
> ```
>
> Why it works: like a stack of plates, the last letter put down ends up on top, so
> first.
>
> | Pass | letter | `letter + result` | result |
> |---|---|---|---|
> | before | – | | `""` |
> | 1 | `"R"` | `"R" + ""` | `"R"` |
> | 2 | `"u"` | `"u" + "R"` | `"uR"` |
> | 3 | `"b"` | `"b" + "uR"` | `"buR"` |
> | 4 | `"y"` | `"y" + "buR"` | `"ybuR"` |

### Exercise 6: The longest word

Level: ★★☆

In the sentence `"the small kitten sleeps on the sofa"`, find the longest word and print
it with its length.

Expected output:

```text
kitten (6 letters)
```

> **Hint:** `split` cuts the sentence into words. `max_by { |word| word.length }` keeps
> the element for which the block gives the biggest number.

> **Solution**
>
> ```ruby
> sentence = "the small kitten sleeps on the sofa"
> longest = sentence.split.max_by { |word| word.length }
> puts "#{longest} (#{longest.length} letters)"
> ```
>
> ```sortie
> kitten (6 letters)
> ```
>
> Why it works: `max_by` measures each word with the block. `kitten` and `sleeps` both
> have 6 letters: when there is a tie, `max_by` keeps the **first** one.
> See [blocks](blocs.md).

### Exercise 7: Sort by initial

Level: ★★☆

Sort the first names `["Alice", "Bob", "Anna", "Chloé", "Bruno"]` by their first
letter. Print one line per letter.

Expected output:

```text
A: Alice, Anna
B: Bob, Bruno
C: Chloé
```

> **Hint:** `name[0]` gives the first letter. A hash can link each letter to an array
> of names.

> **Hint:** `group_by` does this job in one go: it sorts the elements according to the
> value returned by the block.

> **Solution**
>
> ```ruby
> names = ["Alice", "Bob", "Anna", "Chloé", "Bruno"]
> groups = names.group_by { |name| name[0] }
> groups.each do |letter, list|
>   puts "#{letter}: #{list.join(", ")}"
> end
> ```
>
> ```sortie
> A: Alice, Anna
> B: Bob, Bruno
> C: Chloé
> ```
>
> Why it works: `group_by` makes labeled drawers. Here, it builds the hash
> `{"A" => ["Alice", "Anna"], "B" => ["Bob", "Bruno"], "C" => ["Chloé"]}`. `join` glues
> the names back together with a comma.

### Exercise 8: The sum of the digits

Level: ★★☆

Write a method `digit_sum(n)` that adds up the digits of a number, without turning it
into a string. Test it with `2026`.

Expected output:

```text
10
```

> **Hint:** `n % 10` gives the last digit (`2026 % 10` is 6). `n / 10` removes that
> last digit (`2026 / 10` is 202).

> **Hint:** repeat with `while n > 0`.

> **Solution**
>
> ```ruby
> def digit_sum(n)
>   sum = 0
>   while n > 0
>     sum += n % 10
>     n = n / 10
>   end
>   sum
> end
>
> puts digit_sum(2026)
> ```
>
> ```sortie
> 10
> ```
>
> Why it works: we "eat" the number from the right, one digit per pass.
>
> | Pass | Test `n > 0` | `n % 10` | sum | n |
> |---|---|---|---|---|
> | before | | | 0 | 2026 |
> | 1 | yes | 6 | 6 | 202 |
> | 2 | yes | 2 | 8 | 20 |
> | 3 | yes | 0 | 8 | 2 |
> | 4 | yes | 2 | 10 | 0 |
> | end | `0 > 0` no | | 10 | 0 |

### Exercise 9: Check a password

Level: ★★☆

Write a method `problems(password)` that returns the list of problems of a password:

- fewer than 8 characters: `"too short"`;
- no digit: `"no digit"`;
- no uppercase letter: `"no uppercase"`.

Test it with `"cat"`, `"kitten2026"` and `"Kitten2026"`.

Expected output:

```text
cat: too short, no digit, no uppercase
kitten2026: no uppercase
Kitten2026: OK
```

> **Hint:** start from an empty array `list = []` and add a message with `<<` for each
> rule that is not met.

> **Hint:** `password.chars.any? { |c| ("0".."9").include?(c) }` answers `true` if there
> is at least one digit. For an uppercase letter, use `("A".."Z")`.

> **Solution**
>
> ```ruby
> def problems(password)
>   list = []
>   list << "too short" if password.length < 8
>   list << "no digit" unless password.chars.any? { |c| ("0".."9").include?(c) }
>   list << "no uppercase" unless password.chars.any? { |c| ("A".."Z").include?(c) }
>   list
> end
>
> ["cat", "kitten2026", "Kitten2026"].each do |password|
>   list = problems(password)
>   if list.empty?
>     puts "#{password}: OK"
>   else
>     puts "#{password}: #{list.join(", ")}"
>   end
> end
> ```
>
> ```sortie
> cat: too short, no digit, no uppercase
> kitten2026: no uppercase
> Kitten2026: OK
> ```
>
> Why it works: each rule is a separate line, like a checklist before leaving on a
> trip. A rule that is not met adds its message. An empty array at the end means
> "everything is fine".

## Going further

### Exercise 10: Guess the number

Level: ★★★

The computer must find a secret number between 1 and 100. On each try, it only learns
"too low" or "too high". Write a program that finds `37` by always guessing the
**middle** of the range that is still possible.

Expected output:

```text
Try 1: 50, too high
Try 2: 25, too low
Try 3: 37, found!
```

> **Hint:** keep two variables, `low = 1` and `high = 100`. The middle is
> `(low + high) / 2`.

> **Hint:** if the middle is too high, the secret is below it: `high = middle - 1`.
> If it is too low: `low = middle + 1`.

> **Solution**
>
> ```ruby
> secret = 37
> low = 1
> high = 100
> try = 0
> loop do
>   try += 1
>   middle = (low + high) / 2
>   if middle == secret
>     puts "Try #{try}: #{middle}, found!"
>     break
>   elsif middle > secret
>     puts "Try #{try}: #{middle}, too high"
>     high = middle - 1
>   else
>     puts "Try #{try}: #{middle}, too low"
>     low = middle + 1
>   end
> end
> ```
>
> ```sortie
> Try 1: 50, too high
> Try 2: 25, too low
> Try 3: 37, found!
> ```
>
> Why it works: on each try, we throw away half of the possible numbers, like when you
> look for a word in a dictionary by opening it in the middle.
>
> | Try | low | high | middle | Answer | New limit |
> |---|---|---|---|---|---|
> | 1 | 1 | 100 | 50 | too high | high = 49 |
> | 2 | 1 | 49 | 25 | too low | low = 26 |
> | 3 | 26 | 49 | 37 | found | |
>
> With this method, 7 tries are always enough for a number between 1 and 100.

### Exercise 11: Properly closed brackets

Level: ★★★

Write a method `balanced?(text)` that checks that the parentheses `()`, square brackets
`[]` and curly braces `{}` are properly closed, in the right order. Test it with
`"(a[b]{c})"`, `"(a]"` and `"(("`.

Expected output:

```text
(a[b]{c}) : true
(a] : false
(( : false
```

> **Hint:** use an array like a stack of plates. An opening sign: you put it on top
> with `push`. A closing sign: you take the top one off with `pop`, and it must match.

> **Hint:** a hash `{ ")" => "(", "]" => "[", "}" => "{" }` gives the expected opening
> sign for each closing sign. At the end, the stack must be empty.

> **Solution**
>
> ```ruby
> PAIRS = { ")" => "(", "]" => "[", "}" => "{" }
>
> def balanced?(text)
>   stack = []
>   text.chars.each do |c|
>     if PAIRS.value?(c)
>       stack.push(c)
>     elsif PAIRS.key?(c)
>       return false if stack.pop != PAIRS[c]
>     end
>   end
>   stack.empty?
> end
>
> ["(a[b]{c})", "(a]", "(("].each do |text|
>   puts "#{text} : #{balanced?(text)}"
> end
> ```
>
> ```sortie
> (a[b]{c}) : true
> (a] : false
> (( : false
> ```
>
> Why it works: the last sign opened must be the first one closed. Letters are ignored.
> Here is the trace for `"(a[b]{c})"`:
>
> | c | Action | stack after |
> |---|---|---|
> | `(` | put on | `["("]` |
> | `a` | nothing | `["("]` |
> | `[` | put on | `["(", "["]` |
> | `b` | nothing | `["(", "["]` |
> | `]` | take off `[`, it matches | `["("]` |
> | `{` | put on | `["(", "{"]` |
> | `c` | nothing | `["(", "{"]` |
> | `}` | take off `{`, it matches | `["("]` |
> | `)` | take off `(`, it matches | `[]` |
>
> For `"(a]"`, the `]` takes off `(` instead of `[`: `false`. For `"(("`, the stack is
> not empty at the end: `false`.

### Exercise 12: Compress a string

Level: ★★★

Write a method `compress(text)` that replaces each run of identical letters with the
letter followed by the number of repetitions. Test it with `"aaabccdddd"`.

Expected output:

```text
a3b1c2d4
```

> **Hint:** keep the current letter (`current`) and its counter (`count`). When the
> letter changes, add `current` and `count` to the result, then start again with the
> new letter.

> **Hint:** do not forget to add the **last** run after the loop.

> **Solution**
>
> ```ruby
> def compress(text)
>   result = ""
>   current = text[0]
>   count = 0
>   text.chars.each do |letter|
>     if letter == current
>       count += 1
>     else
>       result += "#{current}#{count}"
>       current = letter
>       count = 1
>     end
>   end
>   result + "#{current}#{count}"
> end
>
> puts compress("aaabccdddd")
> ```
>
> ```sortie
> a3b1c2d4
> ```
>
> Why it works: it is like counting cars of the same color driving past in a line. When
> the color changes, you write down the total and start counting again.
>
> | Pass | letter | Same letter? | result | current | count |
> |---|---|---|---|---|---|
> | before | – | | `""` | `"a"` | 0 |
> | 1 | a | yes | `""` | `"a"` | 1 |
> | 2 | a | yes | `""` | `"a"` | 2 |
> | 3 | a | yes | `""` | `"a"` | 3 |
> | 4 | b | no | `"a3"` | `"b"` | 1 |
> | 5 | c | no | `"a3b1"` | `"c"` | 1 |
> | 6 | c | yes | `"a3b1"` | `"c"` | 2 |
> | 7 | d | no | `"a3b1c2"` | `"d"` | 1 |
> | 8 to 10 | d | yes | `"a3b1c2"` | `"d"` | 4 |
>
> After the loop, the last run (`d4`) is not in `result` yet: the last line of the
> method adds it.

## Summary

- Exercises 1 to 4 practice calculations, simple loops and the "record".
- Exercises 5 to 9 practice strings, arrays, hashes and blocks.
- Exercises 10 to 12 ask you to keep several variables in mind: make the tracing table
  on paper.
- When a solution looks like magic, follow it step by step, pass by pass.

Next: [the Ruby cheat sheet](aide-memoire.md)
