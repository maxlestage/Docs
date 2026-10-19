# Las condiciones

Esta página muestra cómo comparar valores y tomar decisiones con `if` y `switch`.

> **Objetivos:**
>
> - sabrás comparar valores y combinar condiciones con `&&`, `||` y `!`;
> - sabrás escribir una condición con `if`, `else if` y `else`;
> - sabrás elegir entre varios casos con `switch`.

## Comparar valores

Un programa a menudo tiene que decidir: abrir la puerta o no, mostrar «ganaste» o
«perdiste». Para decidir, primero hace una pregunta, igual que tú te preguntas
«¿tengo suficiente dinero para este pastel?».

Una [condición](../debuter/glossaire.md#condicion) es una pregunta que se
responde con verdadero (`true`) o falso (`false`). Se construye con operadores de
comparación.

| Operador | Significado | Ejemplo | Resultado |
|-----------|------|---------|----------|
| `==` | igual a | `3 == 3` | `true` |
| `!=` | distinto de | `3 != 4` | `true` |
| `<`, `>` | menor, mayor | `2 > 5` | `false` |
| `<=`, `>=` | menor o igual, mayor o igual | `5 >= 5` | `true` |

Las condiciones se combinan con los operadores lógicos:

- `&&`: «y» (las dos deben ser verdaderas);
- `||`: «o» (al menos una debe ser verdadera);
- `!`: «no» (invierte verdadero y falso).

```go
package main

import "fmt"

func main() {
	edad := 15
	conPadres := true
	fmt.Println(edad >= 18)
	fmt.Println(edad >= 18 || conPadres)
	fmt.Println(edad > 12 && edad < 18)
	fmt.Println(!conPadres)
}
```

```sortie
false
true
true
false
```

Línea por línea:

- `edad := 15` guarda 15 en `edad`, y `conPadres := true` guarda «verdadero» en
  `conPadres`;
- `edad >= 18` pregunta «¿15 es mayor o igual que 18?». No: `false`;
- con `||`, basta una respuesta verdadera: `conPadres` vale `true`, así que `true`;
- con `&&`, hacen falta las dos: 15 es mayor que 12 **y** menor que 18, así que
  `true`;
- `!conPadres` invierte `true`: se obtiene `false`.

> **Atención:** `=` guarda un valor, `==` compara dos valores. No los confundas.

## if y else

En la vida tomas decisiones todo el tiempo: si llueve, coges un paraguas; si no, lo
dejas en casa. `if` le da ese poder a tu programa: hacer una cosa u otra según la
situación.

`if` («si») ejecuta un bloque solo si la condición es verdadera. `else` («si no»)
da el bloque que se ejecuta en caso contrario.

```go
package main

import "fmt"

func main() {
	temperatura := 28
	if temperatura > 25 {
		fmt.Println("Hace calor.")
	} else {
		fmt.Println("Hace buen tiempo.")
	}
}
```

```sortie
Hace calor.
```

Línea por línea:

- `temperatura := 28` guarda 28 en la variable `temperatura`;
- `if temperatura > 25 {` hace la pregunta «¿28 es mayor que 25?». La respuesta
  es sí;
- así que Go ejecuta el primer bloque y muestra «Hace calor.»;
- el bloque del `else` se salta: solo sirve si la respuesta es no.

En Go:

- no hay paréntesis alrededor de la condición;
- las llaves `{ }` son **obligatorias**, incluso para una sola línea;
- `else` se coloca en la misma línea que la llave de cierre: `} else {`.

### else if

Para más de dos casos, encadena con `else if`. Go prueba las condiciones en orden y
se detiene en la primera verdadera.

```go
package main

import "fmt"

func main() {
	nota := 14
	if nota >= 16 {
		fmt.Println("Sobresaliente")
	} else if nota >= 12 {
		fmt.Println("Notable")
	} else if nota >= 10 {
		fmt.Println("Aprobado")
	} else {
		fmt.Println("Suspenso")
	}
}
```

```sortie
Notable
```

`nota >= 10` también es verdadera, pero Go ni siquiera la prueba: ya ha encontrado
`nota >= 12`.

**Te toca:** escribe un programa que guarde `10` en una variable `edad` y luego
muestre «Niño» si la edad es menor que 13, «Adolescente» si es menor que 18, y
«Adulto» en otro caso.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	edad := 10
> 	if edad < 13 {
> 		fmt.Println("Niño")
> 	} else if edad < 18 {
> 		fmt.Println("Adolescente")
> 	} else {
> 		fmt.Println("Adulto")
> 	}
> }
> ```
>
> ```sortie
> Niño
> ```

### if con una instrucción de inicialización

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Go permite colocar una pequeña instrucción **antes** de la condición, separada por un `;`.
La variable creada ahí solo existe dentro del `if` y sus `else`.

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	if n, err := strconv.Atoi("42"); err != nil {
		fmt.Println("No es un número:", err)
	} else {
		fmt.Println("El doble vale", n*2)
	}
}
```

