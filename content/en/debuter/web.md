# How the web works

When you open a page, your browser sends a request to a faraway computer that answers it: this page explains this conversation, addresses, response codes, HTML, CSS, JavaScript, APIs, and where Ruby, Rust and Rythme fit in.

> **Goals:**
>
> - understand the roles of the client and the server;
> - read a URL piece by piece;
> - read an HTTP request and response, and its codes (200, 404…);
> - know what HTML, CSS, JavaScript, APIs and JSON are for;
> - create your first web page and open it in the browser.

## The client and the server

The web is a conversation between two computers:

- the [client](glossaire.md#client) **asks**: it is your browser
  (Firefox, Chrome, Safari…);
- the [server](glossaire.md#server) **answers**: it is a computer,
  often in a big data center, that waits for requests day and
  night.

Think of a restaurant. You (the client) place an order, the kitchen (the
server) prepares the dish and brings it to you. You never see the kitchen:
you only see the dish.

The same server answers thousands of clients at once. And a server
is just a program running on a computer: you can start
one on yours.

## The address: the URL

To ask for a page, you need its address: the [URL](glossaire.md#url).
Let's break one down:

```text
https://www.example.com:443/recipes/crepes?servings=4#ingredients
└─┬─┘   └──────┬──────┘ └┬┘└──────┬──────┘ └───┬────┘ └────┬────┘
protocol    machine    port     path       parameters    anchor
```

- the **protocol** says how to talk: `https` (encrypted) or `http`;
- the **machine** (the domain name) says which server to talk to;
- the **port** is the server's "door"; it is almost always left out
  (443 for `https`, 80 for `http`);
- the **path** says which page you want, like a
  [file path](ordinateur.md);
- the **parameters**, after `?`, make the request more precise: `name=value`,
  separated by `&`;
- the **anchor**, after `#`, points to a place in the page.

Ruby can split a URL:

```ruby
require "uri"

address = URI("https://www.example.com:443/recipes/crepes?servings=4#ingredients")
puts address.scheme
puts address.host
puts address.port
puts address.path
puts address.query
puts address.fragment
```

```sortie
https
www.example.com
443
/recipes/crepes
servings=4
ingredients
```

In Rust, you can do it by hand, with `split_once`:

```rust
let url = "https://www.example.com/recipes/crepes?servings=4";
let (protocol, rest) = url.split_once("://").unwrap();
let (machine, path) = rest.split_once('/').unwrap();
println!("protocol: {protocol}");
println!("machine: {machine}");
println!("path: /{path}");
```

```sortie
protocol: https
machine: www.example.com
path: /recipes/crepes?servings=4
```

## The HTTP request and response

The client and the server speak a common language: [HTTP](glossaire.md#http).
It is **text**, line by line.

### The request

When you type `https://example.com/` in the browser, it sends roughly
this:

```text
GET / HTTP/1.1
Host: example.com
User-Agent: Mozilla/5.0 (X11; Linux x86_64) Firefox/131.0
Accept: text/html
```

- `GET` is the **method**: "give me". `POST` means "here is some
  data" (a submitted form, for example);
- `/` is the requested path;
- the following lines are the **headers**: extra information,
  in the form `Name: value`.

### The response

The server answers with a status line, headers, an empty line, then
the **body**: the page itself.

```text
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
Content-Length: 1256
Date: Sun, 18 Oct 2026 20:14:17 GMT

<!doctype html>
<html>
<head><title>Example Domain</title></head>
<body><h1>Example Domain</h1> ...
```

The browser reads this body and draws the page on the screen.

> **Tip:** in your browser, press **F12** and open the
> **Network** tab. Reload the page: you see all the
> requests and all the responses.

Since HTTP is text, a program can read it easily:

```ruby
request = "GET /recipes/crepes HTTP/1.1\r\nHost: www.example.com\r\n\r\n"
first_line = request.lines.first.strip
method, path, version = first_line.split(" ")
puts "Method: #{method}"
puts "Path: #{path}"
puts "Version: #{version}"
```

```sortie
Method: GET
Path: /recipes/crepes
Version: HTTP/1.1
```

## Status codes

The number on the first line of the response says how things went.
The first digit gives the family: 2 = success, 3 = somewhere else, 4 = client
error, 5 = server error.

| Code | Name                  | Meaning                                     |
|------|-----------------------|---------------------------------------------|
| 200  | OK                    | all good, here is the page                  |
| 301  | Moved Permanently     | the page has moved, here is its new address |
| 404  | Not Found             | this page doesn't exist                     |
| 500  | Internal Server Error | the server's program crashed                |

A `match` in Rust reads a code easily:

```rust
let response = "HTTP/1.1 404 Not Found";
let code: u16 = response.split(' ').nth(1).unwrap().parse().unwrap();
let meaning = match code {
    200 => "all good",
    301 => "the page has moved",
    404 => "page not found",
    500 => "the server crashed",
    _ => "other response",
};
println!("{code}: {meaning}");
```

```sortie
404: page not found
```

And in Rythme, with one function per case:

```rythme
fn meaning(200) = "all good"
fn meaning(404) = "page not found"
fn meaning(_) = "other response"

each([200, 404, 418], fn(code) = print("{code}: {meaning(code)}"))
```

```sortie
200: all good
404: page not found
418: other response
```

## HTML, CSS and JavaScript

A web page is made of three languages, each with its own role. Think of
a house:

- [HTML](glossaire.md#html) is the **structure**: the walls, the rooms.
  It says "this is a heading, this is a paragraph, this is a link";
- **CSS** is the **decoration**: the colors, the fonts, the position of
  each element;
- **JavaScript** is the **electricity**: it brings the page to life (a button
  that reacts, a menu that opens).

HTML is written with **tags**: `<h1>` opens a heading, `</h1>` closes
it. CSS says how to display them:

```text
h1 {
  color: darkblue;
  font-family: sans-serif;
}
```

These three languages run **in the browser**, on the client. Ruby,
Rust and Rythme usually run **on the server**, or on your
computer.

## APIs and JSON

An HTML page is made for humans. But often, a program wants to
talk to another program: a weather app asks a server for the temperature,
for example. For that, the server offers an
[API](glossaire.md#api): addresses that answer with **data**,
not with a page.

This data is almost always in [JSON](glossaire.md#json), a simple
text format:

```text
GET /api/weather?city=Lyon HTTP/1.1
Host: api.example.com

HTTP/1.1 200 OK
Content-Type: application/json

{"city": "Lyon", "temperature": 18, "rain": false}
```

Turning data into JSON text is called
[serialization](glossaire.md#serialization). In Ruby:

```ruby
require "json"

recipe = { "name" => "Crêpes", "servings" => 4, "vegetarian" => true }
text = JSON.generate(recipe)
puts text

read_back = JSON.parse(text)
puts read_back["servings"] + 1
```

```sortie
{"name":"Crêpes","servings":4,"vegetarian":true}
5
```

In Rythme, `to_json` and `parse_json` are already there:

```rythme
recipe = {name: "Crêpes", servings: 4, vegetarian: true}
text = to_json(recipe)
print(text)
match parse_json(text)
  Ok(r) -> print(r.servings + 1)
  Err(e) -> print("Invalid JSON:", e)
end
```

```sortie
{"name": "Crêpes", "servings": 4, "vegetarian": true}
5
```

In Rust, you use the [package](glossaire.md#package) `serde_json`, which you
add to the project: see [formats in Rust](../rust/formats.md).

## Where Ruby, Rust and Rythme fit in

### Ruby and Rails

Ruby is famous for the web thanks to **Ruby on Rails**, a *framework*:
a toolbox that already provides the server, the
pages, the database. GitHub, Shopify and many other sites have
used it.

```sh
gem install rails
rails new my_site
cd my_site
bin/rails server
```

See [Ruby and the web](../ruby/web.md).

### Rust and its packages: axum

Rust has no web server in its standard library: you add
packages (*crates*), like **axum** for the server and **tokio** for
[asynchronous code](glossaire.md#async). The result is very fast and
uses little memory. This code needs these packages, so it is not
run here:

```rust norun
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello from Rust!" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

See [Rust and the web](../rust/web.md).

### Rythme

Rythme is not made for writing a server. But it can read and write
JSON, write HTML files, and it also runs in the browser
thanks to its *playground*. See [formats in Rythme](../rythme/formats.md).

## Your first web page

No need for a server to start: an HTML file opens directly
in the browser.

1. In your [editor](editeur.md), create a file `index.html`.
2. Type this content:

```text
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>My first page</title>
    <style>
      body { font-family: sans-serif; }
      h1 { color: darkblue; }
    </style>
  </head>
  <body>
    <h1>Hello, web!</h1>
    <p>This page is just a file on my computer.</p>
    <a href="https://www.ruby-lang.org/en/">The Ruby website</a>
  </body>
</html>
```

3. Save, then double-click the file (or drag it into the
   browser). The address starts with `file://`: it is your disk, not
   a server.

A program can also **build** a page. In Rythme:

```rythme
dishes = ["Crêpes", "Waffles", "Apple pie"]
lines = map(dishes, fn(d) = "  <li>{d}</li>")
page = "<h1>My recipes</h1>\n<ul>\n" + join(lines, "\n") + "\n</ul>"
write_file("recipes.html", page)
print(unwrap(read_file("recipes.html")))
```

```sortie
<h1>My recipes</h1>
<ul>
  <li>Crêpes</li>
  <li>Waffles</li>
  <li>Apple pie</li>
</ul>
```

Open `recipes.html` in the browser: your list appears. That is the
principle behind every website: a program writes HTML.

## Common mistakes

### Putting a space in a URL

```ruby norun
require "uri"
address = URI("https://example.com/my photos")
puts address.host
```

```text
/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/uri/rfc3986_parser.rb:130:in `split': bad URI(is not URI?): "https://example.com/my photos" (URI::InvalidURIError)
	from main.rb:2:in `<main>'
```

A URL contains no spaces. You replace them with `%20`:
`https://example.com/my%20photos`. Better: avoid spaces in the
names of your web files (`my-photos`).

### Writing JSON with single quotes

```ruby norun
require "json"
data = JSON.parse("{'name': 'Ada'}")
```

```text
/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/json/common.rb:220:in `parse': unexpected token at '{'name': 'Ada'}' (JSON::ParserError)
	from main.rb:2:in `<main>'
```

JSON only accepts straight double quotes: `{"name": "Ada"}`.

### Forgetting the quotes around JSON keys

```rythme norun
print(parse_json("\{name: \"Ada\"\}"))
```

```text
Err("JSON, line 1, column 2: expected a name in quotes, but found `n`")
```

In Rythme, `{name: "Ada"}` is a valid record, but in JSON, the
keys are **always** in quotes: `{"name": "Ada"}`. The safest is
to let `to_json` write the JSON for you.

## Test yourself

**1.** In `https://shop.com/cart?code=PROMO`, what is the path, and
what is the parameter?

> **Answer**
>
> The path is `/cart`, the parameter is `code=PROMO`.

**2.** The server answers `404`. Who made a mistake, and about what?

> **Answer**
>
> The client: it asked for a page that doesn't exist (often a typo in
> the address). A server error would start with 5, like `500`.

**3.** Which language decides the color of a heading: HTML, CSS or
JavaScript?

> **Answer**
>
> CSS, the decoration. HTML says it is a heading, JavaScript makes it
> interactive.

## Summary

- The client (the browser) asks, the server answers.
- A URL is read in pieces: protocol, machine, path, parameters, anchor.
- HTTP is text: a request (`GET /…`), a response with a code (200, 301, 404, 500).
- HTML structures, CSS decorates, JavaScript brings to life; an API returns data in JSON.
- Ruby (Rails) and Rust (axum) write servers; Rythme reads and writes JSON and HTML.

Next: [Qu'est-ce qu'une base de données ?](bases-de-donnees.md)
