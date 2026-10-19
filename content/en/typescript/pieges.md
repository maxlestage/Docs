# Beginner pitfalls in TypeScript

This page gathers twelve pitfalls that surprise almost every TypeScript beginner: for each one, what you expect, what really happens, why, and how to avoid it.

> **Goals:**
>
> - you will know why `any` and `as` silence TypeScript, and when that is dangerous;
> - you will know that types disappear at run time;
> - you will know to be wary of empty array slots, of text that looks like a number,
>   and of `||` with `0`;
> - you will know how to tell `object` and `{}` apart;
> - you will understand the surprises of structural typing and of `enum`.

## Before you start: TypeScript checks, then steps aside

Keep a picture in mind. TypeScript is an **inspector** at the entrance of a factory. It
inspects your code **before** it runs. Then `tsc` removes all the types, and plain
JavaScript is what runs.

```text
 your .ts code ──► [ tsc checks the types ] ──► .js code without types ──► node runs it
                       the inspector                  the inspector is gone
```

Many pitfalls come from this: the inspector only sees what you show it, and it is no
longer there when the program runs.

## The pitfalls

### 1. any switches off all checking

**You think:** "TypeScript will warn me if I make a mistake."

```typescript norun
let value: any = 42;
console.log(value.toUpperCase());
```

`tsc` says nothing. But the program crashes when it runs (excerpt of the message):

```text
TypeError: value.toUpperCase is not a function
```

