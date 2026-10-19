# Ejercicios resueltos: segunda serie

Esta página propone 12 ejercicios resueltos nuevos, del más fácil al más difícil, con tablas que siguen las variables paso a paso en las soluciones.

## Cómo usar esta página

Estos ejercicios son **nuevos**: no repiten los de los
[ejercicios para empezar bien](exercices-faciles.md) ni los de los
[ejercicios resueltos](exercices.md). Cada uno tiene un nivel:

- ★☆☆: fácil, una sola idea;
- ★★☆: medio, dos o tres ideas juntas;
- ★★★: más difícil, hay que pensar antes de escribir.

Para cada ejercicio:

1. Lee el enunciado y la salida esperada.
2. Escribe los pasos en español en un papel.
3. Escribe el código en un archivo `.py` y ejecútalo con `python3`.
4. ¿Atascado? Lee la pista. ¿Sigues atascado? Abre la solución.
5. Lee «Por qué funciona» aunque lo hayas conseguido.

Muchas soluciones contienen una **tabla de seguimiento**: una columna por variable, una
fila por vuelta del bucle. Si todavía no conoces esta herramienta, lee primero
[Seguir un programa paso a paso](pas-a-pas.md).

> **Recuerda:** un ejercicio fallado y luego entendido vale más que un ejercicio
> resuelto por suerte. Compara siempre tu solución con la de la página.

## Nivel ★☆☆: para calentar

### Ejercicio 1: El tique de caja

Nivel: ★☆☆

Tienes una lista de compras: `[("pan", 2), ("leche", 1), ("queso", 6)]`. Cada compra
es una tupla (nombre, precio). Muestra una línea por compra y luego el total.

```text
pan ... 2 €
leche ... 1 €
queso ... 6 €
TOTAL ... 9 €
```

> **Pista:** `for nombre, precio in articulos:` separa cada tupla en dos variables.
> Guarda una hucha `total = 0` antes del bucle.

> **Solución**
>
> ```python
> articulos = [("pan", 2), ("leche", 1), ("queso", 6)]
> total = 0
> for nombre, precio in articulos:
>     print(f"{nombre} ... {precio} €")
>     total = total + precio
> print(f"TOTAL ... {total} €")
> ```
>
> ```sortie
> pan ... 2 €
> leche ... 1 €
> queso ... 6 €
> TOTAL ... 9 €
> ```
>
> Por qué funciona: como un cajero, el bucle pasa cada artículo, lo muestra y suma su
> precio al total.
>
> | Vuelta | `nombre` | `precio` | `total` después |
> |---|---|---|---|
> | 1 | pan | 2 | 2 |
> | 2 | leche | 1 | 3 |
> | 3 | queso | 6 | 9 |
>
> El último `print` no está sangrado: se ejecuta una sola vez, después del bucle.

### Ejercicio 2: De grados Celsius a grados Fahrenheit

Nivel: ★☆☆

Escribe una función `fahrenheit(celsius)` que devuelva la temperatura en grados
Fahrenheit. La fórmula es: `celsius × 9 / 5 + 32`. Muestra la conversión de 0, 20, 37
y 100.

```text
0 °C = 32.0 °F
20 °C = 68.0 °F
37 °C = 98.6 °F
100 °C = 212.0 °F
```

> **Pista:** la función cabe en una línea, con `return`. Luego llámala en un bucle
> sobre la lista `[0, 20, 37, 100]`.

> **Solución**
>
> ```python
> def fahrenheit(celsius):
>     return celsius * 9 / 5 + 32
>
> for c in [0, 20, 37, 100]:
>     print(f"{c} °C = {fahrenheit(c)} °F")
> ```
>
> ```sortie
> 0 °C = 32.0 °F
> 20 °C = 68.0 °F
> 37 °C = 98.6 °F
> 100 °C = 212.0 °F
> ```
>
> Por qué funciona: la función es una pequeña máquina de convertir; el bucle le da cada
> temperatura por turnos. El resultado tiene un `.0` porque `/` siempre da un número
> decimal (mira [los números](nombres.md)).
>
> Prueba: añade `-40` a la lista. Obtienes `-40 °C = -40.0 °F`: es la única
> temperatura idéntica en las dos unidades.

### Ejercicio 3: La palabra más larga

Nivel: ★☆☆

En la frase `"el gato come un ratón"`, encuentra la palabra más larga, con un bucle,
sin usar `max`.

```text
ratón
```

> **Pista:** `frase.split()` corta la frase en una lista de palabras. Guarda en
> `mas_larga` la palabra más larga vista hasta ahora; al principio, un texto vacío `""`.

