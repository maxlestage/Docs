# Go: introduction and installation

This page introduces Go, explains how to install it, and has you write, run and compile your first program.

> **Goals:**
>
> - you will be able to say what Go is and what it is used for;
> - you will know how to install Go and check that it works with `go version`;
> - you will know how to write a program with `package main`, `import` and `func main`;
> - you will know how to run a program with `go run` and compile it with `go build`;
> - you will know how to format your code with `gofmt`.

## What is Go?

Go is a programming language created at Google in 2009, by Robert Griesemer, Rob
Pike and Ken Thompson. It is also called "Golang", to make it easier to find with a
search engine.

A **programming language** is a precise language that the computer knows how to
follow. A **program** is a text written in that language: a list of orders, like a
cooking recipe.

```go
package main

import "fmt"

func main() {
	fmt.Println("Hello from Go!")
}
```

```sortie
Hello from Go!
```

These few lines are a complete program. We will explain them one by one a bit
further down.

## Why learn Go?

Go was designed to be **simple**. The language has few keywords: you get around them
quickly.

- **It is fast.** Go is a [compiled](../debuter/glossaire.md#compilation) language:
  before it runs, your code is translated into machine language. The resulting
  program runs very fast.
- **It is easy to read.** There is often only one good way to write something.
  Someone else's code looks like yours.
- **It produces a single file.** The result of compilation is a
  [binary](../debuter/glossaire.md#binary): a file that you copy to another machine,
  and that works without installing anything.
- **It can do several things at once.** Go was designed for
  [concurrency](../debuter/glossaire.md#concurrency): running thousands of small
  tasks at the same time is easy.

Go is widely used for **servers** (the programs that answer websites and apps) and
for command-line **tools**. Docker, Kubernetes and Hugo are written in Go.

> **Note:** this site also presents other languages: compare with
> [the introduction to Ruby](../ruby/introduction.md) or
> [the introduction to Rust](../rust/introduction.md).

### Compiled or interpreted?

With an **interpreted** language (like Ruby or Python), a program reads your file and
carries out the orders one by one, every time you run it.

With a **compiled** language like Go, a
[compiler](../debuter/glossaire.md#compiler) first reads your whole file. It
checks that there is no error, then translates it into machine language. It's like
translating a whole book before handing it over to read, instead of translating it
out loud page by page.

> **Remember:** the Go compiler is very strict. It refuses an unused variable or
> import. It is sometimes annoying at first, but your code stays clean.

## Installing Go

This site uses **Go 1.24**. A newer version works too.

Every version can be downloaded from the official website: **go.dev/dl**.

### On Windows

Download the `.msi` file for Windows from go.dev/dl, then run it. Keep the default
options. The installer adds Go to the terminal by itself.

### On macOS

Download the `.pkg` file from go.dev/dl (choose "Apple" or "ARM64" for a recent Mac),
then run it. You can also use Homebrew:

```sh
brew install go
```

### On Linux

Your system's package manager often offers Go, but its version may be old. The
safest choice is the official archive from go.dev/dl:

```sh
sudo rm -rf /usr/local/go
sudo tar -C /usr/local -xzf go1.24.7.linux-amd64.tar.gz
echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.profile
```

- The first line removes any old version.
- The second unpacks Go into `/usr/local/go`.
- The third adds Go's `bin` folder to the `PATH`: the list of folders where the
  terminal looks for commands.

Log out and back in (or type `source ~/.profile`) for the change to take effect.

> **Tip:** on Ubuntu, `sudo snap install go --classic` also installs a recent
> version.

## Checking the installation

Open a terminal and type the following command. If the terminal is new to you, first
read [the terminal](../debuter/terminal.md).

```sh
go version
```

You should see a version number, for example:

```text
go version go1.24.7 linux/amd64
```

The end shows your system (`linux`, `darwin` for macOS, `windows`) and the kind of
processor (`amd64`, `arm64`).

If the terminal answers "command not found", Go is not installed, or the terminal was
opened before the installation: close it and open it again.

## Your first program

Open your [code editor](../debuter/editeur.md). Create a folder `hello`, and inside it
a file `hello.go` with this content:

```go
package main

import "fmt"

func main() {
	fmt.Println("Hello, world!")
}
```

```sortie
Hello, world!
```

Let's read it line by line.

- `package main`: every Go file belongs to a **package**, a group of files that go
  together. The `main` package is special: it is the one for a program you can run.
- `import "fmt"`: we import the `fmt` package (for *format*), which comes with Go. It
  knows how to print text.
- `func main() { ... }`: we define the [function](../debuter/glossaire.md#function)
  `main`. It is the front door: Go always starts with it.
- `fmt.Println("...")`: we call the `Println` function of the `fmt` package. It
  prints the text, then moves to the next line (*ln* for *line*).

The text is between double quotes `"..."`. It is a
[string](../debuter/glossaire.md#string).

### Several lines

The statements in `main` run in order, from top to bottom.

```go
package main

import "fmt"

func main() {
	fmt.Println("Line 1")
	fmt.Println("Line 2")
	fmt.Println(2 + 3)
	fmt.Println("Total:", 2+3, "euros")
}
```

```sortie
Line 1
Line 2
5
Total: 5 euros
```

- `fmt.Println(2 + 3)` prints `5`: without quotes, Go computes.
- With several values separated by commas, `Println` prints them with a space
  between each one.

> **Note:** there is no semicolon at the end of lines. Go adds them by itself while
> reading the file.

## Running a program with go run

In the terminal, go into the folder of the file (with the `cd` command), then type:

```sh
go run hello.go
```

```text
Hello, world!
```

`go run` does two things at once: it compiles the file, then runs the result.
Nothing is kept on disk. It is the command you will use most to try out your code.

## Compiling with go build

`go build` compiles the file and **keeps** the resulting program.

```sh
go build hello.go
./hello
```

```text
Hello, world!
```

- On Linux and macOS, `go build` creates a file `hello`. You run it with
  `./hello`.
- On Windows, it creates `hello.exe`. You run it with `.\hello.exe`.

This file stands on its own: you can copy it to another machine with the same
system, even without Go installed on it, and it will work.

> **Remember:** `go run` to try things out, `go build` to make the final program.

## Formatting with gofmt

In Go, everyone lays out their code the same way. The `gofmt` tool (and the
`go fmt` command) tidies up your file for you: it fixes spaces, aligns lines and uses
tabs for indentation.

```sh
gofmt -w hello.go
```

The `-w` (*write*) option rewrites the file. Without it, `gofmt` only prints the
result.

> **Tip:** most editors (with the Go extension) run `gofmt` every time you save.
> Turn it on: you will never think about layout again.

## Reading an error message

When the compiler doesn't understand your code, it stops and prints an
[error](../debuter/glossaire.md#error). Don't worry: the message helps you.

```text
# command-line-arguments
./main.go:6:6: undefined: fmt.println
```

Read it from left to right:

- `# command-line-arguments`: the name Go gives to the package run directly;
- `./main.go:6:6`: the file, the line (6) and the column (6) of the problem;
- `undefined: fmt.println`: the description (`fmt.println` is not defined).

A few words come up often in these messages: *undefined*, *unexpected*, *expected*,
*missing*, *not used*.

## Common mistakes

### Forgetting the capital letter of Println

```go norun
package main

import "fmt"

func main() {
	fmt.println("Hello")
}
```

```text
# command-line-arguments
./main.go:6:6: undefined: fmt.println
```

Go tells uppercase and lowercase letters apart. The function is called `Println`,
with a capital `P`. Write `fmt.Println("Hello")`.

### Importing a package without using it

```go norun
package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Println("Hello")
}
```

```text
# command-line-arguments
./main.go:5:2: "os" imported and not used
```

The `os` package is imported, but never used. Go refuses to compile. Remove the
`"os"` line.

### Putting the brace on the next line

```go norun
package main

import "fmt"

func main()
{
	fmt.Println("Hello")
}
```

```text
# command-line-arguments
./main.go:6:1: syntax error: unexpected semicolon or newline before {
```

Go adds an invisible semicolon at the end of `func main()`. So the brace `{` must be
on the **same** line: `func main() {`.

### Getting the package name wrong

```go norun
package hello

import "fmt"

func main() {
	fmt.Println("Hello")
}
```

```text
package command-line-arguments is not a main package
```

A program you run must be in the `main` package. Write `package main` on the first
line.

## Test yourself

**Question 1:** what is the difference between `go run` and `go build`?

> **Answer**
>
> `go run` compiles then runs the program, without keeping anything. `go build`
> compiles and creates an executable file, which you then run yourself.

**Question 2:** write a program that prints `I'm learning Go` then, on the next line,
the result of `7 * 6`.

> **Answer**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	fmt.Println("I'm learning Go")
> 	fmt.Println(7 * 6)
> }
> ```
>
> ```sortie
> I'm learning Go
> 42
> ```

**Question 3:** which function does a Go program always start with?

> **Answer**
>
> The `main` function of the `main` package.

## Summary

- Go is a compiled, simple and fast language, created at Google; it is mostly used
  for servers and tools.
- Install it from go.dev/dl, then check with `go version`.
- A program starts with `package main`, imports its packages with `import`, and
  begins in `func main()`.
- `go run file.go` runs the code; `go build` makes an executable.
- `gofmt` formats the code; the compiler refuses unused imports.

Next: [first steps](premiers-pas.md).
