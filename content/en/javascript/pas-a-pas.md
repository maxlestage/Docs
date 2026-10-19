# Following a program step by step in JavaScript

This page teaches you how to run a JavaScript program in your head, line by line, with a table that follows the value of each variable.

> **Goals:**
>
> - you will be able to fill in a trace table, line after line;
> - you will be able to guess what a small program prints before running it;
> - you will be able to follow a condition, a loop and a function call;
> - you will be able to make the program print its steps with `console.log`;
> - you will be able to avoid the most common mistakes when following a program.

## Why play the computer?

The computer guesses nothing. It reads your program **one line at a time**, from top to
bottom, and does exactly what is written. Nothing more, nothing less.

To understand a program, do what it does. It is like following a cooking recipe with
your finger: you put your finger on a line, you do it, then you move down one line.

```text
  line 1   ◄── my finger is here: I do line 1
  line 2
  line 3
```

What you gain:

- you understand **why** the program prints what it prints;
- you find a [bug](../debuter/glossaire.md#bug) without even running the code;
- you read other people's code faster.

> **Remember:** the computer reads a line, does it, then moves on to the next one.
> Following a program means doing the same thing, on paper.

## The trace table

A [variable](../debuter/glossaire.md#variable) is a box with a label. Its value can
change while the program runs. So you don't forget anything, you write everything down
in a **trace table**:

- one column per variable;
- one row per step (one line of code executed);
- one "printed" column for what `console.log` shows.

```text
 ┌───────┬──────┬──────┬───────────┐
 │ Step  │  a   │  b   │ printed   │   ← one column per box
 ├───────┼──────┼──────┼───────────┤
 │   1   │  3   │  –   │           │   ← one row per step
 │   2   │  3   │  5   │           │
 └───────┴──────┴──────┴───────────┘
```

The dash `–` means "the box does not exist yet". When a value changes, you write the new
value on the row of that step. The old value no longer counts.

It is like a cashier's notebook: after each purchase, the cashier writes down the new
total. They don't keep the old one in mind.

> **Remember:** one column per variable, one row per step. You always read the **most
> recent** value, the one on the row just above.

## Program 1: variables that change

Here is a five-line program. It only contains calculations.

```javascript
let a = 3;
let b = 5;
a = a + b;
b = a - b;
a = a - b;
console.log(a, b);
```

**Before running it, guess what is printed.** Take a sheet of paper. Don't read on
right away.

Here is the trace. At each step, you compute the part to the right of the `=` with the
**current** values, then you store the result in the box on the left.

| Step | Line | Calculation | a | b | printed |
| --- | --- | --- | --- | --- | --- |
| 1 | `let a = 3;` | – | 3 | – | |
| 2 | `let b = 5;` | – | 3 | 5 | |
| 3 | `a = a + b;` | 3 + 5 = 8 | 8 | 5 | |
| 4 | `b = a - b;` | 8 − 5 = 3 | 8 | 3 | |
| 5 | `a = a - b;` | 8 − 3 = 5 | 5 | 3 | |
| 6 | `console.log(a, b);` | – | 5 | 3 | `5 3` |

```sortie
5 3
```

The two values have been **swapped**: `a` was 3 and is now 5, `b` was 5 and is now 3.
At step 4, the calculation uses the **new** `a` (8), not the old one (3). That is the
whole secret.

Try it: replace `let a = 3;` with `let a = 10;`. The trace gives `5 10`: the swap works
with any numbers.

> **Remember:** to the right of the `=`, you read the current values. On the left, you
> store the result. Only one box changes per line.

## Program 2: a condition

A [condition](../debuter/glossaire.md#condition) is a railway switch: the program takes
**only one** path. Here is the price of a movie ticket depending on age.

```javascript
const age = 15;
let price = 10;
if (age < 12) {
  price = 5;
} else if (age < 18) {
  price = 7;
}
console.log(`Price: €${price}`);
```

**Before running it, guess what is printed.**

In the trace, you add a column for the answer of each test: `true` or `false`.

| Step | Line | Test | age | price | printed |
| --- | --- | --- | --- | --- | --- |
| 1 | `const age = 15;` | – | 15 | – | |
| 2 | `let price = 10;` | – | 15 | 10 | |
| 3 | `if (age < 12)` | 15 < 12 → `false` | 15 | 10 | |
| 4 | `else if (age < 18)` | 15 < 18 → `true` | 15 | 10 | |
| 5 | `price = 7;` | – | 15 | 7 | |
| 6 | `console.log(…)` | – | 15 | 7 | `Price: €7` |

```sortie
Price: €7
```

The line `price = 5;` is **never** executed: its test was false. You skip it, and it
gets no row in the trace.

```text
            age < 12 ?
           /          \
        yes            no
         │              │
     price = 5      age < 18 ?
                   /          \
                yes            no
                 │              │
             price = 7    (nothing: price stays 10)
```

Try it: write `const age = 30;`. Both tests are false, no block runs, and the program
prints `Price: €10`. With `const age = 12;`, the first test is false (12 is not smaller
than 12): the price is 7.

> **Remember:** at each `if`, write down the test and its answer, `true` or `false`.
> Only the block of the first true test runs; the others are skipped.

## Program 3: a loop that accumulates

A [loop](../debuter/glossaire.md#loop) repeats lines. Here, a piggy bank: in week 1,
you put in €2, in week 2, you put in €4, and so on.

```javascript
let savings = 0;
let week = 1;
while (week <= 4) {
  savings = savings + week * 2;
  week = week + 1;
}
console.log(`Week ${week}, savings €${savings}`);
```

**Before running it, guess what is printed.** Watch out for the value of `week` at the
end.

The test `week <= 4` is read again **before each round**. You write it down every time.

| Step | Line | Test | week | savings |
| --- | --- | --- | --- | --- |
| 1 | `let savings = 0;` | – | – | 0 |
| 2 | `let week = 1;` | – | 1 | 0 |
| 3 | `while (week <= 4)` | 1 <= 4 → `true` | 1 | 0 |
| 4 | `savings = …` | 0 + 1 × 2 | 1 | 2 |
| 5 | `week = …` | 1 + 1 | 2 | 2 |
| 6 | `while (week <= 4)` | 2 <= 4 → `true` | 2 | 2 |
| 7 | `savings = …` | 2 + 2 × 2 | 2 | 6 |
| 8 | `week = …` | 2 + 1 | 3 | 6 |
| 9 | `while (week <= 4)` | 3 <= 4 → `true` | 3 | 6 |
| 10 | `savings = …` | 6 + 3 × 2 | 3 | 12 |
| 11 | `week = …` | 3 + 1 | 4 | 12 |
| 12 | `while (week <= 4)` | 4 <= 4 → `true` | 4 | 12 |
| 13 | `savings = …` | 12 + 4 × 2 | 4 | 20 |
| 14 | `week = …` | 4 + 1 | 5 | 20 |
| 15 | `while (week <= 4)` | 5 <= 4 → `false` | 5 | 20 |
| 16 | `console.log(…)` | – | 5 | 20 |

```sortie
Week 5, savings €20
```

The loop went around **4 times**, but the test was read **5 times**. The fifth test is
false: that is what stops the loop. That is why `week` is 5 at the end, and not 4.

```text
   ┌──► test: week <= 4 ? ── no ──► leave the loop
   │          │
   │         yes
   │          │
   │     add to the savings
   │     week + 1
   └──────────┘
```

Try it: replace `week <= 4` with `week < 4`. The loop goes around 3 times, and the
program prints `Week 4, savings €12`.

> **Remember:** in a loop, write down the test at each round, **including the last
> one**, the one that is false. That is where off-by-one mistakes hide (one round too
> many or too few).

## Program 4: a function called twice

A [function](../debuter/glossaire.md#function) is a small machine: you give it a value,
it works, it gives back a result. Here, you call it twice.

```javascript
function square(n) {
  const result = n * n;
  console.log(`square(${n}) returns ${result}`);
  return result;
}

const a = square(3);
const b = square(a - 5);
console.log(`a + b = ${a + b}`);
```

**Before running it, guess what is printed.** How many lines? In what order?

Lines 1 to 5 **define** the function: nothing runs yet. It is a recipe put away in a
drawer. The program really starts at `const a = square(3);`.

Here is who calls whom, and what comes back:

```text
 program                          square
 ───────                          ──────
 const a = square(3)  ── n = 3 ─► result = 9
                                  prints "square(3) returns 9"
           a = 9      ◄── 9 ────  return 9

 const b = square(4)  ── n = 4 ─► result = 16
                                  prints "square(4) returns 16"
           b = 16     ◄── 16 ───  return 16

 prints "a + b = 25"
```

The trace has a "where?" column so you know which part you are in:

| Step | Where? | Line | n | result | a | b | printed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | program | `const a = square(3);` | – | – | – | – | |
| 2 | square | `const result = n * n;` | 3 | 9 | – | – | |
| 3 | square | `console.log(…)` | 3 | 9 | – | – | `square(3) returns 9` |
| 4 | square | `return result;` | 3 | 9 | – | – | |
| 5 | program | `a` receives 9 | – | – | 9 | – | |
| 6 | program | `const b = square(a - 5);`: 9 − 5 = 4 | – | – | 9 | – | |
| 7 | square | `const result = n * n;` | 4 | 16 | 9 | – | |
| 8 | square | `console.log(…)` | 4 | 16 | 9 | – | `square(4) returns 16` |
| 9 | square | `return result;` | 4 | 16 | 9 | – | |
| 10 | program | `b` receives 16 | – | – | 9 | 16 | |
| 11 | program | `console.log(…)` | – | – | 9 | 16 | `a + b = 25` |

```sortie
square(3) returns 9
square(4) returns 16
a + b = 25
```

Three remarks:

- at each call, `n` and `result` are **brand-new** boxes. They disappear at the
  `return` (the dash `–` at steps 5 and 10);
- before calling, JavaScript computes the [argument](../debuter/glossaire.md#argument):
  `a - 5` becomes `4`, and it is `4` that goes into `n`;
- the `console.log` **inside** the function is printed during the call, before the last
  line of the program.

Try it: replace `square(a - 5)` with `square(square(2))`. The inner call happens first
(`square(2) returns 4`), then the outer call (`square(4) returns 16`). The output becomes
`square(3) returns 9`, `square(2) returns 4`, `square(4) returns 16`, then `a + b = 25`.

> **Remember:** a function call is a round trip. You leave with the argument, you run
> the lines of the function with brand-new boxes, you come back with the value of the
> `return`.

## Program 5: a loop over an array with a condition

The last program mixes everything: an array, a loop and two conditions. It counts the
passing grades (10 or more) and looks for the best grade.

```javascript
const grades = [12, 7, 15, 9, 18];
let passed = 0;
let best = grades[0];

for (const grade of grades) {
  if (grade >= 10) {
    passed = passed + 1;
  }
  if (grade > best) {
    best = grade;
  }
}

console.log(`Passed: ${passed}`);
console.log(`Best: ${best}`);
```

**Before running it, guess what is printed.**

`for (const grade of grades)` puts each value of the array into `grade`, one after the
other. It is like dealing cards: one card per round.

```text
 grades:  [ 12 ,  7 , 15 ,  9 , 18 ]
            │     │    │    │    │
 round:     1     2    3    4    5
```

This time, you write **one row per round** (it is shorter), with both tests:

| Round | grade | grade >= 10 ? | passed | grade > best ? | best |
| --- | --- | --- | --- | --- | --- |
| start | – | – | 0 | – | 12 |
| 1 | 12 | `true` | 1 | 12 > 12 → `false` | 12 |
| 2 | 7 | `false` | 1 | 7 > 12 → `false` | 12 |
| 3 | 15 | `true` | 2 | 15 > 12 → `true` | 15 |
| 4 | 9 | `false` | 2 | 9 > 15 → `false` | 15 |
| 5 | 18 | `true` | 3 | 18 > 15 → `true` | 18 |

```sortie
Passed: 3
Best: 18
```

In round 4, you compare 9 with **15**, not with 12: `best` changed in round 3. You always
read the value on the row above.

Try it: add `20` at the start of the array. The best grade is already the first one:
`best` never changes, and the program prints `Passed: 4` then `Best: 20`.

> **Remember:** for a loop over an array, one row per round is enough: the value of the
> round, the answer of each test, then the boxes that change.

## Making the program print its steps

On paper, you are the one keeping the trace. But the program can also write it itself.
Add a `console.log` **with labels** inside the loop:

```javascript
const grades = [12, 7, 15];
let passed = 0;

for (const grade of grades) {
  if (grade >= 10) {
    passed = passed + 1;
  }
  console.log(`grade=${grade} passed=${passed}`);
}
```

```sortie
grade=12 passed=1
grade=7 passed=1
grade=15 passed=2
```

Each printed line is a row of your trace table. Compare them: the first difference shows
where the problem is.

A handy shortcut: put the variables between curly braces. `console.log` then writes the
name **and** the value of each one.

```javascript
let a = 3;
let b = 5;
console.log({ a, b });
a = a + b;
console.log({ a, b });
```

```sortie
{ a: 3, b: 5 }
{ a: 8, b: 5 }
```

To get a real table, store each step in a list, then use `console.table`:

```javascript
const steps = [];
let total = 0;
for (let i = 1; i <= 3; i++) {
  total = total + i;
  steps.push({ i, total });
}
console.table(steps);
```

```sortie
┌─────────┬───┬───────┐
│ (index) │ i │ total │
├─────────┼───┼───────┤
│ 0       │ 1 │ 1     │
│ 1       │ 2 │ 3     │
│ 2       │ 3 │ 6     │
└─────────┴───┴───────┘
```

> **Remember:** a `console.log` with labels inside a loop is a trace table written by the
> program. Remove it once the bug is found.

## The debugger in two sentences

A [debugger](../debuter/glossaire.md#debugging) pauses the program on a line (with the
`debugger;` statement or a click in the editor's margin) and shows you the value of each
variable. You then move forward one line at a time: it is the trace table, filled in by
the computer; everything is explained in [debugging](debogage.md).

## Common mistakes

### Forgetting the last test of a loop

You assume the loop stops "at the right moment", without checking the last round.

```javascript norun
const grades = [12, 15, 9];
let sum = 0;
for (let i = 0; i <= grades.length; i++) {
  sum = sum + grades[i];
}
console.log(sum);
```

```text
NaN
```

The trace shows the problem. `grades.length` is 3. The test `3 <= 3` is true: there is a
**fourth** round, with `i = 3`. But `grades[3]` does not exist: it is `undefined`, and
`36 + undefined` gives `NaN`.

| Round | i | i <= 3 ? | grades[i] | sum |
| --- | --- | --- | --- | --- |
| 1 | 0 | `true` | 12 | 12 |
| 2 | 1 | `true` | 15 | 27 |
| 3 | 2 | `true` | 9 | 36 |
| 4 | 3 | `true` | `undefined` | `NaN` |

Fix: `i < grades.length`. The [indexes](../debuter/glossaire.md#index) go from `0` to
`grades.length - 1`.

### Reusing an old value

You want to swap `a` and `b`, and in your head you read the **old** value of `a`.

```javascript norun
let a = 3;
let b = 5;
a = b;
b = a;
console.log(a, b);
```

```text
5 5
```

You expected `5 3`. But after `a = b;`, the box `a` contains 5: the 3 is lost. So
`b = a;` copies 5. In the trace, line 3 writes 5 in the `a` column, and line 4 reads that
column.

Fix: keep the old value in a third box.

```javascript
let a = 3;
let b = 5;
const oldA = a;
a = b;
b = oldA;
console.log(a, b);
```

```sortie
5 3
```

### Using a box before creating it

In the trace, a column that holds `–` cannot be read. The program refuses it too.

```javascript norun
let total = total + 5;
console.log(total);
```

```text
let total = total + 5;
            ^

ReferenceError: Cannot access 'total' before initialization
```

Fix: first give a starting value (`let total = 0;`), then change it on the next line
(`total = total + 5;`).

### Changing a const box

Your trace shows that `count` changes at each round. A box that changes must be created
with `let`, not `const`.

```javascript norun
const count = 0;
for (const n of [4, 8]) {
  count = count + n;
}
```

```text
  count = count + n;
        ^

TypeError: Assignment to constant variable.
```

Fix: `let count = 0;`. Tip: if a column in your table changes value, it is a `let`.

## Test yourself

**Question 1:** what is the value of `x` at the end?

```javascript norun
let x = 2;
x = x * 3;
x = x + x;
```

> **Answer**
>
> `12`. Trace: `x` is 2, then 2 × 3 = 6, then 6 + 6 = 12. The third line reads 6, the
> most recent value.

**Question 2:** how many times is the test `i < 3` read in
`for (let i = 0; i < 3; i++)`?

> **Answer**
>
> 4 times: for `i = 0`, `1` and `2` (true, the loop runs), then for `i = 3` (false, the
> loop stops). The loop goes around 3 times, but the test is read one more time.

**Question 3:** what does this program print?

```javascript norun
function plusOne(n) {
  return n + 1;
}
const r = plusOne(plusOne(5));
console.log(r);
```

> **Answer**
>
> `7`. The inner call happens first: `plusOne(5)` returns 6. Then `plusOne(6)` returns
> 7, which is stored in `r`.

## Summary

- A trace table has one column per variable and one row per step.
- To the right of the `=`, you read the **most recent** value; on the left, you store
  the result.
- For an `if`, write down the test and its answer; for a loop, write down the test at
  each round, including the last one, which is false.
- A function call is a round trip, with brand-new boxes at each call.
- `console.log` with labels makes the program write the trace; the debugger shows it
  line by line.

Next: [beginner traps in JavaScript](pieges.md)
