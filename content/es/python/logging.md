# Registrar con logging

Esta página muestra cómo sustituir los `print` de control por un registro de verdad con el módulo `logging`: niveles de importancia, un formato fijo, un registro por módulo, un archivo y excepciones bien contadas.

> **Objetivos:**
>
> - sabrás por qué un registro es mejor que unos `print`;
> - sabrás usar los cinco niveles, de `DEBUG` a `CRITICAL`;
> - sabrás configurar la visualización una sola vez con `basicConfig`;
> - sabrás crear un registro por módulo con `getLogger`;
> - sabrás escribir el registro en un archivo;
> - sabrás guardar una excepción con su traza.

## ¿Por qué no print?

El capitán de un barco lleva un **cuaderno de bitácora**. Apunta la hora, el tiempo,
los incidentes. No lo escribe todo con la misma tinta: un incidente grave se subraya en
rojo. Y meses después, se vuelve a leer el cuaderno para entender lo que pasó.

Un programa puede llevar el mismo tipo de registro. Con `print`, escribes mensajes, pero:

- todos son iguales: imposible saber cuál es grave;
- para ocultarlos, hay que borrarlos uno por uno;
- solo van a la pantalla, nunca a un archivo;
- no dicen de dónde vienen.

El módulo `logging` resuelve estos cuatro problemas.

```text
     print("ok")                       log.info("ok")
         │                                  │
         ▼                                  ▼
    la pantalla                   ┌─ ¿qué nivel? ───┐
                                  │ ¿bastante grave?│── no ──► ignorado
                                  └────────┬────────┘
                                           │ sí
                                   ┌───────┴────────┐
                                   ▼                ▼
                              la pantalla      un archivo
```

El capítulo [depurar](debogage.md) dio un primer vistazo. Aquí vamos más lejos.

> **Recuerda:** `print` sirve para mostrar el **resultado** al usuario. `logging` sirve
> para contar **lo que hace** el programa, para ti o para la persona que lo vigila.

## Un primer registro

Tres líneas bastan para empezar.

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("tienda")

log.info("apertura de la tienda")
log.warning("solo quedan 2 barras de pan")
print("¡Hola!")
```

```sortie
INFO:tienda:apertura de la tienda
WARNING:tienda:solo quedan 2 barras de pan
¡Hola!
```

Línea por línea:

| Código | Lo que hace |
|---|---|
| `logging.basicConfig(...)` | configura el registro, **una vez**, al principio del programa |
| `stream=sys.stdout` | los mensajes van a la salida normal (por defecto: la salida de errores) |
| `level=logging.INFO` | se guardan los mensajes `INFO` y los más graves |
| `format="..."` | la forma de cada línea (ver más abajo) |
| `logging.getLogger("tienda")` | toma un registro llamado `tienda` |
| `log.info(...)`, `log.warning(...)` | escribe un mensaje, con su nivel |

> **Nota:** en esta página, `stream=sys.stdout` está ahí para que veas los mensajes
> mezclados con los `print`. En un programa real, se suele dejar la salida de errores
> por defecto: así el registro no se mezcla con el resultado.

## Los cinco niveles

Cada mensaje tiene un **nivel**: su grado de gravedad. Es como las alertas
meteorológicas, del verde al rojo.

| Nivel | Número | Cuándo usarlo | Ejemplo |
|---|---|---|---|
| `DEBUG` | 10 | los pequeños detalles para la puesta a punto | «x vale 3» |
| `INFO` | 20 | los pasos normales | «archivo cargado» |
| `WARNING` | 30 | raro, pero seguimos | «stock bajo» |
| `ERROR` | 40 | una operación ha fallado | «pago rechazado» |
| `CRITICAL` | 50 | el programa ya no puede continuar | «disco lleno» |

El ajuste `level` es un **umbral**. Un mensaje pasa si es **al menos tan grave** como el
umbral.

```text
 umbral = WARNING (30)

 CRITICAL 50  ████████████  pasa
 ERROR    40  ██████████    pasa
 WARNING  30  ████████      pasa      ◄── umbral
 INFO     20  ██████        ignorado
 DEBUG    10  ████          ignorado
