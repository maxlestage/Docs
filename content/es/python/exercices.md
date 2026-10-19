# Ejercicios resueltos

Esta página propone 12 ejercicios resueltos, del más fácil al más difícil, para practicar todo el recorrido de Python.

## Cómo usar esta página

Cada ejercicio tiene un nivel:

- ★☆☆: fácil;
- ★★☆: medio;
- ★★★: más difícil.

Lee el enunciado, luego escribe tu solución en un archivo `.py` y ejecútalo con
`python3`. Abre la solución solo después de haberlo intentado.

> **Consejo:** a menudo hay varias soluciones buenas. Si tu programa muestra el
> resultado correcto, ¡lo has conseguido!

## Lo básico

### Ejercicio 1: Preséntate

Nivel: ★☆☆

Crea dos variables, `nombre` y `edad`. Luego muestra la frase
`Me llamo Alice y tengo 30 años.`, con tus variables.

> **Pista:** usa una f-string: `f"... {nombre} ..."` (mira
> [los primeros pasos](premiers-pas.md)).

> **Solución**
>
> ```python
> nombre = "Alice"
> edad = 30
> print(f"Me llamo {nombre} y tengo {edad} años.")
> ```
>
> ```sortie
> Me llamo Alice y tengo 30 años.
> ```

### Ejercicio 2: Par o impar

Nivel: ★☆☆

Para el número `7`, muestra `7 es impar`. Para `10`, el programa debe mostrar
`10 es par`.

> **Pista:** `n % 2` da el resto de la división entre 2: `0` si `n` es par (mira
> [los números](nombres.md)).

> **Solución**
>
> ```python
> for n in [7, 10]:
>     if n % 2 == 0:
>         print(f"{n} es par")
>     else:
>         print(f"{n} es impar")
> ```
>
> ```sortie
> 7 es impar
> 10 es par
> ```

### Ejercicio 3: La tabla del 7

Nivel: ★☆☆

Muestra la tabla de multiplicar del 7, de `7 x 1 = 7` a `7 x 5 = 35`.

> **Pista:** `range(1, 6)` da los números del 1 al 5 (el 6 queda excluido).

