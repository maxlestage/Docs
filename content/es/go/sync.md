# sync: Once, RWMutex, atomic y errgroup

Esta página presenta las otras herramientas de coordinación de Go: `sync.Once` para hacer algo una sola vez, `sync.RWMutex` para dejar leer a varios a la vez, `sync/atomic` para los contadores y `errgroup` para esperar goroutines que pueden fallar.

> **Objetivos:**
>
> - reconocer una condición de carrera y hacer que `go run -race` la señale;
> - elegir entre `sync.Mutex` y `sync.RWMutex`;
> - lanzar una inicialización una sola vez con `sync.Once`;
> - contar sin cerrojo con `atomic.Int64`;
> - esperar goroutines y obtener el primer error con `errgroup`.

Este capítulo supone que conoces [las goroutines](concurrence.md), `sync.WaitGroup` y
`sync.Mutex`. Los repasamos rápidamente.

## La caja de herramientas del paquete sync

Varias [goroutines](../debuter/glossaire.md#goroutine) que comparten datos son como
varios cocineros en la misma cocina. Hacen falta reglas. Cada herramienta es una regla
distinta:

| Herramienta | La regla, en la cocina | Sirve para |
|---|---|---|
| `sync.WaitGroup` | el jefe cuenta los cocineros que han terminado | esperar el final |
| `sync.Mutex` | una sola llave para la cámara frigorífica | uno solo a la vez |
| `sync.RWMutex` | todos leen la receta, uno solo la corrige | muchos lectores, un escritor |
| `sync.Once` | el horno se enciende una sola vez por la mañana | hacer algo una vez |
| `atomic.Int64` | un contador de clics en la entrada | contar rápido y sin cerrojo |
| `errgroup` | el jefe espera a todos y anota el primer problema | esperar y obtener un error |

> **Recuerda:** cada herramienta responde a una pregunta precisa. Empieza por
> preguntarte: «¿qué se comparte y quién lo toca?»

## Repaso: esperar con WaitGroup

`wg.Add(1)` antes de lanzar, `defer wg.Done()` en la goroutine, `wg.Wait()` para
esperar. Cada goroutine escribe en **su propia casilla**: no hay nada que proteger.

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	numeros := []int{1, 2, 3, 4}
	cuadrados := make([]int, len(numeros))

	var wg sync.WaitGroup
	for i, n := range numeros {
		wg.Add(1)
		go func() {
			defer wg.Done()
			cuadrados[i] = n * n
		}()
	}
	wg.Wait()
	fmt.Println(cuadrados)
}
```

```sortie
[1 4 9 16]
```

```text
goroutine 0 ──► cuadrados[0]
goroutine 1 ──► cuadrados[1]      cada una su casilla:
goroutine 2 ──► cuadrados[2]      sin peleas
goroutine 3 ──► cuadrados[3]
```

> **Recuerda:** si cada goroutine tiene su propia casilla, un `WaitGroup` basta. Los
> cerrojos solo sirven cuando varias goroutines tocan el **mismo** dato.

## Una condición de carrera, paso a paso

Cuando dos goroutines modifican la misma variable sin ninguna regla, se produce una
**condición de carrera** (en inglés *data race*). Este es el ejemplo más pequeño: dos
goroutines hacen `contador++`.

```go norun
contador := 0
var wg sync.WaitGroup
for i := 0; i < 2; i++ {
	wg.Add(1)
	go func() {
		defer wg.Done()
		contador++
	}()
}
wg.Wait()
fmt.Println(contador)
```

`contador++` parece una sola acción. En realidad, son tres pequeños pasos: **leer**,
**sumar 1**, **escribir**. Este es un orden posible, en el que se pierde una suma:

| Paso | Goroutine A | Goroutine B | contador en memoria |
|---|---|---|---|
| 1 | lee 0 | | 0 |
| 2 | | lee 0 | 0 |
| 3 | calcula 0 + 1 = 1 | | 0 |
| 4 | | calcula 0 + 1 = 1 | 0 |
| 5 | escribe 1 | | 1 |
| 6 | | escribe 1 | **1** en lugar de 2 |

Es como dos personas que anotan las entradas de un museo en el mismo cuaderno: leen
«41» a la vez, las dos escriben «42», y un visitante desaparece.

### El detector -race

El fallo es raro y cambia de una vez a otra: no siempre se ve. Go tiene un detector.
Añade `-race` a `go run` (o a `go test`):

```sh
go run -race main.go
```

```text
==================
WARNING: DATA RACE
Read at 0x00c000122078 by goroutine 7:
  main.main.func1()
      /home/lea/contador/main.go:15 +0x84

