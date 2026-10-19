# Solved exercises

This page offers 12 solved exercises, from easiest to hardest, to practice the whole TypeScript track.

## How to use this page

Each exercise has a level:

- ★☆☆: easy;
- ★★☆: medium;
- ★★★: harder.

Read the instructions, then write your solution in a `.ts` file. Check the types with
`npx tsc --noEmit --strict exercise.ts`, then run it with `npx tsx exercise.ts`.
Open the solution only after you have tried.

> **Tip:** an exercise is done when the program prints the right result **and**
> `tsc --strict` reports no error. All the solutions on this page pass this check.

## The basics

### Exercise 1: Introduce yourself

Level: ★☆☆

Create two constants, `firstName` (a string) and `age` (a number). Then print the
sentence `My name is Alice and I am 30 years old.` using your constants.

> **Hint:** a template literal between backticks: `` `Hello ${name}` `` (see
> [first steps](premiers-pas.md)).

> **Solution**
>
> ```typescript
> const firstName = "Alice";
> const age = 30;
> console.log(`My name is ${firstName} and I am ${age} years old.`);
> ```
>
> ```sortie
> My name is Alice and I am 30 years old.
> ```
>
> No annotation needed: TypeScript infers `string` and `number`.

### Exercise 2: Even or odd

Level: ★☆☆

Write a function `isEven(n: number): boolean`. Use it to print `7 is odd` and
`10 is even`.

> **Hint:** `n % 2` gives the remainder of the division by 2.

> **Solution**
>
> ```typescript
> function isEven(n: number): boolean {
>   return n % 2 === 0;
> }
>
> for (const n of [7, 10]) {
>   console.log(`${n} is ${isEven(n) ? "even" : "odd"}`);
> }
> ```
>
> ```sortie
> 7 is odd
> 10 is even
> ```

### Exercise 3: FizzBuzz

Level: ★☆☆

For the numbers from 1 to 15, print `Fizz` if the number is divisible by 3, `Buzz` if
it is divisible by 5, `FizzBuzz` if it is divisible by both, and the number otherwise.
Print everything on a single line, separated by spaces.

> **Hint:** test the "by 15" case first. Store the words in a `string[]` array, then
> `join(" ")` (see [conditions and loops](conditions-boucles.md)).

> **Solution**
>
> ```typescript
> const words: string[] = [];
> for (let i = 1; i <= 15; i++) {
>   if (i % 15 === 0) words.push("FizzBuzz");
>   else if (i % 3 === 0) words.push("Fizz");
>   else if (i % 5 === 0) words.push("Buzz");
>   else words.push(String(i));
> }
> console.log(words.join(" "));
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```
>
> The `string[]` annotation is useful here: the array is empty at the start.

### Exercise 4: The area of a rectangle

Level: ★☆☆

Write a function `area(width, height)` that returns the area of a rectangle. If the
height is not given, the rectangle is a square: the height equals the width.
Print `area(4, 3)` and `area(5)`.

> **Hint:** a parameter can have a default value that depends on an earlier parameter
> (see [functions](fonctions.md)).

> **Solution**
>
> ```typescript
> function area(width: number, height: number = width): number {
>   return width * height;
> }
>
> console.log(area(4, 3));
> console.log(area(5));
> ```
>
> ```sortie
> 12
> 25
> ```

## Text, arrays and objects

### Exercise 5: Counting vowels

Level: ★★☆

Write a function `vowels(text: string): number` that counts the vowels (a, e, i, o, u,
y) in a text, both uppercase and lowercase. Test it with `"TypeScript is great"`.

> **Hint:** `toLowerCase()`, then go through the letters with `for (const c of text)`
> and test `"aeiouy".includes(c)` (see [working with text](texte.md)).

> **Solution**
>
> ```typescript
> function vowels(text: string): number {
>   let total = 0;
>   for (const c of text.toLowerCase()) {
>     if ("aeiouy".includes(c)) total++;
>   }
>   return total;
> }
>
> console.log(vowels("TypeScript is great"));
> ```
>
> ```sortie
> 6
> ```

### Exercise 6: The average

Level: ★★☆

Write a function `average(grades: readonly number[])` that returns the average, or
`undefined` if the array is empty. Print the average of `[12, 15, 9, 14]`, then the
one of `[]`, writing `no grades` in that case.

> **Hint:** the return type is `number | undefined`. When calling, `??` gives a
> fallback value.

