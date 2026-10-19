# Go: introducción e instalación

Esta página presenta Go, explica cómo instalarlo y te guía para escribir, ejecutar y compilar tu primer programa.

> **Objetivos:**
>
> - sabrás explicar qué es Go y para qué sirve;
> - sabrás instalar Go y comprobar que funciona con `go version`;
> - sabrás escribir un programa con `package main`, `import` y `func main`;
> - sabrás ejecutar un programa con `go run` y compilarlo con `go build`;
> - sabrás dar formato a tu código con `gofmt`.

## ¿Qué es Go?

Go es un lenguaje de programación creado en Google en 2009, por Robert Griesemer, Rob
Pike y Ken Thompson. También se le llama «Golang», para encontrarlo más fácilmente
en un buscador.

Un **lenguaje de programación** es un idioma preciso, que el ordenador sabe seguir.
Un **programa** es un texto escrito en ese idioma: una serie de órdenes, como una
receta de cocina.

```go
package main

import "fmt"

func main() {
	fmt.Println("¡Hola desde Go!")
}
```

```sortie
¡Hola desde Go!
```

Estas pocas líneas son un programa completo. Las explicaremos una a una un poco
más abajo.

## ¿Por qué aprender Go?

Go se pensó para ser **sencillo**. El lenguaje tiene pocas palabras clave: lo
recorres rápido.

