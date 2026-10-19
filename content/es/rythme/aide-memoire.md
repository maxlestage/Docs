# Chuleta

Todo Rythme de un vistazo: la sintaxis, las funciones más útiles, los comandos, y una tabla para quienes vienen de Ruby.

## Valores y nombres

| Escribir | Para |
|----------|------|
| `nombre = valor` | dar nombre a un valor (para siempre) |
| `42`, `3.14`, `"texto"`, `true` | Int, Float, Text, Bool |
| `"Hola {nombre}"`, `"#{nombre}"` | meter un valor dentro de un texto |
| `# ...`, `## ...` | comentario, comentario de documentación |
| `:ok` | un símbolo |
| `()` | el valor vacío |

```rythme
nombre = "Ada"
edad = 36
puts "#{nombre} tiene {edad} años", type_of(edad)
```

```sortie
Ada tiene 36 años Int
```

Ver [primeros pasos](premiers-pas.md).

## Cálculos y comparaciones

| Operadores | Significado |
|------------|-------------|
| `+ - * /` | cálculo (`/` siempre da un Float) |
| `//`, `%`, `**` | división entera, resto, potencia |
| `== != < <= > >=` | comparaciones |
| `and`, `or`, `not` | lógica |
| `a..b` | lista de los enteros de a a b |
| `x \|> f(a)` | `f(x, a)` |

## Condiciones

```rythme
n = 7
print(if n % 2 == 0 then "par" else "impar")
print(unless n > 10 then "pequeño" else "grande")
print(match n
  0 -> "cero"
  1..9 -> "una cifra"
  _ -> "grande"
end)
```

```sortie
impar
pequeño
una cifra
```

Ver [condiciones y repetición](conditions-boucles.md).

## Funciones

```rythme
fn doble(x) = x * 2

fn presentar(nombre)
  mensaje = "Soy {nombre}"
  mensaje + "."
end

fn fact(0) = 1
fn fact(n) = n * fact(n - 1)

fn signo(n) if n < 0 = "negativo"
fn signo(_) = "positivo o cero"

triple = fn(x) = x * 3
print(doble(4), presentar("Ada"), fact(5), signo(-2), triple(2))
```

```sortie
8 Soy Ada. 120 negativo 6
```

Ver [las funciones](fonctions.md).

## Texto

| Función | Ejemplo | Resultado |
|---------|---------|-----------|
| `len`, `upper`, `lower` | `upper("canción")` | `"CANCIÓN"` |
| `capitalize`, `trim` | `trim("  ok ")` | `"ok"` |
| `split`, `join` | `split("a,b", ",")` | `["a", "b"]` |
| `contains`, `starts_with` | `contains("hola", "ol")` | `true` |
| `replace` | `replace("a-b", "-", "+")` | `"a+b"` |
| `chars`, `lines` | `chars("ab")` | `["a", "b"]` |
| `repeat`, `pad_left` | `pad_left("7", 3, "0")` | `"007"` |
| `parse_int`, `parse_float` | `parse_int("42")` | `Ok(42)` |
| `format_number` | `format_number(1234.5, 1, "es")` | `"1.234,5"` |
| `matches`, `find_all`, `replace_all` | `find_all("a1b22", "\\d+")` | `["1", "22"]` |

```rythme
palabras = "el gato duerme" |> split |> map(capitalize)
print(join(palabras, " "), len(palabras), parse_int("x") |> unwrap_or(0))
```

```sortie
El Gato Duerme 3 0
```

Ver [trabajar con texto](texte.md).

## Colecciones

| Escribir | Da |
|----------|----|
| `[1, 2, 3]`, `xs[0]`, `xs[-1]` | una lista, su primer elemento, el último |
| `[..xs, 4]`, `xs + [4]`, `push(xs, 4)` | una lista nueva con 4 añadido |
| `{nombre: "Ada", edad: 36}`, `r.nombre` | un registro, un campo |
| `{..r, edad: 37}` | una copia con un campo cambiado |
| `dict([["a", 1]])`, `d["a"]`, `put(d, "b", 2)` | un diccionario |
| `set([1, 2, 2])` | un conjunto: `set(1, 2)` |

