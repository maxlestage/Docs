# Chuleta

Esta página resume de un vistazo la sintaxis de Python vista en todo el recorrido, con tablas compactas y ejemplos cortos.

## Bases

| Elemento | Ejemplo |
| --- | --- |
| Variable | `edad = 30` |
| Constante (convención) | `TASA = 0.2` |
| Texto | `"Hola"`, `'Hola'` |
| f-string | `f"Hola {nombre}"` |
| Booleanos | `True`, `False` |
| Nada | `None` |
| Comentario | `# texto` |
| Mostrar | `print(a, b, sep=", ", end="")` |
| Leer el teclado | `nombre = input("Nombre: ")` (siempre texto) |
| Tipo de un valor | `type(x)`, `isinstance(x, int)` |

```python
nombre = "Alicia"
edad = 30
print(f"Hola {nombre}, {edad} años")
print(type(edad), type(3.5), type(nombre), type(None))
```

```sortie
Hola Alicia, 30 años
<class 'int'> <class 'float'> <class 'str'> <class 'NoneType'>
```

Detalles: [primeros pasos](premiers-pas.md).

## Números

| Operación | Ejemplo | Resultado |
| --- | --- | --- |
| División | `7 / 2` | `3.5` |
| División entera | `7 // 2` | `3` |
| Resto | `7 % 2` | `1` |
| Potencia | `2 ** 10` | `1024` |
| Redondeo | `round(3.14159, 2)` | `3.14` |
| Conversión | `int("42")`, `float("2.5")`, `str(42)` | `42`, `2.5`, `"42"` |
| Valor absoluto, mín., máx. | `abs(-3)`, `min(4, 2)`, `max(4, 2)` | `3`, `2`, `4` |

```python
import math

print(7 / 2, 7 // 2, 7 % 2, 2 ** 10)
print(round(math.pi, 2), math.sqrt(16))
print(0.1 + 0.2, math.isclose(0.1 + 0.2, 0.3))
print(f"{1234567.891:,.2f}")
```

```sortie
3.5 3 1 1024
3.14 4.0
0.30000000000000004 True
1,234,567.89
```

Detalles: [números y cálculos](nombres.md).

## Condiciones y bucles

| Necesidad | Sintaxis |
| --- | --- |
| Condición | `if … elif … else` |
| Expresión condicional | `"sí" if x > 0 else "no"` |
| Varios casos | `match x:` y luego `case 1:` … `case _:` |
| Bucle sobre una secuencia | `for x in lista:` |
| Contar | `for i in range(1, 6):` |
| Índice y valor | `for i, x in enumerate(lista, start=1):` |
| Dos listas a la vez | `for a, b in zip(l1, l2):` |
| Mientras | `while condicion:` |
| Control | `break`, `continue`, `pass` |

```python
for n in range(1, 6):
    if n == 2:
        continue
    print(n, "par" if n % 2 == 0 else "impar")

match "añadir":
    case "añadir" | "add":
        print("añadido")
    case _:
        print("desconocido")
```

```sortie
1 impar
3 impar
4 par
5 impar
añadido
```

Detalles: [condiciones y bucles](conditions-boucles.md).

## Texto

| Necesidad | Método |
| --- | --- |
| Mayúsculas, minúsculas | `s.upper()`, `s.lower()`, `s.title()` |
| Quitar los espacios | `s.strip()` |
| Cortar, volver a unir | `s.split(",")`, `", ".join(lista)` |
| Reemplazar | `s.replace("a", "b")` |
| Buscar | `"ab" in s`, `s.find("ab")`, `s.startswith("x")` |
| Longitud, trozo | `len(s)`, `s[0]`, `s[-1]`, `s[1:4]`, `s[::-1]` |
| Alinear | `f"{s:<10}"`, `f"{s:>10}"`, `f"{s:^10}"` |
| Número con formato | `f"{x:.2f}"`, `f"{n:03d}"` |

```python
s = "  Hola a todo el mundo  "
palabras = s.strip().split()
print(palabras)
print("-".join(palabras).upper())
print(f"[{'Python':^10}] [{3.14159:.2f}] [{7:03d}]")
print("radar"[::-1], len("año"))
```

