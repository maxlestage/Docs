# Ejercicios para empezar bien

Esta página propone 15 pequeños ejercicios muy guiados, todos fáciles, para practicar las bases de Go: variables, cálculos, condiciones, bucles, funciones, textos, slices y maps.

## Cómo hacer estos ejercicios

Hazlos en orden. Van del más sencillo al menos sencillo. Para cada uno:

1. **Lee** el enunciado y la salida esperada.
2. **Pruébalo en papel**: escribe los pasos en español, como una receta.
3. **Escribe** el código en un archivo `main.go`.
4. **Ejecútalo** con `go run main.go`.
5. **Compara** lo que se muestra con la salida esperada, carácter por carácter.
6. ¿Atascado? Lee **la primera pista**. Luego la segunda. La solución viene al final.

```text
 leer → papel → escribir → ejecutar → comparar
                   ↑                     │
                   └───── no es igual ───┘
```

> **Consejo:** todos los programas empiezan con las mismas líneas. Copia esta plantilla
> y rellena el `main`:
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	fmt.Println("¡te toca!")
> }
> ```
>
> ```sortie
> ¡te toca!
> ```

¿Un mensaje de error? Es normal. Lee [las trampas del principiante](pieges.md):
seguramente está ahí.

## Mostrar y calcular

### Ejercicio 1: Hola

Nivel: ★☆☆

Muestra dos líneas: `¡Hola!` y luego `Estoy aprendiendo Go.`

Salida esperada:

```text
¡Hola!
Estoy aprendiendo Go.
```

> **Pista:** un `fmt.Println` por línea.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	fmt.Println("¡Hola!")
> 	fmt.Println("Estoy aprendiendo Go.")
> }
> ```
>
> ```sortie
> ¡Hola!
> Estoy aprendiendo Go.
> ```
>
> Por qué funciona: cada `fmt.Println` muestra su texto y luego pasa a la línea
> siguiente.

### Ejercicio 2: Mi ficha

Nivel: ★☆☆

Crea una variable `nombre` que valga `"Léa"` y una variable `edad` que valga `11`.
Muestra `Léa tiene 11 años.`

Salida esperada:

```text
Léa tiene 11 años.
```

> **Pista:** crea las variables con `:=`.

> **Pista:** `fmt.Println` pone un espacio entre cada trozo, incluso donde no lo
> quieres (¡prueba `"años"` y `"."` como dos trozos!). Prueba mejor `fmt.Printf` con
> `%s` (un texto) y `%d` (un entero), sin olvidar `\n`.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	nombre := "Léa"
> 	edad := 11
> 	fmt.Printf("%s tiene %d años.\n", nombre, edad)
> }
> ```
>
> ```sortie
> Léa tiene 11 años.
> ```
>
> Por qué funciona: `Printf` sustituye `%s` por `nombre` y `%d` por `edad`, en orden.
> `\n` pasa a la línea siguiente.

### Ejercicio 3: El precio total

Nivel: ★☆☆

Un cuaderno cuesta 3 euros. Compras 4. Calcula el total en una variable y muestra
`Total: 12 euros`.

Salida esperada:

```text
Total: 12 euros
```

> **Pista:** `total := precio * cantidad`.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	precio := 3
> 	cantidad := 4
> 	total := precio * cantidad
> 	fmt.Println("Total:", total, "euros")
> }
> ```
>
> ```sortie
> Total: 12 euros
> ```
>
> Por qué funciona: `*` multiplica. `Println` añade un espacio entre los trozos.

### Ejercicio 4: Repartir caramelos

Nivel: ★☆☆

17 caramelos para 5 niños. ¿Cuántos recibe cada niño y cuántos sobran?

Salida esperada:

```text
Cada uno: 3
Sobran: 2
```

> **Pista:** entre dos enteros, `/` da la división entera.

> **Pista:** `%` (módulo) da el resto de la división.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	caramelos := 17
> 	ninos := 5
> 	fmt.Println("Cada uno:", caramelos/ninos)
> 	fmt.Println("Sobran:", caramelos%ninos)
> }
> ```
>
> ```sortie
> Cada uno: 3
> Sobran: 2
> ```
>
> Por qué funciona: 5 × 3 = 15, y 17 − 15 = 2. `/` da 3, `%` da 2.

### Ejercicio 5: La media

Nivel: ★☆☆

Calcula la media de tres notas: 12, 15 y 14. Muéstrala con un decimal.

Salida esperada:

```text
Media: 13.7
```

> **Pista:** 12 + 15 + 14 = 41. Dividido entre 3, da 13,66…: hace falta un
> `float64`, si no Go corta los decimales.

> **Pista:** `float64(suma) / 3`, y luego `fmt.Printf("%.1f\n", …)` para un solo
> decimal (redondeado).

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	suma := 12 + 15 + 14
> 	media := float64(suma) / 3
> 	fmt.Printf("Media: %.1f\n", media)
> }
> ```
>
> ```sortie
> Media: 13.7
> ```
>
> Por qué funciona: la conversión ocurre **antes** de la división, así que el cálculo
> conserva los decimales. `%.1f` redondea 13,666… a 13,7.

