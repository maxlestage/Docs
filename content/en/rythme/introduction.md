# Rythme: introduction and installation

Rythme is a small, purely functional language with a Ruby flavor, designed to help you learn programming without getting lost: this page explains what it is, how to install it and how to run your first program.

> **Goals:**
>
> - you will be able to say what Rythme is and what makes it different;
> - you will know how to install Rythme, or try it without installing anything;
> - you will know how to try a line of code in interactive mode;
> - you will know how to write a `.ry` file, check it and run it;
> - you will know how to read your first error message.

## What is Rythme

A **programming language** is a precise language that the computer knows how
to follow. A **program** is a text written in that language: a series of
instructions, like a cooking recipe.

Rythme is one of these languages. Rythme programs are text files whose names
end in `.ry`. A program, the `rythme` **interpreter**, reads them and runs
them (see [interpreter](../debuter/glossaire.md#interpreter)). This
interpreter is written in another language, Rust.

Here is a complete program:

```rythme
fn square(x) = x * x

[1, 2, 3, 4]
  |> filter(is_even)
  |> map(square)
  |> sum
  |> print
```

```sortie
20
```

Read it from top to bottom, like an assembly line:

1. `fn square(x) = x * x` defines a [function](../debuter/glossaire.md#function)
   `square`: it receives a number `x` and gives back `x * x`;
2. `[1, 2, 3, 4]` is a [list](../debuter/glossaire.md#list) of numbers;
3. `|>` passes the value on to the next step: we keep the even numbers
   (`[2, 4]`), square each one (`[4, 16]`), add them up (`20`),
   and print the result.

You don't need to understand everything right now: each piece has its own
chapter.

## The philosophy of the language

Rythme wants to be **simpler than Python or Ruby**. It has only 15
**keywords** (words reserved by the language, like `if` or `fn`), and just
one way to do each thing.

- **Purely functional.** A value never changes: it is
  [immutable](../debuter/glossaire.md#immutable). There are no loops: you
  work on whole lists, or with functions that call themselves.
- **Safe like Rust.** No [`null`](../debuter/glossaire.md#null), no
  exceptions. A value that may be missing is an `Option`, an operation that
  may fail gives a `Result`.
- **Pleasant like Ruby.** `do |x| ... end` blocks, calls without
  parentheses (`puts "hi"`), names like `even?`.
- **Help everywhere.** Every error says where it is, why, and how to fix
  it. The program is checked before it runs.
- **Six languages.** Messages exist in French, English, Spanish, German,
  Portuguese and Chinese.

Think of Rythme as a tidy kitchen: fewer utensils, but each one has its
place, and nothing moves behind your back.

> **Note:** keywords and built-in functions are in English
> (`if`, `map`, `print`), and your own names can be in any language,
> accents included: `prénom`, `âge`.

## Installing Rythme

You install Rythme by typing a command in a **terminal**, the window where
you type commands instead of clicking. If you have never used one, read
[the terminal](../debuter/terminal.md) first.

On **Linux and macOS**, this command downloads Rythme and installs it:

```sh
curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh
```

On **Windows**, in PowerShell:

```text
irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex
```

Then check that everything works:

```sh
rythme version
```

```text
rythme 0.1.0
```

If the terminal answers "command not found", close it, open it again, and
try again: it needs to reread the list of installed programs.

## Trying it without installing anything

The [Rythme website](https://maxlestage.github.io/Rythme/) has a **playground**: you
write code in the page, run it, and see the result. It is the real
interpreter, running in the browser. The only difference: it cannot read or
write files on your computer.

## Interactive mode

Type `rythme` with nothing else: this is **interactive mode** (also called
a [REPL](../debuter/glossaire.md#repl)). You type a line, and Rythme shows
the result right away. It's a very clever calculator.

```text
Rythme 0.1.0 — :help for help, :quit to leave
ry> 1 + 2
3
ry> name = "Ada"
ry> "Hello, {name}"
"Hello, Ada"
ry> :quit
```

`ry>` is the **prompt**: Rythme is waiting for your line. You don't type it.

> **Tip:** in interactive mode, `:doc map` shows the help for the `map`
> function, and `:quit` exits.

## Your first program

Interactive mode forgets everything when you leave it. To keep a program,
you write it in a file. With your
[code editor](../debuter/editeur.md), create a file `hello.ry` with this
content:

```rythme
# My first program
print("Hello, world!")
```

```sortie
Hello, world!
```

- The line that starts with `#` is a **comment**: a note for humans, which
  Rythme ignores.
- `print` is a function that displays what you give it between parentheses.
- `"Hello, world!"` is a **text**: it is written between straight double
  quotes `"`.

Then run it in a terminal, from the file's folder (see
[the terminal](../debuter/terminal.md)):

```sh
rythme run hello.ry
```

`rythme hello.ry` works too, without the word `run`.

## Checking without running

`rythme check` looks for problems without running the program:

```sh
rythme check hello.ry
```

```text
✓ hello.ry: no problems found
```

If it finds errors, it shows them all at once, with help for each one.
`rythme run` also does this check before running: a program that has an
error doesn't start at all.

## Getting help

Rythme has its own built-in help, in English:

```sh
rythme help            # the list of topics
rythme help map        # the help for a function
rythme help builtins   # all the built-in functions
rythme learn           # a guided tour of the language
rythme explain E003    # the explanation of an error code
```

To choose the language of the messages, add `--lang en` before the command,
or set the variable `RYTHME_LANG=en`.

## Common mistakes

In the messages below, the file is called `main.ry`. Each message gives the
file, the line and the column (`main.ry:1:7`: line 1, 7th character), then
shows the exact spot with `^`.

### Forgetting the quotes

```rythme norun
print(Hello)
```

```text
error[E003]: unknown name `Hello`
 --> main.ry:1:7
  |
1 | print(Hello)
  |       ^^^^^ not defined
  |
  = help: to create your own kind of value, define a type: `type Name = Hello(field) | Other`
  = more about this error: `rythme explain E003`
```

Without quotes, `Hello` is read as the **name** of a value, and that name
doesn't exist. For a text, add quotes: `print("Hello")`.

### Single quotes instead of double quotes

```rythme norun
print('Hello')
```

```text
error[E018]: unexpected character `'`
 --> main.ry:1:7
  |
1 | print('Hello')
  |       ^
  |
  = help: texts use double quotes: "hello"
  = more about this error: `rythme explain E018`
```

Other languages accept `'...'`. Rythme always wants straight double quotes:
`print("Hello")`.

### Forgetting the closing quote

```rythme norun
print("Hello)
```

```text
error[E002]: this text is never closed
 --> main.ry:1:7
  |
1 | print("Hello)
  |       ^ the text starts here
  |
  = help: add a `"` at the end of the text
  = more about this error: `rythme explain E002`
```

Rythme reads everything after the first `"` as text, all the way to the end
of the file. Close the text: `print("Hello")`.

## Test yourself

**Question 1:** which command checks a program `game.ry` without running
it?

> **Answer**
>
> `rythme check game.ry`. To run it, it's `rythme run game.ry` (or
> `rythme game.ry`).

**Question 2:** what does this program print?

```rythme norun
# print("one")
print("two")
```

> **Answer**
>
> Only `two`. The first line starts with `#`: it's a comment, Rythme
> ignores it.

**Question 3:** in the first example, why is the result 20 and not 30
(1 + 4 + 9 + 16)?

> **Answer**
>
> Because `filter(is_even)` keeps only the even numbers, 2 and 4, before
> squaring them: 4 + 16 = 20.

## Summary

- Rythme is a small, purely functional language: values never change, and
  there are no loops.
- Programs are `.ry` files, run with `rythme run file.ry`.
- `rythme` on its own opens interactive mode; `rythme check file.ry`
  checks a program without running it.
- A text is written between straight double quotes `"`, and `#` starts a
  comment.
- Every error message gives the location, the cause and some help;
  `rythme help` and `rythme explain` tell you more, in English.

Next: [first steps](premiers-pas.md). If you already know Ruby or
Rust, compare with [the introduction to Ruby](../ruby/introduction.md) and
[the introduction to Rust](../rust/introduction.md).
