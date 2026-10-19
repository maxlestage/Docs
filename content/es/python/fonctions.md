# Las funciones

Esta página explica cómo escribir funciones en Python con `def`, darles parámetros y obtener su resultado con `return`.

> **Objetivos:**
>
> - sabrás escribir una función con `def` y llamarla;
> - sabrás devolver un valor con `return`;
> - sabrás usar los valores por defecto, los argumentos con nombre, `*args` y
>   `**kwargs`;
> - sabrás qué variables puede ver una función;
> - sabrás escribir una `lambda` y una función recursiva.

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

## Un número variable de argumentos: *args y **kwargs

A veces no sabes de antemano cuántos valores vas a recibir. Una cuenta de restaurante
puede tener dos platos o diez. Los asteriscos permiten que una función acepte tantos
argumentos como haga falta.

Con un asterisco, `*args` recibe todos los argumentos **posicionales** de más, guardados en
una tupla.

```python
def suma(*numeros):
    total = 0
    for n in numeros:
        total += n
    return total

print(suma(1, 2))
print(suma(1, 2, 3, 4))
print(suma())
```

```sortie
3
10
0
```

Línea por línea:

- `*numeros` guarda todos los argumentos recibidos en una tupla: `(1, 2)`, luego
  `(1, 2, 3, 4)`, luego `()`;
- `total = 0` empieza desde cero;
- el bucle `for` suma cada número a `total`;
- `return total` devuelve la suma. Sin argumentos, el bucle no da ninguna vuelta: 0.