**Why:** [`any`](../debuter/glossaire.md#any) means "check nothing". The inspector
closes its eyes. Here, `value` is a number, and a number has no `toUpperCase`.

**The fix:** use `unknown`. It forces you to check before using the value.

```typescript
const value: unknown = 42;

if (typeof value === "string") {
  console.log(value.toUpperCase());
} else {
  console.log("This is not text:", value);
}
```

```sortie
This is not text: 42
```

> **Remember:** every `any` is a hole in the net. Prefer `unknown`.

### 2. as lies to TypeScript

**You think:** "`as User` turns my object into a user."

```typescript norun
interface User {
  name: string;
}
const u = {} as User;
console.log(u.name.toUpperCase());
```

`tsc` accepts it. When it runs:

```text
TypeError: Cannot read properties of undefined (reading 'toUpperCase')
```

**Why:** `as` doesn't transform anything. It's a **promise** made to the inspector:
"trust me, this is a `User`". The inspector believes you. But the object is empty:
`u.name` is `undefined`.

**The fix:** build a real, complete object. TypeScript will then check that nothing is
missing.

```typescript
interface User {
  name: string;
}
const u: User = { name: "Alice" };
console.log(u.name.toUpperCase());
```

```sortie
ALICE
```

> **Try it:** remove `name: "Alice"`. This time, `tsc` refuses: the property `name` is
> missing. With `: User`, the inspector does its job. With `as`, it keeps quiet.

### 3. Types disappear at run time (JSON.parse)

**You think:** "I wrote `price: number`, so `price` is a number."

```typescript
interface Product {
  name: string;
  price: number;
}

const text = '{"name": "Coffee", "price": "2.5"}';
const product: Product = JSON.parse(text);

console.log(product.price + 1);
console.log(typeof product.price);
```

```sortie
2.51
string
```

**Why:** `JSON.parse` reads a text while the program runs. TypeScript can't know what
it contains: it returns `any`. So your annotation `: Product` is a promise, just like
`as`. Here, the [JSON](../debuter/glossaire.md#json) contains `"2.5"` in quotes: a
text. `+ 1` sticks `"1"` on the end instead of adding.

**The fix:** check data that comes from outside, then convert it.

```typescript
const text = '{"name": "Coffee", "price": "2.5"}';
const raw: unknown = JSON.parse(text);

if (typeof raw === "object" && raw !== null && "price" in raw) {
  const price = Number(raw.price);
  console.log(price + 1);
}
```

```sortie
3.5
```

> **Remember:** types don't exist while the program runs. Everything that comes from a
> file, a server or the keyboard must be **checked**.

### 4. What comes from the keyboard is text

**You think:** "The user typed 5, I add 3, I get 8."

```typescript
const input = "5";
console.log(input + 3);
```

```sortie
53
```

**Why:** an input (form, keyboard, command line) is **always** text. And
`text + number` is allowed in TypeScript: it sticks the two together. `"5" + 3` gives
`"53"`.

**The fix:** convert first with `Number`, see [numbers and calculations](nombres.md).

```typescript
const input = "5";
const n = Number(input);
console.log(n + 3);
```

```sortie
8
```

### 5. An array slot can be empty

**You think:** "`fruits` is a `string[]`, so `fruits[2]` is a `string`."

```typescript norun
const fruits = ["apple", "pear"];
console.log(fruits[2].toUpperCase());
```

`tsc --strict` accepts it. When it runs:

```text
TypeError: Cannot read properties of undefined (reading 'toUpperCase')
```

**Why:** the array has two slots, numbered 0 and 1. Slot 2 doesn't exist: it is
`undefined`. But for TypeScript, a slot of a `string[]` is a `string`. It doesn't
count the slots.

```text
fruits:  [ "apple" | "pear" ]
index:       0         1        2 → nothing (undefined)
```

**The fix:** test the slot before using it, or use `?.`.

```typescript
const fruits = ["apple", "pear"];
const third = fruits[2];

if (third !== undefined) {
  console.log(third.toUpperCase());
} else {
  console.log("No third fruit.");
}
console.log(fruits[0]?.toUpperCase());
```

```sortie
No third fruit.
APPLE
```

> **Note:** an option of the configuration file, `noUncheckedIndexedAccess`, makes
> TypeScript more careful: with it, `fruits[2]` has the type `string | undefined`, and
> forgetting the test becomes an error (see [Common mistakes](#common-mistakes)). It is
> not included in `--strict`. The [configuration](configuration.md) chapter explains how
> to turn it on.

### 6. || also replaces 0

**You think:** "`stock || "unknown"` prints "unknown" only when no stock is given."

```typescript
interface Item {
  name: string;
  stock?: number;
}

const items: Item[] = [
  { name: "pen", stock: 12 },
  { name: "eraser", stock: 0 },
  { name: "ruler" },
];

for (const item of items) {
  console.log(item.name, item.stock || "unknown");
}
```

```sortie
pen 12
eraser unknown
ruler unknown
```

**Why:** `||` replaces every "falsy" value: `undefined`, but also `0`, `""` and
`false`. Yet a stock of 0 is real information.

**The fix:** `??` replaces only `null` and `undefined`.

```typescript
const stocks: (number | undefined)[] = [12, 0, undefined];

for (const s of stocks) {
  console.log(s ?? "unknown");
}
```

```sortie
12
0
unknown
```

### 7. const doesn't stop you from changing an array

**You think:** "My list is `const`, it can't change anymore."

```typescript
const shopping = ["bread"];
shopping.push("milk");
shopping[0] = "butter";
console.log(shopping);
```

```sortie
[ 'butter', 'milk' ]
```

**Why:** `const` seals **the label**, not the contents. You can't put another array in
`shopping`. But the array itself can still be changed, like a box glued to the wall
whose contents you can swap.

**The fix:** if the contents must not change, say so with `readonly`.

```typescript
const shopping: readonly string[] = ["bread", "milk"];
console.log(shopping.length);
```

```sortie
2
```

With `readonly`, `shopping.push(...)` becomes a type error.

### 8. sort() sorts numbers like words

**You think:** "`[10, 9, 1].sort()` gives `[1, 9, 10]`."

```typescript
const scores = [10, 9, 1, 25];
scores.sort();
console.log(scores);
```

```sortie
[ 1, 10, 25, 9 ]
```

**Why:** without instructions, `sort` turns each value into text, then orders them as
in a dictionary. `"10"` comes before `"9"`, because `"1"` comes before `"9"`.

**The fix:** give a comparison function.

```typescript
const scores = [10, 9, 1, 25];
scores.sort((a, b) => a - b);
console.log(scores);
```

```sortie
[ 1, 9, 10, 25 ]
```

`a - b` is negative if `a` is smaller: `a` goes first.

### 9. {} means "almost anything"

**You think:** "`{}` is an empty object, so it only allows objects."

```typescript
const a: {} = 42;
const b: {} = "hello";
const c: {} = { x: 1 };
console.log(a, b, c);
```

```sortie
42 hello { x: 1 }
```

**Why:** the type `{}` means "a value that is neither `null` nor `undefined`". A number
or a text fits. The type `object`, on the other hand, refuses simple values:

```typescript norun
const x: object = 42;
```

```text
main.ts:1:7 - error TS2322: Type 'number' is not assignable to type 'object'.

1 const x: object = 42;
        ~


Found 1 error in main.ts:1
```

**The fix:** use neither `{}` nor `object` to describe your data. Write their real
shape with an `interface` (see [objects](objets.md)).

### 10. A typo that slips through a variable

**You think:** "TypeScript always spots unknown properties."

Written directly, the object is checked properly:

```typescript norun
interface Contact {
  name: string;
  email?: string;
}
const c: Contact = { name: "Alice", emial: "alice@example.com" };
```

```text
main.ts:5:37 - error TS2561: Object literal may only specify known properties, but 'emial' does not exist in type 'Contact'. Did you mean to write 'email'?

5 const c: Contact = { name: "Alice", emial: "alice@example.com" };
                                      ~~~~~


Found 1 error in main.ts:5
```

But if the object goes through a variable first, the typo hides:

```typescript
interface Contact {
  name: string;
  email?: string;
}

const data = { name: "Alice", emial: "alice@example.com" };
const c: Contact = data;
console.log(c.email ?? "no address");
```

```sortie
no address
```

**Why:** the check for extra properties only applies to objects written **on the
spot**. An object stored in a variable may have more properties. And since `email` is
optional, its absence is no problem.

**The fix:** annotate the object as soon as you create it: `const data: Contact = { ... }`.

### 11. Two types with the same shape are interchangeable

**You think:** "`Euros` and `Dollars` are two different types."

```typescript
type Euros = number;
type Dollars = number;

function payInEuros(amount: Euros): string {
  return `Paid: ${amount} €`;
}

const usPrice: Dollars = 100;
console.log(payInEuros(usPrice));
```

```sortie
Paid: 100 €
```

**Why:** `type Euros = number` doesn't create a new type. It gives **another name** to
`number`. And TypeScript compares shapes, not names: this is structural
[typing](../debuter/glossaire.md#typing). Two labels, one and the same box.

**The fix:** give them different shapes, for example an object with the currency.

```typescript
interface Amount {
  value: number;
  currency: "EUR" | "USD";
}

function payInEuros(m: Amount): string {
  if (m.currency !== "EUR") {
    return "Refused: euros are required.";
  }
  return `Paid: ${m.value} €`;
}

console.log(payInEuros({ value: 100, currency: "USD" }));
```

```sortie
Refused: euros are required.
```

### 12. Numeric enums hold surprises

An [enumeration](../debuter/glossaire.md#enum) (`enum`) gives a name to each
value of a small list.

**You think:** "`Color` contains two colors."

```typescript
enum Color {
  Red,
  Green,
}

console.log(Color.Red);
console.log(Object.keys(Color));
```

```sortie
0
[ '0', '1', 'Red', 'Green' ]
```

**Why:** a numeric `enum` numbers its values (`Red` is `0`). And it is a real
JavaScript object, filled **in both directions**: `Color.Red` is `0`, and `Color[0]` is
`"Red"`. If you go through it, you find four keys.

**The fix:** as a beginner, prefer a union of strings. It is simple, readable when
printed, and creates no hidden object.

```typescript
type Color = "red" | "green";
const colors: Color[] = ["red", "green"];

for (const c of colors) {
  console.log(c);
}
```

```sortie
red
green
```

You will see these unions in [the chapter on unions](unions.md).

## Common mistakes

### An array slot may be empty, with noUncheckedIndexedAccess

If your project turns on `noUncheckedIndexedAccess` (pitfall 5), this code is refused:

```typescript norun
const fruits = ["apple", "pear"];
console.log(fruits[2].toUpperCase());
```

```text
main.ts:2:13 - error TS2532: Object is possibly 'undefined'.

2 console.log(fruits[2].toUpperCase());
              ~~~~~~~~~


Found 1 error in main.ts:2
```

"The object may be `undefined`." This is good news: the bug is found before the
program runs. Test the slot (`if (f !== undefined)`) or write
`fruits[2]?.toUpperCase()`.

### Comparing a number and a text

```typescript norun
const age = 18;
const input = "18";
if (age === input) {
  console.log("same age");
}
```

```text
main.ts:3:5 - error TS2367: This comparison appears to be unintentional because the types 'number' and 'string' have no overlap.

3 if (age === input) {
      ~~~~~~~~~~~~~


Found 1 error in main.ts:3
```

"This comparison seems unintentional: a `number` and a `string` have nothing in
common." It would always be false. Convert: `age === Number(input)`.

### Using as between two unrelated types

```typescript norun
const n = "42" as number;
```

```text
main.ts:1:11 - error TS2352: Conversion of type 'string' to type 'number' may be a mistake because neither type sufficiently overlaps with the other. If this was intentional, convert the expression to 'unknown' first.

1 const n = "42" as number;
            ~~~~~~~~~~~~~~


Found 1 error in main.ts:1
```

Even `as` has limits: a text doesn't become a number by changing its label. The
message suggests going through `unknown`… don't do it! Really convert:
`const n = Number("42");`.

## Test yourself

**Question 1:** why is `const p: Product = JSON.parse(text)` risky?

> **Answer**
>
> `JSON.parse` returns `any`: TypeScript doesn't check the contents. If the text doesn't
> match `Product`, the error will only show up at run time. You must check the data
> before trusting it.

**Question 2:** what does this code print?

```typescript norun
const quantity = 0;
console.log(quantity || 10, quantity ?? 10);
```

> **Answer**
>
> `10 0`. `||` replaces the `0`, because `0` is a "falsy" value. `??` only replaces
> `null` and `undefined`: it keeps the `0`.

**Question 3:** what must you write to sort `[3, 20, 100]` in numeric order?

> **Answer**
>
> `list.sort((a, b) => a - b)`. Without a function, `sort` compares texts and gives
> `[100, 20, 3]`.

## Summary

- `any` and `as` silence the inspector: prefer `unknown` and real, complete objects.
- Types disappear at run time: check data that comes from `JSON.parse`, from a file or
  from the keyboard (which always gives text).
- An array slot can be `undefined`; `noUncheckedIndexedAccess` reminds you of it.
- `??` keeps the `0`, `||` doesn't; `const` doesn't stop you from changing an array;
  `sort()` without a function sorts like words.
- `{}` accepts almost anything, `object` refuses simple values: describe the real shape
  instead.
- Typing is structural: two types with the same shape are interchangeable; numeric
  `enum`s have hidden keys.

Next: [exercises to get started](exercices-faciles.md).
