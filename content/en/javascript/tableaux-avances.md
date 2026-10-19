# Arrays in depth

This page goes further with arrays: flattening, grouping, sorting without modifying, searching from the end, making real copies, and understanding `reduce` step by step.

> **Goals:**
>
> - you will be able to flatten an array with `flat` and `flatMap`;
> - you will be able to group elements with `Object.groupBy`;
> - you will be able to sort or change an array **without touching the original** with
>   `toSorted`, `toReversed` and `with`;
> - you will be able to make a real copy with `structuredClone`;
> - you will be able to follow a `reduce` step by step.

## Before you start

This page assumes that you know [arrays](collections.md): `push`, `map`, `filter`,
`find` and `sort`. If you don't, read that page first.

One idea comes back everywhere here. An array method does one of these two things:

```text
 CHANGES the array             RETURNS a new array
 (you write in the notebook)   (you make a photocopy)
 push, pop, sort, reverse      map, filter, flat, toSorted, with
```

Knowing which column a method is in saves you from many surprises.

## The last element with at

`at(-1)` gives the last element. Negative numbers count **from the end**.

```javascript
const queue = ["Alice", "Bob", "Chloe"];

console.log(queue.at(0));
console.log(queue.at(-1));
console.log(queue.at(-2));
console.log(queue[queue.length - 1]);
```

```sortie
Alice
Chloe
Bob
Chloe
```

`queue.at(-1)` and `queue[queue.length - 1]` do the same thing. The first form reads
better: "element 1 counting from the end".

> **Warning:** `queue[-1]` does not work: it gives `undefined`. Only `at` understands
> negative numbers.

## Searching from the end: findLast

`find` starts from the beginning. `findLast` starts from **the end** and returns the
first element that matches, going backwards.

```javascript
const temperatures = [12, 25, 18, 27, 15];

console.log(temperatures.find((t) => t > 20));
console.log(temperatures.findLast((t) => t > 20));
console.log(temperatures.findLastIndex((t) => t > 20));
```

```sortie
25
27
3
```

It is handy for "the **last** day it was warmer than 20 degrees".

Try it: replace `20` with `30`. The first two lines print `undefined`, the last one
prints `-1`: nothing matches.

## some and every: yes or no?

These two methods answer with `true` or `false`.

- `some`: "does **at least one** element match?"
- `every`: "do **all** the elements match?"

```javascript
const grades = [12, 8, 15];

console.log(grades.some((g) => g < 10));
console.log(grades.every((g) => g >= 10));
console.log(grades.every((g) => g > 0));
```

```sortie
true
false
true
```

Think of a bag check at the entrance of a concert. `some` looks for **one** forbidden
item. `every` checks that **all** the items are allowed. As soon as the answer is known,
the search stops.

> **Remember:** `some` = "at least one"; `every` = "all".

## Flattening with flat and flatMap

### flat: unpacking boxes inside boxes

An array can contain other arrays. `flat` **unpacks** them by one level.

```javascript
const drawers = [["socks", "sweaters"], ["books"], ["pens", "erasers"]];

console.log(drawers.flat());
console.log([1, [2, [3, [4]]]].flat());
console.log([1, [2, [3, [4]]]].flat(Infinity));
```

```sortie
[ 'socks', 'sweaters', 'books', 'pens', 'erasers' ]
[ 1, 2, [ 3, [ 4 ] ] ]
[ 1, 2, 3, 4 ]
```

```text
 [ [a, b], [c] ]   ──flat()──▶   [ a, b, c ]
```

By default, `flat` unpacks **one** level. `flat(Infinity)` unpacks everything, however
deep it goes.

### flatMap: transform, then flatten

`flatMap(f)` does a `map(f)`, then a `flat()`. It is useful when each element gives
**several** results.

```javascript
const sentences = ["it is sunny", "let's go out"];

console.log(sentences.map((s) => s.split(" ")));
console.log(sentences.flatMap((s) => s.split(" ")));
```

