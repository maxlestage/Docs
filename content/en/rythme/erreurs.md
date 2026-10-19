# Handling errors

Rythme has neither `null` nor exceptions: this page shows how a missing value becomes an `Option`, how a failure becomes a `Result`, and how to read error messages.

> **Goals:**
>
> - you will know how to use an `Option` for a value that may be missing;
> - you will know how to use a `Result` for an operation that may fail;
> - you will know how to open an `Option` or a `Result` with `match`, `unwrap_or` and `?`;
> - you will know how to transform these values with `map`, `and_then`, `ok_or` and `map_err`;
> - you will know how to read a Rythme error message and find its explanation.

## No null, no exceptions

An [error](../debuter/glossaire.md#error) is what prevents a program from
doing what you expect. In many languages, a value can be "null"
([null](../debuter/glossaire.md#null)), and an error can pop up from
anywhere, like an **exception** that stops everything.

Not in Rythme. Possible problems are ordinary **values**, which you handle
like any other:

- a value that may be missing is an `Option`: `Some(value)` or `None`;
- an operation that may fail gives a `Result`: `Ok(value)` or
  `Err(error)`.

Think of a parcel: before using what's inside, you open the box and check
whether it is full or empty. Rythme doesn't let you forget to check.

## Option: a value that may be missing

`first` gives the first element of a list... if there is one. `find` gives
the first element that matches... if there is one:

```rythme
print(first([10, 20]), first([]))
print(find([1, 4, 7], is_even), find([1, 3], is_even))
```

```sortie
Some(10) None
Some(4) None
```

`Some(10)` is a box that contains 10. `None` is an empty box.

You handle both cases with `match`:

```rythme
fn describe(list) = match first(list)
  Some(x) -> "starts with {x}"
  None -> "empty list"
end

print(describe([5, 6]), "/", describe([]))
```

```sortie
starts with 5 / empty list
```

## Result: an operation that may fail

`parse_int` reads a number from a text. The text may not be a number: so it
gives a `Result`, `Ok(number)` or `Err(message)`.

```rythme
print(parse_int("42"), parse_int("forty"))
```

```sortie
Ok(42) Err("\"forty\" is not a whole number")
```

`Option` only says "nothing"; `Result` also says **why** it failed.

Your functions can return a `Result` too:

```rythme
fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)

fn show(result) = match result
  Ok(v) -> "result: {v}"
  Err(e) -> "error: {e}"
end

print(show(divide(10, 4)))
print(show(divide(1, 0)))
```

```sortie
result: 2.5
error: division by zero
```

## A default value with unwrap_or

`unwrap_or` gives the value inside, or a default value if the box is empty
or contains an error:

```rythme
grades = dict([["Ada", 19]])
print(unwrap_or(get(grades, "Ada"), 0), unwrap_or(get(grades, "Eve"), 0))
print(parse_int("12") |> unwrap_or(0), parse_int("??") |> unwrap_or(0))
```

```sortie
19 0
12 0
```

> **Warning:** `unwrap(x)` gives the value, but stops the program on `None`
> or `Err` (error E013). Prefer `unwrap_or` or `match`.

## The ? operator to pass the error on

Inside a function, `?` after a value means:

- if it is `Ok(x)` or `Some(x)`, go on with `x`;
- if it is `Err(...)` or `None`, **stop the function** and return that
  value.

```rythme
fn add(a, b)
  x = parse_int(a)?
  y = parse_int(b)?
  Ok(x + y)
end

print(add("1", "2"))
print(add("1", "two"))
```

```sortie
Ok(3)
Err("\"two\" is not a whole number")
```

Without `?`, you would need a `match` for each reading. With `?`, the normal
path reads in one go, and the error goes back up on its own, like a damaged
parcel sent back to the sender.

> **Note:** `?` only works inside a function (or a test). The function then
> returns a `Result` or an `Option` itself: hence the `Ok(x + y)` at the
> end.

## Transforming without opening the box

These functions work on the content, and leave `None` or `Err` as they
are:

| Function | What it does |
|----------|--------------|
| `map(x, f)` | applies `f` to the value in `Some` or `Ok` |
| `and_then(x, f)` | one more step that may fail: `f` gives an Option or a Result |
| `ok_or(option, error)` | turns an Option into a Result: `None` becomes `Err(error)` |
| `map_err(result, f)` | changes the message of an `Err` |

```rythme
fn positive(n) = if n > 0 then Ok(n) else Err("not positive")

print(Some(3) |> map(fn(x) = x * 10), None |> map(fn(x) = x * 10))
print(parse_int("8") |> and_then(positive), parse_int("-2") |> and_then(positive))
print(first([]) |> ok_or("empty list"))
print(parse_int("x") |> map_err(fn(e) = "bad input"))
```

```sortie
Some(30) None
Ok(8) Err("not positive")
Err("empty list")
Err("bad input")
```

`ok_or` is handy with `?`: in a function that returns a `Result`,
`first(list) |> ok_or("empty list")?` gives the first element, or stops the
function with a clear message.

## Other tools

| Function | What it does |
|----------|--------------|
| `is_some(x)`, `is_none(x)` | does the Option contain a value? |
| `is_ok(x)`, `is_err(x)` | is the Result a success? |
| `fail(message)` | stops the program with a message |

```rythme
print(is_some(first([1, 2])), is_none(first([])))
print(is_ok(parse_int("7")), is_err(parse_int("x")))
```

```sortie
true true
true true
```

Keep `fail` for impossible situations, when the program really cannot go
on.

## Error messages that help

When something goes wrong, Rythme tells you **where**, **why**, and **how
to fix it**. For example, this program adds a number to a text:

```rythme norun
age = 36
print("age: " + age)
```

```text
error[E005]: cannot use `+` with Text and Int
 --> main.ry:2:15
  |
2 | print("age: " + age)
  |               ^ Text "age: " + Int
  |
  = help: to build a text with a value inside, use braces: "total: {x}"
      or convert the value first: text(x)
  = more about this error: `rythme explain E005`
```

In it, you read:

1. the error **code** (`E005`) and what happened;
2. the file, the line and the column (`main.ry:2:15`);
3. the line of code, with the exact spot marked by `^`;
4. some **help** to fix it;
5. the command to learn more.

This error is found **before** the program runs: nothing was started. A
message that starts with `warning` points out a likely problem, but the
program runs anyway.

## Explaining an error code

Each code has a detailed explanation, with examples:

```sh
rythme explain E005
```

```text
E005 — Wrong kind of value

The operation does not work with this kind of value. Rythme never converts
values silently: "1" + 1 is an error, not "11" or 2.

Wrong:   "age: " + 36
Right:   "age: {36}"              (insert the value in the text)
Right:   "age: " + text(36)       (convert it first)
```

Here are the most common codes when you start:

| Code | Meaning |
|------|---------|
| E001 | syntax error (often a word from another language: `for`, `return`, `null`...) |
| E003 | unknown name (often a typo: Rythme suggests the right name) |
| E004 | wrong number of arguments |
| E005 | wrong kind of value |
| E006 | no case matches |
| E007 | index out of bounds |
| E011 | name already defined (a value does not change) |
| E019 | `if` without `else` |

## Common mistakes

### Looking for null

```rythme norun
name = null
```

```text
error[E001]: `null` is not used in Rythme
 --> main.ry:1:8
  |
1 | name = null
  |        ^^^^ comes from another language
  |
  = help: Rythme has no null: use `None` for "no value" and `Some(x)` for a value
  = more about this error: `rythme explain E001`
```

A value that may be missing is written `None`, and `Some("Ada")` when it is
there.

### Computing with an Option without opening it

```rythme norun
grades = [12, 15]
print(first(grades) + 1)
```

```text
error[E005]: cannot use `+` with Option and Int
 --> main.ry:2:21
  |
2 | print(first(grades) + 1)
  |                     ^ Option + Int 1
  |
  = more about this error: `rythme explain E005`
```

`first(grades)` is the box `Some(12)`, not the number 12. Open it:
`unwrap_or(first(grades), 0) + 1`, or with `match`.

### Using ? outside a function

```rythme norun
n = parse_int("twelve")?
print(n)
```

```text
error[E016]: `?` can only be used inside a function
 --> main.ry:1:5
  |
1 | n = parse_int("twelve")?
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = help: outside of a function, handle the value with `match` or `unwrap_or`
  = more about this error: `rythme explain E016`
```

`?` sends the error back to the calling function; at the top level of the
program, there is none. Use `match`, or `unwrap_or`.

### Forcing the box open with unwrap

```rythme norun
print(unwrap(parse_int("twelve")))
```

```text
error[E013]: `unwrap` found an error: "twelve" is not a whole number
 --> main.ry:1:7
  |
1 | print(unwrap(parse_int("twelve")))
  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: handle the error with `match`, pass it on with `?`, or give a default: `unwrap_or(x, default)`
  = more about this error: `rythme explain E013`
```

`unwrap` assumes everything is fine, and stops the program otherwise. Plan
for the failure case: `parse_int("twelve") |> unwrap_or(0)`.

## Test yourself

**Question 1:** what do `get([1, 2], 5)` and `get([1, 2], 0)` give?

> **Answer**
>
> `None` and `Some(1)`. Position 5 does not exist: the box is empty.

**Question 2:** when should you use `Result` rather than `Option`?

> **Answer**
>
> When you want to say **why** it failed: `Err("file
> not found")` carries a message, `None` says nothing.

**Question 3:** what does `add("5", "x")` return, with the `add` function
from this page?

> **Answer**
>
> `Err("\"x\" is not a whole number")`. The first `?` lets 5 through, the
> second one meets an error and returns it right away.

## Summary

- A value that may be missing is an `Option`: `Some(x)` or `None`.
- An operation that may fail gives a `Result`: `Ok(x)` or `Err(e)`.
- You open them with `match`, or with `unwrap_or` for a default value;
  `map`, `and_then`, `ok_or` and `map_err` transform them.
- Inside a function, `?` goes on with the value, or returns the error.
- Every error message says where, why and how to fix it;
  `rythme explain CODE` tells you more.

Next: [organizing your code in modules](modules.md). Compare with errors in
[Ruby](../ruby/erreurs.md) and in [Rust](../rust/erreurs.md).
