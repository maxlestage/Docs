# Solved exercises: second set

This page offers 12 new solved exercises in JavaScript, from easiest to hardest, with trace tables to follow the solutions step by step.

## How to use this page

These exercises are **new**: they don't repeat the ones from the
[exercises to get started](exercices-faciles.md) or from the
[first set](exercices.md). Each exercise has a level:

- ★☆☆: easy, a single concept;
- ★★☆: medium, two or three concepts together;
- ★★★: harder, you need to think on paper first.

For each exercise:

1. read the instructions and the expected output;
2. make a small plan in plain English, on paper;
3. write the code in an `exercise.mjs` file and run `node exercise.mjs`;
4. stuck? read the hint, then try again;
5. open the solution last, and follow its trace table with your finger.

```text
 instructions → plan on paper → code → run → compare
                                 ▲              │
                                 └─ different? ─┘
```

> **Tip:** to fill in a trace table, read
> [following a program step by step](pas-a-pas.md) again.

## First level

### Exercise 1: The parking ticket

Level: ★☆☆

A car park costs €2 per hour, but never more than €10. Write a function
`parkingPrice(hours)` that returns the price. Print the price for 1, 3 and 8 hours.

Expected output:

```text
1 h: €2
3 h: €6
8 h: €10
```

> **Hint:** first compute `hours * 2`. Then, with an `if`, return `10` if this price
> is more than 10 (see [functions](fonctions.md)).

> **Solution**
>
> ```javascript
> function parkingPrice(hours) {
>   const price = hours * 2;
>   if (price > 10) {
>     return 10;
>   }
>   return price;
> }
>
> for (const h of [1, 3, 8]) {
>   console.log(`${h} h: €${parkingPrice(h)}`);
> }
> ```
>
> ```sortie
> 1 h: €2
> 3 h: €6
> 8 h: €10
> ```
>
> Why it works: `return` stops the function right away. If the price is more than 10,
> we return 10 and the last line is never read. `Math.min(hours * 2, 10)` does the
> same thing in one line.

### Exercise 2: The frost days

Level: ★☆☆

Here are the temperatures of one week: `[3, -2, 5, -7, 0, 4]`. Print each frost day
(temperature below 0), with its number (the first day is day 1), then the number of
frost days.

Expected output:

```text
Frost on day 2: -2 °C
Frost on day 4: -7 °C
Frost days: 2
```

> **Hint:** you need the day number: use a loop with an index,
> `for (let i = 0; i < temperatures.length; i++)`. The day number is `i + 1`.

> **Solution**
>
> ```javascript
> const temperatures = [3, -2, 5, -7, 0, 4];
> let frostDays = 0;
> for (let i = 0; i < temperatures.length; i++) {
>   if (temperatures[i] < 0) {
>     console.log(`Frost on day ${i + 1}: ${temperatures[i]} °C`);
>     frostDays = frostDays + 1;
>   }
> }
> console.log(`Frost days: ${frostDays}`);
> ```
>
> ```sortie
> Frost on day 2: -2 °C
> Frost on day 4: -7 °C
> Frost days: 2
> ```
>
> Why it works: indexes start at 0, days start at 1, hence `i + 1`. Day 5 is `0`:
> `0 < 0` is false, so it is not frost.

### Exercise 3: The longest word

Level: ★☆☆

In the array `["cat", "elephant", "mouse", "bear"]`, find the longest word and print
it with its number of letters.

Expected output:

```text
The longest word: elephant (8 letters)
```

> **Hint:** keep the best word found so far in a box `longest`, which starts with the
> first word. On each turn, compare the `.length` values (see [text](texte.md)).

> **Solution**
>
> ```javascript
> const words = ["cat", "elephant", "mouse", "bear"];
> let longest = words[0];
> for (const word of words) {
>   if (word.length > longest.length) {
>     longest = word;
>   }
> }
> console.log(`The longest word: ${longest} (${longest.length} letters)`);
> ```
>
> ```sortie
> The longest word: elephant (8 letters)
> ```
>
> Why it works: it's the champion idea. We keep the best one, and each new word must
> beat it to take its place.
>
> | Turn | word | length | longer than longest? | longest |
> | --- | --- | --- | --- | --- |
> | start | – | – | – | cat |
> | 1 | cat | 3 | 3 > 3 → no | cat |
> | 2 | elephant | 8 | 8 > 3 → yes | elephant |
> | 3 | mouse | 5 | 5 > 8 → no | elephant |
> | 4 | bear | 4 | 4 > 8 → no | elephant |

