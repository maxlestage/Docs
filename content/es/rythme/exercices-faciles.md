# Ejercicios para empezar bien

Quince ejercicios pequeños y muy guiados, todos fáciles, para practicar lo básico de Rythme: nombres, cálculos, textos, condiciones, funciones, listas, registros y diccionarios.

## Cómo hacer estos ejercicios

Todos los ejercicios tienen el nivel ★☆☆: unas pocas líneas, una sola idea.
Para cada uno, sigue estos pasos:

```text
1. Lee el enunciado y el resultado esperado.
2. Piénsalo en papel: ¿qué valores? ¿qué cálculo?
3. Escribe el código en un archivo, por ejemplo exo.ry.
4. Ejecútalo:  rythme run exo.ry
5. Compara con el resultado esperado.
6. ¿Atascado? Lee la primera pista y luego la segunda.
7. Solo después, abre la solución.
```

Un error no es un fracaso: lee el mensaje, sobre todo la línea `ayuda:`.
La página [las trampas del principiante](pieges.md) explica las más comunes.

> **Consejo:** a menudo hay varias respuestas correctas. Si tu programa
> muestra el resultado correcto, está bien, aunque no se parezca a la
> solución.

## Nombres, cálculos y textos

### Ejercicio 1: Presentarse

Nivel: ★☆☆

Dale un nombre a tu nombre de pila y otro a tu ciudad. Luego muestra
una frase que use los dos. Resultado esperado (con tus valores):

```text
Hola, soy Lucía y vivo en Sevilla.
```

> **Pista:** empieza con `nombre = "Lucía"` y luego `ciudad = "Sevilla"`.

> **Pista:** dentro de un texto, `{nombre}` se reemplaza por el valor del nombre.

> **Solución**
>
> ```rythme
> nombre = "Lucía"
> ciudad = "Sevilla"
> print("Hola, soy {nombre} y vivo en {ciudad}.")
> ```
>
> ```sortie
> Hola, soy Lucía y vivo en Sevilla.
> ```
>
> Por qué funciona: las llaves insertan el valor de un nombre en el
> texto. La tilde de «Lucía» es un carácter como cualquier otro, dentro
> de las comillas.

### Ejercicio 2: El tique de compra

Nivel: ★☆☆

Un cruasán cuesta 3 euros. Compras 4. Calcula el total y
muestra:

```text
4 cruasanes a 3 euros: 12 euros
```

> **Pista:** tres nombres: `precio`, `cantidad` y `total`.

> **Pista:** `total = precio * cantidad`. La estrella `*` quiere decir «por».

> **Solución**
>
> ```rythme
> precio = 3
> cantidad = 4
> total = precio * cantidad
> print("{cantidad} cruasanes a {precio} euros: {total} euros")
> ```
>
> ```sortie
> 4 cruasanes a 3 euros: 12 euros
> ```
>
> Por qué funciona: `total` recibe el resultado del cálculo y luego el texto
> lo inserta. Cambia `cantidad`: el total se actualiza solo.

### Ejercicio 3: Repartir caramelos

Nivel: ★☆☆

Tienes 17 caramelos para 5 niños. Cada uno recibe el mismo número de
caramelos. Muestra cuántos recibe cada niño y cuántos sobran:

```text
Cada niño recibe 3 caramelos.
Sobran 2.
```

> **Pista:** la división entera `//` da la parte de cada uno.

> **Pista:** el resto `%` da lo que sobra: 17 = 3 × 5 + **2**.

> **Solución**
>
> ```rythme
> caramelos = 17
> ninos = 5
> print("Cada niño recibe {caramelos // ninos} caramelos.")
> print("Sobran {caramelos % ninos}.")
> ```
>
> ```sortie
> Cada niño recibe 3 caramelos.
> Sobran 2.
> ```
>
> Por qué funciona: `17 // 5` vale 3 (el 5 cabe 3 veces en 17) y
> `17 % 5` vale 2. Con `/`, obtendrías `3.4`, lo que no tiene sentido
> para caramelos.

### Ejercicio 4: Gritar

Nivel: ★☆☆

Escribe una función `gritar(frase)` que pone la frase en mayúsculas y
la rodea con `¡` y `!`. Resultado esperado para `gritar("qué bien")`:

```text
¡QUÉ BIEN!
```

> **Pista:** `upper` pone un texto en mayúsculas.

> **Pista:** `+` pega textos: `"¡" + upper(frase) + "!"`.

> **Solución**
>
> ```rythme
> fn gritar(frase) = "¡" + upper(frase) + "!"
> print(gritar("qué bien"))
> ```
>
> ```sortie
> ¡QUÉ BIEN!
> ```
>
> Por qué funciona: `upper` da un texto nuevo en mayúsculas, con las tildes
> incluidas, y `+` pega los textos uno detrás de otro.

### Ejercicio 5: La palabra al revés

Nivel: ★☆☆

Muestra la palabra `hola` escrita al revés:

```text
aloh
```

> **Pista:** busca una función que «dé la vuelta» a un valor, como a una
> tortilla.

