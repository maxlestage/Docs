# Modules and npm

This page shows how to split a program into several files with `import` and `export`, then how to use packages written by other developers with npm.

> **Goals:**
>
> - you will be able to export and import code between files;
> - you will be able to tell a default export from a named export;
> - you will be able to create a project with `npm init` and read a `package.json`;
> - you will be able to install a package and run scripts with `npm run`;
> - you will know why `node_modules` does not go into Git.

## Why split into modules?

A 50-line program fits in one file. A 5,000-line program does not: you get lost in it.
So you split it into [modules](../debuter/glossaire.md#module): files that each have a
precise role.

Think of a kitchen: spices in one cupboard, dishes in another. Each cupboard has a
label, and you go and get what you need.

In modern JavaScript, **each file is a module**. What is declared in a file stays
private to it, except what it explicitly **exports**.

## Node's built-in modules

Node provides ready-made modules, whose name starts with `node:`. You import them with
`import { ... } from "..."` at the top of the file.

```javascript
import { join, basename, extname } from "node:path";

const path = join("photos", "holidays", "beach.jpg");
console.log(path);
console.log(basename(path));
console.log(extname(path));
```

```sortie
photos/holidays/beach.jpg
beach.jpg
.jpg
```

Between the curly braces, you choose what you want to take from the module. You can
also rename an item with `as`, or take everything at once with `* as`:

```javascript
import { join as assemble } from "node:path";
import * as path from "node:path";

console.log(assemble("a", "b"));
console.log(path.dirname("/house/kitchen/oven.txt"));
```

```sortie
a/b
/house/kitchen
```

## Exporting and importing your functions

Now for your own modules. There are two ways to export.

### Named export

You write `export` in front of each declaration you want to share. A file can have as
many named exports as it wants. The importing file must use **the same name**, between
curly braces.

```javascript norun
// maths.js
export const VERSION = "1.0";

export function sum(numbers) {
  return numbers.reduce((total, n) => total + n, 0);
}

export function average(numbers) {
  return sum(numbers) / numbers.length;
}
```

```javascript norun
// main.js
import { average, VERSION } from "./maths.js";
```

### Default export

`export default` marks **the** main thing of a file. There is only one per file. The
importing file writes it **without curly braces**, and picks any name it wants.

```javascript norun
// greet.js
export default function greet(name) {
  return `Hello, ${name}!`;
}
```

```javascript norun
// main.js
import greet from "./greet.js";
```

| | Named export | Default export |
|-|--------------|----------------|
| Syntax | `export function sum` | `export default function greet` |
| How many per file? | as many as you want | only one |
| Import | `import { sum } from "./maths.js"` | `import greet from "./greet.js"` |
| Name when importing | the same (or `as`) | free |

> **Tip:** prefer named exports: names stay the same throughout the project, and your
> editor helps you better.

## A project in several files

Here is a small complete project. The file tree:

```text
my-project/
├── package.json
└── src/
    ├── main.js
    ├── maths.js
    └── greet.js
```

`maths.js` and `greet.js` are the ones from the previous section. `main.js` uses them:

```javascript norun
// src/main.js
import greet from "./greet.js";
import { average, VERSION } from "./maths.js";

console.log(greet("Ada"));
console.log(`Average: ${average([12, 15, 18])}`);
console.log(`Version ${VERSION}`);
```

You run the main file from the `my-project` folder:

```sh
node src/main.js
```

```text
Hello, Ada!
Average: 15
Version 1.0
```

Three rules to remember:

- a path to **your** file starts with `./` (same folder) or `../` (parent folder);
- the `.js` extension is **required**;
- `import` statements go at the top of the file.

`sum` is not imported in `main.js`, but `average` still uses it: inside `maths.js`,
everything is visible.

### The "type": "module" field

For Node to read `.js` files as `import`/`export` modules, the project's `package.json`
file must contain `"type": "module"`. Otherwise Node prints a warning. `.mjs` files are
always modules, whatever the `package.json` says.

## The old syntax: require

In older code, you will often come across another syntax called **CommonJS**:

```javascript norun
// Old CommonJS syntax (.cjs files, or projects without "type": "module")
const path = require("node:path");

function greet(name) {
  return `Hello, ${name}!`;
}

module.exports = { greet };
```

`require` plays the role of `import`, `module.exports` the role of `export`. For a new
project, use `import` and `export`: it is the standard syntax, and it also works in the
browser.

## Creating a project with npm

**npm** (*Node Package Manager*) is installed with Node. It manages your project and its
[packages](../debuter/glossaire.md#package): code written by other people, ready to
reuse. It is a huge public library: more than two million packages.

`npm init -y` creates the `package.json` file, the project's ID card (`-y` accepts all
the default answers).

```sh
mkdir my-project
cd my-project
npm init -y
npm pkg set type=module
```

```json
{
  "name": "my-project",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [],
  "author": "",
  "license": "ISC",
  "type": "module"
}
```

This is [JSON](../debuter/glossaire.md#json). The main fields:

| Field | Role |
|-------|------|
| `name`, `version` | the name and version of the project |
| `scripts` | short commands, run with `npm run` |
| `type` | `"module"` to use `import` and `export` |
| `dependencies` | the packages the project needs (added by `npm install`) |

## Installing a package

`npm install` (or `npm i`) downloads a package and adds it to the project. Let's take
`slugify`, which turns a title into a clean web address.

```sh
npm install slugify
```

```text
added 1 package, and audited 2 packages in 535ms

found 0 vulnerabilities
```

Three things have changed in the folder:

- a `node_modules/` folder contains the package's code;
- `package.json` has a new section `"dependencies": { "slugify": "^1.6.9" }`;
- a `package-lock.json` file records the exact version installed.

You import a package by its **name**, without `./`:

```javascript norun
// src/title.js
import slugify from "slugify";

const title = "Crème brûlée: the chef's secret recipe";
console.log(slugify(title, { lower: true, strict: true }));
```

```text
creme-brulee-the-chefs-secret-recipe
```

> **Note:** `^1.6.9` means "version 1.6.9 or a more recent compatible version (1.x)". To
> remove a package: `npm uninstall slugify`.

## npm scripts

The `scripts` section of `package.json` gives short names to commands. You run them
with `npm run <name>`. `start` and `test` have a shortcut: `npm start`, `npm test`.

```json
"scripts": {
  "start": "node src/main.js",
  "title": "node src/title.js"
}
```

```sh
npm start
```

```text
> my-project@1.0.0 start
> node src/main.js

Hello, Ada!
Average: 15
Version 1.0
```

This way, someone discovering the project does not have to guess which file to run:
`npm start` is enough.

## node_modules and .gitignore

`node_modules/` can contain thousands of files. You **never** save it in
[Git](../debuter/git.md): `package.json` and `package-lock.json` are enough to rebuild
it. Add it to the project's `.gitignore` file:

```text
node_modules/
```

Someone who gets your project simply types:

```sh
npm install
```

Without a package name, `npm install` reads `package.json` and installs everything
listed. It is like a shopping list: you keep the list, not the contents of the fridge.

## Running a tool with npx

`npx` runs a tool published on npm **without installing it** in your project: it
downloads it, runs it, and that's it. Handy for trying a tool or for creating a
project.

```sh
npx --yes cowsay "Hello"
```

```text
 _______
< Hello >
 -------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```

`--yes` stops `npx` from asking for permission to download.

## Common mistakes

### Forgetting the .js extension

```javascript norun
import { average } from "./maths";
```

```text
node:internal/modules/esm/resolve:274
    throw new ERR_MODULE_NOT_FOUND(
          ^

Error [ERR_MODULE_NOT_FOUND]: Cannot find module '/home/vous/my-project/src/maths' imported from /home/vous/my-project/src/main.js
```

Node looks for a file called exactly `maths`, without an extension. Write the full
name: `"./maths.js"`.

### Importing a name that is not exported

```javascript norun
import { average, median } from "./maths.js";
```

```text
file:///home/vous/my-project/src/main.js:2
import { average, median } from "./maths.js";
                  ^^^^^^
SyntaxError: The requested module './maths.js' does not provide an export named 'median'
```

"The module does not provide an export named `median`": the function does not exist,
or it has no `export` in front of it. Check the spelling and add `export`. The same
message appears if you write `import { greet }` for a **default** export: then remove
the curly braces.

### Using require in a module

```javascript norun
const fs = require("node:fs");
```

```text
file:///home/vous/my-project/src/old.js:1
const fs = require("node:fs");
           ^

ReferenceError: require is not defined in ES module scope, you can use import instead
```

In a module (`.mjs`, or `"type": "module"`), `require` does not exist. Write
`import fs from "node:fs";` or `import { existsSync } from "node:fs";`.

## Test yourself

**Question 1:** a file contains `export default function display() {}` and
`export const LIMIT = 10;`. How do you import both?

> **Answer**
>
> `import display, { LIMIT } from "./file.js";`: the default first, without curly
> braces, then the names between curly braces.

**Question 2:** what is the difference between `"./slugify.js"` and `"slugify"` in an
`import`?

> **Answer**
>
> `"./slugify.js"` is **your** file, next to the current file. `"slugify"`, without
> `./`, is a **package** installed with npm, looked up in `node_modules`.

**Question 3:** a friend gets your project from Git, but there is no `node_modules`
folder. What should they do?

> **Answer**
>
> Type `npm install` in the project folder. npm reads `package.json` and
> `package-lock.json`, and reinstalls all the packages.

## Summary

- Each file is a module; `export` shares, `import` retrieves.
- Named export: `import { name }`; default export: `import name`, without curly braces.
- Paths to your files start with `./` and end with `.js`.
- `npm init` creates `package.json`; `npm install` adds a package; `npm run` runs a
  script.
- `node_modules/` goes into `.gitignore`; `npx` runs a tool without installing it.

Next: [asynchronous programming](asynchrone.md). Compare with
[modules in Ruby](../ruby/modules.md).
