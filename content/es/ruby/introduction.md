# Ruby: introducción e instalación

Esta página presenta Ruby, explica cómo instalarlo y te hace escribir tu primer programa.

> **Objetivos:**
>
> - sabrás decir qué es Ruby y para qué sirve;
> - sabrás instalar Ruby y comprobar que funciona;
> - sabrás probar una línea de código en `irb`;
> - sabrás escribir un archivo `.rb` y ejecutarlo.

## ¿Qué es Ruby?

Ruby es un lenguaje de programación creado en Japón por Yukihiro Matsumoto, en 1995.
Su objetivo: hacer felices a los programadores. El código se lee casi como inglés.

Un **lenguaje de programación** es un idioma preciso que el ordenador sabe seguir.
Un **programa** es un texto escrito en ese idioma: una serie de órdenes, como una
receta de cocina.

```ruby
3.times { puts "Hola" }
```

```sortie
Hola
Hola
Hola
```

Lee esta línea en voz alta: «3 veces, muestra Hola». `puts` (del inglés
*put string*, «pon el texto») muestra un texto.

Ruby es un lenguaje **interpretado**: escribes un archivo y luego lo ejecutas
directamente. Un programa llamado [intérprete](../debuter/glossaire.md#interprete)
lee tu archivo y ejecuta las órdenes una por una. No hay un paso de preparación
(de «compilación») antes.

## ¿Por qué aprender Ruby?

- La sintaxis (la forma de escribir el código) es corta y legible.
- Todo es un [objeto](../debuter/glossaire.md#objeto), incluso los números: cada valor
  sabe hacer cosas.
- Se usa mucho para la web (con Ruby on Rails) y para scripts.
- Su comunidad comparte miles de bibliotecas, llamadas **gems**.

«Todo es un objeto» quiere decir que puedes hacerle una pregunta a un número o a un
texto. El punto `.` sirve para llamar a un [método](../debuter/glossaire.md#metodo),
es decir, una acción que el valor sabe hacer.

```ruby
puts 5.even?
puts "ruby".upcase
```

```sortie
false
RUBY
```

- `5.even?` le pregunta al número 5: «¿eres par?». Responde `false` (falso).
- `"ruby".upcase` le pide al texto que se ponga en mayúsculas.

> **Nota:** este sitio presenta también otros lenguajes: compara con
> [la introducción a Rust](../rust/introduction.md) o [la introducción a Rythme](../rythme/introduction.md).

## Instalar Ruby

Este sitio usa **Ruby 3.3**. Una versión más reciente también funciona.

### En Linux y macOS

Lo más sencillo es usar el gestor de paquetes de tu sistema: un programa que
instala los demás programas por ti.

```sh
sudo apt install ruby-full     # Debian, Ubuntu
sudo dnf install ruby          # Fedora
brew install ruby              # macOS con Homebrew
```

### Con rbenv

`rbenv` permite instalar varias versiones de Ruby y pasar de una a otra.

```sh
rbenv install 3.3.0
rbenv global 3.3.0
```

> **Consejo:** usa rbenv si trabajas en varios proyectos que piden versiones
> distintas de Ruby.

### En Windows

Descarga **RubyInstaller** desde el sitio rubyinstaller.org y luego ejecuta el instalador.
Deja las opciones propuestas por defecto.

## Comprobar la instalación

Abre una terminal y escribe el siguiente comando. Si la terminal es nueva para ti,
lee primero [la terminal](../debuter/terminal.md).

```sh
ruby -v
```

Debes ver un número de versión, por ejemplo `ruby 3.3.0`. Si la terminal responde
«command not found» (comando no encontrado), Ruby no está instalado, o la terminal
se abrió antes de la instalación: ciérrala y vuelve a abrirla.

## Probar Ruby con irb

`irb` (*interactive Ruby*) es una consola interactiva. Escribes una línea y Ruby
responde enseguida. Es como una calculadora que entiende todo Ruby.

```text
$ irb
irb(main):001> 2 + 3
=> 5
irb(main):002> "hola!".length
=> 5
irb(main):003> exit
```

- La flecha `=>` muestra el resultado de la línea.
- `exit` sale de irb.

> **Consejo:** irb es perfecto para probar una pequeña idea sin crear un archivo.

## Tu primer programa

Abre tu [editor de código](../debuter/editeur.md). Crea un archivo `hello.rb` con
este contenido:

```ruby
puts "¡Hola, mundo!"
```

```sortie
¡Hola, mundo!
```

- `puts` muestra un texto y luego pasa a la línea siguiente.
- El texto va entre comillas `"..."`. Sin comillas, Ruby creería que es un
  nombre de código.

Un programa puede tener varias líneas. Ruby las ejecuta en orden, de arriba abajo.

```ruby
puts "Línea 1"
puts "Línea 2"
puts 2 + 3
```

```sortie
Línea 1
Línea 2
5
```

`puts 2 + 3` muestra `5`: sin comillas, Ruby calcula.

## Ejecutar un programa

En la terminal, colócate en la carpeta del archivo (con el comando `cd`) y luego
escribe:

```sh
ruby hello.rb
```

El programa se ejecuta y muestra `¡Hola, mundo!`.

> **Recuerda:** un programa Ruby es un archivo `.rb`. Lo ejecutas con
> `ruby nombre_del_archivo.rb`.

La guía [la terminal](../debuter/terminal.md) explica cómo abrir una terminal y
cambiar de carpeta.

## Leer un mensaje de error

Cuando Ruby no entiende tu código, se detiene y muestra un
[error](../debuter/glossaire.md#error). No pasa nada: el mensaje te ayuda.

```text
main.rb:1:in `<main>': undefined method `put' for main (NoMethodError)
```

Léelo de izquierda a derecha:

- `main.rb:1`: el archivo y el número de línea del problema;
- ``undefined method `put'``: la descripción («método `put` desconocido»);
- `(NoMethodError)`: el nombre del tipo de error.

Los mensajes están en inglés. Algunas palabras aparecen a menudo: *undefined* (no
definido), *unexpected* (inesperado), *expecting* (esperaba), *missing* (falta).

## Errores frecuentes

### Una errata en puts

```ruby norun
put "Hola"
```

```text
main.rb:1:in `<main>': undefined method `put' for main (NoMethodError)

put "Hola"
^^^
Did you mean?  puts
               putc
```

Ruby no conoce `put`. Incluso propone la ortografía correcta (*Did you mean?*:
«¿querías decir?»). Corrígelo en `puts "Hola"`.

### Olvidar las comillas

```ruby norun
puts Hola
```

```text
main.rb:1:in `<main>': uninitialized constant Hola (NameError)

puts Hola
     ^^^^
```

Sin comillas, `Hola` se lee como un nombre de código, no como un texto. Escribe
`puts "Hola"`.

### Olvidar la comilla final

```ruby norun
puts "Hola
```

```text
main.rb: --> main.rb
expected a closing delimiter for the string literal
> 1  puts "Hola
main.rb:1: unterminated string meets end of file (SyntaxError)
puts "Hola
          ^
```

El texto nunca se cierra (*unterminated string*). Añade la comilla que falta:
`puts "Hola"`.

## Ponte a prueba

**Pregunta 1:** ¿qué comando comprueba que Ruby está instalado?

> **Respuesta**
>
> `ruby -v`. Muestra el número de versión.

**Pregunta 2:** ¿qué muestra este programa?

```ruby norun
puts "2 + 3"
puts 2 + 3
```

> **Respuesta**
>
> `2 + 3`, y luego `5`. Entre comillas, es un texto que se muestra tal cual. Sin
> comillas, Ruby calcula.

**Pregunta 3:** has escrito `juego.rb`. ¿Cómo lo ejecutas?

> **Respuesta**
>
> Colócate en su carpeta con `cd` y luego escribe `ruby juego.rb`.

## En resumen

- Ruby es un lenguaje sencillo y legible, donde todo es un objeto.
- Instálalo con tu gestor de paquetes, rbenv o RubyInstaller.
- `ruby -v` comprueba la instalación, `irb` permite probar código enseguida.
- Un programa es un archivo `.rb`, que se ejecuta con `ruby nombre_del_archivo.rb`.
- `puts` muestra un texto y pasa a la línea siguiente.
- Un mensaje de error da el archivo, la línea y la causa: léelo.

Siguiente: [primeros pasos](premiers-pas.md).
