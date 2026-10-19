# Solved exercises

This page offers 12 solved exercises, from easiest to hardest, to practice the whole JavaScript track.

## How to use this page

Each exercise has a level:

- ★☆☆: easy;
- ★★☆: medium;
- ★★★: harder.

Read the instructions, then write your solution in a `.mjs` file and run it with
`node file.mjs`. Open the solution only after you have tried.

> **Tip:** there are often several good solutions. If your program prints the right
> result, you win!

## The basics

### Exercise 1: Introduce yourself

Level: ★☆☆

Create two constants, `firstName` and `age`. Then print the sentence
`My name is Alice and I am 30 years old.`, using your constants.

> **Hint:** use a template literal `` `…${firstName}…` `` (see
> [first steps](premiers-pas.md)).

> **Solution**
>
> ```javascript
> const firstName = "Alice";
> const age = 30;
> console.log(`My name is ${firstName} and I am ${age} years old.`);
> ```
>
> ```sortie
> My name is Alice and I am 30 years old.
> ```

### Exercise 2: Even or odd

Level: ★☆☆

For each of the numbers `7` and `10`, print `7 is odd` or `10 is even`.

> **Hint:** `n % 2` gives the remainder of the division by 2: `0` for an even number.

> **Solution**
>
> ```javascript
> for (const n of [7, 10]) {
>   if (n % 2 === 0) {
>     console.log(`${n} is even`);
>   } else {
>     console.log(`${n} is odd`);
>   }
> }
> ```
>
> ```sortie
> 7 is odd
> 10 is even
> ```

### Exercise 3: FizzBuzz

Level: ★☆☆

Print the numbers from 1 to 15. But instead of multiples of 3, print `Fizz`; instead
of multiples of 5, `Buzz`; and instead of multiples of both, `FizzBuzz`. Print
everything on one line, separated by spaces.

> **Hint:** test the "multiple of 15" case first, otherwise it will never be reached
> (see [conditions and loops](conditions-boucles.md)).

> **Solution**
>
> ```javascript
> const results = [];
> for (let i = 1; i <= 15; i++) {
>   if (i % 15 === 0) {
>     results.push("FizzBuzz");
>   } else if (i % 3 === 0) {
>     results.push("Fizz");
>   } else if (i % 5 === 0) {
>     results.push("Buzz");
>   } else {
>     results.push(i);
>   }
> }
> console.log(results.join(" "));
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```

### Exercise 4: Convert temperatures

Level: ★☆☆

Write an arrow function `toFahrenheit(celsius)` that returns
`celsius * 9 / 5 + 32`. Print the conversion of `0`, `37` and `100` degrees.

> **Hint:** an arrow function is written `const f = (x) => …;` (see
> [functions](fonctions.md)).

> **Solution**
>
> ```javascript
> const toFahrenheit = (celsius) => celsius * 9 / 5 + 32;
>
> for (const c of [0, 37, 100]) {
>   console.log(`${c} °C = ${toFahrenheit(c).toFixed(1)} °F`);
> }
> ```
>
> ```sortie
> 0 °C = 32.0 °F
> 37 °C = 98.6 °F
> 100 °C = 212.0 °F
> ```

## Text and collections

### Exercise 5: Count the vowels

Level: ★★☆

Write a function `countVowels(text)` that returns the number of vowels (`a`, `e`, `i`,
`o`, `u`, `y`) in a text, uppercase included. Test it with `"JavaScript"`.

> **Hint:** `[...text]` gives an array of characters; `"aeiouy".includes(c)` tells you
> whether `c` is a vowel (see [working with text](texte.md)).

> **Solution**
>
> ```javascript
> function countVowels(text) {
>   return [...text.toLowerCase()].filter((c) => "aeiouy".includes(c)).length;
> }
>
> console.log(countVowels("JavaScript"));
> console.log(countVowels("Goodbye"));
> ```
>
> ```sortie
> 3
> 4
> ```

### Exercise 6: The palindrome

Level: ★★☆

A palindrome reads the same both ways, like `kayak` or `Anna`. Write
`isPalindrome(word)` that ignores uppercase. Test `"kayak"`, `"Radar"` and
`"code"`.

