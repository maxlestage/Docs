# Arrays and tuples

This page explains how to store several values in arrays and tuples, how to transform them, and how types keep these lists consistent.

> **Goals:**
>
> - you will know how to create a typed array (`number[]`), read it and change it;
> - you will know how to protect an array with `readonly` and describe a tuple
>   `[string, number]`;
> - you will know how to transform an array with `map`, `filter` and `reduce`;
> - you will know how to handle the `undefined` returned by `find`.

## Creating an array

Picture a shopping list: one sheet of paper, several items, in a precise order. An
array plays that role in a program: it stores several values under a single name.

An array is an ordered [list](../debuter/glossaire.md#list) of values, written
between square brackets. Its type is written as the type of the items followed by `[]`:

```typescript
const grades: number[] = [12, 15, 9];
const firstNames: string[] = ["Alice", "Bob", "Chloé"];
const empty: number[] = [];

console.log(grades);
console.log(firstNames.length, empty.length);
```

```sortie
[ 12, 15, 9 ]
3 0
```

Line by line:

- `grades` receives three numbers; `number[]` announces "numbers only";
- `firstNames` receives three strings, and `empty` an array with no items at all;
- `console.log(grades)` shows the whole array, between square brackets;
- `.length` gives the number of items: 3 for `firstNames`, 0 for `empty`.

`number[]` reads "array of numbers". Another way of writing it means exactly the same
thing: `Array<number>`. The angle brackets `< >` give the type of the items; you will
see them again in [generics](generiques.md).

```typescript
const cities: Array<string> = ["Lyon", "Nantes"];
console.log(cities);
```

```sortie
[ 'Lyon', 'Nantes' ]
```

With starting values, TypeScript guesses the type: `const grades = [12, 15, 9]` also
gives a `number[]`. Write the type for an empty array: without it, TypeScript can't
know what you are going to put in it.

> **Remember:** all the items of an array have the same type. An array of numbers
> refuses a string: that's the guarantee that every item can be handled the same
> way.

## Reading and changing

A list is alive: you look at an item, change one, add one at the bottom, cross out
the last one. An array lets you do all of that.

Each item has an [index](../debuter/glossaire.md#index) that starts at **0**.

```typescript
const fruits: string[] = ["apple", "pear", "kiwi"];

console.log(fruits[0]);
console.log(fruits.at(-1));
fruits[1] = "mango";
fruits.push("strawberry");
console.log(fruits);

const last = fruits.pop();
console.log(last, fruits);
console.log(fruits.includes("kiwi"), fruits.indexOf("kiwi"));
```

```sortie
apple
kiwi
[ 'apple', 'mango', 'kiwi', 'strawberry' ]
strawberry [ 'apple', 'mango', 'kiwi' ]
true 2
```

Line by line:

- `fruits[0]` reads the first slot: `"apple"`; `at(-1)` counts from the end: `"kiwi"`;
- `fruits[1] = "mango"` replaces `"pear"`, then `push` adds `"strawberry"`: 4 items;
- `pop()` removes `"strawberry"` and stores it in `last`: 3 items remain;
- `"kiwi"` is indeed there, at position 2 (the third slot, since counting starts at 0).

The methods to know:

- `fruits[1] = ...` replaces an item;
- `push` adds to the end, `pop` removes and returns the last one;
- `includes` answers `true` or `false`, `indexOf` gives the position (`-1` if missing).

Even with `const`, the array can change: `const` only prevents you from replacing the
**whole** array with another one.

> **Warning:** reading an index that doesn't exist (`fruits[10]`) gives `undefined`,
> but TypeScript still announces a `string`. Check the length first, or prefer
> `at()`, whose type `string | undefined` reminds you of the risk.

**Your turn:** write a program that creates an array `groceries` with `"bread"` and
`"milk"`, adds `"eggs"` to the end, then shows the number of items and the array.

> **Solution**
>
> ```typescript
> const groceries: string[] = ["bread", "milk"];
> groceries.push("eggs");
> console.log(groceries.length);
> console.log(groceries);
> ```
>
> ```sortie
> 3
> [ 'bread', 'milk', 'eggs' ]
> ```

### Looping over an array

`for...of` gives each item. With `entries()`, you also get the index.

```typescript
const podium = ["Alice", "Bob", "Chloé"];

for (const [i, name] of podium.entries()) {
  console.log(`${i + 1}. ${name}`);
}
```

```sortie
1. Alice
2. Bob
3. Chloé
```

`const [i, name]` splits each pair into two variables: this is **destructuring**,
explained with tuples below.

## Read-only arrays

Some lists should never change: the days of the week, the months of the year.
`readonly` protects you from changing them by mistake.

Add `readonly` in front of the type: the array can no longer be changed. `push`,
`pop` or `fruits[0] = ...` are refused.

```typescript
const days: readonly string[] = ["Monday", "Tuesday", "Wednesday"];

console.log(days[0], days.length);
console.log(days.includes("Tuesday"));
```

```sortie
Monday 3
true
```

Only the methods that **read** remain available. It's useful for a fixed list, or to
promise that a function won't touch the array you give it:

```typescript
function average(values: readonly number[]): number {
  let total = 0;
  for (const v of values) {
    total += v;
  }
  return total / values.length;
}

console.log(average([12, 15, 9]));
```

```sortie
12
```

## Tuples

Sometimes two or three values of different types belong together, like a row in a
scoreboard: a name and some points. A tuple keeps them side by side, in a fixed order.

A [tuple](../debuter/glossaire.md#tuple) is an array of **fixed length**, where each
position has its own type. `[string, number]` means: "exactly two items, a string then
a number".

```typescript
const player: [string, number] = ["Alice", 1200];

console.log(player[0].toUpperCase());
console.log(player[1] + 50);

const [name, score] = player;
console.log(`${name} has ${score} points.`);
```

```sortie
ALICE
1250
Alice has 1200 points.
```

Line by line:

- `player` holds two slots: `"Alice"` (a string) then `1200` (a number);
- `player[0].toUpperCase()` puts the string in capitals: `ALICE`;
- `player[1] + 50` computes with the number: `1250`. `player` doesn't change;
- the end stores `"Alice"` in `name` and `1200` in `score`, then shows the sentence.

TypeScript knows that `player[0]` is a `string` and `player[1]` a `number`. The line
`const [name, score] = player;` stores the first item in `name` and the second in
`score`.

A tuple is handy for returning two values from a function:

```typescript
function minMax(values: number[]): [number, number] {
  return [Math.min(...values), Math.max(...values)];
}

const [smallest, largest] = minMax([4, 9, 1, 7]);
console.log(smallest, largest);
```

```sortie
1 9
```

`...values` (the **spread** syntax) spreads the array out into separate arguments:
`Math.min(4, 9, 1, 7)`.

> **Tip:** beyond two or three items, a tuple becomes hard to read (what does
> `player[3]` mean?). Then prefer an object with property names: see
> [objects](objets.md).

**Your turn:** write a program that stores a product in a tuple `[string, number]`:
`"pen"` and `2`. Split it into two variables, then show "pen costs 2 €".

> **Solution**
>
> ```typescript
> const product: [string, number] = ["pen", 2];
> const [item, price] = product;
> console.log(`${item} costs ${price} €`);
> ```
>
> ```sortie
> pen costs 2 €
> ```

## Transforming: map, filter, reduce

Very often, you want to do the same thing to a whole list: raise every price, keep the
good grades, compute a total. These methods do it in one line, without writing a loop.

These three methods take a *callback* (a function, see
[functions](fonctions-avancees.md#callbacks-passing-a-function-to-a-function)). They don't
change the original array: they return a new one, or a value.

### map: transforming each item

```typescript
const prices = [10, 20, 35];
const pricesWithTax = prices.map((p) => p * 1.2);
const labels = prices.map((p) => `${p} €`);

console.log(pricesWithTax);
console.log(labels);
```

```sortie
[ 12, 24, 42 ]
[ '10 €', '20 €', '35 €' ]
```

Line by line:

- `prices` holds three numbers;
- `prices.map((p) => p * 1.2)` gives each price to `p`, one after the other, and
  stores each result in a new array: `[12, 24, 42]`;
- the next line does the same, but the callback builds a string: `"10 €"`…;
- `prices` itself hasn't changed: it is still `[10, 20, 35]`.

TypeScript guesses everything: `p` is a `number` (an item of `prices`),
`pricesWithTax` is a `number[]`, and `labels` a `string[]`, because the callback
returns a string. Hover over these names in the editor to see it.

### filter: keeping some items

```typescript
const grades = [12, 7, 15, 9, 18];
const passed = grades.filter((n) => n >= 10);

console.log(passed);
console.log(grades);
```

```sortie
[ 12, 15, 18 ]
[ 12, 7, 15, 9, 18 ]
```

The callback answers `true` (keep) or `false` (throw away). `grades` hasn't changed.

### reduce: summing everything up into one value

`reduce` goes through the array while keeping an **accumulator**: a value that grows
at each round, like a piggy bank. The second argument is its starting value.

```typescript
const purchases = [4.5, 12, 3.5];
const total = purchases.reduce((sum, price) => sum + price, 0);
console.log(total);

const words = ["lion", "elephant", "bear"];
const letters = words.reduce((acc, word) => acc + word.length, 0);
console.log(letters);
```

```sortie
20
16
```

In the first round, `sum` is `0` (the starting value); then `4.5`, `16.5`, and
finally `20`. The type of the accumulator comes from the starting value: `0` gives a
`number`.

**Your turn:** starting from `const numbers = [2, 5, 8];`, write a program that
creates the array of doubles with `map`, then computes the sum of `numbers` with
`reduce`. Show both results.

> **Solution**
>
> ```typescript
> const numbers = [2, 5, 8];
> const doubles = numbers.map((n) => n * 2);
> const sum = numbers.reduce((acc, n) => acc + n, 0);
> console.log(doubles);
> console.log(sum);
> ```
>
> ```sortie
> [ 4, 10, 16 ]
> 15
> ```

### Chaining

> **Note:** this part can wait. Read it once the basics feel solid.

Since each method returns an array, you can chain them:

```typescript
const grades = [12, 7, 15, 9, 18];

const totalWithBonus = grades
  .filter((n) => n >= 10)
  .map((n) => n + 1)
  .reduce((acc, n) => acc + n, 0);

console.log(totalWithBonus);
```

```sortie
48
```

Read from top to bottom: you keep the passing grades (`12, 15, 18`), add one bonus
point to each (`13, 16, 19`), then compute the sum.

### Sorting

`sort` sorts the array **in place**. Careful: without a callback, it sorts as text,
even numbers! Always give a comparison function `(a, b) => a - b`.

```typescript
const numbers = [10, 9, 1, 100];

console.log([...numbers].sort());
console.log([...numbers].sort((a, b) => a - b));
console.log([...numbers].sort((a, b) => b - a));
```

```sortie
[ 1, 10, 100, 9 ]
[ 1, 9, 10, 100 ]
[ 100, 10, 9, 1 ]
```

`[...numbers]` creates a copy, so that `numbers` itself isn't changed.

## find: a result that may be missing

Searching a list is like looking for a name in an address book: sometimes it isn't
there. TypeScript makes you plan for that case.

`find` returns the **first** item that passes the test. If it finds none, it returns
`undefined`. Its type says so plainly: for a `string[]`, it is `string | undefined`.

```typescript
const firstNames = ["Alice", "Bob", "Chloé"];

const withC = firstNames.find((p) => p.startsWith("C"));
const withZ = firstNames.find((p) => p.startsWith("Z"));

console.log(withC, withZ);
```

```sortie
Chloé undefined
```

Line by line:

- `find` tries each name with the test `p.startsWith("C")` ("does it start with C?");
- `"Alice"` and `"Bob"` fail, `"Chloé"` passes: `withC` is `"Chloé"`;
- no name starts with Z: `withZ` is `undefined`.

This is where TypeScript protects you: you can't use the result as a `string`
without handling the `undefined` case. Three ways to do it:

```typescript
const firstNames = ["Alice", "Bob", "Chloé"];
const found = firstNames.find((p) => p.startsWith("B"));

// 1. Test before using
if (found !== undefined) {
  console.log(found.toUpperCase());
} else {
  console.log("Nobody.");
}

// 2. Call only if the value exists
console.log(found?.toLowerCase());

// 3. Give a default value
const name = found ?? "unknown";
console.log(name.length);
```

```sortie
BOB
bob
3
```

After the test `found !== undefined`, TypeScript knows that `found` is a `string`.

> **Remember:** a `T | undefined` type is a question TypeScript asks you: "what if
> nothing is found?". Answer it, with `if`, `?.` or `??`.

## Common mistakes

### Storing a value of the wrong type

```typescript norun
const grades: number[] = [12, 15];
grades.push("18");
```

```text
main.ts:2:13 - error TS2345: Argument of type 'string' is not assignable to parameter of type 'number'.

2 grades.push("18");
              ~~~~


Found 1 error in main.ts:2
```

`"18"` is a string. A `number[]` only accepts numbers: `grades.push(18)`.

### Changing a readonly array

```typescript norun
const days: readonly string[] = ["Monday", "Tuesday"];
days.push("Wednesday");
```

```text
main.ts:2:6 - error TS2339: Property 'push' does not exist on type 'readonly string[]'.

2 days.push("Wednesday");
       ~~~~


Found 1 error in main.ts:2
```

A `readonly` array has no `push` method. Create a new array instead:
`const week = [...days, "Wednesday"];`.

### Reversing the order of a tuple

```typescript norun
const player: [string, number] = [42, "Alice"];
```

```text
main.ts:1:35 - error TS2322: Type 'number' is not assignable to type 'string'.

1 const player: [string, number] = [42, "Alice"];
                                    ~~

main.ts:1:39 - error TS2322: Type 'string' is not assignable to type 'number'.

1 const player: [string, number] = [42, "Alice"];
                                        ~~~~~~~


Found 2 errors in the same file, starting at: main.ts:1
```

In a tuple, order matters: first the string, then the number.
Write `["Alice", 42]`.

## Test yourself

**Question 1:** what is the type of `["a", "b"].map((s) => s.length)`?

> **Answer**
>
> `number[]`. The callback returns `s.length`, a number, for each item.

**Question 2:** what is the difference between `string[]` and `[string, string]`?

> **Answer**
>
> `string[]` is an array of strings of any length. `[string, string]` is a tuple:
> exactly two strings.

**Question 3:** complete the code to print the first even number, or `none`.

```typescript norun
const numbers = [3, 7, 8, 5];
const even = numbers.find((n) => n % 2 === 0);
```

> **Answer**
>
> ```typescript
> const numbers = [3, 7, 8, 5];
> const even = numbers.find((n) => n % 2 === 0);
> console.log(even ?? "none");
> ```
>
> ```sortie
> 8
> ```
>
> `even` has the type `number | undefined`: `??` handles the case where nothing is
> found.

## Summary

- An array is written `number[]` or `Array<number>`; all its items have the same
  type. `readonly number[]` prevents it from changing.
- A tuple `[string, number]` has a fixed length and one type per position.
- `map`, `filter` and `reduce` return new values, whose type TypeScript guesses; sort
  numbers with `sort((a, b) => a - b)`.
- `find` returns `T | undefined`: handle the case with `if`, `?.` or `??`.

Next: [Map, Set and Record](map-set.md).
