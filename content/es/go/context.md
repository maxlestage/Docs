# Cancelar y limitar en el tiempo con context

Esta página muestra cómo el paquete `context` permite decirle a una goroutine «detente, ya nadie espera tu trabajo», enseguida o al cabo de cierto tiempo.

> **Objetivos:**
>
> - entender para qué sirve un `context`;
> - cancelar un trabajo con `context.WithCancel`;
> - fijar un tiempo máximo con `context.WithTimeout`;
> - escuchar `ctx.Done()` en un `select` y leer `ctx.Err()`;
> - pasar `ctx` como primer parámetro de tus funciones.

Este capítulo supone que conoces [las goroutines](concurrence.md) y
[los canales](canaux.md), sobre todo `select`.

## ¿Por qué cancelar un trabajo?

Imagina una pizzería. Pides una pizza por teléfono y luego cuelgas: has cambiado de
idea. Si nadie avisa al pizzero, sigue horneando una pizza que nadie se comerá.

Un programa tiene el mismo problema. Una [goroutine](../debuter/glossaire.md#goroutine)
lanzada para un trabajo sigue adelante, aunque ya nadie espere el resultado:

- el usuario cerró la página web;
- la respuesta llega demasiado tarde;
- otra goroutine ya encontró la respuesta.

El **context** (el «contexto») es el teléfono que permite avisar al pizzero: «para,
déjalo».

```text
 main ──── ctx ────► goroutine
   │                    │ trabaja…
 cancel() ─────────►  ctx.Done() se cierra
                        │ «me dijeron que parara»
                        └─ se detiene
```

> **Recuerda:** un `context` transporta una señal de «detente». Quien lanza el trabajo
> la envía; quien trabaja la escucha.

## El punto de partida: context.Background

Todo contexto parte de una raíz: `context.Background()`. Es un contexto vacío. Nunca se
cancela.

```go
package main

import (
	"context"
	"fmt"
)

func main() {
	ctx := context.Background()
	fmt.Println(ctx)
	fmt.Println(ctx.Err())
}
```

```sortie
context.Background
<nil>
```

`ctx.Err()` dice si el contexto ha terminado. `<nil>` quiere decir «no, todo va bien».

> **Nota:** por costumbre, la variable siempre se llama `ctx`.

## Cancelar a mano: WithCancel

`context.WithCancel(padre)` devuelve **dos** cosas:

- un nuevo contexto `ctx`, hijo de `padre`;
- una función `cancel`. Llamarla cancela `ctx`.

```go
package main

import (
	"context"
	"fmt"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	fmt.Println("antes:", ctx.Err())
	cancel()
	fmt.Println("después:", ctx.Err())
}
```

```sortie
antes: <nil>
después: context canceled
```

| Paso | Qué pasa | ctx.Err() |
|---|---|---|
| 1 | `WithCancel` crea `ctx` | `<nil>` |
| 2 | `cancel()` | `context canceled` |

Llamar a `cancel` una segunda vez no hace nada más. No es un error.

## Escuchar ctx.Done() en un select

`ctx.Done()` es un [canal](../debuter/glossaire.md#canal). Se **cierra** cuando el
contexto se cancela. Recibir de un canal cerrado funciona enseguida: esa es la señal.

Aquí hay un trabajador que espera. `main` lo cancela y luego espera a que termine.

```go
package main

import (
	"context"
	"fmt"
)

func trabajador(ctx context.Context, listo chan<- bool) {
	<-ctx.Done()
	fmt.Println("trabajador: me dijeron que parara:", ctx.Err())
	listo <- true
}

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	listo := make(chan bool)

	go trabajador(ctx, listo)
	fmt.Println("main: cancelo")
	cancel()
	<-listo
	fmt.Println("main: terminado")
}
```

```sortie
main: cancelo
trabajador: me dijeron que parara: context canceled
main: terminado
```

El trabajador no puede mostrar nada antes de `cancel()`: está bloqueado en
`<-ctx.Done()`. Por eso el orden es siempre el mismo.

### Un productor que detenemos

En la realidad, un trabajador hace algo más que esperar. Entonces pone `ctx.Done()` en
un `select`, junto a su trabajo. Aquí, un productor envía 1, 2, 3, 4… sin fin. `main`
lee tres y luego cancela.

```go
package main

import (
	"context"
	"fmt"
)

func productor(ctx context.Context, salida chan<- int, listo chan<- bool) {
	for n := 1; ; n++ {
		select {
		case salida <- n:
			// enviado, seguimos con el siguiente
		case <-ctx.Done():
			fmt.Println("productor: detenido")
			listo <- true
			return
		}
	}
}

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	numeros := make(chan int)
	listo := make(chan bool)

	go productor(ctx, numeros, listo)
	for i := 0; i < 3; i++ {
		fmt.Println("recibido:", <-numeros)
	}
	cancel()
	<-listo
}
```

```sortie
recibido: 1
recibido: 2
recibido: 3
productor: detenido
```

Paso a paso:

| Paso | main | productor |
|---|---|---|
| 1 | recibe 1 | envía 1 |
| 2 | recibe 2 | envía 2 |
| 3 | recibe 3 | envía 3 |
| 4 | llama a `cancel()` | espera en `select` para enviar 4 |
| 5 | espera `listo` | `ctx.Done()` está listo: se detiene |

El 4 nunca se envía: ya nadie lee `numeros`. Solo el caso `ctx.Done()` puede ganar.

Pruébalo: cambia `i < 3` por `i < 5`. Recibes del 1 al 5 y luego `productor: detenido`.

> **Recuerda:** un trabajador que se puede cancelar casi siempre contiene
> `case <-ctx.Done():` en su `select`.

## Llamar siempre a cancel con defer

Un contexto cancelable guarda un poco de [memoria](../debuter/glossaire.md#memoire-vive)
mientras no se cancela. La regla es sencilla: justo después de crear el contexto,
escribe `defer cancel()`.

```go norun
ctx, cancel := context.WithCancel(context.Background())
defer cancel() // cancelado al final de la función, pase lo que pase
```

`defer` aplaza la llamada hasta el final de la función (ver
[defer](fonctions-avancees.md#defer-aplazar-hasta-el-final)). Aunque la función termine
antes, el contexto se limpiará.

## Un tiempo máximo: WithTimeout

`context.WithTimeout(padre, duración)` crea un contexto que se cancela **solo** al cabo
de la duración. Es un temporizador de cocina: cuando suena, se para.

El trabajador de abajo espera un resultado en un canal que **nunca** recibe nada. Así
que siempre gana el plazo.

```go
package main

import (
	"context"
	"fmt"
	"time"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancel()

	nunca := make(chan int)
	select {
	case r := <-nunca:
		fmt.Println("resultado:", r)
	case <-ctx.Done():
		fmt.Println("abandono:", ctx.Err())
	}
}
```

```sortie
abandono: context deadline exceeded
```

`deadline exceeded` quiere decir «plazo superado». Es la segunda razón posible de
`ctx.Err()`.

Al revés, si el resultado ya está ahí, pasa antes que el plazo:

```go
package main

import (
	"context"
	"fmt"
	"time"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	resultado := make(chan int, 1)
	resultado <- 42

	select {
	case r := <-resultado:
		fmt.Println("resultado:", r)
	case <-ctx.Done():
		fmt.Println("abandono:", ctx.Err())
	}
	fmt.Println("ctx.Err():", ctx.Err())
}
```

```sortie
resultado: 42
ctx.Err(): <nil>
```

El plazo de un segundo no ha pasado: `ctx.Err()` sigue valiendo `<nil>`.

> **Nota:** `WithTimeout` se parece a `time.After` (ver
> [un tiempo máximo con time.After](canaux.md)). La diferencia: el contexto se
> **transmite** a otras funciones, que se detienen todas a la vez.

## Leer la razón: ctx.Err()

`ctx.Err()` tiene tres valores posibles:

| Situación | ctx.Err() |
|---|---|
| el contexto sigue en marcha | `nil` |
| alguien llamó a `cancel()` | `context.Canceled` |
| el plazo se ha superado | `context.DeadlineExceeded` |

Para comprobar la razón, usa `errors.Is`.

```go
package main

import (
	"context"
	"errors"
	"fmt"
	"time"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel()
	<-ctx.Done()

	err := ctx.Err()
	if errors.Is(err, context.DeadlineExceeded) {
		fmt.Println("demasiado lento")
	} else if errors.Is(err, context.Canceled) {
		fmt.Println("cancelado")
	}
}
```

```sortie
demasiado lento
```

## Pasar ctx como primer parámetro

Una función que puede tardar recibe el contexto como **primer** parámetro, llamado
`ctx`. Es una convención que sigue toda la biblioteca de Go.

```go
package main

import (
	"context"
	"fmt"
	"time"
)

// buscar espera una respuesta que nunca llega: solo ctx puede detenerla.
func buscar(ctx context.Context, palabra string) (string, error) {
	respuesta := make(chan string)
	select {
	case r := <-respuesta:
		return r, nil
	case <-ctx.Done():
		return "", ctx.Err()
	}
}

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 20*time.Millisecond)
	defer cancel()

	_, err := buscar(ctx, "gopher")
	if err != nil {
		fmt.Println("buscar falló:", err)
	}
}
```

```sortie
buscar falló: context deadline exceeded
```

El contexto baja de función en función, como el teléfono pasa de mano en mano. Si
`buscar` llamara a otras funciones, les daría el mismo `ctx`.

> **Recuerda:** `func nombre(ctx context.Context, …)`. No guardes un contexto en una
> estructura: pásalo en cada llamada.

## Los hijos siguen al padre

Un contexto creado a partir de otro es su **hijo**. Cancelar el padre cancela todos sus
hijos. Lo contrario no es cierto.

```go
package main

import (
	"context"
	"fmt"
)

func main() {
	padre, cancelarPadre := context.WithCancel(context.Background())
	hijo, cancelarHijo := context.WithCancel(padre)
	defer cancelarHijo()

	cancelarPadre()
	<-hijo.Done()
	fmt.Println("hijo:", hijo.Err())
}
```

```sortie
hijo: context canceled
```

```text
Background
   └── padre   ← cancel aquí…
         └── hijo   ← …también cancela este
```

## Transportar un valor: WithValue (con moderación)

`context.WithValue(padre, clave, valor)` cuelga una pequeña etiqueta del contexto. Se
usa para una información que acompaña a una petición, como un número de seguimiento.

```go
package main

import (
	"context"
	"fmt"
)

type clave string

const claveSeguimiento clave = "seguimiento"

func procesar(ctx context.Context) {
	numero, ok := ctx.Value(claveSeguimiento).(string)
	if !ok {
		numero = "desconocido"
	}
	fmt.Println("procesando la petición", numero)
}

func main() {
	ctx := context.WithValue(context.Background(), claveSeguimiento, "A-42")
	procesar(ctx)
	procesar(context.Background())
}
```

```sortie
procesando la petición A-42
procesando la petición desconocido
```

- La clave tiene su propio tipo (`clave`). Así no puede mezclarse con la clave de otro
  paquete.
- `ctx.Value` devuelve un valor de tipo `any`. El `.(string)` comprueba que es un texto
  (ver [las interfaces](interfaces.md)).

> **Atención:** no uses `WithValue` para pasar los parámetros normales de una función.
> Un parámetro escrito en la firma se lee mucho mejor.

## Errores frecuentes

### Tirar la función cancel

```go norun
package main

import (
	"context"
	"fmt"
	"time"
)

func main() {
	ctx, _ := context.WithTimeout(context.Background(), time.Second)
	fmt.Println(ctx.Err())
}
```

El programa compila, pero `go vet main.go` te avisa:

```text
# command-line-arguments
# [command-line-arguments]
./main.go:10:7: the cancel function returned by context.WithTimeout should be called, not discarded, to avoid a context leak
```

Guarda `cancel` y escribe `defer cancel()` justo debajo.

### Pasar un contexto nil

```go norun
package main

import (
	"context"
	"fmt"
	"time"
)

func main() {
	var ctx context.Context
	ctx, cancel := context.WithTimeout(ctx, time.Second)
	defer cancel()
	fmt.Println(ctx.Err())
}
```

```text
panic: cannot create context from nil parent

goroutine 1 [running]:
context.WithDeadlineCause({0x0?, 0x0?}, {0x57dbe0?, 0x3b9aca00?, 0x57dbe0?}, {0x0?, 0x0?})
	/usr/local/go/src/context/context.go:625 +0x39e
…
exit status 2
```

Un contexto declarado con `var` vale `nil`. Parte siempre de `context.Background()`.

### Olvidar escuchar ctx.Done()

```go norun
func productor(ctx context.Context, salida chan<- int) {
	for n := 1; ; n++ {
		salida <- n // nunca se mira ctx
	}
}
```

Aquí no se muestra ningún error. Pero cancelar `ctx` no sirve de nada: la goroutine se
queda bloqueada en `salida <- n` para siempre. Recibir `ctx` no basta, hay que
**escucharlo** en un `select`, como en [el productor](#un-productor-que-detenemos).

## Ponte a prueba

**Pregunta 1:** ¿cuánto vale `ctx.Err()` justo después de `context.WithCancel(...)`, y
luego después de `cancel()`?

> **Respuesta**
>
> Primero `nil` (el contexto está en marcha). Después de `cancel()`, `context canceled`.

**Pregunta 2:** ¿qué muestra este programa?

```go norun
ctx, cancel := context.WithTimeout(context.Background(), 30*time.Millisecond)
defer cancel()
nunca := make(chan string)
select {
case m := <-nunca:
	fmt.Println(m)
case <-ctx.Done():
	fmt.Println(ctx.Err())
}
```

> **Respuesta**
>
> `context deadline exceeded`. El canal `nunca` no recibe nada: al cabo de 30 ms, el
> contexto se cancela y gana el segundo caso.

**Pregunta 3:** ¿dónde se coloca el contexto en la lista de parámetros de una función?

> **Respuesta**
>
> En primer lugar, con el nombre `ctx`: `func descargar(ctx context.Context, url string)`.

## En resumen

- Un `context` transporta la señal de «detente» hacia las goroutines.
- Todo parte de `context.Background()`.
- `WithCancel` da una función `cancel`; `WithTimeout` se cancela solo. Escribe siempre
  `defer cancel()`.
- El trabajador escucha `case <-ctx.Done():` en un `select`; `ctx.Err()` da la razón
  (`Canceled` o `DeadlineExceeded`).
- `ctx` se pasa como primer parámetro; `WithValue` queda para pequeñas informaciones.

Siguiente: [sync: Once, RWMutex, atomic y errgroup](sync.md)