### Exercise 4: The triangle of stars

Level: ★☆☆

Print a triangle of 4 lines: one star on the first line, two on the second, and so
on.

Expected output:

```text
*
**
***
****
```

> **Hint:** `"*".repeat(3)` gives `"***"`. Make a loop from 1 to 4 and repeat the star
> as many times as the line number.

> **Solution**
>
> ```javascript
> for (let line = 1; line <= 4; line++) {
>   console.log("*".repeat(line));
> }
> ```
>
> ```sortie
> *
> **
> ***
> ****
> ```
>
> Why it works: the loop variable is used twice, to count the lines and to say how
> many stars to put.
>
> | Turn | line | line <= 4? | printed |
> | --- | --- | --- | --- |
> | 1 | 1 | yes | `*` |
> | 2 | 2 | yes | `**` |
> | 3 | 3 | yes | `***` |
> | 4 | 4 | yes | `****` |
> | end | 5 | no | (the loop stops) |

## Second level

### Exercise 5: Giving change

Level: ★★☆

A till must give back 470 cents with as few coins as possible. The coins are worth
200, 100, 50, 20 and 10 cents. Write `giveChange(amount)` that prints how many coins
of each kind are needed (only the ones used).

Expected output:

```text
2 × 200 c
1 × 50 c
1 × 20 c
```

> **Hint:** start with the biggest coin. As long as the rest is big enough, give this
> coin and subtract its value from the rest. Then move on to the next coin.

> **Hint:** you need two loops: a `for…of` over the coins, and inside it a
> `while (rest >= coin)`. Work in cents to avoid decimals
> (see [numbers](nombres.md)).

> **Solution**
>
> ```javascript
> function giveChange(amount) {
>   const coins = [200, 100, 50, 20, 10];
>   let rest = amount;
>   for (const coin of coins) {
>     let count = 0;
>     while (rest >= coin) {
>       rest = rest - coin;
>       count = count + 1;
>     }
>     if (count > 0) {
>       console.log(`${count} × ${coin} c`);
>     }
>   }
> }
>
> giveChange(470);
> ```
>
> ```sortie
> 2 × 200 c
> 1 × 50 c
> 1 × 20 c
> ```
>
> Why it works: we empty the rest starting with the big coins, like a cashier who
> first takes from the €2 slot.
>
> | Step | coin | rest >= coin? | rest | count |
> | --- | --- | --- | --- | --- |
> | start | – | – | 470 | – |
> | 1 | 200 | 470 >= 200 → yes | 270 | 1 |
> | 2 | 200 | 270 >= 200 → yes | 70 | 2 |
> | 3 | 200 | 70 >= 200 → no | 70 | 2 (printed) |
> | 4 | 100 | 70 >= 100 → no | 70 | 0 |
> | 5 | 50 | 70 >= 50 → yes | 20 | 1 |
> | 6 | 50 | 20 >= 50 → no | 20 | 1 (printed) |
> | 7 | 20 | 20 >= 20 → yes | 0 | 1 |
> | 8 | 20 | 0 >= 20 → no | 0 | 1 (printed) |
> | 9 | 10 | 0 >= 10 → no | 0 | 0 |

### Exercise 6: Removing duplicates

Level: ★★☆

Write `withoutDuplicates(values)` that returns a **new** array without the repeated
values, in the order they first appear. Test it with `[3, 1, 3, 2, 1]`.

Expected output:

```text
[ 3, 1, 2 ]
[ 'a', 'b' ]
```

> **Hint:** create an empty array `result`. For each value, add it only if
> `result.includes(v)` is false (see [arrays](collections.md)).

