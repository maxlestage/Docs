# Ejercicios para empezar bien

Esta página propone 15 ejercicios muy guiados, todos fáciles, para practicar las bases de Python: mostrar, calcular, elegir, repetir, escribir funciones y manejar listas y diccionarios.

## Cómo hacer estos ejercicios

Cada ejercicio es pequeño. Pide una sola idea nueva. Para cada uno:

1. **Lee** el enunciado y la salida esperada. Es lo que tu programa debe mostrar.
2. **Prueba en papel.** Escribe los pasos en español: «primero…, luego…».
3. **Escribe el código** en un archivo `ejercicio.py` y luego ejecuta
   `python3 ejercicio.py`.
4. **Compara** lo que se muestra con la salida esperada, carácter por carácter.
5. **¿Atascado?** Lee la primera pista. ¿Sigues atascado? La segunda. La solución va
   al final.

> **Consejo:** un error en rojo no es un fracaso. Lee la última línea del mensaje: a
> menudo dice qué va mal. La página [leer un error](../debuter/lire-une-erreur.md) te
> ayuda.

A menudo hay varias soluciones buenas. Si tu programa muestra exactamente la salida
esperada, lo has conseguido.

## Mostrar y calcular

### Ejercicio 1: Una tarjeta de visita

Nivel: ★☆☆

Guarda tu nombre y tu ciudad en dos variables. Luego muestra dos líneas, como abajo.

```text
Nombre: Alicia
Ciudad: Sevilla
```

