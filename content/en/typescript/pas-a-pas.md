# Tracing a program step by step

This page teaches you to run a TypeScript program in your head, line by line, with a trace table, on five small programs that get richer and richer.

> **Goals:**
>
> - you will know how to fill in a trace table: one column per variable, one row per
>   step;
> - you will know how to trace a condition, a loop and a function call;
> - you will know how to predict what a program prints **before** you run it;
> - you will know how to make the program print its own steps with `console.log`;
> - you will know where to look when your table and the computer disagree.

## Why trace a program in your head

The computer is a very obedient cook, but with no imagination at all. It reads the
recipe one line at a time, from top to bottom. It does **exactly** what is written,
even if it makes no sense.

To understand a program, do the same. Take a sheet of paper and a pencil. Read a
line, do what it says, write down the result. Then move on to the next line.

This sheet is called a **trace table**. It is the best weapon against
[bugs](../debuter/glossaire.md#bug): most of them come from a gap between what you
**think** the program does and what it **really** does.

```text
   what you think                what the computer does
   ┌────────────────┐            ┌────────────────┐
   │ total is 20    │    ≠       │ total is 12    │   ◄── the bug is here
   └────────────────┘            └────────────────┘
            the trace table shows the gap
```

> **Remember:** the computer never guesses. It follows the lines one by one. To
> understand it, do the same thing, on paper.

## The trace table method

A trace table looks like a notebook:

- a **Step** (or **Line**) column: where the program is;
- one column per [variable](../debuter/glossaire.md#variable): its
  [value](../debuter/glossaire.md#valeur) at that moment;
- a **Prints** column: what `console.log` writes on the screen.

Four simple rules:

1. One row in the table for **each** line of code that runs, in order.
2. When a variable changes, write its **new** value. Cross out the old one in your
   head: it no longer exists.
3. An empty cell means "not created yet" or "not changed".
4. Before running the program, **guess** the output. Then compare.

In the programs on this page, each line has a comment `// L1`, `// L2`… That is the
line number: you use it to fill in the **Step** column.

> **Remember:** one line of code run = one row in the table. Never skip a line, even
> if it looks obvious.

## Program 1: variables that change

Here are a few lines that play with two boxes, `a` and `b`.

```typescript
let a = 5; // L1
let b = 3; // L2
a = a + b; // L3
b = a - b; // L4
a = a - b; // L5
console.log(a, b); // L6
```

**Before running it, guess what it prints.** Write your answer on paper.

```sortie
3 5
```

Here is the trace table. Read it row by row.

| Step | What happens | a | b | Prints |
|---|---|---|---|---|
| L1 | create `a` | 5 | | |
| L2 | create `b` | 5 | 3 | |
| L3 | `a + b` = 5 + 3 = 8 | **8** | 3 | |
| L4 | `a - b` = 8 − 3 = 5 | 8 | **5** | |
| L5 | `a - b` = 8 − 5 = 3 | **3** | 5 | |
| L6 | print | 3 | 5 | `3 5` |

The trap is on line L5. You compute `a - b` with the **new** value of `b` (5), not the
old one (3). So the program has **swapped** the values of `a` and `b`.

The `=` sign does not mean "is equal to" like in maths. It means: "compute the right
side, then put the result in the box on the left".

```text
   a = a + b
   ▲   └─┬─┘
   │     └── 1. compute with the CURRENT values: 5 + 3 = 8
   └──────── 2. put 8 in box a (the old 5 is thrown away)
```

> **Try it:** replace `let b = 3;` with `let b = 10;`. Redo the table. The program
> prints `10 5`: it still swaps the two values.

> **Remember:** on each line, use the most **recent** value of each variable, the one
> from the row just above in your table.

## Program 2: a condition

A cinema charges €10, €5 for children under 12, and €7 for people under 18.

```typescript
const age = 15; // L1
let price = 10; // L2
if (age < 12) { // L3
  price = 5; // L4
} else if (age < 18) { // L5
  price = 7; // L6
}
console.log(`Price: €${price}`); // L7
```

**Before running it, guess what it prints.**

```sortie
Price: €7
```

A [condition](../debuter/glossaire.md#condition) is like railway points: the program
takes **only one** path. Add a **Test** column to write down the result of each
question: `true` or `false`.

| Step | Test | age | price | Prints |
|---|---|---|---|---|
| L1 | | 15 | | |
| L2 | | 15 | 10 | |
| L3 | `15 < 12` → `false` | 15 | 10 | |
| L4 | *skipped* | | | |
| L5 | `15 < 18` → `true` | 15 | 10 | |
| L6 | | 15 | **7** | |
| L7 | | 15 | 7 | `Price: €7` |

Line L4 is **never** run: the first test is false, so the program jumps straight to
the `else if`.

```text
              age < 12 ?
             /          \
          yes            no
           │              │
       price = 5      age < 18 ?
                     /          \
                  yes            no
                   │              │
               price = 7     (nothing: price stays 10)
```

> **Try it:** set `const age = 30;`. Both tests are false: the program prints
> `Price: €10`. Then set `const age = 12;`: `12 < 12` is **false**, so you get
> `Price: €7`. Cases "right on the limit" always deserve a row in your table.

> **Remember:** for a condition, write the test with the real values (`15 < 12`) and
> its result (`false`). Then follow **only one** path.

## Program 3: a loop that adds up

A [loop](../debuter/glossaire.md#boucle) repeats the same lines. Here, we add twice
`i` to `total`, four times.

```typescript
let total = 0; // L1
for (let i = 1; i <= 4; i++) { // L2
  total = total + i * 2; // L3
  console.log(`i = ${i}, total = ${total}`); // L4
}
console.log(`End: ${total}`); // L5
```

**Before running it, guess what it prints.** How many lines? What is the last one?

```sortie
i = 1, total = 2
i = 2, total = 6
i = 3, total = 12
i = 4, total = 20
End: 20
```

Line L2 does three things, at three different moments:

```text
   for (let i = 1;   i <= 4;   i++)
        └──┬────┘    └──┬─┘    └┬┘
     1. only once   2. before   3. at the end
        at the         each        of each
        start          round       round
```

Here is the full table. It is long: that's normal, a loop takes many steps.

| Step | Test `i <= 4` | i | total | Prints |
|---|---|---|---|---|
| L1 | | | 0 | |
| L2 (start) | | 1 | 0 | |
| L2 (test) | `1 <= 4` → `true` | 1 | 0 | |
| L3 | | 1 | 0 + 1 × 2 = **2** | |
| L4 | | 1 | 2 | `i = 1, total = 2` |
| L2 (`i++`, test) | `2 <= 4` → `true` | **2** | 2 | |
| L3 | | 2 | 2 + 2 × 2 = **6** | |
| L4 | | 2 | 6 | `i = 2, total = 6` |
| L2 (`i++`, test) | `3 <= 4` → `true` | **3** | 6 | |
| L3 | | 3 | 6 + 3 × 2 = **12** | |
| L4 | | 3 | 12 | `i = 3, total = 12` |
| L2 (`i++`, test) | `4 <= 4` → `true` | **4** | 12 | |
| L3 | | 4 | 12 + 4 × 2 = **20** | |
| L4 | | 4 | 20 | `i = 4, total = 20` |
| L2 (`i++`, test) | `5 <= 4` → `false` | **5** | 20 | |
| L5 | | | 20 | `End: 20` |

Look at the second-to-last row: `i` is **5**, the test is false, the loop stops. This
last test is the step everybody forgets.

`total` behaves like a piggy bank: on each round, you slip a coin into it, and you
never empty it.

> **Try it:** replace `i <= 4` with `i < 4`. The loop does one round less. The program
> prints three lines, then `End: 12`.

> **Remember:** in a loop, write down the test on **every** round, including the last
> one, the one that is false and stops everything.

## Program 4: a function called twice

A [function](../debuter/glossaire.md#fonction) is a machine: you give it values, it
gives back a result. Here, `discountedPrice` computes a price after a percentage
discount.

```typescript
function discountedPrice(price: number, percent: number): number { // L1
  const discount = (price * percent) / 100; // L2
  return price - discount; // L3
}

const book = discountedPrice(20, 10); // L4
const game = discountedPrice(50, 20); // L5
console.log(book, game, book + game); // L6
```

**Before running it, guess what it prints.**

```sortie
18 40 58
```

Watch out for the first trap: lines L1 to L3 do **nothing** when the program reads
them the first time. They **describe** the machine, without switching it on. The
program really starts on line L4.

Here is who calls whom, and what comes back:

```text
   main program                              discountedPrice
   ────────────                              ───────────────
L4 book = discountedPrice(20, 10)  ────────►  price = 20, percent = 10
                                              L2 discount = 2
          book = 18                ◄────────  L3 returns 18
L5 game = discountedPrice(50, 20)  ────────►  price = 50, percent = 20
                                              L2 discount = 10
          game = 40                ◄────────  L3 returns 40
L6 console.log(...)
```

Each call opens a **new sheet** of scrap paper for the function. Its
[parameters](../debuter/glossaire.md#parametre) `price` and `percent` receive the
[arguments](../debuter/glossaire.md#argument) of the call. At the `return`, the sheet
is thrown away: `discount` no longer exists.

| Step | Where | price | percent | discount | book | game | Prints |
|---|---|---|---|---|---|---|---|
| L4 | main: call `discountedPrice(20, 10)` | | | | | | |
| L1 | inside the function | 20 | 10 | | | | |
| L2 | inside the function | 20 | 10 | 20 × 10 / 100 = 2 | | | |
| L3 | returns 20 − 2 = 18 | 20 | 10 | 2 | | | |
| L4 | main: store 18 | | | | **18** | | |
| L5 | main: call `discountedPrice(50, 20)` | | | | 18 | | |
| L1 | inside the function (new sheet) | 50 | 20 | | 18 | | |
| L2 | inside the function | 50 | 20 | 50 × 20 / 100 = 10 | 18 | | |
| L3 | returns 50 − 10 = 40 | 50 | 20 | 10 | 18 | | |
| L5 | main: store 40 | | | | 18 | **40** | |
| L6 | main | | | | 18 | 40 | `18 40 58` |

The empty cells for `price`, `percent` and `discount` in the main program are not a
mistake: these variables exist **only** during the call.

> **Try it:** replace `discountedPrice(20, 10)` with `discountedPrice(10, 20)`. The
> order of the arguments matters: `price` is now 10 and `percent` 20. The book costs
> `8`, and the program prints `8 40 48`.

> **Remember:** for a function, draw the outgoing arrow (the arguments) and the return
> arrow (the returned value). Each call has its own brand-new sheet.

## Program 5: a loop over an array, with a condition

Last program: it brings everything together. We go through some grades. We count the
passing grades (10 or more) and look for the best one.

```typescript
const grades = [12, 8, 15, 9, 18]; // L1
let passed = 0; // L2
let best = 0; // L3
for (const grade of grades) { // L4
  if (grade >= 10) { // L5
    passed = passed + 1; // L6
  }
  if (grade > best) { // L7
    best = grade; // L8
  }
}
console.log(`Passed: ${passed} out of ${grades.length}`); // L9
console.log(`Best: ${best}`); // L10
```

**Before running it, guess what it prints.**

```sortie
Passed: 3 out of 5
Best: 18
```

`for...of` takes the elements of the array one by one, like taking cards from a deck,
from the first to the last.

```text
   grades:  [ 12 | 8 | 15 | 9 | 18 ]
              ▲    ▲    ▲   ▲    ▲
   round:     1    2    3   4    5      then no more cards: end of the loop
```

To go faster, we group one round of the loop on a single row of the table, with its
two tests:

| Round | grade | `grade >= 10` ? | passed | `grade > best` ? | best |
|---|---|---|---|---|---|
| before | | | 0 | | 0 |
| 1 | 12 | `true` | **1** | `12 > 0` → `true` | **12** |
| 2 | 8 | `false` | 1 | `8 > 12` → `false` | 12 |
| 3 | 15 | `true` | **2** | `15 > 12` → `true` | **15** |
| 4 | 9 | `false` | 2 | `9 > 15` → `false` | 15 |
| 5 | 18 | `true` | **3** | `18 > 15` → `true` | **18** |
| after | | | 3 | | 18 |

The two `if`s are **independent**: this is not an `if … else`. On each round, the
program asks both questions.

> **Try it:** replace the grades with `[-3, -1, -7]`. The program prints `Best: 0`,
> even though 0 is not in the array! Your trace table shows it right away: no
> `grade > best` test is true. To fix it, start with `let best = grades[0];`.

> **Remember:** for a loop over an array, one row of the trace table per element.
> Write each test with the real values.

## Making the program print its steps

The table on paper is your prediction. To check it, ask the program to write its table
**itself**, with `console.log`.

### Labels in console.log

A number alone on the screen doesn't say where it comes from. Always add a label.

```typescript
let total = 0;
for (let i = 1; i <= 3; i++) {
  total = total + i * 2;
  console.log("round", i, "| total =", total);
}
```

```sortie
round 1 | total = 2
round 2 | total = 6
round 3 | total = 12
```

### The curly braces trick

Wrap your variables in curly braces: `console.log({ i, total })`. Node then prints the
**name** and the **value** of each variable, without you typing the labels.

```typescript
let total = 0;
for (let i = 1; i <= 3; i++) {
  total = total + i * 2;
  console.log({ i, total });
}
```

```sortie
{ i: 1, total: 2 }
{ i: 2, total: 6 }
{ i: 3, total: 12 }
```

### Entering and leaving a function

For a function, print what goes in and what comes out. An arrow helps to read it.

```typescript
function discountedPrice(price: number, percent: number): number {
  console.log(`  → in: price = ${price}, percent = ${percent}`);
  const result = price - (price * percent) / 100;
  console.log(`  ← out: ${result}`);
  return result;
}

console.log("call 1");
const book = discountedPrice(20, 10);
console.log("call 2");
const game = discountedPrice(50, 20);
console.log("end:", book, game);
```

```sortie
call 1
  → in: price = 20, percent = 10
  ← out: 18
call 2
  → in: price = 50, percent = 20
  ← out: 40
end: 18 40
```

> **Try it:** in program 5, add `console.log({ grade, passed, best });` at the end of
> the loop. You get five lines: compare them with your trace table.

> **Remember:** a `console.log` with a label is a trace table written by the computer.
> Remove them once the bug is found.

## The debugger, in two sentences

A [debugger](../debuter/glossaire.md#debogage) stops the program on a chosen line (a
breakpoint) and lets you move forward one line at a time, showing all the variables:
it is an automatic trace table. In VS Code, click in the margin to the left of a line,
then start "Run and Debug"; everything is explained in
[the chapter on debugging](debogage.md).

## Common mistakes

### Forgetting the last test of the loop

People often think `i` is 4 after a loop `for (let i = 1; i <= 4; i++)`. But the loop
stops because the test becomes false: `i` is then 5. A beginner writes for example:

```typescript norun
let i = 1;
while (i <= 4) {
  i++;
}
console.log(`The loop ran ${i} times`);
```

```text
The loop ran 5 times
```

The program lies: it ran 4 times. The trace table shows it, because the last row is
`5 <= 4` → `false`. **The fix:** count the rounds in a separate variable.

```typescript
let i = 1;
let rounds = 0;
while (i <= 4) {
  rounds++;
  i++;
}
console.log(`The loop ran ${rounds} times, and i is ${i}`);
```

```sortie
The loop ran 4 times, and i is 5
```

### Reusing an old value

In program 1, many people compute line L5 with `b = 3`, the value from the start. They
get `a = 8 − 3 = 5`. That's wrong: line L4 has already set `b` to 5.

**The fix:** always look at the row **just above** in your table, never the first one.

### Being off by one round (the off-by-one error)

An array of 3 grades has positions 0, 1 and 2. This code goes one cell too far:

```typescript norun
const grades = [12, 8, 15];
for (let i = 0; i <= grades.length; i++) {
  console.log(grades[i].toFixed(1));
}
```

`tsc` accepts this code. The program prints three grades, then crashes (excerpt of the
message):

```text
TypeError: Cannot read properties of undefined (reading 'toFixed')
```

Your trace table would have shown the extra round: `i = 3`, test `3 <= 3` → `true`,
and `grades[3]` is `undefined`. **The fix:** `<` instead of `<=`, or better,
`for...of`.

```typescript
const grades = [12, 8, 15];
for (const grade of grades) {
  console.log(grade.toFixed(1));
}
```

```sortie
12.0
8.0
15.0
```

### Thinking a function's variable still exists

In program 4, `discount` lives on the function's scrap sheet. After the `return`, it
is gone.

```typescript norun
function discountedPrice(price: number, percent: number): number {
  const discount = (price * percent) / 100;
  return price - discount;
}

const book = discountedPrice(20, 10);
console.log(book, discount);
```

```text
main.mts(7,19): error TS2304: Cannot find name 'discount'.
```

**The fix:** if you need a value, make the function **return** it, or compute it again
outside. The chapter on [functions](fonctions.md#variable-scope) explains this scope
rule in detail.

## Test yourself

1. What is `x` at the end? `let x = 2; x = x * 3; x = x - 1; x = x * x;`

   > **Answer**
   >
   > `x` is 25. Table: 2, then 2 × 3 = 6, then 6 − 1 = 5, then 5 × 5 = 25. On each
   > line, you start from the value just above.

2. How many times does `console.log` run in `for (let i = 0; i < 3; i++) { console.log(i); }`,
   and how many times is the test `i < 3` evaluated?

   > **Answer**
   >
   > `console.log` runs 3 times (`i` = 0, 1, 2). The test is evaluated 4 times: the
   > fourth time, `3 < 3` is false and stops the loop.

3. What does this program print?
   `function f(n: number): number { return n + 1; }` then `console.log(f(f(1)));`

   > **Answer**
   >
   > `3`. First you compute the inner call: `f(1)` returns 2. Then the outer call:
   > `f(2)` returns 3. Two calls, two sheets of scrap paper.

## Summary

- A trace table: one column per variable, one row per line of code run.
- Always use the most recent value of a variable.
- For a condition, write the test with the real values and follow only one path.
- For a loop, write down every test, including the last one, which is false.
- For a function, draw the way in (arguments) and the way back (returned value).
- `console.log({ i, total })` makes the program write its own trace table.

Next: [beginner pitfalls in TypeScript](pieges.md)
