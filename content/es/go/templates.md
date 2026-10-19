# Generar texto y HTML con las plantillas

Esta página muestra cómo fabricar texto o páginas web a partir de un modelo con huecos, con los paquetes `text/template` y `html/template` de la biblioteca estándar.

> **Objetivos:**
>
> - escribir una plantilla y rellenarla con `Execute`;
> - mostrar campos con `{{.Campo}}`;
> - repetir con `{{range}}` y elegir con `{{if}}`;
> - usar las funciones de las plantillas y añadir las tuyas con `FuncMap`;
> - entender por qué `html/template` escapa el HTML.

## Una plantilla es una carta con huecos

Piensa en una carta tipo del ayuntamiento: «Hola ____, tu cita es el ____». El texto se
escribe una vez. Los huecos se rellenan para cada persona.

Una **plantilla** (en inglés *template*) funciona igual. El texto fijo se escribe tal
cual. Los huecos van entre llaves dobles: `{{ … }}`. Se llaman **acciones**.

```text
  plantilla:  "Hola {{.}}!"             datos: "Léa"
                    │                            │
                    └────────── Execute ─────────┘
                                   │
  resultado:        "Hola Léa!"
```

> **Recuerda:** una plantilla = texto fijo + huecos `{{ … }}`. `Execute` rellena los
> huecos con tus datos.

## Una primera plantilla

Tres pasos:

1. `template.New("nombre")` crea una plantilla vacía, con un nombre;
2. `.Parse(texto)` lee el texto de la plantilla;
3. `.Execute(destino, datos)` rellena los huecos y escribe el resultado.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	t := template.Must(template.New("saludo").Parse("¡Hola {{.}}!\n"))
	t.Execute(os.Stdout, "Léa")
	t.Execute(os.Stdout, "Tom")
}
```

```sortie
¡Hola Léa!
¡Hola Tom!
```

- `{{.}}` (un punto solo) quiere decir «el dato entero». Aquí, el texto `"Léa"`.
- `os.Stdout` es la pantalla. Podrías escribir en un archivo o en una página web en su
  lugar.
- `template.Must` detiene el programa si la plantilla está mal escrita. Lo ves enseguida,
  al arrancar.

La misma plantilla sirve dos veces: la escribes una vez y la rellenas tantas veces como
quieras.

Prueba: cambia `"Léa"` por `42`. La plantilla muestra `¡Hola 42!`: `{{.}}` acepta
cualquier valor.

> **Recuerda:** `{{.}}` = el dato que le diste a `Execute`.

## Los campos: {{.Campo}}

En general, se pasa una [estructura](../debuter/glossaire.md#structure). `{{.Nombre}}`
lee su campo `Nombre`.

```go
package main

import (
	"os"
	"text/template"
)

type Cita struct {
	Nombre string
	Dia    string
	Hora   int
}

func main() {
	plantilla := "Hola {{.Nombre}}, tu cita es el {{.Dia}} a las {{.Hora}} h.\n"
	t := template.Must(template.New("cita").Parse(plantilla))

	t.Execute(os.Stdout, Cita{Nombre: "Léa", Dia: "lunes", Hora: 9})
	t.Execute(os.Stdout, Cita{Nombre: "Tom", Dia: "martes", Hora: 14})
}
```

```sortie
Hola Léa, tu cita es el lunes a las 9 h.
Hola Tom, tu cita es el martes a las 14 h.
```

Paso a paso, para Léa:

| Paso | Trozo de la plantilla | Lo que se escribe |
|---|---|---|
| 1 | `Hola ` | `Hola ` |
| 2 | `{{.Nombre}}` | `Léa` |
| 3 | `, tu cita es el ` | `, tu cita es el ` |
| 4 | `{{.Dia}}` | `lunes` |
| 5 | ` a las ` | ` a las ` |
| 6 | `{{.Hora}}` | `9` |
| 7 | ` h.` y el salto de línea | ` h.` |

Los campos deben empezar por **mayúscula** (estar exportados). El paquete `template` no
puede leer un campo en minúscula (mira [los errores](#errores-frecuentes)).

Un [map](maps.md) también funciona: `{{.ciudad}}` lee el valor de la clave `"ciudad"`.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	t := template.Must(template.New("m").Parse("{{.ciudad}}: {{.temp}} °C\n"))
	t.Execute(os.Stdout, map[string]any{"ciudad": "Lyon", "temp": 21})
}
```

