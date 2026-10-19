# Embedding files with embed

This page shows how to store files (text, templates, web pages, configuration) directly inside the compiled program, with the `//go:embed` directive and the `io/fs` package.

> **Goals:**
>
> - you will know why you would embed files in a Go program;
> - you will know how to embed a file in a `string` or a `[]byte` with `//go:embed`;
> - you will know how to embed several files or a folder in an `embed.FS`;
> - you will know how to read these files with `fs.ReadFile`, `fs.ReadDir`, `fs.WalkDir` and `fs.Sub`;
> - you will know how to serve embedded files on the web and load templates with `ParseFS`;
> - you will know the rules of `//go:embed` and the cases where it's better not to embed.

## Why embed files?

A Go program compiles into a single [binary](../debuter/glossaire.md#binary): one
executable file that you copy to another machine, and it works. But many programs need
files next to them: page templates, a default configuration, a style sheet. If you
forget to copy them, the program crashes.

**Embedding** a file means putting it *inside* the binary, at compile time. It's like
a suitcase that already holds everything: just one thing to carry. With a `main.go`
that embeds `message.txt` (the code is explained right after):

```sh
go build -o hello .
rm message.txt
./hello
```

```text
Hello from an embedded file!
```

The file was deleted, but its text is still there: it was copied into the binary
during `go build`.

## The //go:embed directive on a string

Three things are enough to embed a file:

1. import the `embed` package. When you don't use it by its name, you import it
   with `_`: `import _ "embed"`;
2. write the special comment `//go:embed file-name`, **with no space** between
   `//` and `go:embed`;
3. declare, right below it, a variable of type `string`.

Here, the program embeds its own code, `main.go`: this file is sure to exist.

```go
package main

import (
	_ "embed"
	"fmt"
	"strings"
)

//go:embed main.go
var source string

func main() {
	lines := strings.Split(source, "\n")
	fmt.Println("First line:", lines[0])
}
```

```sortie
First line: package main
```

The [compiler](../debuter/glossaire.md#compiler) reads `main.go` and puts all its
content in `source`. No `os.ReadFile`, no error to check: if the file is missing, it's
the compilation that fails.

> **Warning:** `// go:embed` (with a space) is just an ordinary comment.
> Go ignores it, and the variable stays empty.

## Embedding in a []byte

For an image or any file that isn't text, use a slice of
[bytes](../debuter/glossaire.md#byte), `[]byte`. It's also the type that
`json.Unmarshal` expects, for example.

```go
package main

import (
	_ "embed"
	"fmt"
)

//go:embed main.go
var content []byte

func main() {
	fmt.Printf("Type: %T\n", content)
	fmt.Println("Start:", string(content[:12]))
	fmt.Println("First byte:", content[0])
}
```

```sortie
Type: []uint8
Start: package main
First byte: 112
```

`[]uint8` is the other name of `[]byte`, and `112` is the code of the letter `p`.

> **Remember:** `string` for text, `[]byte` for bytes. Both receive only **one**
> file.

## Several files with embed.FS

For several files, or a whole folder, the type is `embed.FS`: a small read-only
**file system**, stored in the binary. This time you use the package by its name: the
import is written `import "embed"`, without `_`.

```go
package main

import (
	"embed"
	"fmt"
)

//go:embed main.go
var files embed.FS

func main() {
	content, err := files.ReadFile("main.go")
	fmt.Println("main.go:", len(content) > 0, err)

	_, err = files.ReadFile("missing.txt")
	fmt.Println("missing.txt:", err)
}
```

```sortie
main.go: true <nil>
missing.txt: open missing.txt: file does not exist
```

`ReadFile` returns an error, because the name is chosen while the program runs: the
compiler can't know whether it exists.

### Patterns and folders

After `//go:embed`, you can write several names, **patterns** like `*.txt`, or folder
names. Take this project:

```text
site/
├── go.mod
├── main.go
├── version.txt          (contains "1.4.2")
├── templates/
│   ├── goodbye.txt
│   └── hello.txt
└── static/
    ├── .draft.txt
    ├── _notes.txt
    ├── index.html
    ├── style.css
    └── images/
        └── logo.png
```

```go norun
package main

import (
	"embed"
	"fmt"
	"io/fs"
)

//go:embed version.txt templates/*.txt
var files embed.FS

//go:embed static
var staticFiles embed.FS

func list(name string, fsys fs.FS) {
	fmt.Println(name + ":")
	fs.WalkDir(fsys, ".", func(path string, d fs.DirEntry, err error) error {
		if !d.IsDir() {
			fmt.Println("  " + path)
		}
		return nil
	})
}

func main() {
	version, _ := files.ReadFile("version.txt")
	fmt.Print("Version: ", string(version))
	list("files", files)
	list("staticFiles", staticFiles)
}
```

```text
Version: 1.4.2
files:
  templates/goodbye.txt
  templates/hello.txt
  version.txt
staticFiles:
  static/images/logo.png
  static/index.html
  static/style.css
```

- `templates/*.txt` takes all the `.txt` files of the `templates` folder;
- `static` takes the folder **and its subfolders**;
- the paths keep their folder: you read `static/style.css`, not `style.css`.

`.draft.txt` and `_notes.txt` are missing: this is one of the
[rules to know](#rules-to-know).

## Reading a file system with io/fs

An `embed.FS` satisfies the `fs.FS` interface of the `io/fs` package: all the functions
of this package work on it. To try them without creating files, `testing/fstest`
offers `fstest.MapFS`, a fake file system written as a map. The code is the same as
with a real `embed.FS`.

### fs.ReadFile and fs.ReadDir

`fs.ReadFile` reads a whole file. `fs.ReadDir` lists a folder, sorted by name.

```go
package main

import (
	"fmt"
	"io/fs"
	"testing/fstest"
)

func main() {
	var fsys fs.FS = fstest.MapFS{
		"config.txt":     {Data: []byte("color=blue\n")},
		"docs/help.txt":  {Data: []byte("Type q to quit.\n")},
		"docs/intro.txt": {Data: []byte("Welcome!\n")},
	}

	content, _ := fs.ReadFile(fsys, "config.txt")
	fmt.Print("config.txt: ", string(content))

	entries, _ := fs.ReadDir(fsys, "docs")
	for _, e := range entries {
		fmt.Println("in docs:", e.Name())
	}
}
```

```sortie
config.txt: color=blue
in docs: help.txt
in docs: intro.txt
```

The paths of `io/fs` always use `/`, even on Windows, and the root is called `.`.

### fs.WalkDir: walking the whole tree

`fs.WalkDir` visits every file and every folder, subfolders included, and calls your
function for each one.

```go
package main

import (
	"fmt"
	"io/fs"
	"testing/fstest"
)

func main() {
	fsys := fstest.MapFS{
		"index.html":      {Data: []byte("<h1>Home</h1>")},
		"css/style.css":   {Data: []byte("body {}")},
		"images/logo.png": {Data: []byte("PNG")},
	}

	fs.WalkDir(fsys, ".", func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			fmt.Println("[folder]", path)
		} else {
			fmt.Println("        ", path)
		}
		return nil
	})
}
```

```sortie
[folder] .
[folder] css
         css/style.css
[folder] images
         images/logo.png
         index.html
```

### fs.Sub: going into a subfolder

`fs.Sub` returns a new file system whose root is a subfolder. It's very useful with
`//go:embed static`: you want to read `style.css`, not `static/style.css`.

```go
package main

import (
	"fmt"
	"io/fs"
	"testing/fstest"
)

func main() {
	fsys := fstest.MapFS{
		"static/style.css": {Data: []byte("body { color: navy; }\n")},
	}

	root, _ := fs.Sub(fsys, "static")
	css, _ := fs.ReadFile(root, "style.css")
	fmt.Print(string(css))
}
```

```sortie
body { color: navy; }
```

## Serving files on the web

With an `embed.FS`, a small website fits in a single binary. Go's
[HTTP](../debuter/glossaire.md#http) server serves an `fs.FS` thanks to
`http.FileServerFS` (Go 1.22 and later). The `static/` folder contains `index.html`
(`<h1>Welcome</h1>`) and `style.css`:

```go norun
package main

import (
	"embed"
	"io/fs"
	"log"
	"net/http"
)

//go:embed static
var staticFiles embed.FS

func main() {
	root, err := fs.Sub(staticFiles, "static")
	if err != nil {
		log.Fatal(err)
	}

	http.Handle("/", http.FileServerFS(root))

	log.Println("Server on http://localhost:8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
```

While the server is running, in another terminal:

```sh
curl http://localhost:8080/
curl http://localhost:8080/style.css
```

```text
<h1>Welcome</h1>
body { color: navy; }
```

Without `fs.Sub`, the addresses would be `/static/index.html` and `/static/style.css`.

> **Note:** before Go 1.22, you wrote `http.FileServer(http.FS(root))`:
> `http.FS` turns an `fs.FS` into an `http.FileSystem`, the old expected type. You will
> often see it in existing code. The chapter [the web](web.md) covers the server in
> detail.

## Loading templates with ParseFS

The `text/template` and `html/template` packages read their templates from an `fs.FS`
with `ParseFS`. You give it the file system and one or more patterns. Each template
takes the name of its file.

```go
package main

import (
	"fmt"
	"os"
	"testing/fstest"
	"text/template"
)

func main() {
	fsys := fstest.MapFS{
		"templates/hello.txt":   {Data: []byte("Hello {{.}}!\n")},
		"templates/goodbye.txt": {Data: []byte("Goodbye {{.}}.\n")},
	}

	templates, err := template.ParseFS(fsys, "templates/*.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	templates.ExecuteTemplate(os.Stdout, "hello.txt", "Alice")
	templates.ExecuteTemplate(os.Stdout, "goodbye.txt", "Alice")
}
```

```sortie
Hello Alice!
Goodbye Alice.
```

In a real project, you embed the `templates` folder, then load the templates only
once, at startup, in a package variable. `template.Must` stops the program if a
template is wrong: you see it right away.

```go norun
//go:embed templates/*.html
var templatesDir embed.FS

var pages = template.Must(template.ParseFS(templatesDir, "templates/*.html"))
```

With `html/template`, a name like `Alice <admin>` is displayed as `Alice &lt;admin&gt;`:
the special characters of [HTML](../debuter/glossaire.md#html) don't break the page.

## Rules to know

`//go:embed` follows a few strict rules:

| Rule | Example |
|------|---------|
| only on a **package-level** variable, outside functions | `var page string` at the top of the file |
| type `string`, `[]byte` or `embed.FS` | no `int`, no `[]string` |
| paths **relative to the package folder** (the one of the `.go` file) | `templates/a.html` |
| `/` separator, never `\` | `static/css/style.css` |
| no `..`, no absolute path | neither `../config.txt` nor `/etc/config` |
| the file must exist at compile time | otherwise: `no matching files found` |

### Hidden files and the all: prefix

When you embed a **folder**, Go ignores the files whose name starts with `.` or `_`.
This avoids embedding a `.git` folder or a scratch file by mistake. To include them,
add `all:` in front of the pattern:

```go norun
//go:embed all:static
var allStaticFiles embed.FS
```

With the `site/` project seen above, `list("allStaticFiles", allStaticFiles)`
prints:

```text
allStaticFiles:
  static/.draft.txt
  static/_notes.txt
  static/images/logo.png
  static/index.html
  static/style.css
```

A hidden file named **explicitly**, like `//go:embed static/.draft.txt`, is always
embedded.

## When not to embed

- **Large files** (videos, big databases): they bloat the binary, which becomes slow
  to compile, to copy and to load.
- **Files that change often**, or that the user must edit (their configuration, their
  data): every change would require recompiling.
- **Secrets** (passwords, keys): anyone can read them back from the binary.

A good habit: embed a **default** configuration, and replace it with a file on disk
if one exists, read with `os.ReadFile` as in [the chapter on files](fichiers.md).

## Common mistakes

### A file that doesn't exist

```go norun
//go:embed message.txt
var message string
```

```text
main.go:8:12: pattern message.txt: no matching files found
```

`message.txt` isn't in the folder of `main.go`. Check its name and location: the path
starts from the package folder, not from the folder where you run the command.

### Forgetting to import the embed package

```go norun
package main

import "fmt"

//go:embed main.go
var source string

func main() {
	fmt.Print(source)
}
```

```text
# command-line-arguments
./main.go:5:3: go:embed only allowed in Go files that import "embed"
```

Even for a simple `string`, you must import the package: add `_ "embed"`.

### //go:embed inside a function

```go norun
package main

import (
	_ "embed"
	"fmt"
)

func main() {
	//go:embed main.go
	var source string
	fmt.Print(source)
}
```

```text
# command-line-arguments
./main.go:9:4: go:embed cannot apply to var inside func
```

The file is copied at compile time: the variable must live at the package level.
Move the declaration out of `main`.

### Going up with ..

```go norun
//go:embed ../config.txt
var config string
```

```text
main.go:8:12: pattern ../config.txt: invalid pattern syntax
```

A package only embeds files from its own folder or its subfolders. Move the file, or
embed it from a package placed higher up that exports it.

## Test yourself

**Question 1:** what is the difference between `import _ "embed"` and `import "embed"`?

> **Answer**
>
> Both load the package. With `_`, you don't use it by its name: that's the case for
> a `string` or a `[]byte`. For an `embed.FS`, you write the name `embed`, so the
> normal import is enough.

**Question 2:** you wrote `//go:embed static`. Why does
`fs.ReadFile(staticFiles, "style.css")` fail?

> **Answer**
>
> The paths keep the folder name: the file is called `static/style.css`. Read this
> full path, or create a root with `fs.Sub(staticFiles, "static")`.

**Question 3:** why not embed the configuration file that the user edits every day?

> **Answer**
>
> The embedded file is frozen at the time of `go build`: every change would require
> recompiling the program. Read this file from disk with `os.ReadFile` instead, and
> only embed a default configuration.

## Summary

- `//go:embed` copies files into the binary at compile time: a single file to
  distribute.
- One file goes in a `string` or a `[]byte` (with `import _ "embed"`); several files
  or folders go in an `embed.FS`.
- `fs.ReadFile`, `fs.ReadDir`, `fs.WalkDir` and `fs.Sub` read an `embed.FS`, like any
  `fs.FS`; `fstest.MapFS` lets you try them out.
- `http.FileServerFS` serves embedded files, and `ParseFS` loads templates.
- The variable is at the package level, paths start from the package folder, with no
  `..`; `all:` includes files that start with `.` or `_`.
- Don't embed large files, files that change often, or secrets.

Next: [JSON and CSV](formats.md).
