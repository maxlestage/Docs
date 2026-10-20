# Procs, lambdas y métodos

Esta página muestra cómo guardar código en una variable con una proc o una lambda, pasarlo a un método, devolverlo y combinarlo.

> **Objetivos:**
>
> - sabrás la diferencia entre un bloque, una proc y una lambda;
> - sabrás crear una lambda con `->(x) { }` y llamarla;
> - sabrás pasar de una a otra con `&` y `method(:nombre)`;
> - sabrás que una lambda conserva las variables que la rodean;
> - sabrás escribir un método que devuelve una función, y componer funciones.

## Bloque, proc, lambda: tres primos

Ya conoces los [bloques](blocs.md): código que se le da a un método, entre `{ }`. Un
bloque tiene un límite: solo existe mientras dura la llamada. No puedes guardarlo en
una variable para volver a usarlo.

Una **proc** y una **lambda** son bloques convertidos en objetos. Se pueden guardar,
pasar, devolver. Piensa en una receta: el bloque es una receta dicha en voz alta, la
proc o la lambda es la misma receta **escrita en una ficha** que puedes darle a alguien.

```ruby
saludar_proc = proc { |nombre| "Hola #{nombre}" }
saludar_lambda = ->(nombre) { "Hola #{nombre}" }

puts saludar_proc.call("Alice")
puts saludar_lambda.call("Bob")
puts saludar_proc.class
puts saludar_lambda.lambda?
```

```sortie
Hola Alice
Hola Bob
Proc
true
```

Una lambda es una especie de proc (su clase también es `Proc`), un poco más estricta.

