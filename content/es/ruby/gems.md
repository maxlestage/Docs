# Crear y publicar tu propia gema

Esta página muestra, paso a paso, cómo convertir tu código en una gema: escribirla, probarla, empaquetarla, publicarla y usarla en otro proyecto.

> **Objetivos:**
>
> - saber qué es una gema y qué contiene su gemspec;
> - crear el esqueleto de una gema con `bundle gem`;
> - escribir el código en `lib/`, con un número de versión, y probarlo;
> - construir el paquete con `gem build` y publicarlo en rubygems.org;
> - elegir un número de versión y usar una gema local con Bundler.

## ¿Qué es una gema?

Una [gema](../debuter/glossaire.md#gem) es una [biblioteca](../debuter/glossaire.md#biblioteca)
de Ruby empaquetada para compartirla. Ya usas algunas: la página
[Organizar un proyecto](projet.md) muestra `gem install` y Bundler. Aquí pasas al
otro lado: tú **fabricas** la gema.

Piensa en un paquete postal:

```text
  +--------------------------------------------+
  |  ETIQUETA (el gemspec)                     |
  |  nombre: saludos      versión: 0.1.0       |
  |  autora: Alice        resumen: Decir hola  |
  +--------------------------------------------+
  |  CONTENIDO                                 |
  |    lib/saludos.rb                          |
  |    lib/saludos/version.rb                  |
  |    README.md                               |
  +--------------------------------------------+
         |
         |  gem build   (cierras el paquete)
         v
  saludos-0.1.0.gem
         |
         |  gem push    (lo llevas a correos: rubygems.org)
         v
  cualquiera puede escribir: gem install saludos
```

- El **contenido** es tu código Ruby, guardado en `lib/`.
- La **etiqueta** es el *gemspec*: el nombre, la versión, el autor, la lista de
  archivos.
- La **oficina de correos** es [rubygems.org](https://rubygems.org), el sitio que
  distribuye las gemas.

> **Recuerda:** una gema = código en `lib/` + una etiqueta (el gemspec). Se
> construye con `gem build` y se publica con `gem push`.

## El código de la gema: un módulo y un método

Todo el código de una gema vive en un [módulo](../debuter/glossaire.md#modulo) que lleva su
nombre (mira [los módulos](modules.md)). Así, sus nombres no se mezclan con los de
otras gemas. Nuestra gema `saludos` dice hola en tres idiomas:

```ruby
# lib/saludos.rb
module Saludos
  FORMULAS = { es: "Hola", en: "Hello", fr: "Bonjour" }

  def self.hola(nombre, idioma: :es)
    formula = FORMULAS.fetch(idioma)
    "#{formula}, #{nombre}!"
  end
end

puts Saludos.hola("Alice")
puts Saludos.hola("Bob", idioma: :en)
```

```sortie
Hola, Alice!
Hello, Bob!
```

Sigamos la segunda llamada, `Saludos.hola("Bob", idioma: :en)`:

| Paso | Línea | nombre | idioma | formula | Resultado |
|------|-------|--------|--------|---------|-----------|
| 1 | entrada en `hola` | `"Bob"` | `:en` | — | — |
| 2 | `FORMULAS.fetch(idioma)` | `"Bob"` | `:en` | `"Hello"` | — |
| 3 | `"#{formula}, #{nombre}!"` | `"Bob"` | `:en` | `"Hello"` | `"Hello, Bob!"` |

- `self.hola` se llama sobre el propio módulo: `Saludos.hola`.
- `idioma: :es` es un argumento con nombre, `:es` por defecto (mira [los métodos](fonctions.md)).
- `fetch` lanza un error claro si el idioma no existe.

Prueba: añade `it: "Ciao"` en `FORMULAS` y luego llama a
`Saludos.hola("Léa", idioma: :it)`. Obtienes `Ciao, Léa!`.

> **Recuerda:** el código de una gema se guarda en un módulo con el mismo nombre que la gema.

## El número de versión

Cada gema tiene un número de versión, guardado en una [constante](../debuter/glossaire.md#constante)
`VERSION`, en su propio archivo:

```ruby
# lib/saludos/version.rb
module Saludos
  VERSION = "0.1.0"
end

puts "saludos versión #{Saludos::VERSION}"
```

```sortie
saludos versión 0.1.0
```

El archivo principal `lib/saludos.rb` la carga en su primera línea:
`require_relative "saludos/version"`.

¿Por qué un archivo aparte? Porque el gemspec tiene que leer la versión **sin** cargar
todo el código. Es como escribir el número del paquete en un pósit pegado por fuera: se
lee sin abrir la caja.

> **Recuerda:** la versión es una cadena en `Saludos::VERSION`, en
> `lib/saludos/version.rb`.

## Crear el esqueleto con bundle gem

Bundler crea por ti todas las carpetas y archivos de partida. Escribe en la
[terminal](../debuter/glossaire.md#terminal):

```sh
bundle gem saludos --test=minitest
```

```text
Creating gem 'saludos'...
      create  saludos/Gemfile
      create  saludos/lib/saludos.rb
      create  saludos/lib/saludos/version.rb
      ...
      create  saludos/saludos.gemspec
      create  saludos/test/test_saludos.rb
Gem 'saludos' was successfully created.
```

A veces Bundler hace preguntas antes: responde `n` si dudas. Esta es la organización que obtienes:

```text
saludos/
├── lib/                        ← el CONTENIDO del paquete
│   ├── saludos.rb              ← el módulo Saludos
│   └── saludos/
│       └── version.rb          ← VERSION = "0.1.0"
├── test/                       ← las pruebas (no van en el paquete)
│   ├── test_helper.rb
│   └── test_saludos.rb
├── saludos.gemspec             ← la ETIQUETA
├── Gemfile                     ← las gemas útiles para desarrollar
├── Rakefile                    ← las tareas: rake test, rake build…
└── README.md                   ← el modo de empleo
```

La regla de los nombres: la gema `saludos` → el archivo `lib/saludos.rb` → el módulo
`Saludos`. Para una gema `buen_humor`, sería `lib/buen_humor.rb` y el módulo
`BuenHumor`.

> **Recuerda:** `bundle gem nombre` crea el esqueleto; el código va en `lib/`, las
> pruebas en `test/`.

## El gemspec: la etiqueta del paquete

El archivo `saludos.gemspec` describe la gema. `bundle gem` escribe uno lleno de
`TODO` que hay que reemplazar. Aquí tienes una versión sencilla y completa:

```ruby norun
# saludos.gemspec
require_relative "lib/saludos/version"

Gem::Specification.new do |spec|
  spec.name     = "saludos"
  spec.version  = Saludos::VERSION
  spec.authors  = ["Alice Martin"]
  spec.email    = ["alice@example.com"]
  spec.summary  = "Decir hola con educación, en varios idiomas."
  spec.homepage = "https://github.com/alice/saludos"
  spec.license  = "MIT"
  spec.required_ruby_version = ">= 3.0"

  spec.files = Dir["lib/**/*.rb"] + ["README.md"]
  spec.require_paths = ["lib"]
end
```

| Campo | Lo que dice | ¿Obligatorio? |
|-------|-------------|---------------|
| `name` | el nombre de la gema, el de `gem install` | sí |
| `version` | el número de versión | sí |
| `authors` | quién la escribió | sí |
| `summary` | una frase de resumen | sí |
| `files` | los archivos que van en el paquete | sí, si no el paquete está vacío |
| `license` | lo que los demás pueden hacer con ella | recomendado |
| `require_paths` | dónde buscar el código para `require` | `lib` por defecto |

`Dir["lib/**/*.rb"]`: «todos los archivos `.rb` de `lib/` y de sus subcarpetas».

### Una etiqueta es un objeto Ruby

`Gem::Specification` es una clase Ruby normal, siempre disponible. Puedes
fabricar una etiqueta y leerla:

```ruby
spec = Gem::Specification.new do |s|
  s.name    = "saludos"
  s.version = "0.1.0"
  s.summary = "Decir hola"
  s.authors = ["Alice Martin"]
end

puts spec.name
puts spec.version
puts spec.full_name
puts spec.file_name
```

```sortie
saludos
0.1.0
saludos-0.1.0
saludos-0.1.0.gem
```

`file_name` es el nombre del paquete que va a fabricar `gem build`.

Prueba: reemplaza `"0.1.0"` por `"1.0.0"`. Las dos últimas líneas pasan a ser
`saludos-1.0.0` y `saludos-1.0.0.gem`.

> **Recuerda:** el gemspec es Ruby: un `Gem::Specification.new` con al menos
> el nombre, la versión, los autores, el resumen y los archivos.

## Probar la gema

Antes de enviar un paquete, se comprueba lo que contiene. Para una gema, son las
[pruebas](../debuter/glossaire.md#test) (mira [Probar tu código](tests.md)).

### Con comprobaciones sencillas

Lo más sencillo: comparar el resultado esperado con el resultado obtenido.

```ruby
module Saludos
  FORMULAS = { es: "Hola", en: "Hello", fr: "Bonjour" }

  def self.hola(nombre, idioma: :es)
    "#{FORMULAS.fetch(idioma)}, #{nombre}!"
  end
end

def comprobar(esperado, obtenido)
  estado = esperado == obtenido ? "ok" : "FALLO"
  puts "#{estado}: #{obtenido.inspect}"
end

comprobar "Hola, Alice!", Saludos.hola("Alice")
comprobar "Hello, Ana!", Saludos.hola("Ana", idioma: :en)
comprobar "Bonjour, Bob !", Saludos.hola("Bob", idioma: :fr)
```

```sortie
ok: "Hola, Alice!"
ok: "Hello, Ana!"
FALLO: "Bonjour, Bob!"
```

| Comprobación | esperado | obtenido | ¿Iguales? |
|--------------|----------|----------|-----------|
| 1 | `"Hola, Alice!"` | `"Hola, Alice!"` | sí → `ok` |
| 2 | `"Hello, Ana!"` | `"Hello, Ana!"` | sí → `ok` |
| 3 | `"Bonjour, Bob !"` | `"Bonjour, Bob!"` | no → `FALLO` |

El tercer fallo viene de la **prueba**, no del código: sobra un espacio antes de `!`
en el valor esperado. Una prueba también puede equivocarse: reléela.

Prueba: corrige la tercera línea a `"Bonjour, Bob!"`. Las tres líneas muestran
`ok`.

### Con Minitest

En el esqueleto, `test/test_saludos.rb` usa Minitest, que viene con Ruby:

```ruby
require "minitest/autorun"

module Saludos
  FORMULAS = { es: "Hola", en: "Hello", fr: "Bonjour" }

  def self.hola(nombre, idioma: :es)
    "#{FORMULAS.fetch(idioma)}, #{nombre}!"
  end
end

class TestSaludos < Minitest::Test
  def test_hola_en_espanol_por_defecto
    assert_equal "Hola, Alice!", Saludos.hola("Alice")
  end

  def test_hola_en_ingles
    assert_equal "Hello, Ana!", Saludos.hola("Ana", idioma: :en)
  end

  def test_idioma_desconocido
    assert_raises(KeyError) { Saludos.hola("Max", idioma: :de) }
  end
end
```

En la gema real, el módulo no se copia: `test/test_helper.rb` lo carga desde
`lib/`. Las pruebas se lanzan desde la carpeta de la gema:

```sh
bundle exec rake test
```

```text
Run options: --seed 40213

# Running:

...

Finished in 0.000731s, 4103.9672 runs/s, 4103.9672 assertions/s.

3 runs, 3 assertions, 0 failures, 0 errors, 0 skips
```

Tres puntos, tres pruebas superadas. El número después de `--seed` y el tiempo cambian
cada vez.

> **Recuerda:** prueba la gema antes de construirla: `bundle exec rake test` tiene que
> terminar con `0 failures, 0 errors`.

## Construir el paquete: gem build

`gem build` lee la etiqueta, mete los archivos en el paquete y lo cierra:

```sh
gem build saludos.gemspec
```

```text
  Successfully built RubyGem
  Name: saludos
  Version: 0.1.0
  File: saludos-0.1.0.gem
```

El archivo `saludos-0.1.0.gem` es un archivo comprimido. Dentro hay tres piezas:

```text
saludos-0.1.0.gem
├── metadata.gz       ← la etiqueta (el gemspec, en YAML)
├── data.tar.gz       ← el contenido: lib/ y README.md
└── checksums.yaml.gz ← huellas para comprobar que nada está dañado
```

Antes de publicar, prueba el paquete en casa. `gem install` acepta un archivo local:

```sh
gem install ./saludos-0.1.0.gem
ruby -e 'require "saludos"; puts Saludos.hola("Alice", idioma: :en)'
```

```text
Successfully installed saludos-0.1.0
1 gem installed
Hello, Alice!
```

> **Recuerda:** `gem build archivo.gemspec` fabrica el archivo `.gem`;
> `gem install ./archivo.gem` lo prueba en tu ordenador.

## Publicar en rubygems.org

Publicar es llevar el paquete a correos. Estos comandos necesitan Internet y
una cuenta: no se ejecutan aquí.

Primero crea una cuenta en [rubygems.org](https://rubygems.org/sign_up). Después,
`gem signin` te conecta (una sola vez) y `gem push` envía el paquete:

```sh
gem signin
gem push saludos-0.1.0.gem
```

```text
Pushing gem to https://rubygems.org...
Successfully registered gem: saludos (0.1.0)
```

Unos minutos más tarde, cualquiera puede escribir `gem install saludos`.

> **Atención:** el nombre tiene que estar libre (búscalo antes en rubygems.org), y una
> versión publicada **nunca** se puede reemplazar. Para corregir, publica una nueva
> versión (`0.1.1`). Nunca pongas una contraseña ni una clave secreta en una gema.

> **Recuerda:** `gem signin` una vez, y luego `gem push archivo.gem` en cada nueva
> versión.

## Los números de versión

Las gemas siguen el **versionado semántico** (*semver*): tres números,
`MAYOR.MENOR.PARCHE`. Cada número es un mensaje para los usuarios.

```text
   1   .   4   .   2
   |       |       |
   |       |       +-- PARCHE : un bug corregido, nada más cambia
   |       +---------- MENOR  : una novedad, el código antiguo sigue funcionando
   +------------------ MAYOR  : un cambio que puede romper el código antiguo
```

| Has… | Antes | Después |
|------|-------|---------|
| corregido una errata en `"Hola"` | `1.4.2` | `1.4.3` |
| añadido el idioma `:it` | `1.4.3` | `1.5.0` |
| renombrado `hola` a `saludar` | `1.5.0` | `2.0.0` |
| la primerísima versión, aún inestable | — | `0.1.0` |

Cuando un número sube, los de su derecha vuelven a `0`.

### Comparar versiones

Atención: comparar versiones como textos da un resultado falso.
`Gem::Version` compara número a número:

```ruby
p "1.10.0" > "1.9.0"
p Gem::Version.new("1.10.0") > Gem::Version.new("1.9.0")
p Gem::Version.new("1.4.2").segments
```

```sortie
false
true
[1, 4, 2]
```

Como textos, `"1.1…"` va antes que `"1.9…"`, porque el carácter `1` va antes que `9`.
Como versiones, `10` es mayor que `9`.

### El operador ~>

En un `Gemfile`, `gem "saludos", "~> 1.2"` quiere decir: «`1.2` o más, pero
menos que `2.0`». Aceptas las novedades, no los cambios que rompen.

```ruby
regla = Gem::Requirement.new("~> 1.2")

%w[1.1.0 1.2.0 1.9.3 2.0.0].each do |v|
  puts "#{v}: #{regla.satisfied_by?(Gem::Version.new(v))}"
end
```

```sortie
1.1.0: false
1.2.0: true
1.9.3: true
2.0.0: false
```

| Paso | v | ¿Al menos 1.2? | ¿Menos que 2.0? | ¿Aceptada? |
|------|---|----------------|-----------------|------------|
| 1 | `1.1.0` | no | sí | `false` |
| 2 | `1.2.0` | sí | sí | `true` |
| 3 | `1.9.3` | sí | sí | `true` |
| 4 | `2.0.0` | sí | no | `false` |

Prueba: reemplaza `"~> 1.2"` por `"~> 1.2.0"`. Solo `1.2.0` sigue aceptada: la
regla pasa a ser «`1.2.0` o más, pero menos que `1.3`».

> **Recuerda:** MAYOR.MENOR.PARCHE. Cambia el MAYOR cuando rompes algo,
> el MENOR para una novedad, el PARCHE para un bug.

## Usar una gema local con Bundler

Mientras desarrollas la gema, quieres probarla en otro proyecto, sin
publicarla. Bundler sabe tomar una gema de una carpeta, con `path:`.

```text
mis-proyectos/
├── saludos/          ← la gema
└── bienvenida/       ← el proyecto que la usa
    ├── Gemfile
    └── main.rb
```

```ruby norun
# bienvenida/Gemfile
source "https://rubygems.org"

gem "saludos", path: "../saludos"
```

```ruby norun
# bienvenida/main.rb
require "saludos"

puts Saludos.hola("Bob")
```

```sh
cd bienvenida
bundle install
bundle exec ruby main.rb
```

```text
Resolving dependencies...
Bundle complete! 1 Gemfile dependency, 2 gems now installed.
Use `bundle info [gemname]` to see where a bundled gem is installed.
Hola, Bob!
```

`bundle exec` lanza el programa con las gemas del `Gemfile`. Con `path:`, Bundler lee
directamente la carpeta: no hace falta `gem build`.

Prueba: cambia `"Hola"` por `"Buenas"` en la gema y vuelve a lanzar
`bundle exec ruby main.rb`. El programa muestra `Buenas, Bob!`.

Cuando la gema esté publicada, reemplaza la línea por `gem "saludos", "~> 0.1"`.

> **Recuerda:** `gem "nombre", path: "../carpeta"` usa una gema que aún no está publicada,
> y sigue tus cambios en directo.

## Errores frecuentes

### Falta un campo obligatorio en el gemspec

```ruby norun
Gem::Specification.new do |spec|
  spec.name    = "saludos"
  spec.version = "0.1.0"
  spec.authors = ["Alice Martin"]
  spec.files   = Dir["lib/**/*.rb"]
end
```

```text
WARNING:  See https://guides.rubygems.org/specification-reference/ for help
ERROR:  While executing gem ... (Gem::InvalidSpecificationException)
    missing value for attribute summary
```

`gem build` rechaza un paquete sin resumen. Añade `spec.summary = "…"`. Lee la línea
`missing value for attribute …`: nombra el campo que falta.

### Una clave ausente con fetch

```ruby norun
FORMULAS = { es: "Hola", en: "Hello" }
puts FORMULAS.fetch(:de)
```

```text
main.rb:2:in `fetch': key not found: :de (KeyError)
	from main.rb:2:in `<main>'
```

El idioma `:de` no está en el hash. Añádelo (`de: "Hallo"`), o da un
valor de reserva: `FORMULAS.fetch(idioma, "Hola")`.

### Una ruta incorrecta en path:

```ruby norun
# bienvenida/Gemfile
gem "saludos", path: "../saludo"
```

```text
The path `/home/alice/mis-proyectos/saludo` does not exist.
```

Le falta una `s` al nombre de la carpeta. La ruta parte de la carpeta del `Gemfile`:
compruébala con `ls ../`.

## Ponte a prueba

**Pregunta 1:** ¿qué archivo sirve de «etiqueta» a una gema y qué contiene?

> **Respuesta**
>
> El gemspec (`saludos.gemspec`). Da el nombre, la versión, los autores, el
> resumen, la licencia y la lista de archivos que van en la gema.

**Pregunta 2:** tu gema está en `2.3.1`. Añades un método sin romper nada.
¿Cuál es la nueva versión?

> **Respuesta**
>
> `2.4.0`: una novedad compatible hace subir el MENOR, y el PARCHE vuelve a
> `0`.

**Pregunta 3:** escribe el código que muestra el nombre del archivo `.gem` de una gema
`clima` en la versión `0.2.0`.

> **Respuesta**
>
> ```ruby
> spec = Gem::Specification.new do |s|
>   s.name = "clima"
>   s.version = "0.2.0"
> end
> puts spec.file_name
> ```
>
> ```sortie
> clima-0.2.0.gem
> ```

## En resumen

- Una gema es código en `lib/` (un módulo con el mismo nombre) y un gemspec, su
  etiqueta.
- `bundle gem nombre` crea el esqueleto; la versión vive en `lib/nombre/version.rb`.
- Prueba con Minitest (`bundle exec rake test`) antes de construir.
- `gem build` fabrica el `.gem`, `gem push` lo publica en rubygems.org.
- Las versiones siguen MAYOR.MENOR.PARCHE; `~> 1.2` acepta todo hasta `2.0`
  excluido.
- `gem "nombre", path: "…"` usa una gema local con Bundler.

Siguiente: [proyectos paso a paso](projets.md)