> **Solution**
>
> ```javascript
> function withoutDuplicates(values) {
>   const result = [];
>   for (const v of values) {
>     if (!result.includes(v)) {
>       result.push(v);
>     }
>   }
>   return result;
> }
>
> console.log(withoutDuplicates([3, 1, 3, 2, 1]));
> console.log(withoutDuplicates(["a", "b", "a"]));
> ```
>
> ```sortie
> [ 3, 1, 2 ]
> [ 'a', 'b' ]
> ```
>
> Why it works: `!` flips the answer of `includes`. So we add a value only when it is
> not in `result` yet.
>
> | Turn | v | already in result? | result |
> | --- | --- | --- | --- |
> | start | – | – | `[]` |
> | 1 | 3 | no | `[3]` |
> | 2 | 1 | no | `[3, 1]` |
> | 3 | 3 | yes | `[3, 1]` |
> | 4 | 2 | no | `[3, 1, 2]` |
> | 5 | 1 | yes | `[3, 1, 2]` |
>
> Later, you will simply write `[...new Set(values)]`: a `Set` never keeps the same
> value twice (see [objects](objets.md)).

### Exercise 7: Grouping by initial

Level: ★★☆

Group the first names `["Alice", "Bob", "Anne", "Chloé", "Bruno"]` by first letter,
in an object. Then print one line per letter.

Expected output:

```text
A: Alice, Anne
B: Bob, Bruno
C: Chloé
```

> **Hint:** the object `groups` has one key per letter, and an array of first names as
> its value. `firstName[0]` gives the first letter.

> **Hint:** if `groups[initial]` is `undefined`, first create the empty drawer
> `groups[initial] = []`, then do `push`. For the display, use `Object.entries` (see
> [objects](objets.md)).

> **Solution**
>
> ```javascript
> const firstNames = ["Alice", "Bob", "Anne", "Chloé", "Bruno"];
> const groups = {};
> for (const firstName of firstNames) {
>   const initial = firstName[0];
>   if (groups[initial] === undefined) {
>     groups[initial] = [];
>   }
>   groups[initial].push(firstName);
> }
>
> for (const [letter, list] of Object.entries(groups)) {
>   console.log(`${letter}: ${list.join(", ")}`);
> }
> ```
>
> ```sortie
> A: Alice, Anne
> B: Bob, Bruno
> C: Chloé
> ```
>
> Why it works: it's a chest of drawers. We open the drawer for the letter; if it
> doesn't exist, we create it empty; then we put the first name in it.

### Exercise 8: Checking a password

Level: ★★☆

Write `passwordProblems(password)` that returns the list of problems of a password:
fewer than 8 characters, no digit, no uppercase letter. Test it with `"cat"`,
`"sunshine2024"` and `"Sunshine2024"`.

Expected output:

```text
cat: fewer than 8 characters, no digit, no uppercase letter
sunshine2024: no uppercase letter
Sunshine2024: OK
```

> **Hint:** three separate `if`s (no `else`), because several problems can exist at
> the same time. Each `if` adds a text to a `problems` array.

> **Hint:** a text with no uppercase letter doesn't change with `toLowerCase()`. For
> digits, check whether a character is in `"0123456789"`.

> **Solution**
>
> ```javascript
> function passwordProblems(password) {
>   const problems = [];
>   if (password.length < 8) {
>     problems.push("fewer than 8 characters");
>   }
>   if (![...password].some((c) => "0123456789".includes(c))) {
>     problems.push("no digit");
>   }
>   if (password === password.toLowerCase()) {
>     problems.push("no uppercase letter");
>   }
>   return problems;
> }
>
> for (const password of ["cat", "sunshine2024", "Sunshine2024"]) {
>   const problems = passwordProblems(password);
>   if (problems.length === 0) {
>     console.log(`${password}: OK`);
>   } else {
>     console.log(`${password}: ${problems.join(", ")}`);
>   }
> }
> ```
>
> ```sortie
> cat: fewer than 8 characters, no digit, no uppercase letter
> sunshine2024: no uppercase letter
> Sunshine2024: OK
> ```
>
> Why it works: with `else if`, the program would stop at the first problem. Three
> independent `if`s test everything, like a checklist where you tick each box.

### Exercise 9: The Caesar cipher

Level: ★★☆

Julius Caesar hid his messages by shifting each letter by 3 in the alphabet: `a`
becomes `d`, `z` becomes `c`. Write `caesar(text, shift)` for lowercase texts without
accents; the other characters don't change. Encode `"hello zoe"`, then decode the
result with a shift of 23.

