# Analizar datos

Esta página muestra cómo leer una tabla de datos CSV, calcular estadísticas sencillas (media, mediana, moda, desviación típica), agrupar, contar, ordenar y presentar un pequeño informe, primero con la biblioteca estándar y después con pandas.

> **Objetivos:**
>
> - sabrás leer datos CSV y convertir los números;
> - sabrás calcular la media, la mediana, la moda y la desviación típica con
>   `statistics`;
> - sabrás agrupar y contar con `Counter` y `defaultdict`;
> - sabrás ordenar filas según una columna;
> - sabrás mostrar un informe bien alineado;
> - sabrás cómo es el mismo trabajo con pandas.

## Analizar es responder a una pregunta

Imagina el cuaderno de una panadería. Cada noche, la panadera apunta lo que ha vendido.
Al cabo de una semana, se hace preguntas:

- «¿Cuántas barras vendo al día, de media?»
- «¿Qué producto me deja más dinero?»
- «¿Qué día es el más tranquilo?»

Analizar datos es eso: partir de una **pregunta**, hacer un **cálculo** sobre la tabla,
obtener una **respuesta**.

```text
  pregunta  ──►  leer los datos  ──►  calcular  ──►  respuesta
 «¿cuántas?»    (el archivo CSV)      (media,       «41,7 al día»
                                        total…)
```

En toda la página usamos la misma tablita de ventas:

| dia | producto | cantidad | precio |
|---|---|---|---|
| lunes | barra | 40 | 1.10 |
| lunes | cruasán | 25 | 1.20 |
| lunes | tarta | 3 | 15.00 |
| martes | barra | 35 | 1.10 |
| martes | cruasán | 30 | 1.20 |
| miércoles | barra | 50 | 1.10 |
| miércoles | cruasán | 20 | 1.20 |
| miércoles | tarta | 5 | 15.00 |

> **Recuerda:** empieza siempre escribiendo tu pregunta en una frase. Te dice qué
> columnas mirar y qué cálculo hacer.

## Leer datos CSV

