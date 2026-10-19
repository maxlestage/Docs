# What is a database?

A database stores information in tables so you can find it quickly and without mistakes: this page shows you tables, rows, columns, and the four basic SQL commands, with Python.

> **Goals:**
>
> - understand why a plain text file is not always enough;
> - read a table: its rows, its columns and its key;
> - write the SQL commands `SELECT`, `INSERT`, `UPDATE` and `DELETE`;
> - filter with `WHERE` and sort with `ORDER BY`;
> - link two tables with `JOIN`.

## Why not a plain text file?

Imagine you list your books in a text file: one line per book, with a
`;` between the title, the author and the year. It works… until the day
a title itself contains a `;`. Look:

```python
lines = [
    "The Little Prince;Saint-Exupéry;1943",
    "Paris; a user's guide;Dupont;2001",
]
for line in lines:
    pieces = line.split(";")
    print(len(pieces), "pieces:", pieces)
```

```sortie
3 pieces: ['The Little Prince', 'Saint-Exupéry', '1943']
4 pieces: ['Paris', " a user's guide", 'Dupont', '2001']
```

The second line is cut into 4 pieces instead of 3: the program thinks the
author is called "a user's guide". Other problems: to find a book, you
have to read the **whole** file, and nothing stops you from writing
"nineteen hundred" instead of a year.

A **database** solves these problems: this specialized program keeps
your data, checks it and finds it very quickly.

> **Remember:** a text file is a notebook; a database is a filing cabinet kept by a librarian.

## A table is like a spreadsheet

A database contains **tables**. A table looks like a spreadsheet sheet
(like Excel or LibreOffice Calc):

```text
Table "books"
┌────┬─────────────────────────────┬───────────────┬──────┐
│ id │ title                       │ author        │ year │  ← the columns
├────┼─────────────────────────────┼───────────────┼──────┤
│ 1  │ The Little Prince           │ Saint-Exupéry │ 1943 │  ← a row
│ 2  │ Les Misérables              │ Hugo          │ 1862 │
│ 3  │ The Hunchback of Notre-Dame │ Hugo          │ 1831 │
│ 4  │ The Stranger                │ Camus         │ 1942 │
└────┴─────────────────────────────┴───────────────┴──────┘
```