```sortie
['Hola', 'a', 'todo', 'el', 'mundo']
HOLA-A-TODO-EL-MUNDO
[  Python  ] [3.14] [007]
radar 3
```

Detalles: [trabajar con texto](texte.md).

## Listas y tuplas

| Necesidad | Sintaxis |
| --- | --- |
| Crear | `l = [1, 2, 3]`, `t = (1, 2)` (tupla, no modificable) |
| Añadir | `l.append(x)`, `l.extend([4, 5])`, `l.insert(0, x)` |
| Quitar | `l.pop()`, `l.pop(0)`, `l.remove(x)` |
| Ordenar | `sorted(l)` (lista nueva), `l.sort()` (en el sitio) |
| Ordenar según una clave | `sorted(l, key=len, reverse=True)` |
| Porción | `l[1:3]`, `l[:2]`, `l[-2:]` |
| Agregar | `sum(l)`, `min(l)`, `max(l)`, `len(l)` |
| Comprobar | `x in l`, `any(...)`, `all(...)` |
| Desempaquetar | `a, b = (1, 2)`, `primero, *resto = l` |

```python
frutas = ["pera", "kiwi", "manzana"]
frutas.append("albaricoque")
print(sorted(frutas))
print(sorted(frutas, key=len))
primero, *resto = frutas
print(primero, resto)
print(any(len(f) > 8 for f in frutas), frutas[-2:])
```

```sortie
['albaricoque', 'kiwi', 'manzana', 'pera']
['pera', 'kiwi', 'manzana', 'albaricoque']
pera ['kiwi', 'manzana', 'albaricoque']
True ['manzana', 'albaricoque']
```

Detalles: [listas y tuplas](collections.md).

## Diccionarios

| Necesidad | Sintaxis |
| --- | --- |
| Crear | `d = {"a": 1, "b": 2}` |
| Leer | `d["a"]` (error si no está), `d.get("z", 0)` |
| Escribir, borrar | `d["c"] = 3`, `del d["a"]`, `d.pop("a")` |
| Recorrer | `for clave, valor in d.items():` |
| Claves, valores | `d.keys()`, `d.values()` |
| Comprobar una clave | `"a" in d` |
| Fusionar | `d1 \| d2` |
| Contar | `collections.Counter(lista)` |
| Agrupar | `collections.defaultdict(list)` |

```python
from collections import Counter, defaultdict

stock = {"manzana": 3, "kiwi": 0}
stock["pera"] = 5
print(stock.get("plátano", 0), "kiwi" in stock)
for fruta, cantidad in stock.items():
    print(fruta, cantidad)

print(Counter("abracadabra").most_common(2))
por_inicial = defaultdict(list)
for palabra in ["ala", "arco", "bol"]:
    por_inicial[palabra[0]].append(palabra)
print(dict(por_inicial))
```

```sortie
0 True
manzana 3
kiwi 0
pera 5
[('a', 5), ('b', 2)]
{'a': ['ala', 'arco'], 'b': ['bol']}
```

Detalles: [diccionarios y conjuntos](dictionnaires.md).

## Conjuntos

| Operación | Sintaxis |
| --- | --- |
| Crear | `{1, 2, 3}`, `set()` (vacío), `set(lista)` |
| Añadir, quitar | `s.add(x)`, `s.discard(x)` |
| Unión, intersección | `a \| b`, `a & b` |
| Diferencia | `a - b`, `a ^ b` (solo en uno de los dos) |

```python
a = {1, 2, 3, 4}
b = {3, 4, 5}
print(sorted(a | b), sorted(a & b), sorted(a - b))
print(sorted(set([3, 1, 3, 2, 1])))
```

```sortie
[1, 2, 3, 4, 5] [3, 4] [1, 2]
[1, 2, 3]
```

Un conjunto no tiene orden: `sorted` da una salida estable.

## Comprensiones y generadores

| Forma | Ejemplo |
| --- | --- |
| Lista | `[x * 2 for x in l if x > 0]` |
| Diccionario | `{x: x * x for x in l}` |
| Conjunto | `{len(p) for p in palabras}` |
| Generador (perezoso) | `sum(x * x for x in l)` |
| Función generadora | `yield` dentro de una función |

