# Los gestores de contexto

Esta página explica cómo `with` abre y libera un recurso con total seguridad, y cómo escribir tus propios gestores de contexto con una clase o con `contextlib`.

> **Objetivos:**
>
> - sabrás por qué `with` es más seguro que un `close()` escrito a mano;
> - sabrás abrir varios recursos en un solo `with`;
> - sabrás escribir un gestor con una clase, `__enter__` y `__exit__`;
> - sabrás escribir un gestor con `@contextmanager` y `yield`;
> - sabrás usar `suppress`, `redirect_stdout`, `ExitStack` y
>   `TemporaryDirectory`.

## El problema: olvidar limpiar

Algunas cosas deben **devolverse** después de usarlas: un archivo abierto debe cerrarse,
una conexión cortarse, un cerrojo soltarse. A esto se le llama un **recurso**.

Cerrar a mano parece sencillo:

```python
f = open("notas.txt", "w", encoding="utf-8")
f.write("15\n")
f.close()
print(f.closed)
```

```sortie
True
```

Pero si ocurre un [error](../debuter/glossaire.md#error) entre `open` y `close`, la
línea `f.close()` nunca se alcanza:

```python
f = open("notas.txt", "w", encoding="utf-8")
try:
    f.write("15\n")
    1 / 0
    f.close()
except ZeroDivisionError:
    print("¡error!")
print("¿cerrado?", f.closed)
```

```sortie
¡error!
¿cerrado? False
```

El archivo sigue abierto. En un programa real, los archivos olvidados acaban bloqueando
el sistema, o hay datos que nunca se escriben en el disco.

## La solución: with

`with` garantiza la limpieza. Al salir del bloque con sangría, el recurso se libera,
**haya habido un error o no**. Es como una puerta que se cierra sola detrás de ti.

```python
try:
    with open("notas.txt", "w", encoding="utf-8") as f:
        f.write("15\n")
        1 / 0
except ZeroDivisionError:
    print("¡error!")
print("¿cerrado?", f.closed)
```

```sortie
¡error!
¿cerrado? True
```

Esta vez, el archivo se cierra a pesar del error. El objeto que va después de `with`
(aquí, el archivo) se llama un **gestor de contexto**: sabe qué hacer al entrar y al
salir del bloque.

> **Recuerda:** `with` no sustituye a `try` / `except`. No atrapa el error: limpia, y
> luego deja que el error siga su camino.

## with open(...) as f

Es el uso más habitual, visto en [leer y escribir archivos](fichiers.md). Lee la línea
de izquierda a derecha:

- `open(...)` crea el gestor de contexto;
- `as f` da un nombre a lo que proporciona (el archivo abierto);
- el bloque con sangría es el momento en que el archivo está abierto.

```python
with open("frutas.txt", "w", encoding="utf-8") as f:
    f.write("manzana\npera\n")

with open("frutas.txt", encoding="utf-8") as f:
    for linea in f:
        print(linea.strip())

print(f.closed)
```

```sortie
manzana
pera
True
```

La variable `f` sigue existiendo después del bloque, pero el archivo está cerrado: ya no
se puede leer.

## Varios contextos en un solo with

Para abrir dos archivos a la vez, sepáralos con una coma. Se abren de izquierda a
derecha y se cierran en el orden inverso.

```python
with open("origen.txt", "w", encoding="utf-8") as f:
    f.write("hola\nadiós\n")

with open("origen.txt", encoding="utf-8") as entrada, \
     open("copia.txt", "w", encoding="utf-8") as salida:
    for linea in entrada:
        salida.write(linea.upper())

with open("copia.txt", encoding="utf-8") as f:
    print(f.read(), end="")
```

```sortie
HOLA
ADIÓS
```

## Escribir un gestor con una clase

Cualquier objeto puede ir después de `with`, si tiene dos métodos especiales:

- `__enter__(self)` se llama al **entrar** en el bloque. Lo que devuelve se guarda en
  la variable que va después de `as`;
- `__exit__(self, tipo_err, err, traza)` se llama al **salir** del bloque, siempre.

```python
class Seccion:
    def __init__(self, titulo):
        self.titulo = titulo

    def __enter__(self):
        print(f"[inicio {self.titulo}]")
        return self

    def __exit__(self, tipo_err, err, traza):
        print(f"[fin {self.titulo}]")

with Seccion("cálculo") as s:
    print("dentro del bloque:", s.titulo)
    print(2 + 2)
```

```sortie
[inicio cálculo]
dentro del bloque: cálculo
4
[fin cálculo]
```

Python hace exactamente esto: llama a `__enter__`, ejecuta el bloque y luego llama a
`__exit__`, aunque el bloque haya lanzado una
[excepción](../debuter/glossaire.md#excepcion).

> **Atención:** no olvides `return self` en `__enter__`. Sin `return`, devuelve
> `None`, y la variable que va después de `as` vale `None`.

### Lo que recibe __exit__

`__exit__` recibe tres argumentos que describen el posible error:

| Argumento | Si no hay error | Si hay un error |
|-----------|-----------------|-----------------|
| `tipo_err` | `None` | la clase, por ejemplo `ZeroDivisionError` |
| `err` | `None` | el propio objeto excepción |
| `traza` | `None` | la traza (*traceback*) |

```python
class Espia:
    def __enter__(self):
        return self

    def __exit__(self, tipo_err, err, traza):
        if tipo_err is None:
            print("salida normal")
        else:
            print("salida por error:", tipo_err.__name__, "-", err)

with Espia():
    print("todo va bien")

try:
    with Espia():
        int("abc")
except ValueError:
    print("el error siguió su camino")
```

```sortie
todo va bien
salida normal
salida por error: ValueError - invalid literal for int() with base 10: 'abc'
el error siguió su camino
```

`__exit__` **ve** el error, pero no lo detiene: después sube hasta el `except`.

### Devolver True para tragarse el error

El valor que devuelve `__exit__` cuenta:

- si devuelve un valor **verdadero** (`True`), Python considera el error resuelto:
  desaparece, y el programa continúa **después** del bloque `with`;
- si devuelve un valor falso (`False`, o `None` cuando no hay `return`), el error sigue
  subiendo.

```python
class IgnorarCero:
    def __enter__(self):
        return self

    def __exit__(self, tipo_err, err, traza):
        if tipo_err is ZeroDivisionError:
            print("división por cero ignorada")
            return True
        return False

with IgnorarCero():
    print("antes")
    1 / 0
    print("nunca se muestra")
print("después del with")
```

```sortie
antes
división por cero ignorada
después del with
```

Fíjate bien en dos cosas:

1. `"nunca se muestra"` no se muestra. Tragarse el error no «reanuda» el bloque donde se
   había detenido: el resto del bloque se pierde.
2. Solo se traga `ZeroDivisionError`. Cualquier otro error, como `ValueError`, recibe
   `return False` y sube con normalidad.

> **Atención:** trágate solo los errores **concretos** que esperas. Un `__exit__` que
> siempre devuelve `True` esconde todos los fallos del bloque, como un `except:` solo.

## contextmanager: un gestor con yield

Escribir una clase para cada gestor es un poco largo. El
[decorador](../debuter/glossaire.md#decorador) `@contextmanager` del módulo
`contextlib` convierte un [generador](../debuter/glossaire.md#generador) en un gestor:

- el código **antes** de `yield` hace el papel de `__enter__`;
- el valor que se da a `yield` se guarda en la variable que va después de `as`;
- el código **después** de `yield` hace el papel de `__exit__`.

```python
from contextlib import contextmanager

@contextmanager
def etiqueta(nombre):
    print(f"<{nombre}>")
    yield nombre.upper()
    print(f"</{nombre}>")

with etiqueta("p") as e:
    print("  hola", e)
```

```sortie
<p>
  hola P
</p>
```

### Siempre con try / finally

Si ocurre un error en el bloque `with`, Python lo vuelve a lanzar **en el lugar del
`yield`**. Así que, sin precaución, el código después de `yield` no se ejecuta. Pon la
limpieza en un `finally`:

```python
from contextlib import contextmanager

@contextmanager
def etiqueta(nombre):
    print(f"<{nombre}>")
    try:
        yield
    finally:
        print(f"</{nombre}>")

try:
    with etiqueta("div"):
        print("  contenido")
        1 / 0
except ZeroDivisionError:
    print("error atrapado")
```

```sortie
<div>
  contenido
</div>
error atrapado
```

`</div>` se muestra a pesar del error: la limpieza está garantizada.

> **Recuerda:** un generador decorado con `@contextmanager` debe hacer **un solo**
> `yield`, rodeado de `try` / `finally`.

## suppress: ignorar un error concreto

`contextlib.suppress` («suprimir») hace lo que hacía nuestra clase `IgnorarCero`: se
traga los errores de las clases indicadas, y solo esos.

```python
from contextlib import suppress
from pathlib import Path

with suppress(FileNotFoundError):
    Path("borrador.txt").unlink()
    print("archivo borrado")
print("seguimos")
```

```sortie
seguimos
```

El archivo no existía: `unlink` lanzó `FileNotFoundError`, `suppress` se lo tragó, y la
línea `"archivo borrado"` se saltó. Es un atajo para un `try` /
`except FileNotFoundError: pass`.

## redirect_stdout: capturar lo que muestra print

`print` escribe en la **salida estándar** (*stdout*), normalmente la pantalla.
`contextlib.redirect_stdout` la envía a otro sitio mientras dura el bloque. Con
`io.StringIO`, una especie de archivo guardado en memoria, recuperas el texto en una
cadena.

```python
import io
from contextlib import redirect_stdout

def saludar():
    print("¡Hola!")
    print("¡Adiós!")

bufer = io.StringIO()
with redirect_stdout(bufer):
    saludar()

texto = bufer.getvalue()
print("capturado:", repr(texto))
print(len(texto.splitlines()), "líneas")
```

```sortie
capturado: '¡Hola!\n¡Adiós!\n'
2 líneas
```

Durante el bloque, no se muestra nada en pantalla. Al salir, `print` vuelve a escribir
con normalidad. Es muy práctico para [probar](tests.md) una función que muestra texto.

## ExitStack: un número variable de contextos

Cuando el número de archivos solo se conoce durante la ejecución, no se pueden escribir
todos en el `with`. `contextlib.ExitStack` («pila de salidas») los va acumulando con
`enter_context` y luego los cierra todos al final del bloque, en el orden inverso.

```python
from contextlib import ExitStack

nombres = ["lunes.txt", "martes.txt", "miercoles.txt"]

with ExitStack() as pila:
    archivos = [pila.enter_context(open(nombre, "w", encoding="utf-8")) for nombre in nombres]
    for nombre, f in zip(nombres, archivos):
        f.write(f"diario de {nombre}\n")
    print(len(archivos), "archivos abiertos")

print(all(f.closed for f in archivos))
```

```sortie
3 archivos abiertos
True
```

> **Nota:** no necesitarás `ExitStack` a menudo. Simplemente recuerda que existe para
> el día en que el número de recursos varíe.

## Ejemplo práctico: una carpeta temporal

`tempfile.TemporaryDirectory` crea una carpeta vacía, con un nombre elegido al azar, y
la **borra con todo su contenido** al salir del bloque. Es ideal para archivos de
trabajo desechables.

```python
import tempfile
from pathlib import Path

with tempfile.TemporaryDirectory() as nombre_carpeta:
    carpeta = Path(nombre_carpeta)
    (carpeta / "a.txt").write_text("uno", encoding="utf-8")
    (carpeta / "b.txt").write_text("dos", encoding="utf-8")
    print(sorted(f.name for f in carpeta.iterdir()))
    print("existe:", carpeta.exists())

print("existe después:", carpeta.exists())
```

```sortie
['a.txt', 'b.txt']
existe: True
existe después: False
```

El nombre de la carpeta cambia cada vez (por ejemplo `/tmp/tmpk3x9a1`), así que no se
muestra. Solo se muestra lo que no depende del azar.

## Errores frecuentes

### Usar with con un objeto que no es un gestor

```python norun
with "notas.txt" as f:
    print(f.read())
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    with "notas.txt" as f:
         ^^^^^^^^^^^
TypeError: 'str' object does not support the context manager protocol
```

Una cadena no tiene ni `__enter__` ni `__exit__`. Falta `open`:
`with open("notas.txt", encoding="utf-8") as f:`.

### Olvidar return self en __enter__

```python norun
class Contador:
    def __init__(self):
        self.total = 0

    def __enter__(self):
        print("inicio")

    def __exit__(self, tipo_err, err, traza):
        print("fin")

with Contador() as c:
    c.total += 1
```

```text
inicio
fin
Traceback (most recent call last):
  File "main.py", line 12, in <module>
    c.total += 1
    ^^^^^^^
AttributeError: 'NoneType' object has no attribute 'total'
```

`__enter__` no devuelve nada, así que `c` vale `None`. Añade `return self` al final de
`__enter__`.

### Olvidar los parámetros de __exit__

```python norun
class Seccion:
    def __enter__(self):
        return self

    def __exit__(self):
        print("fin")

with Seccion():
    print("dentro")
```

```text
dentro
Traceback (most recent call last):
  File "main.py", line 8, in <module>
    with Seccion():
         ~~~~~~~^^
TypeError: Seccion.__exit__() takes 1 positional argument but 4 were given
```

Python siempre llama a `__exit__` con tres argumentos además de `self`. Escribe
`def __exit__(self, tipo_err, err, traza):`, aunque no los uses.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```python norun
class Puerta:
    def __enter__(self):
        print("abre")
        return self

    def __exit__(self, tipo_err, err, traza):
        print("cierra")

try:
    with Puerta():
        print("entra")
        raise ValueError("uy")
except ValueError:
    print("atrapado")
```

> **Respuesta**
>
> `abre`, `entra`, `cierra` y luego `atrapado`. `__exit__` se llama antes de que el
> error llegue al `except`. No devuelve nada (por tanto `None`, un valor falso): el
> error no se traga.

**Pregunta 2:** en una función decorada con `@contextmanager`, ¿por qué rodear el
`yield` con `try` / `finally`?

> **Respuesta**
>
> Porque un error en el bloque `with` se vuelve a lanzar en el lugar del `yield`. Sin
> `finally`, el código de limpieza colocado después de `yield` no se ejecutaría.

**Pregunta 3:** escribe con `suppress` un código que convierta `"abc"` en entero y
muestre `"terminado"` sin fallar.

> **Respuesta**
>
> ```python
> from contextlib import suppress
>
> with suppress(ValueError):
>     n = int("abc")
>     print("convertido:", n)
> print("terminado")
> ```
>
> ```sortie
> terminado
> ```

## En resumen

- `with` libera un recurso al salir del bloque, incluso si hay un error; no atrapa el
  error.
- `with a as x, b as y:` abre varios recursos, que se cierran en el orden inverso.
- Una clase se convierte en gestor con `__enter__` (su valor va a `as`) y
  `__exit__(self, tipo_err, err, traza)`.
- Si `__exit__` devuelve `True`, el error se traga y el programa sigue después del
  bloque; si no, sube.
- `@contextmanager`: el código antes de `yield` a la entrada, después de `yield` (en
  `finally`) a la salida.
- `contextlib` también ofrece `suppress`, `redirect_stdout` y `ExitStack`;
  `tempfile.TemporaryDirectory` crea una carpeta desechable.

Siguiente: [los módulos](modules.md).
