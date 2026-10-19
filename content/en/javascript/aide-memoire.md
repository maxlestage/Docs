# Cheat sheet

This page sums up at a glance the JavaScript syntax seen throughout the track, with Node.

## Basics

| Element | Example |
| --- | --- |
| Constant | `const age = 30;` |
| Variable that changes | `let total = 0;` |
| Text | `"Hello"`, `'Hello'` |
| Template | `` `Hello ${name}` `` |
| Booleans | `true`, `false` |
| Absence | `undefined` (no value), `null` (empty on purpose) |
| Comment | `// one line`, `/* several */` |
| Print | `console.log(a, b)` |
| Run | `node main.mjs` |

```javascript
const name = "Alice";
let age = 30;
age += 1;
console.log(`Hello ${name}, ${age} years old`);
console.log(typeof name, typeof age, typeof undefined, typeof null);
```

```sortie
Hello Alice, 31 years old
string number undefined object
```

Details: [first steps](premiers-pas.md) and [types](types.md).

## Numbers

| Need | Syntax |
| --- | --- |
| Operations | `+ - * / ** %` |
| Integer division | `Math.trunc(7 / 2)` |
| Round | `Math.round(x)`, `x.toFixed(2)` (text) |
| Text to number | `Number("12")`, `parseInt("12px")` |
| Not a number | `Number.isNaN(x)` |
| Big integers | `10n ** 20n` (BigInt) |

```javascript
console.log(7 / 2, Math.trunc(7 / 2), 7 % 2, 2 ** 10);
console.log((0.1 + 0.2).toFixed(2), Number("12"), parseInt("12px"));
console.log(Number("abc"), Number.isNaN(Number("abc")));
```

```sortie
3.5 3 1 1024
0.30 12 12
NaN true
```

Details: [numbers](nombres.md).

## Conditions and loops

| Need | Syntax |
| --- | --- |
| Condition | `if (…) { … } else if (…) { … } else { … }` |
| Equality | `===`, `!==` (never `==`) |
| Logic | `&&`, `\|\|`, `!` |
| Ternary | `x > 0 ? "yes" : "no"` |
| Several cases | `switch (x) { case 1: …; break; default: … }` |
| Loops | `for (let i = 0; i < n; i++)`, `while (…)`, `for (const x of list)` |
| Control | `break`, `continue` |

```javascript
for (let n = 1; n <= 5; n++) {
  if (n === 2) continue;
  console.log(n % 2 === 0 ? `${n} even` : `${n} odd`);
}
```

```sortie
1 odd
3 odd
4 even
5 odd
```

Details: [conditions and loops](conditions-boucles.md).

## Functions

| Form | Syntax |
| --- | --- |
| Declaration | `function square(x) { return x * x; }` |
| Arrow | `const square = (x) => x * x;` |
| Default value | `function greet(name = "you") { … }` |
| Rest of the arguments | `function sum(...numbers) { … }` |
| Object parameter | `function create({ name, age }) { … }` |

```javascript
const square = (x) => x * x;
function greet(name = "you") {
  return `Hi ${name}!`;
}
function sum(...numbers) {
  return numbers.reduce((total, n) => total + n, 0);
}
console.log(square(4), greet(), greet("Léa"), sum(1, 2, 3));
```

```sortie
16 Hi you! Hi Léa! 6
```

Details: [functions](fonctions.md).

## Text

| Need | Method |
| --- | --- |
| Length | `s.length` |
| Uppercase | `s.toUpperCase()`, `s.toLowerCase()` |
| Search | `s.includes("a")`, `s.startsWith("a")`, `s.indexOf("a")` |
| Split, join | `s.split(",")`, `list.join(", ")` |
| Piece | `s.slice(0, 3)`, `s.at(-1)` |
| Clean up | `s.trim()` |
| Replace | `s.replaceAll("a", "b")` |
| Align | `s.padStart(5)`, `s.padEnd(5)`, `"-".repeat(10)` |

```javascript
const s = "  Hello the world  ";
const clean = s.trim();
console.log(clean.toUpperCase());
console.log(clean.split(" "), clean.slice(0, 5), clean.at(-1));
console.log(`[${"42".padStart(5)}]`, clean.includes("world"));
```

```sortie
HELLO THE WORLD
[ 'Hello', 'the', 'world' ] Hello d
[   42] true
```