> **Recuerda:** para empezar, usa lambdas. Se comportan como
> [métodos](../debuter/glossaire.md#methode), así que sin sorpresas.

## Escribir y llamar una lambda

La forma habitual es la «flecha»: `->(parámetros) { código }`. El resultado es el
último valor calculado, como en un método.

```ruby
cuadrado = ->(x) { x * x }
area = ->(ancho, alto) { ancho * alto }
hola = -> { "¡Hola!" }

puts cuadrado.call(4)
puts cuadrado.(5)
puts cuadrado[6]
puts area.call(3, 4)
puts hola.call
```

```sortie
16
25
36
12
¡Hola!
```

Las tres formas `call(…)`, `.(…)` y `[…]` hacen lo mismo. `call` es la más legible.

En varias líneas, escribe `do ... end` o llaves en varias líneas:

```ruby
precio_con_iva = lambda do |neto|
  iva = neto * 0.2
  neto + iva
end

puts precio_con_iva.call(50)
```

```sortie
60.0
```

## Las diferencias entre proc y lambda

### El número de argumentos

Una lambda comprueba el número de argumentos, como un método. Una proc es tolerante:
completa con `nil` o ignora los que sobran. `arity` da el número esperado.

```ruby
l = ->(a, b) { "#{a} y #{b}" }
pr = proc { |a, b| "#{a.inspect} y #{b.inspect}" }

puts l.arity
puts pr.call(1)
puts pr.call(1, 2, 3)

begin
  l.call(1)
rescue ArgumentError => e
  puts "Error: #{e.message}"
end
```

```sortie
2
1 y nil
1 y 2
Error: wrong number of arguments (given 1, expected 2)
```

### El sentido de return

En una lambda, `return` sale solo de la lambda. En una proc, `return` sale **del método
que la rodea**: suele ser una sorpresa.

```ruby
def con_lambda
  l = -> { return 1 }
  l.call
  "fin del método"
end

def con_proc
  pr = proc { return 1 }
  pr.call
  "fin del método"
end

puts con_lambda
puts con_proc
```

```sortie
fin del método
1
```

La proc hizo salir de `con_proc` enseguida: la última línea nunca se leyó.

| | Bloque | Proc | Lambda |
| --- | --- | --- | --- |
| Guardado en una variable | no | sí | sí |
| Comprueba el número de argumentos | no | no | sí |
| `return` sale de… | el método que lo rodea | el método que la rodea | la lambda |

## Pasar de una a otra con &

El `&` convierte en los dos sentidos:

- delante de un argumento, `&mi_lambda` le da la lambda al método **como bloque**;
- delante de un parámetro, `&bloque` **recupera** el bloque recibido en forma de proc.

```ruby
doblar = ->(x) { x * 2 }
p [1, 2, 3].map(&doblar)

def quedarse(lista, &regla)
  puts "La regla es una #{regla.class}"
  lista.select(&regla)
end

p quedarse([5, 12, 8, 20]) { |n| n > 10 }
```

```sortie
[2, 4, 6]
La regla es una Proc
[12, 20]
```

Ya conoces `map(&:upcase)`: `&` transforma el
[símbolo](../debuter/glossaire.md#symbole) `:upcase` en una proc que llama a `upcase`.

## Transformar un método en objeto

`method(:nombre)` atrapa un método existente y lo convierte en un objeto que se puede
pasar como una lambda.

```ruby
def gritar(texto)
  "¡" + texto.upcase + "!"
end

gritador = method(:gritar)
puts gritador.call("hola")
p ["sí", "no"].map(&gritador)
p ["3", "14"].map(&method(:Integer))
```

```sortie
¡HOLA!
["¡SÍ!", "¡NO!"]
[3, 14]
```

## Una lambda conserva sus variables

Una lambda, como un bloque, es una [closure](../debuter/glossaire.md#closure): se lleva
las variables visibles en el lugar donde se crea. Incluso ve sus cambios más tarde.

```ruby
contador = 0
incrementar = -> { contador += 1 }

incrementar.call
incrementar.call
incrementar.call
puts contador
```

```sortie
3
```

La lambda modifica la variable `contador` de verdad, no una copia.

## Un método que devuelve una función

Un método puede fabricar y devolver una lambda. Cada lambda conserva **su propio** valor
del parámetro: es una pequeña fábrica de funciones.

```ruby
def multiplicador(factor)
  ->(x) { x * factor }
end

doble = multiplicador(2)
triple = multiplicador(3)
puts doble.call(10)
puts triple.call(10)
p [1, 2, 3].map(&triple)
```

```sortie
20
30
[3, 6, 9]
```

Otro ejemplo útil: un contador cuya variable está escondida, que nadie más puede tocar.

```ruby
def nuevo_contador
  total = 0
  -> { total += 1 }
end

a = nuevo_contador
b = nuevo_contador
a.call
a.call
puts a.call
puts b.call
```

```sortie
3
1
```

## Componer funciones con >> y <<

`f >> g` crea una nueva función: «aplica `f`, luego `g` al resultado». `f << g` hace lo
contrario: «primero `g`, luego `f`». Es una cadena de montaje.

```ruby
limpiar = ->(s) { s.strip }
mayusculas = ->(s) { s.upcase }
enmarcar = ->(s) { "[#{s}]" }

procesar = limpiar >> mayusculas >> enmarcar
puts procesar.call("  hola  ")

sumar_uno = ->(x) { x + 1 }
doblar = ->(x) { x * 2 }
puts (sumar_uno >> doblar).call(5)
puts (sumar_uno << doblar).call(5)
```

```sortie
[HOLA]
12
11
```

`(sumar_uno >> doblar).call(5)` calcula (5 + 1) × 2. Con `<<`, es 5 × 2 + 1.

## Errores frecuentes

### Llamar a una lambda como a un método

```ruby norun
cuadrado = ->(x) { x * x }
puts cuadrado(3)
```

```text
main.rb:2:in `<main>': undefined method `cuadrado' for main (NoMethodError)

puts cuadrado(3)
     ^^^^^^^^
```

`cuadrado` es una variable que contiene una lambda, no un método. Llámala con
`cuadrado.call(3)` o `cuadrado.(3)`.

### Olvidar el & para pasar una lambda

```ruby norun
doblar = ->(x) { x * 2 }
p [1, 2].map(doblar)
```

```text
main.rb:2:in `map': wrong number of arguments (given 1, expected 0) (ArgumentError)

p [1, 2].map(doblar)
             ^^^^^^
	from main.rb:2:in `<main>'
```

`map` espera un bloque, no un argumento. El `&` transforma la lambda en bloque:
`[1, 2].map(&doblar)`.

### Un return en una proc devuelta por un método

```ruby norun
def fabricar
  proc { |x| return x * 2 }
end

doblar = fabricar
puts doblar.call(5)
```

```text
main.rb:2:in `block in fabricar': unexpected return (LocalJumpError)
	from main.rb:6:in `<main>'
```

El `return` de una proc quiere salir del método `fabricar`, pero ese método ya terminó.
Usa una lambda (`->(x) { x * 2 }`): su `return` se queda dentro de ella.

## Ponte a prueba

**Pregunta 1:** escribe una lambda `mas` que sume dos números, y luego llámala con 2 y 3.

> **Respuesta**
>
> ```ruby
> mas = ->(a, b) { a + b }
> puts mas.call(2, 3)
> ```
>
> ```sortie
> 5
> ```

**Pregunta 2:** ¿qué pasa si llamas a `->(a, b) { a + b }` con un solo argumento? ¿Y a
`proc { |a, b| a }`?

> **Respuesta**
>
> La lambda lanza un `ArgumentError` (wrong number of arguments). La proc no dice nada:
> `b` vale simplemente `nil`.

**Pregunta 3:** con `sumar = ->(n) { ->(x) { x + n } }`, ¿cuánto vale
`sumar.call(10).call(5)`?

> **Respuesta**
>
> `15`: `sumar.call(10)` devuelve una lambda que suma 10, llamada después con 5.
>
> ```ruby
> sumar = ->(n) { ->(x) { x + n } }
> puts sumar.call(10).call(5)
> ```
>
> ```sortie
> 15
> ```

## En resumen

- Un bloque acompaña a una llamada; una proc o una lambda es un bloque guardado en un
  objeto.
- `->(x) { ... }` crea una lambda; `call`, `.()` o `[]` la ejecutan.
- La lambda comprueba sus argumentos y su `return` se queda local: prefiérela.
- `&` pasa una lambda como bloque; `method(:nombre)` transforma un método en objeto.
- Una lambda conserva las variables que la rodean: se pueden fabricar funciones.
- `f >> g` y `f << g` encadenan funciones.

Siguiente: [enumeradores y pereza](enumerateurs.md).
