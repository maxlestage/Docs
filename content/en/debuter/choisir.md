# Which language should you choose?

Python, JavaScript, TypeScript, Go, Ruby, Rust and Rythme each have their strengths: this page compares them simply and suggests a path depending on what you want to do.

> **Goals:**
>
> - know the strengths and limits of the seven languages on this site;
> - compare the languages in a table and on the same small program;
> - choose a path depending on what you want to do;
> - know which page to start with.

## There is no wrong choice

Your first language is not a lifelong commitment. The basic ideas
are the same everywhere: variables, conditions, loops, functions, lists.

Once you know one language, the second one is much faster to learn.
What matters is to **get started**.

> **Remember:** learning a language is mostly learning to
> program. That knowledge goes with you everywhere.

## Python: the most used for beginners

Python is a very readable language: few symbols, and indentation
(the shifting of lines to the right) is part of the syntax.

- **Easy to read**: a Python program looks like pseudo-code.
- **Everywhere**: scripts, automation, data science, artificial
  intelligence, websites.
- **A huge community**: thousands of [packages](glossaire.md#package)
  to install with [pip](glossaire.md#pip), and answers online to
  almost every question.

Its limit: it is slower than Go or Rust, and it doesn't run
directly in a browser.

## JavaScript: the language of the web

JavaScript is the only language that **all browsers** understand. It
makes web pages interactive: a button that reacts, a menu that
opens.

- **Nothing to install to get started**: your browser can already run it.
- **The web from end to end**: in the page (the [DOM](glossaire.md#dom)) and
  on the server, with Node.js.
- **A huge community**: the [npm](glossaire.md#npm) registry is the
  largest catalog of packages in the world.

Its limit: it lets a lot of errors through. `"2" + 1` gives `"21"`,
without warning. Some old rules surprise beginners.

## TypeScript: JavaScript with types

TypeScript is JavaScript, plus **[types](glossaire.md#type)** written
in the code. A tool, `tsc`, checks these types before running the
program, then turns it into JavaScript.

- **Fewer surprises**: many errors are found before
  the program runs.
- **A smarter editor**: it completes the code and points out mistakes
  while you type.
- **In high demand**: most big web projects use it.

Its limit: you first need to know the basics of JavaScript. Learn it
second.

## Go: simple, fast, for servers

Go was created at Google to write servers and tools. It is
deliberately **small**: few keywords, only one way to format
the code.

- **Fast**: it is compiled, and a Go program starts in an instant.
- **Easy parallel work**:
  [goroutines](glossaire.md#goroutine) and [channels](glossaire.md#channel)
  do several things at once.
- **A single file to ship**: the compiler produces a complete
  program, with nothing to install alongside it.

Its limit: it is a bit **verbose**. You often write the same lines,
for example to check each error.

## Ruby: to build things fast

Ruby is a language that is pleasant to read, almost like English.

- **Easy to write**: few symbols, lots of things ready to use.
- **Websites**: the Ruby on Rails framework is used to build
  many well-known sites.
- **A welcoming community**: thousands of libraries (the
  [gems](glossaire.md#gem)) and lots of help online.

Its limit: it is slower than Go or Rust. For most
programs, you won't notice.

## Rust: to go very fast and understand the machine

Rust is a compiled language, very fast and very safe.

- **Fast programs**: as fast as those written in C or C++.
- **Safety**: the compiler rejects many errors before it even
  runs the program.
- **Understanding the computer**: Rust makes you think about memory and
  the way data is stored.

Its limit: it is **harder** to learn. The compiler is
demanding, and some ideas, like
[ownership](../rust/possession.md), take time.

## Rythme: for learning

Rythme is a small language, designed for beginners.

- **Few rules**: only 15 keywords, and only one way to do
  each thing.
- **Errors that explain**: each message says where the problem is and
  how to fix it, in plain words and in your language (English, French and more).
- **A functional style**: values never change, which avoids
  many surprises.

Its limit: it is young. Few people use it, and it is not used
for big professional projects.

> **Note:** Rythme itself is written in Rust. The languages of this
> documentation go hand in hand.

## The comparison table

| Language   | Ease                | Speed         | Main uses                            | Where it runs                     | Community               |
|------------|---------------------|---------------|--------------------------------------|-----------------------------------|-------------------------|
| Python     | Very easy           | Medium        | Scripts, data, AI, web               | Computer, server                  | Huge                    |
| JavaScript | Easy                | Good          | Web pages, servers, applications     | Browser, server (Node.js)         | Huge                    |
| TypeScript | Medium              | Good          | Big web projects                     | Like JavaScript, after `tsc`      | Very large              |
| Go         | Easy                | Fast          | Servers, tools, cloud                | Server, terminal                  | Large                   |
| Ruby       | Easy                | Medium        | Websites (Rails), scripts            | Computer, server                  | Large and welcoming     |
| Rust       | Hard at first       | Very fast     | Tools, systems, games, fast web      | Everywhere, even the browser      | Large, growing          |
| Rythme     | Very easy           | Slow          | Learning, small scripts              | Terminal, browser playground      | Small and new           |

Three words to read this table:

- **Interpreted**: Python, JavaScript, Ruby and Rythme are read and run
  directly by an [interpreter](glossaire.md#interpreter).
- **Compiled**: Go and Rust are first translated into machine language by a
  [compiler](glossaire.md#compiler).
- **TypeScript** is a special case: it is checked, then translated into JavaScript.

## An example to compare

Here is the same small program in the seven languages: it adds up
a list of numbers.

### In Python

```python
numbers = [4, 8, 15, 16, 23, 42]
print(sum(numbers))
```

```sortie
108
```

### In JavaScript

```javascript
const numbers = [4, 8, 15, 16, 23, 42];
const total = numbers.reduce((sum, n) => sum + n, 0);
console.log(total);
```

```sortie
108
```

JavaScript has no `sum`: `reduce` adds the numbers one by one,
starting from `0`.

### In TypeScript

```typescript
const numbers: number[] = [4, 8, 15, 16, 23, 42];
const total: number = numbers.reduce((sum, n) => sum + n, 0);
console.log(total);
```

```sortie
108
```

It is the same code, with types added: `number[]` means "a
list of numbers".

### In Go

```go
package main

import "fmt"

func main() {
	numbers := []int{4, 8, 15, 16, 23, 42}
	total := 0
	for _, n := range numbers {
		total += n
	}
	fmt.Println(total)
}
```

```sortie
108
```

Go asks for a complete program (`package main`, `func main`) and a loop
written by hand: it is longer, but everything is visible.

### In Ruby

```ruby
numbers = [4, 8, 15, 16, 23, 42]
puts numbers.sum
```

```sortie
108
```

### In Rust

```rust
let numbers = [4, 8, 15, 16, 23, 42];
let total: i32 = numbers.iter().sum();
println!("{total}");
```

```sortie
108
```

Rust wants to know the type of the result: `i32`, an integer.

### In Rythme

```rythme
numbers = [4, 8, 15, 16, 23, 42]
print(sum(numbers))
```

```sortie
108
```

All seven do the same thing. Python, Ruby and Rythme are the shortest;
Go is the most explicit.

## Which path to follow

### "I want to learn to program"

Start with **Python** or **Rythme**. Python opens the door to real
projects right away; Rythme teaches you the ideas without losing you
in the details, with errors that explain.

### "I want to build websites"

Start with **JavaScript**, then move on to **TypeScript**. JavaScript is
required in the browser; TypeScript then helps you write
bigger projects without getting lost.

### "I want to write servers and tools"

Choose **Go**. It is simple, fast, and widely used for
servers, the cloud and command-line tools.

### "I want very fast programs, or to understand the machine"

Choose **Rust**. Take your time: the beginning is demanding, but
the compiler teaches you an enormous amount.

### "I want to write scripts, or use Rails"

Choose **Ruby**. It is pleasant for automating small tasks, and
it is the direct route to Ruby on Rails.

> **Tip:** still hesitating? Take Python, or Rythme. The
> [Rythme playground](https://maxlestage.github.io/Rythme/) runs
> in the browser, with nothing to install.

## Getting started

Each path starts with an introduction page, which explains how to
install the language and run a first program:

- [Getting started with Python](../python/introduction.md)
- [Getting started with JavaScript](../javascript/introduction.md)
- [Getting started with TypeScript](../typescript/introduction.md)
- [Getting started with Go](../go/introduction.md)
- [Getting started with Ruby](../ruby/introduction.md)
- [Getting started with Rust](../rust/introduction.md)
- [Getting started with Rythme](../rythme/introduction.md)

Before you dive in, make sure you know how to use
[the terminal](terminal.md) and [a code editor](editeur.md).

## Common mistakes

When you switch from one language to another, you quickly mix up the words. Each
language reports it in its own way.

### Writing Ruby in Python

```python norun
puts "Hello"
```

```text
  File "main.py", line 1
    puts "Hello"
         ^^^^^^^
SyntaxError: invalid syntax
```

In Python, you write `print("Hello")`, with parentheses.

### Writing Python in JavaScript

```javascript norun
print("Hello");
```

```text
file:///home/ada/main.mjs:1
print("Hello");
^

ReferenceError: print is not defined
    at file:///home/ada/main.mjs:1:1
```

In JavaScript, you write `console.log("Hello");`.

### Keeping an unused variable in Go

Go refuses to compile a program that creates a variable without using it:

```go norun
package main

import "fmt"

func main() {
	numbers := []int{4, 8, 15}
	fmt.Println("Hello")
}
```

```text
# command-line-arguments
./main.go:6:2: declared and not used: numbers
```

Use the variable, or delete the line. Ruby, Python and JavaScript don't
say anything in this case.

### Writing a for loop in Rythme

```rythme norun
numbers = [1, 2, 3]
for n in numbers
  print(n)
end
```

```text
error[E001]: `for` is not used in Rythme
 --> loop.ry:2:1
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

In Rythme, you write `numbers |> each(print)`. See
[conditions and repetition](../rythme/conditions-boucles.md).

> **Tip:** learn one language at a time. When you move on to a
> second one, keep its cheat sheet open next to you, for example
> [the Python one](../python/aide-memoire.md).

## Test yourself

**1.** You want to create interactive web pages. Which language should you
start with, and which one should you learn next?

> **Answer**
>
> JavaScript first, because it is the language of browsers. Then
> TypeScript, which adds types to avoid many errors.

**2.** Among the seven languages, which ones are compiled to machine language?

> **Answer**
>
> Go and Rust. Python, JavaScript, Ruby and Rythme are interpreted;
> TypeScript is checked, then translated into JavaScript.

**3.** Have you made a "wrong" choice if you switch languages
later?

> **Answer**
>
> No. The basic ideas (variables, conditions, loops, functions,
> lists) are found everywhere, and the second language is much
> faster to learn.

## Summary

- Your first language is not a final choice: the basic ideas are found everywhere.
- To learn to program: Python or Rythme.
- For websites: JavaScript, then TypeScript.
- For servers and tools: Go; for very fast programs: Rust.
- For scripts and Ruby on Rails: Ruby.
- Each path starts with its introduction page.

Next: [the glossary](glossaire.md), to keep at hand.