> **Pista:** una [variable](../debuter/glossaire.md#variable) se crea con `=`:
> `nombre = "Alicia"`. No olvides las comillas alrededor del texto.

> **Pista:** una f-string une texto y una variable: `f"Nombre: {nombre}"`.

> **Solución**
>
> ```python
> nombre = "Alicia"
> ciudad = "Sevilla"
> print(f"Nombre: {nombre}")
> print(f"Ciudad: {ciudad}")
> ```
>
> ```sortie
> Nombre: Alicia
> Ciudad: Sevilla
> ```
>
> Por qué funciona: en una f-string, Python reemplaza `{nombre}` por lo que contiene la
> caja `nombre`. Cada `print` muestra una línea.

### Ejercicio 2: El precio total

Nivel: ★☆☆

Un cuaderno cuesta 4 euros. Compras 3. Guarda el precio y la cantidad en dos variables,
calcula el total y luego muéstralo.

```text
Total: 12 euros
```

> **Pista:** el signo de la multiplicación es `*`.

> **Pista:** crea una tercera variable: `total = precio * cantidad`.

> **Solución**
>
> ```python
> precio = 4
> cantidad = 3
> total = precio * cantidad
> print(f"Total: {total} euros")
> ```
>
> ```sortie
> Total: 12 euros
> ```
>
> Por qué funciona: Python calcula primero `precio * cantidad`, es decir `4 * 3`, y
> luego guarda `12` en `total`.

### Ejercicio 3: De minutos a horas

Nivel: ★☆☆

Una película dura 135 minutos. Muestra su duración en horas y minutos.

```text
2 h 15 min
```

> **Pista:** en 135 minutos, ¿cuántas veces cabe 60? Es la división entera `//`
> (mira [los números](nombres.md)).

> **Pista:** lo que queda después de quitar las horas es el resto `%`:
> `135 % 60`.

> **Solución**
>
> ```python
> duracion = 135
> horas = duracion // 60
> minutos = duracion % 60
> print(f"{horas} h {minutos} min")
> ```
>
> ```sortie
> 2 h 15 min
> ```
>
> Por qué funciona: `135 // 60` vale `2` (dos paquetes completos de 60). Quedan
> `135 - 120 = 15` minutos, y eso es exactamente `135 % 60`.

## Elegir

### Ejercicio 4: Mayor o menor de edad

Nivel: ★☆☆

Guarda la edad `15` en una variable. Muestra `menor` si la edad es inferior a 18 y, si
no, `mayor`.

```text
menor
```

> **Pista:** usa `if` y `else` (mira
> [condiciones y bucles](conditions-boucles.md)). No olvides los dos puntos y los
> 4 espacios.

> **Solución**
>
> ```python
> edad = 15
> if edad < 18:
>     print("menor")
> else:
>     print("mayor")
> ```
>
> ```sortie
> menor
> ```
>
> Por qué funciona: `15 < 18` vale `True`, así que Python ejecuta el bloque del `if` y
> se salta el del `else`.
>
> Prueba: pon `edad = 18`. El programa muestra `mayor`, porque `18 < 18` es falso.

### Ejercicio 5: El tiempo

Nivel: ★☆☆

Guarda una temperatura en una variable. Muestra `frío` por debajo de 10 grados,
`templado` de 10 a 24, y `calor` a partir de 25. Prueba con `18`.

```text
templado
```

> **Pista:** tres casos, así que `if`, `elif` y `else`.

> **Pista:** Python comprueba las condiciones en orden y se detiene en la primera
> verdadera. Empieza por `if temperatura < 10:`.

> **Solución**
>
> ```python
> temperatura = 18
> if temperatura < 10:
>     print("frío")
> elif temperatura < 25:
>     print("templado")
> else:
>     print("calor")
> ```
>
> ```sortie
> templado
> ```
>
> Por qué funciona: `18 < 10` es falso, Python pasa al `elif`. `18 < 25` es verdadero:
> muestra `templado` e ignora el resto.

## Repetir

### Ejercicio 6: Contar hasta 5

Nivel: ★☆☆

Muestra los números del 1 al 5, uno por línea.

```text
1
2
3
4
5
```

> **Pista:** un bucle `for` con `range`.

> **Pista:** `range(1, 6)` da 1, 2, 3, 4, 5. El 6 queda fuera.

> **Solución**
>
> ```python
> for i in range(1, 6):
>     print(i)
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> ```
>
> Por qué funciona: en cada vuelta, `i` toma el siguiente valor de `range(1, 6)`, y
> `print(i)` lo muestra.

### Ejercicio 7: La suma del 1 al 10

Nivel: ★☆☆

Calcula `1 + 2 + 3 + … + 10` con un bucle y luego muestra el resultado.

```text
55
```

> **Pista:** crea una variable `total = 0` **antes** del bucle. Es una hucha vacía.

> **Pista:** en el bucle, añade `i` a la hucha: `total = total + i`.

> **Solución**
>
> ```python
> total = 0
> for i in range(1, 11):
>     total = total + i
> print(total)
> ```
>
> ```sortie
> 55
> ```
>
> Por qué funciona: estas son las primeras vueltas.
>
> | Vuelta | `i` | `total` después de la vuelta |
> |---|---|---|
> | 1 | 1 | 1 |
> | 2 | 2 | 3 |
> | 3 | 3 | 6 |
> | … | … | … |
> | 10 | 10 | 55 |
>
> El `print` está **fuera** del bucle (sin espacios delante): solo muestra una vez, al
> final.

### Ejercicio 8: La cuenta atrás

Nivel: ★☆☆

Con un bucle `while`, muestra 3, 2, 1 y luego `¡Ya!`.

```text
3
2
1
¡Ya!
```

> **Pista:** empieza con `n = 3`. El bucle gira mientras `n > 0`.

> **Pista:** en el bucle, muestra `n` y luego resta 1: `n = n - 1`. Sin esta línea, el
> bucle no se detiene nunca.

> **Solución**
>
> ```python
> n = 3
> while n > 0:
>     print(n)
>     n = n - 1
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
> Por qué funciona: `n` pasa de 3 a 2, luego a 1, luego a 0. Cuando `n` vale 0, `n > 0`
> es falso: el bucle se detiene y Python pasa a la línea siguiente.

## Funciones

### Ejercicio 9: Decir hola

Nivel: ★☆☆

Escribe una función `hola(nombre)` que **devuelva** la frase `¡Hola Alicia!`. Muestra
el resultado para `"Alicia"` y para `"Bruno"`.

```text
¡Hola Alicia!
¡Hola Bruno!
```

> **Pista:** una [función](../debuter/glossaire.md#funcion) empieza por
> `def hola(nombre):` (mira [las funciones](fonctions.md)).

> **Pista:** la función usa `return`, y eres tú quien hace el `print` fuera:
> `print(hola("Alicia"))`.

> **Solución**
>
> ```python
> def hola(nombre):
>     return f"¡Hola {nombre}!"
>
> print(hola("Alicia"))
> print(hola("Bruno"))
> ```
>
> ```sortie
> ¡Hola Alicia!
> ¡Hola Bruno!
> ```
>
> Por qué funciona: en cada llamada, el [parámetro](../debuter/glossaire.md#parametro)
> `nombre` recibe un valor distinto. `return` devuelve la frase, y `print` la muestra.

### Ejercicio 10: El mayor de los dos

Nivel: ★☆☆

Escribe una función `el_mayor(a, b)` que devuelva el mayor de los dos números, **sin**
usar `max`. Muestra `el_mayor(4, 9)` y `el_mayor(7, 2)`.

```text
9
7
```

> **Pista:** en la función, compara `a` y `b` con un `if`.

> **Pista:** una función puede tener dos `return`. El primero que se alcanza detiene la
> función.

> **Solución**
>
> ```python
> def el_mayor(a, b):
>     if a > b:
>         return a
>     return b
>
> print(el_mayor(4, 9))
> print(el_mayor(7, 2))
> ```
>
> ```sortie
> 9
> 7
> ```
>
> Por qué funciona: para `(7, 2)`, `7 > 2` es verdadero, la función devuelve `7` y se
> detiene. Para `(4, 9)`, la prueba es falsa: se llega al segundo `return b`.

## Texto

### Ejercicio 11: Contar las letras

Nivel: ★☆☆

Guarda la palabra `"python"` en una variable. Muéstrala en mayúsculas, seguida de su
número de letras.

```text
PYTHON tiene 6 letras
```

> **Pista:** `len(palabra)` da la longitud de un texto (mira [el texto](texte.md)).

> **Pista:** `palabra.upper()` devuelve el texto en mayúsculas.

> **Solución**
>
> ```python
> palabra = "python"
> print(f"{palabra.upper()} tiene {len(palabra)} letras")
> ```
>
> ```sortie
> PYTHON tiene 6 letras
> ```
>
> Por qué funciona: entre las llaves de una f-string puedes escribir cualquier cálculo,
> incluso una llamada a un método.

## Listas y diccionarios

### Ejercicio 12: La lista de la compra

Nivel: ★☆☆

Parte de la lista `["pan", "leche"]`. Añade `"manzanas"` al final. Muestra cada
artículo precedido de un guion y luego el número de artículos.

```text
- pan
- leche
- manzanas
3 artículos
```

> **Pista:** `compra.append("manzanas")` añade al final (mira
> [las colecciones](collections.md)).

> **Pista:** `for articulo in compra:` recorre la lista, `len(compra)` la cuenta.

> **Solución**
>
> ```python
> compra = ["pan", "leche"]
> compra.append("manzanas")
> for articulo in compra:
>     print(f"- {articulo}")
> print(f"{len(compra)} artículos")
> ```
>
> ```sortie
> - pan
> - leche
> - manzanas
> 3 artículos
> ```
>
> Por qué funciona: `append` modifica la propia lista. Así que el bucle ve los tres
> artículos.

### Ejercicio 13: La mejor nota

Nivel: ★☆☆

Encuentra la nota más alta de la lista `[12, 17, 9, 15]` con un bucle, **sin** usar
`max`.

```text
17
```

> **Pista:** guarda en una variable `mejor` la nota más alta vista hasta ahora. Al
> principio, toma la primera: `mejor = notas[0]`.

> **Pista:** con cada nota, si es mayor que `mejor`, ocupa su lugar.

> **Solución**
>
> ```python
> notas = [12, 17, 9, 15]
> mejor = notas[0]
> for nota in notas:
>     if nota > mejor:
>         mejor = nota
> print(mejor)
> ```
>
> ```sortie
> 17
> ```
>
> Por qué funciona:
>
> | `nota` | ¿`nota > mejor`? | `mejor` después |
> |---|---|---|
> | 12 | no | 12 |
> | 17 | sí | 17 |
> | 9 | no | 17 |
> | 15 | no | 17 |

### Ejercicio 14: Los precios de las frutas

Nivel: ★☆☆

Parte del diccionario `{"manzana": 2, "kiwi": 3}`. Añade la clave `"melón"` con el
precio `5`. Muestra cada fruta con su precio.

```text
manzana: 2 €
kiwi: 3 €
melón: 5 €
```

> **Pista:** una clave se añade igual que se guarda un valor:
> `precios["melón"] = 5` (mira [los diccionarios](dictionnaires.md)).

> **Pista:** `for fruta, importe in precios.items():` da la clave y el valor en cada
> vuelta.

> **Solución**
>
> ```python
> precios = {"manzana": 2, "kiwi": 3}
> precios["melón"] = 5
> for fruta, importe in precios.items():
>     print(f"{fruta}: {importe} €")
> ```
>
> ```sortie
> manzana: 2 €
> kiwi: 3 €
> melón: 5 €
> ```
>
> Por qué funciona: un [diccionario](../debuter/glossaire.md#diccionario) guarda pares
> «clave → valor». Mantiene el orden en el que añadiste las claves.

### Ejercicio 15: Los cuadrados

Nivel: ★☆☆

Construye la lista de los cuadrados de los números del 1 al 5 en **una sola línea**,
con una comprensión, y luego muéstrala.

```text
[1, 4, 9, 16, 25]
```

> **Pista:** el cuadrado de `n` se escribe `n * n`.

> **Pista:** una comprensión tiene la forma `[lo_que_se_añade for n in ...]` (mira
> [las comprensiones](comprehensions.md)).

> **Solución**
>
> ```python
> cuadrados = [n * n for n in range(1, 6)]
> print(cuadrados)
> ```
>
> ```sortie
> [1, 4, 9, 16, 25]
> ```
>
> Por qué funciona: léelo en voz alta: «la lista de los `n * n`, para cada `n` del 1 al
> 5». Lo mismo con un bucle necesitaría tres líneas y un `append`.

## ¿Y ahora qué?

¡Bravo! Si estos ejercicios te parecen fáciles, pasa a los
[ejercicios corregidos](exercices.md), que llegan hasta el nivel ★★★. Si alguno se te
ha resistido, vuelve a leer [las trampas del principiante](pieges.md): a menudo la causa
está ahí.

Siguiente: [clases y objetos](types.md)
