# Keeping data in the browser

This page shows how a web page keeps data from one visit to the next with `localStorage`, `sessionStorage` and IndexedDB, and what you must never store there.

> **Goals:**
>
> - you will be able to save, read back and erase data with `localStorage`;
> - you will be able to store objects and lists thanks to `JSON.stringify` and `JSON.parse`;
> - you will be able to choose between `localStorage` and `sessionStorage`;
> - you will know the limits of storage and the role of IndexedDB;
> - you will know what you must **never** store in the browser.

## Why keep data

The [variables](../debuter/glossaire.md#variable) of a page are written on a
**whiteboard**: when you reload the page, everything is wiped. The counter goes back to
0, the to-do list is empty.

The browser also offers a **notebook**: what you write in it stays, even after you
close the browser and turn the computer back on.

```text
  Variables (whiteboard)             localStorage (notebook)
  ┌──────────────────────┐           ┌──────────────────────┐
  │ counter = 3          │           │ "counter" → "3"      │
  └──────────────────────┘           └──────────────────────┘
         reload ▼                           reload ▼
  ┌──────────────────────┐           ┌──────────────────────┐
  │ counter = 0 (lost)   │           │ "counter" → "3"      │
  └──────────────────────┘           └──────────────────────┘
```

The chapter [the DOM](dom.md) gave you a preview. Here, we take the time to see it all.

> **Remember:** variables disappear when the page reloads. Browser storage stays.

## localStorage: labelled lockers

`localStorage` looks like a wall of lockers. Each locker has a **label** (the key) and
holds **a text** (the value).

```text
  ┌────────────┬────────────┬────────────┐
  │ "theme"    │ "name"     │ "visits"   │   ← the keys (labels)
  │  "dark"    │  "Léa"     │  "4"       │   ← the values (always text)
  └────────────┴────────────┴────────────┘
```

Four methods are enough:

```javascript norun
localStorage.setItem("theme", "dark");        // store (or replace)
const theme = localStorage.getItem("theme");  // read back: "dark"
localStorage.removeItem("theme");             // empty one locker
localStorage.clear();                         // empty all the lockers
```

- `getItem` returns `null` if the locker does not exist;
- `setItem` on an existing key **replaces** the old value;
- `localStorage.length` gives the number of lockers.

This code only runs in a browser: Node has no `localStorage`. To try it in the
browser, open the console (`F12`, **Console** tab) and type these lines one by one.
The **Application** tab (Chrome) or **Storage** tab (Firefox) shows the lockers.

> **Remember:** `setItem(key, text)` stores, `getItem(key)` reads back (or `null`),
> `removeItem(key)` erases.

## Trying the API in Node

To understand and test without a browser, let's build a **fake** `localStorage`: a
[class](../debuter/glossaire.md#class) that has the same methods. It stores
everything in a `Map` (see [collections](collections.md)).

```javascript
class FakeStorage {
  #lockers = new Map();

  get length() {
    return this.#lockers.size;
  }
  setItem(key, value) {
    this.#lockers.set(String(key), String(value)); // everything becomes text
  }
  getItem(key) {
    return this.#lockers.has(key) ? this.#lockers.get(key) : null;
  }
  removeItem(key) {
    this.#lockers.delete(key);
  }
  clear() {
    this.#lockers.clear();
  }
  key(i) {
    return [...this.#lockers.keys()][i] ?? null;
  }
}

const notebook = new FakeStorage();
notebook.setItem("theme", "light");
notebook.setItem("name", "Léa");
notebook.setItem("theme", "dark");
console.log(notebook.getItem("theme"), notebook.length);
notebook.removeItem("name");
console.log(notebook.getItem("name"), notebook.length);
console.log(notebook.key(0));
```

```sortie
dark 2
null 1
theme
```

Let's follow the content of the notebook line by line:

| Step | Instruction | Content of the notebook | length |
|---|---|---|---|
| 1 | `setItem("theme", "light")` | theme → "light" | 1 |
| 2 | `setItem("name", "Léa")` | theme → "light", name → "Léa" | 2 |
| 3 | `setItem("theme", "dark")` | theme → "dark", name → "Léa" | 2 |
| 4 | `removeItem("name")` | theme → "dark" | 1 |

At step 3, `length` stays at 2: the key `theme` already existed, its value is replaced.

Try it: add `notebook.clear();` then `console.log(notebook.length, notebook.key(0));`.
You get `0 null`.

> **Remember:** the real `localStorage` behaves like this small class: a table of keys
> and texts.

## Everything becomes text

The notebook only keeps [strings](../debuter/glossaire.md#string).
If you store a number, it is converted to text. When it comes back, it is no longer a
number.

```javascript
// same conversion as setItem: String(value)
const stored = String(5);
console.log(stored, typeof stored);
console.log(stored + 1);          // texts glued together!
console.log(Number(stored) + 1);  // the right calculation

console.log(String(true));
console.log(String([1, 2, 3]));
console.log(String({ name: "Léa" }));
```

```sortie
5 string
51
6
true
1,2,3
[object Object]
```

| Value stored | Text kept | Problem |
|---|---|---|
| `5` | `"5"` | `"5" + 1` gives `"51"` |
| `true` | `"true"` | `"true"` is not `true` |
| `[1, 2, 3]` | `"1,2,3"` | it is no longer an array |
| `{ name: "Léa" }` | `"[object Object]"` | the data is **lost** |

For a number, convert it when it comes back with `Number(...)`. For everything else, use
JSON (next section).

> **Remember:** what goes into the notebook comes out as text.

## Storing objects with JSON

[JSON](../debuter/glossaire.md#json) is a way to write an object or a list as text, and
then to read it back. It is like taking a piece of furniture apart to put it in a box,
then putting it back together when it arrives (this is called
[serialization](../debuter/glossaire.md#serialization)).

- `JSON.stringify(value)`: object → text (we pack it in a box);
- `JSON.parse(text)`: text → object (we put it back together).

```javascript
const settings = { theme: "dark", size: 18, sounds: false };

const text = JSON.stringify(settings);
console.log(text, typeof text);

const readBack = JSON.parse(text);
console.log(readBack.size + 2, typeof readBack.sounds);
```

```sortie
{"theme":"dark","size":18,"sounds":false} string
20 boolean
```

Numbers and booleans come back intact. See [JSON and CSV](formats.md) for more
details.

### Two functions, save and load

Write two small functions once, and use them everywhere. `load` receives a **default
value** for when the locker is empty, or damaged.

```javascript
class FakeStorage {
  #lockers = new Map();
  setItem(key, value) { this.#lockers.set(key, String(value)); }
  getItem(key) { return this.#lockers.get(key) ?? null; }
}
const notebook = new FakeStorage();

function save(key, value) {
  notebook.setItem(key, JSON.stringify(value));
}

function load(key, fallback) {
  const text = notebook.getItem(key);
  if (text === null) return fallback;   // empty locker
  try {
    return JSON.parse(text);
  } catch {
    return fallback;                    // damaged text
  }
}

console.log(load("tasks", []));
save("tasks", ["bread", "milk"]);
console.log(load("tasks", []));
notebook.setItem("tasks", "{not json");
console.log(load("tasks", []));
```

```sortie
[]
[ 'bread', 'milk' ]
[]
```

| Call | `getItem` returns | Path taken | Result |
|---|---|---|---|
| 1st `load` | `null` | empty locker | `[]` (default) |
| 2nd `load` | `'["bread","milk"]'` | `JSON.parse` succeeds | `[ 'bread', 'milk' ]` |
| 3rd `load` | `'{not json'` | `JSON.parse` fails, `catch` | `[]` (default) |

In the browser, simply replace `notebook` with `localStorage`.

> **Remember:** `JSON.stringify` when storing, `JSON.parse` when reading back, and
> always a default value.

## An example: counting visits

Let's simulate three visits to a page. On each visit, the variables start from zero
again (a new call of the function), but the notebook is the same.

```javascript
class FakeStorage {
  #lockers = new Map();
  setItem(key, value) { this.#lockers.set(key, String(value)); }
  getItem(key) { return this.#lockers.get(key) ?? null; }
}
const notebook = new FakeStorage();  // survives the visits

function visit() {
  const before = Number(notebook.getItem("visits") ?? "0");
  const visits = before + 1;
  notebook.setItem("visits", visits);
  console.log(visits === 1 ? "Welcome!" : `Visit number ${visits}`);
}

visit();
visit();
visit();
```

```sortie
Welcome!
Visit number 2
Visit number 3
```

| Visit | `getItem("visits")` | before | visits | Notebook after |
|---|---|---|---|---|
| 1 | `null` → `"0"` | 0 | 1 | "1" |
| 2 | `"1"` | 1 | 2 | "2" |
| 3 | `"2"` | 2 | 3 | "3" |

Try it: remove `Number(...)` (keep `notebook.getItem("visits") ?? "0"`). The first
visit prints `Visit number 01`, the third `Visit number 0111`: texts are glued together
instead of added.

> **Remember:** read, calculate, write back. And convert the text before calculating.

## sessionStorage: the notebook of a single tab

`sessionStorage` has **exactly** the same methods. The difference is how long it lives:
it is erased when you close the tab. It is a sticky note stuck on the tab, while
`localStorage` is a notebook put away in the drawer.

```javascript norun
sessionStorage.setItem("step", "2");         // the step of a form being filled in
console.log(sessionStorage.getItem("step")); // "2", even after a reload
```

| | `localStorage` | `sessionStorage` |
|---|---|---|
| Survives a reload | yes | yes |
| Survives closing the tab | yes | **no** |
| Shared between tabs of the same site | yes | no (one notebook per tab) |
| Typical use | theme, language, draft | step of a form, filter |

The two functions `save` and `load` work with either one.

> **Remember:** same API; `sessionStorage` lives as long as a tab, `localStorage`
> stays.

## The limits

The notebook is handy, but small and simple.

- **Size**: about 5 MB per site, depending on the browser. Beyond that, `setItem`
  throws an error.
- **Only text**, hence JSON.
- **One notebook per site**: a site cannot see another site's notebook. "Site" here
  means the protocol, the name and the port: `http://localhost:3000` and
  `http://localhost:5173` have two different notebooks.
- **The user decides**: they can erase everything ("clear browsing data"), and private
  browsing empties everything when it is closed.
- **It is synchronous**: the page waits during the read. With big texts, it can slow
  down.

To estimate the space used, measure the length of the JSON text:

```javascript
const tasks = Array.from({ length: 1000 }, (_, i) => ({ text: `Task ${i}`, done: false }));
const text = JSON.stringify(tasks);
console.log(text.length, "characters");
console.log((text.length / 5_000_000 * 100).toFixed(1), "% of 5 million");
```

```sortie
32891 characters
0.7 % of 5 million
```

A thousand tasks fit easily. Photos do not.

When the notebook is full, the browser throws an error (here in Chrome):

```text
Uncaught DOMException: Failed to execute 'setItem' on 'Storage': Setting the value of 'photos' exceeded the quota.
```

> **Remember:** `localStorage` is fine for small data (settings, drafts, short lists).
> For a lot of data, use IndexedDB.

## IndexedDB: a real little database

IndexedDB is a **database** in the browser (see
[databases](../debuter/bases-de-donnees.md)). It stores **objects** (not just text), in
large quantities, and does not block the page. In exchange, its API takes longer to
write. It is the warehouse, next to the small notebook.

```text
 database "notes"
 └── store "tasks"   (like a table)
      ├── { id: 1, text: "bread", done: false }
      └── { id: 2, text: "milk", done: true }
```

### Opening the database

You open the database by its name and its **version**. The first time (or when the
version goes up), the `upgradeneeded` event lets you create the **stores**.

```javascript norun
function openDatabase() {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open("notes", 1);       // name, version

    request.onupgradeneeded = () => {
      const db = request.result;
      db.createObjectStore("tasks", { keyPath: "id" }); // the key is the id field
    };
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
}
```

IndexedDB lets you know through **events** (`onsuccess`, `onerror`). We wrap it in a
[promise](../debuter/glossaire.md#promise) so that we can write `await` (see
[asynchronous code](asynchrone.md)).

### Adding and reading

Every read or write goes through a **transaction**: a small order that succeeds
completely or not at all.

```javascript norun
function store(db, task) {
  return new Promise((resolve, reject) => {
    const tx = db.transaction("tasks", "readwrite");
    tx.objectStore("tasks").put(task);       // adds or replaces
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error);
  });
}

function read(db, id) {
  return new Promise((resolve, reject) => {
    const request = db.transaction("tasks").objectStore("tasks").get(id);
    request.onsuccess = () => resolve(request.result);  // undefined if missing
    request.onerror = () => reject(request.error);
  });
}

const db = await openDatabase();
await store(db, { id: 1, text: "bread", done: false });
console.log(await read(db, 1));   // { id: 1, text: "bread", done: false }
```

Notice: no `JSON.stringify` here. IndexedDB stores objects as they are.

> **Remember:** IndexedDB = a lot of data, objects, without blocking the page. More
> code: wrap it in a few promise-based functions.

## What you must never store there

Browser storage is a **postcard**, not a safe:

- any script running on the page can read it, including a malicious script (an XSS
  attack, see [security](../debuter/securite.md));
- anyone who uses the computer can open it with `F12`;
- nothing in it is encrypted.

**Never** store there:

- a password, even "temporarily";
- a credit card number;
- a login token that gives access to an account (better: an `HttpOnly` cookie, set by
  the [server](../debuter/glossaire.md#server), which JavaScript cannot read);
- sensitive personal data (health, identity documents).

Store there without worry: the theme, the language, a draft, the state of a game, a
shopping list.

> **Remember:** if you would be embarrassed to see it written on a postcard, do not put
> it in `localStorage`.

## Common mistakes

### Reading an empty locker

```javascript norun
const tasks = JSON.parse(localStorage.getItem("tasks"));
console.log(tasks.length);
```

```text
Uncaught TypeError: Cannot read properties of null (reading 'length')
```

On the first visit, the locker does not exist: `getItem` returns `null`, and
`JSON.parse(null)` gives `null`. Plan a default value:
`JSON.parse(localStorage.getItem("tasks") ?? "[]")`, or the `load` function.

### Storing text that is not JSON

```javascript norun
localStorage.setItem("settings", "{theme: dark}");   // written by hand
const settings = JSON.parse(localStorage.getItem("settings"));
```

```text
Uncaught SyntaxError: Expected property name or '}' in JSON at position 1 (line 1 column 2)
```

In JSON, keys and texts are between double quotes. Do not build the text by hand:
always go through `JSON.stringify`, and surround `JSON.parse` with a `try...catch`.

### Calculating with a number read back

```javascript norun
localStorage.setItem("score", 10);
const score = localStorage.getItem("score") + 5;
console.log(score);
```

```text
105
```

No error, but a wrong result: `"10" + 5` glues the texts together. Write
`Number(localStorage.getItem("score")) + 5`.

## Test yourself

**Question 1:** what does `localStorage.getItem("missing")` return if this locker has
never been filled?

> **Answer**
>
> `null`. You must plan for this case, for example with `?? "[]"` before `JSON.parse`.

**Question 2:** a user closes their tab, then opens the site again. What is left in
`localStorage`? And in `sessionStorage`?

> **Answer**
>
> `localStorage` has kept everything. `sessionStorage` is empty: it only lives as long
> as a tab.

**Question 3:** with the `FakeStorage` (or a `Map`), store the list `[3, 1, 2]` under
the key `"grades"`, read it back, and print its sum.

> **Answer**
>
> ```javascript
> const notebook = new Map();
> notebook.set("grades", JSON.stringify([3, 1, 2]));
>
> const grades = JSON.parse(notebook.get("grades") ?? "[]");
> console.log(grades.reduce((a, b) => a + b, 0));
> ```
>
> ```sortie
> 6
> ```

## Summary

- `localStorage` keeps texts by key, even after closing: `setItem`, `getItem`, `removeItem`.
- Everything becomes text: `JSON.stringify` to store, `JSON.parse` to read back, `Number` for a number.
- `getItem` returns `null` for an empty locker: plan a default value.
- `sessionStorage` has the same API but lives as long as a tab.
- For a lot of data or objects, IndexedDB; never any secret in the browser.

Next: [reading and writing files with Node](fichiers.md)
