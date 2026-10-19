# Chuleta

Esta página resume de un vistazo la sintaxis de Go vista en todo el recorrido, con tablas compactas y programas cortos para ejecutar.

## Bases

| Elemento | Ejemplo |
| --- | --- |
| Programa | `package main` + `func main() { … }` |
| Importar | `import "fmt"` o `import ( "fmt"; "os" )` |
| Variable (declaración corta) | `edad := 30` |
| Variable (tipo explícito) | `var precio float64 = 9.5` |
| Constante | `const TasaIVA = 0.21` |
| Varias a la vez | `a, b := 1, 2` ; intercambio: `a, b = b, a` |
| Ignorar un valor | `_ = valor` |
| Comentario | `// línea` o `/* bloque */` |
| Conversión | `float64(n)`, `int(x)`, `string(r)` |
| Texto ↔ número | `strconv.Itoa(42)`, `strconv.Atoi("42")` |

| Tipo | Valor cero | Ejemplo |
| --- | --- | --- |
| `int`, `int64` | `0` | `42` |
| `float64` | `0` | `3.14` |
| `string` | `""` | `"Hola"` |
| `bool` | `false` | `true` |
| `rune` (un carácter) | `0` | `'é'` |
| puntero, slice, map, canal, interfaz, función | `nil` | |

```go
package main

import "fmt"

func main() {
	nombre := "Alicia"
	var edad int
	fmt.Println("Hola", nombre, edad)
	fmt.Println(7/2, 7.0/2, 7%2)
	fmt.Println(float64(7) / 2)
}
```

```sortie
Hola Alicia 0
3 3.5 1
3.5
```

Detalles: [primeros pasos](premiers-pas.md), [los números](nombres.md).

## Mostrar con fmt

| Función | Papel |
| --- | --- |
| `fmt.Println(a, b)` | muestra, separa con espacios, salta de línea |
| `fmt.Printf(formato, …)` | muestra según un formato (añade `\n`) |
| `fmt.Sprintf(formato, …)` | devuelve el texto en lugar de mostrarlo |
| `fmt.Fprintf(w, formato, …)` | escribe en un `io.Writer` |
| `fmt.Errorf(formato, …)` | crea un error (`%w` para envolver otro) |

| Verbo | Muestra | Ejemplo → resultado |
| --- | --- | --- |
| `%v` | el valor | `{Alicia 30}` |
| `%+v` | con los nombres de los campos | `{Nombre:Alicia Edad:30}` |
| `%#v` | como código Go | `main.P{Nombre:"Alicia", Edad:30}` |
| `%T` | el tipo | `main.P` |
| `%d` | un entero | `42` |
| `%5d`, `%-5d`, `%05d` | ancho 5, a la izquierda, con ceros | `   42`, `42   `, `00042` |
| `%f`, `%.2f` | un flotante, 2 decimales | `3.141593`, `3.14` |
| `%g` | un flotante sin ceros inútiles | `2.5` |
| `%s`, `%q` | un texto, entre comillas | `Go`, `"Go"` |
| `%-10s` | un texto en 10 caracteres, a la izquierda | `Go        ` |
| `%t` | un booleano | `true` |
| `%c`, `%U` | un carácter, su código Unicode | `é`, `U+00E9` |
| `%x`, `%b` | hexadecimal, binario | `ff`, `101` |
| `%p` | una dirección | `0xc000012345` |
| `%%` | el signo `%` | `%` |

```go
package main

import "fmt"

type P struct {
	Nombre string
	Edad   int
}

func main() {
	p := P{"Alicia", 30}
	fmt.Printf("%v | %+v | %#v | %T\n", p, p, p, p)
	fmt.Printf("[%5d] [%-5d] [%05d] [%.2f] [%q] [%t]\n", 42, 42, 42, 3.14159, "Go", true)
	s := fmt.Sprintf("%-6s|%6s|", "ab", "cd")
	fmt.Println(s)
}
```

