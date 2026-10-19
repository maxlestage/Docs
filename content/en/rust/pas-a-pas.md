# Following a program step by step

This page teaches you to run a small Rust program in your head, line by line, with a table that tracks the value of each variable.

> **Goals:**
>
> - read a program line by line, in the order the computer runs it;
> - fill in a trace table: one column per variable, one row per step;
> - follow a condition, a loop and a function call;
> - guess a program's output **before** running it;
> - make the program show its own steps with `println!` and `dbg!`.

## Why follow a program by hand

A computer guesses nothing. It reads your lines **one by one**, from top to bottom, and
does exactly what is written. To understand a program, the best thing is to do the
same: play the computer, with a pencil.

Think of a cooking recipe. You don't read the whole recipe at once: you do step 1, then
step 2, and you look at what is in the bowl after each step. A program works the same
way. The bowl is the [variables](../debuter/glossaire.md#variable).

Knowing how to do this will help you all the time:

- to **understand** an example you are reading;
- to **find a bug**: the place where your table and reality no longer agree;
- to **solve the exercises** without trying everything at random.

> **Remember:** following a program means doing what the computer does: one line at a
> time, writing down the value of each variable after each line.

## The trace table method

A **trace table** has one column per variable and one row per step. For each line of
code that runs, you write down the new values. An empty cell means: "not created yet".

```text
   the code                   the table
 ┌──────────────────┐     ┌──────┬────┬────┐
 │ let a = 2;       │ ──▶ │ Step │ a  │ b  │
 │ let b = a + 1;   │     ├──────┼────┼────┤
 │ ...              │     │  1   │ 2  │    │
 └──────────────────┘     │  2   │ 2  │ 3  │
                          └──────┴────┴────┘
```

Three simple rules:

1. **One line of code at a time.** Never two.
2. **Always read the current value** in the table: the row above.
3. **A variable only changes if a line modifies it** (with `=` or `+=`). Otherwise, copy
   its value down.

For each program on this page, do this:

1. read the code;
2. **guess** what is printed, and write it down;
3. fill in the table on paper;
4. compare with our table, then with the real output.

> **Remember:** one column per variable, one row per step. The value to use is always
> the most recent one.

## Program 1: variables that change

Here is a little magic trick with two variables.

```rust
fn main() {
    let mut a = 3;
    let mut b = 5;
    a = a + b;
    b = a - b;
    a = a - b;
    println!("a = {a}, b = {b}");
}
```

Before running it, guess what is printed. Write your answer down.

The word `mut` means the variable is allowed to change
([mutable](../debuter/glossaire.md#mutable)). Let's follow each line:

| Step | Line | Calculation | a | b |
|------|------|-------------|---|---|
| 1 | `let mut a = 3;` | | 3 | |
| 2 | `let mut b = 5;` | | 3 | 5 |
| 3 | `a = a + b;` | 3 + 5 = 8 | **8** | 5 |
| 4 | `b = a - b;` | 8 − 5 = 3 | 8 | **3** |
| 5 | `a = a - b;` | 8 − 3 = 5 | **5** | 3 |

```sortie
a = 5, b = 3
```

The two values have been **swapped**. Look at step 4: `a` is already 8, not 3. You
always use the **most recent** value, the one in the row above.

It's like two glasses: to swap their contents without a third glass, you pour
everything into the first one, then take back the right amount.

> **Try it:** replace `let mut b = 5;` with `let mut b = 10;`. Redo the table. You
> should get `a = 10, b = 3`.

> **Remember:** at each step, you read the **current** values of the variables, not
> their starting values.

## Program 2: a condition

A [condition](../debuter/glossaire.md#condition) picks **a single path**. The other
blocks are skipped, like a railway switch.

```rust
fn main() {
    let temperature = 14;
    let mut outfit = "t-shirt";
    if temperature < 10 {
        outfit = "coat";
    } else if temperature < 20 {
        outfit = "sweater";
    }
    println!("{temperature} degrees: {outfit}");
}
```

Before running it, guess what is printed.

For a condition, add a "Test" column: the question asked and its answer (`true` or
`false`).

| Step | Line | Test | temperature | outfit |
|------|------|------|-------------|--------|
| 1 | `let temperature = 14;` | | 14 | |
| 2 | `let mut outfit = "t-shirt";` | | 14 | t-shirt |
| 3 | `if temperature < 10` | 14 < 10? **false** | 14 | t-shirt |
| 4 | `else if temperature < 20` | 14 < 20? **true** | 14 | t-shirt |
| 5 | `outfit = "sweater";` | | 14 | **sweater** |
| 6 | `println!` | | 14 | sweater |

```sortie
14 degrees: sweater
```

The line `outfit = "coat";` does **not** appear in the table: its test was false, so
Rust skips that block.

```text
          temperature < 10 ?
           /            \
        yes              no
         |                |
      "coat"       temperature < 20 ?
                     /          \
                  yes            no
                   |              |
               "sweater"     (nothing: stays "t-shirt")
```

> **Try it:** set `let temperature = 25;`. Both tests are false: no block runs, and the
> program prints `25 degrees: t-shirt`.

> **Remember:** in a condition, write down the test and its answer. Only the block whose
> test is true goes into the table.

## Program 3: a loop that accumulates

A [loop](../debuter/glossaire.md#loop) repeats a block. In the table, each **turn** of
the loop becomes one or more rows.

```rust
fn main() {
    let mut total = 0;
    for i in 1..5 {
        total += i * 2;
    }
    println!("total = {total}");
}
```

Before running it, guess what is printed. Watch out for `1..5`!

The range `1..5` gives 1, 2, 3 and 4. The 5 is **not** included (to include it, you
need `1..=5`).

| Turn | i | Calculation | total after the turn |
|------|---|-------------|----------------------|
| before | | | 0 |
| 1 | 1 | 0 + 1 × 2 | 2 |
| 2 | 2 | 2 + 2 × 2 | 6 |
| 3 | 3 | 6 + 3 × 2 | 12 |
| 4 | 4 | 12 + 4 × 2 | 20 |
| end | | `i` would be 5: the loop stops | 20 |

```sortie
total = 20
```

`total` is like a piggy bank: it is created **before** the loop, only once, and each
turn drops a coin into it. If you created it inside the loop, it would be emptied on
every turn.

```text
 piggy bank: [0] ──+2──▶ [2] ──+4──▶ [6] ──+6──▶ [12] ──+8──▶ [20]
 turn:            1           2           3             4
```

> **Try it:** replace `1..5` with `1..=5`. There is one more turn (i = 5, +10): the
> program prints `total = 30`.

> **Remember:** for a loop, one row per turn. Write down the value of the loop variable
> **and** that of the accumulator at the end of the turn.

## Program 4: a function called twice

A [function](../debuter/glossaire.md#function) is a small machine: you give it a value,
it works in **its own corner**, then it gives back a result.

```rust
fn price_with_tax(net: i32) -> i32 {
    let tax = net / 5;
    net + tax
}

fn main() {
    let a = price_with_tax(100);
    let b = price_with_tax(30);
    println!("{a} + {b} = {}", a + b);
}
```

Before running it, guess what is printed.

The program **always** starts with `main`. When `main` calls `price_with_tax`, it
pauses and waits for the answer. Here is a drawing of the calls:

```text
 main                                price_with_tax
 ────                                ──────────────
 a = price_with_tax(100) ──100──▶   net = 100, tax = 20
         ◀────────────────120─────  returns 100 + 20
 a = 120

 b = price_with_tax(30)  ──30───▶   net = 30, tax = 6
         ◀────────────────36──────  returns 30 + 6
 b = 36

 println! prints 120 + 36 = 156
```

In the table, a "Where?" column says which function you are in:

| Step | Where? | Line | net | tax | a | b |
|------|--------|------|-----|-----|---|---|
| 1 | main | `price_with_tax(100)`: call | | | | |
| 2 | price_with_tax | `net` receives 100 | 100 | | | |
| 3 | price_with_tax | `let tax = net / 5;` | 100 | 20 | | |
| 4 | price_with_tax | `net + tax`: returns 120 | 100 | 20 | | |
| 5 | main | `let a = …` | | | 120 | |
| 6 | main | `price_with_tax(30)`: call | | | 120 | |
| 7 | price_with_tax | `net` receives 30 | 30 | | 120 | |
| 8 | price_with_tax | `let tax = net / 5;` | 30 | 6 | 120 | |
| 9 | price_with_tax | `net + tax`: returns 36 | 30 | 6 | 120 | |
| 10 | main | `let b = …` | | | 120 | 36 |

```sortie
120 + 36 = 156
```

The `println!` prints `{a}` and `{b}`, so the prices **with** tax (120 and 36), not 100
and 30. If you guessed `100 + 30 = …`, you read the arguments instead of the results.

Two things to notice:

- `net` and `tax` **disappear** when the function is done (empty cells at steps 5 and
  10). They only exist during the call;
- on the second call, `net` and `tax` start from scratch: the function doesn't remember
  the first call.

> **Try it:** add `let c = price_with_tax(a);` before the `println!`, and print `c`. The
> function receives 120, the tax is 24: `c` is 144.

> **Remember:** at each call, you "enter" the function with the value given, follow it
> to the result, then come back to the exact place of the call.

## Program 5: a list and a condition in the loop

Last program: a loop over a [list](../debuter/glossaire.md#list) (a `Vec`), with a
condition **inside** the loop. We count the big expenses (more than 10 €) and add them
up.

```rust
fn main() {
    let expenses = vec![4, 25, 12, 7, 30];
    let mut big = 0;
    let mut sum = 0;
    for e in &expenses {
        if *e > 10 {
            big += 1;
            sum += e;
        }
    }
    println!("{big} big expenses, total {sum}");
}
```

Before running it, guess what is printed.

`for e in &expenses` takes the cells **one by one**, from left to right. The `&` means
"look without taking": the list can still be used afterwards. The star in `*e` reads the
number through that `&`.

```text
 expenses: [ 4 | 25 | 12 |  7 | 30 ]
             ▲    ▲    ▲    ▲    ▲
 turn:       1    2    3    4    5
```

Each turn has a test. Add a column for it:

| Turn | e | e > 10? | big | sum |
|------|---|---------|-----|-----|
| before | | | 0 | 0 |
| 1 | 4 | false | 0 | 0 |
| 2 | 25 | **true** | 1 | 25 |
| 3 | 12 | **true** | 2 | 37 |
| 4 | 7 | false | 2 | 37 |
| 5 | 30 | **true** | 3 | 67 |

```sortie
3 big expenses, total 67
```

On turns 1 and 4, the test is false: `big` and `sum` don't move. You simply **copy**
their values down in the table.

> **Try it:** replace `*e > 10` with `*e >= 12`. The 12 still counts, but not the 4 or
> the 7: the output doesn't change. With `*e > 12`, on the other hand, you get
> `2 big expenses, total 55`.

> **Remember:** when a turn's test is false, the variables keep their value. Copy it
> down: don't skip the row in the table.

## Making the program show its steps

Is your table right? The program can tell you itself: just make it print its variables
while it works. It's like asking a cook to say out loud every move they make.

### With println! and a label

Add a `println!` in the loop, with a **label** so you know where each line comes from:

```rust
fn main() {
    let mut total = 0;
    for i in 1..5 {
        total += i * 2;
        println!("[turn {i}] total = {total}");
    }
    println!("total = {total}");
}
```

```sortie
[turn 1] total = 2
[turn 2] total = 6
[turn 3] total = 12
[turn 4] total = 20
total = 20
```

These are exactly the rows of our table for program 3. Without the `[turn …]` label,
you would only see numbers, without knowing which is which.

### With dbg!

Rust has a [macro](../debuter/glossaire.md#macro) made for this: `dbg!`. It prints the
file name, the line number, the variable name **and** its value. You don't have to
write a label.

```rust
fn main() {
    let mut total = 0;
    for i in 1..5 {
        total += i * 2;
        dbg!(i, total);
    }
    println!("total = {total}");
}
```

```sortie
total = 20
```

The output above has only one line! That's normal: `dbg!` does **not** write to the
normal output (*stdout*), but to the **error output** (*stderr*). In your terminal, with
`cargo run` (the file is then `src/main.rs`), you see both mixed together:

```text
[src/main.rs:5:9] i = 1
[src/main.rs:5:9] total = 2
[src/main.rs:5:9] i = 2
[src/main.rs:5:9] total = 6
[src/main.rs:5:9] i = 3
[src/main.rs:5:9] total = 12
[src/main.rs:5:9] i = 4
[src/main.rs:5:9] total = 20
total = 20
```

`[src/main.rs:5:9]` means: file `src/main.rs`, line 5, column 9.

Why two outputs? Imagine two pipes coming out of the program. The first carries the
**results**, the second the **messages for you**. If you send the results to a file
(`cargo run > result.txt`), the `dbg!` messages stay on the screen and don't clutter
the file.

```text
              ┌─────────────┐  stdout (println!) ──▶ results
 program ───▶ │             │
              └─────────────┘  stderr (dbg!)     ──▶ debugging messages
```

Bonus: `dbg!` **returns** the value you give it. You can wrap it around a calculation
without breaking anything:

```rust
fn price_with_tax(net: i32) -> i32 {
    let tax = net / 5;
    dbg!(net + tax)
}

fn main() {
    let a = price_with_tax(100);
    println!("a = {a}");
}
```

```sortie
a = 120
```

And on the error output, as well:

```text
[src/main.rs:3:5] net + tax = 120
```

> **Try it:** in program 5, add `dbg!(e, big, sum);` at the end of the loop's block.
> Run it with `cargo run`: you get your trace table, written by the program itself.

> **Remember:** `println!` with a label, or `dbg!(variable)`, turn the program into a
> trace table. `dbg!` writes to *stderr*: remember to remove it once you have found the
> problem.

## What about the debugger?

A [debugger](../debuter/glossaire.md#debugging) pauses the program on a line and shows you
all the variables, without adding anything to the code. It fills in your trace table
for you, one line at a time: see the section about a debugger in VS Code, in the chapter
[Debugging in Rust](debogage.md).

## Common mistakes

### Forgetting a step

In program 5, people often skip the turns where "nothing happens". The table then
becomes wrong, because you no longer know where you are in the list.

```text
Incomplete table (wrong):      Complete table (right):
| e  | big | sum |             | e  | big | sum |
| 25 | 1   | 25  |             | 4  | 0   | 0   |
| 12 | 2   | 37  |             | 25 | 1   | 25  |
| 30 | 3   | 67  |             | 12 | 2   | 37  |
                               | 7  | 2   | 37  |
                               | 30 | 3   | 67  |
```

Fix: **one row per turn**, even if nothing changes. Copy the values down.

### Reusing an old value

In program 1, at the line `b = a - b;`, many people compute 3 − 5 with the starting
`a`. But `a` is already 8:

```text
Wrong: b = 3 - 5 = -2     (the "a" from step 1)
Right: b = 8 - 5 = 3      (the "a" from step 3, the row just above)
```

Fix: always read the value in the **last** row of the table.

### One turn too many (or too few)

`for i in 1..5` only does **4** turns. Counting up to 5 gives a wrong total (30
instead of 20):

```text
1..5   → 1, 2, 3, 4        (the 5 is excluded)
1..=5  → 1, 2, 3, 4, 5     (the 5 is included)
```

Fix: before filling in the table, write down the list of values the loop variable
will take.

### Forgetting mut on a variable that changes

When copying a program to follow it, you sometimes forget `mut`. Rust then refuses to
compile it:

```rust norun
fn main() {
    let total = 0;
    for i in 1..5 {
        total += i * 2;
    }
    println!("total = {total}");
}
```

```text
error[E0384]: cannot assign twice to immutable variable `total`
 --> src/main.rs:4:9
  |
2 |     let total = 0;
  |         ----- first assignment to `total`
3 |     for i in 1..5 {
4 |         total += i * 2;
  |         ^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut total = 0;
  |         +++
```

Fix: `let mut total = 0;`. In your table too, it's a good landmark: only `mut`
variables can change from row to row.

### Giving the list to dbg! instead of lending it

`dbg!(expenses)` **takes** the list (we say it is [moved](possession.md)). After that,
it can no longer be used:

```rust norun
fn main() {
    let expenses = vec![4, 25, 12];
    dbg!(expenses);
    println!("{}", expenses.len());
}
```

```text
error[E0382]: borrow of moved value: `expenses`
 --> src/main.rs:4:20
  |
2 |     let expenses = vec![4, 25, 12];
  |         -------- move occurs because `expenses` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     dbg!(expenses);
  |     -------------- value moved here
4 |     println!("{}", expenses.len());
  |                    ^^^^^^^^ value borrowed here after move
  |
help: consider borrowing instead of transferring ownership
  |
3 |     dbg!(&expenses);
  |          +
```

Fix: lend it with `&`: `dbg!(&expenses);`.

## Test yourself

**1.** What is the value of `x` at the end of these three lines?
`let mut x = 2; x = x * 3; x = x - 1;`

> **Answer**
>
> | Step | x |
> |------|---|
> | 1 | 2 |
> | 2 | 2 × 3 = 6 |
> | 3 | 6 − 1 = 5 |
>
> `x` is **5**: each line starts from the value of the line before.

**2.** How many turns does `for i in 0..3` do, and what values does `i` take?

> **Answer**
>
> Three turns: `i` is 0, then 1, then 2. The 3 is excluded.

**3.** What does this program print? Make the table before looking at the answer.

```rust norun
fn main() {
    let mut count = 0;
    for n in [3, 8, 5, 10] {
        if n % 2 == 0 {
            count += n;
        }
    }
    println!("{count}");
}
```

> **Answer**
>
> | Turn | n | n even? | count |
> |------|---|---------|-------|
> | 1 | 3 | false | 0 |
> | 2 | 8 | true | 8 |
> | 3 | 5 | false | 8 |
> | 4 | 10 | true | 18 |
>
> The program adds up the even numbers. Here it is with its output:
>
> ```rust
> fn main() {
>     let mut count = 0;
>     for n in [3, 8, 5, 10] {
>         if n % 2 == 0 {
>             count += n;
>         }
>     }
>     println!("{count}");
> }
> ```
>
> ```sortie
> 18
> ```

## Summary

- Following a program means playing the computer: one line at a time, from top to
  bottom.
- The trace table has one column per variable and one row per step (or per turn).
- Always use the **most recent** value; copy down the values that don't change.
- For a function, enter it with the value it receives, follow it, then come back to the
  call point with the result.
- `println!` with a label or `dbg!` make the program show its steps; `dbg!` writes to
  the error output (*stderr*).

Next: [Beginner traps in Rust](pieges.md)
