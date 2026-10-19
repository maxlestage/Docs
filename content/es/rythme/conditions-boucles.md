# Condiciones y repetición

Esta página muestra cómo elegir entre varios casos con `if`, `unless` y `match`, y cómo repetir una acción sin bucle, con `each`, `map` y la recursión.

> **Objetivos:**
>
> - sabrás escribir una condición con `==`, `<`, `and`, `or` y `not`;
> - sabrás elegir un valor con `if ... then ... else`, y por qué el `else` es obligatorio;
> - sabrás comparar un valor con varios casos usando `match`;
> - sabrás repetir una acción sin bucle, con `each`, `each_with_index` y `map`;
> - sabrás escribir una repetición «mientras» con una función recursiva.

## Verdadero o falso: las comparaciones

Una [condición](../debuter/glossaire.md#condicion) es una pregunta cuya
respuesta es `true` (verdadero) o `false` (falso), un
[booleano](../debuter/glossaire.md#booleano). Se plantea con un operador de
comparación:

| Operador | Pregunta | Ejemplo | Resultado |
|----------|----------|---------|-----------|
| `==` | ¿igual? | `3 == 3` | `true` |
| `!=` | ¿distinto? | `3 != 3` | `false` |
| `<`, `>` | ¿menor, mayor? | `2 < 5` | `true` |
| `<=`, `>=` | ¿menor o igual, mayor o igual? | `5 >= 5` | `true` |

> **Atención:** `=` da un nombre, `==` compara. Son dos cosas distintas.

Para combinar condiciones, se usan palabras:

- `a and b`: verdadero si **las dos** son verdaderas;
- `a or b`: verdadero si **al menos una** es verdadera;
- `not a`: lo contrario de `a`.

```rythme
x = 7
print(x > 0 and x < 10, x < 0 or x == 7, not (x == 7))
```

```sortie
true true false
```

## if da un valor

`if` elige entre dos valores según una condición. En Rythme, `if` no es una
orden: es una **expresión**. Da un valor, como un cálculo. Por eso se puede
guardar en un nombre:

```rythme
edad = 20
estado = if edad >= 18 then "adulto" else "menor"
print(estado)
```

```sortie
adulto
```

La forma es siempre `if condición then valor else otro_valor`. Léelo así:
«si la edad es al menos 18, entonces "adulto", si no, "menor"».

## El else es obligatorio

Como `if` siempre da un valor, necesita los dos casos: ¿cuánto valdría
`estado` si la condición fuera falsa? Un `if` sin `else` es, por tanto, un
error (ver [los errores frecuentes](#errores-frecuentes)).

Cuando no hay nada que hacer en un caso, da el valor vacío `()`, que
significa «nada interesante»:

```rythme
x = 5
if x > 0 then print("positivo") else ()
```

```sortie
positivo
```

Una rama contiene una sola expresión. Para hacer varias cosas, agrúpalas en
un bloque `do ... end`: su última línea da el valor.

```rythme
fn clasificar(n) =
  if n > 100 then do
    print("¡un número grande!")
    "grande"
  end else "pequeño"

print(clasificar(500), clasificar(3))
```

```sortie
¡un número grande!
grande pequeño
```

## Encadenar condiciones

`else if` encadena varias condiciones. `then` y `else` pueden empezar una
línea nueva:

```rythme
fn signo(n) =
  if n < 0 then "negativo"
  else if n == 0 then "cero"
  else "positivo"

print(signo(-3), signo(0), signo(8))
```

```sortie
negativo cero positivo
```

Las condiciones se prueban de arriba abajo, y gana la primera verdadera.

La condición debe ser `true` o `false`. `if 1 then ...` es un error: nada es
«verdadero» por defecto, ni un número, ni un texto, ni una lista.

## unless, lo contrario de if

Como en Ruby, `unless` («a menos que») es lo contrario de `if`: da el primer
valor cuando la condición es **falsa**. También lleva siempre un `else`:

```rythme
existencias = 0
print(unless existencias > 0 then "agotado" else "disponible")
```

```sortie
agotado
```

`unless existencias > 0 then a else b` es exactamente
`if existencias > 0 then b else a`. Usa el que se lea mejor.

## Elegir con match

Cuando hay muchos casos, `match` es más claro que una serie de `else if`.
Compara un valor con varios **patrones**, en orden. El primero que encaja da
el resultado. `_` encaja con todo lo demás:

```rythme
fn dia(n) = match n
  6 -> "sábado"
  7 -> "domingo"
  _ -> "un día entre semana"
end

print(dia(6), dia(2))
```

```sortie
sábado un día entre semana
```

Cada línea se escribe `patrón -> resultado`, y `end` cierra el `match`.
Piensa en la clasificación del correo: cada carta va al primer casillero que
le corresponde.

Un patrón también puede ser un intervalo de números (`16..20`, con los dos
extremos incluidos), o varias posibilidades separadas por `|`:

```rythme
fn calificacion(nota) = match nota
  16..20 -> "sobresaliente"
  12..15 -> "notable"
  10 | 11 -> "aprobado"
  _ -> "a repasar"
end

print(calificacion(17), calificacion(13), calificacion(10), calificacion(8))
```

```sortie
sobresaliente notable aprobado a repasar
```

Un patrón incluso puede reconocer el principio de un texto:

```rythme
fn saludar("Dra. " + nombre) = "Hola, doctora {nombre}"
fn saludar(nombre) = "Hola, {nombre}"

print(saludar("Dra. Grace"), "/", saludar("Ada"))
```

```sortie
Hola, doctora Grace / Hola, Ada
```

`match` sabe hacer mucho más: ver [tus propios tipos y match](types.md).

## Sin bucles

Rythme no tiene ni `for` ni `while`. Si escribes uno, Rythme te explica qué
hacer en su lugar (ver [los errores frecuentes](#errores-frecuentes)).

En lugar de decir «para cada elemento, haz esto», se dice «haz esto con toda
la lista». Es como decir «lava todos los platos» en lugar de «coge el plato
1, lávalo, coge el plato 2...». Estas son las herramientas.

## Hacer algo con cada elemento usando each

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

## Contar de 2 en 2, o hacia atrás

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

## Transformar cada elemento con map

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

Aquí, la función tiene dos definiciones: una para `0` (el caso de parada) y
otra para los demás números. `cuenta_atras(3)` muestra 3 y luego llama a
`cuenta_atras(2)`, y así sucesivamente hasta 0. Ver
[las funciones](fonctions.md#definir-por-casos).

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

### Olvidar el else

```rythme norun
edad = 20
print(if edad >= 18 then "adulto")
```

```text
error[E019]: este `if` no tiene `else`
 --> main.ry:2:7
  |
2 | print(if edad >= 18 then "adulto")
  |       ^^ todo `if` debe tener un `else`
  |
2 | print(if edad >= 18 then "adulto")
  |                                  - se esperaba `else` aquí
  |
  = ayuda: en Rythme, `if` siempre da un valor, así que necesita los dos casos:
        if edad >= 18 then "adulto" else "menor"
  = más sobre este error: `rythme explain E019`
```

Añade el otro caso: `else "menor"`, o `else ()` si no hay nada que
hacer.

### Comparar con = en lugar de ==

```rythme norun
x = 5
if x = 5 then print("cinco") else ()
```

```text
error[E001]: se esperaba `then`, pero se encontró `=`
 --> main.ry:2:6
  |
2 | if x = 5 then print("cinco") else ()
  |      ^ se esperaba `then` aquí
  |
  = ayuda: para comparar dos valores, usa `==`
  = ayuda: un `if` se escribe: `if condición then valor else otro_valor`
  = más sobre este error: `rythme explain E001`
```

Para comparar, dos signos de igual: `if x == 5 then ...`.

### Encadenar dos comparaciones

```rythme norun
nota = 15
print(if 10 <= nota <= 20 then "ok" else "no")
```

```text
error[E001]: las comparaciones no se pueden encadenar
 --> main.ry:2:21
  |
2 | print(if 10 <= nota <= 20 then "ok" else "no")
  |                     ^^
  |
  = ayuda: combínalas con `and`: `a < b and b < c`
  = más sobre este error: `rythme explain E001`
```

Escribe las dos preguntas y únelas: `10 <= nota and nota <= 20`. `&&` y
`||` tampoco existen: escribe `and` y `or`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto vale `unless 3 > 5 then "a" else "b"`?

> **Respuesta**
>
> `"a"`. La condición `3 > 5` es falsa, y `unless` da el primer valor
> cuando la condición es falsa.

**Pregunta 2:** ¿qué muestra este programa?

```rythme norun
fn tamano(n) = match n
  0 -> "nada"
  1..9 -> "pequeño"
  _ -> "grande"
end
print(tamano(0), tamano(7), tamano(42))
```

> **Respuesta**
>
> `nada pequeño grande`. Cada número va al primer patrón que le
> corresponde.

**Pregunta 3:** ¿cómo mostrar los números pares del 2 al 10, sin bucle?

> **Respuesta**
>
> Por ejemplo `2..10 |> step_by(2) |> each(fn(n) = print(n))`, o
> `2..10 |> filter(is_even) |> each(fn(n) = print(n))`.

## En resumen

- `==`, `!=`, `<`, `>`, `<=`, `>=` comparan; `and`, `or` y `not` combinan
  las condiciones.
- `if condición then a else b` da un valor, y el `else` es obligatorio;
  `unless` es su contrario.
- `match` compara un valor con patrones, en orden; `_` atrapa todo lo
  demás.
- No hay bucles: `each` hace algo con cada elemento, `each_with_index` da
  también su posición, `map` transforma cada elemento.
- `step_by` cuenta a saltos, `reverse` cuenta hacia atrás.
- Para «mientras», se escribe una función que se llama a sí misma, con un
  caso en el que se detiene.

Siguiente: [las funciones](fonctions.md). Compara con las condiciones y
bucles de [Ruby](../ruby/conditions-boucles.md) y de
[Rust](../rust/conditions-boucles.md).
