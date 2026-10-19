# Handling errors

This page explains how to throw and catch errors in TypeScript, how to handle an error of type `unknown`, and how to return a result instead of throwing an error.

> **Goals:**
>
> - you will know how to throw an error with `throw new Error(...)`;
> - you will know how to catch it with `try`, `catch` and `finally`;
> - you will know how to narrow an `unknown` error with `instanceof Error`;
> - you will know how to create your own error classes;
> - you will know how to return a result `{ ok: true, … } | { ok: false, … }` and
>   validate unknown data.

## An error stops the program

When a problem makes it impossible to go on, the code **throws** an
[exception](../debuter/glossaire.md#exception) with `throw`. "Throwing" means: setting
off the alarm. If nothing handles it, the program stops.

```typescript norun
function divide(a: number, b: number): number {
  if (b === 0) {
    throw new Error("division by zero");
  }
  return a / b;
}

console.log("Start");
console.log(divide(10, 0));
console.log("End");
```

```text
Start
file:///home/alice/projet/main.js:3
        throw new Error("division by zero");
              ^

Error: division by zero
    at divide (file:///home/alice/projet/main.js:3:15)
    at file:///home/alice/projet/main.js:8:13
    at ModuleJob.run (node:internal/modules/esm/module_job:343:25)
    at async onImport.tracePromise.__proto__ (node:internal/modules/esm/loader:665:26)
    at async asyncRunEntryPointWithESMLoader (node:internal/modules/run_main:117:5)

Node.js v22.22.0
```

`End` is never printed. Here is how to read the message:

- `Error: division by zero`: the type of the error and its description;
- the `at …` lines form the **stack trace**: the path followed up to the error. Look
  for the first line that mentions **your** file.

> **Note:** Node runs the compiled JavaScript file (`main.js`). So the line numbers
> may differ a little from your `.ts` file. The `sourceMap` option fixes that (see
> [the configuration](configuration.md)).

`new Error("…")` creates an error object. It has two useful properties: `message`
(your text) and `name` (`"Error"`). JavaScript also has `TypeError`, `RangeError`,
`SyntaxError`… which work the same way.

## try and catch

`try` surrounds the risky code. If an error is thrown, execution jumps into `catch`.
It's like a net under a trapeze artist: the fall is caught, and the show goes on.

```typescript
function divide(a: number, b: number): number {
  if (b === 0) {
    throw new Error("division by zero");
  }
  return a / b;
}

try {
  console.log(divide(10, 2));
  console.log(divide(10, 0));
  console.log("never printed");
} catch (e) {
  console.log("An error happened.");
}
console.log("The program goes on.");
```

```sortie
5
An error happened.
The program goes on.
```

As soon as `divide(10, 0)` throws the error, the rest of the `try` is skipped.

## The caught error is unknown

In JavaScript, you can throw **anything**: `throw "oops"`, `throw 42`… A library may
do it. So TypeScript can't know what `e` contains. It gives it the type `unknown`.

With `unknown`, TypeScript forbids everything until you have checked the type. This
is the [narrowing](unions.md#narrowing-with-instanceof) you saw in the chapter on
unions: test with `instanceof Error`.

```typescript
function readNumber(text: string): number {
  const n = Number(text);
  if (Number.isNaN(n)) {
    throw new Error(`"${text}" is not a number`);
  }
  return n;
}

for (const input of ["12", "twelve"]) {
  try {
    console.log(readNumber(input) * 2);
  } catch (e) {
    if (e instanceof Error) {
      console.log(`Error: ${e.message}`);
    } else {
      console.log("Unknown error:", e);
    }
  }
}
```

```sortie
24
Error: "twelve" is not a number
```

Inside the `if`, `e` is an `Error`: `e.message` is allowed. The `else` handles the
rare case where someone threw something other than an error.

> **Tip:** a small function saves you from repeating this test everywhere:
>
> ```typescript norun
> function messageOf(e: unknown): string {
>   return e instanceof Error ? e.message : String(e);
> }
> ```

## finally

The `finally` block **always** runs, whether there was an error or not. It's for
tidying up: closing a file, stopping a timer, restoring a state.

```typescript
function handle(value: number) {
  console.log(`Opening for ${value}`);
  try {
    if (value < 0) {
      throw new Error("negative value");
    }
    console.log("Processing succeeded");
  } catch (e) {
    if (e instanceof Error) console.log(`Problem: ${e.message}`);
  } finally {
    console.log("Closing");
  }
}

handle(5);
handle(-1);
```

```sortie
Opening for 5
Processing succeeded
Closing
Opening for -1
Problem: negative value
Closing
```

## Your own error classes

An error is an object like any other: you can create your own [class](types.md) that
**inherits** from `Error` with `extends`. This lets you tell errors apart with
`instanceof`, and add information to them.

```typescript
class InsufficientFundsError extends Error {
  constructor(public missing: number) {
    super(`insufficient funds (${missing} € missing)`);
    this.name = "InsufficientFundsError";
  }
}

function withdraw(balance: number, amount: number): number {
  if (amount > balance) {
    throw new InsufficientFundsError(amount - balance);
  }
  return balance - amount;
}

try {
  withdraw(50, 80);
} catch (e) {
  if (e instanceof InsufficientFundsError) {
    console.log(`Refused: ${e.message}`);
    console.log(`Missing amount: ${e.missing}`);
  } else {
    throw e;
  }
}
```

```sortie
Refused: insufficient funds (30 € missing)
Missing amount: 30
```

Three details:

- `super(message)` calls the constructor of `Error`, which stores the message;
- `this.name` changes the name shown in the stack trace;
- `throw e` in the `else` **rethrows** the errors you don't know how to handle.
  Don't swallow them silently: you would hide real bugs.

## Returning a result instead of throwing

A thrown error is invisible in the signature: nothing in
`function readAge(t: string): number` warns you that it can fail. It's easy to forget
the `try`.

Another approach: the function **returns** either a success or a failure, in a
[discriminated union](unions.md#discriminated-unions). The `ok` field serves as the
tag.

```typescript
type Result<T> = { ok: true; value: T } | { ok: false; error: string };

function readAge(text: string): Result<number> {
  const n = Number(text);
  if (!Number.isInteger(n)) {
    return { ok: false, error: `"${text}" is not an integer` };
  }
  if (n < 0 || n > 150) {
    return { ok: false, error: `${n} is not a realistic age` };
  }
  return { ok: true, value: n };
}

for (const input of ["34", "thirty", "200"]) {
  const r = readAge(input);
  if (r.ok) {
    console.log(`Age accepted: ${r.value}`);
  } else {
    console.log(`Refused: ${r.error}`);
  }
}
```

```sortie
Age accepted: 34
Refused: "thirty" is not an integer
Refused: 200 is not a realistic age
```

The return type clearly announces the possible failure. And TypeScript **forces** you
to test `r.ok` before reading `r.value` (see the
[common mistakes](#common-mistakes)). This is the idea behind Rust's `Result` type
(see [errors in Rust](../rust/erreurs.md)).

> **Remember:** throw an error for what is **abnormal** (a bug, a missing
> configuration file). Return a result for what is **predictable** (invalid input, a
> search with no answer).

## Validating unknown data

Data coming from outside (a [JSON](../debuter/glossaire.md#json) file, user input, a
web response) comes with no guarantee about its shape. `JSON.parse` returns `any`,
which turns off all checking. Store the result in an `unknown` variable instead, then
check its shape step by step.

```typescript
interface Player {
  name: string;
  score: number;
}

function validatePlayer(data: unknown): Result<Player> {
  if (typeof data !== "object" || data === null) {
    return { ok: false, error: "not an object" };
  }
  if (!("name" in data) || typeof data.name !== "string") {
    return { ok: false, error: "missing or invalid name" };
  }
  if (!("score" in data) || typeof data.score !== "number") {
    return { ok: false, error: "missing or invalid score" };
  }
  return { ok: true, value: { name: data.name, score: data.score } };
}

type Result<T> = { ok: true; value: T } | { ok: false; error: string };

const texts = ['{"name": "Lina", "score": 42}', '{"name": "Tom"}', "[1, 2]"];

for (const text of texts) {
  const data: unknown = JSON.parse(text);
  const r = validatePlayer(data);
  console.log(r.ok ? `${r.value.name} has ${r.value.score} points` : `Invalid: ${r.error}`);
}
```

```sortie
Lina has 42 points
Invalid: missing or invalid score
Invalid: missing or invalid name
```

Each test narrows `data` a little more: first an object, then an object with a string
`name`, then with a number `score`. At the end, TypeScript is convinced.

> **Note:** an array is also an `object`: `[1, 2]` passes the first test, then fails
> on `name`. For large formats, libraries like Zod write these checks for you (see
> [modules](modules.md)).

## Common mistakes

### Reading e.message without checking

```typescript norun
try {
  JSON.parse("{oops");
} catch (e) {
  console.log(e.message);
}
```

```text
main.ts(4,15): error TS18046: 'e' is of type 'unknown'.
```

`e` could be anything. Check first:
`if (e instanceof Error) console.log(e.message);`.

### Reading value without testing ok

```typescript norun
type Result<T> = { ok: true; value: T } | { ok: false; error: string };
function divide(a: number, b: number): Result<number> {
  if (b === 0) return { ok: false, error: "division by zero" };
  return { ok: true, value: a / b };
}
const r = divide(10, 2);
console.log(r.value);
```

```text
main.ts(7,15): error TS2339: Property 'value' does not exist on type 'Result<number>'.
  Property 'value' does not exist on type '{ ok: false; error: string; }'.
```

On failure, there is no `value`. Test the tag:
`if (r.ok) console.log(r.value);`.

### Using unknown data directly

```typescript norun
const data: unknown = JSON.parse('{"name": "Alice"}');
console.log(data.name);
```

```text
main.ts(2,13): error TS18046: 'data' is of type 'unknown'.
```

That's on purpose: `unknown` forces you to check. Validate the shape
(`typeof data === "object" && data !== null && "name" in data`) or use a validation
function like `validatePlayer` above.

### Throwing a string instead of an Error

```typescript norun
function readAge(text: string): number {
  const n = Number(text);
  if (Number.isNaN(n)) {
    throw "invalid age";
  }
  return n;
}
try {
  readAge("abc");
} catch (e) {
  if (e instanceof Error) console.log(e.message);
  else console.log("something else:", e);
}
```

```text
something else: invalid age
```

TypeScript accepts `throw "…"`, but that string is not an `Error`: it has no
`message`, no stack trace, and `instanceof Error` fails. Always throw an object:
`throw new Error("invalid age");`.

## Test yourself

**Question 1:** what does this code print?

```typescript norun
try {
  console.log("A");
  throw new Error("failure");
  console.log("B");
} catch (e) {
  console.log("C");
} finally {
  console.log("D");
}
```

> **Answer**
>
> `A`, `C`, then `D`. `B` is never printed: after `throw`, execution jumps to the
> `catch`. `finally` always runs.

**Question 2:** why does the error caught in `catch (e)` have the type `unknown`?

> **Answer**
>
> Because in JavaScript, you can throw any value, not just `Error` objects. TypeScript
> can't know which one: it forces you to check, for example with
> `e instanceof Error`.

**Question 3:** write a function `squareRoot(x: number)` that returns a
`Result<number>`: a failure if `x` is negative, otherwise the square root.

> **Answer**
>
> ```typescript
> type Result<T> = { ok: true; value: T } | { ok: false; error: string };
>
> function squareRoot(x: number): Result<number> {
>   if (x < 0) {
>     return { ok: false, error: "negative number" };
>   }
>   return { ok: true, value: Math.sqrt(x) };
> }
>
> const r1 = squareRoot(16);
> const r2 = squareRoot(-4);
> console.log(r1.ok ? r1.value : r1.error);
> console.log(r2.ok ? r2.value : r2.error);
> ```
>
> ```sortie
> 4
> negative number
> ```

## Summary

- `throw new Error("…")` throws an error; without a `catch`, the program stops.
- `try` / `catch` catches the error, `finally` runs in every case.
- The caught error is `unknown`: narrow it with `instanceof Error`.
- Your errors inherit from `Error`; rethrow the ones you don't know how to handle.
- For predictable failures, return `{ ok: true, value } | { ok: false, error }`.
- Data coming from outside is `unknown`: validate its shape before using it.

Next: [modules and npm](modules.md)
