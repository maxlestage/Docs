# Chuleta

Esta página resume de un vistazo la sintaxis de Ruby vista en toda la documentación.

## Bases

| Elemento | Ejemplo |
| --- | --- |
| Variable | `edad = 30` |
| Constante | `TASA = 0.2` |
| Cadena | `"Hola #{nombre}"` |
| Símbolo | `:rojo` |
| Booleanos | `true`, `false` |
| Nada | `nil` |
| Comentario | `# texto` |
| Mostrar | `puts`, `print`, `p` |

```ruby
nombre = "Alicia"
puts "Hola #{nombre}"
p :ok
p 7 / 2
p 7.0 / 2
```

```sortie
Hola Alicia
:ok
3
3.5
```

Detalles: [primeros pasos](premiers-pas.md).

## Condiciones y bucles

| Necesidad | Sintaxis |
| --- | --- |
| Condición | `if … elsif … else … end` |
| Condición inversa | `unless … end` |
| En una línea | `puts "ok" if x > 0` |
| Ternario | `x > 0 ? "sí" : "no"` |
| Varios casos | `case x when 1 then … end` |
| Bucles | `while`, `until`, `n.times`, `each` |
| Rangos | `1..5` (incluido), `1...5` (excluido) |
| Control | `break`, `next` |

```ruby
(1..5).each do |n|
  next if n == 2
  puts n.even? ? "#{n} par" : "#{n} impar"
end
```

```sortie
1 impar
3 impar
4 par
5 impar
```

Detalles: [condiciones y bucles](conditions-boucles.md).

## Métodos

```ruby
def saludar(nombre, saludo = "Hola", cortes: false)
  texto = "#{saludo} #{nombre}"
  cortes ? "#{texto}, encantado." : texto
end

puts saludar("Alicia")
puts saludar("Bob", "Buenas", cortes: true)
```

```sortie
Hola Alicia
Buenas Bob, encantado.
```

| Convención | Significado |
| --- | --- |
| `vacio?` | devuelve verdadero o falso |
| `ordenar!` | modifica el objeto |
| última línea | valor devuelto |
| `yield` | ejecuta el bloque recibido |

Detalles: [los métodos](fonctions.md).

## Texto

| Necesidad | Ejemplo |
| --- | --- |
| Insertar un valor | `"Hola #{nombre}"` |
| Longitud | `t.length` |
| Mayúsculas, minúsculas | `t.upcase`, `t.downcase`, `t.capitalize` |
| Quitar los espacios de alrededor | `t.strip` |
| Buscar | `t.include?("a")`, `t.start_with?("B")` |
| Cortar, unir | `t.split(",")`, `array.join(" ")` |
| Reemplazar | `t.sub("a", "b")` (una vez), `t.gsub("a", "b")` (en todas partes) |
| Una parte | `t[0]`, `t[0..2]`, `t[-1]` |
| Redondear, formatear | `x.round(2)`, `format("%.2f", x)` |

```ruby
t = "  hola ruby  ".strip
puts t.capitalize
puts t[0..3]
p t.split(" ")
puts t.gsub("o", "0")
puts format("%.2f", 3.14159)
```

```sortie
Hola ruby
hola
["hola", "ruby"]
h0la ruby
3.14
```

Detalles: [trabajar con texto](texte.md).

## Arrays y hashes

| Acción | Array | Hash |
| --- | --- | --- |
| Crear | `[1, 2]` | `{ a: 1 }` |
| Leer | `t[0]` | `h[:a]` |
| Añadir | `t << 3` | `h[:b] = 2` |
| Borrar | `t.delete(3)` | `h.delete(:b)` |
| Tamaño | `t.length` | `h.size` |
| Contiene | `t.include?(1)` | `h.key?(:a)` |

```ruby
t = [3, 1, 2]
h = { a: 1 }
h[:b] = 2
p t.sort
p h
```

```sortie
[1, 2, 3]
{:a=>1, :b=>2}
```

Detalles: [arrays y hashes](collections.md).

## Bloques y Enumerable

| Método | Función |
| --- | --- |
| `each` | recorrer |
| `map` | transformar |
| `select` / `reject` | conservar / quitar |
| `find` | el primero que coincide |
| `reduce` | combinar en un solo valor |
| `sort_by` | ordenar según un criterio |
| `group_by` | agrupar en un hash |
| `each_with_index` | recorrer con el índice |

```ruby
numeros = [1, 2, 3, 4, 5]
p numeros.map { |n| n * 10 }
p numeros.select(&:odd?)
p numeros.reduce(:+)
cuadrado = ->(x) { x * x }
p cuadrado.call(4)
```

```sortie
[10, 20, 30, 40, 50]
[1, 3, 5]
15
16
```

Detalles: [los bloques](blocs.md).

## Clases y objetos

