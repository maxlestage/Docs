# Cheat sheet

This page sums up at a glance the TypeScript syntax seen throughout the track, with a short example for each topic.

## Basic types

| Item | Example |
| --- | --- |
| Constant, variable | `const name = "Alice";`, `let age = 30;` |
| Annotation | `let age: number = 30;` |
| Numbers | `number`: `42`, `3.14`, `NaN`; `bigint`: `10n` |
| Text | `string`: `"a"`, `'a'`, `` `Hello ${name}` `` |
| Boolean | `boolean`: `true`, `false` |
| Absence | `undefined`, `null` |
| Anything, checked | `unknown` (narrow it before use) |
| Anything, unchecked | `any` (avoid it) |
| Returns nothing | `void` |
| Never happens | `never` |
| Comment | `// line`, `/* block */` |
| Print | `console.log(a, b)` |

```typescript
const name = "Alice";
let age: number = 30;
age += 1;
const mystery: unknown = "text";
if (typeof mystery === "string") console.log(mystery.length);
console.log(`${name} is ${age} years old`, typeof age, 7 / 2, Math.floor(7 / 2));
```

```sortie
4
Alice is 31 years old number 3.5 3
```

Details: [first steps](premiers-pas.md).

## Conditions and loops

| Need | Syntax |
| --- | --- |
| Condition | `if (…) { … } else if (…) { … } else { … }` |
| Ternary | `x > 0 ? "yes" : "no"` |
| Strict equality | `===`, `!==` (never `==`) |
| Logic | `&&`, `\|\|`, `!` |
| Default value | `x ?? "default"` (if `null` or `undefined`) |
| Safe access | `obj?.prop`, `f?.()` |
| Several cases | `switch (x) { case 1: …; break; default: … }` |
| Loops | `for (let i = 0; i < n; i++)`, `while (…)`, `for (const x of arr)` |
| Index and value | `for (const [i, x] of arr.entries())` |
| Control | `break`, `continue` |

```typescript
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

| Item | Syntax |
| --- | --- |
| Declaration | `function f(a: number, b: number): number { return a + b; }` |
| Arrow | `const f = (a: number): number => a * 2;` |
| Optional parameter | `function f(name?: string)` |
| Default value | `function f(greeting = "Hello")` |
| Rest | `function sum(...n: number[])` |
| Function type | `type Op = (a: number, b: number) => number;` |
| No return value | `function log(m: string): void` |

```typescript
type Op = (a: number, b: number) => number;

function greet(name: string, greeting = "Hello", polite?: boolean): string {
  const text = `${greeting} ${name}`;
  return polite ? `${text}, nice to meet you.` : text;
}
const sum = (...n: number[]): number => n.reduce((a, b) => a + b, 0);
const times: Op = (a, b) => a * b;

console.log(greet("Alice"));
console.log(greet("Bob", "Hi", true));
console.log(sum(1, 2, 3), times(6, 7));
```

```sortie
Hello Alice
Hi Bob, nice to meet you.
6 42
```

Details: [functions](fonctions.md).

## Text

| Need | Method |
| --- | --- |
| Length | `s.length` |
| Uppercase, lowercase | `s.toUpperCase()`, `s.toLowerCase()` |
| Surrounding spaces | `s.trim()` |
| Contains, starts with | `s.includes("x")`, `s.startsWith("x")` |
| Split, join | `s.split(",")`, `arr.join(", ")` |
| Replace | `s.replaceAll("a", "b")` |
| Piece | `s.slice(0, 3)` |
| Align | `s.padEnd(10)`, `s.padStart(5, "0")` |
| Number → text | `String(n)`, `n.toFixed(2)` |
| Text → number | `Number("12")` (`NaN` if invalid) |

```typescript
const s = "  Hello big world  ";
console.log(s.trim().toUpperCase());
console.log(s.trim().split(" "));
console.log("7".padStart(3, "0"), (2 / 3).toFixed(2), Number("12") + 1);
```

```sortie
HELLO BIG WORLD
[ 'Hello', 'big', 'world' ]
007 0.67 13
```

Details: [text](texte.md).

## Arrays, tuples and Map

| Item | Syntax |
| --- | --- |
| Array | `const a: number[] = [1, 2];` or `Array<number>` |
| Read-only | `readonly number[]` |
| Tuple | `const p: [string, number] = ["Alice", 30];` |
| Add, remove | `a.push(x)`, `a.pop()` |
| Transform | `a.map((x) => x * 2)` |
| Filter | `a.filter((x) => x > 0)` |
| Find | `a.find(…)` → `T \| undefined`, `a.includes(x)` |
| Reduce | `a.reduce((acc, x) => acc + x, 0)` |
| Sort a copy | `[...a].sort((x, y) => x - y)` |
| Destructure | `const [first, ...rest] = a;` |
| Dictionary | `new Map<string, number>()`, `get`, `set`, `has` |
| Set | `new Set<string>()`, `add`, `has`, `size` |

```typescript
const grades = [12, 9, 15, 18];
const [first, ...others] = grades;
console.log(grades.filter((n) => n >= 10).map((n) => n * 2));
console.log([...grades].sort((a, b) => b - a), first, others.length);

