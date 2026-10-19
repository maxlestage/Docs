# ¿Qué es una base de datos?

Una base de datos guarda la información en tablas para encontrarla rápido y sin errores: esta página te muestra las tablas, las filas, las columnas y las cuatro órdenes SQL básicas, con Python.

> **Objetivos:**
>
> - entender por qué un simple archivo de texto no siempre basta;
> - leer una tabla: sus filas, sus columnas y su clave;
> - escribir las órdenes SQL `SELECT`, `INSERT`, `UPDATE` y `DELETE`;
> - filtrar con `WHERE` y ordenar con `ORDER BY`;
> - unir dos tablas con `JOIN`.

## ¿Por qué no un simple archivo de texto?

Imagina que apuntas tus libros en un archivo de texto: una línea por
libro, con un `;` entre el título, el autor y el año. Funciona… hasta el
día en que un título contiene él mismo un `;`. Mira:

```python
lineas = [
    "El principito;Saint-Exupéry;1943",
    "París; modo de empleo;Dupont;2001",
]
for linea in lineas:
    trozos = linea.split(";")
    print(len(trozos), "trozos:", trozos)
```

```sortie
3 trozos: ['El principito', 'Saint-Exupéry', '1943']
4 trozos: ['París', ' modo de empleo', 'Dupont', '2001']
```

La segunda línea se corta en 4 trozos en lugar de 3: el programa cree que
el autor se llama «modo de empleo». Otros problemas: para encontrar un
libro, hay que leer **todo** el archivo, y nada impide escribir
«mil novecientos» en lugar de un año.

Una **base de datos** resuelve estos problemas: este programa especializado
guarda tus datos, los comprueba y los encuentra muy rápido.

> **Recuerda:** un archivo de texto es un cuaderno; una base de datos, un archivador llevado por un bibliotecario.

## Una tabla es como una hoja de cálculo

Una base de datos contiene **tablas**. Una tabla se parece a una hoja de
cálculo (como Excel o LibreOffice Calc):

```text
Tabla «libros»
┌────┬─────────────────────────┬───────────────┬──────┐
│ id │ titulo                  │ autor         │ anio │  ← las columnas
├────┼─────────────────────────┼───────────────┼──────┤
│ 1  │ El principito           │ Saint-Exupéry │ 1943 │  ← una fila
│ 2  │ Los miserables          │ Hugo          │ 1862 │
│ 3  │ Nuestra Señora de París │ Hugo          │ 1831 │
│ 4  │ El extranjero           │ Camus         │ 1942 │
└────┴─────────────────────────┴───────────────┴──────┘
```

