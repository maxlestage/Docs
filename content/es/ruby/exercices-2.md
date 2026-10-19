# Ejercicios resueltos: segunda serie

Esta página propone 12 nuevos ejercicios resueltos en Ruby, del más fácil al más difícil, con tablas que siguen las variables paso a paso.

## Cómo usar esta página

Estos ejercicios son **distintos** de los de los
[ejercicios para empezar bien](exercices-faciles.md) y de los
[ejercicios resueltos](exercices.md). Cada ejercicio tiene un nivel:

- ★☆☆: fácil;
- ★★☆: medio;
- ★★★: más difícil.

Para cada ejercicio:

1. Lee el enunciado y la salida esperada.
2. Busca la solución en papel, con una tabla de seguimiento (mira
   [seguir un programa paso a paso](pas-a-pas.md)).
3. Escribe tu programa en un archivo `.rb` y ejecútalo.
4. ¿Atascado? Lee la pista. Abre la solución al final.

> **Consejo:** si tu programa muestra un resultado incorrecto, añade un `puts` con
> etiquetas dentro del bucle. Verás en qué vuelta las cosas se tuercen.

## Calentamiento

### Ejercicio 1: De Celsius a Fahrenheit

Nivel: ★☆☆

Para cada temperatura del array `[0, 20, 37, 100]` (en grados Celsius), muestra su
valor en grados Fahrenheit. La fórmula: `f = c * 9 / 5 + 32`.

Salida esperada:

```text
0 °C = 32.0 °F
20 °C = 68.0 °F
37 °C = 98.6 °F
100 °C = 212.0 °F
```

> **Pista:** `37 * 9 / 5` da un número entero (66), sin decimales. Escribe `9.0` en
> lugar de `9` para obtener un número decimal.

> **Solución**
>
> ```ruby
> [0, 20, 37, 100].each do |c|
>   f = c * 9.0 / 5 + 32
>   puts "#{c} °C = #{f} °F"
> end
> ```
>
> ```sortie
> 0 °C = 32.0 °F
> 20 °C = 68.0 °F
> 37 °C = 98.6 °F
> 100 °C = 212.0 °F
> ```
>
> Por qué funciona: en cuanto uno de los números es decimal (`9.0`), todo el cálculo se
> hace con decimales. Con `9`, Ruby haría una división entera y mostraría `98` para
> 37 °C (mira [los números](nombres.md)).

### Ejercicio 2: El más pequeño, a mano

Nivel: ★☆☆

Encuentra el número más pequeño del array `[8, 3, 11, 5]` **sin** usar `min`.
Usa un bucle.

Salida esperada:

```text
El más pequeño: 3
```

> **Pista:** guarda el «récord» en una variable `menor`. Empieza con el primer
> elemento, `numeros[0]`.

> **Pista:** en cada vuelta, si el número es más pequeño que el récord, se convierte en
> el nuevo récord.

> **Solución**
>
> ```ruby
> numeros = [8, 3, 11, 5]
> menor = numeros[0]
> numeros.each do |n|
>   if n < menor
>     menor = n
>   end
> end
> puts "El más pequeño: #{menor}"
> ```
>
> ```sortie
> El más pequeño: 3
> ```
>
> Por qué funciona: como un campeón que conserva su copa mientras nadie lo haga mejor.
>
> | Vuelta | n | ¿`n < menor`? | menor |
> |---|---|---|---|
> | antes | – | | 8 |
> | 1 | 8 | `8 < 8` no | 8 |
> | 2 | 3 | `3 < 8` sí | 3 |
> | 3 | 11 | `11 < 3` no | 3 |
> | 4 | 5 | `5 < 3` no | 3 |
>
> Partimos de `numeros[0]` y no de `0`: con `0`, ningún número positivo batiría el
> récord, y el programa mostraría 0.

### Ejercicio 3: La escalera

Nivel: ★☆☆

Muestra una escalera de 4 peldaños con estrellas.

Salida esperada:

