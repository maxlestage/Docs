# Literal types, enums and as const

This page explains how to limit a value to a short list of precise choices, with literal types, enums and `as const`.

> **Goals:**
>
> - you will know how to write a literal type and a union of literals;
> - you will know why `let` and `const` don't give the same type;
> - you will know how to create an `enum` and compare it with a union;
> - you will know how to freeze an object with `as const` and derive a type from it;
> - you will know how to check a value without losing its precision with `satisfies`.

## A literal type

Until now, a type stood for a big family of values: `string` accepts every text. A
**literal type** accepts **only one** value. The type `"red"` accepts the text `"red"`,
and nothing else.

On its own, it isn't very useful. But in a [union](unions.md), it becomes a list of
choices, like a drop-down menu:

```typescript
type Color = "red" | "green" | "blue";

function paint(c: Color) {
  console.log(`I paint in ${c}.`);
}

paint("green");
paint("blue");
```

```sortie
I paint in green.
I paint in blue.
```

Writing `paint("pink")` is refused: `"pink"` isn't on the menu. Typos (`"rde"`) are
caught the same way. And your editor suggests the three choices as you type.

Numbers and booleans have their literals too:

```typescript
type Die = 1 | 2 | 3 | 4 | 5 | 6;
type Yes = true;

const roll: Die = 4;
const accepted: Yes = true;
console.log(roll, accepted);
```

```sortie
4 true
```

## let, const and widening

TypeScript doesn't give the same type to `let` and to `const`.

- `const x = "red"` will never change: its type is the literal `"red"`.
- `let y = "red"` may change: TypeScript **widens** its type to `string`.

```typescript
type Color = "red" | "green" | "blue";

function paint(c: Color) {
  console.log(`I paint in ${c}.`);
}

const fixed = "red";
paint(fixed);

let changing: Color = "red";
changing = "blue";
paint(changing);
```

```sortie
I paint in red.
I paint in blue.
```

