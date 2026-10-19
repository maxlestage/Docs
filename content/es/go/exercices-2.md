# Ejercicios resueltos: segunda serie

Esta página propone 12 nuevos ejercicios resueltos en Go, del más fácil al más difícil, con tablas que siguen las variables paso a paso.

## Cómo usar esta página

Estos ejercicios son **nuevos**: no repiten ni
[los ejercicios para empezar bien](exercices-faciles.md) ni
[la primera serie](exercices.md). Cada uno tiene un nivel:

- ★☆☆: fácil (4 ejercicios);
- ★★☆: medio (5 ejercicios);
- ★★★: más difícil (3 ejercicios).

Para cada ejercicio:

1. Lee el enunciado y la salida esperada.
2. Escribe los pasos en español, en papel, como una receta.
3. Programa en `main.go`, luego ejecuta `go run main.go`.
4. ¿Atascado? Lee la pista. La solución va al final.

```text
 enunciado → papel → código → go run → ¿resultado correcto?
                      ▲                       │
                      └───────── no ──────────┘
```

> **Consejo:** cuando un resultado te sorprenda, haz una tabla de seguimiento, como en
> [Seguir un programa paso a paso](pas-a-pas.md). Las soluciones de abajo muestran
> varias.

## Calentamiento

### Ejercicio 1: De segundos a horas

Nivel: ★☆☆

Una película dura 3725 segundos. Muestra su duración en horas, minutos y segundos:
`1 h 2 min 5 s`.

> **Pista:** una hora tiene 3600 segundos, un minuto 60. Entre dos enteros, `/` da la
> división entera y `%` el resto.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	total := 3725
> 	horas := total / 3600
> 	resto := total % 3600
> 	minutos := resto / 60
> 	segundos := resto % 60
> 	fmt.Printf("%d h %d min %d s\n", horas, minutos, segundos)
> }
> ```
>
> ```sortie
> 1 h 2 min 5 s
> ```
>
> Por qué funciona: es como guardar monedas. Primero llenas las «cajas de una hora»
> (1 caja, quedan 125 s), luego las «cajas de un minuto» (2 cajas, quedan 5 s).
>
> | Paso | Cálculo | Resultado |
> |---|---|---|
> | horas | 3725 / 3600 | 1 |
> | resto | 3725 % 3600 | 125 |
> | minutos | 125 / 60 | 2 |
> | segundos | 125 % 60 | 5 |
>
> Prueba: con `total := 59`, el programa muestra `0 h 0 min 59 s`.

### Ejercicio 2: Contar las vocales

Nivel: ★☆☆

Cuenta las vocales (a, e, i, o, u) de la palabra `"programador"`.

Salida esperada:

```text
programador tiene 4 vocales
```

> **Pista:** recorre la palabra con `range`. `strings.ContainsRune("aeiou", letra)`
> responde `true` si la letra es una de las vocales.

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
> func main() {
> 	palabra := "programador"
> 	vocales := 0
> 	for _, letra := range palabra {
> 		if strings.ContainsRune("aeiou", letra) {
> 			vocales++
> 		}
> 	}
> 	fmt.Println(palabra, "tiene", vocales, "vocales")
> }
> ```
>
> ```sortie
> programador tiene 4 vocales
> ```
>
> Por qué funciona: la cadena `"aeiou"` sirve de lista de referencia, como una lista de
> la compra que vas comprobando. Cada letra de la palabra se busca en ella: o, a, a, o
> se encuentran, o sea 4.

### Ejercicio 3: La suma de las cifras

Nivel: ★☆☆

Calcula la suma de las cifras de `4721`, sin convertirlo en texto.

Salida esperada:

```text
La suma de las cifras de 4721 es 14
```

