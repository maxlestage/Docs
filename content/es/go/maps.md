# Los maps

Esta página presenta el map, la estructura de Go que asocia claves con valores, como un diccionario asocia palabras con sus definiciones.

> **Objetivos:**
>
> - sabrás crear un map con `make` o con valores iniciales;
> - sabrás leer, añadir, modificar y borrar una entrada;
> - sabrás comprobar si una clave existe con la forma «coma ok»;
> - sabrás recorrer un map en un orden fijo, ordenando las claves;
> - sabrás guardar slices o estructuras en un map.

## ¿Qué es un map?

Una slice guarda valores por **posición**: 0, 1, 2… Un map los guarda por **clave**: un
nombre que tú eliges. Es un [diccionario](../debuter/glossaire.md#diccionario): buscas
una palabra (la clave) para encontrar su definición (el valor).

El tipo de un map se escribe `map[TipoDeClave]TipoDeValor`. Por ejemplo,
`map[string]int` asocia textos con enteros.

```go
package main

import "fmt"

func main() {
	edades := map[string]int{
		"Alicia": 12,
		"Bob":    9,
	}
	fmt.Println(edades["Alicia"])
	fmt.Println(len(edades))
}
```

```sortie
12
2
```

- Cada entrada se escribe `clave: valor`, seguida de una coma (incluso la última).
- `edades["Alicia"]` lee el valor asociado a la clave `"Alicia"`.
- `len` da el número de entradas.

## Crear un map vacío con make

`make(map[string]int)` crea un map vacío, listo para recibir entradas.

```go
package main

import "fmt"

func main() {
	stock := make(map[string]int)
	stock["manzanas"] = 5
	stock["peras"] = 3
	fmt.Println(stock["manzanas"], len(stock))
}
```

```sortie
5 2
```

> **Atención:** un map declarado con `var stock map[string]int` vale `nil`. Puedes
> leerlo, pero escribir en él hace que el programa falle. Créalo siempre con `make` o
> con `{}`.

## Añadir, modificar, borrar

La misma escritura `m[clave] = valor` añade una entrada si la clave es nueva, y la
**reemplaza** si ya existe. `delete(m, clave)` borra una entrada.

```go
package main

import "fmt"

func main() {
	precios := map[string]float64{"pan": 1.2}
	precios["leche"] = 0.95
	precios["pan"] = 1.3
	fmt.Println(precios["pan"], len(precios))

	delete(precios, "leche")
	fmt.Println(len(precios))
	delete(precios, "chocolate")
	fmt.Println(len(precios))
}
```

```sortie
1.3 2
1
1
```

Borrar una clave que no está (`"chocolate"`) no es un error: no pasa nada.

## Una clave que no está

Leer una clave que no existe no provoca ningún error: Go devuelve el **valor cero** del
tipo de los valores.

```go
package main

import "fmt"

func main() {
	notas := map[string]int{"Alicia": 15, "Bob": 0}
	fmt.Println(notas["Clara"])
	fmt.Println(notas["Bob"])
}
```

```sortie
0
0
```

Problema: `0` para Clara (que no está) y `0` para Bob (que de verdad sacó 0) se
parecen. ¿Cómo distinguirlos?

### La forma «coma ok»

Lee el map con **dos** variables. La segunda, un booleano, dice si la clave existe.

```go
package main

import "fmt"

func main() {
	notas := map[string]int{"Alicia": 15, "Bob": 0}

	nota, ok := notas["Bob"]
	fmt.Println(nota, ok)

	nota, ok = notas["Clara"]
	fmt.Println(nota, ok)

	if n, existe := notas["Alicia"]; existe {
		fmt.Println("Alicia tiene", n)
	}
}
```

```sortie
0 true
0 false
Alicia tiene 15
```

Por costumbre, esta variable se llama `ok`. La última forma, con un `if` que crea la
variable, es muy habitual.

> **Recuerda:** `v := m[k]` da el valor (o cero). `v, ok := m[k]` además dice si la
> clave existe.

## Recorrer un map

`range` recorre todas las entradas de un map, dando la clave y el valor.

```go norun
for nombre, edad := range edades {
	fmt.Println(nombre, edad)
}
```

Pero cuidado: **el orden es aleatorio**. Go lo cambia a propósito de una ejecución a
otra, para que nadie dependa de él. El mismo programa puede mostrar Alicia y luego Bob,
y Bob y luego Alicia en la siguiente ejecución.

### Ordenar las claves antes de mostrar

Para una salida estable, recoge las claves, ordénalas y luego recórrelas en orden.

```go
package main

import (
	"fmt"
	"slices"
)

func main() {
	edades := map[string]int{"Zoe": 8, "Alicia": 12, "Marcos": 10}

	claves := make([]string, 0, len(edades))
	for nombre := range edades {
		claves = append(claves, nombre)
	}
	slices.Sort(claves)

	for _, nombre := range claves {
		fmt.Println(nombre, edades[nombre])
	}
}
```

```sortie
Alicia 12
Marcos 10
Zoe 8
```

### Más corto: maps.Keys y slices.Sorted

Desde Go 1.23, el paquete `maps` ofrece `maps.Keys`, que da las claves una a una.
`slices.Sorted` las reúne en una slice ordenada. Los tres pasos caben en una línea.

```go
package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	edades := map[string]int{"Zoe": 8, "Alicia": 12, "Marcos": 10}
	for _, nombre := range slices.Sorted(maps.Keys(edades)) {
		fmt.Println(nombre, edades[nombre])
	}
}
```

```sortie
Alicia 12
Marcos 10
Zoe 8
```

> **Consejo:** acostúmbrate: en cuanto **muestres** un map, ordena sus claves. Tus
> resultados y tus tests serán siempre los mismos.

## Contar palabras

Contar es el uso más clásico de un map. Como una clave que no está vale 0,
`cuenta[palabra]++` funciona incluso la primera vez.

```go
package main

import (
	"fmt"
	"maps"
	"slices"
	"strings"
)

func main() {
	texto := "el gato y el perro y el pez"
	cuenta := make(map[string]int)
	for _, palabra := range strings.Fields(texto) {
		cuenta[palabra]++
	}

	for _, palabra := range slices.Sorted(maps.Keys(cuenta)) {
		fmt.Printf("%-8s %d\n", palabra, cuenta[palabra])
	}
}
```

```sortie
el       3
gato     1
perro    1
pez      1
y        2
```

## Valores más ricos

Un valor de map puede ser de cualquier tipo: una slice, otro map, una estructura…

### Un map de slices

Para guardar varios valores bajo una misma clave, usa una slice como valor. `append`
sobre una clave que no está parte de la slice vacía (`nil`): no hace falta crearla.

```go
package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	alumnos := []string{"Alicia", "Bob", "Clara", "David", "Emma"}
	clases := []string{"6A", "6B", "6A", "6B", "6A"}

	porClase := make(map[string][]string)
	for i, nombre := range alumnos {
		porClase[clases[i]] = append(porClase[clases[i]], nombre)
	}

	for _, clase := range slices.Sorted(maps.Keys(porClase)) {
		fmt.Println(clase, porClase[clase])
	}
}
```

```sortie
6A [Alicia Clara Emma]
6B [Bob David]
```

### Un map de estructuras

Una estructura agrupa varios campos (ver [los tipos](types.md)). Es un buen valor de
map, como una ficha en un fichero.

```go
package main

import "fmt"

type Producto struct {
	Precio float64
	Stock  int
}

func main() {
	catalogo := map[string]Producto{
		"boli":     {Precio: 1.5, Stock: 40},
		"cuaderno":  {Precio: 3.2, Stock: 12},
	}
	fmt.Printf("%+v\n", catalogo["cuaderno"])

	p := catalogo["boli"]
	p.Stock -= 5
	catalogo["boli"] = p
	fmt.Println(catalogo["boli"].Stock)
}
```

```sortie
{Precio:3.2 Stock:12}
35
```

> **Atención:** `catalogo["boli"].Stock -= 5` lo rechaza el compilador. Hay que sacar
> la estructura, modificarla y luego volver a ponerla en el map.

## Errores frecuentes

### Escribir en un map nil

```go norun
package main

import "fmt"

func main() {
	var stock map[string]int
	stock["manzanas"] = 5
	fmt.Println(stock)
}
```

```text
panic: assignment to entry in nil map

goroutine 1 [running]:
main.main()
	/tmp/tmp.TQHY4cxvk4/main.go:7 +0x28
exit status 2
```

`var stock map[string]int` no crea ningún map, solo una variable `nil`. Escribe
`stock := make(map[string]int)`.

### Declarar ok sin usarlo

```go norun
package main

import "fmt"

func main() {
	edades := map[string]int{"Alicia": 30}
	edad, ok := edades["Bob"]
	fmt.Println(edad)
}
```

```text
# command-line-arguments
./main.go:7:8: declared and not used: ok
```

Si pides `ok`, compruébalo: `if !ok { ... }`. Si no, escribe simplemente
`edad := edades["Bob"]`.

### Usar una slice como clave

```go norun
package main

import "fmt"

func main() {
	m := map[[]int]string{}
	fmt.Println(m)
}
```

```text
# command-line-arguments
./main.go:6:11: invalid map key type []int
```

Una clave debe poder compararse con `==`: texto, número, booleano, array o estructura
simple. Una slice no puede. Usa un array (`[2]int`) o un texto.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este programa?

```go norun
package main

import "fmt"

func main() {
	m := map[string]int{"a": 1}
	m["b"] += 5
	m["a"] = 10
	fmt.Println(m["a"], m["b"], len(m))
}
```

> **Respuesta**
>
> `10 5 2`. `m["b"]` valía 0 (clave que no está), luego 5; `m["a"]` se reemplaza por
> 10. El map tiene dos claves.

**Pregunta 2:** cuenta las letras de la palabra `"banana"` y muéstralas en orden
alfabético.

> **Respuesta**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"maps"
> 	"slices"
> )
>
> func main() {
> 	cuenta := make(map[rune]int)
> 	for _, c := range "banana" {
> 		cuenta[c]++
> 	}
> 	for _, c := range slices.Sorted(maps.Keys(cuenta)) {
> 		fmt.Printf("%c: %d\n", c, cuenta[c])
> 	}
> }
> ```
>
> ```sortie
> a: 3
> b: 1
> n: 2
> ```

**Pregunta 3:** ¿por qué hay que ordenar las claves antes de mostrar un map?

> **Respuesta**
>
> Porque Go recorre un map en un orden aleatorio, distinto en cada ejecución. Ordenar
> las claves da una salida siempre idéntica.

## En resumen

- Un map asocia claves con valores: `map[string]int`.
- Créalo con `make(map[K]V)` o `map[K]V{...}`; un map `nil` no acepta nada.
- `m[k] = v` añade o reemplaza, `delete(m, k)` borra, `len(m)` cuenta.
- Una clave que no está da el valor cero; `v, ok := m[k]` dice si existe.
- El orden de `range` es aleatorio: ordena con `slices.Sorted(maps.Keys(m))`.

Siguiente: [las trampas de Go para principiantes](pieges.md)
