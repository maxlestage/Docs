# Conditions and loops

This page explains how to make choices with `if`, `switch` and the ternary operator, how to repeat actions with loops, and what types add each time.

> **Goals:**
>
> - you will know how to write a condition with `if`, `else if` and `else`;
> - you will know how to compare with `===` and recognize "falsy" values;
> - you will know how to handle a missing value with `??` and `?.`;
> - you will know how to write a `switch` that never forgets a case, thanks to `never`;
> - you will know how to repeat with `for`, `while` and `for...of`.

## if and else

A [condition](../debuter/glossaire.md#condition) chooses which code to run. `if`
runs its block `{ ... }` only if the test in parentheses is true. `else` gives the
block to run otherwise.

```typescript
const temperature: number = 28;

if (temperature > 25) {
  console.log("It's hot.");
} else {
  console.log("It's nice.");
}
```

```sortie
It's hot.
```

For more than two cases, chain them with `else if`. The tests are read in order; the
first true one wins, the others are ignored.

```typescript
const grade: number = 13;

if (grade >= 16) {
  console.log("Very good");
} else if (grade >= 12) {
  console.log("Good");
} else if (grade >= 10) {
  console.log("Pass");
} else {
  console.log("Fail");
}
```

```sortie
Good
```

## Comparing with ===

Comparison operators give a `boolean` (`true` or `false`):

| Operator | Meaning |
|-----------|------|
| `===` | equal to |
| `!==` | not equal to |
| `<`, `<=` | less than, less than or equal |
| `>`, `>=` | greater than, greater than or equal |

You combine them with `&&` ("and"), `||` ("or") and `!` ("not"):

```typescript
const age: number = 20;
const hasTicket: boolean = true;

console.log(age === 20, age !== 20);
console.log(age >= 18 && hasTicket);
console.log(age < 12 || age > 65);
console.log(!hasTicket);
```

```sortie
true false
true
false
false
```

> **Warning:** always write `===` (three signs), never `==` (two). `==` converts
> the values before comparing: `0 == ""` is `true` in JavaScript!
> And a single `=` doesn't compare: it stores a value in a variable.

TypeScript keeps an eye on your comparisons. Comparing a number with a piece of text
is always false: it is surely a mistake, and it tells you so (see the
[common mistakes](#common-mistakes)).

## "Truthy" and "falsy" values

An `if` accepts any value, not just a `boolean`. Six values count as false
(*falsy*):

- `false`, `0`, `NaN` ("not a number"), `""` (empty text), `null` and `undefined`.

All the others count as true (*truthy*), even `"0"`, `" "` or `[]`.

```typescript
const text = "";

if (text) {
  console.log("The text is filled in.");
} else {
  console.log("The text is empty.");
}

console.log(Boolean(0), Boolean(""), Boolean(null));
console.log(Boolean("0"), Boolean(" "), Boolean([]));
```

```sortie
The text is empty.
false false false
true true true
```

`Boolean(x)` shows how `x` is seen by an `if`.

> **Tip:** when you can, write the full test: `text !== ""` or `count !== 0`. The
> reader understands right away what you are testing.

## The ternary operator

The ternary `test ? ifTrue : ifFalse` is an `if/else` that **gives a value**. It fits
on one line.

```typescript
const age: number = 15;
const status = age >= 18 ? "adult" : "minor";
console.log(status);

const points: number = 1;
console.log(`${points} point${points > 1 ? "s" : ""}`);
```

```sortie
minor
1 point
```

Read it like a question: "age ≥ 18? then "adult", otherwise "minor"".

> **Tip:** keep the ternary for simple choices. If you need to nest it, an
> `if/else` is easier to read.

## switch: choosing among several values

`switch` compares a value to a list of cases (`case`). It runs the code of the
matching case. `break` leaves the `switch`; `default` handles all other cases.

```typescript
const day: string = "saturday";

switch (day) {
  case "saturday":
  case "sunday":
    console.log("Weekend!");
    break;
  case "wednesday":
    console.log("Midweek.");
    break;
  default:
    console.log("Work day.");
}
```

```sortie
Weekend!
```

Two `case` lines in a row share the same code: `saturday` and `sunday` both print
"Weekend!".

> **Warning:** without `break`, execution goes on into the next case. Forgetting
> `break` is a classic trap.

## Loops

A [loop](../debuter/glossaire.md#loop) repeats a block of code.

### for: counting

The `for` loop has three parts, separated by semicolons: the start, the condition to
keep going, and the step.

```typescript
for (let i = 1; i <= 3; i++) {
  console.log(`Round ${i}`);
}
```

```sortie
Round 1
Round 2
Round 3
```

- `let i = 1`: the counter starts at 1;
- `i <= 3`: we keep going as long as `i` is at most 3;
- `i++`: after each round, we add 1 to `i`.

### while: as long as

`while` repeats as long as its condition is true. Use it when you don't know in
advance how many rounds you need.

```typescript
let money = 100;
let years = 0;

while (money < 150) {
  money = money * 1.1;
  years++;
}

console.log(`${years} years, ${money.toFixed(2)} euros`);
```

```sortie
5 years, 161.05 euros
```

> **Warning:** if the condition stays true forever, the loop never stops. Make sure
> something changes in each round. To stop a stuck program, press `Ctrl+C` in the
> terminal.

### for...of: going through a list

`for...of` takes the items of a list one by one. The list is written between square
brackets; [arrays](collections.md) are explained later.

```typescript
const fruits = ["apple", "pear", "kiwi"];

for (const fruit of fruits) {
  console.log(fruit.toUpperCase());
}
```

```sortie
APPLE
PEAR
KIWI
```

TypeScript knows that `fruits` holds `string`s: so `fruit` is a `string`, and
`toUpperCase()` is allowed. A `const` is enough: each round creates a new `fruit`.

### break and continue

- `break` leaves the loop right away;
- `continue` jumps straight to the next round.

```typescript
for (let i = 1; i <= 10; i++) {
  if (i % 2 === 0) {
    continue;
  }
  if (i > 7) {
    break;
  }
  console.log(i);
}
```

```sortie
1
3
5
7
```

Even numbers are skipped (`continue`), and the loop stops at 9 (`break`).

## Narrowing: types follow your tests

Here is what types bring to conditions. When a variable can have several types, a
test **narrows** its type inside the block (*narrowing*). It's like a guard at the
door of a room: once you're past, we know who is inside.

```typescript
const answers: (string | number)[] = ["yes", 3.14159];

for (const answer of answers) {
  if (typeof answer === "string") {
    console.log(answer.toUpperCase(), "is a string");
  } else {
    console.log(answer.toFixed(2), "is a number");
  }
}
```

```sortie
YES is a string
3.14 is a number
```

`(string | number)[]` is a list where each item is a string or a number.

In the first block, TypeScript knows that `answer` is a `string`: it allows
`toUpperCase()`. In the `else`, it knows it is a `number`: it allows
`toFixed()`. No need to tell it again.

## Missing values: ?? and ?.

A value of type `string | undefined` may be missing. Two operators help you handle it
safely.

### ??: a default value

`a ?? b` gives `a`, unless `a` is `null` or `undefined`: then it gives `b`.

```typescript
const volumes: (number | null)[] = [7, 0, null];

for (const volume of volumes) {
  console.log(volume ?? 5, volume || 5);
}
```

```sortie
7 7
0 5
5 5
```

Compare the two columns: `||` replaces **all** falsy values, including `0`. `??` only
replaces `null` and `undefined`. A volume of `0` is a real setting: `??` keeps it.

> **Remember:** for a default value, prefer `??` to `||`.

### ?.: calling only if the value exists

`x?.action()` calls `action()` only if `x` exists. Otherwise, the result is
`undefined`, with no error.

```typescript
const inputs: (string | undefined)[] = ["alice", undefined];

for (const input of inputs) {
  console.log(input?.toUpperCase());
  console.log(input?.toUpperCase() ?? "(nothing)");
}
```

```sortie
ALICE
ALICE
undefined
(nothing)
```

Without the `?.`, TypeScript would refuse `input.toUpperCase()`: `input` may be
`undefined`. You often combine `?.` and `??`: "the name in capitals, or "(nothing)"".

## A switch that never forgets a case

Here is a technique only a typed language allows. A type can list its possible
values: `"red" | "yellow" | "green"` (a type that only accepts these three strings).
The `type` keyword gives a name to that type.

```typescript
type Light = "red" | "yellow" | "green";

const lights: Light[] = ["green", "yellow", "red"];

for (const light of lights) {
  switch (light) {
    case "red":
      console.log("Stop");
      break;
    case "yellow":
      console.log("Slow down");
      break;
    case "green":
      console.log("Go");
      break;
    default: {
      const impossible: never = light;
      console.log(impossible);
    }
  }
}
```

```sortie
Go
Slow down
Stop
```

The `never` type means "no possible value". In the `default`, every case has been
handled: `light` can no longer be anything, so its type is `never`, and the line is
accepted.

The benefit shows up the day you add a color, for example `"flashing"`, without
thinking of updating the `switch`. That case then falls into the `default`, and `tsc`
refuses to store `"flashing"` in a `never` variable:

```text
main.ts:15:13 - error TS2322: Type '"flashing"' is not assignable to type 'never'.

15       const impossible: never = light;
               ~~~~~~~~~~


Found 1 error in main.ts:15
```

The error names exactly the forgotten case. It is an alarm: "you added a value,
remember to handle it everywhere".

## Common mistakes

### Comparing a number and a string

```typescript norun
const age: number = 18;
if (age === "18") {
  console.log("Adult");
}
```

```text
main.ts:2:5 - error TS2367: This comparison appears to be unintentional because the types 'number' and 'string' have no overlap.

2 if (age === "18") {
      ~~~~~~~~~~~~


Found 1 error in main.ts:2
```

The types `number` and `string` have nothing in common. A number is never equal to a
string: the `if` would always be false. Compare with the number: `age === 18`.

### Using a value that may be missing

```typescript norun
const names = ["Alice", "Bob"];
const found = names.find((n) => n.startsWith("Z"));
console.log(found.toUpperCase());
```

```text
main.ts:3:13 - error TS18048: 'found' is possibly 'undefined'.

3 console.log(found.toUpperCase());
              ~~~~~


Found 1 error in main.ts:3
```

`find` looks for an item and returns `undefined` if it finds nothing (more on this in
[arrays](collections.md)). TypeScript forces you to plan for that case. Write
`found?.toUpperCase()`, or test first: `if (found !== undefined) { ... }`.

### Using the counter after the loop

```typescript norun
for (let i = 0; i < 3; i++) {
  console.log(i);
}
console.log(i);
```

```text
main.ts:4:13 - error TS2304: Cannot find name 'i'.

4 console.log(i);
              ~


Found 1 error in main.ts:4
```

A variable created with `let` only exists inside its block: that is its
[scope](../debuter/glossaire.md#scope). If you need the value after the loop,
create the variable before it: `let i = 0; for (; i < 3; i++) { ... }`.

### Forgetting a case in an exhaustive switch

You added `"flashing"` to the `Light` type, but not to the `switch`:

```typescript norun
type Light = "red" | "yellow" | "green" | "flashing";
const lights: Light[] = ["red", "yellow", "green"];
for (const light of lights) {
  switch (light) {
    case "red":
      console.log("Stop");
      break;
    case "yellow":
      console.log("Slow down");
      break;
    case "green":
      console.log("Go");
      break;
    default: {
      const impossible: never = light;
      console.log(impossible);
    }
  }
}
```

```text
main.ts:15:13 - error TS2322: Type '"flashing"' is not assignable to type 'never'.

15       const impossible: never = light;
               ~~~~~~~~~~


Found 1 error in main.ts:15
```

Add the missing case: `case "flashing": console.log("Careful"); break;`.

## Test yourself

**Question 1:** what does this code print?

```typescript norun
const stock: number | null = 0;
console.log(stock ?? 10, stock || 10);
```

> **Answer**
>
> `0 10`. `??` only replaces `null` and `undefined`: it keeps `0`. `||` replaces
> all falsy values, including `0`.

**Question 2:** write a loop that prints the numbers from 10 down to 0, in steps of 2.

> **Answer**
>
> ```typescript
> for (let i = 10; i >= 0; i -= 2) {
>   console.log(i);
> }
> ```
>
> ```sortie
> 10
> 8
> 6
> 4
> 2
> 0
> ```
>
> `i -= 2` is a shortcut for `i = i - 2`.

**Question 3:** what is `const impossible: never = light;` for, in the `default` of a
`switch`?

> **Answer**
>
> To check that every case is handled. If a value of the type is forgotten, it
> ends up in the `default`, and `tsc` refuses to store it in a `never` variable:
> the error names the forgotten case.

## Summary

- `if`, `else if` and `else` choose a block; the ternary `a ? b : c` chooses a
  value.
- Compare with `===` and `!==`, never with `==`.
- `false`, `0`, `NaN`, `""`, `null` and `undefined` count as false in an `if`.
- A test like `typeof x === "string"` narrows the type of `x` inside the block.
- `??` gives a default value, `?.` calls an action only if the value exists.
- `for` counts, `while` repeats as long as, `for...of` goes through a list; a `switch`
  with `never` in `default` never forgets a case.

Next: [functions](fonctions.md).