> **Solution**
>
> ```typescript
> function average(grades: readonly number[]): number | undefined {
>   if (grades.length === 0) return undefined;
>   return grades.reduce((a, b) => a + b, 0) / grades.length;
> }
>
> console.log(average([12, 15, 9, 14]) ?? "no grades");
> console.log(average([]) ?? "no grades");
> ```
>
> ```sortie
> 12.5
> no grades
> ```
>
> The return type forces the caller to handle the empty case: `average([]).toFixed(1)`
> would be rejected by `tsc`.

### Exercise 7: The shopping cart

Level: ★★☆

Define an interface `Item` with a `name`, a unit `price` and a `quantity`. Write
`total(cart: readonly Item[]): number`, then print the total of this cart with two
decimals: 3 croissants at €1.20, 1 baguette at €1.10.

> **Hint:** `reduce` adds up `price * quantity`; `toFixed(2)` formats (see
> [objects](objets.md)).

> **Solution**
>
> ```typescript
> interface Item {
>   name: string;
>   price: number;
>   quantity: number;
> }
>
> function total(cart: readonly Item[]): number {
>   return cart.reduce((sum, a) => sum + a.price * a.quantity, 0);
> }
>
> const cart: Item[] = [
>   { name: "croissant", price: 1.2, quantity: 3 },
>   { name: "baguette", price: 1.1, quantity: 1 },
> ];
> console.log(`Total: €${total(cart).toFixed(2)}`);
> ```
>
> ```sortie
> Total: €4.70
> ```

### Exercise 8: Shapes

Level: ★★☆

A shape is either a circle (with a `radius`) or a rectangle (with a `width` and a
`height`). Write the `Shape` type and a function `area(s: Shape): number`. Print the
area of a circle of radius 1 (two decimals) and of a 2 × 3 rectangle.

