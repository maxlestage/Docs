# Solved exercises: second set

This page offers 12 new solved exercises in TypeScript, from easiest to hardest, with hints, explained solutions and trace tables.

## How to use this page

These exercises are **new**: they repeat neither [the exercises to get
started](exercices-faciles.md) nor [the first set](exercices.md). Each one has a
level:

- ★☆☆: easy, with the basics only;
- ★★☆: medium, with arrays, objects and types;
- ★★★: harder, with generics, classes and unions.

For each exercise:

1. Read the instructions and the expected output.
2. Work through the steps on paper, with a [trace table](pas-a-pas.md).
3. Write your code in `exercise.ts`, check it with
   `npx tsc --noEmit --strict exercise.ts`, then run it with `npx tsx exercise.ts`.
4. Stuck? Read the hint. Open the solution last.

> **Tip:** a solution different from ours can be right. What matters: the right
> output, and no error from `tsc --strict`.

## First steps

### Exercise 1: Celsius to Fahrenheit

Level: ★☆☆

Write a function `toFahrenheit(celsius: number): number`. The formula is:
Fahrenheit = Celsius × 9 / 5 + 32. Print the conversion of 0, 20, 37 and 100 degrees.

Expected output:

```text
0 °C = 32 °F
20 °C = 68 °F
37 °C = 98.6 °F
100 °C = 212 °F
```

> **Hint:** write the function first and test it on `0` (the result must be `32`).
> Then go through `[0, 20, 37, 100]` with `for...of`.

> **Solution**
>
> ```typescript
> function toFahrenheit(celsius: number): number {
>   return (celsius * 9) / 5 + 32;
> }
>
> for (const c of [0, 20, 37, 100]) {
>   console.log(`${c} °C = ${toFahrenheit(c)} °F`);
> }
> ```
>
> ```sortie
> 0 °C = 32 °F
> 20 °C = 68 °F
> 37 °C = 98.6 °F
> 100 °C = 212 °F
> ```
>
> Why it works: the function is a small converting machine. The loop gives it each
> temperature, one at a time.
>
> | Round | c | c × 9 | ÷ 5 | + 32 |
> |---|---|---|---|---|
> | 1 | 0 | 0 | 0 | 32 |
> | 2 | 20 | 180 | 36 | 68 |
> | 3 | 37 | 333 | 66.6 | 98.6 |
> | 4 | 100 | 900 | 180 | 212 |

### Exercise 2: The largest of three

Level: ★☆☆

Write a function `largest(a: number, b: number, c: number): number` that returns the
largest of the three numbers, **without** using `Math.max`. Test it with
`(4, 9, 2)`, `(7, 3, 5)` and `(1, 1, 8)`.

Expected output:

```text
9
7
8
```

> **Hint:** store `a` in a variable `max`. Then compare `b` with `max`, then `c` with
> `max`, using two `if`s.

> **Solution**
>
> ```typescript
> function largest(a: number, b: number, c: number): number {
>   let max = a;
>   if (b > max) max = b;
>   if (c > max) max = c;
>   return max;
> }
>
> console.log(largest(4, 9, 2));
> console.log(largest(7, 3, 5));
> console.log(largest(1, 1, 8));
> ```
>
> ```sortie
> 9
> 7
> 8
> ```
>
> Why it works: `max` is the current champion. Each new number challenges it; if it is
> larger, it takes its place.

### Exercise 3: Writing a word backwards

Level: ★☆☆

Write a function `backwards(word: string): string` that returns the word written from
right to left, with a loop (without `reverse`). Test it with `"lemon"`.

Expected output:

```text
nomel
```

> **Hint:** start from an empty string `let result = "";`. Go through the letters with
> `for (const letter of word)`.

> **Hint:** stick each letter **in front of** the result: `result = letter + result;`.

> **Solution**
>
> ```typescript
> function backwards(word: string): string {
>   let result = "";
>   for (const letter of word) {
>     result = letter + result;
>   }
>   return result;
> }
>
> console.log(backwards("lemon"));
> ```
>
> ```sortie
> nomel
> ```
>
> Why it works: each new letter goes **in front of** the others, like a plate put on
> top of a stack.
>
> | Round | letter | result |
> |---|---|---|
> | start | | `""` |
> | 1 | l | `"l"` |
> | 2 | e | `"el"` |
> | 3 | m | `"mel"` |
> | 4 | o | `"omel"` |
> | 5 | n | `"nomel"` |

