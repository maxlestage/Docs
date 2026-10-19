# Testing your code

This page shows how to write automated tests in JavaScript with the tools that come with Node: `console.assert`, the `node:assert` module and the `node:test` test runner.

> **Goals:**
>
> - understand what an automated test is for;
> - check values with `assert.strictEqual`, `assert.deepStrictEqual` and `assert.throws`;
> - write and run tests with `test()` and `describe()`;
> - read a failing test;
> - write the test before the code.

## Why test?

A **test** is a small program that checks another program (see
[the glossary](../debuter/glossaire.md#test)). It calls your code with known values
and compares the result with the one you expect.

Without tests, you check by hand, with `console.log`, and you start over after every
change. With tests:

- a single command checks everything, in one second;
- if a change breaks something, you know right away;
- tests show how to use your code: they are documentation that works.

It is like a car inspection: a list of precise points, always checked the same way.

> **Remember:** a test answers a simple question: "with this input, do I really get
> this output?".

## The simplest: console.assert

`console.assert(condition, message)` checks a condition. If it is true, nothing
happens. If it is false, the message is shown.

```javascript norun
console.assert(1 + 1 === 3, "the calculation is wrong");
console.assert(2 + 2 === 4, "never shown");
console.log("End");
```

```text
Assertion failed: the calculation is wrong
End
```

When everything is fine, `console.assert` stays silent:

```javascript
function double(x) {
  return x * 2;
}

console.assert(double(4) === 8, "double(4) should be 8");
console.assert(double(0) === 0, "double(0) should be 0");
console.log("Checks finished.");
```

```sortie
Checks finished.
```

> **Warning:** `console.assert` does not stop the program: `End` is shown anyway.
> It is handy for a quick check, but a real test must **stop** and report the
> failure. That is the job of `node:assert`.

## The node:assert module

Node provides the `node:assert` module. Import its **strict** version,
`node:assert/strict`: it compares with `===`, without any hidden type conversion.

### assert.strictEqual

`assert.strictEqual(actual, expected)` checks that two values are identical. If they
are, nothing is shown.

```javascript
import assert from "node:assert/strict";

function double(x) {
  return x * 2;
}

assert.strictEqual(double(4), 8);
assert.strictEqual(double(-3), -6);
console.log("All tests pass.");
```

```sortie
All tests pass.
```

The order matters: first the value you **got** (`actual`), then the **expected**
value (`expected`). That way, the failure message is correct.

### Reading a failure

If the values differ, `assert` throws an
[exception](../debuter/glossaire.md#exception) `AssertionError` and the program
stops:

```javascript norun
import assert from "node:assert/strict";

assert.strictEqual(0.1 + 0.2, 0.3);
```

```text
AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:
+ actual - expected

+ 0.30000000000000004
- 0.3
     ^

    at file:///home/alice/project/main.mjs:3:8
```

The `+` line gives the value you got, the `-` line the expected value. The `^` shows
where they start to differ. Here, it is the surprise of decimal numbers (see
[numbers](nombres.md)).

### assert.deepStrictEqual for arrays and objects

Two arrays that contain the same values are **not** the same array: `===` compares
addresses, not content. To compare the content, use `assert.deepStrictEqual`
("deeply equal").

```javascript
import assert from "node:assert/strict";

function evens(numbers) {
  return numbers.filter((n) => n % 2 === 0);
}

assert.deepStrictEqual(evens([1, 2, 3, 4]), [2, 4]);
assert.deepStrictEqual({ name: "Alice", age: 30 }, { age: 30, name: "Alice" });
console.log("Same content.");
```

```sortie
Same content.
```

The order of an object's properties does not matter; the order of an array's elements
does. If it fails, the message shows the difference line by line:

```text
AssertionError [ERR_ASSERTION]: Expected values to be strictly deep-equal:
+ actual - expected

  {
+   age: 30,
-   age: 31,
    name: 'Alice'
  }
```

### assert.throws to check an error

Good code refuses bad values. `assert.throws` checks that a function really
**throws** an error. Give it a function to call, not its result: hence the arrow
`() => …`.

```javascript
import assert from "node:assert/strict";

function checkAge(age) {
  if (age < 0) {
    throw new RangeError("negative age");
  }
  return age;
}

assert.throws(() => checkAge(-1), RangeError);
assert.throws(() => checkAge(-1), { message: "negative age" });
assert.strictEqual(checkAge(12), 12);
console.log("The errors are thrown as expected.");
```

```sortie
The errors are thrown as expected.
```

The second argument describes the expected error: a class (`RangeError`), a regular
expression (`/negative/`) or an object with the properties to check.

### Other checks

| Check | Passes if… |
| --- | --- |
| `assert.ok(value)` | the value is true (*truthy*) |
| `assert.strictEqual(a, b)` | `a === b` |
| `assert.notStrictEqual(a, b)` | `a !== b` |
| `assert.deepStrictEqual(a, b)` | same content (arrays, objects) |
| `assert.throws(() => f())` | `f()` throws an error |
| `assert.match(text, /pattern/)` | the text matches the pattern |

## The node:test test runner

A series of `assert` calls, one after the other, is fine. But at the first failure,
everything stops, and you learn nothing about the following checks. The `node:test`
module puts each check in a **test** that has a name. It runs all the tests, even if
one fails, then shows a summary.

### A first test with test()

`test(name, function)` declares a test. The name describes what the test checks.

```javascript
import { test } from "node:test";
import assert from "node:assert/strict";

function sum(a, b) {
  return a + b;
}

test("adds two numbers", () => {
  assert.strictEqual(sum(2, 3), 5);
});

test("works with negative numbers", () => {
  assert.strictEqual(sum(-2, -3), -5);
});
```

Run the file like any other program:

```sh
node sum.test.js
```

In a terminal, Node shows a summary like this one:

```text
✔ adds two numbers (0.81ms)
✔ works with negative numbers (0.22ms)
ℹ tests 2
ℹ suites 0
ℹ pass 2
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 52.17
```

- `✔`: the test passes; `✖`: it fails.
- The number in parentheses is how long the test took: it changes on every run.
- `pass` counts the tests that passed, `fail` the failures.

> **Note:** when the output does not go to a terminal (to a file, for example),
> Node uses another format, called TAP: `ok 1 - adds two numbers`. The meaning is
> the same.

### Grouping tests with describe()

`describe(name, function)` groups the tests that are about the same thing. It is a
chapter; each `test` is a paragraph.

```javascript
import { describe, test } from "node:test";
import assert from "node:assert/strict";

function capitalize(word) {
  return word.charAt(0).toUpperCase() + word.slice(1);
}

describe("capitalize", () => {
  test("puts the first letter in uppercase", () => {
    assert.strictEqual(capitalize("alice"), "Alice");
  });

  test("does not change an empty word", () => {
    assert.strictEqual(capitalize(""), "");
  });
});
```

```text
▶ capitalize
  ✔ puts the first letter in uppercase (0.62ms)
  ✔ does not change an empty word (0.15ms)
✔ capitalize (1.21ms)
ℹ tests 2
ℹ suites 1
ℹ pass 2
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 49.88
```

### Running all the tests with node --test

Put your tests in files whose names end with `.test.js`, or in a `test/` folder. The
`node --test` command finds them and runs them all:

```sh
node --test
```

In a project, you put it in the `test` script of `package.json`, so you can simply
type `npm test` (see [organizing a project](projet.md)).

### Reading a failing test

Here is a `sum` function that gets it wrong: it subtracts instead of adding.

```javascript norun
function sum(a, b) {
  return a - b;
}

test("adds two numbers", () => {
  assert.strictEqual(sum(2, 3), 5);
});
```

```text
✖ adds two numbers (1.49ms)
ℹ tests 1
ℹ suites 0
ℹ pass 0
ℹ fail 1
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 51.02

✖ failing tests:

test at sum.test.js:8:1
✖ adds two numbers (1.49ms)
  AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:

  -1 !== 5

      at TestContext.<anonymous> (file:///home/alice/project/sum.test.js:9:10)
```

The summary gives you everything: the name of the test, its position
(`sum.test.js:8`), the value you got (`-1`) and the expected value (`5`).

### An asynchronous test

If the function you test returns a promise, write an `async` test and use `await`
(see [asynchronous code](asynchrone.md)). The test waits for the promise to finish.

```javascript
import { test } from "node:test";
import assert from "node:assert/strict";

async function loadScore() {
  return 42;
}

test("loads the score", async () => {
  const score = await loadScore();
  assert.strictEqual(score, 42);
});
```

## Vitest and Jest

In big projects, people often use **Vitest** or **Jest**. They are
[packages](../debuter/glossaire.md#package) you install with npm. Their syntax looks a
lot like `node:test`: `describe`, `test` (or `it`) and `expect` instead of
`assert`.

```javascript norun
import { describe, test, expect } from "vitest";
import { sum } from "../src/sum.js";

describe("sum", () => {
  test("adds two numbers", () => {
    expect(sum(2, 3)).toBe(5);
  });
});
```

```sh
npm install --save-dev vitest
npx vitest run
```

`expect(actual).toBe(expected)` corresponds to `assert.strictEqual`, and `toEqual` to
`assert.deepStrictEqual`. Start with `node:test`: nothing to install, and the ideas
are the same.

## What should you test?

There is no need to test every possible value. Choose a few well-placed cases:

| Case | Example for `average(grades)` |
| --- | --- |
| The normal case | `[10, 12, 14]` gives `12` |
| The limits | a single grade, an empty list |
| The odd values | decimal grades, zeros |
| The errors | a negative grade must be refused |

```javascript
import assert from "node:assert/strict";

function average(grades) {
  if (grades.length === 0) return 0;
  if (grades.some((n) => n < 0)) throw new RangeError("negative grade");
  const sum = grades.reduce((total, n) => total + n, 0);
  return sum / grades.length;
}

assert.strictEqual(average([10, 12, 14]), 12);
assert.strictEqual(average([15]), 15);
assert.strictEqual(average([]), 0);
assert.strictEqual(average([0, 0]), 0);
assert.throws(() => average([10, -2]), RangeError);
console.log("5 cases checked.");
```

```sortie
5 cases checked.
```

> **Tip:** when you find a [bug](../debuter/glossaire.md#bug), first write a test
> that shows it, then fix it. The bug will never come back without you knowing.

## Writing the test first

A common method is to write the test **before** the code. You move forward in three
steps, always the same:

1. **Red**: write a test for a function that does not exist yet. It fails.
2. **Green**: write the simplest code that makes the test pass.
3. **Improve**: make the code clearer, running the tests again each time.

Example: an `isPalindrome` function that tells whether a word reads the same both ways.
First the tests, which describe what you want:

```javascript norun
test("kayak is a palindrome", () => {
  assert.strictEqual(isPalindrome("kayak"), true);
});

test("ignores uppercase letters", () => {
  assert.strictEqual(isPalindrome("Kayak"), true);
});

test("javascript is not a palindrome", () => {
  assert.strictEqual(isPalindrome("javascript"), false);
});
```

Run as they are, they fail with `ReferenceError: isPalindrome is not defined`: that is
the red step. Then the code that makes them pass, the green step:

```javascript
import { test } from "node:test";
import assert from "node:assert/strict";

function isPalindrome(word) {
  const lowercase = word.toLowerCase();
  const reversed = [...lowercase].reverse().join("");
  return lowercase === reversed;
}

test("kayak is a palindrome", () => {
  assert.strictEqual(isPalindrome("kayak"), true);
});

test("ignores uppercase letters", () => {
  assert.strictEqual(isPalindrome("Kayak"), true);
});

test("javascript is not a palindrome", () => {
  assert.strictEqual(isPalindrome("javascript"), false);
});
```

Writing the test first forces you to think about **what** the function must do,
before thinking about **how** to do it.

## Common mistakes

### Giving assert.throws the result instead of a function

```javascript norun
import assert from "node:assert/strict";

function checkAge(age) {
  if (age < 0) throw new RangeError("negative age");
  return age;
}

assert.throws(checkAge(-1), RangeError);
```

```text
file:///home/alice/project/main.mjs:4
  if (age < 0) throw new RangeError("negative age");
                     ^

RangeError: negative age
    at checkAge (file:///home/alice/project/main.mjs:4:22)
    at file:///home/alice/project/main.mjs:8:15
```

`checkAge(-1)` is called **before** `assert.throws`: the error goes off too early and
stops the program. Pass a function: `assert.throws(() => checkAge(-1), RangeError)`.

### Forgetting to import test

```javascript norun
import assert from "node:assert/strict";

test("double", () => {
  assert.strictEqual(2 * 2, 4);
});
```

```text
file:///home/alice/project/main.mjs:3
test("double", () => {
^

ReferenceError: test is not defined
    at file:///home/alice/project/main.mjs:3:1
```

Unlike Jest, `test` does not exist on its own. Add
`import { test } from "node:test";` at the top of the file.

### Comparing arrays with strictEqual

```javascript norun
import assert from "node:assert/strict";

assert.strictEqual([1, 2], [1, 2]);
```

```text
AssertionError [ERR_ASSERTION]: Values have same structure but are not reference-equal:

[
  1,
  2
]

    at file:///home/alice/project/main.mjs:3:8
```

The two arrays have the same content, but they are two different objects. To compare
the content, use `assert.deepStrictEqual([1, 2], [1, 2])`.

## Test yourself

**Question 1:** what is the difference between `console.assert` and `assert.strictEqual`?

> **Answer**
>
> `console.assert` shows a message if the condition is false, but the program
> continues. `assert.strictEqual` throws an `AssertionError` that stops the program (or
> marks the test as failed with `node:test`).

**Question 2:** why does this test fail, and how do you fix it?
`assert.strictEqual({ x: 1 }, { x: 1 })`

> **Answer**
>
> They are two different objects: `===` compares objects by address, not by
> content. Use `assert.deepStrictEqual({ x: 1 }, { x: 1 })`.

**Question 3:** write a check that makes sure `JSON.parse("{oops")` throws a
`SyntaxError`.

> **Answer**
>
> ```javascript
> import assert from "node:assert/strict";
>
> assert.throws(() => JSON.parse("{oops"), SyntaxError);
> console.log("SyntaxError thrown as expected.");
> ```
>
> ```sortie
> SyntaxError thrown as expected.
> ```

## Summary

- A test checks that a known input gives the expected output, with no one stepping in.
- `import assert from "node:assert/strict"` gives you `strictEqual`, `deepStrictEqual`
  and `throws`; a failure throws an `AssertionError`.
- `test()` and `describe()` from `node:test` name and group tests; `node --test` runs
  them all.
- Test the normal case, the limits and the errors; write a test for every bug.
- Red, green, improve: writing the test first makes clear what the code must do.

Next: [debugging in JavaScript](debogage.md). Compare with
[tests in Ruby](../ruby/tests.md).
