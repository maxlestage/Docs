# Functions

This page explains how to write functions in TypeScript, type their parameters and their return value, and pass one function to another.

> **Goals:**
>
> - you will know how to write a function with parameters and a return type;
> - you will know how to make a parameter optional or give it a default value;
> - you will know how to write an arrow function `(x) => ...`;
> - you will know how to write the type of a function: `(x: number) => number`;
> - you will know how to pass a function as an argument (a *callback*) and type it.

## Defining and calling a function

A [function](../debuter/glossaire.md#function) is a named piece of code that you can
reuse. It's like a recipe: you write it once, then you follow it as many times as you
like.

```typescript
function greet(): void {
  console.log("Hello!");
}

greet();
greet();
```

```sortie
Hello!
Hello!
```

- `function` announces a function, `greet` is its name;
- the parentheses `()` hold its parameters (none here);
- `: void` says the function returns nothing useful (*void*: "empty");
- the block `{ ... }` is the body of the function.

Writing `function greet` doesn't run anything: it records the recipe. Writing
`greet()` **calls** the function, that is, runs it. The parentheses are required.

## Parameters and their type

A pancake recipe works for 4 people or for 10: only the quantities change. In the
same way, a function can do the same job with different values on each call.

[Parameters](../debuter/glossaire.md#parameter) are the values the function receives.
They are empty slots, filled in at call time by the
[arguments](../debuter/glossaire.md#argument). In TypeScript, every parameter has a
type.

```typescript
function introduce(firstName: string, age: number): void {
  console.log(`${firstName} is ${age} years old.`);
}

introduce("Alice", 30);
introduce("Bob", 25);
```

```sortie
Alice is 30 years old.
Bob is 25 years old.
```

Line by line:

- the first three lines record the function: nothing is printed yet;
- `introduce("Alice", 30)` runs the function: `firstName` gets `"Alice"` and `age`
  gets `30`, then the body prints the sentence;
- `introduce("Bob", 25)` runs it again with other values: this time, `firstName` is
  `"Bob"` and `age` is `25`.

Here, `firstName` and `age` are the parameters; `"Alice"` and `30` are the arguments.

The type of a parameter is a contract: "give me a number". TypeScript checks
**every call**. `introduce("Alice", "thirty")` is refused before the program even
runs.

> **Remember:** TypeScript doesn't guess the type of a parameter. Always write it,
> otherwise `tsc --strict` shows an error (see the
> [common mistakes](#common-mistakes)).

## Returning a value with return

A calculator doesn't just display things: it **gives** you a result that you can reuse
in another calculation. `return` lets a function do the same.

`return` sends a value back to the code that called the function, and stops the
function. The type of that value is written after the parentheses.

```typescript
function area(width: number, height: number): number {
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

- `area(3, 4)` runs the function with `width` set to 3 and `height` set to 4;
- `return width * height` computes 12 and returns it: `console.log` prints 12;
- `area(2, 5)` returns 10, then 1 is added: `total` is 11;
- the last line prints `total`.

The returned value replaces the call: `area(2, 5) + 1` becomes `10 + 1`.

The return type `: number` can be left out: TypeScript guesses it from the
`return`. Writing it is still a good habit: if you make a mistake in the body of the
function, the error shows up in the function, not further away in the code that
calls it.

**Your turn:** write a function `perimeter` that takes the width and height of a
rectangle (two numbers) and returns its perimeter. Print `perimeter(3, 4)`.

> **Solution**
>
> ```typescript
> function perimeter(width: number, height: number): number {
>   return 2 * (width + height);
> }
>
> console.log(perimeter(3, 4));
> ```
>
> ```sortie
> 14
> ```

### Print or return?

`console.log` **prints** a value on the screen. `return` **gives** it to the code
that called the function, which can then use it.

```typescript
function printDouble(x: number): void {
  console.log(x * 2);
}

function double(x: number): number {
  return x * 2;
}

printDouble(5);
console.log(double(5) + 1);
```

```sortie
10
11
```

`printDouble(5) + 1` would be refused: a `void` function gives no usable value.

> **Tip:** prefer functions that return a value. You can then choose to print it,
> store it or reuse it.

### Leaving early

A `return` in the middle of the function stops it right away:

```typescript
function divide(a: number, b: number): string {
  if (b === 0) {
    return "cannot divide";
  }
  return (a / b).toString();
}

console.log(divide(10, 4));
console.log(divide(10, 0));
```

```sortie
2.5
cannot divide
```

TypeScript checks that **every path** returns a value of the right type. If a
`return` is missing, it tells you.

## Optional parameters and default values

Some information isn't always needed. At a café, you can order "a coffee" without
giving the size: the waiter brings the regular size. A function can also do without
some arguments.

### An optional parameter: ?

A `?` after the name makes the parameter optional. If it isn't given, it is
`undefined`: its type becomes `string | undefined`.

```typescript
function greet(name: string, title?: string): string {
  if (title === undefined) {
    return `Hello, ${name}!`;
  }
  return `Hello, ${title} ${name}!`;
}

console.log(greet("Alice"));
console.log(greet("Martin", "Doctor"));
```

```sortie
Hello, Alice!
Hello, Doctor Martin!
```

Line by line:

- `greet("Alice")` gives only one argument: `name` is `"Alice"` and `title` is
  `undefined`;
- so the test `title === undefined` is true: the function returns the sentence
  without a title;
- `greet("Martin", "Doctor")` gives both: the test is false, and the function returns
  the sentence with the title.

TypeScript forces you to handle the `undefined` case before using `title` as a
string.

### A default value: =

Simpler: give a default value. It is used when the argument is missing. The type is
guessed from that value.

```typescript
function priceWithTax(price: number, rate = 0.2): number {
  return price * (1 + rate);
}

console.log(priceWithTax(100));
console.log(priceWithTax(100, 0.055));
```

```sortie
120
105.5
```

> **Warning:** optional parameters go **after** the required ones.
> `function f(a?: number, b: number)` is refused.

**Your turn:** write a function `order` that takes a drink name and a size, with
`"medium"` as the default size. It returns, for example, `"A medium coffee"`. Print
`order("coffee")` and `order("tea", "large")`.

> **Solution**
>
> ```typescript
> function order(drink: string, size = "medium"): string {
>   return `A ${size} ${drink}`;
> }
>
> console.log(order("coffee"));
> console.log(order("tea", "large"));
> ```
>
> ```sortie
> A medium coffee
> A large tea
> ```

## A variable number of arguments

> **Note:** this part can wait. Read it once the basics feel solid.

Three dots `...` in front of the last parameter turn it into a **rest parameter**:
it receives all the remaining arguments, stored in an array. Its type is an array,
here `number[]` ("list of numbers").

```typescript
function sum(...numbers: number[]): number {
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

- `sum(1, 2, 3, 4)` stores the four arguments in the array `numbers`: `[1, 2, 3, 4]`;
- `total` starts at 0, then the loop adds each number to it: 1, 3, 6, 10;
- `return total` returns 10;
- `sum()` receives an empty array: the loop doesn't run, and the function returns 0.

Each argument is checked: `sum(1, "2")` would be refused.

## Arrow functions

Many functions are tiny: a single line of calculation. Writing `function`, braces and
`return` for so little is long. The arrow function is a shortcut for these cases.

An **arrow function** is a short way of writing a function, with `=>`. You often
store it in a constant.

```typescript
const square = (x: number): number => x * x;

const introduce = (firstName: string): string => {
  const capitals = firstName.toUpperCase();
  return `I am ${capitals}.`;
};

console.log(square(4));
console.log(introduce("léa"));
```

```sortie
16
I am LÉA.
```

Line by line:

- `square` is a constant that holds a function: it takes `x` and returns `x * x`;
- `introduce` holds a longer function, between braces: it puts the first name in
  capitals, then returns the sentence;
- `square(4)` is called like an ordinary function and returns 16;
- `introduce("léa")` returns `"I am LÉA."`.

- With a single expression after `=>`, you need neither braces nor `return`: the
  value is returned automatically.
- With several lines, you add braces and a `return`.

A function with no name, like this one, is also called a
[lambda](../debuter/glossaire.md#lambda). Arrow functions are everywhere in
TypeScript, especially as *callbacks* (below).

**Your turn:** write an arrow function `triple`, stored in a constant, that takes a
number and returns that number multiplied by 3. Print `triple(7)`.

> **Solution**
>
> ```typescript
> const triple = (x: number): number => x * 3;
>
> console.log(triple(7));
> ```
>
> ```sortie
> 21
> ```

## The type of a function

An electric socket only accepts one shape of plug. The type of a function plays the
same role: it says in advance what kind of function a variable or a parameter accepts.

A function is a value, like a number or a string: you can store it in a variable.
Its type is written like an arrow function: the parameters, then `=>`, then the
return type.

```typescript
let operation: (a: number, b: number) => number;

operation = (a, b) => a + b;
console.log(operation(6, 3));

operation = (a, b) => a * b;
console.log(operation(6, 3));
```

```sortie
9
18
```

Line by line:

- the first line creates the variable `operation`, still empty, and says what it may
  hold: a function that takes two numbers and returns a number;
- first, a function that adds is stored in it: `operation(6, 3)` gives 9;
- then it is replaced by a function that multiplies: the same call gives 18.

`(a: number, b: number) => number` reads: "a function that takes two numbers and
returns a number". Notice: in `(a, b) => a + b`, there's no need to write the
parameter types any more. TypeScript already knows them from the type of
`operation`.

To reuse a function type, give it a name with `type`:

```typescript
type Operation = (a: number, b: number) => number;

const plus: Operation = (a, b) => a + b;
const minus: Operation = (a, b) => a - b;

console.log(plus(10, 4), minus(10, 4));
```

```sortie
14 6
```

## Callbacks: passing a function to a function

A **callback** is a function passed as an argument to another function, which will
call it later. It's like leaving your number at the bakery: they will call you back
when the bread is ready.

```typescript
function repeat(times: number, action: (round: number) => void): void {
  for (let i = 1; i <= times; i++) {
    action(i);
  }
}

repeat(3, (round) => {
  console.log(`Round number ${round}`);
});
```

```sortie
Round number 1
Round number 2
Round number 3
```

Line by line:

- `repeat` receives two things: a number (`times`) and a function (`action`);
- in the call, `times` is 3 and `action` is the arrow function written between the
  parentheses;
- the loop gives `i` the values 1, 2 then 3, and calls `action(i)` on each round;
- each call stores `i` in `round` and prints the sentence.

The `action` parameter has a function type: `(round: number) => void`. Thanks to it,
TypeScript knows that `round` is a `number` in the arrow function passed in the call.

Here is a more useful example: a function that transforms each number of a list.

```typescript
function transform(numbers: number[], f: (n: number) => number): number[] {
  const result: number[] = [];
  for (const n of numbers) {
    result.push(f(n));
  }
  return result;
}

console.log(transform([1, 2, 3], (n) => n * 10));
console.log(transform([1, 2, 3], (n) => n - 1));
```

```sortie
[ 10, 20, 30 ]
[ 0, 1, 2 ]
```

`push` adds an item to the end of an array. Arrays already have a `map` method that
does exactly this: you will see it in [arrays](collections.md).

**Your turn:** write a function `apply` that takes a number and a function of type
`(n: number) => number`, and returns the result of that function on the number.
Print `apply(5, (n) => n + 100)`.

> **Solution**
>
> ```typescript
> function apply(x: number, f: (n: number) => number): number {
>   return f(x);
> }
>
> console.log(apply(5, (n) => n + 100));
> ```
>
> ```sortie
> 105
> ```

## Variable scope

Each function has its own workbench. What it puts there stays with it: this keeps a
function from accidentally disturbing the variables of the rest of the program.

The [scope](../debuter/glossaire.md#scope) of a variable is the part of the code
where it exists. Parameters and variables created in a function only exist inside
that function.

```typescript
const vatRate = 0.2;

function priceWithTax(price: number): number {
  const total = price * (1 + vatRate);
  return Math.round(total * 100) / 100;
}

console.log(priceWithTax(19.99));
```

```sortie
23.99
```

Line by line:

- `vatRate` is created at the very top, outside the function: it is 0.2;
- `priceWithTax(19.99)` runs the function: `price` is 19.99;
- `total` is created **inside** the function and is about 23.988;
- `Math.round(total * 100) / 100` rounds to two digits after the decimal point: the
  function returns 23.99.

- The function sees `vatRate`, created **outside** of it, above it.
- Outside, `total` doesn't exist: `console.log(total)` would be refused.

> **Tip:** a function is easier to understand when it receives everything it needs
> as parameters.

## Overloads, briefly

> **Note:** this part can wait. Read it once the basics feel solid.

Sometimes, a function accepts several forms of call, with a different return type
for each. TypeScript lets you list them before the body: these are **overloads**.

```typescript
function doubleIt(x: number): number;
function doubleIt(x: string): string;
function doubleIt(x: number | string): number | string {
  return typeof x === "number" ? x * 2 : x + x;
}

console.log(doubleIt(21));
console.log(doubleIt("ha"));
```

```sortie
42
haha
```

The first two lines are the allowed forms; the third is the real code. Thanks to
them, `doubleIt(21)` is known to be a `number`, and `doubleIt("ha")` a `string`.
You will come across them in libraries; when you're starting out, a union
(`number | string`) is often enough.

## Common mistakes

### Forgetting the type of a parameter

```typescript norun
function double(x) {
  return x * 2;
}
console.log(double(4));
```

```text
main.ts:1:17 - error TS7006: Parameter 'x' implicitly has an 'any' type.

1 function double(x) {
                  ~


Found 1 error in main.ts:1
```

Without a type, `x` could be anything: strict mode refuses. Write
`function double(x: number)`.

### Forgetting an argument

```typescript norun
function area(width: number, height: number): number {
  return width * height;
}
console.log(area(3));
```

```text
main.ts:4:13 - error TS2554: Expected 2 arguments, but got 1.

4 console.log(area(3));
              ~~~~

  main.ts:1:30
    1 function area(width: number, height: number): number {
                                   ~~~~~~~~~~~~~~
    An argument for 'height' was not provided.


Found 1 error in main.ts:4
```

The function expects 2 arguments, it gets 1. The message even says which one is
missing: `height`. Give them all: `area(3, 4)`. Or make the parameter optional with
`?` or a default value.

### Passing an argument of the wrong type

```typescript norun
function area(width: number, height: number): number {
  return width * height;
}
console.log(area(3, "4"));
```

```text
main.ts:4:21 - error TS2345: Argument of type 'string' is not assignable to parameter of type 'number'.

4 console.log(area(3, "4"));
                      ~~~


Found 1 error in main.ts:4
```

`"4"` is a string, not a number. Write `area(3, 4)`, or convert the string with
`Number("4")`.

### Forgetting a return on one path

```typescript norun
function verdict(grade: number): string {
  if (grade >= 10) {
    return "passed";
  } else if (grade < 10) {
    return "failed";
  }
}
console.log(verdict(12));
```

```text
main.ts:1:34 - error TS2366: Function lacks ending return statement and return type does not include 'undefined'.

1 function verdict(grade: number): string {
                                   ~~~~~~


Found 1 error in main.ts:1
```

A `return` is missing at the end. TypeScript doesn't see that the two tests cover
every case. Replace `else if (grade < 10)` with a plain `else`: then every path
returns a string.

## Test yourself

**Question 1:** write an arrow function `average` that takes two numbers and
returns their average.

> **Answer**
>
> ```typescript
> const average = (a: number, b: number): number => (a + b) / 2;
>
> console.log(average(3, 4));
> ```
>
> ```sortie
> 3.5
> ```

**Question 2:** what does the type `(text: string) => boolean` mean?

> **Answer**
>
> It is the type of a function that takes a string and returns `true` or `false`. For
> example: `(text) => text.length > 3`.

**Question 3:** what is the difference between `title?: string` and `title = "Mr."`?

> **Answer**
>
> Both make the parameter optional. With `?`, it is `undefined` when it is missing:
> you have to test that case before using it. With `= "Mr."`, it is `"Mr."` when it is
> missing: it is always a `string`.

## Summary

- A function is written `function name(param: type): returnType { ... }` and called
  with `name(...)`.
- Parameters must always be typed; the return type can be guessed, but writing it
  helps.
- `?` makes a parameter optional (it is then `undefined`), `= value` gives it a
  default value, `...name: T[]` receives all the remaining arguments.
- An arrow function is written `(x: number) => x * 2`; `void` means "returns
  nothing".
- The type of a function is written `(x: number) => number`: it lets you type
  callbacks, whose parameters are then guessed.

Next: [working with text](texte.md).
