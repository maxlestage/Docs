# Testing your code

A test is a small piece of code that checks that a function gives the right result: in Rythme, you write it next to the function, and `rythme test` runs them all at once.

> **Goals:**
>
> - write a test with `test "name" = ...` and `assert_eq`;
> - write a test of several lines, with `assert`;
> - run the tests with `rythme test`, and read the message of a failing test;
> - test a function that returns a `Result`.

## Why test

You write a function, you try it with a `print`, it works.
A week later, you change it... and without noticing, you break a case that
used to work.

A **test** keeps track of what must work: "double of 21 is 42". Instead of
trying everything again by hand, you run one command, and Rythme checks each
test for you, in a fraction of a second.

## A first test

A test starts with the word `test`, then its **name** between quotes, then
`=` and the check:

```rythme
fn double(x) = x * 2

test "double of 21 is 42" = assert_eq(double(21), 42)

print(double(5))
```

```sortie
10
```

`assert_eq(actual, expected)` checks that two values are equal. Here, the
test says: "`double(21)` must give `42`".

Run with `rythme run`, as usual, the program **ignores the tests**: only the
`print` runs. So tests can live in the same file as the code, without
changing anything about what it does.

> **Remember:** the name of a test is a sentence that says what you check.
> When a test fails, this name is what gets shown: it must be clear.

## Several checks in a block

A test can be written on several lines, without `=`, and ends with `end`.
It can then contain several checks:

```rythme
fn double(x) = x * 2

test "double keeps the sign"
  assert(double(-3) < 0, "double of -3 should be negative")
  assert_eq(double(0), 0)
  assert_eq(double(4), 8)
end

print(double(-3))
```

```sortie
-6
```

`assert(condition, message)` checks that a condition is true. If it is
false, the message is shown.

| Function | Checks that |
|----------|-------------|
| `assert_eq(actual, expected)` | the two values are equal |
| `assert(condition, message)` | the condition is true |

> **Tip:** prefer `assert_eq` when you compare two values: if it fails, it
> shows what it got and what it expected.

## Running the tests with rythme test

Here is a file `calculations.ry`, with two functions and three tests:

```rythme
fn double(x) = x * 2

fn average(grades) = sum(grades) / len(grades)

test "double of 21 is 42" = assert_eq(double(21), 42)

test "double keeps the sign"
  assert(double(-3) < 0, "double of -3 should be negative")
  assert_eq(double(0), 0)
end

test "average of 10, 12 and 14" = assert_eq(average([10, 12, 14]), 12.0)

print(double(5))
```

```sortie
10
```

To run its tests, use `rythme test` instead of `rythme run`:

```sh
rythme test calculations.ry
```

```text
Running 3 test(s)
  ✓ double of 21 is 42
  ✓ double keeps the sign
  ✓ average of 10, 12 and 14
3 test(s): 3 passed, 0 failed (1 ms)
```

Each `✓` is a passing test. The last line sums it up, with the time taken
between parentheses (it changes a little from one run to the next).

During `rythme test`, the top-level lines like `print(double(5))` are
**skipped**: only the tests run.

## When a test fails

Let's add a wrong test to `calculations.ry`, on purpose:

```rythme norun
test "a deliberate mistake" = assert_eq(double(3), 7)
```

```text
Running 4 test(s)
  ✓ double of 21 is 42
  ✓ double keeps the sign
  ✓ average of 10, 12 and 14
  ✗ a deliberate mistake
error[E015]: assertion failed: the values are different
  --> calculations.ry:16:31
   |
16 | test "a deliberate mistake" = assert_eq(double(3), 7)
   |                               ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: got:      6
       expected: 7
   = more about this error: `rythme explain E015`

4 test(s): 3 passed, 1 failed (0 ms)
```

Read it from top to bottom:

1. `✗` marks the test that failed, with its name;
2. the line and the column of the problem: `calculations.ry:16:31`;
3. what the function **got** (`6`) and what was **expected** (`7`);
4. the summary: 3 passed, 1 failed.

Here, the test is the one that is wrong: double of 3 really is 6. Sometimes,
it's the function: the test then saved you from a bug.

With `assert`, the message you wrote is shown. This test is wrong too: it
should have checked `not adult?(17)`.

```rythme norun
fn adult?(age) = age >= 18

test "18 years old is an adult" = assert(adult?(18), "18 years old should be an adult")
test "17 years old is a minor" = assert(adult?(17), "17 years old should be a minor")
```

```text
Running 2 test(s)
  ✓ 18 years old is an adult
  ✗ 17 years old is a minor
error[E015]: assertion failed: 17 years old should be a minor
 --> age.ry:4:34
  |
4 | test "17 years old is a minor" = assert(adult?(17), "17 years old should be a minor")
  |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this condition is false
  |
  = more about this error: `rythme explain E015`

2 test(s): 1 passed, 1 failed (0 ms)
```

> **Note:** when all the tests pass, `rythme test` ends with code 0;
> otherwise with code 1. Continuous integration tools (which run the tests
> on every change) rely on it.

## Choosing tests with --filter

When a file has many tests, `--filter` runs only those whose name contains a
word (ignoring upper and lower case):

```sh
rythme test calculations.ry --filter double
```

```text
Running 2 of 4 test(s)
  ✓ double of 21 is 42
  ✓ double keeps the sign
2 test(s): 2 passed, 0 failed, 2 left out by the filter (0 ms)
```

One more reason to give your tests clear names.

## Running again on every change with --watch

```sh
rythme test calculations.ry --watch
```

`--watch` runs the tests, then **waits**. Each time you save the file, the
tests run again on their own:

```text
Watching 1 file(s): the tests run again when one changes (Ctrl+C to stop).
```

