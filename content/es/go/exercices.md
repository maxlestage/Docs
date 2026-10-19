# Ejercicios resueltos

Esta página propone 12 ejercicios resueltos, del más fácil al más difícil, para practicar todo el recorrido de Go.

## Cómo usar esta página

Cada ejercicio tiene un nivel:

- ★☆☆: fácil;
- ★★☆: medio;
- ★★★: más difícil.

Lee el enunciado, luego escribe tu solución en un archivo `main.go` y ejecútala con
`go run main.go`. Abre la solución solo después de haberlo intentado.

> **Consejo:** a menudo hay varias soluciones buenas. Si tu programa muestra el
> resultado correcto, ¡lo has conseguido! Acuérdate también de ejecutar `go vet main.go`:
> detecta los pequeños errores.

## Lo básico

### Ejercicio 1: Presentarse

Nivel: ★☆☆

Crea dos variables, `nombre` y `edad`. Luego muestra la frase
`Me llamo Alice y tengo 30 años.`, con tus variables.

> **Pista:** `fmt.Printf` con `%s` para un texto y `%d` para un entero (mira
> [los primeros pasos](premiers-pas.md)). No olvides `\n` al final.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	nombre := "Alice"
> 	edad := 30
> 	fmt.Printf("Me llamo %s y tengo %d años.\n", nombre, edad)
> }
> ```
>
> ```sortie
> Me llamo Alice y tengo 30 años.
> ```

### Ejercicio 2: Par o impar

Nivel: ★☆☆

Para cada número de la lista `7, 10, 0`, muestra si es par o impar:
`7 es impar`, `10 es par`, `0 es par`.

> **Pista:** `n%2 == 0` es verdadero cuando `n` es par (mira
> [los números](nombres.md)).

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for _, n := range []int{7, 10, 0} {
> 		if n%2 == 0 {
> 			fmt.Println(n, "es par")
> 		} else {
> 			fmt.Println(n, "es impar")
> 		}
> 	}
> }
> ```
>
> ```sortie
> 7 es impar
> 10 es par
> 0 es par
> ```

### Ejercicio 3: FizzBuzz

Nivel: ★☆☆

Muestra los números del 1 al 15, pero sustituye los múltiplos de 3 por `Fizz`, los de 5
por `Buzz`, y los que son múltiplos de 3 y de 5 a la vez por `FizzBuzz`. Muéstralo todo
en una línea, separado por espacios.

> **Pista:** prueba primero el caso «múltiplo de 15»; si no, nunca se alcanzará. Un
> `switch` sin valor se lee bien (mira [condiciones y bucles](conditions-boucles.md)).

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
> func main() {
> 	var palabras []string
> 	for i := 1; i <= 15; i++ {
> 		switch {
> 		case i%15 == 0:
> 			palabras = append(palabras, "FizzBuzz")
> 		case i%3 == 0:
> 			palabras = append(palabras, "Fizz")
> 		case i%5 == 0:
> 			palabras = append(palabras, "Buzz")
> 		default:
> 			palabras = append(palabras, strconv.Itoa(i))
> 		}
> 	}
> 	fmt.Println(strings.Join(palabras, " "))
> }
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```

### Ejercicio 4: Área y perímetro

Nivel: ★☆☆

Escribe una función `rectangulo(ancho, alto float64)` que devuelva **dos** valores: el
área y el perímetro. Muéstralos para un rectángulo de 4 por 2,5.

> **Pista:** una función de Go puede devolver varios resultados:
> `func f() (float64, float64)` (mira [las funciones](fonctions.md)).

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func rectangulo(ancho, alto float64) (float64, float64) {
> 	area := ancho * alto
> 	perimetro := 2 * (ancho + alto)
> 	return area, perimetro
> }
>
> func main() {
> 	area, perimetro := rectangulo(4, 2.5)
> 	fmt.Println("Área:", area)
> 	fmt.Println("Perímetro:", perimetro)
> }
> ```
>
> ```sortie
> Área: 10
> Perímetro: 13
> ```

## Texto y colecciones

### Ejercicio 5: Media, mínimo, máximo

Nivel: ★★☆

Con la slice `notas := []int{12, 15, 9, 14}`, muestra la media (con un decimal), la nota
más baja y la más alta.

> **Pista:** convierte a `float64` antes de dividir. El paquete `slices` ofrece
> `slices.Min` y `slices.Max` (mira [las colecciones](collections.md)).

