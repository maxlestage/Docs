# Solved exercises

Twelve exercises to practice the whole Rythme track, from easiest to hardest, each one with a hint and a checked solution.

## How to do these exercises

Each exercise has a level:

- ★☆☆ easy: a few lines, a single idea;
- ★★☆ medium: you need to combine two or three ideas;
- ★★★ harder: take your time.

Write your solution in a file, for example `exo.ry`, then run it with
`rythme run exo.ry`. Only look at the solution after you have tried: you
learn by searching. There are often several good answers.

## The basics

### Exercise 1: Introduce yourself

Level: ★☆☆

Give a name to your first name and to your age, then print a sentence like
this one, with next year's age computed by the program:

```text
My name is Ada and I am 36 years old. Next year, I will be 37.
```

> **Hint:** a calculation can go between braces: `"{age + 1}"`. See
> [first steps](premiers-pas.md).

> **Solution**
>
> ```rythme
> first_name = "Ada"
> age = 36
> print("My name is {first_name} and I am {age} years old. Next year, I will be {age + 1}.")
> ```
>
> ```sortie
> My name is Ada and I am 36 years old. Next year, I will be 37.
> ```

### Exercise 2: Even or odd

Level: ★☆☆

Write a function `parity(n)` that gives `"even"` or `"odd"`.

> **Hint:** `n % 2` is the remainder of the division by 2. An `if` always
> has an `else`.

> **Solution**
>
> ```rythme
> fn parity(n) = if n % 2 == 0 then "even" else "odd"
>
> print(parity(4), parity(7), parity(0))
> ```
>
> ```sortie
> even odd even
> ```

### Exercise 3: The multiplication table

Level: ★☆☆

Print the table of 7, from `7 x 1 = 7` up to `7 x 5 = 35`.

> **Hint:** there is no `for` loop: use `1..5` and `each`. See
> [conditions and repetition](conditions-boucles.md).

