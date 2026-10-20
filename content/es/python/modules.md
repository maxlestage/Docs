# Módulos, paquetes y pip

Esta página explica cómo importar código, dividir tu programa en varios archivos, usar la biblioteca estándar e instalar paquetes con pip en un entorno virtual.

> **Objetivos:**
>
> - sabrás usar las distintas formas de `import`;
> - sabrás escribir tu propio módulo y usar `if __name__ == "__main__"`;
> - sabrás organizar módulos en un paquete con `__init__.py`;
> - sabrás encontrar ayuda en la biblioteca estándar;
> - sabrás crear un entorno virtual e instalar paquetes con pip.

## ¿Qué es un módulo?

Un [módulo](../debuter/glossaire.md#module) es simplemente **un archivo `.py`** cuyo
contenido se reutiliza: sus funciones, sus clases, sus variables. Python viene con
cientos de módulos listos para usar: es la **biblioteca estándar**.

Un módulo es una caja de herramientas guardada en el taller. `import` va a buscar la
caja; luego tomas las herramientas que necesitas.

## Las formas de import

### import módulo

La forma más sencilla carga todo el módulo. Luego usas sus elementos con un punto:
`módulo.nombre`.

```python
import math

print(math.sqrt(16))
print(math.pi)
print(math.floor(3.7))
```

```sortie
4.0
3.141592653589793
3
```

El prefijo `math.` indica claramente de dónde viene cada función.

### from módulo import nombre

Para usar un elemento sin prefijo, impórtalo directamente:

```python
from math import sqrt, pi

print(sqrt(25))
print(round(pi, 2))
```

```sortie
5.0
3.14
```

### Renombrar con as

`as` da otro nombre, más corto o más claro, al módulo o al elemento importado:

```python
import statistics as stats
from collections import Counter as Contador

notas = [12, 15, 9, 15, 18]
print(stats.mean(notas))
print(stats.median(notas))
print(Contador(notas).most_common(1))
```

```sortie
13.8
15
[(15, 2)]
```

| Forma | Uso | ¿Cuándo? |
|-------|-----|----------|
| `import math` | `math.sqrt(4)` | el caso por defecto, el más claro |
| `from math import sqrt` | `sqrt(4)` | uno o dos elementos que se usan a menudo |
| `import numpy as np` | `np.array(...)` | un nombre largo, con un alias habitual |
| `from math import *` | `sqrt(4)` | **evítalo**: ya no se sabe de dónde vienen los nombres |

> **Consejo:** pon todos los `import` al principio del archivo: se ve enseguida qué
> necesita el programa.

## Escribir tu propio módulo

Cuando un programa crece, se divide en varios archivos. Crea dos archivos en la **misma
carpeta**:

```text
calculos/
├── geometria.py
└── main.py
```

El archivo `geometria.py` contiene funciones:

```python norun
"""Pequeñas funciones de geometría."""

PI = 3.14159


def area_circulo(radio):
    return PI * radio ** 2


def perimetro_circulo(radio):
    return 2 * PI * radio
```

El archivo `main.py` las importa por el nombre del archivo, **sin** `.py`:

```python norun
import geometria
from geometria import perimetro_circulo

print(geometria.area_circulo(2))
print(round(perimetro_circulo(1), 2))
```

Ejecutas el programa principal desde la carpeta `calculos`:

```sh
python3 main.py
```

```text
12.56636
6.28
```

Python busca los módulos primero en la **carpeta del programa que ejecutas**, luego en
la biblioteca estándar y luego en los paquetes instalados.

## if __name__ == "__main__"

Cuando Python importa un módulo, **ejecuta todo su código**, de arriba abajo. Cada
módulo tiene una variable especial, `__name__`:

- si el archivo se ejecuta directamente (`python3 geometria.py`), `__name__` vale
  `"__main__"`;
- si se importa, `__name__` vale el nombre del módulo, `"geometria"`.

Añadamos al final de `geometria.py`:

```python norun
print("geometria.py está cargado, __name__ =", __name__)

if __name__ == "__main__":
    print("Prueba rápida:", area_circulo(1))
```

Según cómo ejecutes el archivo, el resultado cambia:

```sh
python3 main.py
```

```text
geometria.py está cargado, __name__ = geometria
12.56636
6.28
```

```sh
python3 geometria.py
```

```text
geometria.py está cargado, __name__ = __main__
Prueba rápida: 3.14159
```

> **Recuerda:** el código bajo `if __name__ == "__main__":` **solo** se ejecuta si el
> archivo se ejecuta directamente. Ahí se ponen pruebas o el punto de partida del
> programa, para que el import sea silencioso.

Un programa Python bien ordenado suele parecerse a esto:

```python
def main():
    print("El programa arranca")

if __name__ == "__main__":
    main()
```

```sortie
El programa arranca
```

## Los paquetes

Un [paquete](../debuter/glossaire.md#paquet) (*package*) es una **carpeta de módulos**.
Contiene un archivo `__init__.py`, que se ejecuta la primera vez que se importa el
paquete. Este archivo puede estar vacío.

```text
tienda/
├── main.py
└── almacen/
    ├── __init__.py
    ├── precios.py
    └── stock.py
```

```python norun
# almacen/precios.py
IVA = 0.20


def con_iva(importe):
    return round(importe * (1 + IVA), 2)
```

```python norun
# almacen/stock.py
STOCK = {"lapiz": 12, "cuaderno": 0}


def disponible(articulo):
    return STOCK.get(articulo, 0) > 0
```

En `__init__.py`, puedes subir los elementos principales para acortar los imports. El
punto de `.precios` quiere decir «el módulo `precios` de **este** paquete»:

```python norun
# almacen/__init__.py
from .precios import con_iva
```

Luego se importa con puntos, que siguen las carpetas:

```python norun
# main.py
from almacen import con_iva
from almacen.stock import disponible
import almacen.precios

print(con_iva(10))
print(disponible("lapiz"), disponible("cuaderno"))
print(almacen.precios.IVA)
```

```text
12.0
True False
0.2
```

## Un recorrido por la biblioteca estándar

La biblioteca estándar cubre la mayoría de las necesidades habituales, sin instalar
nada. Algunos módulos que conviene conocer:

| Módulo | Para… |
|--------|-------|
| `math`, `statistics` | cálculos, medias, medianas |
| `random` | sorteos al azar |
| `datetime` | fechas y horas (ver [las fechas](dates.md)) |
| `pathlib`, `os` | rutas y archivos (ver [los archivos](fichiers.md)) |
| `json`, `csv` | formatos de datos (ver [los formatos](formats.md)) |
| `collections` | `Counter`, `defaultdict`, `deque` |
| `itertools`, `functools` | herramientas para iteradores y funciones |
| `re` | [expresiones regulares](regex.md) |
| `sys`, `argparse` | [línea de comandos](ligne-de-commande.md) |
| `unittest` | [tests](tests.md) |

Una pequeña muestra:

```python
from collections import Counter, defaultdict
from datetime import date
from pathlib import Path
import json

print(Counter("abracadabra").most_common(2))

por_letra = defaultdict(list)
for palabra in ["ala", "bus", "arco"]:
    por_letra[palabra[0]].append(palabra)
print(dict(por_letra))

print(date(2024, 7, 14).strftime("%d/%m/%Y"))
print(Path("fotos/vacaciones/playa.jpg").suffix)
print(json.dumps({"nombre": "Zoé", "edad": 9}, ensure_ascii=False))
```

```sortie
[('a', 5), ('b', 2)]
{'a': ['ala', 'arco'], 'b': ['bus']}
14/07/2024
.jpg
{"nombre": "Zoé", "edad": 9}
```

> **Consejo:** para explorar un módulo, usa `help(math)` o `dir(math)` en la consola de
> Python, o la documentación oficial en docs.python.org (también en español).

## Instalar paquetes con pip

Cientos de miles de paquetes escritos por otras personas están publicados en **PyPI**
(*Python Package Index*, pypi.org). La herramienta **pip** los descarga y los instala.

Ejemplo con `requests`, un paquete muy usado para hablar con sitios web:

```sh
python3 -m pip install requests
```

```text
Collecting requests
  Using cached requests-2.34.2-py3-none-any.whl.metadata (4.8 kB)
...
Installing collected packages: urllib3, idna, charset_normalizer, certifi, requests
Successfully installed certifi-2026.10.18 charset_normalizer-3.5.2 idna-3.20 requests-2.34.2 urllib3-2.8.0
```

pip también ha instalado las **dependencias** de `requests`: los paquetes que necesita
para funcionar. Después, se importa como un módulo de la biblioteca estándar:

```python norun
import requests

respuesta = requests.get("https://example.com")
print(respuesta.status_code)
```

```text
200
```

Algunos comandos útiles de pip:

```sh
python3 -m pip list                 # los paquetes instalados
python3 -m pip show requests        # detalles de un paquete
python3 -m pip install --upgrade requests
python3 -m pip uninstall requests
```

> **Nota:** escribe `python3 -m pip` en lugar de solo `pip`: así seguro que instalas
> para **ese** Python, si hay varios en el ordenador.

## Los entornos virtuales

Si todos tus proyectos instalan sus paquetes en el mismo sitio, acaban molestándose: el
proyecto A quiere la versión 1 de un paquete, el proyecto B la versión 2. Un **entorno
virtual** es una carpeta que contiene un Python y paquetes **propios de un proyecto**.
Cada proyecto tiene su caja, aislada de las demás.

Se crea con el módulo `venv`, normalmente en una carpeta llamada `.venv`:

```sh
cd mi-proyecto
python3 -m venv .venv
```

Luego se **activa**, una vez por terminal:

```sh
# Linux y macOS
source .venv/bin/activate

# Windows (PowerShell)
.venv\Scripts\Activate.ps1
```

El nombre del entorno aparece entonces al principio del prompt, por ejemplo
`(.venv) $`. A partir de ahora, `python` y `pip` son los del entorno:

```sh
python -m pip install requests
deactivate
```

`deactivate` sale del entorno.

> **Atención:** no pongas la carpeta `.venv` en [Git](../debuter/git.md): añádela al
> archivo `.gitignore`. Se vuelve a crear a partir de `requirements.txt`.

## requirements.txt

Para que otra persona (o tú, en otro ordenador) instale exactamente los mismos
paquetes, se listan en un archivo `requirements.txt`. `pip freeze` muestra los paquetes
instalados con su versión:

```sh
python -m pip freeze > requirements.txt
```

```text
certifi==2026.10.18
charset-normalizer==3.5.2
idna==3.20
requests==2.34.2
urllib3==2.8.0
```

En otra máquina, en un nuevo entorno virtual, un solo comando lo reinstala todo:

```sh
python -m pip install -r requirements.txt
```

## Errores frecuentes

### Importar un paquete no instalado

```python norun
import requests
print(requests.get("https://example.com").status_code)
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    import requests
ModuleNotFoundError: No module named 'requests'
```

El paquete no está instalado **en el Python que ejecuta el programa**. Activa tu entorno
virtual y luego `python -m pip install requests`.

### Llamar a tu archivo como un módulo estándar

Un archivo llamado `random.py` que contiene:

```python norun
import random
print(random.randint(1, 6))
```

```text
Traceback (most recent call last):
  File "random.py", line 1, in <module>
    import random
  File "random.py", line 2, in <module>
    print(random.randint(1, 6))
          ^^^^^^^^^^^^^^
AttributeError: module 'random' has no attribute 'randint' (consider renaming 'random.py' since it has the same name as the standard library module named 'random' and prevents importing that standard library module)
```

Python busca primero en la carpeta del programa: `import random` importa **tu** archivo
en lugar del módulo estándar. Cámbiale el nombre, por ejemplo `dado.py`.

### Importar un nombre que no existe

```python norun
from math import raiz
print(raiz(16))
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    from math import raiz
ImportError: cannot import name 'raiz' from 'math' (unknown location)
```

El módulo existe, pero no ese nombre: los nombres están en inglés. La función se llama
`sqrt`. Compruébalo con `dir(math)`.

## Ponte a prueba

**Pregunta 1:** ¿qué diferencia hay entre `import math` y `from math import sqrt`?

> **Respuesta**
>
> Con `import math`, se escribe `math.sqrt(4)`. Con `from math import sqrt`, se escribe
> directamente `sqrt(4)`, pero ya no se ve de dónde viene la función.

**Pregunta 2:** un archivo `utilidades.py` contiene `print("cargado")` y luego una
función. ¿Qué pasa cuando `main.py` hace `import utilidades`?

> **Respuesta**
>
> Se muestra `cargado`: importar un módulo ejecuta todo su código. Para evitarlo, pon
> ese `print` bajo `if __name__ == "__main__":`.

**Pregunta 3:** ¿en qué orden escribes los comandos para empezar un proyecto que usa
`requests`?

> **Respuesta**
>
> ```sh
> python3 -m venv .venv
> source .venv/bin/activate
> python -m pip install requests
> python -m pip freeze > requirements.txt
> ```

## En resumen

- Un módulo es un archivo `.py`; un paquete es una carpeta de módulos con
  `__init__.py`.
- `import math`, `from math import sqrt` e `import ... as ...` son las tres formas de
  import.
- `if __name__ == "__main__":` contiene el código que solo se ejecuta cuando el archivo
  se ejecuta directamente.
- La biblioteca estándar cubre muchas necesidades antes de instalar nada.
- `python3 -m venv .venv` crea un entorno virtual; `pip install` le añade paquetes;
  `requirements.txt` los lista.

Siguiente: [leer y escribir archivos](fichiers.md). Compara con
[los módulos en Ruby](../ruby/modules.md).
