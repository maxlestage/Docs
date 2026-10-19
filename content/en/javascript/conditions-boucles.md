# Conditions

This page shows how to make choices with `if`, `switch` and the ternary operator, and how to combine comparisons.

> **Goals:**
>
> - you will be able to write a condition with `if`, `else if` and `else`;
> - you will be able to combine comparisons with `&&`, `||` and `!`;
> - you will know what "truthy" and "falsy" values are, and use `??` and `?.`.

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

Line by line:

- `const temperature = 28;` stores 28 in the constant `temperature`;
- `if (temperature > 25)` asks a question: "is 28 greater than 25?". The answer is
  yes;
- so the `console.log` inside the curly braces runs;
- the last `console.log` is outside the curly braces: it runs in every case, even if
  it were cold.

- The question goes between parentheses `( )`.
- The code to run if the answer is true goes between curly braces `{ }`. This is called
  a **block**.
- The block is shifted by two spaces: this is the **indentation**. JavaScript does not
  require it, but it makes the code readable.

### else and else if

Often there is more than one case. A grade can be very good, average or poor, and you
want a different message for each one, like on a school report.

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

**Your turn:** write a program that stores `15` in a constant `hour`, then displays
"Good morning" if the hour is before 12, "Good afternoon" if it is before 18, and
"Good evening" otherwise.

> **Solution**
>
> ```javascript
> const hour = 15;
>
> if (hour < 12) {
>   console.log("Good morning");
> } else if (hour < 18) {
>   console.log("Good afternoon");
> } else {
>   console.log("Good evening");
> }
> ```
>
> ```sortie
> Good afternoon
> ```

## Comparison operators

To decide, a program first asks a question: "is the age 18?", "is the temperature
above 25?". Comparison operators are used to write these questions.

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

Line by line:

- `age` is 17;
- `age >= 18` asks "is 17 greater than or equal to 18?". No: `false`;
- `age !== 18` asks "is 17 different from 18?". Yes: `true`;
- `"abc" < "abd"`: both texts start with "ab", and "c" comes before "d" in the
  alphabet: `true`.

Texts are compared in alphabetical order (more precisely, in the order of the character
codes). A reminder from [the previous page](premiers-pas.md): use `===` and `!==`,
never `==` and `!=`.

## Logical operators

A decision often depends on several things at once. To get into the cinema, you need
to be old enough **and** have a ticket. These three operators join several questions
into one.

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

Line by line:

- `age` is 25 and `hasTicket` is `true`;
- first `if`: `age >= 18` is true **and** `hasTicket` is true, so entry is allowed;
- second `if`: `day` is `"sunday"`. The first test is false, the second is true, and
  with `||` that is enough;
- the last `console.log` displays `!hasTicket`: `true` flipped becomes `false`.

> **Tip:** JavaScript stops as soon as it knows the answer. In `a && b`, if `a` is
> false, `b` is not even looked at.

**Your turn:** write a program that stores `70` in a constant `age`, then displays
"Reduced price" if the age is under 18 or 65 and over, and "Full price" otherwise.

> **Solution**
>
> ```javascript
> const age = 70;
>
> if (age < 18 || age >= 65) {
>   console.log("Reduced price");
> } else {
>   console.log("Full price");
> }
> ```
>
> ```sortie
> Reduced price
> ```

## Truthy and falsy

You often want to know whether a box is filled in: a typed username, a number other
than zero. JavaScript lets you ask this very simply, by putting the value alone in the
`if`.

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

Line by line:

- `values` is a list of nine values of all kinds;
- `for (const v of values)` takes these values one by one and stores each one in `v`.
  This loop is explained in
  [the chapter on loops](boucles.md#for-of-going-through-the-values);
- `Boolean(v)` turns the value into `true` or `false`, and `console.log` displays both
  side by side;
- the third line of the output starts with a space: that is the empty text `""`, which
  displays nothing.

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

> **Note:** this part can wait. Read it once the basics feel solid.

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

A value is sometimes missing: a setting the user did not choose, an address not filled
in yet. These two operators handle that gap without writing an `if` every time.

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

Line by line:

- `let color;` creates a variable with no value: it is `undefined`. So `??` gives the
  fallback value `"blue"`;
- `volume` is `0`, which is neither `null` nor `undefined`: `volume ?? 50` keeps `0`;
- `volume || 50` gives `50`, because `0` is falsy.

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

> **Note:** this part can wait. Read it once the basics feel solid.

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

## Common mistakes

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

**Question 3:** among `0`, `"0"`, `[]` and `""`, which values are falsy?

> **Answer**
>
> ```javascript
> console.log(Boolean(0), Boolean("0"), Boolean([]), Boolean(""));
> ```
>
> ```sortie
> false true true false
> ```
>
> Only `0` and `""` are falsy. `"0"` is a non-empty text, and an empty array `[]` is
> truthy.

## Summary

- `if`, `else if` and `else` choose a path; `switch` compares a value against several
  cases, without forgetting `break`.
- `&&`, `||` and `!` combine conditions; compare with `===` and `!==`.
- `false`, `0`, `""`, `null`, `undefined` and `NaN` are falsy; everything else is
  truthy.
- `cond ? a : b` chooses a value; `??` gives a fallback value; `?.` reads without
  crashing.

Next: [loops](boucles.md).