> **Solution**
>
> ```rythme
> 1..5 |> each(fn(i) = print("7 x {i} = {7 * i}"))
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

### Exercise 4: The ticket price

Level: ★☆☆

A movie ticket costs 5 € under 12, 7 € under 18 and 10 € for everyone
else. Write `price(age)` with a definition by cases and guards.

> **Hint:** `fn price(age) if age < 12 = 5`, then the other cases
> below. See [guards](fonctions.md#guards).

> **Solution**
>
> ```rythme
> fn price(age) if age < 12 = 5
> fn price(age) if age < 18 = 7
> fn price(_) = 10
>
> print(price(8), price(15), price(40))
> ```
>
> ```sortie
> 5 7 10
> ```

### Exercise 5: Adding up to n

Level: ★★☆

Write a **recursive** function `total(n)` that adds the numbers from 1 to
`n`. For example, `total(4)` is `1 + 2 + 3 + 4 = 10`.

> **Hint:** two definitions: `total(0)` is 0, and `total(n)` is
> `n + total(n - 1)`.

> **Solution**
>
> ```rythme
> fn total(0) = 0
> fn total(n) = n + total(n - 1)
>
> print(total(4), total(100))
> print(sum(1..100))
> ```
>
> ```sortie
> 10 5050
> 5050
> ```
>
> The last line checks the result with `sum`, which does the same thing.

## Lists and records

### Exercise 6: The report card

Level: ★☆☆

With the list `grades = [12, 8, 15, 19, 10]`, print the average, the best
grade and the list of grades below 10.

> **Hint:** `sum`, `len`, `max` and `filter`. `/` gives a decimal
> number.

> **Solution**
>
> ```rythme
> grades = [12, 8, 15, 19, 10]
> print("Average:", sum(grades) / len(grades))
> print("Best:", max(grades))
> print("Below 10:", filter(grades, fn(g) = g < 10))
> ```
>
> ```sortie
> Average: 12.8
> Best: 19
> Below 10: [8]
> ```

### Exercise 7: Happy birthday

Level: ★★☆

Write `birthday(person)` that receives a record like
`{name: "Ada", age: 36}` and gives a **new** record that is one year
older.

> **Hint:** `{..p, age: ...}` copies `p` and changes one field. See
> [updating a record](collections.md#updating-a-record).

> **Solution**
>
> ```rythme
> fn birthday(p) = {..p, age: p.age + 1}
>
> ada = {name: "Ada", age: 36}
> older = birthday(ada)
> print(ada)
> print(older)
> ```
>
> ```sortie
> {name: "Ada", age: 36}
> {name: "Ada", age: 37}
> ```
>
> `ada` does not change: values never change.

### Exercise 8: The squares of the odd numbers

Level: ★★☆

Compute the sum of the squares of the **odd** numbers from 1 to 10,
chaining the steps with `|>`. Do it a second time with dots and a
`{ |x| ... }` block.

> **Hint:** `filter(is_odd)`, then `map`, then `sum`. See
> [thinking functionally](fonctionnel.md).

> **Solution**
>
> ```rythme
> with_pipe = 1..10
>   |> filter(is_odd)
>   |> map(fn(x) = x * x)
>   |> sum
>
> with_dots = (1..10)
>   .select(odd?)
>   .map { |x| x * x }
>   .sum
>
> print(with_pipe, with_dots)
> ```
>
> ```sortie
> 165 165
> ```
>
> 1 + 9 + 25 + 49 + 81 = 165.

## Types, errors and text

### Exercise 9: The drinks machine

Level: ★★☆

Create a type `Drink` with three variants: `Coffee`, `Tea` and
`Juice(fruit)`. Write `describe(drink)` that gives a text, with `match`.

> **Hint:** `type Drink = Coffee | Tea | Juice(fruit)`. In the `match`,
> `Juice(f)` gives the name `f` to the fruit. See [your own types](types.md).

> **Solution**
>
> ```rythme
> type Drink = Coffee | Tea | Juice(fruit)
>
> fn describe(drink) = match drink
>   Coffee -> "coffee, 1 €"
>   Tea -> "tea, 1 €"
>   Juice(f) -> "{f} juice, 2 €"
> end
>
> [Coffee, Juice("apple"), Tea] |> each(fn(d) = print(describe(d)))
> ```
>
> ```sortie
> coffee, 1 €
> apple juice, 2 €
> tea, 1 €
> ```

### Exercise 10: Looking up a student

Level: ★★☆

Write `grade_of(students, name)` that looks for a student by name in a
list of records. It gives `"Ada: 17"`, or `"Eve: unknown"` if the student
is not in the list.

> **Hint:** `find` gives an `Option`: `Some(student)` or `None`. Handle
> both cases with `match`. See [handling errors](erreurs.md).

> **Solution**
>
> ```rythme
> students = [{name: "Ada", grade: 17}, {name: "Alan", grade: 12}]
>
> fn grade_of(students, name) = match find(students, fn(s) = s.name == name)
>   Some(s) -> "{name}: {s.grade}"
>   None -> "{name}: unknown"
> end
>
> print(grade_of(students, "Ada"))
> print(grade_of(students, "Eve"))
> ```
>
> ```sortie
> Ada: 17
> Eve: unknown
> ```

### Exercise 11: Reading an age

Level: ★★★

Write `read_age(text)` that gives a `Result`:

- `Err(...)` if the text is not a number (let `parse_int` give the
  error);
- `Err("impossible age")` if the number is negative or greater than 150;
- `Ok(age)` otherwise.

> **Hint:** `age = parse_int(text)?` reads the number, or stops the
> function with the error. Then, an `if` gives `Ok` or `Err`.

> **Solution**
>
> ```rythme
> fn read_age(text)
>   age = parse_int(text)?
>   if age < 0 or age > 150 then Err("impossible age") else Ok(age)
> end
>
> print(read_age("36"))
> print(read_age("-4"))
> print(read_age("thirty"))
> ```
>
> ```sortie
> Ok(36)
> Err("impossible age")
> Err("\"thirty\" is not a whole number")
> ```

### Exercise 12: Palindromes

Level: ★★★

A **palindrome** reads the same both ways, like "kayak". Write
`palindrome?(sentence)` that ignores both capital letters and spaces:
"Never odd or even" is a palindrome.

> **Hint:** convert to lowercase with `lower`, remove the spaces with
> `replace(text, " ", "")`, then compare with `reverse(text)`. See
> [working with text](texte.md).

> **Solution**
>
> ```rythme
> fn palindrome?(sentence)
>   clean = sentence |> lower |> replace(" ", "")
>   clean == reverse(clean)
> end
>
> print(palindrome?("kayak"))
> print(palindrome?("Never odd or even"))
> print(palindrome?("Rythme"))
> ```
>
> ```sortie
> true
> true
> false
> ```

## Going further

Rythme itself contains **30 more** graded exercises, checked by tests. Run
them in a terminal:

```sh
rythme exercise list                  # the list of the 30 exercises
rythme exercise start 1               # writes 01-values.ry in the folder
rythme exercise check 01-values.ry    # checks your solution
```

Keep [the cheat sheet](aide-memoire.md) open while you search. The other
tracks have their exercises too: [in Ruby](../ruby/exercices.md) and
[in Rust](../rust/exercices.md).

Next: [solved exercises: second set](exercices-2.md)
