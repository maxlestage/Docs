# Functions

This page explains how to write functions in TypeScript, type their parameters and their return value, and know where their variables exist.

> **Goals:**
>
> - you will know how to write a function with parameters and a return type;
> - you will know how to make a parameter optional or give it a default value;
> - you will know how to write an arrow function `(x) => ...`;
> - you will know where a variable exists: its scope.

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
TypeScript, especially as *callbacks* (in the next chapter).

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

**Question 2:** what is the difference between `title?: string` and `title = "Mr."`?

> **Answer**
>
> Both make the parameter optional. With `?`, it is `undefined` when it is missing:
> you have to test that case before using it. With `= "Mr."`, it is `"Mr."` when it is
> missing: it is always a `string`.

**Question 3:** a variable `total` is created inside a function. After the call, can you
write `console.log(total)` outside the function?

> **Answer**
>
> No. The scope of `total` is the function: outside it, it doesn't exist, and
> TypeScript refuses (*Cannot find name 'total'*). To get the value back, the function
> must return it with `return`.

## Summary

- A function is written `function name(param: type): returnType { ... }` and called
  with `name(...)`.
- Parameters must always be typed; the return type can be guessed, but writing it
  helps.
- `?` makes a parameter optional (it is then `undefined`), `= value` gives it a
  default value.
- An arrow function is written `(x: number) => x * 2`; `void` means "returns
  nothing".
- Parameters and variables created inside a function only exist in that function:
  that is their scope.

Next: [going further with functions](fonctions-avancees.md).
