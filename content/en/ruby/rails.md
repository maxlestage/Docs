# A first website with Rails, step by step

This page guides you through a first Ruby on Rails website: routes, a controller, a view, a model, then the full journey of a request, step by step.

> **Goals:**
>
> - understand the role of each piece of Rails thanks to the restaurant picture;
> - create a project with `rails new` and start the server;
> - write a route, a controller and an ERB view;
> - create a model and its migration, then generate everything at once with a *scaffold*;
> - follow a request from the browser all the way to the displayed page.

## What is Rails?

**Ruby on Rails** (often just "Rails") is a web *framework*. A *framework* is a working
structure: a house that is already built, with tidy rooms. You bring your furniture;
you do not pour the foundations.

The page [Ruby for the web](web.md) shows Rack and Sinatra. Rails is also built on Rack,
but it brings much more: database, forms, security, tests.

Rails sorts code into three roles, called **MVC**: Model, View, Controller.

### The Rails restaurant

Picture a restaurant:

```text
   You (the browser)
        |
        |  "I would like the list of articles"     (the request)
        v
  +-------------+   "3 articles, please"     +-------------+
  |  WAITER     | -------------------------> |  KITCHEN    |
  | controller  | <------------------------- |  model      |
  +-------------+     the 3 articles         +-------------+
        |
        |  plates the dish following the menu
        v
  +-------------+
  |  MENU /     |
  |  PLATE      |   the view: what you see
  +-------------+
        |
        v
   You receive the HTML page                 (the response)
```

| Restaurant | Rails | Folder | Its job |
|------------|-------|--------|---------|
| the floor plan | the **routes** | `config/routes.rb` | say which request goes to which waiter |
| the waiter | the **controller** | `app/controllers` | take the order, talk to the kitchen |
| the kitchen | the **model** | `app/models` | prepare the data (the database) |
| the menu and the plate | the **view** | `app/views` | present the result to the customer |

The waiter never cooks. The kitchen never talks to the customer. Each to their own job:
that is what keeps a big website easy to organise.

### The restaurant in plain Ruby

