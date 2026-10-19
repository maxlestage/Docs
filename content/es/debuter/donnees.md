# Los tipos de datos: números, texto, listas

Cada valor tiene un tipo, como un número, un texto o una lista: esta página presenta los tipos básicos, primero en Python y luego en JavaScript y en Ruby.

> **Objetivos:**
>
> - entender qué es un tipo y por qué importa;
> - usar números enteros y decimales;
> - escribir texto, unirlo a otro y medir su longitud;
> - convertir un texto en número, y un número en texto;
> - guardar valores en una lista y en un diccionario;
> - reconocer el valor «nada».

## Qué es un tipo

En una cocina, no guardas la harina con los tenedores. Cada cosa tiene su
**clase**: un ingrediente, un utensilio, un plato.

En programación pasa lo mismo. Cada [valor](glossaire.md#valor) tiene una
clase: se llama su [tipo](glossaire.md#tipo).

- `12` es un número;
- `"hola"` es un texto;
- `[12, 15, 9]` es una lista.

El tipo dice **qué puedes hacer** con el valor. Puedes sumar dos números.
No puedes sumar una palabra y un número: `"gato" + 3` no tiene sentido.

Python te da el tipo de un valor con `type`:

```python
print(type(12), type("hola"), type([12, 15, 9]))
```

```sortie
<class 'int'> <class 'str'> <class 'list'>
```

`int` quiere decir número entero, `str` quiere decir texto, `list` quiere
decir lista.

> **Recuerda:** cada valor tiene un tipo. El tipo decide qué puedes hacer
> con él.

## Los números enteros y decimales

Hay dos clases de números:

- los **enteros**, sin coma: `3`, `-12`, `2026` (tipo `int`, mira
  [entero](glossaire.md#entero));
- los **decimales**, con coma: `3.5`, `-0.25` (tipo `float`, mira
  [flotante](glossaire.md#flotante)).

Atención: en programación, la coma decimal se escribe con un **punto**.
Escribes `3.5`, nunca `3,5`.

```python
print(7 + 2, 7 - 2, 7 * 2)
print(7 / 2, 7 // 2, 7 % 2)
```

```sortie
9 5 14
3.5 3 1
```

| Signo | Operación | Ejemplo | Resultado |
|---|---|---|---|
| `+` | suma | `7 + 2` | `9` |
| `-` | resta | `7 - 2` | `5` |
| `*` | multiplicación | `7 * 2` | `14` |
| `/` | división | `7 / 2` | `3.5` |
| `//` | división entera | `7 // 2` | `3` |
| `%` | resto de la división | `7 % 2` | `1` |

Piensa en 7 caramelos para 2 niños: cada uno recibe 3 (`7 // 2`), y sobra
1 (`7 % 2`).

Pruébalo: cambia `7` por `10`. El resto `10 % 2` vale `0`: 10 es par.

### El misterio de 0.1 + 0.2

```python
print(0.1 + 0.2)
print(round(0.1 + 0.2, 2))
```

```sortie
0.30000000000000004
0.3
```

No es un error de Python. El ordenador guarda los decimales en binario (con
ceros y unos). Algunos números, como 0.1, no salen exactos, igual que 1/3,
que se escribe 0.3333… sin fin. Queda un error muy pequeño. JavaScript y
Ruby dan el mismo resultado.

Para mostrarlo limpio, redondeas con `round`: el `2` quiere decir «dos
cifras después de la coma».

> **Recuerda:** los decimales a veces son un poquito inexactos. Redondéalos
> antes de mostrarlos, y nunca cuentes dinero con ellos sin precaución.

## El texto

Un texto se llama [cadena de caracteres](glossaire.md#cadena-de-caracteres),
`str` en Python. Se escribe entre **comillas**: `"hola"` o `'hola'`. Las
comillas le dicen al ordenador: «esto es un texto, no un nombre de
variable».

Unes dos textos con `+`: se dice que los **concatenas**. Mides su longitud
con `len`:

```python
nombre = "Ada"
saludo = "¡Hola, " + nombre + "!"
print(saludo)
print(len(nombre))
print(len(saludo))
```

```sortie
¡Hola, Ada!
3
11
```

`len` cuenta **todos** los caracteres, incluidos los espacios, la coma, `¡` y
`!`.

Pruébalo: escribe `print("nombre")` en lugar de `print(nombre)`. Python
muestra la palabra `nombre` tal cual, no `Ada`: con comillas, es un texto.

## Convertir de un tipo a otro

`"3"` y `3` se parecen, pero no son lo mismo. `"3"` es un texto, como una
etiqueta en la que está escrito 3. `3` es el número tres, con el que se
calcula. Con `+`, la diferencia se ve enseguida:

```python
print("3" + "3", 3 + 3)
```

```sortie
33 6
```

Dos textos se unen: `"33"`. Dos números se suman: `6`.

Para pasar de uno a otro, **conviertes**:

- `int("3")` transforma el texto en número entero;
- `float("3.5")` transforma el texto en número decimal;
- `str(3)` transforma el número en texto.

```python
texto = "12"
numero = int(texto)
print(numero + 1)
print(str(numero) + " años")
```

```sortie
13
12 años
```

Es muy útil con `input`, que lee el teclado: **siempre** da un texto,
aunque el usuario escriba un número.

| Lenguaje | Texto → número | Número → texto |
|---|---|---|
| Python | `int("3")` | `str(3)` |
| JavaScript | `Number("3")` | `String(3)` |
| Ruby | `"3".to_i` | `3.to_s` |

> **Recuerda:** un número entre comillas es un texto. Conviértelo antes de
> calcular.

## Verdadero o falso

Una pregunta cerrada tiene dos respuestas: sí o no. En programación, es
`True` (verdadero) o `False` (falso): el tipo
[booleano](glossaire.md#booleano), `bool` en Python.

Una comparación da un booleano:

```python
edad = 15
print(edad >= 18)
print(edad == 15)
```

```sortie
False
True
```

En JavaScript y en Ruby, se escriben `true` y `false` en minúsculas. Los
booleanos sirven sobre todo en las condiciones: mira [las
condiciones](logique.md).

## Las listas

Una [lista](glossaire.md#lista) guarda varios valores en orden, como una
lista de la compra. Se escribe entre corchetes `[ ]`, con comas.

Cada valor tiene una **posición**, que se llama
[índice](glossaire.md#indice). Atención: se cuenta desde **0**.

```text
frutas = ["manzana", "pera", "kiwi"]

   ┌───────────┬───────────┬───────────┐
   │ "manzana" │  "pera"   │  "kiwi"   │
   └───────────┴───────────┴───────────┘
         0           1           2        ← las posiciones
```

Hay 3 casillas, numeradas 0, 1 y 2. La casilla 3 no existe.

```python
frutas = ["manzana", "pera", "kiwi"]
print(frutas[0])
print(frutas[2])
print(len(frutas))
```

```sortie
manzana
kiwi
3
```

Añades al final con `append`, y cambias una casilla dándole un valor nuevo:

```python
frutas = ["manzana", "pera", "kiwi"]
frutas.append("fresa")
frutas[1] = "plátano"
print(frutas)
```

```sortie
['manzana', 'plátano', 'kiwi', 'fresa']
```

| Paso | Instrucción | La lista después |
|---|---|---|
| 1 | inicio | `manzana, pera, kiwi` |
| 2 | `append("fresa")` | `manzana, pera, kiwi, fresa` |
| 3 | `frutas[1] = "plátano"` | `manzana, plátano, kiwi, fresa` |

Pruébalo: muestra `frutas[-1]`. Python da la **última** casilla: `fresa`.

En JavaScript y en Ruby, es casi igual:

| Acción | Python | JavaScript | Ruby |
|---|---|---|---|
| primera casilla | `frutas[0]` | `frutas[0]` | `frutas[0]` |
| añadir al final | `frutas.append(x)` | `frutas.push(x)` | `frutas.push(x)` |
| longitud | `len(frutas)` | `frutas.length` | `frutas.length` |

> **Recuerda:** la primera casilla de una lista es la casilla **0**. La
> última es la casilla `longitud - 1`.

## Los diccionarios

Una agenda de teléfonos asocia un **nombre** a un **número**. No buscas «la
tercera línea», buscas «Ada».

Un [diccionario](glossaire.md#diccionario) hace lo mismo: asocia
**claves** a **valores**. Se escribe entre llaves `{ }`, con
`clave: valor`.

```text
agenda
   ┌───────┬──────────────────┐
   │ "Ada" │ "06 12 34 56 78" │
   │ "Léo" │ "07 98 76 54 32" │
   └───────┴──────────────────┘
     clave        valor
```

```python
agenda = {"Ada": "06 12 34 56 78", "Léo": "07 98 76 54 32"}
print(agenda["Ada"])
agenda["Zoé"] = "06 00 00 00 01"
print(len(agenda))
print("Léo" in agenda)
```

```sortie
06 12 34 56 78
3
True
```

- `agenda["Ada"]` lee el valor de la clave `"Ada"`;
- `agenda["Zoé"] = …` añade una entrada nueva;
- `"Léo" in agenda` pregunta: «¿existe esta clave?».

Leer una clave que no está, como `agenda["Max"]`, detiene el programa con
el error `KeyError` («clave desconocida»).

En JavaScript, se habla de **objeto**: `{ Ada: "06 12 34 56 78" }`. En
Ruby, de **hash**: `{ "Ada" => "06 12 34 56 78" }`. Se leen de la misma
manera: `agenda["Ada"]`.

> **Recuerda:** una lista se lee por posición (0, 1, 2…), un diccionario se
> lee por clave (`"Ada"`).

## «Nada»: None, null, nil

A veces **no hay valor**: una casilla vacía, una persona que no está en la
agenda. Los lenguajes tienen un valor especial que quiere decir «nada»
(mira [null](glossaire.md#null)): `None` en Python, `null` y `undefined` en
JavaScript, `nil` en Ruby.

En Python, `get` busca una clave y da `None` si no existe:

```python
agenda = {"Ada": "06 12 34 56 78"}
print(agenda.get("Léo"))
print(agenda.get("Léo") is None)
```

```sortie
None
True
```

Compruebas «nada» con `is None`. «Nada» no es `0` ni `""`: es la ausencia
de valor. Casi no puedes hacer nada con él: ni cálculos, ni mayúsculas. Así
que comprueba primero que un valor no es «nada».

## Errores frecuentes

### Sumar un texto y un número

```python norun
edad = "12"
print(edad + 1)
```

```text
Traceback (most recent call last):
  File "/home/ada/edad.py", line 2, in <module>
    print(edad + 1)
          ~~~~~^~~
TypeError: can only concatenate str (not "int") to str
```

`edad` es un texto (tiene comillas). Python no sabe unir un número a un
texto. Corrección: `int(edad) + 1`, o `edad = 12` sin comillas.

### Leer una casilla que no existe

```python norun
frutas = ["manzana", "pera", "kiwi"]
print(frutas[3])
```

```text
Traceback (most recent call last):
  File "/home/ada/frutas.py", line 2, in <module>
    print(frutas[3])
          ~~~~~~^^^
IndexError: list index out of range
```

La lista tiene 3 casillas: 0, 1 y 2. La casilla 3 está «fuera de la lista»
(*out of range*). Corrección: `frutas[2]`, o `frutas[-1]` para la última.

## Ponte a prueba

**1.** ¿Cuánto vale `"5" + "5"` en Python? ¿Y `5 + 5`?

> **Respuesta**
>
> `"5" + "5"` vale `"55"`: dos textos unidos. `5 + 5` vale `10`: dos
> números sumados.

**2.** En `notas = [12, 15, 9, 18]`, ¿qué da `notas[1]`? ¿Y en qué posición
está `18`?

> **Respuesta**
>
> `notas[1]` vale `15`, porque se cuenta desde 0. `18` está en la
> posición 3.

**3.** Quieres guardar la edad de cada alumno y encontrarla por su nombre.
¿Lista o diccionario?

> **Respuesta**
>
> Un diccionario: la clave es el nombre, el valor es la edad.
> `edades = {"Ada": 12, "Léo": 11}`, y luego `edades["Ada"]`.

## En resumen

- Cada valor tiene un tipo: decide qué puedes hacer con él.
- Números enteros (`3`) y decimales (`3.5`, con un punto); los decimales a veces son un poco inexactos, redondéalos.
- El texto se escribe entre comillas; `+` lo une, `len` lo mide.
- `int("3")` y `str(3)` pasan del texto al número y al revés.
- Una lista se lee por posición, desde 0; un diccionario se lee por clave.
- «Nada» se escribe `None` en Python, `null` en JavaScript, `nil` en Ruby.

Siguiente: [leer un mensaje de error sin entrar en pánico](lire-une-erreur.md).
