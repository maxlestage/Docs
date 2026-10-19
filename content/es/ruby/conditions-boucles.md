# Condiciones y bucles

Esta página muestra cómo tomar decisiones con `if`, `unless` y `case`, y cómo repetir acciones con los bucles.

> **Objetivos:**
>
> - sabrás ejecutar código solo si una condición es verdadera;
> - sabrás combinar condiciones con `&&`, `||` y `!`;
> - sabrás elegir entre varios casos con `case`;
> - sabrás repetir código con `while`, `loop`, `times`, `each` y `step`;
> - sabrás detener un bucle o saltarte una vuelta.

## if, elsif y else

Una [condición](../debuter/glossaire.md#condicion) es una pregunta que se responde
con verdadero o falso: «¿la nota es mayor que 10?». Es un cruce de caminos: según
la respuesta, el programa toma un camino u otro.

`if` ejecuta código solo si una condición es verdadera. El bloque termina con `end`.

```ruby
nota = 14
if nota >= 16
  puts "Muy bien"
elsif nota >= 10
  puts "Aprobado"
else
  puts "Suspenso"
end
```

```sortie
Aprobado
```

- `if` («si») hace la primera pregunta.
- `elsif` («si no, si») hace otra pregunta, solo si las anteriores han
  respondido que no. Cuidado con la ortografía: `elsif`, sin `e` en medio.
- `else` («si no») se ejecuta si ninguna pregunta ha respondido que sí.
- Ruby se detiene en la **primera** rama verdadera.

Las comparaciones (`==`, `!=`, `<`, `>`, `<=`, `>=`) se presentan en
[los primeros pasos](premiers-pas.md).

### La forma corta

Para una sola instrucción, puedes poner `if` al final de la línea.

```ruby
temperatura = 35
puts "¡Hace calor!" if temperatura > 30
```

```sortie
¡Hace calor!
```

### if devuelve un valor

En Ruby, `if` da un valor: el de la rama elegida. Puedes guardarlo
en una variable.

```ruby
nota = 8
resultado = if nota >= 10
              "aprobado"
            else
              "suspenso"
            end
puts resultado
```

```sortie
suspenso
```

## unless

`unless` («a menos que») es lo contrario de `if`: el código se ejecuta si la condición
es **falsa**.

```ruby
conectado = false
puts "Inicia sesión, por favor." unless conectado
```

```sortie
Inicia sesión, por favor.
```

> **Consejo:** evita `unless` con `else`: es difícil de leer. Usa mejor `if`.

## Combinar condiciones

`&&` quiere decir «y», `||` quiere decir «o», `!` quiere decir «no».

- `a && b` es verdadero si `a` **y** `b` son verdaderos.
- `a || b` es verdadero si al menos uno de los dos es verdadero.
- `!a` es verdadero si `a` es falso.

```ruby
edad = 20
socio = true
puts "Entrada gratuita" if edad < 26 && socio
puts "No es socio" if !socio
puts "Tarifa reducida" if edad < 18 || edad > 65
puts "Tarifa completa" unless edad < 18 || edad > 65
```

```sortie
Entrada gratuita
Tarifa completa
```

`&&` va antes que `||`, como `*` va antes que `+`. Añade paréntesis para que quede
claro:

```ruby
edad = 70
estudiante = false
socio = false
puts "Descuento" if (edad > 65 || estudiante) && !socio
```

```sortie
Descuento
```

> **Nota:** Ruby conoce también `and`, `or` y `not`. No tienen la misma precedencia
> que `&&` y `||`: para empezar, usa solo `&&`, `||` y `!`.

## El ternario

`condición ? valor_si_verdadero : valor_si_falso` elige un valor en una línea.

```ruby
edad = 15
estado = edad >= 18 ? "mayor de edad" : "menor de edad"
puts estado
```

```sortie
menor de edad
```

Léelo como una pregunta: «¿edad ≥ 18? si sí, mayor de edad; si no, menor de edad».

## case y when

`case` compara un valor con varias posibilidades. Es más legible que una larga
serie de `elsif`.

```ruby
dia = "sábado"
case dia
when "sábado", "domingo"
  puts "¡Fin de semana!"
when "lunes"
  puts "Ánimo..."
else
  puts "Día laborable"
end
```

```sortie
¡Fin de semana!
```

- Una coma en `when` quiere decir «o».
- `else` sirve si ningún `when` coincide.

`when` acepta también rangos:

```ruby
nota = 12
calificacion = case nota
               when 0...10 then "insuficiente"
               when 10...14 then "aprobado"
               else "notable"
               end
puts calificacion
```

```sortie
aprobado
```

`then` permite escribir la respuesta en la misma línea que `when`.

## Los rangos

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

> **Atención:** si la condición sigue siendo siempre verdadera, el bucle no se detiene nunca
> (un **bucle infinito**). Aquí, olvidar `contador += 1` haría que el programa girara
> sin fin. Detenlo con Ctrl+C.

## El bucle until

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

El código entre `{ }` es un **bloque**. Entre las barras `|i|`, nombras el valor que
el bloque recibe en cada vuelta.

> **Recuerda:** `times` cuenta a partir de `0`. `each` recorre cada elemento de una
> colección. Para saber más, lee [los bloques](blocs.md).

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

## Errores frecuentes

### Olvidar end

```ruby norun
nota = 14
if nota >= 10
  puts "Aprobado"
```

```text
main.rb: --> main.rb
Unmatched keyword, missing `end' ?
> 2  if nota >= 10
main.rb:3: syntax error, unexpected end-of-input, expecting `end' or dummy end (SyntaxError)
  puts "Aprobado"
                 ^
```

Cada `if`, `case`, `while`, `until` y `do` debe cerrarse con un `end`. Añade
`end` después de `puts "Aprobado"`. Indenta tu código: un `end` que falta se ve
enseguida.

### Escribir = en lugar de ==

```ruby norun
nota = 14
if nota = 10
  puts "diez"
end
```

```text
main.rb:2: warning: found `= literal' in conditional, should be ==
diez
```

`nota = 10` **guarda** 10 en `nota`: la condición es siempre verdadera. Ruby avisa
(*warning*: «advertencia»), pero el programa continúa. Escribe `if nota == 10`.

### Escribir elseif o else if

```ruby norun
nota = 12
if nota >= 10
  puts "Aprobado"
elseif nota >= 5
  puts "Recuperación"
end
```

```text
Aprobado
main.rb:4:in `<main>': undefined method `elseif' for main (NoMethodError)

elseif nota >= 5
^^^^^^
Did you mean?  elsif
```

Ruby lee `elseif` como un método desconocido. La palabra clave se escribe `elsif`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```ruby norun
x = 5
puts "A" if x > 3 && x < 5
puts "B" if x > 3 || x < 5
```

> **Respuesta**
>
> Solo `B`. `x < 5` es falso (5 no es menor que 5), así que el `&&` es falso.
> Con `||`, basta con que `x > 3` sea verdadero.

**Pregunta 2:** ¿cuántos números contiene `(1...10).to_a`?

> **Respuesta**
>
> 9 números, del 1 al 9: con tres puntos, el último valor queda excluido.

**Pregunta 3:** ¿cómo mostrar 10, 8, 6, 4, 2, 0?

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

## En resumen

- `if`, `elsif` y `else` eligen qué hacer; `unless` es lo contrario de `if`.
- `&&` (y), `||` (o), `!` (no) combinan condiciones.
- `case` y `when` comparan un valor con varias posibilidades.
- `1..5` incluye 5, `1...5` lo excluye.
- `while`, `until` y `loop` repiten; `times`, `each`, `upto` y `step` son más habituales.
- `break` detiene el bucle, `next` pasa a la vuelta siguiente.

Siguiente: [los métodos](fonctions.md). Compara con
[las condiciones y bucles en Rythme](../rythme/conditions-boucles.md).
