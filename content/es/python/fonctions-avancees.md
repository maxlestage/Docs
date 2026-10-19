# Más allá de las funciones

Esta página muestra usos más avanzados de las funciones de Python: un número variable de argumentos, los docstrings, las funciones pasadas como valores, las `lambda` y la recursión.

> **Objetivos:**
>
> - sabrás aceptar un número variable de argumentos con `*args` y `**kwargs`;
> - sabrás escribir una `lambda` y dársela a `sorted`;
> - sabrás escribir una función recursiva con un caso base.

Este capítulo sigue a [las funciones](fonctions.md): volverás a encontrar `def`, los parámetros y `return`, esta vez para ir un poco más lejos.

## Un número variable de argumentos: *args y **kwargs

A veces no sabes de antemano cuántos valores vas a recibir. Una cuenta de restaurante
puede tener dos platos o diez. Los asteriscos permiten que una función acepte tantos
argumentos como haga falta.

Con un asterisco, `*args` recibe todos los argumentos **posicionales** de más, guardados en
una tupla.

```python
def suma(*numeros):
    total = 0
    for n in numeros:
        total += n
    return total

print(suma(1, 2))
print(suma(1, 2, 3, 4))
print(suma())
```

```sortie
3
10
0
```

Línea por línea:

- `*numeros` guarda todos los argumentos recibidos en una tupla: `(1, 2)`, luego
  `(1, 2, 3, 4)`, luego `()`;
- `total = 0` empieza desde cero;
- el bucle `for` suma cada número a `total`;
- `return total` devuelve la suma. Sin argumentos, el bucle no da ninguna vuelta: 0.

