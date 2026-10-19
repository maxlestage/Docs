# First steps

This page shows how to store values in variables with `let` and `const`, what the basic types of JavaScript are, and how to compare them.

> **Goals:**
>
> - you will be able to create a variable with `let` or `const`, and know which one to
>   choose;
> - you will know the basic types: number, text, boolean, `null`, `undefined`;
> - you will be able to display several values and build a text with `${...}`;
> - you will be able to comment your code and name your variables well;
> - you will know why you should compare with `===` rather than `==`.

## Variables with let and const

A [variable](../debuter/glossaire.md#variable) is a box with a label: you store a value
in it, then you find it again by its name. In JavaScript, you create a variable with
`const` or `let`.

```javascript
const firstName = "Alice";
let age = 30;

console.log(firstName);
console.log(age);
```

```sortie
Alice
30
```

The `=` sign does not mean "equal": it means "store this value in this box". This is
called **assigning** a value.

### const: a sealed box

`const` (from *constant*) creates a variable that cannot be reassigned. `let` creates a
variable that you can change later.

```javascript
let score = 0;
score = score + 10;
score += 5;
console.log(score);

const pi = 3.14159;
console.log(pi);
```

```sortie
15
3.14159
```

`score += 5` is a shortcut for `score = score + 5`.

> **Remember:** use `const` by default. Switch to `let` only if the value has to
> change. When reading `const`, the reader knows right away that the value will not
> move anymore.

### What about var?

You will often see `var` in old code or on the Internet. It is the old way to create a
variable, from before 2015. It has surprising behaviors: you can declare the same
variable twice without an error, and it "leaks" out of `{ }` blocks.

```javascript
var city = "Lyon";
var city = "Lille";
console.log(city);
```

```sortie
Lille
```

With `let`, JavaScript would have warned you about the mistake. **Do not use `var`**:
`let` and `const` do the same thing, more safely.

## Basic types

Every value has a [type](../debuter/glossaire.md#type): its nature (a number, a
text…). The type says what you can do with the value. The `typeof` operator gives the
type of a value, as a text.

```javascript
console.log(typeof 42);
console.log(typeof 3.5);
console.log(typeof "hello");
console.log(typeof true);
console.log(typeof undefined);
console.log(typeof 10n);
```

```sortie
number
number
string
boolean
undefined
bigint
```

| Type | Examples | Role |
| --- | --- | --- |
| `number` | `42`, `3.5`, `-7` | all numbers, whole or decimal |
| `string` | `"hello"`, `'hi'` | text |
| `boolean` | `true`, `false` | true or false |
| `undefined` | `undefined` | "no value yet" |
| `null` | `null` | "empty, on purpose" |
| `bigint` | `10n` | giant integers (see [numbers](nombres.md)) |
| `symbol` | `Symbol("id")` | a unique identifier (rare, for later) |

- A **number** is written with a dot, not a comma: `3.5`.
- A [string](../debuter/glossaire.md#string) (a text) is written between
  double quotes `"..."` or single quotes `'...'`.
- A [boolean](../debuter/glossaire.md#boolean) is `true` or `false`.

### undefined and null

Two values mean "nothing", with a nuance.

- `undefined`: the box exists, but **nobody has put anything in it**. It is the value of
  a variable declared without a value.
- `null`: the box is **empty on purpose**. You write it yourself, to say "no value"
  (see [null](../debuter/glossaire.md#null)).

```javascript
let answer;
console.log(answer);

let winner = null;
console.log(winner);
console.log(typeof null);
```

```sortie
undefined
null
object
```

> **Warning:** `typeof null` answers `"object"`. It is a historical mistake in
> JavaScript, kept so as not to break old websites. Just remember it.

## Template literals

To put a value inside a text, write the text between **backticks** `` `...` `` (the key
to the left of **1** on a US keyboard), and the value inside `${...}`. This is a
**template literal**.

```javascript
const firstName = "Alice";
const age = 30;

console.log(`Hello, ${firstName}!`);
console.log(`In 5 years, you will be ${age + 5}.`);
```

```sortie
Hello, Alice!
In 5 years, you will be 35.
```

Inside `${...}`, you can put any calculation. The old way uses `+` to glue the pieces
together: `"Hello, " + firstName + "!"`. The template literal is more readable.

> **Warning:** `${...}` only works between backticks. Between quotes `"..."`, it is
> displayed as it is.

## Displaying several values

`console.log` accepts several values, separated by commas. It displays them on one
line, separated by a space.

```javascript
const item = "coffee";
const price = 2.5;

console.log("Item:", item, "- price:", price);
console.log(item, price, true, null);
```

```sortie
Item: coffee - price: 2.5
coffee 2.5 true null
```

> **Tip:** for debugging, write `console.log("price", price)`: the name in front of the
> value keeps you from mixing up the lines.

## Comments

A [comment](../debuter/glossaire.md#comment) is a note for humans. JavaScript
ignores it.

```javascript
// A comment on one line
const tax = 0.2; // also at the end of a line

/*
  A comment
  on several lines
*/
console.log(100 * (1 + tax));
```

```sortie
120
```

A good comment explains **why**, not **what**: the code already says what it does.

## Naming your variables well

In JavaScript, names are written in **camelCase** (like the humps of a camel): the
first word in lowercase, then a capital letter at the start of each following word.

```javascript
const unitPrice = 4;
const itemCount = 3;
const totalPrice = unitPrice * itemCount;

console.log(totalPrice);
```

```sortie
12
```

A few rules:

- a name contains letters, digits, `_` or `$`, but **does not start with a digit**;
- capital letters matter: `age` and `Age` are two different variables;
- some words are reserved (`let`, `const`, `if`, `function`…);
- choose names that say what the box contains: `totalPrice` rather than `x`.

> **Note:** accented letters are allowed (`const âge = 30`), but the usual practice is to
> avoid them in variable names. Keep them for text between quotes.

## Comparing: == or ===?

Comparing two values gives a boolean. JavaScript has two equality operators:

- `===` (**strict** equality): true if both values have the same type **and** the same
  value;
- `==` (**loose** equality): first converts the values so they have the same type, then
  compares.

```javascript
console.log(5 === 5);
console.log(5 === "5");
console.log(5 == "5");
console.log(0 == "");
console.log(null == undefined);
console.log(null === undefined);
```

```sortie
true
false
true
true
true
false
```

With `==`, the text `"5"` is converted to a number, and `0 == ""` answers `true`: that
is rarely what you want. The conversion rules are long and full of traps.

> **Remember:** always use `===` and its opposite `!==` ("not equal to"). Forget `==`
> and `!=`.

## Common mistakes

### Changing a constant

```javascript norun
const age = 20;
age = 21;
```

```text
file:///home/alice/js/main.mjs:2
age = 21;
    ^

TypeError: Assignment to constant variable.
    at file:///home/alice/js/main.mjs:2:5

Node.js v22.22.0
```

You cannot reassign a `const` variable (*assignment to constant variable*). If the
value has to change, declare it with `let age = 20;`.

### Declaring the same variable twice

```javascript norun
let score = 0;
let score = 10;
```

```text
file:///home/alice/js/main.mjs:2
let score = 10;
    ^

SyntaxError: Identifier 'score' has already been declared

Node.js v22.22.0
```

`let` is used to **create** the variable, only once. To change it afterwards, just
write `score = 10;`, without `let`.

### Using a variable before creating it

```javascript norun
console.log(total);
let total = 5;
```

```text
file:///home/alice/js/main.mjs:1
console.log(total);
            ^

ReferenceError: Cannot access 'total' before initialization
    at file:///home/alice/js/main.mjs:1:13

Node.js v22.22.0
```

JavaScript reads from top to bottom. On line 1, `total` does not have a value yet
(*before initialization*). Declare the variable before using it.

### Getting the capital letter wrong

```javascript norun
const totalPrice = 12;
console.log(totalprice);
```

```text
file:///home/alice/js/main.mjs:2
console.log(totalprice);
            ^

ReferenceError: totalprice is not defined
    at file:///home/alice/js/main.mjs:2:13

Node.js v22.22.0
```

`totalprice` and `totalPrice` are two different names. Copy the name exactly.

## Test yourself

**Question 1:** what does this code display?

```javascript norun
let n = 4;
n = n * 2;
n += 1;
console.log(`n is ${n}`);
```

> **Answer**
>
> `n is 9`. `n` goes from 4 to 8, then to 9.

**Question 2:** what is the difference between `undefined` and `null`?

> **Answer**
>
> `undefined` means "no value yet": JavaScript gives it to an empty variable. `null`
> means "empty on purpose": you write it yourself.

**Question 3:** what do `"3" == 3` and `"3" === 3` answer? Which one should you use?

> **Answer**
>
> ```javascript
> console.log("3" == 3);
> console.log("3" === 3);
> ```
>
> ```sortie
> true
> false
> ```
>
> `==` converts the text into a number before comparing; `===` sees two different
> types. Always use `===`.

## Summary

- `const` creates a variable that you do not reassign, `let` a variable that can
  change; avoid `var`.
- The basic types are `number`, `string`, `boolean`, `undefined`, `null`, plus `bigint`
  and `symbol`; `typeof` gives the type.
- A template literal `` `Hello, ${firstName}` `` inserts a value into a text.
- `console.log(a, b)` displays several values; `//` and `/* */` write comments.
- Names are written in camelCase, and capital letters matter.
- Always compare with `===` and `!==`.

Next: [numbers and calculations](nombres.md).
