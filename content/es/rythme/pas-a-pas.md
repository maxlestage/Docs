# Seguir un programa paso a paso

Esta página te enseña a ejecutar un programa Rythme «en tu cabeza», con una tabla que sigue los valores en cada paso, sobre seis pequeños programas cada vez más completos.

> **Objetivos:**
>
> - sabrás rellenar una tabla de seguimiento, una fila por paso;
> - sabrás seguir una condición, un `fold`, una llamada a función y una función recursiva;
> - sabrás adivinar lo que muestra un programa **antes** de ejecutarlo;
> - sabrás hacer que un programa muestre sus pasos, con `print`, `debug`, `scan` y `rythme steps`.

## Por qué seguir un programa a mano

Un ordenador sigue las instrucciones una por una, sin saltarse nunca una
línea. Para entender un programa, haz lo mismo: juega a ser el ordenador.

Es como seguir una receta de cocina con el dedo: lees una línea, haces lo
que dice, anotas por dónde vas y pasas a la siguiente.

Tu herramienta es una **tabla de seguimiento**: una columna por nombre, una
fila por paso. Necesitas una hoja y un lápiz.

```text
 ┌───────┬──────────────────────┬──────────┬──────────┐
 │ Línea │ Qué pasa             │ nombre 1 │ nombre 2 │
 ├───────┼──────────────────────┼──────────┼──────────┤
 │   1   │ ...                  │   ...    │          │
 │   2   │ ...                  │   ...    │   ...    │
 └───────┴──────────────────────┴──────────┴──────────┘
```

Cada ejemplo de esta página sigue el mismo método:

1. lee el código;
2. **antes de ejecutarlo, adivina lo que se muestra**, y anótalo;
3. rellena la tabla de seguimiento;
4. ejecuta el programa y compara.

> **Recuerda:** equivocarte al adivinar no es grave. Justamente ahí es
> donde se aprende: la tabla muestra dónde se separan tu idea y el
> ordenador.

## Una particularidad de Rythme: los valores nunca cambian

