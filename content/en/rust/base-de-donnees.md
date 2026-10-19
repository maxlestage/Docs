# A database in Rust

A database keeps your information when the program stops: in Rust, you talk to it with the `rusqlite` crate, and you hide this detail behind a trait so that the rest of the program does not depend on it.

> **Goals:**
>
> - understand what a database is for;
> - create a table, add and read rows with `rusqlite`;
> - write a `Repository` trait with methods that return a `Result`;
> - make it work with a simple `HashMap`, then with SQLite;
> - test the whole program without a database.

## Why a database?

Your variables live in [memory](../debuter/glossaire.md#memoire-vive) (RAM).
When the program stops, they disappear. To keep information, you have to write it
to the disk.

You could write a text file (see [JSON, CSV and TOML](formats.md)). But to find
**one** book among 10,000, you would have to read the whole file again. A **database**
is a file organized like an office filing cabinet: labeled drawers, numbered cards, and
a clerk who finds the right card straight away.

```text
  Rust program                   Disk
 ┌──────────────┐  request   ┌──────────────────────┐
 │ let books =  │ ─────────► │ library.db           │
 │   ...        │            │  table "books"       │
 │              │ ◄───────── │  id | title | pages  │
 └──────────────┘  answer    └──────────────────────┘
  (erased at the end)          (stays after the end)
```

A **table** looks like a spreadsheet: columns (`id`, `title`, `pages`) and one row per
item. You talk to it in a small language, **SQL** (see
[Databases](../debuter/bases-de-donnees.md)).

| id | title | pages |
|----|-------|-------|
| 1 | Dune | 412 |
| 2 | The Little Prince | 96 |

> **Remember:** memory is erased when the program stops; a database keeps the
> information on the disk and quickly finds the piece you ask for.

## SQLite and the rusqlite crate

**SQLite** is the simplest database: no server to install, the whole database fits in
**a single file**. The `rusqlite` [crate](../debuter/glossaire.md#crate) lets you use it
from Rust:

```sh
cargo add rusqlite --features bundled
```

Your `Cargo.toml` then contains:

```toml
[dependencies]
rusqlite = { version = "0.40", features = ["bundled"] }
```

> **Note:** `bundled` puts SQLite inside your program. You have nothing else to install
> on the computer.

The examples with `rusqlite` cannot run on this page (they need Cargo). Their output
was obtained with `cargo run`.

## Creating a table, adding and reading

Here is a complete program. It opens (or creates) the file, creates the table, adds two
books and reads them back:

```rust norun
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Book {
    id: i64,
    title: String,
    pages: u32,
}

fn main() -> Result<()> {
    let conn = Connection::open("library.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS books (
            id     INTEGER PRIMARY KEY,
            title  TEXT NOT NULL,
            pages  INTEGER NOT NULL
        )",
        [],
    )?;
    conn.execute("DELETE FROM books", [])?;

    conn.execute("INSERT INTO books (title, pages) VALUES (?1, ?2)", params!["Dune", 412])?;
    conn.execute("INSERT INTO books (title, pages) VALUES (?1, ?2)", params!["The Little Prince", 96])?;
    println!("Last id: {}", conn.last_insert_rowid());

    let mut query = conn.prepare("SELECT id, title, pages FROM books ORDER BY id")?;
    let books = query.query_map([], |row| {
        Ok(Book { id: row.get(0)?, title: row.get(1)?, pages: row.get(2)? })
    })?;
    for book in books {
        println!("{:?}", book?);
    }
    Ok(())
}
```

```text
Last id: 2
Book { id: 1, title: "Dune", pages: 412 }
Book { id: 2, title: "The Little Prince", pages: 96 }
```

Step by step:

| Step | Line | What happens | Table `books` |
|------|------|--------------|---------------|
| 1 | `Connection::open` | opens the file (creates it if it does not exist) | does not exist yet |
| 2 | `CREATE TABLE IF NOT EXISTS` | creates the table, only the first time | empty |
| 3 | `DELETE FROM books` | empties the table (for this example) | empty |
| 4 | 1st `INSERT` | SQLite picks id 1 | (1, Dune, 412) |
| 5 | 2nd `INSERT` | SQLite picks id 2 | + (2, The Little Prince, 96) |
| 6 | `last_insert_rowid` | returns the id of the last insert | 2 |
| 7 | `query_map` | reads each row and builds a `Book` | unchanged |

- `?1`, `?2` are **placeholders** filled by `params![…]`, in order.
- `row.get(0)?` reads column number 0 of the row (counting starts at 0).
- Each call returns a [`Result`](../debuter/glossaire.md#result): `?` stops the
  function at the first error (see [Handling errors](erreurs.md)).

> **Warning:** never write `format!("… VALUES ('{title}')")` to put a value into a
> query. A title like `Charlotte's Web` breaks the query, and a malicious user can slip
> their own SQL commands into it (an "SQL injection"). `?1` placeholders are always
> safe.

Try it: remove the `DELETE FROM books` line and run it twice. The second time, you will
see four books and `Last id: 4`: the file kept the ones from the first time.

> **Remember:** `execute` to modify, `prepare` + `query_map` to read, and `?1`
> placeholders for the values.

## The problem: SQL everywhere

Now imagine a bigger program. If every function calls `conn.execute` directly, SQL is
scattered everywhere:

- to test a function, you need a real database;
- to change databases (PostgreSQL instead of SQLite), you have to reread everything.

The solution: a **counter**. At a library counter, you say "I would like book number
2". You do not know whether the clerk will fetch it from a drawer, a basement or a
warehouse. You only know what you can ask for.

In Rust, this counter is a [trait](../debuter/glossaire.md#trait) (see
[Traits and objects](traits-objets.md)). It is often called a `Repository`.

```text
          the rest of the program
                    │
                    ▼
        ┌───────────────────────┐
        │   trait Repository    │   add, find, all, remove
        └───────────────────────┘
             ▲               ▲
             │               │
 ┌──────────────────┐ ┌──────────────────┐
 │ MemoryRepository │ │ SqliteRepository │
 │    (HashMap)     │ │   (rusqlite)     │
 └──────────────────┘ └──────────────────┘
     to try things        for real
     and to test
```

> **Remember:** the rest of the program only talks to the trait. Behind it, you can
> plug in a `HashMap` or a real database, without changing anything else.

## The Repository trait

The counter offers four requests. Each one can fail, so each one returns a `Result`:

```rust
#[derive(Debug, Clone, PartialEq)]
struct Book {
    id: u32,
    title: String,
    pages: u32,
}

#[derive(Debug, PartialEq)]
enum RepoError {
    NotFound(u32),
}

trait Repository {
    fn add(&mut self, title: &str, pages: u32) -> Result<u32, RepoError>;
    fn find(&self, id: u32) -> Result<Book, RepoError>;
    fn all(&self) -> Result<Vec<Book>, RepoError>;
    fn remove(&mut self, id: u32) -> Result<(), RepoError>;
}
```

This block does nothing: it only **describes**. Let's read each line:

| Method | Receives | Returns if all goes well | Can fail if… |
|--------|----------|--------------------------|--------------|
| `add` | a title, a number of pages | the new id (`u32`) | the database has a problem |
| `find` | an id | a copy of the `Book` | no book has this id |
| `all` | nothing | the list of books | the database has a problem |
| `remove` | an id | nothing (`()`) | no book has this id |

- `&mut self`: the method **modifies** the repository (add, remove).
- `&self`: the method only **looks** (find, all).
- `RepoError` is our own error type (see
  [Your own error type](erreurs.md#your-own-error-type)).

> **Remember:** a trait is a list of promises. Every type that implements it must keep
> all four.

## An in-memory repository with a HashMap

First implementation: a [`HashMap`](hashmap.md) that stores each book under its id,
and a `next_id` counter that hands out the numbers, like the ticket machine at a
queue.

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct Book {
    id: u32,
    title: String,
    pages: u32,
}

#[derive(Debug, PartialEq)]
enum RepoError {
    NotFound(u32),
}

trait Repository {
    fn add(&mut self, title: &str, pages: u32) -> Result<u32, RepoError>;
    fn find(&self, id: u32) -> Result<Book, RepoError>;
    fn all(&self) -> Result<Vec<Book>, RepoError>;
    fn remove(&mut self, id: u32) -> Result<(), RepoError>;
}

struct MemoryRepository {
    books: HashMap<u32, Book>,
    next_id: u32,
}

impl MemoryRepository {
    fn new() -> MemoryRepository {
        MemoryRepository { books: HashMap::new(), next_id: 1 }
    }
}

impl Repository for MemoryRepository {
    fn add(&mut self, title: &str, pages: u32) -> Result<u32, RepoError> {
        let id = self.next_id;
        self.next_id += 1;
        self.books.insert(id, Book { id, title: title.to_string(), pages });
        Ok(id)
    }

    fn find(&self, id: u32) -> Result<Book, RepoError> {
        self.books.get(&id).cloned().ok_or(RepoError::NotFound(id))
    }

    fn all(&self) -> Result<Vec<Book>, RepoError> {
        let mut list: Vec<Book> = self.books.values().cloned().collect();
        list.sort_by_key(|book| book.id);
        Ok(list)
    }

    fn remove(&mut self, id: u32) -> Result<(), RepoError> {
        match self.books.remove(&id) {
            Some(_) => Ok(()),
            None => Err(RepoError::NotFound(id)),
        }
    }
}

fn main() {
    let mut repo = MemoryRepository::new();
    let a = repo.add("Dune", 412).unwrap();
    let b = repo.add("The Little Prince", 96).unwrap();
    let c = repo.add("Bilbo", 310).unwrap();
    println!("ids: {a}, {b}, {c}");

    repo.remove(b).unwrap();
    println!("{:?}", repo.find(b));
    println!("{:?}", repo.find(c));

    for book in repo.all().unwrap() {
        println!("{} - {} ({} pages)", book.id, book.title, book.pages);
    }
}
```

```sortie
ids: 1, 2, 3
Err(NotFound(2))
Ok(Book { id: 3, title: "Bilbo", pages: 310 })
1 - Dune (412 pages)
3 - Bilbo (310 pages)
```

### Step by step

Let's follow the repository during `main`:

| Step | Call | `next_id` after | Keys in `books` | Result |
|------|------|----------------:|-----------------|--------|
| 1 | `MemoryRepository::new()` | 1 | (none) | an empty repository |
| 2 | `add("Dune", 412)` | 2 | 1 | `Ok(1)` |
| 3 | `add("The Little Prince", 96)` | 3 | 1, 2 | `Ok(2)` |
| 4 | `add("Bilbo", 310)` | 4 | 1, 2, 3 | `Ok(3)` |
| 5 | `remove(2)` | 4 | 1, 3 | `Ok(())` |
| 6 | `find(2)` | 4 | 1, 3 | `Err(NotFound(2))` |
| 7 | `find(3)` | 4 | 1, 3 | `Ok(Book { id: 3, … })` |
| 8 | `all()` | 4 | 1, 3 | `Ok(vec![Dune, Bilbo])` |

Look at step 5: id 2 is gone, but `next_id` does not go back. The next book will get
id 4. A real database does the same: a number that was handed out is never reused.

### Three important details

- `get(&id)` returns a **reference** (`Option<&Book>`). `.cloned()` makes a copy of it
  (`Option<Book>`), and `.ok_or(…)` turns `None` into an error.
- `remove` returns `Some(book)` if there was a book, `None` otherwise: that is exactly
  what you need to know whether the removal worked.
- A `HashMap` gives back its values **in no particular order**, and this order changes
  from one run to the next. `sort_by_key` puts them back in id order, like the SQL
  `ORDER BY id`.

Try it: remove the `list.sort_by_key(…)` line and run it several times. Dune and Bilbo
are sometimes displayed in one order, sometimes in the other.

Try it: add `println!("{:?}", repo.remove(2));` at the end of `main`. The program
displays `Err(NotFound(2))`: the book was already removed.

> **Remember:** a `HashMap<u32, Book>` and a counter are enough to imitate a table: the
> id is the key, the book is the value.

## Displaying an error nicely

`Err(NotFound(2))` is meant for the developer. For the user, you write a sentence with
the `Display` trait, then you handle each case with `match`:

```rust
use std::fmt;

#[derive(Debug, PartialEq)]
enum RepoError {
    NotFound(u32),
    Database(String),
}

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RepoError::NotFound(id) => write!(f, "no book with id {id}"),
            RepoError::Database(message) => write!(f, "database error: {message}"),
        }
    }
}

fn look_up(id: u32) -> Result<String, RepoError> {
    match id {
        1 => Ok(String::from("Dune")),
        99 => Err(RepoError::Database(String::from("disk full"))),
        _ => Err(RepoError::NotFound(id)),
    }
}

fn main() {
    for id in [1, 2, 99] {
        match look_up(id) {
            Ok(title) => println!("{id}: {title}"),
            Err(e) => println!("{id}: Error: {e}"),
        }
    }
}
```

```sortie
1: Dune
2: Error: no book with id 2
99: Error: database error: disk full
```

The `Database` variant will be used by the SQLite version: when the disk or the query
causes a problem, you store the `rusqlite` message in it.

| `id` | `look_up(id)` | `match` branch | Line displayed |
|-----:|---------------|----------------|----------------|
| 1 | `Ok("Dune")` | `Ok(title)` | `1: Dune` |
| 2 | `Err(NotFound(2))` | `Err(e)` | `2: Error: no book…` |
| 99 | `Err(Database("disk full"))` | `Err(e)` | `99: Error: database error…` |

> **Remember:** `Debug` (`{:?}`) for you, `Display` (`{}`) for the user.

## Writing the rest of the program only once

Here is a function that works with **any** repository. It receives `&impl Repository`:
"something that keeps the promises of `Repository`" (see [Generics](generiques.md)).
It does not know whether it is a `HashMap` or SQLite.

Add it to the program from the section [An in-memory repository with a
HashMap](#an-in-memory-repository-with-a-hashmap), and add the test lines at the end
of `main`:

```rust norun
// Only knows the trait: works with every repository.
fn total_pages(repo: &impl Repository) -> Result<u32, RepoError> {
    let mut total = 0;
    for book in repo.all()? {
        total += book.pages;
    }
    Ok(total)
}

fn main() {
    // ... the beginning of main, as above ...
    println!("Pages to read: {}", total_pages(&repo).unwrap());

    // Small tests: the repository must refuse an unknown id.
    assert_eq!(repo.remove(7), Err(RepoError::NotFound(7)));
    assert_eq!(repo.all().unwrap().len(), 2);
    println!("Tests passed");
}
```

The new lines displayed at the end:

```text
Pages to read: 722
Tests passed
```

The `total_pages` loop, step by step (book 2 was removed):

| Round | `book.title` | `book.pages` | `total` after |
|------:|--------------|-------------:|--------------:|
| start | | | 0 |
| 1 | Dune | 412 | 412 |
| 2 | Bilbo | 310 | 722 |

The two `assert_eq!` are small tests: if one is false, the program stops with a message
(see [Testing your code](tests.md)). Thanks to the in-memory repository, these tests
run in a fraction of a second, with no file and no database.

Try it: replace `remove(7)` with `remove(1)`. Book 1 exists, the method returns
`Ok(())`, and the program stops on `assertion `left == right` failed`.

> **Remember:** write your functions with `impl Repository`; test them with the
> in-memory repository, use them with the real database.

## The same trait with rusqlite

All that is left is to plug in the real database. `SqliteRepository` keeps a
connection, and each method of the trait becomes an SQL query. The types `Book`,
`RepoError` (with its `Database` variant) and the `Repository` trait are **exactly**
the ones from above:

```rust norun
use rusqlite::{params, Connection, OptionalExtension};

// Turns a rusqlite error into a RepoError.
fn db_error(e: rusqlite::Error) -> RepoError {
    RepoError::Database(e.to_string())
}

struct SqliteRepository {
    conn: Connection,
}

impl SqliteRepository {
    fn open(path: &str) -> Result<SqliteRepository, RepoError> {
        let conn = Connection::open(path).map_err(db_error)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS books (
                id    INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                pages INTEGER NOT NULL
            )",
            [],
        )
        .map_err(db_error)?;
        Ok(SqliteRepository { conn })
    }
}

impl Repository for SqliteRepository {
    fn add(&mut self, title: &str, pages: u32) -> Result<u32, RepoError> {
        self.conn
            .execute(
                "INSERT INTO books (title, pages) VALUES (?1, ?2)",
                params![title, pages],
            )
            .map_err(db_error)?;
        Ok(self.conn.last_insert_rowid() as u32)
    }

    fn find(&self, id: u32) -> Result<Book, RepoError> {
        self.conn
            .query_row(
                "SELECT id, title, pages FROM books WHERE id = ?1",
                [id],
                |row| {
                    Ok(Book {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        pages: row.get(2)?,
                    })
                },
            )
            .optional()
            .map_err(db_error)?
            .ok_or(RepoError::NotFound(id))
    }

    fn all(&self) -> Result<Vec<Book>, RepoError> {
        let mut query = self
            .conn
            .prepare("SELECT id, title, pages FROM books ORDER BY id")
            .map_err(db_error)?;
        let rows = query
            .query_map([], |row| {
                Ok(Book {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    pages: row.get(2)?,
                })
            })
            .map_err(db_error)?;
        rows.collect::<Result<Vec<Book>, _>>().map_err(db_error)
    }

    fn remove(&mut self, id: u32) -> Result<(), RepoError> {
        let count = self
            .conn
            .execute("DELETE FROM books WHERE id = ?1", [id])
            .map_err(db_error)?;
        if count == 0 {
            Err(RepoError::NotFound(id))
        } else {
            Ok(())
        }
    }
}

fn main() {
    let mut repo = SqliteRepository::open("library.db").unwrap();
    let a = repo.add("Dune", 412).unwrap();
    let b = repo.add("The Little Prince", 96).unwrap();
    println!("ids: {a} and {b}");
    println!("{:?}", repo.find(a));
    repo.remove(a).unwrap();
    for book in repo.all().unwrap() {
        println!("{} - {} ({} pages)", book.id, book.title, book.pages);
    }
    match repo.find(a) {
        Ok(book) => println!("found: {}", book.title),
        Err(e) => println!("Error: {e}"),
    }
}
```

First run (the file does not exist yet):

```text
ids: 1 and 2
Ok(Book { id: 1, title: "Dune", pages: 412 })
2 - The Little Prince (96 pages)
Error: no book with id 1
```

Second run: the `library.db` file kept The Little Prince number 2.

```text
ids: 3 and 4
Ok(Book { id: 3, title: "Dune", pages: 412 })
2 - The Little Prince (96 pages)
4 - The Little Prince (96 pages)
Error: no book with id 3
```

Method by method, the translation is simple:

| Trait method | `MemoryRepository` (HashMap) | `SqliteRepository` (SQL) |
|--------------|------------------------------|--------------------------|
| `add` | `insert` + counter | `INSERT` + `last_insert_rowid` |
| `find` | `get(&id)` | `SELECT … WHERE id = ?1` |
| `all` | `values()` + `sort_by_key` | `SELECT … ORDER BY id` |
| `remove` | `remove(&id)` → `None` = not found | `DELETE` → 0 rows = not found |
| database error | never | `map_err(db_error)` → `RepoError::Database` |

- `.optional()` turns "no row" into `Ok(None)`, instead of an error.
- `execute` returns the **number of rows** affected: 0 means the id did not exist.
- `map_err(db_error)` stores any `rusqlite` error in our `RepoError::Database`.
  That way the trait never mentions `rusqlite`.

And the `total_pages` function from the previous section? It works as it is:
`total_pages(&repo)` with a `SqliteRepository`. Not a single line to change.

> **Remember:** to go from memory to SQLite, you write a new
> `impl Repository for …`. The rest of the program does not move.

## Common mistakes

### Forgetting &mut self to modify

```rust norun
use std::collections::HashMap;

struct MemoryRepository {
    books: HashMap<u32, String>,
}

impl MemoryRepository {
    fn add(&self, id: u32, title: &str) {
        self.books.insert(id, title.to_string());
    }
}
```

```text
error[E0596]: cannot borrow `self.books` as mutable, as it is behind a `&` reference
 --> src/main.rs:9:9
  |
9 |         self.books.insert(id, title.to_string());
  |         ^^^^^^^^^^ `self` is a `&` reference, so it cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
8 |     fn add(&mut self, id: u32, title: &str) {
  |             +++
```

`&self` only lets you look. To add or remove, write `&mut self`, in the trait **and**
in the `impl`.

### Forgetting a method of the trait

```rust norun
trait Repository {
    fn add(&mut self, title: &str) -> Result<u32, String>;
    fn remove(&mut self, id: u32) -> Result<(), String>;
}

struct MemoryRepository;

impl Repository for MemoryRepository {
    fn add(&mut self, _title: &str) -> Result<u32, String> {
        Ok(1)
    }
}
```

```text
error[E0046]: not all trait items implemented, missing: `remove`
 --> src/main.rs:8:1
  |
3 |     fn remove(&mut self, id: u32) -> Result<(), String>;
  |     ---------------------------------------------------- `remove` from trait
...
8 | impl Repository for MemoryRepository {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `remove` in implementation
```

A trait is a complete promise: write all of its methods.

### Using a table that does not exist

```rust norun
let conn = Connection::open_in_memory().unwrap();
conn.execute("INSERT INTO books (title, pages) VALUES ('Dune', 412)", [])
    .unwrap();
```

```text
called `Result::unwrap()` on an `Err` value: SqliteFailure(Error { code: Unknown, extended_code: 1 }, Some("no such table: books"))
```

The program compiles: the SQL is only checked when it runs. Create the table first,
with `CREATE TABLE IF NOT EXISTS`, when you open the database.

## Test yourself

1. Why does `all` sort the list in `MemoryRepository`?

> **Answer**
>
> Because a `HashMap` gives back its values in an order that changes from one run to
> the next. `sort_by_key(|book| book.id)` always gives the same order, like
> `ORDER BY id` in SQL.

2. Why does `find` take `&self` while `add` takes `&mut self`?

> **Answer**
>
> `find` only looks at the repository, it changes nothing. `add` modifies the
> repository (it inserts a book and increases the counter), so it needs `&mut self`.

3. What is the point of the `Repository` trait if you only use SQLite in the end?

> **Answer**
>
> You can test the whole program with `MemoryRepository`, with no file and no
> database, very quickly. And the SQL stays in a single place: if you change
> databases, you only write a new `impl Repository`.

## Summary

- A database keeps information on the disk; SQLite fits in a single file.
- With `rusqlite`: `execute` to modify, `prepare` + `query_map` to read, `?1`
  placeholders for the values.
- A `Repository` trait (add, find, all, remove) hides the database from the rest of
  the program; each method returns a `Result`.
- `MemoryRepository` (a `HashMap` and a counter) is used to try things and to test.
- `SqliteRepository` implements the same trait with SQL: nothing else changes.

Next: [Rust for the web](web.md)