```sortie
[ [ 'it', 'is', 'sunny' ], [ "let's", 'go', 'out' ] ]
[ 'it', 'is', 'sunny', "let's", 'go', 'out' ]
```

With `map`, you get an array of arrays. With `flatMap`, a single list of words.

Try it: `[1, 2, 3].flatMap((n) => [n, n])`. You get `[1, 1, 2, 2, 3, 3]`: each number
is doubled.

## Grouping with Object.groupBy

Imagine you are sorting laundry by color: a pile of whites, a pile of colors.
`Object.groupBy` makes these piles for you.

You give it the array and a function that says **which pile** each element goes into.

```javascript
const fruits = [
  { name: "apple", color: "red" },
  { name: "kiwi", color: "green" },
  { name: "strawberry", color: "red" },
];

const piles = Object.groupBy(fruits, (f) => f.color);

console.log(piles.red.map((f) => f.name));
console.log(piles.green.map((f) => f.name));
console.log(Object.keys(piles));
```

```sortie
[ 'apple', 'strawberry' ]
[ 'kiwi' ]
[ 'red', 'green' ]
```

The result is an object: each key is the name of a pile, each value an array.

The function can make up its own pile names:

```javascript
const grades = [12, 7, 15, 9, 18];

const groups = Object.groupBy(grades, (g) => (g >= 10 ? "passed" : "to review"));

console.log(groups["passed"]);
console.log(groups["to review"]);
```

```sortie
[ 12, 15, 18 ]
[ 7, 9 ]
```

> **Note:** if you print the result directly with `console.log(piles)`, Node writes
> `[Object: null prototype]` in front of it. It is a "bare" object, without the usual
> methods. You can use it normally.

## Sorting without modifying: toSorted, toReversed, with

`sort` and `reverse` **modify** the array. Sometimes you want to keep the original
intact: for example, a list in order of arrival **and** the same list sorted.

Three methods return a modified **copy**:

| Modifies the original | Returns a copy |
| --- | --- |
| `sort(f)` | `toSorted(f)` |
| `reverse()` | `toReversed()` |
| `t[i] = x` | `t.with(i, x)` |

```javascript
const arrivals = [42, 7, 19];

const sorted = arrivals.toSorted((a, b) => a - b);
const reversed = arrivals.toReversed();
const fixed = arrivals.with(1, 8);

console.log(sorted);
console.log(reversed);
console.log(fixed);
console.log(arrivals);
```

```sortie
[ 7, 19, 42 ]
[ 19, 7, 42 ]
[ 42, 8, 19 ]
[ 42, 7, 19 ]
```

The last line proves it: `arrivals` has not moved.

Compare with `sort`, which writes into the array:

```javascript
const arrivals = [42, 7, 19];
const sorted = arrivals.sort((a, b) => a - b);

console.log(sorted);
console.log(arrivals);
console.log(sorted === arrivals);
```

```sortie
[ 7, 19, 42 ]
[ 7, 19, 42 ]
true
```

`sort` sorted `arrivals` itself, and `sorted` is **the same** array.

> **Remember:** the methods that start with `to…` and `with` never touch the original.

## Building an array with Array.from

`Array.from({ length: n }, function)` builds an array of `n` slots. The function
receives the index of each slot and returns what goes in it.

```javascript
const indices = Array.from({ length: 5 }, (_, i) => i);
const evens = Array.from({ length: 5 }, (_, i) => i * 2);
const stars = Array.from({ length: 3 }, (_, i) => "*".repeat(i + 1));

console.log(indices);
console.log(evens);
console.log(stars);
```

```sortie
[ 0, 1, 2, 3, 4 ]
[ 0, 2, 4, 6, 8 ]
[ '*', '**', '***' ]
```

The `_` is the first parameter, which we don't use (the empty slot). Only the index `i`
matters to us.

| Slot | i | `i * 2` |
| --- | --- | --- |
| 1st | 0 | 0 |
| 2nd | 1 | 2 |
| 3rd | 2 | 4 |
| 4th | 3 | 6 |
| 5th | 4 | 8 |

