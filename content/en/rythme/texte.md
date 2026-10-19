# Working with text

This page shows Rythme's tools for text: inserting values, changing case, splitting,
searching, replacing, aligning columns, reading a number, and a first look at regular
expressions.

> **Goals:**
>
> - you will know how to build a text with inserted values and special characters;
> - you will know how to transform a text: case, spaces, replacing;
> - you will know how to split a text into pieces and join a list back together;
> - you will know how to read a number typed as text, without crashing;
> - you will know how to search for pieces of text with a simple regular expression.

## Inserting a value into a text

A program often talks to someone: it shows a first name, a total, a score.
Inserting a value is like filling in the blanks of a form letter: the text is ready,
only the values are missing.

A text, also called a
[string](../debuter/glossaire.md#string), is
a sequence of characters: letters, digits, spaces, symbols.

You know it since the [first steps](premiers-pas.md): a value between
braces `{...}` is inserted into the text. You can put a calculation or a
function call there:

```rythme
name = "Ada"
fruits = ["apple", "pear"]
print("Hello {name}!")
print("{name} has {len(fruits)} fruits.")
print("Twice 21 is {21 * 2}.")
```

```sortie
Hello Ada!
Ada has 2 fruits.
Twice 21 is 42.
```

Line by line:

- `name` gets the text `"Ada"`, and `fruits` a list of two texts;
- in `"Hello {name}!"`, Rythme replaces `{name}` with its value, `Ada`, then shows
  the text;
- `{len(fruits)}` is calculated first: the list has 2 elements, so 2 goes into the
  text;
- `{21 * 2}` does the calculation, 42, and puts it in its place.

> **Tip:** to turn any value into text, use `text`: `text(42)` is `"42"`.

## Special characters

A text is written between double quotes. But how do you write a double quote
**inside** the text, or start a new line? You need a way to say "this character is
special".

Some characters can't be typed directly in a text. You write them with a
backslash `\` in front: an **escape sequence**.

| Write | To get |
|--------|--------------|
| `\n` | a line break |
| `\t` | a tab (a wide space, for aligning) |
| `\"` | a double quote, without closing the text |
| `\\` | a backslash |
| `\{` and `\}` | braces, without insertion |

```rythme
print("She said: \"yes\".")
print("line 1\nline 2")
print("Some braces: \{ and \}")
```

```sortie
She said: "yes".
line 1
line 2
Some braces: { and }
```

Line by line:

- `\"` shows a double quote: the text doesn't stop there;
- `\n` breaks the line: `line 2` shows up below `line 1`;
- `\{` and `\}` show braces, without looking for a value to insert.

## Length and case

A program often needs to check or tidy up a text: is a password long enough?
Should a first name typed "élise" be shown as "Élise"?

`len` counts the characters. `upper` puts everything in uppercase, `lower`
everything in lowercase, and `capitalize` only the first letter:

```rythme
word = "élise"
print(len(word))
print(upper(word), lower("HELLO"), capitalize(word))
```

```sortie
5
ÉLISE hello Élise
```

Line by line:

- `word` gets the text `"élise"`;
- `len(word)` counts its characters: 5;
- `upper(word)` gives `"ÉLISE"`, `lower("HELLO")` gives `"hello"` and
  `capitalize(word)` gives `"Élise"`;
- `print` shows these three texts on one line, separated by a space.

Accented letters count as a single character: `len("été")` is 3.

**Your turn:** write a program that stores `"rythme"` in a variable `language`, then
shows its length, and below it the same word in uppercase.

> **Solution**
>
> ```rythme
> language = "rythme"
> print(len(language))
> print(upper(language))
> ```
>
> ```sortie
> 6
> RYTHME
> ```

## Reading a character by its position

Sometimes you only need part of a text: the initial of a first name, or the first
letters of a word.

Each character has a position, starting at **0**. `word[0]` is the first
character, `word[-1]` the last. `slice(word, start, end)` takes a piece,
from `start` included to `end` excluded:

```rythme
word = "sunshine"
print(word[0], word[-1], slice(word, 0, 3), slice(word, 3, 8))
```

```sortie
s e sun shine
```

Like a ruler: the piece from 0 to 3 contains the characters at positions 0,
1 and 2.

## Removing spaces with trim

A text typed by someone often has extra spaces at the start or the end.
`trim` removes them:

```rythme
typed = "   Ada   "
print("[{typed}]")
print("[{trim(typed)}]")
```

```sortie
[   Ada   ]
[Ada]
```

## Splitting and joining

A text often holds several pieces of information stuck together: a shopping list
"bread,milk,eggs", or a sentence made of words. To handle each piece on its own, you
first have to separate them.

`split` cuts a text into a list of pieces. Without a separator, it cuts at
the spaces. `join` does the opposite: it glues a list back together with a
separator.

```rythme
sentence = "the cat sleeps"
pieces = split(sentence)
print(pieces, len(pieces))

print(split("red;green;blue", ";"))
print(join(["red", "green", "blue"], ", "))
```

```sortie
["the", "cat", "sleeps"] 3
["red", "green", "blue"]
red, green, blue
```

Line by line:

- `split(sentence)` cuts `"the cat sleeps"` at the spaces: `pieces` is
  `["the", "cat", "sleeps"]`, a list of 3 texts;
- `split("red;green;blue", ";")` cuts at each `;`;
- `join(…, ", ")` goes the other way: it glues the three words together, putting
  `", "` between them.

With `|>`, you chain the steps: split, transform each word, join.

```rythme
"the cat sleeps"
  |> split
  |> map(capitalize)
  |> join(" ")
  |> print
```

```sortie
The Cat Sleeps
```

**Your turn:** write a program that splits the shopping list `"bread,milk,eggs"` at
the commas, shows the number of items, then joins them back with `" - "` between
them.

> **Solution**
>
> ```rythme
> shopping = split("bread,milk,eggs", ",")
> print(len(shopping))
> print(join(shopping, " - "))
> ```
>
> ```sortie
> 3
> bread - milk - eggs
> ```

## Searching in a text

Before acting, a program often needs to check a text: is this file a photo? Does
this address contain an `@`?

Three questions, three `true` or `false` answers:

- `contains`: does the text contain this piece?
- `starts_with`: does it start with this piece?
- `ends_with`: does it end with this piece?

```rythme
file = "holidays.jpg"
print(contains(file, "liday"))
print(starts_with(file, "hol"), ends_with(file, ".jpg"))
print(ends_with(file, ".png"))
```

```sortie
true
true true
false
```

Line by line:

- `file` gets the name `"holidays.jpg"`;
- `contains` finds `"liday"` in the middle of the name: `true`;
- the name does start with `"hol"` and end with `".jpg"`: `true true`;
- it doesn't end with `".png"`: `false`.

> **Warning:** case matters: `contains("Hello", "hel")` is `false`. Put the
> text in lowercase first with `lower` if needed.

To compare two whole texts, `==` works too, and case matters just the same.
`equals_ignore_case` compares without taking it into account:

```rythme
print("abc" == "abc", "Abc" == "abc", equals_ignore_case("Été", "ÉTÉ"))
```

```sortie
true false true
```

## Replacing a piece

It's the "find and replace" feature of a word processor: fix a word everywhere, or
change a separator.

`replace(text, old, new)` replaces **every** occurrence of `old`:

```rythme
print(replace("the cat and the dog", "the", "a"))
print(replace("2024-05-17", "-", "/"))
```

```sortie
a cat and a dog
2024/05/17
```

As always in Rythme, the starting text doesn't change: `replace` gives a
new text.

**Your turn:** write a program that stores `"my class notes.txt"` in a variable, then
shows this file name with `_` instead of the spaces.

> **Solution**
>
> ```rythme
> file_name = "my class notes.txt"
> print(replace(file_name, " ", "_"))
> ```
>
> ```sortie
> my_class_notes.txt
> ```

## Characters and lines

Sometimes a text is handled character by character (counting the vowels), or line
by line (reading a poem, a list). To do that, you turn it into a list.

`chars` gives the list of characters. `lines` cuts a text into lines (`\n`
means "line break"):

```rythme
print(chars("abc"))

poem = "One\nTwo\nThree"
print(lines(poem))
print(len(lines(poem)), "lines")
```

```sortie
["a", "b", "c"]
["One", "Two", "Three"]
3 lines
```

Line by line:

- `chars("abc")` gives a list of three one-character texts;
- `poem` holds three lines, separated by `\n`;
- `lines(poem)` cuts at each `\n`: a list of 3 lines;
- `len` counts the elements of that list: 3.

With a list of characters, all the list functions work. For example,
counting the vowels:

```rythme
vowels = chars("aeiouy")
how_many = chars("programming") |> filter(fn(c) = contains(vowels, c)) |> len
print(how_many)
```

```sortie
3
```

## Repeating and aligning

> **Note:** this part can wait. Read it once the basics feel solid.

`repeat(text, n)` repeats a text `n` times. Handy for a line:

```rythme
print(repeat("-", 20))
print(repeat("ha", 3))
```

```sortie
--------------------
hahaha
```

`pad_left(text, width)` adds spaces **on the left** up to the wanted width,
and `pad_right` **on the right**. This aligns columns:

```rythme
products = [
  {name: "apple", price: 3},
  {name: "kiwi", price: 12},
  {name: "pineapple", price: 150},
]

products.each do |p|
  print(pad_right(p.name, 10) + pad_left(text(p.price), 5) + " €")
end
```

```sortie
apple         3 €
kiwi         12 €
pineapple   150 €
```

A third argument chooses the fill character:

```rythme
print(pad_left("7", 3, "0"), pad_left("42", 3, "0"))
```

```sortie
007 042
```

## Reading a number from a text

A text like `"42"` is not a number: you can't calculate with it. That is the
case for everything a user types on the keyboard. `parse_int` (whole number)
and `parse_float` (decimal number) read it. The text might not be a number,
so they give a `Result`: `Ok(number)` or `Err(message)`.

```rythme
print(parse_int("42"), parse_int(" 7 "))
print(parse_float("3.5"))
print(parse_int("twelve"))
```

```sortie
Ok(42) Ok(7)
Ok(3.5)
Err("\"twelve\" is not a whole number")
```

Line by line:

- `parse_int("42")` succeeds: it gives `Ok(42)`, the number placed in an `Ok` box;
- `parse_int(" 7 ")` ignores the spaces around it and gives `Ok(7)`;
- `parse_float("3.5")` reads a decimal number, written with a dot;
- `parse_int("twelve")` fails: it gives `Err`, with a message that says why.

To get the number, handle both cases with `match`, or give a default value
with `unwrap_or`:

```rythme
fn double(typed) = match parse_int(typed)
  Ok(n) -> "twice that is {n * 2}"
  Err(_) -> "\"{typed}\" is not a number"
end

print(double("21"))
print(double("twenty"))
print(parse_int("??") |> unwrap_or(0))
```

```sortie
twice that is 42
"twenty" is not a number
0
```

`Result` is explained in detail in [handling errors](erreurs.md).

> **Note:** `int("42")` also reads a number, but stops the program (error
> E022) if the text isn't one. Keep it for a text you wrote yourself; for
> user input, prefer `parse_int`.

**Your turn:** write a program that stores the text `"15"` in `typed`, reads it with
`parse_int` and `unwrap_or(0)`, then shows "In 10 years, you will be 25." by
calculating 25 from the number it read.

> **Solution**
>
> ```rythme
> typed = "15"
> age = parse_int(typed) |> unwrap_or(0)
> print("In 10 years, you will be {age + 10}.")
> ```
>
> ```sortie
> In 10 years, you will be 25.
> ```

## Writing a large number readably

> **Note:** this part can wait. Read it once the basics feel solid.

`format_number(number, decimals, language)` separates the thousands and
rounds. With `"fr"`, it writes the number the French way:

```rythme
print(format_number(1234567.891, 2, "fr"))
print(format_number(1234567.891, 2))
print(text(3.14159, 2))
```

```sortie
1 234 567,89
1,234,567.89
3.14
```

Without a language, it's the English way. And `text(number, decimals)`
simply gives a number with that many decimals.

## Regular expressions

Sometimes you aren't looking for a precise piece, but for a **shape**: "some
digits", "a word". For example, all the numbers in a sentence, whatever they are.

A **regular expression** (a "regex") is a small pattern that describes
pieces of text. A few symbols are enough to get started:

| Symbol | Means |
|-------|-----------|
| `\d` | a digit, from 0 to 9 |
| `\w` | a letter, a digit or `_` |
| `[aeiou]` | one of these characters |
| `+` | the symbol before, once or more |

So `\d+` means "one or more digits".

> **Warning:** in a Rythme text, the backslash is written twice: the regex
> `\d+` is written `"\\d+"`.

Three functions to get started:

- `matches(text, regex)`: does the **whole** text match the pattern?
- `find_all(text, regex)`: the list of all the pieces that match;
- `replace_all(text, regex, new)`: replaces all those pieces.

```rythme
print(matches("2024", "\\d+"), matches("8pm24", "\\d+"))
print(find_all("3 apples and 12 pears", "\\d+"))
print(replace_all("My code: 1234", "\\d", "*"))
```

```sortie
true false
["3", "12"]
My code: ****
```

Line by line:

- `matches("2024", "\\d+")`: the whole text is made only of digits, so `true`; in
  `"8pm24"`, `p` and `m` aren't digits, so `false`;
- `find_all` looks for every group of digits and gives `["3", "12"]`;
- `replace_all` replaces each digit (`\\d`, one at a time) with `*`.

For example, adding up all the numbers in a sentence:

```rythme
sentence = "I have 3 apples, 12 pears and 5 kiwis."
total = find_all(sentence, "\\d+")
  |> map(fn(t) = parse_int(t) |> unwrap_or(0))
  |> sum
print(total)
```

```sortie
20
```

> **Note:** regexes can do much more. `rythme help find_all` gives examples,
> and the Rythme reference lists all the symbols.

## Exercises

### Exercise 1: Initials

Write a function `initials` that gives the initials of a name, in
uppercase: `initials("ada lovelace")` gives `"AL"`.

> **Hint:** split with `split`, take the first character of each word with
> `w[0]`, then join with `join(list, "")`.

> **Solution**
>
> ```rythme
> fn initials(name) = name
>   |> split
>   |> map(fn(w) = upper(w[0]))
>   |> join("")
>
> print(initials("ada lovelace"))
> print(initials("grace brewster hopper"))
> ```
>
> ```sortie
> AL
> GBH
> ```

### Exercise 2: A frame

Write a function `frame` that prints a word surrounded by stars, as below.

```text
**********
* Rythme *
**********
```

> **Hint:** the line of stars has the length of the word plus 4. Use
> `repeat` and `len`.

> **Solution**
>
> ```rythme
> fn frame(word)
>   line = repeat("*", len(word) + 4)
>   print(line)
>   print("* {word} *")
>   print(line)
> end
>
> frame("Rythme")
> ```
>
> ```sortie
> **********
> * Rythme *
> **********
> ```

## Common mistakes

### Calculating directly with parse_int

```rythme norun
typed = "41"
print(parse_int(typed) + 1)
```

```text
error[E005]: cannot use `+` with Result and Int
 --> main.ry:2:24
  |
2 | print(parse_int(typed) + 1)
  |                        ^ Result + Int 1
  |
  = more about this error: `rythme explain E005`
```

`parse_int` doesn't give a number, but a `Result`: a box that contains
either the number or an error. Open it first:
`(parse_int(typed) |> unwrap_or(0)) + 1`, or with `match`.

### Forgetting that the text doesn't change

```rythme norun
name = "ada"
upper(name)
print(name)
```

```text
warning[W008]: the result of `upper` is lost
 --> main.ry:2:1
  |
2 | upper(name)
  | ^^^^^^^^^^^ this value is lost
  |
  = help: values never change: `upper(text)` gives a new value, and changes nothing.
      To keep it, give it a name: `new = upper(text)`
```

It's a warning: the program runs, but prints `ada`. `upper` makes a
**new** text. Keep it: `name_upper = upper(name)`.

### A single backslash in a regex

```rythme norun
print(matches("2024", "\d+"))
```

```text
error[E018]: unknown escape sequence in text
 --> main.ry:1:24
  |
1 | print(matches("2024", "\d+"))
  |                        ^^
  |
  = help: in a regex, write the backslash twice: "\\d" is the text \d
  = help: valid escapes: \n (new line), \t (tab), \r (carriage return), \0 (null character), \\ (backslash),
      \" (quote), \{ and \} (braces), \# (a `#` before a brace), \u{e9} (a character by its Unicode number: é)
  = more about this error: `rythme explain E018`
```

In a text, `\` starts a special character. For the regex `\d+`, write
`"\\d+"`.

## Test yourself

**Question 1:** what is `"  Hello  " |> trim |> lower`?

> **Answer**
>
> `"hello"`: `trim` removes the spaces around it, then `lower` puts it in
> lowercase.

**Question 2:** what does `split("12/05/2024", "/")` give?

> **Answer**
>
> The list `["12", "05", "2024"]`. They are still texts: to calculate, read
> them with `parse_int`.

**Question 3:** why does `parse_int` give a `Result`, and not directly a
number?

> **Answer**
>
> Because the text might not be a number (`"twelve"`). The `Result` forces
> you to plan for that case: `Ok(number)` or `Err(message)`.

## Summary

- A value between braces goes into a text: `"total: {x}"`; `\n`, `\"` and
  `\{` write special characters.
- `upper`, `lower`, `capitalize`, `trim` and `replace` give a new text;
  `contains`, `starts_with` and `ends_with` answer `true` or `false`.
- `split` cuts a text into a list, `join` glues a list back together;
  `chars` and `lines` give the characters and the lines.
- `pad_left`, `pad_right` and `repeat` are for aligning and decorating.
- `parse_int` and `parse_float` give a `Result`: the text might not be a
  number.
- A regex describes pieces of text: `matches`, `find_all` and `replace_all`
  use it.

Next: [lists, records and dictionaries](collections.md). Compare with text
in [Ruby](../ruby/texte.md) and in [Rust](../rust/texte.md).
