# Las funciones

Esta página explica cómo escribir funciones en Go con `func`, sus parámetros, sus valores de retorno, y herramientas más avanzadas como las closures y `defer`.

> **Objetivos:**
>
> - sabrás escribir una función con `func` y llamarla;
> - sabrás dar parámetros tipados y devolver uno o varios valores;
> - sabrás escribir una función que acepta un número variable de argumentos;
> - sabrás guardar una función en una variable y crear una closure;
> - sabrás aplazar una acción hasta el final con `defer`.

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

Escribir `func saludar()` no ejecuta nada: guarda la receta. Escribir `saludar()`
**llama** a la función, es decir, la ejecuta.

> **Nota:** las funciones se definen fuera de `main`, a nivel del archivo.
> El orden no importa: `main` puede llamar a una función escrita más abajo.

## Los parámetros

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

## Devolver un valor

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

## Varios valores de retorno

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

## Un número variable de argumentos

Con `...` delante del tipo, el último parámetro acepta tantos argumentos como
quieras. Dentro de la función, se convierte en una slice. Se habla de función **variádica**.

```go
package main

import "fmt"

func suma(numeros ...int) int {
	total := 0
	for _, n := range numeros {
		total += n
	}
	return total
}

func main() {
	fmt.Println(suma(1, 2))
	fmt.Println(suma(1, 2, 3, 4))
	fmt.Println(suma())

	notas := []int{12, 15, 9}
	fmt.Println(suma(notas...))
}
```

```sortie
3
10
0
36
```

Para pasar una slice existente, añade `...` después de ella: `suma(notas...)`.
Por cierto, `fmt.Println` es variádica: por eso acepta tantos
valores.

## Las funciones son valores

En Go, una función es un valor como cualquier otro. Puedes guardarla en una
variable, o pasarla como argumento a otra función.

```go
package main

import "fmt"

func doble(x int) int {
	return x * 2
}

func aplicar(numeros []int, f func(int) int) []int {
	resultado := []int{}
	for _, n := range numeros {
		resultado = append(resultado, f(n))
	}
	return resultado
}

func main() {
	operacion := doble
	fmt.Println(operacion(5))
	fmt.Println(aplicar([]int{1, 2, 3}, doble))
}
```

```sortie
10
[2 4 6]
```

El tipo `func(int) int` se lee: «una función que recibe un `int` y devuelve un
`int`».

### Las funciones anónimas

Una función se puede escribir sin nombre, directamente donde se necesita: es una
función **anónima**.

```go
package main

import "fmt"

func main() {
	triple := func(x int) int {
		return x * 3
	}
	fmt.Println(triple(7))

	func() {
		fmt.Println("¡Ejecutada al instante!")
	}()
}
```

```sortie
21
¡Ejecutada al instante!
```

Los paréntesis `()` al final de la segunda función la llaman en el acto.

## Las closures

Una función anónima puede usar las variables que la rodean. Se las **lleva**
consigo, incluso cuando sale del lugar donde nació. Entonces se la llama
[closure](../debuter/glossaire.md#closure) («clausura»).

```go
package main

import "fmt"

func contador() func() int {
	n := 0
	return func() int {
		n++
		return n
	}
}

func main() {
	siguiente := contador()
	fmt.Println(siguiente())
	fmt.Println(siguiente())
	fmt.Println(siguiente())

	otro := contador()
	fmt.Println(otro())
}
```

```sortie
1
2
3
1
```

Imagina una mochila: la función devuelta guarda `n` en su mochila. Cada llamada a
`siguiente` aumenta **su** `n`. `otro` tiene su propia mochila, con su propio `n`.

## defer: aplazar hasta el final

`defer` («aplazar») retrasa una llamada a función hasta el momento en que la función en curso
termina.

```go
package main

import "fmt"

func trabajar() {
	fmt.Println("Apertura")
	defer fmt.Println("Cierre")
	fmt.Println("Trabajo 1")
	fmt.Println("Trabajo 2")
}

func main() {
	trabajar()
}
```

```sortie
Apertura
Trabajo 1
Trabajo 2
Cierre
```

Es muy útil para «ordenar»: se escribe el cierre de un archivo justo al lado de
su apertura, y se tiene la seguridad de que ocurrirá, incluso si la función sale antes.

Varios `defer` se ejecutan en orden **inverso**, como una pila de platos: el
último que se pone es el primero que se coge.

```go
package main

import "fmt"

func main() {
	for i := range 3 {
		defer fmt.Println("defer", i)
	}
	fmt.Println("fin de main")
}
```

```sortie
fin de main
defer 2
defer 1
defer 0
```

## Una función que se llama a sí misma

Una función puede llamarse a sí misma: es la
[recursión](../debuter/glossaire.md#recursion). Hace falta un **caso base**; si no,
no se detiene nunca.

```go
package main

import "fmt"

func factorial(n int) int {
	if n <= 1 {
		return 1
	}
	return n * factorial(n-1)
}

func main() {
	fmt.Println(factorial(5))
}
```

```sortie
120
```

`factorial(5)` vale `5 * factorial(4)`, que vale `4 * factorial(3)`… hasta
`factorial(1)`, el caso base, que vale 1. El resultado es 5 × 4 × 3 × 2 × 1 = 120.

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

**Pregunta 1:** ¿qué muestra este programa?

```go norun
package main

import "fmt"

func main() {
	defer fmt.Println("A")
	fmt.Println("B")
	defer fmt.Println("C")
}
```

> **Respuesta**
>
> `B`, luego `C`, luego `A`. `B` se muestra enseguida; los `defer` esperan al final
> de `main` y se ejecutan en orden inverso.

**Pregunta 2:** escribe una función `media(a, b int) float64` que devuelva la
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

**Pregunta 3:** ¿cuál es la diferencia entre un parámetro y un argumento?

> **Respuesta**
>
> El parámetro es el nombre escrito en `func` (`nombre string`). El argumento es el
> valor dado en la llamada (`"Alice"`).

## En resumen

- Una función se escribe `func nombre(parámetro tipo) tipoRetorno { ... }` y devuelve con
  `return`.
- Una función puede devolver varios valores; el último suele ser un `error`.
- `...tipo` acepta un número variable de argumentos, recibidos en una slice.
- Una función es un valor: se guarda en una variable, se pasa como argumento.
- Una closure conserva las variables que la rodean.
- `defer` aplaza una llamada hasta el final de la función, en orden inverso.

Siguiente: [trabajar con texto](texte.md). Compara con
[las funciones en Rust](../rust/fonctions.md).
