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

## The length of a text

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

`empty?` answers `true` when the text is empty.

## Uppercase and lowercase

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

> **Tip:** use `strip` on any text typed by someone: people often add spaces
> without meaning to.

## Searching in a text

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

Arrays are explained in [arrays and hashes](collections.md).

## Replacing a piece

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

> **Remember:** think "g" as in "global": `gsub` replaces everywhere.

## Taking part of the text

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

`reverse` returns the text backwards.

## A text on several lines

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

## Frozen texts

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
