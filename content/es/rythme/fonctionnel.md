# Pensar de forma funcional

Esta página explica la forma de pensar de Rythme: valores que no cambian, cadenas de funciones con `|>` o con puntos, bloques al estilo de Ruby, y la recursión.

> **Objetivos:**
>
> - sabrás por qué una función de Rythme nunca modifica lo que recibe;
> - sabrás encadenar pasos con `|>` o con puntos;
> - sabrás dar un bloque `{ |x| ... }` o `do |x| ... end` a una función;
> - sabrás sustituir cada tipo de bucle por `map`, `filter`, `fold`, `find`...;
> - sabrás escribir una función recursiva con un acumulador.

## Valores que no cambian

Una función de Rythme nunca modifica lo que recibe. Da un valor
**nuevo**:

```rythme
numeros = [3, 1, 2]
ordenados = sort(numeros)
print(numeros, ordenados)
```

```sortie
[3, 1, 2] [1, 2, 3]
```

Así es más fácil de seguir: un valor al que has dado nombre conserva su
sentido hasta el final del programa. Y una función siempre da el mismo
resultado para los mismos argumentos, como una tecla de calculadora: `√9`
vale 3 hoy, mañana, y en cualquier orden.

Esto se llama **programación funcional**: construyes un programa juntando
funciones, en lugar de modificar valores paso a paso.

## Encadenar con |>

`|>` se lee «luego». `x |> f` quiere decir `f(x)`. Y `x |> f(a)` quiere
decir `f(x, a)`: el valor se convierte en el **primer argumento**. Así lees
el cálculo en el orden en que se hace:

```rythme
resultado = [1, 2, 3, 4, 5, 6]
  |> filter(is_even)
  |> map(fn(x) = x * x)
  |> sum
print(resultado)
```

```sortie
56
```

Paso a paso: `[1, 2, 3, 4, 5, 6]`, luego nos quedamos con los pares
`[2, 4, 6]`, luego los elevamos al cuadrado `[4, 16, 36]`, luego los
sumamos: 56. Sin `|>`, habría que leer al revés: `sum(map(filter(...), ...))`.

## Encadenar con puntos

El mismo cálculo, al estilo de Ruby, con llamadas en estilo método. Cada
línea que empieza por `.` continúa la anterior:

```rythme
resultado = [1, 2, 3, 4, 5, 6]
  .select(even?)
  .map(fn(x) = x * x)
  .sum
print(resultado)
```

```sortie
56
```

`select` y `even?` son los nombres Ruby de `filter` y `is_even`. Elige el
estilo que prefieras, y mantenlo en todo tu programa.

## Los bloques al estilo de Ruby

En lugar de escribir `fn(x) = ...`, puedes dar un **bloque** al final de
una llamada. Hay dos formas:

- `do |x| ... end`, a menudo en varias líneas;
- `{ |x| ... }`, a menudo en una línea.

Entre las barras `|x|`, pones nombre al elemento recibido.

```rythme
cuadrados = [1, 2, 3].map { |x| x * x }
print(cuadrados)

[1, 2, 3].each do |x|
  doble = x * 2
  puts "#{x} por 2 = #{doble}"
end
```

```sortie
[1, 4, 9]
1 por 2 = 2
2 por 2 = 4
3 por 2 = 6
```

Un bloque es exactamente una función anónima: `xs.map { |x| x * x }` es
`map(xs, fn(x) = x * x)`.

## map, filter y reject

- `map` transforma cada elemento;
- `filter` (o `select`) conserva aquellos para los que la función responde
  `true`;
- `reject` quita aquellos para los que la función responde `true`.

```rythme
palabras = ["gato", "elefante", "pez", "jirafa"]
print(palabras.map { |p| p.upper })
print(palabras.filter { |p| len(p) > 3 })
print(palabras.reject { |p| len(p) > 3 })
```

```sortie
["GATO", "ELEFANTE", "PEZ", "JIRAFA"]
["gato", "elefante", "jirafa"]
["pez"]
```

Piensa en un colador: `filter` conserva lo que se queda en el colador,
`reject` conserva lo que pasa a través.

## fold y reduce

`fold(lista, inicio, f)` recorre la lista guardando un **acumulador**, un
valor que crece con cada elemento. `f` recibe el acumulador y el elemento,
y da el nuevo acumulador. `reduce` es su nombre Ruby:

```rythme
total = fold([1, 2, 3, 4], 0, fn(acc, x) = acc + x)
mas_larga = ["kiwi", "plátano", "higo"].reduce("") do |mejor, p|
  if len(p) > len(mejor) then p else mejor
end
print(total, mas_larga)
```

```sortie
10 plátano
```

Para `total`, el acumulador vale 0 al principio, luego 0 + 1 = 1, luego
1 + 2 = 3, luego 3 + 3 = 6, luego 6 + 4 = 10. `scan` muestra estos pasos:

```rythme
print(scan([1, 2, 3, 4], 0, fn(acc, x) = acc + x))
```

```sortie
[1, 3, 6, 10]
```

## Sustituir los bucles

| En Python o en Ruby | En Rythme |
|---------------------|-----------|
| un bucle que muestra | `each` |
| un bucle que llena una lista | `map` |
| un bucle con un `if` que conserva | `filter`, `reject` |
| un bucle que suma | `sum`, `fold` |
| un bucle que cuenta | `count` |
| un bucle que busca | `find`, `any`, `all` |
| `break` | `take_while` |
| `while` | una función recursiva |

```rythme
numeros = [4, 8, 15, 16, 23, 42]
print(find(numeros, is_odd), any(numeros, fn(x) = x > 40), all(numeros, fn(x) = x > 0))
print(take_while(numeros, is_even), count(numeros, is_even))
```

