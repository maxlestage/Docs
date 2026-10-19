# Ejercicios resueltos: segunda serie

Doce ejercicios nuevos para seguir practicando Rythme, del más fácil al más difícil, cada uno con una pista, una solución comprobada y una explicación paso a paso.

## Cómo hacer estos ejercicios

Esta serie completa [los ejercicios para empezar bien](exercices-faciles.md)
y [la primera serie de ejercicios resueltos](exercices.md). Ningún ejercicio
se repite. Cada ejercicio tiene un nivel:

- ★☆☆ fácil: unas pocas líneas, una sola idea;
- ★★☆ medio: hay que combinar dos o tres ideas;
- ★★★ más difícil: tómate tu tiempo, y un lápiz.

Para cada ejercicio, sigue el mismo método:

```text
1. Lee el enunciado y el resultado esperado.
2. En papel: ¿qué valores entran? ¿qué valor debe salir?
3. Escribe el código en exo.ry, y luego:  rythme run exo.ry
4. Compara con el resultado esperado.
5. ¿Atascado? Lee la pista. ¿Sigues atascado? Abre la solución.
6. Lee la tabla paso a paso de la solución, aunque lo hayas conseguido.
```

Las tablas paso a paso siguen el valor de cada nombre, etapa por etapa.
Es como ver una película fotograma a fotograma: ves exactamente en qué
momento cambian las cosas. La página [rythme etapes](etapes.md) muestra cómo
Rythme puede hacer este trabajo por ti.

> **Recuerda:** inténtalo primero, aunque sean cinco minutos. Una solución
> leída sin haber buscado se olvida rápido; una solución encontrada después
> de buscar se queda.

## Nivel ★☆☆: una sola idea

### Ejercicio 1: El termómetro estadounidense

Nivel: ★☆☆

En Estados Unidos, la temperatura se da en grados Fahrenheit. Para pasar
de grados Celsius a grados Fahrenheit, se multiplica por 9, se divide
entre 5 y luego se suma 32. Escribe `fahrenheit(c)` y convierte toda
la lista `[0, 20, 37, 100]` de una vez:

```text
[32.0, 68.0, 98.6, 212.0]
```

> **Pista:** empieza por la función para **un solo** número:
> `fn fahrenheit(c) = ...`. Después, `map` la aplica a cada elemento de la
> lista. Ver [pensar en funcional](fonctionnel.md).

> **Solución**
>
> ```rythme
> fn fahrenheit(c) = c * 9 / 5 + 32
>
> print([0, 20, 37, 100] |> map(fahrenheit))
> ```
>
> ```sortie
> [32.0, 68.0, 98.6, 212.0]
> ```
>
> Por qué funciona: `map` es como una máquina de sellar. Cada número
> pasa por debajo, sale transformado, y la nueva lista conserva el mismo
> orden. El `/` siempre da un número decimal, de ahí el `.0`.
>
> | Paso | c | c * 9 | / 5 | + 32 |
> |------|---|-------|-----|------|
> | 1 | 0 | 0 | 0.0 | 32.0 |
> | 2 | 20 | 180 | 36.0 | 68.0 |
> | 3 | 37 | 333 | 66.6 | 98.6 |
> | 4 | 100 | 900 | 180.0 | 212.0 |

Prueba: cambia `/` por `//` (la división entera). Obtienes
`[32, 68, 98, 212]`: el `.6` de 98.6 ha desaparecido, porque `333 // 5` vale 66.

### Ejercicio 2: Las iniciales

Nivel: ★☆☆

Escribe `iniciales(nombre)` que da las iniciales de un nombre, en mayúsculas,
cada una seguida de un punto:

```text
A. L.
J. P. S.
```

para `"ada lovelace"` y `"Jean Paul Sartre"`.

> **Pista:** tres pasos que encadenar con `|>`: `split` corta el nombre
> en palabras; `map` transforma cada palabra en `"A."`; `join(" ")` vuelve a
> pegar los trozos. La primera letra de una palabra `p` se escribe `p[0]`.

