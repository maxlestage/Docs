# Los genéricos

Esta página muestra cómo escribir una sola función o un solo tipo que funcione con varios tipos, gracias a los parámetros de tipo.

> **Objetivos:**
>
> - sabrás escribir una función genérica con `[T any]`;
> - sabrás limitar los tipos aceptados con una restricción (`comparable`, `cmp.Ordered`);
> - sabrás escribir tu propia restricción con `~int | ~float64`;
> - sabrás crear un tipo genérico, como una pila;
> - sabrás cuándo los genéricos no son útiles.

## El problema: escribir dos veces lo mismo

Imagina una función que devuelve el mayor de dos enteros. Luego necesitas lo mismo para
números decimales, y después para textos. Sin genéricos, hay que escribir tres funciones
casi idénticas:

```go norun
func MaxInt(a, b int) int { ... }
func MaxFloat(a, b float64) float64 { ... }
func MaxString(a, b string) string { ... }
```

Un [genérico](../debuter/glossaire.md#generico) resuelve este problema: escribes el
código una vez, con un **parámetro de tipo** en lugar del tipo real. Es como un molde
de pastel: la forma es la misma, tanto si viertes chocolate como vainilla.

## Una primera función genérica

El parámetro de tipo se escribe entre corchetes, justo después del nombre de la función:
`[T any]`. `T` es un nombre de tipo provisional; `any` dice que `T` puede ser cualquier
tipo.

```go
package main

import "fmt"

func Primero[T any](elementos []T) T {
	return elementos[0]
}

func main() {
	fmt.Println(Primero([]int{4, 8, 15}))
	fmt.Println(Primero([]string{"manzana", "pera"}))
	fmt.Println(Primero[float64]([]float64{1.5, 2.5}))
}
```

```sortie
4
manzana
1.5
```

Go **deduce** `T` a partir de los argumentos: con `[]int`, `T` vale `int`. También
puedes darlo tú, como en `Primero[float64](...)`.

## Las restricciones

Con `any`, Go no sabe nada de `T`. Por eso no puedes comparar dos `T` con `>`: algunos
tipos no se pueden comparar. Una **restricción** limita los tipos aceptados y, a cambio,
permite operaciones.

### comparable

`comparable` acepta los tipos que admiten `==` y `!=`: números, textos,
booleanos, estructuras simples…

```go
package main

import "fmt"

func Contiene[T comparable](elementos []T, buscado T) bool {
	for _, e := range elementos {
		if e == buscado {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println(Contiene([]int{1, 2, 3}, 2))
	fmt.Println(Contiene([]string{"rojo", "verde"}, "azul"))
}
```

```sortie
true
false
```

### cmp.Ordered

El paquete `cmp` ofrece `cmp.Ordered`: los tipos que admiten `<`, `<=`, `>` y
`>=`. Son los enteros, los flotantes y los textos.

```go
package main

import (
	"cmp"
	"fmt"
)

func Max[T cmp.Ordered](a, b T) T {
	if a > b {
		return a
	}
	return b
}

func main() {
	fmt.Println(Max(3, 7))
	fmt.Println(Max(2.5, 1.5))
	fmt.Println(Max("albaricoque", "banana"))
}
```

```sortie
7
2.5
banana
```

Los textos se comparan en orden alfabético: `"banana"` va después de `"albaricoque"`.

> **Nota:** desde Go 1.21, `max` y `min` ya existen en el lenguaje, y
> `slices.Max` encuentra el mayor elemento de un slice. Aquí `Max` es un ejercicio.

## Escribir tu propia restricción

Una restricción es una interfaz. Puede enumerar tipos separados por `|`
(«o»). La `~` delante de un tipo quiere decir: «este tipo, o cualquier tipo construido
sobre él». Así, `~int` acepta `int`, pero también un tipo `type Edad int`.

```go
package main

import "fmt"

type Numero interface {
	~int | ~float64
}

func Suma[T Numero](valores []T) T {
	var total T
	for _, v := range valores {
		total += v
	}
	return total
}

type Euros float64

func main() {
	fmt.Println(Suma([]int{1, 2, 3}))
	fmt.Println(Suma([]float64{0.5, 0.25}))
	fmt.Println(Suma([]Euros{9.99, 0.01}))
}
```

```sortie
6
0.75
10
```

`var total T` crea el valor cero de `T`: `0` para un número. Sin la `~`, la línea con
`Euros` sería rechazada.

## Map y Filter

Los genéricos brillan en las funciones que trabajan con
[colecciones](collections.md). `Map` transforma cada elemento; `Filter` conserva los que
pasan una prueba. Reciben una función como parámetro (ver
[las funciones](fonctions.md)).

```go
package main

import (
	"fmt"
	"strings"
)

func Map[T, U any](elementos []T, f func(T) U) []U {
	resultado := make([]U, 0, len(elementos))
	for _, e := range elementos {
		resultado = append(resultado, f(e))
	}
	return resultado
}

func Filter[T any](elementos []T, conservar func(T) bool) []T {
	var resultado []T
	for _, e := range elementos {
		if conservar(e) {
			resultado = append(resultado, e)
		}
	}
	return resultado
}

func main() {
	numeros := []int{1, 2, 3, 4, 5, 6}

	cuadrados := Map(numeros, func(n int) int { return n * n })
	fmt.Println(cuadrados)

	pares := Filter(numeros, func(n int) bool { return n%2 == 0 })
	fmt.Println(pares)

	palabras := Map([]string{"go", "es", "simple"}, strings.ToUpper)
	fmt.Println(palabras)

	longitudes := Map(palabras, func(s string) int { return len(s) })
	fmt.Println(longitudes)
}
```

```sortie
[1 4 9 16 25 36]
[2 4 6]
[GO ES SIMPLE]
[2 2 6]
```

`Map` tiene **dos** parámetros de tipo: `T` para la entrada y `U` para la salida. Pueden
ser distintos, como en la última llamada (de `string` a `int`).

## Los tipos genéricos

Un tipo también puede tener un parámetro de tipo. Aquí tienes una **pila**: una lista en
la que siempre se añade y se quita por arriba, como una pila de platos. El último
elemento colocado es el primero que se retira.

```go
package main

import "fmt"

type Pila[T any] struct {
	elementos []T
}

func (p *Pila[T]) Apilar(v T) {
	p.elementos = append(p.elementos, v)
}

func (p *Pila[T]) Desapilar() (T, bool) {
	var cero T
	if len(p.elementos) == 0 {
		return cero, false
	}
	ultimo := p.elementos[len(p.elementos)-1]
	p.elementos = p.elementos[:len(p.elementos)-1]
	return ultimo, true
}

func (p *Pila[T]) Tamano() int {
	return len(p.elementos)
}

func main() {
	var platos Pila[string]
	platos.Apilar("azul")
	platos.Apilar("rojo")
	platos.Apilar("verde")
	fmt.Println("Tamaño:", platos.Tamano())

	for platos.Tamano() > 0 {
		p, _ := platos.Desapilar()
		fmt.Println(p)
	}

	_, ok := platos.Desapilar()
	fmt.Println("¿Queda alguno?", ok)

	var numeros Pila[int]
	numeros.Apilar(42)
	n, _ := numeros.Desapilar()
	fmt.Println(n + 1)
}
```

```sortie
Tamaño: 3
verde
rojo
azul
¿Queda alguno? false
43
```

En los métodos, se escribe `Pila[T]` para el receptor. `Desapilar` también devuelve un
booleano: la pila puede estar vacía, y hay que decírselo a quien llama.

## Cuándo no usar los genéricos

Los genéricos son útiles, pero hacen el código más abstracto. Evítalos:

- cuando tu función solo sirve para **un único** tipo: escribe ese tipo, sin más;
- cuando basta con una **interfaz**: si solo llamas a métodos, como `Area()`, usa una
  interfaz (ver [las interfaces](interfaces.md));
- cuando el código de cada tipo es **diferente**: un genérico sirve para escribir el
  mismo código para todos.

> **Recuerda:** empieza con código simple para un tipo. Pasa a los genéricos cuando te
> sorprendas copiando la misma función para un segundo tipo.

## Errores frecuentes

### Comparar con la restricción any

```go norun
package main

import "fmt"

func Max[T any](a, b T) T {
	if a > b {
		return a
	}
	return b
}

func main() {
	fmt.Println(Max(3, 7))
}
```

```text
# command-line-arguments
./main.go:6:5: invalid operation: a > b (type parameter T cannot use operator >)
```

`any` acepta todos los tipos, incluidos los que no se pueden comparar. Usa la
restricción `cmp.Ordered` e importa `cmp`.

### Mezclar dos tipos para un mismo T

```go norun
package main

import (
	"cmp"
	"fmt"
)

func Max[T cmp.Ordered](a, b T) T {
	if a > b {
		return a
	}
	return b
}

func main() {
	x := 3
	y := 2.5
	fmt.Println(Max(x, y))
}
```

```text
# command-line-arguments
./main.go:18:21: in call to Max, type float64 of y does not match inferred type int for T
```

`a` y `b` tienen el mismo tipo `T`. Go eligió `int` a partir de `x` y luego encuentra un
`float64`. Convierte: `Max(float64(x), y)`.

### Go no puede deducir T

```go norun
package main

import "fmt"

func Vacia[T any]() []T {
	return []T{}
}

func main() {
	s := Vacia()
	fmt.Println(s)
}
```

```text
# command-line-arguments
./main.go:10:12: in call to Vacia, cannot infer T (declared at ./main.go:5:12)
```

Ningún argumento permite deducir `T`. Indícalo entre corchetes: `Vacia[int]()`.

## Ponte a prueba

**Pregunta 1:** ¿qué restricción eliges para una función que usa `==` con sus
elementos?

> **Respuesta**
>
> `comparable`. Acepta todos los tipos que admiten `==` y `!=`.

**Pregunta 2:** ¿qué quiere decir la `~` en `~int`?

> **Respuesta**
>
> «`int` o cualquier tipo cuyo tipo base sea `int`». Por ejemplo, `type Edad int`
> cumple `~int`, pero no `int` a secas.

**Pregunta 3:** escribe una función genérica `Contar[T any](s []T, f func(T) bool) int`
que cuente los elementos para los que `f` devuelve `true`.

> **Respuesta**
>
> ```go
> package main
>
> import "fmt"
>
> func Contar[T any](s []T, f func(T) bool) int {
> 	n := 0
> 	for _, e := range s {
> 		if f(e) {
> 			n++
> 		}
> 	}
> 	return n
> }
>
> func main() {
> 	notas := []int{12, 8, 15, 9, 18}
> 	fmt.Println(Contar(notas, func(n int) bool { return n >= 10 }))
> }
> ```
>
> ```sortie
> 3
> ```

## En resumen

- `[T any]` declara un parámetro de tipo; Go suele deducirlo a partir de los argumentos.
- Una restricción limita los tipos: `comparable` para `==`, `cmp.Ordered` para `<` y `>`.
- Una restricción propia se escribe como una interfaz: `~int | ~float64`.
- Los tipos también pueden ser genéricos: `Pila[T]`, con métodos sobre `*Pila[T]`.
- Usa los genéricos solo para evitar repeticiones reales.

Siguiente: [gestionar los errores](erreurs.md).
