# Trabajar con texto

Esta página explica cómo manejar texto en Python: cortarlo, buscar en él, reemplazar partes, darle formato y entender los caracteres Unicode.

> **Objetivos:**
>
> - sabrás leer una letra o un trozo de texto con los índices y las porciones;
> - sabrás transformar un texto con `upper`, `strip`, `replace`...;
> - sabrás dividir un texto con `split` y volver a unirlo con `join`;
> - sabrás buscar en un texto con `in`, `find` y `count`;
> - sabrás alinear texto en una f-string y escribir caracteres especiales.

## El tipo str

Un texto en Python es una [cadena de caracteres](../debuter/glossaire.md#cadena-de-caracteres),
de tipo `str`. Imagina un collar de perlas: cada perla es un carácter (una letra, una
cifra, un espacio, un símbolo), y las perlas están en un orden preciso.

```python
mensaje = "Hola"
otro = 'Buenas'
cita = "Dijo 'ya'"
print(mensaje, otro, cita)
print(len(mensaje))
```

```sortie
Hola Buenas Dijo 'ya'
4
```

- Comillas dobles o simples, como prefieras. Usa las dobles si el texto contiene una
  comilla simple.
- `len` (*length*, longitud) da el número de caracteres.

Se unen dos textos con `+` y se repite un texto con `*`:

```python
print("Ho" + "la")
print("ja" * 3)
print("-" * 20)
```

```sortie
Hola
jajaja
--------------------
```

## Leer un carácter: los índices

Cada carácter tiene una posición, su [índice](../debuter/glossaire.md#indice). El
primero está en el índice **0**. Los índices negativos cuentan desde el final: `-1` es
el último.

```text
 P   y   t   h   o   n
 0   1   2   3   4   5
-6  -5  -4  -3  -2  -1
```

```python
palabra = "Python"
print(palabra[0])
print(palabra[3])
print(palabra[-1])
print(palabra[-2])
```

```sortie
P
h
n
o
```

## Cortar: las porciones

Una **porción** (*slice*) toma un trozo del texto: `texto[inicio:fin]`. El inicio se
incluye, el fin se **excluye**, como con `range`.

```python
palabra = "Python"
print(palabra[0:3])
print(palabra[2:])
print(palabra[:4])
print(palabra[-3:])
print(palabra[::2])
print(palabra[::-1])
```

```sortie
Pyt
thon
Pyth
hon
Pto
nohtyP
```

- sin inicio, se empieza desde el principio; sin fin, se llega hasta el final;
- un tercer número da el **paso**: `[::2]` toma una letra de cada dos;
- `[::-1]` recorre al revés: es el truco clásico para dar la vuelta a un texto.

## Un texto no se modifica

Un `str` es [inmutable](../debuter/glossaire.md#inmutable): una vez creado, ya no
cambia. Todas las operaciones sobre un texto devuelven un texto **nuevo**. El original
queda intacto.

```python
nombre = "lucía"
mayusculas = nombre.upper()
print(nombre)
print(mayusculas)
nombre = nombre.capitalize()
print(nombre)
```

```sortie
lucía
LUCÍA
Lucía
```

Para "cambiar" una variable de texto, guarda el nuevo texto en la variable:
`nombre = nombre.capitalize()`.

## Cambiar mayúsculas y limpiar

Un [método](../debuter/glossaire.md#metodo) es una función ligada a un valor. Se llama
con un punto: `texto.metodo()`. Estos son los más útiles:

```python
titulo = "cien años de soledad"
print(titulo.upper())
print(titulo.title())
print(titulo.capitalize())
print("HOLA".lower())

entrada = "   lucia@ejemplo.es  \n"
print(f"[{entrada.strip()}]")
print(f"[{entrada.lstrip()}]".replace("\n", "↵"))
print("---titulo---".strip("-"))
```

```sortie
CIEN AÑOS DE SOLEDAD
Cien Años De Soledad
Cien años de soledad
hola
[lucia@ejemplo.es]
[lucia@ejemplo.es  ↵]
titulo
```

- `upper` y `lower`: todo en mayúsculas, todo en minúsculas;
- `title`: una mayúscula en cada palabra; `capitalize`: solo en la primera letra;
- `strip` quita los espacios y saltos de línea de los dos extremos; `lstrip` y
  `rstrip` de un solo lado. Con un argumento, quita esos caracteres.

> **Consejo:** piensa en `strip()` en cuanto leas un texto escrito por un usuario o
> leído de un archivo: a menudo sobran espacios.

## Dividir y unir: split y join

`split` corta un texto en trozos y devuelve una [lista](../debuter/glossaire.md#lista).
Sin argumento, corta en los espacios.

```python
frase = "el gato duerme en el sofá"
palabras = frase.split()
print(palabras)
print(len(palabras))

linea = "Lucía;30;Sevilla"
print(linea.split(";"))
```

```sortie
['el', 'gato', 'duerme', 'en', 'el', 'sofá']
6
['Lucía', '30', 'Sevilla']
```

`join` hace lo contrario: une los trozos de una lista, con un separador. Se llama
**sobre el separador**.

```python
palabras = ["uno", "dos", "tres"]
print(" ".join(palabras))
print(", ".join(palabras))
print("-".join("abc"))
```

```sortie
uno dos tres
uno, dos, tres
a-b-c
```

> **Recuerda:** `texto.split(sep)` da una lista; `sep.join(lista)` devuelve un texto.

## Reemplazar

`replace(viejo, nuevo)` reemplaza todas las apariciones de un trozo de texto.

```python
frase = "Me gusta el té. El té está caliente."
print(frase.replace("té", "café"))
print(frase.replace("té", "café", 1))
print("1.000.000".replace(".", ""))
```

```sortie
Me gusta el café. El café está caliente.
Me gusta el café. El té está caliente.
1000000
```

El tercer argumento limita el número de reemplazos.

## Buscar en un texto

```python
email = "lucia@ejemplo.es"
print("@" in email)
print("pablo" in email)
print(email.startswith("lucia"))
print(email.endswith(".es"))
print(email.find("@"))
print(email.find("#"))
print("banana".count("a"))
```

```sortie
True
False
True
True
5
-1
3
```

- `in` responde `True` o `False`: "¿está este trozo dentro?";
- `startswith` y `endswith` comprueban el principio y el final;
- `find` da el índice de la primera aparición, o `-1` si no está;
- `count` cuenta las apariciones.

Todas estas búsquedas distinguen mayúsculas y minúsculas. Para no tenerlas en cuenta,
pásalo todo a minúsculas antes:

```python
respuesta = "SÍ"
print(respuesta.lower() == "sí")
```

```sortie
True
```

Algunos métodos comprueban de qué está hecho el texto: `isdigit()` (solo cifras),
`isalpha()` (solo letras), `isspace()` (solo espacios).

```python
print("2026".isdigit(), "12a".isdigit(), "Año".isalpha())
```

```sortie
True False True
```

## Alinear texto

En una f-string, `:` seguido de una alineación y de un ancho coloca el texto en una
columna:

- `<` a la izquierda, `>` a la derecha, `^` en el centro;
- un carácter antes del signo sirve de relleno.

```python
productos = [("Pan", 1.2), ("Queso", 4.5), ("Mermelada", 3.75)]
for nombre, precio in productos:
    print(f"{nombre:<12}{precio:>6.2f} €")
print(f"{'FIN':*^18}")
```

```sortie
Pan           1.20 €
Queso         4.50 €
Mermelada     3.75 €
*******FIN********
```

`{nombre:<12}` reserva 12 caracteres, texto a la izquierda. `{precio:>6.2f}` reserva 6
caracteres, número a la derecha con 2 decimales. Los formatos de números se explican
en [números y cálculos](nombres.md).

## Textos de varias líneas

Entre comillas **triples** `"""..."""`, un texto puede ocupar varias líneas. Los saltos
de línea se conservan.

```python
poema = """Caminante, no hay camino,
se hace camino al andar."""
print(poema)
print(len(poema.splitlines()))
```

```sortie
Caminante, no hay camino,
se hace camino al andar.
2
```

`splitlines()` corta el texto en una lista de líneas.

## Los caracteres especiales

Algunos caracteres se escriben con una barra invertida `\`, seguida de una letra: es
una **secuencia de escape**.

| Secuencia | Significado |
| --- | --- |
| `\n` | salto de línea |
| `\t` | tabulación |
| `\"` y `\'` | una comilla dentro del texto |
| `\\` | una barra invertida |

```python
print("Línea 1\nLínea 2")
print("Nombre\tEdad")
print("Dijo: \"Hola\".")
print("C:\\carpeta\\archivo.txt")
print(r"C:\carpeta\nuevo")
```

```sortie
Línea 1
Línea 2
Nombre	Edad
Dijo: "Hola".
C:\carpeta\archivo.txt
C:\carpeta\nuevo
```

La `r` delante del último texto lo convierte en una cadena **en bruto** (*raw*): las `\`
se quedan tal cual. Es práctico para las rutas de Windows y las
[expresiones regulares](regex.md).

## Unicode: ord y chr

Un ordenador solo guarda números. [Unicode](../debuter/glossaire.md#unicode) da un
número a cada carácter del mundo: letras con tilde, alfabetos, emojis. `ord` da el
número de un carácter, `chr` hace lo contrario.

```python
print(ord("A"), ord("a"), ord("ñ"), ord("€"))
print(chr(65), chr(241), chr(8364))
print(len("año"), len("😀"))
```

```sortie
65 97 241 8364
A ñ €
3 1
```

En Python, un carácter cuenta como 1, incluso un emoji. Los textos de Python están en
Unicode desde el principio: puedes escribir en español, en griego o en japonés sin
configurar nada.

Con `ord` y `chr`, se pueden desplazar las letras. Este es el cifrado César, que
desplaza cada letra 3 posiciones:

```python
def cesar(texto, desplazamiento):
    resultado = ""
    for letra in texto:
        if "a" <= letra <= "z":
            posicion = (ord(letra) - ord("a") + desplazamiento) % 26
            resultado += chr(ord("a") + posicion)
        else:
            resultado += letra
    return resultado

secreto = cesar("hola zoe", 3)
print(secreto)
print(cesar(secreto, -3))
```

```sortie
krod crh
hola zoe
```

## Errores frecuentes

### Modificar una letra de un texto

```python norun
palabra = "gato"
palabra[0] = "G"
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 2, in <module>
    palabra[0] = "G"
    ~~~~~~~^^^
TypeError: 'str' object does not support item assignment
```

Un texto es inmutable. Construye un texto nuevo: `palabra = "G" + palabra[1:]`, o
`palabra = palabra.capitalize()`.

### Un índice demasiado grande

```python norun
palabra = "gato"
print(palabra[10])
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 2, in <module>
    print(palabra[10])
          ~~~~~~~^^^^
IndexError: string index out of range
```

`"gato"` tiene 4 letras: los índices van de 0 a 3 (*out of range*: fuera de rango). El
último carácter es `palabra[-1]` o `palabra[len(palabra) - 1]`.

### Llamar a un método de texto sobre una lista

```python norun
texto = "a,b"
print(texto.split(",").upper())
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 2, in <module>
    print(texto.split(",").upper())
          ^^^^^^^^^^^^^^^^^^^^^^
AttributeError: 'list' object has no attribute 'upper'
```

`split` devuelve una **lista**, y una lista no tiene método `upper`. Pasa a mayúsculas
antes de dividir (`texto.upper().split(",")`), o cada trozo.

### Creer que un método modifica el texto

```python norun
nombre = "  lucía  "
nombre.strip()
print(f"[{nombre}]")
```

```text
[  lucía  ]
```

No hay error, pero nada ha cambiado: `strip()` devuelve un texto nuevo, que se pierde.
Escribe `nombre = nombre.strip()`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto vale `"Buenas"[1:4]`?

> **Respuesta**
>
> `"uen"`: los caracteres de los índices 1, 2 y 3. El índice 4 se excluye.

**Pregunta 2:** transforma `"manzana,pera,kiwi"` en `"manzana / pera / kiwi"`.

> **Respuesta**
>
> ```python
> frutas = "manzana,pera,kiwi"
> print(" / ".join(frutas.split(",")))
> ```
>
> ```sortie
> manzana / pera / kiwi
> ```
>
> `frutas.replace(",", " / ")` también funciona.

**Pregunta 3:** ¿cómo saber si una palabra es un palíndromo (se lee igual en los dos
sentidos)?

> **Respuesta**
>
> ```python
> palabra = "reconocer"
> print(palabra == palabra[::-1])
> ```
>
> ```sortie
> True
> ```

## En resumen

- Un `str` es una sucesión de caracteres, inmutable: cada operación devuelve un texto
  nuevo.
- `texto[i]` lee un carácter (0 el primero, -1 el último); `texto[a:b]` lee una
  porción, fin excluido.
- `upper`, `lower`, `title`, `strip` y `replace` transforman; `split` y `join` dividen
  y vuelven a unir.
- `in`, `find`, `startswith` y `count` buscan.
- `f"{x:<10}"` alinea; `\n` y `\t` son caracteres especiales; `ord` y `chr` pasan del
  carácter a su número Unicode y al revés.

Siguiente: [listas y tuplas](collections.md).
