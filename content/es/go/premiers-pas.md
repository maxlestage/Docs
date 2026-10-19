# Primeros pasos

Esta página muestra cómo guardar valores en variables, cuáles son los tipos básicos de Go y cómo mostrar resultados de forma limpia con `fmt`.

> **Objetivos:**
>
> - sabrás crear una variable con `var` o `:=`;
> - conocerás los tipos básicos y su valor cero;
> - sabrás crear constantes, y series de constantes con `iota`;
> - sabrás mostrar valores con `fmt.Println` y `fmt.Printf`;
> - sabrás por qué algunos nombres empiezan por mayúscula.

## Las variables

Una [variable](../debuter/glossaire.md#variable) es una caja con una etiqueta.
Guardas un valor dentro, y lo encuentras gracias a su nombre.

En Go, la forma más completa de crear una variable usa la palabra `var`:

```go
package main

import "fmt"

func main() {
	var nombre string = "Alice"
	var edad int = 12
	fmt.Println(nombre, "tiene", edad, "años")
}
```

```sortie
Alice tiene 12 años
```

Lee `var edad int = 12` así: «crea una variable `edad`, de tipo `int` (un entero),
que vale 12».

El [tipo](../debuter/glossaire.md#tipo) dice qué clase de valor contiene la caja:
un número, un texto, un verdadero/falso… En Go, una variable conserva **siempre** el mismo tipo.

### Cambiar el valor

El signo `=` guarda un nuevo valor en una variable que ya existe.

```go
package main

import "fmt"

func main() {
	var puntos int = 10
	fmt.Println(puntos)
	puntos = 25
	fmt.Println(puntos)
	puntos = puntos + 5
	fmt.Println(puntos)
}
```

```sortie
10
25
30
```

`puntos = puntos + 5` se lee de derecha a izquierda: Go calcula `puntos + 5` (25 + 5), y luego
guarda el resultado en `puntos`.

## La declaración corta :=

Escribir el tipo cada vez es largo. Si das un valor, Go **adivina** el tipo
él solo. Dentro de una función, se usa entonces `:=`:

```go
package main

import "fmt"

func main() {
	nombre := "Bob"
	edad := 9
	altura := 1.32
	fmt.Println(nombre, edad, altura)
}
```

```sortie
Bob 9 1.32
```

- `nombre := "Bob"` crea una variable de tipo `string`;
- `edad := 9` crea un `int`;
- `altura := 1.32` crea un `float64` (un número decimal).

> **Recuerda:** `:=` **crea** una variable. `=` **cambia** una variable que ya
> existe. Es la diferencia más importante de este capítulo.

`:=` solo funciona dentro de una función. Fuera, a nivel del archivo,
usa `var`.

### Varias variables de una vez

```go
package main

import "fmt"

func main() {
	x, y := 3, 4
	fmt.Println(x, y)
	x, y = y, x
	fmt.Println(x, y)
}
```

```sortie
3 4
4 3
```

La línea `x, y = y, x` intercambia los dos valores, sin variable intermedia.

## Los valores cero

Si creas una variable con `var` sin darle un valor, no está vacía:
recibe el **valor cero** de su tipo.

```go
package main

import "fmt"

func main() {
	var n int
	var precio float64
	var activo bool
	var nombre string
	fmt.Println(n, precio, activo)
	fmt.Printf("[%s]\n", nombre)
}
```

```sortie
0 0 false
[]
```

| Tipo | Valor cero |
|------|-------------|
| `int`, `float64` | `0` |
| `bool` | `false` |
| `string` | `""` (texto vacío) |

> **Nota:** en Go, una variable siempre tiene un valor. No existe ninguna variable
> «sin inicializar» que contenga cualquier cosa.

## Los tipos básicos

| Tipo | Lo que contiene | Ejemplo |
|------|-------------------|---------|
| `int` | un [entero](../debuter/glossaire.md#entero) | `42`, `-7` |
| `float64` | un [número decimal](../debuter/glossaire.md#flotante) | `3.14` |
| `string` | texto | `"hola"` |
| `bool` | un [booleano](../debuter/glossaire.md#booleano): verdadero o falso | `true`, `false` |
| `rune` | un solo carácter | `'é'` |
| `byte` | un [byte](../debuter/glossaire.md#byte) (de 0 a 255) | `65` |

```go
package main

import "fmt"

func main() {
	nota := 15
	media := 13.5
	ciudad := "León"
	aprobado := nota >= 10
	inicial := 'L'
	fmt.Println(nota, media, ciudad, aprobado, inicial)
}
```

```sortie
15 13.5 León true 76
```

Sorpresa: `inicial` se muestra como `76`. Una `rune` es en realidad un número, el código
[Unicode](../debuter/glossaire.md#unicode) del carácter. Para ver la letra, hace falta
`Printf` y `%c`, como vamos a ver.

## Las constantes

Una [constante](../debuter/glossaire.md#constante) es un valor que no cambia nunca.
Se crea con `const`.

```go
package main

import "fmt"

const TasaIVA = 0.2

func main() {
	const pi = 3.14159
	precio := 50.0
	fmt.Println(precio * (1 + TasaIVA))
	fmt.Println(pi * 2)
}
```

```sortie
60
6.28318
```

Una constante se puede declarar fuera de `main`, a nivel del archivo: entonces es
visible en todas partes. Intentar modificarla (`pi = 3`) es un error de compilación.

### Series de constantes con iota

Para numerar constantes, Go ofrece `iota`. En un bloque `const ( ... )`, `iota`
vale 0 en la primera línea, 1 en la siguiente, y así sucesivamente.

```go
package main

import "fmt"

const (
	Lunes = iota
	Martes
	Miercoles
	Jueves
)

func main() {
	fmt.Println(Lunes, Martes, Miercoles, Jueves)
}
```

```sortie
0 1 2 3
```

Las líneas sin valor repiten la fórmula de la línea de arriba, con `iota`
aumentando. Así que se puede hacer un cálculo:

```go
package main

import "fmt"

const (
	Bronce = (iota + 1) * 100
	Plata
	Oro
)

func main() {
	fmt.Println(Bronce, Plata, Oro)
}
```

```sortie
100 200 300
```

## Mostrar con fmt

### Println

`fmt.Println` muestra sus valores separados por espacios y luego pasa a la línea siguiente.
`fmt.Print` hace lo mismo sin pasar a la línea siguiente.

### Printf y los verbos

`fmt.Printf` (*print formatted*, «mostrar con un formato») recibe un **modelo** de
texto. Cada `%` seguido de una letra es un hueco, llamado **verbo**, que se reemplaza por un
valor. Añade `\n` al final para pasar a la línea siguiente.

```go
package main

import "fmt"

func main() {
	nombre := "Alice"
	edad := 12
	fmt.Printf("%s tiene %d años.\n", nombre, edad)
}
```

```sortie
Alice tiene 12 años.
```

Los verbos más útiles:

| Verbo | Muestra | Ejemplo de resultado |
|-------|---------|---------------------|
| `%d` | un entero | `42` |
| `%s` | un texto | `Alice` |
| `%q` | un texto entre comillas | `"Alice"` |
| `%v` | cualquier valor, en el formato por defecto | `[1 2 3]` |
| `%+v` | como `%v`, con el nombre de los campos de una estructura | `{Nombre:Alice Edad:12}` |
| `%T` | el tipo del valor | `float64` |
| `%.2f` | un número decimal, con 2 decimales | `3.14` |
| `%c` | un carácter (una rune) | `L` |
| `%%` | el propio signo `%` | `%` |

```go
package main

import "fmt"

type Alumno struct {
	Nombre string
	Edad   int
}

func main() {
	pi := 3.14159
	fmt.Printf("%.2f\n", pi)
	fmt.Printf("%T y %T\n", pi, 42)
	fmt.Printf("%q\n", "hola")
	fmt.Printf("%c\n", 'L')
	fmt.Printf("%v\n", []int{1, 2, 3})
	a := Alumno{Nombre: "Alice", Edad: 12}
	fmt.Printf("%v\n", a)
	fmt.Printf("%+v\n", a)
	fmt.Printf("Descuento: 20%%\n")
}
```

```sortie
3.14
float64 y int
"hola"
L
[1 2 3]
{Alice 12}
{Nombre:Alice Edad:12}
Descuento: 20%
```

> **Consejo:** ¿perdido? Usa `%v`: funciona con todo. Y `%T` te dice el tipo
> exacto de un valor, muy práctico para entender lo que pasa.

`Alumno` es una **estructura**: un valor que agrupa varios campos. Se
explican en [los tipos](types.md).

## Los comentarios

Un [comentario](../debuter/glossaire.md#comentario) es una nota para los humanos:
Go lo ignora.

```go
package main

import "fmt"

// main es el punto de partida del programa.
func main() {
	fmt.Println("visible") // esto también es un comentario
	/* un comentario
	   en varias líneas */
}
```

```sortie
visible
```

- `//` comenta hasta el final de la línea;
- `/* ... */` comenta todo lo que hay entre los dos.

> **Consejo:** por convención, un comentario justo encima de una función empieza
> por su nombre: `// main es...`. Las herramientas de documentación de Go lo aprovechan.

## Elegir buenos nombres

Un nombre de variable empieza por una letra, seguida de letras, cifras o `_`.
En Go, las palabras se pegan con mayúsculas: `precioTotal`, `numeroDeAlumnos`. Es el
*camelCase* («la caja del camello», por las jorobas).

```go
package main

import "fmt"

func main() {
	precioTotal := 42
	numAlumnos := 25
	fmt.Println(precioTotal, numAlumnos)
}
```

```sortie
42 25
```

### Mayúscula = visible desde otro paquete

En Go, la **primera letra** de un nombre tiene un significado:

- un nombre que empieza por **mayúscula** está **exportado**: los demás paquetes pueden
  usarlo;
- un nombre que empieza por **minúscula** se queda **privado** en su paquete.

Por eso se escribe `fmt.Println` con una `P` mayúscula: `Println` está exportado
por el paquete `fmt`. Una función `fmt.println` (minúscula), aunque existiera,
estaría oculta.

> **Recuerda:** en Go no hay palabra clave `public` ni `private`. Solo cuenta la mayúscula.

## Una variable sin usar es un error

El compilador de Go rechaza una variable creada en una función pero nunca usada.
Es a propósito: una variable olvidada suele esconder un bug.

Si de verdad quieres ignorar un valor, usa el **guion bajo** `_`, la «papelera»
de Go:

```go
package main

import "fmt"

func main() {
	a, _ := 10, 20
	fmt.Println(a)
}
```

```sortie
10
```

El valor `20` se tira: `_` no es una variable de verdad.

## Errores frecuentes

### Crear una variable sin usarla

```go norun
package main

import "fmt"

func main() {
	edad := 12
	nombre := "Alice"
	fmt.Println(nombre)
}
```

```text
# command-line-arguments
./main.go:6:2: declared and not used: edad
```

`edad` está declarada (*declared*) pero nunca se usa (*not used*). Úsala, o
borra la línea.

### Usar := dos veces para la misma variable

```go norun
package main

import "fmt"

func main() {
	puntos := 10
	puntos := 20
	fmt.Println(puntos)
}
```

```text
# command-line-arguments
./main.go:7:9: no new variables on left side of :=
```

`puntos` ya existe: `:=` no tiene nada nuevo que crear. Para cambiar el valor, escribe
`puntos = 20`, con `=`.

### Usar := fuera de una función

```go norun
package main

import "fmt"

ciudad := "Madrid"

func main() {
	fmt.Println(ciudad)
}
```

```text
# command-line-arguments
./main.go:5:1: syntax error: non-declaration statement outside function body
```

Fuera de una función, `:=` está prohibido. Escribe `var ciudad = "Madrid"`.

### Poner un valor del tipo incorrecto

```go norun
package main

import "fmt"

func main() {
	var edad int
	edad = "doce"
	fmt.Println(edad)
}
```

```text
# command-line-arguments
./main.go:7:9: cannot use "doce" (untyped string constant) as int value in assignment
```

`edad` es un `int` para siempre: no se puede guardar texto en ella. Escribe `edad = 12`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este programa?

```go norun
package main

import "fmt"

func main() {
	var total int
	var mensaje string
	fmt.Printf("%d [%s]\n", total, mensaje)
}
```

> **Respuesta**
>
> `0 []`. Las variables reciben su valor cero: `0` para un `int`, el texto vacío
> para un `string`.

**Pregunta 2:** con `Printf`, muestra `Precio: 12.50 €` a partir de la variable
`precio := 12.5`.

> **Respuesta**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	precio := 12.5
> 	fmt.Printf("Precio: %.2f €\n", precio)
> }
> ```
>
> ```sortie
> Precio: 12.50 €
> ```

**Pregunta 3:** ¿cuál es la diferencia entre `x := 5` y `x = 5`?

> **Respuesta**
>
> `x := 5` **crea** una nueva variable `x`. `x = 5` cambia el valor de una variable
> `x` que ya existe.

## En resumen

- `var nombre tipo = valor` crea una variable; `nombre := valor` hace lo mismo dentro de una
  función, adivinando el tipo.
- Una variable sin valor recibe el valor cero de su tipo (`0`, `""`, `false`).
- `const` crea un valor fijo; `iota` numera una serie de constantes.
- `fmt.Printf` reemplaza los verbos `%d`, `%s`, `%v`, `%T`, `%.2f`… por valores.
- Un nombre con mayúscula está exportado; una variable sin usar impide compilar.

Siguiente: [números y cálculos](nombres.md).
