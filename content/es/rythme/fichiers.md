# Leer y escribir: teclado y archivos

Esta página muestra cómo un programa Rythme recibe información (el teclado, la línea de comandos) y cómo guarda sus resultados en archivos: texto, líneas, CSV y JSON.

> **Objetivos:**
>
> - sabrás leer una línea escrita con el teclado, y convertirla en número;
> - sabrás leer las palabras dadas en la línea de comandos;
> - sabrás escribir, completar y leer un archivo de texto, tratando los fallos;
> - sabrás procesar un archivo línea por línea, y leer un pequeño archivo CSV;
> - sabrás guardar datos en JSON.

## Leer del teclado con input

`input(pregunta)` muestra la pregunta, espera a que el usuario escriba una
línea y pulse Intro, y luego da lo que ha escrito, en forma de texto:

```rythme norun
nombre = input("¿Cómo te llamas? ")
print("¡Hola, {nombre}!")
```

En el terminal, esto da:

```text
¿Cómo te llamas? Ada
¡Hola, Ada!
```

Lo que da `input` es siempre un **texto**. Para un número, léelo
con `parse_int`, que da un `Result` (ver
[trabajar con texto](texte.md)):

```rythme norun
entrada = input("¿Tu edad? ")
match parse_int(entrada)
  Ok(edad) -> print("Dentro de diez años, tendrás {edad + 10} años.")
  Err(_) -> print("Eso no es un número.")
end
```

```text
¿Tu edad? 30
Dentro de diez años, tendrás 40 años.
```

> **Nota:** los ejemplos con `input` no se ejecutan en este sitio: esperan
> a alguien frente al teclado. Pruébalos en tu ordenador con `rythme run`.

## Las palabras de la línea de comandos con args

Las palabras escritas después del nombre del archivo, en el terminal, llegan a la
lista `args()`:

```sh
rythme run saludar.ry Ada Alan
```

Aquí está `saludar.ry`. También prevé el caso en que no se da ninguna palabra:

```rythme
fn saludar([]) = print("Da al menos un nombre después del nombre del archivo.")
fn saludar(nombres) = nombres.each do |nombre|
  print("¡Hola, {nombre}!")
end

saludar(args())
```

Ejecutado sin ninguna palabra, muestra:

```sortie
Da al menos un nombre después del nombre del archivo.
```

Y con `Ada Alan`:

```text
¡Hola, Ada!
¡Hola, Alan!
```

## Escribir un archivo con write_file

Un **archivo** es un texto guardado en el disco del ordenador: sigue
ahí cuando el programa termina. El programa lo encuentra por su
**ruta**: su nombre y, si hace falta, las carpetas donde está
(`notas.txt`, `datos/notas.txt`).

`write_file(ruta, contenido)` crea el archivo, o reemplaza todo su contenido.
Escribir puede fallar (una carpeta que no existe, sin permiso para escribir...),
así que la respuesta es un `Result`: `Ok(())` si todo va bien.

```rythme
resultado = write_file("compras.txt", "pan\nleche\nhuevos")
print(resultado)
```

```sortie
Ok(())
```

El archivo `compras.txt` se crea en la carpeta donde ejecutas el
programa. `\n` pasa a la línea siguiente: el archivo tiene tres líneas.

> **Atención:** `write_file` reemplaza **todo** el contenido, sin preguntar.
> Es como pasar una página a limpio: la versión antigua se pierde.

## Leer un archivo con read_file

`read_file(ruta)` da todo el contenido del archivo, dentro de un `Result`:
`Ok(contenido)`, o `Err(mensaje)` si el archivo no existe.

```rythme
write_file("compras.txt", "pan\nleche\nhuevos")

print(read_file("compras.txt"))
print(read_file("olvido.txt"))
```

```sortie
Ok("pan\nleche\nhuevos")
Err("archivo no encontrado: olvido.txt")
```

Tratamos los dos casos con `match`:

```rythme
write_file("compras.txt", "pan\nleche\nhuevos")

match read_file("compras.txt")
  Ok(contenido) -> print(contenido)
  Err(e) -> print("Problema: {e}")
end
```

