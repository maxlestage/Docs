# Thinking like a programmer

Before writing code, you think about the method: this page shows how to break a problem down into small steps, write it in plain English, then translate it into Python, JavaScript, Ruby, Rust and Rythme.

> **Goals:**
>
> - know what an algorithm is;
> - break a problem down into small steps;
> - write a method in plain English, as pseudocode;
> - use variables, conditions and repetitions;
> - translate pseudocode into several languages;
> - check an algorithm by hand, with a table of values.

## What is an algorithm

An **algorithm** is a sequence of precise steps that solves a problem. A
cooking recipe, the directions to a friend's place, the instructions for a
piece of furniture: these are algorithms.

A good algorithm has three qualities:

- each step is **clear**: there's only one way to understand it;
- the steps are in the **right order**;
- it **stops** and gives a result.

A program is an algorithm written in a programming language. The hard
part is often the algorithm, not the language.

> **Remember:** method first, code second.

## Breaking a problem down into small steps

A big problem is scary. Cut into small pieces, it becomes simple.

Take "make a cup of tea". That's too vague for a computer. Let's break it
down:

```text
1. Fill the kettle with water.
2. Switch the kettle on.
3. Wait for the water to boil.
4. Put a tea bag in a cup.
5. Pour the water into the cup.
6. Wait 3 minutes.
7. Remove the tea bag.
```

Each step is now small enough to be done without thinking. If a step is
still fuzzy, break it down again.

> **Tip:** ask yourself at each line: "could an 8-year-old do it?". If the
> answer is no, break it down.

## Write in plain English first: pseudocode

**Pseudocode** is an algorithm written in simple English, shaped like a
program. It has no strict rules: it's for thinking, not for running.

```text
ask for the person's age
if the age is 18 or more
    display "You can vote."
otherwise
    display "Not yet."
```

Pseudocode has two advantages:

- you think about the problem **without fighting** the syntax;
- it then translates into **any language**.

## Variables: boxes with a label

An algorithm has to remember things: an age, a total, a first name. They
are stored in **variables**.

Picture a box with a label. The label is the variable's **name**; what's
inside is its **value**.

```text
put 0 in the box "total"
put 5 in the box "price"
put total + price in the box "total"   (total is now 5)
```

In Ruby, this is written with `=`:

```ruby
total = 0
price = 5
total = total + price
puts total
```

```sortie
5
```

> **Warning:** in programming, `=` doesn't mean "is equal to". It means
> "store in the box". To compare, you write `==`.

Choose names that say what the box holds: `total_price` rather than `x`.

## Conditions in everyday life

You make decisions all day long: "**if** it's raining, I take an umbrella,
**otherwise** I take sunglasses". That's a **condition**.

In pseudocode:

```text
if it's raining
    take an umbrella
otherwise
    take sunglasses
```

In Ruby:

```ruby
raining = true
if raining
  puts "I take an umbrella."
else
  puts "I take sunglasses."
end
```

```sortie
I take an umbrella.
```

A condition is always a question you answer with **yes** or **no**:
`true` or `false`.

## Repetition in everyday life

Many tasks repeat: "for each dirty plate, wash it", "while the batter
isn't smooth, stir". That's a **repetition**, or **loop**.

There are two kinds of repetition:

- **for each** item of a list: you know how many times you repeat;
- **while** something is true: you stop when it changes.

```text
for each plate in the stack
    wash the plate
    put it on the dish rack
```

In Ruby:

```ruby
plates = ["blue", "white", "green"]
plates.each do |plate|
  puts "I wash the #{plate} plate."
end
```

```sortie
I wash the blue plate.
I wash the white plate.
I wash the green plate.
```

> **Remember:** with variables, conditions and repetitions, you can write
> almost every algorithm.

## A complete example: finding the largest number

Here is a list of numbers: `4, 17, 9, 12`. How do you find the largest?

A human looks and answers "17". But if the list holds a million numbers,
you need a **method**.

### Think like a human

Picture cards turned over one by one. You keep in mind the largest number
seen so far. With each new card, you compare: if it's larger, it becomes
your new "largest".

### The pseudocode

```text
put the first number in the box "largest"
for each number in the list
    if number is larger than largest
        put number in the box "largest"
display largest
```

There's one variable, one repetition and one condition. Nothing more.

### In Python

```python
numbers = [4, 17, 9, 12]
largest = numbers[0]
for number in numbers:
    if number > largest:
        largest = number
print(largest)
```

```sortie
17
```

Each line of the pseudocode has become a line of Python. The shift of the
lines (the indentation) shows what is **inside** the loop and **inside**
the condition.

### In JavaScript

```javascript
const numbers = [4, 17, 9, 12];
let largest = numbers[0];
for (const number of numbers) {
  if (number > largest) {
    largest = number;
  }
}
console.log(largest);
```

```sortie
17
```

`let` announces a box whose content will change; `const`, a box that
always keeps the same value. The braces `{ }` replace Python's
indentation.

### In Ruby

```ruby
numbers = [4, 17, 9, 12]
largest = numbers[0]
numbers.each do |number|
  if number > largest
    largest = number
  end
end
puts largest
```

```sortie
17
```

The same shape, with `each` and `end`.

### In Rust

```rust
let numbers = [4, 17, 9, 12];
let mut largest = numbers[0];
for number in numbers {
    if number > largest {
        largest = number;
    }
}
println!("{largest}");
```

