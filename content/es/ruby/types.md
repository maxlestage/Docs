# Clases y objetos

Esta página muestra cómo crear tus propios objetos con las clases, la herencia, los módulos y `Struct`.

> **Objetivos:**
>
> - sabrás escribir una clase y crear objetos con `new`;
> - sabrás dar datos a un objeto con `initialize` y las variables `@`;
> - sabrás hacer legibles esos datos con `attr_reader` y `attr_accessor`;
> - sabrás ocultar un método con `private`;
> - sabrás reutilizar una clase con la herencia y `super`.

## Definir una clase

Una [clase](../debuter/glossaire.md#classe) es un modelo para fabricar objetos.
Piensa en un molde de pasteles: el molde es la clase, cada pastel es un
[objeto](../debuter/glossaire.md#objet). Todos los pasteles tienen la misma forma, pero
cada uno puede tener su sabor. El nombre de una clase empieza por mayúscula.

```ruby
class Perro
  def ladrar
    puts "¡Guau!"
  end
end

rex = Perro.new
rex.ladrar
```

```sortie
¡Guau!
```

`Perro.new` crea un objeto nuevo: una **instancia** de la clase. `rex.ladrar` llama
al método `ladrar` sobre ese objeto.

Usas clases desde el principio: `"hola"` es una instancia de `String`,
`42` una instancia de `Integer`. `class` e `is_a?` lo muestran.

```ruby
class Perro
end

rex = Perro.new
puts rex.class
puts rex.is_a?(Perro)
puts 42.is_a?(Integer)
puts "hola".class
```

```sortie
Perro
true
true
String
```

## initialize y las variables de instancia

`new` llama a `initialize`. Prepara el objeto, como cuando rellenas una ficha
al inscribirte. Los argumentos de `new` se pasan a `initialize`.

Las variables que empiezan por `@` son **variables de instancia**:
pertenecen al objeto. Cada objeto tiene las suyas, y viven tanto tiempo
como él. Todos los métodos del objeto pueden leerlas.

```ruby
class Perro
  def initialize(nombre)
    @nombre = nombre
  end

  def presentarse
    puts "Me llamo #{@nombre}."
  end
end

Perro.new("Rex").presentarse
Perro.new("Pluto").presentarse
```

```sortie
Me llamo Rex.
Me llamo Pluto.
```

## attr_accessor

Las variables `@` están ocultas: no se pueden leer desde fuera del objeto.
Para leer o cambiar `@nombre` desde fuera, usa `attr_reader`, `attr_writer`
o `attr_accessor`. Escriben por ti los pequeños métodos `nombre` y `nombre=`.

```ruby
class Persona
  attr_accessor :nombre
  attr_reader :edad

  def initialize(nombre, edad)
    @nombre = nombre
    @edad = edad
  end
end

alicia = Persona.new("Alicia", 30)
alicia.nombre = "Alicia Martín"
puts alicia.nombre
puts alicia.edad
```

```sortie
Alicia Martín
30
```

> **Recuerda:** `attr_reader` permite leer, `attr_writer` escribir, `attr_accessor`
> las dos cosas.

## Los métodos y to_s

Una clase puede tener tantos métodos como quieras. `to_s` decide cómo se muestra el
objeto con `puts`.

```ruby
class Punto
  def initialize(x, y)
    @x = x
    @y = y
  end

  def distancia_origen
    Math.sqrt(@x**2 + @y**2)
  end

  def to_s
    "(#{@x}, #{@y})"
  end
end

pt = Punto.new(3, 4)
puts pt
puts pt.distancia_origen
```

```sortie
(3, 4)
5.0
```

## self y los métodos de clase

Dentro de un método, `self` es el propio objeto. Delante del nombre de un método
(`def self.nombre`), crea un **método de clase**: se llama sobre la clase, no
sobre un objeto.

```ruby
class Temperatura
  attr_reader :grados

  def initialize(grados)
    @grados = grados
  end

  def self.desde_fahrenheit(f)
    new(((f - 32) * 5 / 9.0).round(1))
  end

  def caliente?
    self.grados > 25
  end
end

t = Temperatura.desde_fahrenheit(86)
puts t.grados
puts t.caliente?
```

```sortie
30.0
true
```

## Los métodos privados

Después de la palabra `private`, los métodos solo se pueden usar **dentro** de la
clase. Sirven de herramientas internas, como la cocina de un restaurante: el cliente
no la ve.

```ruby
class Cuenta
  def initialize(saldo)
    @saldo = saldo
  end

  def mostrar
    "Saldo: #{formato_euros(@saldo)}"
  end

  private

  def formato_euros(cantidad)
    format("%.2f €", cantidad)
  end
end

cuenta = Cuenta.new(42)
puts cuenta.mostrar
puts cuenta.respond_to?(:formato_euros)
```

```sortie
Saldo: 42.00 €
false
```

`respond_to?` pregunta si el objeto acepta un método desde fuera.

## La herencia

Una clase puede heredar de otra con `<`. Recibe todos sus métodos.

```ruby
class Animal
  def initialize(nombre)
    @nombre = nombre
  end

  def hablar
    "#{@nombre} hace un ruido."
  end
end

class Gato < Animal
  def hablar
    "#{@nombre} maúlla."
  end
end

puts Animal.new("Cosa").hablar
puts Gato.new("Félix").hablar
```

```sortie
Cosa hace un ruido.
Félix maúlla.
```

`Gato` es la clase **hija**, `Animal` la clase **padre**. `Gato` redefine
`hablar`: su versión reemplaza a la del padre.

### Llamar al padre con super

En un método redefinido, `super` llama a la versión de la clase padre. Es
práctico para completarla en lugar de reescribirlo todo.

```ruby
class Animal
  def initialize(nombre)
    @nombre = nombre
  end
end

class Perro < Animal
  def initialize(nombre, raza)
    super(nombre)
    @raza = raza
  end

  def to_s
    "#{@nombre} (#{@raza})"
  end
end

puts Perro.new("Rex", "pastor")
puts Perro.new("Rex", "pastor").is_a?(Animal)
```

```sortie
Rex (pastor)
true
```

Un `Perro` **es un** `Animal`: `is_a?(Animal)` responde `true`.

## Los módulos y los mixins

Un [módulo](../debuter/glossaire.md#module) agrupa métodos. Con `include`, una
clase los recibe: es un **mixin**. El capítulo [los módulos](modules.md) los
presenta en detalle.

```ruby
module Volador
  def volar
    "¡#{nombre} echa a volar!"
  end
end

class Pajaro
  include Volador
  attr_reader :nombre

  def initialize(nombre)
    @nombre = nombre
  end
end

puts Pajaro.new("Piolín").volar
```

```sortie
¡Piolín echa a volar!
```

> **Nota:** una clase solo tiene un padre, pero puede incluir varios módulos.

## Struct

`Struct` crea rápido una pequeña clase con atributos: `new`, los accesores y una
visualización legible se escriben por ti.

```ruby
Libro = Struct.new(:titulo, :autor)

libro = Libro.new("Los miserables", "Victor Hugo")
puts libro.titulo
libro.autor = "V. Hugo"
p libro
```

```sortie
Los miserables
#<struct Libro titulo="Los miserables", autor="V. Hugo">
```

## Errores frecuentes

### Olvidar un argumento de new

```ruby norun
class Perro
  def initialize(nombre)
    @nombre = nombre
  end
end
rex = Perro.new
```

```text
main.rb:2:in `initialize': wrong number of arguments (given 0, expected 1) (ArgumentError)
	from main.rb:6:in `new'
	from main.rb:6:in `<main>'
```

`new` pasa sus argumentos a `initialize`, que espera un nombre. Escribe
`Perro.new("Rex")`.

### Leer una variable @ sin accesor

```ruby norun
class Persona
  def initialize(nombre)
    @nombre = nombre
  end
end
alicia = Persona.new("Alicia")
puts alicia.nombre
```

```text
main.rb:7:in `<main>': undefined method `nombre' for an instance of Persona (NoMethodError)

puts alicia.nombre
           ^^^^^^^
```

`@nombre` está oculto dentro del objeto. Añade `attr_reader :nombre` en la clase.

### Un nombre de clase en minúsculas

```ruby norun
class perro
end
```

```text
main.rb: --> main.rb
expected a constant name after `class`
> 1  class perro
> 2  end
main.rb:1: class/module name must be CONSTANT (SyntaxError)
class perro
      ^~~~~
```

Un nombre de clase es una constante: empieza por mayúscula. Escribe
`class Perro`.

## Ponte a prueba

**Pregunta 1:** ¿qué método llama `Libro.new("Dune")`?

> **Respuesta**
>
> `initialize`, con el argumento `"Dune"`.

**Pregunta 2:** ¿qué hay que añadir para que este código muestre `Rex`?

```ruby norun
class Perro
  def initialize(nombre)
    @nombre = nombre
  end
end
puts Perro.new("Rex").nombre
```

> **Respuesta**
>
> Un accesor de lectura en la clase:
>
> ```ruby
> class Perro
>   attr_reader :nombre
>
>   def initialize(nombre)
>     @nombre = nombre
>   end
> end
> puts Perro.new("Rex").nombre
> ```
>
> ```sortie
> Rex
> ```

**Pregunta 3:** en `class Gato < Animal`, ¿cuál es el padre?

> **Respuesta**
>
> `Animal`. `Gato` recibe todos sus métodos y puede redefinirlos.

## En resumen

- Una clase es un modelo; `new` crea un objeto y llama a `initialize`.
- Las variables con `@` pertenecen al objeto.
- `attr_reader`, `attr_writer` y `attr_accessor` las hacen legibles o modificables.
- `private` oculta métodos; `self.` crea un método de clase.
- `class Gato < Animal` hereda los métodos de `Animal`; `super` llama al padre.
- Un módulo comparte métodos con `include`; `Struct` crea rápido una pequeña clase.

Siguiente: [herencia y polimorfismo](heritage.md)