- **Es rápido.** Go es un lenguaje [compilado](../debuter/glossaire.md#compilacion):
  antes de ejecutarse, tu código se traduce a lenguaje máquina. El programa obtenido
  se ejecuta muy rápido.
- **Es fácil de leer.** A menudo hay una sola buena forma de escribir una cosa.
  El código de otra persona se parece al tuyo.
- **Produce un solo archivo.** El resultado de la compilación es un
  [binario](../debuter/glossaire.md#binario): un archivo que copias a otra
  máquina, y que funciona sin instalar nada.
- **Sabe hacer varias cosas a la vez.** Go se diseñó para la
  [concurrencia](../debuter/glossaire.md#concurrencia): lanzar miles de pequeñas
  tareas al mismo tiempo es fácil.

Go se usa mucho para los **servidores** (los programas que responden a los sitios web
y a las aplicaciones) y para las **herramientas** de línea de comandos. Docker, Kubernetes o
Hugo están escritos en Go.

> **Nota:** este sitio también presenta otros lenguajes: compara con
> [la introducción a Ruby](../ruby/introduction.md) o
> [la introducción a Rust](../rust/introduction.md).

### ¿Compilado o interpretado?

Con un lenguaje **interpretado** (como Ruby o Python), un programa lee tu archivo y
ejecuta las órdenes una a una, cada vez que lo lanzas.

Con un lenguaje **compilado** como Go, un
[compilador](../debuter/glossaire.md#compilador) lee primero todo tu archivo.
Comprueba que no haya errores y luego lo traduce a lenguaje máquina. Es como
traducir un libro entero antes de darlo a leer, en lugar de traducirlo en voz alta
página a página.

> **Recuerda:** el compilador de Go es muy estricto. Rechaza una variable o un
> import sin usar. A veces molesta al principio, pero tu código se mantiene limpio.

## Instalar Go

Este sitio usa **Go 1.24**. Una versión más reciente también funciona.

Todas las versiones se descargan del sitio oficial: **go.dev/dl**.

### En Windows

Descarga el archivo `.msi` para Windows en go.dev/dl y ejecútalo. Deja las
opciones que vienen por defecto. El instalador añade Go a la terminal él solo.

### En macOS

Descarga el archivo `.pkg` en go.dev/dl (elige «Apple» o «ARM64» para un
Mac reciente) y ejecútalo. También puedes usar Homebrew:

```sh
brew install go
```

### En Linux

El gestor de paquetes de tu sistema suele ofrecer Go, pero su versión puede
ser antigua. Lo más seguro es el archivo oficial de go.dev/dl:

```sh
sudo rm -rf /usr/local/go
sudo tar -C /usr/local -xzf go1.24.7.linux-amd64.tar.gz
echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.profile
```

- La primera línea borra una posible versión antigua.
- La segunda descomprime Go en `/usr/local/go`.
- La tercera añade la carpeta `bin` de Go al `PATH`: la lista de carpetas donde la
  terminal busca los comandos.

Cierra y vuelve a abrir tu sesión (o escribe `source ~/.profile`) para que el cambio
surta efecto.

> **Consejo:** en Ubuntu, `sudo snap install go --classic` también instala una versión
> reciente.

## Comprobar la instalación

Abre una terminal y escribe el comando siguiente. Si la terminal es nueva para ti,
lee primero [la terminal](../debuter/terminal.md).

```sh
go version
```

Debes ver un número de versión, por ejemplo:

```text
go version go1.24.7 linux/amd64
```

El final indica tu sistema (`linux`, `darwin` para macOS, `windows`) y el tipo de
procesador (`amd64`, `arm64`).

Si la terminal responde «command not found» (comando no encontrado), Go no está
instalado, o la terminal se abrió antes de la instalación: ciérrala y vuelve a abrirla.

## Tu primer programa

Abre tu [editor de código](../debuter/editeur.md). Crea una carpeta `hola`, y
dentro un archivo `hola.go` con este contenido:

```go
package main

import "fmt"

func main() {
	fmt.Println("¡Hola, mundo!")
}
```

```sortie
¡Hola, mundo!
```

Leámoslo línea por línea.

- `package main`: todo archivo Go pertenece a un **paquete** (*package*), un grupo de
  archivos que van juntos. El paquete `main` es especial: es el de un programa
  que se puede ejecutar.
- `import "fmt"`: se importa el paquete `fmt` (de *format*), que viene con Go. Sabe
  mostrar texto.
- `func main() { ... }`: se define la [función](../debuter/glossaire.md#funcion)
  `main`. Es la puerta de entrada: Go siempre empieza por ella.
- `fmt.Println("...")`: se llama a la función `Println` del paquete `fmt`. Muestra
  el texto y luego pasa a la línea siguiente (*ln* de *line*).

El texto va entre comillas dobles `"..."`. Es una
[cadena de caracteres](../debuter/glossaire.md#cadena-de-caracteres).

### Varias líneas

Las instrucciones dentro de `main` se ejecutan en orden, de arriba abajo.

```go
package main

import "fmt"

func main() {
	fmt.Println("Línea 1")
	fmt.Println("Línea 2")
	fmt.Println(2 + 3)
	fmt.Println("Total:", 2+3, "euros")
}
```

```sortie
Línea 1
Línea 2
5
Total: 5 euros
```

- `fmt.Println(2 + 3)` muestra `5`: sin comillas, Go calcula.
- Con varios valores separados por comas, `Println` los muestra con un espacio
  entre cada uno.

> **Nota:** no hay punto y coma al final de las líneas. Go los añade él solo
> al leer el archivo.

## Ejecutar un programa con go run

En la terminal, sitúate en la carpeta del archivo (con el comando `cd`) y
escribe:

```sh
go run hola.go
```

```text
¡Hola, mundo!
```

`go run` hace dos cosas de una vez: compila el archivo y luego ejecuta el resultado.
No se guarda nada en el disco. Es el comando que más usarás para
probar tu código.

## Compilar con go build

`go build` compila el archivo y **guarda** el programa obtenido.

```sh
go build hola.go
./hola
```

```text
¡Hola, mundo!
```

- En Linux y macOS, `go build` crea un archivo `hola`. Lo ejecutas con
  `./hola`.
- En Windows, crea `hola.exe`. Lo ejecutas con `.\hola.exe`.

Este archivo es autónomo: puedes copiarlo a otra máquina con el mismo sistema,
aunque no tenga Go instalado, y funcionará.

> **Recuerda:** `go run` para probar, `go build` para fabricar el programa final.

## Dar formato con gofmt

En Go, todo el mundo presenta su código de la misma manera. La herramienta `gofmt` (y el
comando `go fmt`) ordena tu archivo por ti: corrige los espacios, alinea las
líneas y pone tabulaciones para la sangría.

```sh
gofmt -w hola.go
```

La opción `-w` (*write*) reescribe el archivo. Sin ella, `gofmt` solo muestra el
resultado.

> **Consejo:** la mayoría de los editores (con la extensión de Go) ejecutan `gofmt` cada vez
> que guardas. Actívalo: nunca más tendrás que pensar en la presentación.

## Leer un mensaje de error

Cuando el compilador no entiende tu código, se detiene y muestra un
[error](../debuter/glossaire.md#error). No pasa nada: el mensaje te ayuda.

```text
# command-line-arguments
./main.go:6:6: undefined: fmt.println
```

Léelo de izquierda a derecha:

- `# command-line-arguments`: el nombre que Go da al paquete ejecutado directamente;
- `./main.go:6:6`: el archivo, la línea (6) y la columna (6) del problema;
- `undefined: fmt.println`: la descripción («`fmt.println` no está definido»).

Los mensajes están en inglés. Algunas palabras aparecen a menudo: *undefined* (no
definido), *unexpected* (inesperado), *expected* (esperaba), *missing* (falta),
*not used* (sin usar).

## Errores frecuentes

### Olvidar la mayúscula de Println

```go norun
package main

import "fmt"

func main() {
	fmt.println("Hola")
}
```

```text
# command-line-arguments
./main.go:6:6: undefined: fmt.println
```

Go distingue entre mayúsculas y minúsculas. La función se llama `Println`,
con una `P` mayúscula. Escribe `fmt.Println("Hola")`.

### Importar un paquete sin usarlo

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

El paquete `os` está importado, pero nunca se usa. Go se niega a compilar. Borra la
línea `"os"`.

### Poner la llave en la línea siguiente

```go norun
package main

import "fmt"

func main()
{
	fmt.Println("Hola")
}
```

```text
# command-line-arguments
./main.go:6:1: syntax error: unexpected semicolon or newline before {
```

Go añade un punto y coma invisible al final de `func main()`. Por eso la llave `{` debe
estar en la **misma** línea: `func main() {`.

### Equivocarse de nombre de paquete

```go norun
package hola

import "fmt"

func main() {
	fmt.Println("Hola")
}
```

```text
package command-line-arguments is not a main package
```

Un programa que se ejecuta debe estar en el paquete `main`. Escribe `package main` en
la primera línea.

## Ponte a prueba

**Pregunta 1:** ¿cuál es la diferencia entre `go run` y `go build`?

> **Respuesta**
>
> `go run` compila y luego ejecuta el programa, sin guardar nada. `go build` compila y
> crea un archivo ejecutable, que después ejecutas tú mismo.

**Pregunta 2:** escribe un programa que muestre `Empiezo con Go` y luego, en la línea
siguiente, el resultado de `7 * 6`.

> **Respuesta**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	fmt.Println("Empiezo con Go")
> 	fmt.Println(7 * 6)
> }
> ```
>
> ```sortie
> Empiezo con Go
> 42
> ```

**Pregunta 3:** ¿por qué función empieza siempre un programa Go?

> **Respuesta**
>
> Por la función `main` del paquete `main`.

## En resumen

- Go es un lenguaje compilado, sencillo y rápido, creado en Google; se usa sobre todo para
  servidores y herramientas.
- Instálalo desde go.dev/dl y compruébalo con `go version`.
- Un programa empieza por `package main`, importa sus paquetes con `import`, y
  arranca en `func main()`.
- `go run archivo.go` ejecuta el código; `go build` fabrica un ejecutable.
- `gofmt` da formato al código; el compilador rechaza los imports sin usar.

Siguiente: [primeros pasos](premiers-pas.md).