```sortie
{Alicia 30} | {Nombre:Alicia Edad:30} | main.P{Nombre:"Alicia", Edad:30} | main.P
[   42] [42   ] [00042] [3.14] ["Go"] [true]
ab    |    cd|
```

## Condiciones y bucles

| Necesidad | Sintaxis |
| --- | --- |
| Condición | `if x > 0 { … } else if … { … } else { … }` |
| Con inicialización | `if err := f(); err != nil { … }` |
| Varios casos | `switch x { case 1, 2: … default: … }` |
| Condiciones en cascada | `switch { case x < 0: … }` |
| Bucle clásico | `for i := 0; i < 5; i++ { … }` |
| «Mientras» | `for x < 10 { … }` |
| Infinito | `for { … }` |
| Recorrer | `for i, v := range slice { … }` |
| De 0 a n-1 | `for i := range 5 { … }` |
| Control | `break`, `continue` |
| Lógica | `&&`, `\|\|`, `!` |

```go
package main

import "fmt"

func main() {
	for i := range 5 {
		if i == 1 {
			continue
		}
		switch {
		case i%2 == 0:
			fmt.Println(i, "par")
		default:
			fmt.Println(i, "impar")
		}
	}
}
```

```sortie
0 par
2 par
3 impar
4 par
```

Detalles: [condiciones y bucles](conditions-boucles.md).

## Funciones

| Forma | Ejemplo |
| --- | --- |
| Simple | `func doble(x int) int { return x * 2 }` |
| Mismo tipo compartido | `func area(an, al float64) float64` |
| Varios resultados | `func div(a, b int) (int, int)` |
| Variádica | `func suma(n ...int) int` ; llamada `suma(t...)` |
| Función anónima | `f := func(x int) int { return x + 1 }` |
| Diferir | `defer archivo.Close()` (al salir de la función) |

```go
package main

import "fmt"

func divmod(a, b int) (int, int) {
	return a / b, a % b
}

func suma(numeros ...int) int {
	total := 0
	for _, n := range numeros {
		total += n
	}
	return total
}

func contador() func() int {
	n := 0
	return func() int {
		n++
		return n
	}
}

func main() {
	c, r := divmod(17, 5)
	fmt.Println(c, r)
	fmt.Println(suma(1, 2, 3), suma([]int{4, 5}...))
	siguiente := contador()
	siguiente()
	fmt.Println(siguiente())
}
```

```sortie
3 2
6 9
2
```

Detalles: [las funciones](fonctions.md).

## Texto

| Necesidad | Ejemplo |
| --- | --- |
| Longitud en bytes / en caracteres | `len(s)`, `utf8.RuneCountInString(s)` |
| Mayúsculas, minúsculas | `strings.ToUpper(s)`, `strings.ToLower(s)` |
| Quitar los espacios alrededor | `strings.TrimSpace(s)` |
| Buscar | `strings.Contains`, `strings.HasPrefix`, `strings.Index` |
| Cortar, unir | `strings.Split(s, ",")`, `strings.Fields(s)`, `strings.Join(t, " ")` |
| Reemplazar | `strings.ReplaceAll(s, "a", "b")` |
| Repetir | `strings.Repeat("-", 10)` |
| Construir de forma eficiente | `var sb strings.Builder` ; `sb.WriteString(…)` |
| Recorrer los caracteres | `for i, r := range s` (r es una `rune`) |
| Texto en bruto en varias líneas | `` `línea 1 \n queda tal cual` `` |

```go
package main

import (
	"fmt"
	"strings"
	"unicode/utf8"
)

func main() {
	s := "  Año en Málaga  "
	t := strings.TrimSpace(s)
	fmt.Println(len(t), utf8.RuneCountInString(t))
	fmt.Println(strings.ToUpper(t), strings.Contains(t, "Málaga"))
	fmt.Printf("%q\n", strings.Fields(t))
	fmt.Println(strings.Join([]string{"a", "b", "c"}, "-"))
}
```

