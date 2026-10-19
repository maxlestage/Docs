# Las colecciones avanzadas

Esta página presenta las cajas especializadas de Python: `namedtuple`, `deque`, `heapq`, `bisect`, `Counter` y `defaultdict`, y te ayuda a elegir la adecuada para cada problema.

> **Objetivos:**
>
> - sabrás dar nombres a las casillas de una tupla con `namedtuple`;
> - sabrás gestionar una cola de espera con `deque`;
> - sabrás sacar siempre el elemento más prioritario con `heapq`;
> - sabrás buscar e insertar en una lista ordenada con `bisect`;
> - sabrás contar con `Counter` y agrupar con `defaultdict`;
> - sabrás elegir la colección adecuada gracias a una tabla.

## ¿Por qué otras colecciones?

Ya conoces [las listas](collections.md), [las tuplas](collections.md#las-tuplas)
y [los diccionarios](dictionnaires.md). Hacen casi todo. Pero para ciertos
trabajos, existe una herramienta más adecuada.

Piensa en una cocina. Un cuchillo de cocina lo corta todo. Pero para pelar una patata,
un pelador va más rápido y corta menos los dedos. Las colecciones de esta página
son peladores: cada una hace **una** cosa, muy bien.

```text
El problema                          La herramienta
─────────────────────────────────    ─────────────
«casillas con nombres»           →   namedtuple
«el primero que llega, primero»  →   deque
«lo más urgente primero»         →   heapq
«mantener una lista ordenada»    →   bisect
«contar»                         →   Counter
«ordenar por grupos»             →   defaultdict
```

Casi todas vienen del [módulo](../debuter/glossaire.md#modulo) `collections`.
`heapq` y `bisect` son dos pequeños módulos aparte. Todo viene con Python: no hay nada
que instalar.

> **Recuerda:** estas herramientas no hacen nada que una lista o un diccionario no sepa
> hacer. Lo hacen de forma más **clara**, y a menudo más **rápida**.

## namedtuple: una tupla con nombres

Una [tupla](../debuter/glossaire.md#tupla) guarda valores en posiciones numeradas.
¿Qué es `point[0]`? ¿La x? ¿La y? Hay que acordarse.

Una `namedtuple` es una tupla en la que cada casilla lleva una **etiqueta**. Es una
ficha de identidad: se lee «Apellido», «Nombre», no «casilla 0», «casilla 1».

```python
from collections import namedtuple

Punto = namedtuple("Punto", ["x", "y"])

p = Punto(3, 4)
print(p)
print(p.x)
print(p.y)
print(p[0])
```

```sortie
Punto(x=3, y=4)
3
4
3
```

Línea por línea:

| Código | Lo que hace |
|---|---|
| `namedtuple("Punto", ["x", "y"])` | fabrica un nuevo **modelo** de ficha, con dos casillas |
| `Punto(3, 4)` | rellena una ficha: `x` recibe 3, `y` recibe 4 |
| `p.x` | lee la casilla llamada `x` |
| `p[0]` | sigue funcionando: es una tupla de verdad |

```text
         Punto(3, 4)
   ┌──────────┬──────────┐
   │ x  =  3  │ y  =  4  │
   └──────────┴──────────┘
     p.x o p[0]   p.y o p[1]
```

> **Pruébalo:** crea `Punto(y=10, x=2)`. El orden de los argumentos con nombre no
> importa: `print` muestra `Punto(x=2, y=10)`.

### Desempaquetar y cambiar una casilla

Una `namedtuple` se desempaqueta como una tupla. Es [inmutable](../debuter/glossaire.md#inmutable):
para «cambiar» una casilla, `_replace` fabrica una ficha **nueva**.

```python
from collections import namedtuple

Alumno = namedtuple("Alumno", ["nombre", "nota"])

a = Alumno("Lucía", 12)
nombre, nota = a
print(nombre, nota)

a2 = a._replace(nota=15)
print(a)
print(a2)
print(a2._asdict())
```

```sortie
Lucía 12
Alumno(nombre='Lucía', nota=12)
Alumno(nombre='Lucía', nota=15)
{'nombre': 'Lucía', 'nota': 15}
```

`a` no ha cambiado. `a2` es una copia con otra nota. `_asdict()` convierte la
ficha en diccionario.

> **Recuerda:** usa una `namedtuple` para un pequeño grupo fijo de valores (un punto,
> un color, una fila de una tabla). El código lee `a.nota` en lugar de `a[1]`: se
> entiende solo.

## deque: una cola de espera

Imagina la cola de la panadería. La gente llega **al final**. Se atiende a quien está
**al principio**. El primero que llega es el primero en ser atendido.

Con una lista, quitar el primer elemento (`lista.pop(0)`) obliga a Python a desplazar
todos los demás una posición. Con 10 personas, da igual. Con un millón, es lento.

Una `deque` (se pronuncia «dek», de *double-ended queue*, «cola de dos extremos»)
añade y quita **por los dos extremos** sin desplazar nada.

```python
from collections import deque

cola = deque()
cola.append("Alicia")
cola.append("Bruno")
cola.append("Carmen")
print(cola)

atendida = cola.popleft()
print("Atendemos a", atendida)
print(cola)
```

```sortie
deque(['Alicia', 'Bruno', 'Carmen'])
Atendemos a Alicia
deque(['Bruno', 'Carmen'])
```

Sigamos la cola en cada paso:

| Paso | Código | Cola (principio → final) |
|---|---|---|
| 1 | `deque()` | *(vacía)* |
| 2 | `append("Alicia")` | Alicia |
| 3 | `append("Bruno")` | Alicia, Bruno |
| 4 | `append("Carmen")` | Alicia, Bruno, Carmen |
| 5 | `popleft()` → `"Alicia"` | Bruno, Carmen |

```text
             popleft()                        append(x)
   salida  ◄──────────  [ Bruno | Carmen ]  ◄──────────  entrada
           (principio)                         (final)
```

### Los cuatro gestos

| Gesto | Extremo izquierdo (principio) | Extremo derecho (final) |
|---|---|---|
| añadir | `appendleft(x)` | `append(x)` |
| quitar | `popleft()` | `pop()` |

### Guardar los N últimos: maxlen

Con `maxlen`, la deque tiene un tamaño máximo. Cuando está llena, añadir por la derecha
hace caer el elemento de la izquierda. Es el historial de «las 3 últimas páginas
visitadas».

```python
from collections import deque

historial = deque(maxlen=3)
for pagina in ["inicio", "productos", "carrito", "pago", "gracias"]:
    historial.append(pagina)
    print(list(historial))
```

```sortie
['inicio']
['inicio', 'productos']
['inicio', 'productos', 'carrito']
['productos', 'carrito', 'pago']
['carrito', 'pago', 'gracias']
```

En la 4.ª página, la deque está llena: `"inicio"` cae por la izquierda.

> **Pruébalo:** cambia `maxlen=3` por `maxlen=2`. Cada línea tiene entonces como mucho
> dos páginas, y la última muestra `['pago', 'gracias']`.

### Un ejemplo: atender a los clientes

```python
from collections import deque

clientes = deque(["Alicia", "Bruno", "Carmen"])
turno = 1
while clientes:
    nombre = clientes.popleft()
    print(f"Turno {turno}: atiendo a {nombre}, quedan {len(clientes)}")
    turno += 1
```

```sortie
Turno 1: atiendo a Alicia, quedan 2
Turno 2: atiendo a Bruno, quedan 1
Turno 3: atiendo a Carmen, quedan 0
```

| Turno | `nombre` | `clientes` después de `popleft` |
|---|---|---|
| 1 | Alicia | Bruno, Carmen |
| 2 | Bruno | Carmen |
| 3 | Carmen | *(vacía)* → `while` se detiene |

`while clientes:` continúa mientras la deque no esté vacía, como con una lista.

> **Recuerda:** para una cola de espera, usa `deque` con `append` (entrar) y
> `popleft` (salir). Nunca `lista.pop(0)` sobre una lista grande.

## heapq: lo más urgente primero

En urgencias del hospital, no se atiende por orden de llegada. Se atiende **lo más
grave primero**. Es una **cola de prioridad**.

El módulo `heapq` convierte una lista normal en un **montículo** (*heap*). Un montículo
guarda siempre el elemento **más pequeño** en la posición 0. El resto no está ordenado:
solo está lo bastante colocado para encontrar rápido el más pequeño.

```python
import heapq

monticulo = []
heapq.heappush(monticulo, 5)
heapq.heappush(monticulo, 1)
heapq.heappush(monticulo, 8)
heapq.heappush(monticulo, 3)
print("el más pequeño:", monticulo[0])

print(heapq.heappop(monticulo))
print(heapq.heappop(monticulo))
print("quedan:", sorted(monticulo))
```

```sortie
el más pequeño: 1
1
3
quedan: [5, 8]
```

| Código | Lo que hace |
|---|---|
| `heappush(monticulo, x)` | añade `x` al montículo |
| `monticulo[0]` | mira el más pequeño, sin quitarlo |
| `heappop(monticulo)` | quita y devuelve el más pequeño |

```text
Echamos números en un embudo:
el más pequeño siempre cae primero.

   5  1  8  3
   ╲        ╱
    ╲      ╱
     ╲    ╱
      ╲  ╱
       1      ← heappop() lo saca
```

### Tareas con una prioridad

Guardamos tuplas `(prioridad, tarea)`. Python compara las tuplas **casilla por
casilla**: primero la prioridad. Cuanto más pequeño es el número, más urgente es.

```python
import heapq

tareas = []
heapq.heappush(tareas, (3, "regar las plantas"))
heapq.heappush(tareas, (1, "apagar el horno"))
heapq.heappush(tareas, (2, "abrir al cartero"))

while tareas:
    prioridad, tarea = heapq.heappop(tareas)
    print(prioridad, tarea)
```

```sortie
1 apagar el horno
2 abrir al cartero
3 regar las plantas
```

El orden en que se añadieron era 3, 1, 2. El orden de salida es 1, 2, 3.

| Turno | `heappop` saca | Queda |
|---|---|---|
| 1 | (1, apagar el horno) | (2, …), (3, …) |
| 2 | (2, abrir al cartero) | (3, …) |
| 3 | (3, regar las plantas) | *(vacío)* |

> **Pruébalo:** añade `(0, "llamar a los bomberos")`. Sale la primera.

### Los N mayores o menores

Para obtener solo el podio, bastan `nlargest` y `nsmallest`:

```python
import heapq

puntos = [42, 17, 99, 63, 8, 75]
print(heapq.nlargest(3, puntos))
print(heapq.nsmallest(2, puntos))
```

```sortie
[99, 75, 63]
[8, 17]
```

> **Recuerda:** `heapq` saca siempre el **más pequeño** primero. Guarda tuplas
> `(prioridad, valor)` para una cola de prioridad. El montículo **no** es una lista
> ordenada: no muestres `monticulo` esperando un orden.

## bisect: buscar en una lista ordenada

Para encontrar una palabra en un diccionario de papel, no lees todas las páginas.
Lo abres por la mitad, miras si la palabra está antes o después, y vuelves a empezar
con la mitad buena. Es la **búsqueda binaria**.

El módulo `bisect` hace exactamente eso, sobre una lista **ya ordenada**.

```python
import bisect

notas = [5, 8, 12, 15, 18]
print(bisect.bisect(notas, 13))
```

```sortie
3
```

`bisect(notas, 13)` responde: «para mantener la lista ordenada, 13 iría en la
posición 3» (justo antes de 15). Esta es la búsqueda paso a paso:

| Paso | Trozo mirado | Mitad | 13 está… |
|---|---|---|---|
| 1 | `[5, 8, 12, 15, 18]` (posiciones 0–4) | posición 2 → 12 | después de 12 → nos quedamos con la derecha |
| 2 | `[15, 18]` (posiciones 3–4) | posición 4 → 18 | antes de 18 → nos quedamos con la izquierda |
| 3 | `[15]` (posición 3) | posición 3 → 15 | antes de 15 → es aquí: **3** |

Tres vistazos para 5 elementos. Para un millón de elementos, bastan unos veinte.

### insort: insertar en el sitio correcto

`insort` inserta un elemento **en su sitio**. La lista sigue ordenada, sin llamar a
`sort`.

```python
import bisect

notas = [5, 8, 15, 18]
bisect.insort(notas, 12)
bisect.insort(notas, 1)
print(notas)
```

```sortie
[1, 5, 8, 12, 15, 18]
```

### Una escala de notas con bisect

Un uso clásico: convertir una nota en una calificación. Los umbrales están ordenados;
`bisect` dice en qué «tramo» cae la nota.

```python
import bisect

umbrales = [10, 12, 14, 16]
calificaciones = ["Suspenso", "Aprobado", "Bien", "Notable", "Sobresaliente"]

def calificacion(nota):
    return calificaciones[bisect.bisect(umbrales, nota)]

for nota in [7, 10, 13, 17]:
    print(nota, "→", calificacion(nota))
```

```sortie
7 → Suspenso
10 → Aprobado
13 → Bien
17 → Sobresaliente
```

```text
umbrales:       10        12        14        16
         ───────┼─────────┼─────────┼─────────┼────────
posición:   0   │    1    │    2    │    3    │    4
        Suspenso│Aprobado │  Bien   │ Notable │Sobresal.
                ▲            ▲                     ▲
              nota 10     nota 13               nota 17
```

| `nota` | `bisect(umbrales, nota)` | `calificaciones[...]` |
|---|---|---|
| 7 | 0 | Suspenso |
| 10 | 1 | Aprobado |
| 13 | 2 | Bien |
| 17 | 4 | Sobresaliente |

Una nota igual a un umbral (10) va **después** de él: recibe la calificación del tramo
de arriba.

> **Pruébalo:** prueba `calificacion(16)`. Obtienes `Sobresaliente`. Con
> `bisect.bisect_left`, una nota igual al umbral iría **antes**: 16 daría `Notable`.

> **Recuerda:** `bisect` solo funciona con una lista **ordenada**. Con una lista
> desordenada, no falla: da una respuesta equivocada.

## Counter en detalle

`Counter` ya se presentó en [los diccionarios](dictionnaires.md#con-collections-counter).
Es un diccionario cuyos valores son **números de apariciones**. Piensa en el recuento
de votos en la pizarra: una raya por voto.

```python
from collections import Counter

votos = ["pizza", "sushi", "pizza", "tacos", "pizza", "sushi"]
c = Counter(votos)
print(c)
print(c["pizza"], c["crepes"])
print(c.most_common(1))
```

```sortie
Counter({'pizza': 3, 'sushi': 2, 'tacos': 1})
3 0
[('pizza', 3)]
```

Paso a paso, Counter hace esto por ti:

| Voto leído | pizza | sushi | tacos |
|---|---|---|---|
| pizza | 1 | 0 | 0 |
| sushi | 1 | 1 | 0 |
| pizza | 2 | 1 | 0 |
| tacos | 2 | 1 | 1 |
| pizza | 3 | 1 | 1 |
| sushi | 3 | 2 | 1 |

### Añadir, quitar, sumar

`update` añade recuentos. `subtract` los quita. Dos `Counter` se suman con
`+`.

```python
from collections import Counter

almacen = Counter(manzanas=5, peras=2)
almacen.update({"manzanas": 3, "kiwis": 4})
print(almacen)

almacen.subtract({"manzanas": 6})
print(almacen["manzanas"])

lunes = Counter(cafe=3, te=1)
martes = Counter(cafe=2, chocolate=1)
print(lunes + martes)
print(sum(lunes.values()))
```

```sortie
Counter({'manzanas': 8, 'kiwis': 4, 'peras': 2})
2
Counter({'cafe': 5, 'te': 1, 'chocolate': 1})
4
```

> **Pruébalo:** quita 10 manzanas en lugar de 6. `almacen["manzanas"]` vale entonces
> `-2`: `Counter` acepta recuentos negativos.

> **Recuerda:** en cuanto escribas `cuenta[x] = cuenta.get(x, 0) + 1`, piensa en
> `Counter`. Una clave ausente vale 0, sin error.

## defaultdict en detalle

`defaultdict` también se vio en [los diccionarios](dictionnaires.md#con-defaultdict).
Es un diccionario que, cuando lees una clave ausente, **crea** un valor inicial
en lugar de lanzar `KeyError`.

Es un archivador mágico: pides el separador «B», no existe, el archivador lo fabrica
solo, vacío, y te lo da.

Se le da una **función sin argumentos** que fabrica el valor inicial:

| Escribes | Valor creado para una clave ausente |
|---|---|
| `defaultdict(int)` | `0` |
| `defaultdict(list)` | `[]` |
| `defaultdict(set)` | `set()` |
| `defaultdict(str)` | `""` |

### Agrupar

```python
from collections import defaultdict

alumnos = [("Lucía", "4A"), ("Hugo", "4B"), ("Inés", "4A"), ("Pablo", "4B"), ("Sara", "4A")]

clases = defaultdict(list)
for nombre, clase in alumnos:
    clases[clase].append(nombre)

for clase, nombres in clases.items():
    print(clase, nombres)
```

```sortie
4A ['Lucía', 'Inés', 'Sara']
4B ['Hugo', 'Pablo']
```

| Vuelta | `nombre`, `clase` | ¿Existe la clave? | `clases` después de la vuelta |
|---|---|---|---|
| 1 | Lucía, 4A | no → se crea `[]` | 4A: [Lucía] |
| 2 | Hugo, 4B | no → se crea `[]` | 4A: [Lucía], 4B: [Hugo] |
| 3 | Inés, 4A | sí | 4A: [Lucía, Inés], 4B: [Hugo] |
| 4 | Pablo, 4B | sí | 4A: [Lucía, Inés], 4B: [Hugo, Pablo] |
| 5 | Sara, 4A | sí | 4A: [Lucía, Inés, Sara], 4B: [Hugo, Pablo] |

### Sumar por grupo

Con `int`, el valor inicial es 0: perfecto para hacer totales.

```python
from collections import defaultdict

compras = [("Lucía", 12.5), ("Hugo", 8.0), ("Lucía", 4.5), ("Hugo", 2.0), ("Inés", 10.0)]

total = defaultdict(int)
for nombre, precio in compras:
    total[nombre] += precio

print(dict(total))
```

```sortie
{'Lucía': 17.0, 'Hugo': 10.0, 'Inés': 10.0}
```

### La trampa: leer basta para crear

Con `defaultdict`, **mirar** una clave ausente la crea. Para comprobar sin crear,
usa `in`.

```python
from collections import defaultdict

d = defaultdict(list)
print("x" in d)
print(d["x"])
print("x" in d)
print(dict(d))
```

```sortie
False
[]
True
{'x': []}
```

> **Recuerda:** `defaultdict(list)` para agrupar, `defaultdict(int)` para
> sumar. Comprueba una clave con `in`, no con `d[clave]`.

## ¿Qué colección elegir?

Esta es la tabla para elegir. Lee la columna «Quieres…» y toma la herramienta de enfrente.

| Quieres… | Herramienta | Gesto principal | Ejemplo de la vida real |
|---|---|---|---|
| una secuencia ordenada y modificable | `list` | `append`, `[i]` | una lista de la compra |
| casillas con nombre, fijas | `namedtuple` | `p.x` | una ficha de identidad |
| atender por orden de llegada | `deque` | `append` + `popleft` | la cola de la panadería |
| guardar los N últimos | `deque(maxlen=N)` | `append` | el historial del navegador |
| sacar lo más urgente | `heapq` | `heappush` + `heappop` | urgencias del hospital |
| el podio de los N mejores | `heapq.nlargest` | una sola llamada | las 3 mejores puntuaciones |
| buscar en una lista ordenada | `bisect` | `bisect`, `insort` | el diccionario de papel |
| contar apariciones | `Counter` | `Counter(lista)`, `most_common` | el recuento de votos |
| ordenar por grupos | `defaultdict(list)` | `d[clave].append(x)` | los separadores de un archivador |
| encontrar por una clave | `dict` | `d[clave]` | una guía telefónica |
| sin duplicados | `set` | `add`, `in` | la lista de invitados |

```text
Tengo que…
 ├─ ¿contar?                           → Counter
 ├─ ¿agrupar?                          → defaultdict(list)
 ├─ ¿tratar por orden de llegada?      → deque
 ├─ ¿tratar lo más importante primero? → heapq
 ├─ ¿mantener una lista ordenada?      → bisect
 ├─ ¿nombrar casillas de una tupla?    → namedtuple
 └─ nada de eso                        → list o dict, sin más
```

> **Recuerda:** empieza siempre con una lista o un diccionario. Pasa a una herramienta
> especializada cuando tu código se parezca a una de las filas de la tabla.

## Errores frecuentes

### Modificar una namedtuple

```python norun
from collections import namedtuple
Punto = namedtuple("Punto", ["x", "y"])
p = Punto(1, 2)
p.x = 5
```

```text
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    p.x = 5
    ^^^
AttributeError: can't set attribute
```

Una `namedtuple` es inmutable, como una tupla. Fabrica una ficha nueva:
`p = p._replace(x=5)`. Si tienes que modificarla a menudo, usa mejor una
[dataclass](typage.md).

### Quitar de una deque vacía

```python norun
from collections import deque
cola = deque()
cola.popleft()
```

```text
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    cola.popleft()
    ~~~~~~~~~~~~^^
IndexError: pop from an empty deque
```

No hay nadie en la cola. Compruébalo antes: `if cola: cola.popleft()`, o haz un bucle
con `while cola:`.

### Dos prioridades iguales con valores no comparables

```python norun
import heapq
tareas = []
heapq.heappush(tareas, (2, {"nombre": "lavar"}))
heapq.heappush(tareas, (2, {"nombre": "ordenar"}))
```

```text
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    heapq.heappush(tareas, (2, {"nombre": "ordenar"}))
    ~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
TypeError: '<' not supported between instances of 'dict' and 'dict'
```

Las dos prioridades valen 2. Python compara entonces la casilla siguiente: dos
diccionarios, que no sabe comparar. Añade un número de orden único en medio:
`(2, numero, {"nombre": "ordenar"})`, con `numero` aumentando en 1 en cada inserción.

### bisect sobre una lista sin ordenar

```python
import bisect

notas = [15, 5, 18, 8]
bisect.insort(notas, 16)
print(notas)
```

```sortie
[15, 5, 16, 18, 8]
```

No hay error, pero el resultado no tiene sentido: `insort` cree que la lista está
ordenada y coloca 16 en el sitio equivocado. Ordena primero con `notas.sort()`, y luego
mantén la lista ordenada con `insort`.

## Ponte a prueba

**Pregunta 1:** escribes un juego en el que los jugadores esperan su turno, por orden de
llegada. ¿Qué colección usas, y con qué métodos?

> **Respuesta**
>
> Una `deque`. Se añade un jugador con `append` y se atiende al siguiente con `popleft`.
> A diferencia de `lista.pop(0)`, `popleft` no desplaza nada, incluso con muchos
> jugadores.

**Pregunta 2:** ¿qué muestra este programa?

```python norun
import heapq
monticulo = []
for x in [7, 2, 9, 4]:
    heapq.heappush(monticulo, x)
print(heapq.heappop(monticulo), heapq.heappop(monticulo))
```

> **Respuesta**
>
> ```python
> import heapq
> monticulo = []
> for x in [7, 2, 9, 4]:
>     heapq.heappush(monticulo, x)
> print(heapq.heappop(monticulo), heapq.heappop(monticulo))
> ```
>
> ```sortie
> 2 4
> ```
>
> `heappop` saca siempre el más pequeño: primero 2, luego 4.

**Pregunta 3:** ¿cuál es la diferencia entre `Counter` y `defaultdict(int)`?

> **Respuesta**
>
> Los dos dan 0 para una clave ausente. Pero `Counter` cuenta una lista entera de una
> vez (`Counter(votos)`) y ofrece `most_common`. Además, leer una clave ausente de un
> `Counter` no la crea, mientras que un `defaultdict` la añade.

## En resumen

- `namedtuple` da nombres a las casillas de una tupla: `p.x` en lugar de `p[0]`.
- `deque` es una cola de dos extremos: `append` para entrar, `popleft` para salir;
  `maxlen` guarda los N últimos.
- `heapq` saca siempre el más pequeño: guarda tuplas `(prioridad, valor)`.
- `bisect` busca e `insort` inserta en una lista **ordenada**, muy rápido.
- `Counter` cuenta, `defaultdict(list)` agrupa, `defaultdict(int)` suma.
- En caso de duda, empieza con una lista o un diccionario.

Siguiente: [Funciones avanzadas y decoradores](decorateurs.md)