Details: [text](texte.md).

## Arrays

| Need | Syntax |
| --- | --- |
| Create, read | `const t = [1, 2, 3]`, `t[0]`, `t.at(-1)`, `t.length` |
| Add, remove | `t.push(4)`, `t.pop()`, `t.unshift(0)`, `t.shift()` |
| Transform | `t.map((x) => x * 2)` |
| Keep | `t.filter((x) => x > 1)` |
| Reduce | `t.reduce((acc, x) => acc + x, 0)` |
| Search | `t.find(…)`, `t.includes(2)`, `t.some(…)`, `t.every(…)` |
| Sort without changing | `t.toSorted((a, b) => a - b)` |
| Copy, merge | `[...t]`, `[...a, ...b]` |
| Destructure | `const [first, ...rest] = t` |

```javascript
const grades = [12, 8, 15, 10];
console.log(grades.map((g) => g + 1));
console.log(grades.filter((g) => g >= 10));
console.log(grades.reduce((sum, g) => sum + g, 0));
console.log(grades.toSorted((a, b) => b - a), grades);
const [first, ...others] = grades;
console.log(first, others);
```

```sortie
[ 13, 9, 16, 11 ]
[ 12, 15, 10 ]
45
[ 15, 12, 10, 8 ] [ 12, 8, 15, 10 ]
12 [ 8, 15, 10 ]
```

Details: [collections](collections.md).

## Objects, Map and Set

| Need | Syntax |
| --- | --- |
| Create | `const p = { name: "Alice", age: 30 }` |
| Read | `p.name`, `p["name"]`, `p.address?.city` |
| Default value | `p.city ?? "unknown"` |
| Go through | `Object.keys(p)`, `Object.values(p)`, `Object.entries(p)` |
| Copy with a change | `{ ...p, age: 31 }` |
| Destructure | `const { name, age } = p` |
| Dictionary | `new Map()`, `m.set(k, v)`, `m.get(k)`, `m.has(k)` |
| Set without duplicates | `new Set([1, 1, 2])` |

```javascript
const p = { name: "Alice", age: 30 };
const older = { ...p, age: 31 };
const { name, age } = older;
console.log(name, age, p.age, p.city ?? "unknown");
for (const [key, value] of Object.entries(p)) {
  console.log(`${key} = ${value}`);
}
const seen = new Set(["a", "b", "a"]);
const stock = new Map([["apples", 3]]);
console.log(seen.size, stock.get("apples"), stock.has("pears"));
```

```sortie
Alice 31 30 unknown
name = Alice
age = 30
2 3 false
```

Details: [objects](objets.md).

## Classes and inheritance

| Element | Syntax |
| --- | --- |
| Class | `class Animal { constructor(name) { this.name = name; } }` |
| Object | `new Animal("Rex")` |
| Private field | `#secret = 0` |
| Getter | `get name() { … }` |
| Class method | `static create() { … }` |
| Inheritance | `class Dog extends Animal { … }` |
| Parent | `super(name)`, `super.speak()` |
| Check | `rex instanceof Animal` |

```javascript
class Animal {
  constructor(name) {
    this.name = name;
  }
  speak() {
    return `${this.name} makes a noise.`;
  }
}

class Dog extends Animal {
  speak() {
    return `${this.name} barks.`;
  }
}

const animals = [new Animal("Bob"), new Dog("Rex")];
for (const a of animals) console.log(a.speak());
console.log(animals[1] instanceof Animal);
```

```sortie
Bob makes a noise.
Rex barks.
true
```

Details: [inheritance](heritage.md).

## Closures

A function remembers the variables of the place where it was created.

```javascript
function multiplier(factor) {
  return (x) => x * factor;
}
const double = multiplier(2);
const triple = multiplier(3);
console.log(double(5), triple(5));
```

```sortie
10 15
```

Details: [closures](closures.md).

## Iterators and generators

| Element | Syntax |
| --- | --- |
| Go through an iterable | `for (const x of iterable)` |
| Index and value | `for (const [i, x] of t.entries())` |
| Generator | `function* gen() { yield 1; }` |
| Get everything | `[...gen()]`, `Array.from(gen())` |

```javascript
function* countdown(n) {
  while (n > 0) {
    yield n;
    n -= 1;
  }
}
console.log([...countdown(3)]);
for (const [i, letter] of ["a", "b"].entries()) {
  console.log(i, letter);
}
```