Previous write at 0x00c000122078 by goroutine 8:
  main.main.func1()
      /home/lea/contador/main.go:15 +0x96

Goroutine 7 (running) created at:
  main.main()
      /home/lea/contador/main.go:13 +0x78

Goroutine 8 (finished) created at:
  main.main()
      /home/lea/contador/main.go:13 +0x78
==================
2
Found 1 data race(s)
exit status 66
```

Cómo leerlo:

- `WARNING: DATA RACE`: se ha visto una carrera;
- `Read at … main.go:15` y luego `Previous write at … main.go:15`: una lectura y una
  escritura de la misma casilla de memoria, en la línea 15 (`contador++`), por dos
  goroutines distintas;
- `created at … main.go:13`: la línea donde se lanzan esas goroutines (`go func()`);
- el `2` que se muestra no prueba nada: esta vez hubo suerte.

Pruébalo: ejecuta el programa sin `-race` y luego con él. Sin él, muestra `2` y no dice
nada más. Con él, muestra el informe y termina con el código de salida 66.

> **Recuerda:** ejecuta tus programas y tus [tests](tests.md) con `-race` en cuanto haya
> goroutines. El detector encuentra los fallos que tus ojos no ven.

## Repaso: uno solo a la vez con Mutex

Un `sync.Mutex` es un cerrojo. `mu.Lock()` toma la llave; si alguien ya la tiene, se
espera. `mu.Unlock()` devuelve la llave.

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

Con el cerrojo, la tabla de antes queda así:

| Paso | Goroutine A | Goroutine B | contador |
|---|---|---|---|
| 1 | `Lock()`: toma la llave | | 0 |
| 2 | lee 0, escribe 1 | `Lock()`: espera… | 1 |
| 3 | `Unlock()` | …sigue esperando | 1 |
| 4 | | toma la llave, lee 1, escribe 2 | 2 |
| 5 | | `Unlock()` | 2 |

> **Recuerda:** todo lo que está entre `Lock()` y `Unlock()` lo hace una sola goroutine
> a la vez.

## Leer entre varios: RWMutex

Un `Mutex` hace esperar a todos, incluso a los que solo quieren **leer**. Es una pena:
leer no rompe nada.

Piensa en el tablón de anuncios de una escuela. Veinte alumnos pueden leerlo a la vez.
Pero cuando el director cambia un cartel, pide a todos que se aparten.

Por eso un `sync.RWMutex` tiene dos tipos de cerrojo:

| Método | Para quién | Cuántos a la vez |
|---|---|---|
| `RLock()` / `RUnlock()` | un lector | tantos como quieras |
| `Lock()` / `Unlock()` | un escritor | uno solo, y ningún lector mientras tanto |

```go
package main

import (
	"fmt"
	"sync"
)

type Agenda struct {
	mu      sync.RWMutex
	numeros map[string]string
}

func (a *Agenda) Buscar(nombre string) string {
	a.mu.RLock()
	defer a.mu.RUnlock()
	return a.numeros[nombre]
}

func (a *Agenda) Agregar(nombre, numero string) {
	a.mu.Lock()
	defer a.mu.Unlock()
	a.numeros[nombre] = numero
}

func main() {
	a := &Agenda{numeros: map[string]string{}}
	a.Agregar("Léa", "06 11")
	a.Agregar("Tom", "06 22")

	nombres := []string{"Léa", "Tom", "Léa", "Zoé"}
	encontrados := make([]string, len(nombres))

	var wg sync.WaitGroup
	for i, nombre := range nombres {
		wg.Add(1)
		go func() {
			defer wg.Done()
			encontrados[i] = nombre + " → " + a.Buscar(nombre)
		}()
	}
	wg.Wait()

	for _, e := range encontrados {
		fmt.Println(e)
	}
}
```

```sortie
Léa → 06 11
Tom → 06 22
Léa → 06 11
Zoé → 
```

Las cuatro búsquedas pueden hacerse **a la vez**: todas toman un `RLock`. `Zoé` no está
en el [map](maps.md): se recibe el texto vacío.

```text
 lectores:   Léa ─┐
             Tom ─┼──► RLock  RLock  RLock  RLock    (todos juntos)
             Léa ─┤
             Zoé ─┘
 escritor:   Agregar ──► Lock   (espera a que los lectores terminen,
                                 y luego pasa solo)
