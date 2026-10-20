# Asynchronous programming

This page explains how TypeScript describes operations that take time with the `Promise<T>` type, and how to wait for them cleanly with `async` and `await`.

> **Goals:**
>
> - understand what a promise is and read the `Promise<T>` type;
> - write an `async` function and wait for its result with `await`;
> - start several tasks at the same time with `Promise.all`;
> - catch the errors of a promise with `try` and `catch`;
> - turn `setTimeout` into a promise to take a pause.

## Why wait?

Some operations don't answer right away: reading a file, querying a
[server](../debuter/glossaire.md#serveur), waiting one second. If the program stayed
stuck during that time, it couldn't do anything else.

JavaScript, and so TypeScript, works differently: it **starts** the operation, carries
on with its work, and comes back to the result when it's ready. We say the code is
**asynchronous** (see [the glossary](../debuter/glossaire.md#async)).

It's like at a restaurant: you order, the waiter gives you a ticket, and you chat while
you wait. The ticket is the **promise** of a dish to come.

## The Promise<T> type

A **promise** is an object that will hold a value later. Its type is written
`Promise<T>`, where `T` is the type of the promised value:

| Type | Reads as |
|------|----------|
| `Promise<number>` | "a promise of a number" |
| `Promise<string>` | "a promise of a text" |
| `Promise<void>` | "a promise that gives no value, only an end signal" |

`Promise.resolve(value)` creates a promise that is already fulfilled. To read its value,
you write `await` in front of it:

```typescript
const promise: Promise<number> = Promise.resolve(42);
const value: number = await promise;
console.log(value);
```

```sortie
42
```

`await` means "wait until the promise is fulfilled, then give me its value". The type
changes: `promise` is a `Promise<number>`, `value` is a plain `number`.

> **Remember:** `await` "unwraps" a promise: `await` on a `Promise<T>` gives a `T`.

## async functions

The word `async` in front of a function makes it asynchronous. An `async` function
**always** returns a promise, even if you write `return 42`. TypeScript knows it: the
return type is written `Promise<number>`, not `number`.

```typescript
async function priceWithTax(priceBeforeTax: number): Promise<number> {
  return priceBeforeTax * 1.2;
}

const price = await priceWithTax(50);
console.log(`Price with tax: ${price} €`);
```

```sortie
Price with tax: 60 €
```

Inside an `async` function, you can use `await` to wait for other promises. The code
then reads from top to bottom, like ordinary code:

```typescript
async function loadName(id: number): Promise<string> {
  return id === 1 ? "Ada" : "Unknown";
}

async function greet(id: number): Promise<string> {
  const name = await loadName(id);
  return `Hello ${name}!`;
}

console.log(await greet(1));
console.log(await greet(7));
```

```sortie
Hello Ada!
Hello Unknown!
```

> **Note:** on this site, the examples are modules: `await` works directly at the file
> level ("top-level await"). Elsewhere, `await` is only allowed inside an `async`
> function.

### An async arrow function

The word `async` also goes in front of an arrow function:

```typescript
const square = async (n: number): Promise<number> => n * n;

console.log(await square(9));
```

```sortie
81
```

## Taking a pause with setTimeout

`setTimeout(function, ms)` calls a function after a delay in milliseconds. It doesn't
use a promise, but you can **wrap** it in one: `new Promise` receives a function that
calls `resolve` when the work is done.

```typescript
function wait(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

console.log("I'm ordering.");
await wait(100);
console.log("The dish arrives.");
```

```sortie
I'm ordering.
The dish arrives.
```

The program stops for 100 milliseconds between the two lines. The `Promise<void>` type
says that this promise gives no value: it just signals "it's done".

> **Tip:** Node also provides a ready-made version:
> `import { setTimeout as wait } from "node:timers/promises";`.

### The order of execution

Without `await`, the program doesn't wait: it carries on to the next line. Look at the
order of the messages:

```typescript
function wait(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function task(name: string, ms: number): Promise<void> {
  await wait(ms);
  console.log(`${name} finished`);
}

const slow = task("Slow task", 60);
const fast = task("Fast task", 20);
console.log("Both tasks are started");
await slow;
await fast;
console.log("End");
```

```sortie
Both tasks are started
Fast task finished
Slow task finished
End
```

The two tasks run at the same time. The fast one finishes first, even though it was
started second.

## Promise.all and tuples

`Promise.all` waits for **several** promises at once and returns an array of their
results, in the order you gave them (not in the order they finish).

If the promises have different types, TypeScript gives a
[tuple](../debuter/glossaire.md#tuple): an array where each slot has its own type.
Here, the result has the type `[string, number]`.

```typescript
function wait(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function loadName(): Promise<string> {
  await wait(50);
  return "Ada";
}

async function loadAge(): Promise<number> {
  await wait(10);
  return 36;
}

const [name, age] = await Promise.all([loadName(), loadAge()]);
console.log(`${name} is ${age} years old`);
console.log(name.toUpperCase(), age + 1);
```

```sortie
Ada is 36 years old
ADA 37
```

Thanks to the tuple, TypeScript knows that `name` is a `string` and `age` a `number`:
you can call `toUpperCase()` on one and do a calculation with the other.

### A list of promises

With `map`, you create one promise per element, then `Promise.all` waits for all of
them. The result is a `number[]`:

```typescript
async function wordLength(word: string): Promise<number> {
  return word.length;
}

const words = ["cat", "elephant", "bear"];
const lengths: number[] = await Promise.all(words.map(wordLength));
console.log(lengths);
```

```sortie
[ 3, 8, 4 ]
```

> **Remember:** start all the promises first, then wait for them together with
> `Promise.all`. It's faster than waiting for them one by one.

## Handling errors

A promise can **fail**: we say it is **rejected**. With `await`, a rejection becomes
an ordinary [exception](../debuter/glossaire.md#exception), which you catch with `try`
and `catch`.

In strict mode, the `catch` variable has the type `unknown`: TypeScript doesn't know
what was thrown. Check with `instanceof Error` before reading `.message`.

```typescript
async function divide(a: number, b: number): Promise<number> {
  if (b === 0) {
    throw new Error("division by zero");
  }
  return a / b;
}

try {
  console.log(await divide(10, 2));
  console.log(await divide(1, 0));
  console.log("Never printed");
} catch (e) {
  if (e instanceof Error) {
    console.log(`Error: ${e.message}`);
  }
}
```

```sortie
5
Error: division by zero
```

If a single promise of `Promise.all` fails, the whole `Promise.all` fails.

### Promise.allSettled: keeping all the results

`Promise.allSettled` waits for all the promises, successful or not. Each result has a
`status` field that is `"fulfilled"` or `"rejected"`. TypeScript uses this field to
know whether you can read `value` or `reason`:

```typescript
async function checkAge(age: number): Promise<number> {
  if (age < 0) {
    throw new Error(`invalid age: ${age}`);
  }
  return age;
}

const results = await Promise.allSettled([checkAge(12), checkAge(-3), checkAge(40)]);
for (const r of results) {
  if (r.status === "fulfilled") {
    console.log("OK", r.value);
  } else {
    console.log("Failed", String(r.reason));
  }
}
```

```sortie
OK 12
Failed Error: invalid age: -3
OK 40
```

## Common mistakes

### Forgetting await

```typescript norun
async function readNumber(): Promise<number> {
  return 42;
}

const n: number = readNumber();
console.log(n + 1);
```

```text
main.ts(5,7): error TS2322: Type 'Promise<number>' is not assignable to type 'number'.
```

`readNumber()` returns a promise, not a number. Add `await`:
`const n: number = await readNumber();`.

### Writing the wrong return type

```typescript norun
async function readNumber(): number {
  return 42;
}
```

```text
main.ts(1,30): error TS1064: The return type of an async function or method must be the global Promise<T> type. Did you mean to write 'Promise<number>'?
```

An `async` function always returns a promise. Write `Promise<number>`.

### Using await outside an async function

```typescript norun
function wait(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function start() {
  await wait(100);
  console.log("Ready");
}
```

```text
main.ts(6,3): error TS1308: 'await' expressions are only allowed within async functions and at the top levels of modules.
```

Add `async` in front of the function: `async function start() { … }`.

### Reading e.message without checking

```typescript norun
try {
  await Promise.reject(new Error("breakdown"));
} catch (e) {
  console.log(e.message);
}
```

```text
main.ts(4,15): error TS18046: 'e' is of type 'unknown'.
```

Anything can be thrown, not only an `Error`. Check first:
`if (e instanceof Error) { console.log(e.message); }`.

## Test yourself

**Question 1:** what is the return type of `async function f() { return "ok"; }`?

> **Answer**
>
> `Promise<string>`. An `async` function always wraps its return value in a promise.

**Question 2:** with `const [a, b] = await Promise.all([Promise.resolve(1), Promise.resolve("x")])`,
what are the types of `a` and `b`?

> **Answer**
>
> `a` is a `number` and `b` a `string`. `Promise.all` gives a tuple
> `[number, string]`, in the order of the promises.

**Question 3:** write a function `wait(ms)` that returns a promise fulfilled after `ms`
milliseconds, then use it to print "One", wait, and print "Two".

> **Answer**
>
> ```typescript
> function wait(ms: number): Promise<void> {
>   return new Promise((resolve) => setTimeout(resolve, ms));
> }
>
> console.log("One");
> await wait(50);
> console.log("Two");
> ```
>
> ```sortie
> One
> Two
> ```

## Summary

- A promise `Promise<T>` will hold a value of type `T` later.
- An `async` function always returns a promise; `await` takes its value out.
- `Promise.all` waits for several promises and gives a well-typed tuple.
- A rejected promise becomes an exception: catch it with `try` and `catch`, checking
  `e instanceof Error`.
- `new Promise((resolve) => setTimeout(resolve, ms))` creates a pause.

Next: [configuring TypeScript](configuration.md). Compare with
[concurrency in Ruby](../ruby/concurrence.md).
