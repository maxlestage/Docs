# Generics

This page explains how to write a function or a class that works with any type, without losing the precision of the types.

> **Goals:**
>
> - you will know how to write a generic function with `<T>`;
> - you will know how to let TypeScript guess the type, or give it yourself;
> - you will know how to limit a generic with `extends`;
> - you will know how to write generic interfaces and classes like `Box<T>`;
> - you will know how to link two parameters with `K extends keyof T`.

## The problem

Imagine a function that returns the first element of an array. For numbers, you would
write:

```typescript
function firstNumber(list: number[]): number | undefined {
  return list[0];
}

console.log(firstNumber([10, 20, 30]));
```

```sortie
10
```

And for strings? You would have to copy the function and change `number` into `string`.
Then again for booleans… With `any`, a single function would be enough, but
TypeScript would no longer know anything about the result: it could no longer protect
you.

[Generics](../debuter/glossaire.md#generique) solve this problem: a single function,
and types that stay precise.

## A generic function

A generic is a **type parameter**. It is written between angle brackets `<T>`, right
after the function name. `T` is a type name "to fill in later", just like an ordinary
parameter is a value to fill in later.

```typescript
function first<T>(list: T[]): T | undefined {
  return list[0];
}

const n = first([10, 20, 30]);
const f = first(["apple", "pear"]);

console.log(n);
console.log(f?.toUpperCase());
```

```sortie
10
APPLE
```

Read the signature like this: "for any type `T`, `first` takes an array of `T` and
returns a `T` (or `undefined`)". It is like a blank label on a jar: you write "jam" or
"pickles" on it when you fill it.

> **Note:** `T` is a convention (for *Type*). You can choose another name, such as
> `Element`. With several parameters, people often use `T`, `U`, `K`, `V`.

## Inference

In the example, you never wrote what `T` was. TypeScript **inferred** it (guessed it)
from the arguments: `[10, 20, 30]` is a `number[]`, so `T` is `number`, so `n` is a
`number | undefined`.

You can also give `T` yourself, between angle brackets in the call:

```typescript
function wrap<T>(value: T): T[] {
  return [value];
}

const a = wrap(5);
const b = wrap<string | number>("five");
b.push(5);

console.log(a, b);
```

```sortie
[ 5 ] [ 'five', 5 ]
```

Without `<string | number>`, `b` would be a `string[]`, and `b.push(5)` would be
refused. In general, let TypeScript guess; only be explicit when it guesses too
narrow.

## Several type parameters

A function can have several generics. Here, `pair` puts together two values of
different types:

```typescript
function pair<A, B>(a: A, b: B): [A, B] {
  return [a, b];
}

function transform<T, U>(list: T[], f: (x: T) => U): U[] {
  const result: U[] = [];
  for (const x of list) {
    result.push(f(x));
  }
  return result;
}

console.log(pair("age", 12));
console.log(transform(["a", "bb", "ccc"], (word) => word.length));
```

```sortie
[ 'age', 12 ]
[ 1, 2, 3 ]
```

In `transform`, TypeScript deduces `T = string` from the array, then `U = number` from
what the function returns. This is exactly how the `map` method of arrays is typed.

## Constraints with extends

Sometimes, `T` cannot be **just anything**. To read `.length`, the value must have a
length. `T extends Constraint` means: "`T` can be any type, **as long as it** respects
the constraint".

```typescript
function longest<T extends { length: number }>(a: T, b: T): T {
  return a.length >= b.length ? a : b;
}

console.log(longest("cat", "elephant"));
console.log(longest([1, 2, 3], [4]));
```

```sortie
elephant
[ 1, 2, 3 ]
```

Strings and arrays all have a `length`: they are accepted. A number would be refused.
And the result keeps the exact type: `string` in the first call, `number[]` in the
second.

A constraint can also be an interface:

```typescript
interface WithName {
  name: string;
}

function sortByName<T extends WithName>(list: T[]): T[] {
  return [...list].sort((a, b) => a.name.localeCompare(b.name));
}

const students = [
  { name: "Zoé", grade: 15 },
  { name: "Adam", grade: 12 },
];

for (const s of sortByName(students)) {
  console.log(`${s.name}: ${s.grade}`);
}
```

```sortie
Adam: 12
Zoé: 15
```

Notice that `s.grade` is allowed: the function returns `T`s, that is, objects with
`name` **and** `grade`, not just `WithName`s.

## Generic interfaces and types

A type can also have a parameter. `Box<T>` describes a box that holds a value of type
`T`:

```typescript
interface Box<T> {
  content: T;
  label: string;
}

const numberBox: Box<number> = { content: 42, label: "answer" };
const listBox: Box<string[]> = { content: ["a", "b"], label: "letters" };

console.log(numberBox.content + 1);
console.log(listBox.content.join(", "));
```

```sortie
43
a, b
```

You already use them without knowing it: `Array<number>` is the other way of writing
`number[]`, and `Promise<string>` is a promise that will give a string (see
[asynchronous code](asynchrone.md)).

## Generic classes

A class can have a type parameter. Here is a **stack**: like a stack of plates, you
put things on top and take them back from the top.

```typescript
class Stack<T> {
  private items: T[] = [];

  push(x: T): void {
    this.items.push(x);
  }

  pop(): T | undefined {
    return this.items.pop();
  }

  get size(): number {
    return this.items.length;
  }
}

const plates = new Stack<string>();
plates.push("blue");
plates.push("red");
console.log(plates.pop());
console.log(plates.size);

const numbers = new Stack<number>();
numbers.push(7);
console.log(numbers.pop()! * 2);
```

```sortie
red
1
14
```

`Stack<string>` only accepts strings, `Stack<number>` only numbers. A single class,
and each stack is protected against mix-ups.

> **Note:** the `!` after `pop()` tells TypeScript "I know this is not `undefined`".
> Only use it when you are certain, as here.

## Default types

A type parameter can have a default value, like a function parameter: `<T = string>`.
If nobody specifies `T`, it is `string`.

```typescript
interface ApiResponse<T = string> {
  code: number;
  data: T;
}

const text: ApiResponse = { code: 200, data: "OK" };
const list: ApiResponse<number[]> = { code: 200, data: [1, 2, 3] };

console.log(text.data.toLowerCase());
console.log(list.data.length);
```

```sortie
ok
3
```

## Linking two parameters with keyof

`keyof T` is the union of the property names of `T`. For
`{ name: string; age: number }`, it is `"name" | "age"`. With `K extends keyof T`, you
say: "`K` must be one of the keys of `T`".

```typescript
function getProp<T, K extends keyof T>(obj: T, key: K): T[K] {
  return obj[key];
}

const cat = { name: "Tom", age: 3, greedy: true };

const name = getProp(cat, "name");
const age = getProp(cat, "age");

console.log(name.toUpperCase());
console.log(age + 1);
```

```sortie
TOM
4
```

The return type `T[K]` reads "the type of property `K` in `T`". For `"name"`, it is
`string`; for `"age"`, it is `number`. And a key that does not exist, such as
`"color"`, is refused before the program even runs.

## Common mistakes

### Using a property without a constraint

```typescript norun
function getLength<T>(x: T): number {
  return x.length;
}
```

```text
main.ts(2,12): error TS2339: Property 'length' does not exist on type 'T'.
```

`T` can be anything, even a number, which has no `length`. Add a constraint:
`function getLength<T extends { length: number }>(x: T)`.

### Asking for a key that does not exist

```typescript norun
function getProp<T, K extends keyof T>(obj: T, key: K): T[K] {
  return obj[key];
}
const cat = { name: "Tom", age: 3 };
console.log(getProp(cat, "color"));
```

```text
main.ts(5,26): error TS2345: Argument of type '"color"' is not assignable to parameter of type '"name" | "age"'.
```

The message gives the list of allowed keys: `"name" | "age"`. Fix the typo, or add
the property to the object.

### Mixing types in a generic class

```typescript norun
class Stack<T> {
  private items: T[] = [];
  push(x: T) {
    this.items.push(x);
  }
}
const stack = new Stack<number>();
stack.push("ten");
```

```text
main.ts(8,12): error TS2345: Argument of type 'string' is not assignable to parameter of type 'number'.
```

The stack was created for numbers: `T` is `number` everywhere. Push `10`, or create a
`Stack<string>` for strings.

## Test yourself

**Question 1:** what is the type of `x`?

```typescript norun
function last<T>(list: T[]): T | undefined {
  return list[list.length - 1];
}
const x = last([true, false]);
```

> **Answer**
>
> `boolean | undefined`. TypeScript infers `T = boolean` from `[true, false]`.

**Question 2:** what does `<T extends { id: number }>` mean?

> **Answer**
>
> `T` can be any type, as long as it has an `id` property of type `number`. Inside the
> function, you can therefore read `x.id`, and the exact type of `T` is kept.

**Question 3:** write a generic function `repeat<T>(value: T, n: number)` that
returns an array containing the value `n` times.

> **Answer**
>
> ```typescript
> function repeat<T>(value: T, n: number): T[] {
>   const result: T[] = [];
>   for (let i = 0; i < n; i++) {
>     result.push(value);
>   }
>   return result;
> }
>
> console.log(repeat("ha", 3));
> console.log(repeat(0, 2));
> ```
>
> ```sortie
> [ 'ha', 'ha', 'ha' ]
> [ 0, 0 ]
> ```

## Summary

- A generic `<T>` is a type parameter: a single function for all types.
- TypeScript infers `T` from the arguments; you can also write it in the call.
- `T extends Constraint` limits `T` to the types that have what is needed.
- Interfaces and classes can be generic: `Box<T>`, `Stack<T>`.
- `<T = string>` gives a default type.
- `K extends keyof T` and `T[K]` link a key and the type of its value.

Next: [literal types, enums and as const](litteraux.md)