```

`defer a.mu.RUnlock()` devuelve el cerrojo al salir del método, incluso con un `return`
en medio (ver [defer](fonctions-avancees.md#defer-aplazar-hasta-el-final)).

Pruébalo: cambia `RLock`/`RUnlock` por `Lock`/`Unlock` en `Buscar`. El resultado es el
mismo, pero las búsquedas pasan una a una.

> **Recuerda:** usa un `RWMutex` cuando se lee **mucho más** de lo que se escribe (una
> caché, una configuración). Si no, un simple `Mutex` basta y es más sencillo.

## Una sola vez: sync.Once

Algunas cosas solo deben hacerse una vez: leer un archivo de configuración, abrir una
conexión. Es como encender el horno por la mañana: el primer cocinero lo enciende, los
siguientes ya lo encuentran caliente.

`once.Do(f)` llama a `f` la primera vez, y **nunca más** después.

```go
package main

import (
	"fmt"
	"sync"
)

var (
	once   sync.Once
	config map[string]string
)

func cargar() {
	fmt.Println("cargando la configuración…")
	config = map[string]string{"idioma": "es", "tema": "oscuro"}
}

func leer(clave string) string {
	once.Do(cargar)
	return config[clave]
}

func main() {
	fmt.Println(leer("idioma"))
	fmt.Println(leer("tema"))
	fmt.Println(leer("idioma"))
}
```

```sortie
cargando la configuración…
es
oscuro
es
```

| Llamada | ¿once ya hecho? | ¿se llama a cargar()? | Muestra |
|---|---|---|---|
| `leer("idioma")` | no | sí | el mensaje, y luego `es` |
| `leer("tema")` | sí | no | `oscuro` |
| `leer("idioma")` | sí | no | `es` |

### Con varias goroutines

`sync.Once` también funciona cuando diez goroutines llegan **a la vez**. Solo una ejecuta
la función; las demás esperan a que termine y luego continúan.

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	var once sync.Once
	var wg sync.WaitGroup

	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			once.Do(func() { fmt.Println("el horno está encendido") })
		}()
	}
	wg.Wait()
	fmt.Println("10 cocineros atendidos")
}
```

```sortie
el horno está encendido
10 cocineros atendidos
```

### sync.OnceValue: una vez, con un resultado

Desde Go 1.21, `sync.OnceValue(f)` devuelve una función. La primera llamada calcula el
valor; las siguientes lo devuelven sin volver a calcularlo.

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	precio := sync.OnceValue(func() int {
		fmt.Println("calculando el precio…")
		return 12 * 3
	})

	fmt.Println(precio())
	fmt.Println(precio())
}
```

```sortie
calculando el precio…
36
36
```

Pruébalo: añade un tercer `fmt.Println(precio())`. El mensaje `calculando el precio…`
sigue apareciendo una sola vez.

> **Recuerda:** `once.Do(f)` = «haz `f` la primera vez, ignora las siguientes». Es
> seguro incluso con muchas goroutines.

## Contar sin cerrojo: sync/atomic

Para un simple contador, un `Mutex` es algo pesado. El paquete `sync/atomic` ofrece
números cuyas operaciones son **indivisibles** («atómicas»): leer, sumar y escribir se
hacen en un solo paso que nadie puede cortar.

Es el contador de clics del portero de una discoteca: clic, +1. Dos porteros que hacen
clic a la vez nunca pierden un clic.

```go
package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

func main() {
	var visitas atomic.Int64
	var wg sync.WaitGroup

	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			visitas.Add(1)
		}()
	}
	wg.Wait()
	fmt.Println("visitas:", visitas.Load())
}
```

```sortie
visitas: 1000
```

Los métodos principales:

| Método | Lo que hace |
|---|---|
| `v.Add(n)` | suma `n` y devuelve el nuevo valor |
| `v.Load()` | lee el valor |
| `v.Store(n)` | reemplaza el valor |
| `v.CompareAndSwap(viejo, nuevo)` | reemplaza solo si el valor sigue siendo `viejo` |

Sigamos algunas llamadas, sin goroutines, para ver los valores devueltos:

```go
package main

import (
	"fmt"
	"sync/atomic"
)

