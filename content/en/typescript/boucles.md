# Loops

This page explains how to repeat actions with `for`, `while` and `for...of` loops, and how to stop them with `break` and `continue`.

> **Goals:**
>
> - you will know how to repeat with `for`, `while` and `for...of`;
> - you will know how to skip a round with `continue` and leave a loop with `break`.

This chapter follows [conditions](conditions-boucles.md): a loop stops thanks to a condition, like the ones you already know how to write.

## Loops

Imagine you are asked to print the numbers from 1 to 100. Writing 100 `console.log`
lines would be long and tedious. A loop does this repetitive work for you.

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

**Your turn:** write a program that prints the 7 times table, from "7 x 1 = 7" to
"7 x 5 = 35", one line per calculation.

> **Solution**
>
> ```typescript
> for (let i = 1; i <= 5; i++) {
>   console.log(`7 x ${i} = ${7 * i}`);
> }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

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

Line by line:

- at the start, `money` is 100 and `years` is 0;
- before each round, `while` checks `money < 150`. 100 is smaller: it enters the
  block;
- in each round, `money` grows by 10% (110, then 121, 133.1, 146.41, 161.051) and
  `years` goes up by 1;
- after 5 rounds, `money < 150` is false: the loop stops, and the last `console.log`
  prints the result, rounded to two digits by `toFixed(2)`.

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

**Your turn:** write a program that stores the shopping list `["bread", "milk",
"eggs"]` in a constant, then prints each item on its own line, after a dash.

> **Solution**
>
> ```typescript
> const shopping = ["bread", "milk", "eggs"];
>
> for (const item of shopping) {
>   console.log(`- ${item}`);
> }
> ```
>
> ```sortie
> - bread
> - milk
> - eggs
> ```

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

## Common mistakes

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

### Declaring the counter with const

```typescript norun
for (const i = 0; i < 3; i++) {
  console.log(i);
}
```

```text
main.ts:1:26 - error TS2588: Cannot assign to 'i' because it is a constant.

1 for (const i = 0; i < 3; i++) {
                           ~


Found 1 error in main.ts:1
```

"Cannot assign to `i`: it is a constant." `i++` changes the counter on every round, so
it must be created with `let`, as in `for (let i = 0; i < 3; i++)`. In a `for...of`,
on the other hand, `const` is fine: each round creates a new item.

## Test yourself

**Question 1:** write a loop that prints the numbers from 10 down to 0, in steps of 2.

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

**Question 2:** what does this code print?

```typescript norun
let n = 1;
while (n < 20) {
  n = n * 3;
}
console.log(n);
```

> **Answer**
>
> `27`. `n` is 1, then 3, 9 and 27. At 27, `n < 20` is false: the loop stops and
> `console.log` prints 27.

**Question 3:** what is the difference between `break` and `continue`?

> **Answer**
>
> `break` leaves the loop right away. `continue` skips the rest of the round and
> jumps straight to the next round: the loop keeps going.

## Summary

- A loop repeats a block of code.
- `for` counts: a start, a condition to keep going and a step.
- `while` repeats as long as its condition is true: something must change on every
  round.
- `for...of` goes through a list, one item at a time.
- `break` leaves the loop, `continue` jumps to the next round.

Next: [functions](fonctions.md).
