# Los punteros

Esta página explica qué es un puntero, cómo usarlo con `&` y `*`, y cuándo conviene usarlo.

> **Objetivos:**
>
> - sabrás qué es una dirección y un puntero;
> - sabrás usar `&` para tomar una dirección y `*` para seguirla;
> - sabrás por qué una función modifica (o no) una variable;
> - sabrás evitar el pánico del puntero `nil`;
> - sabrás cuándo elegir un puntero.

## Una dirección en lugar de una copia

Cada variable está guardada en algún lugar de la
[memoria RAM](../debuter/glossaire.md#memoria-ram), en una **dirección**. Un
[puntero](../debuter/glossaire.md#puntero) es una variable que contiene esa dirección.

Piensa en una casa. Puedes darle a un amigo **una foto** de tu casa: puede colorearla, y
tu casa no cambia. O puedes darle **tu dirección**: entonces puede venir a pintar la
puerta de verdad. La foto es una copia. La dirección es un puntero.

## & y *

Bastan dos símbolos:

- `&x` da la dirección de `x` («¿dónde vive `x`?»);
- `*p` sigue la dirección guardada en `p` y da el valor que hay allí («ve a mirar a esa
  dirección»).

El tipo de un puntero a un `int` se escribe `*int`.

```go
package main

import "fmt"

func main() {
	x := 10
	p := &x

	fmt.Println("x vale", x)
	fmt.Println("p apunta a", *p)

	*p = 20
	fmt.Println("x vale ahora", x)
	fmt.Printf("p es de tipo %T\n", p)
}
```

```sortie
x vale 10
p apunta a 10
x vale ahora 20
p es de tipo *int
```

Escribir `*p = 20` cambia el valor **en la dirección** de `x`. Así que es `x` lo que
cambia.

> **Nota:** si muestras `p` en sí, Go enseña una dirección como `0xc000012345`.
> Cambia de una ejecución a otra: nunca cuentes con ella.

## Pasar por valor o por puntero

En Go, una función siempre recibe una **copia** de sus
[argumentos](../debuter/glossaire.md#argumento). Es el paso **por valor**. Por eso la
función no puede modificar la variable de quien la llama.

Para que pueda, pasa la dirección: la función recibe una copia de la dirección, pero
esa dirección lleva a la variable de verdad.

```go
package main

import "fmt"

func duplicarCopia(n int) {
	n = n * 2
}

func duplicar(n *int) {
	*n = *n * 2
}

func main() {
	x := 5
	duplicarCopia(x)
	fmt.Println("Después de duplicarCopia:", x)

	duplicar(&x)
	fmt.Println("Después de duplicar:", x)
}
```

```sortie
Después de duplicarCopia: 5
Después de duplicar: 10
```

### El caso de las estructuras

Una estructura pasada por valor se copia entera. Con un puntero a una estructura, Go te
ahorra escribir `(*p).Campo`: basta con `p.Campo`.

```go
package main

import "fmt"

type Jugador struct {
	Nombre string
	Vidas  int
}

func perderUnaVida(j *Jugador) {
	j.Vidas--
}

func main() {
	j := Jugador{Nombre: "Léa", Vidas: 3}
	perderUnaVida(&j)
	perderUnaVida(&j)
	fmt.Printf("%+v\n", j)
}
```

```sortie
{Nombre:Léa Vidas:1}
```

## Los receptores puntero

Un método es una función como cualquier otra: su receptor sigue la misma regla. Con
`func (c *Cuenta)`, el método modifica el original. Lo viste en
[estructuras y métodos](types.md).

```go
package main

import "fmt"

type Cuenta struct {
	Saldo int
}

func (c *Cuenta) Depositar(cantidad int) {
	c.Saldo += cantidad
}

func (c Cuenta) Mostrar() {
	fmt.Println("Saldo:", c.Saldo)
}

func main() {
	c := Cuenta{}
	c.Depositar(50)
	c.Depositar(25)
	c.Mostrar()
}
```

```sortie
Saldo: 75
```

`Depositar` modifica la cuenta: receptor puntero. `Mostrar` solo lee: basta con un
receptor valor.

## El puntero nil

Un puntero que no apunta a nada vale `nil`. Es el valor cero de los punteros:
`var p *int` vale `nil`. Seguir un puntero `nil` es imposible: no hay ninguna casa en
esa dirección.

Así que comprueba `p != nil` antes de seguir un puntero que puede estar vacío.

```go
package main

import "fmt"

type Jugador struct {
	Nombre string
}

func nombreDe(j *Jugador) string {
	if j == nil {
		return "(nadie)"
	}
	return j.Nombre
}

func main() {
	var ausente *Jugador
	presente := &Jugador{Nombre: "Sam"}

	fmt.Println(ausente == nil)
	fmt.Println(nombreDe(ausente))
	fmt.Println(nombreDe(presente))
}
```

```sortie
true
(nadie)
Sam
```

## new y &T{}

`new(T)` reserva un valor cero de tipo `T` y devuelve su dirección. En la práctica, se
escribe más a menudo `&T{...}`, que permite rellenar los campos enseguida.

```go
package main

import "fmt"

type Punto struct {
	X, Y int
}

func main() {
	n := new(int)
	*n = 7
	fmt.Println(*n)

	p := new(Punto)
	p.X = 3
	fmt.Println(*p)

	q := &Punto{X: 1, Y: 2}
	fmt.Println(*q)
}
```

```sortie
7
{3 0}
{1 2}
```

> **Nota:** en Go, una función puede devolver la dirección de una variable local, como
> `return &Punto{}`. Go mantiene el valor vivo mientras se use: no hay nada que liberar
> a mano.

## ¿Cuándo usar un puntero?

Usa un puntero:

- cuando una función o un método debe **modificar** el valor que recibe;
- cuando la estructura es **grande** y copiarla saldría caro;
- cuando «sin valor» debe ser posible: el puntero vale entonces `nil`.

Quédate con un valor simple en los demás casos, sobre todo para los tipos pequeños
(`int`, `string`, un `Punto`). Un valor no puede ser `nil`, así que no puede provocar un
pánico.

> **Recuerda:** los slices, los maps y los canales ya contienen una especie de puntero
> oculto. Una función que recibe un map puede modificarlo sin `*` (mira
> [los maps](maps.md)).

```go
package main

import "fmt"

func agregar(stock map[string]int, fruta string) {
	stock[fruta]++
}

func main() {
	stock := map[string]int{}
	agregar(stock, "manzana")
	agregar(stock, "manzana")
	fmt.Println(stock["manzana"])
}
```

```sortie
2
```

## Errores frecuentes

### Seguir un puntero nil

```go norun
package main

import "fmt"

type Jugador struct {
	Nombre string
}

func main() {
	var j *Jugador
	fmt.Println(j.Nombre)
}
```

```text
panic: runtime error: invalid memory address or nil pointer dereference
[signal SIGSEGV: segmentation violation code=0x1 addr=0x0 pc=0x491af6]

goroutine 1 [running]:
main.main()
	/ruta/a/main.go:11 +0x16
exit status 2
```

`j` vale `nil`: no hay ningún jugador en esa dirección. El mensaje «nil pointer
dereference» quiere decir «seguir un puntero nil». Crea el valor
(`j := &Jugador{Nombre: "Sam"}`) o comprueba `j != nil` antes.

### Olvidar el &

```go norun
package main

import "fmt"

func duplicar(n *int) {
	*n = *n * 2
}

func main() {
	x := 5
	duplicar(x)
	fmt.Println(x)
}
```

```text
# command-line-arguments
./main.go:11:11: cannot use x (variable of type int) as *int value in argument to duplicar
```

La función espera una dirección (`*int`), no un `int`. Escribe `duplicar(&x)`.

### Hacer cálculos con una dirección

```go norun
package main

import "fmt"

func main() {
	x := 5
	p := &x
	fmt.Println(p + 1)
}
```

```text
# command-line-arguments
./main.go:8:14: invalid operation: p + 1 (mismatched types *int and untyped int)
```

Go prohíbe los cálculos con direcciones. Seguramente querías el valor apuntado:
`*p + 1`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este programa?

```go norun
x := 1
p := &x
*p = *p + 10
fmt.Println(x)
```

> **Respuesta**
>
> `11`. `p` contiene la dirección de `x`, así que `*p = ...` modifica `x`.

**Pregunta 2:** ¿cuál es el valor cero de una variable `var p *string`?

> **Respuesta**
>
> `nil`. El puntero no apunta a nada; escribir `*p` provocaría un pánico.

**Pregunta 3:** escribe una función `intercambiar(a, b *int)` que intercambie los
valores de dos variables.

> **Respuesta**
>
> ```go
> package main
>
> import "fmt"
>
> func intercambiar(a, b *int) {
> 	*a, *b = *b, *a
> }
>
> func main() {
> 	x, y := 1, 2
> 	intercambiar(&x, &y)
> 	fmt.Println(x, y)
> }
> ```
>
> ```sortie
> 2 1
> ```

## En resumen

- Un puntero contiene la dirección de un valor; su tipo se escribe `*T`.
- `&x` toma la dirección de `x`; `*p` lee o modifica el valor apuntado.
- Go lo pasa todo por valor; pasa un puntero para modificar el original.
- Un puntero `nil` no apunta a nada: seguirlo hace que el programa entre en pánico.
- `new(T)` y `&T{...}` crean un valor y devuelven su dirección.

Siguiente: [los genéricos](generiques.md).
