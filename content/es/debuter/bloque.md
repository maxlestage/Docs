# Cuando te bloqueas: un método paso a paso

Quedarse bloqueado ante un problema le pasa a todo el mundo: esta página te da un método sencillo, paso a paso, para avanzar de todos modos, con un ejemplo completo en Python.

> **Objetivos:**
>
> - saber que quedarse bloqueado es normal;
> - reformular un problema con tus propias palabras;
> - resolver un pequeño ejemplo a mano, en papel;
> - dividir un problema y escribirlo paso a paso;
> - usar el pato de goma y la pausa;
> - pedir ayuda de forma eficaz.

## Quedarse bloqueado es normal

Lees el enunciado. No sabes por dónde empezar. Miras fijamente la pantalla
vacía. Te dices: «esto no es para mí».

Es falso. **Todo el mundo** se bloquea, incluso después de veinte años de
oficio. Bloquearse no significa que seas malo: significa que estás
aprendiendo algo nuevo.

La diferencia entre un principiante y alguien con experiencia no es que
nunca se bloquee. Es que tiene un **método** para desbloquearse. Aquí lo
tienes, en seis pasos:

```text
  1. Reformular          → decir el problema con mis palabras
  2. Hacerlo a mano      → un ejemplo muy pequeño, en papel
  3. Dividir             → pasos en español (pseudocódigo)
  4. Programar por pasos → un paso, lo ejecuto, lo compruebo
  5. Explicar            → al pato de goma
  6. Hacer una pausa     → y luego pedir ayuda
```

> **Recuerda:** bloquearse forma parte del trabajo. Lo que cuenta es
> tener un método.

## Paso 1: reformular con tus palabras

Un enunciado suele estar escrito por otra persona, con sus propias
palabras. Reescríbelo con **las tuyas**, en una o dos frases sencillas.

Tomemos este enunciado:

```text
Escribir una función que determine el número de vocales de una cadena.
```

Reformulado:

```text
Recibo una palabra. Tengo que decir cuántas vocales tiene.
Las vocales son: a, e, i, o, u.
```

Hazte tres preguntas:

- **¿Qué recibo?** Una palabra (un texto).
- **¿Qué tengo que devolver?** Un número.
- **¿Qué no está claro?** ¿Cuentan las mayúsculas? Sí: `"A"` también es
  una vocal.

Si no consigues reformularlo, el problema no es el código: es la
comprensión del enunciado. Vuelve a leerlo o pide una aclaración.

## Paso 2: resolver un ejemplo muy pequeño a mano

Toma un lápiz. Elige una palabra corta: `banana`. Cuenta sus vocales,
**despacio**, anotando todo lo que haces.

| Letra | ¿Vocal? | Vocales |
|---|---|---|
| (inicio) | | 0 |
| b | no | 0 |
| a | sí | 1 |
| n | no | 1 |
| a | sí | 2 |
| n | no | 2 |
| a | sí | 3 |

Respuesta: **3**.

Ahora mira lo que has hecho. Has:

1. empezado un contador en 0;
2. mirado cada letra, una por una;
3. sumado 1 al contador cuando era una vocal;
4. dado el contador al final.

Acabas de encontrar el [algoritmo](glossaire.md#algoritmo). Tu cerebro ya
lo conocía: solo había que **frenarlo** para verlo.

> **Recuerda:** si sabes hacerlo a mano, puedes explicárselo al
> ordenador.

## Paso 3: dividir en pseudocódigo

Escribimos estos pasos en español, con la forma de un programa. Es el
[pseudocódigo](glossaire.md#pseudocodigo) (ver [Pensar como un
programador](algorithmes.md)). Llamamos `vocales` al contador:

```text
vocales ← 0
para cada letra de la palabra
    si la letra es una vocal
        vocales ← vocales + 1
dar vocales
```

Cada línea es lo bastante pequeña para convertirse en una línea de código.

## Paso 4: programar un paso cada vez