> **Pista:** `n % 10` da la última cifra (`4721 % 10` vale 1). `n / 10` la quita
> (`4721 / 10` vale 472). Repite mientras `n` no sea 0.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	numero := 4721
> 	n := numero
> 	suma := 0
> 	for n > 0 {
> 		suma += n % 10
> 		n = n / 10
> 	}
> 	fmt.Println("La suma de las cifras de", numero, "es", suma)
> }
> ```
>
> ```sortie
> La suma de las cifras de 4721 es 14
> ```
>
> Por qué funciona: en cada vuelta, «separas» la última cifra, como cuando arrancas la
> última hoja de una libreta.
>
> | Vuelta | n al principio | n % 10 | suma | n al final |
> |---|---|---|---|---|
> | 1 | 4721 | 1 | 1 | 472 |
> | 2 | 472 | 2 | 3 | 47 |
> | 3 | 47 | 7 | 10 | 4 |
> | 4 | 4 | 4 | 14 | 0 |
> | fin | 0 | `0 > 0` es falso | 14 | — |
>
> Copiamos `numero` en `n` para poder mostrar el número de partida al final.

### Ejercicio 4: Un triángulo de estrellas

Nivel: ★☆☆

Muestra un triángulo de 4 líneas: una estrella en la primera, dos en la segunda, y
así sucesivamente.

Salida esperada:

```text
*
**
***
****
```

> **Pista:** dos bucles, uno dentro del otro. El bucle de fuera cuenta las líneas
> (`linea` de 1 a 4). El de dentro muestra `linea` estrellas con `fmt.Print("*")`,
> que no pasa a la línea siguiente.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for linea := 1; linea <= 4; linea++ {
> 		for estrella := 1; estrella <= linea; estrella++ {
> 			fmt.Print("*")
> 		}
> 		fmt.Println()
> 	}
> }
> ```
>
> ```sortie
> *
> **
> ***
> ****
> ```
>
> Por qué funciona: el bucle de dentro da una vuelta **completa** por cada vuelta del
> de fuera, como las agujas de un reloj: la grande da la vuelta entera mientras la
> pequeña avanza una marca. `fmt.Println()` sin nada termina la línea.
>
> Prueba: cambia `estrella <= linea` por `estrella <= 5-linea`. El triángulo queda al
> revés: 4 estrellas, luego 3, 2, 1.

> **Recuerda:** `/` y `%` trocean un número; un bucle dentro de un bucle dibuja líneas
> y columnas.

## Un poco más de reflexión

### Ejercicio 5: Los números primos

Nivel: ★★☆

Un número primo solo es divisible por 1 y por sí mismo (7 lo es, 8 no). Escribe
`esPrimo(n int) bool`, luego muestra los números primos de 2 a 30 en una línea.

Salida esperada:

```text
2 3 5 7 11 13 17 19 23 29
```

> **Pista:** intenta dividir `n` entre 2, 3, 4… Si un resto vale 0 (`n%d == 0`), `n`
> no es primo: devuelve `false` enseguida. Basta con probar los `d` tales que
> `d*d <= n`.

> **Solución**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"strconv"
> 	"strings"
> )
>
> func esPrimo(n int) bool {
> 	if n < 2 {
> 		return false
> 	}
> 	for d := 2; d*d <= n; d++ {
> 		if n%d == 0 {
> 			return false
> 		}
> 	}
> 	return true
> }
>
> func main() {
> 	var primos []string
> 	for n := 2; n <= 30; n++ {
> 		if esPrimo(n) {
> 			primos = append(primos, strconv.Itoa(n))
> 		}
> 	}
> 	fmt.Println(strings.Join(primos, " "))
> }
> ```
>
> ```sortie
> 2 3 5 7 11 13 17 19 23 29
> ```
>
> Por qué funciona: la función es un portero a la entrada de una sala. En cuanto
> encuentra un divisor, dice «no» y se para. Para 25:
>
> | d | ¿d*d <= 25? | 25 % d | Decisión |
> |---|---|---|---|
> | 2 | 4: sí | 1 | seguimos |
> | 3 | 9: sí | 1 | seguimos |
> | 4 | 16: sí | 1 | seguimos |
> | 5 | 25: sí | **0** | `return false` |
>
> Si `n` tuviera un divisor mayor que su raíz, también tendría uno más pequeño, ya
> probado: de ahí `d*d <= n`.

### Ejercicio 6: La sucesión de Fibonacci

Nivel: ★★☆

En la sucesión de Fibonacci, cada número es la suma de los dos anteriores. Empieza
por 0 y 1. Muestra sus 10 primeros números.

Salida esperada:

```text
0 1 1 2 3 5 8 13 21 34
```

> **Pista:** guarda dos variables, `a` (el número actual) y `b` (el siguiente). En
> cada vuelta, muestra `a`, luego avanza un paso con `a, b = b, a+b`.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	a, b := 0, 1
> 	for i := 1; i <= 10; i++ {
> 		fmt.Print(a)
> 		if i < 10 {
> 			fmt.Print(" ")
> 		}
> 		a, b = b, a+b
> 	}
> 	fmt.Println()
> }
> ```
>
> ```sortie
> 0 1 1 2 3 5 8 13 21 34
> ```
>
> Por qué funciona: `a` y `b` avanzan juntos, como dos pies que caminan.
> `a, b = b, a+b` calcula **toda** la parte derecha con los valores antiguos, y luego
> guarda los dos resultados.
>
> | Vuelta | muestra a | nuevo a | nuevo b |
> |---|---|---|---|
> | 1 | 0 | 1 | 1 |
> | 2 | 1 | 1 | 2 |
> | 3 | 1 | 2 | 3 |
> | 4 | 2 | 3 | 5 |
> | 5 | 3 | 5 | 8 |
> | … | … | … | … |
> | 10 | 34 | 55 | 89 |
>
> El `if i < 10` evita un espacio de más al final de la línea.