const person: [string, number] = ["Alice", 30];
const ages = new Map<string, number>([["Alice", 30]]);
ages.set("Bob", 25);
console.log(person[0], ages.get("Bob"), ages.get("Zoe") ?? "unknown");
```

```sortie
[ 24, 30, 36 ]
[ 18, 15, 12, 9 ] 12 3
Alice 25 unknown
```

Details: [collections](collections.md).

## Objects and interfaces

| Item | Syntax |
| --- | --- |
| Interface | `interface Point { x: number; y: number }` |
| Type alias | `type Point = { x: number; y: number };` |
| Optional property | `name?: string` |
| Read-only | `readonly id: number` |
| Extension | `interface Point3D extends Point { z: number }` |
| Dictionary | `Record<string, number>` or `{ [key: string]: number }` |
| Destructure | `const { x, y } = point;` |
| Copy with changes | `{ ...point, x: 5 }` |
| Keys, values | `Object.keys(o)`, `Object.entries(o)` |

```typescript
interface Person {
  readonly id: number;
  name: string;
  email?: string;
}

const alice: Person = { id: 1, name: "Alice" };
const changed = { ...alice, name: "Alice B." };
const { name, email } = changed;
console.log(name, email ?? "no email", alice.name);
console.log(Object.entries({ math: 15, art: 12 }));
```

```sortie
Alice B. no email Alice
[ [ 'math', 15 ], [ 'art', 12 ] ]
```

Details: [objects](objets.md).

## Unions and narrowing

| Need | Syntax |
| --- | --- |
| Union | `string \| number` |
| May be missing | `string \| undefined` |
| Test a simple type | `typeof x === "string"` |
| Test a class | `x instanceof Date` |
| Test a property | `"radius" in shape` |
| Test presence | `if (x !== undefined)`, `if (x)` |
| Discriminated union | `{ type: "a"; … } \| { type: "b"; … }` then `switch (v.type)` |
| Type guard | `function isX(v: unknown): v is X` |
| Exhaustiveness | `const rest: never = v;` in the `default` |

```typescript
type Shape = { type: "circle"; radius: number } | { type: "square"; side: number };

function area(s: Shape): number {
  switch (s.type) {
    case "circle":
      return Math.PI * s.radius ** 2;
    case "square":
      return s.side ** 2;
    default: {
      const rest: never = s;
      return rest;
    }
  }
}

function format(v: string | number): string {
  return typeof v === "string" ? v.toUpperCase() : v.toFixed(1);
}

console.log(area({ type: "square", side: 3 }), format("ok"), format(2));
```

```sortie
9 OK 2.0
```

Details: [unions](unions.md).

## Classes

| Item | Syntax |
| --- | --- |
| Class | `class Cat { … }`, `new Cat("Tom")` |
| Property | `name: string;` (must be initialized) |
| Constructor shorthand | `constructor(private name: string) {}` |
| Visibility | `public` (default), `private`, `protected`, `#secret` |
| Read-only | `readonly id: number` |
| Inheritance | `class Cat extends Animal`, `super(…)` |
| Contract | `class Cat implements Animal` |
| Static | `static create(): Cat` |
| Getter | `get name(): string { … }` |

```typescript
interface Animal {
  speak(): string;
}

class Cat implements Animal {
  static count = 0;
  #lives = 9;

  constructor(readonly name: string) {
    Cat.count++;
  }

  get lives(): number {
    return this.#lives;
  }

  speak(): string {
    return `${this.name}: meow`;
  }
}

const tom = new Cat("Tom");
console.log(tom.speak(), tom.lives, Cat.count);
```

