# Las goroutines

Esta página muestra cómo lanzar varias tareas a la vez con las goroutines, esperarlas con `sync.WaitGroup` y proteger los datos compartidos con `sync.Mutex`.

> **Objetivos:**
>
> - sabrás lanzar una goroutine con la palabra clave `go`;
> - sabrás esperar a que terminen varias goroutines con `sync.WaitGroup`;
> - sabrás reconocer una condición de carrera y corregirla con `sync.Mutex`;
> - sabrás obtener una salida siempre idéntica a pesar del desorden;
> - sabrás organizar el trabajo con un grupo de trabajadores.

## Hacer varias cosas a la vez

Hasta ahora, tus programas hacían una cosa detrás de otra. La
[concurrencia](../debuter/glossaire.md#concurrencia) permite avanzar en varias tareas a
la vez: descargar diez archivos sin esperar a que termine cada uno, responder a varios
visitantes de un sitio web…

Piensa en una cocina. Un solo cocinero prepara el arroz, **luego** el pescado, **luego**
el postre. Una brigada de cocineros lo hace todo a la vez, y la comida está lista mucho
antes. Pero hay que organizarse: saber cuándo han terminado todos, y no pelearse por la
misma cazuela.

## La palabra clave go

Una **goroutine** es una función que se ejecuta en paralelo con el resto del programa.
Para lanzar una, escribe `go` delante de una llamada a función. Eso es todo. Una
goroutine cuesta muy poco: un programa puede lanzar miles. Se parece a un
[hilo](../debuter/glossaire.md#hilo) (*thread*), pero mucho más ligero.

```go norun
go preparar("arroz")
```

`go` lanza la función y pasa **enseguida** a la línea siguiente, sin esperar.
Problema: cuando `main` termina, el programa se detiene, aunque haya goroutines que
sigan trabajando. Así que hay que esperarlas.

## Esperar con sync.WaitGroup

Un `sync.WaitGroup` es un contador de tareas en curso, como el jefe de cocina que
cuenta a sus cocineros:

- `wg.Add(1)`: «una tarea más» (antes de lanzar la goroutine);
- `wg.Done()`: «una tarea terminada» (al final de la goroutine);
- `wg.Wait()`: «espero a que el contador vuelva a cero».

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	platos := []string{"arroz", "pescado", "postre"}
	listos := make([]string, len(platos))

	var wg sync.WaitGroup
	for i, plato := range platos {
		wg.Add(1)
		go func() {
			defer wg.Done()
			listos[i] = plato + ": listo"
		}()
	}
	wg.Wait()

	for _, l := range listos {
		fmt.Println(l)
	}
}
```

```sortie
arroz: listo
pescado: listo
postre: listo
```

`go func() { ... }()` lanza una función anónima escrita en el sitio. `defer wg.Done()`
garantiza que se llama a `Done` al final, aunque la función se detenga antes.

### ¿Por qué guardar los resultados?

Las goroutines terminan en un orden **imprevisible**: la primera en lanzarse no es
necesariamente la primera en terminar. Si cada una mostrara su resultado, el orden
cambiaría de una ejecución a otra.

Aquí, cada goroutine escribe en **su propia casilla** `listos[i]`. Después, el programa
muestra las casillas en orden, tras `wg.Wait()`. La salida es siempre la misma.

> **Nota:** desde Go 1.22, cada vuelta del bucle tiene sus propias variables `i` y
> `plato`. En código más antiguo, verás `go func(i int) { ... }(i)`: antes era
> necesario.

## El problema de los accesos simultáneos

¿Qué pasa si varias goroutines modifican **la misma** variable? Tomemos un contador que
1000 goroutines aumentan en 1.

```go norun
contador := 0
var wg sync.WaitGroup
for i := 0; i < 1000; i++ {
	wg.Add(1)
	go func() {
		defer wg.Done()
		contador++
	}()
}
wg.Wait()
fmt.Println(contador)
```

Esperamos `1000`, pero el resultado puede ser menor, y distinto cada vez.
`contador++` se hace en tres pasos: leer el valor, sumar 1, escribir. Dos goroutines
pueden leer `41` a la vez y luego escribir las dos `42`: se pierde una suma.

Es una **condición de carrera** (*data race*): dos cocineros que salan la misma cazuela
sin hablarse.

### El detector de carreras

Go sabe encontrar estos errores. Añade `-race`:

```sh
go run -race main.go
```

El programa va más lento, pero muestra `WARNING: DATA RACE` con las líneas implicadas
en cuanto se produce una carrera. Tenlo en cuenta cuando escribas código concurrente, y
en tus [pruebas](tests.md) (`go test -race`).

## Proteger con sync.Mutex

Un `sync.Mutex` («exclusión mutua») es un cerrojo. Es la única llave del baño de una
cafetería: una sola persona a la vez, las demás esperan.

- `mu.Lock()` toma el cerrojo (o espera a que esté libre);
- `mu.Unlock()` lo devuelve.

El código entre los dos solo lo ejecuta una goroutine a la vez.

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	contador := 0
	var mu sync.Mutex
	var wg sync.WaitGroup

	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			mu.Lock()
			contador++
			mu.Unlock()
		}()
	}
	wg.Wait()
	fmt.Println(contador)
}
```

```sortie
1000
```

Ahora el resultado es siempre `1000`.

### Un tipo protegido

Lo más limpio es guardar el cerrojo **junto con** los datos que protege, en una
[estructura](types.md). Los métodos usan un receptor puntero: un cerrojo nunca debe
copiarse.

```go
package main

import (
	"fmt"
	"sort"
	"sync"
)

type Votos struct {
	mu     sync.Mutex
	cuenta map[string]int
}

func (v *Votos) Votar(opcion string) {
	v.mu.Lock()
	defer v.mu.Unlock()
	v.cuenta[opcion]++
}

func main() {
	v := &Votos{cuenta: map[string]int{}}
	papeletas := []string{"gato", "perro", "gato", "conejo", "gato", "perro"}

	var wg sync.WaitGroup
	for _, p := range papeletas {
		wg.Add(1)
		go func() {
			defer wg.Done()
			v.Votar(p)
		}()
	}
	wg.Wait()

	nombres := make([]string, 0, len(v.cuenta))
	for nombre := range v.cuenta {
		nombres = append(nombres, nombre)
	}
	sort.Strings(nombres)
	for _, nombre := range nombres {
		fmt.Println(nombre, v.cuenta[nombre])
	}
}
```

```sortie
conejo 1
gato 3
perro 2
```

El orden de un [map](maps.md) cambia en cada recorrido: ordenamos las claves antes de
mostrarlas.

## Un grupo de trabajadores

Lanzar una goroutine por tarea funciona bien para unas pocas tareas. Para miles de
archivos que procesar, es mejor un **grupo de trabajadores** (*worker pool*): un número
fijo de goroutines que se reparten la lista. Es un equipo de tres cajeros para toda una
cola de clientes.

Aquí, cada trabajador toma la tarea siguiente gracias a un índice compartido, protegido
por un cerrojo.

```go
package main

import (
	"fmt"
	"sort"
	"sync"
)

func main() {
	numeros := []int{3, 1, 4, 1, 5, 9, 2, 6}
	var resultados []string

	var mu sync.Mutex
	siguiente := 0

	var wg sync.WaitGroup
	for t := 1; t <= 3; t++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				mu.Lock()
				if siguiente >= len(numeros) {
					mu.Unlock()
					return
				}
				n := numeros[siguiente]
				siguiente++
				mu.Unlock()

				cuadrado := fmt.Sprintf("%d² = %d", n, n*n)

				mu.Lock()
				resultados = append(resultados, cuadrado)
				mu.Unlock()
			}
		}()
	}
	wg.Wait()

	sort.Strings(resultados)
	fmt.Println(len(resultados), "resultados")
	for _, r := range resultados {
		fmt.Println(r)
	}
}
```

```sortie
8 resultados
1² = 1
1² = 1
2² = 4
3² = 9
4² = 16
5² = 25
6² = 36
9² = 81
```

El cálculo se hace **fuera** del cerrojo: se guarda el cerrojo el menor tiempo posible;
si no, los trabajadores se esperan unos a otros. El capítulo siguiente,
[los canales](canaux.md), muestra una forma más natural de repartir las tareas.

## Errores frecuentes

### No esperar a las goroutines

```go norun
package main

import "fmt"

func main() {
	go fmt.Println("Hola desde una goroutine")
	fmt.Println("Fin")
}
```

```text
Fin
```

`main` termina antes de que la goroutine haya tenido tiempo de mostrar nada: su mensaje
se pierde. Espérala con un `sync.WaitGroup`.

### Llamar a Done más veces que a Add

```go norun
package main

import (
	"fmt"
	"sync"
)

func main() {
	var wg sync.WaitGroup
	wg.Done()
	fmt.Println("fin")
}
```

```text
panic: sync: negative WaitGroup counter

goroutine 1 [running]:
sync.(*WaitGroup).Add(0x0?, 0x68?)
	/usr/local/go1.24.7/src/sync/waitgroup.go:64 +0xcd
sync.(*WaitGroup).Done(...)
	/usr/local/go1.24.7/src/sync/waitgroup.go:89
main.main()
	/ruta/a/main.go:10 +0x27
exit status 2
```

El contador ha bajado de cero. Cada `wg.Done()` debe corresponder a un `wg.Add(1)`
hecho **antes** de lanzar la goroutine.

### Copiar un WaitGroup

```go norun
package main

import (
	"fmt"
	"sync"
)

func trabajar(id int, wg sync.WaitGroup) {
	defer wg.Done()
	fmt.Println("trabajo", id)
}

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	go trabajar(1, wg)
	wg.Wait()
}
```

```text
trabajo 1
fatal error: all goroutines are asleep - deadlock!

goroutine 1 [sync.WaitGroup.Wait]:
sync.runtime_SemacquireWaitGroup(0xc00012a000?)
	/usr/local/go1.24.7/src/runtime/sema.go:110 +0x25
sync.(*WaitGroup).Wait(0x0?)
	/usr/local/go1.24.7/src/sync/waitgroup.go:118 +0x48
main.main()
	/ruta/a/main.go:17 +0x72
exit status 2
```

La función ha recibido una **copia** del `WaitGroup`: su `Done` decrementa la copia, y
el original espera para siempre. Pasa un puntero: `wg *sync.WaitGroup` y
`trabajar(1, &wg)`. El comando `go vet` señala esta trampa.

## Ponte a prueba

**Pregunta 1:** ¿qué hace `go f()`?

> **Respuesta**
>
> Lanza `f` en una nueva goroutine y pasa enseguida a la línea siguiente, sin esperar
> a que `f` termine.

**Pregunta 2:** ¿por qué `contador++` en 1000 goroutines no da siempre 1000 sin
cerrojo?

> **Respuesta**
>
> Porque `contador++` lee, suma y luego escribe. Dos goroutines pueden leer el mismo
> valor a la vez: se pierde una suma. Es una condición de carrera; un `sync.Mutex` la
> impide.

**Pregunta 3:** calcula la longitud de cada palabra de `[]string{"go", "veloz", "y",
"simple"}` en goroutines, y muestra los resultados en el orden de las palabras.

> **Respuesta**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"sync"
> )
>
> func main() {
> 	palabras := []string{"go", "veloz", "y", "simple"}
> 	longitudes := make([]int, len(palabras))
> 	var wg sync.WaitGroup
> 	for i, p := range palabras {
> 		wg.Add(1)
> 		go func() {
> 			defer wg.Done()
> 			longitudes[i] = len(p)
> 		}()
> 	}
> 	wg.Wait()
> 	for i, p := range palabras {
> 		fmt.Println(p, longitudes[i])
> 	}
> }
> ```
>
> ```sortie
> go 2
> veloz 5
> y 1
> simple 6
> ```
>
> Cada goroutine escribe en su propia casilla: no hace falta cerrojo.

## En resumen

- `go f()` lanza `f` en una goroutine, sin esperar.
- `sync.WaitGroup` espera a que terminen las goroutines: `Add` antes, `Done` al final,
  `Wait`.
- Varias goroutines que modifican el mismo dato crean una carrera; `sync.Mutex` la
  protege, `go run -race` la detecta.
- Guarda los resultados en casillas separadas u ordénalos para tener una salida
  estable.
- Un grupo de trabajadores limita el número de goroutines.

Siguiente: [los canales](canaux.md).