Ver [listas, registros y diccionarios](collections.md).

## Funciones de listas

| Función | Ejemplo | Resultado |
|---------|---------|-----------|
| `map` | `map([1, 2], fn(x) = x * 10)` | `[10, 20]` |
| `filter`, `reject` | `filter(1..6, is_even)` | `[2, 4, 6]` |
| `fold` | `fold([1, 2, 3], 0, fn(a, x) = a + x)` | `6` |
| `each` | `each(xs, print)` | muestra cada elemento |
| `sum`, `len`, `max`, `min` | `sum([1, 2, 3])` | `6` |
| `sort`, `reverse`, `unique` | `sort([3, 1, 2])` | `[1, 2, 3]` |
| `sort_by`, `max_by` | `sort_by(["ccc", "a"], len)` | `["a", "ccc"]` |
| `first`, `find`, `get` | `first([])` | `None` |
| `take`, `drop` | `take([1, 2, 3], 2)` | `[1, 2]` |
| `join`, `split` | `join(["a", "b"], "-")` | `"a-b"` |

## Estilo funcional y bloques

```rythme
total = [1, 2, 3, 4, 5, 6]
  |> filter(is_even)
  |> map(fn(x) = x * x)
  |> sum

igual = [1, 2, 3, 4, 5, 6]
  .select(even?)
  .map { |x| x * x }
  .sum

[1, 2].each do |x|
  puts "elemento #{x}"
end
print(total, igual)
```

```sortie
elemento 1
elemento 2
56 56
```

Ver [pensar en funcional](fonctionnel.md).

## Tipos y match

```rythme
type Forma = Circulo(radio) | Cuadrado(lado)

fn area(forma) = match forma
  Circulo(r) -> round(pi * r * r, 2)
  Cuadrado(l) -> l * l
end

fn describir([]) = "vacía"
fn describir([x, ..resto]) = "{x} y {len(resto)} más"

print(area(Circulo(1)), area(Cuadrado(3)), describir([7, 8, 9]))
```

```sortie
3.14 9 7 y 2 más
```

Ver [tus propios tipos y match](types.md).

## Errores

| Escribir | Significado |
|----------|-------------|
| `Some(x)`, `None` | un valor presente, ausente (Option) |
| `Ok(x)`, `Err(e)` | un éxito, un fallo (Result) |
| `valor?` | seguir con `x`, o devolver `None` / `Err` |
| `unwrap_or(x, por_defecto)` | el valor, o un valor por defecto |
| `rythme explain E005` | explicar un código de error |

```rythme
fn sumar_textos(a, b)
  x = parse_int(a)?
  y = parse_int(b)?
  Ok(x + y)
end

print(sumar_textos("2", "3"), sumar_textos("2", "tres") |> is_err)
print(first([]) |> unwrap_or(0))
```

```sortie
Ok(5) true
0
```

Ver [gestionar los errores](erreurs.md).

## Teclado y archivos

| Escribir | Da |
|----------|----|
| `input("¿Nombre? ")` | la línea escrita con el teclado (un texto) |
| `args()` | las palabras que siguen al nombre del archivo |
| `write_file(ruta, texto)` | `Ok(())`: reemplaza el contenido |
| `append_file(ruta, texto)` | `Ok(())`: añade al final |
| `read_file(ruta)` | `Ok(texto)` o `Err(mensaje)` |
| `file_exists(ruta)` | `true` o `false` |
| `to_json(valor)`, `parse_json(texto)` | JSON, y la vuelta (un Result) |

```rythme
write_file("compras.txt", "pan\nleche")
append_file("compras.txt", "\nhuevos")
texto = read_file("compras.txt") |> unwrap_or("")
print(lines(texto), file_exists("compras.txt"))
```

```sortie
["pan", "leche", "huevos"] true
```

Ver [leer y escribir: teclado y archivos](fichiers.md).

## Módulos

