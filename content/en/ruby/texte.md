# Working with text

This page shows how to create, join, split and transform text in Ruby.

> **Goals:**
>
> - you will know how to write a text and insert values into it;
> - you will know how to transform a text (uppercase, spaces, replacements);
> - you will know how to search, split and extract part of a text;
> - you will know how to turn a text into a number, and the other way around;
> - you will know how to print a number neatly with `format`.

## Creating a text

Almost every program handles text: a name to greet, a message to show, an answer
typed on the keyboard.

A text is called a **string** (`String`), or
[character string](../debuter/glossaire.md#string): a sequence of
characters (letters, digits, spaces, symbols) strung together like beads. You write it
between double quotes `"..."` or single quotes `'...'`.

```ruby
greeting = "Hello"
name = 'Alice'
puts greeting
puts name
```

```sortie
Hello
Alice
```

> **Tip:** get into the habit of using double quotes. They allow interpolation,
> explained just below.

### Special characters

Between double quotes, the backslash `\` announces a special character. This is called
an **escape sequence**:

- `\n` starts a new line;
- `\t` inserts a tab (a wide space);
- `\"` writes a double quote without closing the text;
- `\\` writes a backslash `\`.

```ruby
puts "Line 1\nLine 2"
puts "Name\tAge"
puts "He said: \"Hello\""
```

```sortie
Line 1
Line 2
Name	Age
He said: "Hello"
```

## Joining texts

A message is often made of pieces: a fixed part and a part that changes. It is like a
form letter where you fill in the name by hand.

### With +

The `+` sign sticks two texts end to end.

```ruby
first_name = "Alice"
puts "Hello " + first_name + "!"
```

```sortie
Hello Alice!
```

`+` only works between two texts. For a number, convert it first with `to_s`:

```ruby
age = 30
puts "I am " + age.to_s + " years old."
```

```sortie
I am 30 years old.
```

### With interpolation

Between double quotes, `#{...}` puts a value inside the text. It is simpler than `+`,
and it works with numbers.

```ruby
first_name = "Alice"
age = 30
puts "#{first_name} is #{age} years old."
puts "Next year: #{age + 1} years old."
```

```sortie
Alice is 30 years old.
Next year: 31 years old.
```

> **Warning:** with single quotes, `'#{age}'` stays as it is. Ruby replaces
> nothing.

### Repeating a text

`*` repeats a text several times.

```ruby
puts "ha" * 3
puts "-" * 10
```

```sortie
hahaha
----------
```

**Your turn:** write a program that stores `"Leo"` in `first_name` and `8` in `age`,
then prints "Leo will be 9 next year." using interpolation.

> **Solution**
>
> ```ruby
> first_name = "Leo"
> age = 8
> puts "#{first_name} will be #{age + 1} next year."
> ```
>
> ```sortie
> Leo will be 9 next year.
> ```

## The length of a text

Knowing the length helps you check what someone typed: a password that is too short,
a box left empty.

`length` gives the number of characters. Spaces count too.

```ruby
puts "cat".length
puts "a cat".length
puts "".empty?
```

```sortie
3
5
true
```

Line by line:

- `"cat".length` counts c, a, t: 3 characters;
- `"a cat".length` gives 5, because the space between the two words counts too;
- `""` is an empty text, with no characters at all: so `empty?` answers `true`.

`empty?` answers `true` when the text is empty.

## Uppercase and lowercase

People do not all type the same way: "alice", "ALICE", "Alice". These methods put a
text back into shape, to display it neatly or to compare it.

- `upcase` puts everything in uppercase.
- `downcase` puts everything in lowercase.
- `capitalize` puts a capital letter at the start, and the rest in lowercase.

```ruby
word = "helLO"
puts word.upcase
puts word.downcase
puts word.capitalize
```

```sortie
HELLO
hello
Hello
```

> **Note:** these methods return a **new** text. The variable `word` does not
> change.

To change the text itself, add a `!`: `word.upcase!`. This is the convention seen
in [methods](fonctions.md#names-ending-in-and).

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

## Removing the spaces around

An extra space cannot be seen on the screen. Yet for Ruby, `"Alice "` and `"Alice"`
are two different texts.

`strip` removes the spaces (and line breaks) at the start and at the end.

```ruby
input = "   Alice   "
puts "[" + input + "]"
puts "[" + input.strip + "]"
```

```sortie
[   Alice   ]
[Alice]
```

Line by line:

- `input` contains "Alice", with three spaces on each side;
- the second line sticks brackets around the text: they show where it starts and
  where it ends, spaces included;
- the third line does the same with `input.strip`: the spaces are gone, the brackets
  touch the name.

> **Tip:** use `strip` on any text typed by someone: people often add spaces
> without meaning to.

**Your turn:** write a program that stores `"  alice  "` in `input`, then prints
"Hello Alice!", without the spaces and with a capital letter.

> **Solution**
>
> ```ruby
> input = "  alice  "
> name = input.strip
> puts "Hello #{name.capitalize}!"
> ```
>
> ```sortie
> Hello Alice!
> ```

## Searching in a text

Searching for a piece of text is useful everywhere: spotting a word in a message,
checking that an address ends with ".com".

- `include?`: does the text contain this piece?
- `start_with?`: does it start with this piece?
- `end_with?`: does it end with this piece?

```ruby
sentence = "I love chocolate"
puts sentence.include?("choco")
puts sentence.start_with?("I love")
puts sentence.end_with?(".")
```

```sortie
true
true
false
```

Line by line:

- `sentence` holds the text to examine;
- `include?("choco")` looks for "choco" anywhere: it is in "chocolate", so `true`;
- `start_with?("I love")` only looks at the start: `true`;
- `end_with?(".")` only looks at the end: the sentence has no final period, so
  `false`.

> **Warning:** the search is case-sensitive. `"Cat".include?("cat")` is `false`.
> Use `downcase` to compare without caring about uppercase.

### Comparing, counting and finding a position

- `==` compares two texts, letter by letter;
- `casecmp?` compares without caring about uppercase;
- `count` counts characters;
- `index` gives the position of a piece (or `nil` if it is missing).

```ruby
puts "Cat" == "cat"
puts "Cat".casecmp?("cat")
puts "banana".count("a")
p "banana".index("n")
p "banana".index("z")
```

```sortie
false
true
3
2
nil
```

## Splitting and joining back

A text often holds several pieces of information: the words of a sentence, a shopping
list separated by commas. To handle them one by one, you must separate them.

`split` cuts a text into pieces and returns an array. `join` does the opposite: it
sticks an array back together into a single text.

```ruby
sentence = "apple banana cherry"
fruits = sentence.split(" ")
p fruits

list = "red,green,blue".split(",")
p list
puts list.join(" / ")
```

```sortie
["apple", "banana", "cherry"]
["red", "green", "blue"]
red / green / blue
```

Line by line:

- `sentence.split(" ")` cuts at each space: three pieces, stored in the array
  `fruits`;
- `p fruits` prints the array, with its brackets and quotes;
- `"red,green,blue".split(",")` cuts at each comma this time;
- `list.join(" / ")` sticks the three colors back together, with "/" between them.

Arrays are explained in [arrays and hashes](collections.md).

**Your turn:** write a program that cuts `"bread;milk;eggs"` at each semicolon, then
joins the pieces back with ", " between them.

> **Solution**
>
> ```ruby
> shopping = "bread;milk;eggs".split(";")
> puts shopping.join(", ")
> ```
>
> ```sortie
> bread, milk, eggs
> ```

## Replacing a piece

It is the "Find and replace" feature of a word processor: you fix a word, or you
change a name everywhere at once.

`sub` replaces the **first** time a piece appears. `gsub` replaces **all** of
them.

```ruby
text = "a cat, a dog"
puts text.sub("a ", "the ")
puts text.gsub("a ", "the ")
```

```sortie
the cat, a dog
the cat, the dog
```

Line by line:

- `text` contains "a " twice;
- `sub("a ", "the ")` only changes the first one;
- `gsub("a ", "the ")` changes both;
- `text` itself does not change: each line starts again from the original text.

> **Remember:** think "g" as in "global": `gsub` replaces everywhere.

## Taking part of the text

Sometimes only part of the text matters to you: the initial of a name, the year in a
date.

As in an array, each character has a position, called an
[index](../debuter/glossaire.md#index). The first one is `0`, not `1`: the index
counts how many steps you take from the start.

```ruby
word = "sunshine"
puts word[0]
puts word[-1]
puts word[0..2]
puts word[3..]
```

```sortie
s
e
sun
shine
```

- `word[0]`: the first character.
- `word[-1]`: the last one.
- `word[0..2]`: from position 0 to position 2, included.
- `word[3..]`: from position 3 to the end.

A position that does not exist gives `nil` (no value):

```ruby
p "cat"[10]
```

```sortie
nil
```

## Characters one by one

To examine each letter of a word, to count them or transform them, you first need to
separate them.

`chars` gives the array of all the characters. Handy for going through them.

```ruby
p "abc".chars
"abc".chars.each { |letter| puts letter.upcase }
puts "abc".reverse
```

```sortie
["a", "b", "c"]
A
B
C
cba
```

Line by line:

- `"abc".chars` gives the array `["a", "b", "c"]`, which `p` prints;
- `each { |letter| ... }` takes each character in turn, stores it in `letter`, then
  runs `puts letter.upcase`: A, then B, then C;
- `reverse` reads the text from the end to the start.

`reverse` returns the text backwards.

## A text on several lines

> **Note:** this part can wait. Read it once the basics feel solid.

Writing a whole letter on a single line, with `\n` everywhere, would be hard to read.
A heredoc lets you write the text the way it will be displayed.

For a long text, use a **heredoc**: `<<~END`, then your lines, then `END`
alone on its line.

```ruby
name = "Alice"
letter = <<~END
  Hello #{name},
  Thank you for your visit.
  See you soon!
END
puts letter
```

```sortie
Hello Alice,
Thank you for your visit.
See you soon!
```

> **Note:** the `~` removes the spaces at the start of the lines. So you can indent
> your text neatly. The word `END` is up to you: `TEXT` or `MSG` work too.

## Text and numbers

A "42" typed on the keyboard is a text, not a number: Ruby refuses to calculate with
it. You must convert it first. And to display a number neatly, you format it.

### Going from one to the other

- `to_i` turns a text into a whole number, `to_f` into a decimal number.
- `to_s` turns a number into a text.

```ruby
puts "42".to_i + 1
puts "2.5".to_f * 2
puts 10.to_s + "0"
```

```sortie
43
5.0
100
```

Line by line:

- `"42".to_i` turns the text into the number 42: you can then add 1, which gives 43;
- `"2.5".to_f` gives the number 2.5; times 2, that makes 5.0;
- `10.to_s` gives the text `"10"`; `+ "0"` sticks a zero at the end: `"100"`. No
  calculation happens.

### Aligning text

`ljust` pads on the right with spaces, `rjust` on the left, `center` on both sides.
Handy for making columns.

```ruby
puts "bread".ljust(8, ".") + "$1.20"
puts "marmalade".ljust(8, ".") + "$3.50"
puts "42".rjust(5, "0")
puts "title".center(11, "*")
```

```sortie
bread...$1.20
marmalade$3.50
00042
***title***
```

The text is never cut: `"marmalade"` is already longer than 8 characters.

### Rounding and formatting

`round(2)` rounds to 2 digits after the decimal point. `format` builds a text from a
template: `%.2f` means "a number with 2 digits after the decimal point".

```ruby
price = 12.34567
puts price.round(2)
puts format("$%.2f", price)
puts format("$%.2f", 5)
puts format("Ticket no. %03d", 7)
```

```sortie
12.35
$12.35
$5.00
Ticket no. 007
```

> **Tip:** `%03d` writes a whole number with 3 digits, with zeros in front.

**Your turn:** write a program that stores the text `"2.5"` (a price) in `price` and
`3` in `quantity`, then prints the total: "Total: $7.50".

> **Solution**
>
> ```ruby
> price = "2.5"
> quantity = 3
> total = price.to_f * quantity
> puts format("Total: $%.2f", total)
> ```
>
> ```sortie
> Total: $7.50
> ```

## Frozen texts

> **Note:** this part can wait. Read it once the basics feel solid.

Freezing a text protects it: a value that must never change, like the name of your
website, can no longer be modified by mistake.

A **frozen** text (`freeze`) can no longer be changed: it becomes
[immutable](../debuter/glossaire.md#immutable). To change it, make a
copy with `dup`.

```ruby
word = "hello".freeze
puts word.frozen?
copy = word.dup
copy << "!"
puts copy
```

```sortie
true
hello!
```

Line by line:

- `"hello".freeze` creates a frozen text, stored in `word`;
- `word.frozen?` checks that it is indeed frozen: `true`;
- `word.dup` makes a copy, which is not frozen;
- `copy << "!"` adds "!" at the end of the copy. `word` is still `"hello"`.

`<<` adds text at the end of a string. On a frozen text, it raises a
`FrozenError`.

> **Note:** you will sometimes see the line `# frozen_string_literal: true` at the top of a
> file. It freezes all the texts written in that file. Methods like `upcase` still
> work: they create a new text.

## Common mistakes

### Forgetting to convert a text into a number

```ruby norun
age = "30"
puts age + 1
```

```text
main.rb:2:in `+': no implicit conversion of Integer into String (TypeError)

puts age + 1
           ^
	from main.rb:2:in `<main>'
```

`"30"` is a text, not a number. Convert it first: `age.to_i + 1`.

### Using a position that does not exist

```ruby norun
word = "cat"
puts word[10].upcase
```

```text
main.rb:2:in `<main>': undefined method `upcase' for nil (NoMethodError)

puts word[10].upcase
             ^^^^^^^
```

`word[10]` is `nil`, and `nil` does not know how to `upcase`. Check the length
(`word.length`), or use `&.`: `word[10]&.upcase` gives `nil` without crashing.

### Changing a frozen text

```ruby norun
word = "hello".freeze
word << "!"
```

```text
main.rb:2:in `<main>': can't modify frozen String: "hello" (FrozenError)
```

A frozen text no longer changes. Create a new text (`word + "!"`), or change a
copy (`word.dup << "!"`).

## Test yourself

**Question 1:** what does `puts "Ruby".downcase.reverse` print?

> **Answer**
>
> `ybur`. `downcase` gives `"ruby"`, then `reverse` turns it around. Methods are
> chained from left to right.

**Question 2:** how do you get `"Hello Alice"` from `"  hello alice "`?

> **Answer**
>
> ```ruby
> text = "  hello alice "
> puts text.strip.split(" ").map(&:capitalize).join(" ")
> ```
>
> ```sortie
> Hello Alice
> ```
>
> `strip` removes the spaces, `split` cuts into words, `map(&:capitalize)` puts a
> capital letter on each word (see [blocks](blocs.md)), `join` sticks them back.

**Question 3:** what is `"12" + "3"`? And `"12".to_i + "3".to_i`?

> **Answer**
>
> `"123"` (two texts stuck together), then `15` (two numbers added).

## Summary

- A text is written between double quotes; `#{...}` inserts a value into it.
- `length`, `upcase`, `downcase`, `capitalize` and `strip` return a new text.
- `include?`, `start_with?` and `end_with?` search for a piece.
- `split` cuts into an array, `join` sticks back; `sub` and `gsub` replace.
- `word[0..2]` takes part of the text, `chars` gives the characters one by one.
- `round` and `format` print numbers neatly; `to_i` and `to_s` convert.

Next: [arrays and hashes](collections.md). Compare with
[text in Rust](../rust/texte.md).
