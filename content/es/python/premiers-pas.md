# Primeros pasos

Esta página presenta las piezas básicas de Python: las variables, los tipos de valores simples, cómo mostrar con `print` y las f-strings.

> **Objetivos:**
>
> - sabrás crear una variable y cambiar su valor;
> - sabrás reconocer los tipos `int`, `float`, `str`, `bool` y `None`;
> - sabrás mostrar valores con `print` y las f-strings;
> - sabrás escribir comentarios y nombrar bien tus variables;
> - sabrás leer lo que escribe el usuario con `input`.

## Las variables

Una [variable](../debuter/glossaire.md#variable) es un nombre puesto sobre un
[valor](../debuter/glossaire.md#valor). Imagina una etiqueta pegada en una caja:
la etiqueta es el nombre; el contenido de la caja es el valor.

Una variable se crea con el signo `=`, que significa «recibe» (y no «es igual a»):

```python
nombre = "Alice"
edad = 30
print(nombre)
print(edad)
```

```sortie
Alice
30
```

Una variable puede recibir un valor nuevo. El anterior se olvida.

```python
puntos = 10
print(puntos)
puntos = 25
print(puntos)
puntos = puntos + 5
print(puntos)
```

```sortie
10
25
30
```

La línea `puntos = puntos + 5` se lee de derecha a izquierda: Python calcula `puntos + 5`
(o sea `30`) y luego guarda el resultado en `puntos`.

> **Consejo:** `puntos += 5` es un atajo de `puntos = puntos + 5`. También existen
> `-=`, `*=` y `/=`.

## Los tipos de valores

Cada valor tiene un [tipo](../debuter/glossaire.md#tipo): su naturaleza. Un número no se
maneja igual que un texto. Estos son los cinco tipos básicos.

| Tipo | Nombre completo | Ejemplos | Sirve para |
| --- | --- | --- | --- |
| `int` | *integer*, entero | `42`, `-7`, `0` | contar |
| `float` | número decimal | `3.14`, `-0.5`, `2.0` | medir |
| `str` | *string*, texto | `"gato"`, `'Hola'` | escribir |
| `bool` | booleano | `True`, `False` | responder sí o no |
| `NoneType` | nada | `None` | decir «sin valor» |

```python
numero = 42
precio = 9.99
nombre = "Python"
esta_activo = True
resultado = None

print(numero, precio, nombre, esta_activo, resultado)
```

```sortie
42 9.99 Python True None
```

Algunas aclaraciones:

- Un [entero](../debuter/glossaire.md#entero) no tiene decimales. Un
  [flotante](../debuter/glossaire.md#flotante) se escribe con un **punto**: `3.14`, no
  `3,14`.
- Una [cadena de caracteres](../debuter/glossaire.md#cadena-de-caracteres) (`str`) se
  pone entre comillas dobles `"..."` o simples `'...'`. Las dos valen igual.
- Un [booleano](../debuter/glossaire.md#booleano) vale `True` (verdadero) o `False` (falso),
  con mayúscula.
- `None` significa «nada, sin valor por ahora». Es el equivalente de
  [null](../debuter/glossaire.md#null) en otros lenguajes.

## Conocer el tipo de un valor

La función `type()` da el tipo de un valor. Es muy útil en la consola
para entender con qué estás trabajando.

```python
print(type(42))
print(type(3.14))
print(type("hola"))
print(type(True))
print(type(None))
print(type("42"))
```

```sortie
<class 'int'>
<class 'float'>
<class 'str'>
<class 'bool'>
<class 'NoneType'>
<class 'str'>
```

Mira la última línea: `"42"` es un **texto**, no un número. Las comillas
lo cambian todo.

## El tipado dinámico

En Python, una variable no tiene un tipo fijo. Es el **valor** el que tiene un tipo. La
misma etiqueta puede pasar de un número a un texto: se dice que el tipado es
**dinámico**.

```python
x = 5
print(type(x))
x = "cinco"
print(type(x))
```

```sortie
<class 'int'>
<class 'str'>
```

Es práctico, pero cuidado: Python no te avisará si lo mezclas todo
por error. Usa un solo tipo por variable, es más claro.

> **Nota:** otros lenguajes, como [Rust](../rust/premiers-pas.md), fijan el tipo
> de una variable de una vez por todas. Se habla de tipado **estático**.

## Mostrar con print

`print` muestra uno o varios valores. Separados por comas, se muestran
con un espacio entre ellos y luego un salto de línea.

```python
print("Hay", 3, "gatos")
print()
print("fin")
```

```sortie
Hay 3 gatos

fin
```

`print()` sin nada muestra una línea vacía.

### Las opciones sep y end

Dos opciones cambian este comportamiento:

- `sep` (*separator*): lo que separa los valores (un espacio por defecto);
- `end`: lo que se añade al final (un salto de línea por defecto).

```python
print("2026", "10", "18", sep="-")
print("a", "b", "c", sep="")
print("Cargando", end="...")
print("listo")
```

```sortie
2026-10-18
abc
Cargando...listo
```

Con `end="..."`, el `print` siguiente continúa en la misma línea.

## Las f-strings

Para poner valores **dentro** de un texto, usa una **f-string**: un texto precedido
de la letra `f`. Entre llaves `{ }`, escribes una variable o un cálculo.

```python
nombre = "Alice"
edad = 30
print(f"¡Hola, {nombre}!")
print(f"{nombre} tiene {edad} años.")
print(f"Dentro de 10 años, tendrá {edad + 10} años.")
```

```sortie
¡Hola, Alice!
Alice tiene 30 años.
Dentro de 10 años, tendrá 40 años.
```

Es como un formulario con huecos: Python rellena cada `{ }` con el valor.

> **Consejo:** escribe `{variable=}` para mostrar el nombre **y** el valor. Muy práctico
> para depurar: `print(f"{edad=}")` muestra `edad=30`.

```python
edad = 30
print(f"{edad=}")
```

```sortie
edad=30
```

## Los comentarios

Un [comentario](../debuter/glossaire.md#comentario) es una nota para las personas.
Python lo ignora. Empieza con `#` y llega hasta el final de la línea.

```python
# Cálculo del precio total
precio = 12  # precio de un artículo, en euros
cantidad = 3
print(precio * cantidad)  # muestra el total
```

```sortie
36
```

> **Recuerda:** un buen comentario explica **por qué**, no **qué**. El código ya dice
> lo que hace.

## Nombrar bien las variables

Un nombre de variable:

- contiene letras, cifras y `_`, pero no empieza por una cifra;
- distingue mayúsculas y minúsculas: `edad` y `Edad` son dos nombres distintos;
- no puede ser una palabra reservada de Python (`if`, `for`, `def`, `class`, `True`...).

En Python, la convención es el **snake_case**: todo en minúsculas, con las palabras separadas
por `_`, como una serpiente que se arrastra por el suelo.

```python
numero_de_jugadores = 4
precio_total_con_iva = 59.90
esta_conectado = False
print(numero_de_jugadores, precio_total_con_iva, esta_conectado)
```

```sortie
4 59.9 False
```

Elige nombres que digan lo que contiene la variable. `numero_de_jugadores` es
mucho más claro que `n` o `x`.

> **Nota:** los acentos están permitidos (`año = 2026`), pero lo habitual es escribir los
> nombres sin acentos ni eñes, en español o en inglés.

## Las constantes

Una [constante](../debuter/glossaire.md#constante) es un valor que no debe
cambiar: un tipo de impuesto, un límite, el nombre de una aplicación. Python no tiene
constantes de verdad. Por **convención**, se escriben en MAYÚSCULAS: es un mensaje para los
demás programadores, «no toques este valor».

```python
TASA_IVA = 0.2
PUNTOS_MAX = 100

precio_sin_iva = 50
print(precio_sin_iva * (1 + TASA_IVA))
print(f"Puntuación máxima: {PUNTOS_MAX}")
```

```sortie
60.0
Puntuación máxima: 100
```

## Leer lo que escribe el usuario

`input()` muestra una pregunta, espera a que el usuario escriba una respuesta y pulse
Intro, y devuelve esa respuesta.

```python norun
nombre = input("¿Cómo te llamas? ")
print(f"¡Encantado, {nombre}!")
```

```text
¿Cómo te llamas? Alice
¡Encantado, Alice!
```

`input` devuelve **siempre** un texto (`str`), aunque el usuario escriba un número.
Para calcular con él, conviértelo con `int()`:

```python norun
edad = int(input("¿Cuántos años tienes? "))
print(f"El año que viene tendrás {edad + 1} años.")
```

```text
¿Cuántos años tienes? 29
El año que viene tendrás 30 años.
```

Las conversiones se explican en detalle en [números y cálculos](nombres.md).

## Errores frecuentes

### Pegar un texto y un número con +

```python norun
edad = 30
print("Edad: " + edad)
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 2, in <module>
    print("Edad: " + edad)
          ~~~~~~~~~^~~~~~
TypeError: can only concatenate str (not "int") to str
```

Python se niega a sumar un texto y un número («solo se puede pegar un `str` a un
`str`»). Usa una f-string: `print(f"Edad: {edad}")`. O convierte:
`"Edad: " + str(edad)`.

### Usar una variable antes de crearla

```python norun
print(nombre)
nombre = "Alice"
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 1, in <module>
    print(nombre)
          ^^^^^^
NameError: name 'nombre' is not defined
```

Python lee el archivo de arriba abajo. En la línea 1, `nombre` todavía no existe.
Crea la variable antes de usarla. Revisa también la ortografía: `Nombre` o
`nómbre` serían otros nombres.

### Calcular con el resultado de input

```python norun
edad = input("¿Edad? ")
print(edad + 1)
```

```text
¿Edad? 20
Traceback (most recent call last):
  File "/home/tu/main.py", line 2, in <module>
    print(edad + 1)
          ~~~~~^~~
TypeError: can only concatenate str (not "int") to str
```

`input` devuelve el texto `"20"`, no el número `20`. Conviértelo:
`edad = int(input("¿Edad? "))`.

### Olvidar la f delante de la cadena

```python norun
nombre = "Alice"
print("¡Hola, {nombre}!")
```

```text
¡Hola, {nombre}!
```

No hay error, pero las llaves se muestran tal cual. Sin `f` delante, es
un texto normal. Escribe `print(f"¡Hola, {nombre}!")`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```python norun
a = 5
b = a
a = 8
print(a, b)
```

> **Respuesta**
>
> `8 5`. La línea `b = a` da a `b` el valor de `a` en ese momento (`5`). Cambiar
> `a` después no cambia `b`.

**Pregunta 2:** ¿cuál es el tipo de `"3.5"`? ¿Y el de `3.5`?

> **Respuesta**
>
> `"3.5"` es un `str` (un texto, por las comillas). `3.5` es un `float`.

**Pregunta 3:** escribe un programa que guarde `"Madrid"` en una variable y muestre
`Vivo en Madrid.` con una f-string.

> **Respuesta**
>
> ```python
> ciudad = "Madrid"
> print(f"Vivo en {ciudad}.")
> ```
>
> ```sortie
> Vivo en Madrid.
> ```

## En resumen

- `nombre = valor` crea una variable; después se le puede dar un valor nuevo.
- Los tipos básicos son `int`, `float`, `str`, `bool` y `None`; `type()` los revela.
- `print` muestra sus valores separados por `sep` y terminados por `end`.
- Una f-string (`f"... {x} ..."`) mete valores dentro de un texto.
- Los nombres se escriben en snake_case; las constantes, en MAYÚSCULAS.
- `input()` siempre devuelve un texto: conviértelo antes de calcular.

Siguiente: [números y cálculos](nombres.md).
