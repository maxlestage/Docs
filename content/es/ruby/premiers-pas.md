# Primeros pasos

Esta página presenta las bases de Ruby: variables, números, textos, símbolos, booleanos, `nil` y la forma de mostrar resultados.

> **Objetivos:**
>
> - sabrás guardar un valor en una variable y cambiarlo;
> - sabrás calcular con enteros y con números decimales;
> - sabrás escribir un texto e insertar un valor en él;
> - sabrás qué son `true`, `false` y `nil`;
> - sabrás mostrar resultados con `puts`, `print` y `p`.

## Las variables

Una [variable](../debuter/glossaire.md#variable) es un nombre que designa un
[valor](../debuter/glossaire.md#valor). Piensa en una etiqueta pegada en una caja:
la etiqueta es el nombre, el contenido de la caja es el valor. La creas con `=`.

```ruby
edad = 30
nombre = "Alicia"
puts nombre
puts edad
```

```sortie
Alicia
30
```

Aquí, `=` no quiere decir «es igual a». Quiere decir «guarda este valor con este
nombre». Se llama **asignación**.

No necesitas indicar el [tipo](../debuter/glossaire.md#tipo) (número,
texto…): Ruby lo adivina. `class` te dice lo que ha adivinado.

```ruby
puts 30.class
puts "Alicia".class
puts 2.5.class
```

```sortie
Integer
String
Float
```

### Cambiar el valor de una variable

Una variable puede cambiar de valor. El nuevo valor reemplaza al anterior.

```ruby
puntos = 10
puntos = puntos + 5
puts puntos
puntos += 1
puts puntos
puntos -= 6
puts puntos
```

```sortie
15
16
10
```

`puntos += 1` es un atajo de `puntos = puntos + 1`. También existen `-=`, `*=` y
`/=`.

## Los números

Ruby distingue los enteros (`Integer`, sin decimales) y los números decimales (`Float`,
llamados **flotantes**). En Ruby, los decimales se escriben con un punto: `2.5`.

### Los operadores

Un **operador** es un signo que hace un cálculo.

| Operador | Significado | Ejemplo | Resultado |
|-----------|------|---------|----------|
| `+` | suma | `7 + 3` | `10` |
| `-` | resta | `7 - 3` | `4` |
| `*` | multiplicación | `7 * 3` | `21` |
| `/` | división | `7 / 2` | `3` |
| `%` | resto de la división | `7 % 2` | `1` |
| `**` | potencia | `2 ** 10` | `1024` |

```ruby
puts 7 + 3
puts 7 / 2
puts 7.0 / 2
puts 7 % 2
puts 2 ** 10
```

```sortie
10
3
3.5
1
1024
```

### División entera y división decimal

> **Atención:** `7 / 2` da `3`. Entre dos enteros, la división solo guarda la
> parte entera. Escribe `7.0 / 2` para obtener `3.5`.

Basta con que **uno solo** de los dos números sea un flotante. Si tus dos números están
en variables, usa `to_f` (*to float*, «a flotante») o `fdiv`.

```ruby
total = 7
partes = 2
puts total / partes
puts total.to_f / partes
puts total.fdiv(partes)
```

```sortie
3
3.5
3.5
```

`%` (el **módulo**) da el resto. Sirve a menudo para saber si un número es par:
`n % 2` vale `0` para un número par.

### El orden de los cálculos

Como en matemáticas, `*`, `/` y `%` van antes que `+` y `-`. `**` va antes que
todo. Es la **precedencia** de los operadores. Los paréntesis cambian el orden.

```ruby
puts 2 + 3 * 4
puts (2 + 3) * 4
puts 2 * 3 ** 2
puts 10 - 4 - 3
```

```sortie
14
20
18
3
```

> **Consejo:** si tienes dudas, añade paréntesis. No cuestan nada y
> hacen claro el cálculo.

### Redondear

```ruby
puts 3.7.round
puts 3.14159.round(2)
puts 3.7.floor
puts 3.2.ceil
puts -5.abs
```

```sortie
4
3.14
3
4
5
```

- `round` redondea al más cercano; `round(2)` guarda 2 cifras decimales.
- `floor` redondea hacia abajo, `ceil` hacia arriba.
- `abs` da el valor sin el signo (el **valor absoluto**).

## Las cadenas de caracteres

Un texto se escribe entre comillas. Se habla de
[cadena de caracteres](../debuter/glossaire.md#cadena-de-caracteres), o de **cadena**
(`String`): las letras están ensartadas una tras otra, como perlas.

```ruby
lenguaje = "Ruby"
puts lenguaje.length
puts lenguaje.upcase
puts lenguaje + " mola"
```

```sortie
4
RUBY
Ruby mola
```

- `length` da el número de caracteres.
- `upcase` pone en mayúsculas.
- `+` pega dos textos uno detrás del otro.

### La interpolación

Entre comillas dobles, `#{...}` inserta un valor en el texto.

```ruby
nombre = "Alicia"
edad = 30
puts "#{nombre} tiene #{edad} años."
puts "Dentro de 5 años: #{edad + 5}"
```

```sortie
Alicia tiene 30 años.
Dentro de 5 años: 35
```

> **Atención:** la interpolación no funciona con comillas simples:
> `'#{edad}'` muestra el texto tal cual.

### Pasar de un texto a un número

Un texto que contiene cifras no es un número: `"5"` no es `5`. Para
pasar de uno a otro:

- `to_i` (*to integer*) convierte en entero;
- `to_f` convierte en flotante;
- `to_s` (*to string*) convierte en texto.

```ruby
puts "5" + "5"
puts "5".to_i + 5
puts 5.to_s + "5"
```

```sortie
55
10
55
```

El capítulo [trabajar con texto](texte.md) va mucho más lejos.

## Los símbolos

Un símbolo empieza por `:`. Es un nombre fijo, usado a menudo como etiqueta.
A diferencia de un texto, no se modifica: solo sirve para nombrar algo.

```ruby
color = :rojo
puts color
p color
```

```sortie
rojo
:rojo
```

Los volverás a ver como claves de hash en [los arrays y hashes](collections.md).

## Los booleanos y nil

`true` (verdadero) y `false` (falso) son los [booleanos](../debuter/glossaire.md#booleano).
`nil` quiere decir «nada, ningún valor»: es una caja vacía.

### Comparar dos valores

Una comparación responde con `true` o `false`.

| Operador | Significado |
|-----------|------|
| `==` | igual a |
| `!=` | distinto de |
| `<`, `>` | menor, mayor |
| `<=`, `>=` | menor o igual, mayor o igual |

```ruby
puts 3 > 2
puts 3 == 4
puts 3 != 4
puts "a" == "a"
p nil
puts nil.nil?
```

```sortie
true
false
true
true
nil
true
```

> **Atención:** `=` guarda un valor, `==` compara dos valores. No los confundas.

> **Recuerda:** en Ruby, solo `false` y `nil` son «falsos». Todo lo demás es verdadero,
> incluso `0` y `""`.

## Mostrar con puts, print y p

- `puts` muestra y luego pasa a la línea siguiente.
- `print` muestra sin pasar a la línea siguiente.
- `p` muestra el valor tal como Ruby lo ve (práctico para depurar).

```ruby
print "a"
print "b"
puts
puts "texto"
p "texto"
p 42
```

```sortie
ab
texto
"texto"
42
```

`p` muestra las comillas: ves enseguida que `"42"` es un texto y `42`
un número.

```ruby
p "42"
p 42
puts "42"
puts 42
```

```sortie
"42"
42
42
42
```

## Los comentarios

Un comentario empieza por `#`. Ruby lo ignora. Sirve para explicar el código a las
personas.

```ruby
# Esto es un comentario
puts "visible" # un comentario al final de la línea
```

```sortie
visible
```

## Las convenciones de nombres

- Variables y métodos: `snake_case` (`precio_total`), en minúsculas con `_`.
- Clases y módulos: `CamelCase` (`CuentaBancaria`).
- Constantes: en mayúsculas (`TASA_IVA`).

Una **constante** es un valor que no debe cambiar. Su nombre empieza por
mayúscula.

```ruby
TASA_IVA = 0.2
precio_sin_iva = 100
precio_con_iva = precio_sin_iva * (1 + TASA_IVA)
puts precio_con_iva
```

```sortie
120.0
```

El resultado es `120.0` y no `120`: un cálculo que mezcla un entero y un flotante
da un flotante.

> **Consejo:** sigue estas convenciones: todo el mundo las usa, y tu código será
> más fácil de leer.

## Errores frecuentes

### Sumar un texto y un número

```ruby norun
puts "Edad: " + 30
```

```text
main.rb:1:in `+': no implicit conversion of Integer into String (TypeError)

puts "Edad: " + 30
                ^^
	from main.rb:1:in `<main>'
```

Ruby se niega a pegar un número a un texto. Convierte el número, o usa
la interpolación:

```ruby
puts "Edad: " + 30.to_s
puts "Edad: #{30}"
```

```sortie
Edad: 30
Edad: 30
```

### Una errata en el nombre de una variable

```ruby norun
nombre = "Alicia"
puts nmbre
```

```text
main.rb:2:in `<main>': undefined local variable or method `nmbre' for main (NameError)

puts nmbre
     ^^^^^
Did you mean?  nombre
```

`nmbre` no existe. Ruby propone el nombre más parecido. Corrígelo en `puts nombre`.

### Dividir entre cero

```ruby norun
puts 10 / 0
```

```text
main.rb:1:in `/': divided by 0 (ZeroDivisionError)
	from main.rb:1:in `<main>'
```

Una división entera entre cero es imposible. Comprueba el divisor antes de calcular.
Con un flotante, Ruby no falla, sino que da `Infinity` (el infinito):

```ruby
puts 10.0 / 0
```

```sortie
Infinity
```

## Ponte a prueba

**Pregunta 1:** ¿cuánto vale `10 / 4`? ¿Y `10 / 4.0`?

> **Respuesta**
>
> `10 / 4` vale `2`: entre dos enteros, la división guarda la parte entera.
> `10 / 4.0` vale `2.5`.

**Pregunta 2:** ¿qué muestra este código?

```ruby norun
x = 2 + 3 * 2
puts x
```

> **Respuesta**
>
> `8`: la multiplicación va antes que la suma. Escribe `(2 + 3) * 2` para obtener
> `10`.

**Pregunta 3:** ¿`0` es verdadero o falso en una condición de Ruby?

> **Respuesta**
>
> Verdadero. Solo `false` y `nil` son falsos.

## En resumen

- Una variable se crea con `=`; Ruby adivina el tipo solo.
- `*` y `/` van antes que `+` y `-`; los paréntesis cambian el orden.
- `7 / 2` vale `3` entre enteros; escribe `7.0 / 2` para obtener `3.5`.
- Un texto se escribe entre comillas; `#{...}` inserta un valor en él.
- Solo `false` y `nil` son falsos; todo lo demás es verdadero.
- `puts` muestra, `print` muestra sin salto de línea, `p` muestra el valor exacto.

Siguiente: [Números y cálculos](nombres.md)
