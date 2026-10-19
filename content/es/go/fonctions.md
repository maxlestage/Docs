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

## Un número variable de argumentos

No siempre sabes cuántos valores vas a recibir: dos notas hoy, diez mañana. En vez de
escribir una función para cada caso, escribes una sola, que acepta cualquier cantidad.

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

Línea por línea:

- `numeros ...int`: todos los números dados en la llamada llegan a la slice `numeros`;
- `total` empieza en 0, y luego el bucle le suma cada número, uno a uno;
- `suma(1, 2)` devuelve 3, `suma(1, 2, 3, 4)` devuelve 10;
- `suma()` no recibe nada: el bucle no da ninguna vuelta, y `total` se queda en 0.

Para pasar una slice existente, añade `...` después de ella: `suma(notas...)`.
Por cierto, `fmt.Println` es variádica: por eso acepta tantos
valores.

**Te toca:** escribe una función `saludarATodos(nombres ...string)` que muestre
«¡Hola, …!» para cada nombre recibido. Llámala con tres nombres.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func saludarATodos(nombres ...string) {
> 	for _, nombre := range nombres {
> 		fmt.Println("¡Hola,", nombre+"!")
> 	}
> }
>
> func main() {
> 	saludarATodos("Alice", "Bob", "Chloé")
> }
> ```
>
> ```sortie
> ¡Hola, Alice!
> ¡Hola, Bob!
> ¡Hola, Chloé!
> ```

## Las funciones son valores

A veces quieres escribir una función general y luego decirle **qué hacer**. Es como un
taladro: la misma máquina perfora o atornilla, según la punta que le pongas.

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

Línea por línea:

- `operacion := doble` guarda la propia función en una variable, sin llamarla (no hay
  paréntesis);
- así que `operacion(5)` llama a `doble(5)`, que devuelve 10;
- `aplicar` recibe una slice y una función `f`. Llama a `f` con cada número y guarda
  los resultados en una slice nueva;
- con `doble`, `[1 2 3]` se convierte en `[2 4 6]`.

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

Puede que una función necesite **recordar** algo entre dos llamadas, como el contador
de la entrada de un museo, que suma 1 por cada visitante.

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

Línea por línea:

- `contador()` crea una variable `n` que vale 0 y luego devuelve una función anónima;
- `siguiente` recibe esa función. Sigue teniendo acceso a `n`, incluso después de que
  `contador` termine;
- cada llamada a `siguiente()` suma 1 a `n` y lo devuelve: 1, luego 2, luego 3;
- `otro := contador()` vuelve a ejecutar `contador`, que crea un `n` **nuevo** a 0: la
  primera llamada devuelve 1.

Imagina una mochila: la función devuelta guarda `n` en su mochila. Cada llamada a
`siguiente` aumenta **su** `n`. `otro` tiene su propia mochila, con su propio `n`.

## defer: aplazar hasta el final

Cuando abres algo, tienes que acordarte de cerrarlo. Es como una nota «apagar la luz»
pegada en la puerta: la escribes al llegar, y sirve en el momento de irte.

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

Línea por línea:

- `main` llama a `trabajar`, que primero muestra «Apertura»;
- la línea `defer` no muestra nada en el acto: apunta «mostrar Cierre» para más tarde;
- se muestran «Trabajo 1» y luego «Trabajo 2»;
- `trabajar` llega a su final: la llamada apartada se ejecuta, y «Cierre» se muestra
  la última.

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

**Te toca:** escribe un `main` que muestre «Adiós» al final, aunque escribas esa línea
primero. Después añade dos mensajes: «Hola» y «¿Cómo estás?».

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	defer fmt.Println("Adiós")
> 	fmt.Println("Hola")
> 	fmt.Println("¿Cómo estás?")
> }
> ```
>
> ```sortie
> Hola
> ¿Cómo estás?
> Adiós
> ```

## Una función que se llama a sí misma

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Algunos problemas contienen una versión más pequeña de sí mismos, como las muñecas
rusas: al abrir una muñeca, encuentras otra más pequeña dentro, hasta la última.

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
