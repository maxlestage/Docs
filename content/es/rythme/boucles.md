# Los bucles

Esta página muestra cómo repetir una acción sin bucle, con `each`, `each_with_index`, `map` y la recursión.

> **Objetivos:**
>
> - sabrás por qué Rythme no tiene ni `for` ni `while`, y qué escribir en su lugar;
> - sabrás repetir una acción sin bucle, con `each`, `each_with_index` y `map`;
> - sabrás escribir una repetición «mientras» con una función recursiva.

Este capítulo sigue a [las condiciones](conditions-boucles.md): ya sabes elegir un valor con `if` y `match`, y ahora vas a aprender a repetir una acción.

## Sin bucles

Rythme no tiene ni `for` ni `while`. Si escribes uno, Rythme te explica qué
hacer en su lugar (ver [los errores frecuentes](#errores-frecuentes)).

En lugar de decir «para cada elemento, haz esto», se dice «haz esto con toda
la lista». Es como decir «lava todos los platos» en lugar de «coge el plato
1, lávalo, coge el plato 2...». Estas son las herramientas.

## Hacer algo con cada elemento usando each

Imagina que te piden mostrar cada producto de una lista de la compra. Escribir un
`print` por producto sería largo y pesado. `each` hace ese trabajo por ti.

`each` llama a una función para cada elemento. Es el sustituto del bucle que
muestra cosas. Aquí, `1..3` es el **intervalo** de los números del 1 al 3, es
decir, la lista `[1, 2, 3]`, y `fn(i) = ...` es una pequeña función sin
nombre (ver [las funciones](fonctions.md#las-funciones-anonimas)):

```rythme
1..3 |> each(fn(i) = print("Vuelta número {i}"))
```

```sortie
Vuelta número 1
Vuelta número 2
Vuelta número 3
```

Lo mismo, al estilo de Ruby, con un **bloque** `do |nombre| ... end`:
`|nombre|` da un nombre al elemento actual.

```rythme
["Ada", "Alan"].each do |nombre|
  puts "¡Hola, #{nombre}!"
end
```

```sortie
¡Hola, Ada!
¡Hola, Alan!
```

**Te toca:** escribe un programa que muestre «Para comprar: …» para cada producto de
la lista `["pan", "leche", "arroz"]`, con `each`.

> **Solución**
>
> ```rythme
> ["pan", "leche", "arroz"] |> each(fn(producto) = print("Para comprar: {producto}"))
> ```
>
> ```sortie
> Para comprar: pan
> Para comprar: leche
> Para comprar: arroz
> ```

## Contar de 2 en 2, o hacia atrás

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

`a..b` cuenta de 1 en 1, subiendo. `step_by(n)` se queda con un elemento de
cada `n`, y `reverse` da la vuelta a la lista:

```rythme
print(0..10 |> step_by(2))
print(reverse(1..5))
print(5..1)
```

```sortie
[0, 2, 4, 6, 8, 10]
[5, 4, 3, 2, 1]
[]
```

> **Atención:** `5..1` está vacío. Para contar hacia atrás, escribe
> `reverse(1..5)`.

## Conocer la posición con each_with_index

A veces quieres numerar lo que recorres, como los pasos de una receta o una lista de
la compra numerada.

`each_with_index` da a la función el elemento **y** su posición, empezando
por 0:

```rythme
["pan", "leche", "huevos"].each_with_index do |producto, i|
  print("{i + 1}. {producto}")
end
```

```sortie
1. pan
2. leche
3. huevos
```

Línea por línea:

- la lista contiene tres productos; `each_with_index` los toma uno a uno;
- en la primera vuelta, `producto` vale `"pan"` e `i` vale 0; en la segunda,
  `"leche"` y 1; en la tercera, `"huevos"` y 2;
- `print` muestra `i + 1` para que la lista empiece en 1, como se cuenta normalmente.

## Transformar cada elemento con map

A veces quieres cambiar toda una lista de golpe: duplicar cada precio, elevar cada
número al cuadrado. Es como una máquina en la que cada objeto entra por un lado y
sale transformado por el otro.

`map` fabrica una lista **nueva**, con el resultado de la función para cada
elemento. Es el sustituto del bucle que llena una lista:

```rythme
cuadrados = 1..5 |> map(fn(x) = x * x)
print(cuadrados)
```

```sortie
[1, 4, 9, 16, 25]
```

`each` sirve para **hacer** algo (mostrar); `map` sirve para **calcular**
una lista nueva.

## Repetir con la recursión

Algunas repeticiones no tienen una lista ya preparada: se repite «mientras» no se haya
terminado, como una cuenta atrás que baja hasta cero.

Para un bucle «mientras», escribe una función que se llame a sí misma: es la
[recursión](../debuter/glossaire.md#recursion). Siempre hace falta un caso
en el que se detenga; si no, no se detiene nunca:

```rythme
fn cuenta_atras(0) = print("¡Despegue!")
fn cuenta_atras(n)
  print(n)
  cuenta_atras(n - 1)
end

cuenta_atras(3)
```

```sortie
3
2
1
¡Despegue!
```

Línea por línea:

- `fn cuenta_atras(0) = ...`: si el número recibido es 0, la función muestra
  «¡Despegue!» y se detiene ahí;
- `fn cuenta_atras(n)`: para cualquier otro número, muestra `n` y luego se llama a sí
  misma con `n - 1`;
- `cuenta_atras(3)` lo pone todo en marcha: muestra 3, luego la llamada sigue con 2,
  después 1, y por último 0, que muestra «¡Despegue!».

Aquí, la función tiene dos definiciones: una para `0` (el caso de parada) y
otra para los demás números. `cuenta_atras(3)` muestra 3 y luego llama a
`cuenta_atras(2)`, y así sucesivamente hasta 0. Ver
[las funciones](fonctions.md#definir-por-casos).

**Te toca:** escribe una función recursiva `aplaudir(n)` que muestre «clap» `n`
veces y luego «¡Bravo!». Llama a `aplaudir(3)`.

> **Solución**
>
> ```rythme
> fn aplaudir(0) = print("¡Bravo!")
> fn aplaudir(n)
>   print("clap")
>   aplaudir(n - 1)
> end
>
> aplaudir(3)
> ```
>
> ```sortie
> clap
> clap
> clap
> ¡Bravo!
> ```

## Errores frecuentes

### Escribir un bucle for

```rythme norun
numeros = [1, 2, 3]
for n in numeros
  print(n)
end
```

```text
error[E001]: `for` no se usa en Rythme
 --> main.ry:2:1
  |
2 | for n in numeros
  | ^^^ viene de otro lenguaje
  |
  = ayuda: Rythme no tiene bucles. Trabaja con la lista entera en su lugar:
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      o escribe una función recursiva. Consulta `rythme help functional`.
  = más sobre este error: `rythme explain E001`
```

Escribe `numeros.each do |n| print(n) end`, o
`numeros |> each(fn(n) = print(n))`. `while` da el mismo error.

### Usar each en lugar de map

```rythme norun
cuadrados = 1..3 |> each(fn(x) = x * x)
print(cuadrados |> map(fn(x) = x + 1))
```

```text
error[E005]: `map` espera una lista, un conjunto, una Option o un Result como argumento 1, pero recibió un valor de tipo Unit
 --> main.ry:2:7
  |
2 | print(cuadrados |> map(fn(x) = x + 1))
  |       ^^^^^^^^^ tipo de valor incorrecto
  |
  = ayuda: uso: `map(list, f)`
  = más sobre este error: `rythme explain E005`
```

`each` hace algo con cada elemento, pero no fabrica ninguna lista: da el valor vacío
`()`, de tipo `Unit`. Para calcular una lista nueva, usa `map`:
`cuadrados = 1..3 |> map(fn(x) = x * x)`.

## Ponte a prueba

**Pregunta 1:** ¿cómo mostrar los números pares del 2 al 10, sin bucle?

> **Respuesta**
>
> Por ejemplo `2..10 |> step_by(2) |> each(fn(n) = print(n))`, o
> `2..10 |> filter(is_even) |> each(fn(n) = print(n))`.

**Pregunta 2:** ¿qué hace `["a", "b"] |> map(fn(x) = x + "!")`, y en qué se diferencia de `each`?

> **Respuesta**
>
> Fabrica una lista nueva, `["a!", "b!"]`. `each` llama a la función para cada
> elemento pero no da ninguna lista: sirve para **hacer** algo (mostrar), `map`
> sirve para **calcular**.

**Pregunta 3:** en la cuenta atrás, ¿qué pasa si quitas la línea
`fn cuenta_atras(0) = print("¡Despegue!")`?

> **Respuesta**
>
> La función ya no se detiene: después de 1, muestra 0, luego -1, -2… sin fin. Una
> función recursiva siempre necesita un caso en el que se detenga.

## En resumen

- No hay ni `for` ni `while`: se trabaja con la lista entera.
- `each` hace algo con cada elemento, `each_with_index` da también su
  posición, `map` transforma cada elemento y da una lista nueva.
- `step_by` cuenta a saltos, `reverse` cuenta hacia atrás.
- Para «mientras», se escribe una función que se llama a sí misma, con un
  caso en el que se detiene.

Siguiente: [las funciones](fonctions.md).
