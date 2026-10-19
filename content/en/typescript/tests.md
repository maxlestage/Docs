# Testing your code

This page shows how to write automated tests in TypeScript, with the tools that ship with Node (`node:assert` and `node:test`), with Vitest, and even how to test the types themselves.

> **Goals:**
>
> - understand what an automated test is for;
> - check results with `node:assert/strict`;
> - organize and run tests with `node:test`;
> - write the same tests with Vitest;
> - check that a wrong usage is really refused with `// @ts-expect-error`.

## Why test?

A [test](../debuter/glossaire.md#test) is a small program that checks another
program. It calls your code with known values and compares the result with the one
you expect.

TypeScript already catches many errors: a text instead of a number, a misspelled
property. But it doesn't know whether your **calculation** is right. An `average`
function that divides by the wrong number has perfect types… and a wrong result.
Types check the **shape**, tests check the **meaning**.

With tests:

- a single command checks everything, in a few seconds;
- if a change breaks something, you know right away;
- tests show how to use your code: documentation that works.

> **Remember:** a test answers a simple question: "with this input, do I really get
> this output?".

## Checking with node:assert/strict

The `node:assert/strict` module provides checking functions, called
**assertions**. If the check succeeds, nothing happens. If it fails, an error
stops the program.

```typescript
import assert from "node:assert/strict";

function double(n: number): number {
  return n * 2;
}

assert.equal(double(4), 8);
assert.equal(double(0), 0);
assert.ok(double(3) > 5);
console.log("All checks pass.");
```

```sortie
All checks pass.
```

`assert.equal(actual, expected)` reads: "check that `double(4)` really equals `8`".
The computed value comes first, the expected value second.

### The most useful assertions

| Assertion | Checks that… |
|-----------|--------------|
| `assert.equal(a, b)` | `a` and `b` are identical (`===`) |
| `assert.deepEqual(a, b)` | two objects or arrays have the same content |
| `assert.ok(value)` | the value is true |
| `assert.throws(() => f(), /message/)` | `f()` throws an error whose message matches |
| `await assert.rejects(promise)` | a promise is rejected |

`deepEqual` compares the **content**: two arrays created separately are never the
same object, but they can hold the same values.

```typescript
import assert from "node:assert/strict";

function sortNumbers(numbers: number[]): number[] {
  return [...numbers].sort((a, b) => a - b);
}

assert.deepEqual(sortNumbers([3, 1, 2]), [1, 2, 3]);
assert.deepEqual({ name: "Ada", age: 36 }, { name: "Ada", age: 36 });
console.log("Same content.");
```

```sortie
Same content.
```

### When an assertion fails

The error message shows the value you got and the expected value. Here, we catch it
to print it:

```typescript
import assert from "node:assert/strict";

function average(grades: number[]): number {
  return grades.reduce((a, b) => a + b, 0) / (grades.length + 1);
}

try {
  assert.equal(average([10, 14, 18]), 14);
} catch (e) {
  if (e instanceof assert.AssertionError) {
    console.log(e.message);
  }
}
```

```sortie
Expected values to be strictly equal:

10.5 !== 14
```

The test found the bug: we divide by `grades.length + 1` instead of `grades.length`.
TypeScript saw nothing.

### Testing errors and promises

`assert.throws` checks that a function really throws an error. For asynchronous code
(see [asynchronous programming](asynchrone.md)), `assert.rejects` does the same thing
with a promise, and is used with `await`:

```typescript
import assert from "node:assert/strict";

function squareRoot(n: number): number {
  if (n < 0) {
    throw new RangeError("negative number");
  }
  return Math.sqrt(n);
}

async function load(id: number): Promise<string> {
  if (id <= 0) {
    throw new Error("invalid id");
  }
  return `item ${id}`;
}

assert.equal(squareRoot(9), 3);
assert.throws(() => squareRoot(-1), RangeError);
assert.throws(() => squareRoot(-1), /negative/);

assert.equal(await load(2), "item 2");
await assert.rejects(load(0), /invalid id/);
console.log("Errors correctly detected.");
```

```sortie
Errors correctly detected.
```

## Organizing tests with node:test

Loose assertions are fine to start with. But at the first error, everything stops,
and you don't know which other tests would have failed. The `node:test` module,
shipped with Node, organizes assertions into named **tests**, grouped with `describe`:

- `describe("name", () => { … })` groups the tests of one function;
- `test("what we check", () => { … })` is a test: a sentence that describes the
  expected behavior.

```typescript
import { describe, test } from "node:test";
import assert from "node:assert/strict";

function average(grades: number[]): number {
  if (grades.length === 0) {
    throw new Error("empty list");
  }
  return grades.reduce((a, b) => a + b, 0) / grades.length;
}

describe("average", () => {
  test("computes the average of several grades", () => {
    assert.equal(average([10, 14, 18]), 14);
  });

  test("works with a single grade", () => {
    assert.equal(average([12]), 12);
  });

  test("refuses an empty list", () => {
    assert.throws(() => average([]), /empty list/);
  });
});
```

A test can also be `async`: write `test("…", async () => { … })` and use `await`
inside.

### Running the tests

Put this code in `src/average.test.ts`. Compile (see
[configuring TypeScript](configuration.md)), then `node --test` runs all the test
files:

```sh
npx tsc
node --test "dist/**/*.test.js"
```

```text
▶ average
  ✔ computes the average of several grades (0.736478ms)
  ✔ works with a single grade (0.23265ms)
  ✔ refuses an empty list (0.831555ms)
✔ average (3.066365ms)
ℹ tests 3
ℹ suites 1
ℹ pass 3
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 121.217153
```

Each `✔` is a passing test. The durations in parentheses change every run.

If a test fails, it is marked `✖`, and the details are shown at the end, with the
faulty line:

```text
▶ average
  ✖ computes the average of several grades (2.054574ms)
  ✔ works with a single grade (0.163259ms)
  ✔ refuses an empty list (0.505934ms)
✖ average (4.078187ms)
ℹ tests 3
ℹ suites 1
ℹ pass 2
ℹ fail 1
...

✖ failing tests:

test at dist/average.test.js:10:5
✖ computes the average of several grades (2.054574ms)
  AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:

  14 !== 15
```

The other tests were still run: you see at once everything that works and everything
that doesn't.

> **Tip:** add `"test": "tsc && node --test \"dist/**/*.test.js\""` to the `scripts`
> section of `package.json`. Then `npm test` is all you need.

## Vitest

**Vitest** is a test tool widely used in TypeScript projects, especially for the
web. Its big advantage: it reads `.ts` files directly, without compiling them
first. Install it in the project:

```sh
npm install --save-dev vitest
```

The code to test, in `src/cart.ts`:

```typescript norun
export interface Item {
  price: number;
  quantity: number;
}

export function total(items: Item[]): number {
  return items.reduce((sum, a) => sum + a.price * a.quantity, 0);
}
```

The tests, in `src/cart.test.ts`. Instead of `assert`, Vitest uses
`expect(value)`, followed by a check: `toBe` (identical), `toEqual` (same
content), `toThrow` (throws an error).

```typescript norun
import { describe, it, expect } from "vitest";
import { total } from "./cart.js";

describe("total", () => {
  it("adds up the items", () => {
    expect(total([{ price: 2, quantity: 3 }, { price: 4, quantity: 1 }])).toBe(10);
  });

  it("is 0 for an empty cart", () => {
    expect(total([])).toBe(0);
  });

  it("returns a well-formed new object", () => {
    const summary = { count: 2, total: total([{ price: 5, quantity: 2 }]) };
    expect(summary).toEqual({ count: 2, total: 10 });
  });
});
```

```sh
npx vitest run
```

```text
 RUN  v3.2.4 /home/you/my-project

 ✓ src/cart.test.ts (3 tests) 6ms

 Test Files  1 passed (1)
      Tests  3 passed (3)
   Start at  21:12:24
   Duration  807ms (transform 181ms, setup 0ms, collect 109ms, tests 6ms, environment 0ms, prepare 156ms)
```

Without `run`, `npx vitest` stays open and reruns the tests every time you save,
like the watch mode of `tsc`.

> **Note:** `it` and `test` are synonyms. `it("adds up the items")` reads like an
> English sentence: "it adds up the items".

## Testing types with @ts-expect-error

With TypeScript, you can also check that a **wrong** usage is really **refused**. The
comment `// @ts-expect-error` says: "the next line must cause a type error". Two
cases:

- the line does contain an error: it is ignored, all is well;
- the line does **not** contain an error: `tsc` reports the useless comment.

So it's a test: if someone weakens your types one day, the compilation fails.

```typescript
interface User {
  name: string;
  age: number;
}

function createUser(name: string, age: number): User {
  return { name, age };
}

function checkTypes(): void {
  // @ts-expect-error: the age must be a number
  createUser("Ada", "36");

  // @ts-expect-error: the age is missing
  createUser("Ada");

  const u = createUser("Ada", 36);
  // @ts-expect-error: the email property doesn't exist
  console.log(u.email);
}

console.log(createUser("Grace", 45));
```

```sortie
{ name: 'Grace', age: 45 }
```

The `checkTypes` function is never called: it is only there for the compiler. Run
`tsc --noEmit` to run these "type tests".

> **Warning:** don't use `@ts-expect-error` (or `@ts-ignore`) to silence a real
> error in your program. Fix it instead.

## What should you test?

You can't test everything. Focus on:

- **the normal case**: an ordinary input and its expected result;
- **the edge cases**: empty list, zero, negative number, empty text, a single element;
- **the errors**: an invalid input must throw the right error;
- **bugs already found**: every fixed bug deserves a test, so that it doesn't come back.

There's no point, on the other hand, in testing what TypeScript already guarantees:
that a function `(n: number) => number` returns a number, for example.

A good habit: write the test **before** the code. Write a test that fails (the
function doesn't exist yet), then the simplest code that makes it pass, then improve
the code while keeping the tests green.

## Common mistakes

### Comparing two objects with equal

```typescript norun
import assert from "node:assert/strict";

assert.equal({ x: 1 }, { x: 1 });
```

```text
AssertionError [ERR_ASSERTION]: Values have same structure but are not reference-equal:

{
  x: 1
}
```

`equal` checks that it is **the same object**. To compare the content, use
`assert.deepEqual` (or `toEqual` with Vitest).

### Forgetting await in an asynchronous test

```typescript norun
import assert from "node:assert/strict";

async function load(): Promise<number> {
  return 3;
}

assert.equal(load(), 3);
```

```text
AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:
+ actual - expected

+ Promise {
+   3
+ }
- 3
```

You are comparing a promise with a number. Write `assert.equal(await load(), 3);`.

### A @ts-expect-error that expects nothing

```typescript norun
function double(n: number): number {
  return n * 2;
}

// @ts-expect-error
double(4);
```

```text
main.ts(5,1): error TS2578: Unused '@ts-expect-error' directive.
```

`double(4)` is correct: there is no error to expect. Remove the comment, or put it
above a really invalid call, such as `double("4")`.

## Test yourself

**Question 1:** what is the difference between `assert.equal` and `assert.deepEqual`?

> **Answer**
>
> `equal` checks that the two values are identical (`===`): for objects, that they are
> the same object. `deepEqual` compares the content, property by property.

**Question 2:** why test a function whose types are already checked by TypeScript?

> **Answer**
>
> Types check the shape of the data, not whether the calculation is right. A function
> can have perfect types and return a wrong result.

**Question 3:** write a function `initials(name: string): string` ("Ada Lovelace"
gives "AL") and check it with two assertions, one of them on an empty text.

> **Answer**
>
> ```typescript
> import assert from "node:assert/strict";
>
> function initials(name: string): string {
>   return name
>     .split(" ")
>     .filter((word) => word !== "")
>     .map((word) => word[0]?.toUpperCase() ?? "")
>     .join("");
> }
>
> assert.equal(initials("Ada Lovelace"), "AL");
> assert.equal(initials(""), "");
> console.log("OK");
> ```
>
> ```sortie
> OK
> ```

## Summary

- Types check the shape of the code; tests check that it does the right calculation.
- `node:assert/strict`: `equal`, `deepEqual`, `ok`, `throws`, `rejects`.
- `node:test` groups tests with `describe` and `test`; `node --test` runs them.
- Vitest reads TypeScript directly and uses `expect(...).toBe(...)`.
- `// @ts-expect-error` checks that a wrong usage is really refused by the compiler.
- Test the normal case, the edge cases, the errors and the bugs already fixed.

Next: [debugging a program](debogage.md). Compare with
[tests in Ruby](../ruby/tests.md).
