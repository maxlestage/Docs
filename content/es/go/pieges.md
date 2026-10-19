# Las trampas del principiante en Go

Esta página reúne once trampas en las que caen casi todos los principiantes en Go, con, para cada una, lo que pasa, por qué y cómo evitarla.

> **Objetivos:**
>
> - reconocer los mensajes de error más habituales de Go;
> - entender por qué un programa que compila a veces da un resultado incorrecto;
> - saber corregir cada trampa en una o dos líneas.

## Cómo leer esta página

Cada trampa sigue el mismo camino:

```text
 Lo que piensas  →  Lo que pasa  →  Por qué  →  La corrección
```

Hay dos tipos de trampas:

- las que el [compilador](../debuter/glossaire.md#compilador) rechaza: aparece un
  mensaje de error, el programa no arranca;
- las que compilan, pero dan un **resultado incorrecto**. Son las más traicioneras.

No hace falta leerlo todo de golpe. Vuelve aquí cuando un programa te sorprenda.

## Las trampas del compilador

### 1. Una variable o un import que no sirve para nada

**Piensas:** «Preparo una variable, ya la usaré más tarde».

**Lo que pasa:** Go se niega a compilar.

```go norun
package main

import (
	"fmt"
	"strings"
)

func main() {
	nombre := "Alice"
	edad := 12
	fmt.Println(nombre)
}
```

```text
# command-line-arguments
./main.go:5:2: "strings" imported and not used
./main.go:10:2: declared and not used: edad
```

**Por qué:** en Go, todo lo que se crea tiene que servir. Una
[variable](../debuter/glossaire.md#variable) olvidada a menudo esconde una errata. Go
prefiere avisarte enseguida.

**La corrección:** borra lo que no sirve. O úsalo.

```go
package main

import "fmt"

func main() {
	nombre := "Alice"
	edad := 12
	fmt.Println(nombre, edad)
}
```

```sortie
Alice 12
```

> **Consejo:** para guardar un valor sin usarlo, dáselo a `_` (el «cajón de la
> basura»): `_ = edad`. Es práctico mientras pruebas algo, pero quítalo después.

### 2. Confundir := y =

**Piensas:** «`:=` y `=` hacen lo mismo».

**Lo que pasa:** nada de eso.

- `:=` **crea** una variable nueva. Es pegar una etiqueta en una caja nueva.
- `=` **cambia** el contenido de una caja que ya existe.

Primer caso: `:=` dos veces para el mismo nombre.

```go norun
package main

import "fmt"

func main() {
	total := 0
	total := 10
	fmt.Println(total)
}
```

```text
# command-line-arguments
./main.go:7:8: no new variables on left side of :=
```

El mensaje quiere decir: «a la izquierda de `:=` no hay nada nuevo que crear».

Segundo caso: `=` para una variable que todavía no existe.

```go norun
package main

import "fmt"

func main() {
	puntos = 5
	fmt.Println(puntos)
}
```

```text
# command-line-arguments
./main.go:6:2: undefined: puntos
./main.go:7:14: undefined: puntos
```

`undefined` quiere decir «no conozco este nombre».

**La corrección:** `:=` la primera vez, `=` las siguientes.

```go
package main

import "fmt"

func main() {
	puntos := 5
	puntos = puntos + 1
	fmt.Println(puntos)
}
```

```sortie
6
```

> **Recuerda:** `:=` = caja nueva. `=` = cambiar lo que hay dentro de la caja.

### 3. Cambiar una letra de un texto

**Piensas:** «Un texto es una serie de letras, puedo cambiar una».

```go norun
package main

import "fmt"

func main() {
	palabra := "café"
	palabra[0] = 'C'
	fmt.Println(palabra)
}
```

```text
# command-line-arguments
./main.go:7:2: cannot assign to palabra[0] (neither addressable nor a map index expression)
```

**Por qué:** en Go, un texto (`string`) nunca se modifica. Se dice que es
[inmutable](../debuter/glossaire.md#inmutable).

**La corrección:** fabrica un texto **nuevo**.

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	palabra := "café"
	palabra = "C" + palabra[1:]
	fmt.Println(palabra)
	fmt.Println(strings.ToUpper("café"))
}
```

```sortie
Café
CAFÉ
```

## Las trampas del cálculo

### 4. La división entera

**Piensas:** «7 dividido entre 2 da 3,5».

```go
package main

import "fmt"

func main() {
	fmt.Println(7 / 2)
	nota1, nota2 := 12, 15
	media := (nota1 + nota2) / 2
	fmt.Println(media)
}
```

```sortie
3
13
```

**Por qué:** entre dos [enteros](../debuter/glossaire.md#entero), `/` da un entero. Go
**corta** la parte después de la coma (no redondea). 27 / 2 da 13, no 13,5.

**La corrección:** convierte a `float64` **antes** de dividir.

```go
package main

import "fmt"

func main() {
	nota1, nota2 := 12, 15
	media := float64(nota1+nota2) / 2
	fmt.Println(media)
}
```

```sortie
13.5
```

Prueba: escribe `float64((nota1 + nota2) / 2)`. Obtienes `13`. La división ocurrió
**antes** de la conversión: demasiado tarde, la parte 0,5 ya se ha perdido.

Mira [los números](nombres.md#la-division-entera-y-el-resto).

### 5. Ignorar el error

**Piensas:** «La conversión del texto a número va a salir bien».

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, _ := strconv.Atoi("12a")
	fmt.Println(n * 2)
}
```

```sortie
0
```

**Por qué:** `"12a"` no es un número. `strconv.Atoi` devuelve entonces `0` **y** un
[error](../debuter/glossaire.md#error). Con `_`, has tirado el error a la basura. El
programa sigue con un `0` falso, sin decir nada.

**La corrección:** comprueba siempre `err`.

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("12a")
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	fmt.Println(n * 2)
}
```

```sortie
Error: strconv.Atoi: parsing "12a": invalid syntax
```

> **Recuerda:** cuando una función devuelve `err`, la línea siguiente es casi siempre
> `if err != nil { ... }`.

## Las trampas de los bucles y los bloques

### 6. Una variable que esconde a otra

**Piensas:** «Cambio `mensaje` dentro del `if`».

```go
package main

import "fmt"

func main() {
	edad := 20
	mensaje := "menor"
	if edad >= 18 {
		mensaje := "mayor"
		fmt.Println("dentro del if:", mensaje)
	}
	fmt.Println("después del if:", mensaje)
}
```

```sortie
dentro del if: mayor
después del if: menor
```

**Por qué:** dentro del `if`, `mensaje :=` crea una **segunda** variable, con el mismo
nombre. Esconde a la primera y desaparece al final del bloque `}`. Se habla de
**ocultación** (*shadowing*). La zona donde existe un nombre se llama su
[ámbito](../debuter/glossaire.md#ambito).

```text
main  ┌─────────────────────────────┐
      │ mensaje = "menor"           │
      │   if  ┌───────────────────┐ │
      │       │ mensaje = "mayor" │ │  ← otra caja
      │       └───────────────────┘ │  ← desaparece aquí
      │ mensaje sigue valiendo "menor"
      └─────────────────────────────┘
```

**La corrección:** un solo carácter. Sustituye `:=` por `=` dentro del `if`.

```go
package main

import "fmt"

func main() {
	edad := 20
	mensaje := "menor"
	if edad >= 18 {
		mensaje = "mayor"
	}
	fmt.Println(mensaje)
}
```

```sortie
mayor
```

### 7. range da una copia

**Piensas:** «Multiplico cada número de la slice por 10».

```go
package main

import "fmt"

func main() {
	numeros := []int{1, 2, 3}
	for _, n := range numeros {
		n = n * 10
	}
	fmt.Println(numeros)
}
```

```sortie
[1 2 3]
```

**Por qué:** `n` es una **copia** de cada casilla, como una fotocopia. Escribes en la
fotocopia, no en el original.

| Vuelta | n antes | n después | numeros |
|---|---|---|---|
| 1 | 1 | 10 | [1 2 3] |
| 2 | 2 | 20 | [1 2 3] |
| 3 | 3 | 30 | [1 2 3] |

**La corrección:** pasa por la posición `i` para escribir en la casilla de verdad.

```go
package main

import "fmt"

func main() {
	numeros := []int{1, 2, 3}
	for i := range numeros {
		numeros[i] = numeros[i] * 10
	}
	fmt.Println(numeros)
}
```

```sortie
[10 20 30]
```

## Las trampas de las slices, los maps y los textos

### 8. Dos slices que comparten las mismas casillas

**Piensas:** «`inicio` es una slice nueva. Añadirle un elemento no toca `base`».

```go
package main

import "fmt"

func main() {
	base := []int{1, 2, 3, 4}
	inicio := base[:2]
	inicio = append(inicio, 99)
	fmt.Println("inicio:", inicio)
	fmt.Println("base:  ", base)
}
```

```sortie
inicio: [1 2 99]
base:   [1 2 99 4]
```

**Por qué:** una [slice](../debuter/glossaire.md#slice) es una **ventana** sobre un
array escondido. `base[:2]` no copia nada: es una ventanita sobre las mismas casillas.
Como queda sitio detrás, `append` escribe 99 en la casilla 2… que también pertenece a
`base`.

```text
array escondido : [ 1 | 2 | 3 | 4 ]
base            :  └───────────────┘
inicio          :  └─────┘
tras el append  : [ 1 | 2 | 99| 4 ]   ← la casilla 2 queda machacada
```

**La corrección:** haz una copia de verdad con `slices.Clone`.

```go
package main

import (
	"fmt"
	"slices"
)

func main() {
	base := []int{1, 2, 3, 4}
	inicio := slices.Clone(base[:2])
	inicio = append(inicio, 99)
	fmt.Println("inicio:", inicio)
	fmt.Println("base:  ", base)
}
```

```sortie
inicio: [1 2 99]
base:   [1 2 3 4]
```

Más detalles en [la trampa del array compartido](collections.md#la-trampa-del-array-compartido).

### 9. Escribir en un map nil

**Piensas:** «He declarado mi map, puedo llenarlo».

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
	/ruta/a/main.go:7 +0x28
exit status 2
```

**Por qué:** `var stock map[string]int` crea una etiqueta, pero no una caja. El map
vale `nil` («nada»). Escribir en él provoca un [pánico](../debuter/glossaire.md#panico):
el programa se para en seco.

**La corrección:** crea el map con `make` (o con `{}`).

```go
package main

import "fmt"

func main() {
	stock := make(map[string]int)
	stock["manzanas"] = 5
	fmt.Println(stock)
}
```

```sortie
map[manzanas:5]
```

### 10. El orden de un map cambia cada vez

**Piensas:** «El map se mostrará en el orden en que puse las claves».

```go norun
package main

import "fmt"

func main() {
	edades := map[string]int{"Zoé": 8, "Alice": 12, "Marc": 10}
	for nombre, edad := range edades {
		fmt.Println(nombre, edad)
	}
}
```

Ejecuta este programa varias veces. El orden cambia:

```text
1.ª ejecución     2.ª ejecución
Marc 10           Alice 12
Zoé 8             Zoé 8
Alice 12          Marc 10
```

**Por qué:** Go mezcla **a propósito** el orden de `range` sobre un map. Así nadie
cuenta con él por error.

**La corrección:** ordena las claves y luego recórrelas.

```go
package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	edades := map[string]int{"Zoé": 8, "Alice": 12, "Marc": 10}
	for _, nombre := range slices.Sorted(maps.Keys(edades)) {
		fmt.Println(nombre, edades[nombre])
	}
}
```

```sortie
Alice 12
Marc 10
Zoé 8
```

> **Nota:** `fmt.Println(edades)` ya muestra las claves ordenadas. La trampa solo
> afecta a `range`.

### 11. Un texto se mide en bytes, no en letras

**Piensas:** «`"época"` tiene 5 letras, así que `len` vale 5 y `palabra[0]` vale `é`».

```go
package main

import "fmt"

func main() {
	palabra := "época"
	fmt.Println(len(palabra))
	fmt.Println(palabra[0])
	fmt.Println(string(palabra[0]))
}
```

```sortie
6
195
Ã
```

**Por qué:** Go guarda el texto en [bytes](../debuter/glossaire.md#byte) (en UTF-8,
mira [Unicode](../debuter/glossaire.md#unicode)). Una letra sencilla como `p` ocupa 1
byte. Una letra con tilde como `é` ocupa **2**. `palabra[0]` da el primer **byte**, no
la primera letra.

```text
letras :  é       p    o    c   a
bytes  : [195 169][112][111][99][97]   → 6 bytes
```

**La corrección:** pasa por las **runas** (una runa = una letra).

```go
package main

import "fmt"

func main() {
	palabra := "época"
	letras := []rune(palabra)
	fmt.Println(len(letras))
	fmt.Println(string(letras[0]))
	for _, r := range palabra {
		fmt.Print(string(r), " ")
	}
	fmt.Println()
}
```

```sortie
5
é
é p o c a 
```

`range` sobre un texto sí da las letras una a una. Mira
[bytes y runas](texte.md#bytes-y-runas).

> **Recuerda:** `len(texto)` cuenta bytes. `len([]rune(texto))` cuenta letras.

## Errores frecuentes

Toda la página habla de errores. Aquí tienes dos mensajes más que verás a menudo.

### Leer una casilla que no existe

```go norun
package main

import "fmt"

func main() {
	notas := []int{12, 8, 15}
	fmt.Println(notas[3])
}
```

```text
panic: runtime error: index out of range [3] with length 3

goroutine 1 [running]:
main.main()
	/ruta/a/main.go:7 +0x15
exit status 2
```

Tres casillas, numeradas 0, 1 y 2. La casilla 3 no existe. La última casilla es
siempre `notas[len(notas)-1]`.

### Mezclar un int y un float64

```go norun
package main

import "fmt"

func main() {
	suma := 27.0
	n := 2
	fmt.Println(suma / n)
}
```

```text
# command-line-arguments
./main.go:8:14: invalid operation: suma / n (mismatched types float64 and int)
```

Go nunca mezcla dos tipos de números por su cuenta. Convierte tú mismo:
`suma / float64(n)`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este programa?

```go norun
package main

import "fmt"

func main() {
	total := 0
	for _, n := range []int{1, 2, 3} {
		total := total + n
		fmt.Print(total, " ")
	}
	fmt.Println(total)
}
```

> **Respuesta**
>
> `1 2 3 0`. Dentro del bucle, `total :=` crea una variable nueva en cada vuelta
> (trampa 6). Vale `0 + n`. El `total` de fuera no ha cambiado nunca: sigue valiendo 0.
> Corrección: `total = total + n`, y el programa muestra `1 3 6 6`.

**Pregunta 2:** ¿por qué `fmt.Println(9 / 4)` muestra `2`?

> **Respuesta**
>
> 9 y 4 son enteros: la división es entera, la parte después de la coma se corta.
> Para obtener `2.25`, escribe `9.0 / 4` o `float64(9) / 4`.

**Pregunta 3:** ¿cuánto vale `len("niño")`?

> **Respuesta**
>
> `5`. n, i y o ocupan 1 byte cada una, `ñ` ocupa 2. Para contar las letras:
> `len([]rune("niño"))`, que vale 4.

## En resumen

- Go rechaza las variables y los imports sin usar: bórralos.
- `:=` crea una variable, `=` la cambia; un `:=` dentro de un bloque puede esconder una
  variable.
- Entre dos enteros, `/` corta los decimales: convierte a `float64` antes.
- Comprueba siempre `err`; `range` da una copia; un map se crea con `make`.
- Dos slices pueden compartir sus casillas; el orden de un map es aleatorio.
- Un texto se mide en bytes: usa `[]rune` para contar las letras.

Siguiente: [ejercicios para empezar bien](exercices-faciles.md).
