# Módulos y mixins

Esta página muestra cómo organizar tu código en módulos y compartir métodos entre varias clases con `include`.

> **Objetivos:**
>
> - guardar métodos y constantes en un módulo;
> - llamar a una función de módulo con `Modulo.metodo`;
> - compartir métodos entre clases con `include` (un mixin);
> - comparar y ordenar tus objetos con `Comparable`;
> - dar los métodos de `Enumerable` a tu propia clase;
> - repartir el código en varios archivos con `require_relative`.

## ¿Qué es un módulo?

Un **módulo** es una caja con nombre. Guarda métodos y constantes que van juntos.
El nombre de un módulo empieza por mayúscula, como una clase.

```ruby
module Saludos
  def self.hola(nombre)
    "¡Hola, #{nombre}!"
  end
end

puts Saludos.hola("Alice")
```

```sortie
¡Hola, Alice!
```

Un módulo se parece a una clase, con una gran diferencia: **no se crea ningún objeto
a partir de un módulo**. Nada de `Saludos.new`.

> **Nota:** la palabra «módulo» existe en otros lenguajes, con un sentido parecido
> (consulta [el glosario](../debuter/glossaire.md#module)).

## Un módulo como espacio de nombres

Dos bibliotecas pueden tener un método o una clase con el mismo nombre. Si las guardas
en módulos distintos, no se estorban: cada módulo es un **espacio de
nombres**.

```ruby
module Espanol
  def self.saludar
    "Hola"
  end
end

module Ingles
  def self.saludar
    "Hello"
  end
end

puts Espanol.saludar
puts Ingles.saludar
```

```sortie
Hola
Hello
```

Una clase también puede vivir dentro de un módulo. Para llegar a ella, escribe
`Modulo::Clase` (dos veces dos puntos).

```ruby
module Zoo
  class Animal
    def initialize(nombre)
      @nombre = nombre
    end

    def to_s
      "#{@nombre} (animal del zoo)"
    end
  end
end

puts Zoo::Animal.new("Rex")
```

```sortie
Rex (animal del zoo)
```

## Las funciones de módulo

Un método definido con `def self.nombre` pertenece al propio módulo. Lo llamas con
`Modulo.nombre`, sin crear ningún objeto. Es práctico para una caja de herramientas.

```ruby
module Calculo
  def self.doble(x)
    x * 2
  end

  def self.promedio(numeros)
    numeros.sum.to_f / numeros.size
  end
end

puts Calculo.doble(21)
puts Calculo.promedio([12, 15, 18])
```

```sortie
42
15.0
```

El propio Ruby guarda así sus herramientas matemáticas, en el módulo `Math`:

```ruby
puts Math.sqrt(16)
puts Math::PI.round(2)
```

```sortie
4.0
3.14
```

## Las constantes de un módulo

Un módulo también guarda **constantes** (valores que no cambian). Su nombre se escribe
en mayúsculas. Fuera del módulo, escribe `Modulo::CONSTANTE`.

```ruby
module Config
  VERSION = "1.2.0"
  IDIOMAS = ["es", "en"]
end

puts Config::VERSION
p Config::IDIOMAS
```

```sortie
1.2.0
["es", "en"]
```

Dentro del módulo, basta con el nombre solo:

```ruby
module Circulo
  PI = 3.14159

  def self.area(radio)
    (PI * radio * radio).round(2)
  end
end

puts Circulo.area(2)
```

```sortie
12.57
```

## Compartir métodos con include

Este es el otro gran uso de los módulos. Escribe métodos **sin** `self.` en un
módulo y luego añade `include NombreDelModulo` en una clase: cada objeto de la clase
recibe esos métodos. A ese módulo se le llama **mixin** (una «mezcla»).

```ruby
module Presentable
  def presentarse
    "Me llamo #{nombre}."
  end
end

class Alumno
  include Presentable
  attr_reader :nombre

  def initialize(nombre)
    @nombre = nombre
  end
end

class Perro
  include Presentable
  attr_reader :nombre

  def initialize(nombre)
    @nombre = nombre
  end
end

puts Alumno.new("Alice").presentarse
puts Perro.new("Rex").presentarse
```

```sortie
Me llamo Alice.
Me llamo Rex.
```

`Alumno` y `Perro` no tienen nada en común, pero comparten el método `presentarse`.
Dentro del módulo, `nombre` llama al método `nombre` del objeto que incluyó el módulo.

> **Recuerda:** una clase solo tiene un padre (`class Gato < Animal`), pero
> puede incluir tantos módulos como quiera. Consulta también
> [la herencia](types.md#la-herencia).

### Comprobar que una clase incluye un módulo

`include?` responde a la pregunta «¿ha recibido esta clase este módulo?».

```ruby
module Volador
  def volar
    "¡Estoy volando!"
  end
end

class Pajaro
  include Volador
end

class Pez
end

puts Pajaro.include?(Volador)
puts Pez.include?(Volador)
puts Pajaro.new.volar
```

```sortie
true
false
¡Estoy volando!
```

## Comparable: comparar tus objetos

Ruby trae mixins ya listos. El primero es `Comparable`. Te da `<`,
`>`, `<=`, `>=`, `==`, `between?` y `clamp`. A cambio, escribes **un solo**
método: `<=>`.

### El operador <=>

`a <=> b` compara dos valores. Devuelve `-1` si `a` es más pequeño, `0` si son
iguales, `1` si `a` es más grande.

```ruby
p 3 <=> 5
p 5 <=> 5
p 7 <=> 5
p "albaricoque" <=> "banana"
```

```sortie
-1
0
1
-1
```

### Incluir Comparable

Define `<=>` comparando lo que te importa (aquí, el valor de la nota)
y luego añade `include Comparable`.

```ruby
class Nota
  include Comparable
  attr_reader :valor

  def initialize(valor)
    @valor = valor
  end

  def <=>(otra)
    valor <=> otra.valor
  end

  def to_s
    "#{valor}/20"
  end
end

a = Nota.new(12)
b = Nota.new(15)

puts a < b
puts a == Nota.new(12)
puts b.between?(Nota.new(10), Nota.new(16))
puts [b, a, Nota.new(9)].sort.join(", ")
puts [b, a].max
```

```sortie
true
true
true
9/20, 12/20, 15/20
15/20
```

> **Consejo:** con `<=>`, `sort`, `min` y `max` también funcionan en un array de
> tus objetos.

## Enumerable: hacer que tu clase se pueda recorrer

El segundo mixin famoso es `Enumerable`, el que da `map`, `select`, `sort`…
a los arrays (consulta [los bloques](blocs.md#los-metodos-de-enumerable)). Puedes dárselo
a tu propia clase. Basta con escribir **un** método: `each`, que pasa
cada elemento al bloque con `yield`.

```ruby
class Playlist
  include Enumerable

  def initialize
    @titulos = []
  end

  def agregar(titulo)
    @titulos << titulo
    self
  end

  def each
    @titulos.each { |titulo| yield titulo }
  end
end

lista = Playlist.new
lista.agregar("Imagine").agregar("Yesterday").agregar("Hey Jude")

lista.each { |t| puts "- #{t}" }
p lista.map(&:upcase)
p lista.select { |t| t.start_with?("Y") }
p lista.sort
p lista.count
p lista.include?("Imagine")
```

```sortie
- Imagine
- Yesterday
- Hey Jude
["IMAGINE", "YESTERDAY", "HEY JUDE"]
["Yesterday"]
["Hey Jude", "Imagine", "Yesterday"]
3
true
```

Con un solo método `each`, tu `Playlist` recibe decenas de métodos:
`map`, `select`, `sort`, `count`, `include?`, `first`, `min`, `sum`…

> **Nota:** `agregar` devuelve `self` (el propio objeto). Eso es lo que permite
> encadenar las llamadas: `lista.agregar(…).agregar(…)`.

## Un módulo por archivo con require_relative

En un proyecto real, cada módulo va en su propio archivo. `require_relative`
carga un archivo situado junto al archivo actual (sin la extensión `.rb`).

```text
mi_proyecto/
├── main.rb
└── lib/
    └── calculo.rb
```

El archivo `lib/calculo.rb` contiene el módulo:

```ruby norun
# lib/calculo.rb
module Calculo
  def self.doble(x)
    x * 2
  end
end
```

El archivo `main.rb` lo carga y luego lo usa:

```ruby norun
# main.rb
require_relative "lib/calculo"

puts Calculo.doble(21)
```

En la terminal, desde la carpeta `mi_proyecto`:

```sh
ruby main.rb
```

```text
42
```

> **Recuerda:** la ruta de `require_relative` parte de la carpeta **del archivo que
> la escribe**, no de la carpeta desde la que lanzas el comando. El capítulo
> [Organizar un proyecto](projet.md) va más allá.

## Errores frecuentes

### Llamar a un método sin self. en el módulo

Un método escrito sin `self.` está pensado para `include`. El propio módulo no lo
conoce.

```ruby norun
module Herramientas
  def doble(x)
    x * 2
  end
end

Herramientas.doble(2)
```

```text
main.rb:7:in `<main>': undefined method `doble' for module Herramientas (NoMethodError)
```

Para llamarlo con `Herramientas.doble`, escribe `def self.doble(x)`.

### Olvidar el nombre del módulo delante de una constante

Fuera del módulo, la constante no es visible con su nombre solo.

```ruby norun
module Geometria
  PI = 3.14
end

puts PI
```

```text
main.rb:5:in `<main>': uninitialized constant PI (NameError)
```

Escribe `Geometria::PI`.

### Incluir Comparable sin escribir <=>

`Comparable` necesita `<=>` para comparar. Sin él, Ruby no sabe hacerlo.

```ruby norun
class Nota
  include Comparable
  attr_reader :valor

  def initialize(valor)
    @valor = valor
  end
end

p Nota.new(12) < Nota.new(15)
```

```text
main.rb:10:in `<': comparison of Nota with Nota failed (ArgumentError)
```

Añade `def <=>(otra) = valor <=> otra.valor`. Del mismo modo, incluir `Enumerable` sin
escribir `each` da un `NoMethodError` («undefined method each»).

### Crear un objeto a partir de un módulo

Un módulo no fabrica objetos.

```ruby norun
module Herramientas
end

Herramientas.new
```

```text
main.rb:4:in `<main>': undefined method `new' for module Herramientas (NoMethodError)
```

Si necesitas objetos, escribe una `class`.

## Ponte a prueba

**1.** ¿Qué diferencia hay entre `def self.area` y `def area` en un módulo?

> **Respuesta**
>
> `def self.area` crea una función del módulo: se llama con `Modulo.area`.
> `def area` crea un método para compartir: una clase lo recibe con `include`.

**2.** Quieres ordenar un array de tus objetos `Jugador` por puntuación. ¿Qué tienes que escribir?

> **Respuesta**
>
> Añade `include Comparable` y un método `<=>` que compare las puntuaciones:
> `def <=>(otro) = puntos <=> otro.puntos`. Después, `jugadores.sort` funciona.

**3.** ¿Qué método tienes que escribir para que una clase que incluye `Enumerable` tenga `map`
y `select`?

> **Respuesta**
>
> El método `each`, que pasa cada elemento al bloque con `yield`.

## En resumen

- Un módulo guarda métodos y constantes; no se crean objetos a partir de él.
- `def self.nombre` crea una función de módulo, que se llama con `Modulo.nombre`; una
  constante se escribe `Modulo::CONSTANTE`.
- `include Modulo` da los métodos del módulo a una clase: es un mixin.
- `Comparable` pide `<=>`; `Enumerable` pide `each`.
- `require_relative "lib/archivo"` carga un archivo de tu proyecto.

Siguiente: [Struct, Data y Set](structures.md)
