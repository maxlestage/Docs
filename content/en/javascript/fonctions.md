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

A function that always does exactly the same thing is not very useful. Parameters make
it flexible: it is like a recipe "for N people", where you choose N each time.

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

Line by line:

- `function greet(firstName)` stores the function. Nothing is displayed yet;
- `greet("Alice")` calls the function: `firstName` receives `"Alice"`, then
  `console.log` displays the sentence with that name;
- `greet("Bob")` does it again, this time with `firstName` set to `"Bob"`.

Here, `firstName` is the parameter; `"Alice"` and `"Bob"` are the arguments. Several
parameters are separated by commas, and the arguments are given in the same order.

**Your turn:** write a function `introduce(firstName, age)` that displays "My name is …
and I am … years old.", then call it with `"Lea"` and `12`.

> **Solution**
>
> ```javascript
> function introduce(firstName, age) {
>   console.log(`My name is ${firstName} and I am ${age} years old.`);
> }
>
> introduce("Lea", 12);
> ```
>
> ```sortie
> My name is Lea and I am 12 years old.
> ```

## Returning a value with return

Often, you do not just want to display a result: you want to keep it and use it later.
A calculator gives you a number, which you can write down, reuse or add to another
one. `return` does the same thing.

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

Line by line:

- `function area(width, height)` stores the function, without computing anything;
- `area(3, 4)` runs the function: `width` is 3, `height` is 4, and `return` gives back
  12. `console.log` displays that 12;
- `area(2, 5)` returns 10, then `+ 1` gives 11, stored in `total`;
- the last `console.log` displays `total`, so 11.

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

**Your turn:** write a function `perimeter(side)` that returns the perimeter of a square
(four times the side). Display `perimeter(5)`, then `perimeter(5) + perimeter(2)`.

> **Solution**
>
> ```javascript
> function perimeter(side) {
>   return side * 4;
> }
>
> console.log(perimeter(5));
> console.log(perimeter(5) + perimeter(2));
> ```
>
> ```sortie
> 20
> 28
> ```

## Default parameters

At a restaurant, the dish comes with fries unless you ask for something else. A default
value works the same way: it saves you from repeating the usual value at every call.

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

Line by line:

- `greeting = "Hello"` provides a fallback value for `greeting`;
- `greet("Alice")` gives only one argument: `firstName` is `"Alice"` and `greeting`
  takes its default value, `"Hello"`;
- `greet("Bob", "Hi")` gives both: `"Hi"` replaces the default value.

## Rest parameters

Sometimes you do not know in advance how many values the function will receive: you
want to add two numbers, or ten. Rest parameters accept any number of arguments.

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

Line by line:

- `...numbers` stores all the arguments in an array: with `sum(1, 2)`, `numbers` is
  `[1, 2]`;
- `total` starts at 0;
- the `for` loop takes each number of the array, one by one, and adds it to `total`;
- `return total` returns the sum. With `sum()`, the array is empty: the loop does not
  run, and the function returns 0.

It is a bit like a bag: you put in as many arguments as you want. Arrays are explained
in [arrays](collections.md).

## Arrow functions

Many functions only do a small calculation. Writing `function`, curly braces and
`return` for a single line is long. The arrow says the same thing in fewer words.

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

**Your turn:** write an arrow function `triple` that returns three times a number, and
an arrow function `welcome` that returns "Welcome, …!" with a name. Display `triple(7)`
and `welcome("Sam")`.

> **Solution**
>
> ```javascript
> const triple = (x) => x * 3;
> const welcome = (firstName) => `Welcome, ${firstName}!`;
>
> console.log(triple(7));
> console.log(welcome("Sam"));
> ```
>
> ```sortie
> 21
> Welcome, Sam!
> ```

## Functions are values

In JavaScript, a function is a value, like a number or a text. You can store it in a
variable, put it in an array, or give it to another function.

What is the point? You can give a function not only data, but also an action to perform
on that data. It is like handing someone a basket of apples with the instruction "peel
them".

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

Line by line:

- `function double(x)` stores a function named `double`;
- `const operation = double;` stores the function itself in `operation`, without
  calling it (no parentheses). `operation` and `double` name the same recipe;
- `operation(21)` therefore calls `double` with 21, which returns 42;
- `typeof double` gives the type of the value: here, `function`.

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

Line by line:

- `applyTwice` receives two things: a function `f` and a value;
- `f(value)` applies `f` a first time, then the `f( )` around it applies it to the
  result;
- first call: `f` adds 10. 1 becomes 11, then 21;
- second call: `f` squares. 3 becomes 9, then 81.

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

**Your turn:** start from the array `const grades = [8, 12, 15];` and use `map` to
create a new array where each grade gets 2 extra points. Display it.

> **Solution**
>
> ```javascript
> const grades = [8, 12, 15];
> const bonusGrades = grades.map((g) => g + 2);
> console.log(bonusGrades);
> ```
>
> ```sortie
> [ 10, 14, 17 ]
> ```

## Hoisting

> **Note:** this part can wait. Read it once the basics feel solid.

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

> **Note:** this part can wait. Read it once the basics feel solid.

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
