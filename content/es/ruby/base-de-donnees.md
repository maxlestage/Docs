# Una base de datos en Ruby

Esta página explica cómo un programa Ruby guarda sus datos en una base de datos SQLite, y cómo escribirlo para poder cambiar de almacenamiento sin reescribirlo todo.

> **Objetivos:**
>
> - sabrás qué son una tabla, una fila y una columna;
> - sabrás leer código Ruby que crea, llena y consulta una base SQLite;
> - sabrás por qué se escribe `?` en vez de pegar los valores en el SQL;
> - sabrás escribir un pequeño **repositorio**: una clase que guarda y encuentra tus datos;
> - sabrás cambiar de almacenamiento sin tocar el resto del programa.

## ¿Por qué una base de datos?

Con [los formatos](formats.md), ya sabes escribir tus datos en un archivo JSON.
Funciona muy bien para 10 libros. Para 100 000 libros, es lento: hay que releerlo
todo para encontrar uno solo.

Una base de datos es un **fichero de fichas bien ordenado**. Pides «el libro número
42», y te lo da enseguida, sin releerlo todo.

La idea general se explica en [las bases de datos](../debuter/bases-de-donnees.md).
Aquí vemos el lado de Ruby.

> **Recuerda:** un archivo JSON se lee entero; una base de datos responde a una
> pregunta precisa.

## Tablas, filas y columnas

Una base guarda sus datos en **tablas**. Una tabla se parece a una tabla de hoja de
cálculo:

```text
Tabla «libros»
┌────┬──────────────────────────────────────┬───────────────┐
│ id │ titulo                               │ autor         │   ← las columnas
├────┼──────────────────────────────────────┼───────────────┤
│  1 │ El principito                        │ Saint-Exupéry │   ← una fila
│  2 │ Veinte mil leguas de viaje submarino │ Verne         │   ← otra fila
└────┴──────────────────────────────────────┴───────────────┘
```

- una **columna** es un tipo de información (el título, el autor);
- una **fila** es una ficha completa (un libro);
- el **id** es un número único: señala una fila sin riesgo de error.

Se habla con la base en un lenguaje aparte: **SQL**. Cuatro frases bastan para
empezar:

| SQL | En español |
|---|---|
| `INSERT INTO libros ...` | añade una fila |
| `SELECT * FROM libros` | dame las filas |
| `UPDATE libros SET ...` | modifica filas |
| `DELETE FROM libros WHERE id = 1` | borra la fila número 1 |

## SQLite con la gema sqlite3

