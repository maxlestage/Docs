# Una base de datos en Rust

Una base de datos guarda tu información cuando el programa se detiene: en Rust, le hablas con el crate `rusqlite`, y escondes este detalle detrás de un trait para que el resto del programa no dependa de él.

> **Objetivos:**
>
> - entender para qué sirve una base de datos;
> - crear una tabla, añadir y leer filas con `rusqlite`;
> - escribir un trait `Repositorio` con métodos que devuelven un `Result`;
> - hacerlo funcionar con un simple `HashMap`, y luego con SQLite;
> - probar todo el programa sin base de datos.

## ¿Por qué una base de datos?

Tus variables viven en la [memoria RAM](../debuter/glossaire.md#memoire-vive).
Cuando el programa se detiene, desaparecen. Para guardar información, hay que
escribirla en el disco.

Podrías escribir un archivo de texto (ver [JSON, CSV y TOML](formats.md)). Pero para
encontrar **un** libro entre 10 000, habría que volver a leer todo el archivo. Una
**base de datos** es un archivo ordenado como un archivador de oficina: cajones con
etiquetas, fichas numeradas y un empleado que encuentra la ficha correcta al instante.

```text
  Programa Rust                  Disco
 ┌──────────────┐  petición  ┌────────────────────────┐
 │ let libros = │ ─────────► │ biblioteca.db          │
 │   ...        │            │  tabla «libros»        │
 │              │ ◄───────── │  id | titulo | paginas │
 └──────────────┘  respuesta └────────────────────────┘
  (se borra al final)          (queda tras el final)
```

Una **tabla** se parece a una hoja de cálculo: columnas (`id`, `titulo`, `paginas`) y
una fila por elemento. Se le habla en un pequeño lenguaje, el **SQL** (ver
[Las bases de datos](../debuter/bases-de-donnees.md)).

| id | titulo | paginas |
|----|--------|---------|
| 1 | Dune | 412 |
| 2 | El principito | 96 |

> **Recuerda:** la memoria se borra cuando el programa se detiene; una base de datos
> guarda la información en el disco y encuentra rápido la que pides.

## SQLite y el crate rusqlite

**SQLite** es la base de datos más sencilla: no hay servidor que instalar, toda la
base cabe en **un solo archivo**. El [crate](../debuter/glossaire.md#crate) `rusqlite`
permite usarla desde Rust:

```sh
cargo add rusqlite --features bundled
```

Tu `Cargo.toml` contiene entonces:

```toml
[dependencies]
rusqlite = { version = "0.40", features = ["bundled"] }
```

> **Nota:** `bundled` mete SQLite dentro de tu programa. No tienes que instalar nada
> más en el ordenador.

Los ejemplos con `rusqlite` no pueden ejecutarse en esta página (hace falta Cargo). Su
salida se obtuvo con `cargo run`.

## Crear una tabla, añadir y leer

Aquí tienes un programa completo. Abre (o crea) el archivo, crea la tabla, añade dos
libros y los vuelve a leer:

```rust norun
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Libro {
    id: i64,
    titulo: String,
    paginas: u32,
}

fn main() -> Result<()> {
    let conn = Connection::open("biblioteca.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS libros (
            id       INTEGER PRIMARY KEY,
            titulo   TEXT NOT NULL,
            paginas  INTEGER NOT NULL
        )",
        [],
    )?;
    conn.execute("DELETE FROM libros", [])?;

    conn.execute("INSERT INTO libros (titulo, paginas) VALUES (?1, ?2)", params!["Dune", 412])?;
    conn.execute("INSERT INTO libros (titulo, paginas) VALUES (?1, ?2)", params!["El principito", 96])?;
    println!("Último id: {}", conn.last_insert_rowid());

    let mut consulta = conn.prepare("SELECT id, titulo, paginas FROM libros ORDER BY id")?;
    let libros = consulta.query_map([], |fila| {
        Ok(Libro { id: fila.get(0)?, titulo: fila.get(1)?, paginas: fila.get(2)? })
    })?;
    for libro in libros {
        println!("{:?}", libro?);
    }
    Ok(())
}
```

```text
Último id: 2
Libro { id: 1, titulo: "Dune", paginas: 412 }
Libro { id: 2, titulo: "El principito", paginas: 96 }
```

Paso a paso:

| Paso | Línea | Lo que pasa | Tabla `libros` |
|------|-------|-------------|----------------|
| 1 | `Connection::open` | abre el archivo (lo crea si no existe) | todavía no existe |
| 2 | `CREATE TABLE IF NOT EXISTS` | crea la tabla, solo la primera vez | vacía |
| 3 | `DELETE FROM libros` | vacía la tabla (para este ejemplo) | vacía |
| 4 | 1.er `INSERT` | SQLite elige el id 1 | (1, Dune, 412) |
| 5 | 2.º `INSERT` | SQLite elige el id 2 | + (2, El principito, 96) |
| 6 | `last_insert_rowid` | devuelve el id de lo último añadido | 2 |
| 7 | `query_map` | lee cada fila y fabrica un `Libro` | sin cambios |

- `?1`, `?2` son **huecos** que rellena `params![…]`, en orden.
- `fila.get(0)?` lee la columna n.º 0 de la fila (se cuenta desde 0).
- Cada llamada devuelve un [`Result`](../debuter/glossaire.md#result): `?` detiene la
  función en el primer error (ver [Gestionar los errores](erreurs.md)).

> **Atención:** nunca escribas `format!("… VALUES ('{titulo}')")` para poner un valor
> en una consulta. Un título como `L'Étranger` rompe la consulta, y un usuario
> malintencionado puede colar sus propias órdenes SQL (una «inyección SQL»). Los huecos
> `?1` siempre son seguros.

Pruébalo: quita la línea `DELETE FROM libros` y ejecuta dos veces. La segunda vez,
verás cuatro libros y `Último id: 4`: el archivo guardó los de la primera vez.

> **Recuerda:** `execute` para modificar, `prepare` + `query_map` para leer, y huecos
> `?1` para los valores.

## El problema: SQL por todas partes

Imagina ahora un programa más grande. Si cada función llama directamente a
`conn.execute`, el SQL queda esparcido por todas partes:

- para probar una función, hace falta una base de verdad;
- para cambiar de base (PostgreSQL en lugar de SQLite), hay que releerlo todo.

La solución: una **ventanilla**. En la ventanilla de una biblioteca, dices «quisiera el
libro n.º 2». No sabes si el empleado va a buscarlo en un cajón, en un sótano o en un
almacén. Solo sabes lo que puedes pedir.

En Rust, esta ventanilla es un [trait](../debuter/glossaire.md#trait) (ver
[Traits y objetos](traits-objets.md)). Se suele llamar `Repositorio` (*repository* en
inglés).

```text
            el resto del programa
                    │
                    ▼
        ┌───────────────────────┐
        │   trait Repositorio   │   agregar, buscar, todos, eliminar
        └───────────────────────┘
             ▲               ▲
             │               │
 ┌───────────────────┐ ┌───────────────────┐
 │ RepositorioMemoria│ │ RepositorioSqlite │
 │     (HashMap)     │ │    (rusqlite)     │
 └───────────────────┘ └───────────────────┘
     para probar           de verdad
     y hacer tests
```

> **Recuerda:** el resto del programa solo le habla al trait. Detrás, se puede conectar
> un `HashMap` o una base de verdad, sin cambiar nada más.

## El trait Repositorio

La ventanilla ofrece cuatro peticiones. Cada una puede fallar, así que cada una devuelve
un `Result`:

```rust
#[derive(Debug, Clone, PartialEq)]
struct Libro {
    id: u32,
    titulo: String,
    paginas: u32,
}

#[derive(Debug, PartialEq)]
enum ErrorRepositorio {
    NoEncontrado(u32),
}

trait Repositorio {
    fn agregar(&mut self, titulo: &str, paginas: u32) -> Result<u32, ErrorRepositorio>;
    fn buscar(&self, id: u32) -> Result<Libro, ErrorRepositorio>;
    fn todos(&self) -> Result<Vec<Libro>, ErrorRepositorio>;
    fn eliminar(&mut self, id: u32) -> Result<(), ErrorRepositorio>;
}
```

Este bloque no hace nada: solo **describe**. Leamos cada línea:

| Método | Recibe | Devuelve si todo va bien | Puede fallar si… |
|--------|--------|--------------------------|------------------|
| `agregar` | un título, un número de páginas | el nuevo id (`u32`) | la base tiene un problema |
| `buscar` | un id | una copia del `Libro` | ningún libro tiene ese id |
| `todos` | nada | la lista de libros | la base tiene un problema |
| `eliminar` | un id | nada (`()`) | ningún libro tiene ese id |

- `&mut self`: el método **modifica** el repositorio (agregar, eliminar).
- `&self`: el método solo **mira** (buscar, todos).
- `ErrorRepositorio` es nuestro propio tipo de error (ver
  [Tu propio tipo de error](erreurs.md#tu-propio-tipo-de-error)).

> **Recuerda:** un trait es una lista de promesas. Cada tipo que lo implementa debe
> cumplir las cuatro.

## Un repositorio en memoria con un HashMap

Primera implementación: un [`HashMap`](hashmap.md) que guarda cada libro bajo su id, y
un contador `siguiente_id` que reparte los números, como el ticket de una cola de
espera.

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct Libro {
    id: u32,
    titulo: String,
    paginas: u32,
}

#[derive(Debug, PartialEq)]
enum ErrorRepositorio {
    NoEncontrado(u32),
}

trait Repositorio {
    fn agregar(&mut self, titulo: &str, paginas: u32) -> Result<u32, ErrorRepositorio>;
    fn buscar(&self, id: u32) -> Result<Libro, ErrorRepositorio>;
    fn todos(&self) -> Result<Vec<Libro>, ErrorRepositorio>;
    fn eliminar(&mut self, id: u32) -> Result<(), ErrorRepositorio>;
}

struct RepositorioMemoria {
    libros: HashMap<u32, Libro>,
    siguiente_id: u32,
}

impl RepositorioMemoria {
    fn new() -> RepositorioMemoria {
        RepositorioMemoria { libros: HashMap::new(), siguiente_id: 1 }
    }
}

impl Repositorio for RepositorioMemoria {
    fn agregar(&mut self, titulo: &str, paginas: u32) -> Result<u32, ErrorRepositorio> {
        let id = self.siguiente_id;
        self.siguiente_id += 1;
        self.libros.insert(id, Libro { id, titulo: titulo.to_string(), paginas });
        Ok(id)
    }

    fn buscar(&self, id: u32) -> Result<Libro, ErrorRepositorio> {
        self.libros.get(&id).cloned().ok_or(ErrorRepositorio::NoEncontrado(id))
    }

    fn todos(&self) -> Result<Vec<Libro>, ErrorRepositorio> {
        let mut lista: Vec<Libro> = self.libros.values().cloned().collect();
        lista.sort_by_key(|libro| libro.id);
        Ok(lista)
    }

    fn eliminar(&mut self, id: u32) -> Result<(), ErrorRepositorio> {
        match self.libros.remove(&id) {
            Some(_) => Ok(()),
            None => Err(ErrorRepositorio::NoEncontrado(id)),
        }
    }
}

fn main() {
    let mut repo = RepositorioMemoria::new();
    let a = repo.agregar("Dune", 412).unwrap();
    let b = repo.agregar("El principito", 96).unwrap();
    let c = repo.agregar("Bilbo", 310).unwrap();
    println!("ids: {a}, {b}, {c}");

    repo.eliminar(b).unwrap();
    println!("{:?}", repo.buscar(b));
    println!("{:?}", repo.buscar(c));

    for libro in repo.todos().unwrap() {
        println!("{} - {} ({} páginas)", libro.id, libro.titulo, libro.paginas);
    }
}
```

```sortie
ids: 1, 2, 3
Err(NoEncontrado(2))
Ok(Libro { id: 3, titulo: "Bilbo", paginas: 310 })
1 - Dune (412 páginas)
3 - Bilbo (310 páginas)
```

### Paso a paso

Sigamos el repositorio durante `main`:

| Paso | Llamada | `siguiente_id` después | Claves en `libros` | Resultado |
|------|---------|-----------------------:|--------------------|-----------|
| 1 | `RepositorioMemoria::new()` | 1 | (ninguna) | un repositorio vacío |
| 2 | `agregar("Dune", 412)` | 2 | 1 | `Ok(1)` |
| 3 | `agregar("El principito", 96)` | 3 | 1, 2 | `Ok(2)` |
| 4 | `agregar("Bilbo", 310)` | 4 | 1, 2, 3 | `Ok(3)` |
| 5 | `eliminar(2)` | 4 | 1, 3 | `Ok(())` |
| 6 | `buscar(2)` | 4 | 1, 3 | `Err(NoEncontrado(2))` |
| 7 | `buscar(3)` | 4 | 1, 3 | `Ok(Libro { id: 3, … })` |
| 8 | `todos()` | 4 | 1, 3 | `Ok(vec![Dune, Bilbo])` |

Fíjate en el paso 5: el id 2 desapareció, pero `siguiente_id` no retrocede. El próximo
libro tendrá el id 4. Una base de verdad hace lo mismo: un número dado nunca se vuelve
a dar.

### Tres detalles importantes

- `get(&id)` devuelve una **referencia** (`Option<&Libro>`). `.cloned()` hace una copia
  (`Option<Libro>`), y `.ok_or(…)` transforma `None` en error.
- `remove` devuelve `Some(libro)` si había un libro, `None` si no: es justo lo que hace
  falta para saber si la eliminación funcionó.
- Un `HashMap` devuelve sus valores **desordenados**, y ese orden cambia de una
  ejecución a otra. `sort_by_key` los vuelve a poner en el orden de los ids, como el
  `ORDER BY id` de SQL.

Pruébalo: quita la línea `lista.sort_by_key(…)` y ejecuta varias veces. Dune y Bilbo
aparecen a veces en un orden, a veces en el otro.

Pruébalo: añade `println!("{:?}", repo.eliminar(2));` al final de `main`. El programa
muestra `Err(NoEncontrado(2))`: el libro ya estaba eliminado.

> **Recuerda:** un `HashMap<u32, Libro>` y un contador bastan para imitar una tabla: el
> id es la clave, el libro es el valor.

## Mostrar un error de forma clara

`Err(NoEncontrado(2))` está hecho para el desarrollador. Para el usuario, se escribe
una frase con el trait `Display`, y luego se trata cada caso con `match`:

```rust
use std::fmt;

#[derive(Debug, PartialEq)]
enum ErrorRepositorio {
    NoEncontrado(u32),
    BaseDatos(String),
}

impl fmt::Display for ErrorRepositorio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorRepositorio::NoEncontrado(id) => write!(f, "ningún libro con el id {id}"),
            ErrorRepositorio::BaseDatos(mensaje) => write!(f, "error de la base: {mensaje}"),
        }
    }
}

fn consultar(id: u32) -> Result<String, ErrorRepositorio> {
    match id {
        1 => Ok(String::from("Dune")),
        99 => Err(ErrorRepositorio::BaseDatos(String::from("disco lleno"))),
        _ => Err(ErrorRepositorio::NoEncontrado(id)),
    }
}

fn main() {
    for id in [1, 2, 99] {
        match consultar(id) {
            Ok(titulo) => println!("{id}: {titulo}"),
            Err(e) => println!("{id}: Error: {e}"),
        }
    }
}
```

```sortie
1: Dune
2: Error: ningún libro con el id 2
99: Error: error de la base: disco lleno
```

La variante `BaseDatos` servirá para la versión SQLite: cuando el disco o la consulta
dan problemas, se guarda dentro el mensaje de `rusqlite`.

| `id` | `consultar(id)` | Rama del `match` | Línea mostrada |
|-----:|-----------------|------------------|----------------|
| 1 | `Ok("Dune")` | `Ok(titulo)` | `1: Dune` |
| 2 | `Err(NoEncontrado(2))` | `Err(e)` | `2: Error: ningún libro…` |
| 99 | `Err(BaseDatos("disco lleno"))` | `Err(e)` | `99: Error: error de la base…` |

> **Recuerda:** `Debug` (`{:?}`) para ti, `Display` (`{}`) para el usuario.

## Escribir el resto del programa una sola vez

Aquí tienes una función que trabaja con **cualquier** repositorio. Recibe
`&impl Repositorio`: «algo que cumple las promesas de `Repositorio`» (ver
[Los genéricos](generiques.md)). No sabe si es un `HashMap` o SQLite.

Añádela al programa de la sección [Un repositorio en memoria con un
HashMap](#un-repositorio-en-memoria-con-un-hashmap), y añade las líneas de prueba al
final de `main`:

```rust norun
// Solo conoce el trait: funciona con todos los repositorios.
fn paginas_totales(repo: &impl Repositorio) -> Result<u32, ErrorRepositorio> {
    let mut total = 0;
    for libro in repo.todos()? {
        total += libro.paginas;
    }
    Ok(total)
}

fn main() {
    // ... el principio de main, como arriba ...
    println!("Páginas por leer: {}", paginas_totales(&repo).unwrap());

    // Pequeños tests: el repositorio debe rechazar un id desconocido.
    assert_eq!(repo.eliminar(7), Err(ErrorRepositorio::NoEncontrado(7)));
    assert_eq!(repo.todos().unwrap().len(), 2);
    println!("Tests superados");
}
```

Las nuevas líneas que se muestran al final:

```text
Páginas por leer: 722
Tests superados
```

El bucle de `paginas_totales`, paso a paso (el libro 2 se eliminó):

| Vuelta | `libro.titulo` | `libro.paginas` | `total` después |
|-------:|----------------|----------------:|----------------:|
| inicio | | | 0 |
| 1 | Dune | 412 | 412 |
| 2 | Bilbo | 310 | 722 |

Los dos `assert_eq!` son pequeños tests: si uno es falso, el programa se detiene con un
mensaje (ver [Probar tu código](tests.md)). Gracias al repositorio en memoria, estos
tests se ejecutan en una fracción de segundo, sin archivo ni base.

Pruébalo: cambia `eliminar(7)` por `eliminar(1)`. El libro 1 existe, el método devuelve
`Ok(())`, y el programa se detiene en `assertion `left == right` failed`.

> **Recuerda:** escribe tus funciones con `impl Repositorio`; pruébalas con el
> repositorio en memoria, úsalas con la base de verdad.

## El mismo trait con rusqlite

Solo queda conectar la base de verdad. `RepositorioSqlite` guarda una conexión, y cada
método del trait se convierte en una consulta SQL. Los tipos `Libro`,
`ErrorRepositorio` (con su variante `BaseDatos`) y el trait `Repositorio` son
**exactamente** los de arriba:

```rust norun
use rusqlite::{params, Connection, OptionalExtension};

// Transforma un error de rusqlite en ErrorRepositorio.
fn error_base(e: rusqlite::Error) -> ErrorRepositorio {
    ErrorRepositorio::BaseDatos(e.to_string())
}

struct RepositorioSqlite {
    conn: Connection,
}

impl RepositorioSqlite {
    fn abrir(ruta: &str) -> Result<RepositorioSqlite, ErrorRepositorio> {
        let conn = Connection::open(ruta).map_err(error_base)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS libros (
                id      INTEGER PRIMARY KEY,
                titulo  TEXT NOT NULL,
                paginas INTEGER NOT NULL
            )",
            [],
        )
        .map_err(error_base)?;
        Ok(RepositorioSqlite { conn })
    }
}

impl Repositorio for RepositorioSqlite {
    fn agregar(&mut self, titulo: &str, paginas: u32) -> Result<u32, ErrorRepositorio> {
        self.conn
            .execute(
                "INSERT INTO libros (titulo, paginas) VALUES (?1, ?2)",
                params![titulo, paginas],
            )
            .map_err(error_base)?;
        Ok(self.conn.last_insert_rowid() as u32)
    }

    fn buscar(&self, id: u32) -> Result<Libro, ErrorRepositorio> {
        self.conn
            .query_row(
                "SELECT id, titulo, paginas FROM libros WHERE id = ?1",
                [id],
                |fila| {
                    Ok(Libro {
                        id: fila.get(0)?,
                        titulo: fila.get(1)?,
                        paginas: fila.get(2)?,
                    })
                },
            )
            .optional()
            .map_err(error_base)?
            .ok_or(ErrorRepositorio::NoEncontrado(id))
    }

    fn todos(&self) -> Result<Vec<Libro>, ErrorRepositorio> {
        let mut consulta = self
            .conn
            .prepare("SELECT id, titulo, paginas FROM libros ORDER BY id")
            .map_err(error_base)?;
        let filas = consulta
            .query_map([], |fila| {
                Ok(Libro {
                    id: fila.get(0)?,
                    titulo: fila.get(1)?,
                    paginas: fila.get(2)?,
                })
            })
            .map_err(error_base)?;
        filas.collect::<Result<Vec<Libro>, _>>().map_err(error_base)
    }

    fn eliminar(&mut self, id: u32) -> Result<(), ErrorRepositorio> {
        let cantidad = self
            .conn
            .execute("DELETE FROM libros WHERE id = ?1", [id])
            .map_err(error_base)?;
        if cantidad == 0 {
            Err(ErrorRepositorio::NoEncontrado(id))
        } else {
            Ok(())
        }
    }
}

fn main() {
    let mut repo = RepositorioSqlite::abrir("biblioteca.db").unwrap();
    let a = repo.agregar("Dune", 412).unwrap();
    let b = repo.agregar("El principito", 96).unwrap();
    println!("ids: {a} y {b}");
    println!("{:?}", repo.buscar(a));
    repo.eliminar(a).unwrap();
    for libro in repo.todos().unwrap() {
        println!("{} - {} ({} páginas)", libro.id, libro.titulo, libro.paginas);
    }
    match repo.buscar(a) {
        Ok(libro) => println!("encontrado: {}", libro.titulo),
        Err(e) => println!("Error: {e}"),
    }
}
```

Primera ejecución (el archivo todavía no existe):

```text
ids: 1 y 2
Ok(Libro { id: 1, titulo: "Dune", paginas: 412 })
2 - El principito (96 páginas)
Error: ningún libro con el id 1
```

Segunda ejecución: el archivo `biblioteca.db` guardó El principito n.º 2.

```text
ids: 3 y 4
Ok(Libro { id: 3, titulo: "Dune", paginas: 412 })
2 - El principito (96 páginas)
4 - El principito (96 páginas)
Error: ningún libro con el id 3
```

Método por método, la traducción es sencilla:

| Método del trait | `RepositorioMemoria` (HashMap) | `RepositorioSqlite` (SQL) |
|------------------|--------------------------------|---------------------------|
| `agregar` | `insert` + contador | `INSERT` + `last_insert_rowid` |
| `buscar` | `get(&id)` | `SELECT … WHERE id = ?1` |
| `todos` | `values()` + `sort_by_key` | `SELECT … ORDER BY id` |
| `eliminar` | `remove(&id)` → `None` = no encontrado | `DELETE` → 0 filas = no encontrado |
| error de la base | nunca | `map_err(error_base)` → `ErrorRepositorio::BaseDatos` |

- `.optional()` transforma «ninguna fila» en `Ok(None)`, en lugar de un error.
- `execute` devuelve el **número de filas** afectadas: 0 quiere decir que el id no
  existía.
- `map_err(error_base)` guarda cualquier error de `rusqlite` en nuestro
  `ErrorRepositorio::BaseDatos`. Así el trait nunca habla de `rusqlite`.

¿Y la función `paginas_totales` de la sección anterior? Funciona tal cual:
`paginas_totales(&repo)` con un `RepositorioSqlite`. Ni una línea que cambiar.

> **Recuerda:** para pasar de la memoria a SQLite, se escribe un nuevo
> `impl Repositorio for …`. El resto del programa no se mueve.

## Errores frecuentes

### Olvidar &mut self para modificar

```rust norun
use std::collections::HashMap;

struct RepositorioMemoria {
    libros: HashMap<u32, String>,
}

impl RepositorioMemoria {
    fn agregar(&self, id: u32, titulo: &str) {
        self.libros.insert(id, titulo.to_string());
    }
}
```

```text
error[E0596]: cannot borrow `self.libros` as mutable, as it is behind a `&` reference
 --> src/main.rs:9:9
  |
9 |         self.libros.insert(id, titulo.to_string());
  |         ^^^^^^^^^^^ `self` is a `&` reference, so it cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
8 |     fn agregar(&mut self, id: u32, titulo: &str) {
  |                 +++
```

`&self` solo permite mirar. Para añadir o eliminar, escribe `&mut self`, en el trait
**y** en el `impl`.

### Olvidar un método del trait

```rust norun
trait Repositorio {
    fn agregar(&mut self, titulo: &str) -> Result<u32, String>;
    fn eliminar(&mut self, id: u32) -> Result<(), String>;
}

struct RepositorioMemoria;

impl Repositorio for RepositorioMemoria {
    fn agregar(&mut self, _titulo: &str) -> Result<u32, String> {
        Ok(1)
    }
}
```

```text
error[E0046]: not all trait items implemented, missing: `eliminar`
 --> src/main.rs:8:1
  |
3 |     fn eliminar(&mut self, id: u32) -> Result<(), String>;
  |     ------------------------------------------------------ `eliminar` from trait
...
8 | impl Repositorio for RepositorioMemoria {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `eliminar` in implementation
```

Un trait es una promesa completa: escribe todos sus métodos.

### Usar una tabla que no existe

```rust norun
let conn = Connection::open_in_memory().unwrap();
conn.execute("INSERT INTO libros (titulo, paginas) VALUES ('Dune', 412)", [])
    .unwrap();
```

```text
called `Result::unwrap()` on an `Err` value: SqliteFailure(Error { code: Unknown, extended_code: 1 }, Some("no such table: libros"))
```

El programa compila: el SQL solo se comprueba durante la ejecución. Crea la tabla
primero, con `CREATE TABLE IF NOT EXISTS`, al abrir la base.

## Ponte a prueba

1. ¿Por qué `todos` ordena la lista en `RepositorioMemoria`?

> **Respuesta**
>
> Porque un `HashMap` devuelve sus valores en un orden que cambia de una ejecución a
> otra. `sort_by_key(|libro| libro.id)` da siempre el mismo orden, como
> `ORDER BY id` en SQL.

2. ¿Por qué `buscar` recibe `&self` mientras que `agregar` recibe `&mut self`?

> **Respuesta**
>
> `buscar` solo mira el repositorio, no cambia nada. `agregar` modifica el
> repositorio (inserta un libro y aumenta el contador), así que necesita `&mut self`.

3. ¿Qué interés tiene el trait `Repositorio` si al final solo se usa SQLite?

> **Respuesta**
>
> Se puede probar todo el programa con `RepositorioMemoria`, sin archivo ni base, muy
> rápido. Y el SQL queda guardado en un solo sitio: si se cambia de base, solo hay que
> escribir un nuevo `impl Repositorio`.

## En resumen

- Una base de datos guarda la información en el disco; SQLite cabe en un solo archivo.
- Con `rusqlite`: `execute` para modificar, `prepare` + `query_map` para leer, huecos
  `?1` para los valores.
- Un trait `Repositorio` (agregar, buscar, todos, eliminar) esconde la base al resto
  del programa; cada método devuelve un `Result`.
- `RepositorioMemoria` (un `HashMap` y un contador) sirve para probar y hacer tests.
- `RepositorioSqlite` implementa el mismo trait con SQL: nada más cambia.

Siguiente: [Rust para la web](web.md)
