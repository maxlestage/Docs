# A database in Go

This page shows how a Go program talks to a database with the `database/sql` package, then how to hide the database behind a `Store` interface that you can test without any database at all.

> **Goals:**
>
> - understand the role of `database/sql` and of a driver;
> - open a database, write with `Exec`, read with `QueryRow` and `Query`;
> - use `?` parameters instead of pasting text into the SQL;
> - write a `Store` interface and an in-memory version (map + lock);
> - see how an SQL version follows the same interface.

If you have never seen SQL, first read
[databases](../debuter/bases-de-donnees.md) in the Beginner track.

## Why a database?

A variable disappears when the program stops. A [file](fichiers.md) stays, but searching
it becomes slow and complicated as it grows.

A **database** is a well-organized library, with a librarian. You don't search the
shelves yourself: you ask for "the books by Hugo, sorted by title", and the librarian
brings them to you. The language you use to talk to them is called **SQL**.

```text
 your program ── "SELECT title FROM books" ──►  database
              ◄──────── the rows found ─────────
```

> **Remember:** the database keeps the data between two runs and knows how to search
> fast. Your program sends it SQL commands.

## database/sql and drivers

Go has a single package for all databases: `database/sql`. But it can't talk to **any**
database on its own. It needs a **driver**: a small
[package](../debuter/glossaire.md#package) that knows one specific database.

It's like a travel plug adapter: `database/sql` is your device, the driver is the
adapter for the country.

```text
           your code
               │   (always the same functions: Open, Exec, Query…)
         database/sql
          ┌────┴─────────────┐
   SQLite driver      PostgreSQL driver
          │                  │
      .db file       PostgreSQL server
```

Drivers are not in the standard library. You add them with `go get`
(see [modules](modules.md)):

| Database | Common driver | Name to give to `sql.Open` | Parameters |
|---|---|---|---|
| SQLite (a simple file) | `modernc.org/sqlite` | `"sqlite"` | `?` |
| PostgreSQL (a server) | `github.com/jackc/pgx/v5/stdlib` | `"pgx"` | `$1`, `$2`… |

You import the driver with an **underscore**: `_ "modernc.org/sqlite"`. The `_` means
"I don't use any name from this package, but load it". While it loads, the driver
registers itself with `database/sql`.

### What happens without a driver?

This program uses only the standard library. It asks for a driver that has not been
imported:

```go
package main

import (
	"database/sql"
	"fmt"
)

func main() {
	db, err := sql.Open("sqlite", "books.db")
	fmt.Println(db == nil)
	fmt.Println(err)
}
```

```sortie
true
sql: unknown driver "sqlite" (forgotten import?)
```

The message says it itself: "unknown driver (forgotten import?)".

> **Remember:** `database/sql` + a driver imported with `_`. The name given to
> `sql.Open` depends on the driver.

## Opening the database

The examples in this section need a driver: they are shown without being run, along
with what they really print.

```go norun
package main

import (
	"database/sql"
	"fmt"
	"log"

	_ "modernc.org/sqlite"
)

func main() {
	db, err := sql.Open("sqlite", "books.db")
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	if err := db.Ping(); err != nil {
		log.Fatal(err)
	}
	fmt.Println("database open")
}
```

```text
database open
```

- `sql.Open` **prepares** the access, but doesn't connect yet.
- `db.Ping()` checks that the database really answers.
- `db` is not a single connection: it's a **pool** of connections, safe to use from
  several [goroutines](concurrence.md). You open it once, at startup, and you keep it.

For PostgreSQL, only two lines change:

```go norun
import _ "github.com/jackc/pgx/v5/stdlib"

db, err := sql.Open("pgx", "postgres://lea:secret@localhost:5432/library")
```

## Writing: Exec

`db.Exec` sends a command that doesn't return rows: create a table, insert, update,
delete.

```go norun
_, err = db.Exec(`CREATE TABLE IF NOT EXISTS books (
	id     INTEGER PRIMARY KEY,
	title  TEXT NOT NULL,
	author TEXT NOT NULL
)`)
if err != nil {
	log.Fatal(err)
}

res, err := db.Exec("INSERT INTO books (title, author) VALUES (?, ?)",
	"Les Misérables", "Victor Hugo")
if err != nil {
	log.Fatal(err)
}
id, _ := res.LastInsertId()
n, _ := res.RowsAffected()
fmt.Println("new id:", id, "- rows added:", n)
```

```text
new id: 1 - rows added: 1
```

The `?` are **holes** filled by the values that follow, in order:

| Hole | Value |
|---|---|
| first `?` | `"Les Misérables"` |
| second `?` | `"Victor Hugo"` |

With PostgreSQL, the holes are written `$1`, `$2`, and you get the id back with
`… RETURNING id` and `QueryRow`.

### Never paste text into the SQL

Why holes, and not `"… VALUES ('" + title + "')"`? Because a visitor could type a title
like `x'); DROP TABLE books; --`. Pasted into the SQL command, this title becomes a
**command** that erases the table. This is **SQL injection** (see
[security](../debuter/securite.md)).

With `?`, the driver sends the value **separately**: it always stays plain data, even if
it contains quotes.

> **Remember:** always use `?` (or `$1`) parameters. Never `+` or `fmt.Sprintf` to build
> an SQL command with data.

## Reading one row: QueryRow

`db.QueryRow` returns **one** row. `Scan` copies each column into a variable, in the
order of the `SELECT`. You give the **address** of the variables (`&title`) so that
`Scan` can fill them (see [pointers](pointeurs.md)).

```go norun
var title, author string
err = db.QueryRow("SELECT title, author FROM books WHERE id = ?", 1).
	Scan(&title, &author)
if errors.Is(err, sql.ErrNoRows) {
	fmt.Println("no book with this id")
} else if err != nil {
	log.Fatal(err)
} else {
	fmt.Println(title, "-", author)
}
```

```text
Les Misérables - Victor Hugo
```

With the id `99`, which doesn't exist, `Scan` returns the error `sql.ErrNoRows`, and the
program prints `no book with this id`. It's not a failure: it's the answer "nothing
found".

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

## Reading several rows: Query

`db.Query` returns a cursor `rows`. You move forward row by row with `rows.Next()`, like
a finger going down a list.

```go norun
rows, err := db.Query("SELECT id, title FROM books ORDER BY id")
if err != nil {
	log.Fatal(err)
}
defer rows.Close()

for rows.Next() {
	var id int
	var title string
	if err := rows.Scan(&id, &title); err != nil {
		log.Fatal(err)
	}
	fmt.Println(id, title)
}
if err := rows.Err(); err != nil {
	log.Fatal(err)
}
```

```text
1 Les Misérables
2 Germinal
3 The Little Prince
```

| Round | `rows.Next()` | id | title |
|---|---|---|---|
| 1 | `true` | 1 | Les Misérables |
| 2 | `true` | 2 | Germinal |
| 3 | `true` | 3 | The Little Prince |
| 4 | `false`: no more rows | | |

The three rules not to forget:

1. `defer rows.Close()` right after `Query`: otherwise the connection stays busy;
2. `rows.Scan` with as many variables as there are columns;
3. `rows.Err()` after the loop: an error may have stopped the reading halfway.

> **Remember:** `QueryRow(…).Scan(…)` for one row; `Query` + `for rows.Next()` +
> `defer rows.Close()` for several.

## Hiding the database behind an interface

Putting `db.Query` calls all over the program causes two problems:

- to test, you need a real database;
- switching databases (SQLite → PostgreSQL) forces you to reread everything.

The solution: an [interface](interfaces.md) that says **what you want to do** with the
books, without saying **how**. It's often called `Store`.

It's like a left-luggage counter: you hand over a bag, you get a ticket back; with the
ticket, you get the bag back. You don't know whether the bags are kept in a cupboard or
in the basement. And you don't need to know.

```text
             main, the web server, the tests…
                          │  only use:
          ┌───────────────┴────────────────┐
          │  Store: Add  Get  List  Delete │
          └───────┬────────────────┬───────┘
                  │                │
          MemoryStore           SQLStore
     (map + lock, tests)   (database/sql, production)
```

### The interface and the in-memory version

Here is a complete program. `MemoryStore` keeps the books in a [map](maps.md),
protected by a `sync.RWMutex` lock (see [sync](sync.md)): several goroutines will be
able to use it at the same time.

```go
package main

import (
	"errors"
	"fmt"
	"sort"
	"sync"
)

type Book struct {
	ID     int
	Title  string
	Author string
}

var ErrNotFound = errors.New("book not found")

// Store says what you can do with books, not how.
type Store interface {
	Add(b Book) (int, error)
	Get(id int) (Book, error)
	List() ([]Book, error)
	Delete(id int) error
}

// MemoryStore keeps the books in a map.
type MemoryStore struct {
	mu     sync.RWMutex
	books  map[int]Book
	nextID int
}

func NewMemoryStore() *MemoryStore {
	return &MemoryStore{books: map[int]Book{}, nextID: 1}
}

func (s *MemoryStore) Add(b Book) (int, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	b.ID = s.nextID
	s.books[b.ID] = b
	s.nextID++
	return b.ID, nil
}

func (s *MemoryStore) Get(id int) (Book, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	b, ok := s.books[id]
	if !ok {
		return Book{}, fmt.Errorf("get %d: %w", id, ErrNotFound)
	}
	return b, nil
}

func (s *MemoryStore) List() ([]Book, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	list := make([]Book, 0, len(s.books))
	for _, b := range s.books {
		list = append(list, b)
	}
	// a map has no order: sort by id
	sort.Slice(list, func(i, j int) bool { return list[i].ID < list[j].ID })
	return list, nil
}

func (s *MemoryStore) Delete(id int) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if _, ok := s.books[id]; !ok {
		return fmt.Errorf("delete %d: %w", id, ErrNotFound)
	}
	delete(s.books, id)
	return nil
}

// show only knows the Store interface.
func show(s Store) {
	list, _ := s.List()
	for _, b := range list {
		fmt.Printf("  %d. %s (%s)\n", b.ID, b.Title, b.Author)
	}
}

func main() {
	var s Store = NewMemoryStore()

	id1, _ := s.Add(Book{Title: "Les Misérables", Author: "Victor Hugo"})
	id2, _ := s.Add(Book{Title: "Germinal", Author: "Émile Zola"})
	s.Add(Book{Title: "The Little Prince", Author: "Saint-Exupéry"})
	fmt.Println("added:", id1, id2)

	b, err := s.Get(2)
	fmt.Println("get 2:", b.Title, err)

	fmt.Println("list:")
	show(s)

	fmt.Println("delete 1:", s.Delete(1))
	fmt.Println("delete 1:", s.Delete(1))

	_, err = s.Get(1)
	fmt.Println("get 1:", err)
	fmt.Println("not found?", errors.Is(err, ErrNotFound))

	fmt.Println("list:")
	show(s)
}
```

```sortie
added: 1 2
get 2: Germinal <nil>
list:
  1. Les Misérables (Victor Hugo)
  2. Germinal (Émile Zola)
  3. The Little Prince (Saint-Exupéry)
delete 1: <nil>
delete 1: delete 1: book not found
get 1: get 1: book not found
not found? true
list:
  2. Germinal (Émile Zola)
  3. The Little Prince (Saint-Exupéry)
```

### Step by step

Let's follow the contents of the store at each call:

| Step | Call | nextID after | Books in the map | Returns |
|---|---|---|---|---|
| 0 | `NewMemoryStore()` | 1 | (empty) | |
| 1 | `Add(Misérables)` | 2 | 1 | `1, nil` |
| 2 | `Add(Germinal)` | 3 | 1, 2 | `2, nil` |
| 3 | `Add(Little Prince)` | 4 | 1, 2, 3 | `3, nil` |
| 4 | `Get(2)` | 4 | 1, 2, 3 | Germinal, `nil` |
| 5 | `Delete(1)` | 4 | 2, 3 | `nil` |
| 6 | `Delete(1)` | 4 | 2, 3 | error: not found |
| 7 | `Get(1)` | 4 | 2, 3 | error: not found |

Three details to notice:

- `nextID` never goes down: a deleted id is **not** reused. An SQL database does the
  same.
- `List` **sorts** the list. A Go map gives back its keys in an order that changes every
  time; without sorting, the display would change from one run to the next.
- The errors wrap `ErrNotFound` with `%w`. The caller tests
  `errors.Is(err, ErrNotFound)` without looking at the text (see [errors](erreurs.md)).

Try it: add a fourth book at the end of `main`, then a `show(s)`. It gets the id `4`,
not `1`: id 1 is not reused.

> **Remember:** the rest of the program only knows `Store`. The in-memory version is
> enough to write and test all the logic, without any database.

### Checking that a type satisfies the interface

A line placed at package level asks the compiler to check that `*MemoryStore` really
has the four methods:

```go norun
var _ Store = (*MemoryStore)(nil)
```

If one is missing, the [compilation](../debuter/glossaire.md#compilation) fails right
away, with the name of the missing method.

## The same interface, SQL version

An SQL version is another type, `SQLStore`, which holds a `*sql.DB`. Its methods have
**exactly** the same signatures. They translate each call into an SQL command.

```go norun
type SQLStore struct {
	db *sql.DB
}

var _ Store = (*SQLStore)(nil)

func (s *SQLStore) Add(b Book) (int, error) {
	res, err := s.db.Exec("INSERT INTO books (title, author) VALUES (?, ?)",
		b.Title, b.Author)
	if err != nil {
		return 0, err
	}
	id, err := res.LastInsertId()
	return int(id), err
}

func (s *SQLStore) Get(id int) (Book, error) {
	b := Book{ID: id}
	err := s.db.QueryRow("SELECT title, author FROM books WHERE id = ?", id).
		Scan(&b.Title, &b.Author)
	if errors.Is(err, sql.ErrNoRows) {
		return Book{}, fmt.Errorf("get %d: %w", id, ErrNotFound)
	}
	return b, err
}

func (s *SQLStore) List() ([]Book, error) {
	rows, err := s.db.Query("SELECT id, title, author FROM books ORDER BY id")
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var list []Book
	for rows.Next() {
		var b Book
		if err := rows.Scan(&b.ID, &b.Title, &b.Author); err != nil {
			return nil, err
		}
		list = append(list, b)
	}
	return list, rows.Err()
}

func (s *SQLStore) Delete(id int) error {
	res, err := s.db.Exec("DELETE FROM books WHERE id = ?", id)
	if err != nil {
		return err
	}
	if n, _ := res.RowsAffected(); n == 0 {
		return fmt.Errorf("delete %d: %w", id, ErrNotFound)
	}
	return nil
}
```

Compare method by method:

| Method | MemoryStore | SQLStore |
|---|---|---|
| `Add` | `s.books[id] = b`, `nextID++` | `INSERT`, then `LastInsertId` |
| `Get` | `s.books[id]` and `ok` | `SELECT … WHERE id = ?`, `ErrNoRows` |
| `List` | go through the map, then sort | `SELECT … ORDER BY id` |
| `Delete` | `delete(s.books, id)` | `DELETE`, then `RowsAffected` |
| lock | `sync.RWMutex` | not needed: `*sql.DB` is already safe |

In `main`, only one line changes:

```go norun
var s Store = NewMemoryStore()         // to try things out, for tests
var s Store = &SQLStore{db: db}        // for real, with the database
```

With SQLite, the same `main` as above prints **the same thing**: that's the whole point
of the interface. And both versions return the same `ErrNotFound` error, so the code
that uses them sees no difference.

> **Remember:** one `Store` interface, two implementations. Memory for
> [tests](tests.md), SQL for production. The rest of the program doesn't change.

## Common mistakes

### Forgetting to import the driver

```go norun
import "database/sql"

db, err := sql.Open("sqlite", "books.db")
if err != nil {
	fmt.Println(err)
	return
}
```

```text
sql: unknown driver "sqlite" (forgotten import?)
```

Add the line `_ "modernc.org/sqlite"` to the imports, and `go get` the module.

### A Scan with the wrong number of variables

```go norun
rows, _ := db.Query("SELECT id, title, author FROM books")
for rows.Next() {
	var id int
	var title string
	err := rows.Scan(&id, &title) // 3 columns, 2 variables
	fmt.Println(err)
}
```

```text
sql: expected 3 destination arguments in Scan, not 2
```

You need one variable per column of the `SELECT`, in the same order.

### A shared map without a lock

If you remove the `sync.RWMutex` from `MemoryStore` and several goroutines call `Add`
at the same time, Go stops the program:

```text
fatal error: concurrent map writes

goroutine 7 [running]:
main.(*MemoryStore).Add(…)
…
exit status 2
```

A Go map is not made to be modified by several goroutines at once. Keep the lock
(`Lock` to write, `RLock` to read).

## Test yourself

**Question 1:** what is the `_` in `import _ "modernc.org/sqlite"` for?

> **Answer**
>
> To load the package without using its names. While it loads, the driver registers
> itself with `database/sql` under the name `"sqlite"`.

**Question 2:** why write `WHERE id = ?` with a parameter rather than pasting the id
into the text of the query?

> **Answer**
>
> To avoid SQL injection: the value is sent separately and stays data, it can never
> become an SQL command.

**Question 3:** in the `MemoryStore` example, why does `List` sort the books?

> **Answer**
>
> Because a Go map gives back its elements in an unpredictable order. Sorting by id
> always gives the same list, as `ORDER BY id` would in SQL.

## Summary

- `database/sql` gives the same functions for all databases; a driver, imported with
  `_`, makes the link with one specific database.
- `Exec` to write, `QueryRow(…).Scan(…)` for one row, `Query` + `rows.Next()` +
  `defer rows.Close()` for several.
- Always `?` or `$1` parameters, never pasted text.
- A `Store` interface hides the database: an in-memory version (map + `RWMutex`) to try
  things out and test, an SQL version for production.
- `var _ Store = (*SQLStore)(nil)` checks at compile time that a type satisfies the
  interface.

Next: [regular expressions](regex.md).
