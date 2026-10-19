# Gestionar los errores

Esta página explica cómo Go trata los errores como valores corrientes, con `error`, `if err != nil`, `fmt.Errorf` y `errors.Is`.

> **Objetivos:**
>
> - sabrás leer y comprobar un error con `if err != nil`;
> - sabrás crear un error con `errors.New` y `fmt.Errorf`;
> - sabrás envolver un error con `%w` y encontrarlo de nuevo con `errors.Is` y `errors.As`;
> - sabrás escribir tus propios tipos de error;
> - sabrás cuándo usar `panic` y `recover` (pocas veces).

## Un error es un valor

Muchos lenguajes lanzan una [excepción](../debuter/glossaire.md#excepcion) que
interrumpe el programa (mira [los errores en Ruby](../ruby/erreurs.md)). Go lo hace de
otra forma: una función que puede fallar **devuelve** el [error](../debuter/glossaire.md#error)
como un resultado más, en última posición.

Piensa en un repartidor. No grita en la calle cuando el paquete está dañado: te da el
paquete **y** una nota «paquete dañado». Te toca a ti leer la nota.

El tipo de los errores se llama `error`. Un error ausente vale `nil`.

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("42")
	fmt.Println(n, err)

	n, err = strconv.Atoi("doce")
	fmt.Println(n, err)
}
```

```sortie
42 <nil>
0 strconv.Atoi: parsing "doce": invalid syntax
```

`strconv.Atoi` transforma un texto en entero. Cuando el texto no es un número,
devuelve `0` y un error que explica el problema.

## if err != nil

Después de cada llamada que puede fallar, se comprueba el error enseguida. Es la línea
más famosa de Go:

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	entradas := []string{"12", "abc", "7"}
	for _, s := range entradas {
		n, err := strconv.Atoi(s)
		if err != nil {
			fmt.Println("Ignorado:", err)
			continue
		}
		fmt.Println("El doble es", n*2)
	}
}
```

```sortie
El doble es 24
Ignorado: strconv.Atoi: parsing "abc": invalid syntax
El doble es 14
```

Se trata el caso de error **primero**, y luego el código normal sigue sin sangría.
Así el camino feliz es fácil de seguir.

## Crear un error

### errors.New

Tu propia función puede devolver un error. `errors.New` crea uno con un mensaje fijo.
Por costumbre, el mensaje empieza por minúscula y no termina en punto.

```go
package main

import (
	"errors"
	"fmt"
)

func dividir(a, b float64) (float64, error) {
	if b == 0 {
		return 0, errors.New("división por cero")
	}
	return a / b, nil
}

func main() {
	r, err := dividir(10, 4)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	fmt.Println("Resultado:", r)

	_, err = dividir(1, 0)
	if err != nil {
		fmt.Println("Error:", err)
	}
}
```

```sortie
Resultado: 2.5
Error: división por cero
```

Cuando todo va bien, la función devuelve `nil` como error.

### fmt.Errorf

`fmt.Errorf` fabrica un error como `fmt.Sprintf` fabrica un texto: puedes meter
valores dentro.

```go
package main

import "fmt"

func comprobarEdad(edad int) error {
	if edad < 0 {
		return fmt.Errorf("edad no válida: %d", edad)
	}
	return nil
}

func main() {
	fmt.Println(comprobarEdad(12))
	fmt.Println(comprobarEdad(-3))
}
```

```sortie
<nil>
edad no válida: -3
```

## Envolver un error con %w

Un error suele subir a través de varias funciones. En cada nivel, se añade
**contexto**: lo que se intentaba hacer. Con el verbo `%w` (*wrap*, «envolver»),
`fmt.Errorf` mete el error original dentro de uno nuevo, como un paquete dentro de una
caja más grande.

```go
package main

import (
	"fmt"
	"strconv"
)

func leerPrecio(texto string) (int, error) {
	precio, err := strconv.Atoi(texto)
	if err != nil {
		return 0, fmt.Errorf("lectura del precio %q: %w", texto, err)
	}
	return precio, nil
}

func main() {
	_, err := leerPrecio("diez")
	fmt.Println(err)
}
```

```sortie
lectura del precio "diez": strconv.Atoi: parsing "diez": invalid syntax
```

El mensaje cuenta toda la historia, de lo más general a lo más preciso.

## Los errores centinela y errors.Is

Un **error centinela** es un error declarado una sola vez, en una variable, para poder
reconocerlo. Su nombre empieza por `Err`. La biblioteca estándar tiene varios, como
`io.EOF` («fin del archivo»).

`errors.Is(err, ErrX)` responde: «¿está `ErrX` dentro de `err`, aunque sea en el fondo
de la caja?». Abre todos los envoltorios hechos con `%w`.

```go
package main

import (
	"errors"
	"fmt"
)

var ErrNoEncontrado = errors.New("no encontrado")

var stock = map[string]int{"manzana": 3}

func cantidad(fruta string) (int, error) {
	n, ok := stock[fruta]
	if !ok {
		return 0, fmt.Errorf("fruta %q: %w", fruta, ErrNoEncontrado)
	}
	return n, nil
}

func main() {
	for _, f := range []string{"manzana", "kiwi"} {
		n, err := cantidad(f)
		if errors.Is(err, ErrNoEncontrado) {
			fmt.Println(err, "-> pedimos", f+"s")
			continue
		}
		fmt.Println(f, ":", n)
	}
}
```

```sortie
manzana : 3
fruta "kiwi": no encontrado -> pedimos kiwis
```

> **Atención:** no compares los mensajes (`err.Error() == "no encontrado"`). Un
> mensaje puede cambiar; la variable centinela, no.

## Tus propios tipos de error

`error` es una simple [interfaz](interfaces.md):

```go norun
type error interface {
	Error() string
}
```

Por lo tanto, todo tipo que tenga un método `Error() string` es un error. Un tipo de
error puede llevar información además del mensaje, como el campo incorrecto.

`errors.As(err, &destino)` busca en `err` un error del tipo de `destino`. Si encuentra
uno, lo guarda en `destino` y devuelve `true`.

```go
package main

import (
	"errors"
	"fmt"
)

type ErrorValidacion struct {
	Campo  string
	Motivo string
}

func (e *ErrorValidacion) Error() string {
	return e.Campo + ": " + e.Motivo
}

func inscribir(nombre string, edad int) error {
	if nombre == "" {
		return &ErrorValidacion{Campo: "nombre", Motivo: "obligatorio"}
	}
	if edad < 13 {
		return &ErrorValidacion{Campo: "edad", Motivo: "13 años como mínimo"}
	}
	return nil
}

func main() {
	err := inscribir("Lou", 9)
	err = fmt.Errorf("inscripción rechazada: %w", err)
	fmt.Println(err)

	var ev *ErrorValidacion
	if errors.As(err, &ev) {
		fmt.Println("Campo que corregir:", ev.Campo)
	}
}
```

```sortie
inscripción rechazada: edad: 13 años como mínimo
Campo que corregir: edad
```

> **Recuerda:** `errors.Is` busca **un valor** preciso (un centinela).
> `errors.As` busca **un tipo** de error y te da acceso a sus campos.

## panic y recover

`panic` detiene el programa de inmediato, con un mensaje y la traza de las llamadas.
Es el [pánico](../debuter/glossaire.md#panico). Go entra en pánico solo en algunos
casos: división entera por cero, índice fuera de una slice, puntero `nil`…

`recover`, llamado en una función `defer`, atrapa un pánico. `defer` programa una
llamada para el final de la función, pase lo que pase.

```go
package main

import "fmt"

func dividirProtegido(a, b int) (resultado int, err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("atrapado: %v", r)
		}
	}()
	return a / b, nil
}

func main() {
	fmt.Println(dividirProtegido(10, 2))
	fmt.Println(dividirProtegido(1, 0))
	fmt.Println("El programa continúa.")
}
```

```sortie
5 <nil>
0 atrapado: runtime error: integer divide by zero
El programa continúa.
```

### ¿Cuándo entrar en pánico?

Casi nunca. Un pánico señala un **bug**: una situación que no debería ocurrir nunca si
el código fuera correcto. Para todo lo que puede fallar normalmente (un archivo que
falta, una entrada incorrecta, una red caída), devuelve un `error`.

- Entrada no válida, archivo que falta: `return ..., err`.
- Bug del programador, estado imposible: `panic`.
- `recover`: solo para que un servidor, por ejemplo, sobreviva a un bug en una
  petición. No para sustituir a `if err != nil`.

## Errores frecuentes

### Olvidar el valor de error

```go norun
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n := strconv.Atoi("12")
	fmt.Println(n)
}
```

```text
# command-line-arguments
./main.go:9:7: assignment mismatch: 1 variable but strconv.Atoi returns 2 values
```

`strconv.Atoi` devuelve dos valores: el número y el error. Escribe
`n, err := strconv.Atoi("12")`, y luego comprueba `err`.

### Recuperar el error sin comprobarlo

```go norun
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("12")
	fmt.Println(n)
}
```

```text
# command-line-arguments
./main.go:9:5: declared and not used: err
```

Go rechaza una variable sin usar. Añade la comprobación `if err != nil { ... }`.

### Ignorar el error con _

```go norun
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, _ := strconv.Atoi("doce")
	fmt.Println(100 / n)
}
```

```text
panic: runtime error: integer divide by zero

goroutine 1 [running]:
main.main()
	/ruta/a/main.go:10 +0x78
exit status 2
```

El `_` tira el error: `n` vale `0` sin avisar, y la división entra en pánico dos líneas
más abajo. El verdadero problema está lejos del mensaje. Guarda el error y compruébalo.

### Envolver con %v en lugar de %w

```go norun
err := fmt.Errorf("fruta %q: %v", fruta, ErrNoEncontrado)
fmt.Println(errors.Is(err, ErrNoEncontrado))
```

```text
false
```

Con `%v`, solo se copia el **texto** del error: el error original se pierde y
`errors.Is` no lo encuentra. Usa `%w`.

## Ponte a prueba

**Pregunta 1:** por costumbre, ¿qué devuelve una función `func f() (int, error)` cuando
todo va bien?

> **Respuesta**
>
> El resultado y `nil`: `return resultado, nil`. Un error `nil` quiere decir «no hay
> error».

**Pregunta 2:** ¿qué diferencia hay entre `errors.Is` y `errors.As`?

> **Respuesta**
>
> `errors.Is` comprueba si un error preciso (un centinela como `io.EOF`) está en la
> cadena. `errors.As` busca un error de cierto **tipo** y lo guarda en una variable
> para leer sus campos.

**Pregunta 3:** escribe una función `retirar(saldo, importe int) (int, error)` que
devuelva el error centinela `ErrSaldoInsuficiente` si el importe supera el saldo.

> **Respuesta**
>
> ```go
> package main
>
> import (
> 	"errors"
> 	"fmt"
> )
>
> var ErrSaldoInsuficiente = errors.New("saldo insuficiente")
>
> func retirar(saldo, importe int) (int, error) {
> 	if importe > saldo {
> 		return saldo, ErrSaldoInsuficiente
> 	}
> 	return saldo - importe, nil
> }
>
> func main() {
> 	fmt.Println(retirar(100, 30))
> 	_, err := retirar(10, 50)
> 	fmt.Println(errors.Is(err, ErrSaldoInsuficiente))
> }
> ```
>
> ```sortie
> 70 <nil>
> true
> ```

## En resumen

- Un error es un valor de tipo `error`, devuelto en último lugar; `nil` quiere decir
  «no hay error».
- Comprueba cada error enseguida con `if err != nil`.
- `errors.New` y `fmt.Errorf` crean errores; `%w` envuelve otro.
- `errors.Is` reconoce un centinela; `errors.As` encuentra un tipo de error.
- `panic` se reserva para los bugs; `recover` en un `defer` lo atrapa.

Siguiente: [paquetes y módulos](modules.md). Compara con
[los errores en Rust](../rust/erreurs.md).