- a **column** is one kind of information (the title, the year…). Each
  column has a [type](glossaire.md#type): text, a whole number…;
- a **row** is one stored thing: here, a book;
- the **key** (here `id`) is a unique number for each row, like a social
  security number: two books can have the same title, never the same `id`.

> **Remember:** a table = columns and rows; the key points to one row without any mistake.

## SQL: the language of databases

You talk to a database in a language: **SQL** (say "S-Q-L" or "sequel").
Its sentences look like simple English:

| SQL command | In plain English     | Example                                  |
|-------------|----------------------|------------------------------------------|
| `SELECT`    | "show me"            | `SELECT title FROM books`                |
| `INSERT`    | "add"                | `INSERT INTO books (title) VALUES (…)`   |
| `UPDATE`    | "change"             | `UPDATE books SET year = 1943`           |
| `DELETE`    | "remove"             | `DELETE FROM books`                      |
| `WHERE`     | "only if…"           | `WHERE author = 'Hugo'`                  |
| `ORDER BY`  | "sorted by…"         | `ORDER BY year`                          |

To try it out, we use **SQLite**, which comes with Python: nothing to install.
With `":memory:"`, the database lives in [memory](glossaire.md#ram)
(RAM) and disappears when the program ends: perfect for practice.

## Creating a table and adding rows

Three steps: connect, create the table, add rows:

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE books (id INTEGER PRIMARY KEY, title TEXT, author TEXT, year INTEGER)")
con.execute("INSERT INTO books (title, author, year) VALUES (?, ?, ?)",
            ("The Little Prince", "Saint-Exupéry", 1943))

for row in con.execute("SELECT * FROM books"):
    print(row)
```

```sortie
(1, 'The Little Prince', 'Saint-Exupéry', 1943)
```

Let's read it line by line:

| Program line              | What happens                                             |
|---------------------------|----------------------------------------------------------|
| `sqlite3.connect(…)`      | opens an empty database                                  |
| `CREATE TABLE books (…)`  | creates the table; `id` is the key, numbered by itself   |
| `INSERT INTO … VALUES`    | adds a row                                               |
| `SELECT * FROM books`     | "show me all the columns of the books table"             |

The `?` are **placeholders**. Python slips the values into them, in
order. It is the right habit: the page on
[security](securite.md) explains why.

Try it: copy the `INSERT` with `("Les Misérables", "Hugo", 1862)`.
A second row appears, with `id` 2.

## Choosing rows with WHERE

`WHERE` keeps only some rows. Here, `executemany` adds all the books at
once:

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE books (id INTEGER PRIMARY KEY, title TEXT, author TEXT, year INTEGER)")
con.executemany("INSERT INTO books (title, author, year) VALUES (?, ?, ?)", [
    ("The Little Prince", "Saint-Exupéry", 1943), ("Les Misérables", "Hugo", 1862),
    ("The Hunchback of Notre-Dame", "Hugo", 1831), ("The Stranger", "Camus", 1942),
])

for row in con.execute("SELECT title, year FROM books WHERE author = 'Hugo'"):
    print(row)
```

```sortie
('Les Misérables', 1862)
('The Hunchback of Notre-Dame', 1831)
```

The database looks at each row and asks the question "is `author` equal
to `'Hugo'`?":

| id | author        | `author = 'Hugo'`? | Kept?    |
|----|---------------|--------------------|----------|
| 1  | Saint-Exupéry | no                 | no       |
| 2  | Hugo          | yes                | **yes**  |
| 3  | Hugo          | yes                | **yes**  |
| 4  | Camus         | no                 | no       |

In SQL, text goes between single quotes `'…'`, and equality is a single
`=` (not `==` like in Python).

Try it: replace the condition with `WHERE year > 1900`. You get
*The Little Prince* and *The Stranger*.

> **Remember:** `SELECT` chooses the columns, `WHERE` chooses the rows.

## Sorting with ORDER BY

`ORDER BY` sorts the result. Add `DESC` for the reverse order (from
largest to smallest):

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE books (id INTEGER PRIMARY KEY, title TEXT, year INTEGER)")
con.executemany("INSERT INTO books (title, year) VALUES (?, ?)", [
    ("The Little Prince", 1943), ("Les Misérables", 1862), ("The Stranger", 1942),
])

for title, year in con.execute("SELECT title, year FROM books ORDER BY year"):
    print(year, title)
```

```sortie
1862 Les Misérables
1942 The Stranger
1943 The Little Prince
```

Each row is a [tuple](glossaire.md#tuple), unpacked into two variables
by `for title, year in …`.

Try it: write `ORDER BY year DESC`. The order flips: 1943 first.

## Changing and removing: UPDATE and DELETE

`UPDATE` changes values, `DELETE` removes rows. Both are **always** used
with a `WHERE`, to target the right rows:

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE books (id INTEGER PRIMARY KEY, title TEXT, year INTEGER)")
con.executemany("INSERT INTO books (title, year) VALUES (?, ?)", [
    ("The Little Prince", 1900), ("Les Misérables", 1862), ("Duplicate book", 2000),
])

con.execute("UPDATE books SET year = ? WHERE id = ?", (1943, 1))
con.execute("DELETE FROM books WHERE id = 3")

for row in con.execute("SELECT * FROM books"):
    print(row)
```

```sortie
(1, 'The Little Prince', 1943)
(2, 'Les Misérables', 1862)
```

| id | Before                   | Command  | After                    |
|----|--------------------------|----------|--------------------------|
| 1  | The Little Prince, 1900  | `UPDATE` | The Little Prince, 1943  |
| 2  | Les Misérables, 1862     | (none)   | Les Misérables, 1862     |
| 3  | Duplicate book, 2000     | `DELETE` | (gone)                   |

> **Remember:** `UPDATE` and `DELETE` without `WHERE` affect **all** the rows.

## Linking two tables with JOIN

Writing "Hugo" on every book repeats the same information. So you store
the authors in **their own table**, and each book keeps only the
**number** of its author:

```text
   Table "authors"                Table "books"
┌────┬───────────────┐     ┌────┬─────────────────────────────┬───────────┐
│ id │ name          │     │ id │ title                       │ author_id │
├────┼───────────────┤     ├────┼─────────────────────────────┼───────────┤
│ 1  │ Saint-Exupéry │◄────┤ 1  │ The Little Prince           │ 1         │
│ 2  │ Hugo          │◄─┬──┤ 2  │ Les Misérables              │ 2         │
└────┴───────────────┘  └──┤ 3  │ The Hunchback of Notre-Dame │ 2         │
                           └────┴─────────────────────────────┴───────────┘
```

A `JOIN` glues the two tables back together: for each book, the database
looks for the author whose `id` equals `author_id`.

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE authors (id INTEGER PRIMARY KEY, name TEXT)")
con.execute("CREATE TABLE books (id INTEGER PRIMARY KEY, title TEXT, author_id INTEGER)")
con.executemany("INSERT INTO authors (name) VALUES (?)", [("Saint-Exupéry",), ("Hugo",)])
con.executemany("INSERT INTO books (title, author_id) VALUES (?, ?)", [
    ("The Little Prince", 1), ("Les Misérables", 2), ("The Hunchback of Notre-Dame", 2),
])

query = """SELECT books.title, authors.name FROM books
           JOIN authors ON books.author_id = authors.id
           ORDER BY books.title"""
for title, name in con.execute(query):
    print(f"{title} — {name}")
```

```sortie
Les Misérables — Hugo
The Hunchback of Notre-Dame — Hugo
The Little Prince — Saint-Exupéry
```

`books.title` means "the `title` column of the `books` table".

Try it: add `WHERE authors.name = 'Hugo'` just before `ORDER BY`.
Only Hugo's two books remain.

> **Remember:** each piece of information is written **only once**; `JOIN` brings them together.

## SQL, NoSQL and the tools

**SQL** databases (called "relational") store everything in tables, like
here. **NoSQL** databases store data differently, often as
[JSON](glossaire.md#json) documents: more flexible, but less strict.

To start, SQLite is enough: it is already in Python. Later, you may
install **DB Browser for SQLite** (to see your tables on screen), then
**PostgreSQL** or **MySQL**, big databases that run on a
[server](glossaire.md#server) for websites. To go further:
[SQLite in Python](../python/sqlite.md) and
[databases in Ruby](../ruby/base-de-donnees.md).

## Common mistakes

### Forgetting the WHERE in an UPDATE

You want to fix the year of a single book. But you forget the `WHERE`:

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE books (id INTEGER PRIMARY KEY, title TEXT, year INTEGER)")
con.executemany("INSERT INTO books (title, year) VALUES (?, ?)", [
    ("The Little Prince", 1900), ("Les Misérables", 1862), ("The Stranger", 1942),
])

con.execute("UPDATE books SET year = 1943")  # oops: no WHERE

for row in con.execute("SELECT * FROM books"):
    print(row)
```

```sortie
(1, 'The Little Prince', 1943)
(2, 'Les Misérables', 1943)
(3, 'The Stranger', 1943)
```

No error message: SQL did exactly what you told it. All **three** books
now have 1943. The fix:
`UPDATE books SET year = 1943 WHERE id = 1`. Tip: first run a `SELECT`
with the same `WHERE` to see which rows will be affected.

### A typo in an SQL command

```python norun
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("SELEC * FROM books")
```

```text
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    con.execute("SELEC * FROM books")
    ~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^
sqlite3.OperationalError: near "SELEC": syntax error
```

`near "SELEC"` means "close to the word `SELEC`": that is where the
database stops understanding. Fix it to `SELECT`.

### Building the SQL command by gluing text together

It is tempting to glue the user's text into the SQL command with `+`.
With a name that contains an apostrophe, everything breaks:

```python norun
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE students (id INTEGER PRIMARY KEY, name TEXT)")
name = "Liam O'Brien"
con.execute("INSERT INTO students (name) VALUES ('" + name + "')")
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    con.execute("INSERT INTO students (name) VALUES ('" + name + "')")
    ~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
sqlite3.OperationalError: near "Brien": syntax error
```

The apostrophe in "O'Brien" closes the text too early. Worse: an attacker
can write SQL instead of a name (**SQL injection**, see
[security](securite.md)). The fix, always the same, is `?`:
`con.execute("INSERT INTO students (name) VALUES (?)", (name,))`.

## Test yourself

**1.** In a table, what is the key (`id`) for?

> **Answer**
>
> To point to one row without any mistake: each row has its own key.

**2.** What does `DELETE FROM books` do without `WHERE`?

> **Answer**
>
> It removes **all** the rows of the table. You need a `WHERE` to target
> only some rows, for example `WHERE id = 3`.

**3.** Which SQL command shows the titles published after 1900, from newest to oldest?

> **Answer**
>
> `SELECT title FROM books WHERE year > 1900 ORDER BY year DESC`

## Summary

- A database stores information in tables: columns, rows, a key.
- SQL talks to the database: `SELECT` reads, `INSERT` adds, `UPDATE` changes, `DELETE` removes.
- `WHERE` chooses the rows, `ORDER BY` sorts them; without `WHERE`, everything is affected.
- `JOIN` links two tables through a shared number.
- Always pass values with `?`, never by gluing text together.

Next: [security basics](securite.md).
