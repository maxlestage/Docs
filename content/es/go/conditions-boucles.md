# Condiciones y bucles

Esta página muestra cómo tomar decisiones con `if` y `switch`, y cómo repetir acciones con `for`, el único bucle de Go.

> **Objetivos:**
>
> - sabrás escribir una condición con `if`, `else if` y `else`;
> - sabrás elegir entre varios casos con `switch`;
> - sabrás escribir las tres formas del bucle `for`;
> - sabrás recorrer una lista, un texto o números con `range`;
> - sabrás detener o saltar una vuelta con `break` y `continue`.

## Comparar valores

Un programa a menudo tiene que decidir: abrir la puerta o no, mostrar «ganaste» o
«perdiste». Para decidir, primero hace una pregunta, igual que tú te preguntas
«¿tengo suficiente dinero para este pastel?».

Una [condición](../debuter/glossaire.md#condicion) es una pregunta que se
responde con verdadero (`true`) o falso (`false`). Se construye con operadores de
comparación.

| Operador | Significado | Ejemplo | Resultado |
|-----------|------|---------|----------|
| `==` | igual a | `3 == 3` | `true` |
| `!=` | distinto de | `3 != 4` | `true` |
| `<`, `>` | menor, mayor | `2 > 5` | `false` |
| `<=`, `>=` | menor o igual, mayor o igual | `5 >= 5` | `true` |

Las condiciones se combinan con los operadores lógicos:

- `&&`: «y» (las dos deben ser verdaderas);
- `||`: «o» (al menos una debe ser verdadera);
- `!`: «no» (invierte verdadero y falso).

```go
package main

import "fmt"

func main() {
	edad := 15
	conPadres := true
	fmt.Println(edad >= 18)
	fmt.Println(edad >= 18 || conPadres)
	fmt.Println(edad > 12 && edad < 18)
	fmt.Println(!conPadres)
}
```

```sortie
false
true
true
false
```

Línea por línea:

- `edad := 15` guarda 15 en `edad`, y `conPadres := true` guarda «verdadero» en
  `conPadres`;
- `edad >= 18` pregunta «¿15 es mayor o igual que 18?». No: `false`;
- con `||`, basta una respuesta verdadera: `conPadres` vale `true`, así que `true`;
- con `&&`, hacen falta las dos: 15 es mayor que 12 **y** menor que 18, así que
  `true`;
- `!conPadres` invierte `true`: se obtiene `false`.

> **Atención:** `=` guarda un valor, `==` compara dos valores. No los confundas.

## if y else

En la vida tomas decisiones todo el tiempo: si llueve, coges un paraguas; si no, lo
dejas en casa. `if` le da ese poder a tu programa: hacer una cosa u otra según la
situación.

`if` («si») ejecuta un bloque solo si la condición es verdadera. `else` («si no»)
da el bloque que se ejecuta en caso contrario.

```go
package main

import "fmt"

func main() {
	temperatura := 28
	if temperatura > 25 {
		fmt.Println("Hace calor.")
	} else {
		fmt.Println("Hace buen tiempo.")
	}
}
```

```sortie
Hace calor.
```

Línea por línea:

- `temperatura := 28` guarda 28 en la variable `temperatura`;
- `if temperatura > 25 {` hace la pregunta «¿28 es mayor que 25?». La respuesta
  es sí;
- así que Go ejecuta el primer bloque y muestra «Hace calor.»;
- el bloque del `else` se salta: solo sirve si la respuesta es no.

En Go:

- no hay paréntesis alrededor de la condición;
- las llaves `{ }` son **obligatorias**, incluso para una sola línea;
- `else` se coloca en la misma línea que la llave de cierre: `} else {`.

### else if

Para más de dos casos, encadena con `else if`. Go prueba las condiciones en orden y
se detiene en la primera verdadera.

```go
package main

import "fmt"

func main() {
	nota := 14
	if nota >= 16 {
		fmt.Println("Sobresaliente")
	} else if nota >= 12 {
		fmt.Println("Notable")
	} else if nota >= 10 {
		fmt.Println("Aprobado")
	} else {
		fmt.Println("Suspenso")
	}
}
```

```sortie
Notable
```

`nota >= 10` también es verdadera, pero Go ni siquiera la prueba: ya ha encontrado
`nota >= 12`.

**Te toca:** escribe un programa que guarde `10` en una variable `edad` y luego
muestre «Niño» si la edad es menor que 13, «Adolescente» si es menor que 18, y
«Adulto» en otro caso.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	edad := 10
> 	if edad < 13 {
> 		fmt.Println("Niño")
> 	} else if edad < 18 {
> 		fmt.Println("Adolescente")
> 	} else {
> 		fmt.Println("Adulto")
> 	}
> }
> ```
>
> ```sortie
> Niño
> ```

### if con una instrucción de inicialización

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Go permite colocar una pequeña instrucción **antes** de la condición, separada por un `;`.
La variable creada ahí solo existe dentro del `if` y sus `else`.

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	if n, err := strconv.Atoi("42"); err != nil {
		fmt.Println("No es un número:", err)
	} else {
		fmt.Println("El doble vale", n*2)
	}
}
```