### Exercise 4: The sum of the digits

Level: ★☆☆

Write a function `digitSum(n: number): number` that adds up the digits of a positive
integer: for 4721, it is 4 + 7 + 2 + 1 = 14. Do not use strings: only calculations.

Expected output:

```text
14
```

> **Hint:** `n % 10` gives the last digit (4721 % 10 = 1).

> **Hint:** `Math.floor(n / 10)` removes the last digit (4721 → 472). Repeat with
> `while (n > 0)`.

> **Solution**
>
> ```typescript
> function digitSum(n: number): number {
>   let sum = 0;
>   while (n > 0) {
>     sum += n % 10;
>     n = Math.floor(n / 10);
>   }
>   return sum;
> }
>
> console.log(digitSum(4721));
> ```
>
> ```sortie
> 14
> ```
>
> Why it works: you "eat" the number from the right, one digit per round.
>
> | Round | n at the start | n % 10 | sum | n at the end |
> |---|---|---|---|---|
> | 1 | 4721 | 1 | 1 | 472 |
> | 2 | 472 | 2 | 3 | 47 |
> | 3 | 47 | 7 | 10 | 4 |
> | 4 | 4 | 4 | 14 | 0 |
>
> On the next round, the test `0 > 0` is false: the loop stops.

## Arrays, objects and types

### Exercise 5: The palindrome

Level: ★★☆

A palindrome reads the same in both directions, if you ignore capital letters and
spaces: "Never odd or even". Write `isPalindrome(sentence: string): boolean` and test
it on three sentences.

Expected output:

```text
kayak: yes
Never odd or even: yes
TypeScript: no
```

> **Hint:** clean the sentence first: `toLowerCase()`, then `replaceAll(" ", "")`.

> **Hint:** to reverse a string: `[...text].reverse().join("")`.

> **Solution**
>
> ```typescript
> function isPalindrome(sentence: string): boolean {
>   const clean = sentence.toLowerCase().replaceAll(" ", "");
>   const reversed = [...clean].reverse().join("");
>   return clean === reversed;
> }
>
> for (const s of ["kayak", "Never odd or even", "TypeScript"]) {
>   console.log(`${s}: ${isPalindrome(s) ? "yes" : "no"}`);
> }
> ```
>
> ```sortie
> kayak: yes
> Never odd or even: yes
> TypeScript: no
> ```
>
> Why it works: you compare the cleaned sentence with its reflection in a mirror.
> `[...clean]` cuts the string into an array of letters, `reverse` flips it,
> `join("")` glues the letters back together.

### Exercise 6: Counting letters

Level: ★★☆

Write a function `countLetters(word: string): Record<string, number>` that counts how
many times each letter appears. Print the result for `"banana"`, one letter per line.

Expected output:

```text
b: 1
a: 3
n: 2
```

> **Hint:** a `Record<string, number>` object works as a notebook: the letter is the
> key, the number is the value. Start with `{}`.

> **Hint:** `count[letter] = (count[letter] ?? 0) + 1;` handles the first encounter,
> when the entry does not exist yet.

> **Solution**
>
> ```typescript
> function countLetters(word: string): Record<string, number> {
>   const count: Record<string, number> = {};
>   for (const letter of word) {
>     count[letter] = (count[letter] ?? 0) + 1;
>   }
>   return count;
> }
>
> const result = countLetters("banana");
> for (const [letter, total] of Object.entries(result)) {
>   console.log(`${letter}: ${total}`);
> }
> ```
>
> ```sortie
> b: 1
> a: 3
> n: 2
> ```
>
> Why it works: the first time you see a letter, its entry is `undefined`, and `?? 0`
> replaces it with 0.
>
> | Round | letter | count after the round |
> |---|---|---|
> | 1 | b | `{ b: 1 }` |
> | 2 | a | `{ b: 1, a: 1 }` |
> | 3 | n | `{ b: 1, a: 1, n: 1 }` |
> | 4 | a | `{ b: 1, a: 2, n: 1 }` |
> | 5 | n | `{ b: 1, a: 2, n: 2 }` |
> | 6 | a | `{ b: 1, a: 3, n: 2 }` |

