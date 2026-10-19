# Los métodos

Esta página explica cómo escribir métodos en Ruby con `def`, sus parámetros y su valor de retorno.

> **Objetivos:**
>
> - sabrás escribir un método con `def` y llamarlo;
> - sabrás dar parámetros a un método, con o sin valor por defecto;
> - sabrás devolver un valor, con o sin `return`;
> - sabrás qué variables puede ver un método;
> - sabrás pasar un bloque a un método con `yield`.

## Definir un método

Un [método](../debuter/glossaire.md#metodo) es un trozo de código con nombre, que
puedes reutilizar. Es como una receta: la escribes una vez y luego la
sigues tantas veces como quieras. Empieza por `def` (*define*, «definir»)
y termina con `end`.

```ruby
def saludar
  puts "¡Hola!"
end

saludar
saludar
```

```sortie
¡Hola!
¡Hola!
```

Línea por línea:

- `def saludar` empieza la receta y le da un nombre: `saludar`;
- `puts "¡Hola!"` es el contenido de la receta. Por ahora, no se muestra nada;
- `end` cierra la receta;
- el primer `saludar` ejecuta el método: se muestra «¡Hola!». El segundo vuelve a hacer
  exactamente lo mismo.

Escribir `def` no ejecuta nada: guarda la receta. Escribir `saludar` **llama** al
método, es decir, lo ejecuta.

> **Nota:** en otros lenguajes se dice [función](../debuter/glossaire.md#funcion).
> En Ruby, todo es un objeto, y cada función pertenece a un objeto: por eso se habla
> de método.

## Los parámetros

Un método que siempre hace exactamente lo mismo sirve de poco. Para saludar a Alicia y
luego a Bob, no quieres escribir dos métodos: quieres una sola receta, con un
ingrediente que cambia cada vez.

Los [parámetros](../debuter/glossaire.md#parametro) son los valores que el método
recibe. Son casillas vacías, que se rellenan en el momento de la llamada. Los valores dados en
la llamada se llaman [argumentos](../debuter/glossaire.md#argumento).

```ruby
def saludar(nombre)
  puts "¡Hola, #{nombre}!"
end

saludar("Alicia")
saludar "Bob"
```

```sortie
¡Hola, Alicia!
¡Hola, Bob!
```

Línea por línea:

- `def saludar(nombre)` crea un método con una casilla vacía, llamada `nombre`;
- `saludar("Alicia")` llama al método: `nombre` recibe `"Alicia"`, y la frase se
  muestra con ese nombre;
- `saludar "Bob"` lo llama una segunda vez. Esta vez, `nombre` vale `"Bob"`.

Aquí, `nombre` es el parámetro; `"Alicia"` y `"Bob"` son los argumentos.

> **Nota:** los paréntesis son opcionales en la llamada. Mantenlos cuando haya
> varios argumentos: es más legible.

Varios parámetros se separan con comas. Los argumentos se dan en el
mismo orden.

```ruby
def area(ancho, alto)
  ancho * alto
end

puts area(3, 4)
```

```sortie
12
```

**Te toca:** escribe un método `presentar(nombre, edad)` que muestre una frase
como «Me llamo Lea y tengo 12 años.». Llámalo para Lea (12 años) y luego para Tom
(9 años).

> **Solución**
>
> ```ruby
> def presentar(nombre, edad)
>   puts "Me llamo #{nombre} y tengo #{edad} años."
> end
>
> presentar("Lea", 12)
> presentar("Tom", 9)
> ```
>
> ```sortie
> Me llamo Lea y tengo 12 años.
> Me llamo Tom y tengo 9 años.
> ```

## El retorno implícito

A menudo, un método calcula un resultado que necesitas después, como una calculadora
que te da un total. **Devolver** un valor es entregar ese resultado al código que llamó
al método.

Un método devuelve automáticamente el valor de su **última expresión**. La palabra
`return` es inútil en ese caso.

```ruby
def cuadrado(x)
  x * x
end

puts cuadrado(4)
resultado = cuadrado(3) + 1
puts resultado
```

```sortie
16
10
```

Línea por línea:

- `def cuadrado(x)` guarda el método. Su última línea, `x * x`, da el resultado;
- `puts cuadrado(4)`: `x` vale 4, el método devuelve 16, y `puts` lo muestra;
- `resultado = cuadrado(3) + 1`: el método devuelve 9, Ruby suma 1 y guarda 10 en
  `resultado`;
- `puts resultado` muestra 10.

El valor devuelto reemplaza a la llamada: `cuadrado(3) + 1` se convierte en `9 + 1`.

`return` sirve sobre todo para salir antes del método:

```ruby
def dividir(a, b)
  return "división imposible" if b.zero?
  a / b
end

puts dividir(10, 2)
puts dividir(10, 0)
```

```sortie
5
división imposible
```

**Te toca:** escribe un método `perimetro(ancho, alto)` que devuelva el perímetro de un
rectángulo (dos veces el ancho más dos veces el alto), sin escribir `return`. Muestra
`perimetro(3, 4)`.

> **Solución**
>
> ```ruby
> def perimetro(ancho, alto)
>   2 * ancho + 2 * alto
> end
>
> puts perimetro(3, 4)
> ```
>
> ```sortie
> 14
> ```

### ¿Mostrar o devolver?

`puts` **muestra** un valor en la pantalla. Devolver un valor lo **entrega** al código que
llamó al método, que puede usarlo. Un método que termina con `puts` devuelve `nil`.

```ruby
def doble_mostrado(x)
  puts x * 2
end

def doble(x)
  x * 2
end

p doble_mostrado(5)
p doble(5)
```

```sortie
10
nil
10
```

> **Recuerda:** prefiere los métodos que devuelven un valor. Luego elegirás
> mostrarlo, guardarlo o reutilizarlo.

### El método en una línea

Para un método muy corto, escribe `def nombre(parámetros) = expresión`.

```ruby
def triple(x) = x * 3

puts triple(7)
```

```sortie
21
```

### Devolver varios valores

Separa los valores con comas. Ruby los devuelve en un array, que puedes
repartir en varias variables.

```ruby
def min_max(numeros)
  return numeros.min, numeros.max
end

menor, mayor = min_max([4, 9, 1])
puts menor
puts mayor
```

```sortie
1
9
```

## Los valores por defecto

Algunos ajustes cambian poco. En una cafetería, si no dices nada más, te sirven un café
normal. Un valor por defecto hace lo mismo: te evita repetirlo todo en cada llamada.

Un parámetro puede tener un valor por defecto. Se usa si falta el argumento.

```ruby
def saludar(nombre, saludo = "Hola")
  "¡#{saludo}, #{nombre}!"
end

puts saludar("Alicia")
puts saludar("Bob", "Buenas")
```

```sortie
¡Hola, Alicia!
¡Buenas, Bob!
```

Línea por línea:

- `saludo = "Hola"`: sin segundo argumento, `saludo` valdrá `"Hola"`;
- `saludar("Alicia")` solo da un argumento: `nombre` vale `"Alicia"`, y `saludo` toma
  su valor por defecto;
- `saludar("Bob", "Buenas")` da los dos: `"Buenas"` reemplaza el valor por defecto;
- el método devuelve la frase, y `puts` la muestra.

**Te toca:** escribe un método `precio_total(precio, cantidad = 1)` que devuelva el
precio multiplicado por la cantidad. Muestra `precio_total(5)` y luego
`precio_total(5, 3)`.

> **Solución**
>
> ```ruby
> def precio_total(precio, cantidad = 1)
>   precio * cantidad
> end
>
> puts precio_total(5)
> puts precio_total(5, 3)
> ```
>
> ```sortie
> 5
> 15
> ```

## Los argumentos con nombre

Cuando un método tiene varios parámetros, olvidas rápido cuál va primero. Dar un nombre
a cada argumento es como rellenar un formulario: cada casilla lleva una etiqueta.

Con `nombre:`, un argumento se da por su nombre. El orden ya no importa.

```ruby
def pedido(producto:, cantidad: 1)
  "#{cantidad} x #{producto}"
end

puts pedido(producto: "café")
puts pedido(cantidad: 3, producto: "té")
```

```sortie
1 x café
3 x té
```

- `producto:` es **obligatorio**: no tiene valor por defecto.
- `cantidad: 1` es opcional: vale `1` si no se da.

> **Consejo:** los argumentos con nombre hacen las llamadas muy claras, sobre todo con
> booleanos: `enviar(urgente: true)` se entiende enseguida.

## Un número variable de argumentos

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

A veces no sabes de antemano cuántos valores vas a recibir, como los artículos de un
ticket de compra: dos un día, diez al siguiente.

Con `*`, un parámetro recibe todos los argumentos restantes, guardados en un array.

```ruby
def suma(*numeros)
  numeros.sum
end

puts suma(1, 2)
puts suma(1, 2, 3, 4)
puts suma
```

```sortie
3
10
0
```

## El ámbito de las variables

Un programa tiene enseguida decenas de variables. Si cada método pudiera tocarlas
todas, un error en un sitio rompería algo en otro. El ámbito evita esa mezcla.

El [ámbito](../debuter/glossaire.md#ambito) de una variable es la zona del código donde
existe. Un método es una habitación cerrada: no ve las variables creadas
fuera. Solo ve sus parámetros y sus propias variables.

```ruby
def precio_con_iva(precio, tasa)
  total = precio * (1 + tasa)
  total.round(2)
end

tasa_iva = 0.2
puts precio_con_iva(100, tasa_iva)
p defined?(total)
```

```sortie
120.0
nil
```

- Para darle un valor a un método, pásalo como argumento.
- `total` solo existe dentro del método: fuera, `defined?(total)` responde `nil`.

Línea por línea:

- Ruby guarda el método y luego guarda 0.2 en `tasa_iva`;
- `precio_con_iva(100, tasa_iva)` da 100 a `precio` y 0.2 a `tasa`;
- dentro del método, `total` vale `100 * 1.2`, es decir 120.0. `total.round(2)`
  redondea a dos cifras decimales: es el valor devuelto, que `puts` muestra;
- después de la llamada, `total` ha desaparecido: `defined?(total)` responde `nil`.

> **Consejo:** una **constante** (`TASA_IVA = 0.2`), en cambio, es visible en todas partes,
> incluso dentro de un método.

## Los nombres con ? y con !

Estos dos signos avisan a quien lee el código, como una etiqueta en un tarro: sabes lo
que contiene antes de abrirlo.

Por convención:

- un método que termina en `?` responde con verdadero o falso;
- un método que termina en `!` modifica el objeto o es «peligroso».

```ruby
def mayor_de_edad?(edad)
  edad >= 18
end

puts mayor_de_edad?(20)

nombre = "alicia"
nombre.capitalize!
puts nombre
```

```sortie
true
Alicia
```

> **Atención:** `capitalize` devuelve una cadena nueva. `capitalize!` modifica la
> propia cadena.

## Un método que se llama a sí mismo

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Algunas tareas se repiten en más pequeño. Contar hacia atrás desde 3 es decir 3 y luego
contar hacia atrás desde 2.

Un método puede llamarse a sí mismo: es la
[recursión](../debuter/glossaire.md#recursion). Hace falta un **caso base**, si no
nunca se detiene.

```ruby
def cuenta_atras(n)
  return puts("¡Ya!") if n.zero?
  puts n
  cuenta_atras(n - 1)
end

cuenta_atras(3)
```

```sortie
3
2
1
¡Ya!
```

Aquí, el caso base es `n.zero?`. Cada llamada acerca `n` a cero.

## Pasar un bloque con yield

A veces, un método sabe **cuándo** actuar, pero no **qué** hacer. Imagina a alguien que
dice «lo hago dos veces»: eres tú quien elige lo que hace.

Un método puede recibir un **bloque** de código, escrito entre `{ }` después de la llamada.
`yield` («ceder el paso») ejecuta ese bloque.

```ruby
def dos_veces
  yield
  yield
end

dos_veces { puts "¡Hip, hip, hurra!" }
```

```sortie
¡Hip, hip, hurra!
¡Hip, hip, hurra!
```

Línea por línea:

- `def dos_veces` define un método sin parámetros, que contiene dos `yield`;
- `dos_veces { puts "¡Hip, hip, hurra!" }` llama al método y le da el bloque escrito
  entre `{ }`;
- el primer `yield` ejecuta el bloque: se muestra la frase. El segundo `yield` lo
  ejecuta una segunda vez.

`yield` también puede pasar un valor al bloque:

```ruby
def con_bonus(puntos)
  yield(puntos + 10)
end

con_bonus(5) { |total| puts "Total: #{total}" }
```

```sortie
Total: 15
```

Los bloques se explican en detalle en [la página sobre los bloques](blocs.md).

**Te toca:** escribe un método `tres_veces` que ejecute tres veces el bloque que
recibe. Llámalo con un bloque que muestre «¡Bravo!».

> **Solución**
>
> ```ruby
> def tres_veces
>   yield
>   yield
>   yield
> end
>
> tres_veces { puts "¡Bravo!" }
> ```
>
> ```sortie
> ¡Bravo!
> ¡Bravo!
> ¡Bravo!
> ```

## Errores frecuentes

### Olvidar un argumento

```ruby norun
def area(ancho, alto)
  ancho * alto
end
puts area(3)
```

```text
main.rb:1:in `area': wrong number of arguments (given 1, expected 2) (ArgumentError)
	from main.rb:4:in `<main>'
```

El método espera 2 argumentos (*expected 2*) y recibe 1 (*given 1*). Dalos
todos: `area(3, 4)`. O dale un valor por defecto al parámetro.

### Llamar al método antes de definirlo

```ruby norun
saludar("Alicia")
def saludar(nombre)
  puts "¡Hola, #{nombre}!"
end
```

```text
main.rb:1:in `<main>': undefined method `saludar' for main (NoMethodError)

saludar("Alicia")
^^^^^^^
```

Ruby lee el archivo de arriba abajo. En la línea 1, `saludar` todavía no existe.
Pon el `def` antes de la llamada.

### Usar una variable de fuera

```ruby norun
tasa = 0.2
def precio_con_iva(precio)
  precio * (1 + tasa)
end
puts precio_con_iva(100)
```

```text
main.rb:3:in `precio_con_iva': undefined local variable or method `tasa' for main (NameError)

  precio * (1 + tasa)
                ^^^^
	from main.rb:5:in `<main>'
```

El método no ve `tasa`. Pásala como parámetro (`def precio_con_iva(precio, tasa)`),
o conviértela en una constante (`TASA = 0.2`).

### Olvidar un argumento con nombre obligatorio

```ruby norun
def pedido(producto:, cantidad: 1)
  "#{cantidad} x #{producto}"
end
puts pedido(cantidad: 2)
```

```text
main.rb:1:in `pedido': missing keyword: :producto (ArgumentError)
	from main.rb:4:in `<main>'
```

Falta `producto:`. Escribe `pedido(producto: "té", cantidad: 2)`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```ruby norun
def f(x)
  x + 1
  x * 2
end
puts f(5)
```

> **Respuesta**
>
> `10`. El método devuelve su **última** expresión, `x * 2`. La línea `x + 1` se
> calcula y luego se olvida.

**Pregunta 2:** escribe un método `media(a, b)` que devuelva la media de dos
enteros, con decimales si hace falta.

> **Respuesta**
>
> ```ruby
> def media(a, b)
>   (a + b) / 2.0
> end
>
> puts media(3, 4)
> ```
>
> ```sortie
> 3.5
> ```
>
> El `2.0` evita la división entera.

**Pregunta 3:** ¿cuál es la diferencia entre un parámetro y un argumento?

> **Respuesta**
>
> El parámetro es el nombre escrito en `def` (`nombre`). El argumento es el valor dado
> en la llamada (`"Alicia"`).

## En resumen

- Un método empieza por `def` y termina con `end`; se llama por su nombre.
- Devuelve el valor de su última línea; `return` sirve para salir antes.
- Un parámetro puede tener un valor por defecto, o tener nombre (`producto:`).
- Un método no ve las variables de fuera: pásalas como argumentos.
- Un nombre con `?` responde verdadero o falso; un nombre con `!` modifica el objeto.
- `yield` ejecuta el bloque dado al método.

Siguiente: [trabajar con texto](texte.md). Compara con
[las funciones en Rust](../rust/fonctions.md).
