# Generating text and HTML with templates

This page shows how to build text or web pages from a fill-in-the-blanks model, with the `text/template` and `html/template` packages from the standard library.

> **Goals:**
>
> - write a template and fill it with `Execute`;
> - show fields with `{{.Field}}`;
> - repeat with `{{range}}` and choose with `{{if}}`;
> - use template functions and add your own with `FuncMap`;
> - understand why `html/template` escapes HTML.

## A template is a fill-in-the-blanks letter

Think of a form letter from the town hall: "Hello ____, your appointment is on ____".
The text is written once. The blanks are filled in for each person.

A **template** works the same way. The fixed text is written as it is. The blanks go
between double curly braces: `{{ … }}`. They are called **actions**.

```text
  template:  "Hello {{.}}!"             data: "Léa"
                    │                           │
                    └────────── Execute ────────┘
                                   │
  result:           "Hello Léa!"
```

> **Remember:** a template = fixed text + blanks `{{ … }}`. `Execute` fills the blanks
> with your data.

## A first template

Three steps:

1. `template.New("name")` creates an empty template, with a name;
2. `.Parse(text)` reads the text of the template;
3. `.Execute(destination, data)` fills the blanks and writes the result.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	t := template.Must(template.New("hello").Parse("Hello {{.}}!\n"))
	t.Execute(os.Stdout, "Léa")
	t.Execute(os.Stdout, "Tom")
}
```

```sortie
Hello Léa!
Hello Tom!
```

- `{{.}}` (a dot on its own) means "the whole data". Here, the text `"Léa"`.
- `os.Stdout` is the screen. You could write to a file or a web page instead.
- `template.Must` stops the program if the template is badly written. You see it right
  away, at startup.

The same template is used twice: you write it once, you fill it as often as you like.

Try it: replace `"Léa"` with `42`. The template prints `Hello 42!`: `{{.}}` accepts any
value.

> **Remember:** `{{.}}` = the data you gave to `Execute`.

## Fields: {{.Field}}

Usually, you pass a [struct](../debuter/glossaire.md#structure). `{{.Name}}` reads its
`Name` field.

```go
package main

import (
	"os"
	"text/template"
)

type Appointment struct {
	Name string
	Day  string
	Hour int
}

func main() {
	tmpl := "Hello {{.Name}}, your appointment is on {{.Day}} at {{.Hour}} o'clock.\n"
	t := template.Must(template.New("appt").Parse(tmpl))

	t.Execute(os.Stdout, Appointment{Name: "Léa", Day: "Monday", Hour: 9})
	t.Execute(os.Stdout, Appointment{Name: "Tom", Day: "Tuesday", Hour: 14})
}
```

```sortie
Hello Léa, your appointment is on Monday at 9 o'clock.
Hello Tom, your appointment is on Tuesday at 14 o'clock.
```

Step by step, for Léa:

| Step | Piece of the template | What is written |
|---|---|---|
| 1 | `Hello ` | `Hello ` |
| 2 | `{{.Name}}` | `Léa` |
| 3 | `, your appointment is on ` | `, your appointment is on ` |
| 4 | `{{.Day}}` | `Monday` |
| 5 | ` at ` | ` at ` |
| 6 | `{{.Hour}}` | `9` |
| 7 | ` o'clock.` and the line break | ` o'clock.` |

Fields must start with an **uppercase letter** (be exported). The `template` package
cannot read a lowercase field (see [the mistakes](#common-mistakes)).

A [map](maps.md) works too: `{{.city}}` reads the value of the key `"city"`.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	t := template.Must(template.New("m").Parse("{{.city}}: {{.temp}} °C\n"))
	t.Execute(os.Stdout, map[string]any{"city": "Lyon", "temp": 21})
}
```

```sortie
Lyon: 21 °C
```

> **Remember:** `{{.Field}}` reads a struct field (with an uppercase letter) or a map
> key.

## Choosing with if

`{{if …}} … {{else}} … {{end}}` chooses a piece of text. The condition is true if the
value is not "empty": `false`, `0`, `""`, an empty list and `nil` are false.

```go
package main

import (
	"os"
	"text/template"
)

type Item struct {
	Name  string
	Stock int
}

func main() {
	tmpl := "{{.Name}}: {{if .Stock}}in stock ({{.Stock}}){{else}}sold out{{end}}\n"
	t := template.Must(template.New("a").Parse(tmpl))

	t.Execute(os.Stdout, Item{"pen", 12})
	t.Execute(os.Stdout, Item{"eraser", 0})
}
```

```sortie
pen: in stock (12)
eraser: sold out
```