### Exercise 7: The minimum and the maximum

Level: ★★☆

Write a function `bounds(numbers: readonly number[])` that returns an object
`{ min, max }` in **a single** pass over the array, or `undefined` if the array is
empty. Print the bounds of the temperatures `[14, 9, 21, 17, 6]`, then those of an
empty array.

Expected output:

```text
min = 6, max = 21
empty array
```

> **Hint:** the return type is `{ min: number; max: number } | undefined`.

> **Hint:** handle the empty case first with a `return`. Then start from `numbers[0]`
> for `min` and `max`.

> **Solution**
>
> ```typescript
> type Bounds = { min: number; max: number };
>
> function bounds(numbers: readonly number[]): Bounds | undefined {
>   if (numbers.length === 0) return undefined;
>   let min = numbers[0];
>   let max = numbers[0];
>   for (const n of numbers) {
>     if (n < min) min = n;
>     if (n > max) max = n;
>   }
>   return { min, max };
> }
>
> for (const list of [[14, 9, 21, 17, 6], []]) {
>   const b = bounds(list);
>   console.log(b === undefined ? "empty array" : `min = ${b.min}, max = ${b.max}`);
> }
> ```
>
> ```sortie
> min = 6, max = 21
> empty array
> ```
>
> Why it works: `min` and `max` are two champions who keep their title as long as
> nobody beats them. The type `Bounds | undefined` forces the caller to test the empty
> case: without the test `b === undefined`, `b.min` would be rejected by `tsc`.

### Exercise 8: Sorting groceries by aisle

Level: ★★☆

A shopping list contains items. Each one has a `name` and an `aisle`: `"fresh"`,
`"grocery"` or `"drinks"`. Write a function `byAisle` that returns, for each aisle,
the list of names. Print one line per aisle.

Expected output:

```text
fresh: yogurt, butter
grocery: pasta, rice
drinks: juice
```

> **Hint:** describe the type `Aisle` as a union of strings, and the result as
> `Record<Aisle, string[]>`.

> **Hint:** create the result with three empty arrays, then do
> `result[item.aisle].push(item.name)` for each item.

> **Solution**
>
> ```typescript
> type Aisle = "fresh" | "grocery" | "drinks";
>
> interface Item {
>   name: string;
>   aisle: Aisle;
> }
>
> function byAisle(items: readonly Item[]): Record<Aisle, string[]> {
>   const result: Record<Aisle, string[]> = { fresh: [], grocery: [], drinks: [] };
>   for (const item of items) {
>     result[item.aisle].push(item.name);
>   }
>   return result;
> }
>
> const shopping: Item[] = [
>   { name: "yogurt", aisle: "fresh" },
>   { name: "pasta", aisle: "grocery" },
>   { name: "juice", aisle: "drinks" },
>   { name: "butter", aisle: "fresh" },
>   { name: "rice", aisle: "grocery" },
> ];
>
> const sorted = byAisle(shopping);
> for (const [aisle, names] of Object.entries(sorted)) {
>   console.log(`${aisle}: ${names.join(", ")}`);
> }
> ```
>
> ```sortie
> fresh: yogurt, butter
> grocery: pasta, rice
> drinks: juice
> ```
>
> Why it works: the result is a chest with three drawers, one per aisle. Each item
> falls into the drawer written on its label. Thanks to the type `Aisle`, a typo like
> `aisle: "frsh"` is rejected by `tsc`.

### Exercise 9: Checking a password

Level: ★★☆

Write a function `problems(password: string): string[]` that returns the list of rules
that are **not** met: at least 8 characters, at least one digit, at least one capital
letter. Test three passwords.

Expected output:

```text
cat: too short, no digit, no capital letter
kitten2024: no capital letter
Kitten2024: perfect
```

> **Hint:** to know if a string contains a digit: `/[0-9]/.test(text)`. For a capital
> letter: `/[A-Z]/.test(text)`.

> **Hint:** start from an empty array `const list: string[] = [];` and do one `push`
> per rule that is not met.

