# Working with text

This page shows how to handle text in JavaScript: read its characters, cut it up, transform it, search it and sort it in alphabetical order.

> **Goals:**
>
> - you will be able to read the length of a text and the character at a given position;
> - you will be able to extract a piece with `slice` and clean up a text with `trim`;
> - you will be able to split with `split` and glue back together with `join`;
> - you will be able to search and replace in a text;
> - you will be able to sort words correctly with `localeCompare`.

## Strings

Almost every program handles text: a first name, a message, an email address, a line
read from a file. Before you can transform text, you need to know how to write it.

A piece of text is called a [string](../debuter/glossaire.md#string): a
sequence of letters, digits, spaces and symbols, like beads on a string. You write it
between double quotes, single quotes or backticks.

```javascript
const a = "Hello";
const b = 'Hi';
const c = `Hey`;
const quote = "He said: 'yes'";

console.log(a, b, c);
console.log(quote);
console.log("It's summer");
```

```sortie
Hello Hi Hey
He said: 'yes'
It's summer
```

Line by line:

- the first three lines store a text in `a`, `b` and `c`, with three kinds of quotes;
- `quote` contains single quotes, so it is wrapped in double quotes;
- `console.log(a, b, c)` prints the three texts on one line, separated by a space;
- the quotes around a text are never printed: they only mark where it starts and
  ends.

Pick the quotes that are not in the text: `"It's summer"` contains an apostrophe, so you
wrap it in double quotes.

### Special characters

A backslash `\` introduces a special character: `\n` for a line break, `\t` for a tab,
`\"` for a double quote inside a text written between double quotes.

```javascript
console.log("Line 1\nLine 2");
console.log("She said \"hello\".");
```

```sortie
Line 1
Line 2
She said "hello".
```

## Length and position

Checking that a password is long enough, taking the first letter of a name: for that,
you need to know the size of a text and how to point at one precise letter, the way you
point at a house by its number on a street.

`length` gives the number of characters. Each character has a position, its
[index](../debuter/glossaire.md#index), which **starts at 0**.

```javascript
const word = "Welcome";

console.log(word.length);
console.log(word[0]);
console.log(word[3]);
console.log(word[word.length - 1]);
console.log(word[10]);
```

```sortie
7
W
c
e
undefined
```

Line by line:

- `word.length` is 7: "Welcome" has seven letters;
- `word[0]` is the first letter, `"W"`, because counting starts at 0;
- `word[3]` is therefore the fourth letter, `"c"`;
- `word.length - 1` is 6: `word[6]` is the last letter, `"e"`;
- `word[10]` points at a box that does not exist: the result is `undefined`.

| Character | W | e | l | c | o | m | e |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Index | 0 | 1 | 2 | 3 | 4 | 5 | 6 |

The last character is at index `length - 1`. A position that does not exist gives
`undefined`.

### Counting from the end with at

`at(i)` works like `[i]`, but also accepts a negative number: `-1` is the last
character, `-2` the one before it.

```javascript
const word = "Welcome";
console.log(word.at(0));
console.log(word.at(-1));
console.log(word.at(-2));
```

```sortie
W
e
m
```

**Your turn:** write a program that stores `"Olivia"` in a variable `name`, then
prints its length, its first letter and its last letter.

> **Solution**
>
> ```javascript
> const name = "Olivia";
> console.log(name.length);
> console.log(name[0]);
> console.log(name.at(-1));
> ```
>
> ```sortie
> 6
> O
> a
> ```

## Texts never change

Learn this rule early, because it causes many beginner mistakes: you think you changed
a text, when you actually made a new one and then forgot it.

A string is [immutable](../debuter/glossaire.md#immutable): you cannot change one of its
characters. All the methods on this page **return a new text** and leave the original
untouched.

```javascript
const name = "alice";
const upper = name.toUpperCase();

console.log(name);
console.log(upper);
```

```sortie
alice
ALICE
```

To keep the result, store it in a variable (with `let`, you can reuse the same one:
`text = text.trim()`).

## Upper case, lower case and spaces

What a user types is rarely clean: extra spaces, random capitals. These methods tidy
the text up before you use it, the way you tidy a drawer before looking for something
in it.

```javascript
const input = "   Marie Curie   ";

console.log(`[${input.toUpperCase()}]`);
console.log(`[${input.toLowerCase()}]`);
console.log(`[${input.trim()}]`);
console.log(`[${input.trimStart()}]`);
console.log(`[${input.trimEnd()}]`);
```

```sortie
[   MARIE CURIE   ]
[   marie curie   ]
[Marie Curie]
[Marie Curie   ]
[   Marie Curie]
```

Line by line:

- `input` contains "Marie Curie" with three spaces on each side;
- the brackets `[` and `]` are only there to show where the spaces start and end;
- each line calls a method on `input` and prints the new text it gets back;
- `input` never changes: each line starts again from the original text, spaces
  included.

- `toUpperCase()` and `toLowerCase()` change the case, accents included
  (`"café"` becomes `"CAFÉ"`).
- `trim()` removes the spaces at the start and at the end; `trimStart()` and `trimEnd()`
  only on one side.

> **Tip:** call `trim()` on everything a user types: a forgotten space at the end of a
> password or an email address is a classic source of bugs.

## Extracting a piece

Often you only care about part of a text: the year in a date, the first letters of a
word. `slice` cuts that piece out, like a pair of scissors.

`slice(start, end)` returns the piece that goes from `start` (included) to `end` (not
included). Without `end`, it goes all the way to the end. Negative numbers count from the
end.

```javascript
const word = "Welcome";

console.log(word.slice(0, 3));
console.log(word.slice(3));
console.log(word.slice(-4));
console.log(word.slice(1, -1));
```

```sortie
Wel
come
come
elcom
```

Line by line:

- `slice(0, 3)` takes indexes 0, 1 and 2: `"Wel"`;
- `slice(3)` starts at index 3 and goes to the end: `"come"`;
- `slice(-4)` takes the last four characters: `"come"` again;
- `slice(1, -1)` removes the first and the last letter: `"elcom"`.

Think of the indexes as **cuts between the letters**: `slice(0, 3)` cuts before letter 0
and before letter 3.

`substring(start, end)` does almost the same thing, but does not understand negative
numbers. Remember `slice`, which also works on arrays.

**Your turn:** write a program that stores `"2026-10-19"` in a variable `date`, then
prints the year (the first four characters) and the day (the last two).

> **Solution**
>
> ```javascript
> const date = "2026-10-19";
> console.log(date.slice(0, 4));
> console.log(date.slice(-2));
> ```
>
> ```sortie
> 2026
> 19
> ```

## Splitting and joining

A text often holds a hidden list: fruits separated by commas, words separated by
spaces. `split` takes that list out of the text, so you can handle the pieces one by
one.

`split(separator)` cuts a text into an array of pieces. `join(separator)` does the
opposite: it glues the pieces of an array back together.

```javascript
const line = "apple,pear,kiwi";
const fruits = line.split(",");
console.log(fruits);
console.log(fruits.length);

console.log(fruits.join(" - "));
console.log("Hello to everyone".split(" "));
console.log("abc".split(""));
```

```sortie
[ 'apple', 'pear', 'kiwi' ]
3
apple - pear - kiwi
[ 'Hello', 'to', 'everyone' ]
[ 'a', 'b', 'c' ]
```

Line by line:

- `line.split(",")` cuts the text at each comma: `fruits` is an array of three texts;
- `fruits.length` is 3: the array has three boxes;
- `fruits.join(" - ")` glues the three pieces back together with `" - "` between them;
- `split(" ")` cuts at each space: you get the words of the sentence.

`split("")` cuts into characters. Arrays are explained on
[the next page](collections.md).

**Your turn:** write a program that counts the words of the sentence
`"The cat sleeps on the sofa"` and prints that number.

> **Solution**
>
> ```javascript
> const sentence = "The cat sleeps on the sofa";
> const words = sentence.split(" ");
> console.log(words.length);
> ```
>
> ```sortie
> 6
> ```

## Searching in a text

A program often needs to check a text: does an address contain an `@`? Does a file name
end with `.jpg`? These methods answer with `true` or `false`, or give a position.

```javascript
const sentence = "The cat sleeps on the sofa";

console.log(sentence.includes("cat"));
console.log(sentence.includes("Cat"));
console.log(sentence.startsWith("The"));
console.log(sentence.endsWith("sofa"));
console.log(sentence.indexOf("sleeps"));
console.log(sentence.indexOf("dog"));
```

```sortie
true
false
true
true
8
-1
```

Line by line:

- `includes("cat")` gives `true`, but `includes("Cat")` gives `false`: the capital
  letter matters;
- `startsWith("The")` and `endsWith("sofa")` give `true`: the sentence does start and
  end that way;
- `indexOf("sleeps")` gives 8: "sleeps" starts at index 8, spaces included;
- `indexOf("dog")` gives `-1`: "dog" is not in the sentence.

- `includes(x)`: does the text contain `x`?
- `startsWith(x)` and `endsWith(x)`: does it start or end with `x`?
- `indexOf(x)`: the position of the first occurrence of `x`, or `-1` if it is not there.

All these searches tell upper case and lower case apart. To ignore the case, first put
both in lower case:

```javascript
const sentence = "The Cat sleeps";
console.log(sentence.toLowerCase().includes("cat"));
```

```sortie
true
```

## Replacing

Fixing a word everywhere in a text, removing the spaces from a phone number: this is
the "find and replace" of a word processor.

`replace(old, new)` replaces the **first** occurrence. `replaceAll` replaces **all** of
them.

```javascript
const sentence = "the cat and the dog";

console.log(sentence.replace("the", "a"));
console.log(sentence.replaceAll("the", "a"));
console.log("555 123 4567".replaceAll(" ", ""));
```

```sortie
a cat and the dog
a cat and a dog
5551234567
```

Line by line:

- `replace("the", "a")` only changes the first "the";
- `replaceAll("the", "a")` changes both;
- the last line replaces each space with an empty text `""`: the spaces disappear.

For finer searches ("all the digits", "a word in capitals"), there are
[regular expressions](regex.md).

**Your turn:** write a program that stores `"10/19/2026"` in a variable, then prints
it with dashes instead of slashes.

> **Solution**
>
> ```javascript
> const date = "10/19/2026";
> console.log(date.replaceAll("/", "-"));
> ```
>
> ```sortie
> 10-19-2026
> ```

## Aligning and repeating

> **Note:** this part can wait. Read it once the basics feel solid.

`padStart(length, filler)` fills a text at the start up to the length you want;
`padEnd` fills at the end. `repeat(n)` repeats a text `n` times.

```javascript
console.log("7".padStart(3, "0"));
console.log(String(42).padStart(5, " "));

const lines = [["Coffee", 2.5], ["Croissant", 1.2]];
for (const [item, price] of lines) {
  console.log(item.padEnd(12, ".") + price.toFixed(2).padStart(6));
}
console.log("=".repeat(18));
```

```sortie
007
   42
Coffee......  2.50
Croissant...  1.20
==================
```

`padStart(6)` without a second argument fills with spaces. With a number, you first have
to turn it into text: `String(42)`.
These methods are perfect for drawing aligned tables in the terminal.

## Multi-line templates

> **Note:** this part can wait. Read it once the basics feel solid.

A template between backticks can span several lines: the line breaks are part of the
text.

```javascript
const name = "Alice";
const total = 42.5;

const receipt = `Customer: ${name}
Total:    ${total.toFixed(2)} €
Thank you for your visit!`;

console.log(receipt);
```

```sortie
Customer: Alice
Total:    42.50 €
Thank you for your visit!
```

> **Warning:** indentation spaces are part of the text too. If you indent lines 2 and 3
> in your code, they will be indented when printed.

## Sorting words alphabetically

Putting names in alphabetical order, like in a phone book, looks simple. But the
computer does not know the alphabet: it orders letters by numbers.

Comparing two texts with `<` or sorting with `sort()` follows the order of the
**character codes** (the [Unicode](../debuter/glossaire.md#unicode) standard). Capital
letters come before lower-case letters, and accented letters go at the end.

```javascript
const words = ["zebra", "Éclair", "apple", "résumé", "Banana"];

console.log([...words].sort());
console.log([...words].sort((a, b) => a.localeCompare(b, "en")));
```

```sortie
[ 'Banana', 'apple', 'résumé', 'zebra', 'Éclair' ]
[ 'apple', 'Banana', 'Éclair', 'résumé', 'zebra' ]
```

Line by line:

- `words` contains five words, with capitals and accents;
- `[...words].sort()` sorts a copy by character codes: `"Banana"` comes before
  `"apple"`, and `"Éclair"`, which starts with `É`, ends up last;
- the next line sorts another copy with `localeCompare`: the order is the dictionary
  order.

`a.localeCompare(b, "en")` compares two texts using the rules of English: it returns a
negative number if `a` comes before `b`, a positive one if after, `0` if they are equal.
`sort` uses it to order the array. `[...words]` makes a copy, so the original is not
changed (see [arrays](collections.md)).

> **Remember:** to sort words that a human will read, always use `localeCompare` with
> the language code (`"en"`, `"fr"`, `"es"`...).

## Common mistakes

### Changing a character

```javascript norun
const word = "cat";
word[0] = "C";
```

```text
file:///home/alice/js/main.mjs:2
word[0] = "C";
        ^

TypeError: Cannot assign to read only property '0' of string 'cat'
    at file:///home/alice/js/main.mjs:2:9

Node.js v22.22.0
```

A text is immutable: its characters are read only. Build a new text instead:
`const newWord = "C" + word.slice(1);`.

### Writing length with parentheses

```javascript norun
const name = "Alice";
console.log(name.length());
```

```text
file:///home/alice/js/main.mjs:2
console.log(name.length());
                 ^

TypeError: name.length is not a function
    at file:///home/alice/js/main.mjs:2:18

Node.js v22.22.0
```

`length` is a **property** (a value), not a method: you do not call it. Write
`name.length`.

### Getting the capital letter wrong in a method name

```javascript norun
const name = "alice";
console.log(name.toUppercase());
```

```text
file:///home/alice/js/main.mjs:2
console.log(name.toUppercase());
                 ^

TypeError: name.toUppercase is not a function
    at file:///home/alice/js/main.mjs:2:18

Node.js v22.22.0
```

The method is called `toUpperCase`, with a capital `C`. JavaScript names are written in
camelCase: each new word starts with a capital letter.

### Forgetting to keep the result

```javascript norun
let input = "  hello  ";
input.trim();
console.log(`[${input}]`);
```

```text
[  hello  ]
```

`trim()` does not change `input`: it returns a new text, which is lost here. Write
`input = input.trim();`.

## Test yourself

**Question 1:** what does `"JavaScript".slice(0, 4)` give?

> **Answer**
>
> ```javascript
> console.log("JavaScript".slice(0, 4));
> ```
>
> ```sortie
> Java
> ```
>
> Indexes 0, 1, 2 and 3: the end (4) is not included.

**Question 2:** how do you turn `"a;b;c"` into `"a, b, c"`?

> **Answer**
>
> ```javascript
> console.log("a;b;c".split(";").join(", "));
> ```
>
> ```sortie
> a, b, c
> ```
>
> `replaceAll(";", ", ")` works too.

**Question 3:** why sort with `localeCompare(b, "en")` rather than with `sort()` on its
own?

> **Answer**
>
> `sort()` on its own orders by character codes: capital letters before lower-case
> letters, and `"é"` after `"z"`. `localeCompare` follows the dictionary order.

## Summary

- A text is written between `"..."`, `'...'` or `` `...` ``; it is immutable, each
  method returns a new text.
- `length` gives the length; `text[i]` and `text.at(i)` give a character, starting at 0.
- `slice`, `trim`, `toUpperCase` and `toLowerCase` extract and clean up.
- `split` cuts into an array, `join` glues back; `includes`, `startsWith` and `indexOf`
  search; `replace` and `replaceAll` replace.
- `padStart`, `padEnd` and `repeat` align; a template can span several lines.
- Sort words with `localeCompare(b, "en")`.

Next: [arrays](collections.md).