```sortie
El doble vale 84
```

Esta forma es muy habitual en Go, sobre todo para comprobar un error. Mantiene las
variables temporales allí donde se usan.

## switch

Piensa en una máquina de bebidas: según el botón que pulses, sirve una bebida
distinta. `switch` funciona igual: según el valor, elige un bloque.

Cuando un mismo valor puede tomar varias formas, `switch` es más legible que una
larga serie de `else if`.

```go
package main

import "fmt"

func main() {
	dia := "sábado"
	switch dia {
	case "sábado", "domingo":
		fmt.Println("¡Es fin de semana!")
	case "miércoles":
		fmt.Println("Mitad de semana.")
	default:
		fmt.Println("A trabajar.")
	}
}
```

```sortie
¡Es fin de semana!
```

Línea por línea:

- `dia` contiene el texto `"sábado"`;
- `switch dia {` compara el valor de `dia` con cada `case`, de arriba abajo;
- el primer `case` lista `"sábado"` y `"domingo"`: `"sábado"` coincide, así que
  Go muestra «¡Es fin de semana!»;
- los demás casos, y `default`, se ignoran.

Recuerda tres cosas sobre `switch`:

- Cada `case` da uno o varios valores, separados por comas.
- `default` («por defecto») se ejecuta si ningún caso coincide.
- Go se detiene en el **primer** caso que coincide y luego sale del `switch`.

> **Nota:** a diferencia de C o Java, no hace falta `break` al final de cada
> caso: Go no «cae» en el caso siguiente. Para forzarlo, existe la palabra
> `fallthrough`, pero casi nunca se usa.

**Te toca:** escribe un programa que guarde `"limón"` en una variable `fruta` y
luego, con un `switch`, muestre «Fruta de huerto» para una manzana o una pera,
«Cítrico» para un limón o una naranja, y «Fruta desconocida» en otro caso.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	fruta := "limón"
> 	switch fruta {
> 	case "manzana", "pera":
> 		fmt.Println("Fruta de huerto")
> 	case "limón", "naranja":
> 		fmt.Println("Cítrico")
> 	default:
> 		fmt.Println("Fruta desconocida")
> 	}
> }
> ```
>
> ```sortie
> Cítrico
> ```

### switch sin valor

Un `switch` sin valor prueba condiciones, como un `switch true`. Es una forma elegante
de escribir una serie de `else if`.

```go
package main

import "fmt"

func main() {
	hora := 14
	switch {
	case hora < 12:
		fmt.Println("Buenos días")
	case hora < 20:
		fmt.Println("Buenas tardes")
	default:
		fmt.Println("Buenas noches")
	}
}
```

```sortie
Buenas tardes
```

## El bucle for

Imagina que te piden mostrar los números del 1 al 100. Escribir 100 `fmt.Println`
sería largo y pesado. Un bucle hace ese trabajo repetitivo por ti, como el estribillo
de una canción que se repite varias veces.

Un [bucle](../debuter/glossaire.md#bucle) repite un bloque de código. Go solo tiene **un
único** bucle: `for`. No hay `while`, ni `do`. Pero `for` se escribe de tres formas.

### Forma 1: el contador

La forma clásica tiene tres partes, separadas por `;`:

```go
package main

import "fmt"

func main() {
	for i := 1; i <= 3; i++ {
		fmt.Println("Vuelta", i)
	}
}
```

```sortie
Vuelta 1
Vuelta 2
Vuelta 3
```

1. `i := 1`: el **inicio**, se ejecuta una sola vez;
2. `i <= 3`: la **condición**, se prueba antes de cada vuelta; si es falsa, el bucle
   se detiene;
3. `i++`: el **paso**, se ejecuta al final de cada vuelta.

### Forma 2: mientras

Con solo una condición, `for` hace el papel de «mientras» (`while` en otros
lenguajes).

```go
package main

import "fmt"

