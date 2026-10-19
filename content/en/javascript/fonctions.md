# Functions

This page explains how to write functions in JavaScript, with `function` or with an arrow `=>`, give them parameters, and get their result back.

> **Goals:**
>
> - you will be able to declare a function with `function` and call it;
> - you will be able to return a value with `return`;
> - you will be able to use default parameters and "rest" parameters;
> - you will be able to write an arrow function `=>`;
> - you will be able to pass a function to another function (a *callback*).

## Declaring a function

A [function](../debuter/glossaire.md#function) is a named piece of code that you can
reuse. It is like a recipe: you write it once, then you follow it as many times as you
want.

```javascript
function greet() {
  console.log("Hello!");
}

greet();
greet();
```

```sortie
Hello!
Hello!
```

- `function` announces a function, followed by its name (`greet`) and parentheses.
- The code of the function, its **body**, is between curly braces `{ }`.
- Writing the function does not run anything: it saves the recipe. Writing `greet()`
  **calls** the function, that is, runs it. The parentheses are required.

## Parameters

[Parameters](../debuter/glossaire.md#parameter) are the values the function receives.
They are empty slots, filled in at the time of the call. The values given in the call
are called [arguments](../debuter/glossaire.md#argument).

```javascript
function greet(firstName) {
  console.log(`Hello, ${firstName}!`);
}

greet("Alice");
greet("Bob");
```

```sortie
Hello, Alice!
Hello, Bob!
```

Here, `firstName` is the parameter; `"Alice"` and `"Bob"` are the arguments. Several
parameters are separated by commas, and the arguments are given in the same order.

## Returning a value with return

`return` stops the function and **returns** a value to the code that called it. The
returned value replaces the call.

```javascript
function area(width, height) {
  return width * height;
}

console.log(area(3, 4));
const total = area(2, 5) + 1;
console.log(total);
```

```sortie
12
11
```

`area(2, 5) + 1` becomes `10 + 1`. A function without `return` returns `undefined`.

`return` can also be used to leave early:

```javascript
function divide(a, b) {
  if (b === 0) {
    return "division impossible";
  }
  return a / b;
}

console.log(divide(10, 4));
console.log(divide(10, 0));
```

```sortie
2.5
division impossible
```

### Display or return?

`console.log` **displays** a value on the screen. `return` **gives** it to the calling
code, which can use it: store it, calculate with it again, display it.

```javascript
function printDouble(x) {
  console.log(x * 2);
}

function double(x) {
  return x * 2;
}

const a = printDouble(5);
const b = double(5);
console.log(a, b);
```

```sortie
10
undefined 10
```

> **Remember:** prefer functions that return a value. You can then choose what to do
> with it.

## Default parameters

If an argument is missing, its parameter is `undefined`. A **default value**, written
with `=`, replaces it.

```javascript
function greet(firstName, greeting = "Hello") {
  return `${greeting}, ${firstName}!`;
}

console.log(greet("Alice"));
console.log(greet("Bob", "Hi"));
```

```sortie
Hello, Alice!
Hi, Bob!
```

## Rest parameters

With three dots `...` in front of the last parameter, the function receives **all the
remaining arguments**, stored in an array.

```javascript
function sum(...numbers) {
  let total = 0;
  for (const n of numbers) {
    total += n;
  }
  return total;
}

console.log(sum(1, 2));
console.log(sum(1, 2, 3, 4));
console.log(sum());
```

```sortie
3
10
0
```

It is a bit like a bag: you put in as many arguments as you want. Arrays are explained
in [arrays](collections.md).

## Arrow functions

An **arrow function** is a shorter way to write a function. You store it in a
constant.

```javascript
const square = (x) => x * x;
const add = (a, b) => a + b;
const sayHello = () => "Hello!";

console.log(square(4));
console.log(add(2, 3));
console.log(sayHello());
```

```sortie
16
5
Hello!
```

- The parameters are before the arrow `=>`, the result after it.
- On one line, the result is **returned automatically**, without `return`.
- With no parameter, you write empty parentheses `()`.

When the body takes several lines, you add curly braces, and `return` becomes required
again:

```javascript
const priceWithTax = (price, rate = 0.2) => {
  const total = price * (1 + rate);
  return Math.round(total * 100) / 100;
};

console.log(priceWithTax(50));
console.log(priceWithTax(50, 0.055));
```

```sortie
60
52.75
```

> **Tip:** use `function` for the main functions of your program, and arrows for small
> functions passed to other ones (see below). Both are correct.

## Functions are values

In JavaScript, a function is a value, like a number or a text. You can store it in a
variable, put it in an array, or give it to another function.

```javascript
function double(x) {
  return x * 2;
}

const operation = double;
console.log(operation(21));
console.log(typeof double);
```

```sortie
42
function
```

Mind the difference: `double` is the function itself (the recipe); `double(21)` is its
result (the dish).

### Callbacks

A function given as an argument to another one is called a **callback**: the other
function will "call it back" when it needs it.

```javascript
function applyTwice(f, value) {
  return f(f(value));
}

console.log(applyTwice((x) => x + 10, 1));
console.log(applyTwice((x) => x * x, 3));
```

```sortie
21
81
```

Arrays use callbacks a lot. `map` applies a function to each element:

```javascript
const prices = [10, 20, 30];
const pricesWithTax = prices.map((p) => p * 1.2);
console.log(pricesWithTax);
```

```sortie
[ 12, 24, 36 ]
```

You will see `map`, `filter` and their friends again in [arrays](collections.md).

## Hoisting

A function declared with `function` can be called **before** the place where it is
written. JavaScript "hoists" it to the top of the file before starting.

```javascript
console.log(triple(5));

function triple(x) {
  return x * 3;
}
```

```sortie
15
```

This is **not** the case for an arrow function stored in a `const`: the constant only
exists from its line onwards. See the common mistakes below.

## A function that calls itself

A function can call itself: this is
[recursion](../debuter/glossaire.md#recursion). It needs a **stopping case**, otherwise
it never stops.

```javascript
function countdown(n) {
  if (n === 0) {
    console.log("Go!");
    return;
  }
  console.log(n);
  countdown(n - 1);
}

countdown(3);
```

```sortie
3
2
1
Go!
```

A classic example is the factorial: 4! = 4 × 3 × 2 × 1.

```javascript
function factorial(n) {
  if (n <= 1) {
    return 1;
  }
  return n * factorial(n - 1);
}

console.log(factorial(4));
```

```sortie
24
```

## Common mistakes

### Forgetting return in an arrow with curly braces

```javascript norun
const double = (x) => {
  x * 2;
};
console.log(double(5));
```

```text
undefined
```

With curly braces, the result is no longer returned on its own: the function returns
`undefined`. Add `return x * 2;`, or remove the curly braces: `(x) => x * 2`.

### Calling an arrow before defining it

```javascript norun
console.log(square(4));

const square = (x) => x * x;
```

```text
file:///home/alice/js/main.mjs:1
console.log(square(4));
            ^

ReferenceError: Cannot access 'square' before initialization
    at file:///home/alice/js/main.mjs:1:13

Node.js v22.22.0
```

A `const` only exists from its line onwards. Put the definition before the call, or use
a declaration `function square(x) { ... }`, which is hoisted.

### Forgetting the parentheses of the call

```javascript norun
function greet() {
  return "Hello";
}
console.log(greet);
```

```text
[Function: greet]
```

Without parentheses, you are talking about the function itself, without running it.
Write `greet()`.

### Forgetting the stopping case

```javascript norun
function count(n) {
  return count(n - 1);
}
count(3);
```

```text
file:///home/alice/js/main.mjs:2
  return count(n - 1);
  ^

RangeError: Maximum call stack size exceeded
    at count (file:///home/alice/js/main.mjs:2:3)
    at count (file:///home/alice/js/main.mjs:2:10)
    at count (file:///home/alice/js/main.mjs:2:10)
```

The function calls itself endlessly, until it fills the memory reserved for calls
(*maximum call stack size exceeded*). Add a stopping case: `if (n === 0) return 0;`.

## Test yourself

**Question 1:** what does this code display?

```javascript norun
function f(x) {
  return x + 1;
  return x * 2;
}
console.log(f(5));
```

> **Answer**
>
> `6`. The first `return` stops the function: the next line is never run.

**Question 2:** write an arrow function `average(a, b)` that returns the average of two
numbers.

> **Answer**
>
> ```javascript
> const average = (a, b) => (a + b) / 2;
> console.log(average(3, 4));
> ```
>
> ```sortie
> 3.5
> ```
>
> The parentheses around `a + b` do the addition before the division.

**Question 3:** what is `apply((n) => n * 10, 4)` if
`function apply(f, x) { return f(x); }`?

> **Answer**
>
> `40`. `apply` calls the callback `f` with `4`, which returns `4 * 10`.

## Summary

- `function name(parameters) { ... }` declares a function; `name(arguments)` calls it.
- `return` returns a value and stops the function; without `return`, you get
  `undefined`.
- `param = value` gives a default value; `...rest` gathers the remaining arguments into
  an array.
- `(x) => x * 2` is an arrow function: without curly braces, it returns its result on
  its own.
- A function is a value: you can pass it as an argument (a callback).
- A `function` declaration is hoisted; an arrow in a `const` is not.

Next: [working with text](texte.md).