> **Pista:** `reverse` funciona con las listas, y también con los textos.

> **Solución**
>
> ```rythme
> print(reverse("hola"))
> ```
>
> ```sortie
> aloh
> ```
>
> Por qué funciona: `reverse` lee los caracteres del último al primero.
> La palabra `"hola"` en sí no cambia: `reverse` fabrica una
> nueva.

## Condiciones y funciones

### Ejercicio 6: Mayor o menor de edad

Nivel: ★☆☆

Escribe una función `estado(edad)` que da `"mayor"` a partir de 18 años,
y `"menor"` si no. Resultado esperado para 20, 12 y 18:

```text
mayor menor mayor
```

> **Pista:** `if condicion then valor else otro_valor`.

> **Pista:** la condición es `edad >= 18`: «por lo menos 18».

> **Solución**
>
> ```rythme
> fn estado(edad) = if edad >= 18 then "mayor" else "menor"
> print(estado(20), estado(12), estado(18))
> ```
>
> ```sortie
> mayor menor mayor
> ```
>
> Por qué funciona: `>=` incluye el 18. Con `>`, `estado(18)` daría
> `"menor"`. Acuérdate siempre de probar el valor límite.

### Ejercicio 7: El más grande

Nivel: ★☆☆

Escribe una función `mas_grande(a, b)` que da el más grande de los dos
números. Resultado esperado para `(3, 8)`, `(10, 2)` y `(5, 5)`:

```text
8 10 5
```

> **Pista:** compara `a` y `b` con `>`.

> **Pista:** si `a > b`, la respuesta es `a`; si no, es `b`.

> **Solución**
>
> ```rythme
> fn mas_grande(a, b) = if a > b then a else b
> print(mas_grande(3, 8), mas_grande(10, 2), mas_grande(5, 5))
> ```
>
> ```sortie
> 8 10 5
> ```
>
> Por qué funciona: cuando los dos son iguales, la condición es falsa y
> se da `b`, que también vale 5. Rythme ya tiene una función para esto:
> `max([a, b])`.

### Ejercicio 8: El semáforo

Nivel: ★☆☆

Escribe una función `indicacion(color)`: `"verde"` da `"pasa"`,
`"amarillo"` da `"frena"`, `"rojo"` da `"detente"`, y cualquier
otro color da `"color desconocido"`.

```text
detente
pasa
color desconocido
```

> **Pista:** varios casos: usa `match color`, un caso por línea,
> y luego `end`.

> **Pista:** cada línea se escribe `"verde" -> "pasa"`. El último caso,
> `_`, atrapa todo lo demás.

> **Solución**
>
> ```rythme
> fn indicacion(color) = match color
>   "verde" -> "pasa"
>   "amarillo" -> "frena"
>   "rojo" -> "detente"
>   _ -> "color desconocido"
> end
> print(indicacion("rojo"))
> print(indicacion("verde"))
> print(indicacion("azul"))
> ```
>
> ```sortie
> detente
> pasa
> color desconocido
> ```
>
> Por qué funciona: `match` prueba los casos de arriba abajo. `"azul"` no
> corresponde a ningún texto, así que cae en `_`, siempre al final.

### Ejercicio 9: La media

Nivel: ★☆☆

Calcula la media de las notas `[12, 15, 10]`. Muéstrala tal cual,
y luego redondeada a una cifra decimal:

```text
12.333333333333334
12.3
```

> **Pista:** la media es la suma dividida por el número de notas:
> `sum` y `len`.

> **Pista:** `round(x, 1)` deja una cifra decimal.

> **Solución**
>
> ```rythme
> notas = [12, 15, 10]
> media = sum(notas) / len(notas)
> print(media)
> print(round(media, 1))
> ```
>
> ```sortie
> 12.333333333333334
> 12.3
> ```
>
> Por qué funciona: `sum(notas)` vale 37 y `len(notas)` vale 3. `/`
> da un número decimal, que `round` acorta.

### Ejercicio 10: La cuenta atrás

Nivel: ★☆☆

Muestra una cuenta atrás de 3 a 1, y luego `¡Ya!`:

```text
3
2
1
¡Ya!
```

> **Pista:** en Rythme no hay bucles. `reverse(1..3)` da `[3, 2, 1]`.

> **Pista:** `each` llama a una función para cada elemento:
> `each(fn(n) = print(n))`.

> **Solución**
>
> ```rythme
> reverse(1..3) |> each(fn(n) = print(n))
> print("¡Ya!")
> ```
>
> ```sortie
> 3
> 2
> 1
> ¡Ya!
> ```
>
> Por qué funciona: `1..3` es la lista `[1, 2, 3]`, `reverse` le da la
> vuelta y `each` muestra cada número. Atención: `3..1` sería una
> lista vacía.
>
> | Paso | n | Se muestra |
> |------|---|------------|
> | 1 | 3 | `3` |
> | 2 | 2 | `2` |
> | 3 | 1 | `1` |
> | fin | | `¡Ya!` |

## Listas, registros y diccionarios

