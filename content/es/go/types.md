# Estructuras y métodos

Esta página muestra cómo crear tus propios tipos en Go con `struct`, darles métodos y combinarlos mediante la incrustación.

> **Objetivos:**
>
> - sabrás definir una estructura y crear valores de ese tipo;
> - sabrás leer y modificar los campos de una estructura;
> - sabrás escribir métodos, con un receptor valor o puntero;
> - sabrás escribir un constructor `NewX`;
> - sabrás reutilizar una estructura dentro de otra mediante la incrustación.

## Definir una estructura

Una [estructura](../debuter/glossaire.md#structure) agrupa varios valores bajo un solo
nombre. Piensa en una ficha de inscripción: tiene casillas (nombre, edad, ciudad), y
cada persona rellena su propia ficha. La ficha en blanco es el **tipo**, cada ficha
rellenada es un **valor** de ese tipo.

En Go, se escribe `type`, el nombre, luego `struct` y la lista de **campos** entre
llaves. Cada campo tiene un nombre y un [tipo](../debuter/glossaire.md#type).

```go
package main

import "fmt"

type Persona struct {
	Nombre string
	Edad   int
	Ciudad string
}

func main() {
	alicia := Persona{Nombre: "Alicia", Edad: 30, Ciudad: "Lyon"}
	fmt.Println(alicia)
}
```

```sortie
{Alicia 30 Lyon}
```

La estructura se declara **fuera** de `main`, a nivel del archivo. El nombre del tipo
suele empezar por mayúscula: verás por qué en [paquetes y módulos](modules.md).

## Crear valores

Hay varias formas de escribir un valor de estructura, llamado **literal**.

- Con los nombres de los campos: `Persona{Nombre: "Alicia", Edad: 30}`. Es la forma
  recomendada: el orden no importa y el código se lee solo.
- Sin los nombres: `Persona{"Bob", 25, "París"}`. Entonces hay que dar **todos** los
  campos, en orden.
- Vacío: `Persona{}`. Cada campo recibe su **valor cero**: `""` para un texto, `0`
  para un número, `false` para un booleano.

```go
package main

import "fmt"

type Persona struct {
	Nombre string
	Edad   int
	Ciudad string
}

func main() {
	a := Persona{Nombre: "Alicia", Edad: 30}
	b := Persona{"Bob", 25, "París"}
	var c Persona

	fmt.Println(a)
	fmt.Println(b)
	fmt.Println(c)
	fmt.Printf("%+v\n", a)
	fmt.Printf("%#v\n", b)
}
```

```sortie
{Alicia 30 }
{Bob 25 París}
{ 0 }
{Nombre:Alicia Edad:30 Ciudad:}
main.Persona{Nombre:"Bob", Edad:25, Ciudad:"París"}
```

> **Consejo:** para mostrar una estructura mientras buscas un error, usa `%+v`: muestra
> el nombre de cada campo. `%#v` muestra además el nombre del tipo.

## Leer y modificar los campos

Un punto `.` da acceso a un campo: `alicia.Edad`. Puedes leerlo y darle un nuevo
valor.

```go
package main

import "fmt"

type Cuenta struct {
	Titular string
	Saldo   float64
}

func main() {
	c := Cuenta{Titular: "Alicia", Saldo: 100}
	c.Saldo = c.Saldo + 50
	fmt.Println(c.Titular, "tiene", c.Saldo, "euros")
}
```

```sortie
Alicia tiene 150 euros
```

### Una copia es independiente

Cuando asignas una estructura a otra variable, Go **copia** todas las casillas.
Modificar la copia no toca el original: es como fotocopiar la ficha.

```go
package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	a := Point{X: 1, Y: 2}
	b := a
	b.X = 100
	fmt.Println(a, b)
}
```

```sortie
{1 2} {100 2}
```

`X, Y int` declara dos campos del mismo tipo en una sola línea.

## Comparar dos estructuras

Dos estructuras del mismo tipo se comparan con `==`: son iguales si **todos** sus
campos son iguales. Funciona mientras los campos sean a su vez comparables (números,
textos, booleanos… pero no los slices ni los maps).

```go
package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	a := Point{1, 2}
	b := Point{1, 2}
	c := Point{2, 1}
	fmt.Println(a == b)
	fmt.Println(a == c)
}
```

```sortie
true
false
```

## Los métodos

Un [método](../debuter/glossaire.md#methode) es una función unida a un tipo. Se
escribe como una función, con además un **receptor** entre paréntesis antes del
nombre: `func (r Rectangulo) Area()`. El receptor es el valor sobre el que se llama al
método. Por costumbre, se le da un nombre corto: la primera letra del tipo.

```go
package main

import "fmt"

type Rectangulo struct {
	Ancho, Alto float64
}

func (r Rectangulo) Area() float64 {
	return r.Ancho * r.Alto
}

func (r Rectangulo) Perimetro() float64 {
	return 2 * (r.Ancho + r.Alto)
}

func main() {
	r := Rectangulo{Ancho: 3, Alto: 4}
	fmt.Println("Área:", r.Area())
	fmt.Println("Perímetro:", r.Perimetro())
}
```

```sortie
Área: 12
Perímetro: 14
```

Go no tiene clases: una estructura y sus métodos cumplen el mismo papel. Compara con
[las clases en Ruby](../ruby/types.md).

## Receptor valor o puntero

Con `func (r Rectangulo)`, el método recibe una **copia** del valor. Puede leer los
campos, pero una modificación se pierde al terminar el método.

Para modificar el valor original, escribe un asterisco: `func (c *Contador)`. El método
recibe entonces la **dirección** del valor, un [puntero](../debuter/glossaire.md#pointeur),
y trabaja sobre el original. El capítulo [los punteros](pointeurs.md) lo explica en
detalle.

```go
package main

import "fmt"

type Contador struct {
	Valor int
}

func (c Contador) IncrementarCopia() {
	c.Valor++
}

func (c *Contador) Incrementar() {
	c.Valor++
}

func main() {
	c := Contador{}
	c.IncrementarCopia()
	fmt.Println("Después de la copia:", c.Valor)
	c.Incrementar()
	c.Incrementar()
	fmt.Println("Después del puntero:", c.Valor)
}
```

```sortie
Después de la copia: 0
Después del puntero: 2
```

No necesitas escribir `(&c).Incrementar()`: Go toma la dirección solo cuando escribes
`c.Incrementar()`.

> **Recuerda:** un método que **modifica** la estructura toma un receptor puntero `*T`.
> Si un solo método del tipo tiene un receptor puntero, dale uno a todos: es más
> coherente.

## Los constructores NewX

Go no tiene palabra clave para construir un objeto. La costumbre es escribir una
función `NewNombreDelTipo` que prepara un valor válido y lo devuelve. Se le llama
**constructor**. A menudo devuelve un puntero.

```go
package main

import (
	"fmt"
	"strings"
)

type Jugador struct {
	Nombre string
	Vidas  int
	Puntos int
}

func NewJugador(nombre string) *Jugador {
	return &Jugador{
		Nombre: strings.TrimSpace(nombre),
		Vidas:  3,
	}
}

func main() {
	j := NewJugador("  Zoé ")
	j.Puntos += 10
	fmt.Printf("%+v\n", *j)
}
```

```sortie
{Nombre:Zoé Vidas:3 Puntos:10}
```

El constructor garantiza que un jugador empieza siempre con 3 vidas y un nombre
limpio. Fíjate en la coma después del último campo: cuando un literal ocupa varias
líneas, Go la exige.

## La incrustación

Go no tiene [herencia](../debuter/glossaire.md#heritage). En su lugar, una estructura
puede **incrustar** otra: se escribe el tipo solo, sin nombre de campo. Los campos y
los métodos del tipo incrustado pasan a ser accesibles directamente.

Es como un coche que contiene un motor: se dice «el coche arranca», aunque sea el
motor el que hace el trabajo.

```go
package main

import "fmt"

type Animal struct {
	Nombre string
}

func (a Animal) Presentarse() string {
	return "Me llamo " + a.Nombre
}

type Perro struct {
	Animal
	Raza string
}

func main() {
	rex := Perro{Animal: Animal{Nombre: "Rex"}, Raza: "labrador"}
	fmt.Println(rex.Nombre)
	fmt.Println(rex.Presentarse())
	fmt.Println(rex.Animal.Nombre)
}
```

```sortie
Rex
Me llamo Rex
Rex
```

`rex.Nombre` es un atajo de `rex.Animal.Nombre`. El campo incrustado lleva el nombre
de su tipo: `Animal`.

## Las etiquetas de campo

Después del tipo de un campo, puedes añadir una **etiqueta** (*tag*) entre acentos
graves. Go la ignora, pero algunos paquetes la leen. El más habitual es
`encoding/json`, que la usa para elegir el nombre del campo en el
[JSON](../debuter/glossaire.md#json) (ver [los formatos](formats.md)).

```go
package main

import (
	"encoding/json"
	"fmt"
)

type Libro struct {
	Titulo  string `json:"titulo"`
	Paginas int    `json:"paginas"`
	Secreto string `json:"-"`
}

func main() {
	l := Libro{Titulo: "Dune", Paginas: 600, Secreto: "oculto"}
	datos, _ := json.Marshal(l)
	fmt.Println(string(datos))
}
```

```sortie
{"titulo":"Dune","paginas":600}
```

`json:"-"` quiere decir: no escribir este campo.

## Errores frecuentes

### Mezclar campos con nombre y sin nombre

```go norun
package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	p := Point{X: 1, 2}
	fmt.Println(p)
}
```

```text
# command-line-arguments
./main.go:10:19: mixture of field:value and value elements in struct literal
```

Un literal usa los nombres para todos los campos o para ninguno. Escribe
`Point{X: 1, Y: 2}`.

### Olvidar campos sin los nombres

```go norun
package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	p := Point{1}
	fmt.Println(p)
}
```

```text
# command-line-arguments
./main.go:10:14: too few values in struct literal of type Point
```

Sin los nombres, hay que dar **todos** los campos. Escribe `Point{X: 1}`: el campo
olvidado vale entonces `0`.

### Equivocarse de mayúscula

```go norun
package main

import "fmt"

type Persona struct {
	nombre string
}

func main() {
	p := Persona{nombre: "Alicia"}
	fmt.Println(p.Nombre)
}
```

```text
# command-line-arguments
./main.go:11:16: p.Nombre undefined (type Persona has no field or method Nombre, but does have field nombre)
```

Go distingue mayúsculas y minúsculas: `Nombre` y `nombre` son dos nombres distintos.
El mensaje incluso te da la grafía correcta: `p.nombre`.

### Modificar una copia en un método

```go norun
func (c Contador) Incrementar() {
	c.Valor++
}
```

```text
0
```

Aquí no hay ningún mensaje de error, pero el contador se queda en `0`: el método
modificó una copia. Escribe `func (c *Contador) Incrementar()`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra `fmt.Printf("%+v\n", Point{X: 3})` si `Point` tiene dos
campos `X` e `Y` de tipo `int`?

> **Respuesta**
>
> `{X:3 Y:0}`. El campo `Y` no se da: toma el valor cero de los enteros.

**Pregunta 2:** ¿cuándo hay que escribir `func (c *Cuenta)` en lugar de
`func (c Cuenta)`?

> **Respuesta**
>
> Cuando el método debe modificar la cuenta original. Con `(c Cuenta)`, el método
> trabaja sobre una copia y los cambios se pierden.

**Pregunta 3:** escribe una estructura `Circulo` con un campo `Radio` y un método
`Diametro()` que devuelva el doble del radio.

> **Respuesta**
>
> ```go
> package main
>
> import "fmt"
>
> type Circulo struct {
> 	Radio float64
> }
>
> func (c Circulo) Diametro() float64 {
> 	return 2 * c.Radio
> }
>
> func main() {
> 	c := Circulo{Radio: 2.5}
> 	fmt.Println(c.Diametro())
> }
> ```
>
> ```sortie
> 5
> ```

## En resumen

- `type Nombre struct { ... }` define una estructura; cada uno de sus campos tiene un
  tipo.
- Un literal `T{Campo: valor}` crea un valor; los campos olvidados valen cero.
- `%+v` muestra una estructura con el nombre de los campos; `==` compara todos los
  campos.
- Un método tiene un receptor; usa `*T` para modificar el valor original.
- Una función `NewT` sirve de constructor; la incrustación sustituye a la herencia.

Siguiente: [las interfaces](interfaces.md).
