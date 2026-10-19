# Install a code editor

A code editor is the software you write your programs in: this page helps you install VS Code, create your first file and add the right extensions.

> **Goals:**
>
> - understand the difference between a code editor and a word
>   processor;
> - install VS Code and open a folder;
> - create, name and save a code file;
> - run a program from the built-in terminal;
> - add the useful extensions for Ruby, Rust and Rythme.

## An editor, not a word processor

Word or LibreOffice are for writing letters. They add fonts, colors,
margins. To the computer, all of that is noise: a program must be **plain
text**.

A **code editor** writes plain text, and it helps you:

- it **colors** the code to make it readable;
- it **points out mistakes** while you type;
- it **completes** the words you start;
- it contains a **terminal** to run your programs.

> **Warning:** never write code in Word. Quotes and spaces get
> transformed there, and the program stops working.

## Installing VS Code

This page uses **Visual Studio Code** (VS Code). It's free, it exists on
every system, and it suits all three languages.

### On Windows

Go to the site `code.visualstudio.com`, click **Download for Windows**,
then open the downloaded file. Accept the suggested choices. Tick **Add to
PATH** if the box appears.

### On macOS

Download the macOS version from the same site. Open the `.zip` file, then
drag **Visual Studio Code** into the **Applications** folder.

### On Linux

Download the `.deb` package (Ubuntu, Debian) or `.rpm` package (Fedora)
from the site, and open it. You can also use your system's software
center: search for **Visual Studio Code**.

## Opening a folder

A program lives in a **folder**. Create a folder `code` (see
[the terminal in 10 minutes](terminal.md#creating-a-folder-mkdir)), then, in
VS Code: menu **File › Open Folder…**, and choose it.

The folder shows up on the left, in the **explorer**. All your files will
appear there.

> **Tip:** always open a folder, not a single file. The editor understands
> your project better, and the terminal starts in the right place.

## Creating a file

In the explorer, click the **New File** icon and type its name, for
example `hello.rb`. Write in it:

```ruby
puts "Hello!"
```

```sortie
Hello!
```

### File names

The **extension**, the end of the name, tells the language:

| Language | Extension | Example       |
|----------|-----------|---------------|
| Ruby     | `.rb`     | `hello.rb`    |
| Rust     | `.rs`     | `main.rs`     |
| Rythme   | `.ry`     | `hello.ry`    |

Choose simple names: lowercase, no spaces or accents. To separate words,
use `_`: `average_grade.rb`.

## Saving

An unsaved file doesn't exist for the terminal. Save with **Ctrl + S**
(Windows, Linux) or **Cmd + S** (macOS).

A white dot on the file's tab means there are unsaved changes.

### Auto save

So you don't have to think about it, turn on the menu **File › Auto
Save**. VS Code then saves on its own, a few moments after each change.

> **Remember:** if your program doesn't see your changes, first check that
> the file is saved.

## The built-in terminal

VS Code contains a terminal. Open it with the menu **Terminal › New
Terminal**, or with **Ctrl + `** (the backtick key).

It opens at the bottom of the window, already in your folder. Run your
program:

```sh
ruby hello.rb
```

```text
Hello!
```

## Useful extensions

An **extension** adds features to VS Code. Click the **Extensions** icon
on the left (four small squares), search for a name and click
**Install**.

- **Ruby LSP** (by Shopify): colors, completion and errors for Ruby.
- **rust-analyzer**: the must-have help for Rust. It shows types and
  errors even before compiling.
- **Rythme**: colors, help on hover, explained errors and a **▶ Run**
  button above the code. Install Rythme itself first (see the
  [introduction to Rythme](../rythme/introduction.md)).

> **Note:** if the Rythme extension doesn't show up in the search yet, the
> Rythme repository explains how to install it from its `editors/vscode`
> folder.

## Other editors

VS Code isn't the only choice. Here are three others:

- **Zed**: very fast and modern, with an extension for Rythme.
- **Sublime Text**: light and fast, paid after a trial period.
- **Neovim**: used entirely with the keyboard, in the terminal; powerful,
  but long to learn.

To start, stick with VS Code: you can switch later if you want.

## Common mistakes

### Transformed quotes

Code copied from Word, an email or some websites contains "curly" quotes
(`“ ”`) instead of straight quotes (`"`):

```ruby norun
puts “Hello”
```

```text
quotes.rb:1:in `<main>': undefined local variable or method `“Hello”' for main (NameError)
```

Ruby doesn't recognize `“` as a quote. Retype the quotes in the code
editor: `puts "Hello"`.

### Forgetting to save

You change `"Hello!"` to `"Hi!"`, you run it again… and the terminal still
displays:

```text
Hello!
```

There's no error: the terminal reads the **saved** file, not what's on the
screen. Look for the white dot on the tab, save with Ctrl + S (Cmd + S),
then run it again. Or turn on *Auto Save*.

### A hidden extension

On Windows or macOS, a file created somewhere other than in the editor
may actually be called `hello.rb.txt`. The terminal can't find it:

```sh
ruby hello.rb
```

```text
ruby: No such file or directory -- hello.rb (LoadError)
```

Create your files from the VS Code explorer, which shows the full name,
and check with `ls` in the terminal.

## Test yourself

**1.** Why not write your code in Word?

> **Answer**
>
> Word doesn't write plain text: it adds formatting and transforms quotes
> and spaces. The program stops working.

**2.** Which file extension do you use for a Rythme program? And for
Ruby?

> **Answer**
>
> `.ry` for Rythme, `.rb` for Ruby (and `.rs` for Rust).

**3.** Why open a folder rather than a single file in VS Code?

> **Answer**
>
> The editor understands the whole project better, the explorer shows all
> its files, and the built-in terminal starts right in the right place.

## Summary

- A code editor writes plain text and helps you read and fix it.
- Install VS Code, then open a folder with *File › Open Folder*.
- The file extension gives the language: `.rb`, `.rs` or `.ry`.
- Save with Ctrl + S (Cmd + S on macOS), or turn on *Auto Save*.
- Add the Ruby LSP, rust-analyzer and Rythme extensions.

Next: [finding and fixing your mistakes](deboguer.md).
