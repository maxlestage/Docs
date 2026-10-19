# Loops

This page shows how to repeat an action without a loop, with `each`, `each_with_index`, `map` and recursion.

> **Goals:**
>
> - you will know why Rythme has neither `for` nor `while`, and what to write instead;
> - you will know how to repeat an action without a loop, with `each`, `each_with_index` and `map`;
> - you will know how to write a "while" repetition with a recursive function.

This chapter follows [conditions](conditions-boucles.md): you know how to choose a value with `if` and `match`, and now you will learn how to repeat an action.

## No loops

Rythme has neither `for` nor `while`. If you write one, Rythme explains
what to do instead (see [common mistakes](#common-mistakes)).

Instead of saying "for each element, do this", you say "do this to the
whole list". It's like saying "wash all the plates" instead of "take plate
1, wash it, take plate 2...". Here are the tools.

## Doing something for each element with each

Imagine you are asked to print each item of a shopping list. Writing one `print` per
item would be long and tedious. `each` does this work for you.

`each` calls a function for each element. It replaces the loop that prints.
Here, `1..3` is the **range** of numbers from 1 to 3, that is the list
`[1, 2, 3]`, and `fn(i) = ...` is a small function without a name (see
[functions](fonctions.md#anonymous-functions)):

```rythme
1..3 |> each(fn(i) = print("Round number {i}"))
```

```sortie
Round number 1
Round number 2
Round number 3
```

The same thing, Ruby style, with a `do |name| ... end` **block**: `|name|`
gives a name to the current element.

```rythme
["Ada", "Alan"].each do |name|
  puts "Hello, #{name}!"
end
```

```sortie
Hello, Ada!
Hello, Alan!
```

**Your turn:** write a program that prints "To buy: …" for each item of the list
`["bread", "milk", "rice"]`, using `each`.

> **Solution**
>
> ```rythme
> ["bread", "milk", "rice"] |> each(fn(item) = print("To buy: {item}"))
> ```
>
> ```sortie
> To buy: bread
> To buy: milk
> To buy: rice
> ```

## Counting by 2, or backwards

> **Note:** this part can wait. Read it once the basics feel solid.

`a..b` counts up by 1. `step_by(n)` keeps one element out of every `n`, and
`reverse` turns the list around:

```rythme
print(0..10 |> step_by(2))
print(reverse(1..5))
print(5..1)
```

```sortie
[0, 2, 4, 6, 8, 10]
[5, 4, 3, 2, 1]
[]
```

> **Warning:** `5..1` is empty. To count down, write `reverse(1..5)`.

## Knowing the position with each_with_index

Sometimes you want to number what you go through, like the steps of a recipe or a
numbered shopping list.

`each_with_index` gives the function the element **and** its position,
starting from 0:

```rythme
["bread", "milk", "eggs"].each_with_index do |item, i|
  print("{i + 1}. {item}")
end
```

```sortie
1. bread
2. milk
3. eggs
```

Line by line:

- the list holds three items; `each_with_index` takes them one by one;
- on the first round, `item` is `"bread"` and `i` is 0; on the second, `"milk"` and
  1; on the third, `"eggs"` and 2;
- `print` shows `i + 1` so that the list starts at 1, the way people usually count.

## Transforming each element with map

Sometimes you want to change a whole list at once: double every price, square every
number. It's like a machine where each object goes in on one side and comes out
transformed on the other.

`map` builds a **new** list, with the result of the function for each
element. It replaces the loop that fills a list:

```rythme
squares = 1..5 |> map(fn(x) = x * x)
print(squares)
```

```sortie
[1, 4, 9, 16, 25]
```

`each` is for **doing** something (printing); `map` is for **calculating**
a new list.

## Repeating with recursion

Some repetitions have no ready-made list: you repeat "while" the job is not done,
like a countdown that goes down to zero.

For a "while" loop, write a function that calls itself: this is
[recursion](../debuter/glossaire.md#recursion). There must always be a case
where it stops, otherwise it never stops:

```rythme
fn countdown(0) = print("Liftoff!")
fn countdown(n)
  print(n)
  countdown(n - 1)
end

countdown(3)
```

```sortie
3
2
1
Liftoff!
```

Line by line:

- `fn countdown(0) = ...`: if the number received is 0, the function prints
  "Liftoff!" and stops there;
- `fn countdown(n)`: for any other number, it prints `n`, then calls itself with
  `n - 1`;
- `countdown(3)` starts everything: 3 is printed, then the call goes on with 2, then
  1, then 0, which prints "Liftoff!".

Here, the function has two definitions: one for `0` (the stopping case),
one for the other numbers. `countdown(3)` prints 3 then calls
`countdown(2)`, and so on down to 0. See
[functions](fonctions.md#defining-by-cases).

**Your turn:** write a recursive function `applaud(n)` that prints "clap" `n` times,
then "Bravo!". Call `applaud(3)`.

> **Solution**
>
> ```rythme
> fn applaud(0) = print("Bravo!")
> fn applaud(n)
>   print("clap")
>   applaud(n - 1)
> end
>
> applaud(3)
> ```
>
> ```sortie
> clap
> clap
> clap
> Bravo!
> ```

## Common mistakes

### Writing a for loop

```rythme norun
numbers = [1, 2, 3]
for n in numbers
  print(n)
end
```

```text
error[E001]: `for` is not used in Rythme
 --> main.ry:2:1
  |
2 | for n in numbers
  | ^^^ comes from another language
  |
  = help: Rythme has no loops. Work on the whole list instead:
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      or write a recursive function. See `rythme help functional`.
  = more about this error: `rythme explain E001`
```

Write `numbers.each do |n| print(n) end`, or
`numbers |> each(fn(n) = print(n))`. `while` gives the same error.

### Using each instead of map

```rythme norun
squares = 1..3 |> each(fn(x) = x * x)
print(squares |> map(fn(x) = x + 1))
```

```text
error[E005]: `map` expects a list, a set, an Option or a Result as argument 1, but got a value of type Unit
 --> main.ry:2:7
  |
2 | print(squares |> map(fn(x) = x + 1))
  |       ^^^^^^^ wrong kind of value
  |
  = help: usage: `map(list, f)`
  = more about this error: `rythme explain E005`
```

`each` does something for each element, but does not build a list: it gives the
empty value `()`, of type `Unit`. To calculate a new list, use `map`:
`squares = 1..3 |> map(fn(x) = x * x)`.

## Test yourself

**Question 1:** how do you print the even numbers from 2 to 10, without a
loop?

> **Answer**
>
> For example `2..10 |> step_by(2) |> each(fn(n) = print(n))`, or
> `2..10 |> filter(is_even) |> each(fn(n) = print(n))`.

**Question 2:** what does `["a", "b"] |> map(fn(x) = x + "!")` do, and how is it different from `each`?

> **Answer**
>
> It builds a new list, `["a!", "b!"]`. `each` calls the function for each
> element but gives no list: it is for **doing** something (printing), `map` is
> for **calculating**.

**Question 3:** in the countdown, what happens if you remove the line
`fn countdown(0) = print("Liftoff!")`?

> **Answer**
>
> The function never stops: after 1, it prints 0, then -1, -2… forever. A
> recursive function always needs a case where it stops.

## Summary

- There is no `for` and no `while`: you work on the whole list.
- `each` does something for each element, `each_with_index` also gives its
  position, `map` transforms each element and gives a new list.
- `step_by` counts in steps, `reverse` counts backwards.
- For "while", you write a function that calls itself, with a case where it
  stops.

Next: [functions](fonctions.md).