func main() {
	var n atomic.Int64
	fmt.Println(n.Add(5))
	fmt.Println(n.Add(-2))
	n.Store(10)
	fmt.Println(n.Load())
	fmt.Println(n.CompareAndSwap(10, 20), n.Load())
	fmt.Println(n.CompareAndSwap(10, 99), n.Load())
}
```

```sortie
5
3
10
true 20
false 20
```

| Paso | Llamada | n después | Devuelve |
|---|---|---|---|
| 1 | `Add(5)` | 5 | 5 |
| 2 | `Add(-2)` | 3 | 3 |
| 3 | `Store(10)` | 10 | nada |
| 4 | `CompareAndSwap(10, 20)` | 20 | `true`: n valía 10 |
| 5 | `CompareAndSwap(10, 99)` | 20 | `false`: n vale 20, no 10 |

También existen `atomic.Int32`, `atomic.Uint64` y `atomic.Bool`. Su valor inicial es
cero (o `false`): no hace falta crearlos.

Pruébalo: cambia `visitas.Add(1)` por `visitas.Add(2)`. El programa muestra
`visitas: 2000`.

> **Recuerda:** para **un** número compartido, `atomic.Int64` basta. En cuanto haya que
> modificar varias cosas juntas (un map y un contador), vuelve a un `Mutex`.

## Esperar y obtener un error: errgroup

Un `WaitGroup` espera, pero no sabe nada de los errores. Si una goroutine falla, ¿cómo
saberlo? El paquete `errgroup` responde a esta pregunta.

No forma parte de la biblioteca estándar: está en el módulo `golang.org/x/sync`,
mantenido por el equipo de Go. Hay que añadirlo a tu proyecto (ver
[los módulos](modules.md)):

```sh
go get golang.org/x/sync/errgroup
```

El principio:

- `g.Go(f)` lanza `f` en una goroutine; `f` devuelve un `error`;
- `g.Wait()` espera a todas las goroutines y devuelve el **primer** error (o `nil`).

```go norun
package main

import (
	"errors"
	"fmt"

	"golang.org/x/sync/errgroup"
)

func descargar(nombre string) (int, error) {
	if nombre == "c.txt" {
		return 0, errors.New("c.txt: no encontrado")
	}
	return len(nombre) * 100, nil
}

func main() {
	archivos := []string{"a.txt", "b.txt", "c.txt"}
	tamanos := make([]int, len(archivos))

	var g errgroup.Group
	for i, nombre := range archivos {
		g.Go(func() error {
			t, err := descargar(nombre)
			if err != nil {
				return err
			}
			tamanos[i] = t
			return nil
		})
	}
	if err := g.Wait(); err != nil {
		fmt.Println("error:", err)
		return
	}
	fmt.Println("tamaños:", tamanos)
}
```

```text
error: c.txt: no encontrado
```

| Goroutine | Archivo | Devuelve |
|---|---|---|
| 1 | `a.txt` | `nil` (tamaño 500) |
| 2 | `b.txt` | `nil` (tamaño 500) |
| 3 | `c.txt` | `c.txt: no encontrado` |
| `g.Wait()` | | el primer error: `c.txt: no encontrado` |

Pruébalo: cambia `"c.txt"` por `"d.txt"` en la lista. Ya nada falla, y el programa
muestra `tamaños: [500 500 500]`.

Compara con un `WaitGroup`: ya no hay `Add` ni `Done`. `errgroup` los hace por ti.

### Detener a las demás con WithContext

`errgroup.WithContext(ctx)` da un grupo **y** un [context](context.md). En cuanto una
goroutine devuelve un error, ese contexto se cancela: las demás pueden detenerse.
`g.SetLimit(n)` limita cuántas goroutines funcionan a la vez.

```go norun
g, ctx := errgroup.WithContext(context.Background())
g.SetLimit(3) // como mucho 3 descargas a la vez
for _, url := range urls {
	g.Go(func() error {
		return descargarCon(ctx, url) // se detiene si ctx se cancela
	})
}
err := g.Wait()
```

> **Recuerda:** `errgroup` = un `WaitGroup` que informa del primer error. Con
> `WithContext`, un error también avisa a todas las demás goroutines.

## ¿Qué herramienta elegir?

```text
¿Las goroutines comparten un dato?
├── no ─► WaitGroup (o errgroup si pueden fallar)
└── sí
    ├── ¿es un simple número? ─► atomic.Int64
    ├── ¿se lee mucho y se escribe poco? ─► RWMutex
    ├── ¿se inicializa una sola vez? ─► sync.Once
    └── si no ─► Mutex (o un canal, ver la página de canales)