```ruby
module Mostrable
  def mostrar
    puts to_s
  end
end

class Animal
  include Mostrable
  attr_reader :nombre

  def initialize(nombre)
    @nombre = nombre
  end

  def to_s
    "Animal #{nombre}"
  end
end

class Gato < Animal
  def to_s
    "Gato #{nombre}"
  end
end

Gato.new("Félix").mostrar
Punto = Struct.new(:x, :y)
p Punto.new(1, 2)
```

```sortie
Gato Félix
#<struct Punto x=1, y=2>
```

Detalles: [clases y objetos](types.md).

## Módulos y mixins

| Necesidad | Sintaxis |
| --- | --- |
| Función de módulo | `module Utiles` y luego `def self.nombre`; llamada `Utiles.nombre` |
| Constante | `module Config; VERSION = "1.0"; end`; `Config::VERSION` |
| Clase guardada en un módulo | `Zoo::Animal.new` |
| Compartir métodos | `include NombreDelModulo` en la clase |
| Comparar, ordenar | `include Comparable` + `def <=>(otro)` |
| `map`, `select`… | `include Enumerable` + `def each` con `yield` |
| Cargar un archivo | `require_relative "lib/calculo"` |

```ruby
module Calculo
  TASA = 0.2

  def self.con_iva(precio) = precio * (1 + TASA)
end

class Nota
  include Comparable
  attr_reader :valor

  def initialize(valor) = @valor = valor
  def <=>(otra) = valor <=> otra.valor
end

puts Calculo.con_iva(10)
puts Calculo::TASA
puts Nota.new(12) < Nota.new(15)
p [Nota.new(15), Nota.new(9)].min.valor
```

```sortie
12.0
0.2
true
9
```

Detalles: [módulos y mixins](modules.md).

## Errores

| Palabra clave | Función |
| --- | --- |
| `begin … rescue … end` | capturar un error |
| `rescue Tipo => e` | leer el error en `e` |
| `ensure` | se ejecuta siempre |
| `raise` | lanzar un error |
| `retry` | volver a empezar el `begin` |
| `&.` | llamada segura si es `nil` |

```ruby
class MiError < StandardError
end

begin
  raise MiError, "¡ay!"
rescue MiError => e
  puts "Capturado: #{e.message}"
ensure
  puts "Terminado"
end
p nil&.length
```

```sortie
Capturado: ¡ay!
Terminado
nil
```

Detalles: [gestionar los errores](erreurs.md).

## Teclado y archivos

| Necesidad | Sintaxis |
| --- | --- |
| Leer una línea del teclado | `gets.chomp` |
| Argumentos del programa | `ARGV[0]` |
| Escribir (lo reemplaza todo) | `File.write("f.txt", texto)` |
| Leerlo todo | `File.read("f.txt")` |
| Leer las líneas | `File.readlines("f.txt", chomp: true)` |
| Línea por línea | `File.foreach("f.txt")` con un bloque |
| ¿Existe el archivo? | `File.exist?("f.txt")` |
| Añadir al final | `File.open("f.txt", "a")` con un bloque, y luego `f.puts` |
| JSON | `require "json"`, `JSON.generate(h)`, `JSON.parse(t)` |

```ruby
File.write("f.txt", "uno\n")
File.open("f.txt", "a") { |f| f.puts "dos" }
p File.readlines("f.txt", chomp: true)
puts File.exist?("f.txt")
```

```sortie
["uno", "dos"]
true
```

Detalles: [teclado y archivos](fichiers.md).

## Pruebas con Minitest

| Elemento | Sintaxis |
| --- | --- |
| Cargar Minitest | `require "minitest/autorun"` |
| Clase de prueba | `class TestCalculo < Minitest::Test` |
| Una prueba | un método `def test_…` |
| Antes de cada prueba | `def setup` |
| Igualdad | `assert_equal esperado, obtenido` |
| Verdadero, falso, nil | `assert x`, `refute x`, `assert_nil x` |
| Números decimales | `assert_in_delta 0.3, x, 0.001` |
| Se lanza un error | `assert_raises(ArgumentError) { … }` |
| Ejecutar | `ruby test/test_calculo.rb` |
| Resultado | `.` superada, `F` fallo, `E` error |

```ruby
require "minitest/autorun"

def doble(x) = x * 2

class TestDoble < Minitest::Test
  def test_doble
    assert_equal 8, doble(4)
  end

  def test_error
    assert_raises(ZeroDivisionError) { 1 / 0 }
  end
end
```

Detalles: [probar tu código](tests.md).

## Proyecto y herramientas

| Comando | Función |
| --- | --- |
| `ruby archivo.rb` | ejecutar un programa |
| `irb` | consola interactiva |
| `gem install nombre` | instalar una gema |
| `bundle install` | instalar las gemas del `Gemfile` |
| `bundle exec …` | ejecutar con las gemas del proyecto |
| `rubocop` | comprobar el estilo |

Detalles: [organizar un proyecto](projet.md). Para practicar:
[los proyectos paso a paso](projets.md) y [los ejercicios resueltos](exercices.md). Compara con
[la chuleta de Rust](../rust/aide-memoire.md) y [la chuleta de Rythme](../rythme/aide-memoire.md).
