# Una base de datos con sqlite3

Esta página muestra cómo guardar, encontrar, modificar y borrar datos en una pequeña base de datos SQL con el módulo `sqlite3`, que viene con Python.

> **Objetivos:**
>
> - sabrás crear una tabla y añadirle filas;
> - sabrás leer filas con `SELECT` y `fetchall`;
> - sabrás modificar y borrar filas;
> - sabrás agrupar cambios en una transacción con `with conn:`;
> - sabrás por qué siempre hay que pasar los valores con `?`.

## ¿Qué es una base de datos?

Una base de datos es un archivador de fichas muy bien ordenado. Cada cajón es una
**tabla**. Cada ficha es una **fila**. Cada casilla de la ficha es una **columna**.

```text
tabla "amigos"
┌────┬─────────┬──────┐
│ id │ nombre  │ edad │   ← las columnas
├────┼─────────┼──────┤
│ 1  │ Alice   │ 30   │   ← una fila
│ 2  │ Bob     │ 25   │   ← otra fila
└────┴─────────┴──────┘
```

Le hablas a la base en un pequeño lenguaje aparte: **SQL**. Una frase SQL se llama una
**consulta**. Se parece al inglés: `SELECT nombre FROM amigos` quiere decir
"elige los nombres en la tabla amigos".

