# TypeScript: introduction and installation

This page introduces TypeScript, explains how to install it, and has you write, check and then run your first program.

> **Goals:**
>
> - you will be able to say what TypeScript is and what it adds to JavaScript;
> - you will know how to install Node.js and TypeScript, then check that they work;
> - you will know how to write a `.ts` file, check it with `tsc` and run it with `node`;
> - you will know how to run a `.ts` file directly, without an extra step;
> - you will know how to read your first type error.

## What is TypeScript?

TypeScript is a programming language created by Microsoft in 2012. It is
**JavaScript with types**.

- **JavaScript** is the language of web pages. It runs in every browser,
  and also on your computer thanks to Node.js.
- A [type](../debuter/glossaire.md#type) is the *kind* of a value: a number, a
  piece of text, a true/false…

In TypeScript, you can write down the kind of each value. Here is a first example:

```typescript
const firstName: string = "Alice";
const age: number = 30;

console.log(firstName);
console.log(age + 1);
```

```sortie
Alice
31
```

- `const firstName` creates a [variable](../debuter/glossaire.md#variable) named `firstName`.
- `: string` is the **type annotation**: "this variable holds some text".
- `: number` means "this variable holds a number".
- `console.log(...)` prints a value in the terminal.

Without the `: string` and `: number`, this would be ordinary JavaScript. So almost all
JavaScript code is already TypeScript: you don't need to know JavaScript to get
started, this track teaches you both at once.

> **Note:** the [JavaScript](../javascript/introduction.md) track presents the same
> language without the types. You can read it later, out of curiosity.

## Why types?

Types help you find [bugs](../debuter/glossaire.md#bug) **before** you run
the program.

Think of a spell checker: it underlines mistakes while you are writing, before the
letter goes out. TypeScript does the same thing with your code. It reads everything
and underlines what cannot work.

Here is a JavaScript program with a mistake. It computes a total price:

```javascript norun
const price = "10";
const total = price * 2 + price;
console.log(total);
```

```text
2010
```

We expected `30`. But `price` is text (`"10"`, in quotes). `price * 2`
gives `20`, then `20 + "10"` glues the two pieces of text together: `"2010"`.
JavaScript says nothing, it prints a wrong result.

With TypeScript, we declare that `price` is a number. The mistake jumps out right
away, before anything runs:

```typescript norun
const price: number = "10";
```

```text
main.ts:1:7 - error TS2322: Type 'string' is not assignable to type 'number'.

1 const price: number = "10";
        ~~~~~


Found 1 error in main.ts:1
```

You fix it with `const price: number = 10;` and the program prints `30`.

> **Remember:** JavaScript discovers errors along the way, sometimes never.
> TypeScript finds them by reading the code, before running it.

Types have other benefits:

- your [code editor](../debuter/editeur.md) knows what each variable holds:
  it suggests the right actions and completes names;
- the code documents itself: `age: number` clearly says what is expected;
- when you change a program, TypeScript shows you every place you need to update.

## How it works: tsc and node

The computer cannot run TypeScript directly. It goes through two steps:

1. **`tsc`** (*TypeScript compiler*) checks the types, then translates the `.ts` file
   into a `.js` file: it simply removes the annotations. It is a
   [compiler](../debuter/glossaire.md#compiler).
2. **`node`** runs the resulting `.js` file.

```text
hello.ts  --(tsc: checks and translates)-->  hello.js  --(node: runs)-->  output
```

So types only exist during the check. When the program runs, they are gone: only
JavaScript is left.

## Installing Node.js

TypeScript is installed with **npm**, the package manager that comes with Node.js.
So install Node.js first. This site uses **Node 22**; a newer version works too.

### On Windows and macOS

Download the "LTS" installer (the stable version) from the nodejs.org website, then
run it. Keep the default options.

### On Linux

Use your system's package manager, or the `nvm` tool, which installs the version of
your choice:

```sh
sudo apt install nodejs npm    # Debian, Ubuntu (sometimes an old version)
nvm install 22                 # with nvm: Node 22
```

### Checking Node.js

Open a terminal and type these two commands. If the terminal is new to you, first
read [the terminal](../debuter/terminal.md).

```sh
node --version
npm --version
```

You should see two numbers, for example `v22.22.0` and `10.9.4`. If the terminal
answers "command not found", close it and open it again after the installation.

## Installing TypeScript

A single command is enough:

```sh
npm install -g typescript
```

- `npm install` downloads and installs a package;
- `-g` (*global*) installs it for the whole computer: the `tsc` command will work in
  any folder.

Check the installation:

```sh
tsc --version
```

```text
Version 5.8.3
```

This site checks its examples with TypeScript 5.8; a newer version works too.

> **Tip:** on Linux or macOS, if `npm install -g` answers *permission denied*,
> install Node.js with `nvm`: it puts everything in your home folder, with no need
> for administrator rights.

## Your first program

Open your [code editor](../debuter/editeur.md). Create a folder `practice`, and
inside it a file `hello.ts` with this content:

```typescript
const firstName: string = "Alice";
console.log(`Hello, ${firstName}!`);
```

```sortie
Hello, Alice!
```

- Text between backticks `` ` `` can contain `${...}`: the value of
  `firstName` is inserted at that spot.
- Each statement ends with a semicolon `;`.

### Compiling with tsc

In the terminal, go into the `practice` folder (with `cd`), then type:

```sh
tsc hello.ts
```

If all goes well, `tsc` prints **nothing**: no news is good news. A file
`hello.js` has appeared next to it. Open it: it is the same program, without the
`: string` annotation.

```javascript norun
"use strict";
const firstName = "Alice";
console.log(`Hello, ${firstName}!`);
```

The line `"use strict";` asks JavaScript to be stricter; you can ignore it.

### Running with node

```sh
node hello.js
```

```text
Hello, Alice!
```

> **Remember:** `tsc file.ts` checks and produces `file.js`;
> `node file.js` runs it.

## Running a .ts file directly

Compiling then running means two commands for every try. There are two shortcuts.

### With Node 22

Node itself can run a `.ts` file: it erases the annotations on the fly.

```sh
node --experimental-strip-types hello.ts
```

Since Node 22.18, the option is not even needed: `node hello.ts` is enough.

### With tsx

`tsx` is a small tool that runs a TypeScript file in one command. `npx` downloads
it the first time you use it:

```sh
npx tsx hello.ts
```

> **Warning:** these two shortcuts **do not check the types**. They remove the
> annotations and run the program, even if it contains a type error. Run
> `tsc --noEmit hello.ts` to check without creating a `.js` file.

## Trying it online: the Playground

The **typescriptlang.org/play** website (the "Playground") lets you try TypeScript
in the browser, without installing anything.

- On the left, you type your code; errors are underlined in red.
- On the right, the ".JS" tab shows the JavaScript produced.
- The "Run" button runs the code; the "Logs" tab shows what `console.log`
  prints.

> **Tip:** hover over a variable with the mouse: the Playground shows its type.
> It is the best way to see what TypeScript has understood.

## Reading a type error

When `tsc` finds a problem, it prints an error. Take this file, with a typo in the
variable name:

```typescript norun
const firstName: string = "Alice";
console.log(fristName);
```

```text
main.ts:2:13 - error TS2552: Cannot find name 'fristName'. Did you mean 'firstName'?

2 console.log(fristName);
              ~~~~~~~~~

  main.ts:1:7
    1 const firstName: string = "Alice";
            ~~~~~~~~~
    'firstName' is declared here.


Found 1 error in main.ts:2
```

Read it piece by piece:

- `main.ts:2:13`: the file, line 2, column 13;
- `error TS2552`: the error number (useful for searching the web);
- `Cannot find name 'fristName'`: TypeScript doesn't know any `fristName`;
- `Did you mean 'firstName'?`: it suggests the closest name;
- the squiggles `~~~` underline the exact spot;
- the indented part shows where `firstName` was declared (*declared here*).

A few words come up often in these messages: *type*, *assignable* (can be stored
in), *property*, *argument*, *expected*.

## Common mistakes

### Running node on the .ts file with an old Node

```sh
node hello.ts
```

```text
hello.ts:1
const firstName: string = "Alice";
      ^^^^^^^^^

SyntaxError: Missing initializer in const declaration
    at wrapSafe (node:internal/modules/cjs/loader:1638:18)
```

The message goes on with more `at ...` lines, which don't matter here. Before Node
22.18, `node` does not understand annotations like `: string`. Compile first
(`tsc hello.ts`) then run `node hello.js`. Or use
`node --experimental-strip-types hello.ts`.

### Thinking tsc stopped the program

```typescript norun
const age: number = "thirty";
console.log(age);
```

```text
main.ts:1:7 - error TS2322: Type 'string' is not assignable to type 'number'.

1 const age: number = "thirty";
        ~~~


Found 1 error in main.ts:1
```

By default, `tsc` prints the error **and** still creates the `.js` file. Don't run
`node` while `tsc` reports errors: fix them first, here with
`const age: number = 30;`.

### Forgetting to install TypeScript

```sh
tsc hello.ts
```

```text
tsc: command not found
```

TypeScript is not installed, or the terminal was opened before the installation.
Type `npm install -g typescript`, then open a new terminal.

## Test yourself

**Question 1:** what does `tsc hello.ts` do?

> **Answer**
>
> It checks the types of the file, then creates `hello.js`: the same program, without
> the annotations. You then run it with `node hello.js`.

**Question 2:** why is TypeScript useful, when the final program is JavaScript?

> **Answer**
>
> Because it finds errors **before** the program runs: text instead of a number, a
> typo in a name… The program you run is JavaScript, but JavaScript that has already
> been proofread.

**Question 3:** `npx tsx game.ts` runs your program without any error. Is it
correctly typed?

> **Answer**
>
> Not necessarily: `tsx` does not check types. Run `tsc --noEmit game.ts` to be
> sure.

## Summary

- TypeScript is JavaScript with types: `const age: number = 30;`.
- Types find errors before the program runs, like a spell checker.
- Install Node.js, then `npm install -g typescript`; `tsc --version` checks it.
- `tsc hello.ts` checks and produces `hello.js`, run with `node hello.js`.
- `node --experimental-strip-types` and `npx tsx` run a `.ts` directly, without
  checking types.
- The Playground (typescriptlang.org/play) lets you try without installing anything.

Next: [first steps](premiers-pas.md).