> **Hint:** a discriminated union, with a common `type` property, and a `switch`
> (see [unions](unions.md#discriminated-unions)).

> **Solution**
>
> ```typescript
> type Shape =
>   | { type: "circle"; radius: number }
>   | { type: "rectangle"; width: number; height: number };
>
> function area(s: Shape): number {
>   switch (s.type) {
>     case "circle":
>       return Math.PI * s.radius ** 2;
>     case "rectangle":
>       return s.width * s.height;
>   }
> }
>
> console.log(area({ type: "circle", radius: 1 }).toFixed(2));
> console.log(area({ type: "rectangle", width: 2, height: 3 }));
> ```
>
> ```sortie
> 3.14
> 6
> ```

## Richer types

### Exercise 9: The last element

Level: ★★☆

Write a **generic** function `last` that returns the last element of an array of any
type, or `undefined` if it is empty. The return type must follow the type of the
array: `last(["a", "b"])` is a `string | undefined`.

> **Hint:** `function last<T>(array: readonly T[]): T | undefined` (see
> [generics](generiques.md)).

> **Solution**
>
> ```typescript
> function last<T>(array: readonly T[]): T | undefined {
>   return array[array.length - 1];
> }
>
> const word = last(["apple", "pear", "kiwi"]);
> const num = last([1, 2, 3]);
> console.log(word?.toUpperCase());
> console.log(num);
> console.log(last([]));
> ```
>
> ```sortie
> KIWI
> 3
> undefined
> ```
>
> Thanks to `T`, TypeScript knows that `word` is a string: `toUpperCase` is allowed.
> The `?.` operator handles the `undefined` case.

### Exercise 10: The traffic light

Level: ★★★

Define the `Light` type: `"green"`, `"orange"` or `"red"`. Write
`next(light: Light): Light` (green → orange → red → green). Add an exhaustiveness
check: if someday a color is added to the type, `tsc` must report the omission.
Print four changes starting from `"green"`.

> **Hint:** in the `default` of the `switch`, store the value in a variable of type
> `never` (see [literal types](litteraux.md) and
> [unions](unions.md#checking-every-case-with-never)).

> **Solution**
>
> ```typescript
> type Light = "green" | "orange" | "red";
>
> function next(light: Light): Light {
>   switch (light) {
>     case "green":
>       return "orange";
>     case "orange":
>       return "red";
>     case "red":
>       return "green";
>     default: {
>       const impossible: never = light;
>       throw new Error(`Unknown light: ${impossible}`);
>     }
>   }
> }
>
> let light: Light = "green";
> const steps: Light[] = [light];
> for (let i = 0; i < 4; i++) {
>   light = next(light);
>   steps.push(light);
> }
> console.log(steps.join(" -> "));
> ```
>
> ```sortie
> green -> orange -> red -> green -> orange
> ```
>
> Add `"blinking"` to the `Light` type: the line `const impossible: never = light;`
> becomes an error, because `light` can still be `"blinking"` at that point.

### Exercise 11: The bank account

Level: ★★★

Write an `Account` class with a read-only `owner` and a **private** balance, which
starts at 0. It has three methods: `deposit(amount)`, `withdraw(amount)` and
`balance()`. A negative deposit or withdrawal, or a withdrawal larger than the balance,
throws an `Error`. Make a deposit of 100, a withdrawal of 30, then try a withdrawal of
500 while catching the error.

> **Hint:** `private` or `#balance` hides the balance; `throw new Error("…")` throws an
> error; in the `catch`, check `e instanceof Error` before reading `e.message`
> (see [errors](erreurs.md)).

> **Solution**
>
> ```typescript
> class Account {
>   #balance = 0;
>
>   constructor(readonly owner: string) {}
>
>   deposit(amount: number): void {
>     if (amount <= 0) throw new Error("The deposit must be positive");
>     this.#balance += amount;
>   }
>
>   withdraw(amount: number): void {
>     if (amount <= 0) throw new Error("The withdrawal must be positive");
>     if (amount > this.#balance) throw new Error(`Insufficient funds (€${this.#balance})`);
>     this.#balance -= amount;
>   }
>
>   balance(): number {
>     return this.#balance;
>   }
> }
>
> const account = new Account("Alice");
> account.deposit(100);
> account.withdraw(30);
> console.log(`${account.owner}: €${account.balance()}`);
>
> try {
>   account.withdraw(500);
> } catch (e) {
>   console.log(e instanceof Error ? `Refused: ${e.message}` : "Unknown error");
> }
> console.log(`${account.owner}: €${account.balance()}`);
> ```
>
> ```sortie
> Alice: €70
> Refused: Insufficient funds (€70)
> Alice: €70
> ```
>
> `#balance` is truly private, even at run time: `account.#balance` is impossible
> outside the class. The balance only changes through the methods, which check
> everything.

### Exercise 12: Loading profiles

Level: ★★★

Here is an asynchronous function that simulates a slow server:

```typescript norun
const PROFILES: Record<number, string> = { 1: "Alice", 2: "Bob" };

function loadName(id: number): Promise<string> {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      const name = PROFILES[id];
      if (name === undefined) reject(new Error(`Profile ${id} not found`));
      else resolve(name);
    }, 10);
  });
}
```

Write a function `async function loadAll(ids: number[])` that loads all the profiles
**at the same time**, and returns a result for each one:
`{ ok: true; name: string }` or `{ ok: false; error: string }`. Print the results for
the numbers 1, 3 and 2, in that order.

> **Hint:** `Promise.allSettled` waits for all the promises, fulfilled or not, and
> keeps the order. Each response has a `status`: `"fulfilled"` (with `value`) or
> `"rejected"` (with `reason`) (see [asynchronous code](asynchrone.md)).

> **Solution**
>
> ```typescript
> type Result = { ok: true; name: string } | { ok: false; error: string };
>
> const PROFILES: Record<number, string> = { 1: "Alice", 2: "Bob" };
>
> function loadName(id: number): Promise<string> {
>   return new Promise((resolve, reject) => {
>     setTimeout(() => {
>       const name = PROFILES[id];
>       if (name === undefined) reject(new Error(`Profile ${id} not found`));
>       else resolve(name);
>     }, 10);
>   });
> }
>
> async function loadAll(ids: number[]): Promise<Result[]> {
>   const responses = await Promise.allSettled(ids.map(loadName));
>   return responses.map((r): Result => {
>     if (r.status === "fulfilled") return { ok: true, name: r.value };
>     const message = r.reason instanceof Error ? r.reason.message : String(r.reason);
>     return { ok: false, error: message };
>   });
> }
>
> for (const r of await loadAll([1, 3, 2])) {
>   console.log(r.ok ? `OK: ${r.name}` : `Failed: ${r.error}`);
> }
> ```
>
> ```sortie
> OK: Alice
> Failed: Profile 3 not found
> OK: Bob
> ```
>
> With `Promise.all`, the first failure would have stopped everything. `allSettled`
> lets each load run to the end. The annotated return type `(r): Result` helps
> TypeScript recognize the discriminated union.

## Summary

- Start with the ★☆☆ exercises, then move up in difficulty.
- Always try on your own, and check your types with `tsc --strict` before opening the
  solution.
- Exercises 1 to 4 cover the basics: constants, conditions, loops, functions.
- Exercises 5 to 8 work on text, arrays, interfaces and unions.
- Exercises 9 to 12 use generics, literals, classes, errors and asynchronous code.

Next: [solved exercises: second set](exercices-2.md)