```sortie
Tom: meow 9 1
```

Details: [classes](types.md).

## Generics

| Item | Syntax |
| --- | --- |
| Generic function | `function first<T>(a: T[]): T \| undefined` |
| Generic type | `type Box<T> = { value: T };` |
| Constraint | `<T extends { length: number }>` |
| Several parameters | `function pair<A, B>(a: A, b: B): [A, B]` |
| Default value | `<T = string>` |
| Specify at call time | `first<number>([])` (rarely useful) |

```typescript
function first<T>(a: readonly T[]): T | undefined {
  return a[0];
}

function longer<T extends { length: number }>(a: T, b: T): T {
  return a.length >= b.length ? a : b;
}

type Box<T> = { value: T };
const b: Box<number> = { value: 42 };

console.log(first(["a", "b"]), longer("cat", "dog"), longer([1, 2], [3]), b.value);
```

```sortie
a cat [ 1, 2 ] 42
```

Details: [generics](generiques.md).

## Literal types

| Item | Syntax |
| --- | --- |
| Literal | `type Yes = "yes";` |
| Union of literals | `type Size = "S" \| "M" \| "L";` |
| Freeze a value | `const S = ["S", "M"] as const;` |
| Union from an array | `type Size = (typeof S)[number];` |
| Template literal type | `` type Id = `user-${number}`; `` |
| Check without widening | `const c = { … } satisfies Config;` |

```typescript
const SIZES = ["S", "M", "L"] as const;
type Size = (typeof SIZES)[number];
type Id = `user-${number}`;

const s: Size = "M";
const id: Id = "user-42";
console.log(s, id, SIZES.length);
```

```sortie
M user-42 3
```

Details: [literal types](litteraux.md).

## Utility types and advanced types

| Type | Effect |
| --- | --- |
| `Partial<T>` | all properties optional |
| `Required<T>` | all properties required |
| `Readonly<T>` | all properties read-only |
| `Pick<T, "a" \| "b">` | only some properties |
| `Omit<T, "a">` | all except some |
| `Record<K, V>` | an object with keys `K` and values `V` |
| `ReturnType<typeof f>` | the return type of a function |
| `Awaited<P>` | the type you get after `await` |
| `keyof T` | the union of the property names |
| `T[K]` | the type of a property |
| `typeof x` | the type of a value |
| `T extends U ? A : B` | conditional type |
| `{ [K in keyof T]: … }` | mapped type |

```typescript
interface Person {
  name: string;
  age: number;
}

function update(p: Person, changes: Partial<Person>): Person {
  return { ...p, ...changes };
}

type Summary = Pick<Person, "name">;
type Key = keyof Person;

const alice: Readonly<Person> = { name: "Alice", age: 30 };
const r: Summary = { name: alice.name };
const key: Key = "age";
console.log(update(alice, { age: 31 }), r, alice[key]);
```

```sortie
{ name: 'Alice', age: 31 } { name: 'Alice' } 30
```

Details: [advanced types](types-avances.md).

## Errors

| Need | Syntax |
| --- | --- |
| Throw | `throw new Error("message")` |
| Catch | `try { … } catch (e) { … } finally { … }` |
| Type of `e` | `unknown`: test `e instanceof Error` |
| Error class | `class MyError extends Error { … }` |
| Result without exceptions | `type R<T> = { ok: true; value: T } \| { ok: false; error: string }` |

```typescript
class InvalidInput extends Error {
  override name = "InvalidInput";
}

function readAge(text: string): number {
  const n = Number(text);
  if (!Number.isInteger(n)) throw new InvalidInput(`"${text}" is not an integer`);
  return n;
}

for (const input of ["34", "abc"]) {
  try {
    console.log(readAge(input));
  } catch (e) {
    if (e instanceof Error) console.log(`${e.name}: ${e.message}`);
  }
}
```

```sortie
34
InvalidInput: "abc" is not an integer
```

Details: [errors](erreurs.md).

## Asynchronous code

