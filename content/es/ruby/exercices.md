# Ejercicios resueltos

Esta página propone 12 ejercicios resueltos, del más fácil al más difícil, para practicar todo el recorrido de Ruby.

## Cómo usar esta página

Cada ejercicio tiene un nivel:

- ★☆☆: fácil;
- ★★☆: medio;
- ★★★: más difícil.

Lee el enunciado, luego escribe tu solución en un archivo `.rb` y ejecútalo. Abre la
solución solo después de haberlo intentado.

> **Consejo:** a menudo hay varias soluciones buenas. Si tu programa muestra el
> resultado correcto, ¡lo has conseguido!

## Lo básico

### Ejercicio 1: Preséntate

Nivel: ★☆☆

Crea dos variables, `nombre` y `edad`. Después muestra la frase
`Me llamo Alice y tengo 30 años.`, con tus variables.

> **Pista:** usa la interpolación `#{...}` (mira [los primeros pasos](premiers-pas.md)).

> **Solución**
>
> ```ruby
> nombre = "Alice"
> edad = 30
> puts "Me llamo #{nombre} y tengo #{edad} años."
> ```
>
> ```sortie
> Me llamo Alice y tengo 30 años.
> ```

### Ejercicio 2: Par o impar

Nivel: ★☆☆

Para el número `7`, muestra `7 es impar`. Para `10`, el programa debe mostrar
`10 es par`.

> **Pista:** `n.even?` vale `true` si `n` es par.

> **Solución**
>
> ```ruby
> [7, 10].each do |n|
>   if n.even?
>     puts "#{n} es par"
>   else
>     puts "#{n} es impar"
>   end
> end
> ```
>
> ```sortie
> 7 es impar
> 10 es par
> ```

### Ejercicio 3: La tabla del 7

Nivel: ★☆☆

Muestra la tabla de multiplicar del 7, de `7 x 1 = 7` a `7 x 5 = 35`.

> **Pista:** un bucle `(1..5).each` da 5 vueltas.