> **Hint:** reverse the word with `[...word].reverse().join("")`.

> **Solution**
>
> ```javascript
> function isPalindrome(word) {
>   const w = word.toLowerCase();
>   return w === [...w].reverse().join("");
> }
>
> for (const word of ["kayak", "Radar", "code"]) {
>   console.log(`${word}: ${isPalindrome(word)}`);
> }
> ```
>
> ```sortie
> kayak: true
> Radar: true
> code: false
> ```

### Exercise 7: Count the words

Level: ★★☆

In the sentence `"the cat and the dog and the rabbit"`, count how many times each word
appears. Print the words from most frequent to least frequent.

> **Hint:** an object works as a counter: `count[word] = (count[word] ?? 0) + 1`.
> `Object.entries` turns it into an array of pairs, which you can sort (see
> [objects](objets.md)).

> **Solution**
>
> ```javascript
> const sentence = "the cat and the dog and the rabbit";
> const count = {};
> for (const word of sentence.split(" ")) {
>   count[word] = (count[word] ?? 0) + 1;
> }
>
> const sorted = Object.entries(count).toSorted((a, b) => b[1] - a[1]);
> for (const [word, number] of sorted) {
>   console.log(`${word}: ${number}`);
> }
> ```
>
> ```sortie
> the: 3
> and: 2
> cat: 1
> dog: 1
> rabbit: 1
> ```
>
> When there is a tie, `toSorted` keeps the original order: `cat`, `dog`, then `rabbit`.

### Exercise 8: The shopping cart

Level: ★★☆

Here is a shopping cart:

```javascript norun
const cart = [
  { name: "Book", price: 12, quantity: 2 },
  { name: "Pen", price: 1.5, quantity: 4 },
  { name: "Bag", price: 30, quantity: 1 },
];
```

With `filter`, `map` and `reduce`, print the names of the items under 20 €, then the
cart total.

> **Hint:** the total of a line is `price * quantity`; `reduce` adds them up starting
> from `0` (see [collections](collections.md)).

> **Solution**
>
> ```javascript
> const cart = [
>   { name: "Book", price: 12, quantity: 2 },
>   { name: "Pen", price: 1.5, quantity: 4 },
>   { name: "Bag", price: 30, quantity: 1 },
> ];
>
> const cheap = cart.filter((a) => a.price < 20).map((a) => a.name);
> console.log(cheap.join(", "));
>
> const total = cart.reduce((sum, a) => sum + a.price * a.quantity, 0);
> console.log(`Total: ${total.toFixed(2)} €`);
> ```
>
> ```sortie
> Book, Pen
> Total: 60.00 €
> ```

## Objects, classes and functions

### Exercise 9: A private counter

Level: ★★☆

Write a function `createCounter()` that returns an object with two methods:
`increment()`, which adds 1 and returns the new value, and `value()`. The count
variable must not be reachable from the outside. Two counters must be independent.

