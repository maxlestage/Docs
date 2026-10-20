# Scope and closures

This page explains where a variable is visible, how a function remembers its variables (a closure), and what `this` is worth depending on how you write a function.

> **Goals:**
>
> - you will know where a variable declared with `let` or `const` is visible;
> - you will understand that a function remembers the variables around it;
> - you will be able to build functions with a private state;
> - you will be able to avoid the `var` trap in a loop;
> - you will know what `this` is worth in an arrow function and with `bind`.

## The scope of a variable

The [scope](../debuter/glossaire.md#portee) of a variable is the part of the program where
it exists. With `let` and `const`, that part is the **block**: everything between the
curly braces `{ }` where the variable is declared.

Think of the rooms of a house: what is stored in the kitchen cannot be seen from the
living room. But from the kitchen, you can see what is in the shared hallway.

```javascript
const city = "Lyon";

if (true) {
  const district = "Croix-Rousse";
  console.log(city, district);
}

console.log(city);
console.log(typeof district);
```

```sortie
Lyon Croix-Rousse
Lyon
undefined
```

- `city` is declared at the very top: it is visible everywhere.
- `district` is declared in the `if` block: it disappears at the closing curly brace.

> **Note:** `typeof` is the only way to touch a variable that does not exist without an
> error: it answers `"undefined"`. Writing `district` directly would throw a
> `ReferenceError`.

### Blocks fit inside each other

An inner block sees the variables of the blocks around it. The opposite is not true.
A function also has its own scope: its parameters and its variables stay inside.

```javascript
const rate = 0.2;

function priceWithTax(priceBeforeTax) {
  const tax = priceBeforeTax * rate;
  return priceBeforeTax + tax;
}

console.log(priceWithTax(100));
console.log(typeof tax);
```

```sortie
120
undefined
```

`priceWithTax` reads `rate`, declared outside. But `tax` only exists during the call.

### Shadowing

An inner variable can have the same name as an outer variable. It **shadows** it: inside
the block, the inner one is the one you see.

```javascript
const name = "outer";

{
  const name = "inner";
  console.log(name);
}

console.log(name);
```

```sortie
inner
outer
```

> **Tip:** avoid reusing the same names: the code is easier to read.

## A function remembers its variables

Here is the central idea of this chapter. A function created **inside** another function
keeps access to the variables that were around it, **even after** the outer function has
finished.

This pair "function + variables it remembers" is called a
[closure](../debuter/glossaire.md#closure). Imagine a backpack: when it leaves the house,
the function takes along the variables it needs.

```javascript
function makeGreeting(greeting) {
  return function (firstName) {
    return `${greeting}, ${firstName}!`;
  };
}

const hello = makeGreeting("Hello");
const hi = makeGreeting("Hi");

console.log(hello("Ada"));
console.log(hi("Linus"));
```

```sortie
Hello, Ada!
Hi, Linus!
```

When you call `hello("Ada")`, `makeGreeting` finished a long time ago. Yet the function
remembers that `greeting` is `"Hello"`. Each call to `makeGreeting` creates a new
backpack: `hello` and `hi` each have their own.

## A counter factory

The backpack does not hold a frozen copy: the function can **change** the variables it
remembers. This way, you can build a counter.

```javascript
function makeCounter() {
  let count = 0;
  return () => {
    count += 1;
    return count;
  };
}

const counterA = makeCounter();
const counterB = makeCounter();

console.log(counterA());
console.log(counterA());
console.log(counterA());
console.log(counterB());
```

```sortie
1
2
3
1
```

`counterA` and `counterB` each have their own `count` variable. Calling one does not
change the other.

## A private state

Nobody except the function can touch `count`: there is no way to write
`counterA.count`. The closure gives a **private state**, like the `#` fields of a
[class](types.md).

You can return several functions in an object; they share the same backpack.

```javascript
function makePiggyBank() {
  let balance = 0;

  return {
    deposit(amount) {
      if (amount > 0) {
        balance += amount;
      }
    },
    withdraw(amount) {
      if (amount > balance) {
        console.log("Not enough money.");
        return;
      }
      balance -= amount;
    },
    getBalance() {
      return balance;
    },
  };
}

const piggyBank = makePiggyBank();
piggyBank.deposit(20);
piggyBank.withdraw(50);
piggyBank.withdraw(5);
console.log(piggyBank.getBalance());
console.log(piggyBank.balance);
```

```sortie
Not enough money.
15
undefined
```

`piggyBank.balance` is `undefined`: the balance is well hidden. To change it, you have to
go through `deposit` and `withdraw`, which check the amounts.

### Remembering to go faster

A closure can also keep a **cache**: the function remembers the results it has already
computed.

```javascript
function makeSquare() {
  const known = new Map();
  return (n) => {
    if (known.has(n)) {
      console.log(`(${n} already known)`);
      return known.get(n);
    }
    const result = n * n;
    known.set(n, result);
    return result;
  };
}

const square = makeSquare();
console.log(square(9));
console.log(square(9));
```

```sortie
81
(9 already known)
81
```

## The var trap in a loop

Before `let` and `const`, variables were declared with `var`. You will see it in old
code. `var` does **not** have block scope: the variable exists in the whole function. In
a loop, this gives a single variable shared by every turn.

```javascript
const functionsVar = [];
for (var i = 0; i < 3; i++) {
  functionsVar.push(() => i);
}
console.log(functionsVar.map((f) => f()));

const functionsLet = [];
for (let j = 0; j < 3; j++) {
  functionsLet.push(() => j);
}
console.log(functionsLet.map((f) => f()));
```

```sortie
[ 3, 3, 3 ]
[ 0, 1, 2 ]
```

With `var`, the three functions remember **the same** variable `i`. When you call them,
the loop is over and `i` is `3`. With `let`, each turn of the loop creates a **new**
variable `j`: each function keeps its own.

> **Remember:** never use `var`. Write `const` by default, and `let` when the value must
> change.

## this in functions

In a method, `this` means the object you call it on. But be careful: for a function
written with `function`, `this` depends on **how it is called**, not on where it is
written.

An arrow function `=>` has no `this` of its own: it uses the one from the place where it
is written, like any other variable in its backpack.

```javascript
class Timer {
  constructor(name) {
    this.name = name;
  }

  withArrow() {
    [1, 2].forEach((n) => {
      console.log(`${this.name}: ${n}`);
    });
  }

  withFunction() {
    [1, 2].forEach(function (n) {
      console.log(`${this === undefined ? "this lost" : this.name}: ${n}`);
    });
  }
}

const t = new Timer("Tick");
t.withArrow();
t.withFunction();
```

```sortie
Tick: 1
Tick: 2
this lost: 1
this lost: 2
```

In `withFunction`, the function is called by `forEach`, not by the object: its `this` is
`undefined`.

> **Tip:** for a function passed to another one (`forEach`, `map`, `setTimeout`…), use an
> arrow function. For a method of an object or a class, use the normal form
> `method() { }`.

## Fixing this with bind

The same problem happens when you **detach** a method from its object, for example to
store it in a variable. `bind(object)` builds a new function whose `this` is fixed once
and for all.

```javascript
const cat = {
  name: "Kitty",
  introduce() {
    return `I am ${this.name}`;
  },
};

console.log(cat.introduce());

const detached = cat.introduce;
console.log(detached === cat.introduce);

const attached = cat.introduce.bind(cat);
console.log(attached());
```

```sortie
I am Kitty
true
I am Kitty
```

`detached()` would cause an error: called on its own, without `cat.` in front, it no
longer has a `this`. `attached`, on the other hand, remembers `cat`.

## Common mistakes

### Using a variable outside its block

```javascript norun
if (true) {
  let message = "hello";
}
console.log(message);
```

```text
file:///home/vous/projet/main.mjs:4
console.log(message);
            ^

ReferenceError: message is not defined
    at file:///home/vous/projet/main.mjs:4:13
```

`message` only exists in the `if` block. Declare it **before** the block
(`let message;`), then give it its value inside.

### Using a variable before its declaration

```javascript norun
console.log(total);
let total = 10;
```

```text
file:///home/vous/projet/main.mjs:1
console.log(total);
            ^

ReferenceError: Cannot access 'total' before initialization
```

The message says: you cannot access `total` before it is initialized. Always declare a
variable before using it.

### Losing this by detaching a method

```javascript norun
const cat = { name: "Kitty", introduce() { return "I am " + this.name; } };
const f = cat.introduce;
console.log(f());
```

```text
file:///home/vous/projet/main.mjs:1
const cat = { name: "Kitty", introduce() { return "I am " + this.name; } };
                                                                 ^

TypeError: Cannot read properties of undefined (reading 'name')
    at introduce (file:///home/vous/projet/main.mjs:1:66)
    at file:///home/vous/projet/main.mjs:3:13
```

`f()` is called with no object in front: `this` is `undefined`. Call
`cat.introduce()`, or fix `this` with `cat.introduce.bind(cat)`. The same message
appears with a `function` passed to `forEach`: use an arrow function.

## Test yourself

**Question 1:** what does this code print?

```javascript norun
function make() {
  let n = 10;
  return () => (n -= 1);
}
const a = make();
const b = make();
a();
a();
console.log(a(), b());
```

> **Answer**
>
> `7 9`. `a` and `b` each have their own `n`. After three calls, the one of `a` is `7`;
> the one of `b` was decreased only once: `9`.

**Question 2:** why does a `for (var i ...)` loop that creates functions often give the
same value everywhere?

> **Answer**
>
> Because `var` creates **a single** variable for the whole loop. All the functions
> remember it, and read it when the loop is over. With `let`, each turn has its own
> variable.

**Question 3:** write a function `makeAccumulator()` that returns a function: each call
`add(x)` adds `x` to a hidden total and returns the new total.

> **Answer**
>
> ```javascript
> function makeAccumulator() {
>   let total = 0;
>   return (x) => {
>     total += x;
>     return total;
>   };
> }
>
> const add = makeAccumulator();
> add(5);
> add(10);
> console.log(add(1));
> ```
>
> ```sortie
> 16
> ```

## Summary

- A `let` or `const` variable only exists in its block `{ }`; an inner block sees the
  outer variables.
- A closure is a function that remembers the variables that were around it.
- A factory (`makeCounter`) gives each function its own private state.
- `var` has no block scope: do not use it.
- An arrow function keeps the `this` of the place where it is written; `bind` fixes the
  `this` of a function.

Next: [iterators and generators](iterateurs.md). Compare with
[closures in Rust](../rust/fonctions.md).
