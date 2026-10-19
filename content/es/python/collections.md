# Listas y tuplas

Esta página explica cómo guardar varios valores juntos en Python, en una lista que puedes cambiar o en una tupla fija.

> **Objetivos:**
>
> - sabrás crear una lista, leer sus elementos y sacar una porción;
> - sabrás añadir, insertar, quitar y ordenar elementos;
> - sabrás evitar la trampa de dos nombres para una misma lista;
> - sabrás usar una tupla y desempaquetarla en variables;
> - sabrás convertir un `range` en lista.

## Crear una lista

Una [lista](../debuter/glossaire.md#lista) guarda varios valores en un orden preciso.
Imagina una lista de la compra: una hoja con líneas numeradas. Se escribe entre
corchetes `[ ]`, con comas entre los elementos.

```python
compra = ["pan", "leche", "huevos"]
notas = [12, 15, 9, 18]
vacia = []
mezcla = ["Alicia", 30, True]

print(compra)
print(len(notas))
print(vacia, len(vacia))
print(mezcla)
```

```sortie
['pan', 'leche', 'huevos']
4
[] 0
['Alicia', 30, True]
```

Una lista puede contener valores de tipos distintos. En la práctica, se suelen guardar
valores del mismo tipo: notas, nombres, precios.

## Leer un elemento

Como en un texto, cada elemento tiene un [índice](../debuter/glossaire.md#indice), que
empieza en **0**. Los índices negativos cuentan desde el final.

```python
colores = ["rojo", "verde", "azul", "amarillo"]
print(colores[0])
print(colores[2])
print(colores[-1])
```

```sortie
rojo
azul
amarillo
```

A diferencia de un texto, una lista es [mutable](../debuter/glossaire.md#mutable): se
puede cambiar un elemento.

```python
colores = ["rojo", "verde", "azul"]
colores[1] = "morado"
print(colores)
```

```sortie
['rojo', 'morado', 'azul']
```

## Las porciones

Una porción `lista[inicio:fin]` devuelve una **nueva** lista con una parte de los
elementos. El fin queda excluido, exactamente como con [el texto](texte.md).

```python
numeros = [10, 20, 30, 40, 50, 60]
print(numeros[1:4])
print(numeros[:3])
print(numeros[-2:])
print(numeros[::2])
print(numeros[::-1])
```

```sortie
[20, 30, 40]
[10, 20, 30]
[50, 60]
[10, 30, 50]
[60, 50, 40, 30, 20, 10]
```

## Añadir y quitar elementos

| Método | Efecto |
| --- | --- |
| `append(x)` | añade `x` al final |
| `insert(i, x)` | inserta `x` en el índice `i` |
| `extend(otra)` | añade todos los elementos de otra lista |
| `remove(x)` | quita el primer `x` encontrado |
| `pop()` | quita y devuelve el último elemento |
| `pop(i)` | quita y devuelve el elemento del índice `i` |

```python
fila = ["Alicia", "Bruno"]
fila.append("Carla")
print(fila)
fila.insert(0, "Zoe")
print(fila)
fila.remove("Bruno")
print(fila)
ultimo = fila.pop()
print(ultimo, fila)
primero = fila.pop(0)
print(primero, fila)
fila.extend(["David", "Elena"])
print(fila)
```

```sortie
['Alicia', 'Bruno', 'Carla']
['Zoe', 'Alicia', 'Bruno', 'Carla']
['Zoe', 'Alicia', 'Carla']
Carla ['Zoe', 'Alicia']
Zoe ['Alicia']
['Alicia', 'David', 'Elena']
```

Estos métodos modifican la lista **en el sitio**: cambian la propia lista.

## Buscar y medir

```python
notas = [12, 15, 9, 18, 15]
print(15 in notas)
print(20 in notas)
print(notas.index(18))
print(notas.count(15))
print(len(notas), sum(notas), min(notas), max(notas))
print(sum(notas) / len(notas))
```

```sortie
True
False
3
2
5 69 9 18
13.8
```

- `in` comprueba si un valor está presente;
- `index` da la posición de la primera aparición;
- `count` cuenta las apariciones;
- `len`, `sum`, `min` y `max` dan el tamaño, la suma, el más pequeño y el más grande.
  La media es `sum / len`.

## Ordenar: sort y sorted

Hay dos formas de ordenar:

- `lista.sort()` ordena la lista **en el sitio** y no devuelve nada (`None`);
- `sorted(lista)` devuelve una **nueva** lista ordenada y deja la original intacta.

```python
notas = [12, 15, 9, 18]
ordenadas = sorted(notas)
print(ordenadas)
print(notas)

notas.sort()
print(notas)

notas.sort(reverse=True)
print(notas)
```

```sortie
[9, 12, 15, 18]
[12, 15, 9, 18]
[9, 12, 15, 18]
[18, 15, 12, 9]
```

`reverse=True` ordena del más grande al más pequeño.

### Ordenar según un criterio con key

`key` recibe una función. Python la aplica a cada elemento y ordena según el
resultado. Es como decir «coloca a los alumnos **por estatura**» en lugar de «por
nombre».

```python
palabras = ["kiwi", "Piña", "fresa", "manzana"]
print(sorted(palabras))
print(sorted(palabras, key=str.lower))
print(sorted(palabras, key=len))

alumnos = [("Alicia", 15), ("Bruno", 12), ("Carla", 18)]
print(sorted(alumnos, key=lambda a: a[1], reverse=True))
```

```sortie
['Piña', 'fresa', 'kiwi', 'manzana']
['fresa', 'kiwi', 'manzana', 'Piña']
['kiwi', 'Piña', 'fresa', 'manzana']
[('Carla', 18), ('Alicia', 15), ('Bruno', 12)]
```

Las `lambda` se explican en [las funciones](fonctions.md).

> **Atención:** por defecto, las mayúsculas van antes que las minúsculas (`"Z"` antes
> que `"a"`). `key=str.lower` ordena sin tener en cuenta mayúsculas y minúsculas.

### Invertir el orden

`reverse()` da la vuelta a la lista en el sitio. `reversed()` o `[::-1]` dan una
versión al revés sin tocar la original.

```python
pasos = [1, 2, 3]
print(pasos[::-1])
print(list(reversed(pasos)))
pasos.reverse()
print(pasos)
```

```sortie
[3, 2, 1]
[3, 2, 1]
[3, 2, 1]
```

## Recorrer una lista

El bucle `for` recorre los elementos uno a uno. Se explica en
[condiciones y bucles](conditions-boucles.md).

```python
precios = [4.5, 12.0, 3.25]
total = 0
for p in precios:
    total += p
print(f"Total: {total:.2f} €")
```

```sortie
Total: 19.75 €
```

## Las listas anidadas

Una lista puede contener otras listas. Es práctico para una tabla de dos dimensiones:
una cuadrícula, un tablero de juego.

```python
cuadricula = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
]
print(cuadricula[1])
print(cuadricula[1][2])
for fila in cuadricula:
    print(fila[0], fila[1], fila[2])
```

```sortie
[4, 5, 6]
6
1 2 3
4 5 6
7 8 9
```

`cuadricula[1][2]` se lee en dos pasos: la fila de índice 1 (`[4, 5, 6]`), y luego el
elemento de índice 2 de esa fila (`6`).

## ¿Copiar o compartir? La trampa clásica

Escribir `b = a` **no** copia la lista. Pega una segunda etiqueta en la **misma**
lista. Si la modificas a través de `b`, `a` también cambia: es un **alias**.

```python
a = [1, 2, 3]
b = a
b.append(4)
print(a)
print(b)
print(a is b)
```

```sortie
[1, 2, 3, 4]
[1, 2, 3, 4]
True
```

`is` comprueba si dos nombres se refieren al mismo objeto. Aquí, sí: solo hay una
lista.

Para una copia de verdad, usa `list(a)`, `a.copy()` o `a[:]`:

```python
a = [1, 2, 3]
b = a.copy()
b.append(4)
print(a)
print(b)
print(a is b)
```

```sortie
[1, 2, 3]
[1, 2, 3, 4]
False
```

> **Atención:** `copy()` hace una copia **superficial**. En una lista de listas, las
> listas interiores siguen compartidas. En ese caso, usa `copy.deepcopy` (módulo
> `copy`).

```python
import copy

cuadricula = [[0, 0], [0, 0]]
superficial = cuadricula.copy()
profunda = copy.deepcopy(cuadricula)
cuadricula[0][0] = 9
print(superficial)
print(profunda)
```

```sortie
[[9, 0], [0, 0]]
[[0, 0], [0, 0]]
```

## Las tuplas

Una [tupla](../debuter/glossaire.md#tupla) se parece a una lista, pero es **fija**: no
se puede añadir, quitar ni cambiar ningún elemento. Se escribe con paréntesis.

```python
punto = (3, 4)
color = (255, 128, 0)
solo = (42,)

print(punto[0], punto[1])
print(len(color))
print(type(solo))
```

```sortie
3 4
3
<class 'tuple'>
```

Una tupla de un solo elemento necesita una coma: `(42,)`. Sin ella, `(42)` es solo el
número 42 entre paréntesis.

Se usa una tupla para un grupo de valores que van juntos y no cambian: unas
coordenadas, una fecha, un color RGB.

### Desempaquetar una tupla

El **desempaquetado** (*unpacking*) guarda cada elemento en su propia variable, en una
sola línea.

```python
punto = (3, 4)
x, y = punto
print(f"x = {x}, y = {y}")

a, b = 1, 2
a, b = b, a
print(a, b)

primero, *resto = [10, 20, 30, 40]
print(primero, resto)
```

```sortie
x = 3, y = 4
2 1
10 [20, 30, 40]
```

- `a, b = b, a` intercambia dos variables, sin variable temporal;
- `*resto` recoge todos los elementos restantes en una lista.

## range y las listas

`range` produce una serie de números a demanda, sin guardarla. Para obtener una lista
de verdad, pásalo a `list()`.

```python
print(list(range(5)))
print(list(range(2, 11, 2)))
print(sum(range(1, 101)))
```

```sortie
[0, 1, 2, 3, 4]
[2, 4, 6, 8, 10]
5050
```

`sum(range(1, 101))` suma los números del 1 al 100 sin crear ninguna lista.

## Errores frecuentes

### Un índice demasiado grande

```python norun
frutas = ["manzana", "pera"]
print(frutas[2])
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 2, in <module>
    print(frutas[2])
          ~~~~~~^^^
IndexError: list index out of range
```

La lista tiene 2 elementos, en los índices 0 y 1. El último es `frutas[-1]`.

### Guardar el resultado de sort

```python norun
notas = [12, 8, 15]
ordenadas = notas.sort()
print(ordenadas)
```

```text
None
```

`sort()` ordena en el sitio y devuelve `None`. Escribe `ordenadas = sorted(notas)`, o
llama a `notas.sort()` y luego usa `notas`.

### Quitar un elemento que no está

```python norun
frutas = ["manzana", "pera"]
frutas.remove("kiwi")
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 2, in <module>
    frutas.remove("kiwi")
    ~~~~~~~~~~~~~^^^^^^^^
ValueError: list.remove(x): x not in list
```

`"kiwi"` no está en la lista. Compruébalo antes: `if "kiwi" in frutas:`.

### Modificar una tupla

```python norun
punto = (3, 4)
punto[0] = 10
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 2, in <module>
    punto[0] = 10
    ~~~~~^^^
TypeError: 'tuple' object does not support item assignment
```

Una tupla es fija. Crea una nueva (`punto = (10, punto[1])`), o usa una lista si los
valores tienen que cambiar.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```python norun
a = ["x"]
b = a
b.append("y")
print(len(a))
```

> **Respuesta**
>
> `2`. `a` y `b` se refieren a la misma lista: añadir a través de `b` también modifica
> `a`.

**Pregunta 2:** ¿cuál es la diferencia entre `notas.sort()` y `sorted(notas)`?

> **Respuesta**
>
> `notas.sort()` modifica la lista y devuelve `None`. `sorted(notas)` devuelve una
> nueva lista ordenada y deja `notas` intacta.

**Pregunta 3:** muestra las tres mejores notas de `[12, 17, 9, 15, 19, 11]`.

> **Respuesta**
>
> ```python
> notas = [12, 17, 9, 15, 19, 11]
> print(sorted(notas, reverse=True)[:3])
> ```
>
> ```sortie
> [19, 17, 15]
> ```

## En resumen

- Una lista `[...]` guarda valores en orden; se lee con `lista[i]` y `lista[a:b]`.
- `append`, `insert`, `remove` y `pop` la modifican en el sitio.
- `sorted` devuelve una nueva lista ordenada; `sort` ordena en el sitio; `key` elige
  el criterio.
- `b = a` comparte la lista; `a.copy()` la copia.
- Una tupla `(...)` es una lista fija; `x, y = punto` la desempaqueta.
- `list(range(...))` convierte una serie de números en lista.

Siguiente: [diccionarios y conjuntos](dictionnaires.md). Compara con
[las colecciones en Ruby](../ruby/collections.md).
