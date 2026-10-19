# Números y cálculos

Esta página explica cómo calcula Python: los operadores, los enteros y los flotantes, las conversiones, los módulos `math` y `random`, y cómo mostrar los números de forma cuidada.

> **Objetivos:**
>
> - sabrás usar los operadores, entre ellos `/`, `//`, `%` y `**`;
> - sabrás por qué `0.1 + 0.2` no vale exactamente `0.3`, y qué hacer;
> - sabrás convertir un texto en número, y al revés;
> - sabrás usar los módulos `math` y `random`;
> - sabrás mostrar un número con 2 decimales o con separadores de miles.

## Los operadores básicos

Un [operador](../debuter/glossaire.md#operador) es un símbolo que hace un cálculo.
Python sabe hacer todo lo que hace una calculadora.

| Operador | Significado | Ejemplo | Resultado |
| --- | --- | --- | --- |
| `+` | suma | `7 + 2` | `9` |
| `-` | resta | `7 - 2` | `5` |
| `*` | multiplicación | `7 * 2` | `14` |
| `/` | división | `7 / 2` | `3.5` |
| `//` | división entera | `7 // 2` | `3` |
| `%` | resto (módulo) | `7 % 2` | `1` |
| `**` | potencia | `7 ** 2` | `49` |

```python
print(7 + 2, 7 - 2, 7 * 2)
print(7 / 2, 7 // 2, 7 % 2)
print(7 ** 2, 2 ** 10)
```

```sortie
9 5 14
3.5 3 1
49 1024
```

## Tres divisiones: /, // y %

Imagina que repartes 17 caramelos entre 5 niños.

- `/` es la división «exacta»: `17 / 5` vale `3.4`. **Siempre** da un `float`, incluso
  cuando la cuenta es exacta.
- `//` es la división **entera**: cada niño recibe `17 // 5`, es decir, `3` caramelos.
- `%` es el **resto**: quedan `17 % 5`, es decir, `2` caramelos en la bolsa.

```python
caramelos = 17
ninos = 5
print(caramelos / ninos)
print(caramelos // ninos)
print(caramelos % ninos)
print(10 / 2)
```

```sortie
3.4
3
2
5.0
```

Fíjate en que `10 / 2` da `5.0` y no `5`: el `/` devuelve un `float`.

El resto `%` es muy útil para saber si un número es par: un número par tiene un resto
de 0 cuando lo divides entre 2.

```python
print(14 % 2 == 0)
print(15 % 2 == 0)
```

```sortie
True
False
```

> **Consejo:** para convertir segundos en minutos y segundos, combina `//` y `%`:
> `125 // 60` da `2` minutos, `125 % 60` da `5` segundos.

## El orden de las operaciones

Python sigue las reglas de las matemáticas: primero `**`, luego `*`, `/`, `//`, `%`, y
después `+` y `-`. Con la misma prioridad, calcula de izquierda a derecha.

```python
print(2 + 3 * 4)
print((2 + 3) * 4)
print(2 * 3 ** 2)
print(10 - 4 - 3)
```

```sortie
14
20
18
3
```

> **Recuerda:** si tienes dudas, pon paréntesis. No cuestan nada y hacen que el cálculo
> sea evidente para quien lo lee.

## Enteros sin límite

En muchos lenguajes, un entero tiene un tamaño máximo. En Python no: un `int` crece
tanto como haga falta. Solo lo limita la memoria del ordenador.

```python
print(2 ** 100)
print(1_000_000 * 1_000_000)
```

```sortie
1267650600228229401496703205376
1000000000000
```

El guion bajo `_` en un número solo sirve para hacerlo legible: `1_000_000` es el mismo
número que `1000000`.

## Los flotantes son aproximados

Un [flotante](../debuter/glossaire.md#flotante) se guarda en
[binario](../debuter/glossaire.md#binario). Algunos números sencillos en decimal, como
`0.1`, no son exactos en binario. Es como `1/3` en decimal: `0.3333...` no se acaba
nunca.

```python
print(0.1 + 0.2)
print(0.1 + 0.2 == 0.3)
```

```sortie
0.30000000000000004
False
```

No es un fallo de Python: todos los lenguajes hacen lo mismo. Dos soluciones sencillas:

- **redondear** para mostrar el resultado, con `round(numero, decimales)`;
- **comparar con una tolerancia**, con `math.isclose`.

```python
import math

total = 0.1 + 0.2
print(round(total, 2))
print(math.isclose(total, 0.3))
```

```sortie
0.3
True
```

> **Atención:** `round` redondea «al par más cercano» cuando el valor cae justo en la
> mitad: `round(2.5)` da `2`, y `round(3.5)` da `4`. Es el redondeo del banquero, que
> evita redondear siempre hacia arriba.

```python
print(round(2.5), round(3.5), round(2.567, 1))
```

```sortie
2 4 2.6
```

## Decimal y Fraction: cálculos exactos

Con dinero, un pequeño error de redondeo puede ser un problema. El módulo `decimal`
calcula en **decimal**, como en papel. Crea los números a partir de **textos**:

```python
from decimal import Decimal

print(Decimal("0.1") + Decimal("0.2"))
precio = Decimal("19.99")
print(precio * 3)
```

```sortie
0.3
59.97
```

El módulo `fractions` calcula con fracciones exactas, como en clase de matemáticas:

```python
from fractions import Fraction

tercio = Fraction(1, 3)
print(tercio + tercio)
print(tercio * 3)
print(Fraction(3, 4) + Fraction(1, 6))
```

```sortie
2/3
1
11/12
```

La fracción siempre se simplifica: `1/3 * 3` da `1`.

> **Nota:** `Decimal` y `Fraction` son más lentos que los `float`. Úsalos cuando la
> exactitud importa de verdad: importes de dinero, cálculos de precisión.

## El módulo math

Un [módulo](../debuter/glossaire.md#modulo) es una caja de herramientas. El módulo
`math` contiene las funciones matemáticas. Se abre con `import math`, y luego se
escribe `math.` delante de cada herramienta.

```python
import math

print(math.sqrt(16))
print(math.pi)
print(math.floor(3.7), math.ceil(3.2))
print(math.gcd(12, 18))
print(abs(-5), max(3, 9, 4), min(3, 9, 4))
```

```sortie
4.0
3.141592653589793
3 4
6
5 9 3
```

- `sqrt`: raíz cuadrada (*square root*);
- `floor` redondea hacia abajo (el suelo), `ceil` hacia arriba (el techo);
- `gcd`: el máximo común divisor;
- `abs`, `max` y `min` no necesitan `import`: siempre están disponibles.

## El módulo random

El módulo `random` saca números al azar: perfecto para un dado o un juego. Pero un
programa cuya salida cambia cada vez es difícil de probar. `random.seed(n)` fija el
punto de partida del azar: con la misma semilla, siempre obtienes las mismas tiradas.

```python
import random

random.seed(42)
print(random.randint(1, 6))
print(random.randint(1, 6))
print(random.choice(["piedra", "papel", "tijera"]))
```

```sortie
6
1
piedra
```

- `randint(1, 6)`: un entero entre 1 y 6, **ambos incluidos**;
- `choice(lista)`: un elemento de la lista;
- `random()`: un flotante entre 0 y 1.

> **Consejo:** quita `random.seed(42)` en un juego de verdad; si no, el dado dará
> siempre la misma secuencia. Mantenlo para las pruebas.

## Convertir entre texto y números

Tres funciones convierten un valor de un tipo a otro:

- `int(x)`: a entero;
- `float(x)`: a flotante;
- `str(x)`: a texto.

```python
texto = "42"
numero = int(texto)
print(numero + 1)
print(float("3.5") * 2)
print(str(100) + " puntos")
print(int(7.9))
```

```sortie
43
7.0
100 puntos
7
```

Cuidado con `int(7.9)`: **corta** la parte decimal, no redondea. Para redondear, usa
`round(7.9)`, que da `8`.

## Mostrar los números de forma bonita

En una f-string, añade `:` y luego un **formato** después del valor:

- `:.2f`: 2 cifras después del punto decimal (*f* de *fixed*, coma fija);
- `:,`: un separador de miles;
- `:%`: un porcentaje;
- `:5`: un ancho de 5 caracteres.

```python
precio = 3.14159
poblacion = 68373433
tasa = 0.256

print(f"{precio:.2f}")
print(f"{poblacion:,}")
print(f"{poblacion:_}")
print(f"{tasa:.1%}")
print(f"[{42:5}]")
print(f"{1234567.891:,.2f}")
```

```sortie
3.14
68,373,433
68_373_433
25.6%
[   42]
1,234,567.89
```

> **Nota:** el formato `,` usa la coma inglesa. Para mostrar los miles con puntos, como
> en español (`68.373.433`), reemplázala: `f"{poblacion:,}".replace(",", ".")`.

## Errores frecuentes

### Convertir un texto con decimales usando int

```python norun
n = int("3.5")
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 1, in <module>
    n = int("3.5")
ValueError: invalid literal for int() with base 10: '3.5'
```

`int` solo acepta un texto que contenga un entero. Para `"3.5"`, usa `float("3.5")`. Y
para obtener el entero: `int(float("3.5"))`.

### Dividir entre cero

```python norun
print(10 / 0)
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 1, in <module>
    print(10 / 0)
          ~~~^~~
ZeroDivisionError: division by zero
```

Dividir entre cero no tiene sentido. Comprueba el divisor antes: `if divisor != 0:`.
Las condiciones se explican en [condiciones y bucles](conditions-boucles.md).

### Olvidar import math

```python norun
print(math.sqrt(16))
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 1, in <module>
    print(math.sqrt(16))
          ^^^^
NameError: name 'math' is not defined. Did you forget to import 'math'?
```

Python solo conoce `math` después de `import math`. Añade esta línea al principio del
archivo.

### Comparar dos flotantes con ==

```python norun
print(0.1 * 3 == 0.3)
```

```text
False
```

No hay error, pero el resultado sorprende: `0.1 * 3` vale `0.30000000000000004`.
Compara con `math.isclose(0.1 * 3, 0.3)`, que responde `True`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto valen `20 / 4`, `20 // 6` y `20 % 6`?

> **Respuesta**
>
> `5.0` (un `float`, siempre con `/`), `3` (división entera) y `2` (el resto:
> 20 = 3 × 6 + 2).

**Pregunta 2:** convierte 200 segundos en minutos y segundos, y muestra
`3 min 20 s`.

> **Respuesta**
>
> ```python
> segundos = 200
> print(f"{segundos // 60} min {segundos % 60} s")
> ```
>
> ```sortie
> 3 min 20 s
> ```

**Pregunta 3:** ¿cómo se muestra `1/3` con 3 decimales?

> **Respuesta**
>
> ```python
> print(f"{1 / 3:.3f}")
> ```
>
> ```sortie
> 0.333
> ```

## En resumen

- `/` siempre da un `float`; `//` da la división entera y `%` el resto.
- `**` es la potencia; los paréntesis fijan el orden de los cálculos.
- Los `int` no tienen límite; los `float` son aproximados: redondea con `round` o
  compara con `math.isclose`.
- `Decimal` y `Fraction` calculan de forma exacta.
- `int()`, `float()` y `str()` convierten; `import math` e `import random` abren cajas
  de herramientas.
- `f"{x:.2f}"` y `f"{n:,}"` muestran los números de forma limpia.

Siguiente: [condiciones y bucles](conditions-boucles.md).
