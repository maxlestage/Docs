# Advanced types

This page explains how to build new types from existing types, with utility types, `keyof`, `typeof` and a few more powerful tools.

> **Goals:**
>
> - you will know how to use the common utility types: `Partial`, `Pick`, `Omit`, `Record`…;
> - you will know how to get a type from a value with `typeof` and keys with `keyof`;
> - you will know how to read the type of a property with `Type["key"]`;
> - you will know how to read a mapped type and a simple conditional type;
> - you will know how to build text types with template literal types.

## Why build types?

Imagine a `User` type. For the edit form, you need the same type, but with every field
optional. For the public display, you need the same type, without the password.
Copying the type three times would be fragile: the day you add a field, you would have
to remember all three copies.

TypeScript lets you **compute** a type from another one, the way a spreadsheet formula
computes a cell from another cell. When the original changes, the computed types
follow.

This whole chapter relies on this type:

```typescript norun
interface User {
  id: number;
  name: string;
  email: string;
  password: string;
}
```

## Partial and Required

`Partial<T>` makes **every** property optional. It's ideal for an update, where you
only change some fields.

```typescript
interface User {
  id: number;
  name: string;
  email: string;
  password: string;
}

function update(u: User, changes: Partial<User>): User {
  return { ...u, ...changes };
}

const alice: User = { id: 1, name: "Alice", email: "a@ex.com", password: "x" };
const updated = update(alice, { email: "alice@ex.com" });
console.log(updated.email);
```

```sortie
alice@ex.com
```

`Required<T>` does the opposite: every property becomes required.

```typescript
interface Options {
  color?: string;
  size?: number;
}

const defaults: Required<Options> = { color: "black", size: 12 };

function configure(o: Options): Required<Options> {
  return { ...defaults, ...o };
}

console.log(configure({ size: 16 }));
```

```sortie
{ color: 'black', size: 16 }
```

## Readonly

`Readonly<T>` forbids changing the properties. Useful for a configuration that must
not move.

```typescript
interface Config {
  port: number;
  host: string;
}

const config: Readonly<Config> = { port: 8080, host: "localhost" };
console.log(`${config.host}:${config.port}`);
```

```sortie
localhost:8080
```

Writing `config.port = 3000` would be refused. For arrays, there is
`ReadonlyArray<T>`, which you can also write `readonly T[]`.

## Pick and Omit

`Pick<T, Keys>` keeps only some properties; `Omit<T, Keys>` removes some. The keys are
written as a union of strings.

```typescript
interface User {
  id: number;
  name: string;
  email: string;
  password: string;
}

type Summary = Pick<User, "id" | "name">;
type Public = Omit<User, "password">;

function summarize(u: User): Summary {
  return { id: u.id, name: u.name };
}

function makePublic(u: User): Public {
  const { password, ...rest } = u;
  return rest;
}

const bob: User = { id: 2, name: "Bob", email: "b@ex.com", password: "secret" };
console.log(summarize(bob));
console.log(makePublic(bob));
```

```sortie
{ id: 2, name: 'Bob' }
{ id: 2, name: 'Bob', email: 'b@ex.com' }
```

> **Tip:** pick whichever one needs the shorter list. To keep 2 fields out of 10,
> `Pick`; to remove 1 out of 10, `Omit`.

## Record

`Record<Keys, Value>` describes an object whose keys are known and whose values all
have the same type. With a union of literals as keys, TypeScript requires **every**
key.

```typescript
type Day = "mon" | "tue" | "wed";

const hours: Record<Day, number> = { mon: 7, tue: 8, wed: 6 };

let total = 0;
for (const day of Object.keys(hours) as Day[]) {
  total += hours[day];
}
console.log(`Total: ${total} h`);

const stock: Record<string, number> = {};
stock["apples"] = 12;
stock["pears"] = 3;
console.log(stock);
```

```sortie
Total: 21 h
{ apples: 12, pears: 3 }
```