| Item | `.Stock` | true? | Chosen piece |
|---|---|---|---|
| pen | 12 | yes | `in stock (12)` |
| eraser | 0 | no (0 is empty) | `sold out` |

### Comparing

In a template, you don't write `>` or `==`. You use **functions** placed **before** the
values: `eq` (equal), `ne` (not equal), `lt` (less than), `le`, `gt` (greater than),
`ge`.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	tmpl := "{{.}}: {{if ge . 10}}good grade{{else}}needs work{{end}}\n"
	t := template.Must(template.New("g").Parse(tmpl))
	for _, grade := range []int{15, 8, 10} {
		t.Execute(os.Stdout, grade)
	}
}
```

```sortie
15: good grade
8: needs work
10: good grade
```

`ge . 10` reads "is `.` greater than or equal to 10?".

Try it: replace `ge` with `gt`. The grade 10 becomes `needs work`, because 10 is not
strictly greater than 10.

> **Remember:** `{{if}}` … `{{else}}` … `{{end}}`, and comparisons are written
> `eq a b`, `gt a b`…

## Repeating with range

`{{range .List}} … {{end}}` repeats the piece for each element. **Inside** the loop,
the dot `.` becomes the current element.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	tmpl := "Shopping:\n{{range .}}- {{.}}\n{{end}}"
	t := template.Must(template.New("s").Parse(tmpl))
	t.Execute(os.Stdout, []string{"bread", "milk", "apples"})
}
```

```sortie
Shopping:
- bread
- milk
- apples
```

| Round | Value of `.` | What is written |
|---|---|---|
| before | the whole list | `Shopping:` |
| 1 | `"bread"` | `- bread` |
| 2 | `"milk"` | `- milk` |
| 3 | `"apples"` | `- apples` |

It's like a rubber stamp: the same drawing, printed once per element.

### The index, and the case of an empty list

`{{range $i, $x := .}}` also gives the index. Names that start with `$` are **template
variables**. `{{else}}` inside a `range` is shown when the list is empty.

```go
package main

import (
	"os"
	"text/template"
)

type Cart struct {
	Customer string
	Items    []string
}

func main() {
	tmpl := `{{.Customer}}'s cart:
{{range $i, $a := .Items}}  {{$i}}. {{$a}}
{{else}}  (empty)
{{end}}`
	t := template.Must(template.New("c").Parse(tmpl))
	t.Execute(os.Stdout, Cart{"Léa", []string{"pen", "notebook"}})
	t.Execute(os.Stdout, Cart{"Tom", nil})
}
```

```sortie
Léa's cart:
  0. pen
  1. notebook
Tom's cart:
  (empty)
```

### Removing extra spaces with {{- and -}}

A template copies **everything**, even line breaks. A dash stuck to the braces erases
the spaces and line breaks on that side: `{{-` on the left, `-}}` on the right.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	tmpl := `Numbers:
{{- range .}}
  {{.}}
{{- end}}
`
	t := template.Must(template.New("n").Parse(tmpl))
	t.Execute(os.Stdout, []int{1, 2, 3})
}
```

```sortie
Numbers:
  1
  2
  3
```

Try it: remove both dashes. Blank lines appear between the numbers.

> **Remember:** in `{{range}}`, the dot `.` is the current element. After `{{end}}`, it
> becomes the whole data again.

## Template functions

The package provides a few functions. You call them by writing the name, then the
arguments, separated by spaces (no parentheses or commas).

| Function | Example | Result |
|---|---|---|
| `len` | `{{len .Items}}` | the number of elements |
| `index` | `{{index .Grades 0}}` | the first element |
| `printf` | `{{printf "%.2f" .Price}}` | like `fmt.Sprintf` |
| `and`, `or`, `not` | `{{if and .A .B}}` | logic |

```go
package main

import (
	"os"
	"text/template"
)

type Bill struct {
	Lines []string
	Total float64
}

func main() {
	tmpl := "{{len .Lines}} lines, the first one: {{index .Lines 0}}, total {{printf \"%.2f\" .Total}} €\n"
	t := template.Must(template.New("b").Parse(tmpl))
	t.Execute(os.Stdout, Bill{[]string{"coffee", "croissant"}, 3.5})
}
```

```sortie
2 lines, the first one: coffee, total 3.50 €
```

### The pipe |

`{{.Total | printf "%.2f"}}` sends the value on the left as the **last** argument of the
function on the right. It's a pipe, like in the [terminal](../debuter/terminal.md).
`{{printf "%.2f" .Total}}` and `{{.Total | printf "%.2f"}}` give the same thing.

### Your own functions: FuncMap

A `template.FuncMap` is a "name → Go function" map. You give it to the template with
`.Funcs(...)`, **before** `Parse`.

```go
package main

