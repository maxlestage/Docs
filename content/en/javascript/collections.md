# Arrays

This page explains how to store several values in an array, add them, remove them, search for them, transform them with `map`, `filter` and `reduce`, and sort them without falling into the traps.

> **Goals:**
>
> - you will be able to create an array, read an element and get its length;
> - you will be able to add and remove elements at the start or at the end;
> - you will be able to search an array with `includes`, `indexOf` and `find`;
> - you will be able to transform an array with `map`, `filter` and `reduce`;
> - you will be able to sort numbers correctly and copy an array.

## Creating an array

Imagine a shopping list of twenty items. Creating twenty variables would be long and
tedious. An array stores all these values under a single name.

An array is a [list](../debuter/glossaire.md#list) of values stored in order, like the
cars of a train. You write it between square brackets `[ ]`, with commas between the
elements.

```javascript
const fruits = ["apple", "pear", "kiwi"];
const grades = [12, 15, 9];
const empty = [];
const mixed = ["Alice", 30, true];

console.log(fruits);
console.log(grades.length);
console.log(empty.length);
console.log(mixed);
```

```sortie
[ 'apple', 'pear', 'kiwi' ]
3
0
[ 'Alice', 30, true ]
```

Line by line:

- `fruits` holds three texts, `grades` three numbers, and `empty` no element at all;
- `console.log(fruits)` displays the whole array, between square brackets;
- `grades.length` is 3: there are three grades. `empty.length` is 0;
- `mixed` shows that one array can hold a text, a number and a boolean.

`length` gives the number of elements. An array can mix types, but in practice you store
values of the same kind in it.

## Reading and changing an element

Once the list is stored, you need to get one specific element out of it, like reading
the third line of your shopping list, or to fix one.

Each element has an [index](../debuter/glossaire.md#index): its position, **starting
from 0**. The first car is car 0.

```javascript
const fruits = ["apple", "pear", "kiwi"];

console.log(fruits[0]);
console.log(fruits[2]);
console.log(fruits.at(-1));
console.log(fruits[5]);

fruits[1] = "mango";
console.log(fruits);
```

```sortie
apple
kiwi
kiwi
undefined
[ 'apple', 'mango', 'kiwi' ]
```

Line by line:

- `fruits[0]` reads the first element, `"apple"`; `fruits[2]` reads the third, `"kiwi"`;
- `fruits.at(-1)` counts from the end: it gives the last one, `"kiwi"` again;
- `fruits[5]` does not exist: JavaScript answers `undefined`;
- `fruits[1] = "mango"` replaces `"pear"`, which was at index 1.

- `at(-1)` gives the last element, just like with [text](texte.md).
- An index that does not exist gives `undefined`, without an error.
- `fruits` is a `const`, and yet you can change its contents. `const` forbids replacing
  **the whole array** (`fruits = [...]`), not changing what it contains.

## Adding and removing

A list keeps changing. Think of a queue: people join at the back and leave from the
front. These methods do exactly that.

Four methods add or remove an element at one end of the array. They **change** the
array.

```javascript
const queue = ["Alice", "Bob"];

queue.push("Chloe");
console.log(queue);

const last = queue.pop();
console.log(last, queue);

queue.unshift("Zoe");
console.log(queue);

const first = queue.shift();
console.log(first, queue);
```

```sortie
[ 'Alice', 'Bob', 'Chloe' ]
Chloe [ 'Alice', 'Bob' ]
[ 'Zoe', 'Alice', 'Bob' ]
Zoe [ 'Alice', 'Bob' ]
```

Line by line:

- `push("Chloe")` adds Chloe at the end: the queue holds three names;
- `pop()` removes Chloe, the last one, and stores her in `last`;
- `unshift("Zoe")` puts Zoe at the start, in front of Alice;
- `shift()` removes Zoe from the start and stores her in `first`.

| Method | End | Action |
| --- | --- | --- |
| `push(x)` | end | adds `x` |
| `pop()` | end | removes and returns the last one |
| `unshift(x)` | start | adds `x` |
| `shift()` | start | removes and returns the first one |

**Your turn:** write a program that creates an array `groceries` holding `"bread"` and
`"milk"`, adds `"eggs"` at the end, then displays the array and its length.

> **Solution**
>
> ```javascript
> const groceries = ["bread", "milk"];
> groceries.push("eggs");
> console.log(groceries);
> console.log(groceries.length);
> ```
>
> ```sortie
> [ 'bread', 'milk', 'eggs' ]
> 3
> ```

## slice and splice

Sometimes you only want part of the list, for example the top three of a ranking. Or
you want to remove an element in the middle, not at one end.

These two names look alike, but they do very different things.

- `slice(start, end)` **copies** a piece, without touching the array (as with text).
- `splice(start, count, ...additions)` **cuts into** the array: it removes elements,
  can insert some, and changes the array.

```javascript
const a = [1, 2, 3, 4, 5];
console.log(a.slice(1, 3));
console.log(a);

const b = [1, 2, 3, 4, 5];
const removed = b.splice(1, 2);
console.log(removed);
console.log(b);

b.splice(1, 0, "x", "y");
console.log(b);
```

```sortie
[ 2, 3 ]
[ 1, 2, 3, 4, 5 ]
[ 2, 3 ]
[ 1, 4, 5 ]
[ 1, 'x', 'y', 4, 5 ]
```

Line by line:

- `a.slice(1, 3)` copies the elements at indexes 1 and 2 (the end, 3, is left out);
- `console.log(a)` proves it: `a` has not changed;
- `b.splice(1, 2)` removes 2 and 3 from `b`, and returns them in `removed`;
- the last `splice` inserts two elements without removing anything.

`b.splice(1, 2)` starts at index 1 and removes 2 elements. `b.splice(1, 0, "x", "y")`
removes none and inserts `"x"` and `"y"` at index 1.

> **Tip:** to remember, think of splicing a film: you cut into it and join the pieces,
> so `splice` changes the array. `slice` is just a slice, copied.

## Searching an array

Before adding an item to your shopping list, you check whether it is already there.
These methods ask that question for you, even in a very long list.

```javascript
const fruits = ["apple", "pear", "kiwi"];

console.log(fruits.includes("kiwi"));
console.log(fruits.indexOf("pear"));
console.log(fruits.indexOf("banana"));
```

```sortie
true
1
-1
```

`includes` answers yes or no; `indexOf` gives the position, or `-1` if the element is
missing.

### find and findIndex

To search by a **rule**, `find` takes a function that answers true or false. It returns
the first element that fits, or `undefined`.

```javascript
const grades = [8, 14, 11, 17];

console.log(grades.find((n) => n > 10));
console.log(grades.findIndex((n) => n > 10));
console.log(grades.find((n) => n > 20));
```

```sortie
14
1
undefined
```

The function given to `find` is a *callback* (see [functions](fonctions.md#callbacks)).
`find` calls it on each element, until it answers `true`.

**Your turn:** starting from `const ages = [12, 17, 21, 15]`, display whether 21 is in
the array, then the first age greater than or equal to 18.

> **Solution**
>
> ```javascript
> const ages = [12, 17, 21, 15];
> console.log(ages.includes(21));
> console.log(ages.find((age) => age >= 18));
> ```
>
> ```sortie
> true
> 21
> ```

## Looping and transforming

Very often, you want to do the same thing with each element: display each item, add
tax to each price, keep only the good grades. These methods do it for you, without
writing the same code for each element.

### forEach: doing something for each element

```javascript
const fruits = ["apple", "pear"];

fruits.forEach((fruit, i) => {
  console.log(`${i + 1}. ${fruit}`);
});
```

```sortie
1. apple
2. pear
```

Line by line:

- `forEach` calls the function once per element, so twice here;
- on the first round, `fruit` is `"apple"` and `i` is 0; on the second, `"pear"` and 1;
- `${i + 1}` adds 1 to the index, to number from 1.

The callback receives the element, then its index. A `for...of` loop does the same job;
pick the one you find easier to read.

### map: transforming each element

`map` creates a **new array**, of the same length, where each element is transformed.

```javascript
const prices = [10, 20, 30];
const pricesWithTax = prices.map((p) => p * 1.2);
const names = ["alice", "bob"].map((name) => name.toUpperCase());

console.log(pricesWithTax);
console.log(names);
console.log(prices);
```

```sortie
[ 12, 24, 36 ]
[ 'ALICE', 'BOB' ]
[ 10, 20, 30 ]
```

The original array has not changed.

### filter: keeping some elements

`filter` creates a new array with only the elements for which the callback answers
`true`.

```javascript
const grades = [8, 14, 11, 17, 6];
const passed = grades.filter((n) => n >= 10);

console.log(passed);
console.log(grades.filter((n) => n > 20));
```

```sortie
[ 14, 11, 17 ]
[]
```

### reduce: summing everything up into one value

`reduce` goes through the array while keeping an **accumulator**: a value that grows at
each step. It is like a piggy bank: for each element, you put something in it.

```javascript
const grades = [12, 15, 9];

const total = grades.reduce((sum, n) => sum + n, 0);
console.log(total);
console.log(total / grades.length);
```

```sortie
36
12
```

- `0` is the starting value of the accumulator `sum`.
- Step 1: `0 + 12 = 12`; step 2: `12 + 15 = 27`; step 3: `27 + 9 = 36`.

### Chaining methods

`map` and `filter` return arrays: you can chain them.

```javascript
const cart = [4.5, 12, 2, 30];

const total = cart
  .filter((price) => price > 3)
  .map((price) => price * 0.9)
  .reduce((sum, price) => sum + price, 0);

console.log(total.toFixed(2));
```

```sortie
41.85
```

Read from top to bottom: keep the prices above €3, apply a 10% discount, then add them
up.

**Your turn:** starting from `const prices = [5, 12, 8]`, use `map` to create an array
of the doubled prices and display it. Then display its sum, computed with `reduce`.

> **Solution**
>
> ```javascript
> const prices = [5, 12, 8];
> const doubled = prices.map((p) => p * 2);
> console.log(doubled);
> console.log(doubled.reduce((sum, p) => sum + p, 0));
> ```
>
> ```sortie
> [ 10, 24, 16 ]
> 50
> ```

## Sorting

Putting things in order is useful everywhere: grades from lowest to highest, a ranking,
a list of names in alphabetical order.

`sort` sorts the array **in place** (it changes it). With simple texts, it works as
expected. With numbers, there is a trap.

```javascript
const numbers = [10, 1, 5, 100, 25];
numbers.sort();
console.log(numbers);
```

```sortie
[ 1, 10, 100, 25, 5 ]
```

By default, `sort` turns each element into **text** and puts them in dictionary order:
`"10"` comes before `"5"` because `"1"` comes before `"5"`, just as "ab" comes before
"b".

Give it a comparison function `(a, b) => ...`, which returns a number: negative if `a`
must come before `b`, positive if after.

```javascript
const numbers = [10, 1, 5, 100, 25];

numbers.sort((a, b) => a - b);
console.log(numbers);

numbers.sort((a, b) => b - a);
console.log(numbers);
```

```sortie
[ 1, 5, 10, 25, 100 ]
[ 100, 25, 10, 5, 1 ]
```

`a - b` sorts from smallest to largest, `b - a` from largest to smallest. For words
with accents, use `(a, b) => a.localeCompare(b)` (see [text](texte.md#sorting-words-alphabetically)).

> **Remember:** to sort numbers, **always** write `sort((a, b) => a - b)`.

**Your turn:** sort the array `[42, 7, 19, 3]` from largest to smallest, then display
it.

> **Solution**
>
> ```javascript
> const scores = [42, 7, 19, 3];
> scores.sort((a, b) => b - a);
> console.log(scores);
> ```
>
> ```sortie
> [ 42, 19, 7, 3 ]
> ```

## Spread and destructuring

These two short notations let you join several arrays into one, or take elements out of
an array and store them in well-named variables.

### Spread: spreading an array out

Three dots `...` in front of an array **spread it out**: its elements are laid down one
by one.

```javascript
const fruits = ["apple", "pear"];
const vegetables = ["carrot"];

const groceries = [...fruits, ...vegetables, "bread"];
console.log(groceries);
console.log(Math.max(...[3, 8, 1]));
```

```sortie
[ 'apple', 'pear', 'carrot', 'bread' ]
8
```

Line by line:

- `...fruits` lays down `"apple"` and `"pear"`, then `...vegetables` lays down `"carrot"`;
- `"bread"` comes next: `groceries` is a new array of four elements;
- `Math.max(...[3, 8, 1])` is the same as writing `Math.max(3, 8, 1)`: the largest is 8.

### Destructuring

**Destructuring** puts the elements of an array into several variables at once.

```javascript
const podium = ["Alice", "Bob", "Chloe", "David"];

const [first, second, ...others] = podium;
console.log(first);
console.log(second);
console.log(others);

let a = 1;
let b = 2;
[a, b] = [b, a];
console.log(a, b);
```

```sortie
Alice
Bob
[ 'Chloe', 'David' ]
2 1
```

The last line swaps two variables without a temporary variable.

## Building arrays

> **Note:** this part can wait. Read it once the basics feel solid.

Sometimes you do not want to write each element by hand, for example the squares of the
numbers from 0 to 4. JavaScript can build the array for you.

`Array.from` builds an array from something else: a text, or a length and a function.
`flat` flattens an array of arrays.

```javascript
console.log(Array.from("abc"));
console.log(Array.from({ length: 5 }, (_, i) => i * i));

const grid = [[1, 2], [3, 4], [5]];
console.log(grid.flat());
console.log(Array.isArray(grid));
```

```sortie
[ 'a', 'b', 'c' ]
[ 0, 1, 4, 9, 16 ]
[ 1, 2, 3, 4, 5 ]
true
```

In `(_, i) => i * i`, the `_` is a parameter you do not use: only the index `i` matters.
`Array.isArray` checks that a value is an array (`typeof` only answers `"object"`).

## Copying or sharing?

You will often want to keep the original intact and work on a copy, just as you
photocopy a form before filling it in. You still need a real copy, though.

A variable does not hold the array itself, but a **reference** to it: an address, like a
label attached to a box. Writing `b = a` sticks a second label on **the same box**.

```javascript
const a = [1, 2, 3];
const b = a;
b.push(4);

console.log(a);
console.log(a === b);
```

```sortie
[ 1, 2, 3, 4 ]
true
```

Line by line:

- `const b = a` copies nothing: `b` points to the same array as `a`;
- `b.push(4)` adds 4 to this single array;
- so `a` shows the 4 too, and `a === b` answers `true`: it is the same box.

Changing `b` changed `a`: it is the same array. For a real copy, use spread (or
`slice()`).

```javascript
const a = [1, 2, 3];
const copy = [...a];
copy.push(4);

console.log(a);
console.log(copy);
console.log([1, 2] === [1, 2]);
```

```sortie
[ 1, 2, 3 ]
[ 1, 2, 3, 4 ]
false
```

`===` compares references, not contents: two separate arrays are never equal, even with
the same elements.

## Common mistakes

### Sorting numbers without a comparison function

```javascript norun
const ages = [9, 25, 100, 3];
ages.sort();
console.log(ages);
```

```text
[ 100, 25, 3, 9 ]
```

No error, but the order is wrong: `sort` compares texts. Write
`ages.sort((a, b) => a - b)`.

### Reading past the end of the array

```javascript norun
const fruits = ["apple", "pear"];
console.log(fruits[2].toUpperCase());
```

```text
file:///home/alice/js/main.mjs:2
console.log(fruits[2].toUpperCase());
                      ^

TypeError: Cannot read properties of undefined (reading 'toUpperCase')
    at file:///home/alice/js/main.mjs:2:23

Node.js v22.22.0
```

The array has 2 elements, at indexes 0 and 1. `fruits[2]` is `undefined`, which has no
`toUpperCase` method. The last element is `fruits[fruits.length - 1]` or `fruits.at(-1)`.

### Forgetting the starting value of reduce

```javascript norun
const grades = [];
const total = grades.reduce((sum, n) => sum + n);
```

```text
file:///home/alice/js/main.mjs:2
const total = grades.reduce((sum, n) => sum + n);
                     ^

TypeError: Reduce of empty array with no initial value
    at Array.reduce (<anonymous>)
    at file:///home/alice/js/main.mjs:2:22

Node.js v22.22.0
```

Without a starting value, `reduce` takes the first element... which does not exist in an
empty array. Always give the starting value: `reduce((sum, n) => sum + n, 0)`.

### Thinking you copied an array

```javascript norun
const original = ["a", "b"];
const copy = original;
copy.pop();
console.log(original);
```

```text
[ 'a' ]
```

`copy` and `original` point to the same array. Copy with `[...original]`.

## Test yourself

**Question 1:** what does `t` contain after this code?

```javascript norun
const t = [1, 2, 3];
t.push(4);
t.shift();
```

> **Answer**
>
> `[2, 3, 4]`. `push` adds 4 at the end, `shift` removes the 1 at the start.

**Question 2:** starting from `[3, 8, 12, 5]`, how do you get the doubles of the numbers
greater than 4?

> **Answer**
>
> ```javascript
> const result = [3, 8, 12, 5].filter((n) => n > 4).map((n) => n * 2);
> console.log(result);
> ```
>
> ```sortie
> [ 16, 24, 10 ]
> ```

**Question 3:** what is the difference between `slice` and `splice`?

> **Answer**
>
> `slice` copies a piece without changing the array. `splice` removes (and can insert)
> elements **in** the array, which it changes.

## Summary

- An array is written `[a, b, c]`; its indexes start at 0; `length` gives its size.
- `push`/`pop` work at the end, `unshift`/`shift` at the start; `splice` changes,
  `slice` copies.
- `includes`, `indexOf`, `find` and `findIndex` search; `map`, `filter` and `reduce`
  transform without changing the original.
- Sort numbers with `sort((a, b) => a - b)`, never with `sort()` alone.
- `...` spreads an array out; `const [a, b] = array` destructures it.
- `b = a` shares the same array; `[...a]` makes a copy.

Next: [objects](objets.md).
