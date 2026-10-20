# Iteradores y generadores

Esta página explica lo que pasa de verdad en un bucle `for`, cómo escribir tus propios generadores con `yield` y cómo el módulo `itertools` trata secuencias de valores, incluso infinitas.

> **Objetivos:**
>
> - sabrás distinguir un iterable de un iterador;
> - sabrás usar `iter` y `next`, y reconocer `StopIteration`;
> - sabrás escribir un generador con `yield` y encadenar varios generadores;
> - sabrás usar las herramientas principales de `itertools`;
> - sabrás qué es la evaluación perezosa.

## Iterable e iterador

Estas dos palabras se parecen, pero no quieren decir lo mismo:

- un **iterable** es todo lo que se puede recorrer con `for`: una lista, una cadena, un
  diccionario, un `range`, un archivo…;
- un [iterador](../debuter/glossaire.md#iterador) es el objeto que hace el recorrido.
  Recuerda **por dónde va** y te da el elemento siguiente cuando se lo pides.

Piensa en un libro: el libro es el iterable; el **marcapáginas** es el iterador. Varios
lectores pueden leer el mismo libro, cada uno con su marcapáginas.

## iter y next

`iter(iterable)` crea un iterador; `next(iterador)` da el elemento siguiente.

```python
frutas = ["manzana", "pera", "kiwi"]
it = iter(frutas)
print(next(it))
print(next(it))
print(next(it))
```

```sortie
manzana
pera
kiwi
```

Cuando ya no queda nada, `next` lanza la excepción `StopIteration`. También puedes dar
a `next` un valor por defecto, que se devuelve al final en lugar del error:

```python
it = iter(["manzana"])
print(next(it))
print(next(it, "fin"))
```

```sortie
manzana
fin
```

### Lo que hace de verdad for

Un bucle `for` no es más que un atajo: llama a `iter`, luego a `next` una y otra vez, y
se detiene limpiamente en el primer `StopIteration`. Estos dos códigos hacen lo mismo:

```python
for letra in "abc":
    print(letra)

it = iter("abc")
while True:
    try:
        letra = next(it)
    except StopIteration:
        break
    print(letra)
```

```sortie
a
b
c
a
b
c
```

### Un iterador se gasta

Un iterador nunca vuelve atrás. Una vez que llega al final, está vacío.

```python
it = iter([1, 2, 3])
print(list(it))
print(list(it))
```

```sortie
[1, 2, 3]
[]
```

La lista, en cambio, se puede recorrer tantas veces como quieras: cada `for` crea un
iterador nuevo.

## Escribir un generador con yield

Un **generador** es la forma más sencilla de crear un iterador. Es una función que
contiene `yield` («producir») en lugar de `return`.

```python
def cuenta_atras(n):
    while n > 0:
        yield n
        n -= 1
    print("¡Despegue!")

for x in cuenta_atras(3):
    print(x)
```

```sortie
3
2
1
¡Despegue!
```

¿Qué ocurre?

1. Llamar a `cuenta_atras(3)` **no ejecuta nada**: devuelve un generador, en pausa al
   principio de la función.
2. Cada `next` (aquí, hecho por `for`) hace avanzar la función hasta el siguiente
   `yield`. Se entrega el valor y luego la función **se pone en pausa**, conservando sus
   variables.
3. Cuando la función termina, el generador lanza `StopIteration` y el bucle se detiene.

Se ve claramente llamando a `next` a mano:

```python
def etapas():
    print("inicio")
    yield 1
    print("mitad")
    yield 2

g = etapas()
print("generador creado")
print(next(g))
print(next(g))
```

```sortie
generador creado
inicio
1
mitad
2
```

> **Recuerda:** `return` devuelve un valor y **termina** la función. `yield` devuelve un
> valor y **pone en pausa** la función, que seguirá en el mismo sitio.

## Las cadenas de generadores

Los generadores se pueden conectar unos con otros, como tuberías: cada uno recibe los
valores del anterior, los transforma y los pasa al siguiente. Los valores circulan
**de uno en uno** por toda la cadena.

```python
lineas = [
    "alice,15",
    "",
    "bob,8",
    "# comentario",
    "chloé,12",
]

def no_vacias(lineas):
    for linea in lineas:
        if linea and not linea.startswith("#"):
            yield linea

def separar(lineas):
    for linea in lineas:
        nombre, nota = linea.split(",")
        yield nombre, int(nota)

def aprobados(alumnos):
    for nombre, nota in alumnos:
        if nota >= 10:
            yield nombre

for nombre in aprobados(separar(no_vacias(lineas))):
    print(nombre)
```

```sortie
alice
chloé
```

Cada etapa es una pequeña función fácil de probar. Y si `lineas` fuera un archivo de un
millón de líneas, solo habría una línea en memoria a la vez.

### yield from

`yield from` entrega, de una en una, todos los valores de otro iterable:

```python
def todo(*listas):
    for lista in listas:
        yield from lista

print(list(todo([1, 2], [3], [4, 5])))
```

```sortie
[1, 2, 3, 4, 5]
```

## La evaluación perezosa

Un generador es **perezoso**: solo calcula un valor cuando se lo pides. Esto permite
incluso escribir una secuencia **infinita**:

```python
def numeros_pares():
    n = 0
    while True:
        yield n
        n += 2

gen = numeros_pares()
for _ in range(5):
    print(next(gen), end=" ")
print()
```

```sortie
0 2 4 6 8 
```

`while True` no bloquea el programa: la función se detiene en cada `yield` y espera a
que le pidas el siguiente. En cambio, cuidado con `list(numeros_pares())`: no
terminaría nunca.

Otro ejemplo, la sucesión de Fibonacci, en la que cada número es la suma de los dos
anteriores:

```python
def fibonacci():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b

for n in fibonacci():
    if n > 100:
        break
    print(n, end=" ")
print()
```

```sortie
0 1 1 2 3 5 8 13 21 34 55 89 
```

## El módulo itertools

El módulo `itertools` de la biblioteca estándar ofrece iteradores listos para usar.
Todos son perezosos.

### count e islice

`count(inicio, paso)` cuenta hasta el infinito. `islice(iterable, n)` toma los `n`
primeros elementos, como `[:n]` en una lista, pero para cualquier iterador.

```python
from itertools import count, islice

print(list(islice(count(10, 5), 4)))

def cuadrados():
    for n in count(1):
        yield n * n

print(list(islice(cuadrados(), 6)))
print(list(islice("abcdefgh", 2, 6)))
```

```sortie
[10, 15, 20, 25]
[1, 4, 9, 16, 25, 36]
['c', 'd', 'e', 'f']
```

### chain

`chain` pone varios iterables uno detrás de otro, sin crear una lista nueva.

```python
from itertools import chain

desayuno = ["café", "cruasán"]
comida = ["ensalada"]
for plato in chain(desayuno, comida, ["manzana"]):
    print(plato)
```

```sortie
café
cruasán
ensalada
manzana
```

### groupby

`groupby` agrupa los elementos **consecutivos** que tienen la misma clave. Por eso hay
que **ordenar** antes según esa misma clave.

```python
from itertools import groupby

palabras = ["avión", "bus", "arco", "barco", "casa"]
palabras_ordenadas = sorted(palabras, key=lambda p: p[0])
for letra, grupo in groupby(palabras_ordenadas, key=lambda p: p[0]):
    print(letra, list(grupo))
```

```sortie
a ['avión', 'arco']
b ['bus', 'barco']
c ['casa']
```

### product

`product` da todas las combinaciones, como bucles `for` anidados.

```python
from itertools import product

for talla, color in product(["S", "M"], ["rojo", "azul"]):
    print(talla, color)
print(len(list(product("01", repeat=3))))
```

```sortie
S rojo
S azul
M rojo
M azul
8
```

`repeat=3` repite el mismo iterable tres veces: hay 8 números binarios de tres cifras.

### accumulate

`accumulate` da los totales sucesivos: cada valor es la suma de todos los anteriores.

```python
from itertools import accumulate

gastos = [12, 5, 30, 8]
print(list(accumulate(gastos)))
print(list(accumulate(gastos, max)))
```

```sortie
[12, 17, 47, 55]
[12, 12, 30, 30]
```

Con `max`, obtienes el número más grande visto hasta ahora.

| Herramienta | Función |
|-------------|---------|
| `count(inicio, paso)` | cuenta sin fin |
| `islice(it, n)` / `islice(it, inicio, fin)` | recorta un iterador |
| `chain(a, b, …)` | pone iterables uno detrás de otro |
| `groupby(it, key)` | agrupa los elementos consecutivos |
| `product(a, b)` | todas las combinaciones |
| `accumulate(it)` | totales acumulados |

## Errores frecuentes

### Llamar a next sobre una lista

```python norun
frutas = ["manzana", "pera"]
print(next(frutas))
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(next(frutas))
          ~~~~^^^^^^^^
TypeError: 'list' object is not an iterator
```

Una lista es iterable, pero no es un iterador. Crea primero el marcapáginas:
`next(iter(frutas))`.

### Ir más allá del final

```python norun
it = iter(["manzana"])
print(next(it))
print(next(it))
```

```text
manzana
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    print(next(it))
          ~~~~^^^^
StopIteration
```

El iterador está vacío después de `manzana`. Usa un bucle `for`, que se detiene solo, o
un valor por defecto: `next(it, None)`.

### Pedir la longitud de un generador

```python norun
def pares(n):
    for i in range(0, n, 2):
        yield i

print(len(pares(10)))
```

```text
Traceback (most recent call last):
  File "main.py", line 5, in <module>
    print(len(pares(10)))
          ~~~^^^^^^^^^^^
TypeError: object of type 'generator' has no len()
```

Un generador no sabe cuántos valores va a producir antes de haberlos producido.
Conviértelo en lista: `len(list(pares(10)))`, o cuenta con `sum(1 for _ in pares(10))`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```python norun
def letras():
    yield "a"
    yield "b"

g = letras()
print(list(g), list(g))
```

> **Respuesta**
>
> `['a', 'b'] []`. El primer `list(g)` vacía el generador; el segundo ya no encuentra
> nada.

**Pregunta 2:** ¿por qué `groupby` da dos grupos `a` para `["avión", "bus", "arco"]`
si no ordenas?

> **Respuesta**
>
> Porque `groupby` solo agrupa los elementos **que van seguidos**. `avión` y `arco`
> están separados por `bus`: forman dos grupos. Ordena primero con la misma clave.

**Pregunta 3:** escribe un generador infinito `multiplos(k)` y luego muestra los 4
primeros múltiplos de 7.

> **Respuesta**
>
> ```python
> from itertools import islice
>
> def multiplos(k):
>     n = k
>     while True:
>         yield n
>         n += k
>
> print(list(islice(multiplos(7), 4)))
> ```
>
> ```sortie
> [7, 14, 21, 28]
> ```

## En resumen

- Un iterable se recorre; un iterador hace el recorrido y recuerda su posición.
- `iter` crea un iterador, `next` da el elemento siguiente, `StopIteration` marca el
  final.
- Una función con `yield` es un generador: se pone en pausa en cada valor.
- Los generadores se encadenan y pueden ser infinitos: son perezosos.
- `itertools` ofrece `count`, `islice`, `chain`, `groupby`, `product`, `accumulate`.

Siguiente: [las colecciones avanzadas](collections-avancees.md)
