# The terminal in 10 minutes

The terminal is a window where you talk to the computer by typing text: this page teaches you how to open it, move around in it and run your programs from it.

> **Goals:**
>
> - open the terminal on Windows, macOS or Linux;
> - know where you are and move between folders;
> - create a folder for your experiments;
> - save time with the up arrow, Tab and Ctrl + C;
> - run a Ruby, Rust or Rythme program;
> - read an error message in the terminal.

## What is the terminal

Usually, you use the computer with the mouse: you click on folders and
icons.

The **terminal** does the same thing, but with the keyboard. You type a
**command**, you press Enter, and the computer answers with text.

That's where you'll run your programs. Don't panic: about ten commands are
enough to get started.

## Opening the terminal

### On Windows

Open the Start menu, type **Terminal** and press Enter. If nothing shows
up, type **PowerShell**. Both are fine.

### On macOS

Press **Cmd + Space**, type **Terminal** and press Enter. The app is also
in *Applications › Utilities*.

### On Linux

The shortcut **Ctrl + Alt + T** works on most systems. Otherwise, look for
**Terminal** in the list of applications.

## The command prompt

Once open, the terminal shows a line waiting for your command. That's the
**prompt**. It looks like this:

```text
alice@laptop:~$
```

or, on Windows:

```text
PS C:\Users\Alice>
```

The prompt often shows the **folder you're in**. On this page, commands
are written without the prompt: type only what's in the block.

## The basic commands

### Where am I: pwd

`pwd` displays the current folder (*print working directory*).

```sh
pwd
```

```text
/home/alice
```

### What's in this folder: ls or dir

`ls` lists the files and folders. On Windows, `dir` does the same thing
(`ls` works in PowerShell too).

```sh
ls
```

```text
Desktop  Documents  Downloads  Pictures
```

### Going into a folder: cd

`cd` (*change directory*) moves you into a folder.

```sh
cd Documents
```

To go up one level, to the parent folder, type `cd ..`:

```sh
cd ..
```

### Creating a folder: mkdir

`mkdir` (*make directory*) creates a folder. Create one for your
experiments:

```sh
mkdir code
cd code
```

### Clearing the screen: clear

`clear` clears the screen. On Windows, `cls` does the same thing.

## Shortcuts that save time

- **Up arrow**: brings back the previous command. Press it several times
  to go further back.
- **Tab**: completes a name. Type `cd Doc` then Tab: the terminal writes
  `Documents` for you.
- **Ctrl + C**: stops the running program. Handy if it won't stop.

> **Tip:** use Tab all the time. It's faster, and it avoids typos in file
> names.

> **Warning:** in the terminal, Ctrl + C doesn't copy: it stops. To copy,
> use Ctrl + Shift + C (Linux), Cmd + C (macOS) or mouse selection
> (Windows).

## Running a program

First, with `cd`, go into the folder that contains your file. Then type
the name of the language followed by the name of the file.

A Ruby program:

```sh
ruby hello.rb
```

A Rythme program:

```sh
rythme hello.ry
```

A Rust program lives in a **project**. In the project folder, this command
compiles then runs the program:

```sh
cargo run
```

Each language is installed once. See the introduction pages:
[Ruby](../ruby/introduction.md), [Rust](../rust/introduction.md) and
[Rythme](../rythme/introduction.md).

## Reading an error

If something goes wrong, the terminal displays a message. For example, if
the file doesn't exist:

```text
ruby: No such file or directory -- helo.rb (LoadError)
```

Read it calmly. Here, the name is misspelled: `helo` instead of `hello`.
The most common causes:

- a typo in the name;
- you're not in the right folder (check with `pwd` and `ls`);
- the language isn't installed: the terminal says "command not found" or
  "is not recognized".

## Windows and macOS/Linux

Most commands are the same. Here are the differences:

| Action                    | macOS and Linux | Windows (PowerShell)  |
|---------------------------|-----------------|-----------------------|
| Where am I?               | `pwd`           | `pwd`                 |
| List the files            | `ls`            | `dir` (or `ls`)       |
| Go into a folder          | `cd folder`     | `cd folder`           |
| Go up one level           | `cd ..`         | `cd ..`               |
| Create a folder           | `mkdir code`    | `mkdir code`          |
| Clear the screen          | `clear`         | `cls` (or `clear`)    |
| Path separator            | `/`             | `\`                   |

On Windows, a path is written with backslashes:

```powershell
cd C:\Users\Alice\Documents\code
```

## Common mistakes

### A folder name with a space

The terminal separates words with spaces. Here, it thinks it's getting two
folders, `My` and `Documents`:

```sh
cd My Documents
```

```text
bash: cd: too many arguments
```

Put the name between quotes: `cd "My Documents"`. Or type `cd My` then
Tab: the terminal completes the name correctly. For your own folders,
avoid spaces: `my_documents`.

### A typo in a name

```sh
cd Documentz
```

```text
bash: cd: Documentz: No such file or directory
```

The folder doesn't exist under that name. Type `ls` to see the exact
names, and use Tab so you stop making typos.

### An unknown command

```sh
rubyy hello.rb
```

```text
bash: rubyy: command not found
```

Either the command name is misspelled (here, `rubyy` instead of `ruby`),
or the language isn't installed. Check with `ruby --version`.

## Test yourself

**1.** You don't remember which folder you're in. Which command do you
type?

> **Answer**
>
> `pwd`, which displays the current folder. Then `ls` (or `dir`) to see
> what it contains.

**2.** How do you go up to the parent folder?

> **Answer**
>
> With `cd ..`: the two dots mean the parent folder.

**3.** Your program runs forever. How do you stop it?

> **Answer**
>
> With **Ctrl + C** in the terminal. Careful: in the terminal, this
> shortcut stops, it doesn't copy.

## Summary

- The terminal lets you talk to the computer by typing commands.
- `pwd` tells you where you are, `ls` (or `dir`) lists, `cd` moves, `mkdir` creates a folder.
- The up arrow brings back a command, Tab completes, Ctrl + C stops.
- You run a program with `ruby file.rb`, `rythme file.ry` or `cargo run`.
- Read an error message calmly: it almost always tells you what to fix.

Next: [install a code editor](editeur.md).
