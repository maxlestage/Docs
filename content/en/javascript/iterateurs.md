# Iterators and generators

This page explains what happens when you write `for...of`, how to make your own objects iterable, and how to produce values one at a time with `function*` generators.

> **Goals:**
>
> - you will understand the iteration protocol behind `for...of`;
> - you will be able to make an object iterable with `Symbol.iterator`;
> - you will be able to write a generator with `function*` and `yield`;
> - you will be able to work with an infinite sequence without blocking the program;
> - you will be able to use `...`, `entries()`, `keys()` and `values()`.

## What for...of goes through

`for...of` goes through many things: an array, a string, a `Map`, a `Set`…
These values are called **iterable**: you can go through them element by element.

```javascript
for (const letter of "cat") {
  console.log(letter);
}

for (const [key, value] of new Map([["a", 1], ["b", 2]])) {
  console.log(key, value);
}
```

```sortie
c
a
t
a 1
b 2
```

But a plain object `{ }` is **not** iterable. To go through its properties, you have to
use `Object.entries` (see [objects](objets.md)).

## The iteration protocol

How does `for...of` do it? It asks the value for an
[iterator](../debuter/glossaire.md#iterator): an object that gives the elements **one
by one**, on demand. Think of a ticket dispenser: each time you press the button, it
gives out the next ticket; when the roll is empty, it tells you.

The button is called `next()`. It returns a small object:

- `value`: the next element;
- `done`: `true` when there is nothing left.

You get the iterator with a method with a special name, `[Symbol.iterator]()`.

```javascript
const fruits = ["apple", "kiwi"];
const iterator = fruits[Symbol.iterator]();

console.log(iterator.next());
console.log(iterator.next());
console.log(iterator.next());
```

```sortie
{ value: 'apple', done: false }
{ value: 'kiwi', done: false }
{ value: undefined, done: true }
```

This is exactly what `for...of` does for you: it calls `next()` in a loop, and stops as
soon as `done` is `true`.

> **Note:** `Symbol.iterator` is a **symbol**: a special property name, which cannot
> clash with your own names. You write it between square brackets.

## Making your object iterable

For `for...of` to accept your object, give it a `[Symbol.iterator]()` method that
returns an iterator, that is, an object with a `next()` method.

Here is a countdown.

```javascript
class Countdown {
  constructor(start) {
    this.start = start;
  }

  [Symbol.iterator]() {
    let current = this.start;
    return {
      next() {
        if (current < 0) {
          return { value: undefined, done: true };
        }
        const value = current;
        current -= 1;
        return { value: value, done: false };
      },
    };
  }
}

for (const n of new Countdown(3)) {
  console.log(n);
}
console.log("Liftoff!");
```

```sortie
3
2
1
0
Liftoff!
```

Each call to `[Symbol.iterator]()` creates a new `current` variable (a
[closure](closures.md)): you can go through the same countdown several times.

It works, but it is long. Generators do the same thing in much less code.

## Generators: function* and yield

A **generator** is a special function, written `function*` (with a star). Instead of
computing everything at once, it produces its values one by one with `yield` ("give
way", "provide").

At each `yield`, the function **pauses** and gives a value. At the next `next()`, it
resumes exactly where it stopped. It is like a book with a bookmark.

```javascript
function* threeColors() {
  console.log("(start)");
  yield "red";
  console.log("(resume)");
  yield "green";
  yield "blue";
}

const gen = threeColors();
console.log(gen.next().value);
console.log(gen.next().value);

for (const color of threeColors()) {
  console.log(color);
}
```

```sortie
(start)
red
(resume)
green
(start)
red
(resume)
green
blue
```

Calling `threeColors()` does not run anything: it returns a **generator object**, which
is an iterator. The code only runs on demand, at each `next()`.

### Rewriting the countdown

With a generator, the countdown fits in a few lines. You write `*[Symbol.iterator]()`:
a generator method.

```javascript
class Countdown {
  constructor(start) {
    this.start = start;
  }

  *[Symbol.iterator]() {
    for (let n = this.start; n >= 0; n--) {
      yield n;
    }
  }
}

console.log([...new Countdown(5)]);
```

```sortie
[ 5, 4, 3, 2, 1, 0 ]
```

No more `next()`, `value` or `done` to write: the generator takes care of it.

### A generator with parameters

A generator receives parameters like an ordinary function.

```javascript
function* range(start, end, step = 1) {
  for (let n = start; n < end; n += step) {
    yield n;
  }
}

for (const n of range(0, 10, 3)) {
  console.log(n);
}
```

```sortie
0
3
6
9
```

## An infinite generator

Since a generator only computes what you ask of it, it can be **infinite**: the
`while (true)` loop does not block the program, because the function pauses at each
`yield`.

To take only part of it, let's write a small tool `take(n, iterable)`: it is itself a
generator, which stops after `n` elements.

```javascript
function* integers() {
  let n = 1;
  while (true) {
    yield n;
    n += 1;
  }
}

function* take(n, iterable) {
  if (n <= 0) return;
  let count = 0;
  for (const element of iterable) {
    yield element;
    count += 1;
    if (count === n) return;
  }
}

function* squares(iterable) {
  for (const x of iterable) {
    yield x * x;
  }
}

console.log([...take(5, integers())]);
console.log([...take(4, squares(integers()))]);
```

```sortie
[ 1, 2, 3, 4, 5 ]
[ 1, 4, 9, 16 ]
```

`return` in a generator ends it: `done` becomes `true`. Generators fit into each other
like pipes: `integers` → `squares` → `take`. Each value goes through the pipes one at a
time, and only four values are ever computed.

### The Fibonacci sequence

Each number is the sum of the two previous ones: 0, 1, 1, 2, 3, 5, 8…

```javascript
function* fibonacci() {
  let [a, b] = [0, 1];
  while (true) {
    yield a;
    [a, b] = [b, a + b];
  }
}

const first = [];
for (const n of fibonacci()) {
  if (n > 50) break;
  first.push(n);
}
console.log(first.join(", "));
```

```sortie
0, 1, 1, 2, 3, 5, 8, 13, 21, 34
```

`break` stops the `for...of` loop; the generator is then cleanly abandoned.

## Unpacking an iterable with ...

The `...` operator (**spread**) unpacks any iterable: array, string, `Set`, `Map`,
generator… You use it to build an array or to pass arguments.

```javascript
const letters = [..."hello"];
console.log(letters);

const noDuplicates = [...new Set([3, 1, 3, 2, 1])];
console.log(noDuplicates);

function* grades() {
  yield 12;
  yield 17;
  yield 9;
}
console.log(Math.max(...grades()));

const [firstGrade, ...rest] = grades();
console.log(firstGrade, rest);
```

```sortie
[ 'h', 'e', 'l', 'l', 'o' ]
[ 3, 1, 2 ]
17
12 [ 17, 9 ]
```

`Array.from(iterable)` does the same thing as `[...iterable]`.

## entries(), keys() and values()

Arrays, `Map`s and `Set`s have three methods that return iterators:

| Method | Gives |
|--------|-------|
| `keys()` | the keys (the indexes for an array) |
| `values()` | the values |
| `entries()` | `[key, value]` pairs |

```javascript
const days = ["mon", "tue", "wed"];

for (const [index, day] of days.entries()) {
  console.log(`${index + 1}. ${day}`);
}

const stock = new Map([
  ["apples", 4],
  ["pears", 0],
]);
console.log([...stock.keys()]);
console.log([...stock.values()]);
```

```sortie
1. mon
2. tue
3. wed
[ 'apples', 'pears' ]
[ 4, 0 ]
```

For a plain object, use the functions `Object.keys`, `Object.values` and
`Object.entries`: they return arrays directly.

```javascript
const scores = { alice: 12, bob: 8 };
for (const [name, score] of Object.entries(scores)) {
  console.log(`${name}: ${score}`);
}
```

```sortie
alice: 12
bob: 8
```

## Common mistakes

### Going through a plain object with for...of

```javascript norun
const person = { name: "Ada", age: 36 };
for (const x of person) console.log(x);
```

```text
file:///home/vous/projet/main.mjs:2
for (const x of person) console.log(x);
                ^

TypeError: person is not iterable
    at file:///home/vous/projet/main.mjs:2:17
```

"`person` is not iterable": a `{ }` object has no `Symbol.iterator`. Write
`for (const [key, value] of Object.entries(person))`.

### Forgetting to call the generator

```javascript norun
function* f() { yield 1; }
const it = f;
for (const x of it) console.log(x);
```

```text
file:///home/vous/projet/main.mjs:3
for (const x of it) console.log(x);
                ^

TypeError: it is not iterable
    at file:///home/vous/projet/main.mjs:3:17
```

`f` is the function; it is `f()`, the call, that returns the generator object. Add the
parentheses: `const it = f();`.

### Unpacking an infinite generator

```javascript norun
function* numbers() { let n = 1; while (true) yield n++; }
console.log([...numbers()].length);
```

```text
<--- JS stacktrace --->
FATAL ERROR: invalid array length Allocation failed - JavaScript heap out of memory
```

After many seconds, Node gives up: the memory is full. `...` wants to unpack everything,
and the sequence never ends. Limit it first: `[...take(10, numbers())]`.

## Test yourself

**Question 1:** what does this code display?

```javascript norun
function* two() {
  yield "a";
  yield "b";
}
const g = two();
console.log([...g]);
console.log([...g]);
```

> **Answer**
>
> `[ 'a', 'b' ]` then `[]`. A generator object can only be gone through **once**: it is
> used up after the first `...`. To start over, call `two()` again.

**Question 2:** what does `next()` return when an iterator has no more elements?

> **Answer**
>
> An object whose `done` is `true` (and whose `value` is usually `undefined`).

**Question 3:** write a generator `evens()` that produces 0, 2, 4, 6… endlessly, and
display its first 4 values.

> **Answer**
>
> ```javascript
> function* evens() {
>   for (let n = 0; ; n += 2) {
>     yield n;
>   }
> }
>
> const result = [];
> for (const n of evens()) {
>   if (result.length === 4) break;
>   result.push(n);
> }
> console.log(result);
> ```
>
> ```sortie
> [ 0, 2, 4, 6 ]
> ```

## Summary

- An iterable has a `[Symbol.iterator]()` method that returns an iterator.
- An iterator has a `next()` method that returns `{ value, done }`; `for...of` calls it
  for you.
- `function*` and `yield` write an iterator in a few lines: the function pauses at each
  `yield`.
- A generator can be infinite: take part of it with `break` or a tool like `take`.
- `...`, `entries()`, `keys()` and `values()` work with all iterables.

Next: [arrays in depth](tableaux-avances.md)
