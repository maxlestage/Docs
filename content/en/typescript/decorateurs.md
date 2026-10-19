# Decorators

A decorator is a function that "wraps" a method, a field or an accessor of a class to add a behavior to it, without touching its code.

> **Goals:**
>
> - you will know what a [decorator](../debuter/glossaire.md#decorator) is and how to
>   read the `@name` syntax;
> - you will know how to write a method decorator, for example to print each call;
> - you will know how to decorate a field and an accessor created with `accessor`;
> - you will know how to write a decorator factory that takes parameters;
> - you will know how to read the context object that TypeScript gives the decorator.

This page uses the **standard** decorators of TypeScript 5. You need to know
[classes](types.md) before reading it.

## A decorator is wrapping paper

Picture a gift. You don't change the object inside: you wrap it. The paper adds
something **around** it (a ribbon, a tag), and the object stays the same inside.

A decorator does the same thing with a method:

- it **receives** the original method (the gift);
- it **returns** a new function that calls the original one (the wrapped gift);
- the class then uses the wrapped version.

```text
   without a decorator          with @wrap

   open()                       open()
     │                            │
     ▼                            ▼
   ┌────────────┐               ┌──────────────────────┐
   │ "A bike!"  │               │ ***  ┌────────────┐  │
   └────────────┘               │      │ "A bike!"  │  │
                                │      └────────────┘ ***
                                └──────────────────────┘
```

You write `@` followed by the name of the function, right above what you decorate:

```typescript
function wrap(method: () => string, context: ClassMethodDecoratorContext) {
  return function (this: unknown): string {
    return "*** " + method.call(this) + " ***";
  };
}

class Gift {
  constructor(public content: string) {}

  @wrap
  open(): string {
    return `A ${this.content}!`;
  }
}

const gift = new Gift("bike");
console.log(gift.open());
```

```sortie
*** A bike! ***
```

Let's read it slowly:

- `wrap` is an ordinary function. It receives two things: the original method
  (`method`) and a `context` object (more on that below);
- it returns a **new** function. This function calls `method` and adds stars around the
  result;
- `method.call(this)` calls the original method with the right object: `this` is the
  gift on which `open()` was called;
- `@wrap` says: "replace `open` with what `wrap` returns".

Try it: remove the `@wrap` line. The program prints `A bike!`, without stars. The code
of `open` hasn't changed: only the wrapping is gone.

> **Remember:** a decorator is a plain function. It receives the original method and
> returns a function that replaces it.

## When the decorator runs

There are two different moments, and that is often what confuses people at first:

1. the `wrap` function runs **only once**, when the class is created;
2. the function it returns runs **on every call** of the method.

Let's add messages to see it:

```typescript
function wrap(method: () => string, context: ClassMethodDecoratorContext) {
  console.log(`Wrapping ${String(context.name)}`);
  return function (this: unknown): string {
    console.log("  unwrapping…");
    return "*** " + method.call(this) + " ***";
  };
}

class Gift {
  constructor(public content: string) {}

  @wrap
  open(): string {
    return `A ${this.content}!`;
  }
}

console.log("Start of the program");
console.log(new Gift("bike").open());
console.log(new Gift("book").open());
```

```sortie
Wrapping open
Start of the program
  unwrapping…
*** A bike! ***
  unwrapping…
*** A book! ***
```

Let's follow the program step by step:

| Step | What happens | Output |
|------|--------------|--------|
| 1 | the class `Gift` is created: `wrap` is called for `open` | `Wrapping open` |
| 2 | the line `console.log("Start…")` | `Start of the program` |
| 3 | `open()` on the bike: the wrapped function runs | `  unwrapping…` |
| 4 | it calls the original, which returns `"A bike!"`, then adds the stars | `*** A bike! ***` |
| 5 | `open()` on the book: same thing, without calling `wrap` again | `  unwrapping…` then `*** A book! ***` |

`Wrapping open` is printed **before** `Start of the program`: the decorator works as soon
as the class is read, before a single gift even exists.

Try it: create three gifts instead of two. `Wrapping open` is still printed only once;
`unwrapping…` is printed three times.

> **Remember:** the decorator wraps once, when the class is created. The wrapped
> function runs on every call.

## Logging calls

A classic use: printing each call of a method, with its arguments and its result. This
is **logging**. Handy to understand a program without adding `console.log` everywhere.

For this decorator to work on **any** method, we make it [generic](generiques.md):
`This` is the type of the object, `Args` the list of parameters, `R` the type of the
result. TypeScript infers them on its own.

```typescript
function logCalls<This, Args extends unknown[], R>(
  method: (this: This, ...args: Args) => R,
  context: ClassMethodDecoratorContext<This, (this: This, ...args: Args) => R>,
) {
  const name = String(context.name);
  return function (this: This, ...args: Args): R {
    console.log(`-> ${name}(${args.join(", ")})`);
    const result = method.call(this, ...args);
    console.log(`<- ${name} returns ${result}`);
    return result;
  };
}

class CashRegister {
  total = 0;

  @logCalls
  add(price: number, quantity: number): number {
    this.total += price * quantity;
    return this.total;
  }
}

const register = new CashRegister();
register.add(3, 2);
register.add(10, 1);
console.log(`Final total: ${register.total}`);
```

```sortie
-> add(3, 2)
<- add returns 6
-> add(10, 1)
<- add returns 16
Final total: 16
```

The picture of one call:

```text
   register.add(3, 2)
          │
   ┌──────▼──────────────────────────┐
   │ wrapping (logCalls)             │  prints "-> add(3, 2)"
   │   ┌───────────────────────────┐ │
   │   │ original add              │ │  total = 0 + 3 * 2 = 6
   │   └───────────────────────────┘ │
   │                                 │  prints "<- add returns 6"
   └──────┬──────────────────────────┘
          ▼
          6
```

And how `total` changes:

| Call | `price` | `quantity` | `total` before | `total` after | Result |
|------|---------|------------|----------------|---------------|--------|
| 1 | 3 | 2 | 0 | 6 | 6 |
| 2 | 10 | 1 | 6 | 16 | 16 |

Thanks to the generic types, the wrapped function keeps exactly the shape of the
original: `register.add("three", 2)` is still a type error.

Try it: add a method `clear(): number` that sets `total` back to 0 and returns 0,
decorate it with `@logCalls`, and call it. You see `-> clear()` then
`<- clear returns 0`: the same decorator works for every method.

> **Remember:** a single generic decorator can wrap every method. It adds a common
> behavior without copying code.

## The context object

The second parameter of a decorator is the **context**: a card that describes what is
being decorated. It's like the label stuck on a parcel: it tells you what's inside,
without opening it.

| Field | What it holds |
|-------|---------------|
| `kind` | the kind of element: `"method"`, `"field"`, `"accessor"`, `"getter"`, `"setter"` or `"class"` |
| `name` | the name of the element (`"add"`) |
| `static` | `true` if the element is `static` |
| `private` | `true` if its name starts with `#` |
| `addInitializer` | a function to run code when each object is created |

A decorator can decorate anything, if it accepts any context. Here is a "spy" that just
prints the card:

```typescript
function spy(_value: unknown, context: DecoratorContext) {
  const details = "static" in context ? ` static=${context.static} private=${context.private}` : "";
  console.log(`${context.kind} ${String(context.name)}${details}`);
}

@spy
class Account {
  @spy balance = 0;
  @spy static bank = "Corner Bank";
  @spy #code = 1234;

  @spy
  deposit(amount: number) {
    this.balance += amount;
  }
}

console.log(new Account().balance);
```

```sortie
method deposit static=false private=false
field bank static=true private=false
field balance static=false private=false
field #code static=false private=true
class Account
0
```

Notice the order: methods first, then `static` fields, then the other fields, and the
class last.
`"static" in context` checks that the card has a `static` field: a class card doesn't
have one, and TypeScript knows it (this is
[narrowing](../debuter/glossaire.md#narrowing)).

### addInitializer: acting on each object

`addInitializer` records a function that runs **when each object is created**. A useful
example: "binding" a method to its object.

First, the problem. When you take a method out of its object (`const press =
d.ring;`, to hand it to someone else), it **loses its `this`**: calling `press()` no
longer knows which doorbell it belongs to. It's like unhooking the button of a doorbell:
it's no longer connected to any door.

The `bound` decorator fixes this: for each new object, it stores in the object a copy of
the method attached to that object for good (`bind`).

```typescript
function bound<This extends object, Args extends unknown[], R>(
  method: (this: This, ...args: Args) => R,
  context: ClassMethodDecoratorContext<This, (this: This, ...args: Args) => R>,
) {
  context.addInitializer(function (this: This) {
    Reflect.set(this, context.name, method.bind(this));
  });
}

class Doorbell {
  constructor(public sound: string) {}

  @bound
  ring() {
    return this?.sound ?? "(this was lost)";
  }
}

const d = new Doorbell("Ding dong");
const press = d.ring;
console.log(press());
```

```sortie
Ding dong
```

`Reflect.set(this, name, value)` writes `this[name] = value` without TypeScript
complaining about the unknown name.

Try it: remove the `@bound` line. The program prints `(this was lost)`.

> **Remember:** the context describes the decorated element (`kind`, `name`, `static`,
> `private`). `addInitializer` runs code for each object created.

## Decorating a field

A **field** has no function to wrap: it has a **starting value**. A field decorator
therefore receives `undefined` as its first parameter, and it can return a function that
**transforms the starting value**.

It's like a stamp at the entrance of an office: each file that comes in gets the stamp,
once, before being filed.

```typescript
function upperCase(_value: undefined, context: ClassFieldDecoratorContext<unknown, string>) {
  return function (startValue: string): string {
    console.log(`  ${String(context.name)}: "${startValue}" becomes "${startValue.toUpperCase()}"`);
    return startValue.toUpperCase();
  };
}

class City {
  @upperCase country = "france";
  @upperCase continent = "europe";
  name = "lyon";
}

console.log("Creating");
const city = new City();
console.log(city.name, city.country, city.continent);
city.country = "italy";
console.log(city.country);
```

```sortie
Creating
  country: "france" becomes "FRANCE"
  continent: "europe" becomes "EUROPE"
lyon FRANCE EUROPE
italy
```

| Step | `country` | `continent` | `name` |
|------|-----------|-------------|--------|
| `new City()` starts | (empty) | (empty) | (empty) |
| field `country`: `"france"` goes through the decorator | `"FRANCE"` | (empty) | (empty) |
| field `continent`: `"europe"` goes through the decorator | `"FRANCE"` | `"EUROPE"` | (empty) |
| field `name`: no decorator | `"FRANCE"` | `"EUROPE"` | `"lyon"` |
| `city.country = "italy"` | `"italy"` | `"EUROPE"` | `"lyon"` |

The last line shows the limit: `"italy"` is **not** put in upper case. To watch every
change, you need an accessor (next section).

> **Warning:** the decorator sees **only the starting value**, the one written after
> `=`. A field without a starting value (filled later in the constructor) arrives as
> `undefined`, and `undefined.toUpperCase()` crashes the program. So decorate a field
> that has a value after `=`.

> **Remember:** a field decorator only transforms the starting value, once per object.

## Decorating an accessor with accessor

The `accessor` keyword in front of a field automatically creates a hidden `get` / `set`
pair (see [accessors](types.md)). From the outside, you use it like a normal field. But
a decorator can then **intercept every read and every write**.

It's like a counter in front of a safe: you no longer touch the safe directly, you go
through the clerk, who can check what you deposit.

```text
   player.health = 150
          │
   ┌──────▼──────────────┐
   │ counter (set)       │  150 is too big: keep 100
   └──────┬──────────────┘
          ▼
   ┌─────────────────────┐
   │ hidden safe: 100    │
   └─────────────────────┘
```

The decorator receives `target`, which holds the original `get` and `set`. It returns an
object with the new versions (`get`, `set`, and `init` for the starting value; each one
is optional).

```typescript
function between0and100<This>(
  target: ClassAccessorDecoratorTarget<This, number>,
  context: ClassAccessorDecoratorContext<This, number>,
): ClassAccessorDecoratorResult<This, number> {
  const clamp = (v: number) => Math.min(100, Math.max(0, v));
  return {
    init(start: number): number {
      return clamp(start);
    },
    set(this: This, v: number) {
      console.log(`  ${String(context.name)}: asked for ${v}, keeping ${clamp(v)}`);
      target.set.call(this, clamp(v));
    },
  };
}

class Player {
  @between0and100 accessor health = 150;
}

const player = new Player();
console.log(player.health);
player.health = 40;
player.health = -10;
console.log(player.health);
```

```sortie
100
  health: asked for 40, keeping 40
  health: asked for -10, keeping 0
0
```

| Step | We write | The counter keeps | `health` |
|------|----------|-------------------|----------|
| creation (`init`) | 150 | 100 | 100 |
| `player.health = 40` | 40 | 40 | 40 |
| `player.health = -10` | -10 | 0 | 0 |

Try it: remove the word `accessor`. TypeScript rejects the program: an accessor
decorator doesn't apply to a plain field (see the
[common mistakes](#common-mistakes)).

> **Remember:** `accessor` turns a field into a `get` / `set` pair. Its decorator sees
> every write, not only the starting value.

## Decorator factories

`between0and100` is stuck on 0 and 100. What if you want 0 to 10 for another field? You
write a **factory**: a function that takes parameters and **returns** a decorator.

It's like an adjustable stamp machine: you first choose the text of the stamp
(`"URGENT"`, `"PAID"`), then you stamp.

```text
   prefix("[till]")       ->  a decorator set to "[till]"
   @prefix("[till]")      ->  this decorator wraps the method
```

```typescript
function prefix(label: string) {
  return function <This, Args extends unknown[]>(
    method: (this: This, ...args: Args) => string,
    _context: ClassMethodDecoratorContext<This, (this: This, ...args: Args) => string>,
  ) {
    return function (this: This, ...args: Args): string {
      return `${label} ${method.call(this, ...args)}`;
    };
  };
}

class Shop {
  @prefix("[till]")
  pay(amount: number): string {
    return `${amount} € paid`;
  }

  @prefix("[stock]")
  store(item: string): string {
    return `${item} stored`;
  }
}

const shop = new Shop();
console.log(shop.pay(12));
console.log(shop.store("soap"));
```

```sortie
[till] 12 € paid
[stock] soap stored
```

Notice the **parentheses**: `@prefix("[till]")` first calls `prefix`, which returns the
real decorator. Without parentheses, TypeScript would think that `prefix` itself is the
decorator (see the [common mistakes](#common-mistakes)).

> **Remember:** a factory is a function that returns a decorator. You write it with
> parentheses: `@limit(0, 100)`.

## What the compiler does

Node 22 doesn't know the `@` syntax yet. With the `es2022` target (the one used by the
examples on this site), `tsc` therefore **rewrites** each decorator into ordinary
JavaScript: helper functions named `__esDecorate` and `__runInitializers` call your
decorators at the right moment. That's why all the examples on this page run in
Node 22.

You have nothing to configure: without any particular option, TypeScript 5 uses these
**standard** decorators.

> **Warning:** many articles and libraries (Angular, NestJS, TypeORM) use the **old**
> decorators, turned on by the option `"experimentalDecorators": true` in
> `tsconfig.json`. They receive other parameters (`target, key, descriptor`) and don't
> mix with the ones on this page. If a project already uses this option, follow the
> documentation of its library.

> **Remember:** standard decorators work without any option; `tsc` translates them into
> ordinary functions for Node.

## Common mistakes

### Writing an old-style decorator

An example copied from an old tutorial expects three parameters:

```typescript norun
function logCalls(target: any, key: string, descriptor: PropertyDescriptor) {
  return descriptor;
}

class Calc {
  @logCalls
  add(a: number, b: number): number {
    return a + b;
  }
}
```

```text
main.ts(6,3): error TS1241: Unable to resolve signature of method decorator when called as an expression.
  The runtime will invoke the decorator with 2 arguments, but the decorator expects 3.
main.ts(6,4): error TS1270: Decorator function return type 'PropertyDescriptor' is not assignable to type 'void | ((a: number, b: number) => number)'.
```

A standard decorator receives **two** parameters: the method and the context. It returns
a function (or nothing). Use the model of `logCalls` seen above.

### Forgetting the parentheses of a factory

```typescript norun
function limit(min: number, max: number) {
  return function (
    target: ClassAccessorDecoratorTarget<unknown, number>,
    context: ClassAccessorDecoratorContext<unknown, number>,
  ) {
    return target;
  };
}

class Player {
  @limit accessor health = 100;
}
```

```text
main.ts(11,4): error TS1240: Unable to resolve signature of property decorator when called as an expression.
  Argument of type 'ClassAccessorDecoratorTarget<Player, number>' is not assignable to parameter of type 'number'.
main.ts(11,4): error TS1270: Decorator function return type '(target: ClassAccessorDecoratorTarget<unknown, number>, context: ClassAccessorDecoratorContext<unknown, number>) => ClassAccessorDecoratorTarget<unknown, number>' is not assignable to type 'void | ClassAccessorDecoratorResult<Player, number>'.
```

`limit` expects `min` and `max`, not the target. Call the factory:
`@limit(0, 100) accessor health = 100;`.

### Forgetting the accessor keyword

```typescript norun
function limit<This>(
  target: ClassAccessorDecoratorTarget<This, number>,
  context: ClassAccessorDecoratorContext<This, number>,
) {
  return target;
}

class Player {
  @limit health = 100;
}
```

```text
main.ts(9,4): error TS1240: Unable to resolve signature of property decorator when called as an expression.
  Argument of type 'undefined' is not assignable to parameter of type 'ClassAccessorDecoratorTarget<Player, number>'.
main.ts(9,4): error TS1270: Decorator function return type 'ClassAccessorDecoratorTarget<Player, number>' is not assignable to type 'void | ((this: Player, value: number) => number)'.
```

Without `accessor`, `health` is a plain field: the decorator receives `undefined`, not a
target. Write `@limit accessor health = 100;`.

## Test yourself

**Question 1:** in what order are `A` and `B` printed?

```typescript
function mark(_m: () => void, _c: ClassMethodDecoratorContext) {
  console.log("A");
}

class Door {
  @mark
  open() {}
}

console.log("B");
new Door().open();
```

> **Answer**
>
> `A` then `B`. The decorator runs once, when the class is created, before the line
> `console.log("B")`. Since it returns nothing, `open` stays the original method.
>
> ```sortie
> A
> B
> ```

**Question 2:** a field decorator turns `"paris"` into `"PARIS"`. What is the value of
the field after `city.name = "nice"`?

> **Answer**
>
> `"nice"`, in lower case. A field decorator only transforms the starting value. To
> watch every write, use `accessor` and an accessor decorator.

**Question 3:** write a factory `repeat(n)` that wraps a method returning a text and
returns that text repeated `n` times.

> **Answer**
>
> ```typescript
> function repeat(n: number) {
>   return function <This>(
>     method: (this: This) => string,
>     _context: ClassMethodDecoratorContext<This, (this: This) => string>,
>   ) {
>     return function (this: This): string {
>       return method.call(this).repeat(n);
>     };
>   };
> }
>
> class Echo {
>   @repeat(3)
>   shout(): string {
>     return "ho! ";
>   }
> }
>
> console.log(new Echo().shout());
> ```
>
> ```sortie
> ho! ho! ho!
> ```

## Summary

- A decorator is a function placed with `@name` above a class member: it wraps it to add
  a behavior to it.
- A method decorator receives the method and the context, and returns the function that
  replaces it.
- It runs once when the class is created; the wrapped function, on every call.
- A field decorator only transforms the starting value; with `accessor`, you intercept
  every read and write.
- A factory takes parameters and returns a decorator: `@limit(0, 100)`.
- Standard decorators work without any option; `experimentalDecorators` is the old,
  different system.

Next: [handling errors](erreurs.md).
