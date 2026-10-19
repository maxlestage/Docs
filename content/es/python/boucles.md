# Los bucles

Esta página explica cómo un programa de Python repite acciones con `while` y `for`, y recorre listas con `range`, `enumerate` y `zip`.

> **Objetivos:**
>
> - sabrás repetir código con `while` y `for`;
> - sabrás contar con `range`;
> - sabrás detener un bucle con `break` o saltarte una vuelta con `continue`;
> - sabrás recorrer una lista con `enumerate` y `zip`.

Este capítulo sigue a [las condiciones](conditions-boucles.md): volverás a encontrar `if` y las comparaciones, esta vez dentro de los bucles.

## El bucle while

Imagina que te piden mostrar los números del 1 al 100. Escribir 100 `print` sería largo
y pesado. Un bucle hace ese trabajo repetitivo por ti.

Un [bucle](../debuter/glossaire.md#bucle) repite código. `while` («mientras»)
repite su bloque **mientras** la condición sea verdadera.

```python
cuenta = 3
while cuenta > 0:
    print(cuenta)
    cuenta = cuenta - 1
print("¡Despegue!")
```

```sortie
3
2
1
¡Despegue!
```

Línea por línea:

- `cuenta = 3`: la variable vale 3 al principio;
- `while cuenta > 0:`: antes de cada vuelta, Python hace la pregunta. `3 > 0` es
  verdadero: entra en el bloque;
- el bloque muestra `cuenta` y luego le resta 1: pasa a valer 2, luego 1, luego 0;
- cuando `cuenta` vale 0, `0 > 0` es falso: Python sale del bucle y pasa al último
  `print`.

En cada vuelta, Python comprueba `cuenta > 0`. Cuando `cuenta` vale 0, la condición es
falsa: el bucle se detiene.

> **Atención:** si la condición nunca pasa a ser falsa, el bucle gira para
> siempre. Comprueba que alguna variable cambia en cada vuelta. Para detener un programa
> bloqueado, pulsa Ctrl+C en el terminal.

**Te toca:** escribe un programa que muestre los números del 1 al 5 con un bucle
`while` y luego «¡Listo!».

> **Solución**
>
> ```python
> n = 1
> while n <= 5:
>     print(n)
>     n = n + 1
> print("¡Listo!")
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> ¡Listo!
> ```

## El bucle for

Muy a menudo tienes una lista de cosas que tratar una a una: cada artículo de la lista
de la compra, cada alumno de una clase. `for` está hecho para eso.

`for` recorre los elementos de una colección, uno a uno. Es el bucle más
usado en Python.

```python
frutas = ["manzana", "plátano", "cereza"]
for fruta in frutas:
    print(f"Me gusta la fruta: {fruta}.")
```

```sortie
Me gusta la fruta: manzana.
Me gusta la fruta: plátano.
Me gusta la fruta: cereza.
```

Línea por línea:

- `frutas` contiene una lista de tres textos;
- `for fruta in frutas:` toma el primer elemento, `"manzana"`, y lo guarda en `fruta`;
- el bloque muestra la frase con `"manzana"`. Luego Python vuelve al `for`, que guarda
  `"plátano"` en `fruta`, y así sucesivamente;
- después de `"cereza"` no quedan elementos: el bucle se detiene.

En cada vuelta, la variable `fruta` recibe el siguiente elemento de la lista. Cada pasada
se llama [iteración](../debuter/glossaire.md#iteracion).

`for` también recorre un texto, letra a letra:

```python
for letra in "Sí":
    print(letra)
```

```sortie
S
í
```

### Contar con range

`range` produce una serie de números. Es perfecto para repetir una acción un número
exacto de veces.

- `range(5)`: de 0 a 4 (5 números, **sin** el 5);
- `range(1, 6)`: de 1 a 5;
- `range(0, 10, 2)`: de 0 a 8, de 2 en 2.

```python
for i in range(3):
    print("Vuelta", i)

print(list(range(1, 6)))
print(list(range(0, 10, 2)))
print(list(range(10, 0, -3)))
```

```sortie
Vuelta 0
Vuelta 1
Vuelta 2
[1, 2, 3, 4, 5]
[0, 2, 4, 6, 8]
[10, 7, 4, 1]
```

`list(...)` convierte el `range` en una lista, para ver todos sus números de golpe. Un paso
negativo (`-3`) cuenta hacia atrás.

> **Recuerda:** el final de un `range` siempre queda **excluido**. `range(1, 6)` se detiene
> en 5.

**Te toca:** escribe un programa que muestre la tabla del 7, de «7 x 1 = 7» a
«7 x 5 = 35», una línea por cálculo.

> **Solución**
>
> ```python
> for i in range(1, 6):
>     print("7 x", i, "=", 7 * i)
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

## break y continue

A veces quieres cambiar el rumbo de un bucle sobre la marcha: dejar de buscar en cuanto
encuentras algo, o ignorar algunos elementos. Para eso sirven estas dos palabras.

- `break` sale del bucle inmediatamente;
- `continue` se salta el resto de la vuelta y pasa a la siguiente.

```python
for n in range(1, 10):
    if n % 2 == 0:
        continue
    if n > 7:
        break
    print(n)
```

```sortie
1
3
5
7
```

Línea por línea:

- `for n in range(1, 10):` da a `n` los valores 1, 2, 3… hasta 9;
- si `n` es par (el resto de dividir entre 2 es 0), `continue` pasa directamente a la
  vuelta siguiente: no se llega al `print`;
- si `n` pasa de 7, `break` detiene todo el bucle: el 9 nunca se muestra;
- si no, `print(n)` muestra el número.

Los números pares se saltan (`continue`). Cuando `n` pasa de 7, el bucle se detiene
(`break`).

### else en un bucle

Un bucle puede tener un `else`. Se ejecuta si el bucle llega **hasta el final**, sin
`break`. Es útil para una búsqueda: «no he encontrado nada».

```python
numeros = [3, 7, 11, 15]
for n in numeros:
    if n % 2 == 0:
        print(f"Primer número par: {n}")
        break
else:
    print("Ningún número par.")
```

```sortie
Ningún número par.
```

## enumerate: el número y el elemento

A veces quieres numerar lo que recorres, como una clasificación o los pasos de una
receta.

Para tener a la vez la posición y el elemento, usa `enumerate`. En cada vuelta,
da dos valores: el número (el [índice](../debuter/glossaire.md#indice)) y
el elemento.

```python
podio = ["Alice", "Bob", "Chloé"]
for puesto, nombre in enumerate(podio, start=1):
    print(f"{puesto}. {nombre}")
```

```sortie
1. Alice
2. Bob
3. Chloé
```

Línea por línea:

- `podio` contiene tres nombres, en orden de llegada;
- en cada vuelta, `enumerate` da una pareja: primero `1` y `"Alice"`, luego `2` y
  `"Bob"`, luego `3` y `"Chloé"`;
- `puesto, nombre` guarda el número en `puesto` y el nombre en `nombre`;
- `start=1` hace que la numeración empiece en 1.

Sin `start=1`, la numeración empieza en 0.

## zip: recorrer dos listas a la vez

A veces dos listas van juntas: los nombres por un lado y sus notas por otro, en el
mismo orden.

`zip` avanza por varias listas al mismo tiempo, como una cremallera que
une dos lados, diente a diente.

```python
nombres = ["Alice", "Bob", "Chloé"]
notas = [8, 6, 9]
for nombre, nota in zip(nombres, notas):
    print(f"{nombre}: {nota}/10")
```

```sortie
Alice: 8/10
Bob: 6/10
Chloé: 9/10
```

Línea por línea:

- `nombres` y `notas` están en el mismo orden: la primera nota es la de Alice;
- `zip` toma el primer elemento de cada lista, luego el segundo, luego el tercero;
- en cada vuelta, `nombre` recibe un nombre y `nota` la nota que le corresponde.

> **Nota:** `zip` se detiene al final de la lista más corta.

## Errores frecuentes

### Recorrer un número con for

```python norun
for i in 5:
    print(i)
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 1, in <module>
    for i in 5:
             ^
TypeError: 'int' object is not iterable
```

No se puede recorrer un número (*not iterable*). Para repetir 5 veces, escribe
`for i in range(5):`.

### Olvidar enumerate

```python norun
podio = ["Alice", "Bob", "Chloé"]
for puesto, nombre in podio:
    print(f"{puesto}. {nombre}")
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 2, in <module>
    for puesto, nombre in podio:
        ^^^^^^^^^^^^^^
ValueError: too many values to unpack (expected 2)
```

Sin `enumerate`, cada vuelta da un solo elemento, el texto `"Alice"`. Python intenta
repartirlo en dos variables y encuentra demasiadas letras (*too many values*). Escribe
`for puesto, nombre in enumerate(podio, start=1):`.

### break fuera del bucle

```python norun
for n in [3, 8, 11]:
    if n % 2 == 0:
        print("Número par:", n)
break
```

```text
  File "/home/tu/main.py", line 4
    break
    ^^^^^
SyntaxError: 'break' outside loop
```

`break` no tiene sangría: ya no está en el bloque del `for` (*outside loop*). Ponlo
debajo del `print`, con la misma sangría, para que forme parte del bloque del `if`.

## Ponte a prueba

**Pregunta 1:** ¿cuántas veces se muestra «hola» con `for i in range(2, 8):`?

> **Respuesta**
>
> 6 veces: `i` toma los valores 2, 3, 4, 5, 6 y 7. El final (8) queda excluido.

**Pregunta 2:** escribe un bucle que muestre los múltiplos de 3 del 3 al 15, en una
línea.

> **Respuesta**
>
> ```python
> for n in range(3, 16, 3):
>     print(n, end=" ")
> print("fin")
> ```
>
> ```sortie
> 3 6 9 12 15 fin
> ```

**Pregunta 3:** ¿qué muestra este código?

```python norun
n = 1
while n < 100:
    n = n * 2
    if n > 10:
        break
print(n)
```

> **Respuesta**
>
> `16`. `n` vale 2, luego 4, luego 8, luego 16. En 16, `n > 10` es verdadero: `break`
> detiene el bucle antes de que la condición del `while` sea falsa.

## En resumen

- `while` repite mientras una condición sea verdadera; una variable debe cambiar en
  cada vuelta.
- `for` recorre una colección o un `range`.
- El final de un `range` queda excluido: `range(1, 6)` va del 1 al 5.
- `break` sale, `continue` se salta una vuelta.
- `enumerate` y `zip` simplifican los recorridos.

Siguiente: [las funciones](fonctions.md).