```sortie
[ 3, 2, 1 ]
0 a
1 b
```

Details: [iterators](iterateurs.md).

## Errors

| Need | Syntax |
| --- | --- |
| Catch | `try { … } catch (error) { … }` |
| Always run | `finally { … }` |
| Throw | `throw new Error("message")` |
| Common types | `TypeError`, `RangeError`, `SyntaxError`, `ReferenceError` |
| Your own error | `class MyError extends Error { … }` |

```javascript
function checkAge(age) {
  if (age < 0) throw new RangeError("negative age");
  return age;
}

try {
  checkAge(-1);
} catch (error) {
  console.log(`${error.name}: ${error.message}`);
} finally {
  console.log("Check done");
}
```

```sortie
RangeError: negative age
Check done
```

Details: [errors](erreurs.md).

## Modules

| Need | Syntax |
| --- | --- |
| Export | `export function f() {}`, `export const X = 1;` |
| Default export | `export default class Cart {}` |
| Import | `import { f, X } from "./tools.js";` |
| Import the default | `import Cart from "./cart.js";` |
| Node module | `import { readFile } from "node:fs/promises";` |

```javascript norun
// tools.js
export const TAX = 0.2;
export function priceWithTax(net) {
  return net * (1 + TAX);
}

// main.js
import { priceWithTax } from "./tools.js";
console.log(priceWithTax(10));
```

Details: [modules](modules.md).

## Asynchronous code

| Need | Syntax |
| --- | --- |
| Async function | `async function load() { … }` |
| Wait for a promise | `const x = await promise;` |
| Create a promise | `new Promise((resolve, reject) => { … })` |
| Later | `setTimeout(f, 100)` |
| All in parallel | `await Promise.all([p1, p2])` |

```javascript
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function load(name, ms) {
  await wait(ms);
  return `${name} loaded`;
}

console.log("Start");
const results = await Promise.all([load("profile", 30), load("grades", 10)]);
console.log(results);
```

```sortie
Start
[ 'profile loaded', 'grades loaded' ]
```

Details: [asynchronous code](asynchrone.md).

## The DOM, in the browser

| Need | Syntax |
| --- | --- |
| Find an element | `document.querySelector("#title")` |
| Several elements | `document.querySelectorAll(".card")` |
| Change the text | `el.textContent = "Hello"` |
| CSS classes | `el.classList.add("active")`, `toggle`, `remove` |
| Create, add | `document.createElement("li")`, `list.append(li)` |
| React | `button.addEventListener("click", () => { … })` |

```javascript norun
const button = document.querySelector("#count");
let clicks = 0;
button.addEventListener("click", () => {
  clicks += 1;
  button.textContent = `Clicked ${clicks} times`;
});
```

Details: [the DOM](dom.md).

## Files and JSON

| Need | Syntax |
| --- | --- |
| Read | `await readFile("f.txt", "utf8")` |
| Write, append | `await writeFile("f.txt", text)`, `await appendFile(…)` |
| Missing file | `error.code === "ENOENT"` |
| Object to JSON | `JSON.stringify(obj, null, 2)` |
| JSON to object | `JSON.parse(text)` |

```javascript
import { readFile, writeFile } from "node:fs/promises";

const data = { name: "Alice", grades: [12, 15] };
await writeFile("student.json", JSON.stringify(data, null, 2));
const reread = JSON.parse(await readFile("student.json", "utf8"));
console.log(reread.grades);

try {
  await readFile("missing.txt", "utf8");
} catch (error) {
  console.log(error.code);
}
```

```sortie
[ 12, 15 ]
ENOENT
```

Details: [files](fichiers.md) and [formats](formats.md).

## Regular expressions

| Element | Meaning |
| --- | --- |
| `\d`, `\w`, `\s` | a digit, a letter or digit, a space |
| `+`, `*`, `?`, `{2,4}` | one or more, zero or more, optional, from 2 to 4 |
| `^`, `$` | start, end |
| `( )`, `(?<name> )` | group, named group |
| Flags | `g` (all), `i` (ignore case) |
| Test, search | `/re/.test(s)`, `s.match(/re/)`, `s.matchAll(/re/g)` |
| Replace | `s.replace(/re/g, "…")` |

