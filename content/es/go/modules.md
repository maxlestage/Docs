# Paquetes y módulos

Esta página explica cómo Go organiza el código en paquetes, cómo crear un módulo con `go mod init` y cómo usar código escrito por otras personas.

> **Objetivos:**
>
> - sabrás importar un paquete y usar sus nombres exportados;
> - sabrás crear un módulo con `go mod init` y leer el archivo `go.mod`;
> - sabrás repartir un proyecto en varios archivos y carpetas;
> - sabrás proteger código con una carpeta `internal`;
> - sabrás añadir un módulo externo y leer la documentación con `go doc`.

## Los paquetes

Un **paquete** (*package*) es una carpeta de archivos Go que trabajan juntos. Cada
archivo empieza con `package nombre`. El paquete `main` es especial: es el de un
[programa](../debuter/glossaire.md#programa), y contiene la función `main`.

Piensa en una biblioteca municipal: cada estantería (paquete) reúne libros sobre un
mismo tema. No te llevas todo el edificio a casa, tomas prestada la estantería que
necesitas.

`import` pone un paquete a tu disposición. Después, escribes el nombre del paquete, un
punto y luego la función: `strings.ToUpper`.

```go
package main

import (
	"fmt"
	"math"
	"strings"
)

func main() {
	fmt.Println(strings.ToUpper("hola"))
	fmt.Println(math.Sqrt(16))
}
```

```sortie
HOLA
4
```

### La ruta y el nombre

El texto entre comillas es la **ruta** del paquete. El nombre que se usa en el código
es la última parte de la ruta: con `"math/rand"`, escribes `rand.Intn`. Si dos paquetes
tienen el mismo nombre, dale un **alias** a uno de ellos.

```go
package main

import (
	"fmt"
	str "strings"
	"unicode/utf8"
)

func main() {
	palabra := "canción"
	fmt.Println(str.Repeat("-", 5))
	fmt.Println(utf8.RuneCountInString(palabra), "letras")
}
```

```sortie
-----
7 letras
```

## Los nombres exportados

En Go, la **mayúscula** decide qué es visible fuera del paquete:

- un nombre que empieza por mayúscula (`ToUpper`, `Pi`, `Jugador`) está **exportado**:
  los demás paquetes pueden usarlo;
- un nombre en minúscula (`mayuscula`, `contador`) sigue siendo **privado** del paquete.

No hay palabra clave `public` ni `private`: basta con la primera letra. Esto vale para
las funciones, los tipos, las variables, las constantes e incluso los campos de las
[estructuras](types.md).

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

## Crear un módulo

Un **módulo** es un proyecto entero: uno o varios paquetes, con un nombre y una lista de
dependencias. `go mod init` lo crea. Le das una **ruta de módulo**, a menudo la
dirección del repositorio donde se publicará (`github.com/tu/proyecto`). Para un
proyecto personal, cualquier nombre sirve.

```sh
mkdir hola
cd hola
go mod init ejemplo.com/hola
```

```text
go: creating new go.mod: module ejemplo.com/hola
```

El comando crea un archivo `go.mod`:

```text
module ejemplo.com/hola

go 1.24.7
```

- `module` da la ruta del módulo: los imports de tus paquetes empezarán por ella.
- `go` indica la versión de Go utilizada.

Escribe un `main.go` al lado y luego ejecuta `go run .`: el punto quiere decir «el
paquete de la carpeta actual».

## Un proyecto en varios archivos

En una misma carpeta, todos los archivos pertenecen al mismo paquete: comparten sus
funciones y sus tipos sin importar nada. Para un paquete nuevo, crea una
**subcarpeta**.

Aquí tienes un proyecto con un paquete `saludo`:

```text
hola/
├── go.mod
├── main.go
└── saludo/
    └── saludo.go
```

El archivo `saludo/saludo.go`:

```go norun
// Package saludo fabrica mensajes de bienvenida.
package saludo

// Hola devuelve un mensaje de bienvenida para nombre.
func Hola(nombre string) string {
	return "¡Hola, " + mayuscula(nombre) + "!"
}

func mayuscula(s string) string {
	if s == "" {
		return s
	}
	return string(s[0]-'a'+'A') + s[1:]
}
```

El archivo `main.go` importa el paquete con la ruta del módulo seguida de la carpeta:

```go norun
package main

import (
	"fmt"

	"ejemplo.com/hola/saludo"
)

func main() {
	fmt.Println(saludo.Hola("alice"))
}
```

```sh
go run .
```

```text
¡Hola, Alice!
```

`main.go` puede llamar a `saludo.Hola` (mayúscula), pero no a `saludo.mayuscula`
(minúscula): es un detalle interno del paquete.

> **Consejo:** un comentario justo encima de `package` o de un nombre exportado se
> convierte en su documentación. Empiézalo con el nombre que describe: «Hola
> devuelve…».

## Los paquetes internal

Una carpeta llamada `internal` está protegida: solo el código **de la carpeta padre y de
sus subcarpetas** puede importar lo que contiene. Es una sala «solo para el personal».

```text
hola/
├── go.mod
├── main.go
└── saludo/
    ├── saludo.go
    └── internal/
        └── herramienta/
            └── herramienta.go
```

Aquí, `saludo` puede importar `ejemplo.com/hola/saludo/internal/herramienta`. Pero
`main.go`, que está **fuera** de `saludo/`, no puede:

```text
package ejemplo.com/hola
	main.go:6:2: use of internal package ejemplo.com/hola/saludo/internal/herramienta not allowed
```

Así puedes reorganizar este código más adelante sin romper los programas de los demás.

## Añadir un módulo externo

Hay miles de módulos publicados en internet. `go get` descarga uno y lo añade a
`go.mod`. Por ejemplo, el módulo `uuid` genera identificadores únicos:

```sh
go get github.com/google/uuid
```

```text
go: downloading github.com/google/uuid v1.6.0
go: added github.com/google/uuid v1.6.0
```

El archivo `go.mod` gana una línea `require`, y aparece un archivo `go.sum`: contiene
las huellas de los archivos descargados, para comprobar que nadie los ha modificado.

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

Después de añadir o quitar imports, `go mod tidy` ordena `go.mod`: añade lo que falta y
quita lo que ya no se usa.

> **Nota:** prefiere la biblioteca estándar cuando basta. Es muy rica (texto, archivos,
> JSON, web, tests…) y no añade ninguna dependencia.

## Leer la documentación con go doc

`go doc` muestra la documentación de un paquete o de una función, directamente en la
[terminal](../debuter/terminal.md).

```sh
go doc strings.ToUpper
```

```text
package strings // import "strings"

func ToUpper(s string) string
    ToUpper returns s with all Unicode letters mapped to their upper case.
```

También funciona con tus propios paquetes, gracias a los comentarios:

```sh
go doc ./saludo
```

```text
package saludo // import "ejemplo.com/hola/saludo"

Package saludo fabrica mensajes de bienvenida.

func Hola(nombre string) string
```

Solo aparecen los nombres exportados. La misma documentación está en internet, en
`pkg.go.dev`.

## Errores frecuentes

### Un import sin usar

```go norun
package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Println("Hola")
}
```

```text
# command-line-arguments
./main.go:5:2: "os" imported and not used
```

Go rechaza un import que no se usa. Borra la línea `"os"`. Muchos editores lo hacen
solos al guardar (con la herramienta `goimports`).

### Llamar a un nombre no exportado

```go norun
fmt.Println(saludo.mayuscula("alice"))
```

```text
# ejemplo.com/hola
./main.go:10:21: name mayuscula not exported by package saludo
```

`mayuscula` empieza por minúscula: es invisible fuera de su paquete. Usa una función
exportada, o cámbiale el nombre a `Mayuscula` si de verdad tiene que ser pública.

### Ejecutar go run . sin módulo

```sh
go run .
```

```text
go: go.mod file not found in current directory or any parent directory; see 'go help modules'
```

La carpeta no tiene `go.mod`. Ejecuta primero `go mod init ejemplo.com/proyecto`, o
colócate en la carpeta del módulo.

## Ponte a prueba

**Pregunta 1:** en un paquete, ¿qué diferencia hay entre `Calcular` y
`calcular`?

> **Respuesta**
>
> `Calcular` empieza por mayúscula: está exportado, los demás paquetes pueden
> llamarlo. `calcular` es privado de su paquete.

**Pregunta 2:** tu módulo se llama `ejemplo.com/juego` y contiene una carpeta
`puntos/`. ¿Qué línea de import escribes en `main.go`?

> **Respuesta**
>
> `import "ejemplo.com/juego/puntos"`, y luego usas `puntos.Algo`.

**Pregunta 3:** ¿para qué sirve `go mod tidy`?

> **Respuesta**
>
> Pone en orden `go.mod` y `go.sum`: añade los módulos importados que faltan y quita
> los que ya no se usan.

## En resumen

- Un paquete es una carpeta; `import` lo pone a tu disposición, y escribes
  `paquete.Nombre`.
- Una mayúscula al principio de un nombre lo exporta; una minúscula lo mantiene
  privado.
- `go mod init ruta` crea un módulo y su archivo `go.mod`.
- Una subcarpeta es un paquete nuevo; `internal/` limita quién puede importarlo.
- `go get` añade un módulo externo; `go doc` muestra la documentación.

Siguiente: [las goroutines](concurrence.md). Compara con
[los módulos en Rust](../rust/modules.md).