> **Solución**
>
> ```ruby
> (1..5).each do |i|
>   puts "7 x #{i} = #{7 * i}"
> end
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

### Ejercicio 4: El área de un rectángulo

Nivel: ★☆☆

Escribe un método `area(ancho, alto)` que devuelva el área de un rectángulo.
Muestra el área de un rectángulo de 4 por 3.

> **Solución**
>
> ```ruby
> def area(ancho, alto)
>   ancho * alto
> end
>
> puts area(4, 3)
> ```
>
> ```sortie
> 12
> ```

## Texto y colecciones

### Ejercicio 5: La media

Nivel: ★★☆

Con el array `notas = [12, 15, 9, 14]`, calcula y muestra la media.

> **Pista:** `sum` da el total, `length` el número de notas. Acuérdate de dividir con
> un número decimal (`to_f`).

> **Solución**
>
> ```ruby
> notas = [12, 15, 9, 14]
> media = notas.sum.to_f / notas.length
> puts "Media: #{media}"
> ```
>
> ```sortie
> Media: 12.5
> ```

### Ejercicio 6: Contar las vocales

Nivel: ★★☆

Escribe un método `vocales(texto)` que cuente las vocales (a, e, i, o, u, y) de un
texto. Pruébalo con `"Hola Ruby"`.

> **Pista:** pasa el texto a minúsculas y luego recorre `chars`.

> **Solución**
>
> ```ruby
> def vocales(texto)
>   cuenta = 0
>   texto.downcase.chars.each do |letra|
>     cuenta += 1 if "aeiouy".include?(letra)
>   end
>   cuenta
> end
>
> puts vocales("Hola Ruby")
> ```
>
> ```sortie
> 4
> ```

### Ejercicio 7: El palíndromo

Nivel: ★★☆

Un palíndromo se lee igual en los dos sentidos, como «kayak». Escribe un método
`palindromo?(palabra)` que responda `true` o `false`. Ignora las mayúsculas.

> **Pista:** compara la palabra con `palabra.reverse` (mira [el texto](texte.md)).

> **Solución**
>
> ```ruby
> def palindromo?(palabra)
>   palabra = palabra.downcase
>   palabra == palabra.reverse
> end
>
> puts palindromo?("Kayak")
> puts palindromo?("Ruby")
> ```
>
> ```sortie
> true
> false
> ```

### Ejercicio 8: Contar las palabras

Nivel: ★★☆

Con la frase `"el gato y el perro y el conejo"`, cuenta cuántas veces aparece cada
palabra. Muestra una línea por palabra.

> **Pista:** usa un hash creado con `Hash.new(0)`: así una clave ausente vale `0`.

> **Solución**
>
> ```ruby
> frase = "el gato y el perro y el conejo"
> cuenta = Hash.new(0)
> frase.split(" ").each do |palabra|
>   cuenta[palabra] += 1
> end
> cuenta.each do |palabra, numero|
>   puts "#{palabra}: #{numero}"
> end
> ```
>
> ```sortie
> el: 3
> gato: 1
> y: 2
> perro: 1
> conejo: 1
> ```

### Ejercicio 9: Quedarse con los pares y duplicarlos

Nivel: ★★☆

Con los números del 1 al 10, quédate solo con los números pares y luego duplícalos.
Muestra el array obtenido.

> **Pista:** `select` filtra, `map` transforma (mira [los bloques](blocs.md)).

> **Solución**
>
> ```ruby
> resultado = (1..10).select { |n| n.even? }.map { |n| n * 2 }
> p resultado
> ```
>
> ```sortie
> [4, 8, 12, 16, 20]
> ```

## Para ir más lejos

### Ejercicio 10: FizzBuzz

Nivel: ★★☆

Muestra los números del 1 al 15. Pero:

- en lugar de los múltiplos de 3, muestra `Fizz`;
- en lugar de los múltiplos de 5, muestra `Buzz`;
- en lugar de los múltiplos de 3 **y** de 5, muestra `FizzBuzz`.

> **Pista:** `n % 3 == 0` es verdadero si `n` es múltiplo de 3. Prueba primero el caso
> «3 y 5».

> **Solución**
>
> ```ruby
> (1..15).each do |n|
>   if n % 3 == 0 && n % 5 == 0
>     puts "FizzBuzz"
>   elsif n % 3 == 0
>     puts "Fizz"
>   elsif n % 5 == 0
>     puts "Buzz"
>   else
>     puts n
>   end
> end
> ```
>
> ```sortie
> 1
> 2
> Fizz
> 4
> Buzz
> Fizz
> 7
> 8
> Fizz
> Buzz
> 11
> Fizz
> 13
> 14
> FizzBuzz
> ```

### Ejercicio 11: La cuenta bancaria

Nivel: ★★★

Crea una clase `Cuenta` con un saldo que empieza en `0`. Añade:

- un método `depositar(cantidad)` que añade dinero;
- un método `retirar(cantidad)` que saca dinero, solo si el saldo alcanza;
- una lectura del saldo con `attr_reader`.

> **Pista:** guarda el saldo en `@saldo`, dentro de `initialize` (mira
> [las clases](types.md)).

> **Solución**
>
> ```ruby
> class Cuenta
>   attr_reader :saldo
>
>   def initialize
>     @saldo = 0
>   end
>
>   def depositar(cantidad)
>     @saldo += cantidad
>   end
>
>   def retirar(cantidad)
>     if cantidad > @saldo
>       puts "Saldo insuficiente"
>     else
>       @saldo -= cantidad
>     end
>   end
> end
>
> cuenta = Cuenta.new
> cuenta.depositar(100)
> cuenta.retirar(30)
> cuenta.retirar(500)
> puts "Saldo: #{cuenta.saldo}"
> ```
>
> ```sortie
> Saldo insuficiente
> Saldo: 70
> ```

### Ejercicio 12: Una edad válida

Nivel: ★★★

Escribe un método `leer_edad(texto)` que convierta un texto en una edad:

- si el texto no es un número, devuelve `nil`;
- si la edad es negativa, lanza un error `ArgumentError`.

Pruébalo con `"25"`, `"abc"` y `"-3"`, sin que el programa se detenga.

> **Pista:** `Integer("abc")` lanza un `ArgumentError`. Usa `rescue` (mira
> [gestionar los errores](erreurs.md)).

> **Solución**
>
> ```ruby
> def leer_edad(texto)
>   begin
>     edad = Integer(texto)
>   rescue ArgumentError
>     return nil
>   end
>   raise ArgumentError, "edad negativa" if edad < 0
>   edad
> end
>
> ["25", "abc", "-3"].each do |texto|
>   begin
>     edad = leer_edad(texto)
>     if edad.nil?
>       puts "#{texto}: no es un número"
>     else
>       puts "#{texto}: #{edad} años"
>     end
>   rescue ArgumentError => e
>     puts "#{texto}: error, #{e.message}"
>   end
> end
> ```
>
> ```sortie
> 25: 25 años
> abc: no es un número
> -3: error, edad negativa
> ```

## En resumen

- Empieza por los ejercicios ★☆☆ y luego sube de dificultad.
- Inténtalo siempre solo antes de abrir la solución.
- Los ejercicios 1 a 4 repasan lo básico: variables, condiciones, bucles, métodos.
- Los ejercicios 5 a 9 trabajan el texto, los arrays, los hashes y los bloques.
- Los ejercicios 10 a 12 lo combinan todo, con las clases y los errores.

Siguiente: [ejercicios resueltos: segunda serie](exercices-2.md)