With `let`, state the type yourself (`let changing: Color`). Without it, the variable
would be a `string`, and `paint(changing)` would be refused (see the
[common mistakes](#common-mistakes)).

## Unions of literals in an object

Literals shine in objects. An order status has a few possible values, not an infinite
number:

```typescript
type Status = "pending" | "shipped" | "delivered";

interface Order {
  number: number;
  status: Status;
}

function message(o: Order): string {
  switch (o.status) {
    case "pending":
      return `Order ${o.number}: we are preparing it.`;
    case "shipped":
      return `Order ${o.number}: it is on its way.`;
    case "delivered":
      return `Order ${o.number}: enjoy!`;
  }
}

console.log(message({ number: 12, status: "shipped" }));
```

```sortie
Order 12: it is on its way.
```

It's the same idea as the `kind` field of
[discriminated unions](unions.md#discriminated-unions).

## Enums

An `enum` (an [enumeration](../debuter/glossaire.md#enumeration)) gives a name to each
choice in a list. It exists in many languages, and TypeScript has it too.

```typescript
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

function move(d: Direction): string {
  if (d === Direction.Up) return "Going up";
  if (d === Direction.Down) return "Going down";
  return "Turning";
}

console.log(move(Direction.Up));
console.log(Direction.Left);
console.log(Direction[Direction.Left]);
```

```sortie
Going up
2
Left
```

By default, the members are worth `0`, `1`, `2`, `3`… in order. So `Direction.Left`
is `2`. That doesn't say much when printed. You can give text values:

```typescript
enum Level {
  Easy = "easy",
  Medium = "medium",
  Hard = "hard",
}

const choice = Level.Medium;
console.log(`Chosen level: ${choice}`);
console.log(Object.values(Level));
```

```sortie
Chosen level: medium
[ 'easy', 'medium', 'hard' ]
```

> **Note:** an `enum` isn't only a type: it creates a real JavaScript object at
> [compilation](../debuter/glossaire.md#compilation) time. Here, that's no problem,
> because we compile with `tsc`. But some tools that simply "erase" the types (like
> running `.ts` files directly with Node) refuse enums.

### Enum or union of literals?

Many TypeScript projects prefer unions of literals:

| | Union of literals | `enum` |
|---|---|---|
| Writing a value | `"easy"` | `Level.Easy` |
| Exists at run time | no (nothing is generated) | yes (an object) |
| List of all the values | no, except with `as const` | yes, with `Object.values` |
| Works everywhere | yes | you must compile |

As a beginner, choose the union of literals. If you need the list of values at run
time, use the next technique.

## Freezing an object with as const

An ordinary object can change, so TypeScript widens its values (`"easy"` becomes
`string`). `as const` freezes the whole object:

- all the properties become `readonly`;
- all the values keep their literal type.

```typescript
const LEVELS = {
  EASY: "easy",
  MEDIUM: "medium",
  HARD: "hard",
} as const;

console.log(LEVELS.MEDIUM);
console.log(Object.values(LEVELS).join(" < "));
```

```sortie
medium
easy < medium < hard
```

Without `as const`, `LEVELS.MEDIUM` would have the type `string`. With it, its type is
exactly `"medium"`. It works for an array too:
`const DAYS = ["mon", "tue"] as const` has the type `readonly ["mon", "tue"]`.

### Deriving a type from a frozen object

Here is the recipe that replaces an `enum`. You write the values **only once** in the
object, then you derive the type from it:

```typescript
const LEVELS = {
  EASY: "easy",
  MEDIUM: "medium",
  HARD: "hard",
} as const;

type Level = (typeof LEVELS)[keyof typeof LEVELS];

function points(l: Level): number {
  switch (l) {
    case "easy":
      return 1;
    case "medium":
      return 2;
    case "hard":
      return 3;
  }
}

console.log(points(LEVELS.HARD));
console.log(points("medium"));
```

```sortie
3
2
```

Let's read `(typeof LEVELS)[keyof typeof LEVELS]` from the inside out:

1. `typeof LEVELS`: the type of the object,
   `{ readonly EASY: "easy"; readonly MEDIUM: "medium"; … }`;
2. `keyof typeof LEVELS`: its keys, `"EASY" | "MEDIUM" | "HARD"`;
3. `(typeof LEVELS)[…]`: the type of the values for these keys,
   `"easy" | "medium" | "hard"`.

It's long to write, but you write it once. These operators are explained in detail in
[advanced types](types-avances.md).

For a frozen array, the recipe is shorter:

```typescript
const DAYS = ["mon", "tue", "wed"] as const;
type Day = (typeof DAYS)[number];

const d: Day = "tue";
console.log(DAYS.indexOf(d));
```

```sortie
1
```

`[number]` means "the type of any element of the array".

## Checking without widening with satisfies

When you annotate a variable (`const x: Type = …`), TypeScript checks the value, but
then it **forgets** the details: it only remembers `Type`. The `satisfies` operator
checks the value **and** keeps its precise type.

```typescript
type Color = "red" | "green" | "blue";

const theme = {
  background: "blue",
  text: "red",
} satisfies Record<string, Color>;

const accent: Color = theme.background;
console.log(accent, theme.text);
```

```sortie
blue red
```

`Record<string, Color>` means "an object whose values are `Color`s". Thanks to
`satisfies`:

- a typo like `background: "bluee"` would be caught;
- `theme.background` keeps a precise type, and TypeScript knows that `theme.text`
  exists.

With the annotation `const theme: Record<string, Color>`, TypeScript would accept
`theme.title` (any text key), which would be `undefined` at run time.

> **Tip:** combine both: `{ … } as const satisfies Type` freezes the object and checks
> its shape.

## Common mistakes

### Passing a let variable to a union of literals

```typescript norun
type Color = "red" | "green" | "blue";
function paint(c: Color) {
  console.log(`I paint in ${c}`);
}
let choice = "red";
paint(choice);
```

```text
main.ts(6,7): error TS2345: Argument of type 'string' is not assignable to parameter of type 'Color'.
```

`let` widens the type to `string`. State the type: `let choice: Color = "red";`, or use
`const` if the value doesn't change.

### A value that isn't on the menu

```typescript norun
type Color = "red" | "green" | "blue";
const c: Color = "pink";
```

```text
main.ts(2,7): error TS2322: Type '"pink"' is not assignable to type 'Color'.
```

`"pink"` isn't in the list. Choose an allowed value, or add `"pink"` to the `Color`
type.

### Changing a frozen object

```typescript norun
const CONFIG = { port: 8080, host: "localhost" } as const;
CONFIG.port = 3000;
```

```text
main.ts(2,8): error TS2540: Cannot assign to 'port' because it is a read-only property.
```

`as const` makes everything read-only. If the value must change, remove `as const` or
create a new object: `const config2 = { ...CONFIG, port: 3000 };`.

### Giving a text instead of an enum

```typescript norun
enum Direction {
  Up,
  Down,
}
const d: Direction = "Up";
```

```text
main.ts(5,7): error TS2322: Type '"Up"' is not assignable to type 'Direction'.
```

An enum value is written with the name of the enum: `Direction.Up`.

## Test yourself

**Question 1:** what is the type of `a`? And the type of `b`?

```typescript norun
const a = "cat";
let b = "cat";
```

> **Answer**
>
> `a` has the literal type `"cat"`, because a constant doesn't change. `b` has the type
> `string`, because a `let` can receive another text later.

**Question 2:** what does `as const` do to an object?

> **Answer**
>
> It makes all its properties `readonly` and keeps the literal type of each value
> (`"easy"` instead of `string`). You can then derive a union from it with
> `(typeof OBJECT)[keyof typeof OBJECT]`.

**Question 3:** starting from `const SIZES = ["S", "M", "L"] as const;`, write a type
`Size` and a function that prints a size.

> **Answer**
>
> ```typescript
> const SIZES = ["S", "M", "L"] as const;
> type Size = (typeof SIZES)[number];
>
> function show(s: Size) {
>   console.log(`Size ${s}`);
> }
>
> show("M");
> ```
>
> ```sortie
> Size M
> ```

## Summary

- A literal type accepts only one value; a union of literals makes a menu of choices.
- `const` keeps the literal type, `let` widens it: annotate your `let`s.
- An `enum` names choices and exists at run time; it requires compilation.
- `as const` freezes an object; `(typeof X)[keyof typeof X]` derives the union of its
  values.
- `satisfies` checks a value without losing its precise type.

Next: [advanced types](types-avances.md)