func main() {
	dinero := 100
	anios := 0
	for dinero < 150 {
		dinero = dinero * 110 / 100
		anios++
	}
	fmt.Println(anios, "años, hay", dinero, "euros")
}
```

```sortie
5 años, hay 160 euros
```

Línea por línea:

- al principio, `dinero` vale 100 y `anios` vale 0;
- `for dinero < 150 {` hace la pregunta antes de cada vuelta. 100 es menor que 150:
  Go entra en el bloque;
- `dinero * 110 / 100` suma un 10 %: 100 pasa a 110, luego 121, 133, 146 y 160 (Go
  calcula con números enteros y descarta los decimales);
- `anios++` suma 1 a `anios` en cada vuelta: cuenta las vueltas;
- cuando `dinero` vale 160, la respuesta es no: el bucle se detiene y la última
  línea muestra el resultado.

No se sabe de antemano cuántas vueltas harán falta: el bucle gira mientras
`dinero < 150`.

**Te toca:** escribe un programa que empiece con `n := 1` y que, mientras `n` sea
menor que 100, muestre `n` y luego lo duplique.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	n := 1
> 	for n < 100 {
> 		fmt.Println(n)
> 		n = n * 2
> 	}
> }
> ```
>
> ```sortie
> 1
> 2
> 4
> 8
> 16
> 32
> 64
> ```

### Forma 3: el bucle infinito

Sin nada en absoluto, `for` gira para siempre. Se sale de él con `break`.

```go
package main

import "fmt"

func main() {
	n := 1
	for {
		n = n * 2
		if n > 50 {
			break
		}
	}
	fmt.Println(n)
}
```

```sortie
64
```

> **Atención:** si la condición nunca se vuelve falsa (o si `break` nunca
> llega), el programa gira sin fin. Pulsa `Ctrl+C` en la terminal para
> detenerlo.

## Recorrer con range

Muy a menudo tienes una lista de cosas que tratar una a una: cada artículo de una
lista de la compra, cada nota de un boletín. `range` te las da en orden, sin que
tengas que contar tú.

`range` recorre los elementos de una colección, uno a uno. En cada vuelta, da
el **índice** (la posición) y el valor.

### Una slice

```go
package main

import "fmt"

func main() {
	frutas := []string{"manzana", "kiwi", "pera"}
	for i, fruta := range frutas {
		fmt.Println(i, fruta)
	}
}
```

```sortie
0 manzana
1 kiwi
2 pera
```

Línea por línea:

- `frutas` contiene una lista de tres textos;
- en la primera vuelta, `range` guarda la posición 0 en `i` y `"manzana"` en
  `fruta`;
- el bloque muestra los dos. Luego Go vuelve al `for`, que da 1 y `"kiwi"`, y
  después 2 y `"pera"`;
- después de `"pera"` no quedan elementos: el bucle se detiene.

`[]string{...}` es una **slice**, una lista de valores: la descubrirás en
[arrays y slices](collections.md). Las posiciones empiezan en 0.

Si no necesitas el índice, reemplázalo por `_`:

```go
package main

import "fmt"

func main() {
	notas := []int{12, 15, 9}
	total := 0
	for _, nota := range notas {
		total += nota
	}
	fmt.Println("Total:", total)
}
```

```sortie
Total: 36
```

**Te toca:** escribe un programa que recorra la lista de la compra
`[]string{"pan", "leche", "huevos"}` y muestre cada artículo precedido de un guion.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	compra := []string{"pan", "leche", "huevos"}
> 	for _, articulo := range compra {
> 		fmt.Println("-", articulo)
> 	}
> }
> ```
>
> ```sortie
> - pan
> - leche
> - huevos
> ```

### Un texto

Sobre una cadena, `range` da la posición (en bytes) y cada carácter, en forma de
`rune`.

```go
package main

import "fmt"

func main() {
	for i, c := range "Go!" {
		fmt.Printf("%d:%c ", i, c)
	}
	fmt.Println()
}
```

```sortie
0:G 1:o 2:! 
```

### Enteros

Desde Go 1.22, `range` acepta un entero: `range 3` da 0, 1 y luego 2. Es lo más
corto para repetir una acción.

```go
package main

import "fmt"

func main() {
	for i := range 3 {
		fmt.Println("Bip", i)
	}
	for range 2 {
		fmt.Println("¡Hurra!")
	}
}
```

```sortie
Bip 0
Bip 1
Bip 2
¡Hurra!
¡Hurra!
```

## break y continue

A veces quieres cambiar el rumbo de un bucle sobre la marcha: dejar de buscar en
cuanto encuentras tus llaves, o ignorar algunos elementos. Para eso sirven estas dos
palabras.

- `break` sale del bucle de inmediato;
- `continue` se salta el final de la vuelta y pasa a la siguiente.

```go
package main