- una **columna** es un tipo de información (el título, el año…). Cada
  columna tiene un [tipo](glossaire.md#tipo): texto, un número entero…;
- una **fila** es una cosa guardada: aquí, un libro;
- la **clave** (aquí `id`) es un número único por fila, como un número de
  la seguridad social: dos libros pueden tener el mismo título, nunca el
  mismo `id`.

> **Recuerda:** una tabla = columnas y filas; la clave señala una fila sin error.

## SQL: el idioma de las bases de datos

Se habla con una base de datos en un lenguaje: **SQL** (se dice «ese-cu-ele»
o «sequel»). Sus frases se parecen a un inglés sencillo:

| Orden SQL | En español           | Ejemplo                                  |
|-----------|----------------------|------------------------------------------|
| `SELECT`  | «muéstrame»          | `SELECT titulo FROM libros`              |
| `INSERT`  | «añade»              | `INSERT INTO libros (titulo) VALUES (…)` |
| `UPDATE`  | «modifica»           | `UPDATE libros SET anio = 1943`          |
| `DELETE`  | «borra»              | `DELETE FROM libros`                     |
| `WHERE`   | «solo si…»           | `WHERE autor = 'Hugo'`                   |
| `ORDER BY`| «ordenado por…»      | `ORDER BY anio`                          |

Para probar, usamos **SQLite**, que viene con Python: no hay nada que
instalar. Con `":memory:"`, la base vive en la [memoria RAM](glossaire.md#memoria-ram)
y desaparece al terminar el programa: perfecto para practicar.

## Crear una tabla y añadir filas

Tres pasos: conectarse, crear la tabla, añadir filas:

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE libros (id INTEGER PRIMARY KEY, titulo TEXT, autor TEXT, anio INTEGER)")
con.execute("INSERT INTO libros (titulo, autor, anio) VALUES (?, ?, ?)",
            ("El principito", "Saint-Exupéry", 1943))

for fila in con.execute("SELECT * FROM libros"):
    print(fila)
```

```sortie
(1, 'El principito', 'Saint-Exupéry', 1943)
```

Leámoslo línea por línea:

| Línea del programa        | Qué pasa                                                 |
|---------------------------|----------------------------------------------------------|
| `sqlite3.connect(…)`      | abre una base vacía                                      |
| `CREATE TABLE libros (…)` | crea la tabla; `id` es la clave, se numera sola          |
| `INSERT INTO … VALUES`    | añade una fila                                           |
| `SELECT * FROM libros`    | «muéstrame todas las columnas de la tabla libros»        |

Los `?` son **marcadores de posición**. Python mete ahí los valores, en
orden. Es la buena costumbre: la página sobre
[la seguridad](securite.md) explica por qué.

Prueba: copia el `INSERT` con `("Los miserables", "Hugo", 1862)`.
Aparece una segunda fila, con el `id` 2.

## Elegir filas con WHERE

`WHERE` solo conserva algunas filas. Aquí, `executemany` añade todos los
libros de una vez:

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE libros (id INTEGER PRIMARY KEY, titulo TEXT, autor TEXT, anio INTEGER)")
con.executemany("INSERT INTO libros (titulo, autor, anio) VALUES (?, ?, ?)", [
    ("El principito", "Saint-Exupéry", 1943), ("Los miserables", "Hugo", 1862),
    ("Nuestra Señora de París", "Hugo", 1831), ("El extranjero", "Camus", 1942),
])

for fila in con.execute("SELECT titulo, anio FROM libros WHERE autor = 'Hugo'"):
    print(fila)
```

```sortie
('Los miserables', 1862)
('Nuestra Señora de París', 1831)
```

La base mira cada fila y se pregunta «¿`autor` vale `'Hugo'`?»:

| id | autor         | ¿`autor = 'Hugo'`? | ¿Se queda? |
|----|---------------|--------------------|------------|
| 1  | Saint-Exupéry | no                 | no         |
| 2  | Hugo          | sí                 | **sí**     |
| 3  | Hugo          | sí                 | **sí**     |
| 4  | Camus         | no                 | no         |

En SQL, el texto se escribe entre comillas simples `'…'`, y la igualdad es
un solo `=` (no `==` como en Python).

Prueba: cambia la condición por `WHERE anio > 1900`. Obtienes
*El principito* y *El extranjero*.

> **Recuerda:** `SELECT` elige las columnas, `WHERE` elige las filas.

## Ordenar con ORDER BY

`ORDER BY` ordena el resultado. Añade `DESC` para el orden inverso (de
mayor a menor):

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE libros (id INTEGER PRIMARY KEY, titulo TEXT, anio INTEGER)")
con.executemany("INSERT INTO libros (titulo, anio) VALUES (?, ?)", [
    ("El principito", 1943), ("Los miserables", 1862), ("El extranjero", 1942),
])

for titulo, anio in con.execute("SELECT titulo, anio FROM libros ORDER BY anio"):
    print(anio, titulo)
```

```sortie
1862 Los miserables
1942 El extranjero
1943 El principito
```

Cada fila es una [tupla](glossaire.md#tupla), separada en dos variables
por `for titulo, anio in …`.

Prueba: escribe `ORDER BY anio DESC`. El orden se invierte: 1943 primero.

## Modificar y borrar: UPDATE y DELETE

`UPDATE` cambia valores, `DELETE` quita filas. Los dos se usan
**siempre** con un `WHERE`, para apuntar a las filas correctas:

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE libros (id INTEGER PRIMARY KEY, titulo TEXT, anio INTEGER)")
con.executemany("INSERT INTO libros (titulo, anio) VALUES (?, ?)", [
    ("El principito", 1900), ("Los miserables", 1862), ("Libro repetido", 2000),
])

con.execute("UPDATE libros SET anio = ? WHERE id = ?", (1943, 1))
con.execute("DELETE FROM libros WHERE id = 3")

for fila in con.execute("SELECT * FROM libros"):
    print(fila)
```

```sortie
(1, 'El principito', 1943)
(2, 'Los miserables', 1862)
```

| id | Antes                  | Orden    | Después                |
|----|------------------------|----------|------------------------|
| 1  | El principito, 1900    | `UPDATE` | El principito, 1943    |
| 2  | Los miserables, 1862   | (nada)   | Los miserables, 1862   |
| 3  | Libro repetido, 2000   | `DELETE` | (desaparecida)         |

> **Recuerda:** `UPDATE` y `DELETE` sin `WHERE` afectan a **todas** las filas.

## Unir dos tablas con JOIN

Escribir «Hugo» en cada libro repite la misma información. Así que se
guardan los autores en **su propia tabla**, y cada libro solo conserva el
**número** de su autor:

```text
   Tabla «autores»                Tabla «libros»
┌────┬───────────────┐     ┌────┬─────────────────────────┬──────────┐
│ id │ nombre        │     │ id │ titulo                  │ autor_id │
├────┼───────────────┤     ├────┼─────────────────────────┼──────────┤
│ 1  │ Saint-Exupéry │◄────┤ 1  │ El principito           │ 1        │
│ 2  │ Hugo          │◄─┬──┤ 2  │ Los miserables          │ 2        │
└────┴───────────────┘  └──┤ 3  │ Nuestra Señora de París │ 2        │
                           └────┴─────────────────────────┴──────────┘
```

Un `JOIN` («unir») vuelve a pegar las dos tablas: para cada libro, la base
busca el autor cuyo `id` vale `autor_id`.

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE autores (id INTEGER PRIMARY KEY, nombre TEXT)")
con.execute("CREATE TABLE libros (id INTEGER PRIMARY KEY, titulo TEXT, autor_id INTEGER)")
con.executemany("INSERT INTO autores (nombre) VALUES (?)", [("Saint-Exupéry",), ("Hugo",)])
con.executemany("INSERT INTO libros (titulo, autor_id) VALUES (?, ?)", [
    ("El principito", 1), ("Los miserables", 2), ("Nuestra Señora de París", 2),
])

consulta = """SELECT libros.titulo, autores.nombre FROM libros
              JOIN autores ON libros.autor_id = autores.id
              ORDER BY libros.titulo"""
for titulo, nombre in con.execute(consulta):
    print(f"{titulo} — {nombre}")
```

```sortie
El principito — Saint-Exupéry
Los miserables — Hugo
Nuestra Señora de París — Hugo
```

`libros.titulo` quiere decir «la columna `titulo` de la tabla `libros`».

Prueba: añade `WHERE autores.nombre = 'Hugo'` justo antes de `ORDER BY`.
Solo quedan los dos libros de Hugo.

> **Recuerda:** cada información se escribe **una sola vez**; `JOIN` las reúne.

## SQL, NoSQL y las herramientas

Las bases **SQL** (llamadas «relacionales») lo guardan todo en tablas,
como aquí. Las bases **NoSQL** guardan los datos de otra forma, a menudo
como documentos [JSON](glossaire.md#json): más flexibles, pero menos
estrictas.

Para empezar, SQLite basta: ya está en Python. Más adelante, quizá
instales **DB Browser for SQLite** (para ver tus tablas en pantalla), y
luego **PostgreSQL** o **MySQL**, bases grandes que funcionan en un
[servidor](glossaire.md#servidor) para los sitios web. Para ir más lejos:
[SQLite en Python](../python/sqlite.md) y
[las bases de datos en Ruby](../ruby/base-de-donnees.md).

## Errores frecuentes

### Olvidar el WHERE en un UPDATE

Quieres corregir el año de un solo libro. Pero olvidas el `WHERE`:

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE libros (id INTEGER PRIMARY KEY, titulo TEXT, anio INTEGER)")
con.executemany("INSERT INTO libros (titulo, anio) VALUES (?, ?)", [
    ("El principito", 1900), ("Los miserables", 1862), ("El extranjero", 1942),
])

con.execute("UPDATE libros SET anio = 1943")  # ups: no hay WHERE

for fila in con.execute("SELECT * FROM libros"):
    print(fila)
```

```sortie
(1, 'El principito', 1943)
(2, 'Los miserables', 1943)
(3, 'El extranjero', 1943)
```

Ningún mensaje de error: SQL hizo exactamente lo que le dijiste. Los
**tres** libros tienen ahora 1943. La corrección:
`UPDATE libros SET anio = 1943 WHERE id = 1`. Consejo: lanza primero un
`SELECT` con el mismo `WHERE` para ver qué filas se van a tocar.

### Una errata en una orden SQL

```python norun
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("SELEC * FROM libros")
```

```text
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    con.execute("SELEC * FROM libros")
    ~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
sqlite3.OperationalError: near "SELEC": syntax error
```

`near "SELEC"` quiere decir «cerca de la palabra `SELEC`»: ahí es donde la
base deja de entender. Corrígelo a `SELECT`.

### Fabricar la orden SQL pegando texto

Es tentador pegar el texto del usuario en la orden SQL con `+`. Con un
nombre que contiene un apóstrofo, todo se rompe:

```python norun
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE alumnos (id INTEGER PRIMARY KEY, nombre TEXT)")
nombre = "Luca D'Angelo"
con.execute("INSERT INTO alumnos (nombre) VALUES ('" + nombre + "')")
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    con.execute("INSERT INTO alumnos (nombre) VALUES ('" + nombre + "')")
    ~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
sqlite3.OperationalError: near "Angelo": syntax error
```

El apóstrofo de «D'Angelo» cierra el texto demasiado pronto. Peor aún: un
pirata puede escribir SQL en lugar de un nombre (la **inyección SQL**, ver
[la seguridad](securite.md)). La corrección, siempre la misma, son los `?`:
`con.execute("INSERT INTO alumnos (nombre) VALUES (?)", (nombre,))`.

## Ponte a prueba

**1.** En una tabla, ¿para qué sirve la clave (`id`)?

> **Respuesta**
>
> Para señalar una fila sin error: cada fila tiene su propia clave.

**2.** ¿Qué hace `DELETE FROM libros` sin `WHERE`?

> **Respuesta**
>
> Borra **todas** las filas de la tabla. Hace falta un `WHERE` para
> apuntar solo a algunas filas, por ejemplo `WHERE id = 3`.

**3.** ¿Qué orden SQL muestra los títulos publicados después de 1900, del más reciente al más antiguo?

> **Respuesta**
>
> `SELECT titulo FROM libros WHERE anio > 1900 ORDER BY anio DESC`

## En resumen

- Una base de datos guarda la información en tablas: columnas, filas, una clave.
- SQL habla con la base: `SELECT` lee, `INSERT` añade, `UPDATE` modifica, `DELETE` borra.
- `WHERE` elige las filas, `ORDER BY` las ordena; sin `WHERE`, se toca todo.
- `JOIN` une dos tablas gracias a un número común.
- Pasa siempre los valores con `?`, nunca pegando texto.

Siguiente: [las bases de la seguridad](securite.md).