import (
	"fmt"
	"os"
	"strings"
	"text/template"
)

func main() {
	functions := template.FuncMap{
		"upper": strings.ToUpper,
		"euros": func(cents int) string {
			return fmt.Sprintf("%d.%02d €", cents/100, cents%100)
		},
	}

	tmpl := "{{upper .Name}} costs {{euros .Price}}\n"
	t := template.Must(template.New("p").Funcs(functions).Parse(tmpl))

	type Product struct {
		Name  string
		Price int
	}
	t.Execute(os.Stdout, Product{"notebook", 249})
	t.Execute(os.Stdout, Product{"pen", 90})
}
```

```sortie
NOTEBOOK costs 2.49 €
PEN costs 0.90 €
```

| Product | `.Price` | `cents/100` | `cents%100` | `euros .Price` |
|---|---|---|---|---|
| notebook | 249 | 2 | 49 | `2.49 €` |
| pen | 90 | 0 | 90 | `0.90 €` |

Try it: add `"double": func(n int) int { return n * 2 }` to the `FuncMap`, then
`{{euros (double .Price)}}` in the template. The parentheses compute `double .Price`
first: the notebook costs `4.98 €`.

> **Remember:** functions are called without parentheses: `{{name arg1 arg2}}`. Yours
> are added with `.Funcs(template.FuncMap{…})` before `Parse`.

## html/template: HTML without danger

To build a web page, use `html/template` instead of `text/template`. Both are written
**exactly** the same way. The difference: `html/template` **escapes** the data.

Escaping means turning the special characters of [HTML](../debuter/glossaire.md#html)
(`<`, `>`, `"`, `&`) into harmless codes. Imagine a visitor whose name is
`<script>alert("hi")</script>`. Copied as it is into the page, this text would become a
**program** run for every reader. It's a well-known attack, the XSS flaw (see
[security](../debuter/securite.md)).

Let's compare the two packages with the same booby-trapped name:

```go
package main

import (
	htmlt "html/template"
	"os"
	textt "text/template"
)

func main() {
	name := `<script>alert("hi")</script>`
	tmpl := "<p>Hello {{.}}</p>\n"

	t1 := textt.Must(textt.New("t").Parse(tmpl))
	t1.Execute(os.Stdout, name)

	t2 := htmlt.Must(htmlt.New("h").Parse(tmpl))
	t2.Execute(os.Stdout, name)
}
```

```sortie
<p>Hello <script>alert("hi")</script></p>
<p>Hello &lt;script&gt;alert(&#34;hi&#34;)&lt;/script&gt;</p>
```

| Character | Becomes | The browser shows |
|---|---|---|
| `<` | `&lt;` | `<` (text, not a tag) |
| `>` | `&gt;` | `>` |
| `"` | `&#34;` | `"` |
| `&` | `&amp;` | `&` |

With `html/template`, the browser **shows** the name instead of running it.

### It knows the context

`html/template` knows **where** each blank is: in text, in a link, in an address. It
escapes differently depending on the place.

```go
package main

import (
	"html/template"
	"os"
)

func main() {
	tmpl := `<a href="{{.}}">link</a>` + "\n"
	t := template.Must(template.New("link").Parse(tmpl))
	t.Execute(os.Stdout, "https://go.dev/?a=1&b=2")
	t.Execute(os.Stdout, "javascript:alert(1)")
}
```

```sortie
<a href="https://go.dev/?a=1&amp;b=2">link</a>
<a href="#ZgotmplZ">link</a>
```

