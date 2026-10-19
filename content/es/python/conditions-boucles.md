# Condiciones y bucles

Esta página explica cómo un programa de Python toma decisiones con `if` y `match`, y repite acciones con `while` y `for`.

> **Objetivos:**
>
> - sabrás escribir una condición con `if`, `elif` y `else`;
> - sabrás combinar pruebas con `and`, `or` y `not`;
> - sabrás elegir entre varios casos con `match`;
> - sabrás repetir código con `while` y `for`;
> - sabrás recorrer una lista con `enumerate` y `zip`.

## La condición if

Una [condición](../debuter/glossaire.md#condicion) permite ejecutar código solo si
algo es verdadero. Es como una señal de tráfico: «si la carretera está mojada, reduce la velocidad».

```python
temperatura = 30
if temperatura > 25:
    print("Hace calor.")
    print("Acuérdate de beber agua.")
print("¡Que tengas un buen día!")
```

```sortie
Hace calor.
Acuérdate de beber agua.
¡Que tengas un buen día!
```

Tres reglas que recordar:

- la línea `if` termina con **dos puntos** `:`;
- las líneas que dependen del `if` llevan **sangría** (desplazadas 4 espacios): forman
  un **bloque**;
- la primera línea sin desplazar (`print("¡Que tengas un buen día!")`) está fuera del bloque:
  se ejecuta siempre.

> **Recuerda:** en Python, la sangría no es decorativa. Es la que dice
> qué líneas forman parte del bloque. Usa siempre 4 espacios.

## else y elif

`else` («si no») da el camino que seguir cuando la condición es falsa. `elif`
(contracción de *else if*, «si no, si») añade otras pruebas.

```python
nota = 7
if nota >= 9:
    print("Sobresaliente")
elif nota >= 7:
    print("Notable")
elif nota >= 5:
    print("Aprobado")
else:
    print("Suspenso")
```

```sortie
Notable
```

Python prueba las condiciones **en orden** y se detiene en la primera verdadera. Aquí,
`nota >= 9` es falso y `nota >= 7` es verdadero: muestra «Notable» y se salta el resto.

## Comparar valores

| Operador | Significado |
| --- | --- |
| `==` | igual a |
| `!=` | distinto de |
| `<`, `>` | menor, mayor |
| `<=`, `>=` | menor o igual, mayor o igual |

Una comparación da un [booleano](../debuter/glossaire.md#booleano): `True` o
`False`.

```python
edad = 17
print(edad == 18)
print(edad != 18)
print(edad < 18)
print("gato" == "gato")
print(10 <= edad < 20)
```

```sortie
False
True
True
True
True
```

La última línea muestra una comodidad de Python: `10 <= edad < 20` significa «`edad` está
entre 10 (incluido) y 20 (no incluido)», como en matemáticas.

> **Atención:** `=` guarda un valor en una variable. `==` compara dos valores.
> No los confundas.

## Combinar: and, or, not

- `a and b` es verdadero si `a` **y** `b` son verdaderos;
- `a or b` es verdadero si **al menos uno** de los dos es verdadero;
- `not a` invierte: verdadero pasa a falso, y falso pasa a verdadero.

```python
edad = 25
tiene_entrada = True
es_vip = False

if edad >= 18 and tiene_entrada:
    print("Entrada permitida")
if tiene_entrada or es_vip:
    print("Acceso a la sala")
if not es_vip:
    print("Sin acceso a la zona VIP")
```

```sortie
Entrada permitida
Acceso a la sala
Sin acceso a la zona VIP
```

## Verdadero o falso: la «veracidad»

En un `if`, Python acepta cualquier valor, no solo `True` o `False`.
Algunos valores cuentan como **falsos**:

- `False` y `None`;
- el cero: `0` y `0.0`;
- todo lo que está vacío: `""`, `[]`, `{}`.

Todo lo demás cuenta como **verdadero**.

```python
carrito = []
if carrito:
    print("Tienes artículos.")
else:
    print("Tu carrito está vacío.")

nombre = "Alice"
if nombre:
    print(f"¡Hola, {nombre}!")
```

```sortie
Tu carrito está vacío.
¡Hola, Alice!
```

`if carrito:` se lee «si el carrito contiene algo». Es más corto que
`if len(carrito) > 0:`.

## La expresión condicional

Para elegir entre dos valores en una sola línea, escribe
`valor_si_verdadero if condicion else valor_si_falso`.

```python
edad = 15
estado = "mayor de edad" if edad >= 18 else "menor de edad"
print(estado)

n = 7
print("par" if n % 2 == 0 else "impar")
```

```sortie
menor de edad
impar
```

> **Consejo:** reserva esta forma para elecciones sencillas. En cuanto es más larga, un
> `if` normal en varias líneas se lee mejor.

## Elegir entre varios casos: match

Desde Python 3.10, `match` compara un valor con varios **patrones** (*patterns*).
Es como un cambio de agujas de tren: el valor sale por la primera vía que
encaja.

```python
def describir(orden):
    match orden:
        case "norte" | "sur":
            return "vertical"
        case "este" | "oeste":
            return "horizontal"
        case "salir":
            return "adiós"
        case _:
            return "orden desconocida"

print(describir("norte"))
print(describir("oeste"))
print(describir("saltar"))
```

```sortie
vertical
horizontal
orden desconocida
```

- `|` significa «o»: `"norte" | "sur"` acepta uno u otro;
- `case _:` atrapa todo lo demás: es el caso por defecto.

`match` también sabe descomponer un valor. Aquí reconoce una lista de dos elementos y
guarda cada trozo en una variable:

```python
for accion in [["ir", "norte"], ["coger", "llave"], ["dormir"]]:
    match accion:
        case ["ir", direccion]:
            print(f"Vas hacia el {direccion}.")
        case ["coger", objeto]:
            print(f"Coges: {objeto}.")
        case _:
            print("No entiendo.")
```

```sortie
Vas hacia el norte.
Coges: llave.
No entiendo.
```

Las funciones (`def`) se explican en [las funciones](fonctions.md).

## El bucle while

Un [bucle](../debuter/glossaire.md#bucle) repite código. `while` («mientras»)
repite su bloque **mientras** la condición sea verdadera.

```python
cuenta = 3
while cuenta > 0:
    print(cuenta)
    cuenta = cuenta - 1
print("¡Despegue!")
```

```sortie
3
2
1
¡Despegue!
```

En cada vuelta, Python comprueba `cuenta > 0`. Cuando `cuenta` vale 0, la condición es
falsa: el bucle se detiene.

> **Atención:** si la condición nunca pasa a ser falsa, el bucle gira para
> siempre. Comprueba que alguna variable cambia en cada vuelta. Para detener un programa
> bloqueado, pulsa Ctrl+C en el terminal.

## El bucle for

`for` recorre los elementos de una colección, uno a uno. Es el bucle más
usado en Python.

```python
frutas = ["manzana", "plátano", "cereza"]
for fruta in frutas:
    print(f"Me gusta la fruta: {fruta}.")
```

```sortie
Me gusta la fruta: manzana.
Me gusta la fruta: plátano.
Me gusta la fruta: cereza.
```

En cada vuelta, la variable `fruta` recibe el siguiente elemento de la lista. Cada pasada
se llama [iteración](../debuter/glossaire.md#iteracion).

`for` también recorre un texto, letra a letra:

```python
for letra in "Sí":
    print(letra)
```

```sortie
S
í
```

### Contar con range

`range` produce una serie de números. Es perfecto para repetir una acción un número
exacto de veces.

- `range(5)`: de 0 a 4 (5 números, **sin** el 5);
- `range(1, 6)`: de 1 a 5;
- `range(0, 10, 2)`: de 0 a 8, de 2 en 2.

```python
for i in range(3):
    print("Vuelta", i)

print(list(range(1, 6)))
print(list(range(0, 10, 2)))
print(list(range(10, 0, -3)))
```

```sortie
Vuelta 0
Vuelta 1
Vuelta 2
[1, 2, 3, 4, 5]
[0, 2, 4, 6, 8]
[10, 7, 4, 1]
```

`list(...)` convierte el `range` en una lista, para ver todos sus números de golpe. Un paso
negativo (`-3`) cuenta hacia atrás.

> **Recuerda:** el final de un `range` siempre queda **excluido**. `range(1, 6)` se detiene
> en 5.

## break y continue

- `break` sale del bucle inmediatamente;
- `continue` se salta el resto de la vuelta y pasa a la siguiente.

```python
for n in range(1, 10):
    if n % 2 == 0:
        continue
    if n > 7:
        break
    print(n)
```

```sortie
1
3
5
7
```

Los números pares se saltan (`continue`). Cuando `n` pasa de 7, el bucle se detiene
(`break`).

### else en un bucle

Un bucle puede tener un `else`. Se ejecuta si el bucle llega **hasta el final**, sin
`break`. Es útil para una búsqueda: «no he encontrado nada».

```python
numeros = [3, 7, 11, 15]
for n in numeros:
    if n % 2 == 0:
        print(f"Primer número par: {n}")
        break
else:
    print("Ningún número par.")
```

```sortie
Ningún número par.
```

## enumerate: el número y el elemento

Para tener a la vez la posición y el elemento, usa `enumerate`. En cada vuelta,
da dos valores: el número (el [índice](../debuter/glossaire.md#indice)) y
el elemento.

```python
podio = ["Alice", "Bob", "Chloé"]
for puesto, nombre in enumerate(podio, start=1):
    print(f"{puesto}. {nombre}")
```

```sortie
1. Alice
2. Bob
3. Chloé
```

Sin `start=1`, la numeración empieza en 0.

## zip: recorrer dos listas a la vez

`zip` avanza por varias listas al mismo tiempo, como una cremallera que
une dos lados, diente a diente.

```python
nombres = ["Alice", "Bob", "Chloé"]
notas = [8, 6, 9]
for nombre, nota in zip(nombres, notas):
    print(f"{nombre}: {nota}/10")
```

```sortie
Alice: 8/10
Bob: 6/10
Chloé: 9/10
```

> **Nota:** `zip` se detiene al final de la lista más corta.

## Errores frecuentes

### Olvidar los dos puntos

```python norun
edad = 20
if edad >= 18
    print("mayor de edad")
```

```text
  File "/home/tu/main.py", line 2
    if edad >= 18
                 ^
SyntaxError: expected ':'
```

Python esperaba `:` (*expected ':'*). Toda línea `if`, `elif`, `else`, `while`, `for`
y `match` termina con dos puntos.

### Escribir = en lugar de ==

```python norun
edad = 20
if edad = 18:
    print("justo")
```

```text
  File "/home/tu/main.py", line 2
    if edad = 18:
       ^^^^^^^^^
SyntaxError: invalid syntax. Maybe you meant '==' or ':=' instead of '='?
```

`=` guarda un valor; para comparar hace falta `==`. Python lo adivina y te lo
sugiere. Escribe `if edad == 18:`.

### Olvidar la sangría

```python norun
edad = 20
if edad >= 18:
print("mayor de edad")
```

```text
  File "/home/tu/main.py", line 3
    print("mayor de edad")
    ^^^^^
IndentationError: expected an indented block after 'if' statement on line 2
```

Después de `if ... :`, Python espera un bloque desplazado. Añade 4 espacios delante de
`print("mayor de edad")`.

### Recorrer un número con for

```python norun
for i in 5:
    print(i)
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 1, in <module>
    for i in 5:
             ^
TypeError: 'int' object is not iterable
```

No se puede recorrer un número (*not iterable*). Para repetir 5 veces, escribe
`for i in range(5):`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```python norun
x = 0
if x:
    print("A")
elif x == 0:
    print("B")
else:
    print("C")
```

> **Respuesta**
>
> `B`. `0` cuenta como falso, así que la primera prueba falla. `x == 0` es verdadero:
> Python muestra `B` e ignora el `else`.

**Pregunta 2:** ¿cuántas veces se muestra «hola» con `for i in range(2, 8):`?

> **Respuesta**
>
> 6 veces: `i` toma los valores 2, 3, 4, 5, 6 y 7. El final (8) queda excluido.

**Pregunta 3:** escribe un bucle que muestre los múltiplos de 3 del 3 al 15, en una
línea.

> **Respuesta**
>
> ```python
> for n in range(3, 16, 3):
>     print(n, end=" ")
> print("fin")
> ```
>
> ```sortie
> 3 6 9 12 15 fin
> ```

## En resumen

- `if`, `elif` y `else` eligen un camino; cada línea termina con `:` y el bloque
  lleva una sangría de 4 espacios.
- `==` compara, `=` guarda; `and`, `or` y `not` combinan las pruebas.
- `0`, `None` y las colecciones vacías cuentan como falsos.
- `match` elige entre varios casos; `case _:` atrapa el resto.
- `while` repite mientras una condición sea verdadera; `for` recorre una colección o un
  `range`.
- `break` sale, `continue` se salta una vuelta; `enumerate` y `zip` simplifican los
  recorridos.

Siguiente: [las funciones](fonctions.md).