```sortie
Lyon: 21 °C
```

> **Recuerda:** `{{.Campo}}` lee un campo de estructura (con mayúscula) o una clave de
> map.

## Elegir con if

`{{if …}} … {{else}} … {{end}}` elige un trozo de texto. La condición es verdadera si el
valor no está «vacío»: `false`, `0`, `""`, una lista vacía y `nil` son falsos.

```go
package main

import (
	"os"
	"text/template"
)

type Articulo struct {
	Nombre string
	Stock  int
}

func main() {
	plantilla := "{{.Nombre}}: {{if .Stock}}en stock ({{.Stock}}){{else}}agotado{{end}}\n"
	t := template.Must(template.New("a").Parse(plantilla))

	t.Execute(os.Stdout, Articulo{"bolígrafo", 12})
	t.Execute(os.Stdout, Articulo{"goma", 0})
}
```

```sortie
bolígrafo: en stock (12)
goma: agotado
```

| Artículo | `.Stock` | ¿verdadero? | Trozo elegido |
|---|---|---|---|
| bolígrafo | 12 | sí | `en stock (12)` |
| goma | 0 | no (0 está vacío) | `agotado` |

### Comparar

En una plantilla no se escribe `>` ni `==`. Se usan **funciones** colocadas **antes** de
los valores: `eq` (igual), `ne` (distinto), `lt` (menor), `le`, `gt` (mayor), `ge`.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	plantilla := "{{.}}: {{if ge . 10}}nota correcta{{else}}hay que repasar{{end}}\n"
	t := template.Must(template.New("n").Parse(plantilla))
	for _, nota := range []int{15, 8, 10} {
		t.Execute(os.Stdout, nota)
	}
}
```

```sortie
15: nota correcta
8: hay que repasar
10: nota correcta
```

`ge . 10` se lee «¿es `.` mayor o igual que 10?».

Prueba: cambia `ge` por `gt`. La nota 10 pasa a `hay que repasar`, porque 10 no es
estrictamente mayor que 10.

> **Recuerda:** `{{if}}` … `{{else}}` … `{{end}}`, y las comparaciones se escriben
> `eq a b`, `gt a b`…

## Repetir con range

`{{range .Lista}} … {{end}}` repite el trozo para cada elemento. **Dentro** del bucle,
el punto `.` pasa a ser el elemento actual.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	plantilla := "Compras:\n{{range .}}- {{.}}\n{{end}}"
	t := template.Must(template.New("c").Parse(plantilla))
	t.Execute(os.Stdout, []string{"pan", "leche", "manzanas"})
}
```

```sortie
Compras:
- pan
- leche
- manzanas
```

| Vuelta | Valor de `.` | Lo que se escribe |
|---|---|---|
| antes | la lista entera | `Compras:` |
| 1 | `"pan"` | `- pan` |
| 2 | `"leche"` | `- leche` |
| 3 | `"manzanas"` | `- manzanas` |

Es como un sello de tinta: el mismo dibujo, impreso una vez por elemento.

### El índice, y el caso de una lista vacía

`{{range $i, $x := .}}` da también el índice. Los nombres que empiezan por `$` son
**variables de la plantilla**. `{{else}}` dentro de un `range` se muestra cuando la
lista está vacía.

```go
package main

import (
	"os"
	"text/template"
)

type Cesta struct {
	Cliente   string
	Articulos []string
}

func main() {
	plantilla := `Cesta de {{.Cliente}}:
{{range $i, $a := .Articulos}}  {{$i}}. {{$a}}
{{else}}  (vacía)
{{end}}`
	t := template.Must(template.New("c").Parse(plantilla))
	t.Execute(os.Stdout, Cesta{"Léa", []string{"bolígrafo", "cuaderno"}})
	t.Execute(os.Stdout, Cesta{"Tom", nil})
}
```

```sortie
Cesta de Léa:
  0. bolígrafo
  1. cuaderno
Cesta de Tom:
  (vacía)
```

### Quitar los espacios de más con {{- y -}}

