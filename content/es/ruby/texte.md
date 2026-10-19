# Trabajar con texto

Esta página muestra cómo crear, unir, cortar y transformar texto en Ruby.

> **Objetivos:**
>
> - sabrás escribir un texto e insertar valores en él;
> - sabrás transformar un texto (mayúsculas, espacios, reemplazos);
> - sabrás buscar, cortar y extraer una parte de un texto;
> - sabrás pasar de un texto a un número, y al revés;
> - sabrás mostrar un número de forma limpia con `format`.

## Crear un texto

Casi todos los programas manejan texto: un nombre que saludar, un mensaje que mostrar,
una respuesta escrita con el teclado.

Un texto se llama **cadena** (`String`), o
[cadena de caracteres](../debuter/glossaire.md#cadena-de-caracteres): una serie de
caracteres (letras, cifras, espacios, signos) ensartados como perlas. Lo escribes
entre comillas dobles `"..."` o simples `'...'`.

```ruby
saludo = "Hola"
nombre = 'Alicia'
puts saludo
puts nombre
```

```sortie
Hola
Alicia
```

> **Consejo:** acostúmbrate a usar comillas dobles. Permiten la interpolación,
> explicada justo después.

### Los caracteres especiales

Entre comillas dobles, la barra `\` anuncia un carácter especial. Se llama
**secuencia de escape**:

- `\n` pasa a la línea siguiente;
- `\t` inserta un tabulador (un espacio grande);
- `\"` escribe una comilla sin cerrar el texto;
- `\\` escribe una barra `\`.

```ruby
puts "Línea 1\nLínea 2"
puts "Nombre\tEdad"
puts "Dijo: \"Hola\""
```

```sortie
Línea 1
Línea 2
Nombre	Edad
Dijo: "Hola"
```

## Unir textos

Un mensaje suele estar hecho de trozos: una parte fija y una parte que cambia. Es como
una carta modelo en la que completas el nombre a mano.

### Con +

El signo `+` pega dos textos uno detrás de otro.

```ruby
nombre = "Alicia"
puts "¡Hola, " + nombre + "!"
```

```sortie
¡Hola, Alicia!
```

`+` solo funciona entre dos textos. Para un número, conviértelo primero con `to_s`:

```ruby
edad = 30
puts "Tengo " + edad.to_s + " años."
```

```sortie
Tengo 30 años.
```

### Con la interpolación

Entre comillas dobles, `#{...}` coloca un valor dentro del texto. Es más sencillo
que `+`, y funciona con los números.

```ruby
nombre = "Alicia"
edad = 30
puts "#{nombre} tiene #{edad} años."
puts "El año que viene: #{edad + 1} años."
```

```sortie
Alicia tiene 30 años.
El año que viene: 31 años.
```

> **Atención:** con comillas simples, `'#{edad}'` se queda tal cual. Ruby no
> reemplaza nada.

### Repetir un texto

`*` repite un texto varias veces.

```ruby
puts "ja" * 3
puts "-" * 10
```

```sortie
jajaja
----------
```

**Te toca:** escribe un programa que guarde `"Leo"` en `nombre` y `8` en `edad`, y
luego muestre «Leo cumplirá 9 años el año que viene.» con la interpolación.

> **Solución**
>
> ```ruby
> nombre = "Leo"
> edad = 8
> puts "#{nombre} cumplirá #{edad + 1} años el año que viene."
> ```
>
> ```sortie
> Leo cumplirá 9 años el año que viene.
> ```

## La longitud de un texto

Conocer la longitud sirve para comprobar lo que alguien ha escrito: una contraseña
demasiado corta, una casilla vacía.

`length` da el número de caracteres. Los espacios también cuentan.

```ruby
puts "gato".length
puts "un gato".length
puts "".empty?
```

```sortie
4
7
true
```

Línea por línea:

- `"gato".length` cuenta g, a, t, o: 4 caracteres;
- `"un gato".length` da 7, porque el espacio entre las dos palabras también cuenta;
- `""` es un texto vacío, sin ningún carácter: por eso `empty?` responde `true`.

`empty?` responde `true` cuando el texto está vacío.

## Mayúsculas y minúsculas

No todo el mundo escribe igual: «alicia», «ALICIA», «Alicia». Estos métodos ponen un
texto en orden, para mostrarlo de forma limpia o para compararlo.

- `upcase` pone todo en mayúsculas.
- `downcase` pone todo en minúsculas.
- `capitalize` pone una mayúscula al principio, y el resto en minúsculas.

```ruby
palabra = "hoLA"
puts palabra.upcase
puts palabra.downcase
puts palabra.capitalize
```

```sortie
HOLA
hola
Hola
```

> **Nota:** estos métodos devuelven un texto **nuevo**. La variable `palabra` no
> cambia.

Para modificar el propio texto, añade un `!`: `palabra.upcase!`. Es la convención
vista en [los métodos](fonctions.md#los-nombres-con-y-con).

```ruby
palabra = "hola"
palabra.upcase
puts palabra
palabra.upcase!
puts palabra
```

```sortie
hola
HOLA
```

## Quitar los espacios de alrededor

Un espacio de más no se ve en la pantalla. Sin embargo, para Ruby, `"Alicia "` y
`"Alicia"` son dos textos distintos.

`strip` quita los espacios (y los saltos de línea) al principio y al final.

```ruby
entrada = "   Alicia   "
puts "[" + entrada + "]"
puts "[" + entrada.strip + "]"
```

```sortie
[   Alicia   ]
[Alicia]
```

Línea por línea:

- `entrada` contiene «Alicia», con tres espacios a cada lado;
- la segunda línea pega corchetes alrededor del texto: muestran dónde empieza y dónde
  termina, espacios incluidos;
- la tercera hace lo mismo con `entrada.strip`: los espacios han desaparecido, los
  corchetes tocan el nombre.

> **Consejo:** usa `strip` con cualquier texto que alguien escriba: a menudo se añaden
> espacios sin querer.

**Te toca:** escribe un programa que guarde `"  alicia  "` en `entrada`, y luego
muestre «¡Hola, Alicia!», sin los espacios y con mayúscula.

> **Solución**
>
> ```ruby
> entrada = "  alicia  "
> nombre = entrada.strip
> puts "¡Hola, #{nombre.capitalize}!"
> ```
>
> ```sortie
> ¡Hola, Alicia!
> ```

## Buscar en un texto

Buscar un trozo de texto sirve en todas partes: encontrar una palabra en un mensaje,
comprobar que una dirección termina en «.es».

- `include?`: ¿contiene el texto este trozo?
- `start_with?`: ¿empieza por este trozo?
- `end_with?`: ¿termina con este trozo?

```ruby
frase = "Me encanta el chocolate"
puts frase.include?("choco")
puts frase.start_with?("Me encanta")
puts frase.end_with?(".")
```

```sortie
true
true
false
```

Línea por línea:

- `frase` contiene el texto que se examina;
- `include?("choco")` busca «choco» en cualquier sitio: está en «chocolate», así que
  `true`;
- `start_with?("Me encanta")` solo mira el principio: `true`;
- `end_with?(".")` solo mira el final: la frase no tiene punto final, así que
  `false`.

> **Atención:** la búsqueda distingue mayúsculas y minúsculas.
> `"Gato".include?("gato")` vale `false`. Usa `downcase` para comparar sin tener en
> cuenta las mayúsculas.

### Comparar, contar y encontrar una posición

- `==` compara dos textos, letra por letra;
- `casecmp?` compara sin tener en cuenta las mayúsculas;
- `count` cuenta caracteres;
- `index` da la posición de un trozo (o `nil` si no está).

```ruby
puts "Gato" == "gato"
puts "Gato".casecmp?("gato")
puts "banana".count("a")
p "banana".index("n")
p "banana".index("z")
```

```sortie
false
true
3
2
nil
```

## Cortar y volver a pegar

Un texto suele reunir varios datos: las palabras de una frase, una lista de la compra
separada por comas. Para tratarlos uno por uno, hay que separarlos.

`split` corta un texto en trozos y devuelve un array. `join` hace lo contrario:
vuelve a pegar un array en un solo texto.

```ruby
frase = "manzana plátano cereza"
frutas = frase.split(" ")
p frutas

lista = "rojo,verde,azul".split(",")
p lista
puts lista.join(" / ")
```

```sortie
["manzana", "plátano", "cereza"]
["rojo", "verde", "azul"]
rojo / verde / azul
```

Línea por línea:

- `frase.split(" ")` corta en cada espacio: tres trozos, guardados en el array
  `frutas`;
- `p frutas` muestra el array, con sus corchetes y sus comillas;
- `"rojo,verde,azul".split(",")` corta esta vez en cada coma;
- `lista.join(" / ")` vuelve a pegar los tres colores, con «/» entre cada uno.

Los arrays se explican en [los arrays y los hashes](collections.md).

**Te toca:** escribe un programa que corte `"pan;leche;huevos"` en cada punto y coma,
y luego vuelva a pegar los trozos con «, » entre ellos.

> **Solución**
>
> ```ruby
> compra = "pan;leche;huevos".split(";")
> puts compra.join(", ")
> ```
>
> ```sortie
> pan, leche, huevos
> ```

## Reemplazar un trozo

Es la función «Buscar y reemplazar» de un procesador de textos: corriges una palabra,
o cambias un nombre en todas partes de una vez.

`sub` reemplaza la **primera** vez que aparece un trozo. `gsub` las reemplaza
**todas**.

```ruby
texto = "un gato, un perro"
puts texto.sub("un", "el")
puts texto.gsub("un", "el")
```

```sortie
el gato, un perro
el gato, el perro
```

Línea por línea:

- `texto` contiene dos veces «un»;
- `sub("un", "el")` solo cambia el primer «un»;
- `gsub("un", "el")` cambia los dos;
- `texto` en sí no cambia: cada línea parte del texto original.

> **Recuerda:** piensa en «g» de «global»: `gsub` reemplaza en todas partes.

## Tomar una parte del texto

A veces solo te interesa una parte del texto: la inicial de un nombre, el año de una
fecha.

Como en un array, cada carácter tiene una posición, llamada
[índice](../debuter/glossaire.md#indice). La primera es `0`, no `1`: el índice
cuenta cuántos pasos hay que dar desde el principio.

```ruby
palabra = "girasol"
puts palabra[0]
puts palabra[-1]
puts palabra[0..3]
puts palabra[4..]
```

```sortie
g
l
gira
sol
```

- `palabra[0]`: el primer carácter.
- `palabra[-1]`: el último.
- `palabra[0..3]`: de la posición 0 a la posición 3, incluida.
- `palabra[4..]`: de la posición 4 hasta el final.

Una posición que no existe da `nil` (ningún valor):

```ruby
p "gato"[10]
```

```sortie
nil
```

## Los caracteres uno por uno

Para examinar cada letra de una palabra, para contarlas o transformarlas, primero hay
que separarlas.

`chars` da el array de todos los caracteres. Práctico para recorrerlos.

```ruby
p "abc".chars
"abc".chars.each { |letra| puts letra.upcase }
puts "abc".reverse
```

```sortie
["a", "b", "c"]
A
B
C
cba
```

Línea por línea:

- `"abc".chars` da el array `["a", "b", "c"]`, que `p` muestra;
- `each { |letra| ... }` toma cada carácter por turnos, lo guarda en `letra` y luego
  ejecuta `puts letra.upcase`: A, luego B, luego C;
- `reverse` lee el texto del final al principio.

`reverse` devuelve el texto al revés.

## Un texto en varias líneas

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Escribir una carta entera en una sola línea, con `\n` por todas partes, sería
ilegible. El heredoc te deja escribir el texto tal como se mostrará.

Para un texto largo, usa un **heredoc**: `<<~FIN`, luego tus líneas, luego `FIN`
solo en su línea.

```ruby
nombre = "Alicia"
carta = <<~FIN
  Hola, #{nombre}:
  Gracias por tu visita.
  ¡Hasta pronto!
FIN
puts carta
```

```sortie
Hola, Alicia:
Gracias por tu visita.
¡Hasta pronto!
```

> **Nota:** el `~` quita los espacios del principio de las líneas. Así puedes sangrar
> tu texto de forma limpia. La palabra `FIN` es libre: `TEXTO` o `MSG` también sirven.

## Texto y números

Un «42» escrito con el teclado es un texto, no un número: Ruby se niega a calcular con
él. Primero hay que convertirlo. Y para mostrar un número de forma limpia, se le da
formato.

### Pasar de uno a otro

- `to_i` convierte un texto en entero, `to_f` en número decimal.
- `to_s` convierte un número en texto.

```ruby
puts "42".to_i + 1
puts "2.5".to_f * 2
puts 10.to_s + "0"
```

```sortie
43
5.0
100
```

Línea por línea:

- `"42".to_i` convierte el texto en el número 42: ya se le puede sumar 1, lo que da 43;
- `"2.5".to_f` da el número 2.5; por 2, son 5.0;
- `10.to_s` da el texto `"10"`; `+ "0"` pega un cero al final: `"100"`. No se hace
  ningún cálculo.

### Alinear texto

`ljust` rellena con espacios a la derecha, `rjust` a la izquierda, `center` a ambos
lados. Práctico para hacer columnas.

```ruby
puts "pan".ljust(8, ".") + "1,20 €"
puts "mermelada".ljust(8, ".") + "3,50 €"
puts "42".rjust(5, "0")
puts "título".center(12, "*")
```

```sortie
pan.....1,20 €
mermelada3,50 €
00042
***título***
```

El texto nunca se corta: `"mermelada"` ya tiene más de 8 caracteres.

### Redondear y dar formato

`round(2)` redondea a 2 cifras después de la coma. `format` construye un texto con un
modelo: `%.2f` quiere decir «un número con 2 cifras después de la coma».

```ruby
precio = 12.34567
puts precio.round(2)
puts format("%.2f €", precio)
puts format("%.2f €", 5)
puts format("Ticket n.º %03d", 7)
```

```sortie
12.35
12.35 €
5.00 €
Ticket n.º 007
```

> **Consejo:** `%03d` escribe un entero con 3 cifras, con ceros delante.

**Te toca:** escribe un programa que guarde el texto `"2.5"` (un precio) en `precio` y
`3` en `cantidad`, y luego muestre el total: «Total: 7.50 €».

> **Solución**
>
> ```ruby
> precio = "2.5"
> cantidad = 3
> total = precio.to_f * cantidad
> puts format("Total: %.2f €", total)
> ```
>
> ```sortie
> Total: 7.50 €
> ```

## Los textos congelados

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Congelar un texto lo protege: un valor que nunca debe cambiar, como el nombre de tu
sitio web, ya no se puede modificar por error.

Un texto **congelado** (`freeze`) ya no se puede modificar: se vuelve
[inmutable](../debuter/glossaire.md#inmutable). Para cambiarlo, haz una
copia con `dup`.

```ruby
palabra = "hola".freeze
puts palabra.frozen?
copia = palabra.dup
copia << " mundo"
puts copia
```

```sortie
true
hola mundo
```

Línea por línea:

- `"hola".freeze` crea un texto congelado, guardado en `palabra`;
- `palabra.frozen?` comprueba que está congelado: `true`;
- `palabra.dup` fabrica una copia, que no está congelada;
- `copia << " mundo"` añade `" mundo"` al final de la copia. `palabra` sigue valiendo
  `"hola"`.

`<<` añade texto al final de una cadena. Con un texto congelado, provoca un error
`FrozenError`.

> **Nota:** a veces verás la línea `# frozen_string_literal: true` arriba de un
> archivo. Congela todos los textos escritos en ese archivo. Los métodos como `upcase`
> siguen funcionando: crean un texto nuevo.

## Errores frecuentes

### Olvidar convertir un texto en número

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

`"30"` es un texto, no un número. Conviértelo primero: `edad.to_i + 1`.

### Usar una posición que no existe

```ruby norun
palabra = "gato"
puts palabra[10].upcase
```

```text
main.rb:2:in `<main>': undefined method `upcase' for nil (NoMethodError)

puts palabra[10].upcase
                ^^^^^^^
```

`palabra[10]` vale `nil`, y `nil` no sabe hacer `upcase`. Comprueba la longitud
(`palabra.length`), o usa `&.`: `palabra[10]&.upcase` da `nil` sin fallar.

### Modificar un texto congelado

```ruby norun
palabra = "hola".freeze
palabra << " mundo"
```

```text
main.rb:2:in `<main>': can't modify frozen String: "hola" (FrozenError)
```

Un texto congelado ya no cambia. Crea un texto nuevo (`palabra + " mundo"`), o modifica
una copia (`palabra.dup << " mundo"`).

## Ponte a prueba

**Pregunta 1:** ¿qué muestra `puts "Ruby".downcase.reverse`?

> **Respuesta**
>
> `ybur`. `downcase` da `"ruby"`, y luego `reverse` le da la vuelta. Los métodos
> se encadenan de izquierda a derecha.

**Pregunta 2:** ¿cómo obtener `"Hola Alicia"` a partir de `"  hola alicia "`?

> **Respuesta**
>
> ```ruby
> texto = "  hola alicia "
> puts texto.strip.split(" ").map(&:capitalize).join(" ")
> ```
>
> ```sortie
> Hola Alicia
> ```
>
> `strip` quita los espacios, `split` corta en palabras, `map(&:capitalize)` pone una
> mayúscula a cada palabra (mira [los bloques](blocs.md)), `join` vuelve a pegar.

**Pregunta 3:** ¿cuánto vale `"12" + "3"`? ¿Y `"12".to_i + "3".to_i`?

> **Respuesta**
>
> `"123"` (dos textos pegados), y luego `15` (dos números sumados).

## En resumen

- Un texto se escribe entre comillas dobles; `#{...}` inserta un valor en él.
- `length`, `upcase`, `downcase`, `capitalize` y `strip` devuelven un texto nuevo.
- `include?`, `start_with?` y `end_with?` buscan un trozo.
- `split` corta en un array, `join` vuelve a pegar; `sub` y `gsub` reemplazan.
- `palabra[0..3]` toma una parte del texto, `chars` da los caracteres uno por uno.
- `round` y `format` muestran los números de forma limpia; `to_i` y `to_s` convierten.

Siguiente: [los arrays y los hashes](collections.md). Compara con
[el texto en Rust](../rust/texte.md).