En muchos lenguajes, una [variable](../debuter/glossaire.md#variable)
es una caja cuyo contenido se cambia. En Rythme, un
[valor](../debuter/glossaire.md#valor) es
[inmutable](../debuter/glossaire.md#inmutable): un nombre, una vez dado,
conserva su valor hasta el final (mira [las trampas](pieges.md)).

Buena noticia para el seguimiento: en tu tabla, **cada casilla se rellena
una sola vez**. Nunca se borra nada. La tabla baja en escalera.

Y como no hay bucles, las repeticiones se hacen con `map`, `fold` o una
función recursiva. Se siguen con una fila por **vuelta**.

## Programa 1: nombres uno tras otro

```rythme
precio = 8
cantidad = 3
total = precio * cantidad
descuento = total // 4
a_pagar = total - descuento
print("A pagar: {a_pagar} €")
```

**Antes de ejecutarlo, adivina lo que se muestra.** Recuerda: `//` es la
división entera, solo conserva la parte entera.

La tabla de seguimiento, línea por línea:

| Línea | Qué pasa | precio | cantidad | total | descuento | a_pagar |
|-------|----------|--------|----------|-------|-----------|---------|
| 1 | `precio` recibe 8 | 8 | | | | |
| 2 | `cantidad` recibe 3 | 8 | 3 | | | |
| 3 | `8 * 3` | 8 | 3 | 24 | | |
| 4 | `24 // 4` | 8 | 3 | 24 | 6 | |
| 5 | `24 - 6` | 8 | 3 | 24 | 6 | 18 |
| 6 | muestra el texto | 8 | 3 | 24 | 6 | 18 |

```sortie
A pagar: 18 €
```

En la línea 6, Rythme sustituye `{a_pagar}` por el valor de la columna
`a_pagar`: 18. Mira la escalera: cada fila añade **una** casilla, y las
casillas ya rellenas no se mueven.

Prueba: pon `cantidad = 5`. Rellena primero la tabla: `total` vale 40,
`descuento` 10, y el programa muestra `A pagar: 30 €`.

> **Recuerda:** una línea del programa, una fila de la tabla. Para
> calcular, busca el valor de un nombre en **su columna**, más arriba.

## Programa 2: una condición

```rythme
temperatura = 18
lluvia = true
ropa = if temperatura >= 25 then "un pantalón corto"
  else if temperatura >= 15 then "un jersey"
  else "un abrigo"
accesorio = if lluvia then " y un paraguas" else ""
print("Hoy: {ropa}{accesorio}")
```

**Antes de ejecutarlo, adivina lo que se muestra.**

Para una [condición](../debuter/glossaire.md#condicion), añade dos
columnas: la **pregunta** que se hace, y la **respuesta**, `true` o `false`.

| Línea | Pregunta | Respuesta | Qué pasa | ropa | accesorio |
|-------|----------|-----------|----------|------|-----------|
| 1-2 | | | `temperatura` = 18, `lluvia` = true | | |
| 3 | ¿`18 >= 25`? | `false` | se pasa al `else` | | |
| 4 | ¿`18 >= 15`? | `true` | se toma `"un jersey"` | `"un jersey"` | |
| 6 | ¿`lluvia`? | `true` | se toma el texto del `then` | `"un jersey"` | `" y un paraguas"` |
| 7 | | | se pegan los dos textos | `"un jersey"` | `" y un paraguas"` |

```sortie
Hoy: un jersey y un paraguas
```

Las preguntas se hacen **de arriba abajo**, y la primera respuesta `true`
gana: la línea 5 (`"un abrigo"`) nunca se lee.

```text
 ¿18 >= 25? ── no ──► ¿18 >= 15? ── sí ──► "un jersey"   (se para ahí)
     │
     sí
     ▼
 "un pantalón corto"
```

Prueba: pon `temperatura = 30` y `lluvia = false`. El programa muestra
`Hoy: un pantalón corto`: la primera pregunta responde `true`, y
`accesorio` vale el texto vacío `""`.

> **Recuerda:** para una condición, escribe la pregunta y su respuesta en
> la tabla. Solo se calcula la rama elegida.

## Programa 3: sumar con fold

En Rythme, un «bucle que suma» se escribe con
[fold](fonctionnel.md). `fold` lleva un valor de una vuelta a la
siguiente, el **acumulador** (`acc`), como una cesta que llenas mientras
avanzas por una tienda.

```rythme
compras = [4, 10, 3, 7]
total = fold(compras, 0, fn(acc, x) = acc + x)
print("Total: {total}")
```

**Antes de ejecutarlo, adivina lo que se muestra.**

`fold` empieza en `0` y luego llama a la pequeña función una vez **por
elemento**. Cada vuelta recibe el acumulador de la vuelta anterior:

| Vuelta | acc (recibido) | x | acc + x (dado a la vuelta siguiente) |
|--------|----------------|---|--------------------------------------|
| inicio | | | 0 |
| 1 | 0 | 4 | 4 |
| 2 | 4 | 10 | 14 |
| 3 | 14 | 3 | 17 |
| 4 | 17 | 7 | 24 |
| fin | | | `total` = 24 |

```sortie
Total: 24
```

El secreto de una buena tabla para `fold`: la columna **acc (recibido)** de
una vuelta es la última columna de la vuelta **anterior**. Sigue la flecha:

```text
 0 ──(+4)──► 4 ──(+10)──► 14 ──(+3)──► 17 ──(+7)──► 24
```

Rythme puede comprobar tu tabla. `scan` hace el mismo cálculo que `fold`,
pero da **todos** los resultados intermedios, es decir, tu última
columna:

```rythme
compras = [4, 10, 3, 7]
print(scan(compras, 0, fn(acc, x) = acc + x))
```

```sortie
[4, 14, 17, 24]
```

Prueba: cambia `0` por `100` en el `fold`, como un monedero que ya tiene
100 €. La tabla empieza en 100, y el programa muestra `Total: 124`.

> **Recuerda:** para `fold`, una fila por elemento; el acumulador recibido
> es siempre el resultado de la fila de arriba.

## Programa 4: una función llamada dos veces

```rythme
fn con_iva(neto) = neto + neto * 20 // 100

libro = con_iva(15)
boli = con_iva(5)
print(libro, boli, libro + boli)
```

**Antes de ejecutarlo, adivina lo que se muestra.** `con_iva` añade un 20 %
a un precio.

Una [función](../debuter/glossaire.md#funcion) es una pequeña máquina:
le metes un valor y sale otro. En cada llamada, su
[parámetro](../debuter/glossaire.md#parametro) `neto` toma el valor dado,
solo mientras dura la llamada.

```text
 programa                              función con_iva
 ────────                              ───────────────
 libro = con_iva(15)  ── neto = 15 ──►   15 + 15 * 20 // 100
                                         = 15 + 300 // 100
                                         = 15 + 3
                      ◄── 18 ─────────   = 18
 boli = con_iva(5)    ── neto = 5 ───►   5 + 5 * 20 // 100
                                         = 5 + 100 // 100
                                         = 5 + 1
                      ◄── 6 ──────────   = 6
```

La tabla de seguimiento tiene una columna para `neto`. Solo tiene valor
**durante** una llamada:

| Paso | Quién trabaja | neto | Cálculo | libro | boli |
|------|---------------|------|---------|-------|------|
| 1 | el programa llama a `con_iva(15)` | 15 | | | |
| 2 | `con_iva` | 15 | `15 + 300 // 100` = 18 | | |
| 3 | retorno: `libro` recibe 18 | | | 18 | |
| 4 | el programa llama a `con_iva(5)` | 5 | | 18 | |
| 5 | `con_iva` | 5 | `5 + 100 // 100` = 6 | 18 | |
| 6 | retorno: `boli` recibe 6 | | | 18 | 6 |
| 7 | `print`, con `18 + 6` = 24 | | | 18 | 6 |

```sortie
18 6 24
```

Las dos llamadas son independientes: el `neto` de la primera llamada se
olvida antes de la segunda. Es como una máquina de café: cada taza se
prepara desde cero.

Prueba: añade `cuaderno = con_iva(con_iva(10))`. Sigue primero la llamada
de **dentro**: `con_iva(10)` da 12, luego `con_iva(12)` da
`12 + 240 // 100`, o sea 14.

> **Recuerda:** en cada llamada, anota el valor del parámetro, calcula, y
> luego lleva el resultado al sitio donde estaba escrita la llamada.

## Programa 5: una lista, con una condición dentro

Contamos los alumnos aprobados (nota de 10 o más):

```rythme
notas = [12, 7, 15, 9, 10]

fn contar_si_aprueba(cuenta, nota) =
  if nota >= 10 then cuenta + 1 else cuenta

aprobados = fold(notas, 0, contar_si_aprueba)
print("{aprobados} aprobados de {len(notas)}")
```

**Antes de ejecutarlo, adivina lo que se muestra.** Cuidado con la nota 10.

Es un `fold` (programa 3) con una condición (programa 2) en cada vuelta.
La tabla junta las dos cosas:

| Vuelta | cuenta (recibida) | nota | ¿`nota >= 10`? | cuenta (dada) |
|--------|-------------------|------|----------------|---------------|
| inicio | | | | 0 |
| 1 | 0 | 12 | `true` | 1 |
| 2 | 1 | 7 | `false` | 1 |
| 3 | 1 | 15 | `true` | 2 |
| 4 | 2 | 9 | `false` | 2 |
| 5 | 2 | 10 | `true` | 3 |

```sortie
3 aprobados de 5
```

En la vuelta 5, `10 >= 10` es verdadero: `>=` quiere decir «mayor **o
igual**». Con `>`, el alumno que tiene 10 no se contaría.

Cuando hayas entendido la tabla, aquí tienes la versión corta. `count` hace
el mismo trabajo en una línea:

```rythme
notas = [12, 7, 15, 9, 10]
print(count(notas, fn(n) = n >= 10))
```

```sortie
3
```

Prueba: cambia `>=` por `>` en `contar_si_aprueba`. Solo cambia la fila de
la vuelta 5 en la tabla (`false`, la cuenta se queda en 2), y el programa
muestra `2 aprobados de 5`.

> **Recuerda:** una condición dentro de un `fold` añade una columna
> «¿pregunta?» en cada vuelta. Cuando la respuesta es `false`, el
> acumulador pasa tal cual.

## Programa 6: una función recursiva

Una función [recursiva](../debuter/glossaire.md#recursion) se llama a sí
misma, sobre una lista más corta, hasta llegar a la lista vacía (mira
[la recursión](recursion.md)). Aquí tienes una suma hecha a mano:

```rythme
fn sumar([], total) = total
fn sumar([x, ..resto], total) = sumar(resto, total + x)

print(sumar([4, 10, 3, 7], 0))
```

**Antes de ejecutarlo, adivina lo que se muestra.**

Para cada llamada, anota **qué definición** se elige, y los valores de los
parámetros:

| Llamada | Lista recibida | total recibido | Definición elegida | x | resto | Llamada siguiente |
|---------|----------------|----------------|--------------------|---|-------|-------------------|
| 1 | `[4, 10, 3, 7]` | 0 | `[x, ..resto]` | 4 | `[10, 3, 7]` | `sumar([10, 3, 7], 4)` |
| 2 | `[10, 3, 7]` | 4 | `[x, ..resto]` | 10 | `[3, 7]` | `sumar([3, 7], 14)` |
| 3 | `[3, 7]` | 14 | `[x, ..resto]` | 3 | `[7]` | `sumar([7], 17)` |
| 4 | `[7]` | 17 | `[x, ..resto]` | 7 | `[]` | `sumar([], 24)` |
| 5 | `[]` | 24 | `[]`: se para | | | resultado: 24 |

```sortie
24
```

Compara con el programa 3: la columna `total` es exactamente la columna
`acc` del `fold`. Un `fold` es esta recursión ya escrita para ti.

```text
 sumar([4, 10, 3, 7], 0)
   └─► sumar([10, 3, 7], 4)
         └─► sumar([3, 7], 14)
               └─► sumar([7], 17)
                     └─► sumar([], 24)  ──►  24
```

Prueba: llama a `sumar([], 0)`. La tabla solo tiene una fila: se elige
enseguida la primera definición, y el programa muestra `0`.

> **Recuerda:** para una recursión, una fila por llamada. Anota qué
> definición se elige: es ella la que decide si se sigue.

## Hacer que el programa muestre sus pasos

¿Tu tabla y el programa no están de acuerdo? Pide al programa que muestre
sus pasos.

### print, con etiquetas

Añade un `print` **con una etiqueta**: el nombre de lo que muestras. Un
número solo, en medio de otros, no dice nada. Dentro de un `fold`, un
bloque `do ... end` permite mostrar y luego dar el nuevo acumulador (la
última línea del bloque):

```rythme
notas = [12, 7, 15]
aprobados = fold(notas, 0, fn(cuenta, nota) = do
  nueva = if nota >= 10 then cuenta + 1 else cuenta
  print("cuenta = {cuenta}, nota = {nota} -> {nueva}")
  nueva
end)
print("aprobados = {aprobados}")
```

```sortie
cuenta = 0, nota = 12 -> 1
cuenta = 1, nota = 7 -> 1
cuenta = 1, nota = 15 -> 2
aprobados = 2
```

Cada línea mostrada es una fila de tu tabla. Compáralas una por una: la
primera que difiere muestra tu error.

> **Consejo:** cuando lo hayas encontrado, quita esos `print`. Sirven para
> buscar, no para quedarse.

### debug: mostrar sin cambiar nada

`debug(valor)` muestra un valor **y lo devuelve tal cual**. Así que puedes
añadirlo en medio de un cálculo sin romperlo:

```rythme
precio = 8
total = debug(precio * 3) - 6
print("total = {total}")
```

```sortie
total = 18
```

El programa también muestra, aparte, la posición del `debug` en el archivo
(línea 2, columna 9) y el valor:

```text
[debug main.ry:2:9] 24
```

Este mensaje va a la **salida de error**, un canal separado de la salida
normal: no se mezcla con los resultados de tu programa.

### scan: todas las vueltas de un fold

Lo viste en el programa 3: cambia `fold` por `scan`, y obtienes la última
columna de tu tabla, de una vez.

```rythme
notas = [12, 7, 15, 9, 10]
print(scan(notas, 0, fn(c, n) = if n >= 10 then c + 1 else c))
```

```sortie
[1, 1, 2, 2, 3]
```

### rythme steps: el cálculo completo

`rythme steps archivo.ry` muestra cada paso de cada línea, como tu tabla.
Con el programa 1 guardado en `compras.ry`:

```text
$ rythme steps compras.ry
precio = 8

cantidad = 3

total = precio * cantidad
  → 8 * cantidad
  → 8 * 3
  → 24

descuento = total // 4
  → 24 // 4
  → 6

a_pagar = total - descuento
  → 24 - descuento
  → 24 - 6
  → 18

print("A pagar: {a_pagar} €")
  → print("A pagar: 18 €")
A pagar: 18 €
  → ()
```

Cada `→` es un paso: un nombre sustituido por su valor, o una operación
hecha. Para una sola expresión, dala entre comillas. Con el programa 6 en
`suma.ry`:

```text
$ rythme steps suma.ry "sumar([4, 10, 3, 7], 0)"
sumar([4, 10, 3, 7], 0)
  → sumar([10, 3, 7], 4)
  → sumar([3, 7], 14)
  → sumar([7], 17)
  → sumar([], 24)
  → 24
```

Es exactamente la columna «Llamada siguiente» de tu tabla.

> **Recuerda:** `print` con etiquetas para unos pocos valores, `debug` en
> medio de un cálculo, `scan` para un `fold`, `rythme steps` para verlo
> todo.

### ¿Y un depurador?

Rythme no tiene un depurador que pare el programa línea por línea: como
los valores nunca cambian, `rythme steps` (o el botón «Steps» del
playground, o `:steps` en modo interactivo) muestra todo el cálculo en su
lugar. El capítulo [entender un cálculo paso a paso](etapes.md) explica
todas sus opciones, y [depurar](../debuter/deboguer.md) da un método para
buscar un error.

## Errores frecuentes

Estos errores se cometen **al rellenar la tabla**. Los programas de abajo
muestran lo que pasa cuando se escribe el código con la misma idea
equivocada.

### Creer que un nombre cambia en cada vuelta

Piensas «en cada vuelta, `total` aumenta», como una caja que se llena:

```rythme norun
compras = [4, 10, 3]
total = 0
compras.each do |x|
  total = total + x
end
print(total)
```

```text
advertencia[W001]: `total` nunca se usa
 --> main.ry:4:3
  |
4 |   total = total + x
  |   ^^^^^
  |
  = ayuda: quítalo, o llámalo `_total` si es a propósito

0
```

El programa muestra `0`. Dentro del bloque, `total = total + x` crea un
**nuevo** `total`, que desaparece al final del bloque. El `total` de fuera
sigue valiendo 0. En la tabla, la casilla `total` de la línea 2 nunca
cambia. Para llevar un valor de una vuelta a otra, usa `fold`:
`total = fold(compras, 0, fn(acc, x) = acc + x)`.

### Volver al valor inicial en vez del último

En un `fold`, es frecuente equivocarse de columna: se vuelve a empezar con
el valor inicial en vez del de la vuelta anterior.

```rythme
compras = [4, 10, 3, 7]
print(fold(compras, 0, fn(acc, x) = 0 + x))
```

```sortie
7
```

Queríamos 24. Aquí, cada vuelta ignora `acc` y vuelve a empezar desde `0`:
el resultado es el último elemento. La tabla lo muestra enseguida: la
columna «acc (recibido)» nunca se usa. Escribe `acc + x`.

### Olvidar una vuelta, o contar una de más

`1..5` contiene **5** números: 1 y 5 incluidos. Pero `slice(lista, 1, 3)`
se para **antes** de la posición 3, y las posiciones empiezan en 0.

```rythme
print(len(1..5), 1..5)
print(slice([10, 20, 30, 40], 1, 3))
```

```sortie
5 [1, 2, 3, 4, 5]
[20, 30]
```

Antes de rellenar la tabla, escribe la lista entera y numera sus
posiciones empezando en 0. Cuenta las filas al final: tiene que haber una
por elemento.

### Confundir lo que se muestra con lo que se devuelve

`print` **muestra** un texto, pero lo que **devuelve** es `()`, el valor
«nada interesante».

```rythme
x = print("hola")
print(x)
```

```sortie
hola
()
```

En la tabla, mantén separadas dos cosas: la columna del valor, y lo que
aparece en la pantalla.

## Ponte a prueba

**Pregunta 1:** rellena la tabla de este programa. ¿Qué muestra?

```rythme norun
a = 5
b = a * 2
c = b - a
print(a + b + c)
```

> **Respuesta**
>
> | Línea | a | b | c |
> |-------|---|---|---|
> | 1 | 5 | | |
> | 2 | 5 | 10 | |
> | 3 | 5 | 10 | 5 |
>
> Muestra `20`: 5 + 10 + 5.

**Pregunta 2:** ¿cuánto vale `fold([2, 3, 4], 1, fn(acc, x) = acc * x)`?
Haz la tabla de las vueltas.

> **Respuesta**
>
> | Vuelta | acc (recibido) | x | acc * x |
> |--------|----------------|---|---------|
> | 1 | 1 | 2 | 2 |
> | 2 | 2 | 3 | 6 |
> | 3 | 6 | 4 | 24 |
>
> El resultado es `24`. Se empieza en 1, no en 0: con 0, todo se
> multiplicaría por cero.

**Pregunta 3:** con `fn f(n) = if n > 3 then n - 3 else n + 3`, ¿cuánto
vale `f(f(2))`?

> **Respuesta**
>
> Se empieza por la llamada de dentro. `f(2)`: la pregunta `2 > 3`
> responde `false`, así que 2 + 3 = 5. Luego `f(5)`: `5 > 3` responde
> `true`, así que 5 - 3 = 2. El resultado es `2`.

## En resumen

- Para entender un programa, juega a ser el ordenador: una línea de
  código, una fila de la tabla de seguimiento.
- Adivina la salida **antes** de ejecutar, y luego compara: la diferencia
  muestra lo que te queda por entender.
- En Rythme, cada casilla se rellena una sola vez: los valores nunca
  cambian.
- Para `fold` y la recursión, una fila por vuelta o por llamada; el valor
  recibido es el de la fila de arriba.
- `print` con etiquetas, `debug`, `scan` y `rythme steps` hacen que el
  programa muestre sus pasos.

Siguiente: [las trampas del principiante en Rythme](pieges.md).
