# Ruby: introduction and installation

This page introduces Ruby, explains how to install it and has you write your first program.

> **Goals:**
>
> - you will be able to say what Ruby is and what it is used for;
> - you will know how to install Ruby and check that it works;
> - you will know how to try a line of code in `irb`;
> - you will know how to write a `.rb` file and run it.

## What is Ruby?

Ruby is a programming language created in Japan by Yukihiro Matsumoto, in 1995.
Its goal: make programmers happy. The code reads almost like English.

A **programming language** is a precise language that the computer can follow.
A **program** is a text written in that language: a list of instructions, like a
cooking recipe.

```ruby
3.times { puts "Hello" }
```

```sortie
Hello
Hello
Hello
```

Read this line out loud: "3 times, print Hello". `puts` (short for *put string*)
prints a text.

Ruby is an **interpreted** language: you write a file, then you run it
directly. A program called an [interpreter](../debuter/glossaire.md#interpreter)
reads your file and carries out the instructions one by one. There is no preparation
step (no "compilation") before.

## Why learn Ruby?

- The syntax (the way you write the code) is short and readable.
- Everything is an [object](../debuter/glossaire.md#object), even numbers: every value
  knows how to do things.
- It is widely used for the web (with Ruby on Rails) and for scripts.
- Its community shares thousands of libraries, called **gems**.

"Everything is an object" means that you can ask a number or a text a question.
The dot `.` calls a [method](../debuter/glossaire.md#method),
that is, an action the value knows how to do.

```ruby
puts 5.even?
puts "ruby".upcase
```

```sortie
false
RUBY
```

- `5.even?` asks the number 5: "are you even?". It answers `false`.
- `"ruby".upcase` asks the text to turn itself into capital letters.

> **Note:** this site also covers other languages: compare with
> [the introduction to Rust](../rust/introduction.md) or [the introduction to Rythme](../rythme/introduction.md).

## Installing Ruby

This site uses **Ruby 3.3**. A newer version works too.

### On Linux and macOS

The easiest way is to use your system's package manager: a
program that installs other programs for you.

```sh
sudo apt install ruby-full     # Debian, Ubuntu
sudo dnf install ruby          # Fedora
brew install ruby              # macOS with Homebrew
```

### With rbenv

`rbenv` lets you install several versions of Ruby and switch from one to another.

```sh
rbenv install 3.3.0
rbenv global 3.3.0
```

> **Tip:** use rbenv if you work on several projects that need
> different versions of Ruby.

### On Windows

Download **RubyInstaller** from the rubyinstaller.org website, then run the installer.
Keep the default options.

## Checking the installation

Open a terminal and type the following command. If the terminal is new to you,
first read [the terminal](../debuter/terminal.md).

```sh
ruby -v
```

You should see a version number, for example `ruby 3.3.0`. If the terminal answers
"command not found", Ruby is not installed, or the terminal
was opened before the installation: close it and open it again.

## Trying Ruby with irb

`irb` (*interactive Ruby*) is an interactive console. You type a line, Ruby
answers right away. It is like a calculator that understands all of Ruby.

```text
$ irb
irb(main):001> 2 + 3
=> 5
irb(main):002> "hello".length
=> 5
irb(main):003> exit
```

- The arrow `=>` shows the result of the line.
- `exit` quits irb.

> **Tip:** irb is perfect for testing a small idea without creating a file.

## Your first program

Open your [code editor](../debuter/editeur.md). Create a file `hello.rb` with
this content:

```ruby
puts "Hello, world!"
```

```sortie
Hello, world!
```

- `puts` prints a text, then goes to the next line.
- The text is between quotes `"..."`. Without quotes, Ruby would think it is a
  name in the code.

A program can have several lines. Ruby runs them in order, from top to bottom.

```ruby
puts "Line 1"
puts "Line 2"
puts 2 + 3
```

```sortie
Line 1
Line 2
5
```

`puts 2 + 3` prints `5`: without quotes, Ruby does the math.

## Running a program

In the terminal, go to the file's folder (with the `cd` command), then
type:

```sh
ruby hello.rb
```

The program runs and prints `Hello, world!`.

> **Remember:** a Ruby program is a `.rb` file. You run it with
> `ruby file_name.rb`.

The guide [the terminal](../debuter/terminal.md) explains how to open a terminal and
change folders.

## Reading an error message

When Ruby does not understand your code, it stops and shows an
[error](../debuter/glossaire.md#error). Don't worry: the message helps you.

```text
main.rb:1:in `<main>': undefined method `put' for main (NoMethodError)
```

Read it from left to right:

- `main.rb:1`: the file and the line number of the problem;
- ``undefined method `put'``: the description (the method `put` does not exist);
- `(NoMethodError)`: the name of the kind of error.

A few words come up often: *undefined* (not defined), *unexpected* (not what was
expected), *expecting* (what Ruby was waiting for), *missing* (something is missing).

## Common mistakes

### A typo in puts

```ruby norun
put "Hello"
```

```text
main.rb:1:in `<main>': undefined method `put' for main (NoMethodError)

put "Hello"
^^^
Did you mean?  puts
               putc
```

Ruby does not know `put`. It even suggests the right spelling (*Did you mean?*).
Fix it to `puts "Hello"`.

### Forgetting the quotes

```ruby norun
puts Hello
```

```text
main.rb:1:in `<main>': uninitialized constant Hello (NameError)

puts Hello
     ^^^^^
```

Without quotes, `Hello` is read as a name in the code, not as a text. Write
`puts "Hello"`.

### Forgetting the closing quote

```ruby norun
puts "Hello
```

```text
main.rb: --> main.rb
expected a closing delimiter for the string literal
> 1  puts "Hello
main.rb:1: unterminated string meets end of file (SyntaxError)
puts "Hello
           ^
```

The text is never closed (*unterminated string*). Add the missing quote:
`puts "Hello"`.

## Test yourself

**Question 1:** which command checks that Ruby is installed?

> **Answer**
>
> `ruby -v`. It prints the version number.

**Question 2:** what does this program print?

```ruby norun
puts "2 + 3"
puts 2 + 3
```

> **Answer**
>
> `2 + 3`, then `5`. Between quotes, it is a text printed as is. Without
> quotes, Ruby does the math.

**Question 3:** you wrote `game.rb`. How do you run it?

> **Answer**
>
> Go to its folder with `cd`, then type `ruby game.rb`.

## Summary

- Ruby is a simple, readable language where everything is an object.
- Install it with your package manager, rbenv or RubyInstaller.
- `ruby -v` checks the installation, `irb` lets you try code right away.
- A program is a `.rb` file, run with `ruby file_name.rb`.
- `puts` prints a text and goes to the next line.
- An error message gives the file, the line and the cause: read it.

Next: [first steps](premiers-pas.md).
