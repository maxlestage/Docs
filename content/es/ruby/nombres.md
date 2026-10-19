# Números y cálculos

Esta página muestra cómo calcula Ruby con números enteros y decimales, cómo redondear, contar dinero sin errores y mostrar un número de forma bonita.

> **Objetivos:**
>
> - sabrás diferenciar un `Integer` de un `Float`;
> - sabrás evitar la trampa de la división entera;
> - sabrás redondear, convertir y dar formato a un número;
> - sabrás por qué `0.1 + 0.2` no vale exactamente `0.3`, y qué hacer con el dinero;
> - sabrás sacar números al azar de forma reproducible.

## Dos tipos de números

Ruby tiene dos tipos principales de números:

- los [enteros](../debuter/glossaire.md#entero) (`Integer`): `3`, `-12`, `1_000_000`;
- los [flotantes](../debuter/glossaire.md#flotante) (`Float`), con la coma decimal
  escrita como un punto: `3.5`, `-0.25`.

```ruby
puts 42.class
puts 3.14.class
puts 1_000_000
puts 2**100
```

```sortie
Integer
Float
1000000
1267650600228229401496703205376
```

El `_` solo sirve para que el número se lea mejor, como el espacio en «1 000 000».
Un entero de Ruby no tiene límite de tamaño: `2**100` funciona sin problema.

## Los operadores

Los [operadores](../debuter/glossaire.md#operador) hacen los cálculos básicos.

```ruby
puts 7 + 2
puts 7 - 2
puts 7 * 2
puts 7 / 2
puts 7 % 2
puts 7**2
```

```sortie
9
5
14
3
1
49
```

- `%` da el **resto** de la división: 7 = 2 × 3 + **1**.
- `**` es la potencia: `7**2` vale 7 × 7.

> **Consejo:** `n % 2 == 0` comprueba si `n` es par. Ruby también tiene `n.even?` y
> `n.odd?`.

### El orden de las operaciones

Como en el colegio, `**` va antes que `*`, `/` y `%`, que van antes que `+` y `-`. Los
paréntesis van antes que todo.

```ruby
puts 2 + 3 * 4
puts (2 + 3) * 4
puts 2 * 3**2
puts -2**2
```

```sortie
14
20
18
-4
```

El último resultado sorprende: `-2**2` se lee `-(2**2)`. Escribe `(-2)**2` para obtener
`4`. Ante la duda, pon paréntesis: no cuestan nada.

## División entera y división decimal

Si los dos números son enteros, `/` da un entero: la parte después de la coma se
**descarta**. Es como repartir 7 caramelos entre 2 niños: cada uno recibe 3, y sobra 1.

```ruby
puts 7 / 2
puts 7.0 / 2
puts 7 / 2.0
puts 7.fdiv(2)
p 7.divmod(2)
```

```sortie
3
3.5
3.5
3.5
[3, 1]
```

En cuanto uno de los dos números es un `Float`, el resultado es un `Float`. `fdiv` fuerza
una división decimal, y `divmod` da el cociente y el resto de una vez.

> **Atención:** `-7 / 2` vale `-4`, no `-3`: Ruby siempre redondea hacia abajo.

## Redondear

```ruby
x = 3.756
puts x.round
puts x.round(1)
puts x.floor
puts x.ceil
puts x.truncate
puts 1234.round(-2)
```

```sortie
4
3.8
3
4
3
1200
```

- `round` redondea al más cercano; `round(1)` conserva una cifra después de la coma.
- `floor` redondea hacia abajo (el «suelo»), `ceil` hacia arriba (el «techo»).
- `truncate` corta la parte después de la coma.
- Un número negativo en `round` redondea a las decenas, centenas…

## La trampa de los flotantes

Un `Float` se guarda en binario en el ordenador. Algunos números sencillos para
nosotros, como `0.1`, no salen exactos en binario, igual que `1/3` no sale exacto en
decimal (`0.3333…`). De ahí este resultado sorprendente:

```ruby
puts 0.1 + 0.2
puts 0.1 + 0.2 == 0.3
puts (0.1 + 0.2 - 0.3).abs < 1e-9
```

```sortie
0.30000000000000004
false
true
```

Para comparar dos flotantes, mira si su diferencia es minúscula en lugar de usar `==`.

### Rational: las fracciones exactas

Un `Rational` es una fracción de verdad. Se escribe con una `r` al final.

```ruby
tercio = 1/3r
puts tercio
puts tercio * 3
puts 0.1r + 0.2r == 0.3r
puts (3/4r).to_f
```

```sortie
1/3
1/1
true
0.75
```

### BigDecimal: para el dinero

Para precios, usa `BigDecimal`. Calcula en decimal, como en un tique de caja. Hay que
cargarlo con `require`.

```ruby
require "bigdecimal"

precio = BigDecimal("19.99")
total = precio * 3
puts total.to_s("F")
puts BigDecimal("0.1") + BigDecimal("0.2") == BigDecimal("0.3")
```

```sortie
59.97
true
```

> **Recuerda:** crea un `BigDecimal` a partir de un **texto** (`"19.99"`), no de un
> flotante: el flotante ya sería impreciso. `to_s("F")` lo muestra sin notación
> científica.

Otra solución sencilla: contar en céntimos, con enteros (`1999` en lugar de `19.99`).

## El módulo Math

`Math` agrupa las funciones matemáticas.

```ruby
puts Math.sqrt(16)
puts Math::PI.round(4)
puts Math.hypot(3, 4)
puts Math.log10(1000)
puts 10.pow(3)
puts -7.abs
```

```sortie
4.0
3.1416
5.0
3.0
1000
7
```

`Math.sqrt` es la raíz cuadrada, `Math.hypot(3, 4)` la hipotenusa de un triángulo
rectángulo. `abs` da el valor sin el signo.

## El azar con rand

`rand(1..6)` saca un número entre 1 y 6, como un dado. El resultado cambia cada vez que
ejecutas el programa. Para obtener siempre la misma secuencia (útil para hacer pruebas),
da una **semilla** con `srand` o crea un generador `Random`.

```ruby
dado = Random.new(42)
p 5.times.map { dado.rand(1..6) }

dado = Random.new(42)
p 5.times.map { dado.rand(1..6) }
```

```sortie
[4, 5, 3, 5, 5]
[4, 5, 3, 5, 5]
```

Misma semilla, misma secuencia. `srand(42)` hace lo mismo para `rand` a secas. Sin
semilla, cada ejecución da otros números.

## Convertir

```ruby
puts "42".to_i + 1
puts "3.5".to_f * 2
puts "12abc".to_i
puts "abc".to_i
puts 7.to_f
puts 7.9.to_i
puts Integer("42")
```

```sortie
43
7.0
12
0
7.0
7
42
```

`to_i` es tolerante: lee el principio del texto y devuelve `0` si no encuentra nada.
`Integer("...")` es estricto: lanza un error si el texto no es un número. Es mejor para
comprobar lo que escribe el usuario.

```ruby
begin
  Integer("12abc")
rescue ArgumentError => e
  puts "Entrada no válida: #{e.message}"
end
```

```sortie
Entrada no válida: invalid value for Integer(): "12abc"
```

### Las cifras de un número

`digits` da las cifras, **empezando por el final**.

```ruby
p 1234.digits
puts 1234.digits.sum
puts 255.to_s(2)
puts "ff".to_i(16)
```

```sortie
[4, 3, 2, 1]
10
11111111
255
```

`to_s(2)` escribe el número en binario, `to_i(16)` lee un número escrito en hexadecimal.

## Dar formato a un número

`format` construye un texto a partir de un modelo. `%.2f` quiere decir «un flotante con
2 cifras después de la coma», `%05d` «un entero de 5 cifras, completado con ceros».

```ruby
puts format("%.2f", 3.14159)
puts format("%05d", 42)
puts format("%-6s|", "ab")
puts format("%d %% de aciertos", 87)
```

```sortie
3.14
00042
ab    |
87 % de aciertos
```

### Separar los miles

Ruby no tiene un método ya hecho para escribir `1 234 567`. Una pequeña expresión
regular lo hace:

```ruby
def miles(n)
  n.to_s.reverse.scan(/\d{1,3}/).join(" ").reverse
end

puts miles(1234567)
puts miles(999)
puts format("%.2f €", 1234.5).sub(".", ",")
```

```sortie
1 234 567
999
1234,50 €
```

Se le da la vuelta al texto, se corta en grupos de 3, se vuelve a pegar con espacios y
luego se le da la vuelta otra vez.

## Errores frecuentes

### La división que pierde los decimales

```ruby norun
media = (12 + 15) / 2
puts media
```

```text
13
```

No hay error, pero el resultado es falso: la media real es `13.5`. Los dos números son
enteros, así que la división es entera. Escribe `(12 + 15) / 2.0` o
`(12 + 15).fdiv(2)`.

### Dividir un entero entre cero

```ruby norun
puts 10 / 0
```

```text
main.rb:1:in `/': divided by 0 (ZeroDivisionError)
	from main.rb:1:in `<main>'
```

Comprueba el divisor antes de dividir. Fíjate en que `10 / 0.0` no falla: da
`Infinity`, que a menudo no es mejor.

### Sumar un texto y un número

```ruby norun
edad = "30"
puts edad + 1
```

```text
main.rb:2:in `+': no implicit conversion of Integer into String (TypeError)

puts edad + 1
            ^
	from main.rb:2:in `<main>'
```

`"30"` es un texto. Conviértelo primero: `edad.to_i + 1`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto valen `17 / 5` y `17 % 5`?

> **Respuesta**
>
> `3` y `2`: 17 = 5 × 3 + 2.
>
> ```ruby
> p 17.divmod(5)
> ```
>
> ```sortie
> [3, 2]
> ```

**Pregunta 2:** ¿cómo muestras `2.5` redondeado al entero superior?

> **Respuesta**
>
> Con `ceil`.
>
> ```ruby
> puts 2.5.ceil
> ```
>
> ```sortie
> 3
> ```

**Pregunta 3:** ¿por qué usar `BigDecimal("0.1")` en lugar de `0.1` para un precio?

> **Respuesta**
>
> Porque `0.1` es un `Float`, guardado en binario de forma aproximada. Los pequeños
> errores se acumulan (`0.1 + 0.2` da `0.30000000000000004`). `BigDecimal` calcula en
> decimal, de forma exacta.

## En resumen

- `Integer` para los enteros, `Float` para los números decimales.
- Entre dos enteros, `/` descarta los decimales: usa `2.0` o `fdiv`.
- `round`, `floor`, `ceil` redondean; `format("%.2f", x)` muestra el número limpio.
- Los flotantes son aproximados: `Rational` o `BigDecimal` para cálculos exactos.
- `Integer("...")` convierte comprobando; `to_i` convierte sin quejarse.
- `Random.new(semilla)` da un azar reproducible.

Siguiente: [Condiciones y bucles](conditions-boucles.md)
