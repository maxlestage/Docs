# Las interfaces

Esta página explica cómo describir un comportamiento con una interfaz, para escribir código que acepte varios tipos distintos.

> **Objetivos:**
>
> - sabrás definir una interfaz y satisfacerla sin declarar nada;
> - sabrás guardar valores de tipos distintos en una misma slice;
> - sabrás hacer que un tipo se pueda mostrar con `fmt.Stringer`;
> - sabrás recuperar el tipo escondido detrás de `any` con una aserción o un `switch`;
> - sabrás por qué Go prefiere las interfaces pequeñas.

## ¿Qué es una interfaz?

Una **interfaz** es una lista de métodos. Dice **lo que un tipo sabe hacer**, no lo que
es.

Piensa en un enchufe. El enchufe no pregunta si conectas una lámpara, una tostadora o un
cargador. Solo pregunta: «¿tienes la clavija correcta?». Una interfaz es ese enchufe:
cualquier tipo que tenga los métodos correctos puede conectarse.

```go
package main

import "fmt"

type Forma interface {
	Area() float64
}

type Rectangulo struct {
	Ancho, Alto float64
}

func (r Rectangulo) Area() float64 {
	return r.Ancho * r.Alto
}

func mostrarArea(f Forma) {
	fmt.Println("Área:", f.Area())
}

func main() {
	r := Rectangulo{Ancho: 3, Alto: 4}
	mostrarArea(r)
}
```

```sortie
Área: 12
```

La función `mostrarArea` acepta cualquier `Forma`. No conoce `Rectangulo`: solo sabe que
puede llamar a `Area()`.

## Una implementación implícita

Fíjate en que `Rectangulo` no dice en ningún sitio «implemento `Forma`». En Go, esto es
**implícito**: un tipo satisface una interfaz en cuanto tiene todos sus métodos, con los
mismos nombres y los mismos tipos.

Es muy práctico: puedes crear una interfaz para tipos escritos por otra persona, sin
modificarlos. En Rust, en cambio, hay que escribir `impl Trait for Type` (mira
[los traits en Rust](../rust/types.md)).

## El polimorfismo

Añadamos un segundo tipo, `Circulo`. También tiene un método `Area()`: por lo tanto,
también es una `Forma`. Una slice `[]Forma` puede contener rectángulos **y** círculos.

Esta palabra culta, **polimorfismo** («muchas formas»), quiere decir simplemente: la
misma llamada `f.Area()` hace un trabajo distinto según el tipo real de `f`.

```go
package main

import (
	"fmt"
	"math"
)

type Forma interface {
	Area() float64
}

type Rectangulo struct {
	Ancho, Alto float64
}

func (r Rectangulo) Area() float64 {
	return r.Ancho * r.Alto
}

type Circulo struct {
	Radio float64
}

func (c Circulo) Area() float64 {
	return math.Pi * c.Radio * c.Radio
}

func main() {
	formas := []Forma{
		Rectangulo{Ancho: 3, Alto: 4},
		Circulo{Radio: 1},
		Rectangulo{Ancho: 2, Alto: 2},
	}

	total := 0.0
	for _, f := range formas {
		fmt.Printf("%.2f\n", f.Area())
		total += f.Area()
	}
	fmt.Printf("Total: %.2f\n", total)
}
```

```sortie
12.00
3.14
4.00
Total: 19.14
```

## fmt.Stringer: elegir cómo se muestra

El paquete `fmt` define una interfaz muy pequeña:

```go norun
type Stringer interface {
	String() string
}
```

Si tu tipo tiene un método `String() string`, `fmt.Println` y `%v` lo usan para
mostrarlo. Es como dar tu tarjeta de visita.

```go
package main

import "fmt"

type Temperatura struct {
	Grados float64
}

func (t Temperatura) String() string {
	return fmt.Sprintf("%.1f °C", t.Grados)
}

func main() {
	t := Temperatura{Grados: 21.456}
	fmt.Println(t)
	fmt.Printf("Hace %v fuera.\n", t)
}
```

```sortie
21.5 °C
Hace 21.5 °C fuera.
```

## La interfaz vacía: any

