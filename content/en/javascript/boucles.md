# Loops

This page shows how to repeat actions with `while`, `for` and `for...of`, and how to stop a loop or skip a round.

> **Goals:**
>
> - you will be able to repeat code with `while`, `for` and `for...of`;
> - you will be able to leave a loop with `break` or skip a round with `continue`;
> - you will be able to choose between `for...of` (the values) and `for...in` (the keys).

This chapter follows [conditions](conditions-boucles.md): a loop stops thanks to a
condition, just like the ones you write in an `if`.

## The while loop

Imagine you are asked to display the numbers from 1 to 100. Writing 100 `console.log`
would be long and tedious. A loop does this repetitive work for you.

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

Line by line:

- `let counter = 3;`: the variable starts at 3. It is a `let`, because it will change;
- `while (counter > 0)`: before each round, JavaScript asks the question. `3 > 0` is
  true: it enters the block;
- the block displays `counter`, then `counter--` takes 1 away: it becomes 2, then 1,
  then 0;
- when `counter` is 0, `0 > 0` is false: JavaScript leaves the loop and moves on to
  the last `console.log`.

> **Warning:** if the condition always stays true, the loop never stops. Here,
> `counter--` brings the counter closer to 0 on each round. To stop a stuck program,
> press **Ctrl + C** in the terminal.

**Your turn:** write a program that displays the numbers from 1 to 5 with a `while`
loop, then "Done!".

> **Solution**
>
> ```javascript
> let n = 1;
>
> while (n <= 5) {
>   console.log(n);
>   n++;
> }
> console.log("Done!");
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> Done!
> ```

## The for loop

Many loops count: from 1 to 5, from 1 to 10… With `while`, the start, the question and
the step are scattered over several lines, and it is easy to forget one. `for` brings
them together.

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

**Your turn:** write a program that displays the 7 times table, from "7 x 1 = 7" to
"7 x 5 = 35", one line per calculation.

> **Solution**
>
> ```javascript
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

## for...of and for...in

Very often, you have a list of things to handle one by one: each item on a shopping
list, each student in a class. These two loops are made for that.

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

Line by line:

- `fruits` holds an array of three texts;
- `for (const fruit of fruits)` takes the first element, `"apple"`, and stores it in
  `fruit`;
- the block displays the sentence with `"apple"`. Then JavaScript goes back to the
  `for`, which stores `"pear"` in `fruit`, and so on;
- after `"kiwi"`, there are no more elements: the loop stops. The second loop does the
  same with the letters of `"abc"`.

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

Sometimes you want to change the course of a loop on the way: stop searching as soon
as you have found something, or ignore some elements. That is the job of these two
words.

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

Line by line:

- `for (const n of numbers)` gives `n` the values 4, 7, -1, 12… one by one;
- if `n` is negative, `continue` moves straight on to the next round: the
  `console.log` is not reached;
- if `n` is 0, `break` stops the whole loop;
- otherwise, `console.log(n)` displays the number.

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

**Question 1:** how many times does this code display "Hello"?

```javascript norun
let n = 0;
while (n < 3) {
  console.log("Hello");
  n++;
}
```

> **Answer**
>
> Three times. `n` is 0, 1 then 2 during the rounds; when it is 3, `n < 3` is false
> and the loop stops.

**Question 2:** what does this code display?

```javascript norun
for (const n of [1, 2, 3, 4, 5]) {
  if (n === 2) {
    continue;
  }
  if (n === 4) {
    break;
  }
  console.log(n);
}
```

> **Answer**
>
> `1` then `3`. `2` is skipped by `continue`. At `4`, `break` stops the loop: `5` is never reached.

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

- `while` repeats as long as a condition is true, `for` counts, `for...of` goes through
  the values, `for...in` the keys of an object.
- `break` leaves the loop, `continue` moves on to the next round.
- In a counting `for`, declare the counter with `let`; in a `for...of`, `const` is
  fine.

Next: [functions](fonctions.md).
