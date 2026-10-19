# What is programming?

Programming means writing instructions that a computer follows to the letter: this page explains what a program, a language and a bug are, and how people learn.

> **Goals:**
>
> - know what a program and a programming language are;
> - recognize source code and its file extensions;
> - read the same small program in several languages;
> - tell an interpreted language from a compiled one;
> - know what a bug is, and how people learn to program.

## A program is a list of instructions

A computer guesses nothing. It does exactly what you tell it, in order,
very fast.

A **program** is a list of instructions written for it. For example:

```text
1. Ask for the person's first name.
2. Display "Hello" followed by that first name.
3. Stop.
```

**Programming** means writing these instructions. A person who programs is
called a developer.

## Like a cooking recipe

A recipe looks a lot like a program:

- a list of **ingredients**: these are the **data** (numbers, pieces of
  text);
- **steps** in a precise order: these are the **instructions**;
- **choices**: "if the batter is too thick, add some milk";
- **repetitions**: "stir until smooth".

The big difference: a cook understands "a pinch of salt". The computer
doesn't. You have to say everything, with no ambiguity.

> **Remember:** a program is a very precise recipe, written for someone
> who obeys without thinking.

## Source code

Instructions are written in a **text file**, like a note. This text is
called the **source code**, or simply the **code**.

The file name ends with an **extension** that tells which language it is
written in: `.py` for Python, `.js` for JavaScript, `.ts` for TypeScript,
`.go` for Go, `.rb` for Ruby, `.rs` for Rust, `.ry` for Rythme.

To write this text, you use a **code editor**: see
[Install a code editor](editeur.md).

## Programming languages

The computer doesn't understand English. It understands a **programming
language**: a small vocabulary, with strict rules. These rules are called
the **syntax**.

There are hundreds of languages. This documentation covers seven: Python,
JavaScript, TypeScript, Go, Ruby, Rust and Rythme. They often say the same
thing with different words. To know which one to start with, see
[Which language should you choose?](choisir.md).

## The same program in three languages

This small program stores a first name in a **variable**, says hello, then
displays the result of a calculation.

In Ruby:

```ruby
first_name = "Ada"
puts "Hello, #{first_name}!"
puts 2 + 3
```

```sortie
Hello, Ada!
5
```

In Rust:

```rust
let first_name = "Ada";
println!("Hello, {first_name}!");
println!("{}", 2 + 3);
```

```sortie
Hello, Ada!
5
```

In Rythme:

```rythme
first_name = "Ada"
print("Hello, {first_name}!")
print(2 + 3)
```

```sortie
Hello, Ada!
5
```

### And in Python or JavaScript

In Python:

```python
first_name = "Ada"
print(f"Hello, {first_name}!")
print(2 + 3)
```

```sortie
Hello, Ada!
5
```

In JavaScript:

```javascript
const firstName = "Ada";
console.log(`Hello, ${firstName}!`);
console.log(2 + 3);
```

```sortie
Hello, Ada!
5
```

They all do the same thing. Only the words change: `puts`, `println!`,
`print` and `console.log` all mean "display".

## Interpreted or compiled

The computer has to turn your text into actions. There are two ways to do
it.

- **Interpreted**: a program, the **interpreter**, reads your code and runs
  it right away, line after line. This is the case for **Python**,
  **JavaScript**, **Ruby** and **Rythme**.
- **Compiled**: a program, the **compiler**, first translates all your code
  into machine language. Only then do you run the result. This is the case
  for **Go** and **Rust**.

TypeScript is a special case: it is first checked and translated into
JavaScript, which is then interpreted.

Think of a book in a foreign language. The interpreter reads it to you out
loud, translating as they go. The translator, on the other hand, first
writes the whole book in English: it takes longer at the start, but
reading it afterwards is very fast.

> **Note:** a compiler checks a lot of things before running anything. It
> finds mistakes earlier, but it is also more demanding.

## What is a bug?

A **bug** is a mistake in a program. The program doesn't work, or it
doesn't do what you wanted.

There are two main kinds of bugs:

- the computer **rejects** your code: a syntax mistake, like a forgotten
  parenthesis. It shows you an error message;
- the code **works, but gives a wrong result**: you wrote `a - b` instead
  of `a + b`. The computer can't guess that.

Finding and fixing a bug is called **debugging**.

> **Remember:** everybody makes bugs, all the time, even experts. It isn't
> a failure: it's normal work.

## How you learn to program

You don't learn to program just by reading. You learn by trying.

1. **Read** a short example.
2. **Type it** yourself, without copy-pasting.
3. **Run it** and watch what happens.
4. **Change** something: a number, a word. Run it again.
5. **Make mistakes**, and read the error message.

An error message isn't a reproach. It's a clue. It often tells you
**where** the problem is (the file and the line) and **why**.

> **Tip:** always read the first line of an error message in full, slowly.
> The answer is often right there.

## What's next

Before writing code, you need to learn how to break a problem down into
steps: that's the topic of [thinking like a programmer](algorithmes.md).
To run a program, you will then need to open a terminal: see
[the terminal in 10 minutes](terminal.md). New words are explained in the
[glossary](glossaire.md).

## Common mistakes

### Writing instructions in plain English

The computer only knows the words of its language. `show` doesn't exist in
Ruby:

```ruby norun
show "Hello"
```

```text
en.rb:1:in `<main>': undefined method `show' for main (NoMethodError)

show "Hello"
^^^^
```

In Ruby, "display" is spelled `puts`: `puts "Hello"`.

### Forgetting the quotes around a text

Without quotes, the language thinks `Hello` is a name it should know:

```ruby norun
puts Hello
```

```text
q.rb:1:in `<main>': uninitialized constant Hello (NameError)

puts Hello
     ^^^^^
```

A text is always written between quotes: `puts "Hello"`.

### Changing a capital letter

For the computer, `first_name` and `First_name` are two different names:

```ruby norun
first_name = "Ada"
puts "Hello, " + First_name
```

```text
pm.rb:2:in `<main>': uninitialized constant First_name (NameError)

puts "Hello, " + First_name
                 ^^^^^^^^^^
```

Write the name exactly as you created it: `first_name`.

## Test yourself

**1.** What do a program and a cooking recipe have in common?

> **Answer**
>
> Both are lists of instructions in a precise order, with data (the
> ingredients), choices and repetitions. But the program must be far more
> precise: the computer guesses nothing.

**2.** What is the difference between an interpreted language and a
compiled language? Give an example of each.

> **Answer**
>
> An interpreter reads and runs the code right away (Python, Ruby,
> Rythme). A compiler first translates all the code, then you run the
> result (Go, Rust).

**3.** Your program runs but displays a wrong result. Is it a bug?

> **Answer**
>
> Yes. A bug isn't only a program that crashes: it's also a program that
> doesn't do what you wanted.

## Summary

- A program is a list of precise instructions, like a recipe.
- Source code is a text file written in a programming language.
- Python, JavaScript, Ruby and Rythme are interpreted; Go and Rust are compiled before they run.
- A bug is a mistake in the program, and everybody makes them.
- You learn by trying, making mistakes and reading error messages.

Next: [How a computer works](ordinateur.md)
