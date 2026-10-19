# Variables, explained simply

A variable is a name given to a value so you can remember it: this page shows how to create one, read it, change it, and why the order of the lines matters.

> **Goals:**
>
> - understand what a variable is: a name and a value;
> - create a variable, read it, then change its value;
> - follow a program line by line with a table;
> - swap the values of two variables;
> - choose clear names;
> - recognize the same program in Python, JavaScript, Ruby and Rust.

## A value, a name

A program handles **values**: a number like `42`, a text
like `"Ada"`. See [value](glossaire.md#value).

To remember a value, you give it a **name**. The name and the
value, together, make a **variable**. See
[variable](glossaire.md#variable).

```python
age = 30
print(age)
```

```sortie
30
```

Read the first line from right to left: "take `30`, and call it
`age`". The second line displays the value that has the name `age`.

> **Remember:** a variable is a name stuck on a value.

## The box with a label

A picture helps a lot: a **box** with a **label**.

```text
   label: age
  +-----------+
  |    30     |   <- the value, in the box
  +-----------+
```

- The label is the **name**: `age`.
- What's inside is the **value**: `30`.
- When you write `age`, the computer opens the box and takes what's
  inside.

A box can hold a number (`30`, `1.65`), a text (`"Ada"`), or
something else. The kind of value is called the **type** (see
[type](glossaire.md#type)).

A text is called a **string**, and you write it between
quotes (see [string](glossaire.md#string)).
A decimal number is written with a **dot**: `1.65`.

## The limits of the box picture

The box picture is very useful when you start. But it isn't
quite accurate.

In reality, the value lives somewhere in the computer's memory. The
name is more like a **label tied to the value** with a thread.

Why does it matter? Because two labels can be tied to **the same**
value. With numbers and texts, you won't see the difference. With
lists, you will:

```python
groceries = ["bread", "milk"]
pauls_list = groceries
pauls_list.append("eggs")
print(groceries)
```

```sortie
['bread', 'milk', 'eggs']
```

```text
  groceries ───────┐
                   ├──> ["bread", "milk", "eggs"]
  pauls_list ──────┘
```

There is only **one** list, with two labels. When Paul adds
eggs, the `groceries` list changes too, since it's the same one.
You'll see this in detail in [data](donnees.md).

> **Remember:** for numbers and texts, think "box". For
> lists, think "tied label".

## Creating a variable

You create a variable by giving it a first value, with the `=` sign:
`city = "Lyon"`.

Careful: here, `=` does **not** mean "is equal to", as in math.
It means "store this value under this name". It's called
**assignment**.

Some languages ask for an extra word, like `let` in JavaScript and
in Rust: you'll see it further down.

## Reading a variable

Reading a variable means using its name. The computer replaces the name
with the value.

```python
price = 4
quantity = 3
total = price * quantity
print(total)
```

```sortie
12
```

On line 3, the computer reads `price` (4) and `quantity` (3), calculates
`4 * 3`, then stores `12` under the name `total`.

Reading a variable doesn't change it. After the calculation, `price` is still 4.

Try it: set `quantity = 10`. The program displays `40`.

## Changing a variable

You can store a new value under a name that already exists. The old
value is **forgotten**.

With `weather = "rain"` then `weather = "sun"`, the `weather` box holds
`"sun"`: `"rain"` is gone.

Very often, you calculate the new value from the old one:

```python
score = 10
score = score + 5
print(score)
```

```sortie
15
```

`score = score + 5` looks wrong in math. In programming, it's normal:
the computer **first** calculates the right side (`10 + 5`, which is 15), **then**
stores the result in `score`.

There's a shortcut: `score += 5` means `score = score + 5`.

> **Remember:** the computer first calculates the right side of the `=`, then stores
> the result on the left.

## The order of the lines matters

The computer reads the lines **one by one, from top to bottom**. A variable
holds whatever was stored in it **last**.

To follow a program, make a **table**: one row per step, one
column per variable.

```python
a = 2
b = 3
a = a + b
b = a * 2
print(a, b)
```

```sortie
5 10
```

| Step | Line | a | b |
|---|---|---|---|
| 1 | `a = 2` | 2 | – |
| 2 | `b = 3` | 2 | 3 |
| 3 | `a = a + b` | 5 | 3 |
| 4 | `b = a * 2` | 5 | 10 |

At step 4, `a` is already 5: so `b` gets `10`, not `4`.

Swap lines 3 and 4: the program displays `6 4`. This time, `b`
is calculated while `a` is still 2: `b` gets 4, then `a` becomes
`2 + 4`, which is 6. Same lines, different order: different result.

Try it: make the table on paper **before** running the program.
Then compare with what it displays.

> **Remember:** when in doubt, make a table, step by step.

## Swapping two values

You have two glasses: one of juice, one of water. You want to swap their
contents. You need a **third glass**, an empty one.

The classic trap, without a third glass:

```python
x = 1
y = 2
x = y
y = x
print(x, y)
```

```sortie
2 2
```

On the line `x = y`, the `1` in `x` is overwritten. It's lost, and it's too late.

With a third glass, `temp` (for "temporary"):

```python
x = 1
y = 2
temp = x
x = y
y = temp
print(x, y)
```

```sortie
2 1
```

| Step | Line | x | y | temp |
|---|---|---|---|---|
| 1 | `x = 1` | 1 | – | – |
| 2 | `y = 2` | 1 | 2 | – |
| 3 | `temp = x` | 1 | 2 | 1 |
| 4 | `x = y` | 2 | 2 | 1 |
| 5 | `y = temp` | 2 | 1 | 1 |

Python (and Ruby) have a shortcut that does it all at once:

```python
x = 1
y = 2
x, y = y, x
print(x, y)
```

```sortie
2 1
```

## Naming clearly

A variable's name is for **humans**, not for the computer. To it,
`x` or `bread_price` is the same. To you, a month from now, it isn't.

Compare `t = p * n` with this:

```python
bread_price = 2
number_of_loaves = 3
total_to_pay = bread_price * number_of_loaves
print(total_to_pay)
```

```sortie
6
```

The result is the same, but this program reads like a sentence.

A few rules that hold almost everywhere:

- no spaces in a name: `bread price` is not allowed;
- words are joined with `_` in Python, Ruby and Rust (`bread_price`), and
  with a capital letter in JavaScript (`breadPrice`);
- a name doesn't start with a digit: `2price` is not allowed;
- capitals matter: `Age` and `age` are two different names;
- avoid accents: `cafe` rather than `café`.

> **Tip:** if you're unsure about a name, ask yourself "what's
> in the box?" and write the answer.

## The same program in four languages

Here is a small program: a basket, a price, a quantity that changes.

In Python:

```python
price = 4
quantity = 3
print("Total:", price * quantity)
quantity = quantity + 2
print("Total:", price * quantity)
```

```sortie
Total: 12
Total: 20
```

In JavaScript, `let` creates a variable that can change; `const`, a
variable that will never change:

```javascript
const price = 4;
let quantity = 3;
console.log("Total:", price * quantity);
quantity = quantity + 2;
console.log("Total:", price * quantity);
```

```sortie
Total: 12
Total: 20
```

In Ruby, as in Python, no extra word:

```ruby
price = 4
quantity = 3
puts "Total: #{price * quantity}"
quantity = quantity + 2
puts "Total: #{price * quantity}"
```

```sortie
Total: 12
Total: 20
```

In Rust, `let` creates the variable. By default, it can **not** change.
You have to write `let mut` (for "mutable", which can change; see
[mutable](glossaire.md#mutable)). Without `mut`, Rust refuses to compile and
suggests adding `mut` itself:

```rust
let price = 4;
let mut quantity = 3;
println!("Total: {}", price * quantity);
quantity = quantity + 2;
println!("Total: {}", price * quantity);
```

```sortie
Total: 12
Total: 20
```

| | Python | JavaScript | Ruby | Rust |
|---|---|---|---|---|
| Create | `x = 1` | `let x = 1;` | `x = 1` | `let mut x = 1;` |
| Will not change | – | `const x = 1;` | – | `let x = 1;` |
| Change | `x = 2` | `x = 2;` | `x = 2` | `x = 2;` |
| Display | `print(x)` | `console.log(x);` | `puts x` | `println!("{x}");` |

> **Remember:** the words change from one language to another, the idea stays the
> same: a name, a value, and `=` to store.

## Common mistakes

### Using a variable before creating it

The computer reads from top to bottom. On line 1, `age` doesn't exist yet.

```python norun
print(age)
age = 30
```

```text
Traceback (most recent call last):
  File "age.py", line 1, in <module>
    print(age)
          ^^^
NameError: name 'age' is not defined
```

`NameError` means "I don't know this name". In JavaScript:

```javascript norun
console.log(age);
let age = 30;
```

```text
console.log(age);
            ^

ReferenceError: Cannot access 'age' before initialization
```

The fix: create the variable **before** using it. Swap the
two lines: `age = 30` first, `print(age)` next.

### A typo in the name

To the computer, `frist_name` and `first_name` are two different names.

```python norun
first_name = "Ada"
print(frist_name)
```

```text
Traceback (most recent call last):
  File "first_name.py", line 2, in <module>
    print(frist_name)
          ^^^^^^^^^^
NameError: name 'frist_name' is not defined. Did you mean: 'first_name'?
```

Python helps you: "Did you mean: 'first_name'?" suggests the name you
probably meant. Ruby does the same.

The fix: write the name exactly the same everywhere, capitals
included. To read these messages, see [reading an error](lire-une-erreur.md).

## Test yourself

**1.** What is `n` at the end?

```python
n = 5
n = n * 2
n = n - 1
print(n)
```

> **Answer**
>
> 9. Step 1: `n` is 5. Step 2: `5 * 2`, so 10. Step 3: `10 - 1`,
> so 9.

**2.** Why don't `x = y` then `y = x` swap two values?

> **Answer**
>
> Because after `x = y`, the old value of `x` is lost: both
> variables hold the same thing. You first have to keep it in a
> third variable, `temp`.

**3.** Which of these names is best for a price in euros:
`p`, `price euros`, `price_in_euros`?

> **Answer**
>
> `price_in_euros`. `p` says nothing, and `price euros` contains a space,
> which is not allowed.

## Summary

- A variable is a name tied to a value: a box with a label.
- `=` stores the value on the right under the name on the left; it isn't "equal".
- The computer reads the lines from top to bottom: a variable holds the last value stored.
- To follow a program, make a table step by step.
- To swap two values, keep one of them in a temporary variable.
- Choose names that say what the box holds.

Next: [true, false and conditions](logique.md)
