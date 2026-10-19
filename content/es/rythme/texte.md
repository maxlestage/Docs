# Trabajar con texto

Esta página muestra las herramientas de Rythme para los textos: insertar valores, cambiar
mayúsculas y minúsculas, dividir, buscar, reemplazar, alinear columnas, leer un número,
y un primer contacto con las expresiones regulares.

> **Objetivos:**
>
> - sabrás construir un texto con valores insertados y caracteres especiales;
> - sabrás transformar un texto: mayúsculas, espacios, reemplazo;
> - sabrás dividir un texto en trozos y volver a unir una lista;
> - sabrás leer un número escrito como texto, sin que el programa se detenga;
> - sabrás buscar trozos de texto con una expresión regular sencilla.

## Insertar un valor en un texto

Un texto, que también se llama
[cadena de caracteres](../debuter/glossaire.md#cadena-de-caracteres), es
una sucesión de caracteres: letras, cifras, espacios, signos.

Lo sabes desde los [primeros pasos](premiers-pas.md): un valor entre
llaves `{...}` se inserta en el texto. Puedes poner ahí un cálculo o una
llamada a una función:

```rythme
nombre = "Ada"
frutas = ["manzana", "pera"]
print("¡Hola, {nombre}!")
print("{nombre} tiene {len(frutas)} frutas.")
print("El doble de 21 es {21 * 2}.")
```

```sortie
¡Hola, Ada!
Ada tiene 2 frutas.
El doble de 21 es 42.
```

> **Consejo:** para convertir cualquier valor en texto, usa `text`:
> `text(42)` vale `"42"`.

## Los caracteres especiales

Algunos caracteres no se pueden escribir directamente en un texto. Se
escriben con una barra invertida `\` delante: una **secuencia de escape**.

| Escribe | Para obtener |
|--------|--------------|
| `\n` | un salto de línea |
| `\t` | una tabulación (un espacio grande, para alinear) |
| `\"` | una comilla, sin cerrar el texto |
| `\\` | una barra invertida |
| `\{` y `\}` | llaves, sin inserción |

```rythme
print("Dijo: \"sí\".")
print("línea 1\nlínea 2")
print("Unas llaves: \{ y \}")
```

```sortie
Dijo: "sí".
línea 1
línea 2
Unas llaves: { y }
```

## Longitud y mayúsculas

`len` cuenta los caracteres. `upper` lo pone todo en mayúsculas, `lower`
todo en minúsculas, y `capitalize` solo la primera letra:

```rythme
palabra = "ángela"
print(len(palabra))
print(upper(palabra), lower("HOLA"), capitalize(palabra))
```

```sortie
6
ÁNGELA hola Ángela
```

Las letras con tilde cuentan como un solo carácter: `len("canción")` vale 7.

## Leer un carácter por su posición

Cada carácter tiene una posición, empezando en **0**. `palabra[0]` es el
primer carácter, `palabra[-1]` el último. `slice(palabra, inicio, fin)`
toma un trozo, desde `inicio` incluido hasta `fin` no incluido:

```rythme
palabra = "girasol"
print(palabra[0], palabra[-1], slice(palabra, 0, 4), slice(palabra, 4, 7))
```

```sortie
g l gira sol
```

Como una regla graduada: el trozo de 0 a 4 contiene los caracteres de las
posiciones 0, 1, 2 y 3.

## Quitar los espacios con trim

Un texto escrito por alguien suele tener espacios de más al principio o al
final. `trim` los quita:

```rythme
entrada = "   Ada   "
print("[{entrada}]")
print("[{trim(entrada)}]")
```

```sortie
[   Ada   ]
[Ada]
```

## Dividir y volver a unir

`split` divide un texto en una lista de trozos. Sin separador, corta en los
espacios. `join` hace lo contrario: vuelve a unir una lista con un
separador.

```rythme
frase = "el gato duerme"
palabras = split(frase)
print(palabras, len(palabras))

print(split("rojo;verde;azul", ";"))
print(join(["rojo", "verde", "azul"], ", "))
```

```sortie
["el", "gato", "duerme"] 3
["rojo", "verde", "azul"]
rojo, verde, azul
```

Con `|>`, se encadenan los pasos: se divide, se transforma cada palabra, se
vuelve a unir.

```rythme
"el gato duerme"
  |> split
  |> map(capitalize)
  |> join(" ")
  |> print
```

```sortie
El Gato Duerme
```

## Buscar en un texto

Tres preguntas, tres respuestas `true` o `false`:

- `contains`: ¿contiene el texto este trozo?
- `starts_with`: ¿empieza por este trozo?
- `ends_with`: ¿termina por este trozo?

```rythme
archivo = "vacaciones.jpg"
print(contains(archivo, "cacio"))
print(starts_with(archivo, "vac"), ends_with(archivo, ".jpg"))
print(ends_with(archivo, ".png"))
```

```sortie
true
true true
false
```

> **Atención:** las mayúsculas cuentan: `contains("Hola", "hol")` vale
> `false`. Si hace falta, pasa antes el texto a minúsculas con `lower`.

Para comparar dos textos enteros, `==` también funciona, y las mayúsculas
cuentan igual. `equals_ignore_case` compara sin tenerlas en cuenta:

```rythme
print("abc" == "abc", "Abc" == "abc", equals_ignore_case("Árbol", "ÁRBOL"))
```

```sortie
true false true
```

## Reemplazar un trozo

`replace(texto, viejo, nuevo)` reemplaza **cada** aparición de `viejo`:

```rythme
print(replace("el gato y el perro", "el", "un"))
print(replace("2024-05-17", "-", "/"))
```

```sortie
un gato y un perro
2024/05/17
```

Como siempre en Rythme, el texto de partida no cambia: `replace` da un
texto nuevo.

## Los caracteres y las líneas

`chars` da la lista de los caracteres. `lines` divide un texto en líneas
(`\n` quiere decir «salto de línea»):

```rythme
print(chars("abc"))

poema = "Uno\nDos\nTres"
print(lines(poema))
print(len(lines(poema)), "líneas")
```

```sortie
["a", "b", "c"]
["Uno", "Dos", "Tres"]
3 líneas
```

Con una lista de caracteres, funcionan todas las funciones de listas. Por
ejemplo, contar las vocales:

```rythme
vocales = chars("aeiou")
cantidad = chars("ordenador") |> filter(fn(c) = contains(vocales, c)) |> len
print(cantidad)
```

```sortie
4
```

## Repetir y alinear

`repeat(texto, n)` repite un texto `n` veces. Práctico para una raya:

```rythme
print(repeat("-", 20))
print(repeat("ja", 3))
```

```sortie
--------------------
jajaja
```

`pad_left(texto, ancho)` añade espacios **a la izquierda** hasta el ancho
deseado, y `pad_right` **a la derecha**. Así se alinean columnas:

```rythme
productos = [
  {nombre: "manzana", precio: 3},
  {nombre: "kiwi", precio: 12},
  {nombre: "piña", precio: 150},
]

productos.each do |p|
  print(pad_right(p.nombre, 8) + pad_left(text(p.precio), 5) + " €")
end
```

```sortie
manzana     3 €
kiwi       12 €
piña      150 €
```

Un tercer argumento elige el carácter de relleno:

```rythme
print(pad_left("7", 3, "0"), pad_left("42", 3, "0"))
```

```sortie
007 042
```

## Leer un número en un texto

Un texto como `"42"` no es un número: no se puede calcular con él. Es el
caso de todo lo que un usuario escribe con el teclado. `parse_int` (número
entero) y `parse_float` (número decimal) lo leen. El texto puede no ser un
número, así que dan un `Result`: `Ok(número)` o `Err(mensaje)`.

```rythme
print(parse_int("42"), parse_int(" 7 "))
print(parse_float("3.5"))
print(parse_int("doce"))
```

```sortie
Ok(42) Ok(7)
Ok(3.5)
Err("\"doce\" no es un número entero")
```

Para obtener el número, trata los dos casos con `match`, o da un valor por
defecto con `unwrap_or`:

```rythme
fn doblar(entrada) = match parse_int(entrada)
  Ok(n) -> "el doble es {n * 2}"
  Err(_) -> "«{entrada}» no es un número"
end

print(doblar("21"))
print(doblar("veinte"))
print(parse_int("??") |> unwrap_or(0))
```

```sortie
el doble es 42
«veinte» no es un número
0
```

`Result` se explica en detalle en [gestionar los errores](erreurs.md).

> **Nota:** `int("42")` también lee un número, pero detiene el programa
> (error E022) si el texto no es uno. Guárdalo para un texto escrito por ti;
> para lo que escribe el usuario, mejor `parse_int`.

## Escribir un número grande de forma legible

`format_number(número, decimales, idioma)` separa los miles y redondea. Con
`"es"`, escribe el número a la española:

```rythme
print(format_number(1234567.891, 2, "es"))
print(format_number(1234567.891, 2))
print(text(3.14159, 2))
```

```sortie
1.234.567,89
1,234,567.89
3.14
```

Sin idioma, es la forma inglesa. Y `text(número, decimales)` da simplemente
un número con esa cantidad de decimales.

## Las expresiones regulares

Una **expresión regular** (una «regex») es un pequeño modelo que describe
trozos de texto. Unos pocos signos bastan para empezar:

| Signo | Quiere decir |
|-------|-----------|
| `\d` | una cifra, de 0 a 9 |
| `\w` | una letra, una cifra o `_` |
| `[aeiou]` | uno de estos caracteres |
| `+` | el signo anterior, una vez o más |

Así, `\d+` quiere decir «una o varias cifras».

> **Atención:** en un texto de Rythme, la barra invertida se escribe dos
> veces: la regex `\d+` se escribe `"\\d+"`.

Tres funciones para empezar:

- `matches(texto, regex)`: ¿corresponde el texto **entero** al modelo?
- `find_all(texto, regex)`: la lista de todos los trozos que corresponden;
- `replace_all(texto, regex, nuevo)`: reemplaza todos esos trozos.

```rythme
print(matches("2024", "\\d+"), matches("20h24", "\\d+"))
print(find_all("3 manzanas y 12 peras", "\\d+"))
print(replace_all("Mi código: 1234", "\\d", "*"))
```

```sortie
true false
["3", "12"]
Mi código: ****
```

Por ejemplo, sumar todos los números de una frase:

```rythme
frase = "Tengo 3 manzanas, 12 peras y 5 kiwis."
total = find_all(frase, "\\d+")
  |> map(fn(t) = parse_int(t) |> unwrap_or(0))
  |> sum
print(total)
```

```sortie
20
```

> **Nota:** las regex pueden hacer mucho más. `rythme help find_all` da
> ejemplos, y la referencia de Rythme enumera todos los signos.

## Ejercicios

### Ejercicio 1: Iniciales

Escribe una función `iniciales` que dé las iniciales de un nombre, en
mayúsculas: `iniciales("ada lovelace")` da `"AL"`.

> **Pista:** divide con `split`, toma el primer carácter de cada palabra con
> `p[0]`, y vuelve a unir con `join(lista, "")`.

> **Solución**
>
> ```rythme
> fn iniciales(nombre) = nombre
>   |> split
>   |> map(fn(p) = upper(p[0]))
>   |> join("")
>
> print(iniciales("ada lovelace"))
> print(iniciales("grace brewster hopper"))
> ```
>
> ```sortie
> AL
> GBH
> ```

### Ejercicio 2: Un marco

Escribe una función `marco` que muestre una palabra rodeada de estrellas,
como abajo.

```text
**********
* Rythme *
**********
```

> **Pista:** la línea de estrellas tiene la longitud de la palabra más 4.
> Usa `repeat` y `len`.

> **Solución**
>
> ```rythme
> fn marco(palabra)
>   linea = repeat("*", len(palabra) + 4)
>   print(linea)
>   print("* {palabra} *")
>   print(linea)
> end
>
> marco("Rythme")
> ```
>
> ```sortie
> **********
> * Rythme *
> **********
> ```

## Errores frecuentes

### Calcular directamente con parse_int

```rythme norun
entrada = "41"
print(parse_int(entrada) + 1)
```

```text
error[E005]: no se puede usar `+` con Result y Int
 --> main.ry:2:26
  |
2 | print(parse_int(entrada) + 1)
  |                          ^ Result + Int 1
  |
  = más sobre este error: `rythme explain E005`
```

`parse_int` no da un número, sino un `Result`: una caja que contiene o bien
el número, o bien un error. Ábrela primero:
`(parse_int(entrada) |> unwrap_or(0)) + 1`, o con `match`.

### Olvidar que el texto no cambia

```rythme norun
nombre = "ada"
upper(nombre)
print(nombre)
```

```text
advertencia[W008]: el resultado de `upper` se pierde
 --> main.ry:2:1
  |
2 | upper(nombre)
  | ^^^^^^^^^^^^^ este valor se pierde
  |
  = ayuda: los valores nunca cambian: `upper(text)` da un valor nuevo, y no cambia nada.
      Para conservarlo, dale un nombre: `nuevo = upper(text)`
```

Es una advertencia: el programa se ejecuta, pero muestra `ada`. `upper`
fabrica un texto **nuevo**. Guárdalo: `nombre_may = upper(nombre)`.

### Una sola barra invertida en una regex

```rythme norun
print(matches("2024", "\d+"))
```

```text
error[E018]: secuencia de escape desconocida en el texto
 --> main.ry:1:24
  |
1 | print(matches("2024", "\d+"))
  |                        ^^
  |
  = ayuda: en una regex, escribe la barra invertida dos veces: "\\d" es el texto \d
  = ayuda: escapes válidos: \n (nueva línea), \t (tabulador), \r (retorno de carro), \0 (carácter nulo), \\ (barra invertida),
      \" (comilla), \{ y \} (llaves), \# (un `#` delante de una llave), \u{e9} (un carácter por su número Unicode: é)
  = más sobre este error: `rythme explain E018`
```

En un texto, `\` empieza un carácter especial. Para la regex `\d+`, escribe
`"\\d+"`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto vale `"  Hola  " |> trim |> lower`?

> **Respuesta**
>
> `"hola"`: `trim` quita los espacios de alrededor, y luego `lower` lo pasa
> a minúsculas.

**Pregunta 2:** ¿qué da `split("12/05/2024", "/")`?

> **Respuesta**
>
> La lista `["12", "05", "2024"]`. Siguen siendo textos: para calcular,
> léelos con `parse_int`.

**Pregunta 3:** ¿por qué `parse_int` da un `Result`, y no directamente un
número?

> **Respuesta**
>
> Porque el texto puede no ser un número (`"doce"`). El `Result` te obliga
> a prever ese caso: `Ok(número)` o `Err(mensaje)`.

## En resumen

- Un valor entre llaves entra en un texto: `"total: {x}"`; `\n`, `\"` y
  `\{` escriben caracteres especiales.
- `upper`, `lower`, `capitalize`, `trim` y `replace` dan un texto nuevo;
  `contains`, `starts_with` y `ends_with` responden `true` o `false`.
- `split` divide un texto en una lista, `join` vuelve a unir una lista;
  `chars` y `lines` dan los caracteres y las líneas.
- `pad_left`, `pad_right` y `repeat` sirven para alinear y decorar.
- `parse_int` y `parse_float` dan un `Result`: el texto puede no ser un
  número.
- Una regex describe trozos de texto: `matches`, `find_all` y `replace_all`
  la usan.

Siguiente: [las listas, registros y diccionarios](collections.md). Compara
con los textos en [Ruby](../ruby/texte.md) y en [Rust](../rust/texte.md).