El formato CSV se presentó en [los formatos de archivo](formats.md).
Normalmente se lee un archivo. Aquí, para que cada ejemplo quepa en un bloque, el texto
CSV está **dentro** del programa. `io.StringIO` lo hace pasar por un archivo: es un
archivo falso, en [memoria](../debuter/glossaire.md#memoria-ram).

```python
import csv
import io

TEXTO = """dia,producto,cantidad,precio
lunes,barra,40,1.10
lunes,cruasán,25,1.20
"""

filas = list(csv.DictReader(io.StringIO(TEXTO)))
print(len(filas), "filas")
print(filas[0])
```

```sortie
2 filas
{'dia': 'lunes', 'producto': 'barra', 'cantidad': '40', 'precio': '1.10'}
```

Cada fila se convierte en un [diccionario](../debuter/glossaire.md#diccionario). Las
claves vienen de la primera línea del CSV.

> **Nota:** con un archivo de verdad, sustituye `io.StringIO(TEXTO)` por un archivo
> abierto: `with open("ventas.csv", encoding="utf-8", newline="") as f:` y luego
> `csv.DictReader(f)`. El resto del código no cambia.

### Convertir los números

Fíjate bien: `'40'` tiene comillas. En CSV, **todo es texto**. Antes de calcular, hay
que convertir cada columna al tipo correcto.

```python
import csv
import io

TEXTO = """dia,producto,cantidad,precio
lunes,barra,40,1.10
lunes,cruasán,25,1.20
lunes,tarta,3,15.00
"""

ventas = []
for fila in csv.DictReader(io.StringIO(TEXTO)):
    ventas.append({
        "dia": fila["dia"],
        "producto": fila["producto"],
        "cantidad": int(fila["cantidad"]),
        "precio": float(fila["precio"]),
    })

print(ventas[2])
print(ventas[2]["cantidad"] * ventas[2]["precio"])
```

```sortie
{'dia': 'lunes', 'producto': 'tarta', 'cantidad': 3, 'precio': 15.0}
45.0
```

| Columna | Texto leído | Conversión | Valor utilizable |
|---|---|---|---|
| dia | `'lunes'` | ninguna | `'lunes'` |
| cantidad | `'3'` | `int` | `3` |
| precio | `'15.00'` | `float` | `15.0` |

> **Pruébalo:** quita el `int(...)` que rodea `fila["cantidad"]`. La última línea falla:
> `TypeError: can't multiply sequence by non-int of type 'float'`. No se puede
> multiplicar un texto por un número decimal.

> **Recuerda:** leer y **convertir enseguida**. Después, solo manejas números de verdad.

## Las estadísticas básicas

El módulo `statistics` calcula las medidas clásicas. Tomemos las barras vendidas
durante seis días.

```python
import statistics

barras = [40, 35, 50, 35, 42, 48]

print("media:", round(statistics.mean(barras), 2))
print("mediana:", statistics.median(barras))
print("moda:", statistics.mode(barras))
print("desviación típica:", round(statistics.stdev(barras), 2))
print("mín, máx:", min(barras), max(barras))
```

```sortie
media: 41.67
mediana: 41.0
moda: 35
desviación típica: 6.35
mín, máx: 35 50
```

Veamos qué significa cada medida.

### La media: repartir a partes iguales

La media es el total repartido a partes iguales. Si juntaras todas las barras en un
montón y las repartieras entre los seis días, ¿cuántas tendría cada día?

| Paso | Valor leído | Total |
|---|---|---|
| 1 | 40 | 40 |
| 2 | 35 | 75 |
| 3 | 50 | 125 |
| 4 | 35 | 160 |
| 5 | 42 | 202 |
| 6 | 48 | 250 |

Total 250, dividido entre 6 días: 41,67.

### La mediana: el valor del medio

Se **ordenan** los valores y se toma el del medio. Como una fila de niños colocados por
estatura: la mediana es el niño del centro.

```text
ordenados: 35   35   40 │ 42   48   50
                        ▲
         6 valores: no hay un único centro,
         se toma la media de los dos del centro:
         (40 + 42) / 2 = 41.0
```

Con un número impar de valores, hay un solo elemento en el medio, y es ese.

¿Por qué la mediana? No se deja engañar por un valor extremo:

```python
import statistics

salarios = [1800, 1900, 2000, 2100, 25000]
print("media:", statistics.mean(salarios))
print("mediana:", statistics.median(salarios))
```

```sortie
media: 6560
mediana: 2000
```

Un solo salario muy alto hace subir la media muy por encima de lo que gana casi todo el
mundo. La mediana, en cambio, se queda en 2000: la mitad gana menos, la mitad gana más.

### La moda: lo más frecuente

La moda es el valor que más se repite. En `[40, 35, 50, 35, 42, 48]`, 35 aparece dos
veces: es la moda. Si hay empate, `statistics.multimode` da todos los ganadores.

```python
import statistics

tallas = [38, 40, 38, 42, 40, 39]
print(statistics.mode(tallas))
print(statistics.multimode(tallas))
```

```sortie
38
[38, 40]
```

### La desviación típica: ¿cambia mucho?

La desviación típica mide cuánto se **alejan** los valores de la media, en general.

- desviación típica pequeña: los valores están apretados alrededor de la media;
- desviación típica grande: están muy dispersos.

```python
import statistics

regular = [40, 41, 39, 40, 40]
irregular = [10, 70, 25, 60, 35]
print(statistics.mean(regular), round(statistics.stdev(regular), 2))
print(statistics.mean(irregular), round(statistics.stdev(irregular), 2))
```

```sortie
40 0.71
40 24.75
```

```text
regular     :  39 ● 40 ●●● 41 ●              (muy cerca de 40)
irregular   :  10 ●   25 ●  35 ●    60 ●  70 ●   (por todas partes)
                         media 40
```

La misma media, 40, pero días muy distintos. El primer panadero sabe cuánto preparar; el
segundo nunca lo sabe.

| Medida | Función | Pregunta a la que responde |
|---|---|---|
| media | `statistics.mean` | «¿cuánto, repartiendo a partes iguales?» |
| mediana | `statistics.median` | «¿cuál es el valor del medio?» |
| moda | `statistics.mode` | «¿qué valor se repite más?» |
| desviación típica | `statistics.stdev` | «¿cambia mucho?» |

> **Pruébalo:** en `salarios`, sustituye 25000 por 2200. La media vuelve a bajar a 2000
> y se acerca a la mediana: cuando no hay ningún valor extremo, las dos se parecen.

> **Recuerda:** mira la media **y** la mediana. Si son muy distintas, un valor extremo
> está tirando de la media.

## Agrupar y contar

La mayoría de las preguntas tienen la forma «… **por** …»: las ventas **por** producto,
los ingresos **por** día. Hay que agrupar las filas. Las herramientas son las de las
[colecciones avanzadas](collections-avancees.md).

### Contar filas con Counter

«¿Cuántas veces aparece cada producto en el cuaderno?»

```python
from collections import Counter

productos = ["barra", "cruasán", "tarta", "barra",
             "cruasán", "barra", "cruasán", "tarta"]
print(Counter(productos))
```

```sortie
Counter({'barra': 3, 'cruasán': 3, 'tarta': 2})
```

### Sumar por grupo con defaultdict

«¿Cuántas unidades de cada producto se han vendido?» Ya no se trata de contar las filas,
sino de **sumar** una columna por grupo.

```python
import csv
import io
from collections import defaultdict

TEXTO = """dia,producto,cantidad,precio
lunes,barra,40,1.10
lunes,cruasán,25,1.20
lunes,tarta,3,15.00
martes,barra,35,1.10
martes,cruasán,30,1.20
"""

total = defaultdict(int)
for fila in csv.DictReader(io.StringIO(TEXTO)):
    total[fila["producto"]] += int(fila["cantidad"])

print(dict(total))
```

```sortie
{'barra': 75, 'cruasán': 55, 'tarta': 3}
```

Sigamos el diccionario `total` fila tras fila:

| Fila leída | producto | cantidad | barra | cruasán | tarta |
|---|---|---|---|---|---|
| 1 | barra | 40 | 40 | — | — |
| 2 | cruasán | 25 | 40 | 25 | — |
| 3 | tarta | 3 | 40 | 25 | 3 |
| 4 | barra | 35 | 75 | 25 | 3 |
| 5 | cruasán | 30 | 75 | 55 | 3 |

Un guion significa «la clave todavía no existe». La primera vez que aparece,
`defaultdict(int)` la crea con 0 y luego se suma.

```text
Las filas del cuaderno             Las cajas por producto
 barra     40 ──────────────►  ┌  barra   ┐  40 + 35 = 75
 cruasán   25 ─────────┐       └──────────┘
 tarta      3 ──┐      └────►  ┌ cruasán  ┐  25 + 30 = 55
 barra     35 ──┼──► (barra)   └──────────┘
 cruasán   30 ──┼──► (cruasán)
                └──────────►   ┌  tarta   ┐  3
                               └──────────┘
```

> **Pruébalo:** sustituye `int(fila["cantidad"])` por
> `int(fila["cantidad"]) * float(fila["precio"])`, y `defaultdict(int)` por
> `defaultdict(float)`. Obtienes los ingresos por producto en lugar de las cantidades.

> **Recuerda:** «contar las filas por …» → `Counter`. «sumar una columna por …» →
> `defaultdict(int)` con `+=`.

## Ordenar

«¿Qué producto se vende mejor?» Hay que ordenar los grupos según su total. Se ordenan
los pares `(producto, total)` con `sorted` y una clave (mira
[ordenar según un criterio](collections.md#ordenar-segun-un-criterio-con-key)).

```python
total = {"barra": 125, "cruasán": 75, "tarta": 8}

por_total = sorted(total.items(), key=lambda par: par[1], reverse=True)
print(por_total)

for puesto, (producto, cantidad) in enumerate(por_total, start=1):
    print(puesto, producto, cantidad)
```

```sortie
[('barra', 125), ('cruasán', 75), ('tarta', 8)]
1 barra 125
2 cruasán 75
3 tarta 8
```

| Código | Qué hace |
|---|---|
| `total.items()` | da los pares `('barra', 125)`, `('cruasán', 75)`… |
| `key=lambda par: par[1]` | ordena según la 2.ª casilla del par: el total |
| `reverse=True` | de mayor a menor |
| `enumerate(..., start=1)` | numera los puestos empezando por 1 |

### Ordenar filas según dos columnas

Se pueden ordenar las propias filas. Con una tupla como clave, Python ordena primero
según la primera casilla y desempata con la segunda.

```python
ventas = [
    {"dia": "lunes", "producto": "tarta", "cantidad": 3},
    {"dia": "martes", "producto": "barra", "cantidad": 35},
    {"dia": "lunes", "producto": "barra", "cantidad": 40},
    {"dia": "martes", "producto": "cruasán", "cantidad": 30},
]

for v in sorted(ventas, key=lambda v: (v["producto"], -v["cantidad"])):
    print(v["producto"], v["dia"], v["cantidad"])
```

```sortie
barra lunes 40
barra martes 35
cruasán martes 30
tarta lunes 3
```

El signo `-` delante de la cantidad invierte el orden solo para esa columna: los
productos están en orden alfabético y, para un mismo producto, la venta más grande va
primero.

> **Recuerda:** `sorted(..., key=...)` ordena según lo que tú quieras. `reverse=True`
> para ir de mayor a menor.

## Un pequeño informe en tabla

Una respuesta se lee mejor cuando está bien alineada. Las
[f-strings](texte.md) saben reservar un ancho: `:<12` alinea a la izquierda en 12
caracteres, `:>6` a la derecha en 6, `:>8.2f` a la derecha en 8 con 2 cifras
decimales.

```python
import csv
import io
from collections import defaultdict

TEXTO = """dia,producto,cantidad,precio
lunes,barra,40,1.10
lunes,cruasán,25,1.20
lunes,tarta,3,15.00
martes,barra,35,1.10
martes,cruasán,30,1.20
miércoles,barra,50,1.10
miércoles,cruasán,20,1.20
miércoles,tarta,5,15.00
"""

cantidades = defaultdict(int)
ingresos = defaultdict(float)
for fila in csv.DictReader(io.StringIO(TEXTO)):
    c = int(fila["cantidad"])
    cantidades[fila["producto"]] += c
    ingresos[fila["producto"]] += c * float(fila["precio"])

print(f"{'Producto':<12}{'Cant.':>6}{'Ingresos':>10}")
print("-" * 28)
for producto in sorted(ingresos, key=ingresos.get, reverse=True):
    print(f"{producto:<12}{cantidades[producto]:>6}{ingresos[producto]:>10.2f}")
print("-" * 28)
print(f"{'Total':<12}{sum(cantidades.values()):>6}{sum(ingresos.values()):>10.2f}")
```

```sortie
Producto     Cant.  Ingresos
----------------------------
barra          125    137.50
tarta            8    120.00
cruasán         75     90.00
----------------------------
Total          208    347.50
```

```text
 {producto:<12}        {cantidad:>6}  {ingresos:>10.2f}
 ├───── 12 ─────┤├─── 6 ──┤├────── 10 ──────┤
 barra                125          137.50
```

`key=ingresos.get` ordena los nombres de los productos según sus ingresos:
`ingresos.get("tarta")` vale 120.0, etc.

> **Pruébalo:** sustituye `:<12` por `:>12`. Los nombres de los productos se alinean
> entonces a la derecha, pegados a los números.

> **Recuerda:** en un informe, el mismo ancho para el encabezado y para las filas. Los
> números a la derecha, el texto a la izquierda.

## ¿Y pandas?

Para tablas grandes, los analistas usan **pandas**, una
[biblioteca](../debuter/glossaire.md#biblioteca) externa. No viene con Python: se
instala con [pip](../debuter/glossaire.md#pip) (`pip install pandas`), preferiblemente
en un [entorno virtual](../debuter/glossaire.md#entorno-virtual). Por eso los
bloques siguientes no se ejecutan en este sitio, pero sus salidas son reales.

pandas guarda los datos en un **DataFrame**: una tabla, como una hoja de cálculo, con
columnas con nombre. Los tipos se adivinan solos.

```python norun
import io
import pandas as pd

TEXTO = """dia,producto,cantidad,precio
lunes,barra,40,1.10
lunes,cruasán,25,1.20
lunes,tarta,3,15.00
martes,barra,35,1.10
martes,cruasán,30,1.20
miércoles,barra,50,1.10
miércoles,cruasán,20,1.20
miércoles,tarta,5,15.00
"""

df = pd.read_csv(io.StringIO(TEXTO))
print(df.head(3))
print(df["cantidad"].mean())
```

```text
     dia producto  cantidad  precio
0  lunes    barra        40     1.1
1  lunes  cruasán        25     1.2
2  lunes    tarta         3    15.0
26.0
```

Agrupar, sumar y ordenar cabe en una línea:

```python norun
df["ingresos"] = df["cantidad"] * df["precio"]
informe = df.groupby("producto")[["cantidad", "ingresos"]].sum()
print(informe.sort_values("ingresos", ascending=False))
```

```text
          cantidad  ingresos
producto
barra          125     137.5
tarta            8     120.0
cruasán         75      90.0
```

| Con la biblioteca estándar | Con pandas |
|---|---|
| `csv.DictReader` + conversiones a mano | `pd.read_csv` (tipos adivinados) |
| `statistics.mean(lista)` | `df["columna"].mean()` |
| `defaultdict` + bucle | `df.groupby("producto").sum()` |
| `sorted(..., key=...)` | `df.sort_values("columna")` |
| f-strings para alinear | visualización alineada automática |

> **Recuerda:** para unos cientos de filas, `csv` y `statistics` bastan y no piden
> instalar nada. pandas se vuelve útil cuando las tablas son grandes o las preguntas
> muchas. Entender primero la versión «a mano» ayuda a entender lo que pandas hace por
> ti.

## Errores frecuentes

### Calcular con textos

```python norun
import statistics
notas = ["12", "15", "9"]
print(statistics.mean(notas))
```

```text
Traceback (most recent call last):
  ...
  File "/usr/lib/python3.13/statistics.py", line 334, in _exact_ratio
    raise TypeError(msg)
TypeError: can't convert type 'str' to numerator/denominator
```

Los valores leídos de un CSV son textos. Convierte primero:
`statistics.mean([int(n) for n in notas])`.

### Una desviación típica con un solo valor

```python norun
import statistics
print(statistics.stdev([12]))
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(statistics.stdev([12]))
          ~~~~~~~~~~~~~~~~^^^^^^
  File "/usr/lib/python3.13/statistics.py", line 1207, in stdev
    raise StatisticsError('stdev requires at least two data points')
statistics.StatisticsError: stdev requires at least two data points
```

Con un solo valor, no se puede medir ninguna variación. Del mismo modo,
`statistics.mean([])` lanza `StatisticsError: mean requires at least one data point`.
Comprueba el tamaño de la lista antes de calcular: `if len(valores) >= 2:`.

### Ordenar números que siguen siendo texto

```python
cantidades = ["40", "5", "125", "8"]
print(sorted(cantidades))
print(sorted(cantidades, key=int))
```

```sortie
['125', '40', '5', '8']
['5', '8', '40', '125']
```

No hay error, pero sí un orden raro: los textos se ordenan **carácter a carácter**, como
en un diccionario. `"125"` empieza por `1`, así que va antes que `"40"`. Convierte a
números, u ordena con `key=int`.

## Ponte a prueba

**Pregunta 1:** las notas de una clase son `[2, 14, 15, 15, 16]`. ¿Por qué la media y
la mediana son distintas?

> **Respuesta**
>
> La media vale 12,4 y la mediana vale 15. La nota 2, muy baja, tira de la media hacia
> abajo. La mediana, en cambio, solo toma el valor del medio de las notas ordenadas.

**Pregunta 2:** ¿qué muestra este programa?

```python norun
from collections import defaultdict
compras = [("Léa", 5), ("Tom", 3), ("Léa", 2)]
total = defaultdict(int)
for nombre, n in compras:
    total[nombre] += n
print(dict(total))
```

> **Respuesta**
>
> ```python
> from collections import defaultdict
> compras = [("Léa", 5), ("Tom", 3), ("Léa", 2)]
> total = defaultdict(int)
> for nombre, n in compras:
>     total[nombre] += n
> print(dict(total))
> ```
>
> ```sortie
> {'Léa': 7, 'Tom': 3}
> ```
>
> Léa: 0 + 5 + 2 = 7. Tom: 0 + 3 = 3.

**Pregunta 3:** lees un CSV con `csv.DictReader`. ¿Por qué `fila["cantidad"] * 2` da
`'4040'` en lugar de `80`?

> **Respuesta**
>
> Porque `fila["cantidad"]` es el texto `'40'`. Multiplicar un texto por 2 lo repite.
> Hay que convertir: `int(fila["cantidad"]) * 2`.

## En resumen

- Parte de una pregunta; te dice qué columnas mirar y qué calcular.
- `csv.DictReader` lee un CSV (un archivo o `io.StringIO`); todo es texto: convierte
  enseguida con `int` y `float`.
- `statistics` da `mean`, `median`, `mode` y `stdev`; compara la media y la mediana.
- `Counter` cuenta las filas por grupo, `defaultdict(int)` suma una columna por grupo.
- `sorted(..., key=..., reverse=True)` clasifica los resultados; las f-strings `:<12` y
  `:>8.2f` alinean un informe.
- pandas hace todo esto en pocas líneas, para las tablas grandes.

Siguiente: [las expresiones regulares](regex.md)
