# Primeros pasos

Esta página presenta las bases de Rythme: dar un nombre a un valor, los tipos de valores, los cálculos, los textos, cómo mostrar resultados y los comentarios.

> **Objetivos:**
>
> - sabrás dar un nombre a un valor, y por qué ese nombre ya no cambia;
> - sabrás reconocer los tipos de valores simples: `Int`, `Float`, `Text`, `Bool`;
> - sabrás calcular con `+`, `-`, `*`, `/`, `//`, `%` y `**`;
> - sabrás insertar un valor en un texto y mostrarlo;
> - sabrás escribir un comentario.

## Dar un nombre a un valor

Un [valor](../debuter/glossaire.md#valor) es un dato: un número, un
texto... Para encontrarlo más tarde, le damos un **nombre**. Se escribe el
nombre, `=` y luego el valor:

```rythme
nombre = "Ada"
nacimiento = 1815
print(nombre, nacimiento)
```

```sortie
Ada 1815
```

Línea por línea:

- `nombre = "Ada"` guarda el texto `"Ada"` bajo el nombre `nombre`;
- `nacimiento = 1815` hace lo mismo con el número 1815;
- `print(nombre, nacimiento)` encuentra los dos valores gracias a sus
  nombres y luego los muestra.

Imagina una etiqueta pegada en una caja: `nombre` es la etiqueta, `"Ada"` es
lo que hay dentro de la caja. En otros lenguajes, a esto se le llama
[variable](../debuter/glossaire.md#variable).

`print` muestra sus valores separados por un espacio y luego pasa a la línea
siguiente.

## Los nombres no cambian nunca

Un valor que cambia por el camino es una fuente frecuente de errores: ya no
sabes cuánto vale en tal punto del programa. Rythme evita este problema con
una regla sencilla.

En Rythme, un valor no cambia nunca: es
[inmutable](../debuter/glossaire.md#inmutable). Un nombre designa un solo
valor, para siempre. Si quieres otro valor, dale un nombre nuevo:

```rythme
total = 10
total2 = total + 5
print(total, total2)
```

```sortie
10 15
```

Línea por línea:

- `total = 10`: el nombre `total` designa 10;
- `total2 = total + 5` calcula 10 + 5 y luego da el resultado, 15, al nombre
  nuevo `total2`;
- `total` no se ha movido: por eso `print` muestra 10 y luego 15.

La etiqueta queda pegada para siempre: no se despega, se pega una nueva en
una caja nueva. Así es más fácil de seguir: cuando lees `total`, sabes que
vale 10 en todo el programa.

> **Recuerda:** nada de `x = x + 1`, nada de `x += 1`. Un valor nuevo
> recibe un nombre nuevo. Rythme rechaza las dos cosas (ver
> [los errores frecuentes](#errores-frecuentes)).

**Te toca:** escribe un programa que dé el nombre `manzanas` al valor 4,
luego el nombre `manzanas2` a `manzanas` más 3, y que muestre los dos.

> **Solución**
>
> ```rythme
> manzanas = 4
> manzanas2 = manzanas + 3
> print(manzanas, manzanas2)
> ```
>
> ```sortie
> 4 7
> ```

## Los tipos de valores

No se hace lo mismo con todos los valores: sumas dos números, pasas un texto
a mayúsculas, pero un número en mayúsculas no tiene sentido. Conocer el tipo
de un valor es saber qué puedes hacer con él.

Cada valor pertenece a una clase: su [tipo](../debuter/glossaire.md#tipo). Estos son
los cuatro más simples:

| Tipo | Ejemplos | Para qué |
|------|----------|----------|
| `Int` | `42`, `-7`, `1_000_000` | los números [enteros](../debuter/glossaire.md#entero) |
| `Float` | `3.14`, `2.0` | los números con decimales, o [de coma flotante](../debuter/glossaire.md#flotante) |
| `Text` | `"hola"` | los textos, o [cadenas de caracteres](../debuter/glossaire.md#cadena-de-caracteres) |
| `Bool` | `true`, `false` | verdadero o falso, los [booleanos](../debuter/glossaire.md#booleano) |

El `_` en `1_000_000` solo sirve para leer más fácilmente. Un número con
decimales se escribe con un **punto**: `3.14`, no `3,14`.

`type_of` da el tipo de un valor:

```rythme
print(type_of(42), type_of(3.14), type_of("hola"), type_of(true))
```

```sortie
Int Float Text Bool
```

Un `Float` siempre se muestra con un punto, incluso cuando es redondo:
`2.0`. Así nunca se confunde con el `Int` `2`.

## Calcular

Un programa sirve muy a menudo para calcular: el total de la compra, una
media, un reparto. Como en una calculadora, escribes el cálculo y Rythme da
el resultado.

Estos son los **operadores** de cálculo:

| Operador | Significa | Ejemplo | Resultado |
|----------|-----------|---------|-----------|
| `+` | más | `7 + 2` | `9` |
| `-` | menos | `7 - 2` | `5` |
| `*` | por | `7 * 2` | `14` |
| `/` | dividido entre | `7 / 2` | `3.5` |
| `//` | división entera | `7 // 2` | `3` |
| `%` | resto de la división | `7 % 2` | `1` |
| `**` | potencia | `7 ** 2` | `49` |

```rythme
print(2 + 3 * 4, (2 + 3) * 4)
print(7 / 2, 6 / 3, 7 // 2, 7 % 3, 2 ** 10)
```

```sortie
14 20
3.5 2.0 3 1 1024
```

Algunas reglas que conviene conocer:

- `*`, `/`, `//` y `%` van antes que `+` y `-`, como en la escuela. Los
  paréntesis cambian el orden.
- `/` da **siempre** un `Float`, incluso cuando la división es exacta:
  `6 / 3` vale `2.0`.
- `//` da la **división entera**: ¿cuántas veces cabe 2 en 7? 3 veces. `%`
  da lo que sobra: 7 = 3 × 2 + **1**.
- `%` sirve a menudo para saber si un número es par: `n % 2 == 0`.

Piensa en caramelos: 7 caramelos para 2 niños, cada uno recibe `7 // 2`,
o sea 3, y sobra `7 % 2`, o sea 1.

Los números enteros no tienen límite de tamaño:

```rythme
print(2 ** 100)
```

```sortie
1267650600228229401496703205376
```

Para redondear o convertir un número:

```rythme
print(round(3.7), round(3.14159, 2), int(3.9), float(2))
```

```sortie
4 3.14 3 2.0
```

`round(x)` redondea al entero más cercano, `round(x, 2)` conserva dos
decimales, `int` corta los decimales, `float` da un número con decimales.

**Te toca:** se reparten 17 caramelos entre 5 amigos. Escribe un programa
que muestre cuántos recibe cada uno y luego cuántos sobran.

> **Solución**
>
> ```rythme
> caramelos = 17
> amigos = 5
> print(caramelos // amigos, caramelos % amigos)
> ```
>
> ```sortie
> 3 2
> ```

## Los textos y la inserción

Un programa suele hablar con la persona que lo usa: «Hola, Ada», «Total: 15
euros». Por eso tiene que fabricar frases que contienen valores, un poco como
un formulario con huecos que se rellenan.

Un texto se escribe entre comillas rectas `"`. Para poner un valor dentro de
un texto, escríbelo entre llaves `{...}`: es la **inserción**.

```rythme
nombre = "Ada"
edad = 36
print("{nombre} tiene {edad} años.")
print("Dentro de diez años: {edad + 10} años.")
```

```sortie
Ada tiene 36 años.
Dentro de diez años: 46 años.
```

Línea por línea:

- `nombre` designa `"Ada"` y `edad` designa 36;
- en `"{nombre} tiene {edad} años."`, Rythme sustituye `{nombre}` por Ada y
  `{edad}` por 36, y luego muestra la frase;
- en la última línea, primero calcula `edad + 10`, o sea 46, y luego lo
  inserta en el texto.

Entre las llaves se puede escribir un cálculo, no solo un nombre. Como en
Ruby, `#{...}` también funciona:

```rythme
nombre = "Grace"
print("¡Hola, #{nombre}!")
```

```sortie
¡Hola, Grace!
```

`+` une dos textos uno detrás de otro. Pero Rythme nunca mezcla texto y
número en silencio: `"edad: " + 36` es un error. Escribe `"edad: {36}"`, o
convierte primero el número en texto con `text(36)`.

```rythme
print("Ho" + "la", "edad: " + text(36))
```

```sortie
Hola edad: 36
```

Algunas funciones útiles para los textos (la página
[trabajar con texto](texte.md) muestra muchas más):

```rythme
print(upper("hola"), len("café"), trim("  ok  "), "a,b,c" |> split(","))
```

```sortie
HOLA 4 ok ["a", "b", "c"]
```

`upper` pasa a mayúsculas, `len` cuenta los caracteres, `trim` quita los
espacios de alrededor, `split` corta en trozos.

**Te toca:** da el nombre `cruasanes` a 3 y el nombre `precio_unidad` a 2.
Con la inserción, muestra la frase `3 cruasanes cuestan 6 euros.`

> **Solución**
>
> ```rythme
> cruasanes = 3
> precio_unidad = 2
> print("{cruasanes} cruasanes cuestan {cruasanes * precio_unidad} euros.")
> ```
>
> ```sortie
> 3 cruasanes cuestan 6 euros.
> ```

## Mostrar con print y puts

Mostrar es tu única forma de ver lo que hace un programa: sin `print`,
calcula en silencio y no ves nada.

`print` y `puts` son la misma función. `puts` es el nombre de Ruby. Al
principio de una línea, incluso puedes olvidarte de los paréntesis:

```rythme
print("con paréntesis")
puts "sin paréntesis"
puts "dos valores:", 1 + 1
```

```sortie
con paréntesis
sin paréntesis
dos valores: 2
```

Línea por línea:

- la primera línea muestra su texto, con paréntesis;
- `puts "sin paréntesis"` hace exactamente lo mismo, sin paréntesis;
- la última línea da dos valores: un texto y el cálculo `1 + 1`. Rythme
  calcula 2 y luego muestra los dos.

Los valores que se dan a `print` se separan con comas. Se muestran
separados por un espacio.

## Los comentarios

`#` empieza un **comentario**, hasta el final de la línea. Rythme lo
ignora: es una nota para las personas, incluido tú dentro de tres meses.

```rythme
# Un comentario normal.
precio = 12  # un comentario después del código
print(precio * 2)
```

```sortie
24
```

Línea por línea:

- la primera línea es solo un comentario: Rythme se la salta;
- `precio = 12` se ejecuta, y el comentario que lo sigue se ignora;
- `print(precio * 2)` calcula 12 × 2 y muestra 24.

También verás `##`, dos almohadillas: es un comentario que describe la
función escrita justo debajo (ver
[documentar con rythme doc](projet.md)).

## Acentos en los nombres

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Un nombre bien elegido hace que un programa sea fácil de releer. Aun así,
debe seguir algunas reglas.

Un nombre empieza por una letra o `_`, y luego contiene letras, cifras y
`_`. Los acentos están permitidos:

```rythme
canción = "Bésame mucho"
año = 1940
print("{canción} es de {año}.")
```

```sortie
Bésame mucho es de 1940.
```

Las mayúsculas y las minúsculas cuentan: `edad` y `Edad` son dos nombres
distintos. Un nombre que empieza por mayúscula está reservado a los tipos y
a sus variantes (ver [tus propios tipos](types.md)).

## Errores frecuentes

### Cambiar el valor de un nombre

En muchos lenguajes se escribe `x = x + 1`. En Rythme no:

```rythme norun
total = 1
total = total + 1
print(total)
```

```text
error[E011]: `total` ya está definido
 --> main.ry:2:1
  |
2 | total = total + 1
  | ^^^^^ definido de nuevo aquí
  |
1 | total = 1
  | ----- definido por primera vez aquí
  |
  = ayuda: los valores nunca cambian en Rythme: da un nombre nuevo al valor nuevo, como `total2 = total + 1`
  = más sobre este error: `rythme explain E011`
```

El error se encuentra incluso antes de ejecutar el programa. Usa un nombre
nuevo: `total2 = total + 1`.

### Usar += para sumar

```rythme norun
puntos = 0
puntos += 10
```

```text
error[E011]: los valores no se pueden cambiar en Rythme
 --> main.ry:2:8
  |
2 | puntos += 10
  |        ^^
  |
  = ayuda: los valores nunca cambian en Rythme: da un nombre nuevo al valor nuevo, como `total2 = total + 1`
  = más sobre este error: `rythme explain E011`
```

`+=` modificaría `puntos`. Escribe mejor `puntos2 = puntos + 10`.

### Sumar un número a un texto

```rythme norun
edad = 36
print("edad: " + edad)
```

```text
error[E005]: no se puede usar `+` con Text y Int
 --> main.ry:2:16
  |
2 | print("edad: " + edad)
  |                ^ Text "edad: " + Int
  |
  = ayuda: para construir un texto con un valor dentro, usa llaves: "total: {x}"
      o convierte antes el valor: text(x)
  = más sobre este error: `rythme explain E005`
```

Rythme no adivina si quieres un texto o un número. Escribe
`print("edad: {edad}")`.

### Escribir let o var delante de un nombre

```rythme norun
let edad = 36
print(edad)
```

```text
error[E001]: `let` no se usa en Rythme
 --> main.ry:1:1
  |
1 | let edad = 36
  | ^^^ viene de otro lenguaje
  |
  = ayuda: escribe simplemente `nombre = valor` (los valores nunca cambian, así que no hace falta ninguna palabra clave)
  = más sobre este error: `rythme explain E001`
```

No hace falta ninguna palabra: `edad = 36`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto valen `9 / 2`, `9 // 2` y `9 % 2`?

> **Respuesta**
>
> `4.5`, `4` y `1`. `/` da siempre un número con decimales, `//` la
> división entera, `%` el resto.

**Pregunta 2:** ¿qué muestra este programa?

```rythme norun
precio = 5
cantidad = 3
print("Total: {precio * cantidad} euros")
```

> **Respuesta**
>
> `Total: 15 euros`. Se hace el cálculo entre llaves y luego se inserta en
> el texto.

**Pregunta 3:** tienes `puntos = 10` y quieres 5 puntos más. ¿Qué hay que
escribir?

> **Respuesta**
>
> Un nombre nuevo: `puntos2 = puntos + 5`. `puntos = puntos + 5` es el
> error E011, porque un nombre no cambia nunca.

## En resumen

- `nombre = valor` da un nombre a un valor, para siempre: un valor nuevo
  recibe un nombre nuevo.
- Los tipos de valores simples son `Int`, `Float`, `Text` y `Bool`;
  `type_of` da el tipo de un valor.
- `/` da siempre un número con decimales, `//` la división entera, `%` el
  resto, `**` la potencia.
- `"Hola {nombre}"` inserta un valor en un texto; texto y número nunca se
  mezclan con `+`.
- `print` (o `puts`) muestra, y `#` empieza un comentario.

Siguiente: [Números y cálculos](nombres.md)