Expected output:

```text
khoor crh
hello zoe
```

> **Hint:** `ALPHABET.indexOf(c)` gives the position of a letter (`-1` if it is not a
> letter). The new position is `(position + shift) % 26`.

> **Solution**
>
> ```javascript
> const ALPHABET = "abcdefghijklmnopqrstuvwxyz";
>
> function caesar(text, shift) {
>   let result = "";
>   for (const c of text) {
>     const position = ALPHABET.indexOf(c);
>     if (position === -1) {
>       result += c;
>     } else {
>       result += ALPHABET[(position + shift) % 26];
>     }
>   }
>   return result;
> }
>
> const secret = caesar("hello zoe", 3);
> console.log(secret);
> console.log(caesar(secret, 23));
> ```
>
> ```sortie
> khoor crh
> hello zoe
> ```
>
> Why it works: `% 26` makes the alphabet go round in a circle, like the hands of a
> clock. For `z`: position 25, 25 + 3 = 28, 28 % 26 = 2, that's `c`. Shifting by 3
> then by 23 makes 26: a full turn, we are back where we started.
>
> ```text
>  a b c d e … x y z
>  ▲               │
>  └───────────────┘
>   after z, back to a
> ```

## Third level

### Exercise 10: Bubble sort

Level: ★★★

Sort `[5, 2, 4, 1]` from smallest to largest **without** using `sort`. Method: go
through the array and swap two neighbours if they are in the wrong order. Start again
until everything is sorted. Print the array after each pass. The starting array must
not change.

Expected output:

```text
After pass 1: 2 4 1 5
After pass 2: 2 1 4 5
After pass 3: 1 2 4 5
Sorted: 1 2 4 5, original: 5 2 4 1
```

> **Hint:** work on a copy: `const t = [...array];`. To swap `t[i]` and `t[i + 1]`,
> keep one of them in a box `temp` (see
> [following a program step by step](pas-a-pas.md)).

> **Hint:** after each pass, the biggest remaining number has reached the end, like a
> bubble rising. For an array of 4 numbers, you need 3 passes, and pass number `p`
> stops before the last `p` cells.

> **Solution**
>
> ```javascript
> function bubbleSort(array) {
>   const t = [...array];
>   for (let pass = 1; pass < t.length; pass++) {
>     for (let i = 0; i < t.length - pass; i++) {
>       if (t[i] > t[i + 1]) {
>         const temp = t[i];
>         t[i] = t[i + 1];
>         t[i + 1] = temp;
>       }
>     }
>     console.log(`After pass ${pass}: ${t.join(" ")}`);
>   }
>   return t;
> }
>
> const numbers = [5, 2, 4, 1];
> const sorted = bubbleSort(numbers);
> console.log(`Sorted: ${sorted.join(" ")}, original: ${numbers.join(" ")}`);
> ```
>
> ```sortie
> After pass 1: 2 4 1 5
> After pass 2: 2 1 4 5
> After pass 3: 1 2 4 5
> Sorted: 1 2 4 5, original: 5 2 4 1
> ```
>
> Why it works: each pass pushes the biggest remaining number to the right. Here is
> the first pass in detail:
>
> | pass | i | compare | swap? | t |
> | --- | --- | --- | --- | --- |
> | 1 | 0 | 5 > 2 | yes | `[2, 5, 4, 1]` |
> | 1 | 1 | 5 > 4 | yes | `[2, 4, 5, 1]` |
> | 1 | 2 | 5 > 1 | yes | `[2, 4, 1, 5]` |
>
> The 5 is at the end: it is in its place. Pass 2 only looks at the first 3 cells,
> pass 3 at the first 2.

### Exercise 11: Guessing a number in few tries

Level: ★★★

A secret number is between 1 and 100. After each try, you are told "too low" or "too
high". Write `guess(secret)` that always tries the **middle** of the range that is
still possible, and prints each try. Test it with `71`.

Expected output:

```text
50: too low
75: too high
62: too low
68: too low
71: found in 5 tries
```

> **Hint:** keep two bounds, `low = 1` and `high = 100`. The middle is
> `Math.floor((low + high) / 2)`.

