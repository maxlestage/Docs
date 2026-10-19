# Exercises to get started

This page offers 15 very guided exercises, all easy, to practice the basics of JavaScript: variables, numbers, conditions, loops, functions, text, arrays and objects.

## How to do these exercises

Take them in order. Each one asks for only **one** new idea.

For each exercise:

1. **Read** the instructions and the expected output.
2. **Try on paper**: write the steps in plain English ("first I create…, then I
   display…").
3. **Write** the code in a file `exercise.mjs`.
4. **Run it** with `node exercise.mjs`.
5. **Compare** what is displayed with the expected output, character by character.
6. Stuck? Read **the first hint**, try again. Then the second one. The solution comes
   last.

```text
 read → paper → write → run → compare
                  ▲               │
                  └── not same? ──┘
```

> **Tip:** an error is not a failure: it is one more hint. Read the message from top to
> bottom (see [reading an error](../debuter/lire-une-erreur.md)).

## Variables and numbers

### Exercise 1: A business card

Level: ★☆☆

Create three constants: `firstName`, `city` and `job`. Display them on three lines, with
a label in front of each one.

Expected output:

```text
First name: Alice
City: Lyon
Job: baker
```

> **Hint:** a constant is created with `const firstName = "Alice";`.

> **Hint:** to put a variable inside a text, use a template literal with backticks:
> `` `First name: ${firstName}` `` (see [first steps](premiers-pas.md)).

> **Solution**
>
> ```javascript
> const firstName = "Alice";
> const city = "Lyon";
> const job = "baker";
>
> console.log(`First name: ${firstName}`);
> console.log(`City: ${city}`);
> console.log(`Job: ${job}`);
> ```
>
> ```sortie
> First name: Alice
> City: Lyon
> Job: baker
> ```
>
> Why it works: in a template literal, `${firstName}` is replaced by the value stored in
> the box `firstName`.

### Exercise 2: The area of a rectangle

Level: ★☆☆

A rectangle is 7 m long and 4 m wide. Calculate its area (length × width) and display
it.

Expected output:

```text
Area: 28 m²
```

> **Hint:** store the length and the width in two constants.

> **Hint:** multiplication is written `*`: `const area = length * width;`.

> **Solution**
>
> ```javascript
> const length = 7;
> const width = 4;
> const area = length * width;
>
> console.log(`Area: ${area} m²`);
> ```
>
> ```sortie
> Area: 28 m²
> ```
>
> Why it works: `7 * 4` is calculated once, stored in `area`, then displayed.

### Exercise 3: The price with VAT

Level: ★☆☆

A book costs €12.50 before tax. VAT adds 5.5%. Display the price including tax, with
**two** digits after the decimal point.

Expected output:

```text
Price with tax: 13.19 €
```

> **Hint:** adding 5.5% means multiplying by `1.055`. In JavaScript, decimal numbers are
> written with a **dot**.

> **Hint:** `number.toFixed(2)` rounds and keeps two digits after the decimal point
> (see [numbers](nombres.md#displaying-with-tofixed)).

> **Solution**
>
> ```javascript
> const priceBeforeTax = 12.5;
> const priceWithTax = priceBeforeTax * 1.055;
>
> console.log(`Price with tax: ${priceWithTax.toFixed(2)} €`);
> ```
>
> ```sortie
> Price with tax: 13.19 €
> ```
>
> Why it works: `12.5 * 1.055` is `13.1875`. `toFixed(2)` rounds it to `"13.19"`.

### Exercise 4: Minutes into hours

Level: ★☆☆

A movie lasts 135 minutes. Display its length in hours and minutes.

Expected output:

```text
2 h 15 min
```

> **Hint:** one hour is 60 minutes. How many times does 60 "fit" into 135? That is
> `Math.floor(135 / 60)`.

> **Hint:** what is left after the hours is the **remainder** of the division:
> `135 % 60`.

> **Solution**
>
> ```javascript
> const duration = 135;
> const hours = Math.floor(duration / 60);
> const minutes = duration % 60;
>
> console.log(`${hours} h ${minutes} min`);
> ```
>
> ```sortie
> 2 h 15 min
> ```
>
> Why it works: `135 / 60` is `2.25`; `Math.floor` keeps `2`. And `135 - 2 × 60` leaves
> `15`, which is what `135 % 60` gives.

## Conditions and loops

### Exercise 5: Adult or minor

Level: ★☆☆

For each age `15` and `18`, display whether the person is an adult or a minor.

Expected output:

```text
15 years old: minor
18 years old: adult
```

> **Hint:** start with a single age: `const age = 15;` and an `if (age >= 18)`.

> **Hint:** to handle both ages, put the `if` inside a loop
> `for (const age of [15, 18])` (see [conditions and loops](conditions-boucles.md)).

> **Solution**
>
> ```javascript
> for (const age of [15, 18]) {
>   if (age >= 18) {
>     console.log(`${age} years old: adult`);
>   } else {
>     console.log(`${age} years old: minor`);
>   }
> }
> ```
>
> ```sortie
> 15 years old: minor
> 18 years old: adult
> ```
>
> Why it works: `>=` means "greater than or equal to". `18 >= 18` is true, so 18 years
> old is an adult.

### Exercise 6: The larger of the two

Level: ★☆☆

With `a = 8` and `b = 13`, display the larger of the two numbers, with a single line of
calculation.

Expected output:

```text
The larger one: 13
```

> **Hint:** the ternary operator is written `condition ? ifTrue : ifFalse`.

> **Hint:** the condition is `a > b`.

> **Solution**
>
> ```javascript
> const a = 8;
> const b = 13;
> const larger = a > b ? a : b;
>
> console.log(`The larger one: ${larger}`);
> ```
>
> ```sortie
> The larger one: 13
> ```
>
> Why it works: `8 > 13` is false, so you get the value after the `:`, `b`.
> `Math.max(a, b)` also gives the right result.

### Exercise 7: The countdown

Level: ★☆☆

Display a countdown from 5 to 1, then `Go!`.

Expected output:

```text
5
4
3
2
1
Go!
```

> **Hint:** a `for` loop can **count down**: you start from 5 and subtract 1 at each
> turn with `i--`.

> **Hint:** the loop goes on as long as `i >= 1`. The final message goes **after** the
> closing curly brace of the loop.

> **Solution**
>
> ```javascript
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
> Why it works: when `i` is 0, `0 >= 1` is false, and the loop stops. The next line runs
> only once.

### Exercise 8: The 7 times table

Level: ★☆☆

Display the multiplication table of 7, from 1 to 5.

Expected output:

```text
7 x 1 = 7
7 x 2 = 14
7 x 3 = 21
7 x 4 = 28
7 x 5 = 35
```

> **Hint:** a loop `for (let i = 1; i <= 5; i++)` gives `i` = 1, 2, 3, 4, 5.

> **Hint:** inside the template literal, the calculation can be done directly:
> `${7 * i}`.

> **Solution**
>
> ```javascript
> for (let i = 1; i <= 5; i++) {
>   console.log(`7 x ${i} = ${7 * i}`);
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
> Why it works: at each turn, `i` changes, and the same line displays a new
> calculation.

### Exercise 9: The sum from 1 to 10

Level: ★☆☆

Add up all the numbers from 1 to 10 with a loop, then display the total.

Expected output:

```text
Total: 55
```

> **Hint:** create a box `total` that holds `0` **before** the loop. It must be able to
> change: use `let`.

> **Hint:** at each turn, add `i` into the box: `total = total + i;` (or
> `total += i;`). Here are the first turns:
>
> | Step | i | total |
> | --- | --- | --- |
> | start | – | 0 |
> | 1 | 1 | 1 |
> | 2 | 2 | 3 |
> | 3 | 3 | 6 |

> **Solution**
>
> ```javascript
> let total = 0;
> for (let i = 1; i <= 10; i++) {
>   total += i;
> }
> console.log(`Total: ${total}`);
> ```
>
> ```sortie
> Total: 55
> ```
>
> Why it works: `total` is created outside the loop, so it keeps its value from one
> turn to the next. If it were created inside, it would start again from 0 at each turn.

## Functions

### Exercise 10: Saying hello

Level: ★☆☆

Write a function `sayHello(firstName)` that **returns** `Hello, <first name>!`. Call it
with `"Alice"` then `"Bob"`, and display the results.

Expected output:

```text
Hello, Alice!
Hello, Bob!
```

> **Hint:** a function is written `function sayHello(firstName) { ... }`.

> **Hint:** inside, `return` sends the text back. Outside,
> `console.log(sayHello("Alice"))` displays it (see [functions](fonctions.md)).

> **Solution**
>
> ```javascript
> function sayHello(firstName) {
>   return `Hello, ${firstName}!`;
> }
>
> console.log(sayHello("Alice"));
> console.log(sayHello("Bob"));
> ```
>
> ```sortie
> Hello, Alice!
> Hello, Bob!
> ```
>
> Why it works: the parameter `firstName` receives `"Alice"` on the first call, then
> `"Bob"` on the second. The same recipe is used twice.

### Exercise 11: The square of a number

Level: ★☆☆

Write an arrow function `square(n)` that returns `n × n`. Display the square of 3, then
the sum of the squares of 3 and 4.

Expected output:

```text
9
25
```

> **Hint:** a short arrow function is written `const square = (n) => n * n;`.

> **Hint:** the result of a function is a value: you can add up
> `square(3) + square(4)`.

> **Solution**
>
> ```javascript
> const square = (n) => n * n;
>
> console.log(square(3));
> console.log(square(3) + square(4));
> ```
>
> ```sortie
> 9
> 25
> ```
>
> Why it works: without curly braces, an arrow automatically returns its calculation.
> `square(3) + square(4)` becomes `9 + 16`.

## Text, arrays and objects

### Exercise 12: Shouting a word

Level: ★☆☆

With the word `"hello"`, display it in uppercase, followed by its number of letters.

Expected output:

```text
HELLO (5 letters)
```

> **Hint:** `word.toUpperCase()` puts it in uppercase.

> **Hint:** `word.length` gives the number of characters. Warning: **no** parentheses
> (see [working with text](texte.md)).

> **Solution**
>
> ```javascript
> const word = "hello";
> console.log(`${word.toUpperCase()} (${word.length} letters)`);
> ```
>
> ```sortie
> HELLO (5 letters)
> ```
>
> Why it works: `toUpperCase()` returns a **new** text in uppercase; `word` itself does
> not change.

### Exercise 13: The initials

Level: ★☆☆

With the full name `"Ada Lovelace"`, display its initials.

Expected output:

```text
A.L.
```

> **Hint:** `fullName.split(" ")` cuts the text at the spaces and gives an array:
> `["Ada", "Lovelace"]`.

> **Hint:** the first letter of a word `w` is `w[0]`.

> **Solution**
>
> ```javascript
> const fullName = "Ada Lovelace";
> const [firstName, lastName] = fullName.split(" ");
>
> console.log(`${firstName[0]}.${lastName[0]}.`);
> ```
>
> ```sortie
> A.L.
> ```
>
> Why it works: `split` gives two words, stored in `firstName` and `lastName`. The index
> `[0]` takes the first character of each one.

### Exercise 14: The average grade

Level: ★☆☆

With the array `[12, 15, 9, 16]`, calculate the average of the grades and display it.

Expected output:

```text
Average: 13
```

> **Hint:** first the sum, as in exercise 9, but with `for (const grade of grades)`.

> **Hint:** the average is the sum divided by the number of grades: `grades.length`
> (see [arrays](collections.md)).

> **Solution**
>
> ```javascript
> const grades = [12, 15, 9, 16];
>
> let sum = 0;
> for (const grade of grades) {
>   sum += grade;
> }
>
> console.log(`Average: ${sum / grades.length}`);
> ```
>
> ```sortie
> Average: 13
> ```
>
> Why it works: the sum is `52`, there are `4` grades, and `52 / 4` makes `13`.

### Exercise 15: A book's record

Level: ★☆☆

Create an object `book` with a `title` (`"The Little Prince"`), an `author`
(`"Saint-Exupéry"`) and a `year` (`1943`). Display each property on its own line.

Expected output:

```text
title: The Little Prince
author: Saint-Exupéry
year: 1943
```

> **Hint:** an object is written between curly braces:
> `{ title: "...", author: "...", ... }`.

> **Hint:** `Object.entries(book)` gives the list of `[key, value]` pairs. Loop over it
> with `for (const [key, value] of ...)` (see
> [objects](objets.md#looping-over-an-object)).

> **Solution**
>
> ```javascript
> const book = {
>   title: "The Little Prince",
>   author: "Saint-Exupéry",
>   year: 1943,
> };
>
> for (const [key, value] of Object.entries(book)) {
>   console.log(`${key}: ${value}`);
> }
> ```
>
> ```sortie
> title: The Little Prince
> author: Saint-Exupéry
> year: 1943
> ```
>
> Why it works: `Object.entries` turns the object into an array of pairs, in the order
> the properties were written. Each pair is unpacked into `key` and `value`.

## What next?

Well done! If you did these 15 exercises, you know the basics. More exercises, more
varied, are waiting for you in [the solved exercises](exercices.md).

Next: [classes](types.md).