```sortie
17
```

The shape is the same. Rust asks for `mut`: the box `largest` is going to
change its content, and Rust wants you to say so.

### In Rythme

In Rythme, a value never changes: you don't fill the box several times.
Instead, you describe **one step** ("between the current largest and a
number, keep the larger one"), then apply it to the whole list with
`fold`:

```rythme
fn keep_the_largest(largest, number) = if number > largest then number else largest

numbers = [4, 17, 9, 12]
print(fold(numbers, numbers[0], keep_the_largest))
```

```sortie
17
```

`fold` starts from `numbers[0]`, then passes each number to
`keep_the_largest`, along with the result of the previous step. It's the
same algorithm, said differently. See [thinking functionally](../rythme/fonctionnel.md).

### The shortcut

Finding the largest is so common that every language already does it:

```python
print(max([4, 17, 9, 12]))
```

```sortie
17
```

```javascript
console.log(Math.max(...[4, 17, 9, 12]));
```

```sortie
17
```

```ruby
puts [4, 17, 9, 12].max
```

```sortie
17
```

```rust
let numbers = [4, 17, 9, 12];
println!("{:?}", numbers.iter().max());
```

```sortie
Some(17)
```

```rythme
print(max([4, 17, 9, 12]))
```

```sortie
17
```

Rust answers `Some(17)`: for an empty list, it would answer `None`,
because there is no largest number. The other languages each react in
their own way to an empty list: Python raises an error, JavaScript answers
`-Infinity`. Use the shortcuts in your real programs; write the algorithm
yourself to learn.

## Testing by hand with a table of values

Before trusting an algorithm, follow it **by hand**, with paper and a
pencil. Write down the value of each variable at each round.

For the list `4, 17, 9, 12`:

| Round | number | number > largest?     | largest after    |
|-------|--------|-----------------------|------------------|
| start | –      | –                     | 4                |
| 1     | 4      | no (4 > 4 is false)   | 4                |
| 2     | 17     | yes                   | 17               |
| 3     | 9      | no                    | 17               |
| 4     | 12     | no                    | 17               |

At the end, `largest` is 17: that's right.

Then try **tricky** cases:

- the largest is the **first** one: `20, 3, 5`;
- the largest is the **last** one: `3, 5, 20`;
- all numbers are **negative**: `-8, -2, -5`;
- the list has **only one** number: `7`;
- the list is **empty**.

The last case is a problem: there is no "first number". A good programmer
thinks about these cases before the program crashes.

> **Tip:** in Rythme, `rythme steps file.ry` shows the calculation step by
> step, like your table.

## Common mistakes

### Starting from zero instead of the first number

It's tempting: you put `0` in `largest` at the start. But with negative
numbers, none is larger than 0.

```ruby norun
numbers = [-8, -2, -5]
largest = 0
numbers.each do |number|
  largest = number if number > largest
end
puts largest
```

```text
0
```

There's no error message: it's a **logic error**. The program answers 0,
which isn't even in the list. Start from the first number:
`largest = numbers[0]`.

### Forgetting the empty list case

With an empty list, `numbers[0]` doesn't exist. Rust stops the program:

```rust norun
fn main() {
    let numbers: Vec<i32> = vec![];
    let largest = numbers[0];
    println!("{largest}");
}
```

```text
thread 'main' (3304) panicked at main.rs:3:26:
index out of bounds: the len is 0 but the index is 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Check first that the list isn't empty, or use the shortcut
`numbers.iter().max()`, which answers `None` for an empty list.

### A "while" loop that never stops

If nothing changes inside the loop, the condition stays true forever.

```ruby norun
i = 0
while i < 3
  puts i
end
```

The program displays `0` endlessly. Stop it with **Ctrl + C**:

```text
loop.rb:3:in `write': Interrupt
	from loop.rb:3:in `puts'
	from loop.rb:3:in `puts'
	from loop.rb:3:in `<main>'
```

`i = i + 1` was missing in the loop. In your table of values, you would
have seen that `i` never changes.

## Test yourself

**1.** What is pseudocode, and what is it for?

> **Answer**
>
> It's an algorithm written in simple English, shaped like a program. It
> helps you think about the method without worrying about syntax; you then
> translate it into the language of your choice.

**2.** Write the pseudocode to find the **smallest** number in a list.

> **Answer**
>
> ```text
> put the first number in the box "smallest"
> for each number in the list
>     if number is smaller than smallest
>         put number in the box "smallest"
> display smallest
> ```
>
> Only the comparison changes: "smaller" instead of "larger".

**3.** With the list `5, 2, 8`, which values does `largest` take in our
algorithm?

> **Answer**
>
> 5 at the start, 5 after the first round (5 isn't larger than 5), 5 after
> the second (2 is smaller), then 8 after the third. The result is 8.

## Summary

- An algorithm is a sequence of clear steps, in the right order, that stops.
- Break a big problem down into small steps, then write them as pseudocode.
- A variable is a box with a label; `=` stores a value in it.
- Conditions choose, repetitions start over.
- The same algorithm can be written in Python, JavaScript, Ruby, Rust or Rythme: only the words change.
- Test by hand with a table of values, without forgetting the tricky cases.

Next: [the terminal in 10 minutes](terminal.md).
