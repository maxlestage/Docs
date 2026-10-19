# Python: introduction and installation

This page introduces Python, explains how to install it, and has you write and run your first program.

> **Goals:**
>
> - you will be able to say what Python is and what it is used for;
> - you will be able to install Python on Windows, macOS or Linux;
> - you will be able to try a line of code in the `>>>` console;
> - you will be able to write a `.py` file and run it from the terminal.

## What is Python?

Python is a programming language created by Guido van Rossum, in the Netherlands, in 1991.
Its name comes from a British comedy show, *Monty Python*, not from the snake.

A **programming language** is a precise language that the computer can follow.
A [program](../debuter/glossaire.md#program) is a text written in that language:
a list of instructions, like a cooking recipe.

```python
print("Hello")
print("Hello")
print("Hello")
```

```sortie
Hello
Hello
Hello
```

`print` displays a text on the screen. Python runs the lines in order, from top to
bottom.

Python is an **interpreted** language: you write a file, then you run it directly. A
program called an [interpreter](../debuter/glossaire.md#interpreter) reads your file
and runs the instructions one by one. There is no preparation step (no "compilation")
beforehand.

## What is Python used for?

Python is one of the most widely used languages in the world. You find it almost
everywhere:

- **scripts**: renaming 500 files, sorting photos, filling in a spreadsheet;
- **data analysis**: statistics, charts, tables of numbers;
- **artificial intelligence**: most machine learning tools;
- **the web**: websites and online services (with Django or Flask);
- **teaching**: it is often the first language people learn in high school and at
  university.

Why is it so popular? The code reads almost like English. There are few symbols to
remember. And Python comes with a large standard
[library](../debuter/glossaire.md#library): thousands of ready-to-use tools,
without installing anything else.

```python
numbers = [4, 8, 15, 16, 23, 42]
print(sum(numbers))
print(max(numbers))
```

```sortie
108
42
```

Read this code out loud: "the sum of the numbers", "the max of the numbers".

> **Note:** this site also covers other languages: compare with
> [the introduction to Ruby](../ruby/introduction.md) or
> [the introduction to Rust](../rust/introduction.md).

## Installing Python

This site uses **Python 3.13**. Any version from 3.10 onwards is enough for almost all
the examples.

> **Warning:** there is an old Python 2, abandoned since 2020. Always install a
> Python **3**.

### On Windows

1. Go to the official website, **python.org**, section *Downloads*.
2. Download the installer for Windows and run it.
3. **Check the box "Add python.exe to PATH"** at the bottom of the first window.
4. Click *Install Now*.

The "Add to PATH" box is important. The *PATH* is the list of folders where the
terminal looks for programs. Without it, the terminal will not find Python.

Windows also installs the `py` **launcher**. It is a small program that finds the right
Python for you. On Windows, you can therefore type `py` instead of `python`.

```sh
py --version
py hello.py
```

### On macOS

macOS does not always come with a recent Python. Two options:

- download the macOS installer from **python.org**, then run it;
- or, if you use Homebrew (a package manager for Mac):

```sh
brew install python
```

On macOS, the command is called `python3`, with a 3 at the end.

### On Linux

Python is almost always already installed. If not, use your system's package manager: a
program that installs other programs for you.

```sh
sudo apt install python3        # Debian, Ubuntu
sudo dnf install python3        # Fedora
```

Here too, the command is called `python3`.

> **Remember:** depending on the system, the command is `python3` (macOS, Linux),
> `python` or `py` (Windows). On this site, we write `python3`.

## Checking the installation

Open a terminal and type the following command. If the terminal is new to you, first
read [the terminal](../debuter/terminal.md).

```sh
python3 --version
```

You should see a version number:

```text
Python 3.13.0
```

If the terminal answers "command not found" or "is not recognized", there are two
possible causes:

- Python is not installed, or the "Add to PATH" box was not checked;
- the terminal was opened before the installation: close it and open it again.

## Trying Python in the console

Just type `python3`, without a file name. You enter the **interactive console**, also
called the [REPL](../debuter/glossaire.md#repl). You type a line, and Python answers
right away. It is like a calculator that understands all of Python.

```text
$ python3
Python 3.13.0 (main, Oct  7 2024, 05:02:14) [GCC 13.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> 2 + 3
5
>>> "hi".upper()
'HI'
>>> len("hello")
5
>>> exit()
```

- The `>>>` sign is the **prompt**: Python is waiting for your line.
- The line below it is Python's answer. No need for `print` in the console.
- `exit()` (or Ctrl+D on macOS and Linux, Ctrl+Z then Enter on Windows) leaves the
  console.

> **Tip:** the console is perfect for testing a small idea without creating a file.
> Keep one open while you read this site.

## Your first program

The console forgets everything when you leave it. To keep a program, write it in a
**file**. Open your [code editor](../debuter/editeur.md). Create a file `hello.py` with
this content:

```python
print("Hello, world!")
```

```sortie
Hello, world!
```

- `print(...)` displays what is between the parentheses, then goes to a new line.
- The text is between quotes `"..."`. Without quotes, Python would think it is a name
  in the code.
- The file ends with `.py`: that is how you recognize a Python program.

A program can have several lines. Python runs them in order.

```python
print("Line 1")
print("Line 2")
print(2 + 3)
print("2 + 3")
```

```sortie
Line 1
Line 2
5
2 + 3
```

`print(2 + 3)` displays `5`: without quotes, Python calculates. With quotes, it
displays the text as it is.

## Running a program

In the terminal, go to the file's folder (with the `cd` command), then type:

```sh
python3 hello.py
```

The program runs and displays `Hello, world!`. Then the terminal gives you back
control.

> **Remember:** a Python program is a `.py` file. You run it with
> `python3 file_name.py` (or `py file_name.py` on Windows).

The guide [the terminal](../debuter/terminal.md) explains how to open a terminal and
change folders.

### Indentation matters

In Python, the spaces at the start of a line (the **indentation**) have a meaning. They
show which lines belong together, like the indents in an essay outline. You will see
this in [conditions and loops](conditions-boucles.md).

```python
for i in range(2):
    print("inside the loop")
print("after the loop")
```

```sortie
inside the loop
inside the loop
after the loop
```

For now, remember one rule: **do not put spaces at the start of a line** without a
reason.

## Reading an error message

When Python does not understand your code, it stops and displays an
[error](../debuter/glossaire.md#error). That is fine: the message helps you.

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 1, in <module>
    Print("Hello")
    ^^^^^
NameError: name 'Print' is not defined. Did you mean: 'print'?
```

Read it **from the bottom up**:

- the last line gives the type of error (`NameError`) and the cause ("the name `Print`
  is not defined");
- above it, the faulty line, with `^^^^^` under the part at fault;
- further up, the file and the line number (`line 1`).

The word *Traceback* means "tracing back the path": Python shows where it was when the
error happened. Some words come up often: *not defined*, *invalid*, *expected*,
*missing*, *Did you mean*.

## Common mistakes

### A capital letter in print

```python norun
Print("Hello")
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 1, in <module>
    Print("Hello")
    ^^^^^
NameError: name 'Print' is not defined. Did you mean: 'print'?
```

Python makes a difference between uppercase and lowercase letters. `Print` and `print`
are two different names. Python even suggests the right spelling. Fix it to
`print("Hello")`.

### Forgetting the parentheses

```python norun
print "Hello"
```

```text
  File "/home/you/main.py", line 1
    print "Hello"
    ^^^^^^^^^^^^^
SyntaxError: Missing parentheses in call to 'print'. Did you mean print(...)?
```

That was the way to write it in Python 2. In Python 3, `print` is a
[function](../debuter/glossaire.md#function): it needs parentheses.
Write `print("Hello")`.

### Forgetting the closing quote

```python norun
print("Hello)
```

```text
  File "/home/you/main.py", line 1
    print("Hello)
          ^
SyntaxError: unterminated string literal (detected at line 1)
```

The text is never closed (*unterminated string*). Add the missing quote:
`print("Hello")`.

### An extra space at the start

```python norun
print("Line 1")
  print("Line 2")
```

```text
  File "/home/you/main.py", line 2
    print("Line 2")
IndentationError: unexpected indent
```

Line 2 starts with spaces for no reason (*unexpected indent*). Remove them.

## Test yourself

**Question 1:** which command checks that Python is installed?

> **Answer**
>
> `python3 --version` (or `py --version` on Windows). It displays the version number.

**Question 2:** what does this program display?

```python norun
print("4 * 2")
print(4 * 2)
```

> **Answer**
>
> `4 * 2`, then `8`. Between quotes, it is a text displayed as it is. Without quotes,
> Python calculates.

**Question 3:** you have written `game.py`. How do you run it?

> **Answer**
>
> Go to its folder with `cd`, then type `python3 game.py`.

## Summary

- Python is an interpreted, readable language, used for scripts, data, AI and the web.
- Install it from python.org (check "Add to PATH" on Windows) or with your package
  manager.
- `python3 --version` checks the installation; `python3` alone opens the `>>>` console.
- A program is a `.py` file, run with `python3 file_name.py`.
- `print(...)` displays a value and goes to a new line.
- An error message is read from the bottom up: type, cause, then the faulty line.

Next: [first steps](premiers-pas.md).