| Escribir | Para |
|----------|------|
| `use geometria` | usar el archivo `geometria.ry` (sin `.ry`) |
| `geometria.area_cuadrado(4)` | un nombre del módulo, precedido del nombre del módulo |
| `use formas/circulo` | el archivo `formas/circulo.ry`, usado como `circulo` |
| `use formas/circulo as disco` | el mismo, con el nombre `disco` |
| `use ../herramientas/calculos` | `..` sube una carpeta |
| `fn _ayuda(x) = ...` | un nombre privado: invisible desde los demás archivos |

```rythme norun
use geometria              # arriba del archivo, fuera de cualquier función
print(geometria.area_cuadrado(4))
```

Ver [los módulos](modules.md).

## Pruebas

| Escribir | Para |
|----------|------|
| `test "nombre" = assert_eq(obtenido, esperado)` | una prueba en una línea |
| `test "nombre"` ... `end` | una prueba de varias líneas |
| `assert(condicion, mensaje)` | comprobar que una condición es verdadera |
| `rythme test f.ry` | ejecutar las pruebas (`rythme run` las ignora) |
| `rythme test f.ry --filter palabra` | solo las pruebas cuyo nombre contiene `palabra` |
| `rythme test f.ry --watch` | volver a ejecutar tras cada cambio |
| `rythme test .` | las pruebas de todos los archivos de la carpeta |

```rythme
fn cuadrado(x) = x * x

test "cuadrado de 3" = assert_eq(cuadrado(3), 9)

test "varias comprobaciones"
  assert(cuadrado(-2) > 0, "un cuadrado es positivo")
  assert_eq(parse_int("12")?, 12)
end

print(cuadrado(12))
```

```sortie
144
```

Una prueba falla si se detiene en un error, o si termina con `false`,
`Err(...)` o `None`. Ver [probar tu código](tests.md), y luego
[organizar un proyecto](projet.md).

## Comandos

| Comando | Qué hace |
|---------|----------|
| `rythme` | el modo interactivo |
| `rythme run f.ry` | ejecutar un programa |
| `rythme check f.ry` | comprobar sin ejecutar |
| `rythme test f.ry` | ejecutar las pruebas |
| `rythme fmt f.ry` | dar formato al código |
| `rythme doc f.ry` | escribir la documentación |
| `rythme new nombre` | crear un proyecto |
| `rythme help tema` | la ayuda (`rythme help builtins`: todas las funciones) |
| `rythme explain E005` | explicar un error |
| `rythme learn` | la visita guiada |
| `rythme exercise` | los 30 ejercicios |

## Si vienes de Ruby

| Ruby | Rythme |
|------|--------|
| `def doble(x) x * 2 end` | `fn doble(x) = x * 2` |
| `x += 1` | `x2 = x + 1` (un nombre nuevo) |
| `nil` | `None` (y `Some(x)` para un valor) |
| `raise` / `rescue` | `Err(e)` / `match` o `?` |
| `if x then a end` | `if x then a else b` (el `else` es obligatorio) |
| `case x when 1 then ... end` | `match x` y luego `1 -> ...`, cerrado con `end` |
| `for x in xs` | `xs.each do \|x\| ... end` |
| `class Point` | `{x: 1, y: 2}` o `type Point = Point(x, y)` |
| `{ada: 36}` (un hash) | `dict([[:ada, 36]])` o `{ada: 36}` (un registro) |
| `return x` | la última línea es el resultado |
| `x = len xs` | `x = len(xs)` (paréntesis dentro de una expresión) |
| `xs.map { \|x\| x * 2 }` | igual |
| `puts "Hola #{nombre}"` | igual |
| `xs.select(&:even?)` | `xs.select(even?)` |
| `gets` | `input("pregunta ")` |
| `File.read("f.txt")` | `read_file("f.txt")` (un Result) |
| `"42".to_i` | `parse_int("42")` (un Result: puede que el texto no sea un número) |

`rythme help ruby` detalla lo que es igual y lo que cambia. Ver también
[la chuleta de Ruby](../ruby/aide-memoire.md) y
[la de Rust](../rust/aide-memoire.md).

> **Recuerda:** los valores nunca cambian, todo `if` tiene un `else`, la
> última línea de una función es su resultado, y `rythme help` siempre está
> ahí.