Keep the terminal open next to your editor: you see the result every time
you save. `Ctrl+C` stops watching.

## Testing errors and Results

A function that can fail returns a `Result` (see
[handling errors](erreurs.md)). You test both cases with `assert_eq`, by
writing the expected `Ok(...)` or `Err(...)`:

```rythme
fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)

test "divide 10 by 4" = assert_eq(divide(10, 4), Ok(2.5))

test "dividing by zero gives an error" = assert_eq(divide(1, 0), Err("division by zero"))

test "two divisions in a row"
  half = divide(100, 2)?
  tenth = divide(half, 5)?
  assert_eq(tenth, 10.0)
end

print(divide(10, 4), divide(1, 0))
```

```sortie
Ok(2.5) Err("division by zero")
```

The third test uses `?`, as in a function: if a step gives `Err(...)`, the
test stops and **fails**, showing the error. For example, with this test, on
line 13 of the file `bank.ry`:

```rythme norun
test "a failed division"
  x = divide(100, 0)?
  assert_eq(x, 1)
end
```

```text
  ✗ a failed division
error[E015]: the test ended with Err("division by zero")
  --> bank.ry:13:1
   |
13 | test "a failed division"
   | ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: a test passes when it ends without error, and its value is not `false`, `Err(...)` or `None`
   = more about this error: `rythme explain E015`
```

The rule is simple: a test **passes** if it ends without error, and its
result is not `false`, `Err(...)` or `None`.

## Where to put your tests

**Next to the code.** It's the simplest way: the tests of a function sit
right below it, in the same file. When you change the function, its tests
are right in front of you.

**In a separate file.** For a module (see [modules](modules.md)), you can
also write the tests in another file, which uses it with `use`. Here is
`geometry_test.ry`, next to `geometry.ry`:

```rythme norun
use geometry

test "area of a 2 by 3 rectangle" = assert_eq(geometry.rectangle_area(2, 3), 6)

test "area of a square with side 4" = assert_eq(geometry.square_area(4), 16)
```

**A whole folder at once.** `rythme test .` runs the tests of all the `.ry`
files of the folder (and of its subfolders):

```sh
rythme test .
```

```text
Running 3 test(s) of ./calculations.ry
  ✓ double of 21 is 42
  ✓ double keeps the sign
  ✓ average of 10, 12 and 14
Running 2 test(s) of ./geometry_test.ry
  ✓ area of a 2 by 3 rectangle
  ✓ area of a square with side 4
2 file(s), 5 test(s): 5 passed, 0 failed (1 ms)
```

In a project created with `rythme new`, the tests go in the `tests/` folder,
and a plain `rythme test` runs them (see
[organizing a project](projet.md)).

## Common mistakes

### Forgetting the quotes around the name

```rythme norun
fn double(x) = x * 2

test double = assert_eq(double(21), 42)
```

```text
error[E001]: expected the name of the test, as a text, found the name `double`
 --> calculations.ry:3:6
  |
3 | test double = assert_eq(double(21), 42)
  |      ^^^^^^ expected the name of the test, as a text here
  |
  = help: example: `test "addition works" = assert_eq(1 + 1, 2)`
  = more about this error: `rythme explain E001`
```

Fix: the name is a text, `test "double of 21" = ...`.

### Writing a test inside a function

```rythme norun
fn double(x)
  test "double" = assert_eq(x * 2, 42)
  x * 2
end
```

```text
error[E001]: `test` can only be used at the top level of a file
 --> calculations.ry:2:3
  |
2 |   test "double" = assert_eq(x * 2, 42)
  |   ^^^^
  |
  = help: move it outside of the function or block
  = more about this error: `rythme explain E001`
```

Fix: write the test **after** the function, at the top level, without
indentation: `test "double of 21" = assert_eq(double(21), 42)`.

### Comparing with == instead of assert_eq

A test that ends with `false` fails, but without saying why:

```rythme norun
fn double(x) = x * 2

test "double of 3" = double(3) == 7
```

```text
Running 1 test(s)
  ✗ double of 3
error[E015]: the test returned `false`
 --> calculations.ry:3:1
  |
3 | test "double of 3" = double(3) == 7
  | ^^^^^^^^^^^^^^^^^^
  |
  = help: use `assert_eq(actual, expected)` to see the values
  = more about this error: `rythme explain E015`

1 test(s): 0 passed, 1 failed (0 ms)
```

Fix: `test "double of 3" = assert_eq(double(3), 6)`. If it fails,
`assert_eq` shows the actual value and the expected value.

## Test yourself

1. What does `rythme run` do with a file that contains tests?

> **Answer**
>
> It ignores the tests and runs the program normally. Only `rythme test`
> runs the tests (and then skips the other top-level lines, like the
> `print`s).

2. Write a test that checks that `upper("rythme")` gives `"RYTHME"`.

> **Answer**
>
> ```rythme
> test "upper capitalizes" = assert_eq(upper("rythme"), "RYTHME")
> ```

3. How do you run only the tests whose name contains "average"?

> **Answer**
>
> `rythme test calculations.ry --filter average`.

## Summary

- `test "name" = assert_eq(actual, expected)` writes a test on one line;
  `test "name"` ... `end` writes one of several lines.
- `assert_eq` compares two values, `assert(condition, message)` checks a
  condition.
- `rythme test file.ry` runs the tests; `rythme run` ignores them.
- A failing test shows its name, the line, and what was got and expected.
- `--filter word` chooses tests, `--watch` runs them again on every change,
  `rythme test .` runs the whole folder.
- A test fails if it stops on an error or ends with `false`, `Err(...)` or
  `None`: `?` is allowed in it.

Next: [writing good Rythme](style.md)