Con dos asteriscos, `**kwargs` (*keyword arguments*) recibe todos los argumentos **con nombre**
de más, guardados en un [diccionario](../debuter/glossaire.md#diccionario).

```python
def ficha(nombre, **datos):
    print(f"Ficha de {nombre}")
    for clave, valor in datos.items():
        print(f"  {clave}: {valor}")

ficha("Alice", edad=30, ciudad="Lima")
```

```sortie
Ficha de Alice
  edad: 30
  ciudad: Lima
```

Los nombres `args` y `kwargs` son una convención: solo cuentan los asteriscos. Los
diccionarios se explican en [diccionarios y conjuntos](dictionnaires.md).

## Documentar con un docstring

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Un **docstring** es un texto entre comillas triples, colocado justo debajo de la línea
`def`. Explica lo que hace la función. Los editores lo muestran cuando pasas el ratón
por encima del nombre, y `help()` lo enseña.

```python
def precio_con_iva(precio_sin_iva, tasa=0.2):
    """Devuelve el precio con impuestos incluidos, redondeado al céntimo."""
    return round(precio_sin_iva * (1 + tasa), 2)

print(precio_con_iva(10))
print(precio_con_iva.__doc__)
```

```sortie
12.0
Devuelve el precio con impuestos incluidos, redondeado al céntimo.
```

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

## Las funciones son valores

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

En Python, una función es un valor como cualquier otro. Sin paréntesis, su nombre
designa la función misma: puedes guardarla en una variable o pasarla a
otra función.

```python
def gritar(texto):
    return texto.upper() + "!"

def susurrar(texto):
    return texto.lower() + "..."

def hablar(texto, estilo):
    return estilo(texto)

print(hablar("Hola", gritar))
print(hablar("Hola", susurrar))
```

```sortie
HOLA!
hola...
```

`hablar` recibe una función (`estilo`) y la llama. Es muy útil para ordenar, por
ejemplo: `sorted` acepta una función que dice según qué ordenar.

```python
palabras = ["kiwi", "manzana", "fresa", "pera"]
print(sorted(palabras, key=len))
```

```sortie
['kiwi', 'pera', 'fresa', 'manzana']
```

## Las lambdas

A veces una función solo sirve una vez, para una tarea muy pequeña, como decirle a
`sorted` según qué ordenar. Darle un nombre con `def` sería pesado.

Una [lambda](../debuter/glossaire.md#lambda) es una pequeña función sin nombre, escrita
en una línea: `lambda parámetros: expresión`. Devuelve automáticamente el valor
de la expresión.

```python
cuadrado = lambda x: x * x
print(cuadrado(5))

alumnos = [("Alice", 7), ("Bob", 6), ("Chloé", 9)]
por_nota = sorted(alumnos, key=lambda alumno: alumno[1])
print(por_nota)
```

```sortie
25
[('Bob', 6), ('Alice', 7), ('Chloé', 9)]
```

Línea por línea:

- `cuadrado = lambda x: x * x` crea una pequeña función y la guarda en `cuadrado`;
- `cuadrado(5)` calcula `5 * 5` y devuelve 25;
- `alumnos` es una lista de parejas (nombre, nota);
- `sorted(..., key=lambda alumno: alumno[1])` mira la nota de cada pareja y ordena
  a los alumnos de la nota más baja a la más alta.

`key=lambda alumno: alumno[1]` significa «ordena según el segundo elemento». La ordenación
se explica en detalle en [listas y tuplas](collections.md).

> **Consejo:** una lambda sirve sobre todo como argumento de otra función. Para todo
> lo demás, un `def` normal con nombre es más claro.

**Te toca:** la lista `amigos = [("Lea", 25), ("Tom", 19), ("Inés", 31)]` contiene
nombres y edades. Muéstrala ordenada por edad, con `sorted` y una lambda.

> **Solución**
>
> ```python
> amigos = [("Lea", 25), ("Tom", 19), ("Inés", 31)]
> print(sorted(amigos, key=lambda amigo: amigo[1]))
> ```
>
> ```sortie
> [('Tom', 19), ('Lea', 25), ('Inés', 31)]
> ```

## Una función que se llama a sí misma

Algunos problemas contienen una versión más pequeña de sí mismos. Contar hacia atrás
desde 3 es decir «3» y luego contar hacia atrás desde 2. Una función puede seguir la
misma idea.

Una función puede llamarse a sí misma: es la
[recursión](../debuter/glossaire.md#recursion). Hace falta un **caso base**, si no,
no se detiene nunca.

```python
def cuenta_atras(n):
    if n == 0:
        print("¡Ya!")
        return
    print(n)
    cuenta_atras(n - 1)

cuenta_atras(3)
```

```sortie
3
2
1
¡Ya!
```

Línea por línea:

- `cuenta_atras(3)`: `n` vale 3, no es 0. La función muestra 3 y luego se llama a
  sí misma con 2;
- lo mismo con 2 y luego con 1: cada llamada muestra su número;
- con 0, el `if` es verdadero: la función muestra «¡Ya!» y `return` lo detiene todo.
  Es el caso base.

Un ejemplo clásico es el factorial: `5!` vale `5 × 4 × 3 × 2 × 1`. Dicho de otro modo,
`n!` vale `n × (n - 1)!`, y `1!` vale 1.

```python
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

print(factorial(5))
```

```sortie
120
```

> **Nota:** Python limita la profundidad de las llamadas (unas 1000). Para repeticiones
> muy largas, es mejor un bucle.

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

**Pregunta 3:** ¿qué recibe `args` en `def f(*args)` llamada con `f(1, 2, 3)`?

> **Respuesta**
>
> La tupla `(1, 2, 3)`. El asterisco guarda todos los argumentos posicionales en una tupla.

## En resumen

- `def nombre(parámetros):` define una función; `nombre(argumentos)` la llama.
- `return` devuelve un valor y sale de la función; sin `return`, devuelve
  `None`.
- Los parámetros pueden tener un valor por defecto; los argumentos pueden ir
  con nombre; `*args` y `**kwargs` aceptan un número variable de argumentos.
- Las variables de una función son locales; evita `global`.
- Una función es un valor: se puede pasar a otra, o escribir como `lambda`.
- Una función recursiva se llama a sí misma y siempre tiene un caso base.

Siguiente: [trabajar con texto](texte.md). Compara con
[las funciones en Ruby](../ruby/fonctions.md).
