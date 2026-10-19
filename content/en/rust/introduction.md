# Rust: introduction and installation

Rust is a fast and safe language: this page explains what it brings, how to install it and how to run your first program.

> **Goals:**
>
> - know what a compiled language is and what Rust brings;
> - install Rust with `rustup`;
> - create and run a project with `cargo`;
> - read a small Rust program and change it;
> - understand a first error message from the compiler.

## What is Rust?

Rust is a **compiled** programming language. A
[compiler](../debuter/glossaire.md#compiler) is a program that translates your
code into an executable file, which the computer runs directly. The translation happens
only once, before running: the resulting program is very fast.

Think of a book translated in advance: the foreign reader reads it in one go, without
waiting for an interpreter to translate sentence by sentence.

Rust has three big strengths:

- **Speed**: a Rust program is as fast as a C or C++ program.
- **Safety**: the compiler rejects many mistakes even before the program runs
  (memory used the wrong way, a forgotten missing value…).
- **Tools**: `cargo` creates, compiles, tests and formats your projects.

Here is a tiny Rust program:

```rust
fn main() {
    println!("Hello, Rust!");
}
```

```sortie
Hello, Rust!
```

> **Note:** Rust is demanding at first. The compiler often complains, but its
> error messages are clear and even suggest fixes.

## Installing Rust

The official tool is called `rustup`. It installs two programs:

- `rustc`, the compiler;
- `cargo`, the tool that manages your projects (it calls `rustc` for you).

### On Linux and macOS

Open a terminal and type the command below. If the terminal is new to you,
first read [Using the terminal](../debuter/terminal.md).

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the instructions, then open a new terminal.

### On Windows

Download `rustup-init.exe` from the [rustup.rs](https://rustup.rs) website and run it.
The installer may ask you to also install the Visual Studio build
tools: accept.

### Checking the installation

```sh
rustc --version
cargo --version
```

Each command prints a version number, for example `rustc 1.97.0`.

> **Tip:** to move to the latest version of Rust later, type
> `rustup update`.

## Creating a project with cargo

`cargo new` creates a ready-to-use project folder:

```sh
cargo new hello
cd hello
```

The folder contains two things:

```text
hello/
├── Cargo.toml      the project description (name, version…)
└── src/
    └── main.rs     the program's code
```

To compile and run the program, a single command:

```sh
cargo run
```

Cargo compiles the code, then runs it. You see `Hello, world!` printed.

> **Remember:** `cargo new` creates a project, `cargo run` compiles it and runs it.

## Your first program

Open `src/main.rs` and replace its content:

```rust
fn main() {
    let first_name = "Alice";
    let age = 30;
    println!("My name is {first_name} and I am {age} years old.");
}
```

```sortie
My name is Alice and I am 30 years old.
```

Let's read it line by line:

- `fn main()` declares the [function](../debuter/glossaire.md#function) `main`. It is the
  starting point of every Rust program: it always starts there.
- The curly braces `{ … }` surround the **body** of the function: the instructions to
  run, in order.
- `let` creates a [variable](../debuter/glossaire.md#variable): a name put on a
  [value](../debuter/glossaire.md#value).
- `println!` prints a line. The `!` indicates a **macro**, a kind of special
  function. `{first_name}` is replaced by the value of `first_name`.
- Each instruction ends with a semicolon `;`.

## Compiling without cargo

For a single file, you can also use `rustc` directly. It creates an
executable, which you then run:

```sh
rustc main.rs
./main
```

In a real project, always prefer `cargo`. It is covered in detail in
[Projects, crates and tests](projet.md).

## Common mistakes

### Forgetting the ! of println!

```rust norun
fn main() {
    println("Hello!");
}
```

```text
error[E0423]: expected function, found macro `println`
 --> src/main.rs:2:5
  |
2 |     println("Hello!");
  |     ^^^^^^^ not a function
help: use `!` to invoke the macro
```

`println!` is a macro: the `!` is required. Write `println!("Hello!");`.

### Forgetting a semicolon

```rust norun
fn main() {
    let first_name = "Alice"
    println!("Hello, {first_name}!");
}
```

```text
error: expected `;`, found `println`
 --> src/main.rs:2:29
  |
2 |     let first_name = "Alice"
  |                             ^ help: add `;` here
3 |     println!("Hello, {first_name}!");
  |     ------- unexpected token
```

The compiler shows where to add the `;`: at the end of line 2.

### Putting the text between single quotes

```rust norun
fn main() {
    println!('Hello!');
}
```

```text
error[E0762]: unterminated character literal
 --> src/main.rs:2:21
  |
2 |     println!('Hello!');
  |                     ^^^
help: if you meant to write a string literal, use double quotes
```

In Rust, a text is written between double quotes: `"Hello!"`. Single quotes
`'a'` are reserved for a single character.

### Misspelling main

```rust norun
fn Main() {
    println!("Hello!");
}
```

```text
error[E0601]: `main` function not found in crate `main`
 --> src/main.rs:3:2
  |
3 | }
  |  ^ consider adding a `main` function to `src/main.rs`
```

Rust tells capital letters apart from lowercase ones: the starting function is called `main`,
in lowercase.

## Test yourself

**1.** Which command creates a project named `test`? And which one runs it?

> **Answer**
>
> `cargo new test` creates the project. Then, in the `test` folder, `cargo run`
> compiles it and runs it.

**2.** What does the `!` in `println!` mean?

> **Answer**
>
> It indicates a **macro**, a kind of special function. Without the `!`, the program
> does not compile.

**3.** What does this program print?

```rust norun
fn main() {
    let city = "Nantes";
    println!("I live in {city}.");
}
```

> **Answer**
>
> ```sortie
> I live in Nantes.
> ```

## Summary

- Rust is a compiled, fast and safe language.
- `rustup` installs Rust; `rustup update` updates it.
- `cargo new name` creates a project, `cargo run` compiles it and runs it.
- A program always starts with `fn main()`.
- `println!` prints a line of text; each instruction ends with `;`.

Compare with [the introduction to Ruby](../ruby/introduction.md) or
[the introduction to Rythme](../rythme/introduction.md).

Next: [First steps](premiers-pas.md).
