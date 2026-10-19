# Diccionarios y conjuntos

Esta página explica cómo asociar claves a valores con los diccionarios, y cómo trabajar con colecciones sin duplicados con los conjuntos.

> **Objetivos:**
>
> - sabrás crear un diccionario, leer sus valores y modificarlos;
> - sabrás recorrer sus claves, sus valores o ambos;
> - sabrás contar palabras con un diccionario, y después con `Counter`;
> - sabrás usar un conjunto y sus operaciones (unión, intersección...);
> - sabrás elegir entre lista, tupla, diccionario y conjunto.

## Crear un diccionario

Un [diccionario](../debuter/glossaire.md#diccionario) (`dict`) asocia **claves** a
**valores**. Piensa en un diccionario de verdad: buscas una palabra (la clave) para
encontrar su definición (el valor). O en una agenda: un nombre te da un número.

Se escribe entre llaves `{ }`, con pares `clave: valor` separados por comas.

```python
edades = {"Alicia": 30, "Bruno": 25, "Carla": 35}
capitales = {"Francia": "París", "Italia": "Roma"}
vacio = {}

print(edades)
print(len(edades))
print(capitales["Italia"])
print(vacio)
```

```sortie
{'Alicia': 30, 'Bruno': 25, 'Carla': 35}
3
Roma
{}
```

- Las claves suelen ser textos o números. Cada clave es **única**.
- Los valores pueden ser cualquier cosa, incluso listas u otros diccionarios.
- El diccionario conserva el orden en el que se añadieron las claves.

También puedes crearlo con `dict(...)` y argumentos con nombre:

```python
persona = dict(nombre="Alicia", ciudad="Sevilla")
print(persona)
```

```sortie
{'nombre': 'Alicia', 'ciudad': 'Sevilla'}
```

## Leer un valor: [] o get

Hay dos formas de leer un valor:

- `d[clave]`: si la clave no existe, es un error (`KeyError`);
- `d.get(clave)`: si la clave no existe, devuelve `None`, o el valor por defecto que
  le des.

```python
stock = {"manzana": 12, "pera": 0}
print(stock["manzana"])
print(stock.get("manzana"))
print(stock.get("kiwi"))
print(stock.get("kiwi", 0))
print("kiwi" in stock)
```

```sortie
12
12
None
0
False
```

`in` comprueba si existe una **clave** (no un valor).

> **Recuerda:** usa `d[clave]` cuando la clave tiene que existir (si falta, es un
> bug). Usa `d.get(clave, defecto)` cuando puede faltar.

## Añadir, modificar, eliminar

Para añadir o modificar, asignas un valor a una clave. Si la clave existe, su valor se
reemplaza; si no, se crea.

```python
edades = {"Alicia": 30}
edades["Bruno"] = 25
edades["Alicia"] = 31
print(edades)

edades.update({"Carla": 35, "Bruno": 26})
print(edades)

del edades["Bruno"]
print(edades)

edad_carla = edades.pop("Carla")
print(edad_carla, edades)
```

```sortie
{'Alicia': 31, 'Bruno': 25}
{'Alicia': 31, 'Bruno': 26, 'Carla': 35}
{'Alicia': 31, 'Carla': 35}
35 {'Alicia': 31}
```

- `d[clave] = valor` añade o reemplaza;
- `update` añade o reemplaza varias claves de una vez;
- `del d[clave]` elimina; `pop(clave)` elimina y devuelve el valor.

## Claves, valores y pares

Tres métodos dan una vista del contenido:

- `keys()`: las claves;
- `values()`: los valores;
- `items()`: los pares `(clave, valor)`.

```python
notas = {"Alicia": 15, "Bruno": 12, "Carla": 18}
print(list(notas.keys()))
print(list(notas.values()))
print(list(notas.items()))
print(sum(notas.values()) / len(notas))
```

```sortie
['Alicia', 'Bruno', 'Carla']
[15, 12, 18]
[('Alicia', 15), ('Bruno', 12), ('Carla', 18)]
15.0
```

## Recorrer un diccionario

Un bucle `for` sobre un diccionario recorre sus **claves**. Para tener la clave y el
valor juntos, recorre `items()` y separa cada par.

```python
notas = {"Alicia": 15, "Bruno": 12, "Carla": 18}

for nombre in notas:
    print(nombre)

for nombre, nota in notas.items():
    print(f"{nombre}: {nota}/20")
```

```sortie
Alicia
Bruno
Carla
Alicia: 15/20
Bruno: 12/20
Carla: 18/20
```

Para recorrerlo en otro orden, ordena. `sorted(notas.items(), key=...)` ordena los
pares, por ejemplo de la nota más alta a la más baja:

```python
notas = {"Alicia": 15, "Bruno": 12, "Carla": 18}
for nombre, nota in sorted(notas.items(), key=lambda par: par[1], reverse=True):
    print(nombre, nota)
```

```sortie
Carla 18
Alicia 15
Bruno 12
```

## Valores complejos

Un valor puede ser una lista u otro diccionario. Así se construyen datos más ricos,
como una ficha.

```python
alumno = {
    "nombre": "Alicia",
    "notas": [15, 12, 17],
    "direccion": {"ciudad": "Sevilla", "codigo": "41001"},
}
print(alumno["notas"][0])
print(alumno["direccion"]["ciudad"])
alumno["notas"].append(19)
print(alumno["notas"])
```

```sortie
15
Sevilla
[15, 12, 17, 19]
```

Lee `alumno["direccion"]["ciudad"]` de izquierda a derecha: la ficha, luego su
dirección, luego la ciudad de esa dirección.

## Contar palabras

Contar es el uso más clásico de un diccionario: la clave es la palabra, el valor es el
número de veces que aparece.

```python
texto = "el gato y el perro y el conejo"
contador = {}
for palabra in texto.split():
    contador[palabra] = contador.get(palabra, 0) + 1
print(contador)
```

```sortie
{'el': 3, 'gato': 1, 'y': 2, 'perro': 1, 'conejo': 1}
```

`contador.get(palabra, 0)` devuelve la cuenta actual, o 0 si la palabra es nueva. Le
sumas 1 y guardas el resultado.

### Con collections.Counter

El módulo `collections` ofrece `Counter`, un diccionario especializado en contar. Hace
todo el trabajo en una línea, y sabe darte los más frecuentes.

```python
from collections import Counter

texto = "el gato y el perro y el conejo"
contador = Counter(texto.split())
print(contador["el"])
print(contador["tigre"])
print(contador.most_common(2))
print(Counter("abracadabra").most_common(3))
```

```sortie
3
0
[('el', 3), ('y', 2)]
[('a', 5), ('b', 2), ('r', 2)]
```

Una palabra ausente vale 0, sin error. `most_common(n)` da los `n` elementos más
frecuentes.

### Con defaultdict

`defaultdict` crea automáticamente un valor inicial para una clave que no existe. Le
das el tipo de ese valor: `list` para una lista vacía, `int` para 0.

```python
from collections import defaultdict

palabras = ["avión", "barco", "aguacate", "banana", "cereza"]
por_letra = defaultdict(list)
for palabra in palabras:
    por_letra[palabra[0]].append(palabra)

print(dict(por_letra))
```

```sortie
{'a': ['avión', 'aguacate'], 'b': ['barco', 'banana'], 'c': ['cereza']}
```

Sin `defaultdict`, tendrías que comprobar cada vez si la clave existe antes de añadir a
su lista.

## Los conjuntos

Un [conjunto](../debuter/glossaire.md#conjunto) (`set`) es una colección **sin
duplicados** y **sin orden**. Piensa en una bolsa de canicas: cada canica está una sola
vez, y no están ordenadas. Se escribe entre llaves, sin `:`.

```python
colores = {"rojo", "verde", "azul", "rojo"}
print(len(colores))
print("verde" in colores)

colores.add("amarillo")
colores.discard("rojo")
print(sorted(colores))

notas = [12, 15, 12, 18, 15]
print(sorted(set(notas)))
```

```sortie
3
True
['amarillo', 'azul', 'verde']
[12, 15, 18]
```

- el duplicado `"rojo"` se guarda una sola vez;
- `add` añade, `discard` quita (sin error si el elemento no está);
- `set(lista)` elimina los duplicados de una lista.

Como un conjunto no tiene orden, lo ordenas con `sorted` antes de mostrarlo.

> **Atención:** `{}` crea un diccionario vacío, no un conjunto. Para un conjunto
> vacío, escribe `set()`.

### Las operaciones de conjuntos

Los conjuntos saben hacer las operaciones de las matemáticas:

| Operación | Método | Contiene |
| --- | --- | --- |
| unión | `a.union(b)` | los elementos de `a` **o** de `b` |
| intersección | `a.intersection(b)` | los elementos de `a` **y** de `b` |
| diferencia | `a.difference(b)` | los elementos de `a` que no están en `b` |
| diferencia simétrica | `a.symmetric_difference(b)` | los elementos de uno solo de los dos |

Cada operación tiene también un signo más corto: `a | b` para la unión, `a & b` para
la intersección, `a - b` para la diferencia y `a ^ b` para la diferencia simétrica.

```python
alicia = {"piano", "tenis", "lectura"}
bruno = {"tenis", "cocina", "lectura", "bici"}

print(sorted(alicia | bruno))
print(sorted(alicia & bruno))
print(sorted(alicia - bruno))
print(sorted(alicia ^ bruno))
print({"tenis"} <= alicia)
```

```sortie
['bici', 'cocina', 'lectura', 'piano', 'tenis']
['lectura', 'tenis']
['piano']
['bici', 'cocina', 'piano']
True
```

`<=` comprueba que un conjunto está **incluido** en otro.

> **Consejo:** `in` es muy rápido en un conjunto, incluso con un millón de elementos.
> En una lista, Python tiene que recorrerlo todo. Para muchas búsquedas, prefiere un
> conjunto.

### frozenset: un conjunto congelado

Un `frozenset` es un conjunto [inmutable](../debuter/glossaire.md#inmutable): ya no se
puede añadir ni quitar nada. Por eso puede servir de clave de diccionario, algo que no
está permitido para un `set` normal.

```python
vocales = frozenset("aeiou")
print("e" in vocales)

trayectos = {frozenset({"Madrid", "Sevilla"}): 530}
print(trayectos[frozenset({"Sevilla", "Madrid"})])
```

```sortie
True
530
```

El orden no importa en un conjunto: `{"Madrid", "Sevilla"}` y `{"Sevilla", "Madrid"}`
indican el mismo trayecto.

## ¿Qué colección elegir?

| Colección | Escritura | Ordenada | Modificable | Duplicados | Uso típico |
| --- | --- | --- | --- | --- | --- |
| `list` | `[1, 2, 3]` | sí | sí | sí | una serie de elementos: compras, notas |
| `tuple` | `(1, 2)` | sí | no | sí | un grupo fijo: coordenadas, fecha |
| `dict` | `{"a": 1}` | sí (orden de inserción) | sí | claves únicas | encontrar un valor por su clave |
| `set` | `{1, 2}` | no | sí | no | eliminar duplicados, comprobar si un elemento está |
| `frozenset` | `frozenset({1, 2})` | no | no | no | un conjunto que sirve de clave |

Hazte dos preguntas:

- «¿Busco por **posición** o por **nombre**?» Posición: lista o tupla. Nombre:
  diccionario.
- «¿Tienen sentido los duplicados?» No: conjunto.

## Errores frecuentes

### Leer una clave que no existe

```python norun
edades = {"Alicia": 30}
print(edades["Bruno"])
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 2, in <module>
    print(edades["Bruno"])
          ~~~~~~^^^^^^^^^
KeyError: 'Bruno'
```

La clave `"Bruno"` no existe. Usa `edades.get("Bruno")`, o comprueba antes con
`if "Bruno" in edades:`.

### Una lista como clave

```python norun
d = {[1, 2]: "a"}
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 1, in <module>
    d = {[1, 2]: "a"}
        ^^^^^^^^^^^^^
TypeError: unhashable type: 'list'
```

Una clave tiene que ser inmutable: una lista puede cambiar, así que se rechaza
(*unhashable*). Usa una tupla: `{(1, 2): "a"}`.

### Eliminar mientras recorres

```python norun
stock = {"manzana": 0, "pera": 3}
for fruta in stock:
    if stock[fruta] == 0:
        del stock[fruta]
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 2, in <module>
    for fruta in stock:
                 ^^^^^
RuntimeError: dictionary changed size during iteration
```

No puedes cambiar el tamaño de un diccionario mientras lo recorres. Recorre una copia
de las claves: `for fruta in list(stock):`.

### Creer que {} es un conjunto vacío

```python norun
s = {}
s.add(1)
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 2, in <module>
    s.add(1)
    ^^^^^
AttributeError: 'dict' object has no attribute 'add'
```

`{}` es un diccionario vacío. Para un conjunto vacío, escribe `s = set()`.

## Ponte a prueba

**Pregunta 1:** ¿qué devuelve `{"a": 1}.get("b", 5)`?

> **Respuesta**
>
> `5`. La clave `"b"` no existe, así que `get` devuelve el valor por defecto.

**Pregunta 2:** ¿cómo cuentas las letras de `"mississippi"` y muestras la más
frecuente?

> **Respuesta**
>
> ```python
> from collections import Counter
>
> print(Counter("mississippi").most_common(1))
> ```
>
> ```sortie
> [('i', 4)]
> ```
>
> `i` y `s` aparecen 4 veces cada una; en caso de empate, `Counter` conserva el orden
> de aparición, así que `i` va primero.

**Pregunta 3:** ¿qué alumnos están inscritos en los dos clubes?

```python norun
ajedrez = {"Alicia", "Bruno", "Carla"}
teatro = {"Bruno", "David", "Carla"}
```

> **Respuesta**
>
> La intersección:
>
> ```python
> ajedrez = {"Alicia", "Bruno", "Carla"}
> teatro = {"Bruno", "David", "Carla"}
> print(sorted(ajedrez & teatro))
> ```
>
> ```sortie
> ['Bruno', 'Carla']
> ```

## En resumen

- Un diccionario `{clave: valor}` encuentra un valor por su clave; las claves son
  únicas e inmutables.
- `d[clave]` falla si la clave no existe; `d.get(clave, defecto)` no.
- `keys()`, `values()` e `items()` dan las claves, los valores y los pares;
  `for clave, valor in d.items():` lo recorre todo.
- `Counter` cuenta, `defaultdict` crea los valores iniciales por sí solo.
- Un conjunto `{...}` no tiene ni orden ni duplicados; `|`, `&`, `-` y `^` los
  combinan; `set()` es el conjunto vacío.

Siguiente: [las comprensiones](comprehensions.md).
