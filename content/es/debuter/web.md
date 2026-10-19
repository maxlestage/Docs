# Cómo funciona la web

Cuando abres una página, tu navegador envía una petición a un ordenador lejano que le responde: esta página explica ese diálogo, las direcciones, los códigos de respuesta, HTML, CSS, JavaScript, las API y el lugar de Ruby, Rust y Rythme.

> **Objetivos:**
>
> - entender el papel del cliente y del servidor;
> - leer una URL pieza a pieza;
> - leer una petición y una respuesta HTTP, y sus códigos (200, 404…);
> - saber para qué sirven HTML, CSS, JavaScript, las API y JSON;
> - crear tu primera página web y abrirla en el navegador.

## El cliente y el servidor

La web es una conversación entre dos ordenadores:

- el [cliente](glossaire.md#cliente) **pide**: es tu navegador (Firefox,
  Chrome, Safari…);
- el [servidor](glossaire.md#servidor) **responde**: es un ordenador, a
  menudo en un gran centro de datos, que espera las peticiones día y
  noche.

Piensa en un restaurante. Tú (el cliente) haces el pedido, la cocina (el
servidor) prepara el plato y te lo trae. Nunca ves la cocina: solo ves el
plato.

Un mismo servidor responde a miles de clientes a la vez. Y un servidor no
es más que un programa que se ejecuta en un ordenador: puedes lanzar uno
en el tuyo.

## La dirección: la URL

Para pedir una página, necesitas su dirección: la [URL](glossaire.md#url).
Vamos a descomponer una:

```text
https://www.ejemplo.es:443/recetas/crepes?raciones=4#ingredientes
└─┬─┘   └─────┬──────┘ └┬┘└──────┬──────┘ └───┬────┘ └────┬─────┘
protocolo  máquina   puerto    ruta      parámetros     ancla
```

- el **protocolo** dice cómo hablar: `https` (cifrado) o `http`;
- la **máquina** (el nombre de dominio) dice a qué servidor dirigirse;
- el **puerto** es la «puerta» del servidor; casi siempre se omite (443
  para `https`, 80 para `http`);
- la **ruta** dice qué página queremos, como una
  [ruta de archivo](ordinateur.md);
- los **parámetros**, después de `?`, precisan la petición:
  `nombre=valor`, separados por `&`;
- el **ancla**, después de `#`, señala un lugar dentro de la página.

Ruby sabe descomponer una URL:

```ruby
require "uri"

direccion = URI("https://www.ejemplo.es:443/recetas/crepes?raciones=4#ingredientes")
puts direccion.scheme
puts direccion.host
puts direccion.port
puts direccion.path
puts direccion.query
puts direccion.fragment
```

```sortie
https
www.ejemplo.es
443
/recetas/crepes
raciones=4
ingredientes
```

En Rust, puedes hacerlo a mano, con `split_once`:

```rust
let url = "https://www.ejemplo.es/recetas/crepes?raciones=4";
let (protocolo, resto) = url.split_once("://").unwrap();
let (maquina, ruta) = resto.split_once('/').unwrap();
println!("protocolo: {protocolo}");
println!("máquina: {maquina}");
println!("ruta: /{ruta}");
```

```sortie
protocolo: https
máquina: www.ejemplo.es
ruta: /recetas/crepes?raciones=4
```

## La petición y la respuesta HTTP

El cliente y el servidor hablan un idioma común: [HTTP](glossaire.md#http).
Es **texto**, línea a línea.

### La petición

Cuando escribes `https://example.com/` en el navegador, este envía más o
menos esto:

```text
GET / HTTP/1.1
Host: example.com
User-Agent: Mozilla/5.0 (X11; Linux x86_64) Firefox/131.0
Accept: text/html
```

- `GET` es el **método**: «dame». `POST` quiere decir «aquí tienes unos
  datos» (un formulario enviado, por ejemplo);
- `/` es la ruta pedida;
- las líneas siguientes son las **cabeceras**: información extra, con la
  forma `Nombre: valor`.

### La respuesta

El servidor responde con una línea de estado, unas cabeceras, una línea
vacía y después el **cuerpo**: la página en sí.

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

El navegador lee ese cuerpo y dibuja la página en la pantalla.

> **Consejo:** en tu navegador, pulsa **F12** y abre la pestaña **Red**
> (*Network*). Recarga la página: verás todas las peticiones y todas las
> respuestas.

Como HTTP es texto, un programa puede leerlo fácilmente:

```ruby
peticion = "GET /recetas/crepes HTTP/1.1\r\nHost: www.ejemplo.es\r\n\r\n"
primera_linea = peticion.lines.first.strip
metodo, ruta, version = primera_linea.split(" ")
puts "Método: #{metodo}"
puts "Ruta: #{ruta}"
puts "Versión: #{version}"
```

```sortie
Método: GET
Ruta: /recetas/crepes
Versión: HTTP/1.1
```

## Los códigos de estado

El número de la primera línea de la respuesta dice cómo ha ido todo. La
primera cifra indica la familia: 2 = éxito, 3 = en otro sitio, 4 = error
del cliente, 5 = error del servidor.

| Código | Nombre                | Significado                                 |
|--------|-----------------------|---------------------------------------------|
| 200    | OK                    | todo va bien, aquí tienes la página         |
| 301    | Moved Permanently     | la página se ha mudado, esta es su nueva dirección |
| 404    | Not Found             | esta página no existe                       |
| 500    | Internal Server Error | el programa del servidor ha fallado         |

Un `match` en Rust lee un código fácilmente:

```rust
let respuesta = "HTTP/1.1 404 Not Found";
let codigo: u16 = respuesta.split(' ').nth(1).unwrap().parse().unwrap();
let significado = match codigo {
    200 => "todo va bien",
    301 => "la página se ha mudado",
    404 => "página no encontrada",
    500 => "el servidor ha fallado",
    _ => "otra respuesta",
};
println!("{codigo}: {significado}");
```

```sortie
404: página no encontrada
```

Y en Rythme, con una función por caso:

```rythme
fn significado(200) = "todo va bien"
fn significado(404) = "página no encontrada"
fn significado(_) = "otra respuesta"

each([200, 404, 418], fn(codigo) = print("{codigo}: {significado(codigo)}"))
```

```sortie
200: todo va bien
404: página no encontrada
418: otra respuesta
```

## HTML, CSS y JavaScript

Una página web está hecha con tres lenguajes, cada uno con su papel.
Piensa en una casa:

- [HTML](glossaire.md#html) es la **estructura**: las paredes, las
  habitaciones. Dice «esto es un título, esto es un párrafo, esto es un
  enlace»;
- **CSS** es la **decoración**: los colores, las fuentes, el lugar de cada
  elemento;
- **JavaScript** es la **electricidad**: hace que la página cobre vida (un
  botón que reacciona, un menú que se abre).

El HTML se escribe con **etiquetas**: `<h1>` abre un título, `</h1>` lo
cierra. El CSS dice cómo mostrarlos:

```text
h1 {
  color: darkblue;
  font-family: sans-serif;
}
```

Estos tres lenguajes se ejecutan **en el navegador**, en el cliente. Ruby,
Rust y Rythme, en cambio, se ejecutan en general **en el servidor**, o en
tu ordenador.

## Las API y JSON

Una página HTML está hecha para las personas. Pero a menudo un programa
quiere hablar con otro programa: una aplicación del tiempo le pide la
temperatura a un servidor, por ejemplo. Para eso, el servidor ofrece una
[API](glossaire.md#api): direcciones que responden con **datos**, no con
una página.

Esos datos casi siempre están en [JSON](glossaire.md#json), un formato de
texto sencillo:

```text
GET /api/tiempo?ciudad=Sevilla HTTP/1.1
Host: api.ejemplo.es

HTTP/1.1 200 OK
Content-Type: application/json

{"ciudad": "Sevilla", "temperatura": 24, "lluvia": false}
```

Convertir datos en texto JSON se llama
[serialización](glossaire.md#serializacion). En Ruby:

```ruby
require "json"

receta = { "nombre" => "Crepes", "raciones" => 4, "vegetariana" => true }
texto = JSON.generate(receta)
puts texto

releida = JSON.parse(texto)
puts releida["raciones"] + 1
```

```sortie
{"nombre":"Crepes","raciones":4,"vegetariana":true}
5
```

En Rythme, `to_json` y `parse_json` ya están disponibles:

```rythme
receta = {nombre: "Crepes", raciones: 4, vegetariana: true}
texto = to_json(receta)
print(texto)
match parse_json(texto)
  Ok(r) -> print(r.raciones + 1)
  Err(e) -> print("JSON no válido:", e)
end
```

```sortie
{"nombre": "Crepes", "raciones": 4, "vegetariana": true}
5
```

En Rust, se usa el [paquete](glossaire.md#paquete) `serde_json`, que hay
que añadir al proyecto: consulta [los formatos en Rust](../rust/formats.md).

## Dónde encajan Ruby, Rust y Rythme

### Ruby y Rails

Ruby es famoso en la web gracias a **Ruby on Rails**, un *framework* (en
español, «marco de trabajo»): una caja de herramientas que ya trae el
servidor, las páginas y la base de datos. GitHub, Shopify y muchos otros
sitios lo han usado.

```sh
gem install rails
rails new mi_sitio
cd mi_sitio
bin/rails server
```

Consulta [Ruby y la web](../ruby/web.md).

### Rust y sus paquetes: axum

Rust no tiene servidor web en su biblioteca estándar: se añaden paquetes
(los *crates*), como **axum** para el servidor y **tokio** para lo
[asíncrono](glossaire.md#async). El resultado es muy rápido y consume poca
memoria. Este código necesita esos paquetes, así que no se ejecuta aquí:

```rust norun
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "¡Hola desde Rust!" }));
    let escucha = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(escucha, app).await.unwrap();
}
```

Consulta [Rust y la web](../rust/web.md).

### Rythme

Rythme no está hecho para escribir un servidor. Pero sabe leer y escribir
JSON, escribir archivos HTML, y también funciona en el navegador gracias a
su *playground*. Consulta [los formatos en Rythme](../rythme/formats.md).

## Tu primera página web

No necesitas un servidor para empezar: un archivo HTML se abre
directamente en el navegador.

1. En tu [editor](editeur.md), crea un archivo `index.html`.
2. Escribe este contenido:

```text
<!doctype html>
<html lang="es">
  <head>
    <meta charset="utf-8">
    <title>Mi primera página</title>
    <style>
      body { font-family: sans-serif; }
      h1 { color: darkblue; }
    </style>
  </head>
  <body>
    <h1>¡Hola, web!</h1>
    <p>Esta página es un simple archivo en mi ordenador.</p>
    <a href="https://www.ruby-lang.org/es/">El sitio de Ruby</a>
  </body>
</html>
```

3. Guarda y haz doble clic en el archivo (o arrástralo al navegador). La
   dirección empieza por `file://`: es tu disco, no un servidor.

Un programa también puede **fabricar** una página. En Rythme:

```rythme
platos = ["Crepes", "Gofres", "Tarta de manzana"]
lineas = map(platos, fn(p) = "  <li>{p}</li>")
pagina = "<h1>Mis recetas</h1>\n<ul>\n" + join(lineas, "\n") + "\n</ul>"
write_file("recetas.html", pagina)
print(unwrap(read_file("recetas.html")))
```

```sortie
<h1>Mis recetas</h1>
<ul>
  <li>Crepes</li>
  <li>Gofres</li>
  <li>Tarta de manzana</li>
</ul>
```

Abre `recetas.html` en el navegador: aparece tu lista. Es el principio de
todos los sitios web: un programa escribe HTML.

## Errores frecuentes

### Poner un espacio en una URL

```ruby norun
require "uri"
direccion = URI("https://ejemplo.es/mis fotos")
puts direccion.host
```

```text
/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/uri/rfc3986_parser.rb:130:in `split': bad URI(is not URI?): "https://ejemplo.es/mis fotos" (URI::InvalidURIError)
	from main.rb:2:in `<main>'
```

Una URL no contiene espacios. Se sustituyen por `%20`:
`https://ejemplo.es/mis%20fotos`. Mejor aún: evita los espacios en los
nombres de tus archivos web (`mis-fotos`).

### Escribir JSON con comillas simples

```ruby norun
require "json"
datos = JSON.parse("{'nombre': 'Ada'}")
```

```text
/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/json/common.rb:220:in `parse': unexpected token at '{'nombre': 'Ada'}' (JSON::ParserError)
	from main.rb:2:in `<main>'
```

JSON solo acepta comillas dobles rectas: `{"nombre": "Ada"}`.

### Olvidar las comillas alrededor de las claves JSON

```rythme norun
print(parse_json("\{nombre: \"Ada\"\}"))
```

```text
Err("JSON, línea 1, columna 2: se esperaba un nombre entre comillas, pero se encontró `n`")
```

En Rythme, `{nombre: "Ada"}` es un registro válido, pero en JSON las
claves van **siempre** entre comillas: `{"nombre": "Ada"}`. Lo más seguro
es dejar que `to_json` escriba el JSON por ti.

## Ponte a prueba

**1.** En `https://tienda.es/carrito?codigo=PROMO`, ¿cuál es la ruta y
cuál es el parámetro?

> **Respuesta**
>
> La ruta es `/carrito`, el parámetro es `codigo=PROMO`.

**2.** El servidor responde `404`. ¿Quién se ha equivocado, y en qué?

> **Respuesta**
>
> El cliente: ha pedido una página que no existe (a menudo una errata en
> la dirección). Un error del servidor empezaría por 5, como `500`.

**3.** ¿Qué lenguaje decide el color de un título: HTML, CSS o
JavaScript?

> **Respuesta**
>
> CSS, la decoración. HTML dice que es un título, JavaScript lo hace
> interactivo.

## En resumen

- El cliente (el navegador) pide, el servidor responde.
- Una URL se lee por piezas: protocolo, máquina, ruta, parámetros, ancla.
- HTTP es texto: una petición (`GET /…`) y una respuesta con un código (200, 301, 404, 500).
- HTML estructura, CSS decora, JavaScript anima; una API devuelve datos en JSON.
- Ruby (Rails) y Rust (axum) escriben servidores; Rythme lee y escribe JSON y HTML.

Siguiente: [aprender bien a programar](apprendre.md).
