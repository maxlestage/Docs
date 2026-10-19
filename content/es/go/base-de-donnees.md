# Una base de datos en Go

Esta página muestra cómo un programa Go habla con una base de datos con el paquete `database/sql`, y luego cómo esconder la base detrás de una interfaz `Store` que puedes probar sin ninguna base.

> **Objetivos:**
>
> - entender el papel de `database/sql` y de un driver;
> - abrir una base, escribir con `Exec`, leer con `QueryRow` y `Query`;
> - usar parámetros `?` en lugar de pegar texto en el SQL;
> - escribir una interfaz `Store` y una versión en memoria (map + cerrojo);
> - ver cómo una versión SQL respeta la misma interfaz.

Si nunca has visto SQL, lee primero
[las bases de datos](../debuter/bases-de-donnees.md) en el recorrido Empezar.

## ¿Por qué una base de datos?

Una variable desaparece cuando el programa se detiene. Un [archivo](fichiers.md) se
queda, pero buscar en él se vuelve lento y complicado cuando crece.

Una **base de datos** es una biblioteca bien ordenada, con un bibliotecario. No rebuscas
tú en las estanterías: pides «los libros de Hugo, ordenados por título», y él te los
trae. El idioma para hablarle se llama **SQL**.

```text
 tu programa ── «SELECT titulo FROM libros» ──►  base de datos
             ◄──────── las filas encontradas ─────
```

> **Recuerda:** la base guarda los datos entre dos ejecuciones y sabe buscar rápido. Tu
> programa le envía órdenes SQL.

## database/sql y los drivers

