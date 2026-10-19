# Numbers and calculations

This page explains how JavaScript does math: operators, the `Math` object, the surprises of decimal numbers, `NaN`, `Infinity`, `BigInt` and formatting numbers for display.

> **Goals:**
>
> - you will know how to calculate with `+ - * /`, the remainder `%` and the power `**`;
> - you will know how to round and use the `Math` functions;
> - you will know why `0.1 + 0.2` does not give exactly `0.3`;
> - you will know how to turn a text into a number and recognize `NaN`;
> - you will know how to format a number for display with `Intl.NumberFormat`.

## A single number type

Programs calculate all the time: the total of a shopping cart, a score, a person's
age. Before doing any math, let's see what a number looks like in JavaScript.

Many languages separate [integers](../debuter/glossaire.md#integer) (`3`) and
[floats](../debuter/glossaire.md#float) (`3.5`). JavaScript has only one type for
both: `number`.

```javascript
console.log(typeof 3);
console.log(typeof 3.5);
console.log(3 === 3.0);
console.log(Number.isInteger(5.0));
```

```sortie
number
number
true
true
```

Line by line:

- `typeof 3` asks for the type of the value `3`: the answer is `number`;
- `typeof 3.5` also gives `number`: same type, with or without a decimal point;
- `3 === 3.0` compares the two values: they are equal, so `true`;
- `Number.isInteger(5.0)` answers `true`, because `5.0` has nothing after the point.

`3` and `3.0` are exactly the same value. `Number.isInteger` tells you whether a number
has no decimal part.

## Operators

A program often does the math for you, like a calculator: the price of three coffees,
the average grade of a class. Operators are the signs that write these calculations.

The calculation [operators](../debuter/glossaire.md#operator) are the ones from school,
plus two new ones.

```javascript
console.log(7 + 2);
console.log(7 - 2);
console.log(7 * 2);
console.log(7 / 2);
console.log(7 % 2);
console.log(2 ** 10);
```

```sortie
9
5
14
3.5
1
1024
```

- `/` is a real division: `7 / 2` gives `3.5`, not `3`.
- `%` gives the **remainder** of the whole-number division: 7 = 2 × 3 + **1**.
- `**` is the power: `2 ** 10` means 2 multiplied by itself 10 times.

The order of operations is the same as in math: `**`, then `*` `/` `%`, then `+` `-`.
Parentheses change the order.

```javascript
console.log(2 + 3 * 4);
console.log((2 + 3) * 4);
```

```sortie
14
20
```

### What is the remainder for?

The remainder `%` answers everyday questions: is a number even? How many minutes are
left after the full hours?

```javascript
const n = 17;
console.log(n % 2 === 0);

const minutes = 135;
console.log(`${Math.floor(minutes / 60)} h ${minutes % 60} min`);
```

```sortie
false
2 h 15 min
```

A number is even if its remainder when divided by 2 is 0.

### Shortcuts

Like `+=`, which you have already seen, there are `-=`, `*=` and `/=`. To add or remove
1, you write `++` and `--`.

```javascript
let lives = 3;
lives -= 1;
lives *= 10;
lives++;
console.log(lives);
```

```sortie
21
```

**Your turn:** write a program that shares 23 candies among 5 children. It
displays how many candies each child gets, then how many are left.

> **Solution**
>
> ```javascript
> const candies = 23;
> const children = 5;
> console.log(Math.floor(candies / children));
> console.log(candies % children);
> ```
>
> ```sortie
> 4
> 3
> ```

## The Math object

Some calculations come up again and again: rounding a price, finding the best grade
in a list. Instead of writing them yourself, you take them ready-made.

`Math` is a toolbox that comes with JavaScript. It contains calculation functions and a
few constants.

```javascript
console.log(Math.round(2.6));
console.log(Math.floor(2.6));
console.log(Math.ceil(2.1));
console.log(Math.trunc(-2.6));
console.log(Math.abs(-4));
console.log(Math.sqrt(16));
console.log(Math.max(3, 8, 1));
console.log(Math.min(3, 8, 1));
console.log(Math.PI);
```

```sortie
3
2
3
-2
4
4
8
1
3.141592653589793
```

| Function | Role |
| --- | --- |
| `Math.round(x)` | rounds to the nearest |
| `Math.floor(x)` | rounds down (the "floor") |
| `Math.ceil(x)` | rounds up (the "ceiling") |
| `Math.trunc(x)` | cuts off the decimal part |
| `Math.abs(x)` | absolute value (removes the minus sign) |
| `Math.sqrt(x)` | square root |
| `Math.max(...)`, `Math.min(...)` | the largest, the smallest |

`Math.round` rounds to a whole number. To keep two digits after the decimal point, here
is a trick: multiply by 100, round, then divide by 100.

```javascript
console.log(Math.round(3.456 * 100) / 100);
```

```sortie
3.46
```

### A random number

`Math.random()` gives a random number between 0 (included) and 1 (not included). For a
six-sided die, you stretch it and then round it down:

```javascript norun
const die = Math.floor(Math.random() * 6) + 1;
console.log(die);
```

```text
4
```

The result changes every time you run it: yours will probably be different.

**Your turn:** write a program that displays the best of the three grades 12, 17
and 9, then their average rounded to the nearest whole number.

> **Solution**
>
> ```javascript
> console.log(Math.max(12, 17, 9));
> console.log(Math.round((12 + 17 + 9) / 3));
> ```
>
> ```sortie
> 17
> 13
> ```

## Decimal numbers are not exact

This part saves you from a classic trap, such as a cart total with about fifteen
digits after the decimal point. Better to know it before you run into it.

Here is a famous surprise:

```javascript
console.log(0.1 + 0.2);
console.log(0.1 + 0.2 === 0.3);
```

```sortie
0.30000000000000004
false
```

The computer stores numbers in binary (with 0s and 1s). Some simple decimal numbers,
like 0.1, cannot be written exactly in binary, just as 1/3 cannot be written exactly in
decimal (0.3333…). A tiny error remains.

This is not a JavaScript bug: Python, Ruby, Java… do the same. Two rules are enough:

- to **display**, round (see `toFixed` below);
- to **compare**, check that the difference is tiny.

```javascript
const sum = 0.1 + 0.2;
console.log(Math.abs(sum - 0.3) < Number.EPSILON);
```

```sortie
true
```

`Number.EPSILON` is a tiny number, meant for this kind of comparison.

> **Tip:** for money, count in **cents** with integers: `1999` rather than `19.99`.
> Integers are exact.

## Numbers and text

Numbers and texts often turn into each other. What you type on the keyboard arrives as
a text, even "42". And to display a price nicely, you turn a number into a text. This
part builds the bridge in both directions.

### Displaying with toFixed

`toFixed(n)` returns a **text** with exactly `n` digits after the decimal point.

```javascript
const price = 0.1 + 0.2;
console.log(price.toFixed(2));
console.log((10).toFixed(2));
console.log(typeof price.toFixed(2));
```

```sortie
0.30
10.00
string
```

Line by line:

- `price` receives `0.1 + 0.2`, that is `0.30000000000000004`;
- `price.toFixed(2)` keeps two digits after the decimal point and rounds: `0.30`;
- `(10).toFixed(2)` adds the missing zeros: `10.00`;
- `typeof` shows that the result of `toFixed` is a `string`, so a text.

The result is a string: perfect for display, not for further calculations.

> **Note:** `(10).toFixed(2)` needs parentheses. Without them, `10.` would be read as
> the start of a decimal number.

### Turning a text into a number

A text that contains digits is not a number. Three tools convert it:

```javascript
console.log(Number("42"));
console.log(Number("3.5"));
console.log(Number("12px"));
console.log(parseInt("12px"));
console.log(parseInt("3.9"));
console.log(parseFloat("3.5kg"));
```

```sortie
42
3.5
NaN
12
3
3.5
```

- `Number(text)` is strict: the whole text must be a number.
- `parseInt(text)` reads an integer at the start of the text and stops at the first
  character that is not a digit.
- `parseFloat(text)` does the same, but keeps the decimal part.

> **Remember:** prefer `Number(...)`. It rejects doubtful texts instead of silently
> taking a piece of them.

**Your turn:** write a program that turns the text `"19.99"` into a number, adds
5 to it, then displays the result with two digits after the decimal point.

> **Solution**
>
> ```javascript
> const text = "19.99";
> const total = Number(text) + 5;
> console.log(total.toFixed(2));
> ```
>
> ```sortie
> 24.99
> ```

## NaN: "not a number"

A user may type "thirty" instead of "30". Your program must then be able to spot
that the conversion failed.

When a calculation makes no sense, JavaScript does not stop: it returns `NaN` (*Not a
Number*). And `NaN` contaminates every calculation that uses it.

```javascript
const age = Number("thirty");
console.log(age);
console.log(age + 1);
console.log(typeof age);
console.log(age === NaN);
console.log(Number.isNaN(age));
```

```sortie
NaN
NaN
number
false
true
```

Line by line:

- `Number("thirty")` cannot read this word as a number: `age` is `NaN`;
- `age + 1` still gives `NaN`: a calculation with `NaN` always gives `NaN`;
- `typeof age` answers `number`, which is surprising (see below);
- `age === NaN` answers `false`, while `Number.isNaN(age)` does answer `true`.

Two oddities:

- `typeof NaN` answers `"number"`: `NaN` is a special value of the number type;
- `NaN` is equal to nothing, **not even to itself**: `age === NaN` is always false.

To test for `NaN`, use `Number.isNaN(x)`. There is also an old `isNaN(x)`, which first
converts its argument to a number: `isNaN("abc")` answers `true` even though `"abc"` is a
text, not `NaN`.

```javascript
console.log(isNaN("abc"));
console.log(Number.isNaN("abc"));
```

```sortie
true
false
```

## Infinity

> **Note:** this part can wait. Read it once the basics feel solid.

Dividing by zero does not cause an error in JavaScript: you get infinity.

```javascript
console.log(1 / 0);
console.log(-1 / 0);
console.log(0 / 0);
console.log(Number.isFinite(1 / 0));
```

```sortie
Infinity
-Infinity
NaN
false
```

`Number.isFinite(x)` checks that a number is a real number: neither `Infinity` nor `NaN`.

## Very large integers: BigInt

> **Note:** this part can wait. Read it once the basics feel solid.

A `number` keeps integers exact up to about 9 quadrillion. Beyond that, it loses
precision.

```javascript
console.log(Number.MAX_SAFE_INTEGER);
console.log(2 ** 53);
console.log(2 ** 53 + 1);
```

```sortie
9007199254740991
9007199254740992
9007199254740992
```

`2 ** 53 + 1` should end with 3. For huge integers, add an `n` at the end: that is a
`BigInt`, exact whatever its size.

```javascript
console.log(2n ** 53n + 1n);
console.log(2n ** 64n);
console.log(7n / 2n);
console.log(BigInt(10) + 5n);
```

```sortie
9007199254740993n
18446744073709551616n
3n
15n
```

Dividing two `BigInt` values cuts off the decimal part: `7n / 2n` gives `3n`.

## Formatting numbers for display

In the United States, you write `1,234,567.89`: a comma between the thousands and a
point before the decimals. Other countries do it differently. `Intl.NumberFormat`
formats a number according to a language.

```javascript
const us = new Intl.NumberFormat("en-US");
console.log(us.format(1234567.891));

const dollars = new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" });
console.log(dollars.format(1234.5));

const percent = new Intl.NumberFormat("en-US", { style: "percent" });
console.log(percent.format(0.25));
```

```sortie
1,234,567.891
$1,234.50
25%
```

- `"en-US"` means "English from the United States".
- `style: "currency"` displays an amount of money, with the currency `USD`.
- `style: "percent"` multiplies by 100 and adds `%`.

> **Warning:** in other languages, the spaces are not always ordinary spaces. With
> `"fr-FR"` (French from France), `1234.5` becomes `1 234,5`: between the thousands, it
> is a **narrow no-break space** (Unicode code U+202F). It prevents a line break in the
> middle of a number. As a result, comparing the output with a string typed on your
> keyboard answers `false`.

**Your turn:** write a program that displays the price 12500 in US dollars, in the
United States format.

> **Solution**
>
> ```javascript
> const dollars = new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" });
> console.log(dollars.format(12500));
> ```
>
> ```sortie
> $12,500.00
> ```

## Common mistakes

### Adding a text and a number

```javascript norun
const quantity = "5";
console.log(quantity + 3);
```

```text
53
```

No error message, but a wrong result. With a text, `+` **sticks together** instead of
adding: `"5" + 3` gives the text `"53"`. Convert first: `Number(quantity) + 3` gives
`8`.

### Calling toFixed on a text

```javascript norun
const n = "3.14159";
console.log(n.toFixed(2));
```

```text
file:///home/alice/js/main.mjs:2
console.log(n.toFixed(2));
              ^

TypeError: n.toFixed is not a function
    at file:///home/alice/js/main.mjs:2:15

Node.js v22.22.0
```

`toFixed` exists for numbers, not for texts. Convert: `Number(n).toFixed(2)`.

### Mixing BigInt and number

```javascript norun
const big = 10n;
console.log(big + 1);
```

```text
file:///home/alice/js/main.mjs:2
console.log(big + 1);
                ^

TypeError: Cannot mix BigInt and other types, use explicit conversions
    at file:///home/alice/js/main.mjs:2:17

Node.js v22.22.0
```

You cannot mix a `BigInt` and a `number` in a calculation. Write `big + 1n`, or
convert: `Number(big) + 1`.

### Comparing with NaN

```javascript norun
const age = Number("abc");
if (age === NaN) {
  console.log("Invalid age");
}
```

```text
(nothing is displayed)
```

`NaN` is equal to nothing, not even to itself: the condition is always false. Write
`if (Number.isNaN(age))`.

## Test yourself

**Question 1:** what are `17 % 5` and `2 ** 3`?

> **Answer**
>
> ```javascript
> console.log(17 % 5);
> console.log(2 ** 3);
> ```
>
> ```sortie
> 2
> 8
> ```
>
> 17 = 5 × 3 + **2**, and 2 × 2 × 2 = 8.

**Question 2:** why does `0.1 + 0.2 === 0.3` answer `false`?

> **Answer**
>
> Because 0.1 and 0.2 cannot be written exactly in binary: the sum is
> `0.30000000000000004`. Round to display, and compare the difference with
> `Number.EPSILON`.

**Question 3:** how do you display the price `19.9` as `19.90`?

> **Answer**
>
> ```javascript
> console.log((19.9).toFixed(2));
> ```
>
> ```sortie
> 19.90
> ```
>
> For a price with its currency (`$19.90`), use `Intl.NumberFormat("en-US", ...)`.

## Summary

- JavaScript has only one `number` type for integers and decimal numbers.
- `%` gives the remainder, `**` the power; `Math` contains `round`, `floor`, `ceil`,
  `max`, `min`…
- Decimal numbers are approximate: round with `toFixed` to display them.
- `Number("...")` converts a text; an impossible calculation gives `NaN`, to test with
  `Number.isNaN`.
- Dividing by zero gives `Infinity`; huge integers are written as `BigInt` (`10n`).
- `Intl.NumberFormat("en-US")` formats numbers for a given language.

Next: [conditions and loops](conditions-boucles.md).
