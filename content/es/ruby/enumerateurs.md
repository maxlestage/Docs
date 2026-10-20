# Enumeradores y pereza

Esta página muestra qué es un enumerador, cómo recorrer una colección a tu ritmo, por lotes o por pares, y cómo trabajar con secuencias infinitas gracias a `lazy`.

> **Objetivos:**
>
> - sabrás qué devuelve `each` cuando no le das ningún bloque;
> - sabrás avanzar paso a paso con `next`;
> - sabrás dividir una colección con `each_slice`, `each_cons` y `chunk_while`;
> - sabrás tratar una secuencia infinita con `lazy`;
> - sabrás crear tu propio enumerador con `Enumerator.new`.

## each sin bloque

¿Qué pasa si llamas a `each` o a `map` **sin** bloque? Ruby no falla: devuelve un
**enumerador** (`Enumerator`).

```ruby
e = [10, 20, 30].each
p e
puts e.class
```

```sortie
#<Enumerator: [10, 20, 30]:each>
Enumerator
```

Un enumerador es un recorrido **en espera**. Piensa en un marcapáginas en un libro: sabe
por dónde vas y puede darte la página siguiente cuando se la pides. Es un
[iterador](../debuter/glossaire.md#iterador) que puedes guardar en una variable.

Conoce todos los métodos de [Enumerable](blocs.md#los-metodos-de-enumerable):

```ruby
e = [10, 20, 30].each
p e.map { |x| x + 1 }
p e.to_a
```

```sortie
[11, 21, 31]
[10, 20, 30]
```

## Avanzar paso a paso con next

`next` da el elemento siguiente, uno cada vez. Cuando ya no quedan, lanza
`StopIteration`. `loop` atrapa este error y se detiene solo.

```ruby
semaforo = ["verde", "ámbar", "rojo"].each
puts semaforo.next
puts semaforo.next
puts semaforo.next

semaforo.rewind
loop do
  puts "Color: #{semaforo.next}"
end
```

```sortie
verde
ámbar
rojo
Color: verde
Color: ámbar
Color: rojo
```

`rewind` vuelve a poner el marcapáginas al principio. `peek` mira el elemento siguiente
sin avanzar.

## Añadir la posición con with_index

Como un enumerador es un objeto, puedes encadenarle métodos. `with_index` añade la
posición a cualquier recorrido, y puede empezar en 1.

```ruby
frutas = ["manzana", "kiwi", "pera"]

frutas.each.with_index(1) do |fruta, i|
  puts "#{i}. #{fruta}"
end

p frutas.map.with_index { |fruta, i| "#{fruta}#{i}" }
```

```sortie
1. manzana
2. kiwi
3. pera
["manzana0", "kiwi1", "pera2"]
```

`map.with_index` no es un método aparte: es `map` sin bloque (un enumerador) al que le
pides `with_index`.

## Por lotes con each_slice

`each_slice(n)` corta la colección en lotes de `n` elementos. El último lote puede ser
más pequeño.

```ruby
alumnos = ["Ana", "Ben", "Cleo", "Dan", "Eva"]
alumnos.each_slice(2) { |grupo| p grupo }
```

```sortie
["Ana", "Ben"]
["Cleo", "Dan"]
["Eva"]
```

## Por ventanas con each_cons

`each_cons(n)` desliza una ventana de `n` elementos, de una posición en una posición.
Práctico para comparar cada valor con el siguiente.

```ruby
temperaturas = [12, 15, 14, 18]
temperaturas.each_cons(2) do |antes, despues|
  puts "#{antes} -> #{despues}: #{despues - antes}"
end
```

```sortie
12 -> 15: 3
15 -> 14: -1
14 -> 18: 4
```

## Agrupar lo que va seguido con chunk_while

`chunk_while` agrupa los elementos vecinos mientras el bloque responda `true` para cada
par. Aquí agrupamos los números que van seguidos.

```ruby
dias = [1, 2, 3, 7, 8, 12]
grupos = dias.chunk_while { |a, b| b == a + 1 }
p grupos.to_a
p grupos.map { |g| g.size > 1 ? "#{g.first}-#{g.last}" : g.first.to_s }
```

```sortie
[[1, 2, 3], [7, 8], [12]]
["1-3", "7-8", "12"]
```

Su contrario, `slice_when`, corta cuando el bloque responde `true`.

## Repetir con cycle y take

`cycle` repite una colección sin fin. `take(n)` toma los `n` primeros elementos, lo que
permite detenerse.

```ruby
equipos = ["rojo", "azul"]
jugadores = ["Ana", "Ben", "Cleo", "Dan", "Eva"]

jugadores.zip(equipos.cycle.take(jugadores.size)).each do |jugador, equipo|
  puts "#{jugador}: #{equipo}"
end
```

```sortie
Ana: rojo
Ben: azul
Cleo: rojo
Dan: azul
Eva: rojo
```

## La pereza con lazy

Normalmente, cada etapa de una cadena trata **toda** la colección antes de pasar a la
siguiente. Con una secuencia infinita, es imposible: `map` no terminaría nunca.

`lazy` hace que la cadena sea **perezosa**: cada elemento atraviesa todas las etapas uno
tras otro, y solo cuando hace falta. Es como una cocina que prepara un plato por
encargo, en lugar de cocinarlo todo por adelantado.

```ruby
cuadrados_pares = (1..)
  .lazy
  .map { |n| n * n }
  .select(&:even?)
  .first(5)
p cuadrados_pares
```

```sortie
[4, 16, 36, 64, 100]
```

`(1..)` es infinito, pero solo se calculan los 10 primeros números. `first(5)` (o
`take(5).to_a`) pone en marcha el trabajo y dice cuándo parar.

```ruby
etapas = (1..3).lazy.map { |n| puts "cálculo de #{n}"; n * 10 }
puts "Todavía no se ha calculado nada"
p etapas.first(2)
```

```sortie
Todavía no se ha calculado nada
cálculo de 1
cálculo de 2
[10, 20]
```

> **Recuerda:** una cadena `lazy` no hace nada mientras no pidas un resultado con
> `first`, `to_a`, `each` o `force`.

## Crear tu enumerador con Enumerator.new

`Enumerator.new` fabrica una secuencia bajo demanda. En el bloque, `y << valor` (el
«yielder») **entrega** un valor y luego espera a que le pidas el siguiente.

```ruby
fibonacci = Enumerator.new do |y|
  a, b = 0, 1
  loop do
    y << a
    a, b = b, a + b
  end
end

p fibonacci.take(10)
p fibonacci.lazy.select(&:odd?).first(5)
puts fibonacci.next
puts fibonacci.next
```

```sortie
[0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
[1, 1, 3, 5, 13]
0
1
```

El bucle `loop` es infinito, pero no pasa nada: el bloque se detiene en cada `y <<` y
solo continúa si pides otro valor.

## Errores frecuentes

### Llamar a next después del final

```ruby norun
e = [1, 2].each
e.next
e.next
e.next
```

```text
main.rb:4:in `next': iteration reached an end (StopIteration)
	from main.rb:4:in `<main>'
```

Solo hay dos elementos. Usa `loop do ... end`, que se detiene limpiamente al final, o
`rewind` para volver al principio.

### Olvidar lazy en una secuencia infinita

```ruby norun
p (1..).map { |n| n * 2 }.first(3)
```

```text
(el programa no se detiene nunca)
```

`map` quiere tratar **todos** los números antes de pasar a `first`. Añade `lazy`:
`(1..).lazy.map { |n| n * 2 }.first(3)`.

### Mostrar una cadena lazy en lugar de su resultado

```ruby norun
p (1..).lazy.map { |n| n * 2 }.select(&:even?)
```

```text
#<Enumerator::Lazy: #<Enumerator::Lazy: #<Enumerator::Lazy: 1..>:map>:select>
```

No hay error, pero tampoco números: la cadena espera a que le pidas algo. Termínala con
`first(n)`.

## Ponte a prueba

**Pregunta 1:** ¿qué devuelve `[1, 2, 3].map` sin bloque?

> **Respuesta**
>
> Un enumerador, a la espera de un bloque o de otro método.
>
> ```ruby
> p [1, 2, 3].map
> ```
>
> ```sortie
> #<Enumerator: [1, 2, 3]:map>
> ```

**Pregunta 2:** ¿cómo mostrar `[1, 2, 3, 4, 5, 6]` en filas de 3?

> **Respuesta**
>
> ```ruby
> [1, 2, 3, 4, 5, 6].each_slice(3) { |fila| puts fila.join(" ") }
> ```
>
> ```sortie
> 1 2 3
> 4 5 6
> ```

**Pregunta 3:** ¿cómo obtener los 4 primeros múltiplos de 7 mayores que 100?

> **Respuesta**
>
> ```ruby
> p (101..).lazy.select { |n| n % 7 == 0 }.first(4)
> ```
>
> ```sortie
> [105, 112, 119, 126]
> ```

## En resumen

- `each`, `map` y sus primos devuelven un `Enumerator` cuando no les das bloque.
- `next` avanza paso a paso; `with_index(1)` añade una posición.
- `each_slice` corta en lotes, `each_cons` desliza una ventana, `chunk_while` agrupa lo
  que va seguido.
- `lazy` trata los elementos uno por uno, bajo demanda: ideal para las secuencias
  infinitas.
- `Enumerator.new { |y| y << ... }` crea una secuencia a medida.

Siguiente: [clases y objetos](types.md)