> **Solución**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"slices"
> )
>
> func main() {
> 	notas := []int{12, 15, 9, 14}
> 	total := 0
> 	for _, n := range notas {
> 		total += n
> 	}
> 	media := float64(total) / float64(len(notas))
> 	fmt.Printf("Media: %.1f\n", media)
> 	fmt.Println("Mínimo:", slices.Min(notas))
> 	fmt.Println("Máximo:", slices.Max(notas))
> }
> ```
>
> ```sortie
> Media: 12.5
> Mínimo: 9
> Máximo: 15
> ```

### Ejercicio 6: Contar las palabras

Nivel: ★★☆

Cuenta cuántas veces aparece cada palabra en la frase
`"el gato y el perro y el conejo"`. Muestra las palabras **en orden alfabético**, con
su número.

> **Pista:** `strings.Fields` corta en los espacios; un `map[string]int` cuenta. El
> orden de recorrido de un map cambia de una ejecución a otra: ordena las claves (mira
> [los maps](maps.md)).

> **Solución**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"maps"
> 	"slices"
> 	"strings"
> )
>
> func main() {
> 	frase := "el gato y el perro y el conejo"
> 	cuenta := map[string]int{}
> 	for _, palabra := range strings.Fields(frase) {
> 		cuenta[palabra]++
> 	}
> 	for _, palabra := range slices.Sorted(maps.Keys(cuenta)) {
> 		fmt.Printf("%s: %d\n", palabra, cuenta[palabra])
> 	}
> }
> ```
>
> ```sortie
> conejo: 1
> el: 3
> gato: 1
> perro: 1
> y: 2
> ```

### Ejercicio 7: Invertir un texto

Nivel: ★★☆

Escribe una función `invertir(s string) string` que devuelva el texto al revés. Debe
funcionar con los acentos: `"Noël"` da `"lëoN"`.

> **Pista:** una cadena de Go es una sucesión de **bytes**, y `ë` ocupa dos.
> Convierte a `[]rune` para trabajar carácter por carácter (mira
> [el texto](texte.md)).

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func invertir(s string) string {
> 	r := []rune(s)
> 	for i, j := 0, len(r)-1; i < j; i, j = i+1, j-1 {
> 		r[i], r[j] = r[j], r[i]
> 	}
> 	return string(r)
> }
>
> func main() {
> 	fmt.Println(invertir("Hola"))
> 	fmt.Println(invertir("Noël"))
> }
> ```
>
> ```sortie
> aloH
> lëoN
> ```

### Ejercicio 8: Palíndromo

Nivel: ★★☆

Un palíndromo se lee igual en los dos sentidos, sin tener en cuenta los espacios ni las
mayúsculas. Escribe `esPalindromo(s string) bool` y pruébala con `"Kayak"`,
`"Anita lava la tina"` y `"Hola"`.

> **Pista:** pasa el texto a minúsculas con `strings.ToLower`, quita los espacios con
> `strings.ReplaceAll` y luego compara las runas de dos en dos.

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
> func esPalindromo(s string) bool {
> 	r := []rune(strings.ReplaceAll(strings.ToLower(s), " ", ""))
> 	for i := 0; i < len(r)/2; i++ {
> 		if r[i] != r[len(r)-1-i] {
> 			return false
> 		}
> 	}
> 	return true
> }
>
> func main() {
> 	for _, s := range []string{"Kayak", "Anita lava la tina", "Hola"} {
> 		fmt.Printf("%q: %t\n", s, esPalindromo(s))
> 	}
> }
> ```
>
> ```sortie
> "Kayak": true
> "Anita lava la tina": true
> "Hola": false
> ```

## Tipos y errores

### Ejercicio 9: Formas y una interfaz

Nivel: ★★☆

Crea dos tipos, `Rectangulo` (ancho, alto) y `Circulo` (radio), que tengan cada uno un
método `Area() float64`. Define una interfaz `Forma` y luego muestra el área de cada
forma de una slice `[]Forma`, con dos decimales, y el área total.

> **Pista:** un tipo cumple una interfaz en cuanto tiene sus métodos, sin declarar nada
> (mira [las interfaces](interfaces.md)). `math.Pi` da π.