> **Solución**
>
> ```rythme
> fn inicial(palabra) = upper(palabra[0]) + "."
>
> fn iniciales(nombre) = split(nombre) |> map(inicial) |> join(" ")
>
> print(iniciales("ada lovelace"))
> print(iniciales("Jean Paul Sartre"))
> ```
>
> ```sortie
> A. L.
> J. P. S.
> ```
>
> Por qué funciona: cada `|>` pasa el resultado al paso siguiente,
> como en una cadena de montaje.
>
> ```text
> "ada lovelace"
>    │ split
>    ▼
> ["ada", "lovelace"]
>    │ map(inicial)
>    ▼
> ["A.", "L."]
>    │ join(" ")
>    ▼
> "A. L."
> ```

Prueba: cambia `join(" ")` por `join("")`. Obtienes `A.L.`, sin
espacio.

### Ejercicio 3: Contar las vocales

Nivel: ★☆☆

¿Cuántas vocales (a, e, i, o, u, y) hay en la palabra
`anticonstitucionalmente`? Resultado esperado:

```text
10
```

> **Pista:** `chars(palabra)` da la lista de las letras. `count(lista, f)`
> cuenta los elementos para los que `f` responde `true`. Para saber si una
> letra es una vocal, búscala en el texto `"aeiouy"` con
> `contains`.

> **Solución**
>
> ```rythme
> fn vocal?(letra) = contains("aeiouy", letra)
>
> palabra = "anticonstitucionalmente"
> print(count(chars(palabra), vocal?))
> ```
>
> ```sortie
> 10
> ```
>
> Por qué funciona: `vocal?` responde a una pregunta con sí o no,
> de ahí el `?` al final de su nombre. `count` hace la pregunta a cada
> letra y cuenta los «sí». Es como un portero que cuenta las
> personas con una pulsera.
>
> | Letras | a | n | t | i | c | o | n | s | t | i | ... |
> |--------|---|---|---|---|---|---|---|---|---|---|-----|
> | vocal? | sí | no | no | sí | no | sí | no | no | no | sí | ... |
> | cuenta | 1 | 1 | 1 | 2 | 2 | 3 | 3 | 3 | 3 | 4 | ... |

Prueba: cambia la palabra por `"Rythme"`. Obtienes `2`: la `y` y
la `e`. Con `"CASA"`, obtienes `0`, porque `"aeiouy"` solo contiene
minúsculas. Añade `lower` para corregirlo: `chars(lower(palabra))`.

### Ejercicio 4: La cesta

Nivel: ★☆☆

Una cesta de la compra contiene artículos, cada uno guardado en un registro con
un nombre y un precio. Muestra el número de artículos y el total a pagar:

```text
3 artículos, total: 12 €
```

> **Pista:** `len` cuenta los artículos. `sum(lista, f)` suma los
> valores que `f` da para cada elemento: aquí, el campo `precio`.

> **Solución**
>
> ```rythme
> cesta = [
>   {articulo: "pan", precio: 2},
>   {articulo: "queso", precio: 7},
>   {articulo: "zumo", precio: 3}
> ]
> total = sum(cesta, fn(a) = a.precio)
> print("{len(cesta)} artículos, total: {total} €")
> ```
>
> ```sortie
> 3 artículos, total: 12 €
> ```
>
> Por qué funciona: `fn(a) = a.precio` es una pequeña función sin nombre
> que «abre» cada registro y saca el precio. `sum` suma
> estos precios uno a uno.
>
> | Paso | a.articulo | a.precio | total acumulado |
> |------|------------|----------|-----------------|
> | inicio | | | 0 |
> | 1 | pan | 2 | 2 |
> | 2 | queso | 7 | 9 |
> | 3 | zumo | 3 | 12 |

Prueba: añade `{articulo: "manzanas", precio: 4}` a la cesta. El mensaje
se convierte en `4 artículos, total: 16 €`, sin cambiar nada más.