## Condiciones y bucles

### Ejercicio 6: Mayor o menor de edad

Nivel: ★☆☆

Con `edad := 15`, muestra `Menor`. Con 18 o más, el programa debe mostrar `Mayor`.

Salida esperada:

```text
Menor
```

> **Pista:** `if edad >= 18 { … } else { … }`.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	edad := 15
> 	if edad >= 18 {
> 		fmt.Println("Mayor")
> 	} else {
> 		fmt.Println("Menor")
> 	}
> }
> ```
>
> ```sortie
> Menor
> ```
>
> Por qué funciona: 15 >= 18 es falso, así que Go ejecuta el bloque `else`. Prueba con
> `edad := 18`.

### Ejercicio 7: Contar hasta 5

Nivel: ★☆☆

Muestra los números del 1 al 5, uno por línea.

Salida esperada:

```text
1
2
3
4
5
```

> **Pista:** `for i := 1; i <= 5; i++ { … }`.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for i := 1; i <= 5; i++ {
> 		fmt.Println(i)
> 	}
> }
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> ```
>
> Por qué funciona: `i` empieza en 1, el bucle gira mientras `i <= 5`, e `i++` suma 1
> en cada vuelta.

### Ejercicio 8: La tabla del 7

Nivel: ★☆☆

Muestra la tabla de multiplicar del 7, del 1 al 5.

Salida esperada:

```text
7 x 1 = 7
7 x 2 = 14
7 x 3 = 21
7 x 4 = 28
7 x 5 = 35
```

> **Pista:** reutiliza el bucle del ejercicio 7.

> **Pista:** dentro del bucle: `fmt.Printf("7 x %d = %d\n", i, 7*i)`.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for i := 1; i <= 5; i++ {
> 		fmt.Printf("7 x %d = %d\n", i, 7*i)
> 	}
> }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```
>
> Por qué funciona: el primer `%d` recibe `i`, el segundo recibe `7*i`.

### Ejercicio 9: Sumar del 1 al 10

Nivel: ★☆☆

Calcula 1 + 2 + 3 + … + 10 con un bucle.

Salida esperada:

```text
Suma: 55
```

> **Pista:** crea `total := 0` **antes** del bucle. Dentro del bucle, suma `i` a
> `total`.

> **Pista:** sigue los valores en papel:
>
> | Vuelta | i | total |
> |---|---|---|
> | 1 | 1 | 1 |
> | 2 | 2 | 3 |
> | 3 | 3 | 6 |
> | … | … | … |
> | 10 | 10 | 55 |

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	total := 0
> 	for i := 1; i <= 10; i++ {
> 		total += i
> 	}
> 	fmt.Println("Suma:", total)
> }
> ```
>
> ```sortie
> Suma: 55
> ```
>
> Por qué funciona: `total += i` quiere decir `total = total + i`. Cuidado con no
> escribir `total := …` dentro del bucle: sería otra variable.

### Ejercicio 10: FizzBuzz hasta 15

Nivel: ★☆☆

Para cada número del 1 al 15: muestra `Fizz` si es divisible entre 3, `Buzz` si es
divisible entre 5, `FizzBuzz` si es divisible entre los dos, y si no, el número.

Salida esperada:

```text
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
```

> **Pista:** «divisible entre 3» se escribe `i%3 == 0`.

> **Pista:** comprueba primero el caso «los dos» (`i%15 == 0`). Si no, 15 mostraría
> solo `Fizz`. Un `switch` sin valor se lee bien aquí.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for i := 1; i <= 15; i++ {
> 		switch {
> 		case i%15 == 0:
> 			fmt.Println("FizzBuzz")
> 		case i%3 == 0:
> 			fmt.Println("Fizz")
> 		case i%5 == 0:
> 			fmt.Println("Buzz")
> 		default:
> 			fmt.Println(i)
> 		}
> 	}
> }
> ```
>
> ```sortie
> 1
> 2
> Fizz
> 4
> Buzz
> Fizz
> 7
> 8
> Fizz
> Buzz
> 11
> Fizz
> 13
> 14
> FizzBuzz
> ```
>
> Por qué funciona: el `switch` toma el **primer** caso verdadero. El caso 15 está
> primero, así que pasa antes que `Fizz` y `Buzz`.

## Funciones y textos

### Ejercicio 11: Una función cuadrado

Nivel: ★☆☆

Escribe una función `cuadrado` que reciba un entero y devuelva su cuadrado. Muestra
`cuadrado(4)` y `cuadrado(9)`.

Salida esperada:

```text
16
81
```

> **Pista:** `func cuadrado(n int) int { … }`. El `int` después del paréntesis es el
> tipo de lo que se devuelve.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func cuadrado(n int) int {
> 	return n * n
> }
>
> func main() {
> 	fmt.Println(cuadrado(4))
> 	fmt.Println(cuadrado(9))
> }
> ```
>
> ```sortie
> 16
> 81
> ```
>
> Por qué funciona: `return` devuelve `n * n` al lugar de la llamada. `cuadrado(4)` se
> sustituye por 16.