> **Solución**
>
> ```python
> frase = "el gato come un ratón"
> mas_larga = ""
> for palabra in frase.split():
>     if len(palabra) > len(mas_larga):
>         mas_larga = palabra
> print(mas_larga)
> ```
>
> ```sortie
> ratón
> ```
>
> Por qué funciona: es un concurso. Cada palabra se enfrenta al campeón actual; si es
> más larga, ocupa su lugar.
>
> | `palabra` | `len(palabra)` | `len(mas_larga)` antes | ¿Más larga? | `mas_larga` después |
> |---|---|---|---|---|
> | el | 2 | 0 | sí | el |
> | gato | 4 | 2 | sí | gato |
> | come | 4 | 4 | no | gato |
> | un | 2 | 4 | no | gato |
> | ratón | 5 | 4 | sí | ratón |
>
> Prueba: cambia `ratón` por `pez`. El programa muestra `gato`. En caso de empate, la
> primera palabra conserva su lugar, porque `>` es estricto: por eso `come` no gana a
> `gato`.

### Ejercicio 4: Dar la vuelta a una lista

Nivel: ★☆☆

Parte de la lista `["a", "b", "c", "d"]`. Construye una lista **nueva** al revés, con
un bucle, sin usar `reversed` ni `[::-1]`.

```text
['d', 'c', 'b', 'a']
```

> **Pista:** `lista.insert(0, x)` añade `x` **al principio** de la lista, delante de
> todos los demás (mira [las colecciones](collections.md)).

> **Solución**
>
> ```python
> letras = ["a", "b", "c", "d"]
> al_reves = []
> for letra in letras:
>     al_reves.insert(0, letra)
> print(al_reves)
> ```
>
> ```sortie
> ['d', 'c', 'b', 'a']
> ```
>
> Por qué funciona: piensa en una pila de platos. Cada plato nuevo se pone **delante**
> de los otros; el último que pones acaba el primero.
>
> | `letra` | `al_reves` después |
> |---|---|
> | a | `['a']` |
> | b | `['b', 'a']` |
> | c | `['c', 'b', 'a']` |
> | d | `['d', 'c', 'b', 'a']` |

## Nivel ★★☆: combinar varias ideas

> **Recuerda:** en este nivel, divide el problema. Escribe primero una pequeña función
> que haga una sola cosa, pruébala y luego júntalo todo.

### Ejercicio 5: El año bisiesto

Nivel: ★★☆

Un año es bisiesto (tiene un 29 de febrero) si es divisible entre 4, **salvo** si es
divisible entre 100, **salvo** si es divisible entre 400. Escribe
`es_bisiesto(anio)` y prueba 2024, 2023, 1900 y 2000.

```text
2024 True
2023 False
1900 False
2000 True
```

> **Pista:** `anio % 4 == 0` comprueba «divisible entre 4». Empieza por el caso más
> raro: 400, luego 100, luego 4.

> **Solución**
>
> ```python
> def es_bisiesto(anio):
>     if anio % 400 == 0:
>         return True
>     if anio % 100 == 0:
>         return False
>     return anio % 4 == 0
>
> for anio in [2024, 2023, 1900, 2000]:
>     print(anio, es_bisiesto(anio))
> ```
>
> ```sortie
> 2024 True
> 2023 False
> 1900 False
> 2000 True
> ```
>
> Por qué funciona: el primer `return` que se alcanza detiene la función. Por eso se
> comprueba primero la excepción más fuerte.
>
> | `anio` | ¿`% 400 == 0`? | ¿`% 100 == 0`? | ¿`% 4 == 0`? | Resultado |
> |---|---|---|---|---|
> | 2024 | no | no | sí | True |
> | 2023 | no | no | no | False |
> | 1900 | no | sí | (no se comprueba) | False |
> | 2000 | sí | (no se comprueba) | (no se comprueba) | True |

### Ejercicio 6: El cajero automático

Nivel: ★★☆

Un cajero tiene billetes de 50, 20 y 10 €. Siempre da el menor número de billetes
posible: primero los más grandes. Escribe `entregar(importe)` y pruébalo con 180 €.

```text
3 x 50 €
1 x 20 €
1 x 10 €
```

> **Pista:** para cada billete, `importe // billete` da cuántos se pueden dar, e
> `importe % billete` lo que queda por pagar.