```text
*
**
***
****
```

> **Pista:** en Ruby, `"*" * 3` da `"***"`: se puede multiplicar un texto por un
> número.

> **Solución**
>
> ```ruby
> (1..4).each do |peldano|
>   puts "*" * peldano
> end
> ```
>
> ```sortie
> *
> **
> ***
> ****
> ```
>
> Por qué funciona: en cada vuelta, `peldano` crece en 1, así que la línea tiene una
> estrella más.
>
> | Vuelta | peldano | `"*" * peldano` |
> |---|---|---|
> | 1 | 1 | `"*"` |
> | 2 | 2 | `"**"` |
> | 3 | 3 | `"***"` |
> | 4 | 4 | `"****"` |

### Ejercicio 4: De segundos a horas

Nivel: ★☆☆

Una película dura `3725` segundos. Muestra su duración en horas, minutos y segundos.

Salida esperada:

```text
3725 s = 1 h 2 min 5 s
```

> **Pista:** una hora tiene 3600 segundos. `/` da el número de horas enteras, `%` da
> lo que sobra.

> **Pista:** haz lo mismo con el resto y 60 para los minutos.

> **Solución**
>
> ```ruby
> total = 3725
> horas = total / 3600
> resto = total % 3600
> minutos = resto / 60
> segundos = resto % 60
> puts "#{total} s = #{horas} h #{minutos} min #{segundos} s"
> ```
>
> ```sortie
> 3725 s = 1 h 2 min 5 s
> ```
>
> Por qué funciona: es como dar el cambio empezando por los billetes grandes.
>
> | Línea | Cálculo | horas | resto | minutos | segundos |
> |---|---|---|---|---|---|
> | 2 | `3725 / 3600` | 1 | – | – | – |
> | 3 | `3725 % 3600` | 1 | 125 | – | – |
> | 4 | `125 / 60` | 1 | 125 | 2 | – |
> | 5 | `125 % 60` | 1 | 125 | 2 | 5 |

## Textos y colecciones

### Ejercicio 5: Dar la vuelta a una palabra a mano

Nivel: ★★☆

Escribe un método `invertir(palabra)` que devuelva la palabra escrita al revés,
**sin** usar `reverse`. Pruébalo con `"Ruby"`.

Salida esperada:

```text
ybuR
```

> **Pista:** parte de un texto vacío. Recorre las letras con `palabra.chars.each`.
> Añade cada letra **delante** del resultado, no detrás.

> **Solución**
>
> ```ruby
> def invertir(palabra)
>   resultado = ""
>   palabra.chars.each do |letra|
>     resultado = letra + resultado
>   end
>   resultado
> end
>
> puts invertir("Ruby")
> ```
>
> ```sortie
> ybuR
> ```
>
> Por qué funciona: como una pila de platos, la última letra que pones queda arriba, y
> por tanto la primera.
>
> | Vuelta | letra | `letra + resultado` | resultado |
> |---|---|---|---|
> | antes | – | | `""` |
> | 1 | `"R"` | `"R" + ""` | `"R"` |
> | 2 | `"u"` | `"u" + "R"` | `"uR"` |
> | 3 | `"b"` | `"b" + "uR"` | `"buR"` |
> | 4 | `"y"` | `"y" + "buR"` | `"ybuR"` |

### Ejercicio 6: La palabra más larga

Nivel: ★★☆

En la frase `"el gatito duerme en el sofá"`, encuentra la palabra más larga y muéstrala
con su longitud.

Salida esperada:

```text
gatito (6 letras)
```

> **Pista:** `split` corta la frase en palabras. `max_by { |palabra| palabra.length }`
> se queda con el elemento para el que el bloque da el número más grande.

> **Solución**
>
> ```ruby
> frase = "el gatito duerme en el sofá"
> larga = frase.split.max_by { |palabra| palabra.length }
> puts "#{larga} (#{larga.length} letras)"
> ```
>
> ```sortie
> gatito (6 letras)
> ```
>
> Por qué funciona: `max_by` mide cada palabra con el bloque. `gatito` y `duerme`
> tienen las dos 6 letras: en caso de empate, `max_by` se queda con la **primera**.
> Mira [los bloques](blocs.md).

