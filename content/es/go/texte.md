# Trabajar con texto

Esta página explica cómo guarda Go el texto (bytes y runas), y cómo cortarlo, transformarlo y juntarlo con los paquetes `strings`, `fmt` y `strconv`.

> **Objetivos:**
>
> - sabrás qué son los bytes y las runas de una cadena;
> - sabrás contar los caracteres y recorrer un texto con `range`;
> - sabrás usar las funciones habituales del paquete `strings`;
> - sabrás construir texto con `strings.Builder` y `fmt.Sprintf`;
> - sabrás escribir un texto en bruto entre acentos graves.

## Las cadenas de caracteres

Un texto se llama una [cadena de caracteres](../debuter/glossaire.md#cadena-de-caracteres),
de tipo `string`. Se escribe entre comillas dobles.

```go
package main

import "fmt"

func main() {
	nombre := "Alicia"
	saludo := "Hola, " + nombre + "."
	fmt.Println(saludo)
	fmt.Println(nombre == "Alicia")
}
```

```sortie
Hola, Alicia.
true
```

- `+` pega dos cadenas una detrás de otra;
- `==` compara dos cadenas.

Algunos caracteres especiales se escriben con una barra invertida `\`:

| Escritura | Significado |
|----------|------|
| `\n` | pasar a la línea siguiente |
| `\t` | una tabulación |
| `\"` | unas comillas dentro del texto |
| `\\` | la propia barra `\` |

```go
package main

import "fmt"

func main() {
	fmt.Println("Línea 1\nLínea 2")
	fmt.Println("Dijo: \"Hola\"")
}
```

```sortie
Línea 1
Línea 2
Dijo: "Hola"
```

> **Recuerda:** una cadena de Go es **inmutable**: no puedes cambiar una de sus
> letras. Todas las funciones que «modifican» un texto en realidad crean uno nuevo.

## Bytes y runas

Una cadena de Go es una secuencia de [bytes](../debuter/glossaire.md#byte) (números de
0 a 255), codificados en UTF-8. Las letras simples (`a`, `Z`, `5`) ocupan 1 byte. Las
letras con tilde ocupan 2, y un emoji ocupa 4.

Un carácter se llama **runa** (*rune*) en Go. Una runa se escribe entre comillas
simples: `'é'`.

`len` cuenta los **bytes**, no los caracteres. Para contar los caracteres, usa
`utf8.RuneCountInString`.

```go
package main

import (
	"fmt"
	"unicode/utf8"
)

func main() {
	palabra := "ñandú"
	fmt.Println(len(palabra))
	fmt.Println(utf8.RuneCountInString(palabra))
	fmt.Println(len("abc"))
}
```

```sortie
7
5
3
```

`"ñandú"` tiene 5 caracteres, pero 7 bytes: `ñ` y `ú` ocupan 2 cada una.

### Leer un byte o una runa

`palabra[i]` da el **byte** número `i`. Para obtener los caracteres, convierte la
cadena en una slice de runas con `[]rune(palabra)`.

```go
package main

import "fmt"

func main() {
	palabra := "café"
	fmt.Println(palabra[0])
	fmt.Printf("%c\n", palabra[0])

	runas := []rune(palabra)
	fmt.Println(len(runas))
	fmt.Printf("%c\n", runas[3])
}
```

```sortie
99
c
4
é
```

> **Atención:** `palabra[3]` no da `é`, sino el primero de los dos bytes de `é`.
> En cuanto un texto pueda contener tildes, pasa por `[]rune` o por `range`.

### Recorrer un texto con range

`range` sobre una cadena avanza **carácter por carácter**. Da la posición (en bytes)
y la runa.

```go
package main

import "fmt"

func main() {
	for i, c := range "Año" {
		fmt.Printf("%d %c\n", i, c)
	}
}
```

```sortie
0 A
1 ñ
3 o
```

La posición salta de 1 a 3: `ñ` ocupa los bytes 1 y 2.

## El paquete strings

El paquete `strings` contiene las herramientas básicas para el texto.

### Buscar

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	frase := "El gato duerme en el sofá"
	fmt.Println(strings.Contains(frase, "gato"))
	fmt.Println(strings.HasPrefix(frase, "El"))
	fmt.Println(strings.HasSuffix(frase, "cama"))
	fmt.Println(strings.Index(frase, "duerme"))
	fmt.Println(strings.Count(frase, "e"))
}
```

```sortie
true
true
false
8
4
```

- `Contains`: ¿el texto contiene este trozo?
- `HasPrefix`, `HasSuffix`: ¿empieza, o termina, por este trozo?
- `Index`: la posición del trozo (`-1` si no está);
- `Count`: cuántas veces aparece el trozo.

### Transformar

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Println(strings.ToUpper("hola"))
	fmt.Println(strings.ToLower("CANCIÓN"))
	fmt.Println(strings.Replace("a-b-c", "-", "+", 1))
	fmt.Println(strings.ReplaceAll("a-b-c", "-", "+"))
	fmt.Println(strings.Repeat("ja", 3))
	fmt.Printf("[%s]\n", strings.TrimSpace("   hola  \n"))
}
```

```sortie
HOLA
canción
a+b-c
a+b+c
jajaja
[hola]
```

- `Replace(texto, viejo, nuevo, n)` reemplaza las `n` primeras veces; `ReplaceAll`
  las reemplaza todas;
- `TrimSpace` quita los espacios y saltos de línea del principio y del final. Muy útil
  para limpiar lo que ha escrito un usuario.

### Cortar y volver a pegar

`strings.Split` corta un texto en cada separador y devuelve una slice de trozos.
`strings.Join` hace lo contrario.

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	linea := "manzana;kiwi;pera"
	frutas := strings.Split(linea, ";")
	fmt.Println(len(frutas), frutas[1])

	fmt.Println(strings.Join(frutas, ", "))

	palabras := strings.Fields("  uno   dos\ttres  ")
	fmt.Println(len(palabras), palabras)
}
```

