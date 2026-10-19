# Gestionar los errores

Esta página explica cómo leer un mensaje de error de Python, cómo atrapar los errores con `try` y `except`, cómo lanzarlos con `raise` y cómo `with` garantiza la limpieza.

> **Objetivos:**
>
> - sabrás leer una traza de error línea por línea;
> - sabrás atrapar un error concreto con `try` y `except`;
> - sabrás usar `else` y `finally`;
> - sabrás lanzar tus propios errores con `raise` y clases de excepción;
> - sabrás usar `with` y elegir entre «pedir perdón» y «pedir permiso».

## Un error detiene el programa

Cuando Python se encuentra con un problema, lanza una
[excepción](../debuter/glossaire.md#excepcion): un objeto que describe el
[error](../debuter/glossaire.md#error). «Lanzar» quiere decir: hacer sonar la alarma.
Si nadie se ocupa de ella, el programa se detiene y muestra una **traza** (*traceback*).

```python norun
def promedio(notas):
    return sum(notas) / len(notas)

def boletin(alumno, notas):
    m = promedio(notas)
    return f"{alumno}: {m:.1f}"

print(boletin("Alice", [12, 15]))
print(boletin("Bob", []))
```

```text
Alice: 13.5
Traceback (most recent call last):
  File "main.py", line 9, in <module>
    print(boletin("Bob", []))
          ~~~~~~~^^^^^^^^^^^
  File "main.py", line 5, in boletin
    m = promedio(notas)
  File "main.py", line 2, in promedio
    return sum(notas) / len(notas)
           ~~~~~~~~~~~^~~~~~~~~~~~
ZeroDivisionError: division by zero
```

La primera línea sí se mostró: el programa se detiene **en el momento** del error,
no antes.

### Leer la traza línea por línea

Una traza se lee **de abajo arriba**: la causa está al final.

1. `ZeroDivisionError: division by zero`: la **clase** del error (su apellido),
   y luego su descripción, «división por cero». Empieza siempre por ahí.
2. `File "main.py", line 2, in promedio`: el lugar exacto, en la función `promedio`.
   Los `~~~^~~~` subrayan la operación culpable: la división.
3. `line 5, in boletin`: a `promedio` la había llamado `boletin`, en la línea 5.
4. `line 9, in <module>`: y a `boletin`, el programa principal (`<module>`),
   en la línea 9.

`Traceback (most recent call last)` lo recuerda: la llamada más reciente va al final.
Aquí se entiende rápido: `boletin` recibió una lista vacía, y `len([])` vale 0.

> **Nota:** en tu ordenador, `File` muestra la ruta completa, por ejemplo
> `File "/home/tu/proyecto/main.py"`. En esta guía se acorta a `main.py`.

## try y except

`try` («intentar») y `except` («salvo si») atrapan un error. El programa sigue en lugar
de detenerse. Es la red bajo un trapecista: la caída se detiene, el espectáculo
continúa.

- El código arriesgado va en el bloque `try`.
- El código de emergencia va en el bloque `except`, con la clase de error esperada.

```python
try:
    resultado = 10 / 0
    print("Esta línea nunca se alcanza")
except ZeroDivisionError:
    print("¡No se divide por cero!")
print("El programa continúa.")
```

```sortie
¡No se divide por cero!
El programa continúa.
```

En cuanto ocurre un error en `try`, Python salta directamente al `except`: el resto del
bloque `try` se abandona.

### Obtener el mensaje

Con `as e`, obtienes el objeto excepción. `str(e)` (o `print(e)`) da su mensaje.

```python
try:
    edad = int("doce")
except ValueError as e:
    print("Error:", e)
```

```sortie
Error: invalid literal for int() with base 10: 'doce'
```

### Varios except

Se pueden escribir varios `except`, uno por clase de error, o agrupar varias en una
tupla.

```python
def dividir(texto_a, texto_b):
    try:
        return int(texto_a) / int(texto_b)
    except ValueError:
        return "No es un número."
    except ZeroDivisionError:
        return "División por cero."

print(dividir("10", "4"))
print(dividir("diez", "4"))
print(dividir("10", "0"))
```

```sortie
2.5
No es un número.
División por cero.
```

> **Atención:** atrapa errores **concretos**. Un `except:` solo, o
> `except Exception:`, lo atrapa todo, incluso tus errores al teclear: un bug se vuelve
> entonces invisible.

## else y finally

Dos bloques opcionales completan `try`:

- `else` se ejecuta solo si **no** hubo error;
- `finally` se ejecuta **siempre**, haya error o no. Sirve para limpiar.

```python
def leer_edad(texto):
    try:
        edad = int(texto)
    except ValueError:
        print(f"{texto!r}: no es un número")
    else:
        print(f"{texto!r}: {edad} años")
    finally:
        print("  (comprobación terminada)")

leer_edad("42")
leer_edad("viejo")
```

```sortie
'42': 42 años
  (comprobación terminada)
'viejo': no es un número
  (comprobación terminada)
```

¿Por qué `else` en lugar de poner el `print` dentro de `try`? Para dejar en `try`
**solo** la línea arriesgada: así no atrapas por error un problema que viene de otro
sitio.

## Lanzar un error con raise

`raise` lanza un error tú mismo, por ejemplo para rechazar un valor absurdo. Indica la
clase del error y un mensaje claro.

```python
def retirar(saldo, cantidad):
    if cantidad <= 0:
        raise ValueError(f"cantidad no válida: {cantidad}")
    if cantidad > saldo:
        raise ValueError("saldo insuficiente")
    return saldo - cantidad

print(retirar(100, 30))
try:
    retirar(100, -5)
except ValueError as e:
    print("Rechazado:", e)
```

```sortie
70
Rechazado: cantidad no válida: -5
```

Lanzar un error en lugar de devolver un valor raro (`-1`, `None`…) obliga a quien llama
a ocuparse de él: el error no puede pasar desapercibido.

### Relanzar un error

Dentro de un `except`, un `raise` solo relanza el error atrapado, después de hacer algo
(por ejemplo, anotar el problema).

```python
try:
    try:
        int("abc")
    except ValueError:
        print("Anoto el problema y luego lo transmito.")
        raise
except ValueError as e:
    print("Atrapado más arriba:", e)
```

```sortie
Anoto el problema y luego lo transmito.
Atrapado más arriba: invalid literal for int() with base 10: 'abc'
```

## Tus propias excepciones

Una excepción es un objeto como cualquier otro, de una [clase](types.md) que hereda de
`Exception` (ver [la herencia](heritage.md)). Crear tu propia clase da un nombre claro
a tus errores, que quien llama puede atrapar con precisión.

```python
class SaldoInsuficienteError(Exception):
    def __init__(self, saldo, cantidad):
        super().__init__(f"faltan {cantidad - saldo} €")
        self.falta = cantidad - saldo

class Cuenta:
    def __init__(self, saldo):
        self.saldo = saldo

    def retirar(self, cantidad):
        if cantidad > self.saldo:
            raise SaldoInsuficienteError(self.saldo, cantidad)
        self.saldo -= cantidad

cuenta = Cuenta(50)
try:
    cuenta.retirar(80)
except SaldoInsuficienteError as e:
    print("Rechazado:", e)
    print("Falta:", e.falta)
```

```sortie
Rechazado: faltan 30 €
Falta: 30
```

> **Recuerda:** hereda de `Exception` y termina el nombre en `Error`. Para una
> excepción muy sencilla, basta con `class MiError(Exception): pass`.

## assert: comprobar una suposición

`assert condición, "mensaje"` lanza un `AssertionError` si la condición es falsa.
Sirve para comprobar lo que **debe** ser cierto si tu código es correcto: es una
protección para ti, el desarrollador.

```python
def porcentaje(parte, total):
    assert total > 0, "total debe ser positivo"
    return parte / total * 100

print(porcentaje(1, 4))
try:
    porcentaje(1, 0)
except AssertionError as e:
    print("Bug detectado:", e)
```

```sortie
25.0
Bug detectado: total debe ser positivo
```

> **Atención:** no valides lo que escribe el usuario con `assert`: las aserciones se
> pueden desactivar (`python3 -O`). Para eso, usa `raise`.

## Los gestores de contexto: with

Algunos recursos deben **liberarse** después de usarlos: un archivo abierto debe
cerrarse, aunque ocurra un error. `with` se encarga de ello por ti.

```python
with open("notas.txt", "w") as f:
    f.write("15\n12\n")

with open("notas.txt") as f:
    print(f.read().split())
print(f.closed)
```

```sortie
['15', '12']
True
```

Al salir del bloque `with`, el archivo se cierra automáticamente, haya habido un error
o no. Es un `try … finally` ya hecho. Los archivos se explican en detalle en
[leer y escribir archivos](fichiers.md).

### Escribir tu propio gestor

El módulo `contextlib` permite escribir uno con un generador (ver
[iteradores y generadores](iterateurs.md)): el código antes de `yield` se ejecuta a la
entrada, y el código de después, en `finally`, a la salida.

```python
from contextlib import contextmanager

@contextmanager
def seccion(titulo):
    print(f"[inicio {titulo}]")
    try:
        yield
    finally:
        print(f"[fin {titulo}]")

with seccion("cálculo"):
    print(2 + 2)

try:
    with seccion("arriesgado"):
        1 / 0
except ZeroDivisionError:
    print("error atrapado")
```

```sortie
[inicio cálculo]
4
[fin cálculo]
[inicio arriesgado]
[fin arriesgado]
error atrapado
```

Incluso con el error, `[fin arriesgado]` se muestra: la limpieza está garantizada.

## ¿Pedir perdón o pedir permiso?

Dos formas de evitar un error:

- **LBYL** (*Look Before You Leap*, «mira antes de saltar»): se comprueba primero
  con un `if`;
- **EAFP** (*Easier to Ask Forgiveness than Permission*, «es más fácil pedir perdón
  que permiso»): se intenta, y se atrapa el error si falla.

```python
stock = {"manzana": 3}

# LBYL: se comprueba antes
if "pera" in stock:
    print(stock["pera"])
else:
    print("no hay peras (LBYL)")

# EAFP: se intenta
try:
    print(stock["pera"])
except KeyError:
    print("no hay peras (EAFP)")
```

```sortie
no hay peras (LBYL)
no hay peras (EAFP)
```

El estilo EAFP es muy habitual en Python, sobre todo cuando la comprobación sería
complicada (¿un texto es un número válido?) o cuando la situación puede cambiar entre
la prueba y la acción (un archivo borrado mientras tanto). Para un caso sencillo como
una clave de diccionario, `stock.get("pera", 0)` es aún más corto.

## Las excepciones habituales

| Clase | ¿Cuándo? |
|--------|---------|
| `NameError` | variable desconocida (a menudo un error al teclear) |
| `TypeError` | tipo incorrecto, por ejemplo `"a" + 1` |
| `ValueError` | tipo correcto, valor incorrecto: `int("abc")` |
| `IndexError` | índice fuera de la lista: `[1, 2][5]` |
| `KeyError` | clave ausente de un diccionario |
| `AttributeError` | atributo o método desconocido: `(5).upper()` |
| `ZeroDivisionError` | división por cero |
| `FileNotFoundError` | archivo no encontrado |
| `ImportError` / `ModuleNotFoundError` | módulo no encontrado |
| `StopIteration` | iterador agotado |
| `AssertionError` | `assert` cuya condición es falsa |

Todas heredan de `Exception`. Atrapar una clase padre atrapa también a sus hijas:
`except LookupError` atrapa a la vez `IndexError` y `KeyError`.

## Errores frecuentes

### Atrapar la clase de error equivocada

```python norun
try:
    edad = int("doce")
except ZeroDivisionError:
    print("Problema")
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    edad = int("doce")
ValueError: invalid literal for int() with base 10: 'doce'
```

`except ZeroDivisionError` solo atrapa las divisiones por cero: el `ValueError` pasa
de largo. Copia la clase escrita al final de la traza: `except ValueError:`.

### Lanzar una cadena en lugar de una excepción

```python norun
def retirar(saldo, cantidad):
    if cantidad > saldo:
        raise "saldo insuficiente"
    return saldo - cantidad

retirar(10, 50)
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    retirar(10, 50)
    ~~~~~~~^^^^^^^^
  File "main.py", line 3, in retirar
    raise "saldo insuficiente"
TypeError: exceptions must derive from BaseException
```

Solo se lanzan objetos excepción. Escribe `raise ValueError("saldo insuficiente")`.

### Abrir un archivo que no existe

```python norun
with open("notas.txt") as f:
    print(f.read())
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    with open("notas.txt") as f:
         ~~~~^^^^^^^^^^^^^
FileNotFoundError: [Errno 2] No such file or directory: 'notas.txt'
```

El archivo no está en la carpeta desde la que lanzas el programa. Comprueba el nombre y
la carpeta, o atrapa `FileNotFoundError` para mostrar un mensaje claro.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```python norun
try:
    print("A")
    int("x")
    print("B")
except ValueError:
    print("C")
else:
    print("D")
finally:
    print("E")
```

> **Respuesta**
>
> `A`, `C` y luego `E`. `B` se salta por culpa del error, y `D` solo se muestra si no
> hay error. `finally` se ejecuta siempre.

**Pregunta 2:** en una traza, ¿dónde encuentras la causa del error?

> **Respuesta**
>
> En la **última** línea (clase y mensaje), y justo encima: el archivo y la línea donde
> se produjo el error.

**Pregunta 3:** escribe una función `edad_valida(n)` que lance un `ValueError` si `n`
es negativo, y que devuelva `n` en caso contrario.

> **Respuesta**
>
> ```python
> def edad_valida(n):
>     if n < 0:
>         raise ValueError("edad negativa")
>     return n
>
> print(edad_valida(12))
> try:
>     edad_valida(-1)
> except ValueError as e:
>     print(e)
> ```
>
> ```sortie
> 12
> edad negativa
> ```

## En resumen

- Una traza se lee de abajo arriba: clase y mensaje, y luego la línea culpable.
- `try` / `except Clase as e` atrapa un error concreto; `else` se ejecuta si no hay
  error, `finally` siempre.
- `raise` lanza un error; tus excepciones heredan de `Exception`.
- `assert` comprueba tus suposiciones de desarrollador, no lo que escribe el usuario.
- `with` libera un recurso automáticamente, incluso si ocurre un error.
- En Python, a menudo se intenta primero y se atrapa el error después (EAFP).

Siguiente: [los gestores de contexto](contextes.md). Compara con
[los errores en Ruby](../ruby/erreurs.md).
