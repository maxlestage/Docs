# Bloques, iteradores y Enumerable

Esta página explica los bloques, los iteradores como `map` y `select`, y presenta rápidamente los procs y las lambdas.

> **Objetivos:**
>
> - sabrás escribir un bloque con `{ }` o `do ... end`;
> - sabrás transformar, filtrar y combinar una colección con `map`, `select` y `reduce`;
> - sabrás hacerle preguntas a una colección (`any?`, `count`, `max_by`…);
> - sabrás que un bloque ve las variables que lo rodean;
> - sabrás guardar código en una variable con una lambda.

## Los bloques

Un [bloque](../debuter/glossaire.md#bloque) es un trozo de código que le das a un
método. El método decide cuándo ejecutarlo, y cuántas veces. Es como darle una
instrucción a un camarero: «a cada plato, échale sal». El camarero (el método) va de
plato en plato; tú solo has escrito la instrucción (el bloque).

```ruby
[1, 2, 3].each { |n| puts n * 2 }
```

```sortie
2
4
6
```

Entre las barras `|n|`, nombras el valor que recibe el bloque. Ese nombre es un
**parámetro del bloque**: cambia en cada pasada (cada
[iteración](../debuter/glossaire.md#iteracion)). Elige un nombre claro: `|fruta|`,
`|nota|`…

Un método que recorre una colección llamando a un bloque se llama un
**iterador**. `each`, `times` y `map` son iteradores.

## do...end o llaves

Hay dos maneras de escribir un bloque:

- `{ ... }` para un bloque corto, en una línea;
- `do ... end` para un bloque de varias líneas.

```ruby
[1, 2].each { |n| puts n }

[1, 2].each do |n|
  cuadrado = n * n
  puts "#{n} al cuadrado: #{cuadrado}"
end
```

```sortie
1
2
1 al cuadrado: 1
2 al cuadrado: 4
```

> **Recuerda:** las dos formas hacen lo mismo. La elección es una cuestión de
> legibilidad.

## Los métodos de Enumerable

Los arrays, los hashes y los rangos comparten los mismos métodos, que vienen del módulo
`Enumerable` («enumerable»: que se puede recorrer uno a uno). Un módulo es una caja de
métodos compartidos: ver [los módulos](modules.md).

> **Recuerda:** estos métodos **devuelven** un resultado nuevo. No modifican la colección
> de partida.

### map: transformar

`map` crea un array nuevo con el resultado del bloque para cada elemento.

```ruby
precios = [10, 20, 30]
p precios.map { |x| x * 2 }
p ["a", "b"].map(&:upcase)
```

```sortie
[20, 40, 60]
["A", "B"]
```

> **Consejo:** `map(&:upcase)` es un atajo para `map { |s| s.upcase }`. Funciona cuando
> el bloque solo llama a un método sin argumentos sobre cada elemento.

### select y reject: filtrar

`select` conserva los elementos para los que el bloque es verdadero. `reject` los quita.

```ruby
numeros = (1..10).to_a
p numeros.select(&:even?)
p numeros.reject { |n| n > 3 }
p numeros.find { |n| n > 4 }
```

```sortie
[2, 4, 6, 8, 10]
[1, 2, 3]
5
```

`find` devuelve el **primer** elemento para el que el bloque es verdadero (o `nil`).

### reduce: combinar

`reduce` combina todos los elementos en un solo valor.

```ruby
numeros = [1, 2, 3, 4]
total = numeros.reduce(0) { |suma, n| suma + n }
puts total
```

```sortie
10
```

En cada vuelta, `suma` contiene el total ya calculado, y `n` el elemento siguiente. El
`0` es el valor de partida.

### Hacerle preguntas a una colección

- `any?`: ¿cumple el bloque al menos un elemento?
- `all?`: ¿lo cumplen todos los elementos?
- `count`: ¿cuántos elementos lo cumplen?
- `min_by` y `max_by`: ¿qué elemento da el valor más pequeño o el más grande?
- `sum`: la suma, si quieres de lo que devuelve el bloque.
- `tally`: cuántas veces aparece cada valor.

```ruby
notas = [12, 7, 15, 9]
puts notas.any? { |n| n < 8 }
puts notas.all? { |n| n >= 10 }
puts notas.count { |n| n >= 10 }

palabras = ["kiwi", "sandía", "manzana"]
puts palabras.max_by { |pal| pal.length }
puts palabras.sum { |pal| pal.length }
p ["a", "b", "a"].tally
```

```sortie
true
false
2
manzana
17
{"a"=>2, "b"=>1}
```

### each_with_index

```ruby
%w[lunes martes].each_with_index do |dia, i|
  puts "#{i}: #{dia}"
end
```

```sortie
0: lunes
1: martes
```

> **Nota:** `%w[lunes martes]` es un atajo para `["lunes", "martes"]`.

### sort_by: ordenar

`sort_by` ordena según el valor que da el bloque.

```ruby
palabras = ["kiwi", "manzana", "frambuesa", "fresa"]
p palabras.sort_by { |pal| pal.length }
p palabras.sort_by { |pal| pal.length }.last
```

```sortie
["kiwi", "fresa", "manzana", "frambuesa"]
"frambuesa"
```

### group_by: agrupar

`group_by` coloca los elementos en un hash, según el valor del bloque.

```ruby
palabras = ["abeja", "caballo", "cabra", "araña"]
p palabras.group_by { |pal| pal[0] }
```

```sortie
{"a"=>["abeja", "araña"], "c"=>["caballo", "cabra"]}
```

### Encadenar los métodos

Cada método devuelve un valor: puedes encadenarlos.

```ruby
resultado = (1..10)
  .select(&:odd?)
  .map { |n| n * n }
  .sum
puts resultado
```

```sortie
165
```

## Un bloque ve las variables que lo rodean

Un método no ve las variables de fuera (ver
[el ámbito](fonctions.md#el-ambito-de-las-variables)). Un bloque, en cambio, las ve y
puede modificarlas. Se dice que el bloque es una [closure](../debuter/glossaire.md#closure)
(«clausura»): se lleva consigo lo que lo rodea.

```ruby
total = 0
[5, 10, 20].each { |precio| total += precio }
puts total
```

```sortie
35
```

En cambio, una variable creada **dentro** del bloque desaparece al final del bloque.

### Escribir un método que recibe un bloque

`yield` ejecuta el bloque recibido (ver [los métodos](fonctions.md#pasar-un-bloque-con-yield)).
`block_given?` responde `true` si se ha dado un bloque.

```ruby
def anunciar(texto)
  if block_given?
    puts yield(texto)
  else
    puts texto
  end
end

anunciar("hola")
anunciar("hola") { |t| "¡" + t.upcase + "!" }
```

```sortie
hola
¡HOLA!
```

## Los procs y las lambdas

Un bloque no se puede guardar en una variable. Para eso, Ruby ofrece los **procs**
y las **lambdas**. Para empezar, basta con saber reconocerlos.

```ruby
doblar = ->(x) { x * 2 }
puts doblar.call(5)
puts doblar.call(7)
```

```sortie
10
14
```

`call` ejecuta la lambda con el valor dado.

Un proc se escribe con `proc { ... }`:

```ruby
saludar = proc { |nombre| puts "Hola #{nombre}" }
saludar.call("Alice")
```

```sortie
Hola Alice
```

> **Atención:** una lambda comprueba el número de argumentos, un proc no. En caso de duda,
> elige la lambda.

## Errores frecuentes

### Olvidar nombrar el parámetro del bloque

```ruby norun
[1, 2, 3].each { puts n * 2 }
```

```text
main.rb:1:in `block in <main>': undefined local variable or method `n' for main (NameError)

[1, 2, 3].each { puts n * 2 }
                      ^
	from main.rb:1:in `each'
	from main.rb:1:in `<main>'
```

El bloque no sabe qué es `n`. Decláralo entre barras: `{ |n| puts n * 2 }`.

### do ... end después de p o puts

```ruby norun
p [1, 2].map do |x|
  x * 2
end
```

```text
#<Enumerator: [1, 2]:map>
```

¡No hay error, pero tampoco el resultado correcto! Con `do ... end`, el bloque se le da
a `p`, no a `map`. `map` sin bloque devuelve un objeto «en espera» (un `Enumerator`).
Guarda primero el resultado en una variable, o usa llaves:

```ruby
p [1, 2].map { |x| x * 2 }
```

```sortie
[2, 4]
```

### Llamar a una lambda con un número de argumentos incorrecto

```ruby norun
sumar = ->(a, b) { a + b }
puts sumar.call(1)
```

```text
main.rb:1:in `block in <main>': wrong number of arguments (given 1, expected 2) (ArgumentError)
	from main.rb:2:in `<main>'
```

Una lambda comprueba sus argumentos, como un método. Dáselos todos:
`sumar.call(1, 2)`.

## Ponte a prueba

**Pregunta 1:** ¿cuál es la diferencia entre `each` y `map`?

> **Respuesta**
>
> `each` ejecuta el bloque para cada elemento y devuelve la colección de partida.
> `map` devuelve un array **nuevo** lleno con los resultados del bloque.
>
> ```ruby
> p [1, 2].each { |x| x * 10 }
> p [1, 2].map { |x| x * 10 }
> ```
>
> ```sortie
> [1, 2]
> [10, 20]
> ```

**Pregunta 2:** ¿cómo conservas solo las palabras de más de 4 letras de
`["gato", "caballo", "pez", "jirafa"]`?

> **Respuesta**
>
> ```ruby
> p ["gato", "caballo", "pez", "jirafa"].select { |pal| pal.length > 4 }
> ```
>
> ```sortie
> ["caballo", "jirafa"]
> ```

**Pregunta 3:** ¿cuánto vale `[1, 2, 3].reduce(10) { |acc, n| acc + n }`?

> **Respuesta**
>
> `16`: se parte de 10, y luego se suman 1, 2 y 3.

## En resumen

- Un bloque es código que se le da a un método, entre `{ }` o entre `do` y `end`.
- `map` transforma, `select` y `reject` filtran, `find` busca el primero.
- `reduce` combina todos los elementos en un solo valor.
- `sort_by` ordena y `group_by` agrupa según el resultado del bloque.
- Un bloque ve las variables que lo rodean; una lambda (`->(x) { ... }`) es un bloque
  guardado en una variable.

Siguiente: [procs, lambdas y métodos](procs-lambdas.md)
