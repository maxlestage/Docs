# Las comprensiones

Esta página muestra cómo construir una lista, un diccionario o un conjunto en una sola línea gracias a las comprensiones, y cuándo es mejor no usarlas.

> **Objetivos:**
>
> - sabrás transformar un bucle `for` en una comprensión de lista;
> - sabrás filtrar elementos con `if`;
> - sabrás escribir comprensiones de diccionario y de conjunto;
> - sabrás usar una expresión generadora con `sum`, `any` y `all`;
> - sabrás reconocer una comprensión demasiado complicada.

## Del bucle a la comprensión

Ya sabes construir una nueva [lista](../debuter/glossaire.md#lista) con un bucle `for`:
se parte de una lista vacía y luego se añade un elemento en cada vuelta.

```python
numeros = [1, 2, 3, 4]
cuadrados = []
for n in numeros:
    cuadrados.append(n * n)
print(cuadrados)
```

```sortie
[1, 4, 9, 16]
```

Este patrón es tan común que Python tiene una forma más corta de escribirlo: la
**comprensión de lista**. Se construye en tres pasos, a partir del bucle:

1. escribe corchetes `[ ]`: el resultado será una lista;
2. pon primero **lo que se añade**: `n * n`;
3. copia después la línea `for` sin los dos puntos: `for n in numeros`.

```python
numeros = [1, 2, 3, 4]
cuadrados = [n * n for n in numeros]
print(cuadrados)
```

```sortie
[1, 4, 9, 16]
```

Léela en voz alta: «la lista de los `n * n`, para cada `n` en `numeros`». Es casi una
frase en español.

> **Recuerda:** una comprensión crea **siempre una lista nueva**. La lista de partida no
> se modifica.

### Cualquier expresión

A la izquierda del `for`, puedes poner cualquier
[expresión](../debuter/glossaire.md#expresion): un cálculo, una llamada a un método, un
f-string…

```python
nombres = ["alice", "bob", "chloé"]
print([n.capitalize() for n in nombres])
print([len(n) for n in nombres])
print([f"¡Hola {n}!" for n in nombres])
```

```sortie
['Alice', 'Bob', 'Chloé']
[5, 3, 5]
['¡Hola alice!', '¡Hola bob!', '¡Hola chloé!']
```

## Filtrar con if

A menudo solo se conservan algunos elementos. Con un bucle, se escribe un `if` dentro
del `for`:

```python
notas = [12, 7, 15, 9, 18]
aprobadas = []
for nota in notas:
    if nota >= 10:
        aprobadas.append(nota)
print(aprobadas)
```

```sortie
[12, 15, 18]
```

En una comprensión, el `if` va **al final**, después del `for`:

```python
notas = [12, 7, 15, 9, 18]
aprobadas = [nota for nota in notas if nota >= 10]
print(aprobadas)
```

```sortie
[12, 15, 18]
```

Es un colador: solo los elementos que pasan la prueba llegan a la nueva lista.

### Transformar según una condición

Para **conservar todos los elementos** pero transformarlos de forma distinta, usa la
expresión condicional `a if condición else b`, colocada **antes** del `for`:

```python
notas = [12, 7, 15, 9]
resultados = ["aprobado" if nota >= 10 else "suspenso" for nota in notas]
print(resultados)
```

```sortie
['aprobado', 'suspenso', 'aprobado', 'suspenso']
```

> **Atención:** los dos lugares no significan lo mismo. Un `if` **después** del `for`
> filtra (la lista puede acortarse). Un `if … else` **antes** del `for` elige un valor
> (la lista conserva la misma longitud).

## Comprensiones de diccionario

El mismo principio funciona para un
[diccionario](../debuter/glossaire.md#diccionario). Se usan llaves `{ }` y se escribe
`clave: valor` antes del `for`.

```python
nombres = ["alice", "bob", "chloé"]
longitudes = {n: len(n) for n in nombres}
print(longitudes)
```

```sortie
{'alice': 5, 'bob': 3, 'chloé': 5}
```

Para recorrer un diccionario existente, usa `.items()`, que da los pares
`(clave, valor)`:

```python
precios = {"manzana": 2.0, "pera": 3.0, "kiwi": 0.5}
rebajas = {fruta: p * 0.8 for fruta, p in precios.items()}
baratas = {fruta: p for fruta, p in precios.items() if p < 2.5}
print(rebajas)
print(baratas)
```

```sortie
{'manzana': 1.6, 'pera': 2.4000000000000004, 'kiwi': 0.4}
{'manzana': 2.0, 'kiwi': 0.5}
```

> **Nota:** `2.4000000000000004` no es un error de la comprensión: es el redondeo de los
> números decimales (consulta [los números](nombres.md)). Usa `round(p * 0.8, 2)` para
> una presentación limpia.

### Invertir un diccionario

Un uso clásico: intercambiar las claves y los valores.

```python
codigos = {"FR": "Francia", "BE": "Bélgica"}
pais_a_codigo = {nombre: codigo for codigo, nombre in codigos.items()}
print(pais_a_codigo)
```

```sortie
{'Francia': 'FR', 'Bélgica': 'BE'}
```

## Comprensiones de conjunto

Con llaves pero **sin** dos puntos, se obtiene un
[conjunto](../debuter/glossaire.md#conjunto) (`set`): cada valor aparece en él una sola
vez.

```python
palabras = ["Gato", "perro", "GATO", "Pez", "perro"]
unicas = {p.lower() for p in palabras}
print(sorted(unicas))
```

```sortie
['gato', 'perro', 'pez']
```

Un conjunto no tiene orden: se ordena con `sorted` para obtener una presentación
estable.

| Escritura | Resultado |
|----------|----------|
| `[x for x in ...]` | una lista |
| `{x for x in ...}` | un conjunto |
| `{k: v for ... in ...}` | un diccionario |
| `(x for x in ...)` | una expresión generadora (ver más abajo) |

## Comprensiones anidadas

Una comprensión puede contener **varios `for`**. Se leen en el mismo orden que unos
bucles anidados: el primer `for` es el bucle exterior.

```python
colores = ["rojo", "azul"]
tallas = ["S", "M"]
articulos = [f"{c}-{t}" for c in colores for t in tallas]
print(articulos)
```

```sortie
['rojo-S', 'rojo-M', 'azul-S', 'azul-M']
```

Es el equivalente de:

```python
colores = ["rojo", "azul"]
tallas = ["S", "M"]
articulos = []
for c in colores:
    for t in tallas:
        articulos.append(f"{c}-{t}")
print(articulos)
```

```sortie
['rojo-S', 'rojo-M', 'azul-S', 'azul-M']
```

### Aplanar una lista de listas

Dos `for` sirven a menudo para «aplanar» una tabla en una sola lista:

```python
cuadricula = [[1, 2, 3], [4, 5, 6]]
plana = [n for fila in cuadricula for n in fila]
print(plana)
```

```sortie
[1, 2, 3, 4, 5, 6]
```

### Una comprensión dentro de una comprensión

Al revés, una comprensión colocada **a la izquierda** de otra construye una lista de
listas. Aquí, una tabla de multiplicar:

```python
tabla = [[fila * col for col in range(1, 4)] for fila in range(1, 4)]
for fila in tabla:
    print(fila)
```

```sortie
[1, 2, 3]
[2, 4, 6]
[3, 6, 9]
```

## Las expresiones generadoras

Con **paréntesis** en lugar de corchetes, obtienes una **expresión generadora**. No
construye la lista en memoria: produce los valores uno a uno, en el momento en que se
piden. Es un grifo más que un cubo lleno de antemano.

Es ideal con las funciones que consumen valores: `sum`, `min`, `max`, `any`, `all`.
Cuando es el único argumento, bastan los paréntesis de la función.

```python
notas = [12, 7, 15, 9, 18]
print(sum(n * 2 for n in notas))
print(max(len(p) for p in ["uno", "tres", "cuatro"]))
```

```sortie
122
6
```

### any y all

- `any(...)` vale `True` si **al menos un** valor es verdadero;
- `all(...)` vale `True` si **todos** los valores son verdaderos.

```python
notas = [12, 7, 15, 9, 18]
print(any(n >= 18 for n in notas))
print(all(n >= 10 for n in notas))
print(all(n >= 0 for n in notas))
```

```sortie
True
False
True
```

Estas dos funciones se detienen en cuanto conocen la respuesta: `any` se detiene en el
primer `True`, `all` en el primer `False`.

> **Consejo:** una expresión generadora solo se recorre **una vez**. Una vez vacía, ya
> no devuelve nada. Para reutilizar los valores, haz una lista. El capítulo
> [iteradores y generadores](iterateurs.md) explica por qué.

```python
cuadrados = (n * n for n in range(4))
print(sum(cuadrados))
print(sum(cuadrados))
```

```sortie
14
0
```

## Saber cuándo parar

Una comprensión debe seguir siendo **legible de un vistazo**. Esta es correcta, pero
difícil de releer:

```python
resultado = [x * y for x in range(1, 5) if x % 2 == 0 for y in range(1, 5) if y > x]
print(resultado)
```

```sortie
[6, 8]
```

Lo mismo con bucles es más largo, pero cada paso está claro:

```python
resultado = []
for x in range(1, 5):
    if x % 2 == 0:
        for y in range(1, 5):
            if y > x:
                resultado.append(x * y)
print(resultado)
```

```sortie
[6, 8]
```

Algunas reglas sencillas:

- **un** bucle y **un** filtro: la comprensión es perfecta;
- dos `for`: aceptable si es corta;
- más allá, o si la línea se sale de la pantalla: vuelve a un bucle;
- si necesitas un `print` o una acción (no un valor), usa un bucle: una comprensión
  sirve para **construir** una colección.

También puedes sacar el cálculo a una [función](fonctions.md) con un nombre claro:

```python
def es_primo(n):
    return n > 1 and all(n % d != 0 for d in range(2, n))

print([n for n in range(20) if es_primo(n)])
```

```sortie
[2, 3, 5, 7, 11, 13, 17, 19]
```

## Errores frecuentes

### Poner un if … else al final

```python norun
numeros = [3, -1, 4]
positivos = [n for n in numeros if n > 0 else 0]
print(positivos)
```

```text
  File "main.py", line 2
    positivos = [n for n in numeros if n > 0 else 0]
                                             ^^^^
SyntaxError: invalid syntax
```

Un `if` colocado al final es un filtro: no acepta `else`. Para elegir un valor, coloca
la expresión antes del `for`: `[n if n > 0 else 0 for n in numeros]`.

### Olvidar los dos puntos de un diccionario

```python norun
precios = {"manzana": 2, "pera": 3}
dobles = {nombre, p * 2 for nombre, p in precios.items()}
print(dobles)
```

```text
  File "main.py", line 2
    dobles = {nombre, p * 2 for nombre, p in precios.items()}
              ^^^^^^^^^^^^^
SyntaxError: did you forget parentheses around the comprehension target?
```

Una coma entre la clave y el valor no crea un diccionario. Escribe
`{nombre: p * 2 for nombre, p in precios.items()}`.

### Usar la variable del bucle después

```python norun
cuadrados = [n * n for n in range(4)]
print(n)
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(n)
          ^
NameError: name 'n' is not defined
```

A diferencia de un bucle `for`, la variable de una comprensión solo existe **dentro**
de los corchetes. Usa el resultado (`cuadrados`), no la variable.

### Tomar un elemento de una expresión generadora

```python norun
cuadrados = (n * n for n in range(4))
print(cuadrados[0])
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(cuadrados[0])
          ~~~~~~~~~^^^
TypeError: 'generator' object is not subscriptable
```

Los paréntesis dan un generador, no una lista: no tiene índice. Usa corchetes
`[n * n for n in range(4)]` si necesitas `cuadrados[0]`.

## Ponte a prueba

**Pregunta 1:** reescribe este bucle como una comprensión.

```python norun
mayusculas = []
for palabra in ["sí", "no"]:
    mayusculas.append(palabra.upper())
```

> **Respuesta**
>
> ```python
> mayusculas = [palabra.upper() for palabra in ["sí", "no"]]
> print(mayusculas)
> ```
>
> ```sortie
> ['SÍ', 'NO']
> ```

**Pregunta 2:** ¿de qué tipo es `{x % 3 for x in range(10)}` y qué contiene?

> **Respuesta**
>
> Es un conjunto (`set`), porque tiene llaves sin dos puntos. Contiene `{0, 1, 2}`: los
> restos posibles, cada uno una sola vez.

**Pregunta 3:** ¿cómo saber si una lista `edades` contiene al menos una persona menor de
edad, sin construir una lista nueva?

> **Respuesta**
>
> Con `any` y una expresión generadora:
>
> ```python
> edades = [34, 17, 52]
> print(any(edad < 18 for edad in edades))
> ```
>
> ```sortie
> True
> ```

## En resumen

- `[expresión for x in colección]` construye una lista nueva a partir de otra.
- Un `if` al final filtra; un `a if condición else b` al principio elige un valor.
- `{k: v for ...}` construye un diccionario, `{x for ...}` un conjunto.
- `(x for ...)` es una expresión generadora: perfecta con `sum`, `any`, `all`, y
  utilizable una sola vez.
- Si la comprensión se vuelve difícil de leer, vuelve a un bucle `for`.

Siguiente: [Suivre un programme pas à pas](pas-a-pas.md)
