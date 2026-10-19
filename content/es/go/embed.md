# Embeber archivos con embed

Esta página muestra cómo guardar archivos (textos, plantillas, páginas web, configuración) directamente dentro del programa compilado, con la directiva `//go:embed` y el paquete `io/fs`.

> **Objetivos:**
>
> - sabrás por qué embeber archivos en un programa Go;
> - sabrás embeber un archivo en un `string` o un `[]byte` con `//go:embed`;
> - sabrás embeber varios archivos o una carpeta en un `embed.FS`;
> - sabrás leer esos archivos con `fs.ReadFile`, `fs.ReadDir`, `fs.WalkDir` y `fs.Sub`;
> - sabrás servir archivos embebidos en la web y cargar plantillas con `ParseFS`;
> - conocerás las reglas de `//go:embed` y los casos en los que es mejor no embeber.

## ¿Por qué embeber archivos?

Un programa Go se compila en un solo [binario](../debuter/glossaire.md#binario): un
archivo ejecutable que copias a otra máquina, y funciona. Pero muchos programas
necesitan archivos al lado: plantillas de páginas, una configuración por defecto, una
hoja de estilos. Si te olvidas de copiarlos, el programa falla.

**Embeber** un archivo es meterlo *dentro* del binario, en el momento de la
compilación. Es como una maleta que ya lo contiene todo: un solo objeto que llevar. Con
un `main.go` que embebe `mensaje.txt` (el código se explica justo después):

```sh
go build -o hola .
rm mensaje.txt
./hola
```

```text
¡Hola desde un archivo embebido!
```

El archivo se ha borrado, pero su texto sigue ahí: se copió en el binario durante
`go build`.

## La directiva //go:embed sobre un string

Bastan tres cosas para embeber un archivo:

1. importar el paquete `embed`. Cuando no lo usas por su nombre, lo importas con
   `_`: `import _ "embed"`;
2. escribir el comentario especial `//go:embed nombre-del-archivo`, **sin espacio**
   entre `//` y `go:embed`;
3. declarar, justo debajo, una variable de tipo `string`.

Aquí, el programa embebe su propio código, `main.go`: este archivo existe seguro.

```go
package main

import (
	_ "embed"
	"fmt"
	"strings"
)

//go:embed main.go
var fuente string

func main() {
	lineas := strings.Split(fuente, "\n")
	fmt.Println("Primera línea:", lineas[0])
}
```

```sortie
Primera línea: package main
```

El [compilador](../debuter/glossaire.md#compilador) lee `main.go` y pone todo su
contenido en `fuente`. Ningún `os.ReadFile`, ningún error que comprobar: si falta el
archivo, es la compilación la que falla.

> **Atención:** `// go:embed` (con un espacio) no es más que un comentario normal.
> Go lo ignora, y la variable se queda vacía.

## Embeber en un []byte

Para una imagen o cualquier archivo que no sea texto, usa un slice de
[bytes](../debuter/glossaire.md#byte), `[]byte`. Es también el tipo que espera
`json.Unmarshal`, por ejemplo.

```go
package main

import (
	_ "embed"
	"fmt"
)

//go:embed main.go
var contenido []byte

func main() {
	fmt.Printf("Tipo: %T\n", contenido)
	fmt.Println("Inicio:", string(contenido[:12]))
	fmt.Println("Primer byte:", contenido[0])
}
```

```sortie
Tipo: []uint8
Inicio: package main
Primer byte: 112
```

`[]uint8` es el otro nombre de `[]byte`, y `112` es el código de la letra `p`.

> **Recuerda:** `string` para un texto, `[]byte` para bytes. Los dos reciben **un
> solo** archivo.

## Varios archivos con embed.FS

Para varios archivos, o una carpeta entera, el tipo es `embed.FS`: un pequeño
**sistema de archivos** de solo lectura, guardado en el binario. Esta vez usas el
paquete por su nombre: el import se escribe `import "embed"`, sin `_`.

```go
package main

import (
	"embed"
	"fmt"
)

//go:embed main.go
var archivos embed.FS

func main() {
	contenido, err := archivos.ReadFile("main.go")
	fmt.Println("main.go:", len(contenido) > 0, err)

	_, err = archivos.ReadFile("ausente.txt")
	fmt.Println("ausente.txt:", err)
}
```

```sortie
main.go: true <nil>
ausente.txt: open ausente.txt: file does not exist
```

`ReadFile` devuelve un error, porque el nombre se elige durante la ejecución: el
compilador no puede saber si existe.

### Patrones y carpetas

Después de `//go:embed`, puedes escribir varios nombres, **patrones** como `*.txt`, o
nombres de carpetas. Tomemos este proyecto:

```text
site/
├── go.mod
├── main.go
├── version.txt          (contiene «1.4.2»)
├── plantillas/
│   ├── adios.txt
│   └── hola.txt
└── static/
    ├── .borrador.txt
    ├── _notas.txt
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

//go:embed version.txt plantillas/*.txt
var archivos embed.FS

//go:embed static
var estaticos embed.FS

func listar(nombre string, fsys fs.FS) {
	fmt.Println(nombre + ":")
	fs.WalkDir(fsys, ".", func(ruta string, d fs.DirEntry, err error) error {
		if !d.IsDir() {
			fmt.Println("  " + ruta)
		}
		return nil
	})
}

func main() {
	version, _ := archivos.ReadFile("version.txt")
	fmt.Print("Versión: ", string(version))
	listar("archivos", archivos)
	listar("estaticos", estaticos)
}
```

```text
Versión: 1.4.2
archivos:
  plantillas/adios.txt
  plantillas/hola.txt
  version.txt
estaticos:
  static/images/logo.png
  static/index.html
  static/style.css
```

- `plantillas/*.txt` toma todos los archivos `.txt` de la carpeta `plantillas`;
- `static` toma la carpeta **y sus subcarpetas**;
- las rutas conservan su carpeta: lees `static/style.css`, no `style.css`.

Faltan `.borrador.txt` y `_notas.txt`: es una de las
[reglas que debes conocer](#las-reglas-que-debes-conocer).

## Leer un sistema de archivos con io/fs

Un `embed.FS` cumple la interfaz `fs.FS` del paquete `io/fs`: todas las funciones de
ese paquete funcionan con él. Para probarlas sin crear archivos, `testing/fstest`
ofrece `fstest.MapFS`, un sistema de archivos falso escrito como un map. El código es
el mismo que con un `embed.FS` de verdad.

### fs.ReadFile y fs.ReadDir

`fs.ReadFile` lee un archivo entero. `fs.ReadDir` lista una carpeta, ordenada por
nombre.

```go
package main

import (
	"fmt"
	"io/fs"
	"testing/fstest"
)

func main() {
	var fsys fs.FS = fstest.MapFS{
		"config.txt":     {Data: []byte("color=azul\n")},
		"docs/ayuda.txt": {Data: []byte("Escribe q para salir.\n")},
		"docs/intro.txt": {Data: []byte("¡Bienvenido!\n")},
	}

	contenido, _ := fs.ReadFile(fsys, "config.txt")
	fmt.Print("config.txt: ", string(contenido))

	entradas, _ := fs.ReadDir(fsys, "docs")
	for _, e := range entradas {
		fmt.Println("en docs:", e.Name())
	}
}
```

```sortie
config.txt: color=azul
en docs: ayuda.txt
en docs: intro.txt
```

Las rutas de `io/fs` usan siempre `/`, incluso en Windows, y la raíz se llama `.`.

### fs.WalkDir: recorrer todo el árbol

`fs.WalkDir` visita cada archivo y cada carpeta, subcarpetas incluidas, y llama a tu
función para cada uno.

```go
package main

import (
	"fmt"
	"io/fs"
	"testing/fstest"
)

func main() {
	fsys := fstest.MapFS{
		"index.html":      {Data: []byte("<h1>Inicio</h1>")},
		"css/style.css":   {Data: []byte("body {}")},
		"images/logo.png": {Data: []byte("PNG")},
	}

	fs.WalkDir(fsys, ".", func(ruta string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			fmt.Println("[carpeta]", ruta)
		} else {
			fmt.Println("         ", ruta)
		}
		return nil
	})
}
```

```sortie
[carpeta] .
[carpeta] css
          css/style.css
[carpeta] images
          images/logo.png
          index.html
```

### fs.Sub: entrar en una subcarpeta

`fs.Sub` devuelve un nuevo sistema de archivos cuya raíz es una subcarpeta. Es muy
útil con `//go:embed static`: quieres leer `style.css`, no `static/style.css`.

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

	raiz, _ := fs.Sub(fsys, "static")
	css, _ := fs.ReadFile(raiz, "style.css")
	fmt.Print(string(css))
}
```

```sortie
body { color: navy; }
```

## Servir archivos en la web

Con un `embed.FS`, un pequeño sitio web cabe en un solo binario. El servidor
[HTTP](../debuter/glossaire.md#http) de Go sirve un `fs.FS` gracias a
`http.FileServerFS` (Go 1.22 y posteriores). La carpeta `static/` contiene
`index.html` (`<h1>Bienvenido</h1>`) y `style.css`:

```go norun
package main

import (
	"embed"
	"io/fs"
	"log"
	"net/http"
)

//go:embed static
var estaticos embed.FS

func main() {
	raiz, err := fs.Sub(estaticos, "static")
	if err != nil {
		log.Fatal(err)
	}

	http.Handle("/", http.FileServerFS(raiz))

	log.Println("Servidor en http://localhost:8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
```

Mientras el servidor está en marcha, en otro terminal:

```sh
curl http://localhost:8080/
curl http://localhost:8080/style.css
```

```text
<h1>Bienvenido</h1>
body { color: navy; }
```

Sin `fs.Sub`, las direcciones serían `/static/index.html` y `/static/style.css`.

> **Nota:** antes de Go 1.22, se escribía `http.FileServer(http.FS(raiz))`:
> `http.FS` convierte un `fs.FS` en un `http.FileSystem`, el tipo que se esperaba
> antes. Lo verás a menudo en código existente. El capítulo [la web](web.md) explica
> el servidor en detalle.

## Cargar plantillas con ParseFS

Los paquetes `text/template` y `html/template` leen sus plantillas de un `fs.FS` con
`ParseFS`. Le das el sistema de archivos y uno o varios patrones. Cada plantilla toma
el nombre de su archivo.

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
		"plantillas/hola.txt":  {Data: []byte("¡Hola, {{.}}!\n")},
		"plantillas/adios.txt": {Data: []byte("Adiós, {{.}}.\n")},
	}

	plantillas, err := template.ParseFS(fsys, "plantillas/*.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	plantillas.ExecuteTemplate(os.Stdout, "hola.txt", "Lucía")
	plantillas.ExecuteTemplate(os.Stdout, "adios.txt", "Lucía")
}
```

```sortie
¡Hola, Lucía!
Adiós, Lucía.
```

En un proyecto real, embebes la carpeta `plantillas` y luego cargas las plantillas una
sola vez, al arrancar, en una variable del paquete. `template.Must` detiene el
programa si una plantilla está mal: lo ves enseguida.

```go norun
//go:embed plantillas/*.html
var dirPlantillas embed.FS

var paginas = template.Must(template.ParseFS(dirPlantillas, "plantillas/*.html"))
```

Con `html/template`, un nombre como `Lucía <admin>` se muestra `Lucía &lt;admin&gt;`:
los caracteres especiales del [HTML](../debuter/glossaire.md#html) no rompen la
página.

## Las reglas que debes conocer

`//go:embed` obedece a unas cuantas reglas estrictas:

| Regla | Ejemplo |
|-------|---------|
| solo sobre una variable **del paquete**, fuera de las funciones | `var pagina string` al principio del archivo |
| tipo `string`, `[]byte` o `embed.FS` | ni `int`, ni `[]string` |
| rutas **relativas a la carpeta del paquete** (la del archivo `.go`) | `plantillas/a.html` |
| separador `/`, nunca `\` | `static/css/style.css` |
| sin `..`, sin ruta absoluta | ni `../config.txt`, ni `/etc/config` |
| el archivo debe existir al compilar | si no: `no matching files found` |

### Los archivos ocultos y el prefijo all:

Cuando embebes una **carpeta**, Go ignora los archivos cuyo nombre empieza por `.` o
`_`. Así se evita embeber por error un `.git` o un archivo de trabajo. Para
incluirlos, añade `all:` delante del patrón:

```go norun
//go:embed all:static
var estaticosCompletos embed.FS
```

Con el proyecto `site/` visto antes, `listar("estaticosCompletos", estaticosCompletos)`
muestra:

```text
estaticosCompletos:
  static/.borrador.txt
  static/_notas.txt
  static/images/logo.png
  static/index.html
  static/style.css
```

Un archivo oculto nombrado **explícitamente**, como `//go:embed static/.borrador.txt`,
siempre se embebe.

## Cuándo no embeber

- **Los archivos grandes** (vídeos, bases de datos grandes): inflan el binario, que
  tarda más en compilarse, copiarse y cargarse.
- **Los archivos que cambian a menudo**, o que el usuario debe modificar (su
  configuración, sus datos): cada cambio obligaría a recompilar.
- **Los secretos** (contraseñas, claves): cualquiera puede volver a leerlos en el
  binario.

Una buena costumbre: embeber una configuración **por defecto**, y reemplazarla por un
archivo del disco si existe, leído con `os.ReadFile` como en
[el capítulo sobre archivos](fichiers.md).

## Errores frecuentes

### Un archivo que no existe

```go norun
//go:embed mensaje.txt
var mensaje string
```

```text
main.go:8:12: pattern mensaje.txt: no matching files found
```

`mensaje.txt` no está en la carpeta de `main.go`. Comprueba su nombre y su ubicación:
la ruta parte de la carpeta del paquete, no de la carpeta desde donde ejecutas el
comando.

### Olvidar importar el paquete embed

```go norun
package main

import "fmt"

//go:embed main.go
var fuente string

func main() {
	fmt.Print(fuente)
}
```

```text
# command-line-arguments
./main.go:5:3: go:embed only allowed in Go files that import "embed"
```

Incluso para un simple `string`, hay que importar el paquete: añade `_ "embed"`.

### //go:embed dentro de una función

```go norun
package main

import (
	_ "embed"
	"fmt"
)

func main() {
	//go:embed main.go
	var fuente string
	fmt.Print(fuente)
}
```

```text
# command-line-arguments
./main.go:9:4: go:embed cannot apply to var inside func
```

El archivo se copia al compilar: la variable debe vivir al nivel del paquete. Saca la
declaración de `main`.

### Subir con ..

```go norun
//go:embed ../config.txt
var config string
```

```text
main.go:8:12: pattern ../config.txt: invalid pattern syntax
```

Un paquete solo embebe archivos de su carpeta o de sus subcarpetas. Mueve el archivo,
o embébelo desde un paquete situado más arriba que lo exporte.

## Ponte a prueba

**Pregunta 1:** ¿qué diferencia hay entre `import _ "embed"` e `import "embed"`?

> **Respuesta**
>
> Los dos cargan el paquete. Con `_`, no lo usas por su nombre: es el caso de un
> `string` o un `[]byte`. Para un `embed.FS`, escribes el nombre `embed`, así que basta
> con el import normal.

**Pregunta 2:** has escrito `//go:embed static`. ¿Por qué falla
`fs.ReadFile(estaticos, "style.css")`?

> **Respuesta**
>
> Las rutas conservan el nombre de la carpeta: el archivo se llama `static/style.css`.
> Lee esa ruta completa, o crea una raíz con `fs.Sub(estaticos, "static")`.

**Pregunta 3:** ¿por qué no embeber el archivo de configuración que el usuario
modifica cada día?

> **Respuesta**
>
> El archivo embebido queda fijado en el momento de `go build`: cada modificación
> obligaría a recompilar el programa. Mejor lee ese archivo del disco con
> `os.ReadFile`, y embebe solo una configuración por defecto.

## En resumen

- `//go:embed` copia archivos en el binario al compilar: un solo archivo que
  distribuir.
- Un archivo va en un `string` o un `[]byte` (con `import _ "embed"`); varios archivos
  o carpetas van en un `embed.FS`.
- `fs.ReadFile`, `fs.ReadDir`, `fs.WalkDir` y `fs.Sub` leen un `embed.FS`, como
  cualquier `fs.FS`; `fstest.MapFS` permite probarlas.
- `http.FileServerFS` sirve archivos embebidos, y `ParseFS` carga plantillas.
- La variable está al nivel del paquete, las rutas parten de la carpeta del paquete,
  sin `..`; `all:` incluye los archivos que empiezan por `.` o `_`.
- No embebas ni archivos grandes, ni los que cambian a menudo, ni secretos.

Siguiente: [JSON y CSV](formats.md).
