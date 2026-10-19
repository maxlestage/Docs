# Ejercicios para empezar con buen pie

Esta página propone 15 ejercicios muy guiados, todos fáciles, para practicar lo básico de Ruby: variables, cálculos, condiciones, bucles, métodos, textos, arrays, hashes y rangos.

## Cómo hacer estos ejercicios

Hazlos en orden. Para cada ejercicio:

1. **Lee** el enunciado y la salida esperada.
2. **Pruébalo en papel**: ¿qué pasos? ¿qué variables?
3. **Escribe** tu programa en un archivo `ejercicio.rb`.
4. **Ejecútalo** con `ruby ejercicio.rb` (mira [la introducción](introduction.md#ejecutar-un-programa)).
5. **Compara** con la salida esperada, carácter por carácter.
6. ¿Atascado? Lee la **primera pista**, luego la segunda. Abre la **solución** al
   final.

> **Consejo:** un error no es un fracaso. Lee el mensaje: te da la línea y la causa.
> Mira [leer un mensaje de error](introduction.md#leer-un-mensaje-de-error).

## Mostrar y calcular

### Ejercicio 1: Saludar

Nivel: ★☆☆

Guarda tu nombre en una variable `nombre`. Después muestra una frase de bienvenida que
contenga ese nombre.

Salida esperada (con `"Léa"`):

```text
Hola Léa, ¡bienvenida!
```

> **Pista:** crea la variable con `nombre = "Léa"`.

> **Pista:** en un texto entre comillas dobles, `#{nombre}` se sustituye por el valor de
> la variable.

> **Solución**
>
> ```ruby
> nombre = "Léa"
> puts "Hola #{nombre}, ¡bienvenida!"
> ```
>
> ```sortie
> Hola Léa, ¡bienvenida!
> ```
>
> Por qué funciona: `#{...}` se llama interpolación. Ruby calcula lo que hay entre las
> llaves y lo pone en el texto.

### Ejercicio 2: Dentro de diez años

Nivel: ★☆☆

Guarda una edad en una variable `edad`. Calcula la edad dentro de 10 años y muéstrala.

Salida esperada (con `edad = 25`):

```text
Hoy: 25 años
Dentro de 10 años: 35 años
```

> **Pista:** crea una segunda variable: `mas_tarde = edad + 10`.

> **Solución**
>
> ```ruby
> edad = 25
> mas_tarde = edad + 10
> puts "Hoy: #{edad} años"
> puts "Dentro de 10 años: #{mas_tarde} años"
> ```
>
> ```sortie
> Hoy: 25 años
> Dentro de 10 años: 35 años
> ```
>
> Por qué funciona: `edad + 10` calcula un valor nuevo sin cambiar `edad`. Lo guardas
> con otro nombre para mostrarlo.

### Ejercicio 3: El precio con IVA

Nivel: ★☆☆

Un artículo cuesta 40 euros sin impuestos. El IVA es del 20 %. Muestra el precio con
impuestos incluidos.

Salida esperada:

```text
Precio con IVA: 48.0 euros
```

> **Pista:** un 20 % más es multiplicar por `1.2`.

> **Solución**
>
> ```ruby
> precio_sin_iva = 40
> precio_con_iva = precio_sin_iva * 1.2
> puts "Precio con IVA: #{precio_con_iva} euros"
> ```
>
> ```sortie
> Precio con IVA: 48.0 euros
> ```
>
> Por qué funciona: `1.2` es un número decimal, así que el resultado también: Ruby
> muestra `48.0`. Mira [los números](nombres.md).

### Ejercicio 4: Repartir caramelos

Nivel: ★☆☆

Se reparten 17 caramelos entre 5 niños. ¿Cuántos recibe cada uno? ¿Cuántos sobran?

Salida esperada:

```text
Cada uno: 3
Sobran: 2
```

> **Pista:** entre dos enteros, `/` da el cociente sin decimales.

> **Pista:** `%` da el resto de la división.

> **Solución**
>
> ```ruby
> caramelos = 17
> ninos = 5
> puts "Cada uno: #{caramelos / ninos}"
> puts "Sobran: #{caramelos % ninos}"
> ```
>
> ```sortie
> Cada uno: 3
> Sobran: 2
> ```
>
> Por qué funciona: 5 × 3 = 15, y 17 − 15 = 2. Aquí, la división entera es justo lo
> que quieres (mira [las trampas](pieges.md)).

## Condiciones y bucles

### Ejercicio 5: Positivo, negativo o cero

Nivel: ★☆☆

Para cada número de la lista `[4, -2, 0]`, muestra si es positivo, negativo o cero.

Salida esperada:

```text
4 es positivo
-2 es negativo
0 es cero
```

> **Pista:** empieza con un solo número, `n = 4`, con `if`, `elsif` y `else`.

> **Pista:** después, rodea tu `if` con un bucle `[4, -2, 0].each do |n| ... end`.

> **Solución**
>
> ```ruby
> [4, -2, 0].each do |n|
>   if n > 0
>     puts "#{n} es positivo"
>   elsif n < 0
>     puts "#{n} es negativo"
>   else
>     puts "#{n} es cero"
>   end
> end
> ```
>
> ```sortie
> 4 es positivo
> -2 es negativo
> 0 es cero
> ```
>
> Por qué funciona: Ruby comprueba las condiciones en orden y se detiene en la primera
> verdadera. Si ninguna es verdadera, toma el `else`.

### Ejercicio 6: Contar hasta 5

Nivel: ★☆☆

Muestra los números del 1 al 5, uno por línea.

Salida esperada:

```text
1
2
3
4
5
```

> **Pista:** `(1..5)` describe los números del 1 al 5, con el 5 incluido.

> **Solución**
>
> ```ruby
> (1..5).each do |n|
>   puts n
> end
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> ```
>
> Por qué funciona: `each` da cada valor a `n`, uno por vuelta. `1.upto(5)` también
> funciona.

### Ejercicio 7: La cuenta atrás

Nivel: ★☆☆

Muestra una cuenta atrás de 3 a 1 y después `¡Despegue!`.

Salida esperada:

```text
3...
2...
1...
¡Despegue!
```

> **Pista:** ¡`(3..1)` está vacío! Usa `3.downto(1)`.

> **Solución**
>
> ```ruby
> 3.downto(1) do |n|
>   puts "#{n}..."
> end
> puts "¡Despegue!"
> ```
>
> ```sortie
> 3...
> 2...
> 1...
> ¡Despegue!
> ```
>
> Por qué funciona: `downto` baja de 3 a 1. El último `puts` está después de `end`:
> solo se ejecuta una vez, al final.

### Ejercicio 8: La suma del 1 al 10

Nivel: ★☆☆

Suma los números del 1 al 10 con un bucle y después muestra el total.

Salida esperada:

```text
Total: 55
```

> **Pista:** crea `total = 0` **antes** del bucle.

> **Pista:** dentro del bucle, añade el número de la vuelta: `total += n`.

> **Solución**
>
> ```ruby
> total = 0
> (1..10).each do |n|
>   total += n
> end
> puts "Total: #{total}"
> ```
>
> ```sortie
> Total: 55
> ```
>
> Por qué funciona: `total` crece en cada vuelta.
>
> | Paso | n | total |
> |---|---|---|
> | antes | – | 0 |
> | vuelta 1 | 1 | 1 |
> | vuelta 2 | 2 | 3 |
> | vuelta 3 | 3 | 6 |
> | … | … | … |
> | vuelta 10 | 10 | 55 |
>
> En Ruby, `(1..10).sum` hace lo mismo de una vez.

## Métodos

### Ejercicio 9: El cuadrado

Nivel: ★☆☆

Escribe un método `cuadrado` que reciba un número y devuelva ese número multiplicado por
sí mismo. Muestra `cuadrado(4)` y `cuadrado(9)`.

Salida esperada:

```text
16
81
```

> **Pista:** un método empieza con `def cuadrado(n)` y termina con `end`.

> **Pista:** la última línea del método es el valor devuelto: `n * n`.

> **Solución**
>
> ```ruby
> def cuadrado(n)
>   n * n
> end
>
> puts cuadrado(4)
> puts cuadrado(9)
> ```
>
> ```sortie
> 16
> 81
> ```
>
> Por qué funciona: cuando llamas a `cuadrado(4)`, `n` vale 4 dentro del método.
> Devuelve `4 * 4`, que `puts` muestra. Mira [los métodos](fonctions.md).

### Ejercicio 10: El mayor de los dos

Nivel: ★☆☆

Escribe un método `mayor(a, b)` que devuelva el mayor de los dos números, con un `if`.

Salida esperada:

```text
8
12
```

para `mayor(8, 3)` y luego `mayor(5, 12)`.

> **Pista:** si `a > b`, la respuesta es `a`. Si no, es `b`.

> **Solución**
>
> ```ruby
> def mayor(a, b)
>   if a > b
>     a
>   else
>     b
>   end
> end
>
> puts mayor(8, 3)
> puts mayor(5, 12)
> ```
>
> ```sortie
> 8
> 12
> ```
>
> Por qué funciona: un `if` devuelve el valor de la rama elegida, y es lo último del
> método. Ruby también tiene `[a, b].max`.

## Textos

### Ejercicio 11: Gritar

Nivel: ★☆☆

Escribe un método `gritar(texto)` que devuelva el texto en mayúsculas, entre `¡` y `!`.

Salida esperada:

```text
¡HOLA!
¡CUIDADO!
```

> **Pista:** `"hola".upcase` da `"HOLA"`.

> **Solución**
>
> ```ruby
> def gritar(texto)
>   "¡#{texto.upcase}!"
> end
>
> puts gritar("hola")
> puts gritar("cuidado")
> ```
>
> ```sortie
> ¡HOLA!
> ¡CUIDADO!
> ```
>
> Por qué funciona: `upcase` devuelve un texto nuevo en mayúsculas. Lo colocas dentro
> de un texto que añade `¡` al principio y `!` al final.

### Ejercicio 12: Las iniciales

Nivel: ★☆☆

A partir de `"Marie Curie"`, muestra las iniciales y el número de letras del nombre.

Salida esperada:

```text
Iniciales: M.C.
El nombre tiene 5 letras
```

> **Pista:** `"Marie Curie".split` da `["Marie", "Curie"]`.

> **Pista:** `palabra[0]` da la primera letra, `palabra.length` el número de caracteres.

> **Solución**
>
> ```ruby
> nombre_completo = "Marie Curie"
> nombre, apellido = nombre_completo.split
> puts "Iniciales: #{nombre[0]}.#{apellido[0]}."
> puts "El nombre tiene #{nombre.length} letras"
> ```
>
> ```sortie
> Iniciales: M.C.
> El nombre tiene 5 letras
> ```
>
> Por qué funciona: `split` corta en los espacios. `nombre, apellido = ...` guarda el
> primer trozo en `nombre` y el segundo en `apellido`. Mira [los textos](texte.md).

## Arrays, hashes y rangos

### Ejercicio 13: La lista de la compra

Nivel: ★☆☆

Guarda `pan`, `leche` y `manzanas` en un array. Muéstralos numerados desde 1 y después
el número de artículos.

Salida esperada:

```text
1. pan
2. leche
3. manzanas
3 artículos
```

> **Pista:** `each_with_index` da el elemento **y** su posición (que empieza en 0).

> **Pista:** muestra `i + 1` para empezar en 1.

> **Solución**
>
> ```ruby
> compra = ["pan", "leche", "manzanas"]
> compra.each_with_index do |articulo, i|
>   puts "#{i + 1}. #{articulo}"
> end
> puts "#{compra.size} artículos"
> ```
>
> ```sortie
> 1. pan
> 2. leche
> 3. manzanas
> 3 artículos
> ```
>
> Por qué funciona: las posiciones van de 0 a 2. Sumando 1, muestras de 1 a 3, como en
> una lista de verdad.

### Ejercicio 14: La media de las notas

Nivel: ★☆☆

Con las notas `[12, 15, 9, 16]`, muestra la más alta, la más baja y la media.

Salida esperada:

```text
Más alta: 16
Más baja: 9
Media: 13.0
```

> **Pista:** un array conoce `max`, `min`, `sum` y `size`.

> **Pista:** `sum / size` haría una división entera. Usa `sum.fdiv(size)`.

> **Solución**
>
> ```ruby
> notas = [12, 15, 9, 16]
> puts "Más alta: #{notas.max}"
> puts "Más baja: #{notas.min}"
> puts "Media: #{notas.sum.fdiv(notas.size)}"
> ```
>
> ```sortie
> Más alta: 16
> Más baja: 9
> Media: 13.0
> ```
>
> Por qué funciona: 12 + 15 + 9 + 16 = 52, y 52 ÷ 4 = 13. `fdiv` conserva los
> decimales, incluso cuando el resultado es exacto.

### Ejercicio 15: El precio de la fruta

Nivel: ★☆☆

Guarda los precios en un hash: manzanas 2, kiwis 3, peras 4. Muestra cada precio y
después la etiqueta de una cesta de 9 euros: `pequeña` hasta 5, `mediana` de 6 a 10,
`grande` por encima.

Salida esperada:

```text
manzanas: 2 €
kiwis: 3 €
peras: 4 €
Cesta de 9 €: mediana
```

> **Pista:** `precios.each do |fruta, importe| ... end` da cada clave y su valor.

> **Pista:** para la etiqueta, un `case` con rangos: `when 0..5`, `when 6..10`.

> **Solución**
>
> ```ruby
> precios = { "manzanas" => 2, "kiwis" => 3, "peras" => 4 }
> precios.each do |fruta, importe|
>   puts "#{fruta}: #{importe} €"
> end
>
> cesta = 9
> etiqueta = case cesta
>            when 0..5 then "pequeña"
>            when 6..10 then "mediana"
>            else "grande"
>            end
> puts "Cesta de #{cesta} €: #{etiqueta}"
> ```
>
> ```sortie
> manzanas: 2 €
> kiwis: 3 €
> peras: 4 €
> Cesta de 9 €: mediana
> ```
>
> Por qué funciona: un hash conserva el orden en el que guardaste las claves. `9` está
> en `6..10`, así que `case` elige `"mediana"`. Mira [los rangos](intervalles.md).

## ¿Y ahora qué?

¡Enhorabuena si has llegado hasta aquí! Otros ejercicios, más variados, te esperan en
[los ejercicios resueltos](exercices.md). Pero primero, descubre los bloques: están por
todas partes en Ruby.

Siguiente: [bloques, iteradores y Enumerable](blocs.md)
