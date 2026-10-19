# A database with sqlite3

This page shows how to store, find, change and delete data in a small SQL database with the `sqlite3` module, which comes with Python.

> **Goals:**
>
> - you will be able to create a table and add rows to it;
> - you will be able to read rows with `SELECT` and `fetchall`;
> - you will be able to change and delete rows;
> - you will be able to group changes in a transaction with `with conn:`;
> - you will know why you must always pass values with `?`.

## What is a database?

A database is a very tidy filing cabinet. Each drawer is a **table**. Each card is a
**row**. Each box on the card is a **column**.

```text
table "friends"
┌────┬─────────┬─────┐
│ id │ name    │ age │   ← the columns
├────┼─────────┼─────┤
│ 1  │ Alice   │ 30  │   ← a row
│ 2  │ Bob     │ 25  │   ← another row
└────┴─────────┴─────┘
```

You talk to the database in a small separate language: **SQL**. A SQL sentence is called
a **query**. It looks like English: `SELECT name FROM friends` means
"select the names in the friends table".

SQLite is a database that fits in a single file, with no server to install. Python
already knows how to talk to it thanks to the [module](../debuter/glossaire.md#module)
`sqlite3`. The page [databases](../debuter/bases-de-donnees.md) explains the general idea.

> **Note:** in this page, the database is created in
> [memory](../debuter/glossaire.md#memoire-vive) with `":memory:"`. It disappears at the
> end of the program. To keep it, give a file name:
> `sqlite3.connect("address_book.db")`.

## Opening a database and creating a table

Three steps: connect, create the table, check.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE friends (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)")
print("Table created")
```

```sortie
Table created
```

Line by line:

| Code | What it does |
|---|---|
| `sqlite3.connect(":memory:")` | opens a database and returns a **connection** |
| `conn.execute("...")` | sends a SQL query to the database |
| `CREATE TABLE friends (...)` | creates the `friends` drawer with three columns |
| `id INTEGER PRIMARY KEY` | a unique number, given automatically by the database |
| `name TEXT`, `age INTEGER` | a text column, a whole-number column |

The connection is the telephone: as long as it is open, you can talk to the
database.

## Adding a row with ?

To add a card, you use `INSERT INTO`. The values are **not** written in the text of
the query. You put a `?` in their place, and you give the values next to it, in a
[tuple](../debuter/glossaire.md#tuple).

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE friends (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)")

conn.execute("INSERT INTO friends (name, age) VALUES (?, ?)", ("Alice", 30))
conn.execute("INSERT INTO friends (name, age) VALUES (?, ?)", ("Bob", 25))

print(conn.execute("SELECT * FROM friends").fetchall())
```

```sortie
[(1, 'Alice', 30), (2, 'Bob', 25)]
```

The first `?` receives `"Alice"`, the second receives `30`. It is like a fill-in-the-blanks
form: the query is the form, the tuple fills the blanks in order.

```text
"INSERT INTO friends (name, age) VALUES (?,      ?)"
                                         ▲       ▲
                                         │       │
                                     ("Alice",   30)
```

We did not give an `id`: the database put `1`, then `2`, by itself.

> **Remember:** never paste a value into the text of the query. Always a `?`, and the
> values in a tuple next to it. The section
> [protecting yourself from SQL injection](#protecting-yourself-from-sql-injection)
> explains why.

> **Try it:** add a third row for `"Chloé"`, 41 years old. The displayed list then has
> three tuples, and Chloé has the `id` 3.

### Adding several rows at once

`executemany` runs the same query for each item of a list.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE friends (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)")

friends = [("Alice", 30), ("Bob", 25), ("Chloé", 41)]
conn.executemany("INSERT INTO friends (name, age) VALUES (?, ?)", friends)

print(conn.execute("SELECT COUNT(*) FROM friends").fetchone())
```

```sortie
(3,)
```

`COUNT(*)` counts the rows. The result is a tuple of **one** value: `(3,)`.

## Reading rows

`SELECT` chooses the rows. Then there are two ways to get them:

- `fetchall()` gives **all** the rows, in a list;
- `fetchone()` gives **the next** row only, or `None` if there are no more.

Each row arrives as a tuple.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE friends (name TEXT, age INTEGER)")
conn.executemany("INSERT INTO friends VALUES (?, ?)",
                 [("Alice", 30), ("Bob", 25), ("Chloé", 41)])

rows = conn.execute("SELECT name, age FROM friends ORDER BY age").fetchall()
for name, age in rows:
    print(name, age)
```

```sortie
Bob 25
Alice 30
Chloé 41
```

`ORDER BY age` sorts by age. Without `ORDER BY`, the database can return the rows in
any order: add it as soon as the order matters.

### Filtering with WHERE

`WHERE` keeps only the rows that pass a test. Here too, the value goes through a
`?`.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE friends (name TEXT, age INTEGER)")
conn.executemany("INSERT INTO friends VALUES (?, ?)",
                 [("Alice", 30), ("Bob", 25), ("Chloé", 41)])

cursor = conn.execute("SELECT name FROM friends WHERE age >= ? ORDER BY name", (30,))
print(cursor.fetchall())

cursor = conn.execute("SELECT age FROM friends WHERE name = ?", ("Bob",))
print(cursor.fetchone())
print(cursor.fetchone())
```

```sortie
[('Alice',), ('Chloé',)]
(25,)
None
```

The second `fetchone()` returns `None`: there was only one row.

> **Warning:** `(30,)` with a comma is a tuple of one value. `(30)` without a comma
> is just the number 30. See [the common mistakes](#common-mistakes).

## Rows with names: sqlite3.Row

With a tuple, you have to remember that the name is at position 0 and the age at
position 1. That is fragile. By setting `row_factory`, each row becomes an object you
read **by the column name**, like a [dictionary](../debuter/glossaire.md#dictionnaire).

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.row_factory = sqlite3.Row
conn.execute("CREATE TABLE friends (name TEXT, age INTEGER)")
conn.execute("INSERT INTO friends VALUES (?, ?)", ("Alice", 30))

row = conn.execute("SELECT name, age FROM friends").fetchone()
print(row["name"])
print(row["age"])
print(row.keys())
print(dict(row))
```

```sortie
Alice
30
['name', 'age']
{'name': 'Alice', 'age': 30}
```

> **Remember:** put `conn.row_factory = sqlite3.Row` right after `connect`. Your
> rows can then be read with `row["name"]`, which is much clearer than `row[0]`.

## Changing and deleting

`UPDATE` changes rows, `DELETE` removes them. Both take a `WHERE`.
`rowcount` tells you how many rows were affected.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE friends (name TEXT, age INTEGER)")
conn.executemany("INSERT INTO friends VALUES (?, ?)",
                 [("Alice", 30), ("Bob", 25), ("Chloé", 41)])

cursor = conn.execute("UPDATE friends SET age = ? WHERE name = ?", (31, "Alice"))
print("updated:", cursor.rowcount)

cursor = conn.execute("DELETE FROM friends WHERE name = ?", ("Bob",))
print("deleted:", cursor.rowcount)

print(conn.execute("SELECT * FROM friends ORDER BY name").fetchall())
```

```sortie
updated: 1
deleted: 1
[('Alice', 31), ('Chloé', 41)]
```

Let's follow the table at each step:

| Step | Query | Content of the table |
|---|---|---|
| 1 | `INSERT` ×3 | Alice 30, Bob 25, Chloé 41 |
| 2 | `UPDATE … WHERE name = 'Alice'` | Alice **31**, Bob 25, Chloé 41 |
| 3 | `DELETE … WHERE name = 'Bob'` | Alice 31, Chloé 41 |

> **Warning:** an `UPDATE` or a `DELETE` **without** `WHERE` affects **all** the
> rows. `DELETE FROM friends` empties the whole drawer. Always reread your `WHERE`.

## Transactions with with conn:

A **transaction** is a bundle of changes that succeeds **entirely** or not at all.
Example: a bank transfer. You take 50 € out of one account and add it to the other. If
the program crashes in the middle, the money must not disappear.

`with conn:` takes care of everything:

- if the block ends well, the changes are **saved** (`commit`);
- if an [exception](../debuter/glossaire.md#exception) happens, they are **cancelled**
  (`rollback`).

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE accounts (name TEXT, balance INTEGER)")
conn.executemany("INSERT INTO accounts VALUES (?, ?)", [("Alice", 100), ("Bob", 20)])
conn.commit()

def transfer(source, target, amount):
    with conn:
        conn.execute("UPDATE accounts SET balance = balance - ? WHERE name = ?", (amount, source))
        balance = conn.execute("SELECT balance FROM accounts WHERE name = ?", (source,)).fetchone()[0]
        if balance < 0:
            raise ValueError("insufficient balance")
        conn.execute("UPDATE accounts SET balance = balance + ? WHERE name = ?", (amount, target))

transfer("Alice", "Bob", 50)
print(conn.execute("SELECT * FROM accounts ORDER BY name").fetchall())

try:
    transfer("Bob", "Alice", 500)
except ValueError as error:
    print("Refused:", error)
print(conn.execute("SELECT * FROM accounts ORDER BY name").fetchall())
```

```sortie
[('Alice', 50), ('Bob', 70)]
Refused: insufficient balance
[('Alice', 50), ('Bob', 70)]
```

For the second transfer, Bob first went down to `-430`. Then the error cancelled the
whole bundle: Bob went back to `70`.

```text
transfer("Bob", "Alice", 500)
  BEGIN ─► Bob: 70 → -430 ─► balance < 0? yes ─► ValueError
                                                     │
                                       ROLLBACK ◄────┘   Bob goes back to 70
```

> **Remember:** without `commit`, the changes made in a file are lost when it is
> closed. Put your writes in a `with conn:`: it saves or cancels for you.
> The chapter [errors](erreurs.md) explains `try` and `except`.

## Protecting yourself from SQL injection

Imagine a search by name. You are tempted to paste the name into the query with an
f-string. Here is what happens if someone types a slightly special name:

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE friends (name TEXT, secret TEXT)")
conn.executemany("INSERT INTO friends VALUES (?, ?)",
                 [("Alice", "cat"), ("Bob", "1234")])

user_input = "x' OR '1'='1"
query = f"SELECT * FROM friends WHERE name = '{user_input}'"
print(query)
print(conn.execute(query).fetchall())
```

```sortie
SELECT * FROM friends WHERE name = 'x' OR '1'='1'
[('Alice', 'cat'), ('Bob', '1234')]
```

The input **changed the meaning** of the query. `OR '1'='1'` is always true: the
database returns all the rows, with all the secrets. This is a **SQL injection**, one of
the best-known attacks (see [security](../debuter/securite.md)).

With `?`, the value stays a plain value. It is never read as SQL:

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE friends (name TEXT, secret TEXT)")
conn.executemany("INSERT INTO friends VALUES (?, ?)",
                 [("Alice", "cat"), ("Bob", "1234")])

user_input = "x' OR '1'='1"
print(conn.execute("SELECT * FROM friends WHERE name = ?", (user_input,)).fetchall())
```

```sortie
[]
```

The database looks for someone who is really called `x' OR '1'='1`. Nobody: empty list.

> **Remember:** the `?` is a closed box. What you put in it cannot get out
> to become SQL code.

## A complete example: the address book

Let's put everything together in small [functions](fonctions.md).

```python
import sqlite3

def open_book():
    conn = sqlite3.connect(":memory:")
    conn.row_factory = sqlite3.Row
    conn.execute("""CREATE TABLE contacts (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        phone TEXT
    )""")
    return conn

def add(conn, name, phone):
    with conn:
        conn.execute("INSERT INTO contacts (name, phone) VALUES (?, ?)", (name, phone))

def search(conn, start):
    query = "SELECT * FROM contacts WHERE name LIKE ? ORDER BY name"
    return conn.execute(query, (start + "%",)).fetchall()

def change_phone(conn, name, phone):
    with conn:
        conn.execute("UPDATE contacts SET phone = ? WHERE name = ?", (phone, name))

def show(conn):
    for c in conn.execute("SELECT * FROM contacts ORDER BY name"):
        print(f"{c['id']}. {c['name']}: {c['phone']}")

book = open_book()
add(book, "Alice Martin", "06 11 22 33 44")
add(book, "Bob Durand", "07 55 66 77 88")
add(book, "Alain Petit", "06 99 00 11 22")
show(book)

print("--- starts with Al ---")
for c in search(book, "Al"):
    print(c["name"])

change_phone(book, "Bob Durand", "07 00 00 00 00")
print("--- after the change ---")
show(book)
```

```sortie
3. Alain Petit: 06 99 00 11 22
1. Alice Martin: 06 11 22 33 44
2. Bob Durand: 07 55 66 77 88
--- starts with Al ---
Alain Petit
Alice Martin
--- after the change ---
3. Alain Petit: 06 99 00 11 22
1. Alice Martin: 06 11 22 33 44
2. Bob Durand: 07 00 00 00 00
```

A few details:

- the `id` numbers follow the order in which contacts were added, but the display
  follows alphabetical order thanks to `ORDER BY name`;
- `NOT NULL` forbids an empty name ([`NULL`](../debuter/glossaire.md#null) is SQL's
  "nothing" value, like `None` in Python);
- `LIKE ?` with `"Al%"` looks for the names that **start** with `Al`: `%` means
  "any sequence of letters";
- you can loop over the result of `execute` directly with `for`, without `fetchall`.

> **Try it:** add a function `delete(conn, name)` with a
> `DELETE FROM contacts WHERE name = ?` inside a `with conn:`. Delete Bob, then
> show the address book: two contacts are left.

## Common mistakes

### Forgetting the comma of a one-value tuple

```python norun
import sqlite3
conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE friends (name TEXT)")
conn.execute("SELECT * FROM friends WHERE name = ?", ("Alice"))
```

```text
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    conn.execute("SELECT * FROM friends WHERE name = ?", ("Alice"))
    ~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
sqlite3.ProgrammingError: Incorrect number of bindings supplied. The current statement uses 1, and there are 5 supplied.
```

`("Alice")` without a comma is not a tuple: it is the text `"Alice"`. Python sees it as
5 letters, so 5 values for only 1 `?`. Write `("Alice",)` with a comma.

### Reading a table that does not exist

```python norun
import sqlite3
conn = sqlite3.connect(":memory:")
conn.execute("SELECT * FROM friends")
```

```text
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    conn.execute("SELECT * FROM friends")
    ~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
sqlite3.OperationalError: no such table: friends
```

The table was not created, or its name is misspelled. A `":memory:"` database is empty
every time the program starts: create the table first. For a file,
`CREATE TABLE IF NOT EXISTS` creates the table only if it is missing.

### Reading a column by its name without sqlite3.Row

```python norun
import sqlite3
conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE friends (name TEXT, age INTEGER)")
conn.execute("INSERT INTO friends VALUES (?, ?)", ("Alice", 30))
row = conn.execute("SELECT name, age FROM friends").fetchone()
print(row["name"])
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    print(row["name"])
          ~~~^^^^^^^^
TypeError: tuple indices must be integers or slices, not str
```

By default, a row is a tuple: you read it with `row[0]`. To write `row["name"]`, add
`conn.row_factory = sqlite3.Row` after `connect`.

## Test yourself

**Question 1:** why must you never build a query with an f-string?

> **Answer**
>
> Because a value typed by someone can contain SQL and change the meaning of the
> query: that is a SQL injection. With `?`, the value always stays a plain
> value.

**Question 2:** what does this program display?

```python norun
import sqlite3
conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE grades (value INTEGER)")
conn.executemany("INSERT INTO grades VALUES (?)", [(12,), (8,), (15,)])
print(conn.execute("SELECT value FROM grades WHERE value >= ? ORDER BY value", (10,)).fetchall())
```

> **Answer**
>
> ```python
> import sqlite3
> conn = sqlite3.connect(":memory:")
> conn.execute("CREATE TABLE grades (value INTEGER)")
> conn.executemany("INSERT INTO grades VALUES (?)", [(12,), (8,), (15,)])
> print(conn.execute("SELECT value FROM grades WHERE value >= ? ORDER BY value", (10,)).fetchall())
> ```
>
> ```sortie
> [(12,), (15,)]
> ```
>
> `WHERE` keeps 12 and 15, `ORDER BY` sorts them. Each row is a tuple of one value.

**Question 3:** in a `with conn:`, what happens if an exception happens in the
middle?

> **Answer**
>
> All the changes made since the start of the block are cancelled (`rollback`). The
> database goes back to how it was before the block, then the exception continues on its
> way.

## Summary

- `sqlite3.connect(":memory:")` opens a database in memory; a file name keeps it
  on the disk.
- `conn.execute(query, values)` sends SQL; `executemany` repeats it for a list.
- `SELECT` then `fetchall()` or `fetchone()` to read; `UPDATE` and `DELETE` with a
  `WHERE` to change and delete.
- `conn.row_factory = sqlite3.Row` lets you write `row["name"]`.
- `with conn:` saves the changes, or cancels them all if there is an error.
- Always `?` for values, never an f-string: that is what protects you from
  SQL injection.

Next: [analyzing data](donnees.md)
