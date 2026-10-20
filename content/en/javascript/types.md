# Classes

This page shows how to create your own objects with `class`: constructor, fields, methods, `this`, accessors, static members and private fields.

> **Goals:**
>
> - you will know how to write a class and create objects with `new`;
> - you will know how to store data in an object with `constructor` and `this`;
> - you will know how to add methods, getters and setters;
> - you will know how to hide data with a private `#` field;
> - you will know how to check an object's class with `instanceof`.

## Why classes?

You already know how to create an object with curly braces (see [objects](objets.md)):

```javascript
const tom = { name: "Tom", age: 3 };
const felix = { name: "Felix", age: 5 };
console.log(tom.name, felix.name);
```

```sortie
Tom Felix
```

That's handy for one or two objects. But if you have a hundred cats, you copy the same
shape a hundred times, and a typo (`nmae` instead of `name`) goes unnoticed.

A [class](../debuter/glossaire.md#class) is a **template** for building objects that
all have the same shape. Think of a cake mould: the mould is the class, each cake is an
[object](../debuter/glossaire.md#object). They all have the same shape, but each one has
its own flavour.

## Defining a class

The `class` keyword defines a class. By convention, its name starts with a capital
letter. `new` builds a new object from the class: an **instance**.

```javascript
class Cat {
  meow() {
    console.log("Meow!");
  }
}

const tom = new Cat();
tom.meow();
```

```sortie
Meow!
```

`meow` is a [method](../debuter/glossaire.md#method): a function that belongs to the
objects of the class. Notice how it is written: no `function` keyword, and no comma
between methods.

## The constructor and this

`constructor` is a special method, called automatically by `new`. It prepares the
object, like filling in a form when you sign up. The arguments of `new` are passed to it.

Inside a method, `this` means **the current object**: "myself". `this.name = name`
means: "store the value `name` in my `name` property".

```javascript
class Cat {
  constructor(name, age) {
    this.name = name;
    this.age = age;
  }

  introduce() {
    console.log(`My name is ${this.name} and I am ${this.age} years old.`);
  }
}

const tom = new Cat("Tom", 3);
const felix = new Cat("Felix", 5);
tom.introduce();
felix.introduce();
console.log(tom);
```

```sortie
My name is Tom and I am 3 years old.
My name is Felix and I am 5 years old.
Cat { name: 'Tom', age: 3 }
```

Each object has its own data: `tom.name` and `felix.name` are different. The method,
however, is the same for all of them; `this` tells it which object to work on.

`console.log` shows the class name, then the object's properties.

## Fields

A **field** is a property declared directly in the class, with a starting value. It is
easier to read when the value doesn't depend on the arguments.

```javascript
class Counter {
  value = 0;

  increment() {
    this.value += 1;
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

Fields are created **before** the constructor runs. Each object gets its own field: two
counters don't share their `value`.

## Methods that compute

A method can return a value with `return`, and change the object. It can also call
another method of the same object with `this.`.

```javascript
class Cart {
  items = [];

  add(name, price) {
    this.items.push({ name, price });
    return this;
  }

  total() {
    return this.items.reduce((sum, item) => sum + item.price, 0);
  }

  show() {
    console.log(`${this.items.length} items, total ${this.total()} €`);
  }
}

const cart = new Cart();
cart.add("bread", 2).add("milk", 1.5);
cart.show();
```

```sortie
2 items, total 3.5 €
```

> **Tip:** `add` returns `this`, the object itself. So you can chain the calls:
> `cart.add(...).add(...)`.

## Getters and setters

A **getter** (`get`) is a method that you read like a property, without parentheses. It
is used for a computed value.

A **setter** (`set`) is called when you assign a value with `=`. It is used to check the
value before storing it.

```javascript
class Rectangle {
  constructor(width, height) {
    this.width = width;
    this.height = height;
  }

  get area() {
    return this.width * this.height;
  }

  set side(value) {
    this.width = value;
    this.height = value;
  }
}

const r = new Rectangle(3, 4);
console.log(r.area);
r.side = 5;
console.log(r.width, r.height, r.area);
```

```sortie
12
5 5 25
```

`r.area` is written without parentheses, but it really is code that runs on every read:
the area is always up to date.

## Static members

A `static` member belongs to the **class itself**, not to its objects. You call it with
the class name: `Temperature.fromFahrenheit(...)`. It is useful for a shared constant or
for another way to build an object.

```javascript
class Temperature {
  static ABSOLUTE_ZERO = -273.15;

  constructor(celsius) {
    this.celsius = celsius;
  }

  static fromFahrenheit(f) {
    return new Temperature(((f - 32) * 5) / 9);
  }
}

console.log(Temperature.ABSOLUTE_ZERO);
const t = Temperature.fromFahrenheit(212);
console.log(t.celsius);
```

```sortie
-273.15
100
```

You already know some static methods: `Math.max`, `Number.parseInt`, `Array.isArray`,
`Object.keys`…

## Private fields

By default, anyone can read and change an object's properties. For a bank account, that
is a problem: anyone could write `account.balance = 1000000`.

A field whose name starts with `#` is **private**: only code written inside the class can
access it. It's like a bank vault: you go through the counter (the methods), never
straight to the vault.

```javascript
class Account {
  #balance = 0;

  deposit(amount) {
    if (amount <= 0) {
      console.log("Amount refused.");
      return;
    }
    this.#balance += amount;
  }

  get balance() {
    return this.#balance;
  }
}

const account = new Account();
account.deposit(50);
account.deposit(-10);
console.log(account.balance);
try {
  account.balance = 1000000;
} catch (error) {
  console.log(error.message);
}
```

```sortie
Amount refused.
50
Cannot set property balance of #<Account> which has only a getter
```

The `balance` getter lets you **read** the balance, but there is no setter: the
assignment `account.balance = 1000000` throws an error. Only `deposit` can change
`#balance`, and it checks the amount. `try` and `catch` catch the error; you will see
them in detail in [handling errors](erreurs.md).

A method can be private too: `#computeInterest() { ... }`. You call it with
`this.#computeInterest()`, and only from inside the class.

## Displaying an object with toString

When JavaScript needs to turn an object into text (in a `${}` template or with
`String(...)`), it calls its `toString` method. By default, it gives a not very useful
text: `[object Object]`. Write your own.

```javascript
class Point {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }

  toString() {
    return `(${this.x}, ${this.y})`;
  }
}

const p = new Point(2, 5);
console.log(`The point is at ${p}`);
console.log(String(p));
```

```sortie
The point is at (2, 5)
(2, 5)
```

> **Note:** `console.log(p)` doesn't use `toString`: it shows the structure of the
> object, `Point { x: 2, y: 5 }`. That's handy for debugging.

## Checking the class with instanceof

`instanceof` answers the question: "was this object built with this class?". The result
is a [boolean](../debuter/glossaire.md#boolean).

```javascript
class Cat {}
class Dog {}

const tom = new Cat();
console.log(tom instanceof Cat);
console.log(tom instanceof Dog);
console.log([1, 2] instanceof Array);
console.log(tom.constructor.name);
console.log(typeof Cat);
```

```sortie
true
false
true
Cat
function
```

`constructor.name` gives the name of an object's class. And `typeof Cat` reveals a
secret: in JavaScript, a class is a kind of function, the one that builds the objects.
The chapter on [inheritance](heritage.md) explains what is hidden behind it.

## Common mistakes

### Forgetting new

```javascript norun
class Cat {
  constructor(name) { this.name = name; }
}
const c = Cat("Tom");
```

```text
file:///home/vous/projet/main.mjs:4
const c = Cat("Tom");
          ^

TypeError: Class constructor Cat cannot be invoked without 'new'
    at file:///home/vous/projet/main.mjs:4:11
```

A class isn't called like a function. Write `new Cat("Tom")`.

### Forgetting this in a method

```javascript norun
class Cat {
  constructor(name) { this.name = name; }
  introduce() { console.log("I am " + name); }
}
new Cat("Tom").introduce();
```

```text
file:///home/vous/projet/main.mjs:3
  introduce() { console.log("I am " + name); }
                                      ^

ReferenceError: name is not defined
    at Cat.introduce (file:///home/vous/projet/main.mjs:3:39)
    at file:///home/vous/projet/main.mjs:5:16
```

In `introduce`, `name` on its own is a variable that doesn't exist. The object's
property is written `this.name`.

### Reading a private field from outside

```javascript norun
class Account {
  #balance = 0;
}
const c = new Account();
console.log(c.#balance);
```

```text
file:///home/vous/projet/main.mjs:5
console.log(c.#balance);
             ^

SyntaxError: Private field '#balance' must be declared in an enclosing class
```

The program doesn't even start: it's a [syntax](../debuter/glossaire.md#syntax) error.
Add a method or a getter in the class (`get balance() { return this.#balance; }`), then
read `c.balance`.

## Test yourself

**Question 1:** what does this code print?

```javascript norun
class Box {
  contents = [];
}
const a = new Box();
const b = new Box();
a.contents.push("apple");
console.log(b.contents.length);
```

> **Answer**
>
> `0`. Each object gets its own `contents` field: filling the one in `a` doesn't change
> the one in `b`.

**Question 2:** what is the difference between `static` and a normal field?

> **Answer**
>
> A normal field exists in **each object** (`tom.name`). A `static` member exists **only
> once**, on the class (`Temperature.ABSOLUTE_ZERO`).

**Question 3:** write a `Person` class with a `firstName` and a `lastName`, and a
`fullName` getter that returns both separated by a space.

> **Answer**
>
> ```javascript
> class Person {
>   constructor(firstName, lastName) {
>     this.firstName = firstName;
>     this.lastName = lastName;
>   }
>
>   get fullName() {
>     return `${this.firstName} ${this.lastName}`;
>   }
> }
>
> console.log(new Person("Ada", "Lovelace").fullName);
> ```
>
> ```sortie
> Ada Lovelace
> ```

## Summary

- A class is a template; `new` builds an object, and calls `constructor`.
- `this` means the current object; its properties are written `this.name`.
- `get` and `set` give computed or checked properties.
- `static` belongs to the class; `#field` is private, invisible outside the class.
- `toString` chooses an object's text; `instanceof` checks its class.

Next: [inheritance and prototypes](heritage.md). Compare with
[classes in Ruby](../ruby/types.md).