`Record<string, number>` works as a simple [dictionary](../debuter/glossaire.md#dictionnaire):
any string key, numeric values.

> **Note:** `Object.keys` always returns a `string[]`. The `as Day[]` tells
> TypeScript that these really are `Day` values, which is true here.

## typeof and keyof

You have met them in the previous chapters. As a reminder:

- `typeof value`, **in a type position**, gives the type of an existing value;
- `keyof Type` gives the union of the property names of a type.

```typescript
const settings = { volume: 5, muted: false, language: "en" };

type Settings = typeof settings;
type SettingName = keyof Settings;

function read(name: SettingName): string {
  return `${name} = ${settings[name]}`;
}

console.log(read("volume"));
console.log(read("language"));
```

```sortie
volume = 5
language = en
```

Here, `Settings` is `{ volume: number; muted: boolean; language: string }` and
`SettingName` is `"volume" | "muted" | "language"`. You didn't write any type by
hand: they follow the object.

## Indexed access types

`Type["key"]` gives the type of **one** property. It's the same notation as for reading
a value in an object, but applied to types.

```typescript
interface Order {
  number: number;
  customer: { name: string; city: string };
  lines: { product: string; quantity: number }[];
}

type Customer = Order["customer"];
type Line = Order["lines"][number];

const c: Customer = { name: "Lea", city: "Leeds" };
const l: Line = { product: "Notebook", quantity: 3 };
console.log(`${c.name} orders ${l.quantity} × ${l.product}`);
```

```sortie
Lea orders 3 × Notebook
```

`[number]` after an array type gives the type of one element. No need to declare
`Customer` and `Line` separately: you extract them.

## ReturnType, Parameters and Awaited

These three utilities read the type of a **function**:

- `ReturnType<F>`: the type of what it returns;
- `Parameters<F>`: the types of its parameters, as a tuple;
- `Awaited<P>`: the type you get after `await` on a promise.

```typescript
function createPlayer(name: string, level: number) {
  return { name, level, lives: 3 };
}

type Player = ReturnType<typeof createPlayer>;
type Arguments = Parameters<typeof createPlayer>;

const args: Arguments = ["Sam", 2];
const p: Player = createPlayer(...args);
console.log(p);

async function loadScore(): Promise<number> {
  return 1200;
}

type Score = Awaited<ReturnType<typeof loadScore>>;
const s: Score = await loadScore();
console.log(s + 1);
```

```sortie
{ name: 'Sam', level: 2, lives: 3 }
1201
```

`Player` is `{ name: string; level: number; lives: number }`, without you writing it.
If the function changes, the type follows.

## NonNullable

`NonNullable<T>` removes `null` and `undefined` from a union.

```typescript
type MaybeMissing = string | null | undefined;
type Present = NonNullable<MaybeMissing>;

const names: MaybeMissing[] = ["Ana", null, "Leo", undefined];
const present: Present[] = names.filter((n): n is Present => n != null);
console.log(present);
```

```sortie
[ 'Ana', 'Leo' ]
```

`n != null` (with `!=`, not `!==`) removes both `null` and `undefined`. The type guard
`n is Present` comes from the chapter on [unions](unions.md#your-own-type-guards).

## Mapped types

How is `Partial` written? With a **mapped type**. It goes through the keys of a type,
like a `for` loop, and builds a new type.

```typescript
type Flags<T> = {
  [K in keyof T]: boolean;
};

interface Form {
  name: string;
  age: number;
  email: string;
}

const changed: Flags<Form> = { name: true, age: false, email: false };
const fields = Object.entries(changed)
  .filter(([, v]) => v)
  .map(([k]) => k);
console.log(`Changed fields: ${fields.join(", ")}`);
```

```sortie
Changed fields: name
```

`[K in keyof T]: boolean` reads: "for each key `K` of `T`, a property of type
`boolean`". The result has the same keys as `Form`, with booleans. You can also add
`?` or `readonly`: `Partial<T>` is simply `{ [K in keyof T]?: T[K] }`.

## Conditional types

A **conditional type** picks a type based on a condition. It looks like the ternary
operator `a ? b : c`, but for types: `T extends U ? Yes : No`.

```typescript
type Description<T> = T extends string ? "text" : T extends number ? "number" : "other";

function describe<T>(value: T, description: Description<T>): string {
  return `${String(value)}: ${description}`;
}

console.log(describe("abc", "text"));
console.log(describe(42, "number"));
console.log(describe(true, "other"));
```

```sortie
abc: text
42: number
true: other
```

Here, `describe(42, "text")` would be refused: for a number, `Description<number>` is
`"number"`. `NonNullable` and `Awaited` are written with conditional types.

> **Note:** you will often read conditional types in libraries. You will rarely write
> them at first: utility types are enough in most cases.

## Template literal types

Template strings `` `Hello ${name}` `` also exist for types. Combined with unions, they
produce every combination.

```typescript
type Size = "small" | "large";
type Color = "red" | "blue";
type Variant = `${Size}-${Color}`;

const v1: Variant = "small-red";
const v2: Variant = "large-blue";
console.log(v1, v2);

type Event = "click" | "hover";
type Handler = `on${Capitalize<Event>}`;

const handlers: Record<Handler, () => string> = {
  onClick: () => "clicked",
  onHover: () => "hovered",
};
console.log(handlers.onClick());
```

```sortie
small-red large-blue
clicked
```

`Variant` is `"small-red" | "small-blue" | "large-red" | "large-blue"`. `Capitalize`
turns the first letter into a capital: `Handler` is `"onClick" | "onHover"`. There are
also `Uppercase`, `Lowercase` and `Uncapitalize`.

## Common mistakes

### Reading a property removed by Omit

```typescript norun
type User = { name: string; email: string; password: string };
type Public = Omit<User, "password">;
function display(u: Public) {
  console.log(u.password);
}
```

```text
main.ts(4,17): error TS2339: Property 'password' does not exist on type 'Public'.
```

That's the whole point of `Omit`: this field no longer exists. If you need it here, use
the full `User` type.

### Forgetting a key in a Record

```typescript norun
type Day = "mon" | "tue" | "wed";
const hours: Record<Day, number> = { mon: 7, tue: 8 };
```

```text
main.ts(2,7): error TS2741: Property 'wed' is missing in type '{ mon: number; tue: number; }' but required in type 'Record<Day, number>'.
```

With literal keys, every key is required. Add `wed`, or use
`Partial<Record<Day, number>>` if some can be missing.

### Writing Type.key instead of Type["key"]

```typescript norun
type Product = { name: string; price: number };
type Price = Product.price;
```

```text
main.ts(2,14): error TS2713: Cannot access 'Product.price' because 'Product' is a type, but not a namespace. Did you mean to retrieve the type of the property 'price' in 'Product' with 'Product["price"]'?
```

For types, the dot doesn't work. Use brackets: `Product["price"]`. The message even
suggests the fix.

## Test yourself

**Question 1:** which utility type should you pick for an update function that only
receives the fields to change?

> **Answer**
>
> `Partial<T>`. Every property becomes optional: you can send `{ email: "…" }` on its
> own.

**Question 2:** what is `keyof { x: number; y: number }`?

> **Answer**
>
> The union of the keys: `"x" | "y"`.

**Question 3:** from `interface Item { title: string; price: number; stock: number }`,
create a `Label` type that contains only `title` and `price`, and use it.

> **Answer**
>
> ```typescript
> interface Item {
>   title: string;
>   price: number;
>   stock: number;
> }
>
> type Label = Pick<Item, "title" | "price">;
>
> const e: Label = { title: "Eraser", price: 1.5 };
> console.log(`${e.title}: ${e.price} €`);
> ```
>
> ```sortie
> Eraser: 1.5 €
> ```
>
> `Omit<Item, "stock">` gives the same result.

## Summary

- Utility types compute a type from another one: when the original changes, they
  follow.
- `Partial`, `Required`, `Readonly` change every property; `Pick` and `Omit` choose
  some; `Record` creates a key → value object.
- `typeof` gives the type of a value, `keyof` the union of the keys, `T["key"]` the type
  of a property.
- `ReturnType`, `Parameters`, `Awaited` and `NonNullable` read functions and unions.
- Mapped types (`[K in keyof T]`), conditional types (`A extends B ? X : Y`) and
  template literal types (`` `on${Name}` ``) are the building blocks of utility types.

Next: [decorators](decorateurs.md)