Una plantilla copia **todo**, incluso los saltos de línea. Un guion pegado a las llaves
borra los espacios y saltos de línea de ese lado: `{{-` a la izquierda, `-}}` a la
derecha.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	plantilla := `Números:
{{- range .}}
  {{.}}
{{- end}}
`
	t := template.Must(template.New("n").Parse(plantilla))
	t.Execute(os.Stdout, []int{1, 2, 3})
}
```

```sortie
Números:
  1
  2
  3
```

Prueba: quita los dos guiones. Aparecen líneas vacías entre los números.

> **Recuerda:** en `{{range}}`, el punto `.` es el elemento actual. Después de
> `{{end}}`, vuelve a ser el dato entero.

## Las funciones de las plantillas

El paquete trae algunas funciones. Se llaman escribiendo el nombre y luego los
argumentos, separados por espacios (sin paréntesis ni comas).

| Función | Ejemplo | Resultado |
|---|---|---|
| `len` | `{{len .Articulos}}` | el número de elementos |
| `index` | `{{index .Notas 0}}` | el primer elemento |
| `printf` | `{{printf "%.2f" .Precio}}` | como `fmt.Sprintf` |
| `and`, `or`, `not` | `{{if and .A .B}}` | lógica |

```go
package main

import (
	"os"
	"text/template"
)

type Factura struct {
	Lineas []string
	Total  float64
}

func main() {
	plantilla := "{{len .Lineas}} líneas, la primera: {{index .Lineas 0}}, total {{printf \"%.2f\" .Total}} €\n"
	t := template.Must(template.New("f").Parse(plantilla))
	t.Execute(os.Stdout, Factura{[]string{"café", "cruasán"}, 3.5})
}
```

```sortie
2 líneas, la primera: café, total 3.50 €
```

### La tubería |

`{{.Total | printf "%.2f"}}` envía el valor de la izquierda como **último** argumento de
la función de la derecha. Es una tubería, como en la [terminal](../debuter/terminal.md).
`{{printf "%.2f" .Total}}` y `{{.Total | printf "%.2f"}}` dan lo mismo.

### Tus propias funciones: FuncMap

Una `template.FuncMap` es un map «nombre → función Go». Se la das a la plantilla con
`.Funcs(...)`, **antes** de `Parse`.

```go
package main

import (
	"fmt"
	"os"
	"strings"
	"text/template"
)

func main() {
	funciones := template.FuncMap{
		"mayusculas": strings.ToUpper,
		"euros": func(centimos int) string {
			return fmt.Sprintf("%d,%02d €", centimos/100, centimos%100)
		},
	}

	plantilla := "{{mayusculas .Nombre}} cuesta {{euros .Precio}}\n"
	t := template.Must(template.New("p").Funcs(funciones).Parse(plantilla))

	type Producto struct {
		Nombre string
		Precio int
	}
	t.Execute(os.Stdout, Producto{"cuaderno", 249})
	t.Execute(os.Stdout, Producto{"bolígrafo", 90})
}
```

```sortie
CUADERNO cuesta 2,49 €
BOLÍGRAFO cuesta 0,90 €
```

| Producto | `.Precio` | `centimos/100` | `centimos%100` | `euros .Precio` |
|---|---|---|---|---|
| cuaderno | 249 | 2 | 49 | `2,49 €` |
| bolígrafo | 90 | 0 | 90 | `0,90 €` |

Prueba: añade `"doble": func(n int) int { return n * 2 }` en la `FuncMap`, y luego
`{{euros (doble .Precio)}}` en la plantilla. Los paréntesis calculan primero
`doble .Precio`: el cuaderno cuesta `4,98 €`.

> **Recuerda:** las funciones se llaman sin paréntesis: `{{nombre arg1 arg2}}`. Las
> tuyas se añaden con `.Funcs(template.FuncMap{…})` antes de `Parse`.

## html/template: el HTML sin peligro

Para fabricar una página web, usa `html/template` en lugar de `text/template`. Los dos
se escriben **exactamente** igual. La diferencia: `html/template` **escapa** los datos.

Escapar es transformar los caracteres especiales del [HTML](../debuter/glossaire.md#html)
(`<`, `>`, `"`, `&`) en códigos inofensivos. Imagina un visitante que se llama
`<script>alert("hola")</script>`. Copiado tal cual en la página, ese texto se
convertiría en un **programa** ejecutado en el navegador de todos los lectores. Es un
ataque conocido, la vulnerabilidad XSS (mira [la seguridad](../debuter/securite.md)).

