# Modules and npm

This page explains how to split a TypeScript program into several files, how to share types between them, and how to use npm packages with their types.

> **Goals:**
>
> - you will know how to export and import functions, values and types;
> - you will know when to write `import type`;
> - you will know how to organize and compile a project made of several files;
> - you will know how to install an npm package and its `@types/…` types;
> - you will know what a `.d.ts` declaration file is.

## Why modules?

A program that grows becomes hard to read in a single file. You split it into
[modules](../debuter/glossaire.md#module): one file per topic. It's like tidying up a
kitchen: the spices in one drawer, the pans in another.

In TypeScript, **every file that contains `import` or `export` is a module**. What it
declares stays private, except what it explicitly **exports**.

## Importing a Node module

Node provides ready-made modules. Their name starts with `node:`. You import only what
you need, between curly braces.

```typescript
import { writeFileSync, readFileSync } from "node:fs";
import path from "node:path";

const filePath = path.join("notes", "..", "shopping.txt");
console.log(filePath);

writeFileSync(filePath, "bread\nmilk\n");
const content = readFileSync(filePath, "utf8");
console.log(content.split("\n").filter((l) => l !== "").length, "items");
```

```sortie
shopping.txt
2 items
```

There are two forms of import:

- `import { a, b } from "…"`: **named exports**, picked one by one;
- `import path from "…"`: the **default export** of the module, under the name you
  want.

Thanks to Node's types, TypeScript knows the signature of `readFileSync`: it knows
that with `"utf8"`, it returns a `string`.

## Exporting from your files

Put `export` in front of what you want to share: a function, a constant, a class, and
also a **type** or an **interface**.

```typescript norun
// src/types.ts
export interface Task {
  title: string;
  done: boolean;
}
```

```typescript norun
// src/tools.ts
import type { Task } from "./types.js";

export function format(t: Task): string {
  return `${t.done ? "[x]" : "[ ]"} ${t.title}`;
}

export default function count(tasks: Task[]): number {
  return tasks.filter((t) => !t.done).length;
}

const SECRET = 42; // not exported: invisible from other files
```

A file has as many named exports as you want, and at most one `export default`.

> **Tip:** prefer named exports. The name is the same everywhere, and your editor
> finds them and renames them easily.

### The .js extension in imports

```typescript norun
// src/main.ts
import count, { format } from "./tools.js";
import type { Task } from "./types.js";

const tasks: Task[] = [
  { title: "Buy bread", done: true },
  { title: "Review TypeScript", done: false },
];

for (const t of tasks) {
  console.log(format(t));
}
console.log(`Still to do: ${count(tasks)}`);
```

The file is called `tools.ts`, but the import says `./tools.js`. That's on purpose:
Node will run the **compiled** file, which will be called `tools.js`. TypeScript
understands that it must check `tools.ts`. The `./` means a file from your project,
not a package.

## import type

A type doesn't exist at run time: [compilation](../debuter/glossaire.md#compilation)
erases all types. `import type` says clearly: "I'm only importing a type". This import
disappears entirely from the JavaScript produced.

```typescript norun
import type { Task } from "./types.js";         // only types
import { format, type Options } from "./tools.js"; // a mix
```

You can also put `type` in front of a single name, in the middle of an ordinary
import.

Why bother? The reader sees right away what is a type. And some tools, which remove
types without analyzing everything, need it to know what to erase. Make it a habit: a
type import is written with `type`.

## A project with several files

Here is how a small project is organized:

```text
tasks/
├── package.json
├── tsconfig.json
├── src/
│   ├── main.ts
│   ├── tools.ts
│   └── types.ts
└── dist/            ← created by tsc
    ├── main.js
    ├── tools.js
    └── types.js
```

The source code is in `src/`, the compiled JavaScript in `dist/`. The `package.json`
file describes the project for npm. The line `"type": "module"` tells Node to use
`import` / `export` modules:

```json
{
  "name": "tasks",
  "type": "module",
  "scripts": {
    "build": "tsc",
    "start": "node dist/main.js"
  },
  "devDependencies": {
    "typescript": "^5.8.3"
  }
}
```

The `tsconfig.json` file sets up the compiler (see [the configuration](configuration.md)
for the details):

```json
{
  "compilerOptions": {
    "target": "es2022",
    "module": "nodenext",
    "strict": true,
    "rootDir": "src",
    "outDir": "dist"
  },
  "include": ["src"]
}
```

You compile the whole project, then run the main file:

```sh
npm run build
npm start
```

```text
[x] Buy bread
[ ] Review TypeScript
Still to do: 1
```

`npm run build` runs `tsc`, which reads `tsconfig.json`, checks **all** the files of
`src/` together and writes `dist/`. If a file misuses a type exported by another one,
the error shows up at that moment.

> **Note:** `types.js` is almost empty: the file only contained an interface, and
> types are erased.

## npm packages

npm is the [package](../debuter/glossaire.md#paquet) manager of JavaScript. It
downloads libraries written by other people into the `node_modules/` folder and
records them in `package.json`.

```sh
npm install lodash
```

Then you import the package by its name, without `./`:

```typescript norun
import _ from "lodash";

console.log(_.chunk([1, 2, 3, 4, 5], 2));
```

Surprise: TypeScript refuses to compile.

```text
src/main.ts(1,15): error TS7016: Could not find a declaration file for module 'lodash'. '/home/alice/tasks/node_modules/lodash/lodash.js' implicitly has an 'any' type.
  Try `npm i --save-dev @types/lodash` if it exists or add a new declaration (.d.ts) file containing `declare module 'lodash';`
```

`lodash` is written in JavaScript, without types. TypeScript doesn't know what `_`
contains.

### @types packages

For many JavaScript packages, volunteers have written the types separately, in the
**DefinitelyTyped** project. They are published under the name `@types/<package>`.
The error message suggests it to you:

```sh
npm install --save-dev @types/lodash
```

```text
[ [ 1, 2 ], [ 3, 4 ], [ 5 ] ]
```

The program compiles and runs. `--save-dev` puts this package in `devDependencies`:
the types are only used during development, not at run time.

```json
{
  "dependencies": {
    "lodash": "^4.18.1"
  },
  "devDependencies": {
    "@types/lodash": "^4.17.26"
  }
}
```

Node's own types come from `@types/node`.

### Packages that ship their types

Recent packages are often written in TypeScript and **ship their types**. Nothing to
add: `npm install zod` is enough. To know where you stand:

| Situation | What to do? |
|-----------|-------------|
| The package ships its types | nothing |
| Error TS7016, and `@types/<name>` exists | `npm install --save-dev @types/<name>` |
| Error TS7016, and nothing exists | write a small `.d.ts` file |

> **Tip:** on the npm website, a small "TS" badge next to the name marks a package
> that ships its types; a "DT" badge marks types available in `@types`.

## .d.ts declaration files

A `.d.ts` file contains **only types**, without any executable code. It's the user
manual of a module: it describes what the module exports and with which types.
`@types/…` packages contain nothing else.

### Writing a declaration

Imagine a `text-colors` package, in JavaScript, with no types anywhere. You can write
its manual yourself, in `src/types/text-colors.d.ts`:

```typescript norun
declare module "text-colors" {
  export function red(text: string): string;
  export function green(text: string): string;
}
```

`declare` means: "this exists somewhere else, I'm only describing its shape".
TypeScript takes your word for it: describe only the functions you use, and describe
them correctly.

### Producing declarations

The other way around, if you write a library, the `"declaration": true` option of
`tsconfig.json` asks `tsc` to write a `.d.ts` next to each `.js`. For `tools.ts`, it
produces:

```typescript norun
import type { Task } from "./types.js";
export declare function format(t: Task): string;
export default function count(tasks: Task[]): number;
```

The function bodies are gone: only the signatures remain. People who use your library
will get the types this way, without installing anything else.

## Common mistakes

### Forgetting the .js extension

```typescript norun
// src/main.ts
import { format } from "./tools";
```

```text
src/main.ts(2,24): error TS2835: Relative import paths need explicit file extensions in ECMAScript imports when '--moduleResolution' is 'node16' or 'nodenext'. Did you mean './tools.js'?
```

With `"module": "nodenext"`, Node requires the full extension. Write `"./tools.js"`,
as the message suggests.

### Importing what isn't exported

```typescript norun
// src/main.ts
import { SECRET } from "./tools.js";
```

```text
src/main.ts(2,10): error TS2614: Module '"./tools.js"' has no exported member 'SECRET'. Did you mean to use 'import SECRET from "./tools.js"' instead?
```

`SECRET` exists in `tools.ts`, but without `export`. Add `export` in front of its
declaration, if you really want to share it. Don't follow the message's suggestion:
`import SECRET from …` would import the default export, that is, the `count`
function.

### Using a type import as a value

```typescript norun
// src/main.ts
import type { format } from "./tools.js";
console.log(format({ title: "Read", done: false }));
```

```text
src/main.ts(3,13): error TS1361: 'format' cannot be used as a value because it was imported using 'import type'.
```

`import type` is erased at compilation: the function would no longer exist at run
time. For a function, a class or a constant, use an ordinary `import`.

## Test yourself

**Question 1:** in `src/main.ts`, why do you write `import { f } from "./tools.js"`
when the file is called `tools.ts`?

> **Answer**
>
> Because Node runs the compiled code, where the file is called `tools.js`. With
> `"module": "nodenext"`, TypeScript requires the path that Node will use, and knows
> that it must check `tools.ts` instead.

**Question 2:** you install a package and get error TS7016. What do you do?

> **Answer**
>
> The package doesn't ship its types. Install `@types/<name>` with
> `npm install --save-dev @types/<name>` if it exists; otherwise, write a `.d.ts` file
> with `declare module "<name>" { … }`.

**Question 3:** with the `node:path` module, print the extension and the base name
(without folder) of the path `"photos/holidays/beach.jpg"`.

> **Answer**
>
> ```typescript
> import path from "node:path";
>
> const filePath = "photos/holidays/beach.jpg";
> console.log(path.extname(filePath));
> console.log(path.basename(filePath));
> ```
>
> ```sortie
> .jpg
> beach.jpg
> ```

## Summary

- A file with `import` or `export` is a module; only what is exported is visible
  elsewhere.
- You import your own files with `./name.js`, and Node's modules with `node:…`.
- `import type` imports only a type: it disappears at compilation.
- `tsc` compiles the whole project from `src/` to `dist/`, guided by `tsconfig.json`.
- An npm package without types is completed with `@types/<name>` or a `.d.ts` file.

Next: [asynchronous programming](asynchrone.md)