```javascript
const text = "Order 42 on 2025-03-14, order 7 on 2025-04-01";
console.log(/\d+/.test(text));
console.log(text.match(/\d+/g).join(" "));
const date = text.match(/(?<year>\d{4})-(?<month>\d{2})/);
console.log(date.groups.year, date.groups.month);
console.log("a  b   c".replace(/\s+/g, " "));
```

```sortie
true
42 2025 03 14 7 2025 04 01
2025 03
a b c
```

Details: [regular expressions](regex.md).

## Dates

| Need | Syntax |
| --- | --- |
| Now | `new Date()` |
| A precise date | `new Date("2025-03-14T10:30:00Z")` |
| Parts | `d.getFullYear()`, `d.getMonth()` (0 = January), `d.getDate()` |
| ISO format | `d.toISOString()` |
| English format | `d.toLocaleDateString("en-US", { … })` |
| Gap | `d2 - d1` (in milliseconds) |

```javascript
const d = new Date("2025-03-14T10:30:00Z");
console.log(d.toISOString());
console.log(d.getUTCFullYear(), d.getUTCMonth() + 1, d.getUTCDate());
console.log(d.toLocaleDateString("en-US", { dateStyle: "full", timeZone: "UTC" }));
const end = new Date("2025-03-24T10:30:00Z");
console.log((end - d) / (1000 * 60 * 60 * 24), "days");
```

```sortie
2025-03-14T10:30:00.000Z
2025 3 14
Friday, March 14, 2025
10 days
```

Details: [dates](dates.md).

## Command line and web

| Need | Syntax |
| --- | --- |
| Arguments | `process.argv.slice(2)` |
| Environment variable | `process.env.HOME` |
| Quit with a code | `process.exit(1)` |
| Error on screen | `console.error("…")` |
| HTTP request | `const res = await fetch(url); await res.json()` |
| Small server | `http.createServer((req, res) => { … }).listen(3000)` |

```javascript norun
const [name = "unknown"] = process.argv.slice(2);
console.log(`Hello ${name}`);

const response = await fetch("https://api.example.com/students");
if (!response.ok) throw new Error(`HTTP ${response.status}`);
const students = await response.json();
```

Details: [the command line](ligne-de-commande.md) and [the web](web.md).

## Tests

| Need | Syntax |
| --- | --- |
| Import | `import { test, describe } from "node:test";` |
| Assertions | `import assert from "node:assert/strict";` |
| Equality | `assert.strictEqual(actual, expected)` |
| Contents | `assert.deepStrictEqual([1, 2], [1, 2])` |
| Thrown error | `assert.throws(() => f(), RangeError)` |
| Run | `node --test` or `npm test` |

```javascript
import assert from "node:assert/strict";

const double = (x) => x * 2;
assert.strictEqual(double(4), 8);
assert.deepStrictEqual([1, 2].map(double), [2, 4]);
assert.throws(() => JSON.parse("{oops"), SyntaxError);
console.log("3 checks passed");
```

```sortie
3 checks passed
```

Details: [testing your code](tests.md).

## Debugging and style

| Need | Tool |
| --- | --- |
| Names and values | `console.log({ a, b })` |
| List of objects | `console.table(list)` |
| Deep object | `console.dir(obj, { depth: null })` |
| Pause | `debugger;` with the DevTools (`F12`) |
| Pause with Node | `node --inspect-brk main.js` then `chrome://inspect` |
| Style | `const` by default, `===`, early returns, `?.` and `??` |
| Check, format | `npx eslint src/`, `npx prettier --write src/` |

Details: [debugging](debogage.md) and [writing good JavaScript](style.md).

## Project and tools

| Command | Role |
| --- | --- |
| `node main.js` | run a program |
| `node --watch main.js` | rerun on every change |
| `npm init -y` | create `package.json` |
| `npm install name` | add a package (`dependencies`) |
| `npm install --save-dev name` | add a tool (`devDependencies`) |
| `npm install` | install the packages of a downloaded project |
| `npm start`, `npm test`, `npm run name` | run a script |
| `npx tool` | run a tool of the project |

Details: [organizing a project](projet.md). To practice:
[step-by-step projects](projets.md) and [solved exercises](exercices.md). Compare
with [the Ruby cheat sheet](../ruby/aide-memoire.md) and
[the Rust cheat sheet](../rust/aide-memoire.md).