Here is the idea in pure Ruby, without Rails. Each role is a small
[class](../debuter/glossaire.md#classe).

```ruby
class Kitchen # the model
  def self.dishes = ["Soup", "Gratin", "Pie"]
end

class Menu # the view
  def self.show(dishes) = dishes.map { |d| "- #{d}" }.join("\n")
end

class Waiter # the controller
  def order = Menu.show(Kitchen.dishes)
end

puts Waiter.new.order
```

```sortie
- Soup
- Gratin
- Pie
```

Try it: add `"Salad"` to `Kitchen.dishes`. Only the model changes, and yet the page
shows one more line. The waiter and the menu noticed nothing.

> **Remember:** MVC = three separate roles. The controller receives the request, the
> model provides the data, the view presents it.

## Installing Rails and creating a project

Rails is a [gem](../debuter/glossaire.md#gem) (a Ruby library). You type these commands
in the [terminal](../debuter/glossaire.md#terminal). They need the Internet: they are
not run here.

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

Version numbers may be different on your computer. `rails new` builds the house: dozens of folders and files. Here are the
rooms that matter when you start:

```text
blog/
├── app/
│   ├── controllers/   ← the waiters (controllers)
│   ├── models/        ← the kitchen (models)
│   └── views/         ← the menus (ERB views)
├── config/
│   └── routes.rb      ← the floor plan (routes)
├── db/
│   └── migrate/       ← the plans of the tables
└── Gemfile            ← the list of gems (see projet.md)
```

### Starting the server

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

Open `http://localhost:3000` in your browser: Rails shows its welcome page.
`Ctrl+C` in the terminal stops the server.

Try it: run `bin/rails server -p 4000`. The site then answers on port `4000`
instead of `3000`.

> **Remember:** `rails new name` creates the project; `bin/rails server` starts it on
> `http://localhost:3000`.

## Routes: the floor plan

A **route** links an address ([URL](../debuter/glossaire.md#url)) to an action of a
controller. You write it in `config/routes.rb`:

```ruby norun
# config/routes.rb
Rails.application.routes.draw do
  root "pages#home"              # the home page: /
  get "/articles", to: "articles#index"
end
```

`"articles#index"` reads as: "the `articles` controller, the `index` method".
`bin/rails routes` shows the plan:

```sh
bin/rails routes
```

```text
  Prefix Verb URI Pattern         Controller#Action
    root GET  /                   pages#home
articles GET  /articles(.:format) articles#index
```

### A router in plain Ruby

A router is mostly a [dictionary](../debuter/glossaire.md#dictionnaire): an address
gives the name of a [method](../debuter/glossaire.md#methode). `send` calls the method
whose name you give it.

```ruby
ROUTES = {
  "/" => :home,
  "/articles" => :index
}

def home = "Welcome to the blog"
def index = "List of articles"

["/", "/articles", "/contact"].each do |path|
  action = ROUTES[path]
  if action
    puts "#{path} -> #{send(action)}"
  else
    puts "#{path} -> 404: page not found"
  end
end
```

```sortie
/ -> Welcome to the blog
/articles -> List of articles
/contact -> 404: page not found
```

Let's follow the loop step by step:

| Step | path | action | What is printed |
|------|------|--------|-----------------|
| 1 | `"/"` | `:home` | `/ -> Welcome to the blog` |
| 2 | `"/articles"` | `:index` | `/articles -> List of articles` |
| 3 | `"/contact"` | `nil` (not in the hash) | `/contact -> 404: page not found` |

Try it: add `"/contact" => :contact` to `ROUTES` and a method
`def contact = "Write to us"`. The third line becomes `/contact -> Write to us`.

> **Remember:** a route says "this address → this controller#action". An address
> without a route gives a 404 error.

## The controller: the restaurant's waiter

A **controller** is a class. Each public method is an **action**: an order the waiter
knows how to take.

```ruby norun
# app/controllers/articles_controller.rb
class ArticlesController < ApplicationController
  def index
    @articles = ["Hello Rails", "My holidays", "A recipe"]
  end
end
```

Two things to notice:

- `@articles` starts with `@`: it is an **instance variable**. Rails hands it to the
  view automatically.
- There is no `render`: by default, Rails shows the view with the same name as the
  action, here `app/views/articles/index.html.erb`.

The command `bin/rails generate controller Articles index` writes these two files
for you.

> **Remember:** a controller action prepares the data in `@…` variables, then Rails
> shows the view with the same name.

## The view: the page in ERB

A **view** is an ERB template: [HTML](../debuter/glossaire.md#html) with holes filled
in by Ruby (see [ERB templates](web.md)).

- `<%= ... %>` computes and **inserts** a value;
- `<% ... %>` runs code without inserting anything (a loop, a condition).

```text
<%# app/views/articles/index.html.erb %>
<h1>The articles</h1>
<ul>
  <% @articles.each do |title| %>
    <li><%= title %></li>
  <% end %>
</ul>
```

### The same view with the erb library

ERB ships with Ruby. Here is the same work, without Rails. `binding` gives the template
access to the object's `@…` variables, as Rails does for you.

```ruby
require "erb"

VIEW = <<~ERB
  <h1>The articles</h1>
  <ul>
  <%- @articles.each do |title| -%>
    <li><%= title %></li>
  <%- end -%>
  </ul>
ERB

class ArticlesController
  def index
    @articles = ["Hello Rails", "My holidays"]
    ERB.new(VIEW, trim_mode: "-").result(binding)
  end
end

puts ArticlesController.new.index
```

```sortie
<h1>The articles</h1>
<ul>
  <li>Hello Rails</li>
  <li>My holidays</li>
</ul>
```

`trim_mode: "-"` and the `<%-` `-%>` tags avoid empty lines in the result.
The template is filled line after line:

| Step | Template line | title | Text produced |
|------|---------------|-------|---------------|
| 1 | `<h1>The articles</h1>` | — | `<h1>The articles</h1>` |
| 2 | `<ul>` | — | `<ul>` |
| 3 | `@articles.each` (1st round) | `"Hello Rails"` | nothing |
| 4 | `<li><%= title %></li>` | `"Hello Rails"` | `  <li>Hello Rails</li>` |
| 5 | `@articles.each` (2nd round) | `"My holidays"` | nothing |
| 6 | `<li><%= title %></li>` | `"My holidays"` | `  <li>My holidays</li>` |
| 7 | `</ul>` | — | `</ul>` |

Try it: replace `<%= title %>` with `<%= title.upcase %>`. The titles are shown in
capitals: `HELLO RAILS`, `MY HOLIDAYS`.

> **Remember:** the view only presents. It reads the `@…` variables prepared by the
> controller.

## The model and its migration

For now, the articles are hard-coded in the controller. A real website stores them in a
[database](base-de-donnees.md). That is the job of the **model**.

```sh
bin/rails generate model Article title:string content:text
```

This command creates `app/models/article.rb` and a migration
`db/migrate/20261019120000_create_articles.rb`. The long number is the date and time of
creation: it will be different for you.

### The migration: the plan of the table

A **migration** describes a change to the database. This one creates an `articles`
table with two columns:

```ruby norun
# db/migrate/20261019120000_create_articles.rb
class CreateArticles < ActiveRecord::Migration[8.0]
  def change
    create_table :articles do |t|
      t.string :title
      t.text :content

      t.timestamps
    end
  end
end
```

`t.timestamps` adds two automatic columns: `created_at` (created on) and
`updated_at` (updated on). The plan does nothing until you apply it:

```sh
bin/rails db:migrate
```

```text
== 20261019120000 CreateArticles: migrating ===================================
-- create_table(:articles)
   -> 0.0021s
== 20261019120000 CreateArticles: migrated (0.0022s) ==========================
```

```text
Table articles (empty for now)
+----+-------+---------+------------+------------+
| id | title | content | created_at | updated_at |
+----+-------+---------+------------+------------+
```

### The model: one row = one object

The file `app/models/article.rb` is almost empty. Yet `Article` already knows how to
read and write in the table: that is **ActiveRecord**.

```ruby norun
# app/models/article.rb
class Article < ApplicationRecord
  validates :title, presence: true
end
```

`validates` adds a rule: an article without a title is refused. In the console
(`bin/rails console`), `Article.create(title: "Hello")` adds a row and
`Article.count` answers `1`.

### The idea in plain Ruby

Here is a pocket model, without a database: an array keeps the rows.

```ruby
Article = Struct.new(:id, :title)
ROWS = [] # the "table"

def create_article(title)
  return nil if title.empty? # the "title required" rule
  ROWS << Article.new(ROWS.size + 1, title)
end

create_article("Hello Rails")
create_article("")
create_article("My holidays")

puts "Count: #{ROWS.size}"
ROWS.each { |a| puts "#{a.id}. #{a.title}" }
```

```sortie
Count: 2
1. Hello Rails
2. My holidays
```

| Step | Call | Empty title? | Rows after |
|------|------|--------------|------------|
| 1 | `create_article("Hello Rails")` | no | 1 row |
| 2 | `create_article("")` | yes → `nil`, nothing is added | 1 row |
| 3 | `create_article("My holidays")` | no | 2 rows |

Try it: replace `""` with `"A recipe"`. The count goes up to `3` and a third line is
printed.

> **Remember:** a migration creates or changes a table; an ActiveRecord model turns a
> row of the table into a Ruby object.

## The scaffold: everything at once

*Scaffold* is the frame builders put up around a building. A single command creates
the model, the migration, the controller, the views and the routes to **list, show,
create, edit and delete** articles.

```sh
bin/rails generate scaffold Article title:string content:text
bin/rails db:migrate
```

In `config/routes.rb`, the line `resources :articles` creates seven routes at once:

| Address | Verb | Action | What it does |
|---------|------|--------|--------------|
| `/articles` | GET | `index` | list the articles |
| `/articles/new` | GET | `new` | show the empty form |
| `/articles` | POST | `create` | save the new article |
| `/articles/1` | GET | `show` | show article no. 1 |
| `/articles/1/edit` | GET | `edit` | show the filled-in form |
| `/articles/1` | PATCH | `update` | save the changes |
| `/articles/1` | DELETE | `destroy` | delete the article |

Open `http://localhost:3000/articles`: you can already create articles with a form.
The scaffold is perfect for learning: read the files it created, one by one.

> **Remember:** `generate scaffold` writes the whole MVC of a resource. It is a starting
> point to read and change, not magic code.

## The journey of a request, step by step

You type `http://localhost:3000/articles` and press Enter. What happens?

| Step | Who works | What happens | File |
|------|-----------|--------------|------|
| 1 | the browser | sends the request `GET /articles` | — |
| 2 | the Puma server | receives the request and passes it to Rails | — |
| 3 | the router | finds `GET /articles` → `articles#index` | `config/routes.rb` |
| 4 | the controller | creates an `ArticlesController` and calls `index` | `app/controllers/articles_controller.rb` |
| 5 | the model | `Article.all` reads the `articles` table | `app/models/article.rb` |
| 6 | the controller | stores the result in `@articles` | `app/controllers/articles_controller.rb` |
| 7 | the view | fills the ERB template with `@articles` | `app/views/articles/index.html.erb` |
| 8 | Rails | sends back the `200` response with the HTML | — |
| 9 | the browser | shows the page | — |

### The full journey in plain Ruby

This program replays steps 3 to 8 and prints each of them.

```ruby
require "erb"

class Article # the model
  def self.all = ["Hello Rails", "My holidays"]
end

class ArticlesController # the controller
  VIEW = "<ul><% @articles.each do |t| %><li><%= t %></li><% end %></ul>"

  def index
    puts "4. the controller calls the model"
    @articles = Article.all
    puts "5-6. @articles holds #{@articles.size} titles"
    puts "7. the view fills the ERB template"
    ERB.new(VIEW).result(binding)
  end
end

ROUTES = { ["GET", "/articles"] => [ArticlesController, :index] }

def handle(verb, path)
  puts "3. the router looks for #{verb} #{path}"
  klass, action = ROUTES[[verb, path]]
  return [404, "Not found"] unless klass

  html = klass.new.send(action)
  puts "8. response 200"
  [200, html]
end

code, body = handle("GET", "/articles")
puts body
```

```sortie
3. the router looks for GET /articles
4. the controller calls the model
5-6. @articles holds 2 titles
7. the view fills the ERB template
8. response 200
<ul><li>Hello Rails</li><li>My holidays</li></ul>
```

Try it: call `handle("GET", "/contact")`. Only the `3.` line is printed, then `body`
is `"Not found"`: the request stops at the router, like a 404 error.

> **Remember:** a request always follows the same path: route → controller → model →
> view → response. When something breaks, look for the step where it happens.

## Common mistakes

### An address without a route

You open `http://localhost:3000/articlez` (with a typo). Rails shows:

```text
Routing Error
No route matches [GET] "/articlez"
```

No line of `config/routes.rb` matches. Check the address, or add the route.
`bin/rails routes` lists all the existing routes.

### Forgetting to fill the view's variable

```ruby norun
require "erb"

class ArticlesController
  def index
    # forgotten: @articles = [...]
    ERB.new("<% @articles.each do |a| %><%= a %> <% end %>").result(binding)
  end
end

puts ArticlesController.new.index
```

```text
(erb):1:in `index': undefined method `each' for nil (NoMethodError)
	from /opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/erb.rb:429:in `eval'
	from /opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/erb.rb:429:in `result'
	from main.rb:6:in `index'
	from main.rb:10:in `<main>'
```

An `@…` variable that was never filled is `nil`. In Rails, this is the most common
message in a view. Fill `@articles` in the controller action, with exactly the same
name.

### A migration not applied yet

You generated a model, but forgot `bin/rails db:migrate`. The page shows:

```text
ActiveRecord::PendingMigrationError
Migrations are pending. To resolve this issue, run:

        bin/rails db:migrate
```

The message gives the fix: run `bin/rails db:migrate`, then reload the page.

## Test yourself

**Question 1:** in the restaurant picture, who is the controller, and what does it do?

> **Answer**
>
> The waiter. It receives the order (the request), asks the kitchen (the model) for the
> data and has the menu (the view) present it. It does not cook itself.

**Question 2:** the `index` action of the `ArticlesController` controller does not call
`render`. Which view does Rails show?

> **Answer**
>
> `app/views/articles/index.html.erb`: the view named after the controller and the
> action.

**Question 3:** complete this router so that `/hello` prints `Hi!`.

> **Answer**
>
> ```ruby
> ROUTES = { "/hello" => :hello }
>
> def hello = "Hi!"
>
> puts send(ROUTES["/hello"])
> ```
>
> ```sortie
> Hi!
> ```

## Summary

- Rails organises a website in MVC: the controller receives, the model provides the
  data, the view presents.
- `rails new` creates the project, `bin/rails server` starts it on port 3000.
- A route links an address to `controller#action` in `config/routes.rb`.
- A migration creates the table; an ActiveRecord model turns a row into an object.
- `generate scaffold` writes the whole MVC of a resource at once.
- A request goes: route → controller → model → view → response.

Next: [testing your code](tests.md)
