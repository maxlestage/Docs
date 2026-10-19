# Ejercicios resueltos

Doce ejercicios para practicar todo el recorrido de Rythme, del más fácil al más difícil, cada uno con una pista y una solución comprobada.

## Cómo hacer estos ejercicios

Cada ejercicio tiene un nivel:

- ★☆☆ fácil: unas pocas líneas, una sola idea;
- ★★☆ medio: hay que combinar dos o tres ideas;
- ★★★ más difícil: tómate tu tiempo.

Escribe tu solución en un archivo, por ejemplo `exo.ry`, y luego ejecútalo
con `rythme run exo.ry`. No mires la solución hasta haberlo intentado: se
aprende buscando. A menudo hay varias respuestas correctas.

## Lo básico

### Ejercicio 1: Presentarse

Nivel: ★☆☆

Da un nombre a tu nombre de pila y a tu edad, y luego muestra una frase como
esta, con la edad del año que viene calculada por el programa:

```text
Me llamo Ada y tengo 36 años. El año que viene, tendré 37 años.
```

> **Pista:** un cálculo puede ir entre llaves: `"{edad + 1}"`. Ver
> [primeros pasos](premiers-pas.md).

> **Solución**
>
> ```rythme
> nombre = "Ada"
> edad = 36
> print("Me llamo {nombre} y tengo {edad} años. El año que viene, tendré {edad + 1} años.")
> ```
>
> ```sortie
> Me llamo Ada y tengo 36 años. El año que viene, tendré 37 años.
> ```

### Ejercicio 2: Par o impar

Nivel: ★☆☆

Escribe una función `paridad(n)` que dé `"par"` o `"impar"`.

> **Pista:** `n % 2` es el resto de la división entre 2. Un `if` siempre
> tiene un `else`.

> **Solución**
>
> ```rythme
> fn paridad(n) = if n % 2 == 0 then "par" else "impar"
>
> print(paridad(4), paridad(7), paridad(0))
> ```
>
> ```sortie
> par impar par
> ```

### Ejercicio 3: La tabla de multiplicar

Nivel: ★☆☆

Muestra la tabla del 7, desde `7 x 1 = 7` hasta `7 x 5 = 35`.

> **Pista:** no hay bucle `for`: usa `1..5` y `each`. Ver
> [condiciones y repetición](conditions-boucles.md).

