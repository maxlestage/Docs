# Numbers and calculations

This page shows how TypeScript does arithmetic: the `number` type, the operators, the remainder `%`, rounding, displaying numbers, turning text into a number and the little surprises of decimal numbers.

> **Goals:**
>
> - you will know that a single type, `number`, is used for integers and decimal numbers;
> - you will know how to calculate with `+ - * / ** %` and update a variable with `+=`;
> - you will know how to round with `Math.round`, `Math.floor`, `Math.ceil` and `Math.trunc`;
> - you will know how to display a number with `toFixed` and `Intl.NumberFormat`;
> - you will know how to turn text into a number and spot `NaN`;
> - you will know why `0.1 + 0.2` does not give exactly `0.3`.

## A single type for all numbers

A program calculates all the time: a price, an age, a score, a distance. Before
calculating, you need to know how TypeScript stores these numbers.

In many languages, there is one type for [integers](../debuter/glossaire.md#entier)
(`7`) and another for decimal numbers, the [floats](../debuter/glossaire.md#flottant)
(`7.5`). Not in TypeScript: both are `number`.

```typescript
const apples: number = 7;
const price: number = 2.5;
const temperature: number = -3;

console.log(apples, price, temperature);
console.log(typeof apples, typeof price);
console.log(Number.isInteger(apples), Number.isInteger(price));
```

```sortie
7 2.5 -3
number number
true false
```

Line by line:

- the three `const` store three numbers: an integer, a decimal number and a negative
  number. All three have the type `number`;
- the first `console.log` prints the three values, separated by a space;
- `typeof` gives the type of a value: here, `number` for both;
- `Number.isInteger` answers `true` for `7` and `false` for `2.5`.

`Number.isInteger` answers the question "is this an integer?".

> **Warning:** in code, the decimal separator is always a **dot**: `2.5`, never
> `2,5`.

## Arithmetic operators

Calculating is the basic job of a computer. Operators are the keys of its calculator:
you combine them to get a total, an average, a price.

An [operator](../debuter/glossaire.md#operateur) is a symbol that calculates. Here are
the ones for numbers:

| Operator | Role | Example | Result |
|---|---|---|---|
| `+` | addition | `7 + 2` | `9` |
| `-` | subtraction | `7 - 2` | `5` |
| `*` | multiplication | `7 * 2` | `14` |
| `/` | division | `7 / 2` | `3.5` |
| `**` | power | `2 ** 3` | `8` |
| `%` | remainder of the division | `7 % 2` | `1` |

```typescript
console.log(7 + 2, 7 - 2, 7 * 2);
console.log(7 / 2);
console.log(2 ** 3);
console.log(1 / 0, 0 / 0);
```

```sortie
9 5 14
3.5
8
Infinity NaN
```

Line by line:

- the first line does three calculations and prints the three results side by side;
- `7 / 2` gives `3.5`: division keeps the part after the decimal point;
- `2 ** 3` means 2 × 2 × 2, which is `8`;
- the last line shows two strange cases, explained just below.

- `/` does a real division: `7 / 2` gives `3.5`, not `3`.
- Dividing by zero does not crash: you get `Infinity`.
- `0 / 0` gives `NaN`: "*Not a Number*". We will come back to it.

### Order of operations

Just like at school, `*` and `/` come before `+` and `-`. Parentheses come before
everything.

```typescript
console.log(2 + 3 * 4);
console.log((2 + 3) * 4);
console.log(10 - 4 - 3);
```

```sortie
14
20
3
```

```text
2 + 3 * 4        (2 + 3) * 4
    -----         -------
2 +  12            5    * 4
   14                 20
```

> **Try it:** replace `10 - 4 - 3` with `10 - (4 - 3)`. The result becomes `9`: the
> parentheses are calculated first.

## The remainder: %

`%` gives what **remains** after a whole-number division. Think of eggs packed in boxes
of 6: with 14 eggs, you fill 2 boxes, and 2 eggs are left over.

```text
14 eggs, boxes of 6:

[oooooo] [oooooo]  oo
  box 1    box 2   remainder = 14 % 6 = 2
```

```typescript
console.log(14 % 6);
console.log(10 % 2, 11 % 2);
console.log(Math.floor(14 / 6));
```

```sortie
2
0 1
2
```

Line by line:

- `14 % 6`: 14 contains 6 twice (that is 12), and `2` remains;
- `10 % 2` gives `0` (10 is even) and `11 % 2` gives `1` (11 is odd);
- `Math.floor(14 / 6)` gives the number of full boxes: `14 / 6` is `2.33…`, and
  `Math.floor` keeps the whole part, `2`. You will see `Math.floor` again below.

Two very common uses:

- **even or odd**: `n % 2` is `0` if `n` is even, `1` otherwise;
- **splitting a duration**: 135 minutes is 2 hours and 15 minutes.

```typescript
const minutes = 135;
const hours = Math.floor(minutes / 60);
const remainder = minutes % 60;
console.log(`${hours} h ${remainder} min`);
```

```sortie
2 h 15 min
```

| Step | Calculation | Value |
|---|---|---|
| 1 | `minutes / 60` | `2.25` |
| 2 | `Math.floor(2.25)` | `2` → `hours` |
| 3 | `135 % 60` | `15` → `remainder` |

> **Try it:** set `minutes = 59`. You get `0 h 59 min`.

> **Remember:** `/` divides, `%` gives the remainder. Together, they split a number into
> pieces (hours and minutes, boxes and leftover eggs).

**Your turn:** write a program that splits 200 seconds into minutes and seconds, and
prints `3 min 20 s`.

> **Solution**
>
> ```typescript
> const seconds = 200;
> const min = Math.floor(seconds / 60);
> const remainder = seconds % 60;
> console.log(`${min} min ${remainder} s`);
> ```
>
> ```sortie
> 3 min 20 s
> ```

## Updating a variable that holds a number

Many numbers change while the program runs: a score that goes up, a stock that goes
down, the balance of a piggy bank. You update the same variable instead of creating a
new one each time.

To add something to a variable, you can write `total = total + 5`. There are
shortcuts:

| Shortcut | Means |
|---|---|
| `total += 5` | `total = total + 5` |
| `total -= 2` | `total = total - 2` |
| `total *= 3` | `total = total * 3` |
| `total++` | `total = total + 1` |
| `total--` | `total = total - 1` |

```typescript
let total = 10;
total += 5;
total -= 2;
total *= 3;
total++;
console.log(total);
```

```sortie
40
```

| Step | Statement | total |
|---|---|---|
| 1 | `let total = 10` | 10 |
| 2 | `total += 5` | 15 |
| 3 | `total -= 2` | 13 |
| 4 | `total *= 3` | 39 |
| 5 | `total++` | 40 |

These shortcuts only work with `let`: a `const` never changes.

## Rounding with Math

A calculation often gives a number full of decimals, like `2.4` buses or `3.333…`
dollars. In real life, you need a round number: that is what rounding is for.

`Math` is a toolbox that is always available. Four tools round numbers:

- `Math.round`: to the nearest;
- `Math.floor`: down (the "floor");
- `Math.ceil`: up (the "ceiling");
- `Math.trunc`: cuts off everything after the decimal point.

```typescript
for (const x of [2.4, 2.5, 2.6, -2.5, -2.6]) {
  console.log(x, Math.round(x), Math.floor(x), Math.ceil(x), Math.trunc(x));
}
```

```sortie
2.4 2 2 3 2
2.5 3 2 3 2
2.6 3 2 3 2
-2.5 -2 -3 -2 -2
-2.6 -3 -3 -2 -2
```

Line by line:

- `for (const x of [...])` takes the numbers of the list one by one, and stores each
  one in turn in `x`. Loops are explained in the
  [next chapter](conditions-boucles.md);
- on each round, `console.log` prints `x`, then its four roundings, in this order:
  `round`, `floor`, `ceil`, `trunc`;
- so each line of the output matches one number of the list.

Look at the negative numbers: `Math.floor(-2.6)` gives `-3`, because `-3` is
**lower** than `-2.6`. `Math.trunc` simply cuts: `-2`.

A concrete use of `Math.ceil`: how many 50-seat buses for 120 students?

```typescript
const students = 120;
console.log(students / 50);
console.log(Math.ceil(students / 50));
```

```sortie
2.4
3
```

You need 3 buses: you cannot order 2.4 buses.

**Your turn:** bottles are packed in cartons of 12. Write a program that prints how
many cartons you need for 30 bottles.

> **Solution**
>
> ```typescript
> const bottles = 30;
> console.log(Math.ceil(bottles / 12));
> ```
>
> ```sortie
> 3
> ```

### Smallest, largest, distance

```typescript
console.log(Math.min(4, 9, 2));
console.log(Math.max(4, 9, 2));
console.log(Math.abs(-7), Math.abs(7));
console.log(Math.sqrt(16));
```

```sortie
2
9
7 7
4
```

`Math.abs` gives the value without its sign: the "distance" to zero. `Math.sqrt` gives
the square root.

> **Try it:** for a grade between 0 and 20, write `Math.min(20, Math.max(0, grade))`.
> With `grade = 23`, you get `20`; with `grade = -1`, you get `0`.

## Displaying a number nicely

A calculation can give `3.3333333333333335`. Nobody wants to read that on a receipt.
These tools choose how a number is displayed, without changing the calculation.

### toFixed: a fixed number of decimals

`toFixed(2)` gives the number with 2 digits after the decimal point.

```typescript
const price = 19.9;
console.log(price.toFixed(2));
console.log((10 / 3).toFixed(2));
console.log((2.5).toFixed(0));
console.log(typeof price.toFixed(2));
```

```sortie
19.90
3.33
3
string
```

Line by line:

- `price` is `19.9`. `price.toFixed(2)` writes it with two decimals: `19.90`;
- `10 / 3` is `3.333…`, and `toFixed(2)` keeps `3.33`;
- `toFixed(0)` keeps no decimals: `2.5` becomes `3`;
- `typeof` shows that the result of `toFixed` is a `string`, a piece of text.

> **Warning:** `toFixed` returns **text** (`string`), not a number. Use it at the very
> end, just to display. Do not calculate with it anymore.

### Intl.NumberFormat: the local way

In the United States, you write `$1,234.50`: a comma between thousands, a dot for
decimals. `Intl.NumberFormat` knows these habits for each country. It is presented in
detail in [the chapter on text](texte.md#formatting-numbers-with-intl).

```typescript
const dollars = new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" });
const number = new Intl.NumberFormat("en-US", { maximumFractionDigits: 2 });

console.log(dollars.format(1234.5));
console.log(number.format(1234567.891));
console.log(number.format(0.5));
```

```sortie
$1,234.50
1,234,567.89
0.5
```

> **Note:** other countries have other habits. With `"fr-FR"` and `"EUR"`, you get
> `1 234,50 €`, and those spaces are not ordinary spaces: they are **non-breaking**
> spaces, which prevent a number from being split at the end of a line. So never compare
> formatted text with text typed on the keyboard.

## From text to number

Text that contains digits is still text: `"42"` is not `42`. When a value comes from a
form, a file or the keyboard, it is **always** text. Three tools turn it into a number.

```typescript
console.log(Number("42"), parseInt("42"), parseFloat("42"));
console.log(Number("3.9"), parseInt("3.9"), parseFloat("3.9"));
console.log(Number("12px"), parseInt("12px"), parseFloat("12px"));
console.log(Number(""), parseInt(""), parseFloat(""));
console.log(Number(" 7 "), Number("3,5"));
```

```sortie
42 42 42
3.9 3 3.9
NaN 12 12
0 NaN NaN
7 NaN
```

`Number(" 7 ")` ignores the spaces around the number. `Number("3,5")` fails: in code,
the decimal separator is a dot.

| Tool | How it reads | `"3.9"` | `"12px"` | `""` |
|---|---|---|---|---|
| `Number` | the whole text, or nothing | `3.9` | `NaN` | `0` |
| `parseInt` | the beginning, up to the first "non-digit", without decimals | `3` | `12` | `NaN` |
| `parseFloat` | the beginning, with decimals | `3.9` | `12` | `NaN` |

> **Remember:** use `Number` by default: it rejects half-numeric texts. Watch out for its
> trap: an empty text gives `0`, not `NaN`.

**Your turn:** two fields of a form contain the texts `"12"` and `"30"`. Write a program
that turns them into numbers and prints their sum, `42`.

> **Solution**
>
> ```typescript
> const field1 = "12";
> const field2 = "30";
> console.log(Number(field1) + Number(field2));
> ```
>
> ```sortie
> 42
> ```

### NaN: "not a number"

When the conversion fails, you get `NaN`. Oddly, `NaN` has the type `number`. And it is
equal to nothing, not even to itself. To spot it, use `Number.isNaN`:

```typescript
for (const input of ["12", "twelve", "4.5"]) {
  const n = Number(input);
  if (Number.isNaN(n)) {
    console.log(`"${input}" is not a number`);
  } else {
    console.log(`${n} x 2 = ${n * 2}`);
  }
}
```

```sortie
12 x 2 = 24
"twelve" is not a number
4.5 x 2 = 9
```

`NaN` is contagious: `NaN + 1` gives `NaN`. A single `NaN` at the start of a
calculation, and the whole result becomes `NaN`. So check your conversions right away.

## 0.1 + 0.2: the decimal surprise

As soon as you calculate a price or an average, you use decimal numbers. They hide a
small trap: better to know it before it surprises you.

```typescript
console.log(0.1 + 0.2);
console.log(0.1 + 0.2 === 0.3);
```

```sortie
0.30000000000000004
false
```

This is not a TypeScript bug. The computer stores numbers in
[binary](../debuter/glossaire.md#binaire) (with 0s and 1s). But `0.1` cannot be written
exactly in binary, just as `1/3` cannot be written exactly in decimal (`0.3333…`). A
tiny error remains, and it sometimes shows up. Python, Java and JavaScript do exactly
the same.

Two solutions:

1. **round to display**, with `toFixed` or `Intl`;
2. **count in cents**, with integers, for money.

```typescript
const total = 0.1 + 0.2;
console.log(total.toFixed(2));
console.log(Math.round(total * 100) / 100);

const priceInCents = 10 + 20;
console.log(priceInCents / 100);
```

```sortie
0.30
0.3
0.3
```

Line by line:

- `total` is `0.30000000000000004`, with the small error;
- `total.toFixed(2)` displays it with two decimals: `0.30`;
- `Math.round(total * 100) / 100` rounds to the cent: `30.000…04` becomes `30`, then
  divided by 100, `0.3`;
- `priceInCents` adds two integers, `10 + 20`, without any error. You divide by 100
  only to display.

> **Remember:** never compare two decimal numbers with `===`. For money, calculate in
> cents (integers), and divide by 100 only to display.

## Very large integers: bigint

> **Note:** this part can wait. Read it once the basics are solid.

A `number` stays exact up to `Number.MAX_SAFE_INTEGER`, about 9 quadrillion. Beyond
that, it gets things wrong.

```typescript
console.log(Number.MAX_SAFE_INTEGER);
console.log(9007199254740992 + 1);
```

```sortie
9007199254740991
9007199254740992
```

`+ 1` changed nothing! For larger integers, there is another type: `bigint`. You write
it with an `n` at the end.

```typescript
const big: bigint = 9007199254740992n;
console.log(big + 1n);
console.log(2n ** 100n);
console.log(typeof big);
```

```sortie
9007199254740993n
1267650600228229401496703205376n
bigint
```

A `bigint` has no decimal part: `7n / 2n` gives `3n`. You will rarely need it as a
beginner.

## Number literal types

> **Note:** this part can wait. Read it once the basics are solid.

A type can also be **one exact number**: `type Face = 1 | 2 | 3 | 4 | 5 | 6;` only
accepts these six values, and `const f: Face = 7;` is rejected. You will see this idea
in detail in [literal types](litteraux.md).

## Common mistakes

### Calculating with the result of toFixed

```typescript norun
const price = 19.9;
const rounded: number = price.toFixed(2);
```

```text
main.ts:2:7 - error TS2322: Type 'string' is not assignable to type 'number'.

2 const rounded: number = price.toFixed(2);
        ~~~~~~~


Found 1 error in main.ts:2
```

`toFixed` returns text. For a rounded number, use `Math.round(price * 100) / 100`. Keep
`toFixed` for display.

### Testing NaN with ===

```typescript norun
const n = Number("twelve");
if (n === NaN) {
  console.log("not a number");
}
```

```text
main.ts:2:5 - error TS2845: This condition will always return 'false'.

2 if (n === NaN) {
      ~~~~~~~~~

  main.ts:2:5
    2 if (n === NaN) {
          ~
    Did you mean 'Number.isNaN(n)'?


Found 1 error in main.ts:2
```

"This condition will always return 'false'." `NaN` is equal to nothing. TypeScript
suggests the fix: `if (Number.isNaN(n))`.

### Giving two arguments to Math.round

```typescript norun
console.log(Math.round(3.14159, 2));
```

```text
main.ts:1:33 - error TS2554: Expected 1 arguments, but got 2.

1 console.log(Math.round(3.14159, 2));
                                  ~


Found 1 error in main.ts:1
```

`Math.round` always rounds to an integer. For 2 decimals:
`Math.round(3.14159 * 100) / 100` gives `3.14`.

### Mixing bigint and number

```typescript norun
const big = 10n;
console.log(big + 1);
```

```text
main.ts:2:13 - error TS2365: Operator '+' cannot be applied to types '10n' and '1'.

2 console.log(big + 1);
              ~~~~~~~


Found 1 error in main.ts:2
```

A `bigint` does not mix with a `number`. Write `big + 1n`, or convert with `BigInt(1)`.

## Test yourself

**Question 1:** what are `17 % 5` and `Math.floor(17 / 5)`?

> **Answer**
>
> `2` and `3`. 17 contains 5 three times (that is 15), and 2 remains.

**Question 2:** what does `console.log(Number("8 kg"), parseInt("8 kg"));` print?

> **Answer**
>
> `NaN 8`. `Number` reads the whole text: `" kg"` makes it fail. `parseInt` reads the
> beginning, `8`, and stops at the first character that is not a digit.

**Question 3:** why count a price in cents rather than in dollars?

> **Answer**
>
> Cents are integers, and integers are exact. Decimal numbers have tiny errors:
> `0.1 + 0.2` gives `0.30000000000000004`.

## Summary

- `number` is used for all numbers, integers or decimals; the decimal separator is `.`.
- `/` really divides, `%` gives the remainder, `**` the power; `+=` and `++` update a
  `let` variable.
- `Math.round`, `Math.floor`, `Math.ceil` and `Math.trunc` each round in their own way.
- `toFixed` and `Intl.NumberFormat` are for display: they return text.
- `Number(text)` converts; a failure gives `NaN`, which you test with `Number.isNaN`.
- `0.1 + 0.2` is not exactly `0.3`: round to display, count money in cents.

Next: [conditions and loops](conditions-boucles.md).