### Ejercicio 7: Ordenar por inicial

Nivel: ★★☆

Agrupa los nombres `["Alice", "Bob", "Anna", "Chloé", "Bruno"]` según su primera
letra. Muestra una línea por letra.

Salida esperada:

```text
A: Alice, Anna
B: Bob, Bruno
C: Chloé
```

> **Pista:** `nombre[0]` da la primera letra. Un hash puede asociar a cada letra un
> array de nombres.

> **Pista:** `group_by` hace este trabajo de una vez: agrupa los elementos según el
> valor que devuelve el bloque.

> **Solución**
>
> ```ruby
> nombres = ["Alice", "Bob", "Anna", "Chloé", "Bruno"]
> grupos = nombres.group_by { |nombre| nombre[0] }
> grupos.each do |letra, lista|
>   puts "#{letra}: #{lista.join(", ")}"
> end
> ```
>
> ```sortie
> A: Alice, Anna
> B: Bob, Bruno
> C: Chloé
> ```
>
> Por qué funciona: `group_by` hace cajones con etiqueta. Aquí construye el hash
> `{"A" => ["Alice", "Anna"], "B" => ["Bob", "Bruno"], "C" => ["Chloé"]}`. `join`
> vuelve a pegar los nombres con una coma.

### Ejercicio 8: La suma de las cifras

Nivel: ★★☆

Escribe un método `suma_cifras(n)` que sume las cifras de un número, sin convertirlo en
texto. Pruébalo con `2026`.

Salida esperada:

```text
10
```

> **Pista:** `n % 10` da la última cifra (`2026 % 10` vale 6). `n / 10` quita esa
> última cifra (`2026 / 10` vale 202).

> **Pista:** repite con `while n > 0`.

> **Solución**
>
> ```ruby
> def suma_cifras(n)
>   suma = 0
>   while n > 0
>     suma += n % 10
>     n = n / 10
>   end
>   suma
> end
>
> puts suma_cifras(2026)
> ```
>
> ```sortie
> 10
> ```
>
> Por qué funciona: nos «comemos» el número por la derecha, una cifra por vuelta.
>
> | Vuelta | Prueba `n > 0` | `n % 10` | suma | n |
> |---|---|---|---|---|
> | antes | | | 0 | 2026 |
> | 1 | sí | 6 | 6 | 202 |
> | 2 | sí | 2 | 8 | 20 |
> | 3 | sí | 0 | 8 | 2 |
> | 4 | sí | 2 | 10 | 0 |
> | fin | `0 > 0` no | | 10 | 0 |

### Ejercicio 9: Comprobar una contraseña

Nivel: ★★☆

Escribe un método `problemas(clave)` que devuelva la lista de problemas de una
contraseña:

- menos de 8 caracteres: `"demasiado corta"`;
- ninguna cifra: `"sin cifra"`;
- ninguna mayúscula: `"sin mayúscula"`.

Pruébalo con `"gato"`, `"gatito2026"` y `"Gatito2026"`.

Salida esperada:

```text
gato: demasiado corta, sin cifra, sin mayúscula
gatito2026: sin mayúscula
Gatito2026: OK
```

> **Pista:** parte de un array vacío `lista = []` y añade un mensaje con `<<` por cada
> regla que no se cumpla.

> **Pista:** `clave.chars.any? { |c| ("0".."9").include?(c) }` responde `true` si hay
> al menos una cifra. Para una mayúscula, usa `("A".."Z")`.

