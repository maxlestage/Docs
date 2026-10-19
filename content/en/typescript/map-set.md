# Map, Set and Record

This page explains how to link keys to values with `Map` and `Record`, and how to keep values without duplicates with `Set`.

> **Goals:**
>
> - you will know how to link keys to values with `Map<K, V>`;
> - you will know how to handle the `undefined` returned by `get`;
> - you will know how to remove duplicates with `Set<T>`;
> - you will know how to describe a key-value object with `Record` and choose between
>   `Record` and `Map`.

This chapter follows [arrays and tuples](collections.md): you will find `for...of`,
`[a, b]` destructuring and the `??` reflex you saw with `find` again here.

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

Line by line:

- `new Map<string, number>()` creates an empty `Map`: string keys, number values;
- the `set` calls store `apples → 10` and `pears → 4`, then replace 10 with 12;
- `get("apples")` gives 12; the key `"kiwis"` doesn't exist, and the `Map` has 2
  pairs;
- the `for...of` loop gives each pair, in the order they were added.

The methods to know:

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

**Your turn:** write a program that creates a `Map` of ages, stores `"Alice"` (30)
and `"Bob"` (25) in it, then shows Bob's age and the age of `"Zoe"`, with 0 if she
is missing.

> **Solution**
>
> ```typescript
> const ages = new Map<string, number>();
> ages.set("Alice", 30);
> ages.set("Bob", 25);
> console.log(ages.get("Bob"));
> console.log(ages.get("Zoe") ?? 0);
> ```
>
> ```sortie
> 25
> 0
> ```

## Set: a collection without duplicates

Think of a party's guest list: writing the same name twice is pointless. A `Set`
takes care of it for you and ignores duplicates.

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

Line by line:

- `visitors` starts empty;
- `add` adds `"Alice"`, then `"Bob"`. The second `"Alice"` is ignored: she is already
  there;
- so `size` is 2, and `has("Bob")` answers `true`;
- the end creates a `Set` from the array, then turns it back into an array with `[...]`.

`new Set(array)` then `[...]` is the classic trick to remove duplicates from an array,
keeping the order in which they first appear.

## Record: a key-value object

A `Record` is like a score sheet: a name, a number, a name, a number. You write it in
one go, like an ordinary object.

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

Line by line:

- `scores` starts with two pairs: `alice → 12` and `bob → 8`;
- `scores.chloe = 15` adds a key; `scores["bob"] += 1` brings Bob up to 9;
- `Object.keys` lists the keys, in the order they were added;
- the loop gives each pair; `padEnd(6)` pads the name with spaces, to line up the
  columns.

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

### Reading a Map with square brackets

```typescript norun
const stock = new Map<string, number>([["apples", 10]]);
console.log(stock["apples"]);
```

```text
main.ts:2:13 - error TS7052: Element implicitly has an 'any' type because type 'Map<string, number>' has no index signature. Did you mean to call 'stock.get'?

2 console.log(stock["apples"]);
              ~~~~~~~~~~~~~~~


Found 1 error in main.ts:2
```

A `Map` is not an object: you don't read its values with square brackets. The
message suggests it: write `stock.get("apples")`.

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

### Forgetting a key in a Record

```typescript norun
type Day = "monday" | "tuesday" | "wednesday";
const hours: Record<Day, number> = { monday: 7, tuesday: 8 };
```

```text
main.ts:2:7 - error TS2741: Property 'wednesday' is missing in type '{ monday: number; tuesday: number; }' but required in type 'Record<Day, number>'.

2 const hours: Record<Day, number> = { monday: 7, tuesday: 8 };
        ~~~~~


Found 1 error in main.ts:2
```

With a union as the key type, every day needs its value. Add the missing key:
`{ monday: 7, tuesday: 8, wednesday: 4 }`.

## Test yourself

**Question 1:** what is the value of `new Set(["a", "b", "a"]).size`?

> **Answer**
>
> `2`. A `Set` keeps each value only once: the second `"a"` is ignored.

**Question 2:** `ages` is a `Map<string, number>`. What is the type of
`ages.get("Bob")`, and why?

> **Answer**
>
> `number | undefined`. The key `"Bob"` may not exist: in that case, `get` returns
> `undefined`. Handle it with `if`, `?.` or `??`.

**Question 3:** complete the code to print the number of different names.

```typescript norun
const signups = ["Lea", "Tom", "Lea", "Ines", "Tom"];
```

> **Answer**
>
> ```typescript
> const signups = ["Lea", "Tom", "Lea", "Ines", "Tom"];
> console.log(new Set(signups).size);
> ```
>
> ```sortie
> 3
> ```
>
> The `Set` removes the duplicates; `size` counts what is left.

## Summary

- `Map<K, V>` links keys to values: `set`, `get`, `has`, `delete` and `size`.
- `get` returns `V | undefined`: handle the case with `if`, `?.` or `??`.
- `Set<T>` keeps each value only once; `[...new Set(array)]` removes the duplicates
  from an array.
- `Record<string, number>` describes a key-value object; with a union of keys, it
  requires all of them.
- `Record` for fixed data written in the code, `Map` when keys change often.

Next: [objects, interfaces and type aliases](objets.md).
