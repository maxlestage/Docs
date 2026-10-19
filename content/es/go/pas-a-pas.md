# Seguir un programa paso a paso en Go

Esta página te enseña a ejecutar un programa Go en tu cabeza, línea por línea, con una tabla que sigue el valor de cada variable.

> **Objetivos:**
>
> - rellenar una tabla de seguimiento (también llamada «tabla de traza») para un
>   programa pequeño;
> - prever lo que un programa va a mostrar **antes** de ejecutarlo;
> - seguir una condición, un bucle y una llamada a función sin perderte;
> - hacer que el programa muestre sus propios pasos con `fmt.Printf`.

## Jugar a ser el ordenador

Un ordenador no adivina nada. Lee una línea, la ejecuta y pasa a la siguiente.
Siempre en el mismo orden. Tú puedes hacer lo mismo, con un lápiz.

Piensa en una **hoja de puntuaciones** durante un juego de mesa. Cada vez que cambia
una puntuación, tachas la antigua y escribes la nueva. Una
[variable](../debuter/glossaire.md#variable) es una línea de esa hoja: una etiqueta
(el nombre) y un número (el valor), que puede cambiar.

La **tabla de seguimiento** es esa hoja, puesta en columnas:

- una columna por variable;
- una fila por paso (una línea de código ejecutada);
- en cada casilla, el valor **después** del paso.

```text
 código                  hoja
 ─────────────           ┌───────┬─────┬─────┐
 a := 5        ───────►  │ Paso  │  a  │  b  │
 b := 3        ───────►  ├───────┼─────┼─────┤
 a = a + b     ───────►  │   1   │  5  │  —  │
                         │   2   │  5  │  3  │
                         │   3   │  8  │  3  │
                         └───────┴─────┴─────┘
```

El guion `—` quiere decir «la variable todavía no existe».

El método, siempre el mismo:

1. Lee el programa entero una vez.
2. **Adivina** lo que va a mostrar. Escríbelo.
3. Rellena la tabla, una fila cada vez. No te saltes nada.
4. Ejecuta el programa con `go run main.go` y compáralo con lo que adivinaste.

> **Recuerda:** una línea de código = una fila de la tabla. Se lee el valor
> **actual** en la tabla, nunca el que «crees» que tiene.

## Programa 1: variables que cambian

```go
package main

import "fmt"

func main() {
	a := 5
	b := 3
	a = a + b
	b = a * 2
	a = a - 1
	fmt.Println(a, b)
}
```

Antes de ejecutarlo, adivina lo que se muestra. Escribe tu respuesta.

Este es el seguimiento, línea por línea:

| Paso | Línea | Cálculo | a | b |
|---|---|---|---|---|
| 1 | `a := 5` | — | 5 | — |
| 2 | `b := 3` | — | 5 | 3 |
| 3 | `a = a + b` | 5 + 3 | **8** | 3 |
| 4 | `b = a * 2` | 8 × 2 | 8 | **16** |
| 5 | `a = a - 1` | 8 − 1 | **7** | 16 |
| 6 | `fmt.Println(a, b)` | muestra | 7 | 16 |

```sortie
7 16
```

El punto importante es el paso 4. `b = a * 2` usa el valor de `a` **en ese momento**:
8, no 5. La tabla te lo muestra: basta con leer la casilla justo encima.

Lee `a = a + b` de **derecha a izquierda**:

```text
   a  =  a + b
   ▲     └─┬─┘
   │       └── 1. se calcula con los valores actuales: 5 + 3 = 8
   └────────── 2. se guarda el resultado en a (el antiguo 5 se borra)
```

Prueba: cambia `b := 3` por `b := 10`. Rehaz la tabla antes de ejecutarlo.
El programa muestra `14 30`.

> **Recuerda:** el signo `=` no quiere decir «igual» como en matemáticas. Quiere decir
> «calcula a la derecha y luego guarda a la izquierda».

## Programa 2: una condición

Una [condición](../debuter/glossaire.md#condition) es un cambio de vía: el programa
toma un solo camino. En la tabla, anotas la respuesta a la pregunta (verdadero o
falso) y luego saltas directamente al bloque correcto.

```go
package main

import "fmt"

func main() {
	temperatura := 12
	consejo := "Quédate en casa calentito"
	if temperatura > 20 {
		consejo = "Sal en camiseta"
	} else if temperatura > 10 {
		consejo = "Llévate un jersey"
	}
	fmt.Println(temperatura, "grados:", consejo)
}
```

Antes de ejecutarlo, adivina lo que se muestra.

| Paso | Línea | Pregunta | Respuesta | temperatura | consejo |
|---|---|---|---|---|---|
| 1 | `temperatura := 12` | — | — | 12 | — |
| 2 | `consejo := ...` | — | — | 12 | Quédate en casa calentito |
| 3 | `if temperatura > 20` | ¿12 > 20? | falso | 12 | Quédate en casa calentito |
| 4 | `else if temperatura > 10` | ¿12 > 10? | **verdadero** | 12 | Quédate en casa calentito |
| 5 | `consejo = "Llévate un jersey"` | — | — | 12 | **Llévate un jersey** |
| 6 | `fmt.Println(...)` | — | — | 12 | Llévate un jersey |

```sortie
12 grados: Llévate un jersey
```

En el paso 3, la respuesta es «falso»: el primer bloque se **salta**. La línea
`consejo = "Sal en camiseta"` nunca aparece en la tabla. Es normal: una línea saltada
no tiene fila en la tabla.

```text
            ¿temperatura > 20?
              /            \
        verdadero          falso
            │                │
      «camiseta»     ¿temperatura > 10?
                        /          \
                  verdadero        falso
                      │              │
                  «jersey»    (nada: se queda
                               «Quédate en casa calentito»)
```

Prueba: pon `temperatura := 20`. El programa muestra `20 grados: Llévate un jersey`,
porque 20 no es **mayor** que 20. Con `temperatura := 5`, muestra
`5 grados: Quédate en casa calentito`: no se toma ningún bloque.

> **Recuerda:** para una condición, anota la pregunta **y** su respuesta en la tabla.
> Después, sáltate los bloques que no se toman.

## Programa 3: un bucle que acumula

Un [bucle](../debuter/glossaire.md#boucle) repite líneas. Piensa en una hucha: en cada
vuelta, metes una moneda y el total crece. La variable `total` es la hucha.

```go
package main

import "fmt"

func main() {
	total := 0
	for i := 1; i <= 5; i++ {
		total = total + i
	}
	fmt.Println("Total:", total)
}
```

Antes de ejecutarlo, adivina lo que se muestra.

En la tabla, una fila por **vuelta**. También anotamos la pregunta `¿i <= 5?`, que se
hace **antes** de cada vuelta.

| Vuelta | i | ¿i <= 5? | Cálculo | total |
|---|---|---|---|---|
| inicio | — | — | — | 0 |
| 1 | 1 | verdadero | 0 + 1 | 1 |
| 2 | 2 | verdadero | 1 + 2 | 3 |
| 3 | 3 | verdadero | 3 + 3 | 6 |
| 4 | 4 | verdadero | 6 + 4 | 10 |
| 5 | 5 | verdadero | 10 + 5 | 15 |
| fin | 6 | **falso** | se sale | 15 |

```sortie
Total: 15
```

Mira la última fila: `i` vale 6, la pregunta responde «falso» y el bucle se detiene
**sin** hacer ningún cálculo. Muchos principiantes olvidan esta fila. Es la que
explica por qué el bucle se detiene.

```text
        ┌─────────────────────────────────┐
        ▼                                 │
  ¿i <= 5? ── verdadero ──► total = total + i ──► i++
        │
      falso
        ▼
  fmt.Println(...)
```

Prueba: cambia `i <= 5` por `i < 5`. La vuelta 5 desaparece. El programa muestra
`Total: 10`.

> **Recuerda:** para un bucle, una fila por vuelta, más una última fila en la que la
> pregunta se vuelve falsa. Esa fila es la que lo detiene todo.

## Programa 4: una función llamada dos veces

Una [función](../debuter/glossaire.md#fonction) es como una cafetera. Le metes algo
(el [parámetro](../debuter/glossaire.md#parametre)), trabaja y te devuelve un
resultado (`return`). Mientras trabaja, tú esperas.

```go
package main

import "fmt"

func cuadrado(n int) int {
	fmt.Println("  cuadrado recibe", n)
	resultado := n * n
	return resultado
}

func main() {
	fmt.Println("inicio")
	x := cuadrado(3)
	y := cuadrado(x - 5)
	fmt.Println("x =", x, "y =", y)
}
```

Antes de ejecutarlo, adivina lo que se muestra. Atención: hay cuatro líneas.

Para una función, añade una columna «¿Dónde?». Indica en qué función estás. Las
variables de `cuadrado` (`n`, `resultado`) solo existen durante la llamada.

| Paso | ¿Dónde? | Línea | n | resultado | x | y | Muestra |
|---|---|---|---|---|---|---|---|
| 1 | main | `fmt.Println("inicio")` | — | — | — | — | inicio |
| 2 | main | `x := cuadrado(3)`: se llama | — | — | — | — | |
| 3 | cuadrado | se recibe `n` | 3 | — | — | — | |
| 4 | cuadrado | `fmt.Println(...)` | 3 | — | — | — | cuadrado recibe 3 |
| 5 | cuadrado | `resultado := n * n` | 3 | 9 | — | — | |
| 6 | cuadrado | `return resultado` | 3 | 9 | — | — | |
| 7 | main | se guarda 9 en `x` | — | — | **9** | — | |
| 8 | main | `y := cuadrado(x - 5)`: 9 − 5 = 4 | — | — | 9 | — | |
| 9 | cuadrado | se recibe `n` | 4 | — | 9 | — | |
| 10 | cuadrado | `fmt.Println(...)` | 4 | — | 9 | — | cuadrado recibe 4 |
| 11 | cuadrado | `resultado := n * n` | 4 | 16 | 9 | — | |
| 12 | cuadrado | `return resultado` | 4 | 16 | 9 | — | |
| 13 | main | se guarda 16 en `y` | — | — | 9 | **16** | |
| 14 | main | `fmt.Println(...)` | — | — | 9 | 16 | x = 9 y = 16 |

```sortie
inicio
  cuadrado recibe 3
  cuadrado recibe 4
x = 9 y = 16
```

En los pasos 7 y 13, `n` y `resultado` vuelven a ser `—`: la llamada ha terminado y
sus variables desaparecen. En la segunda llamada, `cuadrado` vuelve a empezar desde
cero, con un `n` nuevo.

Este es el mismo camino, dibujado. Las flechas hacia la derecha son las llamadas, las
de la izquierda los valores devueltos:

```text
 main                            cuadrado
 ────                            ────────
 Println("inicio")
 x := cuadrado(3)   ── n = 3 ──►  Println(...)
                                  resultado = 9
                    ◄── 9 ─────   return
 y := cuadrado(4)   ── n = 4 ──►  Println(...)
                                  resultado = 16
                    ◄── 16 ────   return
 Println(x, y)
```

Prueba: cambia `cuadrado(x - 5)` por `cuadrado(x)`. La segunda llamada recibe 9. El
programa muestra `cuadrado recibe 9` y luego `x = 9 y = 81`.

> **Recuerda:** una llamada a función es un viaje de ida y vuelta. El programa entra
> en la función, la ejecuta entera y luego vuelve **exactamente** a la línea de la que
> salió, con el valor devuelto.

## Programa 5: un slice y una condición dentro del bucle

Último programa: lo combinamos todo. Un [slice](../debuter/glossaire.md#slice) de
notas, un bucle `range` que las lee una a una y un `if` dentro. Imagina a un profesor
que hojea exámenes: mira cada nota y solo cuenta las que valen al menos 10.

```go
package main

import "fmt"

func main() {
	notas := []int{12, 7, 15, 9, 18}
	aprobados := 0
	suma := 0
	for _, nota := range notas {
		if nota >= 10 {
			aprobados++
			suma += nota
		}
	}
	fmt.Println("Notas >= 10:", aprobados)
	fmt.Println("Su suma:", suma)
}
```

Antes de ejecutarlo, adivina las dos líneas que se muestran.

| Vuelta | nota | ¿nota >= 10? | aprobados | suma |
|---|---|---|---|---|
| inicio | — | — | 0 | 0 |
| 1 | 12 | verdadero | 1 | 12 |
| 2 | 7 | falso | 1 | 12 |
| 3 | 15 | verdadero | 2 | 27 |
| 4 | 9 | falso | 2 | 27 |
| 5 | 18 | verdadero | 3 | 45 |
| fin | — | no quedan notas | 3 | 45 |

```sortie
Notas >= 10: 3
Su suma: 45
```

En las vueltas 2 y 4, la respuesta es «falso»: `aprobados` y `suma` **copian** la fila
de encima. No cambia nada. Ahí es donde uno suele equivocarse: suma 7 «por costumbre».

El bucle `range` avanza solo por el slice, como un dedo que sigue las casillas:

```text
 notas:     [ 12 |  7 | 15 |  9 | 18 ]
 vuelta 1:     ▲
 vuelta 2:          ▲
 vuelta 3:               ▲
 vuelta 4:                    ▲
 vuelta 5:                         ▲
```

Prueba: cambia `nota >= 10` por `nota > 12`. Solo pasan 15 y 18. El programa muestra
`Notas >= 10: 2` y `Su suma: 33` (el texto mostrado, en cambio, no cambia: acuérdate
de corregirlo también).

> **Recuerda:** cuando la condición es falsa, copia sin más la fila de encima. Una
> casilla que no cambia es una información, no un olvido.

## Hacer que el programa muestre sus pasos

La tabla también puedes hacer que la escriba **el programa**. Añade un `fmt.Printf`
dentro del bucle, con una etiqueta delante de cada valor:

```go
package main

import "fmt"

func main() {
	notas := []int{12, 7, 15, 9, 18}
	aprobados := 0
	suma := 0
	for i, nota := range notas {
		if nota >= 10 {
			aprobados++
			suma += nota
		}
		fmt.Printf("vuelta %d: nota=%d aprobados=%d suma=%d\n", i+1, nota, aprobados, suma)
	}
	fmt.Println("Su suma:", suma)
}
```

```sortie
vuelta 1: nota=12 aprobados=1 suma=12
vuelta 2: nota=7 aprobados=1 suma=12
vuelta 3: nota=15 aprobados=2 suma=27
vuelta 4: nota=9 aprobados=2 suma=27
vuelta 5: nota=18 aprobados=3 suma=45
Su suma: 45
```

¡Es exactamente la tabla de la sección anterior! Tres consejos:

- **Pon etiquetas** (`nota=`): unos números sueltos (`12 1 12`) son ilegibles.
- **Coloca el `Printf` al final de la vuelta**, para ver los valores después de los
  cálculos.
- `range` cuenta desde 0: mostramos `i+1` para decir «vuelta 1».

Prueba: mueve el `fmt.Printf` **antes** del `if`. Cada línea muestra entonces los
valores de la vuelta anterior: `vuelta 1: nota=12 aprobados=0 suma=0`.

> **Recuerda:** un `fmt.Printf` con etiquetas convierte el programa en una tabla de
> seguimiento automática. Quítalo cuando hayas encontrado el error.

## El depurador en dos frases

Un depurador, como Delve para Go, pone el programa en pausa en la línea que elijas y
te muestra todas las variables, y luego avanza una línea con cada orden. Es la tabla
de seguimiento, rellenada por el ordenador: lo descubrirás en
[el capítulo sobre depuración](debogage.md).

## Errores frecuentes

### Reutilizar un valor antiguo

Quieres intercambiar `a` y `b`. Piensas que `b` recibe el antiguo `a`.

```go
package main

import "fmt"

func main() {
	a := 1
	b := 2
	a = b
	b = a
	fmt.Println(a, b)
}
```

```sortie
2 2
```

La tabla muestra el error: después de `a = b`, `a` vale 2. El 1 se ha **borrado**.
Luego, `b = a` copia ese 2.

| Paso | Línea | a | b |
|---|---|---|---|
| 1 | `a := 1` | 1 | — |
| 2 | `b := 2` | 1 | 2 |
| 3 | `a = b` | **2** | 2 |
| 4 | `b = a` | 2 | **2** |

La corrección: en Go, el intercambio se hace en una sola línea, `a, b = b, a`. El
programa muestra entonces `2 1`.

### Equivocarse en una vuelta

Quieres sumar 1, 2, 3, 4 y 5. Escribes `<` en lugar de `<=`.

```go
package main

import "fmt"

func main() {
	total := 0
	for i := 1; i < 5; i++ {
		total += i
	}
	fmt.Println("Total:", total)
}
```

```sortie
Total: 10
```

En la tabla, la fila en la que `i` vale 5 responde «falso» a `¿5 < 5?`: el 5 nunca se
suma. La corrección: `i <= 5`. Comprueba siempre la **primera** y la **última**
vuelta.

### Creer que una variable sigue existiendo después del bucle

`i` se crea en la línea del `for`. Desaparece cuando termina el bucle.

```go norun
package main

import "fmt"

func main() {
	total := 0
	for i := 1; i <= 3; i++ {
		total += i
	}
	fmt.Println("Último i:", i)
	fmt.Println("Total:", total)
}
```

```text
# command-line-arguments
./main.go:10:28: undefined: i
```

En la tabla, pon un `—` en la columna `i` en cuanto se sale del bucle. La corrección:
borra la línea, o crea `i` **antes** del bucle si lo necesitas después.

### Poner la hucha a cero en cada vuelta

Quieres mostrar los totales 1, 3, 6. Pero `total := 0` está **dentro** del bucle.

```go
package main

import "fmt"

func main() {
	for i := 1; i <= 3; i++ {
		total := 0
		total += i
		fmt.Println(total)
	}
}
```

```sortie
1
2
3
```

En cada vuelta, la línea `total := 0` se ejecuta de nuevo: la tabla vuelve a poner
`total` a 0 en cada vuelta. La corrección: coloca `total := 0` **antes** del `for`.

## Ponte a prueba

**1.** ¿Cuánto vale `x` después de estas cuatro líneas: `x := 4`, `y := x + 1`,
`x = y * 2`, `y = 0`?

> **Respuesta**
>
> `x` vale 10. Paso a paso: `x` = 4, luego `y` = 5, luego `x` = 5 × 2 = 10. Poner `y`
> a 0 después no cambia `x`: el valor 10 ya se ha guardado.

**2.** ¿Cuántas veces se muestra `Hola` con `for i := 0; i < 3; i++`?
¿Cuál es el valor de `i` cuando el bucle se detiene?

> **Respuesta**
>
> Tres veces: para `i` = 0, 1 y 2. El bucle se detiene cuando `i` vale 3, porque
> `3 < 3` es falso.

**3.** ¿Qué muestra este programa?

```go
package main

import "fmt"

func sumar(a int, b int) int {
	return a + b
}

func main() {
	r := sumar(2, 3)
	r = sumar(r, r)
	fmt.Println(r)
}
```

> **Respuesta**
>
> Muestra 10. La primera llamada devuelve 2 + 3 = 5, que se guarda en `r`. La segunda
> llamada recibe `a` = 5 y `b` = 5, y devuelve 10, que sustituye al antiguo 5.

## En resumen

- Una tabla de seguimiento tiene una columna por variable y una fila por paso.
- `=` calcula a la derecha con los valores **actuales** y luego guarda a la izquierda.
- Para una condición, anota la pregunta y su respuesta, y luego sáltate los bloques
  que no se toman.
- Para un bucle, una fila por vuelta, más la fila en la que la pregunta se vuelve
  falsa.
- Una llamada a función es un viaje de ida y vuelta: sus variables nacen y
  desaparecen con ella.
- `fmt.Printf` con etiquetas hace que el programa escriba la tabla.

Siguiente: [las trampas del principiante en Go](pieges.md)