Una interfaz sin ningún método, `interface{}`, la satisfacen **todos** los tipos. Go le
da un nombre más corto: `any` («cualquier cosa»).

```go
package main

import "fmt"

func main() {
	var caja any
	caja = 42
	fmt.Println(caja)
	caja = "hola"
	fmt.Println(caja)

	mezcla := []any{1, "dos", 3.0, true}
	fmt.Println(len(mezcla), "elementos")
}
```

```sortie
42
hola
4 elementos
```

> **Atención:** con `any` pierdes la ayuda del compilador: ya no sabe qué tipo hay
> guardado dentro. Úsalo solo cuando de verdad haga falta. `fmt.Println` es un buen
> ejemplo: acepta valores `any` para poder mostrarlo todo.

## Las aserciones de tipo

Para recuperar el valor guardado en una interfaz, se escribe una **aserción de tipo**:
`x.(string)`. Quiere decir: «creo que `x` contiene un `string`».

Si te equivocas, el programa se detiene. Usa por tanto la forma con dos resultados,
llamada **coma ok**: `s, ok := x.(string)`. Si el tipo es el correcto, `ok` vale `true`;
si no, `ok` vale `false` y `s` vale el valor cero.

```go
package main

import "fmt"

func main() {
	var x any = "hola"

	s, ok := x.(string)
	fmt.Println(s, ok)

	n, ok := x.(int)
	fmt.Println(n, ok)

	if texto, ok := x.(string); ok {
		fmt.Println("Longitud:", len(texto))
	}
}
```

```sortie
hola true
0 false
Longitud: 4
```

## El switch de tipo

Cuando hay varios tipos posibles, un **switch de tipo** se lee mejor. Se escribe
`x.(type)`, solo dentro de un `switch`. En cada `case`, la variable tiene el tipo
correcto.

```go
package main

import "fmt"

func describir(x any) string {
	switch v := x.(type) {
	case int:
		return fmt.Sprintf("un entero, su doble vale %d", v*2)
	case string:
		return fmt.Sprintf("un texto de %d letras", len(v))
	case bool:
		return "un booleano"
	default:
		return fmt.Sprintf("un tipo desconocido: %T", v)
	}
}

func main() {
	fmt.Println(describir(21))
	fmt.Println(describir("Go"))
	fmt.Println(describir(true))
	fmt.Println(describir(3.5))
}
```

```sortie
un entero, su doble vale 42
un texto de 2 letras
un booleano
un tipo desconocido: float64
```

`%T` muestra el nombre del tipo de un valor.

## Las interfaces pequeñas

En Go, las buenas interfaces son **pequeñas**: a menudo un solo método. Su nombre suele
terminar entonces en `-er`: `Stringer`, `Reader`, `Writer`. Un proverbio de Go dice:
«cuanto más grande es la interfaz, más débil es la abstracción».

Una interfaz pequeña es fácil de satisfacer, así que muchos tipos pueden conectarse a
ella. El mejor ejemplo es `io.Writer`, «algo en lo que se puede escribir»:

```go norun
type Writer interface {
	Write(p []byte) (n int, err error)
}
```

Un archivo, la salida estándar `os.Stdout`, una conexión de red y un `strings.Builder`
son todos `io.Writer`. Una función que escribe en un `io.Writer` funciona por tanto con
cada uno de ellos.

```go
package main

import (
	"fmt"
	"io"
	"os"
	"strings"
)

func escribirLista(w io.Writer, compras []string) {
	for i, c := range compras {
		fmt.Fprintf(w, "%d. %s\n", i+1, c)
	}
}

func main() {
	compras := []string{"pan", "leche", "manzanas"}

	escribirLista(os.Stdout, compras)

	var b strings.Builder
	escribirLista(&b, compras)
	fmt.Println("El builder contiene", len(b.String()), "bytes.")
}
```

```sortie
1. pan
2. leche
3. manzanas
El builder contiene 28 bytes.
```

`fmt.Fprintf` es como `fmt.Printf`, pero escribe en el `io.Writer` que le das. El mismo
código escribe una vez en la pantalla y otra vez en memoria.

