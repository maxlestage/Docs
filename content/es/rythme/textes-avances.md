# Textos avanzados: números, columnas e informes

Esta página muestra cómo escribir números legibles con `format_number`, alinear columnas con `pad_left` y `pad_right`, y después construir paso a paso un pequeño informe a partir de una lista de registros.

> **Objetivos:**
>
> - sabrás escribir un número para las personas: miles separados, decimales, coma a la
>   española;
> - sabrás elegir entre `format_number`, `text(x, 2)` y `round`;
> - sabrás rellenar un texto por la izquierda o por la derecha para alinear columnas;
> - sabrás calcular el ancho de una columna a partir de los datos;
> - sabrás construir un informe línea a línea, y después volver a unir las líneas con
>   `join`.

## El objetivo: un informe legible

Tienes una lista de ventas,
[registros](../debuter/glossaire.md#enregistrement) como
`{nombre: "Fresas", cant: 12, precio: 0.85}`. Mostrarlos tal cual es
correcto, pero difícil de leer. Al final de esta página, sabrás escribir
esto:

```text
Producto  Cant.     Importe
---------------------------
Fresas       12     10,20 €
Mangos        3      7,50 €
Limones      40     48,00 €
---------------------------
Total               65,70 €
```

Piensa en un tique de caja: cada cosa tiene su columna, los números están
alineados a la derecha, el total está abajo. Primero cada herramienta,
después el informe.

Lo básico de los textos (`split`, `join`, `pad_left`, `repeat`) está en
[trabajar con texto](texte.md). Esta página va más lejos. Para leer datos
de un archivo, mira [los archivos](fichiers.md).

## Escribir un número para las personas

`format_number(numero, decimales, idioma)` transforma un número en un
[texto](../debuter/glossaire.md#chaine-de-caracteres) fácil de leer:

- separa los miles;
- escribe **exactamente** el número de decimales pedido;
- sigue la forma de escribir de un idioma: `"es"` para el español.

```rythme
print(format_number(1234567.891, 2, "es"))
print(format_number(1234567.891, 2))
print(format_number(1234567))
```

```sortie
1.234.567,89
1,234,567.89
1,234,567
```

Sin idioma, es la escritura inglesa: una coma entre los miles, un punto
antes de los decimales. En español, es al revés: un punto entre los miles,
una coma antes de los decimales.

El resultado es un **texto**, no un número: se muestra, ya no se calcula
con él.

```rythme
bonito = format_number(1500, 2, "es")
print(bonito, type_of(bonito))
```

```sortie
1.500,00 Text
```

> **Recuerda:** calcula con números, y llama a `format_number` al final
> del todo, justo antes de mostrar.

### Los tres argumentos, uno a uno

| Llamada | Lo que da | Por qué |
|---------|-----------|---------|
| `format_number(7)` | `7` | no se piden decimales: el número tal cual |
| `format_number(7, 2)` | `7.00` | exactamente 2 decimales, ceros si hace falta |
| `format_number(7, 2, "es")` | `7,00` | la coma española |
| `format_number(2.5, 0)` | `3` | 0 decimales: se redondea |
| `format_number(12345, 0, "de")` | `12.345` | en alemán, también un punto entre los miles |

Los idiomas conocidos son `"en"` (por defecto), `"fr"`, `"de"`, `"es"`,
`"it"`, `"nl"` y `"pt"`.

Prueba: cambia `"es"` por `"fr"` en el primer ejemplo. Obtienes
`1 234 567,89`: espacios entre los miles, una coma antes de los decimales.

### Esconde los pequeños errores de los números decimales

Un [número decimal](../debuter/glossaire.md#flottant) no siempre es exacto
en un ordenador (mira [las trampas](pieges.md)). `format_number` redondea,
y el problema desaparece al mostrarlo:

```rythme
print(0.1 + 0.2)
print(format_number(0.1 + 0.2, 2, "es"))
```

```sortie
0.30000000000000004
0,30
```

Es como una báscula de cocina que muestra 300 g: no enseña los
miligramos, y así está muy bien.

## Redondear: tres herramientas parecidas

Tres funciones saben redondear. No dan lo mismo:

| Herramienta | Ejemplo | Resultado | Es... |
|-------------|---------|-----------|-------|
| `round(x, 2)` | `round(3.14159, 2)` | `3.14` | un **número** |
| `text(x, 2)` | `text(3.14159, 2)` | `"3.14"` | un texto, con un punto |
| `format_number(x, 2, "es")` | `format_number(3.14159, 2, "es")` | `"3,14"` | un texto, con miles y coma |

```rythme
x = 1234.5
print(round(x, 2), type_of(round(x, 2)))
print(text(x, 2), type_of(text(x, 2)))
print(format_number(x, 2, "es"), type_of(format_number(x, 2, "es")))
```

```sortie
1234.5 Float
1234.50 Text
1.234,50 Text
```

Mira la primera línea: `round` da un número, y un número no guarda los
ceros del final. Para **mostrar** dos decimales, hace falta un texto:
`text(x, 2)` o `format_number`.

¿Qué función elegir?

```text
 Quiero seguir calculando ................ round(x, 2)
 Quiero un texto simple (archivo, JSON) ... text(x, 2)
 Quiero mostrárselo a una persona ......... format_number(x, 2, "es")
```

> **Recuerda:** `round` da un número; `text` y `format_number` dan un
> texto, listo para mostrar.

### Los importes en céntimos

Para el dinero, una buena costumbre: contar en **céntimos**, con
[enteros](../debuter/glossaire.md#entier). Los enteros siempre son exactos.
Solo se divide entre 100 para mostrar:

```rythme
precios_centimos = [199, 250, 1005]
total = sum(precios_centimos)
print(total)
print(format_number(total / 100, 2, "es") + " €")
```

```sortie
1454
14,54 €
```

El entero `1454` es exacto; solo se convierte en un número decimal
(`14.54`), y después en un texto (`"14,54 €"`), en el último momento.

## Rellenar un texto para alinearlo

Una columna bien alineada son textos que tienen **todos el mismo ancho**.
Se completan con espacios.

- `pad_right(texto, ancho)` añade espacios **a la derecha**: el texto se
  queda pegado a la izquierda. Para las palabras.
- `pad_left(texto, ancho)` añade espacios **a la izquierda**: el texto se
  empuja a la derecha. Para los números.

Para **ver** los espacios, rellenemos con puntos (el tercer argumento elige
el carácter):

```rythme
print(pad_right("Fresas", 10, "."))
print(pad_left("12", 5, "."))
print(pad_left("12", 5) + "|")
```

```sortie
Fresas....
...12
   12|
```

Como una regla graduada: se parte de la longitud del texto, y se añade
justo lo necesario para llegar al ancho.

```text
pad_right("Fresas", 10)          pad_left("12", 5)
 F r e s a s · · · ·              · · · 1 2
 └─── 6 ───┘ └─ 4 ─┘              └─ 3 ─┘ └2┘
 └────── 10 ───────┘              └─── 5 ───┘
```

### Por qué los números van a la derecha

Alineadas a la derecha, las unidades quedan bajo las unidades y las decenas
bajo las decenas. Se comparan de un vistazo:

```rythme
numeros = [5, 120, 42]
numeros |> each(fn(n) = print(pad_right(n, 5) + "|" + pad_left(n, 5) + "|"))
```

```sortie
5    |    5|
120  |  120|
42   |   42|
```

A la izquierda, `pad_right`: las cifras no están unas debajo de otras. A
la derecha, `pad_left`: la columna se lee como una suma hecha en el
colegio.

Prueba: cambia `5` por `2` en `pad_left(n, 5)`. El número `120` ya tiene 3
caracteres: se sale, y la columna se descoloca. La siguiente sección
resuelve este problema.

> **Recuerda:** las palabras a la izquierda con `pad_right`, los números a
> la derecha con `pad_left`.

### Un texto demasiado largo no se corta

Si el texto ya es más largo que el ancho, `pad_left` y `pad_right` lo
devuelven **tal cual**. Nunca cortan:

```rythme
print("[" + pad_right("Melocotones", 8) + "]")
print("[" + take("Melocotones", 8) + "]")
```

```sortie
[Melocotones]
[Melocoto]
```

Para cortar, usa `take(texto, n)`: guarda los `n` primeros caracteres. Una
pequeña función hace las dos cosas, cortar y después rellenar:

```rythme
fn ajustar(t, ancho) = pad_right(take(t, ancho), ancho)

["Kiwi", "Melocotones"] |> each(fn(p) = print("[" + ajustar(p, 8) + "]"))
```

```sortie
[Kiwi    ]
[Melocoto]
```

## Elegir el ancho de una columna

En lugar de adivinar el ancho, calculémoslo: es la longitud del texto
**más largo** de la columna.

```rythme
nombres = ["Fresas", "Mangos", "Limones"]
ancho = nombres |> map(len) |> max
print(ancho)
nombres |> each(fn(n) = print(pad_right(n, ancho) + " |"))
```

```sortie
7
Fresas  |
Mangos  |
Limones |
```

Sigamos el cálculo de `ancho`, un paso cada vez:

| Paso | Valor |
|------|-------|
| `nombres` | `["Fresas", "Mangos", "Limones"]` |
| `\|> map(len)` | `[6, 6, 7]` |
| `\|> max` | `7` |

Prueba: añade `"Mandarinas"` a la lista `nombres`. El ancho pasa a 10, y
las barras `|` siguen alineadas.

> **Recuerda:** el ancho de una columna = la longitud de su texto más
> largo: `textos |> map(len) |> max`.

## Construir un informe paso a paso

Vamos con el informe del principio. Seguimos una receta en cinco pasos,
como al montar un mueble: una pieza cada vez, y se comprueba en cada paso.

```text
 1. una función para UNA línea
 2. map: una línea para cada venta
 3. el total
 4. la cabecera, las rayas y el pie
 5. join: volver a unirlo todo con "\n", y un solo print
```

### Paso 1: una sola línea

Primero escribimos una [función](../debuter/glossaire.md#fonction) que
transforma **una** venta en **una** línea de texto. Y una pequeña función
`euros` para los importes:

```rythme
fn euros(x) = format_number(x, 2, "es") + " €"

fn fila(v) =
  pad_right(v.nombre, 10) + pad_left(v.cant, 5) + pad_left(euros(v.cant * v.precio), 12)

print(fila({nombre: "Fresas", cant: 12, precio: 0.85}))
print(fila({nombre: "Mangos", cant: 3, precio: 2.5}))
```

```sortie
Fresas       12     10,20 €
Mangos        3      7,50 €
```

Descompongamos la primera línea, trozo a trozo:

| Trozo | Cálculo | Texto obtenido | Ancho |
|-------|---------|----------------|-------|
| nombre | `pad_right("Fresas", 10)` | `"Fresas    "` | 10 |
| cantidad | `pad_left(12, 5)` | `"   12"` | 5 |
| importe | `12 * 0.85` → `10.2` → `euros(10.2)` | `"10,20 €"` | 7 |
| importe alineado | `pad_left("10,20 €", 12)` | `"     10,20 €"` | 12 |
| la línea | los tres unidos con `+` | `"Fresas       12     10,20 €"` | 27 |

`pad_left` también acepta un número, como `12`: primero lo transforma en
texto.

### Paso 2: una línea para cada venta

No hay bucles en Rythme: `map` aplica `fila` a cada venta y da la lista de
las líneas.

```rythme
fn euros(x) = format_number(x, 2, "es") + " €"
fn fila(v) =
  pad_right(v.nombre, 10) + pad_left(v.cant, 5) + pad_left(euros(v.cant * v.precio), 12)

ventas = [
  {nombre: "Fresas", cant: 12, precio: 0.85},
  {nombre: "Mangos", cant: 3, precio: 2.5},
  {nombre: "Limones", cant: 40, precio: 1.2},
]
cuerpo = ventas |> map(fila)
print(len(cuerpo))
print(cuerpo[2])
```

```sortie
3
Limones      40     48,00 €
```

| Vuelta de `map` | `v.nombre` | `v.cant * v.precio` | Línea fabricada |
|-----------------|------------|---------------------|-----------------|
| 1 | `"Fresas"` | `10.2` | `Fresas       12     10,20 €` |
| 2 | `"Mangos"` | `7.5` | `Mangos        3      7,50 €` |
| 3 | `"Limones"` | `48.0` | `Limones      40     48,00 €` |

### Paso 3: el total

`sum` con una función suma lo que la función da para cada venta:
`total = ventas |> sum(fn(v) = v.cant * v.precio)`. El programa completo,
en el paso siguiente, lo calcula así:

| Venta | Importe | Total hasta aquí |
|-------|---------|------------------|
| (inicio) | | `0` |
| Fresas | `10.2` | `10.2` |
| Mangos | `7.5` | `17.7` |
| Limones | `48.0` | `65.7` |

### Pasos 4 y 5: montar y volver a unir

Fabricamos la cabecera, una raya y el pie con los **mismos anchos** (10, 5
y 12). Después ponemos todas las líneas en una sola lista, y
`join(lista, "\n")` las vuelve a unir con un salto de línea entre cada una.

```rythme
fn euros(x) = format_number(x, 2, "es") + " €"
fn fila(v) =
  pad_right(v.nombre, 10) + pad_left(v.cant, 5) + pad_left(euros(v.cant * v.precio), 12)

ventas = [
  {nombre: "Fresas", cant: 12, precio: 0.85},
  {nombre: "Mangos", cant: 3, precio: 2.5},
  {nombre: "Limones", cant: 40, precio: 1.2},
]
cuerpo = ventas |> map(fila)
total = ventas |> sum(fn(v) = v.cant * v.precio)

cabecera = pad_right("Producto", 10) + pad_left("Cant.", 5) + pad_left("Importe", 12)
raya = repeat("-", 27)
pie = pad_right("Total", 15) + pad_left(euros(total), 12)

informe = [cabecera, raya, ..cuerpo, raya, pie] |> join("\n")
print(informe)
```

```sortie
Producto  Cant.     Importe
---------------------------
Fresas       12     10,20 €
Mangos        3      7,50 €
Limones      40     48,00 €
---------------------------
Total               65,70 €
```

`..cuerpo` pone las líneas de `cuerpo` **en ese lugar** de la lista, una a
una. Sin los dos puntos, `cuerpo` sería una lista dentro de la lista.

```text
[ cabecera, raya, ..cuerpo,                     raya, pie ]
                   │
                   ▼
[ cabecera, raya, linea1, linea2, linea3,       raya, pie ]
                   │
          join("\n")
                   ▼
"cabecera\nraya\nlinea1\nlinea2\nlinea3\nraya\npie"
```

La raya mide 27 caracteres: 10 + 5 + 12, la suma de los anchos. El pie usa
15 para la palabra «Total»: ocupa el sitio del nombre **y** de la cantidad
(10 + 5).

Prueba: añade una cuarta venta, `{nombre: "Kiwis", cant: 7, precio: 0.4}`.
Aparece una línea más, y el total pasa a ser `68,50 €`. No tienes que
cambiar nada más.

> **Recuerda:** una función para una línea, `map` para todas las líneas,
> `join("\n")` para hacer con ellas un solo texto.

## Volver a unir líneas y palabras

`join` es lo contrario de `split` y de `lines`. El separador elige lo que
va **entre** los trozos, nunca al principio ni al final:

```rythme
trozos = ["a", "b", "c"]
print(join(trozos, ", "))
print(join(trozos, ""))
print(join(trozos, "\n"))
```

```sortie
a, b, c
abc
a
b
c
```

`join` también acepta una lista de números: `join([1, 2, 3], " + ")` da
`"1 + 2 + 3"`.

### Una lista en español: «a, b y c»

En español, se escribe «fresas, kiwis y piñas»: comas, salvo antes de la
última palabra. Separamos el problema en tres casos, con una
[definición por casos](fonctions.md#definir-por-casos):

```rythme
fn lista_humana([]) = ""
fn lista_humana([sola]) = sola
fn lista_humana(palabras) =
  join(take(palabras, len(palabras) - 1), ", ") + " y " + unwrap(last(palabras))

print(lista_humana(["fresas", "kiwis", "piñas"]))
print(lista_humana(["fresas", "kiwis"]))
print(lista_humana(["fresas"]))
```

```sortie
fresas, kiwis y piñas
fresas y kiwis
fresas
```

Para `["fresas", "kiwis", "piñas"]`:

| Paso | Cálculo | Valor |
|------|---------|-------|
| 1 | `take(palabras, 2)` | `["fresas", "kiwis"]` |
| 2 | `join(..., ", ")` | `"fresas, kiwis"` |
| 3 | `unwrap(last(palabras))` | `"piñas"` |
| 4 | `"fresas, kiwis" + " y " + "piñas"` | `"fresas, kiwis y piñas"` |

`last` da una [Option](../debuter/glossaire.md#option) (la lista podría
estar vacía); aquí sabemos que no lo está, así que `unwrap` la abre.

## Errores frecuentes

### Darle un texto a format_number

Los trozos de un `split` son **textos**, aunque contengan cifras:

```rythme norun
print(format_number("1234"))
```

```text
error[E005]: `format_number` espera un número como argumento 1, pero recibió Text "1234"
 --> main.ry:1:21
  |
1 | print(format_number("1234"))
  |                     ^^^^^^ tipo de valor incorrecto
  |
  = ayuda: uso: `format_number(x, decimals, language)`
  = más sobre este error: `rythme explain E005`
```

Primero lee el número: `parse_float("1234") |> unwrap_or(0.0)`, y después
dáselo a `format_number`.

### Rellenar con varios caracteres

```rythme norun
print(pad_left(7, 3, "00"))
```

```text
error[E023]: `pad_left` necesita un solo carácter, pero "00" tiene 2
 --> main.ry:1:7
  |
1 | print(pad_left(7, 3, "00"))
  |       ^^^^^^^^^^^^^^^^^^^^
  |
  = más sobre este error: `rythme explain E023`
```

El relleno es **un** carácter, repetido tantas veces como haga falta:
`pad_left(7, 3, "0")` da `"007"`.

### El mayor ancho de una lista vacía

```rythme norun
nombres = []
ancho = nombres |> map(len) |> max
print(ancho)
```

```text
error[E023]: `max` de una lista vacía
 --> main.ry:2:9
  |
2 | ancho = nombres |> map(len) |> max
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = más sobre este error: `rythme explain E023`
```

Una lista vacía no tiene elemento mayor. Añade un valor de partida, por
ejemplo la longitud del título de la columna:
`[len("Producto"), ..(nombres |> map(len))] |> max` vale 8 aunque
`nombres` esté vacía. De todas formas, el título tiene que caber en la
columna.

## Ponte a prueba

**Pregunta 1:** ¿qué da `format_number(9876.5, 2, "es")`? ¿Y es un
número?

> **Respuesta**
>
> El texto `"9.876,50"`: un punto entre los miles, una coma, y exactamente
> dos decimales. Es un **texto** (`Text`): se muestra, ya no se calcula con
> él.

**Pregunta 2:** ¿qué dan `pad_left("abc", 6, "*")` y
`pad_right("abc", 2)`?

> **Respuesta**
>
> `"***abc"`: tres estrellas a la izquierda para llegar a 6 caracteres. Y
> `"abc"`: el texto ya es más largo que 2, se devuelve tal cual, sin
> cortarlo.

**Pregunta 3:** tienes una lista `filas` de textos. ¿Cómo la muestras con
una línea por elemento, en un solo `print`?

> **Respuesta**
>
> `print(join(filas, "\n"))`: `join` pone un salto de línea **entre** cada
> elemento, y después `print` lo muestra todo.

## En resumen

- `format_number(x, decimales, "es")` escribe un número para las personas:
  `1.234,50`. Da un texto: llámalo justo antes de mostrar.
- `round` da un número; `text(x, 2)` y `format_number` dan un texto. Para el
  dinero, cuenta en céntimos y divide entre 100 al final.
- `pad_right` alinea las palabras a la izquierda, `pad_left` alinea los
  números a la derecha; nunca cortan: `take` corta.
- El ancho de una columna es la longitud de su texto más largo:
  `map(len) |> max`.
- Un informe: una función para una línea, `map` para todas, y después
  `join("\n")` y un solo `print`.

Siguiente: [las expresiones regulares](regex.md).
