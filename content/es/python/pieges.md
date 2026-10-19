# Las trampas del principiante en Python

Esta página reúne las doce trampas que más a menudo bloquean a los principiantes en Python, con, para cada una, lo que pasa, por qué y cómo evitarla.

> **Objetivos:**
>
> - sabrás reconocer las doce trampas más comunes de Python;
> - entenderás por qué Python se comporta así;
> - sabrás corregir cada trampa en una o dos líneas.

## Cómo leer esta página

Cada trampa sigue el mismo camino:

1. lo que **piensas** que va a pasar;
2. el código, y lo que pasa **de verdad**;
3. **por qué**;
4. **la corrección**.

No lo leas todo de golpe. Ten esta página a mano. Cuando un programa te sorprenda,
vuelve aquí y busca la trampa que se parezca a tu problema.

> **Recuerda:** una trampa no es culpa tuya. Todo el mundo cae en ellas al principio.
> Lo que cuenta es reconocerla la segunda vez.

## Las trampas de sintaxis

La [sintaxis](../debuter/glossaire.md#sintaxis) son las reglas de escritura del lenguaje,
como la gramática para el español.

### Escribir = en lugar de ==

**Piensas:** «compruebo si la edad vale 18».

```python norun
edad = 18
if edad = 18:
    print("mayor de edad")
```

```text
  File "main.py", line 2
    if edad = 18:
       ^^^^^^^^^
SyntaxError: invalid syntax. Maybe you meant '==' or ':=' instead of '='?
```

**Por qué:** en Python, los dos signos no quieren decir lo mismo.

| Signo | Se lee | Función |
|---|---|---|
| `=` | «recibe» | guarda un valor en una variable |
| `==` | «¿es igual a?» | hace una pregunta, responde `True` o `False` |

Piensa en una caja con una etiqueta. `edad = 18` mete 18 **dentro** de la caja.
`edad == 18` **mira** dentro de la caja y responde sí o no.

**La corrección:**

```python
edad = 18
if edad == 18:
    print("mayor de edad")
```

```sortie
mayor de edad
```

Python te ayuda: el mensaje dice `Maybe you meant '=='` («¿querías decir `==`?»).

### Olvidar la sangría

**Piensas:** «el `print` forma parte del `if`».

```python norun
edad = 18
if edad >= 18:
print("mayor de edad")
```

```text
  File "main.py", line 3
    print("mayor de edad")
    ^^^^^
IndentationError: expected an indented block after 'if' statement on line 2
```

**Por qué:** en Python, la [sangría](../debuter/glossaire.md#indentacion) (los espacios
al principio de la línea) no es decorativa. Es ella la que dice qué líneas están
**dentro** del `if`.

```text
if edad >= 18:                ← la pregunta
    print("mayor de edad")    ← 4 espacios: esta línea está DENTRO del if
print("fin")                  ← 0 espacios: esta línea está DESPUÉS del if
```

**La corrección:** añade 4 espacios delante de cada línea del bloque.

```python
edad = 18
if edad >= 18:
    print("mayor de edad")
print("fin")
```

```sortie
mayor de edad
fin
```

La trampa contraria también existe: demasiados espacios, sin motivo. Python responde
entonces `IndentationError: unexpected indent` («sangría inesperada»).

> **Atención:** no mezcles nunca tabulaciones y espacios. Configura tu editor para que
> ponga 4 espacios cuando pulses Tab.

## Las trampas de tipos

### Sumar texto y un número

**Piensas:** `"12" + 1` da `13`.

```python norun
edad = "12"
print(edad + 1)
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(edad + 1)
          ~~~~~^~~
TypeError: can only concatenate str (not "int") to str
```

**Por qué:** `"12"` tiene comillas. Es **texto** (una
[cadena de caracteres](../debuter/glossaire.md#cadena-de-caracteres)), no un número.
Para el texto, `+` quiere decir «pegar uno detrás de otro». Python no sabe pegar un
número a un texto, y se niega a adivinar.

El mismo `+` da resultados muy distintos según el tipo:

```python
print(12 + 1)
print("12" + "1")
print("12" * 3)
```

```sortie
13
121
121212
```

**La corrección:** convierte el texto en número con `int()`.

```python
edad = "12"
print(int(edad) + 1)
```

```sortie
13
```

Esta trampa aparece a menudo con `input()`: lo que escribe el usuario es **siempre**
texto. Escribe `int(input("Tu edad: "))`.

> **Pruébalo:** sustituye `int(edad)` por `str(1)` y `+ 1` por `+ str(1)`. Obtienes
> `121`: esta vez, todo es texto.

### La división que da un número con decimales

**Piensas:** `10 / 2` da `5`.

```python
print(10 / 2)
print(7 / 2)
print(7 // 2)
print(7 % 2)
```

```sortie
5.0
3.5
3
1
```

**Por qué:** en Python, `/` da **siempre** un número con decimales (un
[flotante](../debuter/glossaire.md#flotante)), incluso cuando la división es exacta.

| Operador | Nombre | `7 ? 2` |
|---|---|---|
| `/` | división | `3.5` |
| `//` | división entera | `3` |
| `%` | resto | `1` |

**La corrección:** usa `//` cuando quieras un número entero, por ejemplo para repartir
7 caramelos entre 2 niños.

### 0.1 + 0.2 no da 0.3

**Piensas:** `0.1 + 0.2 == 0.3` es verdadero.

```python
print(0.1 + 0.2)
print(0.1 + 0.2 == 0.3)
```

```sortie
0.30000000000000004
False
```

**Por qué:** el ordenador guarda los números con decimales en binario. `0.1` no es
exacto en binario, igual que `1/3` no es exacto en decimal (`0.3333…`). Queda un error
muy pequeño.

**La corrección:** redondea antes de comparar.

```python
print(round(0.1 + 0.2, 2) == 0.3)
```

```sortie
True
```

El capítulo [los números](nombres.md) muestra también `math.isclose` y `Decimal`.

> **Recuerda:** mira siempre el **tipo** de un valor. Si tienes dudas, muestra
> `type(valor)`.

## Las trampas de listas

### Contar desde 0, y range que se para antes

**Piensas:** una lista de 3 elementos tiene un elemento número 3.

```python norun
numeros = [10, 20, 30]
print(numeros[3])
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(numeros[3])
          ~~~~~~~^^^
IndexError: list index out of range
```

**Por qué:** en Python, las posiciones se cuentan a partir de **0**. El número de una
posición se llama [índice](../debuter/glossaire.md#indice).

```text
numeros = [ 10 , 20 , 30 ]
índice :     0    1    2      ← no hay posición 3
```

`range` sigue la misma lógica: `range(1, 5)` se para **antes** del 5.

```python
print(list(range(1, 5)))
print(list(range(5)))
```

```sortie
[1, 2, 3, 4]
[0, 1, 2, 3, 4]
```

**La corrección:** el último elemento está en el índice `len(lista) - 1`, o más
sencillo, `lista[-1]`. Para contar de 1 a 5 incluido, escribe `range(1, 6)`.

```python
numeros = [10, 20, 30]
print(numeros[2])
print(numeros[-1])
```

```sortie
30
30
```

### Dos nombres para la misma lista

**Piensas:** `copia = notas` hace una copia.

```python
notas = [12, 15]
copia = notas
copia.append(8)
print(notas)
```

```sortie
[12, 15, 8]
```

**Por qué:** `copia = notas` no copia nada. Pega una **segunda etiqueta** en la
**misma** lista.

```text
notas ──┐
        ├──► [12, 15, 8]     una sola lista, dos etiquetas
copia ──┘
```

Una lista es [mutable](../debuter/glossaire.md#mutable): se puede modificar. Así que lo
que haces con una etiqueta se ve con la otra.

**La corrección:** pide una copia de verdad con `.copy()`.

```python
notas = [12, 15]
copia = notas.copy()
copia.append(8)
print(notas)
print(copia)
```

```sortie
[12, 15]
[12, 15, 8]
```

> **Pruébalo:** sustituye `.copy()` por `[:]` (`copia = notas[:]`). El resultado es el
> mismo: `[:]` toma una porción que lo contiene todo.

### Borrar elementos mientras recorres la lista

**Piensas:** este código quita todos los números negativos.

```python
numeros = [-1, -2, 3, 4]
for n in numeros:
    if n < 0:
        numeros.remove(n)
print(numeros)
```

```sortie
[-2, 3, 4]
```

**Por qué:** el bucle avanza una posición en cada vuelta. Pero `remove` desplaza todos
los elementos hacia la izquierda. Así que el bucle se salta un elemento. Sigámoslo paso
a paso:

| Vuelta | Posición leída | Lista antes | `n` | Acción | Lista después |
|---|---|---|---|---|---|
| 1 | 0 | `[-1, -2, 3, 4]` | `-1` | borrar | `[-2, 3, 4]` |
| 2 | 1 | `[-2, 3, 4]` | `3` | nada | `[-2, 3, 4]` |
| 3 | 2 | `[-2, 3, 4]` | `4` | nada | `[-2, 3, 4]` |

En la vuelta 2, el bucle lee la posición 1. El `-2` se ha deslizado a la posición 0:
nunca se lee.

**La corrección:** no modifiques la lista que recorres. Construye una lista **nueva**
con lo que quieres conservar, por ejemplo con una [comprensión](comprehensions.md).

```python
numeros = [-1, -2, 3, 4]
numeros = [n for n in numeros if n >= 0]
print(numeros)
```

```sortie
[3, 4]
```

### sort() no devuelve nada

**Piensas:** `palabras = palabras.sort()` guarda la lista ordenada en `palabras`.

```python
palabras = ["pera", "albaricoque"]
palabras = palabras.sort()
print(palabras)
```

```sortie
None
```

**Por qué:** `.sort()` ordena la lista **en el sitio**: modifica la propia lista y luego
devuelve `None` («nada»). Así que has guardado `None` en `palabras`.

**La corrección:** dos formas, a elegir.

```python
palabras = ["pera", "albaricoque"]
palabras.sort()
print(palabras)

frutas = ["pera", "albaricoque"]
ordenadas = sorted(frutas)
print(ordenadas)
print(frutas)
```

```sortie
['albaricoque', 'pera']
['albaricoque', 'pera']
['pera', 'albaricoque']
```

`lista.sort()` modifica la lista. `sorted(lista)` deja la lista tranquila y devuelve
una lista nueva ordenada.

> **Recuerda:** una lista puede tener varios nombres. Antes de modificarla, pregúntate
> «¿quién más está mirando esta lista?».

## Las trampas de texto

### Los métodos de texto no modifican el texto

**Piensas:** `nombre.upper()` pone `nombre` en mayúsculas.

```python
nombre = "alice"
nombre.upper()
print(nombre)
```

```sortie
alice
```

**Por qué:** una cadena de caracteres es [inmutable](../debuter/glossaire.md#inmutable):
no se puede cambiar nunca. `upper()` fabrica una cadena **nueva**. Si no la guardas en
ningún sitio, se pierde.

Es lo contrario de la trampa de `sort()`: para el texto, el método devuelve el
resultado y no toca el original.

**La corrección:** guarda el resultado.

```python
nombre = "alice"
nombre = nombre.upper()
print(nombre)
```

```sortie
ALICE
```

## Las trampas de funciones

### Mostrar en lugar de devolver

**Piensas:** esta función da el doble de un número.

```python
def doble(x):
    print(x * 2)

resultado = doble(5)
print(resultado)
```

```sortie
10
None
```

**Por qué:** `print` **muestra** un valor en la pantalla. `return` **da** un valor al
resto del programa. Una función sin `return` devuelve `None`.

Piensa en una máquina expendedora de bebidas. `print` es la bebida que aparece en la
pantalla. `return` es la bebida que cae en la bandeja: puedes cogerla.

**La corrección:**

```python
def doble(x):
    return x * 2

resultado = doble(5)
print(resultado)
print(resultado + 1)
```

```sortie
10
11
```

### Una lista vacía como valor por defecto

**Piensas:** cada llamada empieza con una cesta vacía.

```python
def agregar(articulo, cesta=[]):
    cesta.append(articulo)
    return cesta

print(agregar("pan"))
print(agregar("leche"))
```

```sortie
['pan']
['pan', 'leche']
```

**Por qué:** el valor por defecto `[]` se crea **una sola vez**, cuando Python lee la
línea `def`. Después, todas las llamadas comparten **la misma** lista.

**La corrección:** pon `None` por defecto, y crea la lista dentro de la función.

```python
def agregar(articulo, cesta=None):
    if cesta is None:
        cesta = []
    cesta.append(articulo)
    return cesta

print(agregar("pan"))
print(agregar("leche"))
```

```sortie
['pan']
['leche']
```

### Modificar una variable de fuera

**Piensas:** la función suma 1 al contador.

```python norun
contador = 0

def incrementar():
    contador = contador + 1

incrementar()
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    incrementar()
    ~~~~~~~~~~~^^
  File "main.py", line 4, in incrementar
    contador = contador + 1
               ^^^^^^^^
UnboundLocalError: cannot access local variable 'contador' where it is not associated with a value
```

**Por qué:** en cuanto una función **guarda** algo en un nombre (`contador = …`),
Python considera que ese nombre pertenece a la función. Es una cuestión de
[alcance](../debuter/glossaire.md#ambito). Así que la función busca su propio
`contador`, que todavía no tiene ningún valor.

**La corrección:** pasa el valor como parámetro y devuelve el resultado.

```python
def incrementar(contador):
    return contador + 1

contador = 0
contador = incrementar(contador)
contador = incrementar(contador)
print(contador)
```

```sortie
2
```

La palabra clave `global` también existe, pero hace que los programas sean difíciles
de seguir. Evítala al principio.

### Llamar a una variable list o str

**Piensas:** `list` es un buen nombre para una lista.

```python norun
list = [3, 1, 2]
print(list("abc"))
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(list("abc"))
          ~~~~^^^^^^^
TypeError: 'list' object is not callable
```

**Por qué:** `list`, `str`, `int`, `sum`, `max`… son funciones de Python. Si guardas
otra cosa con ese nombre, **ocultas** la función. Python encuentra tu lista e intenta
«llamarla» como si fuera una función.

**La corrección:** elige un nombre que diga lo que contiene la variable: `numeros`,
`nombres`, `texto`, `total`.

> **Recuerda:** una función **devuelve** con `return`, trabaja con sus
> **parámetros**, y no debe reutilizar los nombres de Python.

## Errores frecuentes

Dos errores más, que no son trampas pero que ocurren todos los días.

### Olvidar las comillas

```python norun
print(hola)
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    print(hola)
          ^^^^
NameError: name 'hola' is not defined
```

Sin comillas, `hola` es un **nombre de variable**, y esa variable no existe. Escribe
`print("hola")`. El mismo error aparece con una errata en un nombre: `nombe` en lugar
de `nombre`.

### Una clave que no está en el diccionario

```python norun
edades = {"Alice": 30}
print(edades["Bob"])
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(edades["Bob"])
          ~~~~~~^^^^^^^
KeyError: 'Bob'
```

La clave `"Bob"` no existe. Usa `.get`, que devuelve un valor de reserva en lugar de
fallar:

```python
edades = {"Alice": 30}
print(edades.get("Bob", "desconocido"))
```

```sortie
desconocido
```

Para leer con calma un mensaje de error, consulta
[leer un error](../debuter/lire-une-erreur.md).

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este programa, y por qué?

```python norun
a = [1, 2]
b = a
b.append(3)
print(a)
```

> **Respuesta**
>
> Muestra `[1, 2, 3]`. `b = a` no copia la lista: `a` y `b` son dos etiquetas en la
> misma lista. Para una copia de verdad, escribe `b = a.copy()`.

**Pregunta 2:** este programa muestra `10 / 4 = 2.5`. ¿Cómo obtener `2`?

```python norun
print("10 / 4 =", 10 / 4)
```

> **Respuesta**
>
> Con la división entera `//`:
>
> ```python
> print("10 // 4 =", 10 // 4)
> ```
>
> ```sortie
> 10 // 4 = 2
> ```

**Pregunta 3:** ¿por qué este programa muestra `None`?

```python norun
def cuadrado(x):
    print(x * x)

print(cuadrado(3))
```

> **Respuesta**
>
> La función muestra `9`, pero no devuelve nada: sin `return`, devuelve `None`.
> Así que el segundo `print` muestra `None`. Sustituye `print(x * x)` por
> `return x * x`.

## En resumen

- `=` guarda un valor, `==` compara dos valores.
- La sangría forma parte del código: 4 espacios por nivel.
- `"12"` es texto: conviértelo con `int()` antes de calcular. `/` da siempre un
  flotante, `//` un entero.
- Se cuenta a partir de 0, y `range(a, b)` se para antes de `b`.
- `b = a` no copia una lista; no modifiques una lista mientras la recorres.
- Una función devuelve con `return`; nada de lista vacía como valor por defecto.

Siguiente: [ejercicios para empezar bien](exercices-faciles.md)