### Ejercicio 11: Los cuadrados

Nivel: ★☆☆

Fabrica la lista de los cuadrados de los números del 1 al 6 (un cuadrado es un
número multiplicado por sí mismo):

```text
[1, 4, 9, 16, 25, 36]
```

> **Pista:** `1..6` da los números del 1 al 6.

> **Pista:** `map` fabrica una lista nueva: `map(fn(x) = x * x)`.

> **Solución**
>
> ```rythme
> print(1..6 |> map(fn(x) = x * x))
> ```
>
> ```sortie
> [1, 4, 9, 16, 25, 36]
> ```
>
> Por qué funciona: `map` aplica `x * x` a cada número y guarda los
> resultados en una lista nueva, en el mismo orden.

### Ejercicio 12: La lista de la compra

Nivel: ★☆☆

Empieza con `["pan", "leche"]`. Añade `"huevos"` y luego muestra el número
de artículos y la lista numerada a partir de 1:

```text
3 artículos
1. pan
2. leche
3. huevos
```

> **Pista:** `push(compras, "huevos")` da una lista **nueva**:
> dale un nombre nuevo.

> **Pista:** `each_with_index` da el artículo y su posición, que
> empieza en 0: muestra `{i + 1}`.

> **Solución**
>
> ```rythme
> compras = ["pan", "leche"]
> compras2 = push(compras, "huevos")
> print("{len(compras2)} artículos")
> compras2.each_with_index do |articulo, i|
>   print("{i + 1}. {articulo}")
> end
> ```
>
> ```sortie
> 3 artículos
> 1. pan
> 2. leche
> 3. huevos
> ```
>
> Por qué funciona: `compras` no ha cambiado, `compras2` tiene tres
> artículos. Las posiciones van de 0 a 2, de ahí el `+ 1` para un humano.

### Ejercicio 13: La ficha de un libro

Nivel: ★☆☆

Guarda un libro en un registro con un título, un autor y un
número de páginas. Muestra una frase, luego fabrica una ficha nueva
con 120 páginas y muestra los dos números de páginas:

```text
El principito, de Saint-Exupéry (96 páginas)
96 120
```

> **Pista:** un registro se escribe `{titulo: "...", autor: "...", paginas: 96}`
> y un campo se lee con un punto: `libro.titulo`.

> **Pista:** `{..libro, paginas: 120}` copia la ficha cambiando un solo
> campo.

> **Solución**
>
> ```rythme
> libro = {titulo: "El principito", autor: "Saint-Exupéry", paginas: 96}
> print("{libro.titulo}, de {libro.autor} ({libro.paginas} páginas)")
> libro2 = {..libro, paginas: 120}
> print(libro.paginas, libro2.paginas)
> ```
>
> ```sortie
> El principito, de Saint-Exupéry (96 páginas)
> 96 120
> ```
>
> Por qué funciona: `libro.paginas = 120` sería un error, porque nada se
> modifica. `{..libro, paginas: 120}` fabrica una copia, y el original conserva
> sus 96 páginas.

### Ejercicio 14: Las capitales

Nivel: ★☆☆

Guarda tres países y su capital en un diccionario. Muestra la
capital de Italia y luego busca la de Perú, que no está:

```text
Roma
desconocida
```

> **Pista:** `dict([["Francia", "París"], ["Italia", "Roma"], ...])`, y luego
> `capitales["Italia"]`.

> **Pista:** `capitales["Perú"]` detendría el programa. `get` da
> una opción, y `unwrap_or("desconocida")` un valor de reserva.

> **Solución**
>
> ```rythme
> capitales = dict([["Francia", "París"], ["Italia", "Roma"], ["Japón", "Tokio"]])
> print(capitales["Italia"])
> print(get(capitales, "Perú") |> unwrap_or("desconocida"))
> ```
>
> ```sortie
> Roma
> desconocida
> ```
>
> Por qué funciona: `get` da `None` para una clave que no está, y
> `unwrap_or` reemplaza `None` por `"desconocida"`.

### Ejercicio 15: Las letras distintas

Nivel: ★☆☆

¿Cuántas letras distintas hay en `abracadabra`? Muéstralas
y luego cuántas son:

```text
set("a", "b", "c", "d", "r") 5
```

> **Pista:** `chars(palabra)` da la lista de los caracteres.

> **Pista:** un conjunto (`set`) guarda cada valor una sola vez.

> **Solución**
>
> ```rythme
> palabra = "abracadabra"
> letras = set(chars(palabra))
> print(letras, len(letras))
> ```
>
> ```sortie
> set("a", "b", "c", "d", "r") 5
> ```
>
> Por qué funciona: `chars` da 11 caracteres, con repeticiones.
> `set` quita los duplicados y ordena las letras.

## ¿Y ahora qué?

¡Bravo si has llegado hasta aquí! Ejercicios más variados, de lo fácil
a lo difícil, te esperan en [los ejercicios resueltos](exercices.md).
`rythme exercise` también propone 30 ejercicios en el terminal.

Siguiente: [pensar de forma funcional](fonctionnel.md)
