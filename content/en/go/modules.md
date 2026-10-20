# Packages and modules

This page explains how Go organizes code into packages, how to create a module with `go mod init`, and how to use code written by other people.

> **Goals:**
>
> - you will know how to import a package and use its exported names;
> - you will know how to create a module with `go mod init` and read the `go.mod` file;
> - you will know how to split a project across several files and folders;
> - you will know how to protect code with an `internal` folder;
> - you will know how to add an external module and read the documentation with `go doc`.

## Packages

A **package** is a folder of Go files that work together. Each file starts with
`package name`. The `main` package is special: it's the package of a
[program](../debuter/glossaire.md#program), and it contains the `main` function.

Think of a public library: each shelf (package) gathers books on the same subject. You
don't take the whole building home, you borrow from the shelf you need.

`import` makes a package available. Then you write the package name, a dot, then the
function: `strings.ToUpper`.

```go
package main

import (
	"fmt"
	"math"
	"strings"
)

func main() {
	fmt.Println(strings.ToUpper("hello"))
	fmt.Println(math.Sqrt(16))
}
```

```sortie
HELLO
4
```

### The path and the name

The text in quotes is the package's **path**. The name to use in the code is the last
part of the path: with `"math/rand"`, you write `rand.Intn`. If two packages have the
same name, give one of them an **alias**.

```go
package main

import (
	"fmt"
	str "strings"
	"unicode/utf8"
)

func main() {
	word := "café"
	fmt.Println(str.Repeat("-", 5))
	fmt.Println(utf8.RuneCountInString(word), "letters")
}
```

```sortie
-----
4 letters
```

## Exported names

In Go, the **capital letter** decides what is visible outside the package:

- a name that starts with a capital letter (`ToUpper`, `Pi`, `Player`) is **exported**:
  other packages can use it;
- a lowercase name (`capitalize`, `counter`) stays **private** to the package.

No `public` or `private` keyword: the first letter is enough. This applies to
functions, types, variables, constants and even the fields of
[structs](types.md).

```go
package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Printf("%.4f\n", math.Pi)
	fmt.Println(math.MaxInt8)
}
```

```sortie
3.1416
127
```

## Creating a module

A **module** is a whole project: one or more packages, with a name and a list of
dependencies. `go mod init` creates it. You give it a **module path**, often the address
of the repository where it will be published (`github.com/you/project`). For a personal
project, any name will do.

```sh
mkdir hello
cd hello
go mod init example.com/hello
```

```text
go: creating new go.mod: module example.com/hello
```

The command creates a `go.mod` file:

```text
module example.com/hello

go 1.24.7
```

- `module` gives the module path: the imports of your packages will start with it.
- `go` states the Go version used.

Write a `main.go` next to it, then run `go run .`: the dot means "the package in the
current folder".

## A project in several files

In the same folder, all files belong to the same package: they share their functions
and types without importing anything. For a new package, create a **subfolder**.

Here is a project with a `greeting` package:

```text
hello/
├── go.mod
├── main.go
└── greeting/
    └── greeting.go
```

The file `greeting/greeting.go`:

```go norun
// Package greeting builds welcome messages.
package greeting

// Hello returns a welcome message for name.
func Hello(name string) string {
	return "Hello, " + capitalize(name) + "!"
}

func capitalize(s string) string {
	if s == "" {
		return s
	}
	return string(s[0]-'a'+'A') + s[1:]
}
```

The file `main.go` imports the package with the module path followed by the folder:

```go norun
package main

import (
	"fmt"

	"example.com/hello/greeting"
)

func main() {
	fmt.Println(greeting.Hello("alice"))
}
```

```sh
go run .
```

```text
Hello, Alice!
```

`main.go` can call `greeting.Hello` (capital letter), but not `greeting.capitalize`
(lowercase): it's an internal detail of the package.

> **Tip:** a comment right above `package` or an exported name becomes its
> documentation. Start it with the name it describes: "Hello returns…".

## Internal packages

A folder named `internal` is protected: only the code **of the parent folder and its
subfolders** can import what it contains. It's a "staff only" room.

```text
hello/
├── go.mod
├── main.go
└── greeting/
    ├── greeting.go
    └── internal/
        └── tool/
            └── tool.go
```

Here, `greeting` can import `example.com/hello/greeting/internal/tool`. But `main.go`,
which is **outside** `greeting/`, cannot:

```text
package example.com/hello
	main.go:6:2: use of internal package example.com/hello/greeting/internal/tool not allowed
```

This way you can reorganize this code later without breaking other people's programs.

## Adding an external module

Thousands of modules are published online. `go get` downloads one and adds it to
`go.mod`. For example, the `uuid` module generates unique identifiers:

```sh
go get github.com/google/uuid
```

```text
go: downloading github.com/google/uuid v1.6.0
go: added github.com/google/uuid v1.6.0
```

The `go.mod` file gains a `require` line, and a `go.sum` file appears: it contains the
checksums of the downloaded files, to check that nobody has modified them.

```go norun
package main

import (
	"fmt"

	"github.com/google/uuid"
)

func main() {
	fmt.Println(uuid.NewString())
}
```

After adding or removing imports, `go mod tidy` tidies up `go.mod`: it adds what is
missing and removes what is no longer used.

> **Note:** prefer the standard library when it's enough. It's very rich (text, files,
> JSON, web, tests…) and adds no dependency.

## Reading the documentation with go doc

`go doc` shows the documentation of a package or a function, right in the
[terminal](../debuter/terminal.md).

```sh
go doc strings.ToUpper
```

```text
package strings // import "strings"

func ToUpper(s string) string
    ToUpper returns s with all Unicode letters mapped to their upper case.
```

It also works for your own packages, thanks to the comments:

```sh
go doc ./greeting
```

```text
package greeting // import "example.com/hello/greeting"

Package greeting builds welcome messages.

func Hello(name string) string
```

Only exported names appear. The same documentation is online at `pkg.go.dev`.

## Common mistakes

### An unused import

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

Go refuses an import that isn't used. Delete the `"os"` line. Many editors do it for
you when you save (with the `goimports` tool).

### Calling a name that isn't exported

```go norun
fmt.Println(greeting.capitalize("alice"))
```

```text
# example.com/hello
./main.go:10:23: name capitalize not exported by package greeting
```

`capitalize` starts with a lowercase letter: it's invisible outside its package. Use an
exported function, or rename it `Capitalize` if it really needs to be public.

### Running go run . without a module

```sh
go run .
```

```text
go: go.mod file not found in current directory or any parent directory; see 'go help modules'
```

The folder has no `go.mod`. Run `go mod init example.com/project` first, or move into
the module's folder.

## Test yourself

**Question 1:** in a package, what is the difference between `Compute` and
`compute`?

> **Answer**
>
> `Compute` starts with a capital letter: it's exported, other packages can call it.
> `compute` is private to its package.

**Question 2:** your module is called `example.com/game` and contains a `score/`
folder. Which import line do you write in `main.go`?

> **Answer**
>
> `import "example.com/game/score"`, then you use `score.Something`.

**Question 3:** what is `go mod tidy` for?

> **Answer**
>
> It puts `go.mod` and `go.sum` in order: it adds the imported modules that are missing
> and removes the ones that are no longer used.

## Summary

- A package is a folder; `import` makes it available, and you write `pkg.Name`.
- A capital letter at the start of a name exports it; a lowercase letter keeps it
  private.
- `go mod init path` creates a module and its `go.mod` file.
- A subfolder is a new package; `internal/` limits who can import it.
- `go get` adds an external module; `go doc` shows the documentation.

Next: [goroutines](concurrence.md). Compare with
[modules in Rust](../rust/modules.md).