Go tiene un solo paquete para todas las bases: `database/sql`. Pero no sabe hablar con
**ninguna** base por sí solo. Necesita un **driver** (controlador): un pequeño
[paquete](../debuter/glossaire.md#paquete) que conoce una base concreta.

Es como un adaptador de enchufe de viaje: `database/sql` es tu aparato, el driver es el
adaptador del país.

```text
           tu código
               │   (siempre las mismas funciones: Open, Exec, Query…)
         database/sql
          ┌────┴─────────────┐
   driver SQLite      driver PostgreSQL
          │                  │
    archivo .db      servidor PostgreSQL
```

Los drivers no están en la biblioteca estándar. Se añaden con `go get`
(ver [los módulos](modules.md)):

| Base | Driver habitual | Nombre que dar a `sql.Open` | Parámetros |
|---|---|---|---|
| SQLite (un simple archivo) | `modernc.org/sqlite` | `"sqlite"` | `?` |
| PostgreSQL (un servidor) | `github.com/jackc/pgx/v5/stdlib` | `"pgx"` | `$1`, `$2`… |

El driver se importa con un **guion bajo**: `_ "modernc.org/sqlite"`. El `_` quiere decir
«no uso ningún nombre de este paquete, pero cárgalo». Al cargarse, el driver se
registra en `database/sql`.

### ¿Qué pasa sin driver?

Este programa usa solo la biblioteca estándar. Pide un driver que no se ha importado:

```go
package main

import (
	"database/sql"
	"fmt"
)

func main() {
	db, err := sql.Open("sqlite", "libros.db")
	fmt.Println(db == nil)
	fmt.Println(err)
}
```

```sortie
true
sql: unknown driver "sqlite" (forgotten import?)
```

El mensaje lo dice él mismo: «driver desconocido (¿import olvidado?)».

> **Recuerda:** `database/sql` + un driver importado con `_`. El nombre que se da a
> `sql.Open` depende del driver.

## Abrir la base

Los ejemplos de esta sección necesitan un driver: se muestran sin ejecutarse, con lo que
muestran de verdad.

```go norun
package main

import (
	"database/sql"
	"fmt"
	"log"

	_ "modernc.org/sqlite"
)

func main() {
	db, err := sql.Open("sqlite", "libros.db")
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	if err := db.Ping(); err != nil {
		log.Fatal(err)
	}
	fmt.Println("base abierta")
}
```

```text
base abierta
```

- `sql.Open` **prepara** el acceso, pero todavía no se conecta.
- `db.Ping()` comprueba que la base responde de verdad.
- `db` no es una sola conexión: es una **reserva** (*pool*) de conexiones, segura con
  varias [goroutines](concurrence.md). Se abre una vez, al arrancar, y se conserva.

Para PostgreSQL, solo cambian dos líneas:

```go norun
import _ "github.com/jackc/pgx/v5/stdlib"

db, err := sql.Open("pgx", "postgres://lea:secret@localhost:5432/biblio")
```

## Escribir: Exec

`db.Exec` envía una orden que no devuelve filas: crear una tabla, insertar, modificar,
borrar.

```go norun
_, err = db.Exec(`CREATE TABLE IF NOT EXISTS libros (
	id     INTEGER PRIMARY KEY,
	titulo TEXT NOT NULL,
	autor  TEXT NOT NULL
)`)
if err != nil {
	log.Fatal(err)
}

res, err := db.Exec("INSERT INTO libros (titulo, autor) VALUES (?, ?)",
	"Los miserables", "Victor Hugo")
if err != nil {
	log.Fatal(err)
}
id, _ := res.LastInsertId()
n, _ := res.RowsAffected()
fmt.Println("nuevo id:", id, "- filas añadidas:", n)
```

```text
nuevo id: 1 - filas añadidas: 1
```

Los `?` son **huecos** que se rellenan con los valores que siguen, en orden:

| Hueco | Valor |
|---|---|
| primer `?` | `"Los miserables"` |
| segundo `?` | `"Victor Hugo"` |

Con PostgreSQL, los huecos se escriben `$1`, `$2`, y el id se obtiene con
`… RETURNING id` y `QueryRow`.

### Nunca pegues texto en el SQL

¿Por qué huecos, y no `"… VALUES ('" + titulo + "')"`? Porque un visitante podría
escribir un título como `x'); DROP TABLE libros; --`. Pegado en la orden SQL, ese título
se convierte en una **orden** que borra la tabla. Es la **inyección SQL** (ver
[la seguridad](../debuter/securite.md)).

Con `?`, el driver envía el valor **aparte**: sigue siendo siempre un simple dato,
aunque contenga comillas.

> **Recuerda:** siempre parámetros `?` (o `$1`). Nunca `+` ni `fmt.Sprintf` para
> fabricar una orden SQL con datos.

## Leer una fila: QueryRow

`db.QueryRow` devuelve **una** fila. `Scan` copia cada columna en una variable, en el
orden del `SELECT`. Se da la **dirección** de las variables (`&titulo`) para que `Scan`
pueda rellenarlas (ver [los punteros](pointeurs.md)).

```go norun
var titulo, autor string
err = db.QueryRow("SELECT titulo, autor FROM libros WHERE id = ?", 1).
	Scan(&titulo, &autor)
if errors.Is(err, sql.ErrNoRows) {
	fmt.Println("ningún libro con ese id")
} else if err != nil {
	log.Fatal(err)
} else {
	fmt.Println(titulo, "-", autor)
}
```

```text
Los miserables - Victor Hugo
```

Con el id `99`, que no existe, `Scan` devuelve el error `sql.ErrNoRows`, y el programa
muestra `ningún libro con ese id`. No es una avería: es la respuesta «no se encontró
nada».

```go
package main

import (
	"database/sql"
	"fmt"
)

func main() {
	fmt.Println(sql.ErrNoRows)
}
```

```sortie
sql: no rows in result set
```

## Leer varias filas: Query

`db.Query` devuelve un cursor `rows`. Se avanza fila a fila con `rows.Next()`, como un
dedo que baja por una lista.

```go norun
rows, err := db.Query("SELECT id, titulo FROM libros ORDER BY id")
if err != nil {
	log.Fatal(err)
}
defer rows.Close()

for rows.Next() {
	var id int
	var titulo string
	if err := rows.Scan(&id, &titulo); err != nil {
		log.Fatal(err)
	}
	fmt.Println(id, titulo)
}
if err := rows.Err(); err != nil {
	log.Fatal(err)
}
```

```text
1 Los miserables
2 Germinal
3 El principito
```

| Vuelta | `rows.Next()` | id | titulo |
|---|---|---|---|
| 1 | `true` | 1 | Los miserables |
| 2 | `true` | 2 | Germinal |
| 3 | `true` | 3 | El principito |
| 4 | `false`: no hay más filas | | |

Las tres reglas que no hay que olvidar:

1. `defer rows.Close()` justo después de `Query`: si no, la conexión sigue ocupada;
2. `rows.Scan` con tantas variables como columnas;
3. `rows.Err()` después del bucle: un error ha podido detener la lectura a medias.

> **Recuerda:** `QueryRow(…).Scan(…)` para una fila; `Query` + `for rows.Next()` +
> `defer rows.Close()` para varias.

## Esconder la base detrás de una interfaz

Poner `db.Query` por todo el programa plantea dos problemas:

- para probar, hace falta una base de verdad;
- cambiar de base (SQLite → PostgreSQL) obliga a releerlo todo.

La solución: una [interfaz](interfaces.md) que dice **lo que se quiere hacer** con los
libros, sin decir **cómo**. Se suele llamar `Store` («almacén»).

Es como una consigna de equipajes: entregas una bolsa y te dan un tique; con el tique,
te devuelven la bolsa. No sabes si las bolsas se guardan en un armario o en el sótano.
Y no necesitas saberlo.

```text
             main, el servidor web, los tests…
                          │  solo usan:
          ┌───────────────┴────────────────┐
          │  Store: Add  Get  List  Delete │
          └───────┬────────────────┬───────┘
                  │                │
         MemoriaStore          SQLStore
    (map + cerrojo, tests)  (database/sql, producción)
```

### La interfaz y la versión en memoria

Aquí tienes un programa completo. `MemoriaStore` guarda los libros en un
[map](maps.md), protegido por un cerrojo `sync.RWMutex` (ver [sync](sync.md)): varias
goroutines podrán usarlo a la vez.

```go
package main

import (
	"errors"
	"fmt"
	"sort"
	"sync"
)

type Libro struct {
	ID     int
	Titulo string
	Autor  string
}

var ErrNoEncontrado = errors.New("libro no encontrado")

// Store dice lo que se puede hacer con los libros, no cómo.
type Store interface {
	Add(l Libro) (int, error)
	Get(id int) (Libro, error)
	List() ([]Libro, error)
	Delete(id int) error
}

// MemoriaStore guarda los libros en un map.
type MemoriaStore struct {
	mu        sync.RWMutex
	libros    map[int]Libro
	siguiente int
}

func NewMemoriaStore() *MemoriaStore {
	return &MemoriaStore{libros: map[int]Libro{}, siguiente: 1}
}

func (s *MemoriaStore) Add(l Libro) (int, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	l.ID = s.siguiente
	s.libros[l.ID] = l
	s.siguiente++
	return l.ID, nil
}

func (s *MemoriaStore) Get(id int) (Libro, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	l, ok := s.libros[id]
	if !ok {
		return Libro{}, fmt.Errorf("get %d: %w", id, ErrNoEncontrado)
	}
	return l, nil
}

func (s *MemoriaStore) List() ([]Libro, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	lista := make([]Libro, 0, len(s.libros))
	for _, l := range s.libros {
		lista = append(lista, l)
	}
	// un map no tiene orden: ordenamos por id
	sort.Slice(lista, func(i, j int) bool { return lista[i].ID < lista[j].ID })
	return lista, nil
}

func (s *MemoriaStore) Delete(id int) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if _, ok := s.libros[id]; !ok {
		return fmt.Errorf("delete %d: %w", id, ErrNoEncontrado)
	}
	delete(s.libros, id)
	return nil
}

// mostrar solo conoce la interfaz Store.
func mostrar(s Store) {
	lista, _ := s.List()
	for _, l := range lista {
		fmt.Printf("  %d. %s (%s)\n", l.ID, l.Titulo, l.Autor)
	}
}

func main() {
	var s Store = NewMemoriaStore()

	id1, _ := s.Add(Libro{Titulo: "Los miserables", Autor: "Victor Hugo"})
	id2, _ := s.Add(Libro{Titulo: "Germinal", Autor: "Émile Zola"})
	s.Add(Libro{Titulo: "El principito", Autor: "Saint-Exupéry"})
	fmt.Println("añadidos:", id1, id2)

	l, err := s.Get(2)
	fmt.Println("get 2:", l.Titulo, err)

	fmt.Println("lista:")
	mostrar(s)

	fmt.Println("delete 1:", s.Delete(1))
	fmt.Println("delete 1:", s.Delete(1))

	_, err = s.Get(1)
	fmt.Println("get 1:", err)
	fmt.Println("¿no encontrado?", errors.Is(err, ErrNoEncontrado))

	fmt.Println("lista:")
	mostrar(s)
}
```

```sortie
añadidos: 1 2
get 2: Germinal <nil>
lista:
  1. Los miserables (Victor Hugo)
  2. Germinal (Émile Zola)
  3. El principito (Saint-Exupéry)
delete 1: <nil>
delete 1: delete 1: libro no encontrado
get 1: get 1: libro no encontrado
¿no encontrado? true
lista:
  2. Germinal (Émile Zola)
  3. El principito (Saint-Exupéry)
```

### Paso a paso

Sigamos el contenido del almacén en cada llamada:

| Paso | Llamada | siguiente después | Libros en el map | Devuelve |
|---|---|---|---|---|
| 0 | `NewMemoriaStore()` | 1 | (vacío) | |
| 1 | `Add(Miserables)` | 2 | 1 | `1, nil` |
| 2 | `Add(Germinal)` | 3 | 1, 2 | `2, nil` |
| 3 | `Add(Principito)` | 4 | 1, 2, 3 | `3, nil` |
| 4 | `Get(2)` | 4 | 1, 2, 3 | Germinal, `nil` |
| 5 | `Delete(1)` | 4 | 2, 3 | `nil` |
| 6 | `Delete(1)` | 4 | 2, 3 | error: no encontrado |
| 7 | `Get(1)` | 4 | 2, 3 | error: no encontrado |

Tres detalles en los que fijarse:

- `siguiente` nunca baja: un id borrado **no** se reutiliza. Una base SQL hace lo
  mismo.
- `List` **ordena** la lista. Un map de Go devuelve sus claves en un orden que cambia
  cada vez; sin ordenar, lo que se muestra cambiaría de una ejecución a otra.
- Los errores envuelven `ErrNoEncontrado` con `%w`. Quien llama comprueba
  `errors.Is(err, ErrNoEncontrado)` sin mirar el texto (ver [los errores](erreurs.md)).

Prueba: añade un cuarto libro al final de `main`, y luego un `mostrar(s)`. Recibe el id
`4`, no `1`: el id 1 no se reutiliza.

> **Recuerda:** el resto del programa solo conoce `Store`. La versión en memoria basta
> para escribir y probar toda la lógica, sin ninguna base.

### Comprobar que un tipo respeta la interfaz

Una línea puesta a nivel del paquete pide al compilador que compruebe que
`*MemoriaStore` tiene de verdad los cuatro métodos:

```go norun
var _ Store = (*MemoriaStore)(nil)
```

Si falta uno, la [compilación](../debuter/glossaire.md#compilacion) falla enseguida,
con el nombre del método que falta.

## La misma interfaz, versión SQL

Una versión SQL es otro tipo, `SQLStore`, que contiene un `*sql.DB`. Sus métodos tienen
**exactamente** las mismas firmas. Traducen cada llamada en una orden SQL.

```go norun
type SQLStore struct {
	db *sql.DB
}

var _ Store = (*SQLStore)(nil)

func (s *SQLStore) Add(l Libro) (int, error) {
	res, err := s.db.Exec("INSERT INTO libros (titulo, autor) VALUES (?, ?)",
		l.Titulo, l.Autor)
	if err != nil {
		return 0, err
	}
	id, err := res.LastInsertId()
	return int(id), err
}

func (s *SQLStore) Get(id int) (Libro, error) {
	l := Libro{ID: id}
	err := s.db.QueryRow("SELECT titulo, autor FROM libros WHERE id = ?", id).
		Scan(&l.Titulo, &l.Autor)
	if errors.Is(err, sql.ErrNoRows) {
		return Libro{}, fmt.Errorf("get %d: %w", id, ErrNoEncontrado)
	}
	return l, err
}

func (s *SQLStore) List() ([]Libro, error) {
	rows, err := s.db.Query("SELECT id, titulo, autor FROM libros ORDER BY id")
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var lista []Libro
	for rows.Next() {
		var l Libro
		if err := rows.Scan(&l.ID, &l.Titulo, &l.Autor); err != nil {
			return nil, err
		}
		lista = append(lista, l)
	}
	return lista, rows.Err()
}

func (s *SQLStore) Delete(id int) error {
	res, err := s.db.Exec("DELETE FROM libros WHERE id = ?", id)
	if err != nil {
		return err
	}
	if n, _ := res.RowsAffected(); n == 0 {
		return fmt.Errorf("delete %d: %w", id, ErrNoEncontrado)
	}
	return nil
}
```

Compara método por método:

| Método | MemoriaStore | SQLStore |
|---|---|---|
| `Add` | `s.libros[id] = l`, `siguiente++` | `INSERT`, luego `LastInsertId` |
| `Get` | `s.libros[id]` y `ok` | `SELECT … WHERE id = ?`, `ErrNoRows` |
| `List` | recorrer el map, luego ordenar | `SELECT … ORDER BY id` |
| `Delete` | `delete(s.libros, id)` | `DELETE`, luego `RowsAffected` |
| cerrojo | `sync.RWMutex` | innecesario: `*sql.DB` ya es seguro |

En `main`, solo cambia una línea:

```go norun
var s Store = NewMemoriaStore()        // para probar, para los tests
var s Store = &SQLStore{db: db}        // de verdad, con la base
```

Con SQLite, el mismo `main` de antes muestra **lo mismo**: es toda la gracia de la
interfaz. Y las dos versiones devuelven el mismo error `ErrNoEncontrado`, así que el
código que lo usa no ve ninguna diferencia.

> **Recuerda:** una interfaz `Store`, dos implementaciones. La memoria para los
> [tests](tests.md), SQL para producción. El resto del programa no cambia.

## Errores frecuentes

### Olvidar importar el driver

```go norun
import "database/sql"

db, err := sql.Open("sqlite", "libros.db")
if err != nil {
	fmt.Println(err)
	return
}
```

```text
sql: unknown driver "sqlite" (forgotten import?)
```

Añade la línea `_ "modernc.org/sqlite"` a los imports, y haz `go get` del módulo.

### Un Scan con un número de variables equivocado

```go norun
rows, _ := db.Query("SELECT id, titulo, autor FROM libros")
for rows.Next() {
	var id int
	var titulo string
	err := rows.Scan(&id, &titulo) // 3 columnas, 2 variables
	fmt.Println(err)
}
```

```text
sql: expected 3 destination arguments in Scan, not 2
```

Hace falta una variable por columna del `SELECT`, en el mismo orden.

### Un map compartido sin cerrojo

Si quitas el `sync.RWMutex` de `MemoriaStore` y varias goroutines llaman a `Add` a la
vez, Go detiene el programa:

```text
fatal error: concurrent map writes

goroutine 7 [running]:
main.(*MemoriaStore).Add(…)
…
exit status 2
```

Un map de Go no está hecho para que varias goroutines lo modifiquen a la vez. Conserva
el cerrojo (`Lock` para escribir, `RLock` para leer).

## Ponte a prueba

**Pregunta 1:** ¿para qué sirve el `_` en `import _ "modernc.org/sqlite"`?

> **Respuesta**
>
> Para cargar el paquete sin usar sus nombres. Al cargarse, el driver se registra en
> `database/sql` con el nombre `"sqlite"`.

**Pregunta 2:** ¿por qué escribir `WHERE id = ?` con un parámetro en lugar de pegar el
id en el texto de la consulta?

> **Respuesta**
>
> Para evitar la inyección SQL: el valor se envía aparte y sigue siendo un dato, nunca
> puede convertirse en una orden SQL.

**Pregunta 3:** en el ejemplo `MemoriaStore`, ¿por qué `List` ordena los libros?

> **Respuesta**
>
> Porque un map de Go devuelve sus elementos en un orden imprevisible. Ordenar por id da
> siempre la misma lista, como lo haría `ORDER BY id` en SQL.

## En resumen

- `database/sql` da las mismas funciones para todas las bases; un driver, importado con
  `_`, hace el enlace con una base concreta.
- `Exec` para escribir, `QueryRow(…).Scan(…)` para una fila, `Query` + `rows.Next()` +
  `defer rows.Close()` para varias.
- Siempre parámetros `?` o `$1`, nunca texto pegado.
- Una interfaz `Store` esconde la base: una versión en memoria (map + `RWMutex`) para
  probar y hacer tests, una versión SQL para producción.
- `var _ Store = (*SQLStore)(nil)` comprueba al compilar que un tipo respeta la
  interfaz.

Siguiente: [las expresiones regulares](regex.md).
