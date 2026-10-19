# Diccionarios y conjuntos

Esta página profundiza en dos colecciones: el diccionario, que encuentra un valor por su clave, y el conjunto, que guarda cada valor una sola vez.

> **Objetivos:**
>
> - sabrás crear un diccionario y leer, añadir o quitar una clave;
> - sabrás recorrer un diccionario, sus claves y sus valores;
> - sabrás contar palabras con un diccionario;
> - sabrás crear un conjunto y combinar dos conjuntos;
> - sabrás elegir entre un registro, un diccionario y un conjunto.

## Crear un diccionario

Un [diccionario](../debuter/glossaire.md#diccionario) asocia un **valor**
a cada **clave**. Es como una agenda de teléfonos: buscas un nombre (la
clave) y encuentras un número (el valor).

Rythme no tiene una escritura especial para los diccionarios: se llama a la
función `dict` con una lista de pares `[clave, valor]`:

```rythme
capitales = dict([["Francia", "París"], ["Italia", "Roma"], ["Japón", "Tokio"]])
print(capitales)
print(len(capitales))
```

```sortie
dict("Francia" => "París", "Italia" => "Roma", "Japón" => "Tokio")
3
```

Al mostrarlo, aparece una flecha `=>` entre cada clave y su valor. Esta
forma solo sirve para mostrar: en el código, escribe siempre
`dict([[clave, valor], ...])`.

`dict()` es el diccionario vacío. Cuando una clave aparece dos veces, gana
el último valor:

```rythme
print(dict(), dict([[1, "uno"], [2, "dos"], [1, "UNO"]]))
```

```sortie
dict() dict(1 => "UNO", 2 => "dos")
```

## Claves de todo tipo

Las claves no son siempre textos. Pueden ser números,
[símbolos](../debuter/glossaire.md#simbolo) e incluso listas. Aquí, una
casilla de una cuadrícula se identifica por sus coordenadas `[x, y]`:

```rythme
cuadricula = dict([[[0, 0], "salida"], [[2, 1], "llegada"]])
print(cuadricula[[2, 1]])
stock = dict([[:manzana, 3], [:pera, 0]])
print(stock[:manzana])
```

```sortie
llegada
3
```

Solo las funciones no pueden ser claves.

> **Nota:** un diccionario siempre está ordenado según sus claves (el orden
> alfabético para los textos), sea cual sea el orden en que lo hayas
> construido.

## Leer un valor

Hay dos formas de leer, como con las listas:

- `dic[clave]` da el valor, y detiene el programa si falta la clave;
- `get(dic, clave)` da una `Option`: `Some(valor)` o `None`.

`has(dic, clave)` dice si la clave existe:

```rythme
capitales = dict([["Francia", "París"], ["Japón", "Tokio"]])
print(capitales["Japón"])
print(get(capitales, "Japón"), get(capitales, "Chile"))
print(has(capitales, "Francia"), has(capitales, "Chile"))
print(unwrap_or(get(capitales, "Chile"), "desconocida"))
```

```sortie
Tokio
Some("Tokio") None
true false
desconocida
```

Usa `[...]` cuando estés seguro de que la clave existe, y `get` si no. Las
opciones se explican en [gestionar los errores](erreurs.md).

## Añadir y quitar una clave

Un diccionario no cambia nunca: es
[inmutable](../debuter/glossaire.md#inmutable). `put` y `remove` fabrican un
diccionario **nuevo**:

```rythme
capitales = dict([["Francia", "París"], ["Italia", "Roma"]])
mas = put(capitales, "Chile", "Santiago")
menos = remove(capitales, "Italia")
corregido = put(capitales, "Francia", "PARÍS")
print(mas)
print(menos)
print(corregido)
print(capitales)
```

```sortie
dict("Chile" => "Santiago", "Francia" => "París", "Italia" => "Roma")
dict("Francia" => "París")
dict("Francia" => "PARÍS", "Italia" => "Roma")
dict("Francia" => "París", "Italia" => "Roma")
```

`put` añade la clave, o reemplaza su valor si ya existe. El diccionario
`capitales` original no ha cambiado.

## Claves, valores y pares

```rythme
capitales = dict([["Francia", "París"], ["Italia", "Roma"]])
print(keys(capitales))
print(values(capitales))
print(entries(capitales))
```

```sortie
["Francia", "Italia"]
["París", "Roma"]
[["Francia", "París"], ["Italia", "Roma"]]
```

| Función | Lo que da |
|---------|-----------|
| `keys(d)` | la lista de las claves |
| `values(d)` | la lista de los valores |
| `entries(d)` | la lista de los pares `[clave, valor]` |

Estas tres funciones dan **listas**: después puedes usar `map`, `sort`,
`filter` y todas las funciones de listas.

## Recorrer un diccionario

`each` llama a una función para cada par, en el orden de las claves. La
función recibe **dos** valores: la clave y el valor.

```rythme
capitales = dict([["Francia", "París"], ["Japón", "Tokio"]])
each(capitales, fn(pais, ciudad) = print("{pais}: {ciudad}"))
capitales.each do |pais, ciudad|
  print("La capital de {pais} es {ciudad}.")
end
```

```sortie
Francia: París
Japón: Tokio
La capital de Francia es París.
La capital de Japón es Tokio.
```

El estilo con `do |pais, ciudad|` es el de Ruby; las dos formas hacen lo
mismo.

`filter` conserva los pares elegidos, y `map_values` transforma cada valor:

```rythme
precios = dict([["manzana", 3], ["pera", 5], ["higo", 9]])
print(filter(precios, fn(fruta, p) = p < 6))
print(map_values(precios, fn(p) = p * 2))
```

```sortie
dict("manzana" => 3, "pera" => 5)
dict("higo" => 18, "manzana" => 6, "pera" => 10)
```

## Contar palabras

Es el uso más clásico de un diccionario: cada palabra es una clave, y su
valor es el número de veces que la has visto.

La idea: empiezas con un diccionario vacío y, para cada palabra, sumas 1 a
su contador (0 si todavía no existe). `fold` pasa el diccionario de una
palabra a la siguiente (ver [pensar en funcional](fonctionnel.md)):

```rythme
fn contar(cuenta, palabra)
  ya = unwrap_or(get(cuenta, palabra), 0)
  put(cuenta, palabra, ya + 1)
end

texto = "el gato y el perro y el gato"
cuenta = fold(split(texto), dict(), contar)
print(cuenta)
print(cuenta["el"])
```

```sortie
dict("el" => 3, "gato" => 2, "perro" => 1, "y" => 2)
3
```

Para encontrar las palabras más frecuentes, se pasa por la lista de pares:

```rythme
cuenta = dict([["el", 3], ["gato", 2], ["perro", 1], ["y", 2]])
podio = entries(cuenta) |> sort_by(fn(par) = -par[1]) |> take(2)
print(podio)
```

```sortie
[["el", 3], ["gato", 2]]
```

Ordenar según `-par[1]` va del número más grande al más pequeño.

## Crear un conjunto

Un **conjunto** (`set`) contiene valores **sin duplicados**, como un álbum
de sellos donde cada sello aparece una sola vez. Se construye con `set` y
una lista:

```rythme
colores = set(["rojo", "verde", "rojo", "azul"])
print(colores, len(colores))
print(set(chars("banana")))
print(set(), is_empty(set()))
```

```sortie
set("azul", "rojo", "verde") 3
set("a", "b", "n")
set() true
```

Como un diccionario, un conjunto está ordenado según sus valores.

## Añadir, quitar, buscar

Los nombres son los mismos que para las listas y los diccionarios:

```rythme
amigos = set(["Ada", "Alan", "Grace"])
print(push(amigos, "Linus"))
print(remove(amigos, "Ada"))
print(contains(amigos, "Ada"), has(amigos, "Linus"))
print(values(amigos))
```

```sortie
set("Ada", "Alan", "Grace", "Linus")
set("Alan", "Grace")
true false
["Ada", "Alan", "Grace"]
```

- `push(c, x)` da el conjunto con `x` además;
- `remove(c, x)` da el conjunto sin `x`;
- `contains(c, x)` y `has(c, x)` dicen si `x` está dentro;
- `values(c)` da sus elementos en forma de lista.

Buscar en un conjunto es muy rápido, incluso con millones de elementos.

## Combinar dos conjuntos

Piensa en dos círculos que se superponen: los amigos de Ada y los amigos de
Linus.

```rythme
a = set(["Alan", "Grace", "Tim"])
b = set(["Alan", "Linus", "Tim"])
print(union(a, b))
print(intersection(a, b))
print(difference(a, b))
print(is_subset(set(["Alan"]), a))
```

```sortie
set("Alan", "Grace", "Linus", "Tim")
set("Alan", "Tim")
set("Grace")
true
```

| Función | Lo que da | Con los círculos |
|---------|-----------|------------------|
| `union(a, b)` | lo que está en `a` **o** en `b` | los dos círculos enteros |
| `intersection(a, b)` | lo que está en `a` **y** en `b` | la parte común |
| `difference(a, b)` | lo que está en `a` pero **no** en `b` | `a` sin la parte común |
| `is_subset(a, b)` | `true` si todo `a` está en `b` | `a` está dentro de `b` |

`union` también funciona con dos diccionarios: para una clave presente en
los dos lados, gana el valor del segundo.

## ¿Registro, diccionario o conjunto?

Los tres se parecen. Una pregunta sencilla ayuda a elegir: **¿conoces los
nombres de antemano?**

| Colección | Cuándo elegirla | Ejemplo |
|-----------|-----------------|---------|
| [registro](../debuter/glossaire.md#registro) | una ficha cuyos campos se conocen al escribir el programa | `{nombre: "Ada", edad: 36}` |
| diccionario | claves que llegan con los datos: palabras, nombres de pila, coordenadas | `dict([["Ada", 36], ["Alan", 41]])` |
| conjunto | solo saber **si** un valor está presente, sin duplicados | `set(["Ada", "Alan"])` |

Un alumno siempre tiene un `nombre` y una `nota`: es un registro. Las
palabras de un libro no se conocen de antemano: para contarlas, un
diccionario. Para saber qué palabras ya has visto, basta con un conjunto.

## Errores frecuentes

### Leer una clave que no existe

```rythme norun
stock = dict([["manzana", 3], ["pera", 0]])
print(stock["kiwi"])
```

```text
error[E010]: este diccionario no tiene la clave "kiwi"
 --> main.ry:2:13
  |
2 | print(stock["kiwi"])
  |             ^^^^^^
  |
  = ayuda: usa `get(dic, clave)` para recibir una Option en su lugar, o `has(dic, clave)` para comprobarlo antes
  = más sobre este error: `rythme explain E010`
```

Si la clave puede faltar, escribe `get(stock, "kiwi")`, o
`unwrap_or(get(stock, "kiwi"), 0)` para tener 0 por defecto.

### Olvidar los corchetes alrededor de los pares

```rythme norun
edades = dict(["Ada", 36])
```

```text
error[E023]: `dict` necesita una lista de pares [clave, valor], pero contiene Text "Ada"
 --> main.ry:1:10
  |
1 | edades = dict(["Ada", 36])
  |          ^^^^^^^^^^^^^^^^^
  |
  = ayuda: cada par es una lista de dos elementos, como `dict([["Ada", 36], ["Alan", 41]])`
  = más sobre este error: `rythme explain E023`
```

Hace falta una lista **de pares**: `dict([["Ada", 36]])`, con dos pares de
corchetes.

### Buscar una función add

```rythme norun
vistos = set([1, 2])
print(add(vistos, 3))
```

```text
error[E003]: nombre desconocido `add`
 --> main.ry:2:7
  |
2 | print(add(vistos, 3))
  |       ^^^ no definido
  |
  = ayuda: en Rythme, esto se llama `push`
  = más sobre este error: `rythme explain E003`
```

Para un conjunto, igual que para una lista, se «añade» con `push`.

### Creer que put modifica el diccionario

```rythme norun
stock = dict([["manzana", 3]])
put(stock, "kiwi", 5)
print(stock)
```

```text
advertencia[W008]: el resultado de `put` se pierde
 --> main.ry:2:1
  |
2 | put(stock, "kiwi", 5)
  | ^^^^^^^^^^^^^^^^^^^^^ este valor se pierde
  |
  = ayuda: los valores nunca cambian: `put(record, name, value)` da un valor nuevo, y no cambia nada.
      Para conservarlo, dale un nombre: `nuevo = put(record, name, value)`

dict("manzana" => 3)
```

El kiwi no aparece. Guarda el resultado: `stock2 = put(stock, "kiwi", 5)`.

## Ponte a prueba

**Pregunta 1:** con `d = dict([["a", 1], ["b", 2]])`, ¿cuánto valen
`get(d, "b")`, `get(d, "z")` y `keys(d)`?

> **Respuesta**
>
> `Some(2)`, `None` y `["a", "b"]`.

**Pregunta 2:** ¿qué muestra `print(len(set([1, 2, 2, 3, 3, 3])))`?

> **Respuesta**
>
> `3`: un conjunto guarda cada valor una sola vez, así que contiene 1, 2
> y 3.

**Pregunta 3:** ¿qué alumnos van a la vez a la clase de piano y a la de
danza?

```rythme norun
piano = set(["Ada", "Lea", "Tom"])
danza = set(["Lea", "Sam", "Tom"])
```

> **Respuesta**
>
> `intersection(piano, danza)`, que vale `set("Lea", "Tom")`.

## En resumen

- `dict([[clave, valor], ...])` crea un diccionario; las claves pueden ser
  textos, números, símbolos, listas...
- `d[clave]` lee un valor seguro, `get(d, clave)` un valor que puede
  faltar, `has(d, clave)` comprueba.
- `put` y `remove` dan un diccionario nuevo; `keys`, `values` y `entries`
  dan listas.
- `set(lista)` crea un conjunto sin duplicados; `push`, `remove`,
  `contains`, `union`, `intersection` y `difference` trabajan con él.
- Campos conocidos de antemano: registro; claves que vienen de los datos:
  diccionario; simple presencia: conjunto.

Siguiente: [las trampas del principiante en Rythme](pieges.md)
