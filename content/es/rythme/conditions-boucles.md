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

Un programa a menudo tiene que decidir: ¿hace falta paraguas? ¿la persona es mayor
de edad? Para decidir, primero hace una pregunta, y la respuesta es sí o no.

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

En la vida eliges todo el rato según la situación: si llueve, coges un abrigo; si no,
una chaqueta. `if` permite al programa hacer el mismo tipo de elección.

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

Línea por línea:

- `edad = 20` da el nombre `edad` al valor 20;
- a la derecha del `=`, `if` hace la pregunta «¿20 es al menos 18?». La respuesta es
  `true`, así que el `if` da el valor que sigue a `then`: `"adulto"`;
- ese valor recibe el nombre `estado`;
- `print(estado)` muestra `adulto`.

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

A menudo no hay solo dos casos. Un número puede ser negativo, cero o positivo, y
quieres una palabra distinta para cada uno, como en un boletín de notas.

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

Línea por línea:

- `fn signo(n) =` define una función: recibe un número, llamado `n`;
- para `signo(-3)`, la primera pregunta `-3 < 0` es verdadera: la función da
  `"negativo"` y no lee el resto;
- para `signo(0)`, `0 < 0` es falsa, se pasa al `else if`: `0 == 0` es verdadera, así
  que `"cero"`;
- para `signo(8)`, las dos preguntas son falsas: se llega al último `else`, que da
  `"positivo"`.

Las condiciones se prueban de arriba abajo, y gana la primera verdadera.

La condición debe ser `true` o `false`. `if 1 then ...` es un error: nada es
«verdadero» por defecto, ni un número, ni un texto, ni una lista.

**Te toca:** escribe una función `saludo(hora)` que dé «Buenos días» si la hora es
anterior a 12, «Buenas tardes» si es anterior a 18, y «Buenas noches» si no. Muestra
`saludo(15)`.

> **Solución**
>
> ```rythme
> fn saludo(hora) =
>   if hora < 12 then "Buenos días"
>   else if hora < 18 then "Buenas tardes"
>   else "Buenas noches"
>
> print(saludo(15))
> ```
>
> ```sortie
> Buenas tardes
> ```

## unless, lo contrario de if

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

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

Línea por línea:

- `fn dia(n) = match n`: la función compara el número `n` con los patrones que siguen;
- para `dia(6)`, el primer patrón `6` encaja: el resultado es `"sábado"`;
- para `dia(2)`, ni `6` ni `7` encajan. `_` acepta todo: el resultado es
  `"un día entre semana"`;
- `print` muestra los dos resultados, separados por un espacio.

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

**Te toca:** escribe una función `estacion(mes)` que dé «invierno» para los meses 12,
1 y 2, «primavera» del 3 al 5, «verano» del 6 al 8, y «otoño» si no. Muestra
`estacion(1)`, `estacion(7)` y `estacion(10)`.

> **Solución**
>
> ```rythme
> fn estacion(mes) = match mes
>   12 | 1 | 2 -> "invierno"
>   3..5 -> "primavera"
>   6..8 -> "verano"
>   _ -> "otoño"
> end
>
> print(estacion(1), estacion(7), estacion(10))
> ```
>
> ```sortie
> invierno verano otoño
> ```

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