```

Mira el mismo programa con el umbral `WARNING`:

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.WARNING,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("meteo")

log.debug("sensor leído: 18.4")
log.info("medición terminada")
log.warning("viento fuerte")
log.error("pluviómetro averiado")
log.critical("estación destruida")
```

```sortie
WARNING:meteo:viento fuerte
ERROR:meteo:pluviómetro averiado
CRITICAL:meteo:estación destruida
```

Sigamos cada mensaje:

| Mensaje | Nivel | Umbral | ¿30 ≤ nivel? | ¿Se muestra? |
|---|---|---|---|---|
| sensor leído | 10 | 30 | no | no |
| medición terminada | 20 | 30 | no | no |
| viento fuerte | 30 | 30 | sí | sí |
| pluviómetro averiado | 40 | 30 | sí | sí |
| estación destruida | 50 | 30 | sí | sí |

> **Pruébalo:** sustituye `level=logging.WARNING` por `level=logging.DEBUG`. Se
> muestran las cinco líneas. Con `level=logging.CRITICAL`, solo queda una.

Sin `level`, el umbral por defecto es `WARNING`. Por eso tus `log.info` a veces parecen
«no hacer nada».

> **Recuerda:** escribes **todos** los mensajes en el código, cada uno con el nivel
> adecuado. Luego **un solo** ajuste, `level`, elige los que ves. Ya no borras líneas
> para hacer limpieza.

## El formato de una línea

El `format` es una plantilla con huecos. Cada `%(nombre)s` se sustituye por una
información sobre el mensaje.

| Hueco | Sustituido por |
|---|---|
| `%(levelname)s` | el nivel: `INFO`, `ERROR`… |
| `%(name)s` | el nombre del registro: `tienda` |
| `%(message)s` | el texto del mensaje |
| `%(funcName)s` | el nombre de la función que escribió |
| `%(lineno)d` | el número de línea |
| `%(asctime)s` | la fecha y la hora |

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.DEBUG,
                    format="[%(levelname)s] %(funcName)s: %(message)s")
log = logging.getLogger("caja")

def cobrar(importe):
    log.debug("importe recibido %s", importe)
    log.info("cobrado")

cobrar(12)
```

```sortie
[DEBUG] cobrar: importe recibido 12
[INFO] cobrar: cobrado
```

> **Nota:** en un programa real, casi siempre se añade `%(asctime)s` para saber
> **cuándo** ocurrió. Esta página no lo usa: la hora cambia en cada ejecución, y las
> salidas que se muestran aquí deben seguir siendo idénticas.

## Pasar valores con %s

Para poner un valor en un mensaje, escribe `%s` en el texto y da el valor después,
separado por una coma.

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("carrito")

articulo = "manzanas"
cantidad = 3
log.info("se añaden %s %s", cantidad, articulo)
log.debug("cálculo muy largo: %s", sum(range(1000)))
```

```sortie
INFO:carrito:se añaden 3 manzanas
```

El primer `%s` recibe `cantidad`, el segundo recibe `articulo`. Es el mismo principio
que el formulario con huecos de [sqlite3](sqlite.md).

¿Por qué no una f-string? Con `%s`, el texto se fabrica **solo si** el mensaje pasa el
umbral. Un mensaje `DEBUG` ignorado no cuesta casi nada. Una f-string, en cambio, se
fabrica siempre, aunque sea para nada.

> **Atención:** `sum(range(1000))` se calcula de todos modos, porque Python evalúa los
> argumentos antes de la llamada. Solo se evita dar forma al texto.

> **Recuerda:** escribe `log.info("total: %s", total)`, no
> `log.info(f"total: {total}")`. Tantos `%s` como valores.

## Un registro por módulo

En un programa grande, cada [módulo](modules.md) tiene su propio registro. Se le da el
nombre del módulo con `__name__`. Así, cada línea dice **de dónde** viene.

```python norun
# archivo carrito.py
import logging

log = logging.getLogger(__name__)   # el nombre será "carrito"

def anadir(articulo):
    log.info("se añade %s", articulo)
```

En el programa principal, `__name__` vale `"__main__"`. Solo este archivo principal
llama a `basicConfig`. Los módulos se limitan a `getLogger(__name__)`.