```sortie
3 kiwi
manzana, kiwi, pera
3 [uno dos tres]
```

`strings.Fields` corta en los espacios, sea cual sea su número. Es la mejor opción
para separar las palabras de una frase.

## Construir texto

### fmt.Sprintf

`fmt.Sprintf` funciona como `Printf`, pero en lugar de mostrar, **devuelve** el texto.

```go
package main

import "fmt"

func main() {
	nombre := "Alicia"
	media := 14.256
	linea := fmt.Sprintf("%-8s|%6.1f", nombre, media)
	fmt.Println(linea)
	fmt.Println(len(linea))
}
```

```sortie
Alicia  |  14.3
15
```

### strings.Builder

Pegar textos con `+` dentro de un bucle crea muchas cadenas intermedias. Para
construir un texto largo trozo a trozo, usa un `strings.Builder`: es como un
cuaderno en el que vas añadiendo líneas.

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	var sb strings.Builder
	for i := 1; i <= 3; i++ {
		sb.WriteString("Línea ")
		fmt.Fprintf(&sb, "%d\n", i)
	}
	sb.WriteString("Fin")
	fmt.Println(sb.String())
}
```

```sortie
Línea 1
Línea 2
Línea 3
Fin
```

- `sb.WriteString(...)` añade un texto;
- `fmt.Fprintf(&sb, ...)` añade un texto con formato;
- `sb.String()` devuelve el texto completo.

El `&` delante de `sb` se explica en [los punteros](pointeurs.md).

## Texto y números: strconv

Para pegar un número a un texto, primero hay que convertirlo. El paquete `strconv`
se encarga de ello (mira también [números y cálculos](nombres.md)).

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	edad := 12
	fmt.Println("Tengo " + strconv.Itoa(edad) + " años")

	n, err := strconv.Atoi("2026")
	fmt.Println(n+1, err)

	ok, _ := strconv.ParseBool("true")
	fmt.Println(ok)

	fmt.Println(strconv.Quote("dijo \"sí\""))
}
```

```sortie
Tengo 12 años
2027 <nil>
true
"dijo \"sí\""
```

