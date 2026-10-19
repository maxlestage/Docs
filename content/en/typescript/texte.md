# Working with text

This page shows how to handle text in TypeScript: measure, search, cut, replace, format numbers and line up columns, with types that protect every step.

> **Goals:**
>
> - you will know how to use the common string methods: `toUpperCase`, `includes`,
>   `slice`, `split`, `replace`…;
> - you will know how to read the type returned by each method;
> - you will know how to convert a text into a number and back;
> - you will know how to limit a text to a few values with a type like `"yes" | "no"`;
> - you will know how to format numbers with `Intl` and line up columns with
>   `padStart`.

## Strings

A text is a [string](../debuter/glossaire.md#string), of type `string`.
Each character has a position, its [index](../debuter/glossaire.md#index), which
starts at **0**.

```typescript
const word: string = "Welcome";

console.log(word.length);
console.log(word[0]);
console.log(word[3]);
console.log(word.at(-1));
```

```sortie
7
W
c
e
```

- `length` gives the number of characters. It's a **property**: no parentheses.
- `word[0]` gives the first character, `word[3]` the fourth.
- `at(-1)` counts from the end: `-1` is the last character.

A [method](../debuter/glossaire.md#method) is an action that a value knows how to do.
You call it with a dot and parentheses: `word.toUpperCase()`. Your editor knows every
method of a `string`: type `word.` and it suggests them, with their type.

## Changing case and cleaning up

```typescript
const input = "  Marie Curie  ";

console.log(`[${input.toUpperCase()}]`);
console.log(`[${input.toLowerCase()}]`);
console.log(`[${input.trim()}]`);
console.log(`[${input.trimStart()}]`);
```

```sortie
[  MARIE CURIE  ]
[  marie curie  ]
[Marie Curie]
[Marie Curie  ]
```

- `toUpperCase()` puts the text in uppercase, `toLowerCase()` in lowercase;
- `trim()` removes the spaces at the start and at the end; `trimStart()` only at the
  start.

These methods don't change `input`: they **return a new text**. A string is
[immutable](../debuter/glossaire.md#immutable): you can never change it in place.
TypeScript forbids it anyway: `word[0] = "w"` is refused (see the
[common mistakes](#common-mistakes)).

> **Tip:** apply `trim()` to any text typed by a user: extra spaces are a classic cause
> of failed comparisons.

## Searching in a text

These methods answer a question. Their return type tells you what shape the answer
has:

| Method | Question | Returns |
|---------|----------|---------|
| `includes(x)` | contains `x`? | `boolean` |
| `startsWith(x)` | starts with `x`? | `boolean` |
| `endsWith(x)` | ends with `x`? | `boolean` |
| `indexOf(x)` | at what position is `x`? | `number` (`-1` if missing) |

```typescript
const sentence = "The cat sleeps on the sofa.";

console.log(sentence.includes("cat"));
console.log(sentence.startsWith("The"));
console.log(sentence.endsWith("!"));
console.log(sentence.indexOf("sleeps"));
console.log(sentence.indexOf("dog"));
```

```sortie
true
true
false
8
-1
```

All these searches tell uppercase and lowercase apart: `"Cat"` is not `"cat"`. To
search without caring about case, put everything in lowercase first.

```typescript
const title = "Learning TypeScript";
console.log(title.toLowerCase().includes("typescript"));
```

```sortie
true
```

## Extracting a piece with slice

`slice(start, end)` cuts out part of the text, from character `start` **included** to
character `end` **excluded**. Without `end`, it goes to the end. A negative number
counts from the end.

```typescript
const file = "report-2024.pdf";

console.log(file.slice(0, 6));
console.log(file.slice(7, 11));
console.log(file.slice(-3));
```

```sortie
report
2024
pdf
```

Think of positions as the marks on a ruler: `slice(0, 6)` takes everything between
mark 0 and mark 6.

## Splitting and joining

`split(separator)` cuts a text into pieces. It returns an **array of texts**, of type
`string[]`. `join(separator)` does the opposite: it glues an array back into a single
text.

```typescript
const line = "apple;pear;kiwi";
const fruits: string[] = line.split(";");

console.log(fruits);
console.log(fruits.length);
console.log(fruits.join(", "));
console.log("abc".split(""));
```

```sortie
[ 'apple', 'pear', 'kiwi' ]
3
apple, pear, kiwi
[ 'a', 'b', 'c' ]
```

`split("")` cuts between every character. Arrays are covered in detail in
[the chapter on arrays](collections.md).

## Replacing

`replace(a, b)` replaces the **first** occurrence of `a` with `b`. `replaceAll`
replaces every occurrence.

```typescript
const text = "The cat sees the cat.";

console.log(text.replace("cat", "dog"));
console.log(text.replaceAll("cat", "dog"));
console.log("06 12 34 56 78".replaceAll(" ", ""));
```

```sortie
The dog sees the cat.
The dog sees the dog.
0612345678
```

## Converting between text and number

A text that contains digits is still a text: `"42"` is not `42`. TypeScript stops you
from mixing them up. To convert, use:

- `Number(text)`: text → number (gives `NaN`, "not a number", if it's impossible);
- `String(number)` or `number.toString()`: number → text.

```typescript
const input = "42";
const value: number = Number(input);

console.log(value + 1);
console.log(input + 1);
console.log(Number("3,5"), Number("3.5"));
console.log(String(7) + String(7));
```

```sortie
43
421
NaN 3.5
77
```

- `value + 1` adds: `43`.
- `input + 1` glues the texts together: `"421"`.
- `Number("3,5")` fails: in code, the decimal separator is written with a dot.

`NaN` is of type `number`, but it isn't a real number. Test it with `Number.isNaN`:

```typescript
const attempts = ["12", "twelve"];

for (const attempt of attempts) {
  const n = Number(attempt);
  if (Number.isNaN(n)) {
    console.log(`"${attempt}" is not a number.`);
  } else {
    console.log(`${n} x 2 = ${n * 2}`);
  }
}
```

```sortie
12 x 2 = 24
"twelve" is not a number.
```

## Literal types: "yes" | "no"

Here is a strength of TypeScript. A type can accept **only certain texts**. You list
the allowed values, separated by `|` ("or"):

```typescript
type Answer = "yes" | "no";

function reply(r: Answer): string {
  return r === "yes" ? "Perfect!" : "Too bad.";
}

console.log(reply("yes"));
console.log(reply("no"));
```

```sortie
Perfect!
Too bad.
```

`"yes"` and `"no"` are **literal types**: each one accepts a single text. Their union
`"yes" | "no"` accepts one or the other, and nothing else. It's like a form with
checkboxes instead of a free text field: there is no way to answer "meh".

- Your editor suggests `"yes"` and `"no"` when you type the argument.
- A typo (`"yess"`) is refused before the program runs.
- A `switch` on this type can check that no case is forgotten (see
  [conditions and loops](conditions-boucles.md#a-switch-that-never-forgets-a-case)).

```typescript
type Color = "red" | "green" | "blue";

const favorite: Color = "green";
const all: Color[] = ["red", "green", "blue"];

console.log(favorite, all.length);
```

```sortie
green 3
```

> **Remember:** when a text can only take a few values (a status, a direction, a
> size), use a union of literal types rather than `string`.

### Template literal types, in short

TypeScript can even describe the **shape** of a text, with the template string syntax.
These are *template literal types*:

```typescript
type Pixels = `${number}px`;

const width: Pixels = "120px";
const height: Pixels = "80px";

console.log(width, height);
```

```sortie
120px 80px
```

`` `${number}px` `` means "a number followed by `px`". `"120"` or `"120 px"` would be
refused. You will see this idea again in [literal types](litteraux.md).

## Formatting numbers with Intl

To display a number the way it is written in France (a space between thousands, a
decimal comma, the € symbol), use `Intl.NumberFormat`. `Intl` (*internationalization*)
knows the habits of each country.

```typescript
const amount = 1234567.891;

const frenchNumber = new Intl.NumberFormat("fr-FR");
const euros = new Intl.NumberFormat("fr-FR", { style: "currency", currency: "EUR" });
const dollars = new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" });
const percent = new Intl.NumberFormat("fr-FR", { style: "percent" });

console.log(frenchNumber.format(amount));
console.log(euros.format(amount));
console.log(dollars.format(amount));
console.log(percent.format(0.256));
```

```sortie
1 234 567,891
1 234 567,89 €
$1,234,567.89
26 %
```

- `"fr-FR"` means "French from France", `"en-US"` "English from the United States".
- `style: "currency"` displays an amount of money; `currency` says which currency.
- `style: "percent"` multiplies by 100 and adds `%`.

> **Note:** the spaces in the French result are **non-breaking spaces**: they prevent a
> line break in the middle of a number. They look like normal spaces, but they are
> different characters. So don't compare this result with a text typed by hand.

The options are typed: `{ style: "money" }` would be refused, because `style` only
accepts a few precise texts (`"decimal"`, `"currency"`, `"percent"`, `"unit"`). Another
union of literal types!

To choose the number of decimals, use `maximumFractionDigits`:

```typescript
const short = new Intl.NumberFormat("fr-FR", { maximumFractionDigits: 1 });
console.log(short.format(3.14159));
console.log(short.format(2));
```

```sortie
3,1
2
```

## Lining up columns with padStart and padEnd

`padStart(length, filler)` fills a text **at the start** up to the wanted length.
`padEnd` fills **at the end**. Without a filler, it uses spaces.

```typescript
console.log("7".padStart(3, "0"));
console.log("42".padStart(5) + "|");
console.log("ok".padEnd(5, ".") + "|");
```

```sortie
007
   42|
ok...|
```

It's perfect for printing a neatly aligned table: texts on the left with `padEnd`,
numbers on the right with `padStart`.

```typescript
function row(item: string, price: number): string {
  return item.padEnd(12, ".") + price.toFixed(2).padStart(8);
}

console.log(row("Bread", 1.2));
console.log(row("Cheese", 12.5));
console.log(row("Coffee", 104));
console.log("-".repeat(20));
console.log("Total".padEnd(12) + (1.2 + 12.5 + 104).toFixed(2).padStart(8));
```

```sortie
Bread.......    1.20
Cheese......   12.50
Coffee......  104.00
--------------------
Total         117.70
```

`repeat(20)` repeats a text 20 times. The decimal points of the prices line up, like on
a receipt.

## Common mistakes

### Changing a character of a string

```typescript norun
let word = "cat";
word[0] = "C";
```

```text
main.ts:2:1 - error TS2542: Index signature in type 'String' only permits reading.

2 word[0] = "C";
  ~~~~~~~


Found 1 error in main.ts:2
```

"The positions of a `String` are read-only." A string is immutable. Build a new text:
`word = "C" + word.slice(1);`.

### Calling length like a method

```typescript norun
const firstName = "Alice";
console.log(firstName.length());
```

```text
main.ts:2:23 - error TS2349: This expression is not callable.
  Type 'Number' has no call signatures.

2 console.log(firstName.length());
                        ~~~~~~


Found 1 error in main.ts:2
```

"This expression can't be called: it's a number." `length` is a property, not a
method. Write `firstName.length`, without parentheses.

### Adding a text and a number

```typescript norun
const age = "30";
const inTenYears: number = age + 10;
```

```text
main.ts:2:7 - error TS2322: Type 'string' is not assignable to type 'number'.

2 const inTenYears: number = age + 10;
        ~~~~~~~~~~


Found 1 error in main.ts:2
```

`age` is a text, so `age + 10` glues `"30"` and `"10"` together: the result is the
text `"3010"`. Convert first: `Number(age) + 10`.

### Giving a value outside the list

```typescript norun
type Answer = "yes" | "no";
function reply(r: Answer): void {
  console.log(`You said ${r}.`);
}
reply("maybe");
```

```text
main.ts:5:7 - error TS2345: Argument of type '"maybe"' is not assignable to parameter of type 'Answer'.

5 reply("maybe");
        ~~~~~~~


Found 1 error in main.ts:5
```

`Answer` only accepts `"yes"` or `"no"`. Use an allowed value, or add `"maybe"` to the
type if it's a real possible answer.

## Test yourself

**Question 1:** what does `"TypeScript".slice(4, 10).toLowerCase()` print?

> **Answer**
>
> `script`. `slice(4, 10)` takes the characters at positions 4 to 9 (`Script`), then
> `toLowerCase()` puts them in lowercase.

**Question 2:** what is the type of `"a-b-c".split("-")`?

> **Answer**
>
> `string[]`, an array of texts: `["a", "b", "c"]`.

**Question 3:** write a type `Direction` that only accepts `"up"`, `"down"`, `"left"`
and `"right"`, then a variable of that type.

> **Answer**
>
> ```typescript
> type Direction = "up" | "down" | "left" | "right";
>
> const d: Direction = "left";
> console.log(d);
> ```
>
> ```sortie
> left
> ```

## Summary

- A `string` is immutable: its methods always return a new text.
- `length` measures, `includes` / `startsWith` / `indexOf` search, `slice` extracts,
  `split` cuts into a `string[]`, `replaceAll` replaces.
- `Number(text)` converts to a number (`NaN` if impossible), `String(n)` to a text.
- A union of literal types like `"yes" | "no"` limits a text to a few values, checked
  before the program runs.
- `Intl.NumberFormat("fr-FR", ...)` formats numbers, currencies and percentages;
  `padStart` and `padEnd` line up columns.

Next: [arrays and tuples](collections.md).