> **Solution**
>
> ```typescript
> function problems(password: string): string[] {
>   const list: string[] = [];
>   if (password.length < 8) list.push("too short");
>   if (!/[0-9]/.test(password)) list.push("no digit");
>   if (!/[A-Z]/.test(password)) list.push("no capital letter");
>   return list;
> }
>
> for (const pwd of ["cat", "kitten2024", "Kitten2024"]) {
>   const list = problems(pwd);
>   console.log(`${pwd}: ${list.length === 0 ? "perfect" : list.join(", ")}`);
> }
> ```
>
> ```sortie
> cat: too short, no digit, no capital letter
> kitten2024: no capital letter
> Kitten2024: perfect
> ```
>
> Why it works: the three `if`s are independent, like three checks in a row. Each
> failed check adds a remark. An empty list means everything is fine. The `string[]`
> annotation is needed: the array is empty at the start.

## Going further

### Exercise 10: Giving change

Level: ★★★

A vending machine gives change with as few coins as possible. The coins are worth, in
cents: 200, 100, 50, 20, 10, 5, 2 and 1. Write `giveChange(cents: number): number[]`
that returns the coins to give back. Test with 386 cents.

Expected output:

```text
200 + 100 + 50 + 20 + 10 + 5 + 1
7 coins
```

> **Hint:** go through the coins from the largest to the smallest.

> **Hint:** for each coin, use `while (remaining >= coin)`: you give the coin back and
> take it off what remains, as long as you can.

> **Solution**
>
> ```typescript
> const COINS = [200, 100, 50, 20, 10, 5, 2, 1] as const;
>
> function giveChange(cents: number): number[] {
>   const change: number[] = [];
>   let remaining = cents;
>   for (const coin of COINS) {
>     while (remaining >= coin) {
>       change.push(coin);
>       remaining -= coin;
>     }
>   }
>   return change;
> }
>
> const coins = giveChange(386);
> console.log(coins.join(" + "));
> console.log(`${coins.length} coins`);
> ```
>
> ```sortie
> 200 + 100 + 50 + 20 + 10 + 5 + 1
> 7 coins
> ```
>
> Why it works: it is what a cashier does. They always take the largest coin that
> "fits" in what is left to give back.
>
> | coin | remaining before | coins given | remaining after |
> |---|---|---|---|
> | 200 | 386 | 200 | 186 |
> | 100 | 186 | 100 | 86 |
> | 50 | 86 | 50 | 36 |
> | 20 | 36 | 20 | 16 |
> | 10 | 16 | 10 | 6 |
> | 5 | 6 | 5 | 1 |
> | 2 | 1 | (none: 1 < 2) | 1 |
> | 1 | 1 | 1 | 0 |
>
> You work in whole cents on purpose: with `3.86` €, decimal calculations would give
> remainders like `0.009999999`.

### Exercise 11: A generic queue

Level: ★★★

Write a **generic** class `Queue<T>`, like the line at a counter: you join at the
back, you leave from the front. It has three methods: `enter(element: T)`,
`leave(): T | undefined` and `size(): number`. Make Alice, Bob and Chloé join, then
make four people leave.

Expected output:

```text
3 people waiting
Next please: Alice
Next please: Bob
Next please: Chloé
Nobody waiting
```

> **Hint:** store the elements in a private array `#elements: T[] = []`. `push` adds
> at the end; `shift` removes the first one and returns `undefined` if the array is
> empty (see [generics](generiques.md)).

> **Solution**
>
> ```typescript
> class Queue<T> {
>   #elements: T[] = [];
>
>   enter(element: T): void {
>     this.#elements.push(element);
>   }
>
>   leave(): T | undefined {
>     return this.#elements.shift();
>   }
>
>   size(): number {
>     return this.#elements.length;
>   }
> }
>
> const counter = new Queue<string>();
> counter.enter("Alice");
> counter.enter("Bob");
> counter.enter("Chloé");
> console.log(`${counter.size()} people waiting`);
>
> for (let i = 0; i < 4; i++) {
>   const next = counter.leave();
>   console.log(next === undefined ? "Nobody waiting" : `Next please: ${next}`);
> }
> ```
>
> ```sortie
> 3 people waiting
> Next please: Alice
> Next please: Bob
> Next please: Chloé
> Nobody waiting
> ```
>
> Why it works: `T` is an empty slot that you fill at creation (`Queue<string>`).
> After that, `counter.enter(42)` would be rejected by `tsc`.
>
> | Step | Action | queue (front → back) | Prints |
> |---|---|---|---|
> | 1 | enter Alice, Bob, Chloé | Alice, Bob, Chloé | `3 people waiting` |
> | 2 | leave | Bob, Chloé | `Next please: Alice` |
> | 3 | leave | Chloé | `Next please: Bob` |
> | 4 | leave | (empty) | `Next please: Chloé` |
> | 5 | leave → `undefined` | (empty) | `Nobody waiting` |

