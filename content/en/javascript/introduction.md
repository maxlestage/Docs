# JavaScript: introduction and installation

This page introduces JavaScript, explains how to install Node.js, and has you write your first program.

> **Goals:**
>
> - you will be able to say what JavaScript is and where it runs;
> - you will be able to install Node.js and check that it works;
> - you will be able to try code in the browser console and in Node;
> - you will be able to write a `.js` file and run it with `node`;
> - you will know what an ES module is and why this site uses them.

## What is JavaScript?

JavaScript is **the language of the web**. It was created in 1995 by Brendan Eich, in
ten days, to bring web pages to life: a button that reacts, a menu that opens, a form
that checks what you type.

A **programming language** is a precise language that the computer can follow.
A [program](../debuter/glossaire.md#program) is a text written in that language:
a list of instructions, like a cooking recipe.

```javascript
console.log("Hello!");
console.log(2 + 3);
```

```sortie
Hello!
5
```

`console.log(...)` displays a value. Read it as "in the console, write this down". The
semicolon `;` marks the end of a statement, like the period at the end of a sentence.

> **Note:** despite its name, JavaScript has almost nothing to do with the Java
> language. The name was chosen in 1995 to ride on Java's popularity.

## Where does JavaScript run?

JavaScript is an **interpreted** language: a program called an
[interpreter](../debuter/glossaire.md#interpreter) reads your code and runs it
directly. There are two main places where this interpreter lives.

- **In the browser** (Firefox, Chrome, Safari…). Every browser contains a JavaScript
  engine. It is what makes web pages move.
- **In Node.js**, a program that runs JavaScript **outside the browser**: in the
  [terminal](../debuter/terminal.md), on a server, for scripts.

It is like the same language spoken in two countries: the words are the same, but some
things only exist in one country. The browser knows how to handle a web page; Node
knows how to read files on your disk.

This site uses **Node.js** for almost all the examples. The page about
[the DOM](dom.md) shows what is specific to the browser.

## Installing Node.js

This site uses **Node.js 22**. A more recent version works too.

### From the official website

Go to **nodejs.org** and download the **LTS** version (*Long Term Support*): it is the
stable version, recommended for everyone. Run the installer and keep the suggested
options. This works on Windows, macOS and Linux.

### With nvm

`nvm` (*Node Version Manager*) installs several versions of Node and switches between
them. Once nvm is installed (see its page on GitHub), type:

```sh
nvm install 22
nvm use 22
```

> **Tip:** use nvm if you work on several projects that need different versions of
> Node. On Windows, the equivalent is called **nvm-windows**.

### With a package manager

On Linux and macOS, you can also use the system's package manager.

```sh
sudo apt install nodejs      # Debian, Ubuntu
brew install node            # macOS with Homebrew
```

> **Warning:** the packages of some Linux distributions are old. Check the version; if
> it is lower than 22, prefer nodejs.org or nvm.

## Checking the installation

Open a terminal and type the following command. If the terminal is new to you, first
read [the terminal](../debuter/terminal.md).

```sh
node --version
```

You should see a version number, for example `v22.22.0`. If the terminal answers
"command not found", Node is not installed, or the terminal was opened before the
installation: close it and open it again.

Node comes with **npm** (*Node Package Manager*), the tool that installs
[libraries](../debuter/glossaire.md#library). `npm --version` checks that it is
there too. You will not need it before the page about [modules](modules.md).

## Trying JavaScript right away

### In the browser console

You already have JavaScript: it is in your browser. Open any page, then press **F12**
(or **Cmd + Option + I** on Mac). Choose the **Console** tab. Type a line, then Enter:
the browser answers right away.

```text
> 2 + 3
5
> "Hel" + "lo"
'Hello'
```

It is an [interactive console](../debuter/glossaire.md#repl): perfect for testing a
small idea without creating a file.

### In Node

Type `node` alone in the terminal. You get the same interactive console, in the
terminal this time.

```text
$ node
Welcome to Node.js v22.22.0.
Type ".help" for more information.
> 1 + 2
3
> "Hel" + "lo"
'Hello'
> .exit
```

- The `>` sign waits for your line. The next line shows the result.
- `.exit` (or **Ctrl + C** twice) leaves the console.

## Your first program

Open your [code editor](../debuter/editeur.md). Create a `js` folder, then a file
`hello.js` with this content:

```javascript
console.log("Hello, world!");
```

```sortie
Hello, world!
```

- `console.log` displays a value, then goes to a new line.
- The text is between quotes `"..."`. Without quotes, JavaScript would think it is a
  variable name.

A program can have several lines. JavaScript runs them in order, from top to bottom.

```javascript
console.log("Line 1");
console.log("Line 2");
console.log(2 + 3);
console.log("2 + 3");
```

```sortie
Line 1
Line 2
5
2 + 3
```

Without quotes, JavaScript calculates. Between quotes, it displays the text as it is.

## Running a program

In the terminal, go to the file's folder (with the `cd` command), then type `node`
followed by the file name:

```sh
cd js
node hello.js
```

```text
Hello, world!
```

> **Remember:** a JavaScript program is a `.js` file. You run it with
> `node file_name.js`.

## ES modules

A **module** is a code file that can share things with other files. JavaScript has two
ways to write modules:

- **ES modules** (*ECMAScript modules*), the modern and standard way, with the words
  `import` and `export`;
- **CommonJS**, the old way specific to Node, with `require(...)`.

This site **always uses ES modules**. To let Node know, there are two solutions:

1. name your files **`.mjs`** instead of `.js` (`hello.mjs`);
2. or add a `package.json` file in the folder, with this content:

```json
{
  "type": "module"
}
```

With this file, all the `.js` files in the folder are ES modules. Here is a program
that uses `import` to read Node's platform:

```javascript
import { platform } from "node:process";

console.log(typeof platform);
```

```sortie
string
```

`import` fetches `platform` from the `node:process` module, which comes with Node.
Do not try to understand everything: [modules](modules.md) have their own page.

> **Note:** Node 22 often guesses on its own that a `.js` file is an ES module when it
> sees `import`. But saying things clearly (`.mjs` or `"type": "module"`) avoids
> surprises.

## Reading an error message

When JavaScript does not understand your code, it stops and displays an
[error](../debuter/glossaire.md#error). That is fine: the message helps you.

```text
/home/alice/js/hello.js:1
console.log(Hello);
            ^

ReferenceError: Hello is not defined
    at Object.<anonymous> (/home/alice/js/hello.js:1:13)

Node.js v22.22.0
```

Read it from top to bottom:

- `hello.js:1`: the file and the line number of the problem;
- the line of code, with a `^` under the exact spot;
- `ReferenceError`: the type of error; `Hello is not defined`: the cause;
- the `at ...` lines: the path the program followed (here `1:13` means line 1,
  column 13).

Some words come up often: *not defined*, *unexpected*, *cannot*, *is not a function*.

## Common mistakes

### A capital letter in console

```javascript norun
Console.log("Hello");
```

```text
/home/alice/js/hello.js:1
Console.log("Hello");
^

ReferenceError: Console is not defined
    at Object.<anonymous> (/home/alice/js/hello.js:1:1)

Node.js v22.22.0
```

JavaScript makes a difference between uppercase and lowercase letters. `Console` and
`console` are two different names, and only the second one exists. Write
`console.log("Hello");`.

### Forgetting the closing quote

```javascript norun
console.log("Hello);
```

```text
/home/alice/js/hello.js:1
console.log("Hello);
            ^^^^^^^^

SyntaxError: Invalid or unexpected token

Node.js v22.22.0
```

The text is never closed: JavaScript no longer knows where it ends (*invalid or
unexpected token*). Add the quote: `console.log("Hello");`.

### Running the file from the wrong folder

```sh
node hello.js
```

```text
node:internal/modules/cjs/loader:1386
  throw err;
  ^

Error: Cannot find module '/home/alice/hello.js'
```

Node looks for `hello.js` in the folder the terminal is in (here `/home/alice`), not
anywhere else. Go to the right folder with `cd js`, or check the file name with `ls`.

### Typing the command in the Node console

```text
> node hello.js
     ^^^^^

Uncaught SyntaxError: Unexpected identifier 'hello'
```

The `>` prompt shows that you are **inside** Node, which expects JavaScript, not a
terminal command. Leave with `.exit`, then type `node hello.js`.

## Test yourself

**Question 1:** which command checks that Node.js is installed?

> **Answer**
>
> `node --version`. It displays the version number, for example `v22.22.0`.

**Question 2:** what does this program display?

```javascript norun
console.log("4 * 2");
console.log(4 * 2);
```

> **Answer**
>
> `4 * 2`, then `8`. Between quotes, it is a text displayed as it is. Without quotes,
> JavaScript calculates.

**Question 3:** you want to use `import` in `game.js`. What do you do?

> **Answer**
>
> Rename the file to `game.mjs`, or add a `package.json` that contains
> `"type": "module"`. Then run it with `node game.mjs` (or `node game.js`).

## Summary

- JavaScript is the language of the web: it runs in the browser and, with Node.js, in
  the terminal.
- Install Node from nodejs.org (LTS version) or with nvm; `node --version` checks it.
- The browser console (F12) and the `node` command alone let you try code right away.
- A program is a `.js` file run with `node file_name.js`; `console.log` displays a
  value.
- This site uses ES modules: `.mjs` files or `"type": "module"`.
- An error message gives the file, the line, the type of error and the cause.

Next: [first steps](premiers-pas.md).