```sortie
Some(15) true true
[4, 8] 4
```

- `find` da el primer elemento que cumple, dentro de una `Option`;
- `any` responde `true` si **al menos un** elemento cumple, `all` si
  **todos** cumplen;
- `take_while` toma los elementos mientras cumplen, y luego se detiene;
- `count` cuenta los que cumplen.

## Repetir con un acumulador

Una función puede llamarse a sí misma (la
[recursión](../debuter/glossaire.md#recursion)). Para que pueda repetirse
**millones de veces**, la llamada a sí misma debe ser lo último de todo
que hace. El truco: guardas el resultado parcial en un parámetro extra, el
**acumulador**.

```rythme
fn sumar([], acc) = acc
fn sumar([x, ..resto], acc) = sumar(resto, acc + x)

print(sumar(1..1000000, 0))
```

```sortie
500000500000
```

Aquí, `sumar` recibe la lista y el total ya calculado. `[x, ..resto]`
separa el primer elemento del resto. Cuando la lista está vacía, el total
es la respuesta.

> **Consejo:** cuando existe una función de listas (`sum`, `fold`, `map`),
> prefiérela: no tiene límite y se lee mejor.

## Un bucle mientras

Esta es la sucesión de Collatz: mientras `n` no valga 1, lo divides entre 2
si es par; si no, calculas `3 * n + 1`. Cuentas los pasos:

```rythme
fn collatz(1, pasos) = pasos
fn collatz(n, pasos) =
  if n % 2 == 0 then collatz(n // 2, pasos + 1)
  else collatz(3 * n + 1, pasos + 1)

print(collatz(27, 0))
```

```sortie
111
```

El «mientras» se convierte en: «si no has llegado, vuelve a empezar con el
valor siguiente». El caso `collatz(1, pasos)` es la condición de parada.

## Errores frecuentes

### Olvidar fn en map

```rythme norun
print([1, 2, 3] |> map(x * 2))
```

```text
error[E003]: nombre desconocido `x`
 --> main.ry:1:24
  |
1 | print([1, 2, 3] |> map(x * 2))
  |                        ^ no definido
  |
  = ayuda: defínelo antes de usarlo: `x = ...` o `fn x(...) = ...`
  = más sobre este error: `rythme explain E003`
```

`map` espera una **función**, no un cálculo. Escribe
`map(fn(x) = x * 2)`, o `[1, 2, 3].map { |x| x * 2 }`.

### Poner los argumentos de fold en desorden

```rythme norun
total = fold([1, 2, 3], fn(acc, x) = acc + x, 0)
print(total)
```

```text
error[E005]: `fold` espera una función como argumento 3, pero recibió Int 0
 --> main.ry:1:47
  |
1 | total = fold([1, 2, 3], fn(acc, x) = acc + x, 0)
  |                                               ^ tipo de valor incorrecto
  |
  = ayuda: uso: `fold(list, start, f)`
  = más sobre este error: `rythme explain E005`
```

El orden es: la lista, el valor inicial y luego la función.
`fold([1, 2, 3], 0, fn(acc, x) = acc + x)`.

### Un filtro que no responde true o false

```rythme norun
print([1, 2, 3].filter { |x| x % 2 })
```

```text
error[E005]: la función dada a `filter` debe devolver true o false, pero devolvió un valor de tipo Int
 --> main.ry:1:24
  |
1 | print([1, 2, 3].filter { |x| x % 2 })
  |                        ^^^^^^^^^^^^^ devuelve un valor de tipo Int
  |
  = ayuda: uso: `filter(list, f)`
  = más sobre este error: `rythme explain E005`
```

En Python, `1` cuenta como verdadero. En Rythme no: haz una pregunta de
verdad, `x % 2 == 1`.

### Calcular justo después de una cadena

```rythme norun
print([1, 2, 3] |> sum + 1)
```

```text
error[E001]: `+` no puede seguir directamente a un paso `|>`
 --> main.ry:1:24
  |
1 | print([1, 2, 3] |> sum + 1)
  |                        ^
  |
  = ayuda: `|>` va después de los cálculos: pon la cadena entre paréntesis,
      como `(x |> f) + 1`, o continúala con una función: `x |> f |> fn(v) = v + 1`
  = más sobre este error: `rythme explain E001`
```

Pon la cadena entre paréntesis: `([1, 2, 3] |> sum) + 1`.

## Ponte a prueba

**Pregunta 1:** reescribe `upper(trim("  hola "))` con `|>`.

> **Respuesta**
>
> `"  hola " |> trim |> upper`. Se lee en orden: primero `trim`, luego
> `upper`.

**Pregunta 2:** ¿cuánto vale `fold([2, 3, 4], 1, fn(acc, x) = acc * x)`?

> **Respuesta**
>
> `24`: 1 × 2 = 2, luego 2 × 3 = 6, luego 6 × 4 = 24.

**Pregunta 3:** ¿qué función sustituye a un bucle que busca el primer
número negativo de una lista?

> **Respuesta**
>
> `find(numeros, fn(x) = x < 0)`. Da `Some(numero)`, o `None` si no hay
> ninguno.

## En resumen

- Una función nunca modifica lo que recibe: da un valor nuevo.
- `x |> f(a)` quiere decir `f(x, a)`: lees el cálculo en orden.
- Un bloque `{ |x| ... }` o `do |x| ... end` es una función sin nombre.
- `map`, `filter`, `reject`, `fold`, `find` y `count` sustituyen a los
  bucles.
- Para repetir «mientras», una función recursiva, con un acumulador si es
  posible.

Siguiente: [la recursión](recursion.md)