import "fmt"

func main() {
	for i := range 10 {
		if i%2 == 0 {
			continue
		}
		if i > 7 {
			break
		}
		fmt.Println(i)
	}
}
```

```sortie
1
3
5
7
```

Línea por línea:

- `for i := range 10 {` da a `i` los valores 0, 1, 2… hasta 9;
- si `i` es par (el resto de dividir entre 2, `i%2`, vale 0), `continue` pasa
  directamente a la vuelta siguiente: no se llega al `fmt.Println`;
- si `i` es mayor que 7, `break` detiene todo el bucle;
- si no, `fmt.Println(i)` muestra el número.

Los números pares se saltan con `continue`. Cuando `i` vale 9, `break` lo detiene todo.

### Las etiquetas

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Con dos bucles uno dentro del otro, `break` solo sale del bucle interior. Para
salir de los dos, dale un nombre (una **etiqueta**) al bucle exterior.

```go
package main

import "fmt"

func main() {
busqueda:
	for fila := 1; fila <= 3; fila++ {
		for col := 1; col <= 3; col++ {
			if fila*col == 6 {
				fmt.Println("Encontrado:", fila, "x", col)
				break busqueda
			}
		}
	}
	fmt.Println("Fin")
}
```

```sortie
Encontrado: 2 x 3
Fin
```

`break busqueda` sale del bucle llamado `busqueda`. `continue busqueda` también
existe: pasa a la vuelta siguiente del bucle exterior.

## Errores frecuentes

### Poner un número como condición

```go norun
package main

import "fmt"

func main() {
	n := 3
	if n {
		fmt.Println("verdadero")
	}
}
```

```text
# command-line-arguments
./main.go:7:5: non-boolean condition in if statement
```

En Go, una condición debe ser un booleano de verdad. `3` no es ni verdadero ni falso. Escribe la
comparación: `if n != 0 {`.

### Poner else en una línea nueva

```go norun
package main

import "fmt"

func main() {
	edad := 20
	if edad >= 18 {
		fmt.Println("mayor de edad")
	}
	else {
		fmt.Println("menor de edad")
	}
}
```

```text
# command-line-arguments
./main.go:10:2: syntax error: unexpected keyword else, expected }
```

`else` debe ir justo después de la llave de cierre: `} else {`.

### Escribir while

```go norun
package main

import "fmt"

func main() {
	i := 0
	while i < 3 {
		fmt.Println(i)
		i++
	}
}
```

```text
# command-line-arguments
./main.go:7:8: syntax error: unexpected name i at end of statement
./main.go:11:1: syntax error: non-declaration statement outside function body
```

`while` no existe en Go. Escribe `for i < 3 {`.

### Usar el contador después del bucle

```go norun
package main

import "fmt"

func main() {
	for i := 0; i < 3; i++ {
		fmt.Println(i)
	}
	fmt.Println(i)
}
```

```text
# command-line-arguments
./main.go:9:14: undefined: i
```

`i` solo existe dentro del bucle. Para conservarla, créala antes:
`i := 0` y luego `for ; i < 3; i++ {`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este programa?

```go norun
package main

import "fmt"

func main() {
	x := 7
	switch {
	case x > 5:
		fmt.Println("grande")
	case x > 2:
		fmt.Println("mediano")
	}
}
```

> **Respuesta**
>
> `grande`, y nada más. Go se detiene en el primer caso verdadero y no pasa al siguiente.

**Pregunta 2:** muestra la tabla del 7, de `7 x 1 = 7` a `7 x 5 = 35`.

> **Respuesta**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for i := 1; i <= 5; i++ {
> 		fmt.Printf("7 x %d = %d\n", i, 7*i)
> 	}
> }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

**Pregunta 3:** ¿cómo se escribe en Go un bucle «mientras `n` sea mayor que 1»?

> **Respuesta**
>
> `for n > 1 { ... }`. La palabra `while` no existe: `for` con una sola condición
> la sustituye.

## En resumen

- `if`, `else if` y `else` toman decisiones; las llaves son obligatorias.
- `if x := ...; condición {` crea una variable que solo vive dentro del `if`.
- `switch` elige entre varios casos, sin `break`; sin valor, prueba
  condiciones.
- `for` es el único bucle: con contador, con una condición, o infinito.
- `range` recorre una slice, un texto o los enteros de 0 a n-1.
- `break` sale del bucle, `continue` pasa a la vuelta siguiente; una etiqueta apunta a un
  bucle exterior.

Siguiente: [las funciones](fonctions.md).