```python
numeros = [3, -1, 4, 0, 5]
print([n * 2 for n in numeros if n > 0])
print({n: n * n for n in numeros if n > 3})
print(sum(n * n for n in numeros))


def cuenta_atras(n):
    while n > 0:
        yield n
        n -= 1


print(list(cuenta_atras(3)))
```

```sortie
[6, 8, 10]
{4: 16, 5: 25}
51
[3, 2, 1]
```

Detalles: [las comprensiones](comprehensions.md), [los iteradores](iterateurs.md).

## Funciones

| Elemento | Sintaxis |
| --- | --- |
| Definir | `def nombre(a, b=0):` y luego `return …` |
| Argumentos con nombre | `nombre(1, b=2)` |
| Número variable | `def f(*args, **kwargs):` |
| Función anónima | `lambda x: x * 2` |
| Anotaciones | `def f(x: int) -> str:` |
| Docstring | `"""Lo que hace la función."""` |
| Decorador | `@functools.cache` encima de `def` |

```python
from functools import cache


def saludar(nombre: str, formula: str = "Hola") -> str:
    """Devuelve una frase de saludo."""
    return f"¡{formula}, {nombre}!"


def total(*numeros):
    return sum(numeros)


@cache
def fib(n):
    return n if n < 2 else fib(n - 1) + fib(n - 2)


print(saludar("Alicia"), saludar("Bob", formula="Buenas"))
print(total(1, 2, 3), fib(50))
print(sorted(["bb", "a", "ccc"], key=lambda p: -len(p)))
```

```sortie
¡Hola, Alicia! ¡Buenas, Bob!
6 12586269025
['ccc', 'bb', 'a']
```

Detalles: [las funciones](fonctions.md), [los decoradores](decorateurs.md),
[el tipado](typage.md).

## Clases y herencia

| Elemento | Sintaxis |
| --- | --- |
| Clase | `class Perro:` |
| Constructor | `def __init__(self, nombre):` |
| Atributo | `self.nombre = nombre` |
| Texto legible | `def __str__(self):` |
| Herencia | `class Perro(Animal):` |
| Llamar al padre | `super().__init__(nombre)` |
| Propiedad calculada | `@property` |

```python
class Animal:
    def __init__(self, nombre):
        self.nombre = nombre

    def hablar(self):
        return "..."

    def __str__(self):
        return f"{self.nombre} dice {self.hablar()}"


class Perro(Animal):
    def hablar(self):
        return "Guau"


for animal in [Animal("Cosita"), Perro("Rex")]:
    print(animal)
print(isinstance(Perro("Rex"), Animal))
```

```sortie
Cosita dice ...
Rex dice Guau
True
```

Detalles: [las clases](types.md), [la herencia](heritage.md).

## Dataclasses

`@dataclass` escribe `__init__`, `__repr__` y `__eq__` por ti.

```python
from dataclasses import dataclass, field


@dataclass
class Punto:
    x: float
    y: float = 0.0


@dataclass(frozen=True)
class Color:
    nombre: str


@dataclass
class Cesta:
    articulos: list[str] = field(default_factory=list)


p = Punto(1, 2)
print(p, p == Punto(1, 2), Punto(3))
print(Color("rojo"))
cesta = Cesta()
cesta.articulos.append("pan")
print(cesta)
```

```sortie
Punto(x=1, y=2) True Punto(x=3, y=0.0)
Color(nombre='rojo')
Cesta(articulos=['pan'])
```

## Excepciones

| Elemento | Sintaxis |
| --- | --- |
| Capturar | `try:` … `except ValueError as e:` |
| Varios tipos | `except (KeyError, IndexError):` |
| Si todo va bien | `else:` |
| Siempre | `finally:` |
| Lanzar | `raise ValueError("mensaje")` |
| Tu propio error | `class SaldoInsuficiente(Exception): pass` |

```python
class SaldoInsuficiente(Exception):
    pass


def retirar(saldo, importe):
    if importe > saldo:
        raise SaldoInsuficiente(f"faltan {importe - saldo} €")
    return saldo - importe


for texto in ["30", "abc", "500"]:
    try:
        print("quedan", retirar(100, int(texto)))
    except ValueError:
        print("no es un número:", texto)
    except SaldoInsuficiente as error:
        print("rechazado:", error)
    finally:
        print("--")
```

