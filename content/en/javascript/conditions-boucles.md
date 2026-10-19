# Conditions and loops

This page shows how to make choices with `if`, `switch` and the ternary operator, then how to repeat actions with `while`, `for` and `for...of`.

> **Goals:**
>
> - you will be able to write a condition with `if`, `else if` and `else`;
> - you will be able to combine comparisons with `&&`, `||` and `!`;
> - you will know what "truthy" and "falsy" values are, and use `??` and `?.`;
> - you will be able to repeat code with `while`, `for` and `for...of`;
> - you will be able to leave a loop with `break` or skip a round with `continue`.

## The if condition

A [condition](../debuter/glossaire.md#condition) lets the program choose. It is a
railway switch: depending on the answer to a question, the train goes one way or the
other.

```javascript
const temperature = 28;

if (temperature > 25) {
  console.log("It is hot.");
}
console.log("End of the program.");
```

```sortie
It is hot.
End of the program.
```

- The question goes between parentheses `( )`.
- The code to run if the answer is true goes between curly braces `{ }`. This is called
  a **block**.
- The block is shifted by two spaces: this is the **indentation**. JavaScript does not
  require it, but it makes the code readable.

### else and else if

`else` gives the path to follow when the condition is false. `else if` adds more
questions, asked in order.

```javascript
const grade = 13;

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

As soon as a question answers true, its block runs and the following ones are ignored.
Here, `grade >= 10` is true too, but it is never tested.

## Comparison operators

A comparison answers with a [boolean](../debuter/glossaire.md#boolean): `true` or
`false`.

| Operator | Meaning | True example |
| --- | --- | --- |
| `===` | equal to | `3 === 3` |
| `!==` | not equal to | `3 !== 4` |
| `<`, `>` | less than, greater than | `2 < 5` |
| `<=`, `>=` | less than or equal, greater than or equal | `5 >= 5` |

```javascript
const age = 17;
console.log(age >= 18);
console.log(age !== 18);
console.log("abc" < "abd");
```

```sortie
false
true
true
```

Texts are compared in alphabetical order (more precisely, in the order of the character
codes). A reminder from [the previous page](premiers-pas.md): use `===` and `!==`,
never `==` and `!=`.

## Logical operators

To combine several questions:

- `&&` ("and"): true if **both** sides are true;
- `||` ("or"): true if **at least one** side is true;
- `!` ("not"): flips the answer.

```javascript
const age = 25;
const hasTicket = true;

if (age >= 18 && hasTicket) {
  console.log("Entry allowed");
}

const day = "sunday";
if (day === "saturday" || day === "sunday") {
  console.log("It is the weekend");
}

console.log(!hasTicket);
```

```sortie
Entry allowed
It is the weekend
false
```

> **Tip:** JavaScript stops as soon as it knows the answer. In `a && b`, if `a` is
> false, `b` is not even looked at.

## Truthy and falsy

In an `if`, you can put any value, not just a boolean. JavaScript then turns it into
`true` or `false`. Six values become `false`: they are called **falsy**.

- `false`
- `0` (and `-0`, `0n`)
- `""` (the empty text)
- `null`
- `undefined`
- `NaN`

**All** other values are **truthy**: they become `true`. Even `"0"`, `"false"`, an
empty array `[]` or an empty object `{}`.

```javascript
const values = [0, 1, "", "hi", null, undefined, NaN, "0", []];

for (const v of values) {
  console.log(v, "→", Boolean(v));
}
```

```sortie
0 → false
1 → true
 → false
hi → true
null → false
undefined → false
NaN → false
0 → true
[] → true
```

`Boolean(v)` shows what a value turns into. This rule is often used to test whether a
text is filled in:

```javascript
const username = "";

if (username) {
  console.log(`Hello, ${username}`);
} else {
  console.log("Empty username!");
}
```

```sortie
Empty username!
```

## The ternary operator

To choose between **two values**, the ternary is shorter than an `if`:
`condition ? valueIfTrue : valueIfFalse`.

```javascript
const age = 20;
const status = age >= 18 ? "adult" : "minor";
console.log(status);

const n = 3;
console.log(`${n} item${n > 1 ? "s" : ""}`);
```

```sortie
adult
3 items
```

Read it like a question: "age ≥ 18? if yes adult, otherwise minor".

> **Warning:** keep the ternary for simple choices. Two nested ternaries are very hard
> to read: use an `if` instead.

## Missing values: ?? and ?.

### ??: a fallback value

`a ?? b` gives `a`, unless `a` is `null` or `undefined`: it then gives `b`.

```javascript
let color;
console.log(color ?? "blue");

const volume = 0;
console.log(volume ?? 50);
console.log(volume || 50);
```

```sortie
blue
0
50
```

Compare with `||`, which you often see in old code: `||` replaces **all** falsy values,
including `0`. A volume of `0` is a real value: `??` keeps it, `||` overwrites it.

### ?.: asking without crashing

`?.` (**optional chaining**) reads a property only if the value exists. Otherwise, it
returns `undefined` instead of causing an error.

```javascript
const user = { name: "Alice", address: null };

console.log(user.name);
console.log(user.address?.city);
console.log(user.address?.city ?? "unknown city");
```

```sortie
Alice
undefined
unknown city
```

Objects `{ ... }` are explained in [the page about objects](objets.md).

## Multiple choice: switch

When you compare **the same value** against several possibilities, `switch` is clearer
than a long series of `else if`.

```javascript
const day = "tuesday";

switch (day) {
  case "saturday":
  case "sunday":
    console.log("Rest");
    break;
  case "wednesday":
    console.log("Half day");
    break;
  default:
    console.log("Work");
}
```

```sortie
Work
```

- Each `case` compares with `===`.
- `break` leaves the `switch`. Without it, execution **continues** into the next
  `case`.
- Two `case` lines in a row share the same code (here saturday and sunday).
- `default` is used when no `case` matches.

## The while loop

A [loop](../debuter/glossaire.md#loop) repeats a block of code. `while` repeats it as
long as the condition is true.

```javascript
let counter = 3;

while (counter > 0) {
  console.log(counter);
  counter--;
}
console.log("Liftoff!");
```

```sortie
3
2
1
Liftoff!
```

> **Warning:** if the condition always stays true, the loop never stops. Here,
> `counter--` brings the counter closer to 0 on each round. To stop a stuck program,
> press **Ctrl + C** in the terminal.

## The for loop

`for` groups on one line the three parts of a counting loop:
`for (start; condition; step)`.

```javascript
for (let i = 1; i <= 5; i++) {
  console.log(`${i} x 3 = ${i * 3}`);
}
```

```sortie
1 x 3 = 3
2 x 3 = 6
3 x 3 = 9
4 x 3 = 12
5 x 3 = 15
```

1. **Start**: `let i = 1`, run once at the beginning.
2. **Condition**: `i <= 5`, checked before each round.
3. **Step**: `i++`, run after each round.

## for...of and for...in

### for...of: going through the values

To go through the elements of an array (or the letters of a text), use `for...of`. No
counter to manage: you receive each value, one by one.

```javascript
const fruits = ["apple", "pear", "kiwi"];

for (const fruit of fruits) {
  console.log(`I like this ${fruit}.`);
}

for (const letter of "abc") {
  console.log(letter);
}
```

```sortie
I like this apple.
I like this pear.
I like this kiwi.
a
b
c
```

Here `const` is fine: on each round, a **new** variable `fruit` is created.

### for...in: going through the keys

`for...in` goes through the **property names** of an object, not the values.

```javascript
const scores = { alice: 12, bob: 8 };

for (const name in scores) {
  console.log(name, scores[name]);
}
```

```sortie
alice 12
bob 8
```

> **Remember:** `for...of` for the **values** of an array or a text; `for...in` for
> the **keys** of an object. Do not mix them up.

## break and continue

- `break` leaves the loop right away.
- `continue` skips the rest of the current round and moves on to the next one.

```javascript
const numbers = [4, 7, -1, 12, 0, 5];

for (const n of numbers) {
  if (n < 0) {
    continue;
  }
  if (n === 0) {
    break;
  }
  console.log(n);
}
```

```sortie
4
7
12
```

`-1` is skipped by `continue`. At `0`, `break` stops everything: `5` is never displayed.

## Common mistakes

### Using const in a counting for

```javascript norun
for (const i = 0; i < 3; i++) {
  console.log(i);
}
```

```text
0
file:///home/alice/js/main.mjs:1
for (const i = 0; i < 3; i++) {
                          ^

TypeError: Assignment to constant variable.
    at file:///home/alice/js/main.mjs:1:27

Node.js v22.22.0
```

The first round works, then `i++` tries to change a constant. In a classic `for`, the
counter changes: write `let i = 0`. (In a `for...of`, `const` is correct.)

### Writing = instead of ===

```javascript norun
const grade = 12;
if (grade = 20) {
  console.log("Perfect!");
}
```

```text
file:///home/alice/js/main.mjs:2
if (grade = 20) {
          ^

TypeError: Assignment to constant variable.
    at file:///home/alice/js/main.mjs:2:11

Node.js v22.22.0
```

A single `=` **stores** a value, it does not compare. Here, JavaScript tries to put 20
in `grade`. With `let`, there would be no error, but the condition would always be
true: that is worse. Compare with `===`: `if (grade === 20)`.

### Forgetting a closing curly brace

```javascript norun
const age = 20;
if (age >= 18) {
  console.log("Adult");
```

```text
file:///home/alice/js/main.mjs:4



SyntaxError: Unexpected end of input

Node.js v22.22.0
```

The file ends (*end of input*) while a `{` block is still open. Every `{` must have its
`}`. A good editor colors pairs of curly braces to help you.

### Using for...in on an array

```javascript norun
const fruits = ["apple", "pear"];
for (const i in fruits) {
  console.log(i + 1);
}
```

```text
01
11
```

`for...in` gives the **keys** of the array, as the texts `"0"` and `"1"`. And `"0" + 1`
glues the two together: `"01"`. For the values, use `for...of`.

## Test yourself

**Question 1:** what does this code display?

```javascript norun
const x = 7;
if (x > 10) {
  console.log("big");
} else if (x > 5) {
  console.log("medium");
} else {
  console.log("small");
}
```

> **Answer**
>
> `medium`. `x > 10` is false, then `x > 5` is true: its block runs and it stops there.

**Question 2:** what do `0 ?? 10` and `0 || 10` give?

> **Answer**
>
> ```javascript
> console.log(0 ?? 10);
> console.log(0 || 10);
> ```
>
> ```sortie
> 0
> 10
> ```
>
> `??` only replaces `null` and `undefined`. `||` replaces any falsy value, including
> `0`.

**Question 3:** write a loop that displays the even numbers from 2 to 10.

> **Answer**
>
> ```javascript
> for (let i = 2; i <= 10; i += 2) {
>   console.log(i);
> }
> ```
>
> ```sortie
> 2
> 4
> 6
> 8
> 10
> ```
>
> The step `i += 2` skips the odd numbers.

## Summary

- `if`, `else if` and `else` choose a path; `switch` compares a value against several
  cases, without forgetting `break`.
- `&&`, `||` and `!` combine conditions; compare with `===` and `!==`.
- `false`, `0`, `""`, `null`, `undefined` and `NaN` are falsy; everything else is
  truthy.
- `cond ? a : b` chooses a value; `??` gives a fallback value; `?.` reads without
  crashing.
- `while` repeats as long as a condition is true, `for` counts, `for...of` goes through
  the values, `for...in` the keys of an object.
- `break` leaves the loop, `continue` moves on to the next round.

Next: [functions](fonctions.md).
