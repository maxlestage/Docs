# Las trampas del principiante en Rythme

Esta página reúne las doce sorpresas que más a menudo bloquean a los principiantes en Rythme, sobre todo a quienes vienen de otro lenguaje, cada una con el mensaje real y la forma correcta de escribirlo.

> **Objetivos:**
>
> - sabrás por qué un valor nunca cambia, y qué escribir en su lugar;
> - sabrás reemplazar un bucle y un valor `null`;
> - sabrás evitar las trampas de los números: `/`, `//` y `0.1 + 0.2`;
> - sabrás por qué un texto y un número no se mezclan;
> - sabrás leer los mensajes de Rythme cuando caigas en una trampa.

## Cómo leer esta página

Cada trampa sigue el mismo orden:

```text
1. Lo que esperas            (la costumbre de otro lenguaje)
2. Lo que hace Rythme        (el código y su mensaje real)
3. Por qué                   (una o dos frases)
4. La corrección             (el código que funciona)
```

No hace falta leerlo todo de una vez. Vuelve aquí cuando un mensaje te
sorprenda.

> **Recuerda:** Rythme casi siempre te avisa. Lee la línea `ayuda:` del
> mensaje: a menudo da la corrección.

## Las trampas de los valores

### 1. No se cambia el valor de un nombre

**Esperas:** que `total = total + 1` sume 1 a `total`, como en casi todos
los lenguajes.

```rythme norun
total = 1
total = total + 1
print(total)
```

```text
error[E011]: `total` ya está definido
 --> main.ry:2:1
  |
2 | total = total + 1
  | ^^^^^ definido de nuevo aquí
  |
1 | total = 1
  | ----- definido por primera vez aquí
  |
  = ayuda: los valores nunca cambian en Rythme: da un nombre nuevo al valor nuevo, como `total2 = total + 1`
  = más sobre este error: `rythme explain E011`
```

