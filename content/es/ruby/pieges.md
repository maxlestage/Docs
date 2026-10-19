# Las trampas del principiante en Ruby

Esta página reúne las 12 trampas que más a menudo bloquean a los principiantes en Ruby, cada una con su explicación y la buena forma de hacerlo.

> **Objetivos:**
>
> - reconocerás las 12 trampas más comunes de Ruby;
> - sabrás por qué sorprenden;
> - sabrás corregirlas o, mejor aún, evitarlas.

## Cómo leer esta página

Cada trampa sigue el mismo camino:

1. lo que **esperas**;
2. el código que **sorprende**;
3. **por qué** Ruby hace eso;
4. la **corrección**.

No hace falta leerlo todo de una vez. Vuelve aquí cuando un programa haga «cualquier
cosa»: la causa suele estar en esta lista.

## Las trampas de la salida y de los cálculos

### 1. puts, print y p no muestran lo mismo

**Esperas:** que los tres muestren lo mismo.

**Lo que pasa:**

```ruby
puts "1"
p "1"
puts 1
p 1
puts nil
p nil
print "a"
print "b"
puts
```

```sortie
1
"1"
1
1

nil
ab
```

**Por qué:** cada [método](../debuter/glossaire.md#metodo) tiene su papel.

| Método | Lo que hace | ¿Para quién? |
|---|---|---|
| `puts` | muestra el texto y luego salta de línea | el usuario |
| `print` | muestra el texto, **sin** saltar de línea | el usuario |
| `p` | muestra el valor **tal como Ruby lo ve** | tú, para buscar un error |

Con `puts`, el texto `"1"` y el número `1` se parecen. ¡`nil` incluso se convierte en
una línea vacía! `p` muestra las comillas y escribe `nil`: ves la verdad.

**La corrección:** para buscar un error, usa `p`, no `puts`.

> **Pruébalo:** muestra `p [1, "1", nil]` y luego `puts [1, "1", nil]`. `puts` escribe
> un elemento por línea, y la línea de `nil` está vacía.

### 2. 7 / 2 da 3

**Esperas:** `3.5`.

**Lo que pasa:**

```ruby
puts 7 / 2
puts 7.0 / 2
puts 7.fdiv(2)
puts 7 % 2
```

```sortie
3
3.5
3.5
1
```

**Por qué:** `7` y `2` son [enteros](../debuter/glossaire.md#entero). Entre dos
enteros, `/` hace una **división entera**: guarda el cociente y tira el resto. Es como
repartir 7 caramelos entre 2 niños: cada uno recibe 3, y sobra 1 (es lo que da `%`).

**La corrección:** pon un [flotante](../debuter/glossaire.md#flotante) en el cálculo
(`7.0`) o usa `fdiv`.

```ruby
notas = [12, 15, 14]
media = notas.sum / notas.size
puts media
media = notas.sum.fdiv(notas.size)
puts media.round(2)
```

```sortie
13
13.67
```

> **Recuerda:** entero / entero = entero. Para una media, usa `fdiv` o `to_f`.

### 3. Un texto no es un número

**Esperas:** que `"5" + 5` dé `10`.

**Lo que pasa:**

```ruby norun
puts "5" + 5
```

```text
main.rb:1:in `+': no implicit conversion of Integer into String (TypeError)
	from main.rb:1:in `<main>'
```

Y cuando no hay error, el resultado también sorprende:

```ruby
puts "5" + "5"
puts "5" * 3
```

```sortie
55
555
```

**Por qué:** `"5"` es una [cadena de caracteres](../debuter/glossaire.md#cadena-de-caracteres),
no un número. Para un texto, `+` **pega** y `*` **repite**. Ruby nunca convierte un
texto en número por su cuenta.

**La corrección:** conviértelo tú mismo.

```ruby
puts "5".to_i + 5
puts "5" + 5.to_s
puts "Total: #{5 + 5}"
puts "abc".to_i
```

```sortie
10
55
Total: 10
0
```

| Escritura | Resultado | Lo que ha pasado |
|---|---|---|
| `"5".to_i + 5` | `10` | texto → número, luego suma |
| `"5" + 5.to_s` | `"55"` | número → texto, luego pegado |
| `"abc".to_i` | `0` | ninguna cifra al principio: ¡`0`, sin error! |

> **Atención:** `to_i` nunca se queja. Si el texto no es un número, obtienes `0`. Para
> obtener un error en su lugar, usa `Integer("abc")`.

## Las trampas de las condiciones

### 4. nil llega de todas partes

**Esperas:** que un valor ausente dé un error claro, o `0`.

**Lo que pasa:**

```ruby norun
stock = { "manzanas" => 3 }
puts stock["peras"] + 1
```

```text
main.rb:2:in `<main>': undefined method `+' for nil (NoMethodError)

puts stock["peras"] + 1
                    ^
```

**Por qué:** `nil` quiere decir «nada» (mira [null](../debuter/glossaire.md#null)). En
Ruby, muchas cosas devuelven `nil` **sin error**:

```ruby
stock = { "manzanas" => 3 }
p stock["peras"]
p [1, 2, 3][10]
p "hola".index("z")
p [].first
```

```sortie
nil
nil
nil
nil
```

El error solo llega más adelante, cuando **usas** ese `nil`. El mensaje
`undefined method ... for nil` quiere decir: «le has pedido algo a `nil`».

```text
stock["peras"]       →  nil       (aquí no hay error)
nil + 1              →  NoMethodError  (el error llega aquí)
```

**La corrección:** busca de dónde viene el `nil` y luego da un valor de reserva.

```ruby
stock = { "manzanas" => 3 }
puts stock.fetch("peras", 0) + 1
puts (stock["peras"] || 0) + 1
nombre = nil
p nombre&.upcase
```

```sortie
1
1
nil
```

`&.` quiere decir: «llama al método solo si no es `nil`».

> **Recuerda:** `undefined method ... for nil`: sube por el programa para encontrar la
> línea que produjo `nil`.

### 5. Escribir = en lugar de == en un if

**Esperas:** que este programa no muestre nada, porque `x` vale 5.

**Lo que pasa:**

```ruby
x = 5
if x = 3
  puts "x vale tres"
end
puts x
```

```sortie
x vale tres
3
```

**Por qué:** `=` **guarda** un valor, `==` **compara**. `x = 3` pone 3 en `x`, y vale
`3`. Y `3` cuenta como verdadero (mira la trampa siguiente). Así que el `if` pasa, ¡y
`x` cambia! Ruby solo muestra un aviso:
`warning: found '= literal' in conditional, should be ==`.

**La corrección:** `if x == 3`.

> **Consejo:** lee `=` como «toma el valor» y `==` como «¿es igual a?». Así evitas la
> confusión.

### 6. 0 y el texto vacío son verdaderos

**Esperas:** que `0`, `""` o `[]` cuenten como «falso», como en otros lenguajes.

**Lo que pasa:**

```ruby
[0, "", [], "false", nil, false].each do |valor|
  if valor
    puts "#{valor.inspect} es verdadero"
  else
    puts "#{valor.inspect} es falso"
  end
end
```

```sortie
0 es verdadero
"" es verdadero
[] es verdadero
"false" es verdadero
nil es falso
false es falso
```

**Por qué:** en Ruby, hay **solo dos valores falsos**: `false` y `nil`. Todo lo demás
es verdadero, incluso `0`, incluso el texto `"false"`.

**La corrección:** haz la pregunta de verdad.

```ruby
numero = 0
texto = ""
puts "cero" if numero.zero?
puts "texto vacío" if texto.empty?
```

```sortie
cero
texto vacío
```

> **Recuerda:** solo `false` y `nil` son falsos. Para comprobar «vacío» o «cero», usa
> `empty?` o `zero?`.

### 7. and y or no son && y ||

**Esperas:** que `ok = true and false` ponga `false` en `ok`.

**Lo que pasa:**

```ruby
ok = true and false
p ok
ok = true && false
p ok
```

```sortie
true
false
```

**Por qué:** `and` es **muy débil**: Ruby hace primero `ok = true`, y luego
`... and false`. Con paréntesis, se ve lo que Ruby entiende:

```text
ok = true and false    →   (ok = true) and false     →  ok vale true
ok = true && false     →   ok = (true && false)      →  ok vale false
```

**La corrección:** en las condiciones y los cálculos, usa siempre `&&`, `||` y `!`.

## Las trampas de los rangos y los bucles

### 8. Dos puntos o tres puntos

**Esperas:** que `1...5` contenga el 5.

**Lo que pasa:**

```ruby
p (1..5).to_a
p (1...5).to_a
letras = ["a", "b", "c"]
p (0...letras.size).to_a
```

```sortie
[1, 2, 3, 4, 5]
[1, 2, 3, 4]
[0, 1, 2]
```

**Por qué:** `..` **incluye** el final, `...` lo **excluye**. El tercer punto «empuja»
el final hacia fuera. `0...tamaño` es práctico para las posiciones de un array: van de
0 a tamaño − 1.

**La corrección:** relee tu [rango](../debuter/glossaire.md#rango) preguntándote:
«¿está dentro el último valor?». Todo se explica en [los rangos](intervalles.md).

### 9. Una variable creada en un bloque desaparece

**Esperas:** encontrar `ultimo` después del bucle.

**Lo que pasa:**

```ruby norun
[1, 2, 3].each { |n| ultimo = n }
puts ultimo
```

```text
main.rb:2:in `<main>': undefined local variable or method `ultimo' for main (NameError)

puts ultimo
     ^^^^^^
```

**Por qué:** una [variable](../debuter/glossaire.md#variable) **creada** en un bloque
`{ ... }` o `do ... end` solo existe dentro (es su
[ámbito](../debuter/glossaire.md#ambito)). Al salir del bloque, se tira.

**La corrección:** crea la variable **antes** del bloque. El bloque podrá cambiarla.

```ruby
ultimo = nil
total = 0
[1, 2, 3].each do |n|
  ultimo = n
  total += n
end
puts ultimo
puts total
```

```sortie
3
6
```

| Paso | n | ultimo | total |
|---|---|---|---|
| antes | – | nil | 0 |
| vuelta 1 | 1 | 1 | 1 |
| vuelta 2 | 2 | 2 | 3 |
| vuelta 3 | 3 | 3 | 6 |

## Las trampas de lo compartido

Estas tres trampas tienen la misma causa: **dos nombres para un solo objeto**. Imagina
una caja con dos etiquetas. Si cambias el contenido a través de una etiqueta, la otra
ve el cambio.

```text
  a ──┐
      ├──►  [ "gato" ]     una sola caja
  b ──┘
```

### 10. Modificar un texto también cambia la otra variable

**Esperas:** que `a` siga siendo `"gato"`.

**Lo que pasa:**

```ruby
a = "gato"
b = a
b << "s"
puts a
puts b
```

```sortie
gatos
gatos
```

**Por qué:** `b = a` no copia el texto. Pega una segunda etiqueta en la misma caja.
`<<` **modifica** la caja: `a` y `b` ven los dos `"gatos"`.

**La corrección:** `+` crea un texto **nuevo**, y deja el antiguo tranquilo.

```ruby
a = "gato"
b = a
b += "s"
puts a
puts b
```

```sortie
gato
gatos
```

| Escritura | ¿Qué hace? |
|---|---|
| `b << "s"` | modifica la caja (los otros nombres ven el cambio) |
| `b += "s"` | crea una caja nueva para `b` (los otros nombres no cambian) |
| `b = a.dup` | hace una copia de verdad |

Pasa lo mismo con los métodos que terminan en `!`: `upcase` devuelve un texto nuevo,
`upcase!` modifica el propio texto.

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

> **Pruébalo:** cambia `palabra.upcase` por `palabra = palabra.upcase`. La primera línea
> muestra entonces `HOLA`.

### 11. Array.new(3, []) comparte el mismo array

**Esperas:** una cuadrícula de 3 filas, y una `"x"` solo en la primera.

**Lo que pasa:**

```ruby
cuadricula = Array.new(3, [])
cuadricula[0] << "x"
p cuadricula
```

```sortie
[["x"], ["x"], ["x"]]
```

**Por qué:** Ruby crea **un solo** array vacío y lo pone 3 veces. Las 3 casillas son
3 etiquetas en la misma caja.

```text
cuadricula[0] ──┐
cuadricula[1] ──┼──►  [ ]     ¡un solo array!
cuadricula[2] ──┘
```

**La corrección:** con un bloque, Ruby crea un array nuevo para cada casilla.

```ruby
cuadricula = Array.new(3) { [] }
cuadricula[0] << "x"
p cuadricula
```

```sortie
[["x"], [], []]
```

### 12. Hash.new([]) comparte el mismo valor por defecto

**Esperas:** que `h[:a] << 1` guarde `[1]` bajo la clave `:a`.

**Lo que pasa:**

```ruby
h = Hash.new([])
h[:a] << 1
p h
p h[:b]
```

```sortie
{}
[1]
```

**Por qué:** el valor por defecto es **un solo** array, compartido por todas las claves
ausentes. `h[:a] << 1` añade 1 a ese array compartido, sin crear la clave `:a`. ¡El
[hash](../debuter/glossaire.md#diccionario) sigue vacío, y `h[:b]` ve el `1`!

**La corrección:** el bloque crea un array nuevo para cada clave, y lo guarda.

```ruby
h = Hash.new { |hash, clave| hash[clave] = [] }
h[:a] << 1
p h
p h[:b]
p h
```

```sortie
{:a=>[1]}
[]
{:a=>[1], :b=>[]}
```

> **Recuerda:** `Array.new(n, valor)` y `Hash.new(valor)` comparten **un solo** objeto.
> Para un array o un texto, pasa un bloque `{ ... }`.

## Errores frecuentes

### Modificar un texto congelado

```ruby norun
# frozen_string_literal: true
nombre = "Alicia"
nombre << " Martín"
```

```text
main.rb:3:in `<main>': can't modify frozen String: "Alicia" (FrozenError)
```

El comentario mágico `# frozen_string_literal: true` (frecuente al principio de los
archivos) **congela** todos los textos escritos entre comillas: se vuelven
[inmutables](../debuter/glossaire.md#inmutable). Usa `+`: `nombre = nombre + " Martín"`,
o parte de una copia: `nombre = +"Alicia"` (un texto no congelado). Mira
[los textos congelados](texte.md#los-textos-congelados).

### Llamar a un método sobre una casilla que no existe

```ruby norun
frutas = ["manzana", "kiwi"]
puts frutas[2].upcase
```

```text
main.rb:2:in `<main>': undefined method `upcase' for nil (NoMethodError)

puts frutas[2].upcase
              ^^^^^^^
```

Las posiciones van de 0 a 1: `frutas[2]` vale `nil` (trampa 4). Comprueba la posición,
o usa `frutas.last` para el último elemento.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este programa?

```ruby norun
puts 10 / 4
puts 10 / 4.0
```

> **Respuesta**
>
> `2` y luego `2.5`. Entre dos enteros, `/` solo guarda la parte entera.
>
> ```ruby
> puts 10 / 4
> puts 10 / 4.0
> ```
>
> ```sortie
> 2
> 2.5
> ```

**Pregunta 2:** ¿muestra este programa «nada»?

```ruby norun
respuesta = 0
puts "nada" unless respuesta
```

> **Respuesta**
>
> No. `0` es verdadero en Ruby, así que `unless respuesta` no pasa. Para comprobar el
> cero, escribe `respuesta.zero?`.
>
> ```ruby
> respuesta = 0
> puts "nada" unless respuesta
> puts "cero" if respuesta.zero?
> ```
>
> ```sortie
> cero
> ```

**Pregunta 3:** ¿cómo creas un array de 2 arrays vacíos **independientes**?

> **Respuesta**
>
> Con un bloque: cada casilla recibe su propio array.
>
> ```ruby
> pares = Array.new(2) { [] }
> pares[1] << "b"
> p pares
> ```
>
> ```sortie
> [[], ["b"]]
> ```

## En resumen

- `p` muestra el valor real (comillas, `nil`): úsalo para buscar un error.
- Entero / entero = entero; un texto nunca se convierte en número por su cuenta.
- `nil` llega sin hacer ruido; solo `false` y `nil` son falsos, `0` es verdadero.
- `=` guarda, `==` compara; prefiere `&&` y `||` a `and` y `or`.
- `..` incluye el final, `...` lo excluye; una variable creada en un bloque se queda ahí.
- `<<`, los métodos en `!`, `Array.new(n, [])` y `Hash.new([])` comparten un objeto.

Siguiente: [ejercicios para empezar bien](exercices-faciles.md)