```text
programa principal  ── basicConfig(...)  ← un solo ajuste
   │
   ├── carrito.py   ── getLogger("carrito")
   ├── pago.py      ── getLogger("pago")
   └── stock.py     ── getLogger("stock")
```

### Nombres en familia

Los nombres con un punto forman una familia: `app.db` es hijo de `app`. Se puede
ajustar el nivel de cada registro por separado.

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")

log_app = logging.getLogger("app")
log_db = logging.getLogger("app.db")
log_web = logging.getLogger("app.web")

log_db.setLevel(logging.DEBUG)

log_app.info("arranque")
log_db.debug("consulta SELECT enviada")
log_web.debug("página pedida")
log_web.info("página enviada")
print(log_db.parent.name)
```

```sortie
INFO:app:arranque
DEBUG:app.db:consulta SELECT enviada
INFO:app.web:página enviada
app
```

| Registro | Nivel ajustado | Umbral usado | ¿Se muestra `debug`? |
|---|---|---|---|
| `app` | nada | el de `basicConfig`: INFO | no |
| `app.db` | DEBUG | DEBUG | sí |
| `app.web` | nada | INFO (heredado) | no |

Es práctico: ¿investigas la base de datos? Pones **solo** `app.db` en `DEBUG`, sin
ahogarte en los detalles del resto.

> **Pruébalo:** añade `log_web.setLevel(logging.WARNING)`. La línea `página enviada`
> también desaparece.

> **Recuerda:** en cada módulo, `log = logging.getLogger(__name__)`. Un solo
> `basicConfig`, en el programa principal.

## Escribir el registro en un archivo

Un registro en pantalla desaparece al cerrar la terminal. Para conservarlo, se envía a
un [archivo](fichiers.md).

### Con basicConfig

Lo más sencillo: `filename=` en lugar de `stream=`.

```python
import logging