```sortie
15 13
AÑO EN MÁLAGA true
["Año" "en" "Málaga"]
a-b-c
```

Detalles: [el texto](texte.md), [las expresiones regulares](regex.md).

## Slices

| Necesidad | Ejemplo |
| --- | --- |
| Crear | `t := []int{1, 2, 3}`, `make([]int, 0, 10)` |
| Longitud, capacidad | `len(t)`, `cap(t)` |
| Leer, modificar | `t[0]`, `t[len(t)-1] = 9` |
| Añadir | `t = append(t, 4, 5)` |
| Sub-slice | `t[1:3]` (índices 1 y 2), `t[:2]`, `t[2:]` |
| Copiar | `c := slices.Clone(t)` |
| Ordenar | `slices.Sort(t)`, `slices.SortFunc(t, cmp)` |
| Buscar | `slices.Contains(t, 3)`, `slices.Index(t, 3)` |
| Mín, máx | `slices.Min(t)`, `slices.Max(t)` |
| Array (tamaño fijo) | `var a [3]int` |

```go
package main

import (
	"fmt"
	"slices"
)

func main() {
	t := []int{5, 2, 8}
	t = append(t, 1)
	slices.Sort(t)
	fmt.Println(t, len(t), t[1:3])
	fmt.Println(slices.Contains(t, 8), slices.Index(t, 8), slices.Max(t))
}
```

```sortie
[1 2 5 8] 4 [2 5]
true 3 8
```

Detalles: [las colecciones](collections.md).

## Maps

| Necesidad | Ejemplo |
| --- | --- |
| Crear | `m := map[string]int{"a": 1}`, `make(map[string]int)` |
| Leer, escribir | `m["a"]`, `m["b"] = 2` |
| Comprobar una clave | `v, ok := m["c"]` |
| Borrar | `delete(m, "a")` |
| Tamaño | `len(m)` |
| Claves ordenadas | `slices.Sorted(maps.Keys(m))` |
| Atención | el orden de `range` sobre un map es aleatorio; escribir en un map `nil` provoca un pánico |

```go
package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	stock := map[string]int{"manzanas": 3, "peras": 0}
	stock["kiwis"] = 7
	v, ok := stock["plátanos"]
	fmt.Println(v, ok)
	delete(stock, "peras")
	for _, k := range slices.Sorted(maps.Keys(stock)) {
		fmt.Println(k, stock[k])
	}
}
```

```sortie
0 false
kiwis 7
manzanas 3
```

Detalles: [los maps](maps.md).

## Structs y métodos

| Necesidad | Ejemplo |
| --- | --- |
| Definir | `type Punto struct { X, Y int }` |
| Crear | `p := Punto{X: 1, Y: 2}`, `Punto{1, 2}` |
| Campo | `p.X` |
| Método (lectura) | `func (p Punto) Norma() float64` |
| Método (modifica) | `func (p *Punto) Mover(dx int)` |
| Puntero | `pp := &p` ; `*pp` ; `pp.X` funciona directamente |
| Constructor (por costumbre) | `func NewPunto(x, y int) *Punto` |
| Exportado / privado | `Mayúscula` / `minúscula` |
| Integrar un tipo | `type Gato struct { Animal }` |

```go
package main

import "fmt"

type Cuenta struct {
	Titular string
	Saldo   int
}

func (c Cuenta) String() string {
	return fmt.Sprintf("%s: %d €", c.Titular, c.Saldo)
}

func (c *Cuenta) Depositar(importe int) {
	c.Saldo += importe
}

func main() {
	c := Cuenta{Titular: "Alicia"}
	c.Depositar(50)
	c.Depositar(20)
	fmt.Println(c)
}
```

```sortie
Alicia: 70 €
```

Detalles: [los tipos](types.md), [los punteros](pointeurs.md).

## Interfaces

