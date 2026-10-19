# Arrays and tuples

This page explains how to store several values in arrays, tuples, `Map`s and `Set`s, and how types keep these collections consistent.

> **Goals:**
>
> - you will know how to create a typed array (`number[]`), read it and change it;
> - you will know how to protect an array with `readonly` and describe a tuple
>   `[string, number]`;
> - you will know how to transform an array with `map`, `filter` and `reduce`;
> - you will know how to handle the `undefined` returned by `find`;
> - you will know how to use `Map`, `Set` and `Record`.

## Creating an array

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

- `fruits[1] = ...` replaces an item;
- `push` adds to the end, `pop` removes and returns the last one;
- `includes` answers `true` or `false`, `indexOf` gives the position (`-1` if missing).

Even with `const`, the array can change: `const` only prevents you from replacing the
**whole** array with another one.

> **Warning:** reading an index that doesn't exist (`fruits[10]`) gives `undefined`,
> but TypeScript still announces a `string`. Check the length first, or prefer
> `at()`, whose type `string | undefined` reminds you of the risk.

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

## Transforming: map, filter, reduce

These three methods take a *callback* (a function, see
[functions](fonctions.md#callbacks-passing-a-function-to-a-function)). They don't
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

### Chaining

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

## Map: linking keys to values

A `Map` is a [dictionary](../debuter/glossaire.md#dictionary): it links a **key**
to a **value**, the way a phone book links a name to a number. Its type is written
`Map<KeyType, ValueType>`.

```typescript
const stock = new Map<string, number>();

stock.set("apples", 10);
stock.set("pears", 4);
stock.set("apples", 12);

console.log(stock.get("apples"));
console.log(stock.has("kiwis"), stock.size);

for (const [fruit, quantity] of stock) {
  console.log(`${fruit}: ${quantity}`);
}
```

```sortie
12
false 2
apples: 12
pears: 4
```

- `set(key, value)` adds or replaces (a key is unique);
- `get(key)` reads the value, `has(key)` tests whether the key exists;
- `size` gives the number of pairs; `delete(key)` removes one.

`get` returns `number | undefined`: the key may not exist. Same reflex as with
`find`:

```typescript
const stock = new Map<string, number>([["apples", 10]]);

const kiwis = stock.get("kiwis") ?? 0;
console.log(kiwis + 5);
```

```sortie
5
```

## Set: a collection without duplicates

A `Set` is a [set](../debuter/glossaire.md#set): each value appears in it only
once. Its type is written `Set<T>`.

```typescript
const visitors = new Set<string>();

visitors.add("Alice");
visitors.add("Bob");
visitors.add("Alice");

console.log(visitors.size);
console.log(visitors.has("Bob"));

const noDuplicates = [...new Set([3, 1, 3, 2, 1])];
console.log(noDuplicates);
```

```sortie
2
true
[ 3, 1, 2 ]
```

`new Set(array)` then `[...]` is the classic trick to remove duplicates from an array,
keeping the order in which they first appear.

## Record: a key-value object

`Record<string, number>` describes a plain object whose keys are strings and whose
values are numbers. It's lighter than a `Map`, and you write it directly with curly
braces.

```typescript
const scores: Record<string, number> = {
  alice: 12,
  bob: 8,
};

scores.chloe = 15;
scores["bob"] += 1;

console.log(scores);
console.log(Object.keys(scores));

for (const [name, points] of Object.entries(scores)) {
  console.log(name.padEnd(6), points);
}
```

```sortie
{ alice: 12, bob: 9, chloe: 15 }
[ 'alice', 'bob', 'chloe' ]
alice  12
bob    9
chloe  15
```

`Object.keys` gives the keys, `Object.entries` the `[key, value]` pairs.

With a union of literal types as keys, `Record` requires **all** the keys:

```typescript
type Day = "monday" | "tuesday" | "wednesday";

const hours: Record<Day, number> = {
  monday: 7,
  tuesday: 8,
  wednesday: 4,
};

console.log(hours.tuesday);
```

```sortie
8
```

Forgetting `wednesday` would be an error.

> **Tip:** use `Record` for fixed data written in the code, and `Map` when you often
> add and remove keys.

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

### Using the result of get without checking

```typescript norun
const stock = new Map<string, number>([["apples", 10]]);
const n = stock.get("pears");
console.log(n + 1);
```

```text
main.ts:3:13 - error TS18048: 'n' is possibly 'undefined'.

3 console.log(n + 1);
              ~


Found 1 error in main.ts:3
```

The key `"pears"` may not exist. Give a default value:
`const n = stock.get("pears") ?? 0;`.

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
- `find` and `Map.get` return `T | undefined`: handle the case with `if`, `?.` or
  `??`.
- `Map<K, V>` links keys to values, `Set<T>` avoids duplicates,
  `Record<string, number>` describes a key-value object.

Next: [objects, interfaces and type aliases](objets.md).
