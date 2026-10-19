# Las funciones

Esta página explica cómo escribir funciones en Python con `def`, darles parámetros, obtener su resultado con `return` y saber qué variables ven.

> **Objetivos:**
>
> - sabrás escribir una función con `def` y llamarla;
> - sabrás devolver un valor con `return`;
> - sabrás usar los valores por defecto y los argumentos con nombre;
> - sabrás qué variables puede ver una función.

## Definir una función

Una [función](../debuter/glossaire.md#funcion) es un trozo de código con nombre que
puedes reutilizar. Es como una receta: la escribes una vez y luego la
sigues tantas veces como quieras. Empieza con `def` (*define*, «definir»),
seguido del nombre, de paréntesis y de dos puntos. El cuerpo lleva sangría.

```python
def saludar():
    print("¡Hola!")

saludar()
saludar()
```

```sortie
¡Hola!
¡Hola!
```

Línea por línea:

- `def saludar():` crea una función llamada `saludar`. Todavía no se muestra nada;
- `print("¡Hola!")` lleva sangría: forma parte de la función, así que espera;
- el primer `saludar()` ejecuta la función: Python ejecuta su cuerpo y muestra
  «¡Hola!»;
- el segundo `saludar()` vuelve a empezar: la misma receta, seguida una segunda vez.

Escribir `def` no ejecuta nada: guarda la receta. Escribir `saludar()` **llama**
a la función, es decir, la ejecuta. Los paréntesis son obligatorios, aunque estén vacíos.

## Los parámetros

Una receta de crepes puede servir para 4 o para 8 personas: solo cambian las
cantidades. Del mismo modo, una función es más útil si puede trabajar con valores
distintos en cada llamada.

Los [parámetros](../debuter/glossaire.md#parametro) son los valores que la función
recibe. Son casillas vacías que se rellenan en el momento de la llamada. Los valores que se dan
en la llamada se llaman [argumentos](../debuter/glossaire.md#argumento).

```python
def saludar(nombre):
    print(f"¡Hola, {nombre}!")

saludar("Alice")
saludar("Bob")
```

```sortie
¡Hola, Alice!
¡Hola, Bob!
```

Línea por línea:

- `def saludar(nombre):` crea la función, con una casilla vacía llamada `nombre`;
- `saludar("Alice")` llama a la función: `nombre` recibe `"Alice"`, y luego el `print`
  muestra «¡Hola, Alice!»;
- `saludar("Bob")` la llama de nuevo: esta vez, `nombre` vale `"Bob"`.

Aquí, `nombre` es el parámetro; `"Alice"` y `"Bob"` son los argumentos. Varios
parámetros se separan con comas.

**Te toca:** escribe una función `cumpleanos(nombre)` que muestre «¡Feliz
cumpleaños, …!» con el nombre recibido. Llámala para Lea y luego para Tom.

> **Solución**
>
> ```python
> def cumpleanos(nombre):
>     print(f"¡Feliz cumpleaños, {nombre}!")
>
> cumpleanos("Lea")
> cumpleanos("Tom")
> ```
>
> ```sortie
> ¡Feliz cumpleaños, Lea!
> ¡Feliz cumpleaños, Tom!
> ```

## Devolver un valor con return

A menudo quieres que una función haga un cálculo y te **dé** el resultado, como una
calculadora. Luego puedes mostrarlo, guardarlo en una variable o reutilizarlo en otro
cálculo.

`return` devuelve un resultado al código que llamó a la función, y sale de la función.

```python
def area(ancho, alto):
    return ancho * alto

print(area(3, 4))
total = area(2, 5) + area(1, 1)
print(total)
```

```sortie
12
11
```

Línea por línea:

- `def area(ancho, alto):` crea una función que recibe dos números;
- `return ancho * alto` calcula el producto y lo devuelve a quien llama;
- `print(area(3, 4))`: la llamada vale 12, así que Python muestra 12;
- `total = area(2, 5) + area(1, 1)`: las dos llamadas valen 10 y 1, y `total`
  recibe 11.

El valor devuelto **sustituye** a la llamada: `area(2, 5) + area(1, 1)` se convierte en `10 + 1`.

`return` también sirve para salir antes:

```python
def dividir(a, b):
    if b == 0:
        return "división imposible"
    return a / b

print(dividir(10, 4))
print(dividir(10, 0))
```

```sortie
2.5
división imposible
```

### ¿Mostrar o devolver?

`print` **muestra** un valor en la pantalla. `return` lo **entrega** al código que llama, que
puede usarlo. Una función sin `return` devuelve `None`.

```python
def doble_muestra(x):
    print(x * 2)

def doble(x):
    return x * 2

resultado1 = doble_muestra(5)
resultado2 = doble(5)
print(resultado1, resultado2)
```

```sortie
10
None 10
```

> **Recuerda:** prefiere las funciones que devuelven un valor. Después decidirás
> si mostrarlo, guardarlo o reutilizarlo.

### Devolver varios valores

Separa los valores con comas. Python los agrupa en una
[tupla](../debuter/glossaire.md#tupla), que puedes descomponer en varias
variables.

```python
def min_max(numeros):
    return min(numeros), max(numeros)

pequeno, grande = min_max([4, 9, 1])
print(pequeno, grande)
```

```sortie
1 9
```

**Te toca:** escribe una función `perimetro(ancho, alto)` que **devuelva** el
perímetro de un rectángulo (el doble de la suma de los lados). Muestra
`perimetro(3, 4)`.

> **Solución**
>
> ```python
> def perimetro(ancho, alto):
>     return 2 * (ancho + alto)
>
> print(perimetro(3, 4))
> ```
>
> ```sortie
> 14
> ```

## Los valores por defecto

En la cafetería te sirven un café normal, salvo que pidas otra cosa. Un valor por
defecto cumple ese papel: evita repetir el valor más habitual.

Un parámetro puede tener un valor por defecto. Se usa si falta el argumento.

```python
def saludar(nombre, saludo="Hola"):
    return f"¡{saludo}, {nombre}!"

print(saludar("Alice"))
print(saludar("Bob", "Buenas"))
```

```sortie
¡Hola, Alice!
¡Buenas, Bob!
```

Línea por línea:

- `saludo="Hola"` da al parámetro `saludo` un valor de reserva;
- `saludar("Alice")` da un solo argumento: `saludo` toma entonces `"Hola"`;
- `saludar("Bob", "Buenas")` da los dos: `"Buenas"` sustituye al valor por defecto.

Los parámetros con valor por defecto se colocan **después** de los demás.

**Te toca:** escribe una función `precio(importe, moneda="euros")` que devuelva un
texto como «5 euros». Muestra `precio(5)` y luego `precio(8, "dólares")`.

> **Solución**
>
> ```python
> def precio(importe, moneda="euros"):
>     return f"{importe} {moneda}"
>
> print(precio(5))
> print(precio(8, "dólares"))
> ```
>
> ```sortie
> 5 euros
> 8 dólares
> ```

## Los argumentos con nombre

En la llamada, puedes dar un argumento por su **nombre**: `nombre=valor`. Así el orden
ya no importa, y la llamada se entiende sola.

```python
def pedido(producto, cantidad=1, urgente=False):
    texto = f"{cantidad} x {producto}"
    if urgente:
        texto += " (urgente)"
    return texto

print(pedido("café"))
print(pedido("té", urgente=True))
print(pedido(cantidad=3, producto="cruasán"))
```

```sortie
1 x café
1 x té (urgente)
3 x cruasán
```

Línea por línea:

- `pedido("café")`: solo se da `producto`; `cantidad` vale 1 y `urgente` vale
  `False`;
- `pedido("té", urgente=True)`: `urgente` se da por su nombre, sin tocar `cantidad`;
- `pedido(cantidad=3, producto="cruasán")`: los dos argumentos llevan nombre, así que
  el orden no importa.

> **Consejo:** da nombre a los argumentos booleanos. `pedido("té", urgente=True)` se
> entiende enseguida; `pedido("té", 1, True)`, mucho menos.

## El alcance de las variables

Una función tiene su propio borrador. Lo que escribe en él mientras trabaja no se
desborda al resto del programa. Así, dos funciones pueden usar el mismo nombre de
variable sin molestarse.

El [alcance](../debuter/glossaire.md#ambito) de una variable es la zona del código donde
existe. Una variable creada **dentro** de una función es **local**: nace con la llamada
y desaparece al final.

```python
def calcular():
    resultado = 42
    return resultado

print(calcular())
print("resultado" in globals())
```

```sortie
42
False
```

Línea por línea:

- `resultado = 42` crea la variable `resultado` **dentro** de la función;
- `print(calcular())` llama a la función, que devuelve 42: Python lo muestra;
- `globals()` da las variables de fuera. `"resultado" in globals()` pregunta si
  `resultado` está entre ellas: la respuesta es `False`.

Después de la llamada, `resultado` ya no existe fuera de la función.

Una función puede **leer** una variable de fuera (una variable **global**). Pero si
le da un valor, Python crea una nueva variable local. Para modificar de verdad la
variable global, hace falta la palabra `global`:

```python
contador = 0

def incrementar():
    global contador
    contador += 1

incrementar()
incrementar()
print(contador)
```

```sortie
2
```

> **Atención:** `global` hace que el código sea difícil de seguir. Es mejor pasar el valor
> como argumento y devolver el resultado: `contador = incrementar(contador)`.

## Errores frecuentes

### Olvidar un argumento

```python norun
def area(ancho, alto):
    return ancho * alto

print(area(3))
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 4, in <module>
    print(area(3))
          ~~~~^^^
TypeError: area() missing 1 required positional argument: 'alto'
```

Falta el argumento `alto` (*missing 1 required argument*). Dalos todos:
`area(3, 4)`. O da un valor por defecto al parámetro.

### Llamar a la función antes de definirla

```python norun
saludar("Alice")

def saludar(nombre):
    print(f"¡Hola, {nombre}!")
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 1, in <module>
    saludar("Alice")
    ^^^^^^^
NameError: name 'saludar' is not defined
```

Python lee el archivo de arriba abajo. En la línea 1, `saludar` todavía no existe.
Coloca el `def` antes de la llamada.

### Modificar una variable global sin global

```python norun
contador = 0

def incrementar():
    contador = contador + 1

incrementar()
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 6, in <module>
    incrementar()
    ~~~~~~~~~~~^^
  File "/home/tu/main.py", line 4, in incrementar
    contador = contador + 1
               ^^^^^^^^
UnboundLocalError: cannot access local variable 'contador' where it is not associated with a value
```

Como la función da un valor a `contador`, Python la considera local. Pero
todavía no tiene valor en el momento de calcular `contador + 1`. Pasa el valor como
parámetro y devuelve el resultado, o añade `global contador`.

### Una lista como valor por defecto

```python norun
def anadir(articulo, carrito=[]):
    carrito.append(articulo)
    return carrito

print(anadir("pan"))
print(anadir("leche"))
```

```text
['pan']
['pan', 'leche']
```

No hay error, pero sí una sorpresa: el segundo carrito todavía contiene el pan. El
valor por defecto se crea **una sola vez**, cuando Python lee el `def`, y luego se
comparte entre todas las llamadas. Usa `None`:

```python
def anadir(articulo, carrito=None):
    if carrito is None:
        carrito = []
    carrito.append(articulo)
    return carrito

print(anadir("pan"))
print(anadir("leche"))
```

```sortie
['pan']
['leche']
```

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```python norun
def f(x):
    x + 1

print(f(5))
```

> **Respuesta**
>
> `None`. La función calcula `x + 1` pero no lo devuelve: falta `return`.

**Pregunta 2:** escribe una función `media(a, b)` que devuelva la media de dos
números.

> **Respuesta**
>
> ```python
> def media(a, b):
>     return (a + b) / 2
>
> print(media(3, 4))
> ```
>
> ```sortie
> 3.5
> ```

**Pregunta 3:** con `def saludar(nombre, saludo="Hola")`, ¿cuánto vale `saludo`
en la llamada `saludar("Alice")`?

> **Respuesta**
>
> `"Hola"`. Falta el argumento, así que Python usa el valor por defecto.

## En resumen

- `def nombre(parámetros):` define una función; `nombre(argumentos)` la llama.
- `return` devuelve un valor y sale de la función; sin `return`, devuelve
  `None`.
- Los parámetros pueden tener un valor por defecto; los argumentos pueden ir
  con nombre.
- Las variables de una función son locales; evita `global`.

Siguiente: [más allá de las funciones](fonctions-avancees.md). Compara con
[las funciones en Ruby](../ruby/fonctions.md).
