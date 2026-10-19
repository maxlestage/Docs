# Seguir un programa paso a paso

Esta página te enseña a ejecutar un programa de Python en tu cabeza, línea por línea, con una tabla que sigue el valor de cada variable, en cinco pequeños programas cada vez más completos.

> **Objetivos:**
>
> - leer un programa en el orden en que Python lo ejecuta;
> - rellenar una tabla de seguimiento (una columna por variable, una fila por paso);
> - adivinar la salida de un programa **antes** de ejecutarlo;
> - seguir una llamada a función: quién llama a quién y qué vuelve;
> - hacer que el programa muestre sus propios pasos con `print`.

## ¿Por qué seguir un programa a mano?

Un ordenador no adivina nada. Lee tus líneas **una por una**, de arriba abajo, y hace
exactamente lo que está escrito. Si sabes hacer lo mismo en tu cabeza, entiendes
cualquier programa y encuentras tus [errores](../debuter/glossaire.md#bug) mucho más
rápido.

Piensa en una receta de cocina. Un buen cocinero no lee la receta «por encima». Sigue
los pasos en orden y mira lo que hay en cada bol después de cada paso. Aquí, los boles
son las [variables](../debuter/glossaire.md#variable).

```text
   la receta (el código)         los boles (las variables)
   ┌────────────────────┐        ┌─────┐  ┌─────┐
   │ 1. a = 5           │  ───►  │ a 5 │  │ b ? │
   │ 2. b = 3           │        └─────┘  └─────┘
   │ 3. a = a + b       │
   └────────────────────┘
        ▲
        └── el dedo: la línea que se ejecuta ahora
```

> **Recuerda:** seguir un programa es avanzar un dedo por el código, una línea cada
> vez, y anotar lo que contiene cada variable después de cada línea.

## La tabla de seguimiento

La herramienta básica es una tabla. A menudo se la llama **tabla de seguimiento** (en
inglés, una *trace*, una «traza»).

- Una **columna por variable**, más una columna «Línea» y una columna «Muestra».
- Una **fila de la tabla por paso**: cada vez que Python ejecuta una línea del código.
- En cada casilla, el valor **después** del paso. Una casilla vacía quiere decir «sin
  cambios».

Basta con una hoja de papel y un lápiz. Escribir a mano te obliga a no saltarte ningún
paso.

Para que sea fácil de seguir, escribiremos cada programa dos veces: primero con números
de línea (`L1`, `L2`…), para la tabla; luego de verdad, para ejecutarlo.

> **Recuerda:** una columna por variable, una fila por paso, y siempre el valor
> **después** del paso.

## Programa 1: variables que cambian

El primer programa no tiene ni condición ni bucle. Solo variables que cambian.

```text
L1  a = 5
L2  b = 3
L3  a = a + b
L4  b = a - b
L5  print(a, b)
```

**Antes de ejecutarlo, adivina lo que se muestra.** Escribe tu respuesta en un papel.

Aquí tienes el seguimiento. En la línea 3, Python calcula primero la parte **a la
derecha** del `=` con los valores actuales, y luego guarda el resultado en la caja de
la izquierda.

| Paso | Línea | Cálculo | `a` | `b` | Muestra |
|---|---|---|---|---|---|
| 1 | L1 | | 5 | | |
| 2 | L2 | | 5 | 3 | |
| 3 | L3 | `5 + 3` | 8 | 3 | |
| 4 | L4 | `8 - 3` | 8 | 5 | |
| 5 | L5 | | 8 | 5 | `8 5` |

La trampa está en el paso 4: `a` ya no vale 5, sino 8. Hay que usar el valor **del
momento**, no el del principio.

Ejecútalo para comprobarlo:

```python
a = 5
b = 3
a = a + b
b = a - b
print(a, b)
```

```sortie
8 5
```

Prueba: añade una línea `a = a - b` justo antes del `print`. Síguelo a mano: `a` pasa a
ser `8 - 5`, o sea `3`. El programa muestra `3 5`: ¡los valores iniciales se han
intercambiado!

> **Recuerda:** en `a = a + b`, la derecha se calcula primero, con los valores del
> momento. Solo después cambia `a`.

## Programa 2: una condición

Añadamos una [condición](../debuter/glossaire.md#condition). Con un `if`, algunas líneas
se **saltan**. La tabla debe mostrarlo.

```text
L1  temperatura = 12
L2  lluvia = True
L3  if temperatura > 20:
L4      consejo = "camiseta"
L5  elif lluvia:
L6      consejo = "paraguas"
L7  else:
L8      consejo = "jersey"
L9  print(consejo)
```

**Antes de ejecutarlo, adivina lo que se muestra.**

Para una condición, añade una columna «Prueba»: la pregunta que se hace y su respuesta,
`True` (verdadero) o `False` (falso).

| Paso | Línea | Prueba | `temperatura` | `lluvia` | `consejo` | Muestra |
|---|---|---|---|---|---|---|
| 1 | L1 | | 12 | | | |
| 2 | L2 | | 12 | True | | |
| 3 | L3 | `12 > 20` → False | | | | |
| 4 | L5 | `lluvia` → True | | | | |
| 5 | L6 | | | | `"paraguas"` | |
| 6 | L9 | | | | | `paraguas` |

Fíjate en las líneas **ausentes** de la tabla: L4, L7 y L8. Python no las ejecuta. Es
como un cambio de agujas de tren: solo se toma una vía.

```text
            temperatura > 20 ?
              /          \
            sí            no
            │               │
       "camiseta"       lluvia ?
                        /     \
                       sí      no
                       │        │
                 "paraguas"  "jersey"
```

```python
temperatura = 12
lluvia = True
if temperatura > 20:
    consejo = "camiseta"
elif lluvia:
    consejo = "paraguas"
else:
    consejo = "jersey"
print(consejo)
```

```sortie
paraguas
```

Prueba: pon `temperatura = 25`. La primera prueba pasa a ser verdadera: Python toma la
primera vía, muestra `camiseta`, y ni siquiera mira `lluvia`.

> **Recuerda:** en un `if` / `elif` / `else`, Python prueba en orden y toma **la
> primera** vía verdadera. Las demás líneas no aparecen en la tabla.

## Programa 3: un bucle que acumula

Con un [bucle](../debuter/glossaire.md#boucle), las mismas líneas vuelven varias veces.
En la tabla aparecen, por tanto, varias veces, una por vuelta.

```text
L1  total = 0
L2  for n in range(1, 5):
L3      total = total + n * 2
L4  print(total)
```

**Antes de ejecutarlo, adivina lo que se muestra.** Recuerda: `range(1, 5)` da 1, 2, 3,
4 (el 5 queda excluido).

`total` es una hucha. Empieza vacía (0), y cada vuelta mete una moneda.

| Paso | Línea | `n` | Cálculo | `total` | Muestra |
|---|---|---|---|---|---|
| 1 | L1 | | | 0 | |
| 2 | L2 | 1 | | 0 | |
| 3 | L3 | 1 | `0 + 1 * 2` | 2 | |
| 4 | L2 | 2 | | 2 | |
| 5 | L3 | 2 | `2 + 2 * 2` | 6 | |
| 6 | L2 | 3 | | 6 | |
| 7 | L3 | 3 | `6 + 3 * 2` | 12 | |
| 8 | L2 | 4 | | 12 | |
| 9 | L3 | 4 | `12 + 4 * 2` | 20 | |
| 10 | L2 | fin | | 20 | |
| 11 | L4 | | | 20 | `20` |

En el paso 10, ya no quedan números en `range(1, 5)`: el bucle termina, y Python pasa a
la línea que sigue al bloque con sangría.

```python
total = 0
for n in range(1, 5):
    total = total + n * 2
print(total)
```

```sortie
20
```

Prueba: desplaza `print(total)` 4 espacios, para meterlo **dentro** del bucle. Entonces
se ejecuta en cada vuelta: el programa muestra `2`, `6`, `12` y luego `20`.

### Lo mismo con while

Un bucle `while` prueba su condición **antes de cada vuelta**. Pon esa prueba en la
tabla.

```python
dinero = 10
dias = 0
while dinero > 0:
    dinero = dinero - 3
    dias = dias + 1
print(dias, dinero)
```

```sortie
4 -2
```

| Vuelta | Prueba `dinero > 0` | `dinero` después | `dias` después |
|---|---|---|---|
| 1 | `10 > 0` → True | 7 | 1 |
| 2 | `7 > 0` → True | 4 | 2 |
| 3 | `4 > 0` → True | 1 | 3 |
| 4 | `1 > 0` → True | -2 | 4 |
| — | `-2 > 0` → False | fin | |

Mucha gente adivina `3 1`. La tabla muestra por qué es falso: cuando queda 1, la prueba
`1 > 0` sigue siendo verdadera, así que hay una cuarta vuelta.

> **Recuerda:** en un bucle, se anota una fila por vuelta. El valor «después» de una
> vuelta se convierte en el valor «antes» de la vuelta siguiente.

## Programa 4: una función llamada dos veces

Una [función](../debuter/glossaire.md#fonction) es una pequeña máquina: le das algo,
trabaja por su cuenta y luego **devuelve** un resultado. Cuando Python encuentra una
llamada, **salta** a la función y luego **vuelve** al lugar de la llamada.

```text
L1  def precio_con_iva(precio):
L2      iva = precio * 20 // 100
L3      return precio + iva
L4
L5  a = precio_con_iva(50)
L6  b = precio_con_iva(a)
L7  print(a, b)
```

**Antes de ejecutarlo, adivina lo que se muestra.**

Lo primero: las líneas L1 a L3 **definen** la función, no la ejecutan. Es como escribir
una receta en un cuaderno: todavía no se cocina. El trabajo de verdad empieza en L5.

Esto es quién llama a quién, y lo que vuelve:

```text
  programa principal                   precio_con_iva
  ──────────────────                   ──────────────
  L5  a = precio_con_iva(50)  ── 50 ──►  precio = 50
                                         iva = 10
      a = 60                  ◄── 60 ──  return 60

  L6  b = precio_con_iva(a)   ── 60 ──►  precio = 60
                                         iva = 12
      b = 72                  ◄── 72 ──  return 72

  L7  print(a, b)  →  60 72
```

La tabla añade una columna «¿Dónde?» para saber si estamos dentro de la función o fuera.

| Paso | ¿Dónde? | Línea | `precio` | `iva` | `a` | `b` | Muestra |
|---|---|---|---|---|---|---|---|
| 1 | fuera | L5 | | | ? | | |
| 2 | función | L1 | 50 | | | | |
| 3 | función | L2 | 50 | 10 | | | |
| 4 | función | L3 | 50 | 10 | | | devuelve 60 |
| 5 | fuera | L5 | | | 60 | | |
| 6 | fuera | L6 | | | 60 | ? | |
| 7 | función | L1 | 60 | | | | |
| 8 | función | L2 | 60 | 12 | | | |
| 9 | función | L3 | 60 | 12 | | | devuelve 72 |
| 10 | fuera | L6 | | | 60 | 72 | |
| 11 | fuera | L7 | | | 60 | 72 | `60 72` |

Dos detalles importantes:

- `precio` e `iva` solo existen **durante** la llamada. En cada nueva llamada, vuelven a
  empezar de cero. Se dice que son variables locales (mira
  [el ámbito](../debuter/glossaire.md#portee)).
- En la segunda llamada, el [parámetro](../debuter/glossaire.md#parametre) `precio`
  recibe el valor de `a`, es decir, 60.

```python
def precio_con_iva(precio):
    iva = precio * 20 // 100
    return precio + iva

a = precio_con_iva(50)
b = precio_con_iva(a)
print(a, b)
```

```sortie
60 72
```

Prueba: sustituye `return precio + iva` por `print(precio + iva)`. La función muestra
`60` y luego no devuelve nada: `a` vale `None`, y la segunda llamada falla. Mira
[los errores frecuentes](#errores-frecuentes) más abajo.

> **Recuerda:** en una llamada, el dedo salta a la función; en el `return`, vuelve a la
> línea de la llamada con el valor devuelto.

## Programa 5: una lista y una condición en el bucle

Último programa: un bucle sobre una [lista](../debuter/glossaire.md#liste), con un `if`
dentro. Todo lo que has visto se combina.

```text
L1  notas = [12, 7, 15, 9, 18]
L2  aprobadas = []
L3  for nota in notas:
L4      if nota >= 10:
L5          aprobadas.append(nota)
L6      else:
L7          print(f"{nota}: hay que repasar")
L8  print(aprobadas)
```

**Antes de ejecutarlo, adivina lo que se muestra.** Atención: hay `print` **dentro**
del bucle y un `print` **después**.

| Vuelta | `nota` | Prueba `nota >= 10` | Acción | `aprobadas` después | Muestra |
|---|---|---|---|---|---|
| 1 | 12 | True | `append(12)` | `[12]` | |
| 2 | 7 | False | `print` | `[12]` | `7: hay que repasar` |
| 3 | 15 | True | `append(15)` | `[12, 15]` | |
| 4 | 9 | False | `print` | `[12, 15]` | `9: hay que repasar` |
| 5 | 18 | True | `append(18)` | `[12, 15, 18]` | |
| después | | | L8 | `[12, 15, 18]` | `[12, 15, 18]` |

Imagina que clasificas el correo: cada carta pasa delante de ti, una cada vez. Las
buenas van a la cesta `aprobadas`, las demás reciben una nota enseguida.

```text
  notas:   12   7   15   9   18
           │    │   │    │   │
           ▼    ▼   ▼    ▼   ▼
         >=10 ? ── sí ──►  cesta: [12, 15, 18]
                └─ no ──►  print «hay que repasar»
```

```python
notas = [12, 7, 15, 9, 18]
aprobadas = []
for nota in notas:
    if nota >= 10:
        aprobadas.append(nota)
    else:
        print(f"{nota}: hay que repasar")
print(aprobadas)
```

```sortie
7: hay que repasar
9: hay que repasar
[12, 15, 18]
```

Los mensajes sobre 7 y 9 salen **antes** que la lista: se muestran durante el bucle,
mientras que la lista solo se muestra al final.

Prueba: cambia la prueba por `nota > 12`. Síguelo a mano antes de ejecutarlo: 12 ya no
pasa, así que el programa también muestra `12: hay que repasar`, y la lista pasa a ser
`[15, 18]`.

> **Recuerda:** en un bucle con un `if`, cada vuelta elige su vía. Anota para cada
> vuelta la prueba, su respuesta y lo que cambia.

## Hacer que el programa muestre los pasos

A veces el programa es demasiado largo para una tabla en papel. Entonces, pídele a
Python que rellene la tabla él mismo: añade un `print` en el bucle, con **etiquetas**
para saber quién es quién.

```python
total = 0
for n in range(1, 5):
    total = total + n * 2
    print(f"vuelta n={n}: total={total}")
print("fin:", total)
```

```sortie
vuelta n=1: total=2
vuelta n=2: total=6
vuelta n=3: total=12
vuelta n=4: total=20
fin: 20
```

Es exactamente la tabla del programa 3, escrita por Python.

Python tiene un atajo práctico: en una f-string, `{x=}` muestra el nombre **y** el
valor.

```python
def precio_con_iva(precio):
    iva = precio * 20 // 100
    print(f"  en precio_con_iva: {precio=} {iva=}")
    return precio + iva

a = precio_con_iva(50)
print(f"{a=}")
b = precio_con_iva(a)
print(f"{b=}")
```

```sortie
  en precio_con_iva: precio=50 iva=10
a=60
  en precio_con_iva: precio=60 iva=12
b=72
```

Los dos espacios al principio de las líneas «en precio_con_iva» muestran que estamos
**dentro** de la función. Se ve la ida y vuelta del dibujo del programa 4.

Prueba: añade `print(f"{nota=}")` como primera línea del bucle del programa 5. Verás
pasar cada nota, una por línea, antes que los demás mensajes de esa vuelta.

> **Consejo:** cuando encuentres el error, quita esos `print`. Sirven para investigar,
> no para quedarse en el programa final.

> **Recuerda:** un `print` con etiquetas, colocado en el bucle o en la función,
> convierte el programa en una tabla de seguimiento automática.

## ¿Y el depurador?

Python también tiene un [depurador](../debuter/glossaire.md#debogage), `pdb`: escribe
`breakpoint()` en tu código, y el programa se detiene ahí para dejarte mirar las
variables (`p total`) y luego avanzar una línea (`n`). Es tu tabla de seguimiento, pero
en directo; todo se explica en [Depurar en Python](debogage.md).

> **Recuerda:** papel, `print` o depurador: siempre es la misma idea, mirar las
> variables un paso tras otro.

## Errores frecuentes

Estos errores vienen a menudo de un paso mal seguido. La tabla de seguimiento ayuda a
verlos **antes** de ejecutar.

### Dar una vuelta de más

Al seguir el programa, se olvida que `range(n)` se detiene **antes** de `n`. Aquí se da
una vuelta de más:

```python norun
notas = [12, 7, 15]
for i in range(len(notas) + 1):
    print(notas[i])
```

```text
12
7
15
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    print(notas[i])
          ~~~~~^^^
IndexError: list index out of range
```

La tabla muestra el problema: en la vuelta 4, `i` vale 3, pero la lista solo tiene las
casillas 0, 1 y 2.

| Vuelta | `i` | `notas[i]` |
|---|---|---|
| 1 | 0 | 12 |
| 2 | 1 | 7 |
| 3 | 2 | 15 |
| 4 | 3 | ¡no hay casilla 3! |

La corrección: recorre directamente la lista, sin contar.

```python
notas = [12, 7, 15]
for nota in notas:
    print(nota)
```

```sortie
12
7
15
```

### Usar una variable que todavía no tiene valor

En la primera vuelta, se lee `total`. Pero ninguna línea de arriba le ha dado un valor:
la primera casilla de la columna `total` está vacía.

```python norun
for n in [3, 4, 5]:
    total = total + n
print(total)
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    total = total + n
            ^^^^^
NameError: name 'total' is not defined
```

La corrección: llena la hucha **antes** del bucle.

```python
total = 0
for n in [3, 4, 5]:
    total = total + n
print(total)
```

```sortie
12
```

### Creer que una función devuelve lo que muestra

Al seguir el programa, se anota «`resultado` vale 6» porque la función muestra 6. Pero
la función no tiene `return`: devuelve `None`.

```python norun
def doble(x):
    print(x * 2)

resultado = doble(3)
print(resultado + 1)
```

```text
6
Traceback (most recent call last):
  File "main.py", line 5, in <module>
    print(resultado + 1)
          ~~~~~~~~~~^~~
TypeError: unsupported operand type(s) for +: 'NoneType' and 'int'
```

En el dibujo de la llamada, la flecha de vuelta lleva `None`, no 6. La corrección:

```python
def doble(x):
    return x * 2

resultado = doble(3)
print(resultado + 1)
```

```sortie
7
```

## Ponte a prueba

**Pregunta 1:** ¿cuánto vale `x` al final?

```python norun
x = 2
x = x * 3
x = x - 1
print(x)
```

> **Respuesta**
>
> | Paso | `x` |
> |---|---|
> | `x = 2` | 2 |
> | `x = x * 3` | 6 |
> | `x = x - 1` | 5 |
>
> El programa muestra `5`.

**Pregunta 2:** ¿cuántas veces se muestra la palabra `vuelta`, y cuánto vale `n` al
final?

```python norun
n = 1
while n < 20:
    print("vuelta")
    n = n * 3
print(n)
```

> **Respuesta**
>
> | Prueba `n < 20` | `n` después de la vuelta |
> |---|---|
> | `1 < 20` → True | 3 |
> | `3 < 20` → True | 9 |
> | `9 < 20` → True | 27 |
> | `27 < 20` → False | fin |
>
> Tres vueltas, así que tres veces `vuelta`, y luego `27`.

**Pregunta 3:** ¿qué muestra este programa?

```python norun
def mas_uno(n):
    return n + 1

a = mas_uno(1)
b = mas_uno(mas_uno(a))
print(a, b)
```

> **Respuesta**
>
> Primera llamada: `mas_uno(1)` devuelve 2, así que `a = 2`. Para `b`, Python calcula
> primero la llamada de **dentro**: `mas_uno(2)` devuelve 3. Luego la llamada de fuera:
> `mas_uno(3)` devuelve 4. El programa muestra `2 4`.

## En resumen

- Python ejecuta las líneas una por una, de arriba abajo; síguelas con el dedo.
- Una tabla de seguimiento tiene una columna por variable y una fila por paso (o por
  vuelta del bucle).
- En `x = x + 1`, la derecha se calcula primero, con el valor **del momento**.
- Una condición salta líneas; un bucle repite líneas; una llamada a función salta a la
  función y luego vuelve con el valor del `return`.
- Adivina la salida antes de ejecutar y luego compara: es el mejor entrenamiento.
- `print(f"{x=}")` hace que Python escriba la tabla; `breakpoint()` pone el programa en
  pausa.

Siguiente: [las trampas del principiante en Python](pieges.md)