No lo escribas todo de golpe. Escribe **un paso**, ejecútalo, compruébalo.
Luego el siguiente. Como cuando montas un mueble: compruebas cada tornillo
antes de pasar al siguiente.

### Intento 1: recorrer las letras

Primero, ¿sé mirar cada letra?

```python
palabra = "banana"
for letra in palabra:
    print(letra)
```

```sortie
b
a
n
a
n
a
```

Sí. Siguiente paso.

### Intento 2: reconocer una vocal

`letra in "aeiou"` pregunta: «¿está esta letra en el texto `aeiou`?». La
respuesta es `True` (verdadero) o `False` (falso).

```python
palabra = "banana"
for letra in palabra:
    print(letra, letra in "aeiou")
```

```sortie
b False
a True
n False
a True
n False
a True
```

La tabla coincide con la que hiciste a mano. Siguiente paso.

### Intento 3: contar… y equivocarse

```python
palabra = "banana"
for letra in palabra:
    vocales = 0
    if letra in "aeiou":
        vocales = vocales + 1
print(vocales)
```

```sortie
1
```

Esperábamos 3. Que no cunda el pánico: lo ejecutamos pronto, así que el
error está en las 3 líneas que acabamos de añadir. Sigamos la variable a
mano:

| Letra | `vocales = 0` | ¿vocal? | vocales después |
|---|---|---|---|
| b | 0 | no | 0 |
| a | 0 | sí | 1 |
| n | 0 | no | 0 |
| a | 0 | sí | 1 |
| n | 0 | no | 0 |
| a | 0 | sí | 1 |

El contador vuelve a 0 **en cada letra**. En el pseudocódigo, `vocales ←
0` estaba **antes** del bucle. Lo hemos puesto dentro.

### Intento 4: la corrección

```python
palabra = "banana"
vocales = 0
for letra in palabra:
    if letra in "aeiou":
        vocales = vocales + 1
print(vocales)
```

```sortie
3
```

Funciona con `banana`. Pero un solo ejemplo no basta.

### Intento 5: probar otras palabras

Probamos una palabra con una mayúscula y una palabra sin vocales:

```python
for palabra in ["banana", "Oreja", "pff"]:
    vocales = 0
    for letra in palabra:
        if letra in "aeiou":
            vocales = vocales + 1
    print(palabra, vocales)
```

```sortie
banana 3
Oreja 2
pff 0
```

`Oreja` tiene 3 vocales (O, e, a), no 2. La `O` mayúscula no está en
`"aeiou"`. ¡Y eso que habíamos anotado la pregunta en el paso 1! Pasamos
cada letra a minúscula con `lower()`.

### Intento 6: la versión final