```sortie
pan
leche
huevos
```

## Pasar el error con ?

En una función, `?` evita un `match` en cada paso: si la lectura
falla, la función se detiene y devuelve el error (ver
[gestionar los errores](erreurs.md#el-operador-para-pasar-el-error)).

```rythme
fn numero_de_lineas(ruta)
  contenido = read_file(ruta)?
  Ok(len(lines(contenido)))
end

write_file("compras.txt", "pan\nleche\nhuevos")
print(numero_de_lineas("compras.txt"))
print(numero_de_lineas("olvido.txt"))
```

```sortie
Ok(3)
Err("archivo no encontrado: olvido.txt")
```

> **Atención:** `?` solo funciona dentro de una función. En el nivel principal del
> programa, usa `match` o `unwrap_or`.

## Añadir al final con append_file

`write_file` reemplaza todo el contenido. `append_file` añade al final, y crea
el archivo si todavía no existe. Es práctico para un diario:

```rythme
append_file("diario.txt", "lunes: 5 km\n")
append_file("diario.txt", "martes: descanso\n")
append_file("diario.txt", "miércoles: 8 km\n")

print(read_file("diario.txt") |> unwrap_or(""))
```

```sortie
lunes: 5 km
martes: descanso
miércoles: 8 km
```

No olvides el `\n` al final de cada línea añadida.

## Saber si un archivo existe

`file_exists(ruta)` responde simplemente `true` o `false`:

```rythme
write_file("notas.txt", "17")

print(file_exists("notas.txt"), file_exists("ausente.txt"))
mensaje = if file_exists("ajustes.txt") then "ajustes encontrados" else "ajustes por defecto"
print(mensaje)
```

```sortie
true false
ajustes por defecto
```

## Procesar un archivo línea por línea

A menudo, se lee el archivo, se corta en líneas con `lines`, y luego se
trabaja con la lista usando `map`, `filter` o `each`:

```rythme
write_file("compras.txt", "pan\nleche\nhuevos")

contenido = read_file("compras.txt") |> unwrap_or("")
lines(contenido) |> each_with_index(fn(articulo, i) = print("{i + 1}. {articulo}"))
```

```sortie
1. pan
2. leche
3. huevos
```

`each_with_index` también da la posición de cada elemento, empezando por 0.

## Un pequeño archivo CSV

Un archivo **CSV** es una tabla escrita como texto: una línea por fila, y
comas entre las columnas. Se lee con `lines`, y luego `split(linea, ",")`
para cada línea:

```rythme
write_file("notas.csv", "Ada,17\nAlan,12\nGrace,19")

fn leer_alumno(linea)
  [nombre, nota] = split(linea, ",")
  {nombre: nombre, nota: parse_int(nota) |> unwrap_or(0)}
end

alumnos = read_file("notas.csv")
  |> unwrap_or("")
  |> lines
  |> map(leer_alumno)

alumnos.each do |a|
  print("{a.nombre}: {a.nota}")
end
print("Media:", sum(map(alumnos, fn(a) = a.nota)) / len(alumnos))
```

```sortie
Ada: 17
Alan: 12
Grace: 19
Media: 16.0
```

`[nombre, nota] = split(...)` da un nombre a cada uno de los dos trozos (ver
[descomponer un valor](collections.md#descomponer-un-valor)).

## El formato JSON

**JSON** es un formato de texto muy extendido para guardar datos.
`to_json` convierte un valor en texto JSON, y `parse_json` vuelve a leer ese
texto. Los registros se convierten en objetos JSON, y al revés.

```rythme
ada = {nombre: "Ada", notas: [17, 19]}
print(to_json(ada))
```

```sortie
{"nombre": "Ada", "notas": [17, 19]}
```

Para conservar datos de una vez para otra, se escribe el JSON en un
archivo, y luego se vuelve a leer. `parse_json` da un `Result`, porque el texto puede
no ser JSON correcto:

```rythme
fn guardar(ruta, valor) = write_file(ruta, to_json(valor))

fn cargar(ruta)
  contenido = read_file(ruta)?
  parse_json(contenido)
end

guardar("ada.json", {nombre: "Ada", notas: [17, 19]})

match cargar("ada.json")
  Ok(alumno) -> print("{alumno.nombre} tiene {len(alumno.notas)} notas.")
  Err(e) -> print("Problema: {e}")
end
print(parse_json("no es json") |> is_err)
```

```sortie
Ada tiene 2 notas.
true
```

> **Consejo:** `to_json(valor, 2)` escribe un elemento por línea, con una sangría de
> 2 espacios: el archivo es más fácil de leer para una persona.

## Errores frecuentes

### Calcular con lo que escribe el usuario

```rythme norun
edad = input("¿Tu edad? ")
print("Dentro de diez años: {edad + 10}")
```

```text
error[E005]: no se puede usar `+` con Text y Int
 --> main.ry:2:35
  |
2 | print("Dentro de diez años: {edad + 10}")
  |                                   ^ Text + Int 10
  |
  = ayuda: para construir un texto con un valor dentro, usa llaves: "total: {x}"
      o convierte antes el valor: text(x)
  = más sobre este error: `rythme explain E005`
```

`input` siempre da un texto, aunque escribas `30`. Léelo primero
con `parse_int`, como en [leer del teclado](#leer-del-teclado-con-input).
El error se detecta incluso antes de hacer la pregunta.

### Olvidar que read_file da un Result

```rythme norun
write_file("compras.txt", "pan\nleche")
contenido = read_file("compras.txt")
print(lines(contenido))
```

```text
error[E005]: `lines` espera un texto como argumento 1, pero recibió un valor de tipo Result
 --> main.ry:3:13
  |
3 | print(lines(contenido))
  |             ^^^^^^^^^ tipo de valor incorrecto
  |
  = ayuda: uso: `lines(text)`
  = más sobre este error: `rythme explain E005`
```

`contenido` es `Ok("pan\nleche")`, no el texto en sí: la lectura podría
haber fallado. Abre el `Result` con `match`, `unwrap_or("")`, o `?` dentro de
una función.

### Equivocarse de separador

```rythme norun
linea = "Ada;17"
[nombre, nota] = split(linea, ",")
print(nombre, nota)
```

```text
error[E006]: el valor List ["Ada;17"] no encaja con este patrón
 --> main.ry:2:1
  |
2 | [nombre, nota] = split(linea, ",")
  | ^^^^^^^^^^^^^^
  |
  = ayuda: usa `match` para tratar las distintas formas posibles de un valor
  = más sobre este error: `rythme explain E006`
```

La línea está separada por `;`, no por `,`: `split` da un solo
trozo, y `[nombre, nota]` espera dos. Mira tu archivo y
escribe `split(linea, ";")`.

## Ponte a prueba

**Pregunta 1:** ¿cuál es la diferencia entre `write_file` y
`append_file`?

> **Respuesta**
>
> `write_file` reemplaza todo el contenido del archivo; `append_file` añade
> al final, sin borrar lo que ya hay.

**Pregunta 2:** ¿qué da `read_file("ausente.txt")` si el archivo no
existe?

> **Respuesta**
>
> `Err("archivo no encontrado: ausente.txt")`. El programa no se detiene:
> te toca a ti tratar este caso.

**Pregunta 3:** el programa `notas.ry` se ejecuta con
`rythme run notas.ry mates 2024`. ¿Cuánto vale `args()`?

> **Respuesta**
>
> `["mates", "2024"]`: una lista de textos, sin el nombre del archivo.

## En resumen

- `input("pregunta ")` lee una línea del teclado, siempre en forma de
  texto; `args()` da las palabras de la línea de comandos.
- `write_file` reemplaza el contenido de un archivo, `append_file` añade al
  final, `file_exists` dice si existe.
- `read_file` da un `Result`: `Ok(contenido)` o `Err(mensaje)`. Dentro de una
  función, `?` pasa el error.
- Para procesar un archivo, se corta con `lines`, y luego `split` para las
  columnas de un CSV.
- `to_json` y `parse_json` escriben y vuelven a leer datos en JSON.

Siguiente: [datos JSON](formats.md)
