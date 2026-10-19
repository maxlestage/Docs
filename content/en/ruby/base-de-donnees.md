# A database in Ruby

This page explains how a Ruby program stores its data in an SQLite database, and how to write it so you can change the storage without rewriting everything.

> **Goals:**
>
> - you will know what a table, a row and a column are;
> - you will know how to read Ruby code that creates, fills and queries an SQLite database;
> - you will know why we write `?` instead of pasting the values into the SQL;
> - you will know how to write a small **repository**: a class that stores and finds your data;
> - you will know how to change the storage without touching the rest of the program.

## Why a database?

With [formats](formats.md), you already know how to write your data to a JSON file.
That works very well for 10 books. For 100,000 books, it is slow: you have to read
everything again to find a single one.

A database is a **well-organized filing cabinet**. You ask for "book number 42", and it
gives it to you right away, without reading everything again.

The general idea is explained in [databases](../debuter/bases-de-donnees.md).
Here, we look at the Ruby side.

> **Remember:** a JSON file is read in full; a database answers a precise
> question.

## Tables, rows and columns

A database stores its data in **tables**. A table looks like a spreadsheet
table:

```text
Table "books"
┌────┬────────────────────────────────────────┬───────────────┐
│ id │ title                                  │ author        │   ← the columns
├────┼────────────────────────────────────────┼───────────────┤
│  1 │ The Little Prince                      │ Saint-Exupéry │   ← a row
│  2 │ Twenty Thousand Leagues Under the Seas │ Verne         │   ← another row
└────┴────────────────────────────────────────┴───────────────┘
```

- a **column** is a kind of information (the title, the author);
- a **row** is a complete record (one book);
- the **id** is a unique number: it points to a row with no risk of mistake.

You talk to the database in a separate language: **SQL**. Four sentences are enough to
get started:

| SQL | In plain English |
|---|---|
| `INSERT INTO books ...` | add a row |
| `SELECT * FROM books` | give me the rows |
| `UPDATE books SET ...` | change some rows |
| `DELETE FROM books WHERE id = 1` | delete row number 1 |

## SQLite with the sqlite3 gem

