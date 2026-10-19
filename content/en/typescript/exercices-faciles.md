# Exercises to get started

This page offers 15 small, very guided exercises, all easy, to practice the basics of TypeScript: variables, numbers, conditions, loops, functions, text, arrays and objects.

## How to do these exercises

Each exercise is done in five steps:

1. **Read** the instructions and the expected output.
2. **Try on paper**: write the steps in plain English, like a recipe.
3. **Write** the code in a file `exercise.ts`, then **run it**
   (`npx tsx exercise.ts`, see [the introduction](introduction.md)).
4. **Compare** what is printed with the expected output, character by character.
5. Stuck? Read the **first hint**, then the second one. Open the solution last.

> **Tip:** an error from `tsc` is not a failure. It is advice. Read it line by
> line, as in [first steps](premiers-pas.md#reading-a-type-error-line-by-line).

All the exercises are level ★☆☆. The solutions pass `tsc --strict`.

## Variables and numbers

### Exercise 1: A business card

Level: ★☆☆

Create three constants: `firstName`, `city` and `age`. Print a sentence with all
three.

Expected output:

```text
My name is Alice, I live in Lyon and I am 30 years old.
```

> **Hint:** create a variable with `const firstName = "Alice";`.

> **Hint:** to insert a variable into a text, use a template literal between
> backticks: `` `Hello ${firstName}` ``.

> **Solution**
>
> ```typescript
> const firstName = "Alice";
> const city = "Lyon";
> const age = 30;
> console.log(`My name is ${firstName}, I live in ${city} and I am ${age} years old.`);
> ```
>
> ```sortie
> My name is Alice, I live in Lyon and I am 30 years old.
> ```
>
> Why it works: each `${...}` is replaced by the value of the variable.
> TypeScript figures out on its own that `age` is a `number`.

### Exercise 2: The area of a rectangle

Level: ★☆☆

A rectangle is 8 meters long and 5 meters wide. Compute its area
(length × width) and its perimeter (2 × (length + width)).

Expected output:

```text
Area: 40 m²
Perimeter: 26 m
```

> **Hint:** multiplication is written `*`.

> **Hint:** for the perimeter, put the addition in parentheses:
> `2 * (length + width)`.

> **Solution**
>
> ```typescript
> const length = 8;
> const width = 5;
> const area = length * width;
> const perimeter = 2 * (length + width);
> console.log(`Area: ${area} m²`);
> console.log(`Perimeter: ${perimeter} m`);
> ```
>
> ```sortie
> Area: 40 m²
> Perimeter: 26 m
> ```
>
> Why it works: the parentheses make `8 + 5` get computed first (13), then
> `2 * 13`. Without them, you would get `2 * 8 + 5`, which is 21.

### Exercise 3: Minutes into hours

Level: ★☆☆

A movie lasts 137 minutes. Print its length in hours and minutes.

Expected output:

```text
137 minutes = 2 h 17 min
```

> **Hint:** the number of hours is `137 / 60` rounded down:
> `Math.floor(...)`.

> **Hint:** the minutes left over are the remainder of the division: `137 % 60`.

> **Solution**
>
> ```typescript
> const duration = 137;
> const hours = Math.floor(duration / 60);
> const minutes = duration % 60;
> console.log(`${duration} minutes = ${hours} h ${minutes} min`);
> ```
>
> ```sortie
> 137 minutes = 2 h 17 min
> ```
>
> Why it works: `137 / 60` is `2.28…`, rounded down to `2`. And `2 × 60 = 120`, so
> `137 - 120 = 17` minutes are left. See [numbers and calculations](nombres.md).

### Exercise 4: The price with VAT

Level: ★☆☆

A book costs 12.5 € before tax. The VAT is 5.5 %. Print the price including tax with
two digits after the decimal point.

Expected output:

```text
Price incl. tax: 13.19 €
```

> **Hint:** 5.5 % is written `0.055` in a calculation. The price including tax is
> `priceBeforeTax * (1 + 0.055)`.

> **Hint:** `number.toFixed(2)` gives a text with two decimals.

> **Solution**
>
> ```typescript
> const priceBeforeTax = 12.5;
> const vat = 0.055;
> const priceWithTax = priceBeforeTax * (1 + vat);
> console.log(`Price incl. tax: ${priceWithTax.toFixed(2)} €`);
> ```
>
> ```sortie
> Price incl. tax: 13.19 €
> ```
>
> Why it works: the calculation gives `13.1875`. `toFixed(2)` rounds it to `13.19`
> for display.

## Conditions and loops

### Exercise 5: Adult or minor

Level: ★☆☆

For each age in the list `[12, 18, 45]`, print whether it is an adult (18 or older)
or a minor.

Expected output:

```text
12 years old: minor
18 years old: adult
45 years old: adult
```

> **Hint:** go through the list with `for (const age of ages) { ... }`.

> **Hint:** in the loop, test `if (age >= 18)`. Careful: `>=` and not `>`,
> because at 18 you are already an adult.

> **Solution**
>
> ```typescript
> const ages = [12, 18, 45];
> for (const age of ages) {
>   if (age >= 18) {
>     console.log(`${age} years old: adult`);
>   } else {
>     console.log(`${age} years old: minor`);
>   }
> }
> ```
>
> ```sortie
> 12 years old: minor
> 18 years old: adult
> 45 years old: adult
> ```
>
> Why it works: the loop takes each age in turn. The `if` picks one of the two
> sentences.

### Exercise 6: The countdown

Level: ★☆☆

Print a countdown from 5 to 1, then `Go!`.

Expected output:

```text
5
4
3
2
1
Go!
```

> **Hint:** a `for` loop can count down: `for (let i = 5; i >= 1; i--)`.

> **Solution**
>
> ```typescript
> for (let i = 5; i >= 1; i--) {
>   console.log(i);
> }
> console.log("Go!");
> ```
>
> ```sortie
> 5
> 4
> 3
> 2
> 1
> Go!
> ```
>
> Why it works: `i` starts at 5. After each round, `i--` takes away 1. The loop
> stops when `i >= 1` becomes false, so when `i` is 0. The last
> `console.log` is **after** the loop: it is printed only once.

### Exercise 7: The 7 times table

Level: ★☆☆

Print the multiplication table of 7, from 1 to 5.

Expected output:

```text
7 x 1 = 7
7 x 2 = 14
7 x 3 = 21
7 x 4 = 28
7 x 5 = 35
```

> **Hint:** a loop `for (let i = 1; i <= 5; i++)` gives `i` = 1, 2, 3, 4, 5.

> **Hint:** in the loop, print `` `7 x ${i} = ${7 * i}` ``.

> **Solution**
>
> ```typescript
> const table = 7;
> for (let i = 1; i <= 5; i++) {
>   console.log(`${table} x ${i} = ${table * i}`);
> }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```
>
> Why it works: `<=` includes the 5. With `<`, the loop would stop at 4. Change
> `table` to get another table.

### Exercise 8: Adding up a list

Level: ★☆☆

Compute the sum of the expenses `[12, 5, 30, 8]` with a loop.

Expected output:

```text
Total: 55 €
```

> **Hint:** create a variable `let total = 0;` before the loop.

> **Hint:** in the `for...of` loop, add each expense: `total += expense;`.

> **Solution**
>
> ```typescript
> const expenses = [12, 5, 30, 8];
> let total = 0;
> for (const expense of expenses) {
>   total += expense;
> }
> console.log(`Total: ${total} €`);
> ```
>
> ```sortie
> Total: 55 €
> ```
>
> Why it works: `total` is a piggy bank that grows at each round.
>
> | Round | expense | total |
> |---|---|---|
> | start | | 0 |
> | 1 | 12 | 12 |
> | 2 | 5 | 17 |
> | 3 | 30 | 47 |
> | 4 | 8 | 55 |

## Functions

### Exercise 9: The square of a number

Level: ★☆☆

Write a function `square` that receives a number and returns that number multiplied by
itself. Print the square of 3 and of 12.

Expected output:

```text
9
144
```

> **Hint:** the shape of a typed function:
> `function square(n: number): number { ... }`.

> **Hint:** in the function, write `return n * n;`.

> **Solution**
>
> ```typescript
> function square(n: number): number {
>   return n * n;
> }
> console.log(square(3));
> console.log(square(12));
> ```
>
> ```sortie
> 9
> 144
> ```
>
> Why it works: `square(3)` puts `3` into `n`, computes `3 * 3` and returns `9`.
> `console.log` prints what is returned.

### Exercise 10: Greeting with a default value

Level: ★☆☆

Write a function `greet` that receives a first name and returns `Hello, <name>!`.
Without a name, it must say `Hello, stranger!`.

Expected output:

```text
Hello, Léa!
Hello, stranger!
```

> **Hint:** a default value is written with `=` in the parameters:
> `function greet(name = "stranger")`.

> **Solution**
>
> ```typescript
> function greet(name: string = "stranger"): string {
>   return `Hello, ${name}!`;
> }
> console.log(greet("Léa"));
> console.log(greet());
> ```
>
> ```sortie
> Hello, Léa!
> Hello, stranger!
> ```
>
> Why it works: when you call `greet()` without an argument, `name` takes the
> default value. See [functions](fonctions.md).

### Exercise 11: Even or odd

Level: ★☆☆

Write a function `isEven` that returns `true` if a number is even, `false`
otherwise. Test it on 4 and 7.

Expected output:

```text
4 is even: true
7 is even: false
```

> **Hint:** a number is even if the remainder of its division by 2 is 0: `n % 2`.

> **Hint:** the comparison `n % 2 === 0` already gives `true` or `false`. You can
> return it directly.

> **Solution**
>
> ```typescript
> function isEven(n: number): boolean {
>   return n % 2 === 0;
> }
> console.log(`4 is even: ${isEven(4)}`);
> console.log(`7 is even: ${isEven(7)}`);
> ```
>
> ```sortie
> 4 is even: true
> 7 is even: false
> ```
>
> Why it works: `4 % 2` is `0`, so `0 === 0` is true. `7 % 2` is `1`, so
> the comparison is false.

## Text

### Exercise 12: The initials

Level: ★☆☆

Starting from `firstName = "ada"` and `lastName = "lovelace"`, print the initials in
uppercase, each followed by a period.

Expected output:

```text
A.L.
```

> **Hint:** the first character of a text is `text[0]`, or better
> `text.charAt(0)`, which always gives a `string`.

> **Hint:** `.toUpperCase()` turns a text into uppercase.

> **Solution**
>
> ```typescript
> const firstName = "ada";
> const lastName = "lovelace";
> const initials = `${firstName.charAt(0)}.${lastName.charAt(0)}.`.toUpperCase();
> console.log(initials);
> ```
>
> ```sortie
> A.L.
> ```
>
> Why it works: you first build `"a.l."`, then `toUpperCase` turns it
> into `"A.L."`. The periods don't change.

### Exercise 13: Counting words

Level: ★☆☆

Count the words of the sentence `"the cat is on the armchair"` and print the longest one.

Expected output:

```text
6 words
The longest: armchair
```

> **Hint:** `sentence.split(" ")` cuts the sentence at each space and gives an array
> of words. Its length is `.length`.

> **Hint:** for the longest one, keep a variable `let longest = "";` and
> replace it in a loop when a word is longer than it.

> **Solution**
>
> ```typescript
> const sentence = "the cat is on the armchair";
> const words = sentence.split(" ");
> console.log(`${words.length} words`);
>
> let longest = "";
> for (const word of words) {
>   if (word.length > longest.length) {
>     longest = word;
>   }
> }
> console.log(`The longest: ${longest}`);
> ```
>
> ```sortie
> 6 words
> The longest: armchair
> ```
>
> Why it works: `longest` keeps the current champion. `"the"` replaces `""`,
> then `"cat"` is not longer, and only `"armchair"` (8 letters) does better.

## Arrays and objects

### Exercise 14: Keeping the good grades

Level: ★☆☆

From the grades `[8, 15, 11, 6, 19]`, keep only those of at least 10, and
print them with how many there are.

Expected output:

```text
[ 15, 11, 19 ]
3 grades above average
```

> **Hint:** `filter` keeps the elements for which a function answers `true`:
> `grades.filter((n) => ...)`.

> **Hint:** the condition is `n >= 10`.

> **Solution**
>
> ```typescript
> const grades = [8, 15, 11, 6, 19];
> const good = grades.filter((n) => n >= 10);
> console.log(good);
> console.log(`${good.length} grades above average`);
> ```
>
> ```sortie
> [ 15, 11, 19 ]
> 3 grades above average
> ```
>
> Why it works: `filter` asks the question `n >= 10` to each grade and keeps
> those that answer yes. The `grades` array is not changed. See
> [arrays](collections.md).

### Exercise 15: A book record

Level: ★☆☆

Write an interface `Book` with a `title` (text) and a number of `pages`. Create
two books in an array, then print one line per book.

Expected output:

```text
Dune: 600 pages
The Little Prince: 96 pages
```

> **Hint:** an interface is written
> `interface Book { title: string; pages: number; }`.

> **Hint:** the array has the type `Book[]`. Go through it with `for...of`.

> **Solution**
>
> ```typescript
> interface Book {
>   title: string;
>   pages: number;
> }
>
> const books: Book[] = [
>   { title: "Dune", pages: 600 },
>   { title: "The Little Prince", pages: 96 },
> ];
>
> for (const book of books) {
>   console.log(`${book.title}: ${book.pages} pages`);
> }
> ```
>
> ```sortie
> Dune: 600 pages
> The Little Prince: 96 pages
> ```
>
> Why it works: the interface is the template of the record. TypeScript checks that
> each book really has a `title` and `pages`. See [objects](objets.md).

Well done! To go further, try the level ★★☆ [solved exercises](exercices.md).

Next: [unions](unions.md).