> **Solución**
>
> ```ruby
> def problemas(clave)
>   lista = []
>   lista << "demasiado corta" if clave.length < 8
>   lista << "sin cifra" unless clave.chars.any? { |c| ("0".."9").include?(c) }
>   lista << "sin mayúscula" unless clave.chars.any? { |c| ("A".."Z").include?(c) }
>   lista
> end
>
> ["gato", "gatito2026", "Gatito2026"].each do |clave|
>   lista = problemas(clave)
>   if lista.empty?
>     puts "#{clave}: OK"
>   else
>     puts "#{clave}: #{lista.join(", ")}"
>   end
> end
> ```
>
> ```sortie
> gato: demasiado corta, sin cifra, sin mayúscula
> gatito2026: sin mayúscula
> Gatito2026: OK
> ```
>
> Por qué funciona: cada regla es una línea independiente, como una lista de
> comprobación antes de salir de viaje. Una regla que no se cumple añade su mensaje. Un
> array vacío al final quiere decir «todo va bien».

## Para ir más lejos

### Ejercicio 10: Adivina el número

Nivel: ★★★

El ordenador debe encontrar un número secreto entre 1 y 100. En cada intento, solo
sabe «demasiado bajo» o «demasiado alto». Escribe un programa que encuentre `37`
proponiendo siempre el **centro** del intervalo que todavía es posible.

Salida esperada:

```text
Intento 1: 50, demasiado alto
Intento 2: 25, demasiado bajo
Intento 3: 37, ¡encontrado!
```

> **Pista:** guarda dos variables, `bajo = 1` y `alto = 100`. El centro es
> `(bajo + alto) / 2`.

> **Pista:** si el centro es demasiado alto, el secreto está por debajo:
> `alto = centro - 1`. Si es demasiado bajo: `bajo = centro + 1`.

> **Solución**
>
> ```ruby
> secreto = 37
> bajo = 1
> alto = 100
> intento = 0
> loop do
>   intento += 1
>   centro = (bajo + alto) / 2
>   if centro == secreto
>     puts "Intento #{intento}: #{centro}, ¡encontrado!"
>     break
>   elsif centro > secreto
>     puts "Intento #{intento}: #{centro}, demasiado alto"
>     alto = centro - 1
>   else
>     puts "Intento #{intento}: #{centro}, demasiado bajo"
>     bajo = centro + 1
>   end
> end
> ```
>
> ```sortie
> Intento 1: 50, demasiado alto
> Intento 2: 25, demasiado bajo
> Intento 3: 37, ¡encontrado!
> ```
>
> Por qué funciona: en cada intento, tiramos la mitad de los números posibles, como
> cuando buscas una palabra en un diccionario abriéndolo por la mitad.
>
> | Intento | bajo | alto | centro | Respuesta | Nuevo límite |
> |---|---|---|---|---|---|
> | 1 | 1 | 100 | 50 | demasiado alto | alto = 49 |
> | 2 | 1 | 49 | 25 | demasiado bajo | bajo = 26 |
> | 3 | 26 | 49 | 37 | encontrado | |
>
> Con este método, 7 intentos bastan siempre para un número entre 1 y 100.

### Ejercicio 11: Los paréntesis bien cerrados

Nivel: ★★★

Escribe un método `equilibrado?(texto)` que compruebe que los paréntesis `()`,
corchetes `[]` y llaves `{}` están bien cerrados, en el orden correcto. Pruébalo con
`"(a[b]{c})"`, `"(a]"` y `"(("`.

Salida esperada:

```text
(a[b]{c}) : true
(a] : false
(( : false
```

> **Pista:** usa un array como una pila de platos. Un signo de apertura: lo pones
> encima con `push`. Un signo de cierre: quitas el de arriba con `pop`, y tiene que
> corresponder.

> **Pista:** un hash `{ ")" => "(", "]" => "[", "}" => "{" }` da el signo de apertura
> esperado para cada signo de cierre. Al final, la pila tiene que estar vacía.