> **Hint:** if the middle is too low, the secret is above it: `low = middle + 1`. If
> it is too high: `high = middle - 1`. Repeat with `while (low <= high)`.

> **Solution**
>
> ```javascript
> function guess(secret) {
>   let low = 1;
>   let high = 100;
>   let tries = 0;
>   while (low <= high) {
>     const middle = Math.floor((low + high) / 2);
>     tries = tries + 1;
>     if (middle === secret) {
>       console.log(`${middle}: found in ${tries} tries`);
>       return tries;
>     } else if (middle < secret) {
>       console.log(`${middle}: too low`);
>       low = middle + 1;
>     } else {
>       console.log(`${middle}: too high`);
>       high = middle - 1;
>     }
>   }
>   return -1;
> }
>
> guess(71);
> ```
>
> ```sortie
> 50: too low
> 75: too high
> 62: too low
> 68: too low
> 71: found in 5 tries
> ```
>
> Why it works: each try cuts the range in two, like when you open a dictionary in the
> middle to look up a word. You never need more than 7 tries for 100 numbers.
>
> | Try | low | high | middle | answer |
> | --- | --- | --- | --- | --- |
> | 1 | 1 | 100 | 50 | too low |
> | 2 | 51 | 100 | 75 | too high |
> | 3 | 51 | 74 | 62 | too low |
> | 4 | 63 | 74 | 68 | too low |
> | 5 | 69 | 74 | 71 | found |

### Exercise 12: Balanced brackets

Level: ★★★

Write `balanced(text)` that says whether the parentheses `()`, square brackets `[]`
and curly braces `{}` of a text are properly closed, in the right order. The other
characters are ignored. Test it with `"(a[b]{c})"`, `"(]"`, `"((a)"` and `"f(x[0])"`.

Expected output:

```text
(a[b]{c}): true
(]: false
((a): false
f(x[0]): true
```

> **Hint:** use an array like a **stack** of plates: `push` puts an opening bracket on
> top, `pop` takes back the one on top.

> **Hint:** at each closing bracket, the last opening bracket put down must be its
> partner. An object `{ ")": "(", "]": "[", "}": "{" }` gives the partner. At the end,
> the stack must be empty.

> **Solution**
>
> ```javascript
> function balanced(text) {
>   const openers = { ")": "(", "]": "[", "}": "{" };
>   const stack = [];
>   for (const c of text) {
>     if (c === "(" || c === "[" || c === "{") {
>       stack.push(c);
>     } else if (c in openers) {
>       if (stack.pop() !== openers[c]) {
>         return false;
>       }
>     }
>   }
>   return stack.length === 0;
> }
>
> for (const text of ["(a[b]{c})", "(]", "((a)", "f(x[0])"]) {
>   console.log(`${text}: ${balanced(text)}`);
> }
> ```
>
> ```sortie
> (a[b]{c}): true
> (]: false
> ((a): false
> f(x[0]): true
> ```
>
> Why it works: the last opening bracket put down is the first one to close, just as
> the last plate put down is the first one taken back. Here is the trace of
> `"(a[b]{c})"`:
>
> | Turn | c | action | stack |
> | --- | --- | --- | --- |
> | 1 | `(` | put down | `["("]` |
> | 2 | `a` | ignore | `["("]` |
> | 3 | `[` | put down | `["(", "["]` |
> | 4 | `b` | ignore | `["(", "["]` |
> | 5 | `]` | take back `[`: partner, ok | `["("]` |
> | 6 | `{` | put down | `["(", "{"]` |
> | 7 | `c` | ignore | `["(", "{"]` |
> | 8 | `}` | take back `{`: partner, ok | `["("]` |
> | 9 | `)` | take back `(`: partner, ok | `[]` |
>
> The stack is empty at the end: `true`. For `"((a)"`, a `(` is left in the stack:
> `false`. For `"(]"`, we take back `(` when `[` was needed: `false` right away.

## Summary

- Exercises 1 to 4 practise one concept at a time: function, index, champion,
  repetition.
- Exercises 5 to 9 combine loops, conditions, arrays, objects and text.
- Exercises 10 to 12 are real little algorithms: sorting, searching, checking.
- When a solution escapes you, fill in its trace table with a pencil.

Next: keep [the cheat sheet](aide-memoire.md) close at hand.