| Necesidad | Ejemplo |
| --- | --- |
| Definir | `type Forma interface { Area() float64 }` |
| Satisfacer | tener los métodos, sin declarar nada |
| Cualquier tipo | `any` (alias de `interface{}`) |
| Aserción de tipo | `c, ok := f.(Circulo)` |
| Switch de tipo | `switch v := x.(type) { case int: … }` |
| Habituales | `fmt.Stringer`, `error`, `io.Reader`, `io.Writer` |

```go
package main

import "fmt"

func describir(x any) string {
	switch v := x.(type) {
	case int:
		return fmt.Sprintf("entero %d", v)
	case string:
		return fmt.Sprintf("texto %q", v)
	default:
		return fmt.Sprintf("otro (%T)", v)
	}
}

func main() {
	for _, x := range []any{42, "Go", 3.5} {
		fmt.Println(describir(x))
	}
}
```

```sortie
entero 42
texto "Go"
otro (float64)
```

Detalles: [las interfaces](interfaces.md).

## Errores

| Necesidad | Ejemplo |
| --- | --- |
| Devolver un error | `return 0, errors.New("mensaje")` |
| Con valores | `fmt.Errorf("nota %d no válida", n)` |
| Envolver | `fmt.Errorf("lectura: %w", err)` |
| Comprobar | `if err != nil { return err }` |
| Error concreto | `errors.Is(err, os.ErrNotExist)` |
| Tipo de error | `var e *MiError; errors.As(err, &e)` |
| Bug grave | `panic("…")` ; `recover()` en un `defer` |

```go
package main

import (
	"errors"
	"fmt"
	"strconv"
)

var ErrNegativo = errors.New("número negativo")

func leer(s string) (int, error) {
	n, err := strconv.Atoi(s)
	if err != nil {
		return 0, fmt.Errorf("leer %q: %w", s, err)
	}
	if n < 0 {
		return 0, ErrNegativo
	}
	return n, nil
}

func main() {
	for _, s := range []string{"12", "x", "-3"} {
		n, err := leer(s)
		switch {
		case errors.Is(err, ErrNegativo):
			fmt.Println(s, "->", "rechazado:", err)
		case err != nil:
			fmt.Println(s, "->", err)
		default:
			fmt.Println(s, "->", n)
		}
	}
}
```

```sortie
12 -> 12
x -> leer "x": strconv.Atoi: parsing "x": invalid syntax
-3 -> rechazado: número negativo
```

Detalles: [los errores](erreurs.md).

## Genéricos

| Necesidad | Ejemplo |
| --- | --- |
| Función genérica | `func Primero[T any](t []T) T` |
| Restricción «comparable» | `func Contiene[T comparable](t []T, v T) bool` |
| Restricción «ordenado» | `func Max[T cmp.Ordered](a, b T) T` |
| Unión de tipos | `type Numero interface { ~int \| ~float64 }` |
| Tipo genérico | `type Pila[T any] struct { elementos []T }` |

```go
package main

import "fmt"

type Numero interface {
	~int | ~float64
}

func Suma[T Numero](valores []T) T {
	var total T
	for _, v := range valores {
		total += v
	}
	return total
}

type Pila[T any] struct {
	elementos []T
}

func (p *Pila[T]) Apilar(v T) { p.elementos = append(p.elementos, v) }

func main() {
	fmt.Println(Suma([]int{1, 2, 3}), Suma([]float64{0.5, 1.25}))
	var p Pila[string]
	p.Apilar("a")
	p.Apilar("b")
	fmt.Println(p.elementos)
}
```

```sortie
6 1.75
[a b]
```

Detalles: [los genéricos](generiques.md).

## Goroutines y canales