Con dos asteriscos, `**kwargs` (*keyword arguments*) recibe todos los argumentos **con nombre**
de más, guardados en un [diccionario](../debuter/glossaire.md#diccionario).

```python
def ficha(nombre, **datos):
    print(f"Ficha de {nombre}")
    for clave, valor in datos.items():
        print(f"  {clave}: {valor}")

ficha("Alice", edad=30, ciudad="Lima")
```

```sortie
Ficha de Alice
  edad: 30
  ciudad: Lima
```

Los nombres `args` y `kwargs` son una convención: solo cuentan los asteriscos. Los
diccionarios se explican en [diccionarios y conjuntos](dictionnaires.md).

## Documentar con un docstring

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Un **docstring** es un texto entre comillas triples, colocado justo debajo de la línea
`def`. Explica lo que hace la función. Los editores lo muestran cuando pasas el ratón
por encima del nombre, y `help()` lo enseña.

```python
def precio_con_iva(precio_sin_iva, tasa=0.2):
    """Devuelve el precio con impuestos incluidos, redondeado al céntimo."""
    return round(precio_sin_iva * (1 + tasa), 2)

print(precio_con_iva(10))
print(precio_con_iva.__doc__)
```

```sortie
12.0
Devuelve el precio con impuestos incluidos, redondeado al céntimo.
```

## Las funciones son valores

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

En Python, una función es un valor como cualquier otro. Sin paréntesis, su nombre
designa la función misma: puedes guardarla en una variable o pasarla a
otra función.

```python
def gritar(texto):
    return texto.upper() + "!"

def susurrar(texto):
    return texto.lower() + "..."

def hablar(texto, estilo):
    return estilo(texto)

print(hablar("Hola", gritar))
print(hablar("Hola", susurrar))
```

```sortie
HOLA!
hola...
```

`hablar` recibe una función (`estilo`) y la llama. Es muy útil para ordenar, por
ejemplo: `sorted` acepta una función que dice según qué ordenar.

```python
palabras = ["kiwi", "manzana", "fresa", "pera"]
print(sorted(palabras, key=len))
```

```sortie
['kiwi', 'pera', 'fresa', 'manzana']
```

## Las lambdas

A veces una función solo sirve una vez, para una tarea muy pequeña, como decirle a
`sorted` según qué ordenar. Darle un nombre con `def` sería pesado.

Una [lambda](../debuter/glossaire.md#lambda) es una pequeña función sin nombre, escrita
en una línea: `lambda parámetros: expresión`. Devuelve automáticamente el valor
de la expresión.

```python
cuadrado = lambda x: x * x
print(cuadrado(5))

alumnos = [("Alice", 7), ("Bob", 6), ("Chloé", 9)]
por_nota = sorted(alumnos, key=lambda alumno: alumno[1])
print(por_nota)
```

```sortie
25
[('Bob', 6), ('Alice', 7), ('Chloé', 9)]
```

Línea por línea:

- `cuadrado = lambda x: x * x` crea una pequeña función y la guarda en `cuadrado`;
- `cuadrado(5)` calcula `5 * 5` y devuelve 25;
- `alumnos` es una lista de parejas (nombre, nota);
- `sorted(..., key=lambda alumno: alumno[1])` mira la nota de cada pareja y ordena
  a los alumnos de la nota más baja a la más alta.

`key=lambda alumno: alumno[1]` significa «ordena según el segundo elemento». La ordenación
se explica en detalle en [listas y tuplas](collections.md).

> **Consejo:** una lambda sirve sobre todo como argumento de otra función. Para todo
> lo demás, un `def` normal con nombre es más claro.

**Te toca:** la lista `amigos = [("Lea", 25), ("Tom", 19), ("Inés", 31)]` contiene
nombres y edades. Muéstrala ordenada por edad, con `sorted` y una lambda.

> **Solución**
>
> ```python
> amigos = [("Lea", 25), ("Tom", 19), ("Inés", 31)]
> print(sorted(amigos, key=lambda amigo: amigo[1]))
> ```
>
> ```sortie
> [('Tom', 19), ('Lea', 25), ('Inés', 31)]
> ```

## Una función que se llama a sí misma

Algunos problemas contienen una versión más pequeña de sí mismos. Contar hacia atrás
desde 3 es decir «3» y luego contar hacia atrás desde 2. Una función puede seguir la
misma idea.

Una función puede llamarse a sí misma: es la
[recursión](../debuter/glossaire.md#recursion). Hace falta un **caso base**, si no,
no se detiene nunca.

```python
def cuenta_atras(n):
    if n == 0:
        print("¡Ya!")
        return
    print(n)
    cuenta_atras(n - 1)

cuenta_atras(3)
```

```sortie
3
2
1
¡Ya!
```

Línea por línea:

- `cuenta_atras(3)`: `n` vale 3, no es 0. La función muestra 3 y luego se llama a
  sí misma con 2;
- lo mismo con 2 y luego con 1: cada llamada muestra su número;
- con 0, el `if` es verdadero: la función muestra «¡Ya!» y `return` lo detiene todo.
  Es el caso base.

Un ejemplo clásico es el factorial: `5!` vale `5 × 4 × 3 × 2 × 1`. Dicho de otro modo,
`n!` vale `n × (n - 1)!`, y `1!` vale 1.

```python
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

print(factorial(5))
```

```sortie
120
```

> **Nota:** Python limita la profundidad de las llamadas (unas 1000). Para repeticiones
> muy largas, es mejor un bucle.

## Errores frecuentes

### Olvidar el caso base

```python norun
def cuenta_atras(n):
    print(n)
    cuenta_atras(n - 1)

cuenta_atras(3)
```

```text
3
2
1
0
-1
...
Traceback (most recent call last):
  File "/home/tu/main.py", line 5, in <module>
    cuenta_atras(3)
    ~~~~~~~~~~~~^^^
  File "/home/tu/main.py", line 3, in cuenta_atras
    cuenta_atras(n - 1)
    ~~~~~~~~~~~~^^^^^^^
  [Previous line repeated 996 more times]
RecursionError: maximum recursion depth exceeded
```

Sin caso base, la función se llama a sí misma sin fin: 3, 2, 1, 0, -1… Python la
detiene tras unas 1000 llamadas (*maximum recursion depth exceeded*, «profundidad
máxima de recursión superada»). Añade el caso base antes de la llamada: `if n == 0:`
seguido de `return`.

### Llamar a la función en lugar de pasarla

```python norun
palabras = ["kiwi", "manzana", "fresa", "pera"]
print(sorted(palabras, key=len()))
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 2, in <module>
    print(sorted(palabras, key=len()))
                               ~~~^^
TypeError: len() takes exactly one argument (0 given)
```

Con los paréntesis, `len()` **llama** a `len` enseguida, sin argumento. Para dar la
función misma, escribe su nombre sin paréntesis: `key=len`.

### Dar argumentos con nombre a *args

```python norun
def suma(*numeros):
    total = 0
    for n in numeros:
        total += n
    return total

print(suma(a=1, b=2))
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 7, in <module>
    print(suma(a=1, b=2))
          ~~~~^^^^^^^^^^
TypeError: suma() got an unexpected keyword argument 'a'
```

`*numeros` solo recibe los argumentos **posicionales** (*unexpected keyword
argument*: «argumento con nombre inesperado»). Escribe `suma(1, 2)`, o añade un
parámetro `**kwargs` para aceptar los argumentos con nombre.

## Ponte a prueba

**Pregunta 1:** ¿qué recibe `args` en `def f(*args)` llamada con `f(1, 2, 3)`?

> **Respuesta**
>
> La tupla `(1, 2, 3)`. El asterisco guarda todos los argumentos posicionales en una tupla.

**Pregunta 2:** reescribe `def triple(x): return x * 3` con una lambda.

> **Respuesta**
>
> ```python
> triple = lambda x: x * 3
> print(triple(4))
> ```
>
> ```sortie
> 12
> ```

**Pregunta 3:** ¿por qué una función recursiva necesita un caso base?

> **Respuesta**
>
> Sin caso base, se llama a sí misma sin fin. Python acaba deteniéndola con un
> `RecursionError`.

## En resumen

- `*args` guarda los argumentos posicionales de más en una tupla; `**kwargs` guarda
  los argumentos con nombre de más en un diccionario.
- Un docstring, justo debajo de la línea `def`, explica lo que hace la función.
- Una función es un valor: se puede pasar a otra, o escribir como `lambda`.
- Una función recursiva se llama a sí misma y siempre tiene un caso base.

Siguiente: [trabajar con texto](texte.md).
