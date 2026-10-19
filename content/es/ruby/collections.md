# Arrays y hashes

Esta página presenta las dos colecciones principales de Ruby: los arrays (`Array`) y los hashes (`Hash`).

> **Objetivos:**
>
> - sabrás crear un array, leer, añadir y quitar sus elementos;
> - sabrás ordenar, contar y buscar en un array;
> - sabrás crear un hash y encontrar un valor por su clave;
> - sabrás manejar una clave ausente con `fetch`, un valor por defecto o `dig`;
> - sabrás recorrer un array y un hash con `each`.

Una **colección** es un valor que contiene varios. Ruby ofrece dos
principales:

- el **array**, una [lista](../debuter/glossaire.md#lista) guardada en orden, como
  una fila de casilleros numerados;
- el **hash**, un [diccionario](../debuter/glossaire.md#diccionario) que asocia una
  etiqueta a cada valor, como una agenda «nombre → número».

## Los arrays

Imagina una lista de la compra de veinte artículos. Crear veinte variables `articulo1`,
`articulo2`… sería pesado. Un array guarda todos esos valores bajo un solo nombre.

Un array es una lista ordenada de valores, entre corchetes `[ ]`, separados por
comas. Cada valor se llama **elemento**.

```ruby
frutas = ["manzana", "plátano", "cereza"]
p frutas
puts frutas.length
```

```sortie
["manzana", "plátano", "cereza"]
3
```

Línea por línea:

- `frutas = [...]` guarda un array de tres textos en la variable `frutas`;
- `p frutas` muestra el array entero, con sus corchetes y sus comillas;
- `frutas.length` pide el número de elementos: 3.

Un array puede contener cualquier valor, incluso mezclados, e incluso otros
arrays. `[]` es un array vacío.

### Leer un elemento

Las posiciones ([índices](../debuter/glossaire.md#indice)) empiezan en `0`. Un índice
negativo cuenta desde el final: `-1` es el último elemento.

```ruby
frutas = ["manzana", "plátano", "cereza"]
puts frutas[0]
puts frutas[-1]
puts frutas.first
p frutas[10]
```

```sortie
manzana
cereza
manzana
nil
```

> **Nota:** leer un índice que no existe no hace fallar el programa: obtienes `nil`.

### Leer varios elementos

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Un rango, o `first(n)` y `last(n)`, dan un trozo del array.

```ruby
letras = ["a", "b", "c", "d", "e"]
p letras[1..3]
p letras.first(2)
p letras.last(2)
```

```sortie
["b", "c", "d"]
["a", "b"]
["d", "e"]
```

### Añadir y quitar

- `<<` y `push` añaden al final; `unshift` añade al principio;
- `pop` quita el último elemento y lo devuelve; `shift` quita el primero;
- `delete` quita todas las casillas que tienen ese valor.

```ruby
numeros = [1, 2]
numeros << 3
numeros.push(4)
p numeros
ultimo = numeros.pop
p ultimo
p numeros
numeros.delete(1)
p numeros
```

```sortie
[1, 2, 3, 4]
4
[1, 2, 3]
[2, 3]
```

Línea por línea:

- `numeros` empieza con dos elementos: `[1, 2]`;
- `<< 3` y luego `push(4)` añaden 3 y 4 al final: el array vale `[1, 2, 3, 4]`;
- `numeros.pop` quita el último elemento, 4, y lo guarda en `ultimo`;
- `delete(1)` quita el **valor** 1 (no el índice 1): queda `[2, 3]`.

```ruby
fila = ["Bob", "Cloe"]
fila.unshift("Alicia")
p fila
primero = fila.shift
puts "Te toca, #{primero}"
p fila
fila[0] = "Zoe"
p fila
```

```sortie
["Alicia", "Bob", "Cloe"]
Te toca, Alicia
["Bob", "Cloe"]
["Zoe", "Cloe"]
```

`fila[0] = "Zoe"` reemplaza el elemento en el índice 0.

**Te toca:** escribe un programa que cree un array vacío `compras`, le añada «pan» y
luego «leche» con `<<`, muestre el array y después su número de elementos.

> **Solución**
>
> ```ruby
> compras = []
> compras << "pan"
> compras << "leche"
> p compras
> puts compras.length
> ```
>
> ```sortie
> ["pan", "leche"]
> 2
> ```

### Métodos útiles

Estos métodos devuelven un resultado **sin modificar** el array.

```ruby
notas = [12, 8, 15, 8]
p notas.sort
p notas.uniq
puts notas.include?(15)
puts notas.sum
puts notas.max
p notas.reverse
puts notas.join(" - ")
```

```sortie
[8, 8, 12, 15]
[12, 8, 15]
true
43
15
[8, 15, 8, 12]
12 - 8 - 15 - 8
```

Otras preguntas útiles:

```ruby
notas = [12, 8, 15, 8]
puts notas.count(8)
p notas.index(15)
puts notas.empty?
puts [].empty?
p notas.sort.reverse
puts notas.sum.fdiv(notas.length)
```

```sortie
2
2
false
true
[15, 12, 8, 8]
10.75
```

- `count(8)` cuenta los 8; `index(15)` da la posición del primer 15.
- `sort.reverse` ordena de mayor a menor.
- `sum.fdiv(length)` calcula la media, con decimales.

## Recorrer un array

A menudo quieres hacer lo mismo con cada elemento: mostrar cada artículo de la lista de
la compra, saludar a cada alumno. En lugar de escribir una línea por elemento, dejas
que `each` repita el trabajo.

`each` ejecuta un bloque para cada elemento.

```ruby
["manzana", "pera"].each do |fruta|
  puts "Me gusta la #{fruta}"
end
```

```sortie
Me gusta la manzana
Me gusta la pera
```

Línea por línea:

- `["manzana", "pera"]` es un array de dos textos; `.each` lo recorre en orden;
- en la primera vuelta, Ruby guarda `"manzana"` en `fruta` (el nombre entre las barras
  `| |`) y ejecuta el bloque entre `do` y `end`;
- en la segunda vuelta, `fruta` vale `"pera"`;
- ya no quedan elementos: el bucle se detiene.

Con `each_with_index`, obtienes también la posición:

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

**Te toca:** escribe un programa que recorra el array `[3, 7, 10]` con `each` y muestre
el doble de cada número.

> **Solución**
>
> ```ruby
> [3, 7, 10].each do |n|
>   puts n * 2
> end
> ```
>
> ```sortie
> 6
> 14
> 20
> ```

## Los hashes

En un array, encuentras un valor por su posición: 0, 1, 2… Pero para encontrar la edad
de Alicia, un número de casilla no dice nada. Un hash te deja buscar por un nombre.

Un hash asocia **claves** a **valores**. Piensa en un guardarropa: das
el ticket (la clave) y te devuelven el abrigo (el valor). Cada clave es única. Las
claves suelen ser [símbolos](premiers-pas.md#los-simbolos).

```ruby
persona = { nombre: "Alicia", edad: 30 }
puts persona[:nombre]
puts persona[:edad]
p persona[:ciudad]
```

```sortie
Alicia
30
nil
```

Línea por línea:

- `persona` contiene dos pares: la clave `:nombre` con `"Alicia"`, la clave `:edad`
  con 30;
- `persona[:nombre]` da el valor guardado bajo la clave `:nombre`: `"Alicia"`;
- `persona[:ciudad]` pide una clave que no existe: obtienes `nil`, sin fallo.

> **Recuerda:** `{ nombre: "Alicia" }` es un atajo para `{ :nombre => "Alicia" }`. Las
> dos formas dan el mismo hash.

### Modificar un hash

- `hash[clave] = valor` añade una clave, o reemplaza su valor si ya existe;
- `delete` quita una clave;
- `key?` pregunta si una clave existe.

```ruby
stock = { "manzanas" => 5 }
stock["peras"] = 3
stock["manzanas"] += 1
stock.delete("peras")
p stock
puts stock.key?("manzanas")
```

```sortie
{"manzanas"=>6}
true
```

Línea por línea:

- `stock` empieza con una sola clave, `"manzanas"`, que vale 5;
- `stock["peras"] = 3` añade una clave nueva;
- `stock["manzanas"] += 1` lee 5, suma 1 y guarda 6 en su lugar;
- `delete("peras")` quita la clave añadida justo antes: solo queda `"manzanas"`.

**Te toca:** escribe un programa que cree un hash `capitales` con la clave `"Francia"`
y el valor `"París"`, le añada la clave `"Italia"` con el valor `"Roma"` y luego
muestre la capital de Italia.

> **Solución**
>
> ```ruby
> capitales = { "Francia" => "París" }
> capitales["Italia"] = "Roma"
> puts capitales["Italia"]
> ```
>
> ```sortie
> Roma
> ```

### Valor por defecto con fetch

`fetch` permite dar un valor de reserva si falta la clave.

```ruby
config = { idioma: "es" }
puts config.fetch(:idioma, "en")
puts config.fetch(:tema, "claro")
```

```sortie
es
claro
```

### Valor por defecto con Hash.new

`Hash.new(0)` crea un hash en el que **todas** las claves ausentes valen `0`. Es ideal
para contar.

```ruby
votos = Hash.new(0)
["manzana", "kiwi", "manzana"].each { |fruta| votos[fruta] += 1 }
p votos
puts votos["fresa"]
```

```sortie
{"manzana"=>2, "kiwi"=>1}
0
```

Sin el `0`, `votos[fruta] += 1` fallaría: no se puede sumar 1 a `nil`.

> **Atención:** no escribas `Hash.new([])`: todas las claves compartirían el mismo
> array. Para un array por clave, usa `Hash.new { |h, k| h[k] = [] }`.

### Leer datos anidados con dig

Un hash puede contener otros hashes y arrays. `dig` baja nivel a nivel
y devuelve `nil` si falta un nivel, en lugar de fallar.

```ruby
alumno = { nombre: "Alicia", direccion: { ciudad: "Lyon" }, notas: [12, 15] }
puts alumno[:direccion][:ciudad]
puts alumno.dig(:direccion, :ciudad)
puts alumno.dig(:notas, 1)
p alumno.dig(:padres, :madre)
```

```sortie
Lyon
Lyon
15
nil
```

`alumno[:padres][:madre]` fallaría, porque `alumno[:padres]` vale `nil`.

## Recorrer un hash

Para mostrar toda una lista de precios o toda una agenda, quieres pasar por cada par
clave-valor, uno a uno. `each` también lo hace con los hashes.

`each` da la clave y el valor en cada vuelta.

```ruby
precios = { cafe: 2, te: 3 }
precios.each do |bebida, importe|
  puts "#{bebida}: #{importe} €"
end
p precios.keys
p precios.values
```

```sortie
cafe: 2 €
te: 3 €
[:cafe, :te]
[2, 3]
```

Línea por línea:

- `precios` asocia `:cafe` a 2 y `:te` a 3;
- en la primera vuelta, `each` guarda la clave `:cafe` en `bebida` y el valor 2 en
  `importe`; en la segunda vuelta, `:te` y 3;
- un símbolo mostrado dentro de un texto pierde sus dos puntos: lees `cafe`;
- `keys` da el array de las claves, `values` el de los valores.

**Te toca:** escribe un programa que recorra el hash `{ "Alicia" => 12, "Bob" => 15 }`
y muestre, para cada persona, una línea como «Alicia tiene 12».

> **Solución**
>
> ```ruby
> notas = { "Alicia" => 12, "Bob" => 15 }
> notas.each do |nombre, nota|
>   puts "#{nombre} tiene #{nota}"
> end
> ```
>
> ```sortie
> Alicia tiene 12
> Bob tiene 15
> ```

> **Consejo:** para transformar o filtrar una colección, usa `map` y `select`.
> Se explican en [la página sobre los bloques](blocs.md).

## Errores frecuentes

### Leer más allá del final del array

```ruby norun
frutas = ["manzana", "plátano", "cereza"]
puts frutas[3].upcase
```

```text
main.rb:2:in `<main>': undefined method `upcase' for nil (NoMethodError)

puts frutas[3].upcase
              ^^^^^^^
```

Tres elementos tienen los índices 0, 1 y 2. `frutas[3]` vale `nil`. El último es
`frutas[2]`, o mejor `frutas[-1]` o `frutas.last`.

### Confundir una clave de texto y una clave símbolo

```ruby norun
stock = { "manzanas" => 5 }
puts stock[:manzanas] + 1
```

```text
main.rb:2:in `<main>': undefined method `+' for nil (NoMethodError)

puts stock[:manzanas] + 1
                      ^
```

`"manzanas"` (un texto) y `:manzanas` (un símbolo) son dos claves distintas. Usa la
misma que en el hash: `stock["manzanas"] + 1`.

### fetch sin valor de reserva

```ruby norun
config = { idioma: "es" }
puts config.fetch(:tema)
```

```text
main.rb:2:in `fetch': key not found: :tema (KeyError)
	from main.rb:2:in `<main>'
```

Sin valor de reserva, `fetch` falla si falta la clave: es a propósito, para detectar
una clave olvidada. Da un valor por defecto: `config.fetch(:tema, "claro")`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto vale `[5, 3, 9][1]`?

> **Respuesta**
>
> `3`. Los índices empiezan en 0: el índice 1 es el segundo elemento.

**Pregunta 2:** ¿cómo contar las letras de `"abracadabra"` en un hash?

> **Respuesta**
>
> ```ruby
> cuenta = Hash.new(0)
> "abracadabra".chars.each { |c| cuenta[c] += 1 }
> p cuenta
> ```
>
> ```sortie
> {"a"=>5, "b"=>2, "r"=>2, "c"=>1, "d"=>1}
> ```

**Pregunta 3:** ¿cuál es la diferencia entre `h[:x]` y `h.fetch(:x)` cuando la clave
`:x` no existe?

> **Respuesta**
>
> `h[:x]` devuelve `nil` (o el valor por defecto del hash). `h.fetch(:x)` lanza un
> error `KeyError`.

## En resumen

- Un array es una lista ordenada: `[1, 2, 3]`. Los índices empiezan en `0`.
- `<<` añade al final, `pop` quita el último, `delete` quita un valor.
- Un hash asocia claves a valores: `{ nombre: "Alicia" }`.
- Una clave ausente da `nil`; `fetch`, `Hash.new(0)` y `dig` manejan las ausencias.
- `each` recorre un array o un hash.

Siguiente: [los rangos](intervalles.md)
