# Las funciones

Esta página explica cómo escribir funciones en Go con `func`, darles parámetros y hacer que devuelvan uno o varios valores.

> **Objetivos:**
>
> - sabrás escribir una función con `func` y llamarla;
> - sabrás dar parámetros tipados y devolver uno o varios valores;
> - sabrás salir antes de una función con `return`.

## Definir una función

Una [función](../debuter/glossaire.md#funcion) es un trozo de código con nombre, que
puedes reutilizar. Es como una receta: la escribes una vez y luego la
sigues tantas veces como quieras. Empieza por la palabra `func`.

```go
package main

import "fmt"

func saludar() {
	fmt.Println("¡Hola!")
}

func main() {
	saludar()
	saludar()
}
```

```sortie
¡Hola!
¡Hola!
```

Línea por línea:

- `func saludar() { ... }` guarda la receta con el nombre `saludar`. Todavía no se
  muestra nada;
- el programa siempre empieza por `main`;
- el primer `saludar()` ejecuta la receta: se muestra «¡Hola!»;
- el segundo `saludar()` la ejecuta una segunda vez.

Escribir `func saludar()` no ejecuta nada: guarda la receta. Escribir `saludar()`
**llama** a la función, es decir, la ejecuta.

> **Nota:** las funciones se definen fuera de `main`, a nivel del archivo.
> El orden no importa: `main` puede llamar a una función escrita más abajo.

## Los parámetros

Una receta de crepes puede servir para 4 o para 8 personas: es la misma receta, con
otra cantidad. Los parámetros permiten que una función trabaje con valores distintos
en cada llamada.

Los [parámetros](../debuter/glossaire.md#parametro) son los valores que la función
recibe. Son casillas vacías, que se rellenan en el momento de la llamada. Los valores dados en
la llamada se llaman [argumentos](../debuter/glossaire.md#argumento).

En Go, cada parámetro tiene un **tipo**, escrito **después** de su nombre.

```go
package main

import "fmt"

func saludar(nombre string) {
	fmt.Println("¡Hola,", nombre+"!")
}

func main() {
	saludar("Alice")
	saludar("Bob")
}
```

```sortie
¡Hola, Alice!
¡Hola, Bob!
```

Línea por línea:

- `func saludar(nombre string)` anuncia una casilla vacía llamada `nombre`, que recibirá
  texto;
- `saludar("Alice")` rellena la casilla: dentro de la función, `nombre` vale `"Alice"`;
- `saludar("Bob")` vuelve a ejecutar la función, y esta vez `nombre` vale `"Bob"`.

Varios parámetros se separan con comas. Si tienen el mismo tipo, puedes
escribirlo una sola vez, al final.

```go
package main

import "fmt"

func presentar(nombre string, edad int) {
	fmt.Printf("%s tiene %d años.\n", nombre, edad)
}

func area(ancho, alto int) {
	fmt.Println("Área:", ancho*alto)
}

func main() {
	presentar("Alice", 12)
	area(3, 4)
}
```

```sortie
Alice tiene 12 años.
Área: 12
```

`ancho, alto int` quiere decir: `ancho` y `alto` son dos `int`.

**Te toca:** escribe una función `anunciar(plato string, precio int)` que muestre
«Plato del día: sopa, 6 euros.». Llámala dos veces desde `main`, con dos platos
distintos.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func anunciar(plato string, precio int) {
> 	fmt.Printf("Plato del día: %s, %d euros.\n", plato, precio)
> }
>
> func main() {
> 	anunciar("sopa", 6)
> 	anunciar("lasaña", 12)
> }
> ```
>
> ```sortie
> Plato del día: sopa, 6 euros.
> Plato del día: lasaña, 12 euros.
> ```

## Devolver un valor

Mostrar un resultado no siempre basta. A menudo quieres **guardarlo** para usarlo
después, como el número que lees en una calculadora antes de seguir con tu cuenta.

Para que una función **entregue** un resultado, indica su tipo después de los paréntesis, y
usa `return`.

```go
package main

import "fmt"

func cuadrado(x int) int {
	return x * x
}

func main() {
	fmt.Println(cuadrado(4))
	resultado := cuadrado(3) + 1
	fmt.Println(resultado)
}
```

```sortie
16
10
```

Línea por línea:

- `func cuadrado(x int) int`: la función recibe un `int` y promete devolver un `int`;
- `return x * x` calcula el resultado y se lo devuelve a quien llamó;
- así que `cuadrado(4)` vale 16, y `fmt.Println` lo muestra;
- `cuadrado(3) + 1` da `9 + 1`: la variable `resultado` vale 10.

El valor devuelto reemplaza a la llamada: `cuadrado(3) + 1` se convierte en `9 + 1`.

`return` también detiene la función en el acto. Se usa para salir antes:

```go
package main

import "fmt"

func calificacion(nota int) string {
	if nota >= 16 {
		return "sobresaliente"
	}
	if nota >= 10 {
		return "aprobado"
	}
	return "suspenso"
}

func main() {
	fmt.Println(calificacion(17))
	fmt.Println(calificacion(11))
	fmt.Println(calificacion(4))
}
```

```sortie
sobresaliente
aprobado
suspenso
```

> **Recuerda:** una función que anuncia un tipo de retorno debe terminar **siempre** con
> un `return`, sea cual sea el camino seguido. Si no, el compilador la rechaza.

**Te toca:** escribe una función `perimetro(lado int) int` que devuelva el perímetro de
un cuadrado (4 veces el lado). En `main`, muestra `perimetro(5)` y luego
`perimetro(2) + 1`.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func perimetro(lado int) int {
> 	return 4 * lado
> }
>
> func main() {
> 	fmt.Println(perimetro(5))
> 	fmt.Println(perimetro(2) + 1)
> }
> ```
>
> ```sortie
> 20
> 9
> ```

## Varios valores de retorno

Algunas preguntas tienen dos respuestas. Repartir 17 caramelos entre 5 niños da un
número de caramelos por niño, y un resto. Go puede devolver los dos a la vez.

Una función de Go puede devolver **varios** valores. Escribe sus tipos entre
paréntesis.

```go
package main

import "fmt"

func dividir(a, b int) (int, int) {
	return a / b, a % b
}

func main() {
	cociente, resto := dividir(17, 5)
	fmt.Println(cociente, resto)

	c, _ := dividir(20, 3)
	fmt.Println(c)
}
```

```sortie
3 2
6
```

Línea por línea:

- `(int, int)` anuncia que `dividir` devuelve dos enteros;
- `return a / b, a % b` devuelve el cociente y luego el resto, separados por una coma;
- `cociente, resto := dividir(17, 5)` guarda 3 en `cociente` y 2 en `resto`;
- `c, _ := dividir(20, 3)` se queda con el cociente, 6, y tira el resto.

El `_` tira el valor que no se necesita.

Así es como Go señala los errores: el último valor devuelto suele ser un
`error`, que vale `nil` si todo va bien.

```go
package main

import (
	"errors"
	"fmt"
)

func raizEntera(n int) (int, error) {
	if n < 0 {
		return 0, errors.New("número negativo")
	}
	r := 0
	for (r+1)*(r+1) <= n {
		r++
	}
	return r, nil
}

func main() {
	r, err := raizEntera(20)
	fmt.Println(r, err)
	r, err = raizEntera(-4)
	fmt.Println(r, err)
}
```

```sortie
4 <nil>
0 número negativo
```

Los errores tienen su propio capítulo: [la gestión de errores](erreurs.md).

### Los resultados con nombre

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Puedes dar un nombre a los valores de retorno. Entonces son variables, creadas
con su valor cero. Un `return` solo devuelve sus valores actuales.

```go
package main

import "fmt"

func minMax(numeros []int) (menor, mayor int) {
	menor, mayor = numeros[0], numeros[0]
	for _, n := range numeros {
		menor = min(menor, n)
		mayor = max(mayor, n)
	}
	return
}

func main() {
	m, M := minMax([]int{4, 9, 1, 7})
	fmt.Println(m, M)
}
```

```sortie
1 9
```

> **Consejo:** los nombres ayudan a documentar lo que devuelve la función. Pero un `return`
> solo en una función larga se vuelve pronto difícil de leer: resérvalo para las
> funciones cortas.

## Errores frecuentes

### Olvidar un return

```go norun
package main

import "fmt"

func signo(n int) string {
	if n >= 0 {
		return "positivo"
	}
}

func main() {
	fmt.Println(signo(3))
}
```

```text
# command-line-arguments
./main.go:9:1: missing return
```

Si `n` es negativo, la función no tiene nada que devolver. Añade un `return "negativo"`
antes de la llave final.

### Olvidar un argumento

```go norun
package main

import "fmt"

func area(ancho, alto int) int {
	return ancho * alto
}

func main() {
	fmt.Println(area(3))
}
```

```text
# command-line-arguments
./main.go:10:19: not enough arguments in call to area
	have (number)
	want (int, int)
```

La función quiere dos `int` (*want*), recibe un solo número (*have*). Dalos
todos: `area(3, 4)`.

### Recibir un solo valor en lugar de dos

```go norun
package main

import "fmt"

func dividir(a, b int) (int, int) {
	return a / b, a % b
}

func main() {
	q := dividir(7, 2)
	fmt.Println(q)
}
```

```text
# command-line-arguments
./main.go:10:7: assignment mismatch: 1 variable but dividir returns 2 values
```

Hace falta una variable por cada valor devuelto: `q, r := dividir(7, 2)`, o `q, _ :=` para
ignorar el resto.

### Pasar un argumento del tipo incorrecto

```go norun
package main

import "fmt"

func doble(n int) int {
	return n * 2
}

func main() {
	fmt.Println(doble("4"))
}
```

```text
# command-line-arguments
./main.go:10:20: cannot use "4" (untyped string constant) as int value in argument to doble
```

`"4"` es texto, no un número. Escribe `doble(4)`, o convierte el texto con
`strconv.Atoi`.

## Ponte a prueba

**Pregunta 1:** escribe una función `media(a, b int) float64` que devuelva la
media de dos enteros.

> **Respuesta**
>
> ```go
> package main
>
> import "fmt"
>
> func media(a, b int) float64 {
> 	return float64(a+b) / 2
> }
>
> func main() {
> 	fmt.Println(media(3, 4))
> }
> ```
>
> ```sortie
> 3.5
> ```
>
> La conversión `float64(...)` evita la división entera.

**Pregunta 2:** ¿cuál es la diferencia entre un parámetro y un argumento?

> **Respuesta**
>
> El parámetro es el nombre escrito en `func` (`nombre string`). El argumento es el
> valor dado en la llamada (`"Alice"`).

**Pregunta 3:** ¿qué hace el `_` en `c, _ := dividir(20, 3)`?

> **Respuesta**
>
> Tira el segundo valor devuelto, el resto. Go pide una variable por cada valor
> devuelto: `_` ocupa el lugar del que no necesitas.

## En resumen

- Una función se escribe `func nombre(parámetro tipo) tipoRetorno { ... }` y devuelve con
  `return`.
- El tipo de un parámetro se escribe después de su nombre; `ancho, alto int` da el
  mismo tipo a los dos.
- `return` devuelve el valor y detiene la función enseguida.
- Una función puede devolver varios valores; el último suele ser un `error`.

Siguiente: [más allá de las funciones](fonctions-avancees.md). Compara con
[las funciones en Rust](../rust/fonctions.md).
