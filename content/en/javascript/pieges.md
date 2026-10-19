# Beginner traps in JavaScript

This page gathers the 11 traps that surprise JavaScript beginners the most, with, for each one, what you expect, what really happens, why, and how to avoid it.

> **Goals:**
>
> - you will recognize the classic JavaScript traps when you run into them;
> - you will understand why the result is surprising;
> - you will know how to write the correct version every time.

## How to read this page

JavaScript was created very quickly, in 1995. It tries to be "nice": instead of
stopping on an error, it often guesses what you meant. Sometimes it guesses wrong.

Each trap follows the same path:

```text
 What you expect  →  What happens  →  Why  →  The fix
```

Read one trap at a time. Run each example yourself: once you have seen a trap with
your own eyes, you never forget it.

## Value traps

### Comparing with == instead of ===

**You expect:** `0 == ""` to answer `false`. A number is not a text.

```javascript
console.log(0 == "");
console.log(1 == "1");
console.log(null == undefined);
console.log(0 === "");
console.log(1 === "1");
```

```sortie
true
true
true
false
false
```

**Why:** `==` ("loose" equality) **converts** both values before comparing them. The
empty text `""` becomes the number `0`, so `0 == ""` is true. `===` ("strict" equality)
converts nothing: different [types](../debuter/glossaire.md#type) are always different.

**The fix:** **always** use `===` and `!==`. See also
[the first steps](premiers-pas.md#comparing-or).

> **Remember:** three signs `===` to compare. Two signs `==`, never.

### "1" + 1 does not make 2

**You expect:** `"1" + 1` to give `2`.

```javascript
console.log("1" + 1);
console.log("5" - 2);
console.log("5" * "2");
console.log("10" + 5 - 3);
```

```sortie
11
3
10
102
```

**Why:** `+` has two jobs. With two numbers, it adds. As soon as there is **a text**, it
sticks the two pieces together, like two pieces of ribbon. The other operators (`-`,
`*`, `/`) only know how to calculate: they turn the text into a number.

Let's follow `"10" + 5 - 3` step by step:

| Step | Calculation | Result |
| --- | --- | --- |
| 1 | `"10" + 5` | `"105"` (a text, stuck together) |
| 2 | `"105" - 3` | `102` (a number, calculated) |

This is very common with a value typed by the user: it **always** arrives as text.

**The fix:** turn the text into a number with `Number(...)` before calculating.

```javascript
const input = "1";
console.log(Number(input) + 1);
```

```sortie
2
```

Try it: replace `"1"` with `"abc"`. You get `NaN`, the next trap.

### NaN is equal to nothing, not even itself

`NaN` means *Not a Number*. It is the result of an impossible calculation.

**You expect:** `result === NaN` to tell you whether the calculation failed.

```javascript
const result = Number("abc");

console.log(result);
console.log(result === NaN);
console.log(NaN === NaN);
console.log(Number.isNaN(result));
```

```sortie
NaN
false
false
true
```

**Why:** by convention, `NaN` is equal to **no** value, itself included. So the
comparison with `===` always answers `false`.

**The fix:** test with `Number.isNaN(value)`. See
[numbers](nombres.md#nan-not-a-number).

### 0.1 + 0.2 does not make 0.3

**You expect:** `0.3`.

```javascript
console.log(0.1 + 0.2);
console.log(0.1 + 0.2 === 0.3);
console.log((0.1 + 0.2).toFixed(2));
```

```sortie
0.30000000000000004
false
0.30
```

**Why:** the computer stores decimal numbers in
[binary](../debuter/glossaire.md#binary) (with 0s and 1s). Some numbers, like `0.1`,
do not come out exactly in binary, just as `1/3` does not come out exactly in decimal
(`0.3333…`). A tiny error remains.

**The fix:**

- to **display**, round with `toFixed(2)`;
- to **compare**, check that the gap is tiny;
- for **money**, count in cents (whole numbers).

```javascript
const gap = Math.abs(0.1 + 0.2 - 0.3);
console.log(gap < 0.000001);

const totalCents = 10 + 20;
console.log(totalCents / 100);
```

```sortie
true
0.3
```

See [numbers](nombres.md#decimal-numbers-are-not-exact).

### undefined and null: two kinds of "empty"

**You expect:** a single value to say "nothing".

```javascript
let firstName;
const cat = { name: "Felix" };
const data = null;

console.log(firstName);
console.log(cat.age);
console.log(data);
console.log(typeof undefined);
console.log(typeof null);
```

```sortie
undefined
undefined
null
undefined
object
```

**Why:** JavaScript has **two** empty values.

| Value | Who puts it there? | Meaning |
| --- | --- | --- |
| `undefined` | JavaScript, on its own | "nothing was ever put here" |
| `null` | you, on purpose | "there is nothing here, I know it" |

And `typeof null` answers `"object"`: it is an old mistake in the language, kept so as
not to break old programs.

**The fix:** to test "empty or not", use `??` for a fallback value and `?.` to read
without crashing (see [missing values](conditions-boucles.md#missing-values-and)).

```javascript
const data = null;
console.log(data ?? "no data");
console.log(data?.length);
```

```sortie
no data
undefined
```

> **Remember:** `undefined` = never filled; `null` = emptied on purpose. `??` and `?.`
> treat both the same way.

## Array and object traps

### sort() sorts numbers like words

**You expect:** `[1, 5, 10, 25, 100]`.

```javascript
const numbers = [10, 1, 5, 100, 25];
numbers.sort();
console.log(numbers);
```

```sortie
[ 1, 10, 100, 25, 5 ]
```

**Why:** without instructions, `sort` turns each number into text and sorts in
dictionary order. In a dictionary, "10" comes before "5", because you compare the first
character first: `1` before `5`.

**The fix:** give a comparison function.

```javascript
const numbers = [10, 1, 5, 100, 25];
numbers.sort((a, b) => a - b);
console.log(numbers);
```

```sortie
[ 1, 5, 10, 25, 100 ]
```

Try it: write `b - a` instead of `a - b`. The order is reversed. See
[arrays](collections.md#sorting).

### const does not prevent changing the contents

**You expect:** an array declared with `const` to never change again.

```javascript
const cart = ["apple"];
cart.push("pear");
cart[0] = "kiwi";
console.log(cart);

const cat = { name: "Felix" };
cat.name = "Tom";
console.log(cat);
```

```sortie
[ 'kiwi', 'pear' ]
{ name: 'Tom' }
```

**Why:** imagine a box with a label. `const` sticks the label forever on **the same
box**. But you can still change what is **inside** the box.

```text
 cart ──(stuck forever)──▶ [ 'kiwi', 'pear' ]
                             ▲ the contents can change
```

What is forbidden is putting the label on **another** box: `cart = ["pear"]` causes
`TypeError: Assignment to constant variable.`

**The fix:** if the contents really must not change, freeze them with `Object.freeze`.

```javascript
const days = Object.freeze(["Monday", "Tuesday"]);
console.log(Object.isFrozen(days));
```

```sortie
true
```

### b = a does not copy

**You expect:** `b` to be an independent copy of `a`.

```javascript
const a = [1, 2, 3];
const b = a;
b.push(4);
console.log(a);
```

```sortie
[ 1, 2, 3, 4 ]
```

**Why:** `b = a` sticks a **second label** on the same box. There is only one array,
with two names.

```text
 a ──┐
     ├──▶ [ 1, 2, 3, 4 ]
 b ──┘
```

**The fix:** make a copy with the spread `...`.

```javascript
const a = [1, 2, 3];
const b = [...a];
b.push(4);
console.log(a);
console.log(b);
```

```sortie
[ 1, 2, 3 ]
[ 1, 2, 3, 4 ]
```

Be careful, this copy only copies the first level. For an array of objects, see
[arrays in depth](tableaux-avances.md).

> **Remember:** for arrays and objects, `=` shares. `[...a]` and `{ ...o }` copy.

### this gets lost when you detach a method

**You expect:** the method to work everywhere, even stored in a variable.

```javascript norun
const account = {
  owner: "Alice",
  summary() {
    return `Account of ${this.owner}`;
  },
};

const show = account.summary;
console.log(show());
```

```text
file:///home/alice/js/main.mjs:4
    return `Account of ${this.owner}`;
                              ^

TypeError: Cannot read properties of undefined (reading 'owner')
    at summary (file:///home/alice/js/main.mjs:4:31)
    at file:///home/alice/js/main.mjs:9:13

Node.js v22.22.0
```

**Why:** `this` is not set when you **write** the method. It is chosen at the moment
of the **call**: it is the object before the dot. In `account.summary()`, `this` is
`account`. In `show()`, there is no dot, so no object: `this` is `undefined`.

**The fix:** call the method with its object, or attach it with `bind`.

```javascript
const account = {
  owner: "Alice",
  summary() {
    return `Account of ${this.owner}`;
  },
};

console.log(account.summary());

const show = account.summary.bind(account);
console.log(show());
```

```sortie
Account of Alice
Account of Alice
```

See [methods and this](objets.md#methods-and-this).

## Time and loop traps

### Forgetting await

**You expect:** `getGrade()` to give you the grade `15`.

```javascript
function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function getGrade() {
  await wait(10);
  return 15;
}

const grade = getGrade();
console.log(grade);
console.log(grade + 1);
```

```sortie
Promise { <pending> }
[object Promise]1
```

**Why:** an `async` function does not return the value right away. It returns a
[promise](../debuter/glossaire.md#promise): an order ticket, like at a fast-food
restaurant. The ticket is not the meal! `<pending>` means "being prepared".

**The fix:** wait for the meal with `await`.

```javascript
function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function getGrade() {
  await wait(10);
  return 15;
}

const grade = await getGrade();
console.log(grade);
console.log(grade + 1);
```

```sortie
15
16
```

See [asynchronous code](asynchrone.md).

### var in a loop: the same box for everyone

**You expect:** `0`, `1`, `2`.

```javascript
for (var i = 0; i < 3; i++) {
  setTimeout(() => console.log("var:", i), 0);
}
```

```sortie
var: 3
var: 3
var: 3
```

**Why:** `setTimeout` displays **later**, after the end of the loop. With `var`, there
is **only one** box `i` for the whole loop. When the messages are displayed, the loop
is over and `i` is 3.

| Moment | i | What happens |
| --- | --- | --- |
| turn 1 | 0 | a display is scheduled for later |
| turn 2 | 1 | a display is scheduled for later |
| turn 3 | 2 | a display is scheduled for later |
| end of loop | 3 | `3 < 3` is false: the loop stops |
| later | 3 | the 3 displays read the same box: `3` |

**The fix:** use `let`. With `let`, **each turn** has its own box `i`.

```javascript
for (let i = 0; i < 3; i++) {
  setTimeout(() => console.log("let:", i), 0);
}
```

```sortie
let: 0
let: 1
let: 2
```

> **Remember:** never use `var`. Write `const`, or `let` if the value has to change.

## Common mistakes

### Reading a property of null

```javascript norun
const response = { data: null };
console.log(response.data.length);
```

```text
file:///home/alice/js/main.mjs:2
console.log(response.data.length);
                          ^

TypeError: Cannot read properties of null (reading 'length')
    at file:///home/alice/js/main.mjs:2:27

Node.js v22.22.0
```

You cannot read anything in an empty box. Use `response.data?.length`, which gives
`undefined` instead of crashing.

### Calling a number method on a text

```javascript norun
const age = "17";
console.log(age.toFixed(1));
```

```text
file:///home/alice/js/main.mjs:2
console.log(age.toFixed(1));
                ^

TypeError: age.toFixed is not a function
    at file:///home/alice/js/main.mjs:2:17

Node.js v22.22.0
```

`"17"` is a text, not a number: it has no `toFixed` method. Convert it first:
`Number(age).toFixed(1)`.

### Replacing an array declared with const

```javascript norun
const cart = ["apple"];
cart = ["pear"];
```

```text
file:///home/alice/js/main.mjs:2
cart = ["pear"];
     ^

TypeError: Assignment to constant variable.
    at file:///home/alice/js/main.mjs:2:6

Node.js v22.22.0
```

Change the contents (`cart.length = 0; cart.push("pear")`), or declare it with `let` if
you really need to switch to another array.

## Test yourself

**Question 1:** what does `console.log("3" + 4 + 5)` display?

> **Answer**
>
> `345`. You read from left to right: `"3" + 4` gives the text `"34"`, then `"34" + 5`
> gives `"345"`.
>
> ```javascript
> console.log("3" + 4 + 5);
> console.log(3 + 4 + "5");
> ```
>
> ```sortie
> 345
> 75
> ```
>
> In the second line, `3 + 4` first makes `7` (two numbers), then `7 + "5"` sticks them
> together: `"75"`.

**Question 2:** how do you know whether `x` is `NaN`?

> **Answer**
>
> With `Number.isNaN(x)`. The test `x === NaN` always answers `false`, because `NaN` is
> equal to nothing.

**Question 3:** why does `[3, 20, 100].sort()` give `[100, 20, 3]`?

> **Answer**
>
> Because `sort()` without an argument compares **texts**: `"1"` comes before `"2"`,
> which comes before `"3"`. Write `sort((a, b) => a - b)`.

## Summary

- Compare with `===`, never with `==`.
- `+` sticks things together as soon as it sees a text: convert with `Number(...)`;
  test `NaN` with `Number.isNaN`.
- Decimal numbers are not exact: round to display, count in cents for money.
- `undefined` = never filled, `null` = emptied on purpose; `??` and `?.` handle both.
- `sort((a, b) => a - b)` for numbers; `const` does not prevent changing the contents;
  `b = a` shares, `[...a]` copies.
- `this` depends on the call; an `async` function returns a promise to wait for with
  `await`; `let` rather than `var`.

Next: [exercises to get started](exercices-faciles.md).
