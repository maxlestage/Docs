# Números y cálculos

Esta página muestra los dos tipos de números de Rythme, los enteros y los decimales, y todo lo que puedes calcular con ellos.

> **Objetivos:**
>
> - sabrás distinguir entre un `Int` y un `Float`;
> - sabrás usar los operadores `+`, `-`, `*`, `/`, `//`, `%` y `**`;
> - sabrás redondear, convertir y usar las funciones matemáticas;
> - sabrás mostrar un número de forma bonita, con decimales y miles separados.

## Dos tipos de números

Un programa maneja números todo el tiempo: una edad, una puntuación, un
precio. Algunos se cuentan uno a uno (3 manzanas), otros tienen decimales
(1,5 kg de manzanas).

Rythme conoce dos tipos de números:

- los [enteros](../debuter/glossaire.md#entero) (`Int`), sin decimales:
  `42`, `-7`, `0`;
- los [flotantes](../debuter/glossaire.md#flotante) (`Float`), los números
  decimales, escritos con un **punto**: `3.14`, `2.0`, `-0.5`.

`type_of` dice de qué tipo es un número:

```rythme
print(type_of(42), type_of(3.14), type_of(2.0))
print(1_000_000, 0xFF, 0b1010)
```

```sortie
Int Float Float
1000000 255 10
```

`2.0` es un Float, aunque no tenga decimales. El `_` separa las cifras
para hacerlas legibles: `1_000_000` es un millón. `0xFF` está escrito en
hexadecimal y `0b1010` en binario.

> **Atención:** se escribe `3.14` con un punto, nunca `3,14`. La coma
> separa valores: `print(3,14)` muestra `3 14`.

## Enteros sin límite de tamaño

En muchos lenguajes, un entero tiene un tamaño máximo, y un cálculo
demasiado grande da un resultado falso. En Rythme, un entero crece tanto
como haga falta, como una cinta que se desenrolla sin fin:

```rythme
print(2 ** 100)
print(1_000_000 * 1_000_000)
fn factorial(n) = if n == 0 then 1 else n * factorial(n - 1)
print(factorial(30))
```

```sortie
1267650600228229401496703205376
1000000000000
265252859812191058636308480000000
```

Línea por línea:

- `print(2 ** 100)` multiplica 2 por sí mismo 100 veces: un número de 31
  cifras;
- la segunda línea multiplica un millón por un millón: un billón;
- `fn factorial(n) = …` define una función. El factorial de `n` es
  1 × 2 × 3 × … × `n`;
- así que `factorial(30)` multiplica todos los números del 1 al 30: el
  resultado tiene 33 cifras, todas correctas.

Todas las cifras son exactas. Los Floats, en cambio, son valores
**aproximados**: guardan unas 16 cifras.

```rythme
print(0.1 + 0.2, 0.1 + 0.2 == 0.3)
```

```sortie
0.30000000000000004 false
```

No es un defecto de Rythme: todos los lenguajes calculan así los números
decimales. Para comparar Floats, redondéalos primero (mira más abajo).

## Las cuatro operaciones

Un programa sirve a menudo de calculadora: el total de una cesta de la
compra, lo que paga cada uno en un restaurante. Estos cuatro signos bastan
para la mayoría de los cálculos.

`+`, `-` y `*` funcionan como en la escuela. Entre dos Int, el resultado es
un Int; en cuanto hay un Float, el resultado es un Float:

```rythme
print(7 + 2, 7 - 2, 7 * 2)
print(7 + 2.0, 3 * 1.0)
```

```sortie
9 5 14
9.0 3.0
```

La división `/` da **siempre un Float**, incluso cuando es exacta:

```rythme
print(7 / 2, 6 / 3, 1 / 3)
```

```sortie
3.5 2.0 0.3333333333333333
```

Así, `7 / 2` vale `3.5`, y no `3` como en otros lenguajes.

**Te toca:** escribe un programa que guarde `precio = 4` y `cantidad = 3`,
luego muestre el total que hay que pagar y, por último, lo que paga cada
uno si dos personas se reparten ese total.

> **Solución**
>
> ```rythme
> precio = 4
> cantidad = 3
> total = precio * cantidad
> print(total)
> print(total / 2)
> ```
>
> ```sortie
> 12
> 6.0
> ```

## División entera, resto y potencia

Para repartir objetos enteros, a menudo quieres el **cociente** y el
**resto**: 17 caramelos para 5 niños son 3 caramelos para cada uno, y
sobran 2.

```rythme
print(17 // 5, 17 % 5)
print(2 ** 10, 2 ** -1, 9 ** 0.5)
```

```sortie
3 2
1024 0.5 3.0
```

| Operador | Nombre | Ejemplo | Resultado |
|----------|--------|---------|-----------|
| `+` `-` `*` | suma, resta, multiplicación | `7 * 2` | `14` |
| `/` | división, siempre un Float | `7 / 2` | `3.5` |
| `//` | división entera, redondeada hacia abajo | `17 // 5` | `3` |
| `%` | resto de la división | `17 % 5` | `2` |
| `**` | potencia | `2 ** 10` | `1024` |

Las prioridades son las de las matemáticas: primero `**`, luego `*`, `/`,
`//`, `%`, y después `+` y `-`. Los paréntesis cambian el orden:

```rythme
print(2 + 3 * 4, (2 + 3) * 4, -2 ** 2)
```

```sortie
14 20 -4
```

`-2 ** 2` vale `-(2 ** 2)`, es decir, `-4`.

> **Consejo:** `n % 2 == 0` dice si `n` es par. Rythme también tiene
> `is_even(n)` e `is_odd(n)`, más legibles.

**Te toca:** tienes 23 huevos y cajas de 6. Escribe un programa que muestre
el número de cajas llenas y después el número de huevos que sobran.

> **Solución**
>
> ```rythme
> huevos = 23
> print(huevos // 6, huevos % 6)
> ```
>
> ```sortie
> 3 5
> ```

## Par o impar

Saber si un número es par sirve más a menudo de lo que crees: colorear una
fila de cada dos en una tabla, formar equipos de dos.

```rythme
print(is_even(4), is_odd(4), is_odd(7))
print(filter(1..10, is_even))
```

```sortie
true false true
[2, 4, 6, 8, 10]
```

`is_even` e `is_odd` son funciones como las demás: puedes dárselas a
`filter` o a `count`.

## Las funciones de cálculo

Algunos cálculos aparecen todo el tiempo: encontrar la puntuación más alta,
quitarle el signo a un número. En lugar de reescribirlos, usas funciones ya
hechas, como las teclas especiales de una calculadora.

Rythme ofrece las funciones matemáticas habituales, sin importar nada:

```rythme
print(abs(-5), abs(-2.5))
print(min(3, 7), max(3, 7.5), max([2, 9, 4]))
print(sqrt(16), sqrt(2))
print(pow(2, 8), pow(2.0, 0.5))
```

```sortie
5 2.5
3 7.5 9
4.0 1.4142135623730951
256 1.4142135623730951
```

- `abs(x)`: el valor absoluto, el número sin su signo;
- `min` y `max`: el más pequeño o el más grande, de varios valores o de
  una lista;
- `sqrt(x)`: la raíz cuadrada, siempre un Float;
- `pow(x, n)`: la potencia, como `x ** n`.

## Redondear

Un resultado como `0.30000000000000004` rara vez es útil tal cual.
Redondear es lo que haces en la caja cuando dices «unos 13 euros».

Cuatro funciones transforman un Float en un Int:

```rythme
print(round(2.5), round(2.4), round(-2.5))
print(floor(2.7), floor(-2.5), ceil(2.1))
print(int(3.99), int(-3.99))
print(round(3.14159, 2), round(0.1 + 0.2, 2) == 0.3)
```

```sortie
3 2 -3
2 -3 3
3 -3
3.14 true
```

| Función | Qué hace | `2.7` | `-2.5` |
|---------|----------|-------|--------|
| `round(x)` | al más cercano (un `.5` se aleja de cero) | `3` | `-3` |
| `floor(x)` | hacia abajo, el «suelo» | `2` | `-3` |
| `ceil(x)` | hacia arriba, el «techo» | `3` | `-2` |
| `int(x)` | corta los decimales | `2` | `-2` |

`round(x, cifras)` conserva ese número de decimales y da un Float.

**Te toca:** una comida cuesta `12.3` €. Escribe un programa que muestre
ese precio redondeado al euro más cercano, luego al euro de arriba y luego
al euro de abajo.

> **Solución**
>
> ```rythme
> precio_comida = 12.3
> print(round(precio_comida), ceil(precio_comida), floor(precio_comida))
> ```
>
> ```sortie
> 12 13 12
> ```

## Convertir

Lo que el usuario escribe con el teclado llega como texto: `"42"` todavía
no es el número 42. Para calcular con él, primero hay que convertirlo.

Rythme **nunca** convierte un valor en silencio. `int` y `float` convierten
cuando tú se lo pides:

```rythme
print(int("42") + 1, float("2.5") * 2, float(3), int(true))
print(parse_int("42"), parse_int("doce"))
```

```sortie
43 5.0 3.0 1
Ok(42) Err("\"doce\" no es un número entero")
```

`int("doce")` detiene el programa. Cuando un texto viene del usuario y
puede no ser un número, es mejor usar `parse_int`: da un `Result`,
explicado en [gestionar los errores](erreurs.md).

## MCD y MCM

Estos dos cálculos sirven para repartir o para volver a coincidir: dos
autobuses que pasan cada 4 y cada 6 minutos vuelven a pasar juntos cada 12
minutos.

`gcd(a, b)` da el máximo común divisor (MCD), y `lcm(a, b)` el mínimo
común múltiplo (MCM):

```rythme
print(gcd(12, 18), lcm(4, 6))
numerador = 18
denominador = 24
d = gcd(numerador, denominador)
print("{numerador}/{denominador} = {numerador // d}/{denominador // d}")
```

```sortie
6 12
18/24 = 3/4
```

Línea por línea:

- la primera línea muestra el MCD de 12 y 18 (6), y luego el MCM de 4 y 6
  (12);
- `numerador` y `denominador` reciben 18 y 24: la fracción 18/24;
- `d` recibe 6, el número más grande que divide a la vez a 18 y a 24;
- la última línea divide arriba y abajo entre `d`: `18 // 6` vale 3 y
  `24 // 6` vale 4.

El MCD sirve, por ejemplo, para simplificar una fracción.

## Pi y la trigonometría

Estas funciones sirven en cuanto hay círculos o ángulos: calcular un área,
dibujar, hacer girar un objeto en un juego.

`pi` es la constante π. `sin`, `cos` y `tan` reciben un ángulo en
**radianes**. Para grados, multiplica por `pi / 180`:

```rythme
print(pi)
angulo = 30 * pi / 180
print(sin(angulo), round(sin(angulo), 2))
print(cos(0), round(cos(60 * pi / 180), 2))
radio = 2
print("área del círculo: {round(pi * radio ** 2, 2)}")
```

```sortie
3.141592653589793
0.49999999999999994 0.5
1.0 0.5
área del círculo: 12.57
```

Línea por línea:

- `print(pi)` muestra π, con 16 cifras;
- `angulo` recibe 30 grados, convertidos a radianes;
- la línea siguiente muestra el seno de ese ángulo, primero tal cual y
  luego redondeado a 2 decimales;
- `cos(0)` vale `1.0`, y el coseno de 60°, redondeado, vale `0.5`;
- `radio` vale 2: el área del círculo, π × 2 × 2, vale unos 12,57.

El seno de 30° vale 0,5; el cálculo aproximado da `0.49999999999999994`,
y `round` lo deja limpio.

## Números al azar

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

El azar sirve sobre todo en los juegos: tirar un dado, elegir quién empieza,
sacar una carta.

`random(a, b)` saca un entero de `a` a `b`, ambos incluidos, como un dado.
`random()` da un Float entre 0 y 1, y `random(lista)` un elemento al azar.
El resultado cambia en cada ejecución:

```rythme norun
print(random(1, 6))
print(random(["cara", "cruz"]))
```

```text
4
cruz
```

Lo que sí puedes comprobar es que el dado se queda entre 1 y 6:

```rythme
dado = random(1, 6)
print(dado >= 1 and dado <= 6, contains(1..6, dado))
```

```sortie
true true
```

## Mostrar un número de forma bonita

Un precio se escribe `13.50 €`, no `13.5 €`. Para que un número sea
agradable de leer, eliges su número de decimales y cómo se escriben los
miles.

`text(x, decimales)` escribe un número con **exactamente** ese número de
decimales. Es perfecto para precios:

```rythme
print(text(3.14159, 2), text(5, 2), text(1 / 3, 3))
precio = 4.5
print("Total: {text(precio * 3, 2)} €")
```

```sortie
3.14 5.00 0.333
Total: 13.50 €
```

Línea por línea:

- `text(3.14159, 2)` conserva dos decimales: `3.14`. `text(5, 2)` los
  añade: `5.00`;
- `precio` recibe 4.5;
- `precio * 3` vale 13.5, y `text(…, 2)` lo escribe `13.50`. Las llaves lo
  colocan dentro del texto.

**Te toca:** un cuaderno cuesta `2.5` €. Escribe un programa que muestre
`4 cuadernos: 10.00 €`, calculando el precio con `text`.

> **Solución**
>
> ```rythme
> precio_cuaderno = 2.5
> cuadernos = 4
> print("{cuadernos} cuadernos: {text(precio_cuaderno * cuadernos, 2)} €")
> ```
>
> ```sortie
> 4 cuadernos: 10.00 €
> ```

`format_number(x, decimales, idioma)` también separa los miles. Con
`"es"`, obtienes la escritura española: un punto para los miles y una coma
para los decimales.

```rythme
print(format_number(1234567.891, 2, "es"))
print(format_number(1234567.891, 2))
print(format_number(-42000, 0, "es"), format_number(0.5, 2, "es"))
```

```sortie
1.234.567,89
1,234,567.89
-42.000 0,50
```

Sin idioma, es la escritura inglesa: una coma para los miles y un punto
para los decimales.

## Errores frecuentes

### Sumar un número a un texto

```rythme norun
edad = "36"
print(edad + 1)
```

```text
error[E005]: no se puede usar `+` con Text y Int
 --> main.ry:2:12
  |
2 | print(edad + 1)
  |            ^ Text + Int 1
  |
  = ayuda: para construir un texto con un valor dentro, usa llaves: "total: {x}"
      o convierte antes el valor: text(x)
  = más sobre este error: `rythme explain E005`
```

`"36"` es un texto, no un número. Conviértelo: `int(edad) + 1` vale `37`.

### Usar el resultado de / como posición

```rythme norun
xs = ["a", "b", "c", "d"]
mitad = len(xs) / 2
print(xs[mitad])
```

```text
error[E005]: un índice debe ser un número entero (Int), pero recibió un valor de tipo Float
 --> main.ry:3:10
  |
3 | print(xs[mitad])
  |          ^^^^^
  |
  = más sobre este error: `rythme explain E005`
```

`/` da siempre un Float: `4 / 2` vale `2.0`. Para una posición, usa la
división entera: `mitad = len(xs) // 2`.

### Dividir entre cero

```rythme norun
notas = []
print(sum(notas) / len(notas))
```

```text
error[E008]: división entre cero
 --> main.ry:2:18
  |
2 | print(sum(notas) / len(notas))
  |                  ^
  |
  = ayuda: comprueba antes el divisor: `if b == 0 then ... else a / b`,
      o devuelve un Result: `if b == 0 then Err("división entre cero") else Ok(a / b)`
  = más sobre este error: `rythme explain E008`
```

Una lista vacía tiene 0 elementos. Compruébalo antes:
`if is_empty(notas) then 0 else sum(notas) / len(notas)`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto valen `7 / 2`, `7 // 2` y `7 % 2`?

> **Respuesta**
>
> `3.5`, `3` y `1`. `/` da siempre un Float, `//` el cociente entero y
> `%` el resto.

**Pregunta 2:** ¿qué dan `round(2.5)`, `floor(2.5)` e `int(-2.5)`?

> **Respuesta**
>
> `3`, `2` y `-2`. `round` va al más cercano (un `.5` se aleja de cero),
> `floor` hacia abajo, `int` corta los decimales.

**Pregunta 3:** ¿cómo muestras `1234.5` en la forma `1.234,50`?

> **Respuesta**
>
> `format_number(1234.5, 2, "es")`: dos decimales, escritura española.

## En resumen

- `Int` para los enteros, sin límite de tamaño; `Float` para los
  decimales, escritos con un punto, y aproximados.
- `/` da siempre un Float; `//` y `%` dan el cociente entero y el resto;
  `**` la potencia.
- `round`, `floor`, `ceil` e `int` redondean; `int`, `float` y `parse_int`
  convierten.
- `abs`, `min`, `max`, `sqrt`, `pow`, `gcd`, `lcm`, `pi`, `sin`, `cos`
  están siempre disponibles.
- `text(x, 2)` y `format_number(x, 2, "es")` muestran un número de forma
  bonita.

Siguiente: [condiciones y repetición](conditions-boucles.md)