> **Solución**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"math"
> )
>
> type Forma interface {
> 	Area() float64
> }
>
> type Rectangulo struct {
> 	Ancho, Alto float64
> }
>
> func (r Rectangulo) Area() float64 { return r.Ancho * r.Alto }
>
> type Circulo struct {
> 	Radio float64
> }
>
> func (c Circulo) Area() float64 { return math.Pi * c.Radio * c.Radio }
>
> func main() {
> 	formas := []Forma{Rectangulo{4, 3}, Circulo{1}, Rectangulo{2, 2}}
> 	total := 0.0
> 	for _, f := range formas {
> 		fmt.Printf("%T: %.2f\n", f, f.Area())
> 		total += f.Area()
> 	}
> 	fmt.Printf("Total: %.2f\n", total)
> }
> ```
>
> ```sortie
> main.Rectangulo: 12.00
> main.Circulo: 3.14
> main.Rectangulo: 4.00
> Total: 19.14
> ```

### Ejercicio 10: Una división segura

Nivel: ★★☆

Escribe `dividir(a, b float64) (float64, error)`. Si `b` vale 0, devuelve el error
`ErrDivisionPorCero`, declarado de una vez por todas. En `main`, prueba `10 / 4` y
`1 / 0`, y reconoce este error concreto con `errors.Is`.

> **Pista:** `var ErrDivisionPorCero = errors.New("división por cero")` crea un error
> centinela (mira [los errores](erreurs.md)).

> **Solución**
>
> ```go
> package main
>
> import (
> 	"errors"
> 	"fmt"
> )
>
> var ErrDivisionPorCero = errors.New("división por cero")
>
> func dividir(a, b float64) (float64, error) {
> 	if b == 0 {
> 		return 0, ErrDivisionPorCero
> 	}
> 	return a / b, nil
> }
>
> func main() {
> 	pares := [][2]float64{{10, 4}, {1, 0}}
> 	for _, p := range pares {
> 		resultado, err := dividir(p[0], p[1])
> 		if errors.Is(err, ErrDivisionPorCero) {
> 			fmt.Printf("%g / %g: imposible (%v)\n", p[0], p[1], err)
> 			continue
> 		}
> 		fmt.Printf("%g / %g = %g\n", p[0], p[1], resultado)
> 	}
> }
> ```
>
> ```sortie
> 10 / 4 = 2.5
> 1 / 0: imposible (división por cero)
> ```

## Para ir más lejos

### Ejercicio 11: Un filtro genérico

Nivel: ★★★

Escribe **una sola** función genérica `Filtrar` que conserve los elementos de una slice
para los que una función devuelve `true`. Úsala para conservar los números pares de una
slice de enteros, y luego las palabras de más de 4 letras de una slice de cadenas.

> **Pista:** `func Filtrar[T any](s []T, conservar func(T) bool) []T` (mira
> [los genéricos](generiques.md)).

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func Filtrar[T any](s []T, conservar func(T) bool) []T {
> 	var resultado []T
> 	for _, v := range s {
> 		if conservar(v) {
> 			resultado = append(resultado, v)
> 		}
> 	}
> 	return resultado
> }
>
> func main() {
> 	numeros := []int{1, 2, 3, 4, 5, 6}
> 	pares := Filtrar(numeros, func(n int) bool { return n%2 == 0 })
> 	fmt.Println(pares)
>
> 	palabras := []string{"go", "gopher", "casa", "canal"}
> 	largas := Filtrar(palabras, func(p string) bool { return len(p) > 4 })
> 	fmt.Println(largas)
> }
> ```
>
> ```sortie
> [2 4 6]
> [gopher canal]
> ```

### Ejercicio 12: Una suma en paralelo

Nivel: ★★★

Divide los números del 1 al 100 en 4 trozos de 25. Lanza una goroutine por trozo: cada
una calcula la suma de su trozo y la envía por un canal. `main` recibe las 4 sumas
parciales y muestra el total.

> **Pista:** un canal `make(chan int)`; cada goroutine hace `ch <- suma`, y `main` hace
> `<-ch` cuatro veces (mira [los canales](canaux.md)). El orden de llegada cambia de una
> ejecución a otra, pero el total no.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func suma(numeros []int, ch chan<- int) {
> 	total := 0
> 	for _, n := range numeros {
> 		total += n
> 	}
> 	ch <- total
> }
>
> func main() {
> 	var numeros []int
> 	for i := 1; i <= 100; i++ {
> 		numeros = append(numeros, i)
> 	}
>
> 	ch := make(chan int)
> 	for inicio := 0; inicio < len(numeros); inicio += 25 {
> 		go suma(numeros[inicio:inicio+25], ch)
> 	}
>
> 	total := 0
> 	for range 4 {
> 		total += <-ch
> 	}
> 	fmt.Println("Total:", total)
> }
> ```
>
> ```sortie
> Total: 5050
> ```
>
> `chan<- int` indica que la función solo puede **enviar** por el canal. El total no
> depende del orden de llegada de las sumas parciales: el resultado siempre es 5050.

## En resumen

- Empieza por los ejercicios ★☆☆ y luego sube de dificultad.
- Inténtalo siempre solo antes de abrir la solución.
- Los ejercicios 1 a 4 repasan lo básico: variables, condiciones, bucles, funciones.
- Los ejercicios 5 a 8 trabajan el texto, las slices y los maps.
- Los ejercicios 9 y 10 usan las interfaces y los errores; el 11 y el 12, los
  genéricos y las goroutines.

Siguiente: [ejercicios resueltos: segunda serie](exercices-2.md)