logging.basicConfig(filename="registro.log", level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s", encoding="utf-8")
log = logging.getLogger("copia")

log.info("copia de 3 archivos")
log.warning("1 archivo ignorado")
logging.shutdown()

with open("registro.log", encoding="utf-8") as f:
    print(f.read(), end="")
```

```sortie
INFO:copia:copia de 3 archivos
WARNING:copia:1 archivo ignorado
```

No se muestra nada durante el programa: todo va a `registro.log`. Se vuelve a leer el
archivo al final para comprobarlo. `logging.shutdown()` cierra el archivo como es debido.

> **Nota:** por defecto, el archivo se completa en cada ejecución (modo `"a"`). Añade
> `filemode="w"` para empezar de nuevo con un archivo vacío.

### Con handlers: la pantalla y el archivo

Un **handler** («manejador») es un destino para los mensajes. Un registro puede tener
varios handlers, cada uno con su propio umbral. Es como una carta enviada con copia:
una copia para ti, una copia para el archivo.

Ejemplo habitual: en pantalla, solo los problemas; en el archivo, todo.

```python
import logging
import sys

log = logging.getLogger("robot")
log.setLevel(logging.DEBUG)
forma = logging.Formatter("%(levelname)s:%(name)s:%(message)s")

pantalla = logging.StreamHandler(sys.stdout)
pantalla.setLevel(logging.WARNING)
pantalla.setFormatter(forma)
log.addHandler(pantalla)

archivo = logging.FileHandler("robot.log", mode="w", encoding="utf-8")
archivo.setLevel(logging.DEBUG)
archivo.setFormatter(forma)
log.addHandler(archivo)

log.debug("motor izquierdo: 40 %")
log.info("avanza 2 m")
log.warning("obstáculo detectado")

archivo.close()
print("--- contenido de robot.log ---")
with open("robot.log", encoding="utf-8") as f:
    print(f.read(), end="")
```

```sortie
WARNING:robot:obstáculo detectado
--- contenido de robot.log ---
DEBUG:robot:motor izquierdo: 40 %
INFO:robot:avanza 2 m
WARNING:robot:obstáculo detectado
```

Sigamos cada mensaje en los dos handlers:

| Mensaje | Nivel | Pantalla (umbral WARNING) | Archivo (umbral DEBUG) |
|---|---|---|---|
| motor izquierdo | DEBUG | no | sí |
| avanza 2 m | INFO | no | sí |
| obstáculo detectado | WARNING | sí | sí |

```text
                    log.warning("obstáculo detectado")
                               │
                   registro "robot" (umbral DEBUG)
                       ┌───────┴────────┐
                       ▼                ▼
             StreamHandler          FileHandler
             umbral WARNING         umbral DEBUG
                   │                    │
              la pantalla           robot.log
```

Hay pues **dos** filtros: el umbral del registro (`log.setLevel`) y luego el umbral de
cada handler. Un mensaje tiene que pasar los dos.

> **Pruébalo:** pon el umbral de la pantalla en `logging.DEBUG`. Los tres mensajes se
> muestran entonces también en pantalla, antes del contenido del archivo.

> **Recuerda:** `basicConfig(filename=...)` para un archivo sencillo. Handlers cuando
> quieres varios destinos con umbrales diferentes.

## Registrar una excepción

Cuando se captura una [excepción](../debuter/glossaire.md#exception), hay que dejar
constancia de **lo que** ha fallado. Hay dos maneras.

### El mensaje del error con log.error

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("importacion")

lineas = ["12", "8", "doce", "5"]
total = 0
for numero, texto in enumerate(lineas, start=1):
    try:
        total += int(texto)
    except ValueError as error:
        log.error("línea %s ignorada: %s", numero, error)

log.info("total = %s", total)
```

```sortie
ERROR:importacion:línea 3 ignorada: invalid literal for int() with base 10: 'doce'
INFO:importacion:total = 25
```

| Vuelta | `numero` | `texto` | `int(texto)` | `total` |
|---|---|---|---|---|
| 1 | 1 | "12" | 12 | 12 |
| 2 | 2 | "8" | 8 | 20 |
| 3 | 3 | "doce" | `ValueError` → mensaje ERROR | 20 |
| 4 | 4 | "5" | 5 | 25 |

El programa no se detiene. Pero la línea errónea queda anotada en el registro.

### Toda la traza con log.exception

`log.exception` escribe un mensaje de nivel `ERROR` **y** la traza completa del error,
como Python la habría mostrado. Solo se llama dentro de un `except`.

```python norun
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("calculo")

def dividir(a, b):
    return a / b

try:
    dividir(10, 0)
except ZeroDivisionError:
    log.exception("la división ha fallado")
print("el programa continúa")
```

```text
ERROR:calculo:la división ha fallado
Traceback (most recent call last):
  File "main.py", line 12, in <module>
    dividir(10, 0)
    ~~~~~~~^^^^^^^
  File "main.py", line 9, in dividir
    return a / b
           ~~^~~
ZeroDivisionError: division by zero
el programa continúa
```

La traza dice exactamente dónde buscar: línea 9, en `dividir`. Y el programa continúa.
El capítulo [los errores](erreurs.md) explica `try` y `except`.

> **Recuerda:** en un `except`, `log.exception("...")` conserva la traza completa. Sin
> traza, un error anotado como «ha fallado» no te ayudará a corregirlo.

## Un ejemplo completo

Un pequeño programa que procesa pedidos. Cuenta lo que hace, sin un solo `print` de
control.

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("pedidos")

stock = {"boli": 10, "cuaderno": 2}

def pedir(articulo, cantidad):
    log.debug("solicitud: %s x %s", cantidad, articulo)
    if articulo not in stock:
        log.error("artículo desconocido: %s", articulo)
        return False
    if stock[articulo] < cantidad:
        log.warning("stock insuficiente para %s (%s pedidos, %s en stock)",
                    articulo, cantidad, stock[articulo])
        return False
    stock[articulo] -= cantidad
    log.info("%s x %s enviados", cantidad, articulo)
    return True

pedir("boli", 3)
pedir("cuaderno", 5)
pedir("goma", 1)
print("Stock final:", stock)
```

```sortie
INFO:pedidos:3 x boli enviados
WARNING:pedidos:stock insuficiente para cuaderno (5 pedidos, 2 en stock)
ERROR:pedidos:artículo desconocido: goma
Stock final: {'boli': 7, 'cuaderno': 2}
```

| Llamada | ¿Artículo conocido? | ¿Bastante stock? | Nivel escrito | `stock` después |
|---|---|---|---|---|
| `pedir("boli", 3)` | sí | sí (10 ≥ 3) | INFO | boli 7, cuaderno 2 |
| `pedir("cuaderno", 5)` | sí | no (2 < 5) | WARNING | boli 7, cuaderno 2 |
| `pedir("goma", 1)` | no | — | ERROR | boli 7, cuaderno 2 |

> **Pruébalo:** pon `level` en `logging.DEBUG`. Cada pedido muestra primero su línea
> `DEBUG:pedidos:solicitud: …`.

## Errores frecuentes

### Más %s que valores (o al revés)

```python norun
import logging
import sys
logging.basicConfig(stream=sys.stdout, format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("app")
log.warning("Hola %s", "Lea", "Hugo")
```

```text
--- Logging error ---
Traceback (most recent call last):
  File "/usr/lib/python3.13/logging/__init__.py", line 1151, in emit
    msg = self.format(record)
  ...
TypeError: not all arguments converted during string formatting
Call stack:
  File "main.py", line 5, in <module>
    log.warning("Hola %s", "Lea", "Hugo")
Message: 'Hola %s'
Arguments: ('Lea', 'Hugo')
```

Un solo `%s` pero dos valores. El programa no se cuelga, pero el mensaje se pierde.
Cuenta: tantos `%s` como valores, `"Hola %s y %s"`.

### Llamar a basicConfig dos veces

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.WARNING,
                    format="%(levelname)s:%(name)s:%(message)s")
logging.basicConfig(stream=sys.stdout, level=logging.DEBUG,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("app")
log.debug("¿estoy aquí?")
log.warning("yo, sí")
```

```sortie
WARNING:app:yo, sí
```

No hay error, pero el segundo `basicConfig` se ignora: el registro ya estaba configurado.
Un solo `basicConfig`, al principio de todo. Para volver a configurarlo todo de verdad,
añade `force=True`.

### log.exception fuera de un except

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, format="%(levelname)s:%(name)s:%(message)s")
logging.getLogger("app").exception("algo va mal")
```

```sortie
ERROR:app:algo va mal
NoneType: None
```

No hay ninguna excepción en curso: no hay traza que mostrar, de ahí el curioso
`NoneType: None`. Fuera de un `except`, usa `log.error`.

## Ponte a prueba

**Pregunta 1:** el umbral es `INFO`. ¿Qué mensajes se muestran entre `debug`, `info`,
`warning` y `error`?

> **Respuesta**
>
> `info`, `warning` y `error`. Solo `debug` (10) está por debajo del umbral `INFO` (20).

**Pregunta 2:** ¿qué muestra este programa?

```python norun
import logging
import sys
logging.basicConfig(stream=sys.stdout, level=logging.ERROR,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("test")
log.warning("cuidado")
log.error("avería %s", 42)
```

> **Respuesta**
>
> ```python
> import logging
> import sys
> logging.basicConfig(stream=sys.stdout, level=logging.ERROR,
>                     format="%(levelname)s:%(name)s:%(message)s")
> log = logging.getLogger("test")
> log.warning("cuidado")
> log.error("avería %s", 42)
> ```
>
> ```sortie
> ERROR:test:avería 42
> ```
>
> `WARNING` (30) está por debajo del umbral `ERROR` (40): se ignora. El `%s` recibe 42.

**Pregunta 3:** quieres ver los errores en pantalla, pero guardar **todos** los detalles
en un archivo. ¿Qué haces?

> **Respuesta**
>
> Un registro con umbral `DEBUG` y dos handlers: un `StreamHandler` con umbral
> `WARNING` (o `ERROR`) para la pantalla, y un `FileHandler` con umbral `DEBUG` para el
> archivo.

## En resumen

- `print` muestra el resultado; `logging` cuenta lo que hace el programa.
- Cinco niveles: `DEBUG` < `INFO` < `WARNING` < `ERROR` < `CRITICAL`. El umbral `level`
  elige lo que pasa; por defecto, es `WARNING`.
- Un solo `basicConfig`, al principio del programa principal; `getLogger(__name__)` en
  cada módulo.
- Valores con `%s`: `log.info("total: %s", total)`.
- `filename=` o un `FileHandler` para guardar el registro en un archivo.
- En un `except`, `log.exception` conserva la traza completa del error.

Siguiente: [escribir buen Python](style.md)