```

No olvides los [canales](canaux.md): en lugar de compartir un dato, una goroutine puede
**enviarlo** a otra. El proverbio de Go dice: «no comuniques compartiendo memoria;
comparte memoria comunicando».

> **Recuerda:** usa la herramienta más sencilla que baste. Y comprueba con `-race`.

## Errores frecuentes

### Copiar una estructura que contiene un Mutex

```go norun
package main

import (
	"fmt"
	"sync"
)

type Contador struct {
	mu sync.Mutex
	n  int
}

func mostrar(c Contador) {
	fmt.Println(c.n)
}

func main() {
	var c Contador
	mostrar(c)
}
```

El programa compila, pero `go vet main.go` protesta:

```text
# command-line-arguments
# [command-line-arguments]
./main.go:13:16: mostrar passes lock by value: command-line-arguments.Contador contains sync.Mutex
./main.go:19:10: call of mostrar copies lock value: command-line-arguments.Contador contains sync.Mutex
```

Una copia del cerrojo es **otro** cerrojo: ya no protege nada. Pasa un puntero:
`func mostrar(c *Contador)` y `mostrar(&c)` (ver [los punteros](pointeurs.md)).

### Devolver una llave que no se ha tomado

```go norun
var mu sync.Mutex
mu.Lock()
fmt.Println("dentro de la sección protegida")
mu.Unlock()
mu.Unlock()
```

```text
dentro de la sección protegida
fatal error: sync: unlock of unlocked mutex

goroutine 1 [running]:
…
exit status 2
```

Un `Unlock` de más detiene el programa. Escribe `defer mu.Unlock()` justo después de
`mu.Lock()`: siempre habrá exactamente un `Unlock`.

### Tomar dos veces el mismo cerrojo

```go norun
var mu sync.RWMutex
mu.RLock()
fmt.Println("lectura en curso")
mu.Lock()
fmt.Println("nunca se muestra")
```

```text
lectura en curso
fatal error: all goroutines are asleep - deadlock!

goroutine 1 [sync.RWMutex.Lock]:
…
exit status 2
```

`Lock()` espera a que todos los lectores terminen… pero el lector eres tú. Nadie puede
avanzar: es un **interbloqueo** (*deadlock*). Los cerrojos de Go no son «reentrantes»:
una goroutine que ya tiene el cerrojo no puede volver a tomarlo. Devuelve primero el
`RLock` y luego toma el `Lock`.

## Ponte a prueba

**Pregunta 1:** ¿por qué `contador++` en 1000 goroutines puede dar menos de 1000? ¿Cómo
verlo?

> **Respuesta**
>
> `contador++` se hace en tres pasos (leer, sumar, escribir). Dos goroutines pueden leer
> el mismo valor y escribir el mismo resultado: se pierde una suma. `go run -race`
> muestra entonces `WARNING: DATA RACE`.

**Pregunta 2:** ¿qué muestra este programa?

```go norun
var once sync.Once
for i := 0; i < 3; i++ {
	once.Do(func() { fmt.Println("vuelta", i) })
}
```

> **Respuesta**
>
> Solo `vuelta 0`. La función pasada a `once.Do` solo se llama en la primera vuelta;
> las dos siguientes la ignoran.

**Pregunta 3:** una lista de precios se lee miles de veces por segundo y se actualiza una
vez por hora. ¿Qué cerrojo elegir?

> **Respuesta**
>
> Un `sync.RWMutex`: `RLock` para las lecturas, que pasan todas juntas, y `Lock` para la
> actualización, que es rara.

## En resumen

- Una condición de carrera pierde datos sin avisar: usa `-race`.
- `Mutex`: uno solo a la vez. `RWMutex`: varios lectores **o** un solo escritor.
- `sync.Once` (y `sync.OnceValue`) hace una inicialización una sola vez.
- `atomic.Int64` cuenta sin cerrojo con `Add` y `Load`.
- `errgroup` (módulo `golang.org/x/sync`) espera goroutines y devuelve el primer error.
- No copies nunca un cerrojo; devuélvelo con `defer`.

Siguiente: [leer y escribir archivos](fichiers.md).