Lo guardamos todo en una [función](glossaire.md#funcion):

```python
def contar_vocales(palabra):
    vocales = 0
    for letra in palabra.lower():
        if letra in "aeiou":
            vocales = vocales + 1
    return vocales

print(contar_vocales("banana"))
print(contar_vocales("Oreja"))
print(contar_vocales("pff"))
```

```sortie
3
3
0
```

Seis pequeños intentos, dos errores encontrados enseguida. Si lo
hubiéramos escrito todo de golpe, los dos errores habrían llegado juntos y
no habríamos sabido por dónde empezar.

Prueba: llama a `contar_vocales("")`, con una palabra vacía. El resultado
es `0`: el bucle no da ni una vuelta.

> **Recuerda:** un paso, lo ejecutas, lo compruebas. Cuanto más a menudo
> ejecutes, más fácil es encontrar el error.

## Paso 5: explicárselo al pato de goma

Pon un pato de goma (o una planta, o una taza) al lado de la pantalla.
Explícale tu código **en voz alta**, línea por línea: «aquí pongo el
contador a cero; aquí, para cada letra…».

Muy a menudo, en mitad de una frase, oyes tú mismo el error: «… y vuelvo a
poner el contador a cero en cada letra. Ah.».

Hablar obliga a ir más despacio. No puedes saltarte una línea al
explicarla.

## Paso 6: hacer una pausa

Después de 20 minutos bloqueado en lo mismo, tu cerebro da vueltas en
círculo. Levántate. Camina, bebe un vaso de agua, haz otra cosa durante diez
minutos.

No es rendirse. El cerebro sigue trabajando en segundo plano. Muchos
programadores encuentran la solución en la ducha o caminando, nunca
delante de la pantalla.

## Pedir ayuda

Pedir ayuda no es hacer trampa. Pero una buena pregunta recibe respuesta
mucho más rápido. Contiene cuatro cosas:

```text
1. Un ejemplo mínimo: el código más pequeño que muestra el problema.
2. Lo que esperaba:   contar_vocales("Oreja") debería dar 3.
3. Lo que obtengo:    da 2 (o: el mensaje de error completo).
4. Lo que he probado: he mostrado cada letra, la O no se cuenta.
```

**Mínimo** quiere decir: quita todo lo que no sirve para mostrar el
problema. A menudo, al reducir el código, encuentras el error tú solo.

Copia el código y el mensaje **como texto**, no como foto de la pantalla:
la persona que te ayude podrá ejecutarlo. Para ir más lejos, mira
[Encontrar y corregir tus errores](deboguer.md).

## Errores frecuentes

### Copiar el pseudocódigo sin el desplazamiento

```python norun
palabra = "banana"
for letra in palabra:
if letra in "aeiou":
    print(letra)
```

```text
  File "/home/ada/vocales.py", line 3
    if letra in "aeiou":
    ^^
IndentationError: expected an indented block after 'for' statement on line 2
```

En Python, lo que está **dentro** del bucle debe ir desplazado hacia la
derecha (la [indentación](glossaire.md#indentacion)). El mensaje lo dice:
se esperaba un bloque desplazado después del `for` de la línea 2.
Corrección: desplaza el `if` y todo lo que sigue 4 espacios.

### Olvidar un paso del pseudocódigo

```python norun
palabra = "banana"
for letra in palabra:
    if letra in "aeiou":
        vocales = vocales + 1
print(vocales)
```

```text
Traceback (most recent call last):
  File "/home/ada/vocales.py", line 4, in <module>
    vocales = vocales + 1
              ^^^^^^^
NameError: name 'vocales' is not defined. Did you mean: 'locals'?
```

Se ha olvidado la línea `vocales ← 0` del pseudocódigo. Python no conoce
`vocales` en el momento de sumar 1. Su sugerencia (`locals`) es falsa
aquí: una sugerencia es una adivinanza, no una orden. Corrección: añade
`vocales = 0` antes del bucle.

## Ponte a prueba

**1.** No sabes por dónde empezar un ejercicio. ¿Qué es lo primero que
hay que hacer?

> **Respuesta**
>
> Reformular el enunciado con tus palabras: ¿qué recibo?, ¿qué tengo que
> devolver? Luego resolver un ejemplo muy pequeño a mano.

**2.** ¿Por qué ejecutar el programa después de cada pequeño paso?

> **Respuesta**
>
> Porque si algo no funciona, el error está por fuerza en las pocas líneas
> que acabas de añadir. Así que es fácil de encontrar.

**3.** ¿Cuáles son las cuatro partes de una buena petición de ayuda?

> **Respuesta**
>
> Un ejemplo mínimo, lo que esperabas, lo que obtienes (o el mensaje de
> error completo) y lo que ya has probado.

## En resumen

- Bloquearse es normal: lo que cuenta es el método.
- Reformula con tus palabras y luego resuelve un pequeño ejemplo a mano.
- Escribe el pseudocódigo y luego programa un paso cada vez, ejecutando después de cada uno.
- Prueba varios ejemplos, incluidos casos raros (mayúscula, palabra vacía).
- Explícaselo al pato, haz una pausa y luego pide ayuda con un ejemplo mínimo.

Siguiente: [la terminal en 10 minutos](terminal.md).