Try it: write `(_, i) => i + 1`. You get the numbers from 1 to 5.

## Really copying: spread or structuredClone

### Spread copies only one level

`[...t]` copies the array. But if the array contains **objects**, these objects are not
copied: the copy contains the **same** objects.

```javascript
const team = [{ name: "Alice", score: 10 }];
const copy = [...team];

copy[0].score = 99;
console.log(team[0].score);

copy.push({ name: "Bob", score: 5 });
console.log(team.length);
```

```sortie
99
1
```

Adding an element to `copy` does not change `team`: the two arrays are different. But
changing Alice's object changes it **in both**, because it is the same object.

```text
 team ──▶ [ • ]─────┐
                    ├──▶ { name: 'Alice', score: 99 }
 copy ──▶ [ • , • ]─┘          (only one object!)
                └──▶ { name: 'Bob', score: 5 }
```

This is called a **shallow** copy: you photocopy the list, not the cards stored in it.

### structuredClone copies every level

`structuredClone` makes a **deep** copy: the array, and everything it contains.

```javascript
const team = [{ name: "Alice", score: 10 }];
const copy = structuredClone(team);

copy[0].score = 99;
console.log(team[0].score);
console.log(copy[0].score);
```

```sortie
10
99
```

| Copy | The array | The objects inside |
| --- | --- | --- |
| `b = a` | shared | shared |
| `[...a]` | copied | shared |
| `structuredClone(a)` | copied | copied |

> **Remember:** an array of numbers or strings is copied with `[...a]`. An array of
> objects that you are going to change is copied with `structuredClone(a)`.

## reduce step by step

`reduce` is the method that scares people the most. Yet it is just a loop with a box
that fills up.

```text
 array.reduce((box, element) => new value of the box, starting value)
```

On each round, the function receives **the box** (the accumulator) and **the element**.
What it returns becomes the new box for the next round.

### A sum

```javascript
const cart = [4, 10, 6];
const total = cart.reduce((box, price) => box + price, 0);
console.log(total);
```

```sortie
20
```

| Round | box (before) | price | box + price (after) |
| --- | --- | --- | --- |
| start | – | – | 0 |
| 1 | 0 | 4 | 4 |
| 2 | 4 | 10 | 14 |
| 3 | 14 | 6 | 20 |

After the last round, `reduce` returns the box: `20`.

### The same calculation with a loop

Here is exactly what `reduce` does, written with a loop:

```javascript
const cart = [4, 10, 6];

let box = 0;
for (const price of cart) {
  box = box + price;
}
console.log(box);
```

```sortie
20
```

If `reduce` looks obscure to you, write the loop first. Then transform it.

### The largest element

The box can hold something other than a total: here, the largest number seen so far.

```javascript
const scores = [7, 21, 13];
const best = scores.reduce((box, s) => (s > box ? s : box), 0);
console.log(best);
```

```sortie
21
```

| Round | box (before) | s | s > box ? | box (after) |
| --- | --- | --- | --- | --- |
| start | – | – | – | 0 |
| 1 | 0 | 7 | yes | 7 |
| 2 | 7 | 21 | yes | 21 |
| 3 | 21 | 13 | no | 21 |

### Counting with an object

The box can also be an object. Let's count the votes:

```javascript
const votes = ["yes", "no", "yes", "yes"];

const count = votes.reduce((box, v) => {
  box[v] = (box[v] ?? 0) + 1;
  return box;
}, {});

console.log(count);
```

```sortie
{ yes: 3, no: 1 }
```

| Round | v | box (after) |
| --- | --- | --- |
| start | – | `{}` |
| 1 | `"yes"` | `{ yes: 1 }` |
| 2 | `"no"` | `{ yes: 1, no: 1 }` |
| 3 | `"yes"` | `{ yes: 2, no: 1 }` |
| 4 | `"yes"` | `{ yes: 3, no: 1 }` |

