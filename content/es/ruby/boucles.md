# Los bucles

Esta página muestra cómo repetir acciones con los bucles, y cómo describir una serie de valores con un rango.

> **Objetivos:**
>
> - sabrás describir una serie de valores con un rango;
> - sabrás repetir código con `while`, `loop`, `times`, `each` y `step`;
> - sabrás detener un bucle o saltarte una vuelta.

Este capítulo sigue a [las condiciones](conditions-boucles.md): un bucle repite código
mientras una condición lo pida.

## Los rangos

Las notas de 10 a 14, los días del 1 al 31, las letras de la a a la e: a menudo
hablamos de valores que van seguidos. Un rango los nombra de una vez, sin escribirlos
uno por uno.

Un rango (`Range`) va de un valor a otro. `1..5` incluye 5, `1...5` lo excluye.

```ruby
p (1..5).to_a
p (1...5).to_a
puts (1..10).include?(7)
p ("a".."e").to_a
```

```sortie
[1, 2, 3, 4, 5]
[1, 2, 3, 4]
true
["a", "b", "c", "d", "e"]
```

> **Consejo:** para recordarlo: el tercer punto «empuja» el último valor fuera.

## El bucle while

Imagina que te piden mostrar los números del 1 al 100. Escribir 100 `puts` sería
largo y pesado. Un bucle hace ese trabajo repetitivo por ti.