> **Solución**
>
> ```python
> def entregar(importe):
>     for billete in [50, 20, 10]:
>         cantidad = importe // billete
>         importe = importe % billete
>         if cantidad > 0:
>             print(f"{cantidad} x {billete} €")
>
> entregar(180)
> ```
>
> ```sortie
> 3 x 50 €
> 1 x 20 €
> 1 x 10 €
> ```
>
> Por qué funciona: en cada vuelta, se toman todos los billetes posibles del valor
> actual y luego se sigue con lo que queda.
>
> | `billete` | `importe` antes | `cantidad` | `importe` después |
> |---|---|---|---|
> | 50 | 180 | 3 | 30 |
> | 20 | 30 | 1 | 10 |
> | 10 | 10 | 1 | 0 |
>
> ```text
> 180 €  →  [50][50][50]  quedan 30  →  [20]  quedan 10  →  [10]  quedan 0
> ```
>
> Prueba: `entregar(90)`. No hay línea para un billete dado 0 veces, gracias al
> `if cantidad > 0`.

### Ejercicio 7: Quitar los duplicados

Nivel: ★★☆

Escribe `sin_duplicados(elementos)` que devuelva una lista sin duplicados, **manteniendo
el orden** de la primera aparición. Pruébala con `[3, 1, 3, 2, 1, 4]` y
`["rojo", "verde", "rojo"]`.

```text
[3, 1, 2, 4]
['rojo', 'verde']
```