| Necesidad | Ejemplo |
| --- | --- |
| Lanzar una goroutine | `go trabajo()` |
| Esperar a un grupo | `var wg sync.WaitGroup` ; `wg.Add(1)` ; `defer wg.Done()` ; `wg.Wait()` |
| Proteger una variable | `var mu sync.Mutex` ; `mu.Lock()` ; `mu.Unlock()` |
| Crear un canal | `ch := make(chan int)`, con búfer: `make(chan int, 10)` |
| Enviar, recibir | `ch <- v`, `v := <-ch`, `v, ok := <-ch` |
| Cerrar, recorrer | `close(ch)` ; `for v := range ch` |
| Esperar varios canales | `select { case v := <-a: … case <-time.After(d): … }` |
| Detectar carreras | `go run -race`, `go test -race` |

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	resultados := make(chan int)
	var wg sync.WaitGroup
	for i := 1; i <= 3; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			resultados <- i * i
		}()
	}
	go func() {
		wg.Wait()
		close(resultados)
	}()

	total := 0
	for r := range resultados {
		total += r
	}
	fmt.Println("Total:", total)
}
```

```sortie
Total: 14
```

Detalles: [la concurrencia](concurrence.md), [los canales](canaux.md).

## Archivos

| Necesidad | Ejemplo |
| --- | --- |
| Leer todo | `datos, err := os.ReadFile("a.txt")` |
| Escribir todo | `os.WriteFile("a.txt", datos, 0o644)` |
| Abrir, crear | `os.Open(nombre)`, `os.Create(nombre)` ; luego `defer f.Close()` |
| Añadir al final | `os.OpenFile(nombre, os.O_APPEND\|os.O_CREATE\|os.O_WRONLY, 0o644)` |
| Leer línea a línea | `sc := bufio.NewScanner(f)` ; `for sc.Scan() { sc.Text() }` |
| ¿Archivo ausente? | `errors.Is(err, os.ErrNotExist)` |
| Rutas | `filepath.Join("carpeta", "a.txt")`, `filepath.Ext(nombre)` |
| Argumentos del programa | `os.Args`, o el paquete `flag` |

```go
package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	err := os.WriteFile("notas.txt", []byte("Alicia 15\nBeto 9\n"), 0o644)
	if err != nil {
		fmt.Println(err)
		return
	}
	f, err := os.Open("notas.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()
	sc := bufio.NewScanner(f)
	for n := 1; sc.Scan(); n++ {
		fmt.Printf("%d: %s\n", n, sc.Text())
	}
	_, err = os.ReadFile("ausente.txt")
	fmt.Println(err)
}
```

```sortie
1: Alicia 15
2: Beto 9
open ausente.txt: no such file or directory
```

Detalles: [los archivos](fichiers.md), [la línea de comandos](ligne-de-commande.md).

## JSON

| Necesidad | Ejemplo |
| --- | --- |
| Struct → JSON | `json.Marshal(v)`, `json.MarshalIndent(v, "", "  ")` |
| JSON → struct | `json.Unmarshal(datos, &v)` |
| Nombre del campo | `` Titulo string `json:"titulo"` `` |
| Omitir si está vacío | `` `json:"nota,omitempty"` `` |
| Ignorar un campo | `` `json:"-"` `` |
| Regla | solo los campos **exportados** (mayúscula) se leen y se escriben |

```go
package main

import (
	"encoding/json"
	"fmt"
)

type Tarea struct {
	Titulo string   `json:"titulo"`
	Hecha  bool     `json:"hecha"`
	Tags   []string `json:"tags,omitempty"`
}

