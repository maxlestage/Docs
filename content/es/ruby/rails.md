# Un primer sitio con Rails, paso a paso

Esta página te guía en un primer sitio Ruby on Rails: las rutas, un controlador, una vista, un modelo y luego el viaje completo de una petición, paso a paso.

> **Objetivos:**
>
> - entender el papel de cada pieza de Rails gracias a la imagen del restaurante;
> - crear un proyecto con `rails new` y arrancar el servidor;
> - escribir una ruta, un controlador y una vista ERB;
> - crear un modelo y su migración, y luego generarlo todo de golpe con un *scaffold*;
> - seguir una petición desde el navegador hasta la página mostrada.

## ¿Qué es Rails?

**Ruby on Rails** (se suele decir «Rails») es un *framework* web. Un *framework* es un
marco de trabajo: una casa ya construida, con las habitaciones ordenadas. Tú pones los
muebles; no echas los cimientos.

La página [Ruby para la web](web.md) muestra Rack y Sinatra. Rails también se apoya en
Rack, pero aporta mucho más: base de datos, formularios, seguridad, pruebas.

Rails ordena el código en tres papeles, llamados **MVC**: Modelo, Vista, Controlador.

### El restaurante Rails

Imagina un restaurante:

```text
   Tú (el navegador)
        |
        |  «Quisiera la lista de artículos»        (la petición)
        v
  +-------------+  «3 artículos, por favor»  +-------------+
  |  CAMARERO   | -------------------------> |  COCINA     |
  | controlador | <------------------------- |  modelo     |
  +-------------+     los 3 artículos        +-------------+
        |
        |  emplata siguiendo la carta
        v
  +-------------+
  |  CARTA /    |
  |  PLATO      |   la vista: lo que ves
  +-------------+
        |
        v
   Recibes la página HTML                    (la respuesta)
```

| Restaurante | Rails | Carpeta | Su trabajo |
|-------------|-------|---------|------------|
| el plano del comedor | las **rutas** | `config/routes.rb` | decir qué pedido va a qué camarero |
| el camarero | el **controlador** | `app/controllers` | tomar el pedido, hablar con la cocina |
| la cocina | el **modelo** | `app/models` | preparar los datos (la base de datos) |
| la carta y el plato | la **vista** | `app/views` | presentar el resultado al cliente |

El camarero nunca cocina. La cocina nunca habla con el cliente. Cada uno a lo suyo:
eso es lo que hace que un sitio grande sea fácil de ordenar.

### El restaurante en Ruby normal

