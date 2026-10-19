# Listas, registros y diccionarios

Esta página presenta las formas de agrupar valores: las listas, los intervalos, los registros, los diccionarios y los conjuntos.

> **Objetivos:**
>
> - sabrás crear una lista, leer un elemento por su posición y «añadir» un elemento;
> - sabrás usar las funciones de listas más comunes: `sort`, `sum`, `filter`, `map`...;
> - sabrás describir una ficha con un registro, y fabricar una versión modificada;
> - sabrás encontrar un valor por su clave en un diccionario;
> - sabrás elegir la colección adecuada para tus datos.

## Las listas

Una [lista](../debuter/glossaire.md#lista) es una serie de valores, en un
orden preciso, como una lista de la compra. Se escribe entre corchetes, con
comas entre los elementos.

Cada elemento tiene una **posición**, su
[índice](../debuter/glossaire.md#indice). Las posiciones empiezan en 0, y
`-1` indica el último elemento:

```rythme
frutas = ["manzana", "pera", "higo"]
print(frutas[0], frutas[-1], len(frutas))
```

```sortie
manzana higo 3
```

`frutas[0]` es la primera fruta, `frutas[2]` la tercera. `len` da el número
de elementos.

## Añadir un elemento sin modificar nada

Una lista nunca cambia. Para «añadir» un elemento, se fabrica una **nueva**
lista, y la antigua se queda tal cual:

```rythme
frutas = ["manzana", "pera"]
mas = push(frutas, "kiwi")
print(frutas)
print(mas)
print(frutas + ["melón"], [..frutas, "ciruela"], ["cereza", ..frutas])
```

```sortie
["manzana", "pera"]
["manzana", "pera", "kiwi"]
["manzana", "pera", "melón"] ["manzana", "pera", "ciruela"] ["cereza", "manzana", "pera"]
```

- `push(lista, x)` da la lista con `x` al final;
- `+` pone dos listas una detrás de otra;
- `..frutas` entre corchetes inserta todos los elementos de `frutas`, al
  principio, en medio o al final.

Es como fotocopiar la lista de la compra y escribir en la copia: el original
no se mueve.

## Leer sin riesgo con get

`frutas[10]` detiene el programa: el elemento no existe. `get` y `first` dan
en cambio una `Option`: `Some(valor)` si hay un valor, `None` si no.

```rythme
frutas = ["manzana", "pera"]
print(get(frutas, 1), get(frutas, 10), first(frutas), first([]))
```

```sortie
Some("pera") None Some("manzana") None
```

Para obtener el valor, da un valor por defecto con `unwrap_or`:

```rythme
frutas = ["manzana", "pera"]
print(get(frutas, 10) |> unwrap_or("nada"))
```

```sortie
nada
```

Las opciones se explican en [gestionar los errores](erreurs.md).

## Los intervalos

`a..b` es la lista de los números enteros de `a` a `b`, ambos incluidos:

```rythme
print(1..5, len(1..100), 5..1)
print(0..20 |> step_by(5), reverse(1..3))
```

```sortie
[1, 2, 3, 4, 5] 100 []
[0, 5, 10, 15, 20] [3, 2, 1]
```

`5..1` está vacío. Para contar de tanto en tanto, usa `step_by`; para contar
hacia atrás, `reverse`.

## Las funciones de listas más comunes

```rythme
numeros = [3, 1, 4, 1, 5, 9, 2, 6]
print(sort(numeros), reverse(numeros), unique(numeros))
print(sum(numeros), max(numeros), min(numeros))
print(take(numeros, 3), drop(numeros, 5), contains(numeros, 9))
print(filter(numeros, is_even), map(numeros, fn(x) = x * 10))
```

```sortie
[1, 1, 2, 3, 4, 5, 6, 9] [6, 2, 9, 5, 1, 4, 1, 3] [3, 1, 4, 5, 9, 2, 6]
31 9 1
[3, 1, 4] [9, 2, 6] true
[4, 2, 6] [30, 10, 40, 10, 50, 90, 20, 60]
```

| Función | Lo que da |
|---------|-----------|
| `sort`, `reverse`, `unique` | la lista ordenada, invertida, sin duplicados |
| `sum`, `max`, `min` | la suma, el mayor, el menor |
| `take(xs, n)`, `drop(xs, n)` | los `n` primeros, o todo salvo los `n` primeros |
| `contains(xs, x)` | `true` si `x` está en la lista |
| `filter(xs, f)` | los elementos para los que `f` responde `true` |
| `map(xs, f)` | el resultado de `f` para cada elemento |
| `is_empty(xs)` | `true` si la lista está vacía |

> **Consejo:** `rythme help lists` muestra todas las funciones de listas, y
> `rythme help sort_by` explica una de ellas.

## Los registros

Un **registro** agrupa valores con nombre, sus **campos**. Es una ficha: una
casilla «nombre», una casilla «nacimiento»... Se escribe entre llaves, y un
campo se lee con un punto:

```rythme
ada = {nombre: "Ada", nacimiento: 1815, ciudad: "Londres"}
print(ada.nombre, ada.nacimiento)
print(ada)
```

```sortie
Ada 1815
{nombre: "Ada", nacimiento: 1815, ciudad: "Londres"}
```

Rythme no tiene clases: un registro basta para describir una persona, un
producto o un punto. `keys` da los nombres de los campos, `has` dice si un
campo existe, y `get` lee un campo que puede faltar:

```rythme
ada = {nombre: "Ada", edad: 36}
print(keys(ada), has(ada, "edad"), get(ada, "ciudad"))
```

```sortie
["nombre", "edad"] true None
```

## Actualizar un registro

Un registro no se modifica: se fabrica uno nuevo.
`{..ada, nacimiento: 1816}` copia todos los campos de `ada` y luego
reemplaza `nacimiento`:

```rythme
ada = {nombre: "Ada", nacimiento: 1815}
corregida = {..ada, nacimiento: 1816}
con_ciudad = {..ada, ciudad: "Londres"}
print(ada)
print(corregida)
print(con_ciudad)
```

```sortie
{nombre: "Ada", nacimiento: 1815}
{nombre: "Ada", nacimiento: 1816}
{nombre: "Ada", nacimiento: 1815, ciudad: "Londres"}
```

`ada` no ha cambiado.

## Descomponer un valor

Se pueden nombrar de una vez las partes de una lista o de un registro:

```rythme
[primero, ..resto] = [10, 20, 30]
{nombre, nacimiento} = {nombre: "Ada", nacimiento: 1815}
print(primero, resto, nombre, nacimiento)
```

```sortie
10 [20, 30] Ada 1815
```

`[primero, ..resto]` quiere decir «el primer elemento, y luego todo lo
demás». `{nombre, nacimiento}` toma los campos con esos nombres. La misma
escritura sirve de patrón en `match` (ver [tus propios tipos](types.md)).

## Una lista de registros

Es la forma más común de los datos, como las filas de una tabla:

```rythme
alumnos = [
  {nombre: "Ada", nota: 17},
  {nombre: "Alan", nota: 12},
  {nombre: "Grace", nota: 19},
]
mejor = max_by(alumnos, fn(a) = a.nota)
print(mejor.nombre)
print(alumnos |> map(fn(a) = a.nombre))
print(sort_by(alumnos, fn(a) = a.nota) |> map(fn(a) = a.nota))
```

```sortie
Grace
["Ada", "Alan", "Grace"]
[12, 17, 19]
```

`max_by` y `sort_by` reciben una función que dice **qué** comparar: aquí, la
nota de cada alumno.

## Los diccionarios

Un [diccionario](../debuter/glossaire.md#diccionario) (`dict`) asocia un
valor a cada **clave**, igual que un diccionario de verdad asocia una
definición a cada palabra. Las claves pueden ser de cualquier tipo: textos,
números, símbolos... Se construye a partir de una lista de pares
`[clave, valor]`:

```rythme
edades = dict([["Bob", 25], ["Ada", 36]])
print(edades["Ada"], get(edades, "Eve"), len(edades))
mayores = put(edades, "Bob", 26)
print(mayores)
print(edades)
```

```sortie
36 None 2
dict("Ada" => 36, "Bob" => 26)
dict("Ada" => 36, "Bob" => 25)
```

`put` da un **nuevo** diccionario, y `edades` no ha cambiado. Un diccionario
siempre está ordenado según sus claves.

Las claves suelen ser **símbolos**, nombres precedidos de `:`. `each` da al
bloque la clave y el valor de cada par:

```rythme
existencias = dict([[:manzana, 3], [:pera, 0]])
print(existencias[:manzana], has(existencias, :kiwi), keys(existencias))
existencias.each do |fruta, n|
  print(fruta, n)
end
```

```sortie
3 false [:manzana, :pera]
:manzana 3
:pera 0
```

> **Nota:** cuando los nombres se conocen de antemano (`nombre`, `edad`), usa
> un registro. Cuando las claves llegan con los datos (nombres de pila,
> palabras), usa un diccionario.

## Los conjuntos

Un **conjunto** (`set`) guarda cada valor una sola vez, como una colección de
sellos sin repetidos:

```rythme
vistos = set([3, 1, 3, 2, 1])
print(vistos, len(vistos), contains(vistos, 2))
```

```sortie
set(1, 2, 3) 3 true
```

## Qué colección elegir

| Necesidad | Elección | Ejemplo |
|-----------|----------|---------|
| valores en un orden | lista | `[1, 2, 3]` |
| números seguidos | intervalo | `1..10` |
| una «ficha» con campos fijos | registro | `{nombre: "Ada", edad: 36}` |
| encontrar un valor por su clave | diccionario | `dict([["Ada", 36]])` |
| valores sin duplicados | conjunto | `set([1, 2])` |

## Errores frecuentes

### Leer después del último elemento

```rythme norun
frutas = ["manzana", "pera", "higo"]
print(frutas[3])
```

```text
error[E007]: el índice 3 está fuera de rango: la lista tiene 3 elemento(s)
 --> main.ry:2:14
  |
2 | print(frutas[3])
  |              ^ no hay ningún elemento aquí
  |
  = ayuda: los índices válidos van de 0 a 2 (o -1 para el último elemento)
  = ayuda: para obtener una Option en vez de un error, usa `get(x, 3)`
  = más sobre este error: `rythme explain E007`
```

Las posiciones empiezan en 0: con 3 elementos, la última es 2. Escribe
`frutas[2]`, `frutas[-1]`, o `get(frutas, 3)` si el elemento puede faltar.

### Creer que push modifica la lista

```rythme norun
frutas = ["manzana", "pera"]
push(frutas, "kiwi")
print(frutas)
```

```text
advertencia[W008]: el resultado de `push` se pierde
 --> main.ry:2:1
  |
2 | push(frutas, "kiwi")
  | ^^^^^^^^^^^^^^^^^^^^ este valor se pierde
  |
  = ayuda: los valores nunca cambian: `push(list, x)` da un valor nuevo, y no cambia nada.
      Para conservarlo, dale un nombre: `nuevo = push(list, x)`
```

El programa sigue mostrando `["manzana", "pera"]`. Guarda la nueva lista:
`frutas2 = push(frutas, "kiwi")`.

### Modificar un campo con =

```rythme norun
ada = {nombre: "Ada", edad: 36}
ada.edad = 37
```

```text
error[E001]: no se puede dar un valor a esto
 --> main.ry:2:1
  |
2 | ada.edad = 37
  | ^^^^^^^^ esto no es un nombre
  |
2 | ada.edad = 37
  |          - `=` da un nombre a un valor
  |
  = ayuda: los registros no se pueden cambiar; construye uno nuevo: `nuevo = {..viejo, edad: valor}`
  = más sobre este error: `rythme explain E001`
```

Fabrica un nuevo registro: `ada2 = {..ada, edad: 37}`.

### Leer un campo que no existe

```rythme norun
ada = {nombre: "Ada", edad: 36}
print(ada.ciudad)
```

```text
error[E010]: este registro no tiene el campo `ciudad`
 --> main.ry:2:11
  |
2 | print(ada.ciudad)
  |           ^^^^^^ campo desconocido
  |
  = note: sus campos son: `nombre`, `edad`
  = ayuda: ¿quisiste decir `edad`?
  = ayuda: para obtener una Option en vez de un error, usa `get(registro, "ciudad")`
  = más sobre este error: `rythme explain E010`
```

Comprueba la ortografía del campo, o usa `get(ada, "ciudad")` si el campo
puede faltar.

## Ponte a prueba

**Pregunta 1:** con `xs = [4, 8, 15, 16]`, ¿cuánto valen `xs[1]`, `xs[-1]` y
`len(xs)`?

> **Respuesta**
>
> `8`, `16` y `4`. La posición 1 es el segundo elemento, `-1` el último.

**Pregunta 2:** ¿qué muestra este programa?

```rythme norun
p = {x: 1, y: 2}
q = {..p, y: 5}
print(p.y, q.y)
```

> **Respuesta**
>
> `2 5`. `q` es un registro nuevo; `p` no ha cambiado.

**Pregunta 3:** quieres contar cuántas veces aparece cada nombre de pila.
¿Qué colección eliges?

> **Respuesta**
>
> Un diccionario: cada nombre es una clave, y su valor es el número de veces
> que aparece.

## En resumen

- Una lista se escribe `[1, 2, 3]`; `xs[0]` es el primer elemento, `xs[-1]`
  el último.
- `get` y `first` dan una `Option`, sin riesgo de detener el programa.
- Un registro `{nombre: "Ada", edad: 36}` agrupa campos, que se leen con un
  punto: `ada.nombre`.
- Un diccionario encuentra un valor por su clave; un conjunto guarda cada
  valor una sola vez.
- Ninguna colección cambia: `push`, `put` y `{..r, campo: valor}` fabrican
  un valor nuevo.

Siguiente: [diccionarios y conjuntos](dictionnaires.md)
