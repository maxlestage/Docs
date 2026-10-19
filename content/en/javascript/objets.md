# Objects

This page explains how to group information in an object, read and change its properties, loop over it, copy it, and when to choose a `Map` or a `Set` instead.

> **Goals:**
>
> - you will be able to create an object and read its properties with `.` and `[ ]`;
> - you will be able to add, change and delete a property;
> - you will be able to write a method that uses `this`;
> - you will be able to loop over an object with `Object.keys`, `Object.values` and
>   `Object.entries`;
> - you will be able to copy an object and choose between an object, a `Map` and a `Set`.

## Creating an object

An [object](../debuter/glossaire.md#object) groups several pieces of information about
the same thing. It is like an ID card: each line has a label (the **key**) and a value.
A key-value pair is called a **property**.

```javascript
const cat = {
  name: "Felix",
  age: 3,
  color: "ginger",
  vaccinated: true,
};

console.log(cat);
```

```sortie
{ name: 'Felix', age: 3, color: 'ginger', vaccinated: true }
```

- The object is written between curly braces `{ }`.
- Each property is written `key: value`, and properties are separated by commas.
- A comma after the last property is allowed: it makes adding a line easier.

An array stores values **in order**, by number. An object stores them **by name**. In
other languages, this is called a [dictionary](../debuter/glossaire.md#dictionary).

## Reading a property

You read a property with a dot `.` or with square brackets `[ ]`.

```javascript
const cat = { name: "Felix", age: 3 };

console.log(cat.name);
console.log(cat["age"]);
console.log(cat.weight);

const key = "name";
console.log(cat[key]);
```

```sortie
Felix
3
undefined
Felix
```

- The dot is the most common: `cat.name`.
- Square brackets take a **string**: useful when the key is in a variable (`cat[key]`)
  or contains a space (`object["date of birth"]`).
- A property that does not exist gives `undefined`, without an error.

## Adding, changing, deleting

You can change an object after creating it, even if it is in a `const` (like an array:
`const` only prevents you from replacing the whole object).

```javascript
const cat = { name: "Felix", age: 3 };

cat.age = 4;
cat.color = "ginger";
delete cat.name;

console.log(cat);
console.log("color" in cat);
console.log("name" in cat);
```

```sortie
{ age: 4, color: 'ginger' }
true
false
```

- Assigning a key that exists **changes** it; a new key is **added**.
- `delete` removes a property.
- `"key" in object` checks whether a property exists.

## Shorthand properties

When a variable has the same name as the key, you only need to write the name once.

```javascript
const name = "Alice";
const age = 30;

const person = { name, age };
console.log(person);
```

```sortie
{ name: 'Alice', age: 30 }
```

`{ name, age }` is shorthand for `{ name: name, age: age }`.

## Methods and this

A property can hold a function: it is then called a
[method](../debuter/glossaire.md#method). Inside it, the word `this` refers to the
object the method is called on.

```javascript
const account = {
  owner: "Alice",
  balance: 100,
  deposit(amount) {
    this.balance += amount;
  },
  summary() {
    return `${this.owner} has ${this.balance} €`;
  },
};

account.deposit(50);
console.log(account.summary());
```

```sortie
Alice has 150 €
```

`deposit(amount) { ... }` is shorthand for `deposit: function (amount) { ... }`.
In `account.deposit(50)`, `this` is the object before the dot: `account`.

> **Warning:** do not write a method with an arrow `=>` if it uses `this`: an arrow
> function does not have its own `this`. Keep the form `name() { ... }`.

Objects that share the same methods are built with **classes**: that will be for
[the page on inheritance](heritage.md).

## Looping over an object

Three functions turn an object into an array:

- `Object.keys(object)`: the keys;
- `Object.values(object)`: the values;
- `Object.entries(object)`: `[key, value]` pairs.

```javascript
const stock = { apples: 12, pears: 5, kiwis: 0 };

console.log(Object.keys(stock));
console.log(Object.values(stock));
console.log(Object.entries(stock));
```

```sortie
[ 'apples', 'pears', 'kiwis' ]
[ 12, 5, 0 ]
[ [ 'apples', 12 ], [ 'pears', 5 ], [ 'kiwis', 0 ] ]
```

With `for...of` and destructuring, you loop over keys and values at the same time:

```javascript
const stock = { apples: 12, pears: 5, kiwis: 0 };

for (const [fruit, quantity] of Object.entries(stock)) {
  console.log(`${fruit}: ${quantity}`);
}

const total = Object.values(stock).reduce((s, n) => s + n, 0);
console.log(`Total: ${total}`);
```

```sortie
apples: 12
pears: 5
kiwis: 0
Total: 17
```

Keys are visited in the order they were added (except keys that are whole numbers,
which come first in increasing order).

## Object destructuring

As with arrays, you can put several properties into variables at once. Here, it is the
**name** that matters, not the position.

```javascript
const movie = { title: "Amélie", year: 2001, length: 122 };

const { title, year } = movie;
console.log(title, year);

const { length: minutes, rating = "not rated" } = movie;
console.log(minutes, rating);
```

```sortie
Amélie 2001
122 not rated
```

- `length: minutes` reads the key `length` and puts it in a variable `minutes`.
- `rating = "not rated"` gives a default value if the key is missing.

This is very handy in a function's parameters:

```javascript
function introduce({ name, age }) {
  return `${name}, ${age} years old`;
}

console.log(introduce({ name: "Alice", age: 30 }));
```

```sortie
Alice, 30 years old
```

## Copying an object

Like arrays, an object is shared **by reference**: `b = a` copies nothing, both names
refer to the same object. To copy, use the spread `...`.

```javascript
const original = { name: "Alice", age: 30 };
const copy = { ...original };
copy.age = 31;

const extended = { ...original, city: "Lyon", age: 40 };

console.log(original);
console.log(copy);
console.log(extended);
```

```sortie
{ name: 'Alice', age: 30 }
{ name: 'Alice', age: 31 }
{ name: 'Alice', age: 40, city: 'Lyon' }
```

In `extended`, the properties written **after** the spread replace those of the
original.

### A shallow copy

The spread makes a **shallow** copy: it only copies the first level. An object or an
array stored inside stays shared.

```javascript
const original = { name: "Alice", hobbies: ["cycling"] };
const copy = { ...original };
copy.hobbies.push("piano");
console.log(original.hobbies);

const deep = structuredClone(original);
deep.hobbies.push("singing");
console.log(original.hobbies);
console.log(deep.hobbies);
```

```sortie
[ 'cycling', 'piano' ]
[ 'cycling', 'piano' ]
[ 'cycling', 'piano', 'singing' ]
```

`structuredClone(object)` makes a **deep** copy: every level is copied.

## Displaying an object with JSON.stringify

In a template literal, an object displays badly: `[object Object]`. `JSON.stringify`
turns it into readable text, in [JSON](../debuter/glossaire.md#json) format.

```javascript
const person = { name: "Alice", age: 30, languages: ["fr", "en"] };

console.log(`Person: ${person}`);
console.log(`Person: ${JSON.stringify(person)}`);
console.log(JSON.stringify(person, null, 2));
```

```sortie
Person: [object Object]
Person: {"name":"Alice","age":30,"languages":["fr","en"]}
{
  "name": "Alice",
  "age": 30,
  "languages": [
    "fr",
    "en"
  ]
}
```

The third argument (`2`) indents the result by 2 spaces. JSON is also used to save data
in a file: see [data formats](formats.md).

## Optional chaining

In nested objects, a property in the middle may be missing. `?.` stops cleanly and
returns `undefined` instead of crashing; `??` then provides a fallback value (see
[conditions](conditions-boucles.md#missing-values-and)).

```javascript
const customers = [
  { name: "Alice", address: { city: "Lyon" } },
  { name: "Bob" },
];

for (const customer of customers) {
  const city = customer.address?.city ?? "unknown";
  console.log(`${customer.name}: ${city}`);
}
```

```sortie
Alice: Lyon
Bob: unknown
```

## Map and Set

### Map: a made-to-measure dictionary

A `Map` also stores key-value pairs, with dedicated methods: `set`, `get`, `has`,
`delete`, and `size` for the number of pairs.

```javascript
const counter = new Map();

for (const word of ["cat", "dog", "cat", "cat"]) {
  counter.set(word, (counter.get(word) ?? 0) + 1);
}

console.log(counter);
console.log(counter.get("cat"));
console.log(counter.has("fish"));
console.log(counter.size);

for (const [word, n] of counter) {
  console.log(word, n);
}
```

```sortie
Map(2) { 'cat' => 3, 'dog' => 1 }
3
false
2
cat 3
dog 1
```

### Set: a collection without duplicates

A `Set` is a [set](../debuter/glossaire.md#set): a collection of **unique**
values. Adding a value that is already there does nothing.

```javascript
const visitors = new Set();
visitors.add("Alice");
visitors.add("Bob");
visitors.add("Alice");

console.log(visitors);
console.log(visitors.size);
console.log(visitors.has("Bob"));

const noDuplicates = [...new Set([3, 1, 3, 2, 1])];
console.log(noDuplicates);
```

```sortie
Set(2) { 'Alice', 'Bob' }
2
true
[ 3, 1, 2 ]
```

`[...new Set(array)]` is the classic trick for removing duplicates from an array.

### Which one to choose?

| Need | Choice |
| --- | --- |
| Describe **one thing** with known fields (name, age…) | an object `{ }` |
| Keys added as the program runs (counting words, a cache) | a `Map` |
| Keys that are not strings (numbers, objects) | a `Map` |
| A list of values **without duplicates**, and testing membership | a `Set` |
| Sending or saving as JSON | an object (JSON knows neither `Map` nor `Set`) |

## Common mistakes

### Forgetting a comma between two properties

```javascript norun
const cat = {
  name: "Felix",
  age: 3
  color: "ginger",
};
```

```text
file:///home/alice/js/main.mjs:4
  color: "ginger",
  ^^^^^

SyntaxError: Unexpected identifier 'color'

Node.js v22.22.0
```

The comma after `age: 3` is missing. JavaScript did not expect to find `color` right
after it (*unexpected identifier*). Add the comma.

### Reading inside a property that does not exist

```javascript norun
const user = { name: "Alice" };
console.log(user.address.city);
```

```text
file:///home/alice/js/main.mjs:2
console.log(user.address.city);
                         ^

TypeError: Cannot read properties of undefined (reading 'city')
    at file:///home/alice/js/main.mjs:2:26

Node.js v22.22.0
```

`user.address` is `undefined`, and you cannot read anything from `undefined`. Use
optional chaining: `user.address?.city`.

### Using the dot with a key stored in a variable

```javascript norun
const grades = { maths: 15, english: 12 };
const subject = "maths";
console.log(grades.subject);
```

```text
undefined
```

`grades.subject` looks for a key literally called `"subject"`. To use the **content** of
the variable, write `grades[subject]`.

### Displaying an object in a template literal

```javascript norun
const basket = { apples: 3 };
console.log(`Basket: ${basket}`);
```

```text
Basket: [object Object]
```

A template literal turns the object into text in the poorest possible way. Write
`${JSON.stringify(basket)}`, or pass the object separately: `console.log("Basket:", basket)`.

## Test yourself

**Question 1:** what does this code display?

```javascript norun
const p = { x: 1, y: 2 };
const q = p;
q.x = 10;
console.log(p.x);
```

> **Answer**
>
> `10`. `q` and `p` refer to the same object: changing one changes the other. To make a
> copy, write `const q = { ...p };`.

**Question 2:** how do you display each key and each value of `{ a: 1, b: 2 }`?

> **Answer**
>
> ```javascript
> for (const [key, value] of Object.entries({ a: 1, b: 2 })) {
>   console.log(key, value);
> }
> ```
>
> ```sortie
> a 1
> b 2
> ```

**Question 3:** you want the list of cities you have visited, without duplicates.
Object, `Map` or `Set`?

> **Answer**
>
> A `Set`: it keeps each value only once, and `has(city)` tells you whether a city is
> already in the list.

## Summary

- An object `{ key: value }` groups properties; you read them with `object.key` or
  `object["key"]`.
- You add or change a property by assigning it, and remove it with `delete`;
  `{ name, age }` is a shorthand.
- A method is a function inside an object; `this` refers to the object before the dot.
- `Object.keys`, `Object.values` and `Object.entries` loop over an object;
  `const { a, b } = object` destructures it.
- `{ ...object }` makes a shallow copy, `structuredClone` a deep copy; `JSON.stringify`
  displays it readably.
- `Map` for keys added as the program runs, `Set` for unique values.

Next: [following a program step by step in JavaScript](pas-a-pas.md).