### Ejercicio 7: Las aficiones en común

Nivel: ★★☆

A Lea le gustan `lectura`, `fútbol`, `piano` y `ajedrez`. A Tom le gustan `piano`,
`natación` y `lectura`. Muestra las aficiones que tienen en común, en el orden de la
lista de Lea.

Salida esperada:

```text
En común: [lectura piano]
```

> **Pista:** pon las aficiones de Tom en un `map[string]bool`. Después, recorre las
> de Lea: `gustaTom[aficion]` vale `true` solo si a Tom también le gusta.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	lea := []string{"lectura", "fútbol", "piano", "ajedrez"}
> 	tom := []string{"piano", "natación", "lectura"}
>
> 	gustaTom := make(map[string]bool)
> 	for _, aficion := range tom {
> 		gustaTom[aficion] = true
> 	}
>
> 	var comunes []string
> 	for _, aficion := range lea {
> 		if gustaTom[aficion] {
> 			comunes = append(comunes, aficion)
> 		}
> 	}
> 	fmt.Println("En común:", comunes)
> }
> ```
>
> ```sortie
> En común: [lectura piano]
> ```
>
> Por qué funciona: el map es una lista de invitados en la puerta. Una clave ausente
> devuelve `false` (el valor cero de un `bool`), así que `gustaTom["fútbol"]` es falso
> sin error. Como recorremos el **slice** de Lea, el orden es siempre el mismo.

### Ejercicio 8: Comprimir un texto

Nivel: ★★☆

Escribe `comprimir(s string) string`, que sustituye cada serie de letras iguales por
la letra seguida del número de repeticiones: `"aaabccdddd"` se convierte en
`"a3b1c2d4"`.

> **Pista:** guarda la letra actual y un contador. Cuando la letra cambia, escribe la
> letra y su contador, y vuelve a empezar en 1. No olvides la última serie, después
> del bucle.

> **Solución**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"strconv"
> )
>
> func comprimir(s string) string {
> 	letras := []rune(s)
> 	if len(letras) == 0 {
> 		return ""
> 	}
> 	resultado := ""
> 	actual := letras[0]
> 	cuenta := 1
> 	for _, l := range letras[1:] {
> 		if l == actual {
> 			cuenta++
> 		} else {
> 			resultado += string(actual) + strconv.Itoa(cuenta)
> 			actual = l
> 			cuenta = 1
> 		}
> 	}
> 	resultado += string(actual) + strconv.Itoa(cuenta)
> 	return resultado
> }
>
> func main() {
> 	fmt.Println(comprimir("aaabccdddd"))
> 	fmt.Println(comprimir("éé"))
> }
> ```
>
> ```sortie
> a3b1c2d4
> é2
> ```
>
> Por qué funciona: es como contar coches del mismo color en un atasco. Cuando el
> color cambia, anotas el grupo y vuelves a empezar.
>
> | Letra leída | ¿Igual? | actual | cuenta | resultado |
> |---|---|---|---|---|
> | (inicio) | — | a | 1 | "" |
> | a | sí | a | 2 | "" |
> | a | sí | a | 3 | "" |
> | b | no | b | 1 | "a3" |
> | c | no | c | 1 | "a3b1" |
> | c | sí | c | 2 | "a3b1" |
> | d | no | d | 1 | "a3b1c2" |
> | d, d, d | sí | d | 4 | "a3b1c2" |
> | (después del bucle) | — | d | 4 | "a3b1c2d4" |
>
> Los `[]rune` hacen que funcionen las letras con tilde, como `é`.

### Ejercicio 9: Agrupar por primera letra

Nivel: ★★☆

Agrupa las frutas `banana`, `aguacate`, `bergamota`, `albaricoque`, `cereza` según su
primera letra. Muestra las letras en orden alfabético.

Salida esperada:

```text
a: [aguacate albaricoque]
b: [banana bergamota]
c: [cereza]
```