**SQLite** es una base de datos que cabe en **un solo archivo**. No hay servidor que
instalar: es perfecta para aprender. Ruby habla con ella gracias a una
[gema](../debuter/glossaire.md#gem) (un paquete de Ruby):

```sh
gem install sqlite3
```

Los ejemplos de esta sección necesitan esta gema: se muestran sin
ejecutarlos, con su salida.

### Crear una tabla

```ruby norun
require "sqlite3"

db = SQLite3::Database.new("biblioteca.db")
db.execute(<<~SQL)
  CREATE TABLE IF NOT EXISTS libros (
    id     INTEGER PRIMARY KEY,
    titulo TEXT NOT NULL,
    autor  TEXT NOT NULL
  )
SQL
puts "Tabla lista"
```

```text
Tabla lista
```

Línea a línea:

1. `require "sqlite3"` carga la gema;
2. `Database.new` abre el archivo `biblioteca.db` (se crea si no existe);
3. `CREATE TABLE` describe las columnas; `IF NOT EXISTS` evita un error si la tabla
   ya existe;
4. `INTEGER PRIMARY KEY`: SQLite da él mismo un `id` a cada fila nueva.

### Añadir filas con ?

```ruby norun
require "sqlite3"

db = SQLite3::Database.new("biblioteca.db")
db.execute("INSERT INTO libros (titulo, autor) VALUES (?, ?)",
           ["El principito", "Saint-Exupéry"])
db.execute("INSERT INTO libros (titulo, autor) VALUES (?, ?)",
           ["Veinte mil leguas de viaje submarino", "Verne"])
puts db.last_insert_row_id
```

```text
2
```

Cada `?` es un **hueco reservado**. La gema pone allí los valores del array, en
orden, protegiéndolos. **Nunca** pegues tú mismo los valores en el texto SQL
(mira [Errores frecuentes](#errores-frecuentes)).

### Leer filas

```ruby norun
require "sqlite3"

db = SQLite3::Database.new("biblioteca.db")
db.results_as_hash = true

db.execute("SELECT * FROM libros").each do |libro|
  puts "#{libro["id"]}. #{libro["titulo"]} (#{libro["autor"]})"
end

libro = db.get_first_row("SELECT * FROM libros WHERE id = ?", [2])
puts libro["titulo"]
```

```text
1. El principito (Saint-Exupéry)
2. Veinte mil leguas de viaje submarino (Verne)
Veinte mil leguas de viaje submarino
```

`results_as_hash = true` da cada fila en forma de hash: se lee una columna por
su nombre. Las claves son **textos** (`"titulo"`), no símbolos.

> **Recuerda:** `execute` envía SQL; los `?` reciben los valores; cada fila
> vuelve en forma de array o de hash.

## ActiveRecord en dos palabras

En una aplicación Ruby on Rails, rara vez se escribe SQL a mano. **ActiveRecord**
convierte cada tabla en una [clase](../debuter/glossaire.md#classe) Ruby, y escribe el SQL
por ti:

```ruby norun
class Libro < ActiveRecord::Base
end

Libro.create(titulo: "El principito", autor: "Saint-Exupéry")
puts Libro.count
puts Libro.find(1).titulo
```

```text
1
El principito
```

Es práctico, pero la idea sigue siendo la misma: **añadir, buscar, listar, borrar**.
Esas cuatro acciones las vas a escribir tú mismo.

## El repositorio: una caja para guardar

Este es el problema. Si `db.execute(...)` está escrito por todas partes en tu programa,
cambiar de base se vuelve una pesadilla: hay que releerlo todo.

La solución: poner todo el almacenamiento en **una sola clase**, el **repositorio**. El
resto del programa solo habla con el repositorio, nunca con la base.

```text
   tu programa                   el repositorio              el almacenamiento
 ┌─────────────────┐  agregar   ┌────────────────┐         ┌───────────────┐
 │ mostrar,        │ ─────────► │ agregar        │ ──────► │ un Hash       │
 │ calcular,       │  buscar    │ buscar         │         │ o un archivo  │
 │ menús…          │ ◄───────── │ todos          │ ◄────── │ o SQLite      │
 └─────────────────┘            │ eliminar       │         └───────────────┘
                                └────────────────┘
```

Es como el mostrador de una biblioteca: pides un libro en el mostrador. No sabes
si los libros están en el sótano o en el desván, y no necesitas
saberlo.

> **Nota:** en inglés se dice *repository*. No lo confundas con el
> [repositorio Git](../debuter/glossaire.md#depot), que es otra cosa.

### Un repositorio en memoria

Empecemos sin base de datos: el repositorio guarda los libros en un hash. La clave es
el id, el valor es el libro.

```ruby
class RepositorioLibros
  def initialize
    @libros = {}
    @proximo_id = 1
  end

  def agregar(titulo, autor)
    libro = { id: @proximo_id, titulo: titulo, autor: autor }
    @libros[@proximo_id] = libro
    @proximo_id += 1
    libro
  end

  def buscar(id)
    @libros[id]
  end

  def todos
    @libros.values
  end

  def eliminar(id)
    !@libros.delete(id).nil?
  end
end

repo = RepositorioLibros.new
repo.agregar("El principito", "Saint-Exupéry")
repo.agregar("Veinte mil leguas de viaje submarino", "Verne")

repo.todos.each do |libro|
  puts "#{libro[:id]}. #{libro[:titulo]} (#{libro[:autor]})"
end
puts repo.buscar(2)[:titulo]
p repo.buscar(99)
```

```sortie
1. El principito (Saint-Exupéry)
2. Veinte mil leguas de viaje submarino (Verne)
Veinte mil leguas de viaje submarino
nil
```

Los cuatro métodos, uno por uno:

| Método | Lo que hace | Lo que devuelve |
|---|---|---|
| `agregar` | crea una ficha con un id nuevo | la ficha creada |
| `buscar(id)` | busca la ficha por su id | la ficha, o `nil` |
| `todos` | lista todas las fichas | un array |
| `eliminar(id)` | quita la ficha | `true` si existía |

Sigamos `@proximo_id` y `@libros` durante los dos `agregar`:

| Paso | @proximo_id | claves de @libros |
|---|---|---|
| después de `new` | 1 | (ninguna) |
| después del 1.er `agregar` | 2 | 1 |
| después del 2.º `agregar` | 3 | 1, 2 |

> **Pruébalo:** agrega un tercer libro y luego muestra `repo.todos.size`.
> Obtienes `3`, y el libro nuevo tiene el id `3`.

### Probar el repositorio

Todo el programa usa el repositorio: comprobemos que funciona. No hace falta ninguna
herramienta: un pequeño método `comprobar` muestra `OK` o `FALLO`. (Para pruebas de
verdad, mira [las pruebas](tests.md).)

```ruby
class RepositorioLibros
  def initialize
    @libros = {}
    @proximo_id = 1
  end

  def agregar(titulo, autor)
    libro = { id: @proximo_id, titulo: titulo, autor: autor }
    @libros[@proximo_id] = libro
    @proximo_id += 1
    libro
  end

  def buscar(id) = @libros[id]
  def todos = @libros.values
  def eliminar(id) = !@libros.delete(id).nil?
end

def comprobar(nombre, condicion)
  puts "#{condicion ? 'OK   ' : 'FALLO'} #{nombre}"
end

repo = RepositorioLibros.new
comprobar("un repositorio nuevo está vacío", repo.todos.empty?)

libro = repo.agregar("Los miserables", "Hugo")
comprobar("el primer id vale 1", libro[:id] == 1)
comprobar("se encuentra el libro", repo.buscar(1)[:titulo] == "Los miserables")
comprobar("un id desconocido da nil", repo.buscar(42).nil?)

comprobar("eliminar un libro presente", repo.eliminar(1) == true)
comprobar("eliminar dos veces", repo.eliminar(1) == false)
comprobar("el repositorio vuelve a estar vacío", repo.todos.empty?)
```

```sortie
OK    un repositorio nuevo está vacío
OK    el primer id vale 1
OK    se encuentra el libro
OK    un id desconocido da nil
OK    eliminar un libro presente
OK    eliminar dos veces
OK    el repositorio vuelve a estar vacío
```

Aquí, los métodos cortos se escriben en una línea con `def nombre = ...` (mira
[el método en una línea](fonctions.md#el-metodo-en-una-linea)).

> **Recuerda:** un repositorio tiene pocos métodos, siempre los mismos: agregar,
> buscar, listar, eliminar.

## Cambiar de almacenamiento

Esta es la ventaja del repositorio. Escribamos un **segundo** repositorio, que guarda los
libros en un archivo JSON. Tiene **los mismos métodos**. El método `mostrar_catalogo`
funciona con los dos, sin cambiar ni una línea.

```ruby
require "json"

class RepositorioMemoria
  def initialize
    @libros = {}
  end

  def agregar(titulo)
    id = @libros.size + 1
    @libros[id] = { "id" => id, "titulo" => titulo }
  end

  def todos = @libros.values
end

class RepositorioJson
  def initialize(ruta)
    @ruta = ruta
    File.write(@ruta, "[]") unless File.exist?(@ruta)
  end

  def agregar(titulo)
    libros = todos
    libro = { "id" => libros.size + 1, "titulo" => titulo }
    File.write(@ruta, JSON.generate(libros + [libro]))
    libro
  end

  def todos = JSON.parse(File.read(@ruta))
end

def mostrar_catalogo(repo)
  repo.agregar("El principito")
  repo.agregar("Los miserables")
  repo.todos.each { |libro| puts "  #{libro["id"]}. #{libro["titulo"]}" }
end

puts "En memoria:"
mostrar_catalogo(RepositorioMemoria.new)
puts "En un archivo:"
mostrar_catalogo(RepositorioJson.new("libros.json"))
puts File.read("libros.json")
```

```sortie
En memoria:
  1. El principito
  2. Los miserables
En un archivo:
  1. El principito
  2. Los miserables
[{"id":1,"titulo":"El principito"},{"id":2,"titulo":"Los miserables"}]
```

`mostrar_catalogo` no sabe adónde van los libros. Solo pide
`agregar` y `todos`. Ruby no comprueba la clase de `repo`: basta con que los
métodos existan.

El día que quieras SQLite, escribes un tercer repositorio con los mismos métodos:

```ruby norun
require "sqlite3"

class RepositorioSqlite
  def initialize(ruta)
    @db = SQLite3::Database.new(ruta)
    @db.results_as_hash = true
    @db.execute("CREATE TABLE IF NOT EXISTS libros (id INTEGER PRIMARY KEY, titulo TEXT)")
  end

  def agregar(titulo)
    @db.execute("INSERT INTO libros (titulo) VALUES (?)", [titulo])
    { "id" => @db.last_insert_row_id, "titulo" => titulo }
  end

  def todos = @db.execute("SELECT id, titulo FROM libros")
end

mostrar_catalogo(RepositorioSqlite.new("biblioteca.db"))
```

```text
  1. El principito
  2. Los miserables
```

El resto del programa no cambia.

> **Recuerda:** empieza con un repositorio en memoria. Cuando necesites una base de
> verdad, cambiarás **una sola clase**.

## Errores frecuentes

### La gema sqlite3 no está instalada

```ruby norun
require "sqlite3"
```

```text
<internal:/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/rubygems/core_ext/kernel_require.rb>:136:in `require': cannot load such file -- sqlite3 (LoadError)
	from <internal:/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/rubygems/core_ext/kernel_require.rb>:136:in `require'
	from main.rb:1:in `<main>'
```

`cannot load such file`: Ruby no encuentra la gema. Instálala con
`gem install sqlite3`, o añade `gem "sqlite3"` al `Gemfile` del proyecto.

### Pegar los valores en el SQL

Un usuario escribe un título con un apóstrofo… o algo peor:

```ruby
titulo = "x'); DELETE FROM libros; --"
puts "INSERT INTO libros (titulo) VALUES ('#{titulo}')"
```

```sortie
INSERT INTO libros (titulo) VALUES ('x'); DELETE FROM libros; --')
```

El texto del usuario se ha convertido en **una orden**: borraría todos los libros.
Es una **inyección SQL** (mira [la seguridad](../debuter/securite.md)). La corrección:
siempre `?`, y los valores aparte: `db.execute("... VALUES (?)", [titulo])`.

### Olvidar que buscar puede devolver nil

```ruby norun
libro = repo.buscar(42)
puts libro[:titulo]
```

```text
main.rb:2:in `<main>': undefined method `[]' for nil (NoMethodError)

puts libro[:titulo]
          ^^^^^^^^^
```

Ningún libro tiene el id 42: `buscar` devuelve `nil`. Compruébalo antes de usarlo:
`puts libro[:titulo] if libro`, o `puts libro&.dig(:titulo) || "Libro no encontrado"`.

## Ponte a prueba

**Pregunta 1:** ¿por qué se escribe `VALUES (?, ?)` en vez de poner los valores
directamente en el texto SQL?

> **Respuesta**
>
> Para que la gema proteja los valores. Si no, un texto escrito por un usuario puede
> convertirse en una orden SQL (inyección SQL).

**Pregunta 2:** ¿qué devuelve `buscar` en nuestro repositorio cuando el id no existe?

> **Respuesta**
>
> `nil`, porque es lo que devuelve un hash para una clave que no está.
>
> ```ruby
> libros = { 1 => { titulo: "El principito" } }
> p libros[7]
> ```
>
> ```sortie
> nil
> ```

**Pregunta 3:** pasas de un repositorio en memoria a un repositorio SQLite. ¿Qué hay que
cambiar en el resto del programa?

> **Respuesta**
>
> Solo la línea que **crea** el repositorio (`RepositorioSqlite.new(...)` en vez de
> `RepositorioLibros.new`). El resto llama a los mismos métodos: `agregar`, `buscar`,
> `todos`, `eliminar`.

## En resumen

- Una base guarda los datos en **tablas**: columnas (las informaciones) y filas (las
  fichas), cada una con un **id**.
- SQLite cabe en un archivo; la gema `sqlite3` le envía SQL con `execute`.
- Los valores pasan siempre por `?`, nunca pegados en el texto SQL.
- Un **repositorio** reúne todo el almacenamiento: agregar, buscar, listar, eliminar.
- Empieza con un repositorio en memoria (un hash); cambia de almacenamiento cambiando
  una sola clase.

Siguiente: [las expresiones regulares](regex.md)