> **Solución**
>
> ```ruby
> PARES = { ")" => "(", "]" => "[", "}" => "{" }
>
> def equilibrado?(texto)
>   pila = []
>   texto.chars.each do |c|
>     if PARES.value?(c)
>       pila.push(c)
>     elsif PARES.key?(c)
>       return false if pila.pop != PARES[c]
>     end
>   end
>   pila.empty?
> end
>
> ["(a[b]{c})", "(a]", "(("].each do |texto|
>   puts "#{texto} : #{equilibrado?(texto)}"
> end
> ```
>
> ```sortie
> (a[b]{c}) : true
> (a] : false
> (( : false
> ```
>
> Por qué funciona: el último signo abierto tiene que ser el primero en cerrarse. Las
> letras se ignoran. Este es el seguimiento para `"(a[b]{c})"`:
>
> | c | Acción | pila después |
> |---|---|---|
> | `(` | se pone | `["("]` |
> | `a` | nada | `["("]` |
> | `[` | se pone | `["(", "["]` |
> | `b` | nada | `["(", "["]` |
> | `]` | se quita `[`, corresponde | `["("]` |
> | `{` | se pone | `["(", "{"]` |
> | `c` | nada | `["(", "{"]` |
> | `}` | se quita `{`, corresponde | `["("]` |
> | `)` | se quita `(`, corresponde | `[]` |
>
> Para `"(a]"`, el `]` quita `(` en lugar de `[`: `false`. Para `"(("`, la pila no está
> vacía al final: `false`.

### Ejercicio 12: Comprimir un texto

Nivel: ★★★

Escribe un método `comprimir(texto)` que sustituya cada serie de letras iguales por la
letra seguida del número de repeticiones. Pruébalo con `"aaabccdddd"`.

Salida esperada:

```text
a3b1c2d4
```

> **Pista:** guarda la letra actual (`actual`) y su contador (`cuenta`). Cuando la
> letra cambia, añade `actual` y `cuenta` al resultado, y vuelve a empezar con la nueva
> letra.

> **Pista:** no olvides añadir la **última** serie después del bucle.

> **Solución**
>
> ```ruby
> def comprimir(texto)
>   resultado = ""
>   actual = texto[0]
>   cuenta = 0
>   texto.chars.each do |letra|
>     if letra == actual
>       cuenta += 1
>     else
>       resultado += "#{actual}#{cuenta}"
>       actual = letra
>       cuenta = 1
>     end
>   end
>   resultado + "#{actual}#{cuenta}"
> end
>
> puts comprimir("aaabccdddd")
> ```
>
> ```sortie
> a3b1c2d4
> ```
>
> Por qué funciona: es como contar coches del mismo color que pasan en fila. Cuando el
> color cambia, apuntas el total y vuelves a empezar a contar.
>
> | Vuelta | letra | ¿Misma letra? | resultado | actual | cuenta |
> |---|---|---|---|---|---|
> | antes | – | | `""` | `"a"` | 0 |
> | 1 | a | sí | `""` | `"a"` | 1 |
> | 2 | a | sí | `""` | `"a"` | 2 |
> | 3 | a | sí | `""` | `"a"` | 3 |
> | 4 | b | no | `"a3"` | `"b"` | 1 |
> | 5 | c | no | `"a3b1"` | `"c"` | 1 |
> | 6 | c | sí | `"a3b1"` | `"c"` | 2 |
> | 7 | d | no | `"a3b1c2"` | `"d"` | 1 |
> | 8 a 10 | d | sí | `"a3b1c2"` | `"d"` | 4 |
>
> Después del bucle, la última serie (`d4`) todavía no está en `resultado`: la última
> línea del método la añade.

## En resumen

- Los ejercicios 1 a 4 trabajan los cálculos, los bucles sencillos y el «récord».
- Los ejercicios 5 a 9 trabajan los textos, los arrays, los hashes y los bloques.
- Los ejercicios 10 a 12 piden tener varias variables en la cabeza: haz la tabla de
  seguimiento en papel.
- Cuando una solución te parezca mágica, síguela paso a paso, vuelta por vuelta.

Siguiente: [la chuleta de Ruby](aide-memoire.md)