func main() {
	datos, _ := json.Marshal(Tarea{Titulo: "Pan"})
	fmt.Println(string(datos))

	var t Tarea
	err := json.Unmarshal([]byte(`{"titulo":"Go","hecha":true,"tags":["estudio"]}`), &t)
	fmt.Printf("%+v %v\n", t, err)
}
```

```sortie
{"titulo":"Pan","hecha":false}
{Titulo:Go Hecha:true Tags:[estudio]} <nil>
```

Detalles: [los formatos](formats.md).

## Fechas y duraciones

Go describe los formatos de fecha con una **fecha modelo**: el lunes 2 de enero de 2006,
a las 15:04:05. Escribe esa fecha tal como quieres ver la tuya.

| Elemento | Código del modelo | Ejemplo |
| --- | --- | --- |
| Año | `2006` | `2026` |
| Mes (número, nombre) | `01`, `Jan`, `January` | `03`, `Mar` |
| Día | `02` | `14` |
| Hora (24 h), minutos, segundos | `15`, `04`, `05` | `09`, `30`, `00` |
| Día de la semana | `Mon`, `Monday` | `Sat` |
| Formato ISO ya listo | `time.RFC3339` | `2026-03-14T09:30:00Z` |

| Necesidad | Ejemplo |
| --- | --- |
| Ahora | `time.Now()` |
| Fecha concreta | `time.Date(2026, 3, 14, 9, 30, 0, 0, time.UTC)` |
| Formatear, leer | `t.Format("02/01/2006")`, `time.Parse("02/01/2006", s)` |
| Duraciones | `90 * time.Minute`, `t.Add(d)`, `t2.Sub(t1)` |
| Añadir días | `t.AddDate(0, 0, 7)` |
| Esperar | `time.Sleep(500 * time.Millisecond)` |

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	t := time.Date(2026, 3, 14, 9, 30, 0, 0, time.UTC)
	fmt.Println(t.Format("02/01/2006 15:04"))
	fmt.Println(t.Format(time.RFC3339), t.Weekday())
	fin := t.Add(90 * time.Minute)
	fmt.Println(fin.Format("15:04"), fin.Sub(t))
	d, err := time.Parse("02/01/2006", "25/12/2026")
	fmt.Println(d.Sub(t).Hours()/24, err)
}
```

```sortie
14/03/2026 09:30
2026-03-14T09:30:00Z Saturday
11:00 1h30m0s
285.6041666666667 <nil>
```

Detalles: [las fechas](dates.md).

## Tests

| Elemento | Regla |
| --- | --- |
| Archivo | `xxx_test.go`, en la misma carpeta que el código |
| Función | `func TestNombre(t *testing.T)` |
| Señalar un fallo | `t.Errorf("got %v, want %v", got, want)` ; `t.Fatalf` detiene el test |
| Subtests | `t.Run("caso", func(t *testing.T) { … })` |
| Lanzar | `go test ./...`, `go test -v`, `go test -run TestNombre` |
| Cobertura, carreras | `go test -cover`, `go test -race` |
| Medir | `func BenchmarkNombre(b *testing.B)` ; `go test -bench .` |

```go norun
// calculo_test.go
package calculo

import "testing"

func TestDoble(t *testing.T) {
	casos := []struct {
		entrada, esperado int
	}{
		{2, 4},
		{0, 0},
		{-3, -6},
	}
	for _, c := range casos {
		if got := Doble(c.entrada); got != c.esperado {
			t.Errorf("Doble(%d) = %d, esperado %d", c.entrada, got, c.esperado)
		}
	}
}
```

Detalles: [los tests](tests.md).

## Proyecto y herramientas

| Comando | Papel |
| --- | --- |
| `go mod init example.com/app` | crear un módulo (`go.mod`) |
| `go run .` / `go run main.go` | compilar y ejecutar |
| `go build -o app ./cmd/app` | crear un ejecutable |
| `go install ./cmd/app` | instalar en `~/go/bin` |
| `GOOS=windows GOARCH=amd64 go build` | compilar para otro sistema |
| `go get module@v1.2.3` | añadir una dependencia |
| `go mod tidy` | añadir lo que falta, quitar lo que sobra |
| `go fmt ./...` / `gofmt -w .` | dar formato |
| `go vet ./...` | detectar código sospechoso |
| `go doc strings.Split` | leer la documentación |
| `dlv debug` | lanzar el depurador Delve |

| Carpeta | Contenido |
| --- | --- |
| `cmd/<nombre>/` | un programa (`package main`) |
| `internal/` | paquetes privados del módulo |
| `*_test.go` | los tests, junto al código |

Detalles: [depurar](debogage.md), [escribir bien Go](style.md),
[organizar un proyecto](projet.md), [los módulos](modules.md).
