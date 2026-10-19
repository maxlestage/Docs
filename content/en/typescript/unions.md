# Unions and narrowing

This page explains how to say that a value can have several types, and how TypeScript works out the exact type from your tests.

> **Goals:**
>
> - you will know how to write a union type with `|`;
> - you will know how to narrow a union with `typeof`, `in`, `instanceof` and `===`;
> - you will know how to build a discriminated union with a `kind` field;
> - you will know how to check that a `switch` handles every case thanks to `never`;
> - you will know how to write your own type guard `x is T`.

## A union type

Sometimes, a value can be of two different [types](../debuter/glossaire.md#type).
An identifier can be a number (`42`) or a text (`"A-42"`). A **union type** is
written with a vertical bar `|`, which reads "or".

```typescript
let identifier: number | string;

identifier = 42;
console.log(identifier);

identifier = "A-42";
console.log(identifier);
```

```sortie
42
A-42
```

A union is like a mailbox that accepts letters **or** parcels. It refuses everything
else: `identifier = true` would be an error.

You can give a union a name with `type`:

```typescript
type Identifier = number | string;

function show(id: Identifier) {
  console.log(`Identifier: ${id}`);
}

show(7);
show("B-7");
```

```sortie
Identifier: 7
Identifier: B-7
```

### What you can do with a union

With a union, TypeScript only allows what works for **all** the types of the union.
`toString()` exists on numbers and on strings: it is allowed. `toUpperCase()` only
exists on strings: it is refused (see the [common mistakes](#common-mistakes)).

```typescript
function asText(x: number | string): string {
  return x.toString();
}

console.log(asText(3.5) + " / " + asText("three"));
```

```sortie
3.5 / three
```

To go further, you first need to **know** which of the types you have.

## Narrowing with typeof

When you test the type of a value in an `if`, TypeScript remembers it. Inside the
`if` block, it **narrows** the union: it knows the value has the tested type. It's
like a detective ruling out suspects one by one.

`typeof x` gives the name of the type as a string: `"string"`, `"number"`,
`"boolean"`, `"object"`, `"undefined"`, `"function"`…

```typescript
function shout(x: string | number): string {
  if (typeof x === "string") {
    // here, x is a string
    return x.toUpperCase();
  }
  // here, only number is left
  return x.toFixed(1);
}

console.log(shout("hello"));
console.log(shout(2));
```

```sortie
HELLO
2.0
```

After the `return`, TypeScript knows the `string` case is handled. Only `number` is
left. No need for an `else`.

## Narrowing by truthiness

An optional value often has the type `string | undefined`. A simple `if (x)` rules out
`undefined` (and also `null`, `""`, `0`): this is called a **truthiness** test.

```typescript
function greet(firstName?: string) {
  if (firstName) {
    console.log(`Hello, ${firstName}!`);
  } else {
    console.log("Hello, stranger!");
  }
}

greet("Alice");
greet();
```

```sortie
Hello, Alice!
Hello, stranger!
```

> **Warning:** `if (x)` also rejects `0` and the empty string `""`. For a number,
> test `x !== undefined` instead, otherwise `0` will be taken as "no value".

## Narrowing by equality

Comparing with `===` narrows too. If `a === b`, both necessarily have a type in
common. It's very handy with `null`:

```typescript
function length(text: string | null): number {
  if (text === null) {
    return 0;
  }
  return text.length;
}

console.log(length("cat"));
console.log(length(null));
```

```sortie
3
0
```

## Narrowing with in

For objects, `typeof` always answers `"object"`. The `in` operator tests whether an
object has a property: `"fly" in animal`.

```typescript
type Bird = { name: string; fly(): string };
type Fish = { name: string; swim(): string };

function move(animal: Bird | Fish): string {
  if ("fly" in animal) {
    return animal.fly();
  }
  return animal.swim();
}

const magpie: Bird = { name: "magpie", fly: () => "The magpie flies." };
const trout: Fish = { name: "trout", swim: () => "The trout swims." };

console.log(move(magpie));
console.log(move(trout));
```

```sortie
The magpie flies.
The trout swims.
```

## Narrowing with instanceof

For objects created with `new` (the [classes](types.md)), `instanceof` tests the
class of the object. `Date` is a class provided by JavaScript.

```typescript
function format(value: Date | string): string {
  if (value instanceof Date) {
    return value.toISOString().slice(0, 10);
  }
  return value;
}

console.log(format(new Date(Date.UTC(2024, 0, 15))));
console.log(format("tomorrow"));
```

```sortie
2024-01-15
tomorrow
```

> **Note:** in `Date.UTC(2024, 0, 15)`, months start at 0: `0` means January.

## Discriminated unions

Here is the most useful tool of this chapter. Each type of the union carries a common
field, often called `kind`, with a **fixed** and different value. This field works as
a label: you just read it to know which type you are dealing with.

```typescript
type Circle = { kind: "circle"; radius: number };
type Rectangle = { kind: "rectangle"; width: number; height: number };
type Shape = Circle | Rectangle;

function area(s: Shape): number {
  if (s.kind === "circle") {
    return Math.PI * s.radius ** 2;
  }
  return s.width * s.height;
}

console.log(area({ kind: "rectangle", width: 3, height: 4 }));
console.log(area({ kind: "circle", radius: 1 }).toFixed(2));
```

```sortie
12
3.14
```

Inside the `if`, TypeScript knows that `s` is a `Circle`: `s.radius` is allowed.
After it, only `Rectangle` is left. The type `"circle"` is a **literal type**: it
accepts only one value (see [literal types](litteraux.md)).

### With a switch

With several cases, a `switch` on `kind` reads better than a chain of `if`s:

```typescript
type AppEvent =
  | { kind: "click"; x: number; y: number }
  | { kind: "key"; key: string }
  | { kind: "close" };

function describe(e: AppEvent): string {
  switch (e.kind) {
    case "click":
      return `Click at (${e.x}, ${e.y})`;
    case "key":
      return `Key ${e.key}`;
    case "close":
      return "Window closed";
  }
}

console.log(describe({ kind: "click", x: 10, y: 20 }));
console.log(describe({ kind: "key", key: "Enter" }));
console.log(describe({ kind: "close" }));
```

```sortie
Click at (10, 20)
Key Enter
Window closed
```

## Checking every case with never

The type `never` means "impossible": no value can have it. It acts as a safety net.
In the `default` of a `switch`, if every case has been handled, the value has the type
`never`. If you forget a case, it has another type, and TypeScript complains.

```typescript
type Light = { kind: "green" } | { kind: "amber" } | { kind: "red" };

function instruction(light: Light): string {
  switch (light.kind) {
    case "green":
      return "Go";
    case "amber":
      return "Slow down";
    case "red":
      return "Stop";
    default: {
      const impossible: never = light;
      return impossible;
    }
  }
}

console.log(instruction({ kind: "amber" }));
```

```sortie
Slow down
```

The day you add `{ kind: "flashing" }` to the `Light` type, the line
`const impossible: never = light` becomes an error. TypeScript shows you the exact
spot to complete. It's a free oversight detector.

## Your own type guards

Sometimes, the test is too complicated for a simple `typeof`. You can write a
function that does the test and returns a boolean, with a special return type:
`x is T` ("x is a T"). This is a **type guard**.

```typescript
type Cat = { name: string; meow(): string };
type Dog = { name: string; bark(): string };

function isCat(animal: Cat | Dog): animal is Cat {
  return "meow" in animal;
}

const animals: (Cat | Dog)[] = [
  { name: "Kitty", meow: () => "Meow" },
  { name: "Rex", bark: () => "Woof" },
];

for (const a of animals) {
  if (isCat(a)) {
    console.log(`${a.name}: ${a.meow()}`);
  } else {
    console.log(`${a.name}: ${a.bark()}`);
  }
}
```

```sortie
Kitty: Meow
Rex: Woof
```

A type guard also works with `filter`: the resulting array has the right type.

```typescript
function isNumber(x: number | string): x is number {
  return typeof x === "number";
}

const mix = [3, "a", 7, "b"];
const numbers: number[] = mix.filter(isNumber);
console.log(numbers);
```

```sortie
[ 3, 7 ]
```

> **Warning:** TypeScript takes your guard at its word. If the function wrongly
> returns `true`, the type will be wrong. Keep the test simple and correct.

## Common mistakes

### Using a method without narrowing

```typescript norun
function shout(x: string | number) {
  return x.toUpperCase();
}
```

```text
main.ts(2,12): error TS2339: Property 'toUpperCase' does not exist on type 'string | number'.
  Property 'toUpperCase' does not exist on type 'number'.
```

`toUpperCase` doesn't exist on numbers. Test the type first:
`if (typeof x === "string") return x.toUpperCase();`.

### Forgetting that the value may be missing

```typescript norun
function initial(name?: string) {
  return name[0];
}
```

```text
main.ts(2,10): error TS18048: 'name' is possibly 'undefined'.
```

An optional parameter has the type `string | undefined`. Handle the missing case:
`if (name === undefined) return "?";`, or give a default value `(name = "?")`.

### Reading a field that only exists in one case

```typescript norun
type Circle = { kind: "circle"; radius: number };
type Square = { kind: "square"; side: number };

function describe(s: Circle | Square) {
  return s.radius;
}
```

```text
main.ts(5,12): error TS2339: Property 'radius' does not exist on type 'Circle | Square'.
  Property 'radius' does not exist on type 'Square'.
```

A square has no radius. Test the label first:
`if (s.kind === "circle") return s.radius;`.

### Forgetting a case in the switch

```typescript norun
type Shape =
  | { kind: "circle"; radius: number }
  | { kind: "square"; side: number }
  | { kind: "triangle"; base: number; height: number };

function area(s: Shape): number {
  switch (s.kind) {
    case "circle":
      return Math.PI * s.radius ** 2;
    case "square":
      return s.side ** 2;
    default: {
      const impossible: never = s;
      return impossible;
    }
  }
}
```

```text
main.ts(13,13): error TS2322: Type '{ kind: "triangle"; base: number; height: number; }' is not assignable to type 'never'.
```

The message names the forgotten case: the triangle. Add
`case "triangle": return (s.base * s.height) / 2;`.

## Test yourself

**Question 1:** in this code, what is the type of `x` inside the `if`?

```typescript norun
function f(x: string | number | boolean) {
  if (typeof x === "number") {
    // ?
  }
}
```

> **Answer**
>
> `number`. The test `typeof x === "number"` narrows the union down to the single type
> `number`. After the `if`, `x` can still be `string | number | boolean`, because the
> block doesn't end with a `return`.

**Question 2:** what is the `kind` field for in a discriminated union?

> **Answer**
>
> It is a label. Each type of the union has a different value for `kind`. By testing
> `kind` (with `if` or `switch`), TypeScript knows exactly which type you have, and so
> which fields are available.

**Question 3:** write a type guard `isText` that tells whether an `unknown` value is
a `string`.

> **Answer**
>
> ```typescript
> function isText(x: unknown): x is string {
>   return typeof x === "string";
> }
>
> const value: unknown = "hi";
> if (isText(value)) {
>   console.log(value.toUpperCase());
> }
> ```
>
> ```sortie
> HI
> ```

## Summary

- A union type `A | B` accepts a value of type `A` or of type `B`.
- On a union, only what works for all the types is allowed.
- `typeof`, `in`, `instanceof`, `===` and the truthiness test narrow the union.
- A discriminated union carries a common label (`kind`) that you test with `switch`.
- `const impossible: never = x` in the `default` flags any forgotten case.
- A type guard `x is T` wraps a test in a reusable function.

Next: [classes](types.md)
