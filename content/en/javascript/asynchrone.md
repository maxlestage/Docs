# Asynchronous programming

This page explains how JavaScript does several things "at the same time" without waiting idly: the event loop, callbacks, promises and `async`/`await`.

> **Goals:**
>
> - you will understand the principle of the event loop;
> - you will be able to predict the display order with `setTimeout`;
> - you will be able to create and use a promise with `then` and `catch`;
> - you will be able to write readable asynchronous code with `async` and `await`;
> - you will be able to start several tasks with `Promise.all`, `allSettled` and `race`.

## Waiting without blocking

Some operations are **slow**: downloading a web page, reading a big file, querying a
database. If the program stayed frozen during that time, nothing else could happen: a
web page would no longer respond to clicks.

JavaScript does **only one thing at a time**. But it never sits there stuck waiting: it
starts the slow operation, moves on, and comes back when the result is ready. This is
[asynchronous](../debuter/glossaire.md#async) programming.

## The event loop

Imagine a single waiter in a restaurant. He takes the order of table 1, passes it on to
the kitchen, and **does not wait** in front of the stoves: he goes to take the order of
table 2. When the kitchen rings, he brings the dish.

JavaScript works the same way:

1. it runs the code of the program, line after line, to the end;
2. meanwhile, the slow operations (timers, network, files) happen "in the kitchen";
3. when an operation is finished, what comes next is placed in a **queue**;
4. as soon as it is free, JavaScript takes the next task from the queue, and starts
   again.

This endless round is called the **event loop**. The program ends when there is nothing
left to run or to wait for.

## setTimeout: scheduling for later

`setTimeout(function, delay)` asks to run a function **later**, after a delay in
milliseconds (1,000 ms = 1 second). The function you give is called a **callback**:
you will be called back.

```javascript
console.log("1. I order");

setTimeout(() => {
  console.log("3. The dish arrives");
}, 100);

console.log("2. I chat while waiting");
```

```sortie
1. I order
2. I chat while waiting
3. The dish arrives
```

`setTimeout` does not block: it notes the request, and the program continues right away.

### Even with a delay of 0

A callback always waits until the current code is **completely** finished, even with a
delay of 0: it goes through the queue.

```javascript
setTimeout(() => console.log("C (delay 200)"), 200);
setTimeout(() => console.log("B (delay 0)"), 0);
setTimeout(() => console.log("D (delay 300)"), 300);
console.log("A (right away)");
```

```sortie
A (right away)
B (delay 0)
C (delay 200)
D (delay 300)
```

The order in the code does not matter: the delay decides. And "normal" code always goes
first.

## Callbacks and their limits

For a long time, every slow operation took a callback: "when you are done, call this
function". Let's simulate a slow database.

```javascript
function loadUser(id, callback) {
  setTimeout(() => callback({ id, name: "Ada" }), 50);
}

function loadOrders(user, callback) {
  setTimeout(() => callback(["book", "pen"]), 50);
}

loadUser(1, (user) => {
  console.log("User:", user.name);
  loadOrders(user, (orders) => {
    console.log("Orders:", orders.join(", "));
  });
});
```

```sortie
User: Ada
Orders: book, pen
```

It works. But each step adds a level of indentation. With five steps and error handling
at each level, the code becomes an unreadable "pyramid", called **callback hell**.
Promises solve this problem.

## Promises

A **promise** (`Promise`) is an object that represents a result **not yet available**.
It is the ticket you get at the counter: for now, no sandwich, but the ticket promises it
will come… or that you will be told why it won't.

A promise is in one of three states:

| State | Meaning |
|------|------|
| *pending* | the result is not there yet |
| *fulfilled* | the result has arrived |
| *rejected* | an error occurred |

### Creating a promise

You create a promise with `new Promise`. You give it a function that receives two tools:
`resolve` (fulfill the promise, with a result) and `reject` (break it, with an error).

```javascript
function rollDie(value) {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      if (value >= 1 && value <= 6) {
        resolve(value);
      } else {
        reject(new Error(`impossible face: ${value}`));
      }
    }, 50);
  });
}

console.log(rollDie(4));
```

```sortie
Promise { <pending> }
```

Right after it is created, the promise is pending: the die is still rolling.

### then and catch

`.then(function)` says what to do **when** the promise is fulfilled: the function
receives the result. `.catch(function)` says what to do if it is rejected: the function
receives the error. `.finally(function)` runs in both cases. With the same `rollDie`
function:

```javascript norun
rollDie(4)
  .then((face) => console.log("Result:", face))
  .catch((error) => console.log("Error:", error.message));

rollDie(9)
  .then((face) => console.log("Result:", face))
  .catch((error) => console.log("Error:", error.message))
  .finally(() => console.log("Roll finished"));
```

```text
Result: 4
Error: impossible face: 9
Roll finished
```

Each `then` returns a new promise: you can **chain** them instead of nesting them. The
final `catch` catches an error that happened at any step.

```javascript
const double = (n) => new Promise((resolve) => setTimeout(() => resolve(n * 2), 20));

double(5)
  .then((n) => double(n))
  .then((n) => double(n))
  .then((n) => console.log("Final result:", n))
  .catch((error) => console.log(error.message));
```

```sortie
Final result: 40
```

## async and await

`async` and `await` let you write asynchronous code **like normal code**, from top to
bottom. It is the recommended way today.

- `async` in front of a function makes it asynchronous: it always returns a promise.
- `await` in front of a promise **pauses the function** until the result, without
  blocking the rest of the program.

Here is the callback example, rewritten:

```javascript
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function loadUser(id) {
  await wait(50);
  return { id, name: "Ada" };
}

async function loadOrders(user) {
  await wait(50);
  return ["book", "pen"];
}

async function showOrders() {
  const user = await loadUser(1);
  console.log("User:", user.name);
  const orders = await loadOrders(user);
  console.log("Orders:", orders.join(", "));
}

await showOrders();
console.log("Done");
```

```sortie
User: Ada
Orders: book, pen
Done
```

No more pyramid: each step is one line. In a module (a `.mjs` file, or
`"type": "module"`), you can also write `await` **directly** at the top level of the
file, as on the second-to-last line.

> **Remember:** `await` can only be used inside an `async` function, or at the top level
> of a module.

## Handling errors with try and catch

With `await`, a rejected promise **throws an error**, like `throw`. So you catch it with
`try` and `catch`, exactly as in [handling errors](erreurs.md).

```javascript
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function loadProfile(name) {
  await wait(30);
  if (name === "") {
    throw new Error("empty name");
  }
  return { name, level: 3 };
}

async function show(name) {
  try {
    const profile = await loadProfile(name);
    console.log(`${profile.name} is at level ${profile.level}`);
  } catch (error) {
    console.log("Impossible:", error.message);
  } finally {
    console.log("--");
  }
}

await show("Ada");
await show("");
```

```sortie
Ada is at level 3
--
Impossible: empty name
--
```

## Several tasks at once

With `await` one after the other, tasks run in sequence: 3 tasks of 100 ms take 300 ms.
If they do not depend on each other, you can start them **all at the same time**.

### Promise.all: all or nothing

`Promise.all(list)` waits until **all** the promises are fulfilled, and gives the list of
results, **in the order of the list** (not in the order they arrive). If a single one is
rejected, everything is rejected.

```javascript
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function download(name, ms) {
  await wait(ms);
  console.log(`  ${name} received`);
  return `${name} (${ms} ms)`;
}

const results = await Promise.all([
  download("image", 150),
  download("text", 50),
  download("sound", 100),
]);
console.log(results);
```

```sortie
  text received
  sound received
  image received
[ 'image (150 ms)', 'text (50 ms)', 'sound (100 ms)' ]
```

The three downloads happen in parallel: the total takes about 150 ms, the time of the
slowest one.

### Promise.allSettled: a report for each one

`Promise.allSettled(list)` waits until all of them are finished, fulfilled **or**
rejected, and gives a report for each one. An error does not stop the others.

```javascript
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function check(site) {
  await wait(20);
  if (site.includes("down")) {
    throw new Error(`${site} is not responding`);
  }
  return `${site} OK`;
}

const reports = await Promise.allSettled([
  check("example.com"),
  check("down.com"),
  check("demo.com"),
]);

for (const report of reports) {
  if (report.status === "fulfilled") {
    console.log("✓", report.value);
  } else {
    console.log("✗", report.reason.message);
  }
}
```

```sortie
✓ example.com OK
✗ down.com is not responding
✓ demo.com OK
```

### Promise.race: first come, first served

`Promise.race(list)` gives the result of the **first** promise to finish. It is often
used to set a time limit.

```javascript
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function serverResponse(ms) {
  await wait(ms);
  return `response in ${ms} ms`;
}

async function timeout(ms) {
  await wait(ms);
  throw new Error(`too long (more than ${ms} ms)`);
}

for (const duration of [50, 300]) {
  try {
    const response = await Promise.race([serverResponse(duration), timeout(150)]);
    console.log(response);
  } catch (error) {
    console.log("Giving up:", error.message);
  }
}
```

```sortie
response in 50 ms
Giving up: too long (more than 150 ms)
```

| Function | Waits for | Result | If an error |
|----------|--------|----------|---------------|
| `Promise.all` | all | the list of values | everything is rejected |
| `Promise.allSettled` | all | one report per promise | noted in the report |
| `Promise.race` | the first | its value | if the first one fails |

## A word about fetch

`fetch(address)` downloads a resource from the web. It is the most common example of an
asynchronous operation: it returns a promise. It is available in Node and in all
browsers.

```javascript norun
try {
  const response = await fetch("https://registry.npmjs.org/slugify/1.6.6");
  if (!response.ok) {
    throw new Error(`HTTP ${response.status}`);
  }
  const pkg = await response.json();
  console.log(`${pkg.name} ${pkg.version}: ${pkg.description}`);
} catch (error) {
  console.log("Failed:", error.message);
}
```

```text
slugify 1.6.6: Slugifies a String
```

This code asks npm for the record of the `slugify` package (see [modules and npm](modules.md)).
If the server answers with an error, `response.ok` is `false`: with a version that does
not exist, the program displays `Failed: HTTP 404`.

Two `await`s: one for the server's response, one to read its content as
[JSON](../debuter/glossaire.md#json). The chapter on [the web](web.md) goes further.

## Common mistakes

### Forgetting await

```javascript norun
async function loadName() {
  return "Ada";
}
const name = loadName();
console.log(`Hello ${name}`);
console.log(name);
```

```text
Hello [object Promise]
Promise { 'Ada' }
```

No error, but a wrong result: an `async` function returns a **promise**, not the value.
Write `const name = await loadName();`.

### Using await outside an async function

```javascript norun
function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
function start() {
  await wait(100);
  console.log("Ready");
}
start();
```

```text
file:///home/vous/projet/main.mjs:5
  await wait(100);
  ^^^^^

SyntaxError: Unexpected reserved word
```

"Unexpected reserved word": `await` is not allowed in an ordinary function. Add `async`:
`async function start() { ... }`.

### Forgetting to handle a rejected promise

```javascript norun
function load() {
  return new Promise((resolve, reject) => {
    setTimeout(() => reject(new Error("server unreachable")), 10);
  });
}
load().then((d) => console.log(d));
console.log("Rest of the program");
```

```text
Rest of the program
file:///home/vous/projet/main.mjs:3
    setTimeout(() => reject(new Error("server unreachable")), 10);
                            ^

Error: server unreachable
    at Timeout._onTimeout (file:///home/vous/projet/main.mjs:3:29)
```

A rejected promise that nobody catches stops the program. Add `.catch((error) => ...)` to
the chain, or use `await` inside a `try`/`catch`.

## Test yourself

**Question 1:** in what order are the letters displayed?

```javascript norun
setTimeout(() => console.log("A"), 100);
console.log("B");
setTimeout(() => console.log("C"), 0);
console.log("D");
```

> **Answer**
>
> `B`, `D`, `C`, `A`. Normal code goes first (`B`, `D`). Then come the callbacks, from
> the shortest delay to the longest: `C` (0 ms), then `A` (100 ms).

**Question 2:** what does an `async` function always return?

> **Answer**
>
> A promise. If the function does `return 5`, the promise is fulfilled with `5`; if it
> throws an error, the promise is rejected.

**Question 3:** write an `async` function `slowSum(a, b)` that waits 50 ms then returns
`a + b`, and start two calls in parallel with `Promise.all`.

> **Answer**
>
> ```javascript
> const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
>
> async function slowSum(a, b) {
>   await wait(50);
>   return a + b;
> }
>
> const results = await Promise.all([slowSum(1, 2), slowSum(10, 20)]);
> console.log(results);
> ```
>
> ```sortie
> [ 3, 30 ]
> ```

## Summary

- JavaScript does one thing at a time, but never waits by blocking: the event loop picks
  tasks back up when they are ready.
- `setTimeout` schedules a callback; the current code always goes first.
- A promise represents a future result: `then` for success, `catch` for the error.
- `async`/`await` write asynchronous code from top to bottom; errors are caught with
  `try`/`catch`.
- `Promise.all`, `allSettled` and `race` start several tasks in parallel.

Next: [the DOM](dom.md). Compare with
[concurrency in Ruby](../ruby/concurrence.md).