**Por qué:** en Rythme, un [valor](../debuter/glossaire.md#valor) es
[inmutable](../debuter/glossaire.md#inmutable): nunca cambia. Un nombre es
una etiqueta pegada para siempre en una caja.

```text
 total ──► [ 1 ]        total2 ──► [ 2 ]
 (etiqueta pegada)      (etiqueta nueva, caja nueva)
```

**La corrección:** da un nombre nuevo al valor nuevo.

```rythme
total = 1
total2 = total + 1
print(total, total2)
```

```sortie
1 2
```

`+=` y `++` tampoco existen, por la misma razón.

Prueba: reemplaza `total2` por `total` en la segunda línea. Rythme rechaza
el programa antes incluso de ejecutarlo.

### 2. push y put no cambian nada

**Esperas:** que `push(compras, "huevos")` añada los huevos a la lista.

```rythme
compras = ["pan", "leche"]
push(compras, "huevos")
print(compras)
```

```sortie
["pan", "leche"]
```

¡Los huevos han desaparecido! Y eso que Rythme te había avisado, con una
advertencia:

```text
advertencia[W008]: el resultado de `push` se pierde
 --> main.ry:2:1
  |
2 | push(compras, "huevos")
  | ^^^^^^^^^^^^^^^^^^^^^^^ este valor se pierde
  |
  = ayuda: los valores nunca cambian: `push(list, x)` da un valor nuevo, y no cambia nada.
      Para conservarlo, dale un nombre: `nuevo = push(list, x)`
```

**Por qué:** `push` fabrica una lista **nueva**. Como nadie le da un nombre,
se pierde. Lo mismo pasa con `put` en un
[diccionario](../debuter/glossaire.md#diccionario).

**La corrección:** guarda el resultado con un nombre nuevo.

```rythme
compras = ["pan", "leche"]
compras2 = push(compras, "huevos")
print(compras)
print(compras2)
```

```sortie
["pan", "leche"]
["pan", "leche", "huevos"]
```

Es como una fotocopia de la lista de la compra: escribes en la copia, y el
original no se mueve.

Un campo de un [registro](../debuter/glossaire.md#registro) tampoco
se puede cambiar:

```rythme norun
ada = {nombre: "Ada", edad: 36}
ada.edad = 37
```

```text
error[E001]: no se puede dar un valor a esto
 --> main.ry:2:1
  |
2 | ada.edad = 37
  | ^^^^^^^^ esto no es un nombre
  |
2 | ada.edad = 37
  |          - `=` da un nombre a un valor
  |
  = ayuda: los registros no se pueden cambiar; construye uno nuevo: `nuevo = {..viejo, edad: valor}`
  = más sobre este error: `rythme explain E001`
```

Se construye una copia con un campo cambiado:

```rythme
ada = {nombre: "Ada", edad: 36}
ada2 = {..ada, edad: 37}
print(ada, ada2)
```

```sortie
{nombre: "Ada", edad: 36} {nombre: "Ada", edad: 37}
```

> **Recuerda:** en Rythme, nada se modifica. Se fabrica un valor nuevo, y se
> le da un nombre nuevo.

### 3. No hay bucle for ni while

**Esperas:** un [bucle](../debuter/glossaire.md#bucle) `for` para
repetir.

```rythme norun
for x in [1, 2]
  print(x)
end
```

```text
error[E001]: `for` no se usa en Rythme
 --> main.ry:1:1
  |
1 | for x in [1, 2]
  | ^^^ viene de otro lenguaje
  |
  = ayuda: Rythme no tiene bucles. Trabaja con la lista entera en su lugar:
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      o escribe una función recursiva. Consulta `rythme help functional`.
  = más sobre este error: `rythme explain E001`
```

**Por qué:** un bucle cambia un contador en cada vuelta. Pero en Rythme
nada cambia. Se dice más bien «haz esto con toda la lista».

**La corrección:** `each` para **hacer** algo con cada elemento, `map` para
**calcular** una lista nueva.

```rythme
[1, 2, 3] |> each(fn(x) = print("elemento {x}"))
dobles = [1, 2, 3] |> map(fn(x) = x * 2)
print(dobles)
```

```sortie
elemento 1
elemento 2
elemento 3
[2, 4, 6]
```

Esto es lo que hace `map`, elemento por elemento:

| Paso | x | x * 2 | Lista fabricada |
|------|---|-------|-----------------|
| 1 | 1 | 2 | `[2]` |
| 2 | 2 | 4 | `[2, 4]` |
| 3 | 3 | 6 | `[2, 4, 6]` |

Para un bucle «mientras», escribe una función que se llame a sí misma
(consulta [los bucles](boucles.md#repetir-con-la-recursion)).

Prueba: reemplaza `x * 2` por `x * x`. Obtienes `[1, 4, 9]`.

### 4. No hay null

**Esperas:** `null` (o `nil`, `None` en Python) para decir «ningún
valor».

```rythme norun
respuesta = null
```

```text
error[E001]: `null` no se usa en Rythme
 --> main.ry:1:13
  |
1 | respuesta = null
  |             ^^^^ viene de otro lenguaje
  |
  = ayuda: Rythme no tiene null: usa `None` para «ningún valor» y `Some(x)` para un valor
  = más sobre este error: `rythme explain E001`
```

**Por qué:** un [null](../debuter/glossaire.md#null) olvidado es la causa
de muchos fallos. Rythme guarda más bien el valor en una caja, una
[opción](../debuter/glossaire.md#option): `Some(valor)` cuando hay algo,
`None` cuando la caja está vacía.

```rythme
print(first(["a", "b"]), first([]))
```

```sortie
Some("a") None
```

No se calcula directamente con la caja: `first([]) + 1` es un error. Se
abre con `unwrap_or`, dando un valor de reserva:

```rythme
vacia = first([]) |> unwrap_or(0)
llena = first([5, 6]) |> unwrap_or(0)
print(vacia + 1, llena + 1)
```

```sortie
1 6
```

Las opciones se explican en detalle en [gestionar los errores](erreurs.md).

> **Recuerda:** nada de `for`, nada de `null`. Rythme te dice qué escribir en
> su lugar, en la línea `ayuda:`.

## Las trampas de los números y los textos

### 5. La división / siempre da un número decimal

**Esperas:** que `6 / 3` valga `2`.

```rythme
print(6 / 3, 7 / 2, 7 // 2)
```

```sortie
2.0 3.5 3
```

**Por qué:** `/` da **siempre** un número decimal, un
[flotante](../debuter/glossaire.md#flotante) (`Float`), incluso cuando la
división es exacta. Para un [entero](../debuter/glossaire.md#entero)
(`Int`), usa la división entera `//`.

La trampa aparece cuando usas el resultado como posición:

```rythme norun
notas = [12, 15, 9, 18]
medio = len(notas) / 2
print(notas[medio])
```

```text
error[E005]: un índice debe ser un número entero (Int), pero recibió un valor de tipo Float
 --> main.ry:3:13
  |
3 | print(notas[medio])
  |             ^^^^^
  |
  = más sobre este error: `rythme explain E005`
```

**La corrección:** `//` da un entero.

```rythme
notas = [12, 15, 9, 18]
medio = len(notas) // 2
print(medio, notas[medio])
```

```sortie
2 9
```

Prueba: muestra `type_of(4 / 2)` y luego `type_of(4 // 2)`. Lees `Float` y
luego `Int`.

### 6. 0.1 + 0.2 no vale exactamente 0.3

**Esperas:** que `0.1 + 0.2 == 0.3` valga `true`.

```rythme
print(0.1 + 0.2, 0.1 + 0.2 == 0.3)
```

```sortie
0.30000000000000004 false
```

**Por qué:** el ordenador guarda los números decimales en binario, con un
número limitado de cifras. Algunos números, como 0.1, no salen exactos,
igual que 1/3, que se escribe 0.3333... en decimal. No es cosa de Rythme:
Python, JavaScript y Ruby hacen lo mismo.

**La corrección:** redondea antes de comparar, o cuenta con enteros
(céntimos en lugar de euros).

```rythme
print(round(0.1 + 0.2, 2) == 0.3, 10 + 20 == 30)
```

```sortie
true true
```

### 7. Un texto y un número no se suman

**Esperas:** que `"edad: " + 36` dé `"edad: 36"`.

```rythme norun
edad = 36
print("edad: " + edad)
```

```text
error[E005]: no se puede usar `+` con Text y Int
 --> main.ry:2:16
  |
2 | print("edad: " + edad)
  |                ^ Text "edad: " + Int
  |
  = ayuda: para construir un texto con un valor dentro, usa llaves: "total: {x}"
      o convierte antes el valor: text(x)
  = más sobre este error: `rythme explain E005`
```

**Por qué:** Rythme nunca adivina. `"3" + 4`, ¿es `"34"` o `7`? En lugar de
elegir en silencio, se niega. Por la misma razón, un texto nunca es igual a
un número:

```rythme
print("1" == 1, 1 == 1.0)
```

```sortie
false true
```

**La corrección:** pon el valor entre llaves, o conviértelo con `text`.

```rythme
edad = 36
print("edad: {edad}", "edad: " + text(edad))
```

```sortie
edad: 36 edad: 36
```

En el otro sentido, `parse_int` lee un número en un texto. Pero da una caja
`Ok(42)`, no `42`, porque puede que el texto no sea un número:

```rythme
print(parse_int("42"), parse_int("hola"))
n = parse_int("42") |> unwrap_or(0)
print(n + 1)
```

```sortie
Ok(42) Err("\"hola\" no es un número entero")
43
```

> **Recuerda:** un texto sigue siendo un texto, un número sigue siendo un
> número. Para pasar de uno a otro: `"{x}"` o `text(x)` en un sentido,
> `parse_int` en el otro.

## Las trampas de la escritura

### 8. Un if siempre tiene un else

**Esperas:** un `if` sin `else`, que no hace nada cuando la condición es
falsa.

```rythme norun
x = 5
if x > 0 then print("positivo")
```

```text
error[E019]: este `if` no tiene `else`
 --> main.ry:2:1
  |
2 | if x > 0 then print("positivo")
  | ^^ todo `if` debe tener un `else`
  |
3 | 
  | - se esperaba `else` aquí
  |
  = ayuda: en Rythme, `if` siempre da un valor, así que necesita los dos casos:
        if edad >= 18 then "adulto" else "menor"
  = más sobre este error: `rythme explain E019`
```

**Por qué:** en Rythme, `if` da un valor, como un cálculo. Necesita un valor
para los dos casos.

**La corrección:** cuando no hay nada que hacer, da el valor vacío `()`.

```rythme
x = 5
if x > 0 then print("positivo") else ()
```

```sortie
positivo
```

Otra sorpresa: la [condición](../debuter/glossaire.md#condicion) debe ser
`true` o `false`. Una lista vacía o el número 0 no significan
«falso»:

```rythme norun
cesta = []
print(if cesta then "llena" else "vacía")
```

```text
error[E005]: una condición debe ser `true` o `false`, pero esto es un valor de tipo List
 --> main.ry:2:10
  |
2 | print(if cesta then "llena" else "vacía")
  |          ^^^^^ no es un Bool
  |
  = ayuda: para comprobar si está vacío, usa `is_empty(x)` o `not is_empty(x)`
  = más sobre este error: `rythme explain E005`
```

Escribe la pregunta completa: `if is_empty(cesta) then "vacía" else "llena"`.

### 9. Es end lo que cierra un bloque, no la sangría

**Esperas (en Python):** que volver a la izquierda baste para terminar la
función.

```rythme norun
fn doble(x)
  x * 2
print(doble(4))
```

```text
error[E020]: falta `end` para cerrar este `fn`
 --> main.ry:1:1
  |
1 | fn doble(x)
  | ^^ este `fn` nunca se cierra
  |
4 | 
  | - el código termina aquí
  |
  = ayuda: añade `end` en su propia línea después de la última línea del bloque
  = más sobre este error: `rythme explain E020`
```

**Por qué:** en Rythme, la [sangría](../debuter/glossaire.md#indentacion)
(los espacios al principio de las líneas) solo sirve para leer. Es la
palabra `end` la que cierra un [bloque](../debuter/glossaire.md#bloque).

**La corrección:**

```rythme
fn doble(x)
  x * 2
end
print(doble(4))
```

```sortie
8
```

```text
fn doble(x)       ◄── abre el bloque
  x * 2           ◄── el contenido (dos espacios: para tus ojos)
end               ◄── cierra el bloque
```

Una función de una sola línea no necesita `end`:
`fn doble(x) = x * 2`.

> **Consejo:** `rythme fmt main.ry` pone los espacios en su sitio por
> ti.

### 10. Los acentos y las mayúsculas cuentan

**Esperas:** que `canción` y `cancion` sean lo mismo.

```rythme norun
canción = "Hola"
print(cancion)
```

```text
error[E003]: nombre desconocido `cancion`
 --> main.ry:2:7
  |
2 | print(cancion)
  |       ^^^^^^^ no definido
  |
  = ayuda: ¿quisiste decir `canción`?
  = más sobre este error: `rythme explain E003`
```

**Por qué:** los acentos están permitidos en los nombres. Pero `ó` y `o` son
dos letras distintas, así que `canción` y `cancion` son dos nombres.

**La corrección:** elige una forma de escribir y mantenla en todas partes.
Mucha gente escribe sus nombres sin acentos, para no dudar.

Una mayúscula al principio de un nombre causa otro problema:

```rythme norun
Total = 3
```

```text
error[E003]: constructor desconocido `Total`
 --> main.ry:1:1
  |
1 | Total = 3
  | ^^^^^ no definido
  |
  = ayuda: los constructores se definen con `type`: `type Nombre = Total(campo) | Otro`
  = más sobre este error: `rythme explain E003`
```

Un nombre que empieza por mayúscula está reservado a los tipos y a sus
variantes (consulta [tus propios tipos](types.md)). Escribe `total = 3`.

### 11. Las posiciones se cuentan desde 0

**Esperas:** que el primer elemento esté en la posición 1.

```rythme
notas = [12, 15, 9]
print(notas[0], notas[2], notas[-1])
```

```sortie
12 9 9
```

```text
 posición:    0    1    2
 notas:     [ 12 , 15 , 9 ]
                        -1  (el último, contando desde el final)
```

Así que el último elemento de una lista de 3 está en la posición 2, no en
la 3:

```rythme norun
frutas = ["manzana", "pera", "kiwi"]
print(frutas[3])
```

```text
error[E007]: el índice 3 está fuera de rango: la lista tiene 3 elemento(s)
 --> main.ry:2:14
  |
2 | print(frutas[3])
  |              ^ no hay ningún elemento aquí
  |
  = ayuda: los índices válidos van de 0 a 2 (o -1 para el último elemento)
  = ayuda: para obtener una Option en vez de un error, usa `get(x, 3)`
  = más sobre este error: `rythme explain E007`
```

**La corrección:** `frutas[-1]` para el último, o `get(frutas, 3)`, que da
`None` en lugar de detenerse.

### 12. Una línea que empieza por - es una nueva línea de código

**Esperas:** que un cálculo largo pueda seguir en la línea siguiente.

```rythme norun
precio = 10
descuento = 3
total = precio
  - descuento
print(total)
```

```text
error[E024]: esta línea calcula un valor pero no hace nada con él
 --> main.ry:4:3
  |
4 |   - descuento
  |   ^^^^^^^^^^^ este valor se pierde
  |
  = ayuda: una línea que empieza por `-` es una instrucción nueva. Para continuar la línea de arriba,
      termina mejor esa línea con `-`:
        total = precio -
          descuento
  = más sobre este error: `rythme explain E024`
```

**Por qué:** `-descuento` solo ya es un cálculo válido («menos descuento»).
Así que Rythme no sabe que debe unirlo a la línea de arriba.

**La corrección:** deja el operador al final de la primera línea.

```rythme
precio = 10
descuento = 3
total = precio -
  descuento
print(total)
```

```sortie
7
```

## Errores frecuentes

Toda la página habla de errores; aquí van tres costumbres más que vienen de
otros lenguajes.

### Comillas simples alrededor de un texto

```rythme norun
nombre = 'Ada'
```

```text
error[E018]: carácter inesperado `'`
 --> main.ry:1:10
  |
1 | nombre = 'Ada'
  |          ^
  |
  = ayuda: los textos van entre comillas dobles: "hola"
  = más sobre este error: `rythme explain E018`
```

Un texto se escribe siempre entre comillas dobles rectas: `nombre = "Ada"`.

### Escribir return

```rythme norun
fn doble(x)
  return x * 2
end
```

```text
error[E001]: `return` no se usa en Rythme
 --> main.ry:2:3
  |
2 |   return x * 2
  |   ^^^^^^ viene de otro lenguaje
  |
  = ayuda: no hace falta `return`: la última expresión de una función es su resultado.
      Para parar antes con un error, devuelve `Err(...)` y usa `?`.
  = más sobre este error: `rythme explain E001`
```

La última línea de una función es su resultado: escribe simplemente
`x * 2`.

### Comparar con = en lugar de ==

```rythme norun
x = 3
print(if x = 3 then "tres" else "otro")
```

```text
error[E001]: se esperaba `then`, pero se encontró `=`
 --> main.ry:2:12
  |
2 | print(if x = 3 then "tres" else "otro")
  |            ^ se esperaba `then` aquí
  |
  = ayuda: para comparar dos valores, usa `==`
  = ayuda: un `if` se escribe: `if condición then valor else otro_valor`
  = más sobre este error: `rythme explain E001`
```

`=` da un nombre, `==` compara: `if x == 3 then ...`.

## Ponte a prueba

**Pregunta 1:** este programa muestra `[1, 2]`. ¿Por qué no está el 3, y
cómo conservarlo?

```rythme norun
xs = [1, 2]
push(xs, 3)
print(xs)
```

> **Respuesta**
>
> `push` no cambia `xs`: fabrica una lista nueva, que aquí se pierde porque
> no tiene nombre. Escribe `xs2 = push(xs, 3)` y luego `print(xs2)`.

**Pregunta 2:** ¿cuánto valen `9 / 3` y `9 // 3`?

> **Respuesta**
>
> `3.0` y `3`. `/` siempre da un número decimal; `//` da un entero.

**Pregunta 3:** ¿cómo mostrar `Puntos: 12` cuando `puntos = 12`?

> **Respuesta**
>
> `print("Puntos: {puntos}")`, o `print("Puntos: " + text(puntos))`.
> `"Puntos: " + puntos` es el error E005: un texto y un número no se
> suman.

## En resumen

- Nada se modifica: `push`, `put` y `{..r, campo: v}` dan un valor nuevo,
  que se guarda con un nombre nuevo.
- Ni bucles ni `null`: `each`, `map` y la recursión por un lado, `Some(x)` y
  `None` por el otro.
- `/` da un número decimal, `//` un entero; redondea antes de comparar
  números decimales.
- Un texto y un número no se mezclan: `"{x}"`, `text(x)` y `parse_int`.
- Todo `if` tiene un `else`, y es `end` lo que cierra un bloque.
- Los nombres distinguen acentos y mayúsculas, y las posiciones empiezan
  en 0.

Siguiente: [ejercicios para empezar bien](exercices-faciles.md)