Comparemos los dos paquetes con el mismo nombre trampa:

```go
package main

import (
	htmlt "html/template"
	"os"
	textt "text/template"
)

func main() {
	nombre := `<script>alert("hola")</script>`
	plantilla := "<p>Hola {{.}}</p>\n"

	t1 := textt.Must(textt.New("t").Parse(plantilla))
	t1.Execute(os.Stdout, nombre)

	t2 := htmlt.Must(htmlt.New("h").Parse(plantilla))
	t2.Execute(os.Stdout, nombre)
}
```

```sortie
<p>Hola <script>alert("hola")</script></p>
<p>Hola &lt;script&gt;alert(&#34;hola&#34;)&lt;/script&gt;</p>
```

| Carácter | Se convierte en | El navegador muestra |
|---|---|---|
| `<` | `&lt;` | `<` (texto, no una etiqueta) |
| `>` | `&gt;` | `>` |
| `"` | `&#34;` | `"` |
| `&` | `&amp;` | `&` |

Con `html/template`, el navegador **muestra** el nombre en lugar de ejecutarlo.

### Conoce el contexto

`html/template` sabe **dónde** está cada hueco: en texto, en un enlace, en una
dirección. Escapa de forma distinta según el sitio.

```go
package main

import (
	"html/template"
	"os"
)

func main() {
	plantilla := `<a href="{{.}}">enlace</a>` + "\n"
	t := template.Must(template.New("enlace").Parse(plantilla))
	t.Execute(os.Stdout, "https://go.dev/?a=1&b=2")
	t.Execute(os.Stdout, "javascript:alert(1)")
}
```

```sortie
<a href="https://go.dev/?a=1&amp;b=2">enlace</a>
<a href="#ZgotmplZ">enlace</a>
```

