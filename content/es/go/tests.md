# Probar tu código

Esta página muestra cómo escribir tests automáticos con el paquete `testing` y lanzarlos con `go test`, desde los tests más simples hasta los tests en tabla.

> **Objetivos:**
>
> - sabrás escribir un archivo `_test.go` y una función `TestX(t *testing.T)`;
> - sabrás señalar un fallo con `t.Errorf` y `t.Fatalf`;
> - sabrás escribir tests en tabla, con subtests `t.Run`;
> - sabrás lanzar los tests con `go test` y leer su resultado;
> - sabrás qué son los benchmarks y los ejemplos testeables.

## ¿Por qué hacer tests?

Un [test](../debuter/glossaire.md#test) es un pequeño programa que comprueba que otro
programa hace bien lo que debe. Lo escribes una vez y luego lo vuelves a lanzar tras
cada cambio, en un segundo.

Es como la lista de comprobación de un piloto antes del despegue: siempre se revisan
los mismos puntos, en el mismo orden, sin fiarse de la memoria.

Sin herramientas, comprobarías a mano, con `fmt.Println`:

```go
package main

import "fmt"

func Suma(numeros []int) int {
	total := 0
	for _, n := range numeros {
		total += n
	}
	return total
}

func main() {
	if Suma([]int{1, 2, 3}) != 6 {
		fmt.Println("FALLO: la suma de 1, 2, 3 debería dar 6")
	} else {
		fmt.Println("OK")
	}
}
```

```sortie
OK
```

Funciona, pero hay que leer la salida, y las comprobaciones se mezclan con el
programa. Go ofrece una herramienta pensada para esto: el paquete `testing` y el
comando `go test`.

## El proyecto que vamos a probar

Los tests viven en un **módulo** (ver [los módulos](modules.md)). Creemos una carpeta
`calculo`:

```sh
mkdir calculo
cd calculo
go mod init calculo
```

Luego el archivo `calculo.go`, con dos funciones:

```go norun
// Package calculo reúne pequeñas funciones de cálculo.
package calculo

import "errors"

// Suma devuelve la suma de los números.
func Suma(numeros []int) int {
	total := 0
	for _, n := range numeros {
		total += n
	}
	return total
}

// Maximo devuelve el número más grande del slice.
func Maximo(numeros []int) (int, error) {
	if len(numeros) == 0 {
		return 0, errors.New("slice vacío")
	}
	mayor := numeros[0]
	for _, n := range numeros[1:] {
		if n > mayor {
			mayor = n
		}
	}
	return mayor, nil
}
```

Este paquete no tiene `main`: no se ejecuta solo. Son los tests los que lo van a
llamar. Por eso los bloques de esta página están marcados `norun`: forman un proyecto
de varios archivos.

## Un primer test

Las reglas de Go son simples:

- los tests están en un archivo cuyo nombre termina en **`_test.go`**, junto al
  código: aquí `calculo_test.go`;
- este archivo está en el **mismo paquete** (`package calculo`);
- cada test es una función cuyo nombre empieza por **`Test`** seguido de una
  mayúscula, con un solo parámetro `t *testing.T`.

```go norun
package calculo

import "testing"

func TestSuma(t *testing.T) {
	resultado := Suma([]int{1, 2, 3})
	if resultado != 6 {
		t.Errorf("Suma([1 2 3]) = %d, se esperaba 6", resultado)
	}
}
```

Si el resultado no es el correcto, `t.Errorf` señala el fallo con un mensaje, como
`fmt.Printf`. Si no, el test no dice nada: pasa.

Lanza los tests desde la carpeta del módulo:

```sh
go test
```

```text
PASS
ok  	calculo	0.003s
```

`PASS`: todos los tests han pasado. El tiempo del final cambia de una vez a otra.

> **Recuerda:** un buen mensaje de fallo dice qué se llamó, qué se obtuvo y qué se
> esperaba. Ganarás tiempo el día que aparezca.

## ¿Errorf o Fatalf?

`t` ofrece dos familias para señalar un problema:

| Método | Efecto |
|--------|--------|
| `t.Errorf(…)` / `t.Error(…)` | anota el fallo y **continúa** el test |
| `t.Fatalf(…)` / `t.Fatal(…)` | anota el fallo y **detiene** este test en el acto |

Usa `t.Fatalf` cuando seguir ya no tiene sentido, por ejemplo si una función devuelve
un error inesperado: no sirve de nada comprobar un valor que no existe.

```go norun
func TestMaximoVacio(t *testing.T) {
	_, err := Maximo([]int{})
	if err == nil {
		t.Error("un slice vacío debería devolver un error")
	}
}
```

Este test comprueba que el **error** se devuelve de verdad: probar los casos de error
es tan importante como probar los casos normales (ver [los errores](erreurs.md)).

## Los tests en tabla

Para probar varias entradas, no se copia el test diez veces. Se escribe una **tabla
de casos**: un slice de structs anónimos, cada uno con un nombre, una entrada y el
resultado esperado. Luego un bucle los comprueba uno a uno. Es la forma más habitual
de escribir tests en Go.

`t.Run(nombre, funcion)` crea un **subtest** para cada caso: si uno falla, Go dice
cuál.

```go norun
func TestMaximo(t *testing.T) {
	casos := []struct {
		nombre   string
		entrada  []int
		esperado int
	}{
		{"positivos", []int{3, 8, 1}, 8},
		{"negativos", []int{-5, -2, -9}, -2},
		{"uno solo", []int{4}, 4},
	}
	for _, c := range casos {
		t.Run(c.nombre, func(t *testing.T) {
			obtenido, err := Maximo(c.entrada)
			if err != nil {
				t.Fatalf("error inesperado: %v", err)
			}
			if obtenido != c.esperado {
				t.Errorf("Maximo(%v) = %d, se esperaba %d", c.entrada, obtenido, c.esperado)
			}
		})
	}
}
```

Añadir un caso es añadir una línea a la tabla.

## Leer el resultado de go test -v

La opción `-v` (*verbose*, «detallado») muestra cada test y cada subtest:

```sh
go test -v
```

```text
=== RUN   TestSuma
--- PASS: TestSuma (0.00s)
=== RUN   TestMaximo
=== RUN   TestMaximo/positivos
=== RUN   TestMaximo/negativos
=== RUN   TestMaximo/uno_solo
--- PASS: TestMaximo (0.00s)
    --- PASS: TestMaximo/positivos (0.00s)
    --- PASS: TestMaximo/negativos (0.00s)
    --- PASS: TestMaximo/uno_solo (0.00s)
=== RUN   TestMaximoVacio
--- PASS: TestMaximoVacio (0.00s)
PASS
ok  	calculo	0.006s
```

Los espacios de los nombres de subtests se convierten en `_`: `uno solo` → `uno_solo`.

### Cuando un test falla

Introduzcamos un bug: en `Maximo`, partimos de `0` en lugar del primer número
(`mayor := 0`). El resultado se vuelve falso con números negativos, y también con un
solo número, porque el bucle se salta el primero:

```text
--- FAIL: TestMaximo (0.00s)
    --- FAIL: TestMaximo/negativos (0.00s)
        calculo_test.go:29: Maximo([-5 -2 -9]) = 0, se esperaba -2
    --- FAIL: TestMaximo/uno_solo (0.00s)
        calculo_test.go:29: Maximo([4]) = 0, se esperaba 4
FAIL
exit status 1
FAIL	calculo	0.003s
```

El informe da todo lo necesario: el subtest (`negativos`), el archivo y la línea
(`calculo_test.go:29`), y luego tu mensaje. El caso `positivos` ha pasado: sin `-v`,
Go solo muestra los fallos.

### Elegir qué tests lanzar

| Comando | Efecto |
|---------|--------|
| `go test` | lanza los tests de la carpeta actual |
| `go test ./...` | lanza los tests de todos los paquetes del módulo |
| `go test -v` | muestra cada test |
| `go test -run Maximo` | lanza solo los tests cuyo nombre contiene `Maximo` |
| `go test -cover` | muestra qué parte del código ejecutan los tests |

```sh
go test -cover
```

```text
PASS
coverage: 100.0% of statements
ok  	calculo	0.003s
```

## Los ejemplos testeables

Una función cuyo nombre empieza por `Example` sirve a la vez de **documentación** y de
test. El comentario `// Output:` da la salida esperada: `go test` ejecuta el ejemplo y
compara. Suelen guardarse en `ejemplo_test.go`.

```go norun
package calculo

import "fmt"

func ExampleSuma() {
	fmt.Println(Suma([]int{10, 20, 12}))
	// Output: 42
}

func ExampleMaximo() {
	mayor, err := Maximo([]int{3, 8, 1})
	fmt.Println(mayor, err)
	// Output: 8 <nil>
}
```

```text
=== RUN   ExampleSuma
--- PASS: ExampleSuma (0.00s)
=== RUN   ExampleMaximo
--- PASS: ExampleMaximo (0.00s)
PASS
ok  	calculo	0.003s
```

Estos ejemplos aparecen en la documentación del paquete, y no pueden volverse falsos
sin que los tests lo señalen. Es además el principio de los bloques `sortie` de este
sitio.

## Los benchmarks

Un **benchmark** mide la velocidad de una función. Su nombre empieza por `Benchmark` y
recibe `b *testing.B`. El bucle `for b.Loop()` repite la llamada tantas veces como
haga falta para obtener una medida fiable.

```go norun
func BenchmarkSuma(b *testing.B) {
	numeros := make([]int, 1000)
	for i := range numeros {
		numeros[i] = i
	}
	for b.Loop() {
		Suma(numeros)
	}
}
```

```sh
go test -bench .
```

```text
goos: linux
goarch: amd64
pkg: calculo
cpu: Intel(R) Xeon(R) Processor @ 2.10GHz
BenchmarkSuma-4   	 1617744	       725.0 ns/op
PASS
ok  	calculo	1.179s
```

La función se llamó 1 617 744 veces, a unos 725 nanosegundos por llamada. Estas cifras
dependen de tu máquina: compáralas solo entre dos versiones de tu código, en el mismo
ordenador.

> **Nota:** `b.Loop()` existe desde Go 1.24. En código más antiguo, verás
> `for i := 0; i < b.N; i++`.

## Errores frecuentes

### Un archivo mal nombrado

```go norun
// archivo calculo_tests.go (con una «s»)
package calculo

import "testing"

func TestSuma(t *testing.T) {
	if Suma([]int{1, 2, 3}) != 6 {
		t.Error("suma incorrecta")
	}
}
```

```text
?   	calculo	[no test files]
```

Go solo busca tests en los archivos que terminan **exactamente** en `_test.go`.
Renombra el archivo a `calculo_test.go`.

### Un nombre de test en minúscula

```go norun
func testSuma(t *testing.T) {
	if Suma([]int{1, 2, 3}) != 6 {
		t.Error("suma incorrecta")
	}
}
```

```text
testing: warning: no tests to run
PASS
ok  	calculo	0.002s
```

Trampa: aparece `PASS`, pero no se ha ejecutado ningún test. El nombre debe empezar por
`Test` con mayúscula: `TestSuma`.

### Olvidar el asterisco de *testing.T

```go norun
func TestSuma(t testing.T) {
	if Suma([]int{1, 2, 3}) != 6 {
		t.Error("suma incorrecta")
	}
}
```

```text
# calculo
…/calculo/calculo_test.go:5:1: wrong signature for TestSuma, must be: func TestSuma(t *testing.T)
FAIL	calculo [setup failed]
```

El mensaje da la forma correcta: el parámetro es un **puntero**, `t *testing.T`.

### Una salida de ejemplo incorrecta

```go norun
func ExampleSuma() {
	fmt.Println(Suma([]int{10, 20, 12}))
	// Output: 43
}
```

```text
--- FAIL: ExampleSuma (0.00s)
got:
42
want:
43
FAIL
exit status 1
FAIL	calculo	0.002s
```

`got` es lo que el ejemplo mostró, `want` lo que anuncia el comentario. Corrige el que
está mal (aquí, el comentario).

## Ponte a prueba

**Pregunta 1:** ¿qué condiciones debe cumplir una función para que `go test` la lance
como un test?

> **Respuesta**
>
> Debe estar en un archivo que termine en `_test.go`, llamarse `TestAlgo` (con una
> mayúscula después de `Test`) y tener un solo parámetro `t *testing.T`.

**Pregunta 2:** ¿qué diferencia hay entre `t.Errorf` y `t.Fatalf`?

> **Respuesta**
>
> `t.Errorf` señala el fallo y deja que el test continúe. `t.Fatalf` señala el fallo y
> detiene este test inmediatamente.

**Pregunta 3:** escribe un test en tabla `TestSuma` con tres casos: un slice vacío
(0), `[5]` (5) y `[1, 2, 3]` (6).

> **Respuesta**
>
> ```go norun
> func TestSuma(t *testing.T) {
> 	casos := []struct {
> 		nombre   string
> 		entrada  []int
> 		esperado int
> 	}{
> 		{"vacío", []int{}, 0},
> 		{"un número", []int{5}, 5},
> 		{"tres números", []int{1, 2, 3}, 6},
> 	}
> 	for _, c := range casos {
> 		t.Run(c.nombre, func(t *testing.T) {
> 			if obtenido := Suma(c.entrada); obtenido != c.esperado {
> 				t.Errorf("Suma(%v) = %d, se esperaba %d", c.entrada, obtenido, c.esperado)
> 			}
> 		})
> 	}
> }
> ```

## En resumen

- Los tests van en archivos `_test.go`, en el mismo paquete que el código.
- Un test se escribe `func TestNombre(t *testing.T)`; `t.Errorf` continúa, `t.Fatalf`
  detiene.
- Los tests en tabla con `t.Run` comprueban muchos casos en pocas líneas.
- `go test` lanza los tests, `-v` los detalla, `-run` los filtra, `-cover` mide la
  cobertura.
- `ExampleX` documenta y prueba a la vez; `BenchmarkX` mide la velocidad.

Siguiente: [depurar un programa](debogage.md). Compara con
[los tests en Ruby](../ruby/tests.md).
