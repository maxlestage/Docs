# Las condiciones

Esta página muestra cómo hacer una pregunta con una comparación y luego elegir entre varios casos con `if`, `unless` y `match`.

> **Objetivos:**
>
> - sabrás escribir una condición con `==`, `<`, `and`, `or` y `not`;
> - sabrás elegir un valor con `if ... then ... else`, y por qué el `else` es obligatorio;
> - sabrás comparar un valor con varios casos usando `match`.

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

## Errores frecuentes

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

**Pregunta 3:** ¿cuánto vale `if 4 >= 4 and not (4 == 5) then "sí" else "no"`?

> **Respuesta**
>
> `"sí"`. `4 >= 4` es verdadera; `4 == 5` es falsa, así que `not (4 == 5)` es
> verdadera. Los dos lados del `and` son verdaderos: el `if` da el valor que sigue a
> `then`.

## En resumen

- `==`, `!=`, `<`, `>`, `<=`, `>=` comparan; `and`, `or` y `not` combinan
  las condiciones.
- `if condición then a else b` da un valor, y el `else` es obligatorio;
  `unless` es su contrario.
- Con `else if`, las condiciones se prueban de arriba abajo: gana la primera
  verdadera.
- `match` compara un valor con patrones, en orden; `_` atrapa todo lo
  demás.

Siguiente: [los bucles](boucles.md). Compara con las condiciones de
[Ruby](../ruby/conditions-boucles.md) y de [Rust](../rust/conditions-boucles.md).