> **Atención:** `string(65)` no da `"65"`, sino `"A"` (la letra de código 65).
> Para convertir un número en texto, usa siempre `strconv.Itoa` o `fmt.Sprint`.

## Las cadenas en bruto

Entre acentos graves `` ` ``, una cadena está **en bruto**: las `\` no se
interpretan, y el texto puede ocupar varias líneas.

```go
package main

import "fmt"

func main() {
	ruta := `C:\Users\alicia\notas.txt`
	fmt.Println(ruta)

	poema := `El cielo es azul,
	y el mar también.`
	fmt.Println(poema)
}
```

```sortie
C:\Users\alicia\notas.txt
El cielo es azul,
	y el mar también.
```

Todo se guarda tal cual, incluso la tabulación al principio de la segunda línea. Es
práctico para una ruta de Windows, una plantilla de texto, JSON o una
[expresión regular](regex.md).

## Errores frecuentes

### Modificar una letra de una cadena

```go norun
package main

import "fmt"

func main() {
	palabra := "hola"
	palabra[0] = 'H'
	fmt.Println(palabra)
}
```

```text
# command-line-arguments
./main.go:7:2: cannot assign to palabra[0] (neither addressable nor a map index expression)
```

Una cadena es inmutable. Crea una nueva: `palabra = "H" + palabra[1:]`, o pasa por
`[]rune(palabra)` y luego `string(runas)`.

### Usar comillas simples para un texto

```go norun
package main

import "fmt"

func main() {
	fmt.Println('Hola')
}
```

```text
# command-line-arguments
./main.go:6:14: more than one character in rune literal
```

Las comillas simples `'...'` están reservadas para **un solo** carácter (una runa).
Para un texto, usa las comillas dobles: `"Hola"`.

### Pegar un número a un texto con +

```go norun
package main

import "fmt"

func main() {
	edad := 12
	fmt.Println("Tengo " + edad + " años")
}
```

```text
# command-line-arguments
./main.go:7:14: invalid operation: "Tengo " + edad (mismatched types untyped string and int)
```

No se puede sumar un texto y un número. Convierte con `strconv.Itoa(edad)`, o
escribe `fmt.Println("Tengo", edad, "años")`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto valen `len("añ")` y `utf8.RuneCountInString("añ")`?

> **Respuesta**
>
> `3` y `2`. `a` ocupa 1 byte, `ñ` ocupa 2: 3 bytes, pero 2 caracteres.

**Pregunta 2:** a partir de `"  alice,BOB , Chloé "`, muestra los nombres en
mayúsculas, separados por ` / `.

> **Respuesta**
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
> 	trozos := strings.Split("  alice,BOB , Chloé ", ",")
> 	for i, t := range trozos {
> 		trozos[i] = strings.ToUpper(strings.TrimSpace(t))
> 	}
> 	fmt.Println(strings.Join(trozos, " / "))
> }
> ```
>
> ```sortie
> ALICE / BOB / CHLOÉ
> ```

**Pregunta 3:** ¿por qué es mejor `strings.Builder` que `+` en un bucle grande?

> **Respuesta**
>
> Cada `+` crea una cadena completa nueva, lo que se vuelve lento cuando el texto
> crece. `strings.Builder` añade los trozos en el mismo sitio, sin copiarlo todo.

## En resumen

- Una cadena es una secuencia de bytes en UTF-8, y es inmutable.
- Un carácter es una `rune` (`'é'`); `len` cuenta los bytes,
  `utf8.RuneCountInString` los caracteres.
- `range` sobre una cadena da las runas una a una.
- `strings` busca (`Contains`, `HasPrefix`), transforma (`ToUpper`, `Replace`,
  `TrimSpace`) y corta (`Split`, `Fields`, `Join`).
- `fmt.Sprintf` y `strings.Builder` construyen texto; `strconv` convierte los
  números.
- Los acentos graves `` `...` `` hacen una cadena en bruto, en varias líneas.

Siguiente: [arrays y slices](collections.md). Compara con
[el texto en Ruby](../ruby/texte.md).