> **Consejo:** cuando escribas una función, pide la interfaz más pequeña que necesite.
> Aquí, `escribirLista` pide un `io.Writer`, no un archivo: es más fácil de reutilizar y
> de [probar](tests.md).

## Errores frecuentes

### Falta un método

```go norun
package main

import "fmt"

type Forma interface {
	Area() float64
	Perimetro() float64
}

type Cuadrado struct {
	Lado float64
}

func (c Cuadrado) Area() float64 {
	return c.Lado * c.Lado
}

func main() {
	var f Forma = Cuadrado{Lado: 2}
	fmt.Println(f.Area())
}
```

```text
# command-line-arguments
./main.go:19:16: cannot use Cuadrado{…} (value of struct type Cuadrado) as Forma value in variable declaration: Cuadrado does not implement Forma (missing method Perimetro)
```

El mensaje da el nombre del método que falta: `Perimetro`. Añádelo a `Cuadrado`.

### Un método con un receptor puntero

```go norun
package main

import "fmt"

type Forma interface {
	Area() float64
}

type Cuadrado struct {
	Lado float64
}

func (c *Cuadrado) Area() float64 {
	return c.Lado * c.Lado
}

func main() {
	var f Forma = Cuadrado{Lado: 2}
	fmt.Println(f.Area())
}
```

```text
# command-line-arguments
./main.go:18:16: cannot use Cuadrado{…} (value of struct type Cuadrado) as Forma value in variable declaration: Cuadrado does not implement Forma (method Area has pointer receiver)
```

El método está definido sobre `*Cuadrado`, así que solo `*Cuadrado` es una `Forma`. Da
un puntero: `var f Forma = &Cuadrado{Lado: 2}`. O bien, si el método no modifica nada,
usa un receptor valor: `func (c Cuadrado) Area()`.

### Una aserción sin coma ok

```go norun
package main

import "fmt"

func main() {
	var x any = "hola"
	n := x.(int)
	fmt.Println(n)
}
```

```text
panic: interface conversion: interface {} is string, not int

goroutine 1 [running]:
main.main()
	/ruta/a/main.go:7 +0x28
exit status 2
```

`x` contiene un `string`, no un `int`: el programa
[entra en pánico](../debuter/glossaire.md#panique). Usa `n, ok := x.(int)` y comprueba
`ok`.

## Ponte a prueba

**Pregunta 1:** un tipo `Gato` tiene un método `String() string`. ¿Qué hace
`fmt.Println(miGato)`?

> **Respuesta**
>
> Llama a `miGato.String()` y muestra el texto devuelto, porque `Gato` satisface la
> interfaz `fmt.Stringer`.

**Pregunta 2:** ¿cuánto valen `v` y `ok` después de `v, ok := x.(int)` si `x` contiene
`"7"`?

> **Respuesta**
>
> `v` vale `0` (el valor cero de los enteros) y `ok` vale `false`: `x` contiene un
> texto, no un entero. El programa no se detiene.

**Pregunta 3:** escribe una interfaz `Animal` con un método `Sonido() string`, dos tipos
`Perro` y `Gato`, y luego muestra el sonido de cada uno en un bucle.

> **Respuesta**
>
> ```go
> package main
>
> import "fmt"
>
> type Animal interface {
> 	Sonido() string
> }
>
> type Perro struct{}
>
> func (Perro) Sonido() string { return "Guau" }
>
> type Gato struct{}
>
> func (Gato) Sonido() string { return "Miau" }
>
> func main() {
> 	animales := []Animal{Perro{}, Gato{}}
> 	for _, a := range animales {
> 		fmt.Println(a.Sonido())
> 	}
> }
> ```
>
> ```sortie
> Guau
> Miau
> ```

## En resumen

- Una interfaz es una lista de métodos; un tipo la satisface sin declararlo.
- Una slice de interfaces guarda tipos distintos que comparten un comportamiento.
- `String() string` hace que `fmt` pueda mostrar un tipo (interfaz `fmt.Stringer`).
- `any` lo acepta todo; `v, ok := x.(T)` y `switch x.(type)` recuperan el tipo real.
- Prefiere las interfaces pequeñas, como `io.Writer`.

Siguiente: [los punteros](pointeurs.md).