> **Pista:** guarda un [conjunto](../debuter/glossaire.md#conjunto) `ya_vistos` con los
> elementos que ya has encontrado. Un conjunto responde muy rápido a la pregunta
> `x in ya_vistos`.

> **Solución**
>
> ```python
> def sin_duplicados(elementos):
>     ya_vistos = set()
>     resultado = []
>     for e in elementos:
>         if e not in ya_vistos:
>             ya_vistos.add(e)
>             resultado.append(e)
>     return resultado
>
> print(sin_duplicados([3, 1, 3, 2, 1, 4]))
> print(sin_duplicados(["rojo", "verde", "rojo"]))
> ```
>
> ```sortie
> [3, 1, 2, 4]
> ['rojo', 'verde']
> ```
>
> Por qué funciona: es el portero de una fiesta con su lista de invitados que ya han
> entrado. Quien ya ha entrado no entra una segunda vez.
>
> | `e` | ¿Ya visto? | `resultado` después |
> |---|---|---|
> | 3 | no | `[3]` |
> | 1 | no | `[3, 1]` |
> | 3 | sí | `[3, 1]` |
> | 2 | no | `[3, 1, 2]` |
> | 1 | sí | `[3, 1, 2]` |
> | 4 | no | `[3, 1, 2, 4]` |
>
> `list(set(elementos))` también quita los duplicados, pero no garantiza el orden.

### Ejercicio 8: El cifrado César

Nivel: ★★☆

Julio César ocultaba sus mensajes desplazando cada letra 3 posiciones en el alfabeto:
`a` se convierte en `d`, `b` en `e`… y `z` vuelve a `c`. Escribe
`cesar(texto, desplazamiento)` para las letras minúsculas; los demás caracteres se
quedan como están. Cifra `"hola zoe"` y luego descifra el resultado.

```text
krod crh
hola zoe
```

> **Pista:** `alfabeto.index(letra)` da la posición de una letra (0 para `a`). Para
> volver al principio después de `z`, toma el resto: `(posicion + desplazamiento) % 26`.

> **Solución**
>
> ```python
> def cesar(texto, desplazamiento):
>     alfabeto = "abcdefghijklmnopqrstuvwxyz"
>     resultado = ""
>     for letra in texto:
>         if letra in alfabeto:
>             posicion = alfabeto.index(letra)
>             resultado += alfabeto[(posicion + desplazamiento) % 26]
>         else:
>             resultado += letra
>     return resultado
>
> secreto = cesar("hola zoe", 3)
> print(secreto)
> print(cesar(secreto, -3))
> ```
>
> ```sortie
> krod crh
> hola zoe
> ```
>
> Por qué funciona: el alfabeto es como la esfera de un reloj. Después de la posición
> 25 (`z`), se vuelve a 0 (`a`). Para `z`: `(25 + 3) % 26` vale `2`, es `c`. Para
> descifrar, se desplaza en el otro sentido, `-3`.
>
> ```text
> a b c d e f ... w x y z
> 0 1 2 3 4 5 ... 22 23 24 25  →  después de 25, se vuelve a 0
> ```

### Ejercicio 9: Ordenar las palabras por inicial

Nivel: ★★☆

Ordena las palabras `["pera", "piña", "banana", "kiwi", "brócoli", "plátano"]` en un
diccionario, según su primera letra. Luego muestra cada grupo.

```text
p: pera, piña, plátano
b: banana, brócoli
k: kiwi
```

> **Pista:** la clave es `palabra[0]`, el valor es una lista. Si la clave todavía no
> existe, crea primero una lista vacía (mira [los diccionarios](dictionnaires.md)).

> **Solución**
>
> ```python
> palabras = ["pera", "piña", "banana", "kiwi", "brócoli", "plátano"]
> grupos = {}
> for palabra in palabras:
>     inicial = palabra[0]
>     if inicial not in grupos:
>         grupos[inicial] = []
>     grupos[inicial].append(palabra)
>
> for inicial, lista in grupos.items():
>     print(f"{inicial}: {', '.join(lista)}")
> ```
>
> ```sortie
> p: pera, piña, plátano
> b: banana, brócoli
> k: kiwi
> ```
>
> Por qué funciona: como un archivador con separadores A, B, C… Se crea un separador
> la primera vez que se necesita y luego se guarda cada palabra detrás.
>
> `", ".join(lista)` pega las palabras de la lista con una coma y un espacio entre
> ellas.

## Nivel ★★★: para ir más lejos

> **Recuerda:** para un ejercicio difícil, haz la tabla de seguimiento **antes** de
> escribir el código, con un ejemplo pequeño. El código sale luego solo.

### Ejercicio 10: Buscar en una lista ordenada

Nivel: ★★★

Para encontrar una palabra en un diccionario de papel, no lees todas las páginas: lo
abres por la mitad y luego te quedas con la mitad buena. Escribe
`buscar(lista, objetivo)` que haga lo mismo con una lista **ordenada**, y devuelva la
posición de `objetivo`, o `None`. Muestra cada paso.

```text
bajo=0 alto=7 medio=3 -> 19
bajo=4 alto=7 medio=5 -> 31
posición: 5
bajo=0 alto=7 medio=3 -> 19
bajo=0 alto=2 medio=1 -> 8
bajo=0 alto=0 medio=0 -> 3
posición: None
```

> **Pista:** guarda dos límites, `bajo` y `alto`, que encierran la zona donde todavía
> puede estar el objetivo. El medio es `(bajo + alto) // 2`. Repite mientras
> `bajo <= alto`.

> **Solución**
>
> ```python
> def buscar(lista, objetivo):
>     bajo = 0
>     alto = len(lista) - 1
>     while bajo <= alto:
>         medio = (bajo + alto) // 2
>         print(f"bajo={bajo} alto={alto} medio={medio} -> {lista[medio]}")
>         if lista[medio] == objetivo:
>             return medio
>         if lista[medio] < objetivo:
>             bajo = medio + 1
>         else:
>             alto = medio - 1
>     return None
>
> numeros = [3, 8, 12, 19, 25, 31, 40, 47]
> print("posición:", buscar(numeros, 31))
> print("posición:", buscar(numeros, 5))
> ```
>
> ```sortie
> bajo=0 alto=7 medio=3 -> 19
> bajo=4 alto=7 medio=5 -> 31
> posición: 5
> bajo=0 alto=7 medio=3 -> 19
> bajo=0 alto=2 medio=1 -> 8
> bajo=0 alto=0 medio=0 -> 3
> posición: None
> ```
>
> Por qué funciona: en cada vuelta, la zona de búsqueda se corta en dos. Para buscar 5:
>
> | Vuelta | `bajo` | `alto` | `medio` | `lista[medio]` | Decisión |
> |---|---|---|---|---|---|
> | 1 | 0 | 7 | 3 | 19 | 19 > 5: `alto = 2` |
> | 2 | 0 | 2 | 1 | 8 | 8 > 5: `alto = 0` |
> | 3 | 0 | 0 | 0 | 3 | 3 < 5: `bajo = 1` |
> | — | 1 | 0 | | | `1 <= 0` falso: fin, `None` |
>
> ```text
> [3, 8, 12, 19, 25, 31, 40, 47]
>  ^           ^              ^
> bajo        medio          alto     vuelta 1: 19 muy grande, queda la izquierda
> ```
>
> Con 1000 números, hacen falta como mucho 10 vueltas, en lugar de 1000 leyéndolo todo.

### Ejercicio 11: Comprimir un texto

Nivel: ★★★

Un texto como `"aaabccdddd"` contiene letras repetidas. Comprímelo escribiendo cada
letra seguida del número de repeticiones: `"a3b1c2d4"`. Escribe `comprimir` y
`descomprimir`, que hace lo contrario. Suponemos que una letra se repite como mucho 9
veces.

```text
a3b1c2d4
aaabccdddd
```

> **Pista:** guarda la letra `actual` y su `cuenta`. Cuando la letra cambia, escribe
> `actual` y `cuenta` en el resultado y vuelve a empezar en 1. No olvides el último
> grupo después del bucle.

> **Pista:** para descomprimir, lee el código de dos en dos caracteres:
> `range(0, len(codigo), 2)`. `"d" * 4` da `"dddd"`.

> **Solución**
>
> ```python
> def comprimir(texto):
>     if texto == "":
>         return ""
>     resultado = ""
>     actual = texto[0]
>     cuenta = 0
>     for letra in texto:
>         if letra == actual:
>             cuenta += 1
>         else:
>             resultado += f"{actual}{cuenta}"
>             actual = letra
>             cuenta = 1
>     resultado += f"{actual}{cuenta}"
>     return resultado
>
> def descomprimir(codigo):
>     resultado = ""
>     for i in range(0, len(codigo), 2):
>         resultado += codigo[i] * int(codigo[i + 1])
>     return resultado
>
> c = comprimir("aaabccdddd")
> print(c)
> print(descomprimir(c))
> ```
>
> ```sortie
> a3b1c2d4
> aaabccdddd
> ```
>
> Por qué funciona: se cuentan las letras idénticas que van seguidas, como se cuentan
> los vagones del mismo color en un tren.
>
> | `letra` | ¿Igual que `actual`? | `actual` | `cuenta` | `resultado` |
> |---|---|---|---|---|
> | a | sí | a | 1 | `""` |
> | a | sí | a | 2 | `""` |
> | a | sí | a | 3 | `""` |
> | b | no | b | 1 | `"a3"` |
> | c | no | c | 1 | `"a3b1"` |
> | c | sí | c | 2 | `"a3b1"` |
> | d | no | d | 1 | `"a3b1c2"` |
> | d, d, d | sí | d | 4 | `"a3b1c2"` |
> | después del bucle | | | | `"a3b1c2d4"` |
>
> La última fila de la tabla muestra por qué hace falta la línea después del bucle: sin
> ella, el grupo `d4` se perdería.

### Ejercicio 12: Los paréntesis equilibrados

Nivel: ★★★

Escribe `equilibrado(texto)` que diga si los signos `()`, `[]` y `{}` de un texto están
bien cerrados, en el orden correcto. Prueba cuatro textos.

```text
(a[b]{c})    True
(]           False
((x)         False
f(x) + [y]   True
```

> **Pista:** usa una lista como una **pila** de platos: `append` pone una apertura
> encima, `pop()` recupera la de encima.

> **Pista:** para un cierre, la pila no debe estar vacía y su parte de arriba debe ser
> la apertura correspondiente. Al final, la pila debe estar vacía.

> **Solución**
>
> ```python
> def equilibrado(texto):
>     pares = {")": "(", "]": "[", "}": "{"}
>     pila = []
>     for signo in texto:
>         if signo in "([{":
>             pila.append(signo)
>         elif signo in pares:
>             if not pila or pila.pop() != pares[signo]:
>                 return False
>     return pila == []
>
> for prueba in ["(a[b]{c})", "(]", "((x)", "f(x) + [y]"]:
>     print(f"{prueba:<12} {equilibrado(prueba)}")
> ```
>
> ```sortie
> (a[b]{c})    True
> (]           False
> ((x)         False
> f(x) + [y]   True
> ```
>
> Por qué funciona: el último paréntesis abierto debe ser el primero cerrado, como
> cajas guardadas unas dentro de otras. Para `"(a[b]{c})"`:
>
> | `signo` | Acción | `pila` después |
> |---|---|---|
> | ( | se pone | `['(']` |
> | a | nada | `['(']` |
> | [ | se pone | `['(', '[']` |
> | b | nada | `['(', '[']` |
> | ] | se recupera `[`: coincide | `['(']` |
> | { | se pone | `['(', '{']` |
> | c | nada | `['(', '{']` |
> | } | se recupera `{`: coincide | `['(']` |
> | ) | se recupera `(`: coincide | `[]` |
>
> La pila está vacía al final: `True`. Para `"((x)"`, queda un `(` en la pila, así que
> `False`.

## En resumen

- Los ejercicios 1 a 4 repasan los bucles, las funciones y las listas.
- Los ejercicios 5 a 9 combinan condiciones, divisiones enteras, conjuntos, texto y
  diccionarios.
- Los ejercicios 10 a 12 piden hacer una tabla de seguimiento antes de escribir el
  código.
- Cuando una solución te sorprenda, copia su tabla a mano: ahí es donde se entiende.

Siguiente: ten [la chuleta](aide-memoire.md) a mano.
