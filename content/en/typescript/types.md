# Classes

This page explains how to create your own kinds of objects with `class`, protect their data and connect them to interfaces.

> **Goals:**
>
> - you will know how to write a class with typed fields and a constructor;
> - you will know how to shorten a constructor with parameter properties;
> - you will know how to protect data with `private`, `protected`, `readonly` and `#`;
> - you will know how a class keeps the promise of an interface with `implements`;
> - you will know how to use `abstract`, `static` and the `get` / `set` accessors.

## A first class

A [class](../debuter/glossaire.md#class) is a mold for objects. The mold describes the
shape; each object made with this mold is an **instance**. It's like a cookie cutter:
a single tool, as many cookies as you want.

```typescript
class Cat {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }

  introduce(): string {
    return `I am ${this.name}, I am ${this.age} years old.`;
  }
}

const kitty = new Cat("Kitty", 3);
const tiger = new Cat("Tiger", 5);
console.log(kitty.introduce());
console.log(tiger.introduce());
```

```sortie
I am Kitty, I am 3 years old.
I am Tiger, I am 5 years old.
```

Let's break it down:

- `name: string;` declares a **field**: a piece of data that every cat has, with its
  type;
- `constructor` is the function called by `new`. It fills in the fields;
- `this` means the object being built (or the one the method is called on);
- `introduce()` is a [method](../debuter/glossaire.md#method): a function attached to
  the object.

A class is also a type: you can write `const c: Cat = ...`.

### A starting value

A field can get its value directly, without going through the constructor.
TypeScript then guesses its type.

```typescript
class Counter {
  value = 0;

  increment() {
    this.value++;
  }
}

const c = new Counter();
c.increment();
c.increment();
console.log(c.value);
```

```sortie
2
```

> **Remember:** in strict mode, every field must get a value, either in its
> declaration or in the constructor. Otherwise, TypeScript refuses (see the
> [common mistakes](#common-mistakes)).

## Parameter properties

Writing each field three times (declaration, parameter, `this.x = x`) is tedious.
TypeScript offers a shortcut: put `public` (or `private`, `readonly`…) in front of a
constructor parameter. The field is declared and filled in automatically.

```typescript
class Point {
  constructor(public x: number, public y: number) {}

  distanceFromOrigin(): number {
    return Math.sqrt(this.x ** 2 + this.y ** 2);
  }
}

const p = new Point(3, 4);
console.log(p.x, p.y);
console.log(p.distanceFromOrigin());
```

```sortie
3 4
5
```

The body of the constructor is empty `{}`: all the work is done by the words in front
of the parameters.

## Protecting data

By default, everything is **public**: any code can read and change the fields. Some
fields shouldn't be touched from the outside. A bank account shouldn't have its balance
changed in just any way.

| Word | Who can access it? |
|-----|----------------------|
| `public` | everyone (this is the default) |
| `private` | only the code of the class |
| `protected` | the class and its child classes |
| `readonly` | everyone can read, nobody changes it after the constructor |

```typescript
class Account {
  private balance = 0;
  readonly owner: string;

  constructor(owner: string) {
    this.owner = owner;
  }

  deposit(amount: number) {
    if (amount <= 0) {
      console.log("Amount refused");
      return;
    }
    this.balance += amount;
  }

  show() {
    console.log(`${this.owner}: ${this.balance} €`);
  }
}

const account = new Account("Alice");
account.deposit(50);
account.deposit(-10);
account.show();
```

```sortie
Amount refused
Alice: 50 €
```

The only way to change the balance is `deposit`, which checks the amount. Writing
`account.balance = 1000` outside would be refused by TypeScript.

### Truly private fields with #

`private` is a protection from TypeScript only: once the code is compiled to
JavaScript, it disappears. JavaScript has its own protection: a name that starts with
`#`. This one stays active at run time.

```typescript
class Safe {
  #code: string;

  constructor(code: string) {
    this.#code = code;
  }

  open(attempt: string): string {
    return attempt === this.#code ? "Open!" : "Wrong code";
  }
}

const safe = new Safe("1234");
console.log(safe.open("0000"));
console.log(safe.open("1234"));
```

```sortie
Wrong code
Open!
```

> **Note:** both are fine when you're starting out. `private` is more common in
> TypeScript code; `#` is useful when the protection must also hold at run time.

## Implementing an interface

An interface describes a shape (see [objects](objets.md)). With `implements`, a class
**promises** to follow this shape. If it forgets a field or a method, TypeScript
reports it.

```typescript
interface Animal {
  name: string;
  speak(): string;
}

class Dog implements Animal {
  constructor(public name: string) {}
  speak() {
    return "Woof";
  }
}

class Cow implements Animal {
  constructor(public name: string) {}
  speak() {
    return "Moo";
  }
}

const farm: Animal[] = [new Dog("Rex"), new Cow("Daisy")];
for (const a of farm) {
  console.log(`${a.name} says ${a.speak()}`);
}
```

```sortie
Rex says Woof
Daisy says Moo
```

The array has the type `Animal[]`: it accepts any object that has the right shape.

## Inheriting from a class

`extends` creates a **child** class from a **parent** class: it gets all its fields and
methods (see [inheritance](../debuter/glossaire.md#inheritance)). In the constructor,
`super(...)` first calls the constructor of the parent.

```typescript
class Vehicle {
  protected speed = 0;

  constructor(public name: string) {}

  accelerate(delta: number) {
    this.speed += delta;
  }
}

class Bike extends Vehicle {
  constructor() {
    super("Bike");
  }

  describe(): string {
    return `${this.name} at ${this.speed} km/h`;
  }
}

const v = new Bike();
v.accelerate(15);
console.log(v.describe());
```

```sortie
Bike at 15 km/h
```

`speed` is `protected`: `Bike`, the child class, can read it; outside code can't.

## Abstract classes

An **abstract** class is an incomplete mold: you can't create an object with it. It
serves as a common base. It declares `abstract` methods without a body, which each
child class must write.

```typescript
abstract class Shape {
  abstract area(): number;

  describe(): string {
    return `Area: ${this.area().toFixed(2)}`;
  }
}

class Square extends Shape {
  constructor(private side: number) {
    super();
  }
  area() {
    return this.side ** 2;
  }
}

class Circle extends Shape {
  constructor(private radius: number) {
    super();
  }
  area() {
    return Math.PI * this.radius ** 2;
  }
}

const shapes: Shape[] = [new Square(2), new Circle(1)];
for (const s of shapes) {
  console.log(s.describe());
}
```

```sortie
Area: 4.00
Area: 3.14
```

`describe` is written only once, in `Shape`. It calls `area()`, which is different for
each shape.

> **Tip:** interface or abstract class? An interface contains no code: it's a
> contract. An abstract class can share code (here, `describe`). When in doubt, start
> with an interface.

## Static members

A `static` member belongs to the **class** itself, not to each object. You use it with
the name of the class: `Temperature.fromFahrenheit(...)`. It's handy for constants or
for factory functions.

```typescript
class Temperature {
  static readonly ABSOLUTE_ZERO = -273.15;
  static created = 0;

  constructor(public celsius: number) {
    Temperature.created++;
  }

  static fromFahrenheit(f: number): Temperature {
    return new Temperature(((f - 32) * 5) / 9);
  }
}

const t = Temperature.fromFahrenheit(212);
console.log(t.celsius);
console.log(Temperature.ABSOLUTE_ZERO);
console.log(`Objects created: ${Temperature.created}`);
```

```sortie
100
-273.15
Objects created: 1
```

## The get and set accessors

An **accessor** looks like a field, but it's a method in disguise. `get` computes a
value when it is read; `set` controls a write. You use them without parentheses.

```typescript
class Rectangle {
  constructor(private _width: number, private height: number) {}

  get area(): number {
    return this._width * this.height;
  }

  get width(): number {
    return this._width;
  }

  set width(value: number) {
    if (value <= 0) {
      console.log("Width refused");
      return;
    }
    this._width = value;
  }
}

const r = new Rectangle(2, 3);
console.log(r.area);
r.width = 10;
r.width = -1;
console.log(r.area);
```

```sortie
6
Width refused
30
```

`r.area` is always up to date: it is computed again on every read. A `get` without a
`set` is read-only.

## Common mistakes

### Forgetting to initialize a field

```typescript norun
class Cat {
  name: string;
}
```

```text
main.ts(2,3): error TS2564: Property 'name' has no initializer and is not definitely assigned in the constructor.
```

The field `name` never gets a value. Give it a starting value (`name = ""`), or fill
it in in the constructor: `constructor(public name: string) {}`.

### Forgetting this

```typescript norun
class Counter {
  value = 0;
  increment() {
    value++;
  }
}
```

```text
main.ts(4,5): error TS2663: Cannot find name 'value'. Did you mean the instance member 'this.value'?
```

In a method, a field is always written with `this.` in front: `this.value++`. The
message even suggests it to you.

### Touching a private field from the outside

```typescript norun
class Account {
  private balance = 0;
}
const c = new Account();
console.log(c.balance);
```

```text
main.ts(5,15): error TS2341: Property 'balance' is private and only accessible within class 'Account'.
```

This is on purpose: the field is protected. Add a public method to read it, for
example `get amount() { return this.balance; }`.

### Forgetting a method of the interface

```typescript norun
interface Animal {
  name: string;
  speak(): string;
}
class Dog implements Animal {
  constructor(public name: string) {}
}
```

```text
main.ts(5,7): error TS2420: Class 'Dog' incorrectly implements interface 'Animal'.
  Property 'speak' is missing in type 'Dog' but required in type 'Animal'.
```

The promise `implements Animal` isn't kept. Add the missing method:
`speak() { return "Woof"; }`.

## Test yourself

**Question 1:** what does this constructor do?

```typescript norun
class Book {
  constructor(public title: string, private pages: number) {}
}
```

> **Answer**
>
> It declares two fields and fills them in: `title`, public, and `pages`, private.
> It's the short version of two field declarations and two lines
> `this.title = title;` and `this.pages = pages;`.

**Question 2:** can you write `new Shape()` if `Shape` is an abstract class?

> **Answer**
>
> No. TypeScript answers "Cannot create an instance of an abstract class". You have
> to create a child class that writes the abstract methods, then use `new` on that
> child class.

**Question 3:** write a class `Person` with a `readonly name` field and a
`get initial` accessor that returns the first letter of the name.

> **Answer**
>
> ```typescript
> class Person {
>   constructor(readonly name: string) {}
>
>   get initial(): string {
>     return this.name[0] ?? "?";
>   }
> }
>
> const p = new Person("Zoé");
> console.log(p.initial);
> ```
>
> ```sortie
> Z
> ```

## Summary

- A class is a mold for objects; `new` makes an instance, `constructor` fills it in.
- `constructor(public x: number)` declares and fills in a field in one go.
- `private`, `protected` and `readonly` protect fields; `#` also protects at run
  time.
- `implements` checks that a class follows an interface; `extends` inherits from a
  class.
- An `abstract` class serves as a base and can't be instantiated; `static` belongs to
  the class; `get` and `set` disguise methods as fields.

Next: [generics](generiques.md)
