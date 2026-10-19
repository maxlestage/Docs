# Los rangos

Esta página muestra cómo describir una serie de valores, del 1 al 10 o de la «a» a la «z», con un rango, y todo lo que puedes hacer con él.

> **Objetivos:**
>
> - sabrás escribir un rango con `..` y con `...`;
> - sabrás recorrerlo, convertirlo en array y avanzar de a pasos;
> - sabrás comprobar si un valor está en un rango;
> - sabrás cortar un array o un texto con un rango;
> - sabrás usar un rango sin inicio o sin fin.

## Escribir un rango

Un [rango](../debuter/glossaire.md#rango) (`Range` en Ruby) describe todos los
valores entre un inicio y un fin. Es como decir «las páginas 1 a 10» en lugar de
nombrarlas una por una.

- `1..10`: del 1 al 10, **10 incluido**;
- `1...10`: del 1 al 10, **10 excluido** (o sea, del 1 al 9).

```ruby
p (1..5).to_a
p (1...5).to_a
puts (1..5).class
puts (1..5).size
puts (1...5).size
```

```sortie
[1, 2, 3, 4, 5]
[1, 2, 3, 4]
Range
5
4
```

`to_a` convierte el rango en array. El rango en sí no guarda sus valores: solo
conoce su inicio y su fin. Por eso `1..1_000_000` casi no ocupa espacio en
memoria.

> **Consejo:** para recordar la diferencia, piensa que el tercer punto «empuja» el
> fin hacia fuera.

## Recorrer un rango

`each` pasa por cada valor, como con un array.

```ruby
(1..3).each { |n| puts "Vuelta #{n}" }

(1...4).each do |i|
  puts "#{i} × 5 = #{i * 5}"
end
```

```sortie
Vuelta 1
Vuelta 2
Vuelta 3
1 × 5 = 5
2 × 5 = 10
3 × 5 = 15
```

Un rango también conoce los métodos de
[Enumerable](blocs.md): `map`, `select`, `sum`…

```ruby
p (1..5).map { |n| n * n }
p (1..10).select(&:even?)
```

```sortie
[1, 4, 9, 16, 25]
[2, 4, 6, 8, 10]
```

## Avanzar de a pasos con step

`step` salta varios valores a la vez. `%` hace lo mismo, de forma más corta.

```ruby
p (0..20).step(5).to_a
p ((0..20) % 5).to_a
p (0.0..1.0).step(0.25).to_a
10.step(1, -3) { |n| print n, " " }
puts
```

```sortie
[0, 5, 10, 15, 20]
[0, 5, 10, 15, 20]
[0.0, 0.25, 0.5, 0.75, 1.0]
10 7 4 1 
```

Para contar hacia atrás, un rango como `10..1` está **vacío**. Usa `step` con un paso
negativo, `downto` o `(1..10).reverse_each`.

## Comprobar la pertenencia

`include?` responde a la pregunta: «¿está este valor en el rango?»

```ruby
edades_adolescentes = 13..19
puts edades_adolescentes.include?(15)
puts edades_adolescentes.include?(20)
puts (1.5..2.5).include?(2.1)
puts (1...10).include?(10)
```

```sortie
true
false
true
false
```

`cover?` hace casi lo mismo: solo compara con el inicio y el fin. La diferencia se
ve con textos.

```ruby
puts ("a".."z").include?("bb")
puts ("a".."z").cover?("bb")
puts (1..10).cover?(3..5)
```

```sortie
false
true
true
```

`"bb"` no es una de las 26 letras, pero sí queda entre `"a"` y `"z"` en orden
alfabético. `cover?` también acepta un rango entero.

## Rangos de letras

Un rango funciona con todo lo que tiene un «siguiente»: también con las letras.

```ruby
p ("a".."e").to_a
puts ("a".."z").count
p ("A".."C").map { |letra| letra * 2 }
p ("aa".."ad").to_a
```

```sortie
["a", "b", "c", "d", "e"]
26
["AA", "BB", "CC"]
["aa", "ab", "ac", "ad"]
```

## Suma, mínimo y máximo

```ruby
puts (1..100).sum
puts (1..10).min
puts (1..10).max
p (1..10).minmax
puts (1..5).reduce(:*)
```

```sortie
5050
1
10
[1, 10]
120
```

`sum` sobre un rango de enteros es instantáneo: Ruby usa la fórmula de Gauss en lugar
de sumar uno por uno. `reduce(:*)` lo multiplica todo: 1 × 2 × 3 × 4 × 5.

## Los rangos en case y when

Con [case y when](conditions-boucles.md#case-y-when), un rango coloca un valor en una
categoría. `when` usa `cover?` por detrás.

```ruby
def calificacion(nota)
  case nota
  when 16..20 then "Sobresaliente"
  when 14...16 then "Notable"
  when 10...14 then "Aprobado"
  else "Suspenso"
  end
end

puts calificacion(17)
puts calificacion(14)
puts calificacion(13.5)
puts calificacion(4)
```

```sortie
Sobresaliente
Notable
Aprobado
Suspenso
```

> **Atención:** con notas decimales, `10..13` dejaría escapar `13.5` entre dos
> casillas. `10...14` lo cubre todo hasta 14, excluido.

## Cortar con un rango

Entre corchetes, un rango toma un trozo de un array o de un texto. Las posiciones
empiezan en 0, y `-1` es la última.

```ruby
letras = ["a", "b", "c", "d", "e"]
p letras[1..3]
p letras[1...3]
p letras[2..]
p letras[..1]
p letras[-2..]

palabra = "paraguas"
puts palabra[0..2]
puts palabra[3..]
puts palabra[..-2]
```

```sortie
["b", "c", "d"]
["b", "c"]
["c", "d", "e"]
["a", "b"]
["d", "e"]
par
aguas
paragua
```

## Sin fin o sin inicio

Puedes dejar un extremo vacío:

- `5..`: del 5 hasta el infinito (rango **sin fin**);
- `..5`: todo lo que es menor o igual que 5 (rango **sin inicio**).

```ruby
p (5..).first(3)
puts (..5).include?(-100)

def categoria(edad)
  case edad
  when ...18 then "menor"
  when 18..64 then "adulto"
  when 65.. then "mayor"
  end
end

puts categoria(12)
puts categoria(40)
puts categoria(80)
```

```sortie
[5, 6, 7]
true
menor
adulto
mayor
```

`first(3)` toma los 3 primeros valores: no llames a `to_a` sobre `5..`, nunca
terminaría.

## Errores frecuentes

### Olvidar los paréntesis alrededor del rango

```ruby norun
p 1..5.to_a
```

```text
main.rb:1:in `<main>': undefined method `to_a' for an instance of Integer (NoMethodError)

p 1..5.to_a
      ^^^^^
Did you mean?  to_c
               to_r
               to_f
               to_i
               to_s
```

`.to_a` se aplica a `5`, no al rango. Rodéalo con paréntesis: `p (1..5).to_a`.

### Un rango al revés

```ruby norun
p (10..1).to_a
```

```text
[]
```

No hay error, pero sí un array vacío: un rango siempre va del más pequeño al más
grande. Para contar hacia atrás: `10.downto(1).to_a` o `(1..10).to_a.reverse`.

### Convertir un rango sin fin en array

```ruby norun
p (1..).to_a
```

```text
main.rb:1:in `to_a': cannot convert endless range to an array (RangeError)
	from main.rb:1:in `<main>'
```

Un rango sin fin tiene infinitos valores. Toma solo algunos con `first(n)` o
`take(n)`.

## Ponte a prueba

**Pregunta 1:** ¿cuántos valores contiene `0...10`?

> **Respuesta**
>
> 10 valores: del 0 al 9.
>
> ```ruby
> puts (0...10).size
> ```
>
> ```sortie
> 10
> ```

**Pregunta 2:** ¿cómo obtienes los 3 últimos elementos de `[1, 2, 3, 4, 5, 6]`?

> **Respuesta**
>
> ```ruby
> p [1, 2, 3, 4, 5, 6][-3..]
> ```
>
> ```sortie
> [4, 5, 6]
> ```

**Pregunta 3:** ¿cómo calculas la suma de los números pares del 1 al 100?

> **Respuesta**
>
> ```ruby
> puts (2..100).step(2).sum
> puts (1..100).select(&:even?).sum
> ```
>
> ```sortie
> 2550
> 2550
> ```

## En resumen

- `a..b` incluye el fin, `a...b` lo excluye.
- `each`, `map`, `select`, `sum`, `min`, `max` funcionan con un rango.
- `step` avanza de a pasos; un rango al revés está vacío.
- `include?` y `cover?` comprueban la pertenencia; `case`/`when` clasifica un valor.
- `array[1..3]` o `texto[2..]` cortan un trozo.
- `5..` no tiene fin, `..5` no tiene inicio: usa `first(n)` para leerlos.

Siguiente: [las trampas del principiante en Ruby](pieges.md)