**SQLite** is a database that fits in **a single file**. No server to
install: it is perfect for learning. Ruby talks to it through a
[gem](../debuter/glossaire.md#gem) (a Ruby package):

```sh
gem install sqlite3
```

The examples in this section need this gem: they are shown without being
run, with their output.

### Creating a table

```ruby norun
require "sqlite3"

db = SQLite3::Database.new("library.db")
db.execute(<<~SQL)
  CREATE TABLE IF NOT EXISTS books (
    id     INTEGER PRIMARY KEY,
    title  TEXT NOT NULL,
    author TEXT NOT NULL
  )
SQL
puts "Table ready"
```

```text
Table ready
```

Line by line:

1. `require "sqlite3"` loads the gem;
2. `Database.new` opens the file `library.db` (it is created if it does not exist);
3. `CREATE TABLE` describes the columns; `IF NOT EXISTS` avoids an error if the table
   already exists;
4. `INTEGER PRIMARY KEY`: SQLite itself gives an `id` to each new row.

### Adding rows with ?

```ruby norun
require "sqlite3"

db = SQLite3::Database.new("library.db")
db.execute("INSERT INTO books (title, author) VALUES (?, ?)",
           ["The Little Prince", "Saint-Exupéry"])
db.execute("INSERT INTO books (title, author) VALUES (?, ?)",
           ["Twenty Thousand Leagues Under the Seas", "Verne"])
puts db.last_insert_row_id
```

```text
2
```

Each `?` is a **placeholder**. The gem puts the values of the array there, in
order, and protects them. **Never** paste the values into the SQL text yourself
(see [Common mistakes](#common-mistakes)).

### Reading rows

```ruby norun
require "sqlite3"

db = SQLite3::Database.new("library.db")
db.results_as_hash = true

db.execute("SELECT * FROM books").each do |book|
  puts "#{book["id"]}. #{book["title"]} (#{book["author"]})"
end

book = db.get_first_row("SELECT * FROM books WHERE id = ?", [2])
puts book["title"]
```

```text
1. The Little Prince (Saint-Exupéry)
2. Twenty Thousand Leagues Under the Seas (Verne)
Twenty Thousand Leagues Under the Seas
```

`results_as_hash = true` gives each row as a hash: you read a column by
its name. The keys are **strings** (`"title"`), not symbols.

> **Remember:** `execute` sends SQL; the `?` receive the values; each row
> comes back as an array or a hash.

## ActiveRecord in a nutshell

In a Ruby on Rails application, you rarely write SQL by hand. **ActiveRecord**
turns each table into a Ruby [class](../debuter/glossaire.md#classe), and writes the SQL
for you:

```ruby norun
class Book < ActiveRecord::Base
end

Book.create(title: "The Little Prince", author: "Saint-Exupéry")
puts Book.count
puts Book.find(1).title
```

```text
1
The Little Prince
```

It is handy, but the idea stays the same: **add, find, list, delete**. You are
going to write these four actions yourself.

## The repository: a storage box

Here is the problem. If `db.execute(...)` is written all over your program, changing
the database becomes a nightmare: you have to read everything again.

The solution: put all the storage in **a single class**, the **repository**. The rest
of the program only talks to the repository, never to the database.

```text
   your program                 the repository               the storage
 ┌─────────────────┐   add      ┌────────────────┐         ┌───────────────┐
 │ display,        │ ─────────► │ add            │ ──────► │ a Hash        │
 │ compute,        │   find     │ find           │         │ or a file     │
 │ menus…          │ ◄───────── │ all            │ ◄────── │ or SQLite     │
 └─────────────────┘            │ delete         │         └───────────────┘
                                └────────────────┘
```

It is like a library desk: you ask for a book at the desk. You do not
know whether the books are in the basement or in the attic, and you do not need to
know.

> **Note:** the French word for it, *dépôt*, also means a Git repository. Do not confuse
> it with a [Git repository](../debuter/glossaire.md#depot), which is something else.

### An in-memory repository

Let's start without a database: the repository stores the books in a hash. The key is
the id, the value is the book.

```ruby
class BookRepository
  def initialize
    @books = {}
    @next_id = 1
  end

  def add(title, author)
    book = { id: @next_id, title: title, author: author }
    @books[@next_id] = book
    @next_id += 1
    book
  end

  def find(id)
    @books[id]
  end

  def all
    @books.values
  end

  def delete(id)
    !@books.delete(id).nil?
  end
end

repo = BookRepository.new
repo.add("The Little Prince", "Saint-Exupéry")
repo.add("Twenty Thousand Leagues Under the Seas", "Verne")

repo.all.each do |book|
  puts "#{book[:id]}. #{book[:title]} (#{book[:author]})"
end
puts repo.find(2)[:title]
p repo.find(99)
```

```sortie
1. The Little Prince (Saint-Exupéry)
2. Twenty Thousand Leagues Under the Seas (Verne)
Twenty Thousand Leagues Under the Seas
nil
```

The four methods, one by one:

| Method | What it does | What it returns |
|---|---|---|
| `add` | creates a record with a new id | the created record |
| `find(id)` | looks for the record by its id | the record, or `nil` |
| `all` | lists all the records | an array |
| `delete(id)` | removes the record | `true` if it existed |

Let's follow `@next_id` and `@books` during the two additions:

| Step | @next_id | keys of @books |
|---|---|---|
| after `new` | 1 | (none) |
| after the 1st `add` | 2 | 1 |
| after the 2nd `add` | 3 | 1, 2 |

> **Try it:** add a third book, then print `repo.all.size`. You
> get `3`, and the new book has the id `3`.

### Testing the repository

The whole program relies on the repository: let's check that it works. No tool needed: a
small `check` method prints `OK` or `FAIL`. (For real tests, see
[testing](tests.md).)

```ruby
class BookRepository
  def initialize
    @books = {}
    @next_id = 1
  end

  def add(title, author)
    book = { id: @next_id, title: title, author: author }
    @books[@next_id] = book
    @next_id += 1
    book
  end

  def find(id) = @books[id]
  def all = @books.values
  def delete(id) = !@books.delete(id).nil?
end

def check(name, condition)
  puts "#{condition ? 'OK  ' : 'FAIL'} #{name}"
end

repo = BookRepository.new
check("a new repository is empty", repo.all.empty?)

book = repo.add("Les Misérables", "Hugo")
check("the first id is 1", book[:id] == 1)
check("we find the book again", repo.find(1)[:title] == "Les Misérables")
check("an unknown id gives nil", repo.find(42).nil?)

check("delete a book that exists", repo.delete(1) == true)
check("delete twice", repo.delete(1) == false)
check("the repository is empty again", repo.all.empty?)
```

```sortie
OK   a new repository is empty
OK   the first id is 1
OK   we find the book again
OK   an unknown id gives nil
OK   delete a book that exists
OK   delete twice
OK   the repository is empty again
```

Here, the short methods are written on one line with `def name = ...` (see
[the one-line method](fonctions.md#the-one-line-method)).

> **Remember:** a repository has few methods, always the same ones: add, find,
> list, delete.

## Changing the storage

Here is the point of the repository. Let's write a **second** repository, which keeps the
books in a JSON file. It has **the same methods**. The `show_catalog` method works with
both, without changing a line.

```ruby
require "json"

class MemoryRepository
  def initialize
    @books = {}
  end

  def add(title)
    id = @books.size + 1
    @books[id] = { "id" => id, "title" => title }
  end

  def all = @books.values
end

class JsonRepository
  def initialize(path)
    @path = path
    File.write(@path, "[]") unless File.exist?(@path)
  end

  def add(title)
    books = all
    book = { "id" => books.size + 1, "title" => title }
    File.write(@path, JSON.generate(books + [book]))
    book
  end

  def all = JSON.parse(File.read(@path))
end

def show_catalog(repo)
  repo.add("The Little Prince")
  repo.add("Les Misérables")
  repo.all.each { |book| puts "  #{book["id"]}. #{book["title"]}" }
end

puts "In memory:"
show_catalog(MemoryRepository.new)
puts "In a file:"
show_catalog(JsonRepository.new("books.json"))
puts File.read("books.json")
```

```sortie
In memory:
  1. The Little Prince
  2. Les Misérables
In a file:
  1. The Little Prince
  2. Les Misérables
[{"id":1,"title":"The Little Prince"},{"id":2,"title":"Les Misérables"}]
```

`show_catalog` does not know where the books go. It only asks for
`add` and `all`. Ruby does not check the class of `repo`: it is enough that the
methods exist.

The day you want SQLite, you write a third repository with the same methods:

```ruby norun
require "sqlite3"

class SqliteRepository
  def initialize(path)
    @db = SQLite3::Database.new(path)
    @db.results_as_hash = true
    @db.execute("CREATE TABLE IF NOT EXISTS books (id INTEGER PRIMARY KEY, title TEXT)")
  end

  def add(title)
    @db.execute("INSERT INTO books (title) VALUES (?)", [title])
    { "id" => @db.last_insert_row_id, "title" => title }
  end

  def all = @db.execute("SELECT id, title FROM books")
end

show_catalog(SqliteRepository.new("library.db"))
```

```text
  1. The Little Prince
  2. Les Misérables
```

The rest of the program does not change.

> **Remember:** start with an in-memory repository. When you need a real
> database, you will change **a single class**.

## Common mistakes

### The sqlite3 gem is not installed

```ruby norun
require "sqlite3"
```

```text
<internal:/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/rubygems/core_ext/kernel_require.rb>:136:in `require': cannot load such file -- sqlite3 (LoadError)
	from <internal:/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/rubygems/core_ext/kernel_require.rb>:136:in `require'
	from main.rb:1:in `<main>'
```

`cannot load such file`: Ruby cannot find the gem. Install it with
`gem install sqlite3`, or add `gem "sqlite3"` to the project's `Gemfile`.

### Pasting the values into the SQL

A user types a title with an apostrophe… or worse:

```ruby
title = "x'); DELETE FROM books; --"
puts "INSERT INTO books (title) VALUES ('#{title}')"
```

```sortie
INSERT INTO books (title) VALUES ('x'); DELETE FROM books; --')
```

The user's text has become **a command**: it would erase all the books.
This is an **SQL injection** (see [security](../debuter/securite.md)). The fix:
always use `?`, with the values on the side: `db.execute("... VALUES (?)", [title])`.

### Forgetting that find can return nil

```ruby norun
book = repo.find(42)
puts book[:title]
```

```text
main.rb:2:in `<main>': undefined method `[]' for nil (NoMethodError)

puts book[:title]
         ^^^^^^^^
```

No book has the id 42: `find` returns `nil`. Check before using it:
`puts book[:title] if book`, or `puts book&.dig(:title) || "Book not found"`.

## Test yourself

**Question 1:** why do we write `VALUES (?, ?)` instead of putting the values
directly in the SQL text?

> **Answer**
>
> So that the gem protects the values. Otherwise, a text typed by a user can
> become an SQL command (SQL injection).

**Question 2:** what does `find` return in our repository when the id does not exist?

> **Answer**
>
> `nil`, because that is what a hash returns for a missing key.
>
> ```ruby
> books = { 1 => { title: "The Little Prince" } }
> p books[7]
> ```
>
> ```sortie
> nil
> ```

**Question 3:** you switch from an in-memory repository to an SQLite repository. What do
you need to change in the rest of the program?

> **Answer**
>
> Only the line that **creates** the repository (`SqliteRepository.new(...)` instead of
> `BookRepository.new`). The rest calls the same methods: `add`, `find`, `all`,
> `delete`.

## Summary

- A database stores data in **tables**: columns (the pieces of information) and
  rows (the records), each one with an **id**.
- SQLite fits in a file; the `sqlite3` gem sends it SQL with `execute`.
- Values always go through `?`, never pasted into the SQL text.
- A **repository** groups all the storage: add, find, list, delete.
- Start with an in-memory repository (a hash); change the storage by changing a
  single class.

Next: [regular expressions](regex.md)