> **Recuerda:** para una lista, piensa primero «¿qué hago con **un**
> elemento?». Escribe esa pequeña función y luego entrega la lista a
> `map`, `count` o `sum`.

## Nivel ★★☆: combinar dos o tres ideas

### Ejercicio 5: FizzBuzz

Nivel: ★★☆

Es un juego de colegio: se cuenta en voz alta, pero se dice «Fizz» para un
múltiplo de 3, «Buzz» para un múltiplo de 5 y «FizzBuzz» para un
múltiplo de los dos. Escribe `fizzbuzz(n)` con guardas y luego muestra las
respuestas de 1 a 15 en una línea:

```text
1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
```

> **Pista:** un número es múltiplo de 3 si `n % 3 == 0`. Escribe una
> definición por casos, con una guarda por caso (ver
> [las guardas](fonctions.md#las-guardas)). El último caso da el número como
> texto: `text(n)`.

> **Pista:** el orden de los casos importa. Rythme prueba las definiciones de
> arriba abajo: el caso «múltiplo de 15» debe ir primero.

> **Solución**
>
> ```rythme
> fn fizzbuzz(n) if n % 15 == 0 = "FizzBuzz"
> fn fizzbuzz(n) if n % 3 == 0 = "Fizz"
> fn fizzbuzz(n) if n % 5 == 0 = "Buzz"
> fn fizzbuzz(n) = text(n)
>
> print(1..15 |> map(fizzbuzz) |> join(" "))
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```
>
> Por qué funciona: un número múltiplo de 3 **y** de 5 es un múltiplo
> de 15. Por eso se prueba 15 primero. Para 15, estos son los casos probados:
>
> | Caso probado | Guarda | Resultado |
> |--------------|--------|-----------|
> | 1 | `15 % 15 == 0` → `true` | `"FizzBuzz"`, se para |
>
> Para 9: `9 % 15` vale 9 (falso), luego `9 % 3` vale 0 (verdadero): `"Fizz"`.
> Para 7, ninguna guarda es verdadera: el último caso da `"7"`.

Prueba: pon el caso `n % 15` al final, justo antes de `text(n)`. Para
15, obtienes `Fizz`: el caso del 3 ha respondido primero.

### Ejercicio 6: El extracto bancario

Nivel: ★★☆

Una cuenta bancaria tiene 100 €. Estas son las operaciones del mes, cada una
con un símbolo `:ingreso` o `:retirada` (ver [los símbolos](symboles.md)):

```text
ingreso 50, retirada 20, ingreso 15, retirada 30
```

Muestra el saldo después de cada operación y luego el saldo final:

```text
[150, 130, 145, 115]
115
```

> **Pista:** escribe `aplicar(saldo, operacion)` que da el nuevo
> saldo, con un `match` sobre el registro. Después, `fold` pasa el
> saldo de una operación a la siguiente, y `scan` muestra cada paso.

> **Solución**
>
> ```rythme
> fn aplicar(saldo, operacion) = match operacion
>   {tipo: :ingreso, importe: m} -> saldo + m
>   {tipo: :retirada, importe: m} -> saldo - m
> end
>
> operaciones = [
>   {tipo: :ingreso, importe: 50},
>   {tipo: :retirada, importe: 20},
>   {tipo: :ingreso, importe: 15},
>   {tipo: :retirada, importe: 30}
> ]
> print(scan(operaciones, 100, aplicar))
> print(fold(operaciones, 100, aplicar))
> ```
>
> ```sortie
> [150, 130, 145, 115]
> 115
> ```
>
> Por qué funciona: `fold` es como el cuaderno de un cajero. Empieza
> en 100, y cada línea del cuaderno usa el saldo de la línea
> anterior. No se modifica nada: cada paso fabrica un saldo nuevo.
>
> | Paso | saldo antes | tipo | importe | saldo después |
> |------|-------------|------|---------|---------------|
> | 1 | 100 | `:ingreso` | 50 | 150 |
> | 2 | 150 | `:retirada` | 20 | 130 |
> | 3 | 130 | `:ingreso` | 15 | 145 |
> | 4 | 145 | `:retirada` | 30 | 115 |

Prueba: cambia el 100 inicial por 0. El saldo final pasa a ser `15`,
y el segundo elemento de `scan` pasa a ser `30`.

### Ejercicio 7: El cifrado César

Nivel: ★★☆

Julio César ocultaba sus mensajes desplazando cada letra 3 posiciones en
el alfabeto: `a` se convierte en `d`, `b` en `e`… y `z` vuelve al principio para
convertirse en `c`. Escribe `cesar(mensaje, desplazamiento)` para un mensaje en
minúsculas sin acentos. Los espacios no cambian. Comprueba que un segundo
desplazamiento de 23 (3 + 23 = 26, una vuelta completa) devuelve el mensaje original:

```text
krod crh
hola zoe
```

> **Pista:** cada carácter tiene un número: `char_code("a")` vale 97,
> y `char(97)` devuelve `"a"`. La posición de una letra en el alfabeto es
> por tanto `char_code(c) - 97` (de 0 para `a` a 25 para `z`).

> **Pista:** para volver al principio después de `z`, toma el resto de la
> división entre 26: `(posicion + desplazamiento) % 26`.

> **Solución**
>
> ```rythme
> alfabeto = "abcdefghijklmnopqrstuvwxyz"
>
> fn desplazar(c, k) =
>   if contains(alfabeto, c) then char((char_code(c) - 97 + k) % 26 + 97) else c
>
> fn cesar(mensaje, k) = chars(mensaje) |> map(fn(c) = desplazar(c, k)) |> join("")
>
> secreto = cesar("hola zoe", 3)
> print(secreto)
> print(cesar(secreto, 23))
> ```
>
> ```sortie
> krod crh
> hola zoe
> ```
>
> Por qué funciona: el alfabeto está colocado en círculo, como las horas
> de un reloj. El `% 26` da la vuelta a la esfera.
>
> ```text
>   ... w  x  y  z  a  b  c  d ...
>                └──────────►c   z avanza 3: 25 + 3 = 28, 28 % 26 = 2 → c
> ```
>
> | c | char_code(c) - 97 | + 3 | % 26 | letra |
> |---|-------------------|-----|------|-------|
> | `h` | 7 | 10 | 10 | `k` |
> | `o` | 14 | 17 | 17 | `r` |
> | `z` | 25 | 28 | 2 | `c` |
> | espacio | | | | espacio (sin cambios) |

Prueba: `cesar("abc", 1)` da `bcd`, y `cesar("abc", 26)` devuelve
`abc`: una vuelta completa no cambia nada.

### Ejercicio 8: La suma de las cifras

Nivel: ★★☆

Escribe una función **recursiva** `suma_cifras(n)` que suma las
cifras de un número entero positivo: para 1234, es 1 + 2 + 3 + 4 = 10.
No uses texto, solo cálculos:

```text
10 9 18
```

para 1234, 9 y 99.

> **Pista:** `n % 10` da la última cifra (1234 % 10 vale 4), y
> `n // 10` quita esa última cifra (1234 // 10 vale 123).

> **Pista:** el caso base es 0: ya no queda ninguna cifra, la suma
> vale 0. Ver [la recursión](recursion.md).

> **Solución**
>
> ```rythme
> fn suma_cifras(0) = 0
> fn suma_cifras(n) = n % 10 + suma_cifras(n // 10)
>
> print(suma_cifras(1234), suma_cifras(9), suma_cifras(99))
> ```
>
> ```sortie
> 10 9 18
> ```
>
> Por qué funciona: en cada llamada, se «arranca» la última cifra,
> como se cortan los billetes de un talonario, hasta que no queda nada.
>
> | Llamada | n | n % 10 | n // 10 | pendiente |
> |---------|---|--------|---------|-----------|
> | 1 | 1234 | 4 | 123 | `4 + ...` |
> | 2 | 123 | 3 | 12 | `4 + 3 + ...` |
> | 3 | 12 | 2 | 1 | `4 + 3 + 2 + ...` |
> | 4 | 1 | 1 | 0 | `4 + 3 + 2 + 1 + ...` |
> | 5 | 0 | caso base | | `4 + 3 + 2 + 1 + 0` = 10 |

Prueba: quita la línea `fn suma_cifras(0) = 0`. La función ya no
se detiene nunca, porque `0 // 10` sigue valiendo 0. Rythme acaba
deteniéndose con el error `E014`: «demasiadas llamadas anidadas».

### Ejercicio 9: Ordenar por inicial

Nivel: ★★☆

Aquí tienes una lista de frutas. Agrúpalas por su primera letra y luego muestra
las frutas que empiezan por `p`:

```text
{p: ["pera", "piña", "plátano"], b: ["banana", "breva"], a: ["albaricoque"]}
["pera", "piña", "plátano"]
```

> **Pista:** `group_by(lista, f)` coloca los elementos en cajones:
> el nombre del cajón es lo que `f` responde para cada elemento. Aquí, `f`
> da la primera letra.

> **Solución**
>
> ```rythme
> frutas = ["pera", "piña", "banana", "albaricoque", "breva", "plátano"]
> por_letra = group_by(frutas, fn(f) = f[0])
> print(por_letra)
> print(por_letra.p)
> ```
>
> ```sortie
> {p: ["pera", "piña", "plátano"], b: ["banana", "breva"], a: ["albaricoque"]}
> ["pera", "piña", "plátano"]
> ```
>
> Por qué funciona: `group_by` hace como alguien que clasifica el
> correo. Para cada carta, lee la etiqueta y la pone en el casillero
> correcto. Se abre un casillero nuevo la primera vez que aparece una
> etiqueta, de ahí el orden `p`, `b`, `a`.
>
> ```text
> ┌───────────────────────────┐ ┌───────────────────┐ ┌─────────────┐
> │ p                         │ │ b                 │ │ a           │
> │ pera  piña  plátano       │ │ banana  breva     │ │ albaricoque │
> └───────────────────────────┘ └───────────────────┘ └─────────────┘
> ```

Prueba: agrupa por longitud con `fn(f) = len(f)`. Los cajones
se llaman entonces `4`, `6`, `11`, `5` y `7`.

> **Recuerda:** un ejercicio medio se divide en pequeñas funciones de una
> línea (`aplicar`, `desplazar`, `inicial`…). Prueba cada una por separado
> antes de juntarlas.

## Nivel ★★★: tómate tu tiempo

### Ejercicio 10: Comprimir un texto

Nivel: ★★★

Para ahorrar espacio, se puede escribir `aaabcc` en la forma `a3b1c2`:
cada letra seguida del número de veces que se repite **seguida**.
Escribe `comprimir(palabra)`:

```text
[["a", 3], ["b", 1], ["c", 2]]
a3b1c2 z10
```

La primera línea muestra los paquetes de `"aaabcc"`, la segunda los textos
comprimidos de `"aaabcc"` y de `"zzzzzzzzzz"`.

> **Pista:** escribe primero una función recursiva `paquetes(letras)`
> sobre la lista de letras. Para la lista `[c, ..resto]`, `take_while`
> toma de `resto` las letras iguales a `c`, y `drop` quita esas
> letras.

> **Pista:** el caso base es la lista vacía, que da `[]`. Cada
> paquete es una pequeña lista `[letra, número]`.

> **Solución**
>
> ```rythme
> fn paquetes([]) = []
> fn paquetes([c, ..resto])
>   iguales = take_while(resto, fn(x) = x == c)
>   siguiente = drop(resto, len(iguales))
>   [[c, 1 + len(iguales)], ..paquetes(siguiente)]
> end
>
> fn comprimir(palabra) =
>   paquetes(chars(palabra)) |> map(fn(p) = "{p[0]}{p[1]}") |> join("")
>
> print(paquetes(chars("aaabcc")))
> print(comprimir("aaabcc"), comprimir("zzzzzzzzzz"))
> ```
>
> ```sortie
> [["a", 3], ["b", 1], ["c", 2]]
> a3b1c2 z10
> ```
>
> Por qué funciona: es como ordenar cuentas de colores. Se toma
> la primera cuenta, se agarran todas las del mismo color que
> siguen, se anota «color, número» y se vuelve a empezar con lo que
> queda.
>
> | Llamada | c | resto | iguales | siguiente | paquete |
> |---------|---|-------|---------|-----------|---------|
> | 1 | `a` | `a a b c c` | `a a` | `b c c` | `["a", 3]` |
> | 2 | `b` | `c c` | (vacía) | `c c` | `["b", 1]` |
> | 3 | `c` | `c` | `c` | (vacía) | `["c", 2]` |
> | 4 | lista vacía | | | | caso base: `[]` |

Prueba: `comprimir("abab")` da `a1b1a1b1`. Las letras tienen que ir
seguidas para formar un paquete: este texto no se comprime nada bien.

### Ejercicio 11: Adivina el número

Nivel: ★★★

Un número secreto está entre 1 y 100. La mejor estrategia: proponer el
número del **medio**. Si es demasiado pequeño, se busca en la mitad alta;
si es demasiado grande, en la mitad baja. Escribe
`adivinar(secreto, bajo, alto, intentos)` que cuenta los intentos necesarios.
Para los secretos 37, 50, 1 y 100:

```text
3 1 6 7
```

> **Pista:** el medio se calcula con la división entera:
> `(bajo + alto) // 2`. Tres casos: encontrado, demasiado pequeño, demasiado grande.

> **Pista:** «demasiado pequeño» quiere decir que el secreto está por encima del
> medio: se vuelve a empezar con `medio + 1` como nuevo `bajo`, y un intento
> más.

> **Solución**
>
> ```rythme
> fn adivinar(secreto, bajo, alto, intentos)
>   medio = (bajo + alto) // 2
>   if medio == secreto then intentos
>   else if medio < secreto then adivinar(secreto, medio + 1, alto, intentos + 1)
>   else adivinar(secreto, bajo, medio - 1, intentos + 1)
> end
>
> print(adivinar(37, 1, 100, 1), adivinar(50, 1, 100, 1), adivinar(1, 1, 100, 1), adivinar(100, 1, 100, 1))
> ```
>
> ```sortie
> 3 1 6 7
> ```
>
> Por qué funciona: en cada intento, la zona de búsqueda se corta en
> dos, como cuando abres un diccionario por la mitad para encontrar una palabra.
> Siete intentos bastan siempre para 100 números. Esta es la partida para el
> secreto 37:
>
> | Intento | bajo | alto | medio | respuesta |
> |---------|------|------|-------|-----------|
> | 1 | 1 | 100 | 50 | demasiado grande: `alto` pasa a 49 |
> | 2 | 1 | 49 | 25 | demasiado pequeño: `bajo` pasa a 26 |
> | 3 | 26 | 49 | 37 | encontrado: la función da 3 |
>
> ```text
> 1 ─────────────────────────────── 100     intento 1: ¿50?
> 1 ─────────────── 49                       intento 2: ¿25?
>             26 ── 49                       intento 3: ¡37!
> ```

Prueba: añade `print(bajo, alto, medio)` justo después de la línea
`medio = ...`. El programa muestra entonces cada intento, y puedes
compararlo con tu tabla.

### Ejercicio 12: La calculadora polaca

Nivel: ★★★

En la notación «polaca inversa», se escriben los números **antes**
de la operación: `3 4 +` quiere decir 3 + 4, y `3 4 + 2 *` quiere decir
(3 + 4) × 2. Las calculadoras de algunos ingenieros funcionan así.
Escribe `rpn(texto)` que da un `Result`: `Ok(resultado)`, o `Err(...)`
si el cálculo es imposible.

```text
Ok(7)
Ok(14)
Ok(4)
Err("falta un número antes de +")
Err("quedan varios números")
```

para `"3 4 +"`, `"3 4 + 2 *"`, `"10 2 3 * -"`, `"3 +"` y `"3 4"`.

> **Pista:** usa una **pila**, como una pila de platos: un
> número se pone encima; una operación toma los dos platos de
> arriba, calcula y vuelve a poner el resultado. Con una lista, la parte de arriba es el
> primer elemento: `[n, ..pila]` pone `n`.

> **Pista:** escribe `paso(pila, palabra)` que da `Ok(nueva_pila)` o
> `Err(mensaje)`, y luego una función recursiva `evaluar(palabras, pila)` que
> usa `?` para pararse en el primer error (ver
> [gestionar los errores](erreurs.md)).

> **Solución**
>
> ```rythme
> operadores = ["+", "-", "*"]
>
> fn calcular(op, a, b) = match op
>   "+" -> a + b
>   "-" -> a - b
>   _ -> a * b
> end
>
> fn paso([b, a, ..resto], op) if contains(operadores, op) =
>   Ok([calcular(op, a, b), ..resto])
> fn paso(_, op) if contains(operadores, op) = Err("falta un número antes de {op}")
> fn paso(pila, palabra)
>   n = parse_int(palabra)?
>   Ok([n, ..pila])
> end
>
> fn evaluar([], [resultado]) = Ok(resultado)
> fn evaluar([], _) = Err("quedan varios números")
> fn evaluar([palabra, ..resto], pila)
>   nueva = paso(pila, palabra)?
>   evaluar(resto, nueva)
> end
>
> fn rpn(texto) = evaluar(split(texto), [])
>
> print(rpn("3 4 +"))
> print(rpn("3 4 + 2 *"))
> print(rpn("10 2 3 * -"))
> print(rpn("3 +"))
> print(rpn("3 4"))
> ```
>
> ```sortie
> Ok(7)
> Ok(14)
> Ok(4)
> Err("falta un número antes de +")
> Err("quedan varios números")
> ```
>
> Por qué funciona: la pila guarda los números que esperan. Cuidado con
> el orden: la parte de arriba de la pila es el **segundo** número (`b`), de ahí
> `calcular(op, a, b)` y no `(op, b, a)`. Para `10 2 3 * -`:
>
> | Paso | palabra | pila antes | pila después |
> |------|---------|------------|--------------|
> | 1 | `10` | `[]` | `[10]` |
> | 2 | `2` | `[10]` | `[2, 10]` |
> | 3 | `3` | `[2, 10]` | `[3, 2, 10]` |
> | 4 | `*` | `[3, 2, 10]` | `[6, 10]` (2 × 3) |
> | 5 | `-` | `[6, 10]` | `[4]` (10 − 6) |
> | fin | | `[4]` | `Ok(4)` |
>
> ```text
>         ┌───┐
>         │ 3 │          ┌───┐
>         │ 2 │   *      │ 6 │   -      ┌───┐
>         │10 │  ───►    │10 │  ───►    │ 4 │
>         └───┘          └───┘          └───┘
> ```

Prueba: `print(rpn("3 x +"))`. La palabra `x` no es un número:
`parse_int` da un error, y `?` lo hace subir hasta el final.

> **Recuerda:** para un ejercicio difícil, haz la tabla paso a paso
> **antes** de escribir el código. Si sabes hacerlo a mano, sabes
> qué pedirle a Rythme.

## ¿Y ahora qué?

¡Enhorabuena! Has terminado las tres series de ejercicios de este recorrido. Para
seguir:

- `rythme exercise list` propone otros 30 ejercicios, comprobados por
  tests;
- [los proyectos guiados](projets.md) lo reúnen todo en programas
  de verdad;
- [la chuleta](aide-memoire.md) resume toda la sintaxis en una página.

Los demás recorridos también tienen su segunda serie, por ejemplo
[en Ruby](../ruby/exercices-2.md) y [en Python](../python/exercices-2.md).

Siguiente: [chuleta](aide-memoire.md)
