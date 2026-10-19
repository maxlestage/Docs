# A typed command-line tool

This page shows how to write a small TypeScript program that you run in the terminal, with arguments, checked options, an exit code and reading from standard input.

> **Goals:**
>
> - you will know how to read arguments with `process.argv` and know that they are always
>   text;
> - you will know how to describe options with `util.parseArgs` and benefit from the
>   types it infers;
> - you will know how to return an exit code (`0` for success, anything else for
>   failure);
> - you will know how to read standard input;
> - you will know how to build a word counter where each piece can be tested with fixed
>   arrays.

The [JavaScript chapter](../javascript/ligne-de-commande.md) presents the same Node
tools. Here, we focus on **types** and on a way of writing the program that makes it
easy to test.

## What is a command-line tool?

It is a program you start by typing its name in the
[terminal](../debuter/glossaire.md#terminal), followed by a few words:

```sh
node count.ts --lines poem.txt
```

Think of a service counter. You hand over a filled-in **order form** (the arguments).
The clerk reads it, does the work, gives you the result, and tells you "all good" or
"there is a problem" (the exit code).

```text
   you ──►  node count.ts --lines poem.txt
                       │
                       ▼
             ┌───────────────────┐
             │  read the options │  --lines = yes, file = poem.txt
             │  do the work      │  count the words
             └─────────┬─────────┘
                       ▼
             result on screen  +  exit code (0 = all good)
```

> **Note:** since Node 22.18, `node file.ts` runs a TypeScript file directly, without
> checking the types (see the [introduction](introduction.md)). Run `npx tsc --noEmit`
> to check the types first.

> **Remember:** a command-line tool receives words, does its work, prints a result and
> returns an exit code.

## Arguments with process.argv

`process.argv` contains everything that was typed. Its type is `string[]`: an array of
texts.

```text
   node  /home/ada/greet.ts  Ada  3
   ───┬  ────────┬─────────  ─┬─  ┬
   slot 0     slot 1         slot 2  slot 3
   (node)     (the script)   your arguments
```

So your arguments start at slot `2`. You remove the first two slots with `slice(2)`.

So that the example always gives the same output, we write the array by hand here, as
if you had typed `node greet.ts Ada 3`:

```typescript
const argv: string[] = ["/usr/bin/node", "/home/ada/greet.ts", "Ada", "3"];

const args = argv.slice(2);
console.log(args);
console.log(`Hello, ${args[0]}!`);
console.log(typeof args[1]);
```

```sortie
[ 'Ada', '3' ]
Hello, Ada!
string
```

Notice the last line: `"3"` is a **text**, not a number. An argument is always text.

In a real program, you simply write:

```typescript norun
const args = process.argv.slice(2);
```

### An argument can be missing

TypeScript announces `args[0]` as a `string`. But if the user types nothing, the slot
does not exist and is `undefined`. TypeScript does not flag it (it does not count the
slots of an array): it is up to you to provide a default value with `??`.

```typescript
function greet(args: string[]): string {
  const name = args[0] ?? "stranger";
  return `Hello, ${name}!`;
}

console.log(greet(["Ada"]));
console.log(greet([]));
```

```sortie
Hello, Ada!
Hello, stranger!
```

Notice the shape: `greet` receives the array **as a parameter**, instead of reading
`process.argv` itself. So you can try it with any array. This idea is used throughout
the page.

Try it: call `greet(["Ada", "Grace"])`. Only `Ada` is greeted: slot 1 is ignored.

### Converting an argument to a number

To get a number, you convert the text with `Number`. If the text is not a number, you
get `NaN` ("not a number"). A small typed function handles both cases:

```typescript
function readInteger(text: string | undefined): number | null {
  if (text === undefined) {
    return null;
  }
  const n = Number(text);
  return Number.isInteger(n) ? n : null;
}

for (const attempt of ["3", "three", "2.5", undefined]) {
  console.log(attempt, "->", readInteger(attempt));
}
```

```sortie
3 -> 3
three -> null
2.5 -> null
undefined -> null
```

| Step | `text` | `Number(text)` | Integer? | Result |
|------|--------|----------------|----------|--------|
| 1 | `"3"` | 3 | yes | 3 |
| 2 | `"three"` | `NaN` | no | `null` |
| 3 | `"2.5"` | 2.5 | no | `null` |
| 4 | `undefined` | (not computed) | | `null` |

> **Remember:** `process.argv.slice(2)` gives your arguments, of type `string[]`. They
> are always texts, and they can be missing.

## Options with util.parseArgs

Real tools accept **options**: `--lines`, `-l`, `--max 10`… Sorting them by hand is
tedious. The `parseArgs` function from the `node:util` module does it for you. You
describe the expected options to it, like a form with boxes to fill in:

- `type: "boolean"`: a checkbox, present or absent (`--lines`);
- `type: "string"`: a box to fill in, followed by a value (`--max 10`);
- `short: "l"`: a one-letter shortcut (`-l`);
- `default`: the value if the option is missing.

Normally, `parseArgs` reads `process.argv` on its own. Here, we give it an `args` array
written by hand: we simulate `node tool.ts -l --max 3 a.txt b.txt`.

```typescript
import { parseArgs } from "node:util";

const { values, positionals } = parseArgs({
  args: ["-l", "--max", "3", "a.txt", "b.txt"],
  options: {
    lines: { type: "boolean", short: "l", default: false },
    max: { type: "string" },
  },
  allowPositionals: true,
});

console.log(values.lines);
console.log(values.max);
console.log(positionals);
```

```sortie
true
3
[ 'a.txt', 'b.txt' ]
```

Here is how `parseArgs` reads the array, slot by slot:

| Step | Slot read | What it is | `values` after | `positionals` after |
|------|-----------|------------|----------------|---------------------|
| 1 | `"-l"` | shortcut for `--lines` | `{ lines: true }` | `[]` |
| 2 | `"--max"` | text option: it takes the next slot | `{ lines: true, max: "3" }` | `[]` |
| 3 | `"3"` | (already taken by `--max`) | | |
| 4 | `"a.txt"` | no dash: argument | | `["a.txt"]` |
| 5 | `"b.txt"` | no dash: argument | | `["a.txt", "b.txt"]` |

`allowPositionals: true` allows arguments without a dash (the file names).

### The types inferred by TypeScript

TypeScript reads the description of the options and **infers** the type of each value:

| Description | Type of the value |
|-------------|-------------------|
| `{ type: "boolean" }` | `boolean \| undefined` |
| `{ type: "boolean", default: false }` | `boolean` |
| `{ type: "string" }` | `string \| undefined` |
| `{ type: "string", default: "10" }` | `string` |

Without `default`, the option can be missing: the type contains `undefined`, and
TypeScript forces you to handle that case. And a `string` option stays a **text**:
`values.max` is never a `number` (see the [common mistakes](#common-mistakes)).

```typescript
import { parseArgs } from "node:util";

function readMax(args: string[]): number {
  const { values } = parseArgs({
    args,
    options: { max: { type: "string", default: "10" } },
  });
  // values.max is a string (thanks to default): we convert it.
  return Number(values.max);
}

console.log(readMax(["--max", "3"]));
console.log(readMax(["--max=25"]));
console.log(readMax([]));
```

```sortie
3
25
10
```

Try it: remove `default: "10"`. The program still compiles (`Number(undefined)` is
allowed), but the last call prints `NaN`. Add `?? "10"` to fix it.

### An unknown option

`parseArgs` is **strict**: an option it does not know throws an error. That is a good
thing, because a typo (`--liens`) does not go unnoticed. You catch it with `try` /
`catch` (see [handling errors](erreurs.md)):

```typescript
import { parseArgs } from "node:util";

function tryArgs(args: string[]): void {
  try {
    const { values } = parseArgs({ args, options: { lines: { type: "boolean" } } });
    console.log("OK:", values.lines);
  } catch (e) {
    const message = e instanceof Error ? e.message : String(e);
    console.log("Rejected:", message);
  }
}

tryArgs(["--lines"]);
tryArgs(["--liens"]);
tryArgs(["poem.txt"]);
```

```sortie
OK: true
Rejected: Unknown option '--liens'
Rejected: Unexpected argument 'poem.txt'. This command does not take positional arguments
```

The third call is rejected because `allowPositionals` is missing: without it, a file
name is forbidden.

> **Remember:** `parseArgs` sorts the options and TypeScript infers their types. A
> `string` option stays a text; without `default`, it can be `undefined`.

## The exit code

When it stops, a program returns an **exit code** to the terminal, a small number:

| Code | Usual meaning |
|------|---------------|
| `0` | everything went well |
| `1` | the work failed (file not found…) |
| `2` | the command was badly written (unknown option…) |

It is the clerk's "all good" or "there is a problem". Scripts and automation tools read
it to know whether the step succeeded.

You choose it with `process.exitCode`. The simplest way: a `main` function that
**returns** the code, and a single line that hands it to Node.

```typescript
function main(args: string[]): number {
  if (args.length === 0) {
    console.log("Error: give at least one name.");
    return 2;
  }
  for (const name of args) {
    console.log(`Hello, ${name}!`);
  }
  return 0;
}

console.log("code:", main(["Ada", "Grace"]));
console.log("code:", main([]));
```

```sortie
Hello, Ada!
Hello, Grace!
code: 0
Error: give at least one name.
code: 2
```

In the real program, the last line becomes:

```typescript norun
process.exitCode = main(process.argv.slice(2));
```

And in the terminal, `echo $?` prints the code of the last command:

```sh
node greet.ts
echo $?
```

```text
Error: give at least one name.
2
```

> **Note:** in a real tool, write error messages with `console.error`: they go to the
> **error output**, separate from the results (see the
> [JavaScript chapter](../javascript/ligne-de-commande.md)). Here, we use `console.log`
> so that the examples print everything.

> **Tip:** prefer `process.exitCode = …` to `process.exit(…)`. `process.exit` stops the
> program abruptly, even in the middle of writing a file.

> **Remember:** `0` means success, any other code means failure. A function
> `main(args): number` makes this easy to test.

## Reading standard input

A tool can also receive text through a **pipe** (`|`): the output of one command becomes
the **standard input** (*stdin*) of the next one.

```text
   cat poem.txt  ──|──►  node count.ts
   (prints the text)     (reads it on its standard input)
```

The `text` function from `node:stream/consumers` reads all of standard input and returns
a [promise](../debuter/glossaire.md#promesse) of `string`. This example cannot run here,
because it waits for text coming from the terminal:

```typescript norun
import { text } from "node:stream/consumers";

const input: string = await text(process.stdin);
const words = input.split(/\s+/).filter((w) => w !== "");
console.log(`${words.length} words`);
```

```sh
printf 'one two\nthree\n' | node count.ts
```

```text
3 words
```

As usual, keep the computation in a function that receives a `string`: you can test it
without a pipe, with a text written by hand. That is what the next section does.

> **Remember:** `await text(process.stdin)` reads everything that comes through the
> pipe, as a single `string`.

## A typed word counter

Let's put it all together: a `count` tool that counts the words of one or more files,
and, on request, the lines.

```text
   Usage: count [-l] <file>...
     -l, --lines    also count the lines
     -h, --help     show this help
```

The program is split into small functions, each with one role:

```text
   args (string[]) ──► parse ──► Command ──► main ──► exit code
                                               │
                                  read(file) ──┤
                                               ▼
                                             count ──► Count
```

Reading the files is **passed as a parameter** (`read`). In the example, we give it a
fake disk: a plain object that maps a file name to its content. In the real program,
we will give it `readFileSync`.

```typescript
import { parseArgs } from "node:util";

type Command =
  | { kind: "help" }
  | { kind: "error"; message: string }
  | { kind: "count"; lines: boolean; files: string[] };

interface Count {
  words: number;
  lines: number;
}

const HELP = "Usage: count [-l] <file>...";

function parse(args: string[]): Command {
  try {
    const { values, positionals } = parseArgs({
      args,
      options: {
        lines: { type: "boolean", short: "l", default: false },
        help: { type: "boolean", short: "h", default: false },
      },
      allowPositionals: true,
    });
    if (values.help) {
      return { kind: "help" };
    }
    if (positionals.length === 0) {
      return { kind: "error", message: "give at least one file" };
    }
    return { kind: "count", lines: values.lines, files: positionals };
  } catch (e) {
    return { kind: "error", message: e instanceof Error ? e.message : String(e) };
  }
}

function count(text: string): Count {
  const words = text.split(/\s+/).filter((w) => w !== "").length;
  const lines = text === "" ? 0 : text.split("\n").filter((l) => l !== "").length;
  return { words, lines };
}

function main(args: string[], read: (file: string) => string | null): number {
  const command = parse(args);
  switch (command.kind) {
    case "help":
      console.log(HELP);
      return 0;
    case "error":
      console.log(`Error: ${command.message}`);
      console.log(HELP);
      return 2;
    case "count": {
      let code = 0;
      for (const file of command.files) {
        const text = read(file);
        if (text === null) {
          console.log(`${file}: not found`);
          code = 1;
          continue;
        }
        const { words, lines } = count(text);
        const detail = command.lines ? `, ${lines} lines` : "";
        console.log(`${file}: ${words} words${detail}`);
      }
      return code;
    }
  }
}

// A fake disk, to try it out without real files.
const disk: Record<string, string> = {
  "poem.txt": "The cat sleeps\non the mat\n",
  "list.txt": "bread milk\neggs\n\nbutter\n",
};
const readFake = (file: string): string | null => disk[file] ?? null;

const attempts = [
  ["poem.txt"],
  ["-l", "poem.txt", "list.txt"],
  ["missing.txt"],
  ["--words", "poem.txt"],
];

for (const args of attempts) {
  console.log(`$ count ${args.join(" ")}`);
  console.log(`(code ${main(args, readFake)})`);
}
```

```sortie
$ count poem.txt
poem.txt: 6 words
(code 0)
$ count -l poem.txt list.txt
poem.txt: 6 words, 2 lines
list.txt: 4 words, 3 lines
(code 0)
$ count missing.txt
missing.txt: not found
(code 1)
$ count --words poem.txt
Error: Unknown option '--words'. To specify a positional argument starting with a '-', place it at the end of the command after '--', as in '-- "--words"
Usage: count [-l] <file>...
(code 2)
```

With `allowPositionals: true`, Node adds a hint to the message: if you really want a
file named `--words`, type it after `--`.

### Following count step by step

Take the text of `list.txt`: `"bread milk\neggs\n\nbutter\n"` (`\n` is a line break).

| Step | Computation | Value |
|------|-------------|-------|
| 1 | `text.split(/\s+/)`: we cut at each group of spaces or line breaks | `["bread", "milk", "eggs", "butter", ""]` |
| 2 | `.filter((w) => w !== "")`: we remove the empty pieces | `["bread", "milk", "eggs", "butter"]` |
| 3 | `.length` | `words` = 4 |
| 4 | `text.split("\n")`: we cut at each line break | `["bread milk", "eggs", "", "butter", ""]` |
| 5 | `.filter((l) => l !== "")`: we remove the empty lines | `["bread milk", "eggs", "butter"]` |
| 6 | `.length` | `lines` = 3 |

### What the types bring

- `Command` is a [discriminated union](unions.md#discriminated-unions): in the
  `case "count"`, TypeScript knows that `command.files` exists; in the
  `case "error"`, that `command.message` exists.
- `read` returns `string | null`: TypeScript forces `main` to handle the missing file
  before calling `count`.
- `main` returns `number` in **every** case: if you forget a `return`, TypeScript
  flags it.

Try it: add `"empty.txt": ""` to the fake disk and the attempt `["-l", "empty.txt"]`.
You get `empty.txt: 0 words, 0 lines` and code 0.

### Plugging it into the real world

All that is left is to replace the fake disk with real files. The file `count.ts`
contains the types and functions above, then these lines instead of the attempts:

```typescript norun
import { readFileSync } from "node:fs";

function readFile(file: string): string | null {
  try {
    return readFileSync(file, "utf8");
  } catch {
    return null;
  }
}

process.exitCode = main(process.argv.slice(2), readFile);
```

```sh
node count.ts -l poem.txt
echo $?
```

```text
poem.txt: 6 words, 2 lines
0
```

Only these few lines touch the real world (`process.argv`, the disk,
`process.exitCode`). Everything else can be tested with fixed arrays, as above. That is
what makes the tool easy to [test](tests.md).

> **Remember:** separate the "core" (functions that receive arrays and texts) from the
> few lines that read `process.argv` and the disk.

## Common mistakes

### Using a text option as a number

```typescript norun
import { parseArgs } from "node:util";

const { values } = parseArgs({ options: { max: { type: "string" } } });
const max: number = values.max;
```

```text
main.ts(4,7): error TS2322: Type 'string | undefined' is not assignable to type 'number'.
  Type 'undefined' is not assignable to type 'number'.
```

A `string` option gives a text, which can be missing. Give a default value and
convert: `const max = Number(values.max ?? "10");`.

### An uncaught unknown option

```typescript norun
import { parseArgs } from "node:util";

const { values } = parseArgs({ options: { words: { type: "boolean" } } });
console.log(values);
```

```sh
node count.ts --wrods
```

```text
TypeError [ERR_PARSE_ARGS_UNKNOWN_OPTION]: Unknown option '--wrods'
```

TypeScript cannot see anything: the error depends on what the user types. Wrap
`parseArgs` in a `try` / `catch`, show the help and return code 2, as in `parse`.

### Reading the wrong slot of process.argv

```typescript norun
const name = process.argv[0];
console.log(`Hello, ${name}!`);
```

```sh
node greet.ts Ada
```

```text
Hello, /usr/bin/node!
```

No error, but a wrong result: slot 0 is the path of `node`. Use
`process.argv.slice(2)`, then slot 0 of that new array.

## Test yourself

**Question 1:** with the command `node tool.ts 5 cats`, what is
`process.argv.slice(2)`, and what is the type of its first slot?

> **Answer**
>
> `["5", "cats"]`. The first slot is the text `"5"`, of type `string`: you must
> convert it with `Number` to compute with it.

**Question 2:** what is the type of `values.verbose` with
`options: { verbose: { type: "boolean" } }`? And with `default: false`?

> **Answer**
>
> Without `default`: `boolean | undefined`, because the option can be missing. With
> `default: false`: `boolean`.

**Question 3:** write `main(args: string[]): number` that prints the sum of the numbers
it receives and returns 0, or prints `Error: X is not a number` and returns 1. Try it
with `["2", "3"]` then `["2", "x"]`.

> **Answer**
>
> ```typescript
> function main(args: string[]): number {
>   let sum = 0;
>   for (const a of args) {
>     const n = Number(a);
>     if (Number.isNaN(n)) {
>       console.log(`Error: ${a} is not a number`);
>       return 1;
>     }
>     sum += n;
>   }
>   console.log(sum);
>   return 0;
> }
>
> console.log("code:", main(["2", "3"]));
> console.log("code:", main(["2", "x"]));
> ```
>
> ```sortie
> 5
> code: 0
> Error: x is not a number
> code: 1
> ```

## Summary

- `process.argv.slice(2)` gives your arguments: a `string[]`, always text, whose slots
  can be missing.
- `parseArgs` from `node:util` sorts the options; TypeScript infers their types
  (`boolean`, `string`, with `| undefined` without `default`).
- `parseArgs` throws an error for an unknown option: catch it and return code 2.
- The exit code is `0` if all goes well; choose it with
  `process.exitCode = main(…)`.
- `await text(process.stdin)` reads standard input.
- Write the core of the tool with functions that receive arrays: you try them with
  fixed values.

Next: [TypeScript for the web](web.md).