| Need | Syntax |
| --- | --- |
| Async function | `async function f(): Promise<string>` |
| Wait | `const x = await f();` |
| Several in parallel | `await Promise.all([f(), g()])` |
| Even if some fail | `await Promise.allSettled([…])` |
| The first one done | `await Promise.race([…])` |
| Wait for a delay | `await setTimeout(100)` with `node:timers/promises` |
| Async error | `try { await f(); } catch (e) { … }` |

```typescript
import { setTimeout as wait } from "node:timers/promises";

async function load(name: string, ms: number): Promise<string> {
  await wait(ms);
  return `${name} loaded`;
}

const results = await Promise.all([load("profile", 20), load("grades", 10)]);
console.log(results);
```

```sortie
[ 'profile loaded', 'grades loaded' ]
```

`Promise.all` keeps the order of the array, even if `grades` finishes first. Details:
[asynchronous code](asynchrone.md).

## Modules

| Need | Syntax |
| --- | --- |
| Export | `export function f() {}`, `export const X = 1;` |
| Default export | `export default class Cart {}` |
| Import | `import { f, X } from "./tools.js";` |
| Rename | `import { f as g } from "./tools.js";` |
| Import everything | `import * as tools from "./tools.js";` |
| Import a type | `import type { Person } from "./types.js";` |
| Node module | `import { readFile } from "node:fs/promises";` |

```typescript norun
// tools.ts
export interface Person {
  name: string;
}
export function greet(p: Person): string {
  return `Hello ${p.name}`;
}

// main.ts
import { greet, type Person } from "./tools.js";
const alice: Person = { name: "Alice" };
console.log(greet(alice));
```

The import ends with `.js`, the name of the **compiled** file. Details:
[modules](modules.md).

## Files and JSON

| Need | Syntax (`node:fs/promises`) |
| --- | --- |
| Write | `await writeFile("f.txt", text, "utf8")` |
| Read | `await readFile("f.txt", "utf8")` |
| Append | `await appendFile("f.txt", text)` |
| Missing file | error with code `"ENOENT"` |
| To JSON | `JSON.stringify(v, null, 2)` |
| From JSON | `JSON.parse(t)`: store it in an `unknown` |
| Arguments | `process.argv.slice(2)` |

```typescript
import { appendFile, readFile, writeFile } from "node:fs/promises";

await writeFile("f.txt", "one\n", "utf8");
await appendFile("f.txt", "two\n");
console.log((await readFile("f.txt", "utf8")).trim().split("\n"));

const data: unknown = JSON.parse(JSON.stringify({ ok: true }));
if (typeof data === "object" && data !== null && "ok" in data) {
  console.log(data.ok);
}
```

```sortie
[ 'one', 'two' ]
true
```

Details: [files](fichiers.md), [formats](formats.md).

## tsconfig.json

| Option | Role |
| --- | --- |
| `"strict": true` | all the strict checks (essential) |
| `"target": "es2022"` | version of the JavaScript produced |
| `"module": "nodenext"` | modules the Node way |
| `"rootDir": "src"` | source folder |
| `"outDir": "dist"` | folder for the compiled JavaScript |
| `"sourceMap": true` | `.js.map` files for debugging |
| `"noUncheckedIndexedAccess": true` | `a[i]` is `T \| undefined` |
| `"noEmit": true` | check without writing anything |
| `"skipLibCheck": true` | don't re-check installed packages |
| `"include": ["src"]` | files to compile |

Details: [configuration](configuration.md).

## Commands

| Command | Role |
| --- | --- |
| `npm init -y` | create a `package.json` |
| `npm install -D typescript @types/node tsx` | install the tools |
| `npx tsc` | check and compile according to `tsconfig.json` |
| `npx tsc --noEmit` | check the types only |
| `npx tsc --init` | create a `tsconfig.json` |
| `npx tsx file.ts` | run directly (without checking the types) |
| `npx tsx watch file.ts` | rerun on every change |
| `node --enable-source-maps dist/main.js` | run with TypeScript lines in the errors |
| `npx tsx --test` | run the `node:test` tests |
| `npx eslint src/`, `npx prettier --write src/` | check and format |

Details: [organizing a project](projet.md), [tests](tests.md),
[debugging](debogage.md), [writing good TypeScript](style.md). To practice:
[step-by-step projects](projets.md) and [solved exercises](exercices.md). Compare
with [the Ruby cheat sheet](../ruby/aide-memoire.md) and
[the Rust cheat sheet](../rust/aide-memoire.md).
