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

Here, `firstName` and `age` are the parameters; `"Alice"` and `30` are the arguments.

The type of a parameter is a contract: "give me a number". TypeScript checks
**every call**. `introduce("Alice", "thirty")` is refused before the program even
runs.

> **Remember:** TypeScript doesn't guess the type of a parameter. Always write it,
> otherwise `tsc --strict` shows an error (see the
> [common mistakes](#common-mistakes)).

## Returning a value with return

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

The returned value replaces the call: `area(2, 5) + 1` becomes `10 + 1`.

The return type `: number` can be left out: TypeScript guesses it from the
`return`. Writing it is still a good habit: if you make a mistake in the body of the
function, the error shows up in the function, not further away in the code that
calls it.

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

## A variable number of arguments

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

Each argument is checked: `sum(1, "2")` would be refused.

## Arrow functions

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

- With a single expression after `=>`, you need neither braces nor `return`: the
  value is returned automatically.
- With several lines, you add braces and a `return`.

A function with no name, like this one, is also called a
[lambda](../debuter/glossaire.md#lambda). Arrow functions are everywhere in
TypeScript, especially as *callbacks* (below).

## The type of a function

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

## Variable scope

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

- The function sees `vatRate`, created **outside** of it, above it.
- Outside, `total` doesn't exist: `console.log(total)` would be refused.

> **Tip:** a function is easier to understand when it receives everything it needs
> as parameters.

## Overloads, briefly

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