> **Hint:** this is a [closure](../debuter/glossaire.md#closure): the methods remember a
> `let` variable declared inside `createCounter` (see [closures](closures.md)).

> **Solution**
>
> ```javascript
> function createCounter() {
>   let count = 0;
>   return {
>     increment() {
>       count += 1;
>       return count;
>     },
>     value() {
>       return count;
>     },
>   };
> }
>
> const a = createCounter();
> const b = createCounter();
> a.increment();
> a.increment();
> b.increment();
> console.log(a.value(), b.value(), a.count);
> ```
>
> ```sortie
> 2 1 undefined
> ```

### Exercise 10: The bank account

Level: ★★★

Write a class `BankAccount` with an owner and a **private** balance (`#balance`). It
has three methods: `deposit(amount)`, `withdraw(amount)` and a getter `balance`.
`withdraw` throws a `RangeError` if the balance is too low; `deposit` throws one if
the amount is not positive.

> **Hint:** a field that starts with `#` can only be read inside the class;
> `get balance()` creates a read-only getter (see [inheritance](heritage.md) and
> [errors](erreurs.md)).

> **Solution**
>
> ```javascript
> class BankAccount {
>   #balance = 0;
>
>   constructor(owner) {
>     this.owner = owner;
>   }
>
>   get balance() {
>     return this.#balance;
>   }
>
>   deposit(amount) {
>     if (amount <= 0) throw new RangeError("invalid amount");
>     this.#balance += amount;
>   }
>
>   withdraw(amount) {
>     if (amount > this.#balance) throw new RangeError("insufficient balance");
>     this.#balance -= amount;
>   }
> }
>
> const account = new BankAccount("Alice");
> account.deposit(100);
> account.withdraw(30);
> console.log(`${account.owner}: ${account.balance} €`);
>
> try {
>   account.withdraw(500);
> } catch (error) {
>   console.log(`Refused: ${error.message}`);
> }
> console.log(`Balance unchanged: ${account.balance} €`);
> ```
>
> ```sortie
> Alice: 70 €
> Refused: insufficient balance
> Balance unchanged: 70 €
> ```

## Going further

### Exercise 11: A Fibonacci generator

Level: ★★★

The Fibonacci sequence starts with `0, 1`, then each number is the sum of the two
previous ones: `0, 1, 1, 2, 3, 5, 8…`. Write an endless generator `fibonacci()`, and a
function `take(iterable, n)` that returns the first `n` values in an array. Print the
first 10 numbers.

> **Hint:** a generator is written `function*` and produces its values with `yield`
> (see [iterators](iterateurs.md)). In `take`, stop the `for…of` loop with `break`.

> **Solution**
>
> ```javascript
> function* fibonacci() {
>   let [a, b] = [0, 1];
>   while (true) {
>     yield a;
>     [a, b] = [b, a + b];
>   }
> }
>
> function take(iterable, n) {
>   const result = [];
>   if (n <= 0) return result;
>   for (const value of iterable) {
>     result.push(value);
>     if (result.length === n) break;
>   }
>   return result;
> }
>
> console.log(take(fibonacci(), 10).join(", "));
> ```
>
> ```sortie
> 0, 1, 1, 2, 3, 5, 8, 13, 21, 34
> ```

### Exercise 12: Load in parallel

Level: ★★★

A function simulates loading a grade from a slow server:

```javascript norun
function loadGrade(student) {
  const grades = { Alice: 15, Bob: 9, Chloé: 12 };
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      if (student in grades) resolve(grades[student]);
      else reject(new Error(`unknown student: ${student}`));
    }, 50);
  });
}
```

1. Load the grades of Alice, Bob and Chloé **at the same time** with `Promise.all`, and
   print their average.
2. Then load the grade of `"Zoé"`, catching the error with `try … catch`.

> **Hint:** `await Promise.all(students.map(loadGrade))` waits for all the promises at
> once (see [asynchronous code](asynchrone.md)).

> **Solution**
>
> ```javascript
> function loadGrade(student) {
>   const grades = { Alice: 15, Bob: 9, Chloé: 12 };
>   return new Promise((resolve, reject) => {
>     setTimeout(() => {
>       if (student in grades) resolve(grades[student]);
>       else reject(new Error(`unknown student: ${student}`));
>     }, 50);
>   });
> }
>
> const students = ["Alice", "Bob", "Chloé"];
> const grades = await Promise.all(students.map(loadGrade));
> const average = grades.reduce((s, n) => s + n, 0) / grades.length;
> console.log(`Grades: ${grades.join(", ")}`);
> console.log(`Average: ${average}`);
>
> try {
>   await loadGrade("Zoé");
> } catch (error) {
>   console.log(`Error: ${error.message}`);
> }
> ```
>
> ```sortie
> Grades: 15, 9, 12
> Average: 12
> Error: unknown student: Zoé
> ```
>
> The three loads happen at the same time: the whole thing takes about 50 ms, not
> 150 ms. `Promise.all` keeps the order of the original array.

## Summary

- Start with the ★☆☆ exercises, then move up in difficulty.
- Always try on your own before opening the solution.
- Exercises 1 to 4 go over the basics: constants, conditions, loops, functions.
- Exercises 5 to 8 work on text, arrays, objects and `map`, `filter`, `reduce`.
- Exercises 9 to 12 combine everything: closures, classes, errors, generators and
  promises.

Next: [solved exercises: second set](exercices-2.md)