> **Solución**
>
> ```python
> for i in range(1, 6):
>     print(f"7 x {i} = {7 * i}")
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

### Ejercicio 4: El área de un rectángulo

Nivel: ★☆☆

Escribe una función `area(ancho, alto)` que devuelva el área de un rectángulo. Si no
se da el alto, el rectángulo es un cuadrado: el alto vale lo mismo que el ancho.
Muestra `area(3, 4)` y `area(5)`.

> **Pista:** dale a `alto` el valor por defecto `None` y luego compruébalo (mira
> [las funciones](fonctions.md)).

> **Solución**
>
> ```python
> def area(ancho, alto=None):
>     if alto is None:
>         alto = ancho
>     return ancho * alto
>
>
> print(area(3, 4))
> print(area(5))
> ```
>
> ```sortie
> 12
> 25
> ```

## Texto y colecciones

### Ejercicio 5: La media

Nivel: ★☆☆

Escribe una función `media(notas)` que devuelva la media de una lista, redondeada a
un decimal, o `None` si la lista está vacía.

> **Pista:** `sum`, `len` y `round` hacen todo el trabajo (mira
> [las colecciones](collections.md)).

> **Solución**
>
> ```python
> def media(notas):
>     if not notas:
>         return None
>     return round(sum(notas) / len(notas), 1)
>
>
> print(media([12, 15, 17]))
> print(media([]))
> ```
>
> ```sortie
> 14.7
> None
> ```

### Ejercicio 6: Contar las vocales

Nivel: ★★☆

Escribe una función `vocales(texto)` que cuente las vocales (`a e i o u`) de un
texto, sin tener en cuenta las mayúsculas.

> **Pista:** `texto.lower()` lo pone todo en minúsculas, y `letra in "aeiou"` comprueba
> si una letra es una vocal (mira [trabajar con texto](texte.md)).

> **Solución**
>
> ```python
> def vocales(texto):
>     return sum(1 for letra in texto.lower() if letra in "aeiou")
>
>
> print(vocales("Hola"))
> print(vocales("PYTHON"))
> ```
>
> ```sortie
> 2
> 1
> ```

### Ejercicio 7: El palíndromo

Nivel: ★★☆

Un palíndromo se lee igual en los dos sentidos, como `"Radar"` o
`"Anita lava la tina"`. Escribe `es_palindromo(texto)`, que no tenga en cuenta las
mayúsculas ni los espacios.

> **Pista:** `texto[::-1]` devuelve el texto al revés; `replace(" ", "")` quita los
> espacios.

> **Solución**
>
> ```python
> def es_palindromo(texto):
>     limpio = texto.lower().replace(" ", "")
>     return limpio == limpio[::-1]
>
>
> print(es_palindromo("Radar"))
> print(es_palindromo("Anita lava la tina"))
> print(es_palindromo("Python"))
> ```
>
> ```sortie
> True
> True
> False
> ```

### Ejercicio 8: Contar las palabras

Nivel: ★★☆

Cuenta cuántas veces aparece cada palabra en la frase
`"el gato y el perro y el conejo"`, y luego muestra las palabras de la más frecuente a
la menos frecuente.

> **Pista:** un [diccionario](dictionnaires.md) asocia cada palabra a su contador;
> `contadores.get(palabra, 0)` vale 0 para una palabra nueva. `collections.Counter` hace
> lo mismo en una línea.

> **Solución**
>
> ```python
> from collections import Counter
>
> frase = "el gato y el perro y el conejo"
>
> contadores = {}
> for palabra in frase.split():
>     contadores[palabra] = contadores.get(palabra, 0) + 1
>
> for palabra, cantidad in sorted(contadores.items(), key=lambda par: -par[1]):
>     print(f"{palabra}: {cantidad}")
>
> print(Counter(frase.split()).most_common(2))
> ```
>
> ```sortie
> el: 3
> y: 2
> gato: 1
> perro: 1
> conejo: 1
> [('el', 3), ('y', 2)]
> ```

### Ejercicio 9: Conservar y duplicar

Nivel: ★★☆

A partir de la lista `[3, -1, 4, 0, -5, 9]`, construye en una sola línea la lista de los
números positivos, multiplicados por dos. Construye también un diccionario que asocie
cada número positivo a su cuadrado.

> **Pista:** una [comprensión](comprehensions.md) filtra con `if` y transforma al mismo
> tiempo.

> **Solución**
>
> ```python
> numeros = [3, -1, 4, 0, -5, 9]
>
> print([n * 2 for n in numeros if n > 0])
> print({n: n * n for n in numeros if n > 0})
> ```
>
> ```sortie
> [6, 8, 18]
> {3: 9, 4: 16, 9: 81}
> ```

## Para ir más lejos

### Ejercicio 10: FizzBuzz

Nivel: ★★☆

Para los números del 1 al 15, muestra `Fizz` si el número es divisible entre 3, `Buzz`
si es divisible entre 5, `FizzBuzz` si es divisible entre los dos, y el número en los
demás casos. Muéstralo todo en una línea, separado por espacios.

> **Pista:** comprueba primero «divisible entre 15»; si no, ese caso no llega nunca.

> **Solución**
>
> ```python
> def fizzbuzz(n):
>     if n % 15 == 0:
>         return "FizzBuzz"
>     if n % 3 == 0:
>         return "Fizz"
>     if n % 5 == 0:
>         return "Buzz"
>     return str(n)
>
>
> print(" ".join(fizzbuzz(n) for n in range(1, 16)))
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```

### Ejercicio 11: La cuenta bancaria

Nivel: ★★★

Escribe una clase `Cuenta` con un `titular` y un `saldo` (0 al principio), y dos
métodos: `depositar(importe)` y `retirar(importe)`. Un importe negativo o nulo lanza
un `ValueError`, y una retirada mayor que el saldo también. Pruébalo con `try` /
`except`.

> **Pista:** mira [las clases](types.md) y [los errores](erreurs.md). `raise
> ValueError("mensaje")` lanza el error.

> **Solución**
>
> ```python
> class Cuenta:
>     def __init__(self, titular):
>         self.titular = titular
>         self.saldo = 0
>
>     def depositar(self, importe):
>         if importe <= 0:
>             raise ValueError("el importe debe ser positivo")
>         self.saldo += importe
>
>     def retirar(self, importe):
>         if importe <= 0:
>             raise ValueError("el importe debe ser positivo")
>         if importe > self.saldo:
>             raise ValueError("saldo insuficiente")
>         self.saldo -= importe
>
>     def __str__(self):
>         return f"{self.titular}: {self.saldo} €"
>
>
> cuenta = Cuenta("Alice")
> cuenta.depositar(100)
> cuenta.retirar(30)
> print(cuenta)
>
> for importe in [-5, 500]:
>     try:
>         cuenta.retirar(importe)
>     except ValueError as error:
>         print(f"Rechazado ({importe}): {error}")
>
> print(cuenta)
> ```
>
> ```sortie
> Alice: 70 €
> Rechazado (-5): el importe debe ser positivo
> Rechazado (500): saldo insuficiente
> Alice: 70 €
> ```

### Ejercicio 12: Un inventario en JSON

Nivel: ★★★

Una tienda describe su stock en JSON:
`[{"nombre": "manzana", "precio": 0.5, "cantidad": 40}, {"nombre": "kiwi", "precio":
0.8, "cantidad": 0}, {"nombre": "pera", "precio": 0.7, "cantidad": 12}]`.

Lee este texto, convierte cada artículo en una `dataclass` `Articulo`, y luego muestra
los artículos en stock, ordenados por nombre, con su valor (`precio * cantidad`).
Termina con el valor total del stock.

> **Pista:** `json.loads` convierte el texto en una lista de diccionarios (mira
> [los formatos de datos](formats.md)); `Articulo(**d)` crea un objeto a partir de un
> diccionario cuyas claves son los nombres de los campos.

> **Solución**
>
> ```python
> import json
> from dataclasses import dataclass
>
> TEXTO = """[
>   {"nombre": "manzana", "precio": 0.5, "cantidad": 40},
>   {"nombre": "kiwi", "precio": 0.8, "cantidad": 0},
>   {"nombre": "pera", "precio": 0.7, "cantidad": 12}
> ]"""
>
>
> @dataclass
> class Articulo:
>     nombre: str
>     precio: float
>     cantidad: int
>
>     def valor(self) -> float:
>         return self.precio * self.cantidad
>
>
> articulos = [Articulo(**d) for d in json.loads(TEXTO)]
> en_stock = sorted((a for a in articulos if a.cantidad > 0), key=lambda a: a.nombre)
>
> for articulo in en_stock:
>     print(f"{articulo.nombre:<7} {articulo.cantidad:>3} x {articulo.precio:.2f} = {articulo.valor():6.2f}")
>
> total = sum(articulo.valor() for articulo in articulos)
> print(f"Valor del stock: {total:.2f} €")
> ```
>
> ```sortie
> manzana  40 x 0.50 =  20.00
> pera     12 x 0.70 =   8.40
> Valor del stock: 28.40 €
> ```

## En resumen

- Empieza por los ejercicios ★☆☆ y luego sube de dificultad.
- Inténtalo siempre por tu cuenta antes de abrir la solución.
- Los ejercicios 1 a 4 repasan lo básico: variables, condiciones, bucles, funciones.
- Los ejercicios 5 a 9 trabajan el texto, las listas, los diccionarios y las
  comprensiones.
- Los ejercicios 10 a 12 lo combinan todo, con las clases, los errores, las dataclasses
  y JSON.

Siguiente: [ejercicios resueltos: segunda serie](exercices-2.md)