```sortie
quedan 70
--
no es un número: abc
--
rechazado: faltan 400 €
--
```

Detalles: [los errores](erreurs.md), [depurar](debogage.md).

## Archivos

| Necesidad | Sintaxis |
| --- | --- |
| Escribir | `with open("f.txt", "w", encoding="utf-8") as f: f.write(texto)` |
| Añadir al final | modo `"a"` |
| Leer todo | `Path("f.txt").read_text(encoding="utf-8")` |
| Leer línea a línea | `for linea in f:` |
| Comprobar, listar | `Path("f.txt").exists()`, `Path(".").glob("*.txt")` |
| Rutas | `Path("carpeta") / "f.txt"`, `p.name`, `p.suffix` |

```python
from pathlib import Path

ruta = Path("notas.txt")
with open(ruta, "w", encoding="utf-8") as f:
    f.write("Alicia 15\nBob 12\n")

with open(ruta, encoding="utf-8") as f:
    for linea in f:
        nombre, nota = linea.split()
        print(nombre, int(nota) + 1)

print(ruta.exists(), ruta.suffix, ruta.read_text(encoding="utf-8").count("\n"))
```

```sortie
Alicia 16
Bob 13
True .txt 2
```

Detalles: [los archivos](fichiers.md).

## JSON y CSV

| Necesidad | Función |
| --- | --- |
| Objeto Python a texto JSON | `json.dumps(obj, indent=2, ensure_ascii=False)` |
| Texto JSON a objeto Python | `json.loads(texto)` |
| Con un archivo | `json.dump(obj, f)`, `json.load(f)` |
| Leer un CSV | `csv.reader(f)`, `csv.DictReader(f)` |

```python
import csv
import io
import json

datos = {"nombre": "Léa", "notas": [15, 12], "aprobada": True}
texto = json.dumps(datos, ensure_ascii=False)
print(texto)
print(json.loads(texto)["notas"])

for fila in csv.DictReader(io.StringIO("nombre,nota\nAlicia,15\nBob,12\n")):
    print(fila["nombre"], int(fila["nota"]))
```

```sortie
{"nombre": "Léa", "notas": [15, 12], "aprobada": true}
[15, 12]
Alicia 15
Bob 12
```

Detalles: [los formatos de datos](formats.md).

## Expresiones regulares

| Patrón | Significado |
| --- | --- |
| `\d`, `\w`, `\s` | una cifra, una letra o cifra, un espacio |
| `.` | cualquier carácter |
| `+`, `*`, `?`, `{2,4}` | 1 o más, 0 o más, 0 o 1, entre 2 y 4 |
| `^`, `$` | inicio, fin |
| `[a-z]`, `[^0-9]` | un carácter de la lista, fuera de la lista |
| `( )`, `(?P<nombre> )` | un grupo, un grupo con nombre |

```python
import re

texto = "Pedido 42 el 2026-03-15, pedido 7 el 2026-04-01"
print(re.findall(r"\d+", texto))

m = re.search(r"(?P<anio>\d{4})-(?P<mes>\d{2})", texto)
print(m.group("anio"), m.group("mes"))

print(re.sub(r"\s+", " ", "demasiados   espacios   aquí"))
print(bool(re.fullmatch(r"[\w.]+@[\w.]+\.\w+", "alicia@example.com")))
```

```sortie
['42', '2026', '03', '15', '7', '2026', '04', '01']
2026 03
demasiados espacios aquí
True
```

Detalles: [las expresiones regulares](regex.md).

## Fechas

| Necesidad | Sintaxis |
| --- | --- |
| Una fecha, una fecha y hora | `date(2026, 3, 15)`, `datetime(2026, 3, 15, 14, 30)` |
| Hoy, ahora | `date.today()`, `datetime.now()` |
| Duración | `timedelta(days=7, hours=2)` |
| Dar formato | `d.strftime("%d/%m/%Y")` |
| Leer un texto | `datetime.strptime("15/03/2026", "%d/%m/%Y")`, `date.fromisoformat("2026-03-15")` |

