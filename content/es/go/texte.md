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

Casi todos los programas manejan texto: un nombre, un mensaje, una línea leída de un
archivo. En Go, el texto tiene su propio tipo, igual que los números tienen el suyo.

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

Línea por línea:

- `nombre := "Alicia"` guarda el texto «Alicia» en la variable `nombre`;
- la línea siguiente pega tres trozos: `"Hola, "`, el contenido de `nombre` y luego
  `"."`. Así, `saludo` vale `"Hola, Alicia."`;
- `fmt.Println(saludo)` muestra ese texto;
- `nombre == "Alicia"` hace una pregunta: «¿son idénticos los dos textos?». La
  respuesta es sí, de ahí el `true`.

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

**Te toca:** escribe un programa que guarde `"Sevilla"` en una variable `ciudad` y
luego muestre «Vivo en Sevilla.» pegando los trozos con `+`.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	ciudad := "Sevilla"
> 	fmt.Println("Vivo en " + ciudad + ".")
> }
> ```
>
> ```sortie
> Vivo en Sevilla.
> ```

## Bytes y runas

¿Por qué preocuparte por esto? En cuanto una palabra lleva una tilde o una `ñ`, contar
sus letras o tomar su primera letra ya no funciona como esperas. Saber cómo guarda Go
el texto te evita esas sorpresas.

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

Línea por línea:

- `palabra := "ñandú"` guarda una palabra de 5 letras, dos de ellas especiales (`ñ` y
  `ú`);
- `len(palabra)` cuenta los bytes: 2 para `ñ`, 2 para `ú` y 1 para cada una de las
  otras tres letras, o sea 7;
- `utf8.RuneCountInString(palabra)` cuenta los caracteres: 5;
- `len("abc")` da 3: sin tildes, un carácter ocupa un solo byte.

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

**Te toca:** escribe un programa que recorra la palabra `"niño"` con `range` y muestre
cada letra en su propia línea.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for _, c := range "niño" {
> 		fmt.Printf("%c\n", c)
> 	}
> }
> ```
>
> ```sortie
> n
> i
> ñ
> o
> ```

## El paquete strings

Cortar una frase en palabras, poner un texto en mayúsculas, comprobar que una dirección
contiene una `@`: son necesidades que aparecen todo el tiempo. En lugar de reescribirlo
todo, tomas la herramienta ya hecha de esta caja de herramientas.

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

Línea por línea:

- `frase` contiene el texto en el que buscas;
- cada línea siguiente hace una pregunta sobre `frase` y muestra la respuesta;
- la frase no termina en «cama», de ahí el `false`;
- «duerme» empieza en la posición 8, porque se cuenta desde 0: `E` está en 0, `l` en 1,
  el espacio en 2, y así sucesivamente;
- la letra `e` aparece 4 veces: dos en «duerme», una en «en» y una en «el». La `E`
  mayúscula de «El» no cuenta.

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

**Te toca:** escribe un programa que corte el texto `"lunes,martes,miércoles"` en cada
coma, muestre el número de días y luego los días en mayúsculas, separados por ` - `.

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
> 	dias := strings.Split("lunes,martes,miércoles", ",")
> 	fmt.Println(len(dias))
> 	fmt.Println(strings.ToUpper(strings.Join(dias, " - ")))
> }
> ```
>
> ```sortie
> 3
> LUNES - MARTES - MIÉRCOLES
> ```

## Construir texto

Un programa a menudo tiene que fabricar un texto con varios trozos: un nombre, una nota,
una fecha. Es como rellenar un formulario: el modelo no cambia, solo cambian las
casillas.

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

Línea por línea:

- `nombre` vale `"Alicia"` y `media` vale `14.256`;
- `%-8s` coloca el nombre en una casilla de 8 caracteres, pegado a la izquierda:
  `Alicia` va seguido de 2 espacios. La `|` se copia tal cual;
- `%6.1f` escribe el número con una sola cifra decimal (`14.3`), en una casilla de 6
  caracteres, pegado a la derecha;
- en ese momento no se muestra nada: el texto se guarda en `linea` y luego lo muestra
  `Println`. Mide 8 + 1 + 6 = 15 caracteres.

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

**Te toca:** escribe un programa que guarde `"Alicia"` en `nombre` y `12` en `edad`,
fabrique con `fmt.Sprintf` el texto «Alicia tiene 12 años.», lo guarde en una variable
`frase` y luego lo muestre.

> **Solución**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	nombre := "Alicia"
> 	edad := 12
> 	frase := fmt.Sprintf("%s tiene %d años.", nombre, edad)
> 	fmt.Println(frase)
> }
> ```
>
> ```sortie
> Alicia tiene 12 años.
> ```

## Texto y números: strconv

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

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

Línea por línea:

- `strconv.Itoa(edad)` convierte el número 12 en el texto `"12"`, que `+` puede pegar;
- `strconv.Atoi("2026")` hace lo contrario: lee el texto y da el número 2026. También
  devuelve `err`, que vale `nil` («nada») cuando todo ha ido bien;
- `strconv.ParseBool("true")` lee el texto `"true"` y da el booleano `true`. El `_`
  ignora el error;
- `strconv.Quote` rodea el texto de comillas y añade las `\` necesarias.

> **Atención:** `string(65)` no da `"65"`, sino `"A"` (la letra de código 65).
> Para convertir un número en texto, usa siempre `strconv.Itoa` o `fmt.Sprint`.

## Las cadenas en bruto

En una cadena normal, cada `\` debe ir doble: una ruta de Windows se vuelve difícil de
leer enseguida, como `"C:\\Users\\alicia"`. Las cadenas en bruto evitan ese problema.

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

Línea por línea:

- `ruta` contiene `\` copiadas tal cual: la `\n` de `\notas` no pasa a la línea
  siguiente;
- `poema` ocupa dos líneas: el salto de línea forma parte del texto.

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
