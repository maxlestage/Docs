# Objects, interfaces and type aliases

This page explains how to group values in objects, describe their shape with `interface` and `type`, and how TypeScript compares objects by their shape.

> **Goals:**
>
> - you will know how to create an object and read its properties;
> - you will know how to describe the shape of an object with `interface` or `type`;
> - you will know how to use optional (`?`) and read-only (`readonly`)
>   properties;
> - you will know how to extend an interface and write an index signature;
> - you will be able to explain structural typing and the check for extra
>   properties.

## Creating an object

An [object](../debuter/glossaire.md#object) groups several values under one name.
Each value has a label: its **property**. You write it between curly braces, with
`name: value` pairs.

```typescript
const book = {
  title: "Dune",
  author: "Frank Herbert",
  pages: 600,
};

console.log(book.title);
console.log(book["pages"]);
book.pages = 612;
console.log(book);
```

```sortie
Dune
600
{ title: 'Dune', author: 'Frank Herbert', pages: 612 }
```

- `book.title` reads a property with a dot;
- `book["pages"]` does the same thing with square brackets and a string;
- `book.pages = 612` changes a property.

An object is like a registration form: each box has a label (`title`, `pages`) and
some content.

TypeScript guesses the type of this object: `{ title: string; author: string; pages:
number }`. So it knows every property and its type. `book.pages = "lots"` would be
refused, and so would a typo like `book.titles`.

## Describing an object: the object type

For a function that receives an object, you have to write its shape. You describe it
between curly braces, like an object, but with types instead of values:

```typescript
function introduce(person: { name: string; age: number }): string {
  return `${person.name}, ${person.age} years old`;
}

console.log(introduce({ name: "Alice", age: 30 }));
```

```sortie
Alice, 30 years old
```

Writing this shape every time would be tedious. So you give it a name, in one of two
ways: `interface` or `type`.

## interface and type

### interface

An **interface** gives a name to the shape of an object. It's a plan, a blank form:
it says which boxes must exist and what they contain.

```typescript
interface Person {
  name: string;
  age: number;
}

const alice: Person = { name: "Alice", age: 30 };
const bob: Person = { name: "Bob", age: 25 };

function introduce(p: Person): string {
  return `${p.name}, ${p.age} years old`;
}

console.log(introduce(alice));
console.log(introduce(bob));
```

```sortie
Alice, 30 years old
Bob, 25 years old
```

By convention, the name of an interface starts with a capital letter.

### type: the type alias

The `type` keyword creates a **type alias**: a name for any type. You have already
seen it for unions (`type Answer = "yes" | "no"`). It works for an object too:

```typescript
type Product = {
  name: string;
  price: number;
};

const coffee: Product = { name: "Coffee", price: 2.5 };
console.log(`${coffee.name}: $${coffee.price}`);
```

```sortie
Coffee: $2.5
```

### Which one should you choose?

To describe an object, the two are almost always equivalent. The difference:

| | `interface` | `type` |
|---|---|---|
| Describe an object | yes | yes |
| Describe a union, a tuple, a function | no | yes |
| Extend with `extends` | yes | with `&` |

> **Remember:** a simple rule to start with: `interface` for objects, `type` for
> everything else (unions, tuples, functions). Above all, stay consistent within a
> project.

## Optional properties

A `?` after the name makes a property optional: the object may have it or not. Its
type becomes `string | undefined`.

```typescript
interface User {
  name: string;
  email?: string;
}

const alice: User = { name: "Alice", email: "alice@example.com" };
const bob: User = { name: "Bob" };

for (const u of [alice, bob]) {
  console.log(`${u.name}: ${u.email ?? "no address"}`);
}
```

```sortie
Alice: alice@example.com
Bob: no address
```

Before you use `u.email` as a string, TypeScript makes you handle the case where it
is missing: with `??`, `?.` or an `if`.

## Read-only properties

`readonly` in front of a property forbids changing it after the object is created.

```typescript
interface Point {
  readonly x: number;
  readonly y: number;
}

const start: Point = { x: 0, y: 0 };
const end: Point = { x: start.x + 3, y: start.y + 4 };

console.log(end);
```

```sortie
{ x: 3, y: 4 }
```

To "move" a point, you create a new one. This check only exists in TypeScript: when
the program runs, the object is a normal JavaScript object.

> **Tip:** mark as `readonly` the properties that must never change, like an
> identifier. TypeScript will keep an eye on them for you.

## Nested objects and methods

A property can hold another object, an array, or a function. A function stored in an
object is called a [method](../debuter/glossaire.md#method).

```typescript
interface Address {
  city: string;
  zipCode: string;
}

interface Customer {
  name: string;
  address: Address;
  orders: number[];
  greet(): string;
}

const customer: Customer = {
  name: "Chloe",
  address: { city: "Boston", zipCode: "02108" },
  orders: [25, 40],
  greet() {
    return `Hello, ${this.name}!`;
  },
};

console.log(customer.address.city);
console.log(customer.orders.length);
console.log(customer.greet());
```

```sortie
Boston
2
Hello, Chloe!
```

- `greet(): string` in the interface: a method with no parameter that returns a
  string;
- in the object, `this` refers to the object itself: `this.name` is `"Chloe"`.

## Extending an interface

`extends` creates an interface that takes all the properties of another one, and adds
some. It's like a "Student" form that takes the "Person" form and adds the grade.

```typescript
interface Person {
  name: string;
  age: number;
}

interface Student extends Person {
  grade: string;
}

const lea: Student = { name: "Lea", age: 15, grade: "10th grade" };

function introduce(p: Person): string {
  return `${p.name} (${p.age} years old)`;
}

console.log(introduce(lea));
console.log(lea.grade);
```

```sortie
Lea (15 years old)
10th grade
```

A `Student` has everything a `Person` has: so you can pass it to `introduce`.

With `type`, you get the same result with `&` ("and"):

```typescript
type Animal = { name: string };
type Dog = Animal & { breed: string };

const rex: Dog = { name: "Rex", breed: "shepherd" };
console.log(rex.name, rex.breed);
```

```sortie
Rex shepherd
```

## Index signatures

Sometimes you don't know the names of the properties in advance: grades per subject,
prices per item… An **index signature** describes "any key of this type, with a value
of this type".

```typescript
interface Grades {
  [subject: string]: number;
}

const grades: Grades = { math: 15, french: 12 };
grades.english = 14;

for (const [subject, grade] of Object.entries(grades)) {
  console.log(`${subject}: ${grade}`);
}
```

```sortie
math: 15
french: 12
english: 14
```

`[subject: string]: number` reads: "for any string key, a number value". The word
`subject` is only a name for documentation. It's the same thing as
`Record<string, number>`, seen in [arrays](collections.md#record-a-key-value-object).

> **Warning:** TypeScript announces `grades.history` as a `number`, but the key
> doesn't exist: the value is `undefined`. With an index signature, check that a key
> exists (`"history" in grades`) before you trust its value.

## Structural typing: only the shape matters

Here is an important idea, specific to TypeScript. To know whether an object fits,
TypeScript doesn't look at **its name**, but at **its shape**: its properties and
their types. This is called **structural typing**.

Take an analogy: an electrical outlet. It accepts any plug that has the right shape,
whatever the brand of the device. In the same way, a function that expects a `Point`
accepts any object that has a numeric `x` and `y`.

```typescript
interface Point {
  x: number;
  y: number;
}

function distanceFromOrigin(p: Point): number {
  return Math.sqrt(p.x ** 2 + p.y ** 2);
}

const pixel = { x: 3, y: 4, color: "red" };

console.log(distanceFromOrigin(pixel));
console.log(distanceFromOrigin({ x: 6, y: 8 }));
```

```sortie
5
10
```

`pixel` was never declared as a `Point`. It even has an extra property, `color`. But
it has `x` and `y`, which are numbers: it has the shape of a `Point`, so it's
accepted. The plug fits the outlet.

> **Note:** other languages, like Java, require you to declare explicitly that an
> object "is a" `Point`. Not TypeScript: having the right shape is enough. People
> sometimes say "if it walks like a duck and quacks like a duck, it's a duck".

## The check for extra properties

An exception to the previous rule: when you write an object **directly** where a type
is expected, TypeScript refuses extra properties.

```typescript norun
interface Book {
  title: string;
  pages: number;
}
const book: Book = { title: "Dune", pages: 600, author: "Herbert" };
```

```text
main.ts:5:49 - error TS2353: Object literal may only specify known properties, and 'author' does not exist in type 'Book'.

5 const book: Book = { title: "Dune", pages: 600, author: "Herbert" };
                                                  ~~~~~~


Found 1 error in main.ts:5
```

"An object written this way (an *object literal*) may only have known properties, and
`author` doesn't exist in `Book`."

Why the difference with `pixel`? An object written in place is only used at that
spot: an unknown property there is almost always a typo (`titles` instead of `title`)
or a property you forgot in the interface. TypeScript prefers to warn you. An object
already stored in a variable, on the other hand, can be used elsewhere with its other
properties: it's accepted.

The two possible fixes:

- remove the extra property;
- or add it to the interface, if it makes sense: `author?: string;`.

```typescript
interface Book {
  title: string;
  pages: number;
  author?: string;
}

const book: Book = { title: "Dune", pages: 600, author: "Herbert" };
console.log(`${book.title}, by ${book.author ?? "unknown author"}`);
```

```sortie
Dune, by Herbert
```

## Common mistakes

### Forgetting a required property

```typescript norun
interface Book {
  title: string;
  pages: number;
}
const book: Book = { title: "Dune" };
```

```text
main.ts:5:7 - error TS2741: Property 'pages' is missing in type '{ title: string; }' but required in type 'Book'.

5 const book: Book = { title: "Dune" };
        ~~~~

  main.ts:3:3
    3   pages: number;
        ~~~~~
    'pages' is declared here.


Found 1 error in main.ts:5
```

"The property `pages` is missing, but `Book` requires it." Add it
(`{ title: "Dune", pages: 600 }`), or make it optional in the interface
(`pages?: number`).

### Getting a property name wrong

```typescript norun
interface Book {
  title: string;
  pages: number;
}
const book: Book = { title: "Dune", pages: 600 };
console.log(book.titles);
```

```text
main.ts:6:18 - error TS2551: Property 'titles' does not exist on type 'Book'. Did you mean 'title'?

6 console.log(book.titles);
                   ~~~~~~

  main.ts:2:3
    2   title: string;
        ~~~~~
    'title' is declared here.


Found 1 error in main.ts:6
```

TypeScript knows all the properties of `Book` and suggests the right one:
`book.title`. In plain JavaScript, `book.titles` would have given `undefined` without
a word.

### Using an optional property without checking

```typescript norun
interface User {
  name: string;
  email?: string;
}
const u: User = { name: "Alice" };
console.log(u.email.toLowerCase());
```

```text
main.ts:6:13 - error TS18048: 'u.email' is possibly 'undefined'.

6 console.log(u.email.toLowerCase());
              ~~~~~~~


Found 1 error in main.ts:6
```

`email` is optional: it may be missing. Write `u.email?.toLowerCase()`, or test
`if (u.email !== undefined)` first.

### Changing a readonly property

```typescript norun
interface Point {
  readonly x: number;
  readonly y: number;
}
const p: Point = { x: 1, y: 2 };
p.x = 5;
```

```text
main.ts:6:3 - error TS2540: Cannot assign to 'x' because it is a read-only property.

6 p.x = 5;
    ~


Found 1 error in main.ts:6
```

`x` is read-only. Create a new object: `const q: Point = { ...p, x: 5 };`.
The `...p` copies all the properties of `p`, then `x: 5` replaces `x`.

## Test yourself

**Question 1:** write an interface `Movie` with a `title` (string), a `year` (number)
and an optional `rating` (number), then a movie with no rating.

> **Answer**
>
> ```typescript
> interface Movie {
>   title: string;
>   year: number;
>   rating?: number;
> }
>
> const movie: Movie = { title: "Spirited Away", year: 2001 };
> console.log(movie.rating ?? "not rated yet");
> ```
>
> ```sortie
> not rated yet
> ```

**Question 2:** a function expects a `{ name: string }`. Can you pass it a variable
`const cat = { name: "Felix", age: 3 }`?

> **Answer**
>
> Yes. Thanks to structural typing, only the shape matters: `cat` does have a string
> `name`. The extra property `age` is no problem, because the object is passed
> through a variable and not written directly in the call.

**Question 3:** what is the difference between `interface Student extends Person`
and `type Student = Person & { grade: string }`?

> **Answer**
>
> The result is the same: a `Student` has all the properties of `Person`, plus
> `grade`. `extends` is used with `interface`, `&` with `type`.

## Summary

- An object groups named values: `{ name: "Alice", age: 30 }`; you read a property
  with `object.name`.
- `interface` and `type` give a name to the shape of an object; `interface` for
  objects and `type` for the rest is a good rule to start with.
- `?` makes a property optional (check it before use), `readonly` stops it from
  changing.
- `extends` (or `&`) extends a shape; `[key: string]: number` accepts any key.
- Typing is structural: an object fits if it has the right shape, even with extra
  properties, unless it is written directly at that spot.

Next: [following a program step by step](pas-a-pas.md).
