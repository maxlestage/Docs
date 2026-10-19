# Going further with functions

This page shows how a TypeScript function receives a variable number of arguments, how to write the type of a function, pass it to another function, and what overloads are.

> **Goals:**
>
> - you will know how to write the type of a function: `(x: number) => number`;
> - you will know how to give a name to a function type with `type`;
> - you will know how to pass a function as an argument (a *callback*) and type it.

This chapter follows [functions](fonctions.md): you will meet arrow functions again, used here as values that you store and pass to other functions.

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

### Putting the rest parameter before another one

```typescript norun
function invite(...names: string[], room: string): void {
  console.log(`${names.join(", ")}: room ${room}`);
}
invite("Alice", "Bob", "B12");
```

```text
main.ts:1:17 - error TS1014: A rest parameter must be last in a parameter list.

1 function invite(...names: string[], room: string): void {
                  ~~~

main.ts:4:24 - error TS2554: Expected 2 arguments, but got 3.

4 invite("Alice", "Bob", "B12");
                         ~~~~~


Found 2 errors in the same file, starting at: main.ts:1
```

"A rest parameter must be the last one in the list." It takes all the remaining
arguments: none would be left for `room`. The second error comes from the first one.
Put it last: `function invite(room: string, ...names: string[])`, then call
`invite("B12", "Alice", "Bob")`.

### Passing a callback of the wrong type

```typescript norun
function repeat(times: number, action: (round: number) => void): void {
  for (let i = 1; i <= times; i++) {
    action(i);
  }
}

repeat(3, (round: string) => {
  console.log(`Round number ${round}`);
});
```

```text
main.ts:7:11 - error TS2345: Argument of type '(round: string) => void' is not assignable to parameter of type '(round: number) => void'.
  Types of parameters 'round' and 'round' are incompatible.
    Type 'number' is not assignable to type 'string'.

7 repeat(3, (round: string) => {
            ~~~~~~~~~~~~~~~~~~~~


Found 1 error in main.ts:7
```

`repeat` calls `action` with a number: a function that expects a string doesn't fit
(*types of parameters are incompatible*). Remove the parameter's type: with
`(round) => { ... }`, TypeScript guesses on its own that `round` is a `number`.

### Calling an overload that doesn't exist

```typescript norun
function doubleIt(x: number): number;
function doubleIt(x: string): string;
function doubleIt(x: number | string): number | string {
  return typeof x === "number" ? x * 2 : x + x;
}

console.log(doubleIt(true));
```

```text
main.ts:7:22 - error TS2769: No overload matches this call.
  Overload 1 of 2, '(x: number): number', gave the following error.
    Argument of type 'boolean' is not assignable to parameter of type 'number'.
  Overload 2 of 2, '(x: string): string', gave the following error.
    Argument of type 'boolean' is not assignable to parameter of type 'string'.

7 console.log(doubleIt(true));
                       ~~~~


Found 1 error in main.ts:7
```

"No overload matches this call." TypeScript tries each allowed form and says why each
one fails: `true` is neither a `number` nor a `string`. Pass a number or a string:
`doubleIt(21)`.

## Test yourself

**Question 1:** how many arguments can you give to `function f(...words: string[])`? What is
`words` in the call `f()`?

> **Answer**
>
> As many as you like, even none: each one must be a `string`. With `f()`, `words`
> is an empty array, `[]`.

**Question 2:** what does the type `(text: string) => boolean` mean?

> **Answer**
>
> It is the type of a function that takes a string and returns `true` or `false`. For
> example: `(text) => text.length > 3`.

**Question 3:** write a function `twice` that receives a callback of type `() => void` and
calls it twice.

> **Answer**
>
> ```typescript
> function twice(action: () => void): void {
>   action();
>   action();
> }
>
> twice(() => console.log("Hello"));
> ```
>
> ```sortie
> Hello
> Hello
> ```

## Summary

- `...name: T[]` receives all the remaining arguments in an array; it goes last.
- The type of a function is written `(x: number) => number`; `type` gives it a name.
- A callback is a function passed as an argument; thanks to the parameter's type, its
  own parameters are guessed.
- Overloads list several forms of call; a union is often enough.

Next: [working with text](texte.md).