```python
from datetime import date, datetime, timedelta

inicio_curso = date(2026, 9, 1)
vacaciones = date(2026, 10, 17)
print((vacaciones - inicio_curso).days, "días")
print(inicio_curso + timedelta(weeks=2))
print(inicio_curso.strftime("%d/%m/%Y"), inicio_curso.isoweekday())

cita = datetime.strptime("15/03/2026 14:30", "%d/%m/%Y %H:%M")
print(cita.isoformat())
```

```sortie
46 días
2026-09-15
01/09/2026 2
2026-03-15T14:30:00
```

Detalles: [las fechas](dates.md).

## Línea de comandos con argparse

```python
import argparse

parser = argparse.ArgumentParser(description="Calcula una media.")
parser.add_argument("notas", type=float, nargs="+", help="las notas")
parser.add_argument("--decimales", type=int, default=1)
parser.add_argument("-v", "--detallado", action="store_true")

args = parser.parse_args(["12", "15", "17", "--decimales", "2", "-v"])
media = sum(args.notas) / len(args.notas)
if args.detallado:
    print("Notas:", args.notas)
print(round(media, args.decimales))
```

```sortie
Notas: [12.0, 15.0, 17.0]
14.67
```

En un programa real, escribe `parser.parse_args()` sin lista: lee `sys.argv`.
`--help` se crea solo. Detalles: [la línea de comandos](ligne-de-commande.md).

## Concurrencia y asyncio

| Elemento | Sintaxis |
| --- | --- |
| Función asíncrona | `async def f():` |
| Esperar | `await otra()` |
| Lanzar el programa | `asyncio.run(main())` |
| En paralelo | `await asyncio.gather(f(), g())` |
| Pausa sin bloquear | `await asyncio.sleep(1)` |
| Hilos | `concurrent.futures.ThreadPoolExecutor` |

```python
import asyncio


async def descargar(nombre, duracion):
    await asyncio.sleep(duracion)
    return f"{nombre} terminado"


async def main():
    resultados = await asyncio.gather(
        descargar("imagen", 0.02),
        descargar("texto", 0.01),
    )
    print(resultados)


asyncio.run(main())
```

```sortie
['imagen terminado', 'texto terminado']
```

`gather` devuelve los resultados en el orden de las llamadas, aunque `texto` termine antes.
Detalles: [la concurrencia](concurrence.md), [async](async.md).

## Pruebas con unittest

| Elemento | Sintaxis |
| --- | --- |
| Clase de prueba | `class TestX(unittest.TestCase):` |
| Una prueba | método `def test_…(self):` |
| Igualdad | `self.assertEqual(obtenido, esperado)` |
| Verdadero, dentro, decimales | `assertTrue`, `assertIn`, `assertAlmostEqual` |
| Error esperado | `with self.assertRaises(ValueError):` |
| Antes de cada prueba | `def setUp(self):` |
| Lanzar | `unittest.main()` o `python3 -m unittest` |

```python
import unittest


def doble(x):
    return x * 2


class TestDoble(unittest.TestCase):
    def test_doble(self):
        self.assertEqual(doble(4), 8)

    def test_error(self):
        with self.assertRaises(ZeroDivisionError):
            1 / 0


programa = unittest.main(argv=["x"], exit=False)
print(programa.result.testsRun, programa.result.wasSuccessful())
```

```sortie
2 True
```

Detalles: [probar tu código](tests.md).

## Proyecto y herramientas

| Comando | Función |
| --- | --- |
| `python3 main.py` | ejecutar un programa |
| `python3` | consola interactiva (REPL) |
| `python3 -m venv .venv` | crear un entorno virtual |
| `source .venv/bin/activate` | activarlo (Linux, macOS) |
| `python -m pip install nombre` | instalar un paquete |
| `python -m pip install -r requirements.txt` | instalar la lista de un proyecto |
| `python -m pip install -e .` | instalar tu propio proyecto |
| `python3 -m pdb main.py` | depurar |
| `pytest`, `ruff check`, `black .` | probar, comprobar, formatear |
| `uv run main.py` | todo en uno con uv |

Detalles: [organizar un proyecto](projet.md), [escribir buen Python](style.md). Para
practicar: [los proyectos paso a paso](projets.md) y [los ejercicios resueltos](exercices.md).
Compara con [la chuleta de Ruby](../ruby/aide-memoire.md) y
[la chuleta de Rust](../rust/aide-memoire.md).