- La primera [URL](../debuter/glossaire.md#url) es normal: solo se escapa el `&`.
- La segunda es peligrosa (`javascript:` lanza un programa al hacer clic). Se sustituye
  por `#ZgotmplZ`, una palabra en clave que quiere decir «valor rechazado».

### Cuando el HTML es seguro: template.HTML

Si **tú** escribiste el HTML (no un visitante), dilo con el tipo `template.HTML`.
Entonces no se escapa.

```go
package main

import (
	"html/template"
	"os"
)

func main() {
	t := template.Must(template.New("h").Parse("<p>{{.}}</p>\n"))
	t.Execute(os.Stdout, "<b>negrita</b>")
	t.Execute(os.Stdout, template.HTML("<b>negrita</b>"))
}
```

```sortie
<p>&lt;b&gt;negrita&lt;/b&gt;</p>
<p><b>negrita</b></p>
```

> **Atención:** nunca pongas un texto que venga de un visitante en `template.HTML`.
> Desactivarías la protección.

> **Recuerda:** para HTML, siempre `html/template`. Escapa solo, según el sitio, y eso
> es lo que protege tu web.

## ¿De dónde vienen las plantillas?

### De una cadena

Es lo que hace esta página: `Parse("…")`. Práctico para plantillas pequeñas. Una
plantilla puede contener otras, con nombre gracias a `{{define "nombre"}}` y llamadas
con `{{template "nombre" .}}`.

```go
package main

import (
	"os"
	"text/template"
)

const plantillas = `
{{define "linea"}}* {{.}}{{end}}
{{define "lista"}}{{range .}}{{template "linea" .}}
{{end}}{{end}}`

func main() {
	t := template.Must(template.New("todo").Parse(plantillas))
	t.ExecuteTemplate(os.Stdout, "lista", []string{"Go", "Rust"})
}
```

```sortie
* Go
* Rust
```

`ExecuteTemplate` elige la plantilla que se rellena por su nombre. Es como un archivador
de cartas tipo: sacas la que necesitas.

### De archivos

Para plantillas de verdad, se guardan en archivos `.html` o `.txt`:

```go norun
t := template.Must(template.ParseFiles("plantillas/inicio.html", "plantillas/pie.html"))
t.ExecuteTemplate(os.Stdout, "inicio.html", datos)
```

Cada plantilla toma el nombre de su archivo. `template.ParseGlob("plantillas/*.html")`
carga una carpeta entera.

### Incrustadas en el programa

Con `//go:embed`, los archivos de plantillas se guardan **dentro** del ejecutable: ya no
hace falta entregarlos al lado. La página [incrustar archivos](embed.md) muestra
`template.ParseFS` en detalle.

```go norun
//go:embed plantillas/*.html
var carpeta embed.FS

var paginas = template.Must(template.ParseFS(carpeta, "plantillas/*.html"))
```

En un servidor [web](web.md), las plantillas se cargan **una sola vez**, al arrancar, y
luego se llama a `paginas.ExecuteTemplate(w, "inicio.html", datos)` en cada manejador:
`w` sustituye a `os.Stdout`.

> **Recuerda:** una cadena para las plantillas pequeñas, archivos o `embed` para las de
> verdad. Se analiza (`Parse`) una vez, se rellena (`Execute`) a menudo.

## Errores frecuentes

### Un campo que no existe

```go norun
type Persona struct{ Nombre string }

t := template.Must(template.New("d").Parse("{{.Apellido}}\n"))
err := t.Execute(os.Stdout, Persona{Nombre: "Léa"})
fmt.Println(err)
```

```text
template: d:1:2: executing "d" at <.Apellido>: can't evaluate field Apellido in type main.Persona
```

El error llega en el `Execute`, no en el `Parse`. `d:1:2` quiere decir: plantilla `d`,
línea 1, columna 2. Comprueba el nombre del campo, mayúsculas incluidas. Y mira siempre
el error que devuelve `Execute`.

### Un campo en minúscula

```go norun
type Persona struct {
	Nombre string
	edad   int
}

t := template.Must(template.New("e").Parse("{{.edad}}\n"))
fmt.Println(t.Execute(os.Stdout, Persona{Nombre: "Léa"}))
```

```text
template: e:1:2: executing "e" at <.edad>: edad is an unexported field of struct type main.Persona
```

La plantilla vive en otro paquete: solo ve los campos exportados. Cambia el nombre del
campo a `Edad`.

### Olvidar {{end}}

```go norun
_, err := template.New("b").Parse("{{if .Nombre}}sí")
fmt.Println(err)
```

```text
template: b:1: unexpected EOF
```

`EOF` quiere decir «fin del texto» (*end of file*). La plantilla se acaba cuando un
`{{if}}` todavía espera su `{{end}}`. Cada `if`, `range` y `define` se cierra con
`{{end}}`.

### Una función añadida demasiado tarde

```go norun
t, err := template.New("c").Parse("{{mayusculas .Nombre}}")
fmt.Println(err)
```

```text
template: c:1: function "mayusculas" not defined
```

La plantilla comprueba los nombres de funciones durante `Parse`. Llama a `.Funcs(...)`
**antes** de `.Parse(...)`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra esta plantilla con el dato `[]int{4, 5}`?

```text
{{range .}}[{{.}}]{{end}} total: {{len .}}
```

> **Respuesta**
>
> `[4][5] total: 2`. En `range`, `.` vale 4 y luego 5. Después de `{{end}}`, `.` vuelve
> a ser la lista entera, que tiene 2 elementos.

**Pregunta 2:** ¿por qué usar `html/template` en lugar de `text/template` para una
página web?

> **Respuesta**
>
> Porque escapa los datos: `<script>` se convierte en `&lt;script&gt;` y se muestra como
> texto, en lugar de que el navegador lo ejecute.

**Pregunta 3:** ¿cómo se escribe «si `.Edad` es menor que 18» en una plantilla?

> **Respuesta**
>
> `{{if lt .Edad 18}} … {{end}}`. La función de comparación va primero, y luego los dos
> valores.

## En resumen

- Una plantilla es un texto con huecos; `Parse` lo lee, `Execute` lo rellena.
- `{{.}}` es el dato, `{{.Campo}}` un campo exportado o una clave de map.
- `{{if}}`/`{{else}}`/`{{end}}` elige, `{{range}}` repite; `eq`, `lt`, `gt` comparan.
- Las funciones se llaman sin paréntesis; `FuncMap` añade las tuyas antes de `Parse`.
- `html/template` escapa el HTML según el contexto: úsalo para toda página web.

Siguiente: [una base de datos en Go](base-de-donnees.md).