> **Pista:** un `map[string][]string`: la clave es la letra, el valor un slice de
> frutas. `fruta[:1]` da la primera letra (aquí, sin tilde). Para el orden, ordena
> las claves con `slices.Sorted(maps.Keys(...))`.

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
> 	frutas := []string{"banana", "aguacate", "bergamota", "albaricoque", "cereza"}
> 	porLetra := make(map[string][]string)
> 	for _, fruta := range frutas {
> 		letra := fruta[:1]
> 		porLetra[letra] = append(porLetra[letra], fruta)
> 	}
> 	for _, letra := range slices.Sorted(maps.Keys(porLetra)) {
> 		fmt.Println(letra+":", porLetra[letra])
> 	}
> }
> ```
>
> ```sortie
> a: [aguacate albaricoque]
> b: [banana bergamota]
> c: [cereza]
> ```
>
> Por qué funciona: el map es una cómoda con cajones, un cajón por letra. Para una
> letra nueva, `porLetra[letra]` vale un slice vacío (`nil`), y `append` sobre `nil`
> funciona: el cajón se crea con la primera fruta.
>
> ```text
>  ┌───── a ─────┐ ┌───── b ─────┐ ┌───── c ─────┐
>  │ aguacate    │ │ banana      │ │ cereza      │
>  │ albaricoque │ │ bergamota   │ │             │
>  └─────────────┘ └─────────────┘ └─────────────┘
> ```

> **Recuerda:** un map sirve para **encontrar rápido** (ejercicio 7) y para
> **agrupar** (ejercicio 9); ordena sus claves para que se muestren siempre igual.

## Para los más valientes

### Ejercicio 10: Paréntesis bien cerrados

Nivel: ★★★

Escribe `equilibrado(s string) bool`. Comprueba que cada `(`, `[` o `{` se cierra con
el signo correcto, en el orden correcto. Prueba `"(a[b]{c})"`, `"(]"` y `"(("`.

Salida esperada:

```text
"(a[b]{c})": true
"(]": false
"((": false
```

> **Pista:** usa una **pila**: un slice donde siempre añades y quitas al final. Un
> signo de apertura se apila. Un signo de cierre debe corresponder al último signo
> apilado, que entonces se quita. Al final, la pila debe estar vacía.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func equilibrado(s string) bool {
> 	pares := map[rune]rune{')': '(', ']': '[', '}': '{'}
> 	var pila []rune
> 	for _, c := range s {
> 		switch c {
> 		case '(', '[', '{':
> 			pila = append(pila, c)
> 		case ')', ']', '}':
> 			if len(pila) == 0 || pila[len(pila)-1] != pares[c] {
> 				return false
> 			}
> 			pila = pila[:len(pila)-1]
> 		}
> 	}
> 	return len(pila) == 0
> }
>
> func main() {
> 	for _, s := range []string{"(a[b]{c})", "(]", "(("} {
> 		fmt.Printf("%q: %t\n", s, equilibrado(s))
> 	}
> }
> ```
>
> ```sortie
> "(a[b]{c})": true
> "(]": false
> "((": false
> ```
>
> Por qué funciona: la pila es una pila de platos. Solo puedes coger el de arriba.
> Para `"(a[b]{c})"` (las letras se ignoran):
>
> | Signo leído | Acción | Pila después |
> |---|---|---|
> | `(` | apilar | `(` |
> | `[` | apilar | `( [` |
> | `]` | arriba está `[`: se quita | `(` |
> | `{` | apilar | `( {` |
> | `}` | arriba está `{`: se quita | `(` |
> | `)` | arriba está `(`: se quita | (vacía) |
>
> Pila vacía al final: `true`. Para `"(("`, quedan dos `(`: `false`.

### Ejercicio 11: La búsqueda por mitades

Nivel: ★★★

El slice `[]int{3, 8, 15, 21, 34, 42, 57, 63}` está ordenado. Escribe
`buscar(s []int, objetivo int) (int, int)`, que devuelve la posición de `objetivo`
(o -1) y el número de intentos. En cada intento, mira el medio y descarta la mitad
inútil. Busca 42 y luego 10.

Salida esperada:

```text
42: posición 5 (2 intentos)
10: no está (3 intentos)
```

> **Pista:** guarda dos límites, `bajo` y `alto`. El medio es `(bajo + alto) / 2`. Si
> el valor del medio es demasiado pequeño, `bajo = medio + 1`; demasiado grande,
> `alto = medio - 1`. Te paras cuando `bajo > alto`.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func buscar(s []int, objetivo int) (int, int) {
> 	bajo, alto := 0, len(s)-1
> 	intentos := 0
> 	for bajo <= alto {
> 		intentos++
> 		medio := (bajo + alto) / 2
> 		if s[medio] == objetivo {
> 			return medio, intentos
> 		} else if s[medio] < objetivo {
> 			bajo = medio + 1
> 		} else {
> 			alto = medio - 1
> 		}
> 	}
> 	return -1, intentos
> }
>
> func main() {
> 	numeros := []int{3, 8, 15, 21, 34, 42, 57, 63}
> 	for _, objetivo := range []int{42, 10} {
> 		pos, intentos := buscar(numeros, objetivo)
> 		if pos == -1 {
> 			fmt.Printf("%d: no está (%d intentos)\n", objetivo, intentos)
> 		} else {
> 			fmt.Printf("%d: posición %d (%d intentos)\n", objetivo, pos, intentos)
> 		}
> 	}
> }
> ```
>
> ```sortie
> 42: posición 5 (2 intentos)
> 10: no está (3 intentos)
> ```
>
> Por qué funciona: es el juego de «mayor o menor». Abres un diccionario por la mitad
> y te quedas solo con la mitad donde puede estar la palabra. Para 10:
>
> | Intento | bajo | alto | medio | s[medio] | Decisión |
> |---|---|---|---|---|---|
> | 1 | 0 | 7 | 3 | 21 | demasiado grande: alto = 2 |
> | 2 | 0 | 2 | 1 | 8 | demasiado pequeño: bajo = 2 |
> | 3 | 2 | 2 | 2 | 15 | demasiado grande: alto = 1 |
> | fin | 2 | 1 | — | — | bajo > alto: no está |
>
> ```text
>  [ 3 | 8 | 15 | 21 | 34 | 42 | 57 | 63 ]   intento 1: medio = 21
>  [ 3 | 8 | 15 ]                             intento 2: medio = 8
>          [ 15 ]                             intento 3: medio = 15
> ```
>
> Con 1000 números, hacen falta como mucho 10 intentos, en lugar de 1000.

### Ejercicio 12: Los números romanos

Nivel: ★★★

Escribe `romano(n int) string`, que escribe un número en números romanos. Prueba
1994, 2026 y 49.

Salida esperada:

```text
1994 = MCMXCIV
2026 = MMXXVI
49 = XLIX
```

> **Pista:** prepara dos slices uno al lado del otro, del mayor al menor, **con** los
> casos especiales: `1000 M, 900 CM, 500 D, 400 CD, 100 C, 90 XC, 50 L, 40 XL, 10 X,
> 9 IX, 5 V, 4 IV, 1 I`. Mientras `n` sea lo bastante grande para el valor, escribe su
> símbolo y réstalo.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func romano(n int) string {
> 	valores := []int{1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1}
> 	simbolos := []string{"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"}
> 	resultado := ""
> 	for i, v := range valores {
> 		for n >= v {
> 			resultado += simbolos[i]
> 			n -= v
> 		}
> 	}
> 	return resultado
> }
>
> func main() {
> 	for _, n := range []int{1994, 2026, 49} {
> 		fmt.Println(n, "=", romano(n))
> 	}
> }
> ```
>
> ```sortie
> 1994 = MCMXCIV
> 2026 = MMXXVI
> 49 = XLIX
> ```
>
> Por qué funciona: es como dar el cambio con las monedas más grandes primero. Para
> 1994 (los valores demasiado grandes se saltan):
>
> | Valor | n antes | Símbolo añadido | n después | resultado |
> |---|---|---|---|---|
> | 1000 | 1994 | M | 994 | M |
> | 900 | 994 | CM | 94 | MCM |
> | 90 | 94 | XC | 4 | MCMXC |
> | 4 | 4 | IV | 0 | MCMXCIV |
>
> Poner 900, 400, 90… en la lista evita escribir `DCCCC` en lugar de `CM`.

> **Recuerda:** para un problema más difícil, encuentra la imagen adecuada (una pila
> de platos, un diccionario, una caja registradora) y compruébala con una tabla de
> seguimiento sobre un ejemplo pequeño.

## En resumen

- Los ejercicios 1 a 4 practican `/`, `%`, `range` sobre un texto y los bucles
  anidados.
- Los ejercicios 5 a 9 usan funciones, el intercambio `a, b = b, a+b`, los maps y los
  slices.
- Los ejercicios 10 a 12 piden un método: una pila, la búsqueda por mitades, el cambio
  voraz.
- Una tabla de seguimiento sobre un ejemplo pequeño vale más que diez relecturas del
  código.

Siguiente: [la chuleta](aide-memoire.md)