- The first [URL](../debuter/glossaire.md#url) is normal: only the `&` is escaped.
- The second one is dangerous (`javascript:` runs a program when you click). It is
  replaced with `#ZgotmplZ`, a code word that means "value refused".

### When the HTML is safe: template.HTML

If **you** wrote the HTML (not a visitor), say so with the `template.HTML` type. It is
then not escaped.

```go
package main

import (
	"html/template"
	"os"
)

func main() {
	t := template.Must(template.New("h").Parse("<p>{{.}}</p>\n"))
	t.Execute(os.Stdout, "<b>bold</b>")
	t.Execute(os.Stdout, template.HTML("<b>bold</b>"))
}
```

```sortie
<p>&lt;b&gt;bold&lt;/b&gt;</p>
<p><b>bold</b></p>
```

> **Warning:** never put text that comes from a visitor into `template.HTML`. You would
> turn off the protection.

> **Remember:** for HTML, always `html/template`. It escapes on its own, depending on
> the place, and that's what protects your site.

## Where do templates come from?

### From a string

That's what this page does: `Parse("…")`. Handy for small templates. A template can
contain others, named with `{{define "name"}}` and called with `{{template "name" .}}`.

```go
package main

import (
	"os"
	"text/template"
)

const templates = `
{{define "line"}}* {{.}}{{end}}
{{define "list"}}{{range .}}{{template "line" .}}
{{end}}{{end}}`

func main() {
	t := template.Must(template.New("all").Parse(templates))
	t.ExecuteTemplate(os.Stdout, "list", []string{"Go", "Rust"})
}
```

```sortie
* Go
* Rust
```

`ExecuteTemplate` chooses the template to fill by its name. It's like a binder of form
letters: you take out the one you need.

### From files

For real templates, you store them in `.html` or `.txt` files:

```go norun
t := template.Must(template.ParseFiles("templates/home.html", "templates/footer.html"))
t.ExecuteTemplate(os.Stdout, "home.html", data)
```

Each template takes the name of its file. `template.ParseGlob("templates/*.html")`
loads a whole folder.

### Embedded in the program

With `//go:embed`, the template files are stored **inside** the executable: no need to
ship them alongside. The page [embedding files](embed.md) shows `template.ParseFS` in
detail.

```go norun
//go:embed templates/*.html
var folder embed.FS

var pages = template.Must(template.ParseFS(folder, "templates/*.html"))
```

In a [web](web.md) server, you load the templates **only once**, at startup, then call
`pages.ExecuteTemplate(w, "home.html", data)` in each handler: `w` replaces
`os.Stdout`.

> **Remember:** a string for small templates, files or `embed` for real ones. You parse
> (`Parse`) once, you fill (`Execute`) often.

## Common mistakes

### A field that doesn't exist

```go norun
type Person struct{ Name string }

t := template.Must(template.New("d").Parse("{{.FirstName}}\n"))
err := t.Execute(os.Stdout, Person{Name: "Léa"})
fmt.Println(err)
```

```text
template: d:1:2: executing "d" at <.FirstName>: can't evaluate field FirstName in type main.Person
```

The error happens at `Execute`, not at `Parse`. `d:1:2` means: template `d`, line 1,
column 2. Check the field name, including uppercase letters. And always look at the
error returned by `Execute`.

### A lowercase field

```go norun
type Person struct {
	Name string
	age  int
}

t := template.Must(template.New("e").Parse("{{.age}}\n"))
fmt.Println(t.Execute(os.Stdout, Person{Name: "Léa"}))
```

```text
template: e:1:2: executing "e" at <.age>: age is an unexported field of struct type main.Person
```

The template lives in another package: it only sees exported fields. Rename the field
`Age`.

### Forgetting {{end}}

```go norun
_, err := template.New("b").Parse("{{if .Name}}yes")
fmt.Println(err)
```

```text
template: b:1: unexpected EOF
```

`EOF` means "end of file", here the end of the text. The template stops while an
`{{if}}` is still waiting for its `{{end}}`. Every `if`, `range` and `define` is closed
with `{{end}}`.

### A function added too late

```go norun
t, err := template.New("c").Parse("{{upper .Name}}")
fmt.Println(err)
```

```text
template: c:1: function "upper" not defined
```

The template checks function names during `Parse`. Call `.Funcs(...)` **before**
`.Parse(...)`.

## Test yourself

**Question 1:** what does this template print with the data `[]int{4, 5}`?

```text
{{range .}}[{{.}}]{{end}} total: {{len .}}
```

> **Answer**
>
> `[4][5] total: 2`. In `range`, `.` is 4 then 5. After `{{end}}`, `.` becomes the
> whole list again, which has 2 elements.

**Question 2:** why use `html/template` rather than `text/template` for a web page?

> **Answer**
>
> Because it escapes the data: `<script>` becomes `&lt;script&gt;` and is shown as
> text, instead of being run by the browser.

**Question 3:** how do you write "if `.Age` is less than 18" in a template?

> **Answer**
>
> `{{if lt .Age 18}} … {{end}}`. The comparison function comes first, then the two
> values.

## Summary

- A template is a text with blanks; `Parse` reads it, `Execute` fills it.
- `{{.}}` is the data, `{{.Field}}` an exported field or a map key.
- `{{if}}`/`{{else}}`/`{{end}}` chooses, `{{range}}` repeats; `eq`, `lt`, `gt`
  compare.
- Functions are called without parentheses; `FuncMap` adds yours before `Parse`.
- `html/template` escapes HTML according to the context: use it for every web page.

Next: [a database in Go](base-de-donnees.md).
