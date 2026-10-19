# Números y cálculos

Go ofrece números enteros y números decimales: este capítulo muestra cómo calcular con ellos, convertirlos, leerlos desde un texto y mostrarlos de forma clara.

> **Objetivos:**
>
> - sabrás elegir entre `int` y `float64`;
> - sabrás calcular con la división entera y el resto `%`;
> - sabrás convertir un número de un tipo a otro, y desde un texto con `strconv`;
> - sabrás usar el paquete `math` y sacar números al azar;
> - sabrás mostrar un número con la cantidad correcta de decimales.

## Los enteros

Un [entero](../debuter/glossaire.md#entero) es un número sin decimales: `3`, `-12`,
`2026`. El tipo más habitual es `int`.

```go
package main

import "fmt"

func main() {
	anio := 2026
	alumnos := 28
	fmt.Println(anio-2000, alumnos*2)
}
```

```sortie
26 56
```

### Los tamaños de enteros

Go tiene varios tipos de enteros. Su nombre dice dos cosas:

- el principio: `int` para un entero **con signo** (puede ser negativo), `uint` para un
  entero **sin signo** (*unsigned*: nunca negativo);
- el número: el tamaño en [bits](../debuter/glossaire.md#bit). Cuantos más bits, más
  grande puede ser el número.

| Tipo | Valor más pequeño | Valor más grande |
|------|-------------------|------------------|
| `int8` | -128 | 127 |
| `uint8` (o `byte`) | 0 | 255 |
| `int16` | -32 768 | 32 767 |
| `int32` (o `rune`) | -2 147 483 648 | 2 147 483 647 |
| `int64` | aprox. -9,2 × 10¹⁸ | aprox. 9,2 × 10¹⁸ |
| `uint64` | 0 | aprox. 1,8 × 10¹⁹ |
| `int` | como `int64` en un ordenador moderno | |

> **Recuerda:** usa `int` por defecto. Los demás tamaños sirven en casos concretos:
> ahorrar memoria, leer un archivo binario…

El guion bajo `_` puede separar las cifras para leerlas mejor:

```go
package main

import "fmt"

func main() {
	poblacion := 8_100_000_000
	fmt.Println(poblacion)
}
```

```sortie
8100000000
```

## Los números decimales

Un número con decimales se llama [flotante](../debuter/glossaire.md#flotante). En Go,
se usa `float64`. Atención: los decimales se separan con un **punto**.

```go
package main

import "fmt"

func main() {
	precio := 19.99
	cantidad := 3.0
	fmt.Println(precio * cantidad)

	a := 0.1
	b := 0.2
	fmt.Println(a + b)
}
```

```sortie
59.97
0.30000000000000004
```

El resultado de `a + b` no es exactamente `0.3`. El ordenador guarda los números
decimales en binario, y algunos números no caen justos, como 1/3 en decimal
(0,3333…). No es un fallo de Go: todos los lenguajes hacen lo mismo.

> **Nota:** `fmt.Println(0.1 + 0.2)` sí muestra `0.3`. Los números escritos
> directamente en el código son **constantes**, que el compilador calcula de forma
> exacta. El problema solo aparece con variables.

> **Consejo:** para el dinero, cuenta en **céntimos** con un `int`: `1999` céntimos en
> lugar de `19.99` euros. Los cálculos siguen siendo exactos.

## Los operadores

| Operador | Función | Ejemplo | Resultado |
|----------|---------|---------|-----------|
| `+` | suma | `7 + 2` | `9` |
| `-` | resta | `7 - 2` | `5` |
| `*` | multiplicación | `7 * 2` | `14` |
| `/` | división | `7 / 2` | `3` |
| `%` | resto de la división | `7 % 2` | `1` |

Las reglas de prioridad son las de las matemáticas: `*` y `/` antes que `+` y `-`. Los
paréntesis van antes que todo.

### La división entera y el resto

Entre dos enteros, `/` da una **división entera**: la parte después del punto decimal
se corta. `%` (se pronuncia «módulo») da el resto.

```go
package main

import "fmt"

func main() {
	fmt.Println(7 / 2)
	fmt.Println(7 % 2)
	fmt.Println(7.0 / 2)

	minutos := 135
	fmt.Println(minutos/60, "h", minutos%60, "min")
}
```

```sortie
3
1
3.5
2 h 15 min
```

Imagina 7 caramelos repartidos entre 2 niños: cada uno recibe 3 (`7 / 2`), y sobra
1 (`7 % 2`).

> **Consejo:** `n % 2 == 0` comprueba si `n` es par. ¡El resto es muy útil!

### Atajos

```go
package main

import "fmt"

func main() {
	puntos := 10
	puntos += 5
	puntos *= 2
	puntos--
	fmt.Println(puntos)
}
```

```sortie
29
```

- `puntos += 5` es un atajo de `puntos = puntos + 5` (igual con `-=`, `*=`, `/=`);
- `puntos++` suma 1, `puntos--` resta 1.

> **Nota:** en Go, `puntos++` es una instrucción sola en su línea. No se puede
> escribir `x := puntos++`.

## El desbordamiento

Un entero tiene un tamaño limitado. Si un cálculo pasa del valor más grande, el número
«da la vuelta», como un cuentakilómetros que vuelve a cero.

```go
package main

import "fmt"

func main() {
	var n int8 = 127
	n++
	fmt.Println(n)

	var octeto uint8 = 255
	octeto++
	fmt.Println(octeto)
}
```

```sortie
-128
0
```

Go no te avisa: el resultado simplemente es incorrecto. Con un `int` (64 bits), el
límite es tan grande que el problema es raro, pero existe.

> **Atención:** si el valor está escrito directamente en el código, el compilador lo
> comprueba: `var n int8 = 200` se rechaza. La vuelta del contador solo ocurre durante
> la ejecución.

## Las conversiones son explícitas

Go nunca mezcla dos tipos de números por su cuenta. Para multiplicar un `int` por un
`float64`, tienes que **convertir** uno de los dos, escribiendo el tipo como una
función: `float64(x)`.

```go
package main

import "fmt"

func main() {
	cantidad := 3
	precio := 2.5
	total := float64(cantidad) * precio
	fmt.Println(total)

	nota := 15.8
	fmt.Println(int(nota))
}
```

```sortie
7.5
15
```

- `float64(cantidad)` convierte `3` en `3.0`.
- `int(nota)` **corta** la parte decimal: `15.8` se convierte en `15`, no en `16`.

### Calcular una media

Es la trampa clásica: la división de dos enteros es entera.

```go
package main

import "fmt"

func main() {
	suma := 47
	total := 4
	fmt.Println(suma / total)
	fmt.Println(float64(suma) / float64(total))
}
```

```sortie
11
11.75
```

## De texto a número: strconv

Un número escrito por el usuario o leído de un archivo llega en forma de **texto**. El
paquete `strconv` (*string conversion*) hace las conversiones.

- `strconv.Atoi`: texto → `int` (*ASCII to integer*);
- `strconv.Itoa`: `int` → texto;
- `strconv.ParseFloat`: texto → `float64`.

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("42")
	fmt.Println(n+1, err)

	texto := strconv.Itoa(2026)
	fmt.Println("Año " + texto)

	f, err := strconv.ParseFloat("3.75", 64)
	fmt.Println(f*2, err)
}
```

```sortie
43 <nil>
Año 2026
7.5 <nil>
```

`Atoi` y `ParseFloat` devuelven **dos** valores: el número y un error. Cuando todo va
bien, el error vale `nil` («nada»). El `64` de `ParseFloat` pide un `float64`.

### Cuando el texto no es un número

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("doce")
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	fmt.Println(n)
}
```

```sortie
Error: strconv.Atoi: parsing "doce": invalid syntax
```

> **Recuerda:** comprueba siempre `err != nil` después de una conversión. Los errores
> se explican en detalle en [la gestión de errores](erreurs.md).

## El paquete math

El paquete `math` contiene las funciones matemáticas habituales.

```go
package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println(math.Sqrt(16))
	fmt.Println(math.Pow(2, 10))
	fmt.Println(math.Abs(-7.5))
	fmt.Println(math.Round(2.5), math.Floor(2.7), math.Ceil(2.1))
	fmt.Println(math.Max(3, 8), math.Min(3, 8))
	fmt.Printf("%.4f\n", math.Pi)
}
```

```sortie
4
1024
7.5
3 2 3
8 3
3.1416
```

| Función | Para qué sirve |
|---------|----------------|
| `math.Sqrt(x)` | raíz cuadrada |
| `math.Pow(x, y)` | x elevado a y |
| `math.Abs(x)` | valor absoluto (sin el signo menos) |
| `math.Round(x)` | redondeo al más cercano |
| `math.Floor(x)`, `math.Ceil(x)` | redondeo hacia abajo, hacia arriba |
| `math.Max(a, b)`, `math.Min(a, b)` | el más grande, el más pequeño |

Estas funciones trabajan con `float64`. Para dos enteros, Go 1.21 añadió las funciones
`max` y `min`, sin import:

```go
package main

import "fmt"

func main() {
	fmt.Println(max(4, 9, 2), min(4, 9, 2))
}
```

```sortie
9 2
```

## Números al azar

El paquete `math/rand` saca números al azar. `rand.Intn(6)` da un entero entre 0 y 5:
se suma 1 para simular un dado.

Un ordenador no sabe realmente sacar números al azar: calcula una serie de números que
*parecen* aleatorios, a partir de un número inicial, la **semilla** (*seed*). Con la
misma semilla, siempre se obtiene la misma serie. Es muy práctico para las pruebas, y
para este sitio: la salida de abajo no cambia nunca.

```go
package main

import (
	"fmt"
	"math/rand"
)

func main() {
	r := rand.New(rand.NewSource(1))
	fmt.Println(r.Intn(6)+1, r.Intn(6)+1, r.Intn(6)+1)
	fmt.Println(r.Intn(6)+1, r.Intn(6)+1, r.Intn(6)+1)
	fmt.Printf("%.3f\n", r.Float64())
}
```

```sortie
6 4 6
6 2 1
0.066
```

- `rand.NewSource(1)` crea una fuente con la semilla `1`;
- `rand.New(...)` crea un generador `r` a partir de esa fuente;
- `r.Intn(n)` da un entero de 0 a n-1, `r.Float64()` un número entre 0 y 1.

> **Consejo:** en un juego de verdad, llama directamente a `rand.Intn(6)` sin crear una
> fuente: Go elige entonces una semilla distinta en cada ejecución.

## Mostrar un número

`fmt.Printf` controla la presentación con sus verbos:

```go
package main

import "fmt"

func main() {
	precio := 3.14159
	fmt.Printf("%.2f\n", precio)
	fmt.Printf("[%8.2f]\n", precio)
	fmt.Printf("[%-6d]\n", 42)
	fmt.Printf("%03d\n", 7)
	fmt.Printf("%e\n", 1234567.0)
	fmt.Printf("%b %x\n", 10, 255)
}
```

```sortie
3.14
[    3.14]
[42    ]
007
1.234567e+06
1010 ff
```

| Formato | Efecto |
|---------|--------|
| `%.2f` | 2 decimales |
| `%8.2f` | 2 decimales, en 8 caracteres (alineado a la derecha) |
| `%-6d` | entero en 6 caracteres, alineado a la izquierda |
| `%03d` | entero en 3 cifras, completado con ceros |
| `%e` | notación científica |
| `%b`, `%x` | binario, hexadecimal |

Para obtener el texto sin mostrarlo, usa `fmt.Sprintf` con los mismos verbos:
`s := fmt.Sprintf("%.2f", precio)`.

## Errores frecuentes

### Mezclar int y float64

```go norun
package main

import "fmt"

func main() {
	cantidad := 3
	precio := 2.5
	fmt.Println(cantidad * precio)
}
```

```text
# command-line-arguments
./main.go:8:14: invalid operation: cantidad * precio (mismatched types int and float64)
```

Los tipos no coinciden (*mismatched types*). Convierte: `float64(cantidad) * precio`.

### Olvidar el segundo valor de strconv.Atoi

```go norun
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n := strconv.Atoi("42")
	fmt.Println(n)
}
```

```text
# command-line-arguments
./main.go:9:7: assignment mismatch: 1 variable but strconv.Atoi returns 2 values
```

`Atoi` devuelve 2 valores: hacen falta 2 variables. Escribe
`n, err := strconv.Atoi("42")` y luego comprueba `err`.

### Dividir por cero

```go norun
package main

import "fmt"

func main() {
	a := 10
	b := 0
	fmt.Println(a / b)
}
```

```text
panic: runtime error: integer divide by zero

goroutine 1 [running]:
main.main()
	/tmp/tmp.nGOUtnybpF/main.go:8 +0x9
exit status 2
```

El programa se detiene en seco: es un **pánico** (*panic*). Comprueba el divisor antes:
`if b != 0 { ... }`.

### Un valor demasiado grande para su tipo

```go norun
package main

import "fmt"

func main() {
	var pequeno int8 = 200
	fmt.Println(pequeno)
}
```

```text
# command-line-arguments
./main.go:6:21: cannot use 200 (untyped int constant) as int8 value in variable declaration (overflows)
```

Un `int8` llega hasta 127: 200 se desborda (*overflows*). Usa un tipo más grande, como
`int`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto valen `17 / 5` y `17 % 5`?

> **Respuesta**
>
> `3` y `2`. 17 = 5 × 3 + 2: el cociente es 3, el resto es 2.

**Pregunta 2:** convierte el texto `"250"` en número y luego muestra su mitad con un
decimal.

> **Respuesta**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"strconv"
> )
>
> func main() {
> 	n, err := strconv.Atoi("250")
> 	if err != nil {
> 		fmt.Println(err)
> 		return
> 	}
> 	fmt.Printf("%.1f\n", float64(n)/2)
> }
> ```
>
> ```sortie
> 125.0
> ```

**Pregunta 3:** ¿por qué `int(9.99)` vale `9`?

> **Respuesta**
>
> La conversión a `int` corta la parte decimal, sin redondear. Para redondear, usa
> `int(math.Round(9.99))`, que da `10`.

## En resumen

- `int` para los enteros, `float64` para los números decimales.
- Entre dos enteros, `/` es una división entera; `%` da el resto.
- Go nunca convierte por su cuenta: escribe `float64(x)` o `int(x)`.
- `strconv.Atoi`, `strconv.Itoa` y `strconv.ParseFloat` pasan del texto a los números,
  y devuelven un error que hay que comprobar.
- `math` ofrece raíces, potencias y redondeos; `math/rand` saca números al azar.
- `%.2f`, `%03d` o `%8.2f` ajustan la presentación.

Siguiente: [condiciones y bucles](conditions-boucles.md).
