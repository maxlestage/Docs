# Regular expressions in TypeScript

This page shows what TypeScript adds to regular expressions: results that can be `null`, typed groups and safe parsing functions.

> **Goals:**
>
> - you will know how to write a regex and recognize the `RegExp` type;
> - you will know how to handle the `null` returned by `match` thanks to narrowing;
> - you will know how to read numbered and named groups without type errors;
> - you will know how to replace with a typed function and loop over `matchAll`;
> - you will know how to write a parser that returns a typed object or `null`, and a type
>   guard that validates a format;
> - you will know how to escape text typed in by the user.

## Writing a regex in TypeScript

A [regular expression](../debuter/glossaire.md#regular-expression) (or **regex**)
describes a text pattern. The pattern syntax (`\d`, `+`, `[a-z]`, `^`, `$`…) is
exactly the same as in JavaScript: it is explained in
[regular expressions in JavaScript](../javascript/regex.md). This page doesn't
repeat it: it shows what **types** change.

A regex is written between two slashes, or built with `new RegExp` from a string. In
both cases, its type is `RegExp`.

```typescript
const zipCode: RegExp = /^\d{5}$/;
const word = "cat";
const pattern = new RegExp(word, "i");

console.log(zipCode.test("90210"));
console.log(zipCode.test("902"));
console.log(pattern.test("The CAT is asleep."));
console.log(pattern.source, pattern.flags);
```

```sortie
true
false
true
cat i
```

`test` returns a `boolean`: it is the simplest method, and it causes no type
problems.

> **Note:** TypeScript also checks the **syntax** of a regex written between slashes.
> A missing parenthesis in `/(abc/` is a compile error. With
> `new RegExp("(abc")`, the pattern is just a string: the error only shows up at
> runtime.

## match can return null

`match` looks for the pattern in a string. If it finds it, it returns an array;
otherwise, it returns `null`. Its return type says so: `RegExpMatchArray | null`.

In strict mode, TypeScript won't let you use the result without ruling out `null`
first (see the [common mistakes](#common-mistakes)). A plain `if` is enough: this is
[narrowing](../debuter/glossaire.md#narrowing).

```typescript
function firstNumber(text: string): number | null {
  const m = text.match(/\d+/);
  if (m === null) {
    return null;
  }
  // Here, m is a RegExpMatchArray.
  return Number(m[0]);
}

console.log(firstNumber("There are 12 seats left."));
console.log(firstNumber("Sold out."));
```

```sortie
12
null
```

For a default value, `?.` and `??` are shorter:
`text.match(/\d+/)?.[0] ?? "unknown"`. `?.[0]` only reads slot 0 if the result isn't
`null`.

### With the g flag

With the `g` flag, `match` returns **all** the matches, as an array of strings. But
when there are none, it is still `null`, not an empty array:

```typescript
function numbers(text: string): number[] {
  const found = text.match(/\d+/g) ?? [];
  return found.map(Number);
}

console.log(numbers("3 apples, 12 pears, 7 kiwis"));
console.log(numbers("no fruit"));
```

```sortie
[ 3, 12, 7 ]
[]
```

`?? []` replaces `null` with an empty array: the rest of the code no longer has to
worry about it.

## Flags

Flags go after the last slash: `/pattern/gi`. The JavaScript chapter covers them in
detail; here is a reminder.

| Flag | Effect |
|------|--------|
| `g` | all the matches |
| `i` | ignores upper and lower case |
| `m` | `^` and `$` work on each line |
| `s` | `.` also accepts a line break |
| `u` | reads accents and emojis correctly, enables `\p{…}` |
| `v` | a newer version of `u`, with richer character sets |

```typescript
const firstNames = "Élise, zoé, Ana";
console.log(firstNames.match(/\p{Lu}\p{Ll}+/gu));
console.log(/start.end/s.test("start\nend"));
```

```sortie
[ 'Élise', 'Ana' ]
true
```

`\p{Lu}` is an uppercase letter from any language, `\p{Ll}` a lowercase one: `zoé` is
left out because it starts with a lowercase letter.

> **Warning:** TypeScript refuses the `v` flag if the compilation target (`target` in
> `tsconfig.json`) is older than `es2024`. The `u` flag is enough in almost every
> case.

## Numbered groups

Parentheses in the pattern form **groups**. The text found by the first group is in
`m[1]`, the second in `m[2]`, and so on. `m[0]` holds the whole match.

```typescript
const m = "Meeting at 14h30.".match(/(\d{1,2})h(\d{2})/);
if (m) {
  const hours = Number(m[1]);
  const minutes = Number(m[2]);
  console.log(`${m[0]} = ${hours * 60 + minutes} minutes`);
}
```

```sortie
14h30 = 870 minutes
```

TypeScript gives the type `string` to `m[1]` and `m[2]`. It doesn't count the groups:
it doesn't know how many your pattern contains.

> **Warning:** an **optional** group that found nothing is `undefined`, even though
> TypeScript says `string`. For a group followed by `?`, annotate the variable with
> `string | undefined` (`const cents: string | undefined = m[2];`) and handle the
> missing case.

## Named groups

A named group is written `(?<name>…)`. You read it in `m.groups`, which is easier to
read than numbers. But watch out for the type of `groups`:

```text
{ [key: string]: string } | undefined
```

Two consequences:

- `groups` can be `undefined` (when the pattern has no named group): so you have to
  check it, like `m`;
- TypeScript doesn't know the names of your groups: `groups.year` and `groups.yaer`
  (a typo) both have the type `string`.

```typescript
const pattern = /(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})/;
const m = "Delivery on 2026-10-19.".match(pattern);

if (m?.groups) {
  const { year, month, day } = m.groups;
  console.log(`${day}/${month}/${year}`);
}
```

```sortie
19/10/2026
```

`m?.groups` checks in one go that `m` isn't `null` and that `groups` exists. Inside
the `if`, TypeScript knows that `m.groups` is an object.

## Looping with matchAll

`matchAll` returns an [iterator](../debuter/glossaire.md#iterator): each item is a
complete match, with its groups. The pattern must have the `g` flag. You loop over it
with `for…of`, and each item has the type `RegExpExecArray`.

```typescript
const basket = "apples=3; pears=12; kiwis=7";
let total = 0;

for (const m of basket.matchAll(/(?<fruit>\w+)=(?<quantity>\d+)/g)) {
  const fruit = m.groups?.fruit ?? "?";
  const quantity = Number(m.groups?.quantity ?? 0);
  console.log(`${fruit.padEnd(7)} ${quantity} (position ${m.index})`);
  total += quantity;
}
console.log(`Total: ${total}`);
```

```sortie
apples  3 (position 0)
pears   12 (position 10)
kiwis   7 (position 20)
Total: 22
```

Unlike `match`, `matchAll` never returns `null`: with no match, the iterator is simply
empty. `Array.from(…)` turns it into a real array.

## Replacing with a typed function

`replace` and `replaceAll` accept a function instead of the replacement string. It
receives the match, then one argument per group. When you pass it directly,
TypeScript guesses the types (`string` for each group). If you write the function
separately, type its parameters yourself. Here, we turn dates into the day/month/year
format:

```typescript
function toDayFirst(_whole: string, year: string, month: string, day: string): string {
  return `${day}/${month}/${year}`;
}

const log = "Start: 2026-10-19, end: 2026-12-31.";
console.log(log.replaceAll(/(\d{4})-(\d{2})-(\d{2})/g, toDayFirst));
```

```sortie
Start: 19/10/2026, end: 31/12/2026.
```

The first parameter is the whole match; the `_` in the name signals that we don't use
it.

> **Note:** `replaceAll` with a regex requires the `g` flag, otherwise it throws a
> `TypeError` at runtime. TypeScript doesn't detect it.

## Splitting with split

`split` accepts a regex as the separator. The result is always a `string[]`: no
`null` to handle.

```typescript
const input = "red, green ;blue  ;  yellow";
const colors: string[] = input.split(/\s*[,;]\s*/);
console.log(colors);
```

```sortie
[ 'red', 'green', 'blue', 'yellow' ]
```

The separator "a comma or a semicolon, with spaces around it" cleans up the input in
a single line.

## Writing a typed parser

The big benefit of TypeScript: wrap the regex in a function that turns a string into
a **typed object**. The rest of the program no longer handles strings, but objects
whose every field is checked.

### Returning T or null

Here is a function that reads a date `2026-10-19`. It returns a `SimpleDate` object,
or `null` if the text doesn't have the right shape:

```typescript
interface SimpleDate {
  year: number;
  month: number;
  day: number;
}

function readDate(text: string): SimpleDate | null {
  const m = text.match(/^(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})$/);
  if (!m?.groups) {
    return null;
  }
  const year = Number(m.groups.year);
  const month = Number(m.groups.month);
  const day = Number(m.groups.day);
  if (month < 1 || month > 12 || day < 1 || day > 31) {
    return null;
  }
  return { year, month, day };
}

for (const text of ["2026-10-19", "2026-13-01", "19/10/2026"]) {
  const d = readDate(text);
  console.log(text, "->", d ?? "invalid date");
}
```

```sortie
2026-10-19 -> { year: 2026, month: 10, day: 19 }
2026-13-01 -> invalid date
19/10/2026 -> invalid date
```

The regex checks the **shape** (four digits, a dash…); the code checks the
**meaning** (a month between 1 and 12). Both are useful.

### Returning a discriminated union

When several shapes are possible, return a
[discriminated union](unions.md#discriminated-unions). Here, a log line is a request, an error, or an
unknown line:

```typescript
type Line =
  | { kind: "request"; method: "GET" | "POST"; path: string; duration: number }
  | { kind: "error"; message: string }
  | { kind: "unknown"; text: string };

function readLine(text: string): Line {
  const req = text.match(/^(?<method>GET|POST) (?<path>\S+) (?<duration>\d+)ms$/);
  if (req?.groups) {
    const method = req.groups.method === "GET" ? "GET" : "POST";
    return { kind: "request", method, path: req.groups.path, duration: Number(req.groups.duration) };
  }
  const err = text.match(/^ERROR: (?<message>.+)$/);
  if (err?.groups) {
    return { kind: "error", message: err.groups.message };
  }
  return { kind: "unknown", text };
}

const log = ["GET /home 12ms", "ERROR: database unavailable", "POST /cart 85ms", "???"];

for (const text of log) {
  const line = readLine(text);
  switch (line.kind) {
    case "request":
      console.log(`${line.method} ${line.path} in ${line.duration} ms`);
      break;
    case "error":
      console.log(`Problem: ${line.message}`);
      break;
    case "unknown":
      console.log(`Ignored: ${line.text}`);
      break;
  }
}
```

```sortie
GET /home in 12 ms
Problem: database unavailable
POST /cart in 85 ms
Ignored: ???
```

In each `case`, TypeScript knows which fields exist: `line.duration` is only allowed
for a request. The raw text is parsed in a single place.

## Validating with a type guard

A regex can also act as a **type guard**: a function that returns `v is T` (see
[your own type guards](unions.md#your-own-type-guards)). Here, the `Email` type describes "a string that
contains an `@`", and the regex does the real check:

```typescript
type Email = `${string}@${string}`;

function isEmail(text: string): text is Email {
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(text);
}

function send(recipient: Email) {
  console.log(`Sending to ${recipient}`);
}

for (const input of ["ada@example.com", "ada@", "not an email"]) {
  if (isEmail(input)) {
    send(input);
  } else {
    console.log(`Address rejected: ${input}`);
  }
}
```

```sortie
Sending to ada@example.com
Address rejected: ada@
Address rejected: not an email
```

`send` only accepts an `Email`: you can't pass it an unchecked string by mistake. The
same pattern works for a ZIP code:

```typescript
type ZipCode = `${number}`;

function isZipCode(text: string): text is ZipCode {
  return /^\d{5}$/.test(text);
}

const inputs = ["90210", "9021", "9O210"];
console.log(inputs.filter(isZipCode));
```

```sortie
[ '90210' ]
```

With `filter`, the type guard narrows the whole array: the result has the type
`ZipCode[]`.

> **Tip:** a perfect email regex doesn't exist. Keep it simple ("something, an `@`,
> something, a dot, something") and send a confirmation email to be sure.

## Template literal types or regexes?

[Template literal types](types-avances.md) like `` `${string}@${string}` `` look like
patterns. But they don't play the same role:

| | Template literal type | Regex |
|---|---|---|
| When? | at compile time | at runtime |
| On what? | strings written in the code | any string, even typed in |
| Precision | rough: `${number}` accepts `"1.5"` | fine: `\d{5}` requires 5 digits |

For example, `"99h99"` does have the type `` `${number}h${number}` ``, but the regex
`/^([01]\d|2[0-3])h[0-5]\d$/` refuses it. The type documents and guides; the regex
really checks. For text coming from outside, only the regex (in a type guard) protects
your program.

## Escaping user input

To search for text typed in by the user, you build the regex with `new RegExp`. But if
the text contains `.`, `(`, `+` or `?`, these characters keep their special meaning:
the search gives wrong results, or throws an error.

A `RegExp.escape` method was recently added to JavaScript. It doesn't exist yet in
Node 22, and TypeScript 5.8 doesn't know it (see the
[common mistakes](#common-mistakes)). So write your own function:

```typescript
function escapeRegex(text: string): string {
  return text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function count(text: string, search: string): number {
  const pattern = new RegExp(escapeRegex(search), "gi");
  return text.match(pattern)?.length ?? 0;
}

console.log(escapeRegex("1+1=2 (true?)"));
console.log(count("Price: 3.5 €, or 3.5 € on sale, not 345 €", "3.5"));
```

```sortie
1\+1=2 \(true\?\)
2
```

In the replacement, `$&` stands for the character found: each special character gets
a backslash in front of it. Without `escapeRegex`, the `.` in `3.5` would also accept
`345`.

## Common mistakes

### Using the result of match without checking for null

```typescript norun
const m = "2026-10-19".match(/(\d{4})-(\d{2})/);
console.log(m[1]);
```

```text
main.ts(2,13): error TS18047: 'm' is possibly 'null'.
```

`match` returns `null` when it finds nothing. Check first: `if (m) { … }`, or read
with `m?.[1]`.

### Reading groups without checking for undefined

```typescript norun
const m = "2026-10-19".match(/(?<year>\d{4})/);
if (m) {
  console.log(m.groups.year);
}
```

```text
main.ts(3,15): error TS18048: 'm.groups' is possibly 'undefined'.
```

Checking `m` isn't enough: `groups` can be missing too. Write
`if (m?.groups) { … }`.

### Using the v flag with an old target

```typescript norun
const emoji = /\p{Emoji}/v;
```

```text
main.ts(1,26): error TS1501: This regular expression flag is only available when targeting 'es2024' or later.
```

Set `target` to `es2024` (or later) in `tsconfig.json`, or use the `u` flag:
`/\p{Emoji}/u`.

### Calling RegExp.escape

```typescript norun
const pattern = new RegExp(RegExp.escape("3.5"));
```

```text
main.ts(1,35): error TS2339: Property 'escape' does not exist on type 'RegExpConstructor'.
```

`RegExp.escape` is too recent for TypeScript 5.8 and for Node 22 (where it is
`undefined`). Use your own `escapeRegex` function, seen above.

## Test yourself

**Question 1:** what is the return type of `text.match(/\d+/)`, and what must you do
before reading `[0]`?

> **Answer**
>
> `RegExpMatchArray | null`. You have to rule out `null`: with an `if (m)`, or with
> `m?.[0]` followed by a fallback value `?? …`.

**Question 2:** why is `if (m) { m.groups.name }` refused?

> **Answer**
>
> `groups` has the type `{ [key: string]: string } | undefined`: it can be missing even
> if `m` exists. You have to check `m.groups`, for example with `if (m?.groups)`.

**Question 3:** write a type guard `isTime(text: string): text is Time` for
``type Time = `${number}:${number}` ``, that accepts `08:30` but refuses `25:00`.

> **Answer**
>
> ```typescript
> type Time = `${number}:${number}`;
>
> function isTime(text: string): text is Time {
>   return /^([01]\d|2[0-3]):[0-5]\d$/.test(text);
> }
>
> console.log(isTime("08:30"), isTime("25:00"));
> ```
>
> ```sortie
> true false
> ```

## Summary

- Regex syntax is the same as in JavaScript; TypeScript gives regexes the type
  `RegExp`.
- `match` returns `RegExpMatchArray | null`: rule out `null` with `if`, `?.` or `??`.
- `m.groups` is `{ [key: string]: string } | undefined`: check it, and remember that
  TypeScript doesn't know the names of your groups.
- Wrap each regex in a function that returns a typed object, `T | null` or a
  discriminated union.
- A regex in a `text is T` type guard checks at runtime what types only check at
  compile time.
- Escape any user input before `new RegExp` with your own `escapeRegex` function.

Next: [dates and times](dates.md).