Esta es la idea en Ruby puro, sin Rails. Cada papel es una pequeña
[clase](../debuter/glossaire.md#clase).

```ruby
class Cocina # el modelo
  def self.platos = ["Sopa", "Gratinado", "Tarta"]
end

class Carta # la vista
  def self.mostrar(platos) = platos.map { |p| "- #{p}" }.join("\n")
end

class Camarero # el controlador
  def pedido = Carta.mostrar(Cocina.platos)
end

puts Camarero.new.pedido
```

```sortie
- Sopa
- Gratinado
- Tarta
```

Prueba: añade `"Ensalada"` en `Cocina.platos`. Solo cambia el modelo y, sin embargo, la
página muestra una línea más. El camarero y la carta no se han enterado de nada.

> **Recuerda:** MVC = tres papeles separados. El controlador recibe la petición, el
> modelo da los datos y la vista los presenta.

## Instalar Rails y crear un proyecto

Rails es una [gema](../debuter/glossaire.md#gem) (una biblioteca de Ruby). Estos
comandos se escriben en el [terminal](../debuter/glossaire.md#terminal). Necesitan
Internet: aquí no se ejecutan.

```sh
gem install rails
rails new blog
```

```text
Successfully installed rails-8.0.2
      create  README.md
      create  app/controllers/application_controller.rb
      ...
Bundle complete! 22 Gemfile dependencies, 108 gems now installed.
```

Los números de versión pueden ser distintos en tu equipo. `rails new` construye la casa: decenas de carpetas y archivos. Estas son las
habitaciones que importan para empezar:

```text
blog/
├── app/
│   ├── controllers/   ← los camareros (controladores)
│   ├── models/        ← la cocina (modelos)
│   └── views/         ← las cartas (vistas ERB)
├── config/
│   └── routes.rb      ← el plano del comedor (rutas)
├── db/
│   └── migrate/       ← los planos de las tablas
└── Gemfile            ← la lista de gemas (ver projet.md)
```

### Arrancar el servidor

```sh
cd blog
bin/rails server
```

```text
=> Rails 8.0.2 application starting in development
Puma starting in single mode...
* Listening on http://127.0.0.1:3000
Use Ctrl-C to stop
```

Abre `http://localhost:3000` en tu navegador: Rails muestra su página de bienvenida.
`Ctrl+C` en el terminal detiene el servidor.

Prueba: ejecuta `bin/rails server -p 4000`. El sitio responde entonces en el puerto
`4000` en lugar de `3000`.

> **Recuerda:** `rails new nombre` crea el proyecto; `bin/rails server` lo arranca en
> `http://localhost:3000`.

## Las rutas: el plano del comedor

Una **ruta** une una dirección ([URL](../debuter/glossaire.md#url)) con una acción de un
controlador. Se escribe en `config/routes.rb`:

```ruby norun
# config/routes.rb
Rails.application.routes.draw do
  root "paginas#inicio"          # la página de inicio: /
  get "/articulos", to: "articulos#index"
end
```

`"articulos#index"` se lee: «el controlador `articulos`, el método `index`».
`bin/rails routes` muestra el plano:

```sh
bin/rails routes
```

```text
   Prefix Verb URI Pattern          Controller#Action
     root GET  /                    paginas#inicio
articulos GET  /articulos(.:format) articulos#index
```

### Un enrutador en Ruby normal

Un enrutador es sobre todo un [diccionario](../debuter/glossaire.md#diccionario): una
dirección da el nombre de un [método](../debuter/glossaire.md#metodo). `send` llama al
método cuyo nombre le das.

```ruby
RUTAS = {
  "/" => :inicio,
  "/articulos" => :index
}

def inicio = "Bienvenido al blog"
def index = "Lista de artículos"

["/", "/articulos", "/contacto"].each do |camino|
  accion = RUTAS[camino]
  if accion
    puts "#{camino} -> #{send(accion)}"
  else
    puts "#{camino} -> 404: página no encontrada"
  end
end
```

```sortie
/ -> Bienvenido al blog
/articulos -> Lista de artículos
/contacto -> 404: página no encontrada
```

Sigamos el bucle paso a paso:

| Paso | camino | accion | Lo que se muestra |
|------|--------|--------|-------------------|
| 1 | `"/"` | `:inicio` | `/ -> Bienvenido al blog` |
| 2 | `"/articulos"` | `:index` | `/articulos -> Lista de artículos` |
| 3 | `"/contacto"` | `nil` (no está en el hash) | `/contacto -> 404: página no encontrada` |

Prueba: añade `"/contacto" => :contacto` en `RUTAS` y un método
`def contacto = "Escríbenos"`. La tercera línea pasa a ser `/contacto -> Escríbenos`.

> **Recuerda:** una ruta dice «tal dirección → tal controlador#acción». Una dirección
> sin ruta da un error 404.

## El controlador: el camarero del restaurante

Un **controlador** es una clase. Cada método público es una **acción**: un pedido que
el camarero sabe tomar.

```ruby norun
# app/controllers/articulos_controller.rb
class ArticulosController < ApplicationController
  def index
    @articulos = ["Hola Rails", "Mis vacaciones", "Una receta"]
  end
end
```

Dos cosas que observar:

- `@articulos` empieza por `@`: es una **variable de instancia**. Rails se la pasa
  automáticamente a la vista.
- No hay `render`: por defecto, Rails muestra la vista que tiene el mismo nombre que la
  acción, aquí `app/views/articulos/index.html.erb`.

El comando `bin/rails generate controller Articulos index` escribe estos dos archivos
por ti.

> **Recuerda:** una acción de controlador prepara los datos en variables `@…` y luego
> Rails muestra la vista con el mismo nombre.

## La vista: la página en ERB

Una **vista** es una plantilla ERB: [HTML](../debuter/glossaire.md#html) con huecos que
Ruby rellena (ver [las plantillas ERB](web.md)).

- `<%= ... %>` calcula e **inserta** un valor;
- `<% ... %>` ejecuta sin insertar nada (un bucle, una condición).

```text
<%# app/views/articulos/index.html.erb %>
<h1>Los artículos</h1>
<ul>
  <% @articulos.each do |titulo| %>
    <li><%= titulo %></li>
  <% end %>
</ul>
```

### La misma vista con la biblioteca erb

ERB viene con Ruby. Este es el mismo trabajo, sin Rails. `binding` da a la plantilla
acceso a las variables `@…` del objeto, como Rails hace por ti.

```ruby
require "erb"

VISTA = <<~ERB
  <h1>Los artículos</h1>
  <ul>
  <%- @articulos.each do |titulo| -%>
    <li><%= titulo %></li>
  <%- end -%>
  </ul>
ERB

class ArticulosController
  def index
    @articulos = ["Hola Rails", "Mis vacaciones"]
    ERB.new(VISTA, trim_mode: "-").result(binding)
  end
end

puts ArticulosController.new.index
```

```sortie
<h1>Los artículos</h1>
<ul>
  <li>Hola Rails</li>
  <li>Mis vacaciones</li>
</ul>
```

`trim_mode: "-"` y las etiquetas `<%-` `-%>` evitan líneas vacías en el resultado.
La plantilla se rellena línea tras línea:

| Paso | Línea de la plantilla | titulo | Texto producido |
|------|-----------------------|--------|-----------------|
| 1 | `<h1>Los artículos</h1>` | — | `<h1>Los artículos</h1>` |
| 2 | `<ul>` | — | `<ul>` |
| 3 | `@articulos.each` (1.ª vuelta) | `"Hola Rails"` | nada |
| 4 | `<li><%= titulo %></li>` | `"Hola Rails"` | `  <li>Hola Rails</li>` |
| 5 | `@articulos.each` (2.ª vuelta) | `"Mis vacaciones"` | nada |
| 6 | `<li><%= titulo %></li>` | `"Mis vacaciones"` | `  <li>Mis vacaciones</li>` |
| 7 | `</ul>` | — | `</ul>` |

Prueba: cambia `<%= titulo %>` por `<%= titulo.upcase %>`. Los títulos se muestran en
mayúsculas: `HOLA RAILS`, `MIS VACACIONES`.

> **Recuerda:** la vista solo presenta. Lee las variables `@…` preparadas por el
> controlador.

## El modelo y su migración

Por ahora, los artículos están escritos a mano en el controlador. Un sitio de verdad
los guarda en una [base de datos](base-de-donnees.md). Ese es el trabajo del **modelo**.

```sh
bin/rails generate model Articulo titulo:string contenido:text
```

Este comando crea `app/models/articulo.rb` y una migración
`db/migrate/20261019120000_create_articulos.rb`. El número largo es la fecha y la hora
de creación: en tu equipo será otro.

### La migración: el plano de la tabla

Una **migración** describe un cambio en la base de datos. Esta crea una tabla
`articulos` con dos columnas:

```ruby norun
# db/migrate/20261019120000_create_articulos.rb
class CreateArticulos < ActiveRecord::Migration[8.0]
  def change
    create_table :articulos do |t|
      t.string :titulo
      t.text :contenido

      t.timestamps
    end
  end
end
```

`t.timestamps` añade dos columnas automáticas: `created_at` (creado el) y
`updated_at` (modificado el). El plano no hace nada hasta que lo aplicas:

```sh
bin/rails db:migrate
```

```text
== 20261019120000 CreateArticulos: migrating ==================================
-- create_table(:articulos)
   -> 0.0021s
== 20261019120000 CreateArticulos: migrated (0.0022s) =========================
```

```text
Tabla articulos (vacía por ahora)
+----+--------+-----------+------------+------------+
| id | titulo | contenido | created_at | updated_at |
+----+--------+-----------+------------+------------+
```

### El modelo: una fila = un objeto

El archivo `app/models/articulo.rb` está casi vacío. Sin embargo, `Articulo` ya sabe
leer y escribir en la tabla: es **ActiveRecord**.

```ruby norun
# app/models/articulo.rb
class Articulo < ApplicationRecord
  validates :titulo, presence: true
end
```

`validates` añade una regla: un artículo sin título se rechaza. En la consola
(`bin/rails console`), `Articulo.create(titulo: "Hola")` añade una fila y
`Articulo.count` responde `1`.

### La idea en Ruby normal

Este es un modelo de bolsillo, sin base de datos: un array guarda las filas.

```ruby
Articulo = Struct.new(:id, :titulo)
FILAS = [] # la «tabla»

def crear_articulo(titulo)
  return nil if titulo.empty? # la regla «título obligatorio»
  FILAS << Articulo.new(FILAS.size + 1, titulo)
end

crear_articulo("Hola Rails")
crear_articulo("")
crear_articulo("Mis vacaciones")

puts "Cantidad: #{FILAS.size}"
FILAS.each { |a| puts "#{a.id}. #{a.titulo}" }
```

```sortie
Cantidad: 2
1. Hola Rails
2. Mis vacaciones
```

| Paso | Llamada | ¿Título vacío? | Filas después |
|------|---------|----------------|---------------|
| 1 | `crear_articulo("Hola Rails")` | no | 1 fila |
| 2 | `crear_articulo("")` | sí → `nil`, no se añade nada | 1 fila |
| 3 | `crear_articulo("Mis vacaciones")` | no | 2 filas |

Prueba: cambia `""` por `"Una receta"`. El contador pasa a `3` y se muestra una tercera
línea.

> **Recuerda:** una migración crea o modifica una tabla; un modelo ActiveRecord convierte
> una fila de la tabla en un objeto Ruby.

## El scaffold: todo de golpe

*Scaffold* significa «andamio». Un solo comando crea el modelo, la migración, el
controlador, las vistas y las rutas para **listar, ver, crear, modificar y borrar**
artículos.

```sh
bin/rails generate scaffold Articulo titulo:string contenido:text
bin/rails db:migrate
```

En `config/routes.rb`, la línea `resources :articulos` crea siete rutas de golpe:

| Dirección | Verbo | Acción | Lo que hace |
|-----------|-------|--------|-------------|
| `/articulos` | GET | `index` | listar los artículos |
| `/articulos/new` | GET | `new` | mostrar el formulario vacío |
| `/articulos` | POST | `create` | guardar el artículo nuevo |
| `/articulos/1` | GET | `show` | ver el artículo n.º 1 |
| `/articulos/1/edit` | GET | `edit` | mostrar el formulario relleno |
| `/articulos/1` | PATCH | `update` | guardar los cambios |
| `/articulos/1` | DELETE | `destroy` | borrar el artículo |

Abre `http://localhost:3000/articulos`: ya puedes crear artículos con un formulario.
El scaffold es perfecto para aprender: lee los archivos creados, uno a uno.

> **Recuerda:** `generate scaffold` escribe todo el MVC de un recurso. Es un punto de
> partida para leer y modificar, no código mágico.

## El viaje de una petición, paso a paso

Escribes `http://localhost:3000/articulos` y pulsas Intro. ¿Qué pasa?

| Paso | Quién trabaja | Lo que pasa | Archivo |
|------|---------------|-------------|---------|
| 1 | el navegador | envía la petición `GET /articulos` | — |
| 2 | el servidor Puma | recibe la petición y se la pasa a Rails | — |
| 3 | el enrutador | encuentra `GET /articulos` → `articulos#index` | `config/routes.rb` |
| 4 | el controlador | crea un `ArticulosController` y llama a `index` | `app/controllers/articulos_controller.rb` |
| 5 | el modelo | `Articulo.all` lee la tabla `articulos` | `app/models/articulo.rb` |
| 6 | el controlador | guarda el resultado en `@articulos` | `app/controllers/articulos_controller.rb` |
| 7 | la vista | rellena la plantilla ERB con `@articulos` | `app/views/articulos/index.html.erb` |
| 8 | Rails | devuelve la respuesta `200` con el HTML | — |
| 9 | el navegador | muestra la página | — |

### El viaje completo en Ruby normal

Este programa repite los pasos 3 a 8 y muestra cada uno de ellos.

```ruby
require "erb"

class Articulo # el modelo
  def self.all = ["Hola Rails", "Mis vacaciones"]
end

class ArticulosController # el controlador
  VISTA = "<ul><% @articulos.each do |t| %><li><%= t %></li><% end %></ul>"

  def index
    puts "4. el controlador llama al modelo"
    @articulos = Articulo.all
    puts "5-6. @articulos contiene #{@articulos.size} títulos"
    puts "7. la vista rellena la plantilla ERB"
    ERB.new(VISTA).result(binding)
  end
end

RUTAS = { ["GET", "/articulos"] => [ArticulosController, :index] }

def tratar(verbo, camino)
  puts "3. el enrutador busca #{verbo} #{camino}"
  clase, accion = RUTAS[[verbo, camino]]
  return [404, "No encontrado"] unless clase

  html = clase.new.send(accion)
  puts "8. respuesta 200"
  [200, html]
end

codigo, cuerpo = tratar("GET", "/articulos")
puts cuerpo
```

```sortie
3. el enrutador busca GET /articulos
4. el controlador llama al modelo
5-6. @articulos contiene 2 títulos
7. la vista rellena la plantilla ERB
8. respuesta 200
<ul><li>Hola Rails</li><li>Mis vacaciones</li></ul>
```

Prueba: llama a `tratar("GET", "/contacto")`. Solo se muestra la línea `3.` y luego
`cuerpo` vale `"No encontrado"`: la petición se detiene en el enrutador, como un error
404.

> **Recuerda:** una petición siempre sigue el mismo camino: ruta → controlador →
> modelo → vista → respuesta. Cuando algo falla, busca en qué paso.

## Errores frecuentes

### Una dirección sin ruta

Abres `http://localhost:3000/articulox` (con una errata). Rails muestra:

```text
Routing Error
No route matches [GET] "/articulox"
```

Ninguna línea de `config/routes.rb` coincide. Revisa la dirección o añade la ruta.
`bin/rails routes` lista todas las rutas que existen.

### Olvidar rellenar la variable de la vista

```ruby norun
require "erb"

class ArticulosController
  def index
    # olvido: @articulos = [...]
    ERB.new("<% @articulos.each do |a| %><%= a %> <% end %>").result(binding)
  end
end

puts ArticulosController.new.index
```

```text
(erb):1:in `index': undefined method `each' for nil (NoMethodError)
	from /opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/erb.rb:429:in `eval'
	from /opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/erb.rb:429:in `result'
	from main.rb:6:in `index'
	from main.rb:10:in `<main>'
```

Una variable `@…` que nunca se rellena vale `nil`. En Rails, es el mensaje más
frecuente en una vista. Rellena `@articulos` en la acción del controlador, exactamente
con el mismo nombre.

### Una migración sin aplicar

Generaste un modelo, pero olvidaste `bin/rails db:migrate`. La página muestra:

```text
ActiveRecord::PendingMigrationError
Migrations are pending. To resolve this issue, run:

        bin/rails db:migrate
```

El mensaje da la solución: ejecuta `bin/rails db:migrate` y recarga la página.

## Ponte a prueba

**Pregunta 1:** en la imagen del restaurante, ¿quién es el controlador y qué hace?

> **Respuesta**
>
> El camarero. Recibe el pedido (la petición), pide los datos a la cocina (el modelo) y
> hace que la carta (la vista) los presente. Él no cocina.

**Pregunta 2:** la acción `index` del controlador `ArticulosController` no llama a
`render`. ¿Qué vista muestra Rails?

> **Respuesta**
>
> `app/views/articulos/index.html.erb`: la vista que lleva el nombre del controlador y
> de la acción.

**Pregunta 3:** completa este enrutador para que `/hola` muestre `¡Buenas!`.

> **Respuesta**
>
> ```ruby
> RUTAS = { "/hola" => :hola }
>
> def hola = "¡Buenas!"
>
> puts send(RUTAS["/hola"])
> ```
>
> ```sortie
> ¡Buenas!
> ```

## En resumen

- Rails ordena un sitio en MVC: el controlador recibe, el modelo da los datos y la vista
  presenta.
- `rails new` crea el proyecto, `bin/rails server` lo arranca en el puerto 3000.
- Una ruta une una dirección con `controlador#acción` en `config/routes.rb`.
- Una migración crea la tabla; un modelo ActiveRecord convierte una fila en un objeto.
- `generate scaffold` escribe todo el MVC de un recurso de golpe.
- Una petición sigue: ruta → controlador → modelo → vista → respuesta.

Siguiente: [probar tu código](tests.md)
