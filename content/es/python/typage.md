# Las anotaciones de tipo

Esta página muestra cómo indicar los tipos esperados en tu código, cómo las dataclasses escriben tus clases por ti, y cómo la herramienta mypy detecta los errores antes de la ejecución.

> **Objetivos:**
>
> - sabrás anotar variables y funciones (`int`, `list[int]`, `dict[str, int]`);
> - sabrás expresar «un valor o nada» con `X | None`;
> - sabrás escribir una clase de datos con `@dataclass`;
> - sabrás describir un diccionario con `TypedDict` y un comportamiento con `Protocol`;
> - sabrás comprobar tu código con mypy.

## ¿Por qué anotar?

Python nunca pide el [tipo](../debuter/glossaire.md#tipo) de una variable: lo descubre
durante la ejecución. Es flexible, pero un error de tipo solo aparece en el momento en
que la línea se ejecuta, a veces semanas después.

Una **anotación de tipo** (*type hint*) es una etiqueta que dice qué tipo esperas. Sirve
de documentación, ayuda a tu editor a completar el código, y permite que una herramienta
de comprobación encuentre errores **sin ejecutar el programa**.

> **Recuerda:** Python mismo **ignora** las anotaciones durante la ejecución. No cambian
> nada en el comportamiento del programa. Son herramientas como mypy las que las
> comprueban.

```python
edad: int = "diez"
print(edad)
```

```sortie
diez
```

Ningún error: Python no ha mirado la anotación. Más abajo veremos cómo mypy, en cambio,
sí la señala.

## Anotar variables y funciones

Para una variable, se escribe `: tipo` después del nombre. Para una función, se anota
cada parámetro y luego el tipo devuelto después de una flecha `->`.

```python
nombre: str = "Alice"
edad: int = 30
altura: float = 1.68
mayor_de_edad: bool = True

def saludar(persona: str, veces: int = 1) -> str:
    return ("¡Hola " + persona + "! ") * veces

def mostrar(mensaje: str) -> None:
    print(mensaje)

mostrar(saludar(nombre, 2))
```

```sortie
¡Hola Alice! ¡Hola Alice! 
```

`-> None` quiere decir: «esta función no devuelve nada útil».

> **Consejo:** no hace falta anotar cada variable. Para `edad = 30`, la herramienta adivina
> sola `int`. Anota sobre todo los **parámetros** y los **valores devueltos** de las
> funciones.

## Las colecciones

Para una colección, también se indica el tipo de lo que contiene, entre corchetes.

| Anotación | Significa |
|-----------|-----------|
| `list[int]` | una lista de enteros |
| `dict[str, int]` | un diccionario con claves `str` y valores `int` |
| `set[str]` | un conjunto de cadenas |
| `tuple[str, int]` | una tupla de dos elementos: un `str` y luego un `int` |
| `tuple[int, ...]` | una tupla de enteros, de cualquier longitud |

```python
def promedio(notas: list[float]) -> float:
    return sum(notas) / len(notas)

def contar_palabras(texto: str) -> dict[str, int]:
    cuenta: dict[str, int] = {}
    for palabra in texto.split():
        cuenta[palabra] = cuenta.get(palabra, 0) + 1
    return cuenta

def min_max(numeros: list[int]) -> tuple[int, int]:
    return min(numeros), max(numeros)

print(promedio([12, 15.5, 9]))
print(contar_palabras("el gato y el perro"))
print(min_max([4, 1, 9]))
```

```sortie
12.166666666666666
{'el': 2, 'gato': 1, 'y': 1, 'perro': 1}
(1, 9)
```

## Un valor o nada: X | None

Una función que busca algo puede no encontrar nada y devolver `None`. Se escribe
`str | None`: «un `str`, o bien `None`».

```python
def buscar(nombres: list[str], inicio: str) -> str | None:
    for nombre in nombres:
        if nombre.startswith(inicio):
            return nombre
    return None

resultado = buscar(["Alice", "Bob"], "B")
if resultado is not None:
    print(resultado.upper())
print(buscar(["Alice", "Bob"], "Z"))
```

```sortie
BOB
None
```

El `if resultado is not None` es importante: dentro, la herramienta sabe que `resultado`
es un `str`, y permite `.upper()`.

### Optional y Union

En código más antiguo, verás las mismas ideas escritas con el módulo `typing`:

| Escritura antigua | Escritura moderna |
|-------------------|-------------------|
| `Optional[str]` | `str \| None` |
| `Union[int, str]` | `int \| str` |

```python
from typing import Optional, Union

def duplicar(x: Union[int, str]) -> Union[int, str]:
    return x * 2

def inicial(nombre: Optional[str]) -> str:
    return nombre[0] if nombre else "?"

print(duplicar(4), duplicar("ab"))
print(inicial("Zoé"), inicial(None))
```

```sortie
8 abab
Z ?
```

Prefiere la escritura con `|`: es más corta y no necesita import.

## Las dataclasses

Escribir `__init__`, `__repr__` y `__eq__` para cada clase pequeña es repetitivo (ver
[clases y objetos](types.md)). El decorador `@dataclass` los **genera por ti** a partir
de las anotaciones.

```python
from dataclasses import dataclass

@dataclass
class Punto:
    x: int
    y: int

p = Punto(1, 2)
print(p)
print(p.x + p.y)
print(p == Punto(1, 2))
```

```sortie
Punto(x=1, y=2)
3
True
```

Bastan tres líneas: cada anotación se convierte en un **campo**, es decir, un parámetro
de `__init__` y un atributo.

### Valores por defecto

Un campo puede tener un valor por defecto. Para una lista o un diccionario, usa
`field(default_factory=list)`: así cada objeto recibe **su propia** lista.

```python
from dataclasses import dataclass, field

@dataclass
class Alumno:
    nombre: str
    clase: str = "1.º A"
    notas: list[int] = field(default_factory=list)

    def promedio(self) -> float:
        return sum(self.notas) / len(self.notas) if self.notas else 0.0

alice = Alumno("Alice")
alice.notas.append(15)
alice.notas.append(12)
bob = Alumno("Bob", "2.º B")
print(alice, alice.promedio())
print(bob)
```

```sortie
Alumno(nombre='Alice', clase='1.º A', notas=[15, 12]) 13.5
Alumno(nombre='Bob', clase='2.º B', notas=[])
```

Una dataclass sigue siendo una clase normal: puedes añadirle métodos.

### frozen y order

Dos opciones útiles:

- `frozen=True` hace el objeto **inmutable**: es imposible modificar sus campos;
- `order=True` genera `<`, `>`, `<=`, `>=`: los objetos se comparan campo por campo, en
  el orden en que están escritos.

```python
from dataclasses import dataclass

@dataclass(frozen=True, order=True)
class Version:
    mayor: int
    menor: int

versiones = [Version(2, 1), Version(1, 9), Version(2, 0)]
print(sorted(versiones))
print(max(versiones))
print(Version(1, 0) < Version(1, 2))
```

```sortie
[Version(mayor=1, menor=9), Version(mayor=2, menor=0), Version(mayor=2, menor=1)]
Version(mayor=2, menor=1)
True
```

> **Nota:** un objeto `frozen` puede servir de clave de diccionario o entrar en un
> conjunto, porque su valor nunca cambiará.

## TypedDict: describir un diccionario

A veces, tus datos siguen siendo diccionarios (por ejemplo, leídos de un
[JSON](../debuter/glossaire.md#json)). `TypedDict` describe las claves esperadas y el
tipo de cada valor. Durante la ejecución, es un diccionario normal.

```python
from typing import TypedDict

class Pelicula(TypedDict):
    titulo: str
    anio: int

def resumir(pelicula: Pelicula) -> str:
    return f"{pelicula['titulo']} ({pelicula['anio']})"

alien: Pelicula = {"titulo": "Alien", "anio": 1979}
print(resumir(alien))
print(type(alien))
```

```sortie
Alien (1979)
<class 'dict'>
```

## Protocol: describir un comportamiento

En el capítulo [herencia y polimorfismo](heritage.md), el *duck typing* aceptaba
cualquier objeto que tuviera el método adecuado. `Protocol` escribe esta regla en forma
de tipo: «cualquier objeto que tenga un método `volar()` que devuelva un `str`».

```python
from typing import Protocol

class Volador(Protocol):
    def volar(self) -> str: ...

class Pajaro:
    def volar(self) -> str:
        return "Bato las alas"

class Avion:
    def volar(self) -> str:
        return "Hago rugir mis motores"

def despegar(v: Volador) -> None:
    print(v.volar())

despegar(Pajaro())
despegar(Avion())
```

```sortie
Bato las alas
Hago rugir mis motores
```

`Pajaro` y `Avion` no heredan de `Volador`: les basta con tener el método. `...` (tres
puntos) significa «cuerpo vacío»: un protocolo describe, no hace nada.

## Comprobar con mypy

**mypy** es el comprobador de tipos más conocido. No forma parte de Python: se instala con
pip (ver [módulos, paquetes y pip](modules.md)).

```sh
python3 -m pip install mypy
```

Tomemos este archivo `main.py`, lleno de errores de tipo:

```python norun
def promedio(notas: list[int]) -> float:
    return sum(notas) / len(notas)

def saludar(nombre: str) -> str:
    return "Hola " + nombre

print(promedio([12, 15]))
print(promedio("12, 15"))
edad: int = "diez"
saludar(42)
```

Ejecutamos mypy sobre el archivo:

```sh
mypy main.py
```

```text
main.py:8: error: Argument 1 to "promedio" has incompatible type "str"; expected "list[int]"  [arg-type]
main.py:9: error: Incompatible types in assignment (expression has type "str", variable has type "int")  [assignment]
main.py:10: error: Argument 1 to "saludar" has incompatible type "int"; expected "str"  [arg-type]
Found 3 errors in 1 file (checked 1 source file)
```

Cada línea da el archivo, el número de línea y el problema. mypy ha encontrado los tres
errores **sin ejecutar** el programa. Python, en cambio, se habría detenido en la línea 8
con un `TypeError`, sin ver las dos siguientes.

> **Consejo:** la mayoría de los editores (VS Code con Pylance, PyCharm) hacen la misma
> comprobación en directo y subrayan los errores mientras escribes.

## Errores frecuentes

### Olvidar el caso None

```python norun
def buscar(nombres: list[str], inicio: str) -> str | None:
    for nombre in nombres:
        if nombre.startswith(inicio):
            return nombre
    return None

resultado = buscar(["Alice", "Bob"], "B")
print(resultado.upper())
```

```text
main.py:8: error: Item "None" of "str | None" has no attribute "upper"  [union-attr]
Found 1 error in 1 file (checked 1 source file)
```

mypy te recuerda que `resultado` puede valer `None`. Compruébalo primero:
`if resultado is not None:`. Sin esta comprobación, el programa fallaría el día en que no
se encuentre nada.

### Una lista como valor por defecto de una dataclass

```python norun
from dataclasses import dataclass

@dataclass
class Alumno:
    nombre: str
    notas: list[int] = []
```

```text
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    @dataclass
     ^^^^^^^^^
  ...
ValueError: mutable default <class 'list'> for field notas is not allowed: use default_factory
```

(El traceback completo pasa por varias líneas del módulo `dataclasses`, recortadas aquí.)
Una sola lista sería compartida por todos los alumnos. Escribe
`notas: list[int] = field(default_factory=list)`.

### Modificar una dataclass congelada

```python norun
from dataclasses import dataclass

@dataclass(frozen=True)
class Punto:
    x: int
    y: int

p = Punto(1, 2)
p.x = 5
```

```text
Traceback (most recent call last):
  File "main.py", line 9, in <module>
    p.x = 5
    ^^^
  File "<string>", line 16, in __setattr__
dataclasses.FrozenInstanceError: cannot assign to field 'x'
```

Un objeto `frozen` no se modifica: crea uno nuevo. La función `replace` del módulo
`dataclasses` lo hace por ti: `p = replace(p, x=5)`.

## Ponte a prueba

**Pregunta 1:** ¿qué anotación usarías para un diccionario que asocia el nombre de una
ciudad a una lista de temperaturas decimales?

> **Respuesta**
>
> `dict[str, list[float]]`.

**Pregunta 2:** ¿este programa lanza un error cuando lo ejecutas con `python3`?

```python norun
def duplicar(x: int) -> int:
    return x * 2

print(duplicar("ja"))
```

> **Respuesta**
>
> No: muestra `jaja`. Python ignora las anotaciones. Es mypy el que señalaría que `"ja"`
> no es un `int`.

**Pregunta 3:** escribe una dataclass `Libro` con un título, un autor y un número de
páginas (100 por defecto).

> **Respuesta**
>
> ```python
> from dataclasses import dataclass
>
> @dataclass
> class Libro:
>     titulo: str
>     autor: str
>     paginas: int = 100
>
> print(Libro("El principito", "Saint-Exupéry"))
> ```
>
> ```sortie
> Libro(titulo='El principito', autor='Saint-Exupéry', paginas=100)
> ```

## En resumen

- `x: int` y `def f(a: str) -> bool:` anotan los tipos; Python los ignora durante la
  ejecución.
- `list[int]`, `dict[str, int]`, `tuple[str, int]` precisan el contenido de las
  colecciones.
- `X | None` (u `Optional[X]`): un valor que puede faltar; comprueba `is not None`.
- `@dataclass` genera `__init__`, `__repr__` y `__eq__`; `frozen` y `order` añaden la
  inmutabilidad y la ordenación.
- `TypedDict` describe un diccionario, `Protocol` un comportamiento.
- `mypy main.py` encuentra los errores de tipo sin ejecutar el programa.

Siguiente: [iteradores y generadores](iterateurs.md)