With curly braces, the arrow must end with `return box;`. Without it, the box for the
next round would be `undefined`.

> **Remember:** in `reduce`, the function always returns **the new box**. Always give a
> starting value.

## Common mistakes

### Thinking that sort does not touch the original

```javascript norun
const arrivals = [42, 7, 19];
const sorted = arrivals.sort((a, b) => a - b);
console.log("First to arrive:", arrivals[0]);
```

```text
First to arrive: 7
```

No error, but a wrong result: the first to arrive was `42`. `sort` modified `arrivals`.
Use `arrivals.toSorted((a, b) => a - b)`.

### Calling groupBy on the array

```javascript norun
const words = ["one", "two", "three"];
console.log(words.groupBy((w) => w.length));
```

```text
file:///home/alice/js/main.mjs:2
console.log(words.groupBy((w) => w.length));
                  ^

TypeError: words.groupBy is not a function
    at file:///home/alice/js/main.mjs:2:19

Node.js v22.22.0
```

`groupBy` is not an array method: it is a function of `Object`. Write
`Object.groupBy(words, (w) => w.length)`.

### Using with on an index that does not exist

```javascript norun
const grades = [12, 15];
const newGrades = grades.with(5, 20);
```

```text
file:///home/alice/js/main.mjs:2
const newGrades = grades.with(5, 20);
                             ^

RangeError: Invalid index : 5
    at Array.with (<anonymous>)
    at file:///home/alice/js/main.mjs:2:30

Node.js v22.22.0
```

`with` replaces a slot that **already exists**. Here, there are only the indices 0 and
1. To add at the end without modifying the original, write `[...grades, 20]`.

### Copying an object that contains a function with structuredClone

```javascript norun
const button = { text: "OK", click() {} };
const copy = structuredClone(button);
```

```text
node:internal/per_context/domexception:76
    const self = new Error();
                 ^

DOMException [DataCloneError]: click() {} could not be cloned.
    at new DOMException (node:internal/per_context/domexception:76:18)
    at structuredClone (node:internal/worker/js_transferable:126:26)
    at file:///home/alice/js/main.mjs:2:14

Node.js v22.22.0
```

`structuredClone` copies **data** (numbers, strings, arrays, objects, dates), not
functions. Keep your data in plain objects, without methods.

## Test yourself

**Question 1:** what is the value of `[[1, 2], [3]].flat().at(-1)`?

> **Answer**
>
> ```javascript
> console.log([[1, 2], [3]].flat().at(-1));
> ```
>
> ```sortie
> 3
> ```
>
> `flat()` gives `[1, 2, 3]`, and `at(-1)` takes the last element.

**Question 2:** you want the sorted list **and** you want to keep the original list.
Which method should you choose?

> **Answer**
>
> `toSorted`, which returns a sorted copy. `sort` would modify the original list.

**Question 3:** fill in the trace table of
`[2, 3, 4].reduce((box, n) => box * n, 1)`.

> **Answer**
>
> | Round | box (before) | n | box (after) |
> | --- | --- | --- | --- |
> | start | – | – | 1 |
> | 1 | 1 | 2 | 2 |
> | 2 | 2 | 3 | 6 |
> | 3 | 6 | 4 | 24 |
>
> ```javascript
> console.log([2, 3, 4].reduce((box, n) => box * n, 1));
> ```
>
> ```sortie
> 24
> ```

## Summary

- `at(-1)` reads the last element; `findLast` searches from the end; `some` and `every`
  answer "at least one" and "all".
- `flat` unpacks nested arrays; `flatMap` transforms, then unpacks.
- `Object.groupBy(array, f)` sorts the elements into piles named by `f`.
- `toSorted`, `toReversed` and `with` return a copy: the original does not change.
- `[...a]` copies one level; `structuredClone(a)` copies everything.
- `reduce` passes a box from one round to the next: follow it in a table.

Next: [handling errors](erreurs.md).
