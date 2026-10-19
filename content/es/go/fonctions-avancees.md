# Más allá de las funciones

Esta página presenta herramientas más avanzadas para las funciones de Go: un número variable de argumentos, las funciones guardadas en variables, las closures, `defer` y la recursión.

> **Objetivos:**
>
> - sabrás escribir una función que acepta un número variable de argumentos;
> - sabrás guardar una función en una variable y crear una closure;
> - sabrás aplazar una acción hasta el final con `defer`.

Este capítulo sigue a [las funciones](fonctions.md): volverás a encontrar `func`, los parámetros y `return`, esta vez para usos más flexibles.

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

### Olvidar los ... para pasar una slice

```go norun
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
	notas := []int{12, 15, 9}
	fmt.Println(suma(notas))
}
```

```text
# command-line-arguments
./main.go:15:19: cannot use notas (variable of type []int) as int value in argument to suma
```

`suma` espera los `int` uno a uno, no una slice. Añade `...` después de la slice:
`suma(notas...)`.

### Llamar a una función en vez de pasarla

```go norun
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
	fmt.Println(aplicar([]int{1, 2, 3}, doble()))
}
```

```text
# command-line-arguments
./main.go:18:38: not enough arguments in call to doble
	have ()
	want (int)
```

Con los paréntesis, `doble()` **llama** a la función, aquí sin argumento. Para pasar
la propia función, escribe solo su nombre: `aplicar([]int{1, 2, 3}, doble)`.

### Poner el parámetro variádico antes de los demás

```go norun
package main

import "fmt"

func presentar(notas ...int, nombre string) {
	fmt.Println(nombre, notas)
}

func main() {
	presentar(12, 15, "Alice")
}
```

```text
# command-line-arguments
./main.go:5:22: can only use ... with final parameter in list
```

El parámetro con `...` debe ser el último. Escribe
`func presentar(nombre string, notas ...int)`, y luego `presentar("Alice", 12, 15)`.

## Ponte a prueba

**Pregunta 1:** tienes `notas := []int{12, 15, 9}` y una función
`suma(numeros ...int) int`. ¿Cómo pasas `notas` a `suma`?

> **Respuesta**
>
> Añade `...` después de la slice: `suma(notas...)`. Sin los `...`, Go se niega:
> espera valores `int`, no una slice.

**Pregunta 2:** ¿qué muestra este programa?

```go norun
package main

import "fmt"

func main() {
	total := 0
	sumar := func(n int) {
		total += n
	}
	sumar(3)
	sumar(4)
	fmt.Println(total)
}
```

> **Respuesta**
>
> `7`. La función anónima usa la variable `total` que la rodea: cada llamada la
> modifica de verdad.

**Pregunta 3:** ¿qué muestra este programa?

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

## En resumen

- `...tipo` acepta un número variable de argumentos, recibidos en una slice.
- Una función es un valor: se guarda en una variable, se pasa como argumento.
- Una closure conserva las variables que la rodean.
- `defer` aplaza una llamada hasta el final de la función, en orden inverso.
- Una función recursiva se llama a sí misma y necesita un caso base.

Siguiente: [trabajar con texto](texte.md).