### Ejercicio 12: Gritar un nombre

Nivel: ★☆☆

Escribe una función `gritar` que reciba un texto y devuelva ese texto en mayúsculas,
seguido de `!!!`. Pruébala con `"hola chloé"`.

Salida esperada:

```text
HOLA CHLOÉ!!!
```

> **Pista:** importa `"strings"` además de `"fmt"`. `strings.ToUpper` pone en
> mayúsculas.

> **Pista:** une dos textos con `+`.

> **Solución**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"strings"
> )
>
> func gritar(texto string) string {
> 	return strings.ToUpper(texto) + "!!!"
> }
>
> func main() {
> 	fmt.Println(gritar("hola chloé"))
> }
> ```
>
> ```sortie
> HOLA CHLOÉ!!!
> ```
>
> Por qué funciona: `ToUpper` fabrica un texto nuevo en mayúsculas (incluso la `é`), y
> luego `+` le pega `!!!`.

### Ejercicio 13: Contar las letras

Nivel: ★☆☆

Cuenta las letras de la palabra `"niño"` y muestra `niño: 4 letras`.

Salida esperada:

```text
niño: 4 letras
```

> **Pista:** `len("niño")` da 5, no 4: cuenta los bytes, y la `ñ` ocupa dos.

> **Pista:** transforma el texto en runas (una runa = una letra): `[]rune(palabra)`.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	palabra := "niño"
> 	fmt.Printf("%s: %d letras\n", palabra, len([]rune(palabra)))
> }
> ```
>
> ```sortie
> niño: 4 letras
> ```
>
> Por qué funciona: `[]rune(palabra)` corta el texto en 4 letras, y `len` las cuenta.

## Slices y maps

### Ejercicio 14: El mayor

Nivel: ★☆☆

Encuentra el número mayor de la slice `[]int{4, 17, 9, 23, 8}`, con un bucle.

Salida esperada:

```text
El mayor: 23
```

> **Pista:** guarda el «campeón» en una variable. Al principio, es la primera casilla:
> `mayor := numeros[0]`.

> **Pista:** recorre con `range`. Si un número es mayor que el campeón, se convierte en
> el nuevo campeón.
>
> | Número leído | ¿Mayor? | mayor |
> |---|---|---|
> | 4 | no | 4 |
> | 17 | sí | 17 |
> | 9 | no | 17 |
> | 23 | sí | 23 |
> | 8 | no | 23 |

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	numeros := []int{4, 17, 9, 23, 8}
> 	mayor := numeros[0]
> 	for _, n := range numeros {
> 		if n > mayor {
> 			mayor = n
> 		}
> 	}
> 	fmt.Println("El mayor:", mayor)
> }
> ```
>
> ```sortie
> El mayor: 23
> ```
>
> Por qué funciona: en cada vuelta, el campeón se compara con el número leído. Al
> final, solo queda el mayor. (El nombre `mayor` evita ocultar la función `max` de
> Go.)

### Ejercicio 15: Contar las frutas

Nivel: ★☆☆

Aquí tienes una cesta: `[]string{"manzana", "pera", "manzana", "kiwi", "manzana", "kiwi"}`.
Cuenta cada fruta con un map y luego muestra las frutas en orden alfabético.

Salida esperada:

```text
kiwi: 2
manzana: 3
pera: 1
```

> **Pista:** `cuenta := make(map[string]int)`, y luego `cuenta[fruta]++` para cada
> fruta. Una clave que no está vale 0: el primer `++` da 1.

> **Pista:** el orden de un map es aleatorio. Para un orden fijo, recorre
> `slices.Sorted(maps.Keys(cuenta))` (importa `"maps"` y `"slices"`).

> **Solución**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"maps"
> 	"slices"
> )
>
> func main() {
> 	cesta := []string{"manzana", "pera", "manzana", "kiwi", "manzana", "kiwi"}
> 	cuenta := make(map[string]int)
> 	for _, fruta := range cesta {
> 		cuenta[fruta]++
> 	}
> 	for _, fruta := range slices.Sorted(maps.Keys(cuenta)) {
> 		fmt.Printf("%s: %d\n", fruta, cuenta[fruta])
> 	}
> }
> ```
>
> ```sortie
> kiwi: 2
> manzana: 3
> pera: 1
> ```
>
> Por qué funciona: el map guarda un contador por fruta. Ordenar las claves da siempre
> la misma salida. Repasa [los maps](maps.md#ordenar-las-claves-antes-de-mostrar) si
> hace falta.

## ¿Y después?

¡Bravo! Has practicado todas las bases. Para ejercicios un poco más difíciles, mira
[los ejercicios resueltos](exercices.md).

Siguiente: [los tipos](types.md).