SQLite es una base de datos que cabe en un solo archivo, sin servidor que instalar.
Python ya sabe hablarle gracias al [módulo](../debuter/glossaire.md#modulo) `sqlite3`.
La página [las bases de datos](../debuter/bases-de-donnees.md) explica la idea general.

> **Nota:** en esta página, la base se crea en
> [memoria](../debuter/glossaire.md#memoria-ram) con `":memory:"`. Desaparece al final
> del programa. Para conservarla, da un nombre de archivo:
> `sqlite3.connect("agenda.db")`.

## Abrir una base y crear una tabla

Tres pasos: conectarse, crear la tabla, comprobar.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amigos (id INTEGER PRIMARY KEY, nombre TEXT, edad INTEGER)")
print("Tabla creada")
```

```sortie
Tabla creada
```

Línea por línea:

| Código | Lo que hace |
|---|---|
| `sqlite3.connect(":memory:")` | abre una base y devuelve una **conexión** |
| `conn.execute("...")` | envía una consulta SQL a la base |
| `CREATE TABLE amigos (...)` | crea el cajón `amigos` con tres columnas |
| `id INTEGER PRIMARY KEY` | un número único, que la base da sola |
| `nombre TEXT`, `edad INTEGER` | una columna de texto, una columna de números enteros |

La conexión es el teléfono: mientras está abierta, puedes hablar con la
base.

## Añadir una fila con ?

Para añadir una ficha, se usa `INSERT INTO`. Los valores **no** se escriben en el texto
de la consulta. Se pone un `?` en su lugar, y se dan los valores al lado, en una
[tupla](../debuter/glossaire.md#tupla).

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amigos (id INTEGER PRIMARY KEY, nombre TEXT, edad INTEGER)")

conn.execute("INSERT INTO amigos (nombre, edad) VALUES (?, ?)", ("Alice", 30))
conn.execute("INSERT INTO amigos (nombre, edad) VALUES (?, ?)", ("Bob", 25))

print(conn.execute("SELECT * FROM amigos").fetchall())
```

```sortie
[(1, 'Alice', 30), (2, 'Bob', 25)]
```

El primer `?` recibe `"Alice"`, el segundo recibe `30`. Es como un formulario con
huecos: la consulta es el formulario, la tupla rellena los huecos en orden.

```text
"INSERT INTO amigos (nombre, edad) VALUES (?,      ?)"
                                           ▲       ▲
                                           │       │
                                       ("Alice",   30)
```

No hemos dado ningún `id`: la base puso `1`, luego `2`, ella sola.

> **Recuerda:** nunca pegues un valor en el texto de la consulta. Siempre un `?`, y los
> valores en una tupla al lado. La sección
> [protegerse de las inyecciones SQL](#protegerse-de-las-inyecciones-sql) explica por qué.

> **Pruébalo:** añade una tercera fila para `"Chloé"`, de 41 años. La lista mostrada
> tiene entonces tres tuplas, y Chloé tiene el `id` 3.

### Añadir varias filas de golpe

`executemany` hace la misma consulta para cada elemento de una lista.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amigos (id INTEGER PRIMARY KEY, nombre TEXT, edad INTEGER)")

amigos = [("Alice", 30), ("Bob", 25), ("Chloé", 41)]
conn.executemany("INSERT INTO amigos (nombre, edad) VALUES (?, ?)", amigos)

print(conn.execute("SELECT COUNT(*) FROM amigos").fetchone())
```

```sortie
(3,)
```

`COUNT(*)` cuenta las filas. El resultado es una tupla de **un** valor: `(3,)`.

## Leer filas

`SELECT` elige las filas. Después, hay dos formas de obtenerlas:

- `fetchall()` da **todas** las filas, en una lista;
- `fetchone()` da solo **la siguiente** fila, o `None` si ya no quedan.

Cada fila llega en forma de tupla.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amigos (nombre TEXT, edad INTEGER)")
conn.executemany("INSERT INTO amigos VALUES (?, ?)",
                 [("Alice", 30), ("Bob", 25), ("Chloé", 41)])

filas = conn.execute("SELECT nombre, edad FROM amigos ORDER BY edad").fetchall()
for nombre, edad in filas:
    print(nombre, edad)
```

```sortie
Bob 25
Alice 30
Chloé 41
```

`ORDER BY edad` ordena por edad. Sin `ORDER BY`, la base puede devolver las filas en
cualquier orden: añádelo en cuanto el orden importe.

### Filtrar con WHERE

`WHERE` conserva solo las filas que pasan una prueba. Aquí también, el valor pasa por un
`?`.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amigos (nombre TEXT, edad INTEGER)")
conn.executemany("INSERT INTO amigos VALUES (?, ?)",
                 [("Alice", 30), ("Bob", 25), ("Chloé", 41)])

cursor = conn.execute("SELECT nombre FROM amigos WHERE edad >= ? ORDER BY nombre", (30,))
print(cursor.fetchall())

cursor = conn.execute("SELECT edad FROM amigos WHERE nombre = ?", ("Bob",))
print(cursor.fetchone())
print(cursor.fetchone())
```

```sortie
[('Alice',), ('Chloé',)]
(25,)
None
```

El segundo `fetchone()` devuelve `None`: solo había una fila.

> **Atención:** `(30,)` con una coma es una tupla de un valor. `(30)` sin coma
> es solo el número 30. Mira [los errores frecuentes](#errores-frecuentes).

## Filas con nombres: sqlite3.Row

Con una tupla, hay que recordar que el nombre está en la posición 0 y la edad en la
posición 1. Es frágil. Al configurar `row_factory`, cada fila se convierte en un objeto
que se lee **por el nombre de la columna**, como un
[diccionario](../debuter/glossaire.md#diccionario).

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.row_factory = sqlite3.Row
conn.execute("CREATE TABLE amigos (nombre TEXT, edad INTEGER)")
conn.execute("INSERT INTO amigos VALUES (?, ?)", ("Alice", 30))

fila = conn.execute("SELECT nombre, edad FROM amigos").fetchone()
print(fila["nombre"])
print(fila["edad"])
print(fila.keys())
print(dict(fila))
```

```sortie
Alice
30
['nombre', 'edad']
{'nombre': 'Alice', 'edad': 30}
```

> **Recuerda:** pon `conn.row_factory = sqlite3.Row` justo después de `connect`. Tus
> filas se leen entonces con `fila["nombre"]`, que es mucho más claro que `fila[0]`.

## Modificar y borrar

`UPDATE` cambia filas, `DELETE` las borra. Las dos llevan un `WHERE`.
`rowcount` dice cuántas filas se han tocado.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amigos (nombre TEXT, edad INTEGER)")
conn.executemany("INSERT INTO amigos VALUES (?, ?)",
                 [("Alice", 30), ("Bob", 25), ("Chloé", 41)])

cursor = conn.execute("UPDATE amigos SET edad = ? WHERE nombre = ?", (31, "Alice"))
print("modificadas:", cursor.rowcount)

cursor = conn.execute("DELETE FROM amigos WHERE nombre = ?", ("Bob",))
print("borradas:", cursor.rowcount)

print(conn.execute("SELECT * FROM amigos ORDER BY nombre").fetchall())
```

```sortie
modificadas: 1
borradas: 1
[('Alice', 31), ('Chloé', 41)]
```

Sigamos la tabla en cada paso:

| Paso | Consulta | Contenido de la tabla |
|---|---|---|
| 1 | `INSERT` ×3 | Alice 30, Bob 25, Chloé 41 |
| 2 | `UPDATE … WHERE nombre = 'Alice'` | Alice **31**, Bob 25, Chloé 41 |
| 3 | `DELETE … WHERE nombre = 'Bob'` | Alice 31, Chloé 41 |

> **Atención:** un `UPDATE` o un `DELETE` **sin** `WHERE` toca **todas** las
> filas. `DELETE FROM amigos` vacía el cajón entero. Relee siempre tu `WHERE`.

## Las transacciones con with conn:

Una **transacción** es un paquete de cambios que sale bien **entero** o no sale en
absoluto. Ejemplo: una transferencia. Se quitan 50 € de una cuenta y se añaden a la
otra. Si el programa falla a mitad, el dinero no debe desaparecer.

`with conn:` se encarga de todo:

- si el bloque termina bien, los cambios se **confirman** (`commit`);
- si ocurre una [excepción](../debuter/glossaire.md#excepcion), se **anulan**
  (`rollback`).

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE cuentas (nombre TEXT, saldo INTEGER)")
conn.executemany("INSERT INTO cuentas VALUES (?, ?)", [("Alice", 100), ("Bob", 20)])
conn.commit()

def transferencia(origen, destino, monto):
    with conn:
        conn.execute("UPDATE cuentas SET saldo = saldo - ? WHERE nombre = ?", (monto, origen))
        saldo = conn.execute("SELECT saldo FROM cuentas WHERE nombre = ?", (origen,)).fetchone()[0]
        if saldo < 0:
            raise ValueError("saldo insuficiente")
        conn.execute("UPDATE cuentas SET saldo = saldo + ? WHERE nombre = ?", (monto, destino))

transferencia("Alice", "Bob", 50)
print(conn.execute("SELECT * FROM cuentas ORDER BY nombre").fetchall())

try:
    transferencia("Bob", "Alice", 500)
except ValueError as error:
    print("Rechazada:", error)
print(conn.execute("SELECT * FROM cuentas ORDER BY nombre").fetchall())
```

```sortie
[('Alice', 50), ('Bob', 70)]
Rechazada: saldo insuficiente
[('Alice', 50), ('Bob', 70)]
```

En la segunda transferencia, Bob bajó primero a `-430`. Luego el error anuló todo el
paquete: Bob volvió a `70`.

```text
transferencia("Bob", "Alice", 500)
  BEGIN ─► Bob: 70 → -430 ─► ¿saldo < 0? sí ─► ValueError
                                                   │
                                     ROLLBACK ◄────┘   Bob vuelve a 70
```

> **Recuerda:** sin `commit`, los cambios hechos en un archivo se pierden al
> cerrarlo. Pon tus escrituras en un `with conn:`: confirma o anula por ti.
> El capítulo [los errores](erreurs.md) explica `try` y `except`.

## Protegerse de las inyecciones SQL

Imagina una búsqueda por nombre. Es tentador pegar el nombre en la consulta con una
f-string. Esto es lo que pasa si alguien escribe un nombre un poco especial:

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amigos (nombre TEXT, secreto TEXT)")
conn.executemany("INSERT INTO amigos VALUES (?, ?)",
                 [("Alice", "gato"), ("Bob", "1234")])

entrada = "x' OR '1'='1"
consulta = f"SELECT * FROM amigos WHERE nombre = '{entrada}'"
print(consulta)
print(conn.execute(consulta).fetchall())
```

```sortie
SELECT * FROM amigos WHERE nombre = 'x' OR '1'='1'
[('Alice', 'gato'), ('Bob', '1234')]
```

La entrada **cambió el sentido** de la consulta. `OR '1'='1'` siempre es verdadero: la
base devuelve todas las filas, con todos los secretos. Es una **inyección SQL**, uno de
los ataques más conocidos (mira [la seguridad](../debuter/securite.md)).

Con `?`, el valor sigue siendo un simple valor. Nunca se lee como SQL:

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amigos (nombre TEXT, secreto TEXT)")
conn.executemany("INSERT INTO amigos VALUES (?, ?)",
                 [("Alice", "gato"), ("Bob", "1234")])

entrada = "x' OR '1'='1"
print(conn.execute("SELECT * FROM amigos WHERE nombre = ?", (entrada,)).fetchall())
```

```sortie
[]
```

La base busca a alguien que se llame de verdad `x' OR '1'='1`. Nadie: lista vacía.

> **Recuerda:** el `?` es una caja cerrada. Lo que pones dentro no puede salir
> para convertirse en código SQL.

## Un ejemplo completo: la agenda de contactos

Juntemos todo en pequeñas [funciones](fonctions.md).

```python
import sqlite3

def abrir():
    conn = sqlite3.connect(":memory:")
    conn.row_factory = sqlite3.Row
    conn.execute("""CREATE TABLE contactos (
        id INTEGER PRIMARY KEY,
        nombre TEXT NOT NULL,
        telefono TEXT
    )""")
    return conn

def agregar(conn, nombre, telefono):
    with conn:
        conn.execute("INSERT INTO contactos (nombre, telefono) VALUES (?, ?)", (nombre, telefono))

def buscar(conn, inicio):
    consulta = "SELECT * FROM contactos WHERE nombre LIKE ? ORDER BY nombre"
    return conn.execute(consulta, (inicio + "%",)).fetchall()

def cambiar_telefono(conn, nombre, telefono):
    with conn:
        conn.execute("UPDATE contactos SET telefono = ? WHERE nombre = ?", (telefono, nombre))

def mostrar(conn):
    for c in conn.execute("SELECT * FROM contactos ORDER BY nombre"):
        print(f"{c['id']}. {c['nombre']}: {c['telefono']}")

agenda = abrir()
agregar(agenda, "Alice Martin", "06 11 22 33 44")
agregar(agenda, "Bob Durand", "07 55 66 77 88")
agregar(agenda, "Alain Petit", "06 99 00 11 22")
mostrar(agenda)

print("--- empieza por Al ---")
for c in buscar(agenda, "Al"):
    print(c["nombre"])

cambiar_telefono(agenda, "Bob Durand", "07 00 00 00 00")
print("--- después del cambio ---")
mostrar(agenda)
```

```sortie
3. Alain Petit: 06 99 00 11 22
1. Alice Martin: 06 11 22 33 44
2. Bob Durand: 07 55 66 77 88
--- empieza por Al ---
Alain Petit
Alice Martin
--- después del cambio ---
3. Alain Petit: 06 99 00 11 22
1. Alice Martin: 06 11 22 33 44
2. Bob Durand: 07 00 00 00 00
```

Algunos detalles:

- los números `id` siguen el orden en que se añadieron, pero la visualización sigue el
  orden alfabético gracias a `ORDER BY nombre`;
- `NOT NULL` prohíbe un nombre vacío ([`NULL`](../debuter/glossaire.md#null) es el
  valor "nada" de SQL, como `None` en Python);
- `LIKE ?` con `"Al%"` busca los nombres que **empiezan** por `Al`: `%` quiere decir
  "cualquier secuencia de letras";
- puedes recorrer el resultado de `execute` directamente con `for`, sin `fetchall`.

> **Pruébalo:** añade una función `borrar(conn, nombre)` con un
> `DELETE FROM contactos WHERE nombre = ?` dentro de un `with conn:`. Borra a Bob y
> luego muestra la agenda: quedan dos contactos.

## Errores frecuentes

### Olvidar la coma de una tupla de un valor

```python norun
import sqlite3
conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amigos (nombre TEXT)")
conn.execute("SELECT * FROM amigos WHERE nombre = ?", ("Alice"))
```

```text
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    conn.execute("SELECT * FROM amigos WHERE nombre = ?", ("Alice"))
    ~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
sqlite3.ProgrammingError: Incorrect number of bindings supplied. The current statement uses 1, and there are 5 supplied.
```

`("Alice")` sin coma no es una tupla: es el texto `"Alice"`. Python lo ve como
5 letras, así que 5 valores para un solo `?`. Escribe `("Alice",)` con una coma.

### Leer una tabla que no existe

```python norun
import sqlite3
conn = sqlite3.connect(":memory:")
conn.execute("SELECT * FROM amigos")
```

```text
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    conn.execute("SELECT * FROM amigos")
    ~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^
sqlite3.OperationalError: no such table: amigos
```

La tabla no se ha creado, o su nombre está mal escrito. Una base `":memory:"` está vacía
en cada ejecución: crea la tabla primero. Para un archivo,
`CREATE TABLE IF NOT EXISTS` solo crea la tabla si falta.

### Leer una columna por su nombre sin sqlite3.Row

```python norun
import sqlite3
conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amigos (nombre TEXT, edad INTEGER)")
conn.execute("INSERT INTO amigos VALUES (?, ?)", ("Alice", 30))
fila = conn.execute("SELECT nombre, edad FROM amigos").fetchone()
print(fila["nombre"])
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    print(fila["nombre"])
          ~~~~^^^^^^^^^^
TypeError: tuple indices must be integers or slices, not str
```

Por defecto, una fila es una tupla: se lee con `fila[0]`. Para escribir
`fila["nombre"]`, añade `conn.row_factory = sqlite3.Row` después de `connect`.

## Ponte a prueba

**Pregunta 1:** ¿por qué nunca hay que construir una consulta con una f-string?

> **Respuesta**
>
> Porque un valor escrito por alguien puede contener SQL y cambiar el sentido de la
> consulta: es una inyección SQL. Con `?`, el valor siempre sigue siendo un simple
> valor.

**Pregunta 2:** ¿qué muestra este programa?

```python norun
import sqlite3
conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE notas (valor INTEGER)")
conn.executemany("INSERT INTO notas VALUES (?)", [(12,), (8,), (15,)])
print(conn.execute("SELECT valor FROM notas WHERE valor >= ? ORDER BY valor", (10,)).fetchall())
```

> **Respuesta**
>
> ```python
> import sqlite3
> conn = sqlite3.connect(":memory:")
> conn.execute("CREATE TABLE notas (valor INTEGER)")
> conn.executemany("INSERT INTO notas VALUES (?)", [(12,), (8,), (15,)])
> print(conn.execute("SELECT valor FROM notas WHERE valor >= ? ORDER BY valor", (10,)).fetchall())
> ```
>
> ```sortie
> [(12,), (15,)]
> ```
>
> `WHERE` conserva 12 y 15, `ORDER BY` los ordena. Cada fila es una tupla de un valor.

**Pregunta 3:** en un `with conn:`, ¿qué pasa si ocurre una excepción a
mitad?

> **Respuesta**
>
> Todos los cambios hechos desde el principio del bloque se anulan (`rollback`). La
> base vuelve a estar como antes del bloque, y luego la excepción sigue su camino.

## En resumen

- `sqlite3.connect(":memory:")` abre una base en memoria; un nombre de archivo la guarda
  en el disco.
- `conn.execute(consulta, valores)` envía SQL; `executemany` lo repite para una lista.
- `SELECT` y luego `fetchall()` o `fetchone()` para leer; `UPDATE` y `DELETE` con un
  `WHERE` para modificar y borrar.
- `conn.row_factory = sqlite3.Row` permite escribir `fila["nombre"]`.
- `with conn:` confirma los cambios, o los anula todos si hay un error.
- Siempre `?` para los valores, nunca una f-string: es lo que protege de las
  inyecciones SQL.

Siguiente: [analizar datos](donnees.md)
