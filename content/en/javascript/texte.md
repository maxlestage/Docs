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

## Texts never change

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

- `toUpperCase()` and `toLowerCase()` change the case, accents included
  (`"café"` becomes `"CAFÉ"`).
- `trim()` removes the spaces at the start and at the end; `trimStart()` and `trimEnd()`
  only on one side.

> **Tip:** call `trim()` on everything a user types: a forgotten space at the end of a
> password or an email address is a classic source of bugs.

## Extracting a piece

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

Think of the indexes as **cuts between the letters**: `slice(0, 3)` cuts before letter 0
and before letter 3.

`substring(start, end)` does almost the same thing, but does not understand negative
numbers. Remember `slice`, which also works on arrays.

## Splitting and joining

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

`split("")` cuts into characters. Arrays are explained on
[the next page](collections.md).

## Searching in a text

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

For finer searches ("all the digits", "a word in capitals"), there are
[regular expressions](regex.md).

## Aligning and repeating

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