```sortie
El doble vale 84
```

Esta forma es muy habitual en Go, sobre todo para comprobar un error. Mantiene las
variables temporales allí donde se usan.

## switch

Piensa en una máquina de bebidas: según el botón que pulses, sirve una bebida
distinta. `switch` funciona igual: según el valor, elige un bloque.

Cuando un mismo valor puede tomar varias formas, `switch` es más legible que una
larga serie de `else if`.

```go
package main

import "fmt"

func main() {
	dia := "sábado"
	switch dia {
	case "sábado", "domingo":
		fmt.Println("¡Es fin de semana!")
	case "miércoles":
		fmt.Println("Mitad de semana.")
	default:
		fmt.Println("A trabajar.")
	}
}
```

```sortie
¡Es fin de semana!
```

Línea por línea:

- `dia` contiene el texto `"sábado"`;
- `switch dia {` compara el valor de `dia` con cada `case`, de arriba abajo;
- el primer `case` lista `"sábado"` y `"domingo"`: `"sábado"` coincide, así que
  Go muestra «¡Es fin de semana!»;
- los demás casos, y `default`, se ignoran.

Recuerda tres cosas sobre `switch`:

- Cada `case` da uno o varios valores, separados por comas.
- `default` («por defecto») se ejecuta si ningún caso coincide.
- Go se detiene en el **primer** caso que coincide y luego sale del `switch`.

> **Nota:** a diferencia de C o Java, no hace falta `break` al final de cada
> caso: Go no «cae» en el caso siguiente. Para forzarlo, existe la palabra
> `fallthrough`, pero casi nunca se usa.

**Te toca:** escribe un programa que guarde `"limón"` en una variable `fruta` y
luego, con un `switch`, muestre «Fruta de huerto» para una manzana o una pera,
«Cítrico» para un limón o una naranja, y «Fruta desconocida» en otro caso.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	fruta := "limón"
> 	switch fruta {
> 	case "manzana", "pera":
> 		fmt.Println("Fruta de huerto")
> 	case "limón", "naranja":
> 		fmt.Println("Cítrico")
> 	default:
> 		fmt.Println("Fruta desconocida")
> 	}
> }
> ```
>
> ```sortie
> Cítrico
> ```

### switch sin valor

Un `switch` sin valor prueba condiciones, como un `switch true`. Es una forma elegante
de escribir una serie de `else if`.

```go
package main

import "fmt"

func main() {
	hora := 14
	switch {
	case hora < 12:
		fmt.Println("Buenos días")
	case hora < 20:
		fmt.Println("Buenas tardes")
	default:
		fmt.Println("Buenas noches")
	}
}
```

```sortie
Buenas tardes
```

## Errores frecuentes

### Poner un número como condición

```go norun
package main

import "fmt"

func main() {
	n := 3
	if n {
		fmt.Println("verdadero")
	}
}
```

```text
# command-line-arguments
./main.go:7:5: non-boolean condition in if statement
```

En Go, una condición debe ser un booleano de verdad. `3` no es ni verdadero ni falso. Escribe la
comparación: `if n != 0 {`.

### Poner else en una línea nueva

```go norun
package main

import "fmt"

func main() {
	edad := 20
	if edad >= 18 {
		fmt.Println("mayor de edad")
	}
	else {
		fmt.Println("menor de edad")
	}
}
```

```text
# command-line-arguments
./main.go:10:2: syntax error: unexpected keyword else, expected }
```

`else` debe ir justo después de la llave de cierre: `} else {`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este programa?

```go norun
package main

import "fmt"

func main() {
	x := 7
	switch {
	case x > 5:
		fmt.Println("grande")
	case x > 2:
		fmt.Println("mediano")
	}
}
```

> **Respuesta**
>
> `grande`, y nada más. Go se detiene en el primer caso verdadero y no pasa al siguiente.

**Pregunta 2:** ¿cuál es la diferencia entre `=` y `==`?

> **Respuesta**
>
> `=` guarda un valor en una variable. `==` compara dos valores y da `true` o
> `false`.

**Pregunta 3:** ¿qué muestra `fmt.Println(5 > 3 && 2 > 4)`?

> **Respuesta**
>
> `false`. `5 > 3` es verdadero, pero `2 > 4` es falso, y `&&` exige que las dos sean
> verdaderas.

## En resumen

- `==`, `!=`, `<`, `>`, `<=` y `>=` comparan; `&&`, `||` y `!` combinan
  condiciones.
- `if`, `else if` y `else` toman decisiones; las llaves son obligatorias.
- `if x := ...; condición {` crea una variable que solo vive dentro del `if`.
- `switch` elige entre varios casos, sin `break`; sin valor, prueba
  condiciones.

Siguiente: [los bucles](boucles.md).
