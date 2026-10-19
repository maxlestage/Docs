# Handling errors

This page explains how to read a Node error message, how to catch an error with `try` and `catch`, and how to throw your own errors with `throw`.

> **Goals:**
>
> - you will be able to read an error message and its stack trace;
> - you will be able to catch an error with `try`, `catch` and `finally`;
> - you will be able to throw an error with `throw new Error(...)`;
> - you will be able to create your own error classes and chain a cause;
> - you will be able to check the data you receive before using it.

## An error stops the program

When JavaScript runs into a problem it does not know how to solve, it **throws** an
[exception](../debuter/glossaire.md#exception): an object that describes the
[error](../debuter/glossaire.md#error). "Throwing" means: sounding the alarm. If nobody
takes care of it, the program stops dead.

Here is a program that calculates a student's average… but we forgot the grades.

```javascript norun
function average(grades) {
  const total = grades.reduce((a, b) => a + b, 0);
  return total / grades.length;
}

function reportCard(student) {
  return `${student.name}: ${average(student.grades)}`;
}

console.log(reportCard({ name: "Lea" }));
```

```text
file:///home/vous/projet/main.mjs:2
  const total = grades.reduce((a, b) => a + b, 0);
                       ^

TypeError: Cannot read properties of undefined (reading 'reduce')
    at average (file:///home/vous/projet/main.mjs:2:24)
    at reportCard (file:///home/vous/projet/main.mjs:7:30)
    at file:///home/vous/projet/main.mjs:10:13
    at ModuleJob.run (node:internal/modules/esm/module_job:343:25)
    at async onImport.tracePromise.__proto__ (node:internal/modules/esm/loader:665:26)
    at async asyncRunEntryPointWithESMLoader (node:internal/modules/run_main:117:5)

Node.js v22.22.0
```

### Reading the message

This block of text looks scary, but you can read it in four pieces:

1. **Where?** The first line gives the file and the line: `main.mjs:2`. Node copies the
   faulty line and puts a `^` under the exact spot.
2. **What?** `TypeError` is the **type** of the error, its family name.
3. **Why?** The message: "cannot read properties of `undefined` (reading `reduce`)". In
   other words: `grades` is `undefined`, and `undefined` has no `reduce` method.
4. **How did we get there?** The `at ...` lines form the **stack trace**: the path the
   program followed, from the deepest function up to the main program.

You read the trace from top to bottom: the error is in `average` (line 2), called by
`reportCard` (line 7), called by the program (line 10). The `node:internal` lines come
from Node itself: ignore them and look for **your** files.

Here, the real culprit is line 10: the student has no `grades` property.

## try and catch

`try` and `catch` form a safety net, like the one under a trapeze artist: if an error
falls, it is caught, and the show goes on.

- The risky code goes in the `try` block.
- The backup code goes in the `catch` block. It receives the error as a parameter.

```javascript
const texts = ['{"name": "Lea"}', "{name: Lea}"];

for (const text of texts) {
  try {
    const data = JSON.parse(text);
    console.log("Read:", data.name);
  } catch (error) {
    console.log("Unreadable:", error.message);
  }
}
console.log("The program goes on.");
```

```sortie
Read: Lea
Unreadable: Expected property name or '}' in JSON at position 1 (line 1 column 2)
The program goes on.
```

As soon as an error happens in `try`, JavaScript **immediately** jumps to the `catch`:
the line `console.log("Read:", ...)` is not run for the second text.

An error object has three useful properties:

| Property | Contents |
|----------|----------|
| `name` | the type of the error: `"SyntaxError"` |
| `message` | the description |
| `stack` | the full stack trace, as text |

> **Tip:** if you do not need the error, you can write `catch { ... }` without
> parentheses.

## finally

The `finally` block **always** runs: after `try` if everything goes well, after `catch`
if there was an error. It is used to tidy up, to close, to put things back in order.

```javascript
function load(text) {
  console.log("Opening");
  try {
    return JSON.parse(text);
  } catch (error) {
    console.log("Problem:", error.name);
    return null;
  } finally {
    console.log("Closing");
  }
}

console.log(load("[1, 2]"));
console.log(load("[1, 2"));
```

```sortie
Opening
Closing
[ 1, 2 ]
Opening
Problem: SyntaxError
Closing
null
```

Even a `return` in `try` does not stop `finally` from running.

## Throwing an error with throw

`throw` lets you throw an error yourself, for example to refuse an impossible value.
Always throw an `Error` object, with a clear message.

```javascript
function withdraw(balance, amount) {
  if (amount > balance) {
    throw new Error("insufficient balance");
  }
  return balance - amount;
}

console.log(withdraw(100, 30));

try {
  withdraw(100, 500);
} catch (error) {
  console.log(`Refused: ${error.message}`);
}
```

```sortie
70
Refused: insufficient balance
```

`throw` stops the function right away, like `return`. The error then goes up the call
stack to the first `catch` it meets. If there is none, the program stops with the
message, like at the start of this page.

## Types of errors

JavaScript has several built-in error classes. The name puts you on the right track.

| Type | When? | Example message |
|------|-------|-----------------|
| `TypeError` | wrong kind of value: property of `undefined`, calling a non-function | `f is not a function` |
| `ReferenceError` | unknown variable (often a typo) | `unknown is not defined` |
| `RangeError` | number outside the allowed limits | `Invalid array length` |
| `SyntaxError` | badly written code or JSON | `missing ) after argument list` |

You can throw these types yourself: `throw new RangeError("grade between 0 and 20")`.
Check the type of a caught error with `instanceof`.

```javascript
const attempts = [
  () => null.length,
  () => unknownVariable,
  () => "ab".repeat(-1),
  () => JSON.parse("{"),
];

for (const attempt of attempts) {
  try {
    attempt();
  } catch (error) {
    const kind = error instanceof TypeError ? "type" : "other";
    console.log(`${error.name} (${kind}): ${error.message}`);
  }
}
```

```sortie
TypeError (type): Cannot read properties of null (reading 'length')
ReferenceError (other): unknownVariable is not defined
RangeError (other): Invalid count value: -1
SyntaxError (other): Expected property name or '}' in JSON at position 1 (line 1 column 2)
```

> **Warning:** a `SyntaxError` in **your code** (a forgotten parenthesis) cannot be
> caught: Node refuses the whole file before running it. Only `JSON.parse` throws a
> `SyntaxError` that you can catch.

## Your own error classes

An error is an object like any other. To tell your errors apart, create a class that
inherits from `Error` (see [inheritance](heritage.md)). Give it a `name`: it will show
up in the messages.

```javascript
class ValidationError extends Error {
  constructor(message, field) {
    super(message);
    this.name = "ValidationError";
    this.field = field;
  }
}

function signUp(user) {
  if (!user.email.includes("@")) {
    throw new ValidationError("invalid address", "email");
  }
  return "Signed up!";
}

try {
  signUp({ email: "ada.example.com" });
} catch (error) {
  if (error instanceof ValidationError) {
    console.log(`Field ${error.field}: ${error.message}`);
    console.log(String(error));
  } else {
    throw error;
  }
}
```

```sortie
Field email: invalid address
ValidationError: invalid address
```

Notice the `else { throw error; }`: you only handle the errors you know, and you
**rethrow** the others. That way, a real bug is not silently hidden.

## Chaining a cause with error.cause

Often, a technical error ("badly formed JSON") hides a problem that means more to the
user ("the configuration is unreadable"). You then throw a new error, keeping the old
one in the `cause` option.

```javascript
function readConfiguration(text) {
  try {
    return JSON.parse(text);
  } catch (error) {
    throw new Error("Unreadable configuration", { cause: error });
  }
}

try {
  readConfiguration("{oops");
} catch (error) {
  console.log(error.message);
  console.log("Cause:", error.cause.name);
}
```

```sortie
Unreadable configuration
Cause: SyntaxError
```

If the error is not caught, Node displays both, with the cause under `[cause]`: you
keep the whole story.

## Checking the data you receive

The best error is the one you detect early. When a function receives data from the
outside (a form, a file), check it **at the start** and clearly refuse what is wrong.
It is like checking tickets when boarding the train, rather than in the middle of the
trip.

```javascript
function readGrade(text) {
  const grade = Number(text);
  if (text.trim() === "" || Number.isNaN(grade)) {
    throw new TypeError(`"${text}" is not a number`);
  }
  if (grade < 0 || grade > 20) {
    throw new RangeError(`${grade} is not between 0 and 20`);
  }
  return grade;
}

for (const input of ["15", "twelve", "25", ""]) {
  try {
    console.log("Grade:", readGrade(input));
  } catch (error) {
    console.log(`${error.name}: ${error.message}`);
  }
}
```

```sortie
Grade: 15
TypeError: "twelve" is not a number
RangeError: 25 is not between 0 and 20
TypeError: "" is not a number
```

> **Remember:** `Number("twelve")` does not throw an error: it returns `NaN`, without a
> word. It is up to you to check it with `Number.isNaN`. Likewise, `Number("")` is `0`:
> hence the test on the empty text.

## Common mistakes

### Throwing a text instead of an Error

```javascript norun
try {
  throw "file not found";
} catch (error) {
  console.log(error.message);
}
```

```text
undefined
```

No error message, but a wrong result: a text has no `message` property, and no stack
trace. Always throw an object: `throw new Error("file not found")`.

### Using a variable declared in try

```javascript norun
try {
  const age = Number.parseInt("42");
} catch (e) {
  console.log("Error");
}
console.log(age + 1);
```

```text
file:///home/vous/projet/main.mjs:6
console.log(age + 1);
            ^

ReferenceError: age is not defined
    at file:///home/vous/projet/main.mjs:6:13
```

The `try` block has its own [scope](closures.md): `age` disappears at the closing curly
brace. Declare `let age;` before the `try`, or use `age` inside the block.

### Believing that try catches a syntax mistake

```javascript norun
console.log("start");
console.log("hello";
```

```text
file:///home/vous/projet/main.mjs:2
console.log("hello";
            ^^^^^^^

SyntaxError: missing ) after argument list
```

Even `start` is not displayed: Node reads the whole file before running it, and refuses
a badly written file. No `try` can catch it. Fix the code: here, the closing
parenthesis is missing.

## Test yourself

**Question 1:** what does this code display?

```javascript norun
try {
  console.log("A");
  throw new Error("failure");
  console.log("B");
} catch {
  console.log("C");
} finally {
  console.log("D");
}
```

> **Answer**
>
> `A`, `C`, then `D`. `B` is never displayed: after `throw`, JavaScript jumps straight
> to the `catch`. `finally` always runs.

**Question 2:** in a stack trace, which line should you look at first?

> **Answer**
>
> The first `at ...` line that mentions **your** file. The `node:internal` lines belong
> to Node.

**Question 3:** write a function `divide(a, b)` that throws a `RangeError` if `b` is
`0`, and call it inside a `try`.

> **Answer**
>
> ```javascript
> function divide(a, b) {
>   if (b === 0) {
>     throw new RangeError("division by zero");
>   }
>   return a / b;
> }
>
> try {
>   console.log(divide(10, 2));
>   console.log(divide(1, 0));
> } catch (error) {
>   console.log(`${error.name}: ${error.message}`);
> }
> ```
>
> ```sortie
> 5
> RangeError: division by zero
> ```

## Summary

- An error message gives the file, the line, the type and the stack trace: look for the
  first line of your code.
- `try` / `catch` catch an error; `finally` always runs.
- `throw new Error("...")` throws an error; it goes up to the first `catch`.
- Your errors inherit from `Error`; `{ cause }` keeps the original error.
- Check data at the start of a function, and refuse it with a clear message.

Next: [modules and npm](modules.md). Compare with
[errors in Ruby](../ruby/erreurs.md).
