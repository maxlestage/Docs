# First steps

This page presents the basic building blocks of TypeScript: variables, simple types, printing, comments and how to read a type error.

> **Goals:**
>
> - you will know how to create a variable with `let` or `const`;
> - you will know how to annotate a variable with `number`, `string` or `boolean`;
> - you will know when TypeScript guesses the type on its own;
> - you will know how to use `null`, `undefined`, `any` and `unknown`;
> - you will know how to read a type error line by line.

## Printing with console.log

`console.log` prints a value in the terminal, then moves to the next line. It accepts
several values, separated by commas: it prints them with a space between them.

```typescript
console.log("Hello!");
console.log(2 + 3);
console.log("Total:", 2 + 3, "euros");
```

```sortie
Hello!
5
Total: 5 euros
```

Text is written between double quotes `"..."` or single quotes `'...'`. Without quotes,
TypeScript computes: `2 + 3` gives `5`.

## Variables: let and const

A [variable](../debuter/glossaire.md#variable) is a box with a label: the label is the
name, the content is the value. Two words let you create one:

- `const`: a **sealed** box, its value never changes;
- `let`: an **open** box, you can replace its value.

```typescript
const firstName = "Alice";
let score = 10;
console.log(firstName, score);

score = 15;
score = score + 5;
console.log(firstName, score);
```

```sortie
Alice 10
Alice 20
```

`score = score + 5` reads from right to left: first compute `score + 5` (that is `20`),
then store the result in `score`.

> **Remember:** use `const` by default. Switch to `let` only if the value has to
> change. That way, the reader knows right away what moves in the program.

A variable name starts with a letter, with no spaces (and accents are best avoided).
Compound names are written in `camelCase`: `totalPrice`, `numberOfPlayers`.

> **Note:** you will sometimes see `var` in old programs. It is the ancestor of
> `let`, with trickier rules. Don't write it.

## Simple types

Every value has a [type](../debuter/glossaire.md#type). The three basic types are:

| Type | Content | Examples |
|------|---------|----------|
| `number` | a number, whole or decimal | `42`, `-3`, `3.14` |
| `string` | some text (a [string](../debuter/glossaire.md#string)) | `"Alice"`, `'yes'` |
| `boolean` | true or false (a [boolean](../debuter/glossaire.md#boolean)) | `true`, `false` |

You can write the type after the name, with a colon: this is an **annotation**.

```typescript
const age: number = 30;
const city: string = "Lyon";
const isAdult: boolean = true;

console.log(age, city, isAdult);
console.log(typeof age, typeof city, typeof isAdult);
```

```sortie
30 Lyon true
number string boolean
```

`typeof` gives the type of a value while the program runs, as text.

### Calculating with numbers

There is only one type for all numbers: `number`. The common operations:

```typescript
console.log(7 + 2, 7 - 2, 7 * 2);
console.log(7 / 2);
console.log(7 % 2);
console.log(2 ** 10);
console.log(Math.round(3.6), Math.floor(3.6));
console.log((10 / 3).toFixed(2));
```

```sortie
9 5 14
3.5
1
1024
4 3
3.33
```

- `/` is a real division: `7 / 2` gives `3.5`.
- `%` gives the remainder of the division (`7 = 2 × 3 + 1`).
- `**` is the power: `2 ** 10` is 1024.
- `Math.round` rounds, `Math.floor` rounds down.
- `toFixed(2)` gives a text with 2 digits after the decimal point.

## Inference: TypeScript guesses

You don't have to write the type. When a variable gets a value as soon as it is
created, TypeScript **guesses** its type: this is **inference**.

```typescript
let score = 10;
const message = "You win";

score = score * 2;
console.log(score, message);
```

```sortie
20 You win
```

TypeScript worked out that `score` is a `number` and `message` a `string`. In your
editor, hover over `score`: it shows `let score: number`.

A guessed type is just as solid as a type written by hand. Trying to store text in
`score` gives an error:

```typescript norun
let score = 10;
score = "twenty";
```

```text
main.ts:2:1 - error TS2322: Type 'string' is not assignable to type 'number'.

2 score = "twenty";
  ~~~~~


Found 1 error in main.ts:2
```

> **Tip:** let TypeScript guess when the value is obvious (`let score = 10`). Write
> the type when the variable is created without a value, or when you want to make
> your intention clear.

### const and precise types

With `const`, TypeScript goes further. Since the value will never change, it keeps
the exact value as the type:

```typescript
const color = "red";
let otherColor = "red";

console.log(color, otherColor);
```

```sortie
red red
```

Hover over both names in your editor: `color` has the type `"red"` (that text and no
other), `otherColor` has the type `string`. These precise types will be very useful
in [the chapter on text](texte.md).

## Template literals

Text between backticks `` ` `` is a **template literal**. It can contain `${...}`:
the expression between the braces is computed and inserted into the text.

```typescript
const firstName = "Alice";
const age = 30;

console.log(`${firstName} is ${age} years old.`);
console.log(`In 5 years, she will be ${age + 5}.`);
console.log(`Line 1
Line 2`);
```

```sortie
Alice is 30 years old.
In 5 years, she will be 35.
Line 1
Line 2
```

A template literal can also span several lines.

With normal quotes, you have to glue the pieces together with `+`:

```typescript
const firstName = "Alice";
console.log("Hello, " + firstName + "!");
```

```sortie
Hello, Alice!
```

## Comments

A [comment](../debuter/glossaire.md#comment) is a note for humans: TypeScript
ignores it.

```typescript
// A one-line comment.
const vat = 0.2; // It can also follow code.

/*
  A comment
  on several lines.
*/

/** The price before tax, in euros. */
const netPrice = 100;

console.log(netPrice * (1 + vat));
```

```sortie
120
```

The `/** ... */` comment is a documentation comment: your editor shows it when you
hover over `netPrice`.

> **Tip:** explain **why** you do something, not what the code does.
> `// French VAT` is useful; `// multiply by 0.2` is not.

## null and undefined: no value

Two special values represent "nothing" (see [null](../debuter/glossaire.md#null)):

- `undefined`: "no value yet". This is what a `let` variable created without a value
  holds;
- `null`: "empty on purpose". You write it yourself.

In strict mode (`--strict`, the one used on this site), a `string` can **not** hold
`null`. If a value can be missing, you have to say so with a bar `|`, which means
"or":

```typescript
let nickname: string | null = null;
console.log(nickname);

nickname = "Ali";
console.log(nickname);

let answer: number | undefined;
console.log(answer);
```

```sortie
null
Ali
undefined
```

`string | null` reads "a string or null". It is a **union** of types: you will see
many of them in [the chapter on unions](unions.md).

> **Remember:** in strict TypeScript, a missing value shows in the type. You cannot
> forget it: this is the cause of a great many bugs in other languages.

## any and unknown: when you don't know

Sometimes you don't know the type of a value (data coming from a file, for example).
Two types exist for that, and they are very different.

### any: switching the check off

`any` means "anything, check nothing". TypeScript accepts everything, even mistakes:

```typescript norun
let value: any = "hello";
console.log(value.toFixed(2));
```

`tsc` says nothing. But when you run it, the program crashes (excerpt of the message):

```text
TypeError: value.toFixed is not a function
```

`toFixed` doesn't exist on text. With `any`, you have lost all of TypeScript's
protection.

### unknown: check before you use

`unknown` means "I don't know yet". You can store anything in it, but to use it, you
must first **check** what it is:

```typescript
const data: unknown = "hi";

if (typeof data === "string") {
  console.log(data.toUpperCase());
}
```

```sortie
HI
```

Inside the `if` block, TypeScript knows that `data` is a `string`, and allows
`toUpperCase()`. Conditions are explained in
[the next chapter](conditions-boucles.md).

> **Remember:** avoid `any`. If you don't know the type, use `unknown`: it is a
> closed parcel that you open carefully, not a parcel you throw around without
> looking.

## Reading a type error, line by line

Here is a program that adds a number and a piece of text:

```typescript norun
let total: number = 0;
const input = "42";
total = total + input;
```

```text
main.ts:3:1 - error TS2322: Type 'string' is not assignable to type 'number'.

3 total = total + input;
  ~~~~~


Found 1 error in main.ts:3
```

Let's read it in order:

1. `main.ts:3:1`: the problem is in `main.ts`, line 3, column 1.
2. `error TS2322`: the error code. TS2322 is the most common one: "wrong type for
   this place".
3. `Type 'string' is not assignable to type 'number'`: a `string` cannot be stored
   in a `number`.
4. The line copied out, with `~~~~~` under `total`: that is the box refusing the value.
5. `Found 1 error`: only one error in total.

Let's follow the reasoning back: `input` is text (`"42"`). So `total + input` glues a
number and a text together, which gives **text** (`"042"`). And `total` only accepts
numbers. The fix: turn the text into a number with `Number(...)`.

```typescript
let total: number = 0;
const input = "42";
total = total + Number(input);
console.log(total);
```

```sortie
42
```

> **Tip:** when the message is long, look for the last sentence: it describes the
> most precise cause.

## Common mistakes

### Changing a constant

```typescript norun
const score = 10;
score = 20;
```

```text
main.ts:2:1 - error TS2588: Cannot assign to 'score' because it is a constant.

2 score = 20;
  ~~~~~


Found 1 error in main.ts:2
```

You cannot change `score`: it is a constant. If the value has to change, create the
variable with `let score = 10;`.

### Using a variable before giving it a value

```typescript norun
let city: string;
console.log(city);
```

```text
main.ts:2:13 - error TS2454: Variable 'city' is used before being assigned.

2 console.log(city);
              ~~~~


Found 1 error in main.ts:2
```

`city` is declared, but empty. Give it a value before using it:
`let city: string = "Lyon";`.

### Storing null in a string

```typescript norun
let nickname: string = null;
```

```text
main.ts:1:5 - error TS2322: Type 'null' is not assignable to type 'string'.

1 let nickname: string = null;
      ~~~~~~~~


Found 1 error in main.ts:1
```

In strict mode, `string` means "always a string". If the value can be missing, say
so: `let nickname: string | null = null;`.

### Using an unknown value without checking

```typescript norun
const data: unknown = "hi";
console.log(data.toUpperCase());
```

```text
main.ts:2:13 - error TS18046: 'data' is of type 'unknown'.

2 console.log(data.toUpperCase());
              ~~~~


Found 1 error in main.ts:2
```

TypeScript refuses: `data` could be a number. Check first with
`if (typeof data === "string") { ... }`.

## Test yourself

**Question 1:** what is the type of `x` after `let x = 3.5;`?

> **Answer**
>
> `number`. TypeScript guesses it (inference): there is only one type for all
> numbers, whole or decimal.

**Question 2:** what does this code print?

```typescript norun
const a = 4;
console.log(`${a} x 2 = ${a * 2}`);
```

> **Answer**
>
> `4 x 2 = 8`. Each `${...}` is computed and inserted into the template.

**Question 3:** what is the difference between `any` and `unknown`?

> **Answer**
>
> Both accept any value. But `any` switches the check off: everything is allowed,
> even mistakes. `unknown` forces you to check the type (with `typeof`, for example)
> before using the value.

## Summary

- `const` creates a fixed variable, `let` a variable you can change; prefer `const`.
- The basic types are `number`, `string` and `boolean`; you write them after `:`.
- TypeScript guesses the type of a variable that gets a value: this is inference.
- In strict mode, `null` and `undefined` must appear in the type:
  `string | null`.
- Prefer `unknown` to `any`: it forces you to check before using.
- A template `` `...${x}...` `` inserts values into text; `//` starts a
  comment.

Next: [conditions and loops](conditions-boucles.md).