> **Solución**
>
> ```rythme
> 1..5 |> each(fn(i) = print("7 x {i} = {7 * i}"))
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

### Ejercicio 4: El precio de la entrada

Nivel: ★☆☆

Una entrada de cine cuesta 5 € para menores de 12 años, 7 € para menores de
18 y 10 € para los demás. Escribe `precio(edad)` con una definición por
casos y guardas.

> **Pista:** `fn precio(edad) if edad < 12 = 5`, y luego los otros casos
> debajo. Ver [las guardas](fonctions.md#las-guardas).

> **Solución**
>
> ```rythme
> fn precio(edad) if edad < 12 = 5
> fn precio(edad) if edad < 18 = 7
> fn precio(_) = 10
>
> print(precio(8), precio(15), precio(40))
> ```
>
> ```sortie
> 5 7 10
> ```

### Ejercicio 5: Sumar hasta n

Nivel: ★★☆

Escribe una función **recursiva** `suma(n)` que sume los números del 1 a
`n`. Por ejemplo, `suma(4)` vale `1 + 2 + 3 + 4 = 10`.

> **Pista:** dos definiciones: `suma(0)` vale 0, y `suma(n)` vale
> `n + suma(n - 1)`.

> **Solución**
>
> ```rythme
> fn suma(0) = 0
> fn suma(n) = n + suma(n - 1)
>
> print(suma(4), suma(100))
> print(sum(1..100))
> ```
>
> ```sortie
> 10 5050
> 5050
> ```
>
> La última línea comprueba el resultado con `sum`, que hace lo mismo.

## Listas y registros

### Ejercicio 6: El boletín de notas

Nivel: ★☆☆

Con la lista `notas = [12, 8, 15, 19, 10]`, muestra la media, la mejor nota
y la lista de las notas menores que 10.

> **Pista:** `sum`, `len`, `max` y `filter`. `/` da un número con
> decimales.

> **Solución**
>
> ```rythme
> notas = [12, 8, 15, 19, 10]
> print("Media:", sum(notas) / len(notas))
> print("Mejor:", max(notas))
> print("Menos de 10:", filter(notas, fn(n) = n < 10))
> ```
>
> ```sortie
> Media: 12.8
> Mejor: 19
> Menos de 10: [8]
> ```

### Ejercicio 7: Feliz cumpleaños

Nivel: ★★☆

Escribe `cumpleanos(persona)` que recibe un registro como
`{nombre: "Ada", edad: 36}` y da un **nuevo** registro con un año más.

> **Pista:** `{..p, edad: ...}` copia `p` cambiando un campo. Ver
> [actualizar un registro](collections.md#actualizar-un-registro).

> **Solución**
>
> ```rythme
> fn cumpleanos(p) = {..p, edad: p.edad + 1}
>
> ada = {nombre: "Ada", edad: 36}
> mayor = cumpleanos(ada)
> print(ada)
> print(mayor)
> ```
>
> ```sortie
> {nombre: "Ada", edad: 36}
> {nombre: "Ada", edad: 37}
> ```
>
> `ada` no cambia: los valores no cambian nunca.

### Ejercicio 8: Los cuadrados de los impares

Nivel: ★★☆

Calcula la suma de los cuadrados de los números **impares** del 1 al 10,
encadenando los pasos con `|>`. Hazlo una segunda vez con puntos y un
bloque `{ |x| ... }`.

> **Pista:** `filter(is_odd)`, luego `map`, luego `sum`. Ver
> [pensar en funcional](fonctionnel.md).

> **Solución**
>
> ```rythme
> con_pipe = 1..10
>   |> filter(is_odd)
>   |> map(fn(x) = x * x)
>   |> sum
>
> con_puntos = (1..10)
>   .select(odd?)
>   .map { |x| x * x }
>   .sum
>
> print(con_pipe, con_puntos)
> ```
>
> ```sortie
> 165 165
> ```
>
> 1 + 9 + 25 + 49 + 81 = 165.

## Tipos, errores y texto

### Ejercicio 9: La máquina de bebidas

Nivel: ★★☆

Crea un tipo `Bebida` con tres variantes: `Cafe`, `Te` y `Zumo(fruta)`.
Escribe `describir(bebida)` que dé un texto, con `match`.

> **Pista:** `type Bebida = Cafe | Te | Zumo(fruta)`. En el `match`,
> `Zumo(f)` da el nombre `f` a la fruta. Ver [tus propios tipos](types.md).

> **Solución**
>
> ```rythme
> type Bebida = Cafe | Te | Zumo(fruta)
>
> fn describir(bebida) = match bebida
>   Cafe -> "un café, 1 €"
>   Te -> "un té, 1 €"
>   Zumo(f) -> "un zumo de {f}, 2 €"
> end
>
> [Cafe, Zumo("manzana"), Te] |> each(fn(b) = print(describir(b)))
> ```
>
> ```sortie
> un café, 1 €
> un zumo de manzana, 2 €
> un té, 1 €
> ```

### Ejercicio 10: Buscar a un alumno

Nivel: ★★☆

Escribe `nota_de(alumnos, nombre)` que busca a un alumno por su nombre en
una lista de registros. Da `"Ada: 17"`, o `"Eve: desconocida"` si el alumno
no está en la lista.

> **Pista:** `find` da un `Option`: `Some(alumno)` o `None`. Trata los dos
> casos con `match`. Ver [gestionar los errores](erreurs.md).

> **Solución**
>
> ```rythme
> alumnos = [{nombre: "Ada", nota: 17}, {nombre: "Alan", nota: 12}]
>
> fn nota_de(alumnos, nombre) = match find(alumnos, fn(a) = a.nombre == nombre)
>   Some(a) -> "{nombre}: {a.nota}"
>   None -> "{nombre}: desconocida"
> end
>
> print(nota_de(alumnos, "Ada"))
> print(nota_de(alumnos, "Eve"))
> ```
>
> ```sortie
> Ada: 17
> Eve: desconocida
> ```

### Ejercicio 11: Leer una edad

Nivel: ★★★

Escribe `leer_edad(texto)` que dé un `Result`:

- `Err(...)` si el texto no es un número (deja que `parse_int` dé el
  error);
- `Err("edad imposible")` si el número es negativo o mayor que 150;
- `Ok(edad)` en otro caso.

> **Pista:** `edad = parse_int(texto)?` lee el número, o detiene la
> función con el error. Después, un `if` da `Ok` o `Err`.

> **Solución**
>
> ```rythme
> fn leer_edad(texto)
>   edad = parse_int(texto)?
>   if edad < 0 or edad > 150 then Err("edad imposible") else Ok(edad)
> end
>
> print(leer_edad("36"))
> print(leer_edad("-4"))
> print(leer_edad("treinta"))
> ```
>
> ```sortie
> Ok(36)
> Err("edad imposible")
> Err("\"treinta\" no es un número entero")
> ```

### Ejercicio 12: Los palíndromos

Nivel: ★★★

Un **palíndromo** se lee igual en los dos sentidos, como «kayak». Escribe
`palindromo?(frase)` que no tenga en cuenta ni las mayúsculas ni los
espacios: «Anita lava la tina» es un palíndromo.

> **Pista:** pasa a minúsculas con `lower`, quita los espacios con
> `replace(texto, " ", "")` y luego compara con `reverse(texto)`. Ver
> [trabajar con texto](texte.md).

> **Solución**
>
> ```rythme
> fn palindromo?(frase)
>   limpia = frase |> lower |> replace(" ", "")
>   limpia == reverse(limpia)
> end
>
> print(palindromo?("kayak"))
> print(palindromo?("Anita lava la tina"))
> print(palindromo?("Rythme"))
> ```
>
> ```sortie
> true
> true
> false
> ```

## Para ir más lejos

Rythme incluye **30 ejercicios más**, graduados y corregidos con pruebas.
Lánzalos en un terminal:

```sh
rythme exercise list                  # la lista de los 30 ejercicios
rythme exercise start 1               # escribe 01-valores.ry en la carpeta
rythme exercise check 01-valores.ry   # comprueba tu solución
```

Ten [la chuleta](aide-memoire.md) abierta mientras buscas. Los otros
recorridos también tienen sus ejercicios: [en Ruby](../ruby/exercices.md)
y [en Rust](../rust/exercices.md).

Siguiente: [ejercicios resueltos: segunda serie](exercices-2.md)