### Exercise 12: Reading stock commands

Level: ★★★

A shop receives lines of text: `"add 10"`, `"remove 3"`… Write:

- a type `Command`: `{ type: "add"; quantity: number }` or
  `{ type: "remove"; quantity: number }`;
- a function `read(line: string): Command | string` that returns the command, or an
  error message (a `string`) if the line is wrong;
- a loop that starts from a stock of 0, applies the valid commands and reports the
  others. A removal larger than the stock is refused.

The lines: `"add 10"`, `"remove 3"`, `"sell 2"`, `"remove 20"`, `"add five"`,
`"add 5"`.

Expected output:

```text
add 10 -> stock 10
remove 3 -> stock 7
sell 2 -> error: unknown action "sell"
remove 20 -> refused: only 7 in stock
add five -> error: invalid quantity "five"
add 5 -> stock 12
```

> **Hint:** `line.split(" ")` gives `["add", "10"]`. `Number("five")` gives `NaN`:
> test it with `Number.isNaN`.

> **Hint:** after the call, `typeof result === "string"` separates the errors from the
> commands. Then `result.type` tells which command to apply (see [unions](unions.md)).

> **Solution**
>
> ```typescript
> type Command =
>   | { type: "add"; quantity: number }
>   | { type: "remove"; quantity: number };
>
> function read(line: string): Command | string {
>   const [action, text] = line.split(" ");
>   const quantity = Number(text);
>   if (Number.isNaN(quantity) || quantity <= 0) {
>     return `invalid quantity "${text}"`;
>   }
>   if (action === "add" || action === "remove") {
>     return { type: action, quantity };
>   }
>   return `unknown action "${action}"`;
> }
>
> const lines = ["add 10", "remove 3", "sell 2", "remove 20", "add five", "add 5"];
> let stock = 0;
>
> for (const line of lines) {
>   const result = read(line);
>   if (typeof result === "string") {
>     console.log(`${line} -> error: ${result}`);
>   } else if (result.type === "add") {
>     stock += result.quantity;
>     console.log(`${line} -> stock ${stock}`);
>   } else if (result.quantity > stock) {
>     console.log(`${line} -> refused: only ${stock} in stock`);
>   } else {
>     stock -= result.quantity;
>     console.log(`${line} -> stock ${stock}`);
>   }
> }
> ```
>
> ```sortie
> add 10 -> stock 10
> remove 3 -> stock 7
> sell 2 -> error: unknown action "sell"
> remove 20 -> refused: only 7 in stock
> add five -> error: invalid quantity "five"
> add 5 -> stock 12
> ```
>
> Why it works: `read` is a mail desk that sorts the post. It returns either a clean
> command or an error letter. After `typeof result === "string"`, TypeScript knows
> that `result` is a `Command` in the other branches: it allows `result.type` and
> `result.quantity`.
>
> | Line | read returns | stock after |
> |---|---|---|
> | add 10 | `{ type: "add", quantity: 10 }` | 10 |
> | remove 3 | `{ type: "remove", quantity: 3 }` | 7 |
> | sell 2 | `'unknown action "sell"'` | 7 |
> | remove 20 | `{ type: "remove", quantity: 20 }`, but 20 > 7 | 7 |
> | add five | `'invalid quantity "five"'` | 7 |
> | add 5 | `{ type: "add", quantity: 5 }` | 12 |

## Summary

- Exercises 1 to 4 go over the basics: functions, conditions, loops and
  calculations.
- Exercises 5 to 9 work on text, `Record` objects, unions of strings and
  `| undefined` returns.
- Exercises 10 to 12 use nested loops, a generic class and a discriminated union.
- When a solution surprises you, redo its trace table on paper.

Next: [cheat sheet](aide-memoire.md)
