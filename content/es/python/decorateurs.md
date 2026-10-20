# Funciones avanzadas y decoradores

Esta página muestra que las funciones de Python son valores como los demás, lo que permite escribir closures, decoradores y usar las herramientas del módulo `functools`.

> **Objetivos:**
>
> - sabrás guardar una función en una variable y pasarla a otra función;
> - sabrás qué es una closure;
> - sabrás escribir un decorador paso a paso, con `functools.wraps`;
> - sabrás escribir un decorador que recibe argumentos;
> - sabrás usar `lru_cache`, `partial`, `map`, `filter` y `reduce`.

## Las funciones son objetos

En Python, una [función](../debuter/glossaire.md#fonction) es un valor, como un número
o una cadena. Sin paréntesis, `saludar` designa **la función misma**; con paréntesis,
`saludar()` **la ejecuta**.

```python
def saludar(nombre):
    return f"Hola {nombre}"

decir = saludar
print(decir("Alice"))
print(saludar.__name__)
print(type(saludar))
```

```sortie
Hola Alice
saludar
<class 'function'>
```

### Pasar una función como argumento

Como una función es un valor, puedes dársela a otra función. Ya lo has hecho con
`sorted(..., key=len)`.

```python
def aplicar(funcion, valores):
    return [funcion(v) for v in valores]

def doble(x):
    return x * 2

print(aplicar(doble, [1, 2, 3]))
print(aplicar(str.upper, ["a", "b"]))
print(aplicar(lambda x: x ** 2, [1, 2, 3]))
```

```sortie
[2, 4, 6]
['A', 'B']
[1, 4, 9]
```

`lambda x: x ** 2` es una [lambda](../debuter/glossaire.md#lambda): una pequeña
función sin nombre, escrita en una línea, que devuelve una sola expresión.

### Devolver una función

Una función también puede **fabricar** y devolver otra función:

```python
def crear_multiplicador(factor):
    def multiplicar(x):
        return x * factor
    return multiplicar

triple = crear_multiplicador(3)
print(triple(5))
print(crear_multiplicador(10)(4))
```

```sortie
15
40
```

## Las closures

En el ejemplo anterior, `multiplicar` usa `factor`, una variable de la función que la
creó. Incluso cuando `crear_multiplicador` ha terminado, `triple` **recuerda** que
`factor` vale 3. Una función que se lleva así variables consigo se llama
[closure](../debuter/glossaire.md#closure) (clausura).

Es una mochila: la función interior se va con las variables que necesita.

Para **modificar** una variable que se lleva, hay que decirlo con `nonlocal`:

```python
def crear_contador():
    total = 0
    def contar():
        nonlocal total
        total += 1
        return total
    return contar

a = crear_contador()
b = crear_contador()
print(a(), a(), a())
print(b())
```

```sortie
1 2 3
1
```

Cada llamada a `crear_contador` crea una mochila nueva: `a` y `b` tienen cada una su
propio `total`.

## Escribir un decorador, paso a paso

Un **decorador** es una función que recibe una función y devuelve una versión mejorada
de ella. Es papel de regalo: el regalo sigue siendo el mismo, pero se añade algo
alrededor.

### Paso 1: una función que envuelve

Escribamos un decorador que cuenta las llamadas. Recibe `funcion`, crea una función
`envoltura` que hace un trabajo extra y luego llama a `funcion`, y devuelve
`envoltura`.

```python
def contar_llamadas(funcion):
    def envoltura(*args, **kwargs):
        envoltura.llamadas += 1
        return funcion(*args, **kwargs)
    envoltura.llamadas = 0
    return envoltura

def sumar(a, b):
    return a + b

sumar = contar_llamadas(sumar)
print(sumar(2, 3))
print(sumar(10, 1))
print(sumar.llamadas)
```

```sortie
5
11
2
```

`*args, **kwargs` significa «todos los argumentos, sean cuales sean»: la envoltura los
pasa tal cual. Así funciona con cualquier función.

### Paso 2: la sintaxis @

`sumar = contar_llamadas(sumar)` es tan habitual que Python tiene un atajo: se escribe
`@contar_llamadas` justo encima de la definición.

```python
def contar_llamadas(funcion):
    def envoltura(*args, **kwargs):
        envoltura.llamadas += 1
        return funcion(*args, **kwargs)
    envoltura.llamadas = 0
    return envoltura

@contar_llamadas
def saludar(nombre):
    return f"Hola {nombre}"

saludar("Alice")
saludar("Bob")
saludar("Chloé")
print(saludar.llamadas)
```

```sortie
3
```

> **Recuerda:** `@decorador` encima de `def f` quiere decir exactamente
> `f = decorador(f)`. Nada más.

### Un decorador de registro

Otro uso típico: mostrar cada llamada y su resultado, para depurar.

```python
def registrar(funcion):
    def envoltura(*args, **kwargs):
        print(f"→ {funcion.__name__}{args}")
        resultado = funcion(*args, **kwargs)
        print(f"← {resultado!r}")
        return resultado
    return envoltura

@registrar
def multiplicar(a, b):
    return a * b

multiplicar(6, 7)
```

```sortie
→ multiplicar(6, 7)
← 42
```

### Un decorador de cronometraje

El mismo esquema sirve para medir la duración de una función con
`time.perf_counter()`. La duración cambia en cada ejecución, así que la salida de abajo
es solo un ejemplo:

```python norun
import time

def cronometrar(funcion):
    def envoltura(*args, **kwargs):
        inicio = time.perf_counter()
        resultado = funcion(*args, **kwargs)
        duracion = time.perf_counter() - inicio
        print(f"{funcion.__name__}: {duracion:.3f} s")
        return resultado
    return envoltura

@cronometrar
def calculo_largo():
    return sum(range(10_000_000))

calculo_largo()
```

```text
calculo_largo: 0.142 s
```

## functools.wraps

Un pequeño defecto: después de decorarla, la función se llama `envoltura`. Su nombre y
su documentación originales se pierden.

```python
def decorar(funcion):
    def envoltura(*args, **kwargs):
        return funcion(*args, **kwargs)
    return envoltura

@decorar
def saludar():
    """Dice hola."""

print(saludar.__name__, saludar.__doc__)
```

```sortie
envoltura None
```

`@functools.wraps(funcion)`, puesto sobre la envoltura, copia el nombre, la
documentación y el resto de la información de la función original. Úsalo **siempre**.

```python
import functools

def decorar(funcion):
    @functools.wraps(funcion)
    def envoltura(*args, **kwargs):
        return funcion(*args, **kwargs)
    return envoltura

@decorar
def saludar():
    """Dice hola."""

print(saludar.__name__, saludar.__doc__)
```

```sortie
saludar Dice hola.
```

## Un decorador con argumentos

Para escribir `@repetir(3)`, hace falta un nivel más: `repetir(3)` debe **devolver un
decorador**. Se obtienen tres funciones anidadas:

1. `repetir(veces)` recibe el argumento y devuelve el decorador;
2. `decorador(funcion)` recibe la función y devuelve la envoltura;
3. `envoltura(*args)` hace el trabajo.

```python
import functools

def repetir(veces):
    def decorador(funcion):
        @functools.wraps(funcion)
        def envoltura(*args, **kwargs):
            for _ in range(veces):
                funcion(*args, **kwargs)
        return envoltura
    return decorador

@repetir(3)
def gritar(palabra):
    print(palabra.upper())

gritar("hip")
```

```sortie
HIP
HIP
HIP
```

`@repetir(3)` equivale a `gritar = repetir(3)(gritar)`.

## lru_cache: memorizar los resultados

`functools.lru_cache` es un decorador ya preparado: guarda en memoria los resultados
ya calculados. Si se vuelve a llamar a la función con los mismos argumentos, el
resultado se devuelve al instante, sin volver a calcularlo.

Es muy eficaz para una función [recursiva](../debuter/glossaire.md#recursion) como
Fibonacci, que recalcula sin parar los mismos valores:

```python
from functools import lru_cache

llamadas = 0

@lru_cache(maxsize=None)
def fib(n):
    global llamadas
    llamadas += 1
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(80))
print(llamadas)
```

```sortie
23416728348467685
81
```

Solo 81 cálculos, uno por valor de `n`. Sin la caché, harían falta miles de millones
de miles de millones.

> **Atención:** la caché solo funciona si los argumentos son *hashables*: números,
> cadenas, tuplas… pero no listas ni diccionarios.

## partial: fijar argumentos

`functools.partial` crea una función nueva en la que algunos argumentos **ya están
rellenados**.

```python
from functools import partial

def potencia(base, exponente):
    return base ** exponente

cuadrado = partial(potencia, exponente=2)
cubo = partial(potencia, exponente=3)
print(cuadrado(5), cubo(2))

binario_a_int = partial(int, base=2)
print(binario_a_int("1010"))
```

```sortie
25 8
10
```

## ¿map, filter, reduce o comprensiones?

Tres funciones clásicas aplican una función a una colección:

- `map(f, valores)` aplica `f` a cada valor;
- `filter(f, valores)` conserva los valores para los que `f` devuelve verdadero;
- `reduce(f, valores)` (del módulo `functools`) combina los valores de dos en dos hasta
  que solo queda uno.

```python
from functools import reduce

numeros = [1, 2, 3, 4, 5]
print(list(map(lambda x: x * 10, numeros)))
print(list(filter(lambda x: x % 2 == 0, numeros)))
print(reduce(lambda acc, x: acc * x, numeros))
```

```sortie
[10, 20, 30, 40, 50]
[2, 4]
120
```

`map` y `filter` devuelven iteradores perezosos: se convierten en lista con `list`. En
Python, las [comprensiones](comprehensions.md) suelen hacer lo mismo de forma más
legible:

```python
numeros = [1, 2, 3, 4, 5]
print([x * 10 for x in numeros])
print([x for x in numeros if x % 2 == 0])
print(sum(numeros), max(numeros))
```

```sortie
[10, 20, 30, 40, 50]
[2, 4]
15 5
```

| Necesidad | Lo más legible |
|-----------|----------------|
| transformar con una función existente | `map(str, numeros)` o una comprensión |
| transformar con un cálculo | comprensión |
| filtrar | comprensión con `if` |
| sumar, máximo, mínimo | `sum`, `max`, `min` |
| otra combinación | `reduce`, o un bucle |

## Errores frecuentes

### Olvidar devolver la envoltura

```python norun
def anunciar(funcion):
    def envoltura():
        print("¡Llamada!")
        funcion()

@anunciar
def saludar():
    print("Hola")

saludar()
```

```text
Traceback (most recent call last):
  File "main.py", line 10, in <module>
    saludar()
    ~~~~~~~^^
TypeError: 'NoneType' object is not callable
```

`anunciar` no devuelve nada, así que `saludar` vale `None`. Añade `return envoltura` al
final del decorador.

### Modificar una variable de closure sin nonlocal

```python norun
def crear_contador():
    total = 0
    def sumar_uno():
        total += 1
        return total
    return sumar_uno

contar = crear_contador()
print(contar())
```

```text
Traceback (most recent call last):
  File "main.py", line 9, in <module>
    print(contar())
          ~~~~~~^^
  File "main.py", line 4, in sumar_uno
    total += 1
    ^^^^^
UnboundLocalError: cannot access local variable 'total' where it is not associated with a value
```

`total += 1` convierte `total` en una variable local de `sumar_uno`, que todavía no
tiene valor. Añade `nonlocal total` al principio de `sumar_uno`.

### Olvidar los paréntesis de un decorador con argumentos

```python norun
def repetir(veces):
    def decorador(funcion):
        def envoltura():
            for _ in range(veces):
                funcion()
        return envoltura
    return decorador

@repetir
def saludar():
    print("Hola")

saludar()
```

```text
Traceback (most recent call last):
  File "main.py", line 13, in <module>
    saludar()
    ~~~~~~~^^
TypeError: repetir.<locals>.decorador() missing 1 required positional argument: 'funcion'
```

Sin paréntesis, `saludar` se ha dado como valor de `veces`, y `saludar` se ha
convertido en el propio decorador. Escribe `@repetir(2)`.

### Poner en caché una función que recibe una lista

```python norun
from functools import lru_cache

@lru_cache
def sumar(numeros):
    return sum(numeros)

print(sumar([1, 2, 3]))
```

```text
Traceback (most recent call last):
  File "main.py", line 7, in <module>
    print(sumar([1, 2, 3]))
          ~~~~~^^^^^^^^^^^
TypeError: unhashable type: 'list'
```

La caché guarda los argumentos como claves de un diccionario: una lista no puede serlo.
Pasa una tupla: `sumar((1, 2, 3))`.

## Ponte a prueba

**Pregunta 1:** ¿qué diferencia hay entre `f` y `f()`?

> **Respuesta**
>
> `f` es la función misma, un valor que se puede guardar o pasar. `f()` la ejecuta y da
> su resultado.

**Pregunta 2:** ¿qué hace este código?

```python norun
@registrar
def cuadrado(x):
    return x * x
```

> **Respuesta**
>
> Define `cuadrado` y luego la sustituye por `registrar(cuadrado)`: es exactamente
> `cuadrado = registrar(cuadrado)`.

**Pregunta 3:** escribe un decorador `mayusculas` que ponga en mayúsculas el texto que
devuelve una función.

> **Respuesta**
>
> ```python
> import functools
>
> def mayusculas(funcion):
>     @functools.wraps(funcion)
>     def envoltura(*args, **kwargs):
>         return funcion(*args, **kwargs).upper()
>     return envoltura
>
> @mayusculas
> def saludar(nombre):
>     return f"hola {nombre}"
>
> print(saludar("zoé"))
> ```
>
> ```sortie
> HOLA ZOÉ
> ```

## En resumen

- Una función es un valor: se puede guardar, pasar, devolver.
- Una closure recuerda las variables de la función que la creó; `nonlocal` permite
  modificarlas.
- Un decorador recibe una función y devuelve una envoltura; `@d` quiere decir
  `f = d(f)`.
- Pon siempre `@functools.wraps(funcion)` sobre la envoltura.
- `lru_cache` memoriza los resultados, `partial` fija argumentos.
- Prefiere las comprensiones a `map` y `filter` cuando son más legibles.

Siguiente: [gestionar los errores](erreurs.md)
