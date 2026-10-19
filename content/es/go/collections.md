# Arrays y slices

Esta página presenta las dos maneras de guardar varios valores seguidos en Go: el array, de tamaño fijo, y la slice, que puede crecer.

> **Objetivos:**
>
> - sabrás crear un array y una slice, y leer sus elementos;
> - sabrás añadir elementos con `append`;
> - sabrás recortar una slice con `s[inicio:fin]` y copiarla con `copy`;
> - sabrás evitar la trampa del array compartido;
> - sabrás ordenar y buscar con los paquetes `slices` y `sort`.

## Los arrays

Un array (*arreglo*) guarda un número **fijo** de valores del mismo tipo. El tamaño
forma parte del tipo: `[3]int` es «un array de 3 enteros».

```go
package main

import "fmt"

func main() {
	var notas [3]int
	notas[0] = 12
	notas[2] = 18
	fmt.Println(notas)

	dias := [3]string{"lun", "mar", "mié"}
	fmt.Println(dias[1], len(dias))
}
```

```sortie
[12 0 18]
mar 3
```

- Las casillas se numeran a partir de **0**: es el [índice](../debuter/glossaire.md#indice).
- Una casilla sin rellenar tiene el valor cero de su tipo (`0` aquí).
- `len` da el número de casillas.

Un array no puede ni crecer ni encogerse. Por eso se usa poco: casi siempre se
prefiere la slice.

## Las slices

Una slice es una [lista](../debuter/glossaire.md#lista) de valores del mismo tipo, cuyo
tamaño puede cambiar. Su tipo se escribe sin tamaño: `[]int`.

```go
package main

import "fmt"

func main() {
	frutas := []string{"manzana", "kiwi", "pera"}
	fmt.Println(frutas)
	fmt.Println(frutas[0], len(frutas))

	frutas[1] = "mango"
	fmt.Println(frutas)
	fmt.Println(frutas[len(frutas)-1])
}
```

```sortie
[manzana kiwi pera]
manzana 3
[manzana mango pera]
pera
```

`frutas[len(frutas)-1]` es el último elemento: con 3 elementos, los índices van de 0
a 2.

### Crear una slice con make

`make([]tipo, longitud)` crea una slice llena de valores cero.

```go
package main

import "fmt"

func main() {
	puntos := make([]int, 4)
	puntos[2] = 7
	fmt.Println(puntos, len(puntos))

	var vacia []string
	fmt.Println(vacia, len(vacia), vacia == nil)
}
```

```sortie
[0 0 7 0] 4
[] 0 true
```

Una slice declarada con `var` sin valor vale `nil`: está vacía, y aun así puedes
añadirle elementos.

## Añadir con append

`append` añade elementos al final de una slice y **devuelve** la nueva slice. Siempre
hay que recoger ese resultado.

```go
package main

import "fmt"

func main() {
	var compras []string
	compras = append(compras, "pan")
	compras = append(compras, "leche", "huevos")
	fmt.Println(compras, len(compras))

	otras := []string{"arroz", "té"}
	compras = append(compras, otras...)
	fmt.Println(compras)
}
```

```sortie
[pan leche huevos] 3
[pan leche huevos arroz té]
```

`otras...` despliega la slice `otras` en varios argumentos.

### La capacidad

Detrás de cada slice se esconde un array, que tiene cierto espacio: la **capacidad**
(`cap`). Cuando falta espacio, `append` crea un array más grande y lo copia todo en él.
Por eso hay que recoger el resultado de `append`: puede que ya no sea el mismo array.

```go
package main

import "fmt"

func main() {
	s := make([]int, 0, 3)
	for i := range 5 {
		s = append(s, i)
		fmt.Println(len(s), cap(s))
	}
}
```

```sortie
1 3
2 3
3 3
4 6
5 6
```

- `len`: el número de elementos presentes;
- `cap`: el número de huecos reservados.

Imagina una huevera: `len` cuenta los huevos, `cap` cuenta los huecos. Cuando la
huevera está llena, Go toma una más grande.

> **Consejo:** si sabes de antemano cuántos elementos vas a añadir, reserva el espacio
> con `make([]int, 0, n)`. Go no tendrá que copiar por el camino.

## Recortar una slice

`s[inicio:fin]` toma los elementos desde el índice `inicio` **incluido** hasta el
índice `fin` **excluido**.

```go
package main

import "fmt"

func main() {
	letras := []string{"a", "b", "c", "d", "e"}
	fmt.Println(letras[1:3])
	fmt.Println(letras[:2])
	fmt.Println(letras[3:])
	fmt.Println(letras[:])
}
```

```sortie
[b c]
[a b]
[d e]
[a b c d e]
```

- `[:2]`: desde el principio hasta 2 (excluido);
- `[3:]`: desde 3 hasta el final;
- `[:]`: todo.

> **Recuerda:** `s[1:3]` contiene `3 - 1 = 2` elementos. El final siempre se excluye.

## La trampa del array compartido

Un recorte no copia **nada**: mira el mismo array escondido, por otra ventana.
Modificar uno modifica el otro.

```go
package main

import "fmt"

func main() {
	numeros := []int{1, 2, 3, 4}
	inicio := numeros[:2]
	inicio[0] = 99
	fmt.Println(numeros)
	fmt.Println(inicio)
}
```

```sortie
[99 2 3 4]
[99 2]
```

`inicio` y `numeros` comparten las mismas casillas: cambiar `inicio[0]` ha cambiado
`numeros[0]`.

Pasa lo mismo cuando **asignas** una slice a otra variable, o cuando la pasas a una
función: los dos nombres designan las mismas casillas.

```go
package main

import "fmt"

func ponerACero(s []int) {
	for i := range s {
		s[i] = 0
	}
}

func main() {
	a := []int{5, 6, 7}
	b := a
	b[0] = 100
	fmt.Println(a)

	ponerACero(a)
	fmt.Println(a)
}
```

```sortie
[100 6 7]
[0 0 0]
```

### Hacer una copia de verdad con copy

Para obtener una slice independiente, crea una nueva y copia en ella los elementos con
`copy(destino, origen)`.

```go
package main

import "fmt"

func main() {
	original := []int{1, 2, 3}
	copia := make([]int, len(original))
	copy(copia, original)
	copia[0] = 99
	fmt.Println(original)
	fmt.Println(copia)
}
```

```sortie
[1 2 3]
[99 2 3]
```

> **Consejo:** desde Go 1.21, `slices.Clone(original)` hace lo mismo en una línea.

## Las slices de dos dimensiones

Una slice puede contener slices: es una cuadrícula, como una hoja de cálculo o un
tablero de juego.

```go
package main

import "fmt"

func main() {
	cuadricula := [][]int{
		{1, 2, 3},
		{4, 5, 6},
	}
	fmt.Println(cuadricula[1][2])

	for i, fila := range cuadricula {
		fmt.Println("fila", i, ":", fila)
	}

	tablero := make([][]string, 2)
	for i := range tablero {
		tablero[i] = make([]string, 3)
	}
	tablero[0][1] = "X"
	fmt.Printf("%q\n", tablero)
}
```

```sortie
6
fila 0 : [1 2 3]
fila 1 : [4 5 6]
[["" "X" ""] ["" "" ""]]
```

`cuadricula[1][2]` se lee: fila 1, columna 2. Con `make`, hay que crear **cada** fila,
una por una. El verbo `%q` muestra los textos entre comillas: así se ven las casillas
vacías `""`.

## Ordenar y buscar

### El paquete slices

Desde Go 1.21, el paquete `slices` reúne las operaciones habituales.

```go
package main

import (
	"fmt"
	"slices"
)

func main() {
	notas := []int{14, 9, 17, 12}
	fmt.Println(slices.Contains(notas, 9))
	fmt.Println(slices.Index(notas, 17))
	fmt.Println(slices.Max(notas), slices.Min(notas))

	slices.Sort(notas)
	fmt.Println(notas)

	nombres := []string{"Zoe", "Alicia", "Marcos"}
	slices.Sort(nombres)
	fmt.Println(nombres)
}
```

```sortie
true
2
17 9
[9 12 14 17]
[Alicia Marcos Zoe]
```

| Función | Papel |
|----------|------|
| `slices.Contains(s, v)` | ¿contiene `s` a `v`? |
| `slices.Index(s, v)` | la posición de `v` (`-1` si no está) |
| `slices.Max(s)`, `slices.Min(s)` | el más grande, el más pequeño |
| `slices.Sort(s)` | ordena `s` en su sitio, en orden creciente |
| `slices.Reverse(s)` | invierte el orden de `s` |

> **Nota:** `slices.Sort` modifica la propia slice, y no devuelve nada.

### Ordenar según una regla: sort.Slice

Para ordenar según tu propia regla, `sort.Slice` recibe una función que responde a la
pregunta: «¿el elemento `i` debe ir antes que el elemento `j`?».

```go
package main

import (
	"fmt"
	"sort"
)

func main() {
	palabras := []string{"mandarina", "kiwi", "ciruela", "mango"}
	sort.Slice(palabras, func(i, j int) bool {
		return len(palabras[i]) < len(palabras[j])
	})
	fmt.Println(palabras)

	notas := []int{12, 18, 9}
	sort.Slice(notas, func(i, j int) bool {
		return notas[i] > notas[j]
	})
	fmt.Println(notas)
}
```

```sortie
[kiwi mango ciruela mandarina]
[18 12 9]
```

La primera ordenación coloca las palabras de la más corta a la más larga, la segunda
las notas de la más alta a la más baja.

> **Consejo:** `sort.SliceStable` hace la misma ordenación, pero conserva el orden de
> partida de los elementos iguales. Úsalo si dos palabras de la misma longitud deben
> quedarse en su orden original.

## Errores frecuentes

### Leer después del final

```go norun
package main

import "fmt"

func main() {
	notas := []int{12, 15, 9}
	fmt.Println(notas[3])
}
```

```text
panic: runtime error: index out of range [3] with length 3

goroutine 1 [running]:
main.main()
	/tmp/tmp.XlL8TvM6jN/main.go:7 +0x15
exit status 2
```

Con 3 elementos, los índices van de 0 a 2. El índice 3 está fuera de los límites
(*out of range*). El último elemento es `notas[len(notas)-1]`.

### Olvidar recoger el resultado de append

```go norun
package main

import "fmt"

func main() {
	notas := []int{12, 15}
	append(notas, 9)
	fmt.Println(notas)
}
```

```text
# command-line-arguments
./main.go:7:2: append(notas, 9) (value of type []int) is not used
```

`append` no modifica `notas`: devuelve una nueva slice. Escribe
`notas = append(notas, 9)`.

### Comparar dos slices con ==

```go norun
package main

import "fmt"

func main() {
	a := []int{1, 2}
	b := []int{1, 2}
	fmt.Println(a == b)
}
```

```text
# command-line-arguments
./main.go:8:14: invalid operation: a == b (slice can only be compared to nil)
```

Una slice solo se compara con `nil`. Para comparar el contenido, usa
`slices.Equal(a, b)`.

### Demasiados valores para un array

```go norun
package main

import "fmt"

func main() {
	dias := [3]string{"lun", "mar", "mié", "jue"}
	fmt.Println(dias)
}
```

```text
# command-line-arguments
./main.go:6:42: index 3 is out of bounds (>= 3)
```

El array tiene 3 casillas, no 4. Agrándalo (`[4]string`), o usa una slice
(`[]string{...}`), que se adapta sola.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este programa?

```go norun
package main

import "fmt"

func main() {
	s := []int{10, 20, 30, 40, 50}
	fmt.Println(s[1:4], len(s[2:]))
}
```

> **Respuesta**
>
> `[20 30 40] 3`. `s[1:4]` va del índice 1 al 3 incluido; `s[2:]` contiene 30, 40 y 50.

**Pregunta 2:** a partir de `[]int{5, 3, 8, 1}`, crea una copia ordenada, sin cambiar
la original, y luego muestra las dos.

> **Respuesta**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"slices"
> )
>
> func main() {
> 	original := []int{5, 3, 8, 1}
> 	ordenada := slices.Clone(original)
> 	slices.Sort(ordenada)
> 	fmt.Println(original, ordenada)
> }
> ```
>
> ```sortie
> [5 3 8 1] [1 3 5 8]
> ```

**Pregunta 3:** después de `b := a[:2]` y luego `b[0] = 0`, ¿ha cambiado `a`?

> **Respuesta**
>
> Sí. Un recorte no copia los elementos: `a` y `b` comparten el mismo array escondido.
> Para una copia independiente, usa `copy` o `slices.Clone`.

## En resumen

- Un array `[3]int` tiene un tamaño fijo; una slice `[]int` puede crecer.
- `append` añade al final y devuelve la slice: escribe `s = append(s, v)`.
- `s[inicio:fin]` recorta, sin incluir el final y sin copiar nada: las dos slices
  comparten sus casillas.
- `copy` o `slices.Clone` hacen una copia de verdad.
- El paquete `slices` ordena, busca y compara; `sort.Slice` ordena según tu regla.

Siguiente: [los maps](maps.md). Compara con
[las colecciones en Rust](../rust/collections.md).