Un [bucle](../debuter/glossaire.md#bucle) repite código. Cada pasada se llama
**vuelta** (o [iteración](../debuter/glossaire.md#iteracion)).

`while` («mientras») repite mientras la condición sea verdadera.

```ruby
contador = 1
while contador <= 3
  puts "Vuelta #{contador}"
  contador += 1
end
```

```sortie
Vuelta 1
Vuelta 2
Vuelta 3
```

Línea por línea:

- `contador = 1`: la variable vale 1 al principio;
- `while contador <= 3`: antes de cada vuelta, Ruby hace la pregunta. `1 <= 3` es
  verdadero: entra en el bucle;
- el bloque muestra «Vuelta 1» y luego `contador += 1` suma 1: `contador` vale 2;
- Ruby vuelve a subir al `while` y repite la pregunta, con 2 y luego con 3;
- cuando `contador` vale 4, `4 <= 3` es falso: Ruby sale del bucle, después del `end`.

> **Atención:** si la condición sigue siendo siempre verdadera, el bucle no se detiene nunca
> (un **bucle infinito**). Aquí, olvidar `contador += 1` haría que el programa girara
> sin fin. Detenlo con Ctrl+C.

**Te toca:** escribe un programa que muestre los números del 1 al 5 con un bucle
`while`, y luego «¡Terminado!».

> **Solución**
>
> ```ruby
> n = 1
> while n <= 5
>   puts n
>   n += 1
> end
> puts "¡Terminado!"
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> ¡Terminado!
> ```

## El bucle until

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

`until` repite **hasta que** la condición se vuelva verdadera.

```ruby
n = 3
until n.zero?
  puts n
  n -= 1
end
puts "¡Ya!"
```

```sortie
3
2
1
¡Ya!
```

## El bucle loop

`loop` repite sin fin. Sales con `break`. Es práctico cuando la condición
de parada está en medio de la vuelta.

```ruby
total = 0
loop do
  total += 7
  break if total > 20
  puts "Total: #{total}"
end
puts "Terminado: #{total}"
```

```sortie
Total: 7
Total: 14
Terminado: 21
```

Línea por línea:

- `total` vale 0 al principio;
- primera vuelta: `total += 7` da 7. `7 > 20` es falso: no hay `break`, y el `puts`
  muestra 7;
- segunda vuelta: `total` vale 14, mismo camino;
- tercera vuelta: `total` vale 21. `21 > 20` es verdadero: `break` sale del bucle
  antes del `puts`;
- Ruby sigue después del `end` y muestra «Terminado: 21».

## times y each

En Ruby, a menudo se prefieren `times` y `each` a los bucles clásicos. No tienes
que manejar un contador: es imposible olvidar `+= 1`.

```ruby
3.times { |i| puts "i vale #{i}" }
(1..3).each { |n| puts n * 10 }
```

```sortie
i vale 0
i vale 1
i vale 2
10
20
30
```

Línea por línea:

- `3.times` ejecuta el bloque tres veces: `i` vale 0, luego 1, luego 2;
- `(1..3).each` toma cada número del rango, uno por uno, y lo guarda en `n`;
- el bloque muestra `n * 10`: 10, luego 20, luego 30.

El código entre `{ }` es un **bloque**. Entre las barras `|i|`, nombras el valor que
el bloque recibe en cada vuelta.

> **Recuerda:** `times` cuenta a partir de `0`. `each` recorre cada elemento de una
> colección. Para saber más, lee [los bloques](blocs.md).

**Te toca:** escribe un programa que muestre la tabla del 7, de «7 x 1 = 7» a
«7 x 5 = 35», una línea por cálculo, con `each` sobre un rango.

> **Solución**
>
> ```ruby
> (1..5).each { |i| puts "7 x #{i} = #{7 * i}" }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

### Contar con upto, downto y step

- `upto` cuenta hacia arriba, `downto` hacia abajo;
- `step` avanza a saltos: de 2 en 2, de 5 en 5…

```ruby
1.upto(3) { |n| puts "upto: #{n}" }
3.downto(2) { |n| puts "downto: #{n}" }
0.step(10, 5) { |n| puts "step: #{n}" }
```

```sortie
upto: 1
upto: 2
upto: 3
downto: 3
downto: 2
step: 0
step: 5
step: 10
```

`0.step(10, 5)` quiere decir: «de 0 a 10, de 5 en 5».

### Tener la posición con each_with_index

`each_with_index` da el elemento **y** su posición (su
[índice](../debuter/glossaire.md#indice)), que empieza en `0`.

```ruby
["oro", "plata", "bronce"].each_with_index do |medalla, i|
  puts "#{i + 1}. #{medalla}"
end
```

```sortie
1. oro
2. plata
3. bronce
```

`do ... end` reemplaza a las llaves cuando el bloque ocupa varias líneas.

## break y next

A veces quieres cambiar el rumbo de un bucle por el camino: dejar de buscar en
cuanto has encontrado algo, o ignorar algunos elementos. Ese es el papel de estas dos
palabras.

`break` detiene el bucle. `next` pasa directamente a la vuelta siguiente.

```ruby
(1..10).each do |n|
  next if n.even?
  break if n > 7
  puts n
end
```

```sortie
1
3
5
7
```

- Para `2`, `4`, `6`: `next` se salta el resto de la vuelta.
- Para `9`: `break` lo detiene todo.

Línea por línea:

- `(1..10).each` da a `n` los valores 1, 2, 3… hasta 10;
- `n.even?` pregunta «¿`n` es par?». Si lo es, `next` pasa a la vuelta siguiente: no
  se llega al `puts`;
- `break if n > 7`: en cuanto `n` pasa de 7 (aquí, en el 9), el bucle se detiene: el 9
  y el 10 no se muestran nunca;
- si no, `puts n` muestra el número.

## Errores frecuentes

### Llamar a each sobre un número

```ruby norun
5.each { |n| puts n }
```

```text
main.rb:1:in `<main>': undefined method `each' for an instance of Integer (NoMethodError)

5.each { |n| puts n }
 ^^^^^
```

Un número no es una colección: no tiene método `each`. Para repetir 5 veces, escribe
`5.times`. Para recorrer los números del 1 al 5, escribe `(1..5).each`.

### Usar la variable del bloque después del bloque

```ruby norun
3.times { |i| puts i }
puts "Final: #{i}"
```

```text
0
1
2
main.rb:2:in `<main>': undefined local variable or method `i' for main (NameError)

puts "Final: #{i}"
               ^
```

`i` solo existe dentro del bloque, entre `{` y `}`. Para conservar un valor después
del bucle, crea la variable antes del bloque (`ultimo = nil`) y cámbiala dentro del
bloque (`ultimo = i`).

## Ponte a prueba

**Pregunta 1:** ¿cuántos números contiene `(1...10).to_a`?

> **Respuesta**
>
> 9 números, del 1 al 9: con tres puntos, el último valor queda excluido.

**Pregunta 2:** ¿cómo mostrar 10, 8, 6, 4, 2, 0?

> **Respuesta**
>
> Con `step` y un paso negativo:
>
> ```ruby
> 10.step(0, -2) { |n| puts n }
> ```
>
> ```sortie
> 10
> 8
> 6
> 4
> 2
> 0
> ```

**Pregunta 3:** ¿qué diferencia hay entre `break` y `next`?

> **Respuesta**
>
> `break` detiene todo el bucle: Ruby sigue después del `end`. `next` solo se salta
> el resto de la vuelta actual y pasa a la vuelta siguiente.

## En resumen

- `1..5` incluye 5, `1...5` lo excluye.
- `while`, `until` y `loop` repiten; `times`, `each`, `upto` y `step` son más habituales.
- `each_with_index` da cada elemento y su posición, empezando en `0`.
- `break` detiene el bucle, `next` pasa a la vuelta siguiente.

Siguiente: [los métodos](fonctions.md). Compara con
[las condiciones y bucles en Rythme](../rythme/conditions-boucles.md).
