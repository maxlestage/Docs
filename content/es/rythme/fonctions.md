# Las funciones

En Rythme, casi todo es una función: esta página muestra cómo escribirlas en una línea o en bloque, por casos, con guardas, sin nombre, y cómo llamarlas.

> **Objetivos:**
>
> - sabrás escribir una función en una línea o en bloque, y llamarla;
> - sabrás distinguir entre parámetro y argumento;
> - sabrás definir una función por casos, con patrones y guardas;
> - sabrás escribir una función anónima y dársela a otra función;
> - sabrás por qué Rythme no tiene `return`.

## Una función en una línea

Una [función](../debuter/glossaire.md#funcion) es una pequeña máquina: le
das valores y te da un resultado. Como una tostadora: entra pan y sale pan
tostado.

Se escribe con la palabra clave `fn`, el nombre, los parámetros entre
paréntesis, `=` y luego el resultado:

```rythme
fn doble(x) = x * 2
fn area(ancho, alto) = ancho * alto

print(doble(21), area(3, 4))
```

```sortie
42 12
```

No hay `return`: lo que sigue a `=` **es** el resultado.

## Parámetros y argumentos

Hay dos palabras que aparecen todo el tiempo:

- un [parámetro](../debuter/glossaire.md#parametro) es el nombre escrito en
  la definición: `ancho` y `alto` en `fn area(ancho, alto)`;
- un [argumento](../debuter/glossaire.md#argumento) es el valor que se da en
  la llamada: `3` y `4` en `area(3, 4)`.

En la llamada, `ancho` recibe 3 y `alto` recibe 4, en orden. Hay que dar
exactamente un argumento por parámetro.

**Llamar** a una función es escribir su nombre seguido de los argumentos
entre paréntesis: `doble(21)`. El resultado se puede usar en cualquier sitio
donde se permita un valor: `doble(doble(5))` vale 20.

## Una función en bloque

Cuando hacen falta varias líneas, se quita el `=` y se cierra con `end`.
**La última línea da el resultado**:

```rythme
fn presentar(nombre, edad)
  anio = 2024 - edad
  "{nombre} nació hacia {anio}."
end

print(presentar("Ada", 36))
```

```sortie
Ada nació hacia 1988.
```

Los nombres definidos dentro de una función, como `anio`, solo existen
dentro de la función: es su [ámbito](../debuter/glossaire.md#ambito). Fuera,
`anio` es desconocido.

Una función sin parámetros conserva sus paréntesis, tanto en la definición
como en la llamada:

```rythme
fn hola() = "¡hola!"
print(hola())
```

```sortie
¡hola!
```

## Definir por casos

Una función puede tener varias definiciones, escritas una debajo de otra.
Rythme las prueba **en orden** y elige la primera que encaja:

```rythme
fn fib(0) = 0
fn fib(1) = 1
fn fib(n) = fib(n - 1) + fib(n - 2)

print(fib(10))
```

```sortie
55
```

Aquí, `fib(0)` usa la primera definición, `fib(1)` la segunda, y todos los
demás números la tercera. Es como una fila de ventanillas: vas a la primera
que acepta tu petición.

Los parámetros son **patrones**, como en `match`. Por ejemplo, se puede
reconocer una lista vacía, o una lista de un solo elemento:

```rythme
fn describir([]) = "una lista vacía"
fn describir([x]) = "un solo elemento: {x}"
fn describir(lista) = "{len(lista)} elementos"

print(describir([]), "/", describir([7]), "/", describir([1, 2, 3]))
```

```sortie
una lista vacía / un solo elemento: 7 / 3 elementos
```

## Las guardas

Una **guarda** añade una condición a una definición: `if` después de los
parámetros, antes del `=`. La definición solo se elige si la condición es
verdadera.

```rythme
fn categoria(edad) if edad < 13 = "niño"
fn categoria(edad) if edad < 18 = "adolescente"
fn categoria(_) = "adulto"

print(categoria(8), categoria(15), categoria(40))
```

```sortie
niño adolescente adulto
```

`_` significa «cualquier valor, no lo uso». Pon siempre el caso más general
al final: si no, lo atraparía todo.

## Las llamadas sin paréntesis

Al principio de una línea, puedes llamar a una función sin paréntesis, como
en Ruby:

```rythme
fn saludar(nombre, saludo) = print("¡#{saludo}, #{nombre}!")

saludar "Ada", "Hola"
saludar("Alan", "Buenas")
```

```sortie
¡Hola, Ada!
¡Buenas, Alan!
```

> **Atención:** esto solo funciona al principio de una instrucción (o de un
> resultado). En medio de una expresión, los paréntesis siguen siendo
> obligatorios: `x = len(xs)`, no `x = len xs`.

## La llamada en estilo método

`valor.f(args)` es otra forma de escribir `f(valor, args)`: el valor delante
del punto se convierte en el primer argumento. Sin más argumentos, los
paréntesis son opcionales:

```rythme
print("hola".upper(), [3, 1, 2].sort, 10.max(20))
```

```sortie
HOLA [1, 2, 3] 20
```

También funciona con tus funciones:

```rythme
fn doble(x) = x * 2
print(21.doble)
```

```sortie
42
```

## Los nombres que terminan en ?

Una función que responde sí o no puede terminar en `?`, como en Ruby:

```rythme
fn adulta?(persona) = persona.edad >= 18

ada = {nombre: "Ada", edad: 36}
print(adulta?(ada), ada.adulta?, even?(4))
```

```sortie
true true true
```

`{nombre: "Ada", edad: 36}` es un registro, y `persona.edad` lee su campo
`edad` (ver [las colecciones](collections.md)).

## Las funciones anónimas

Una función sin nombre se escribe `fn(parámetros) = resultado`. Es un valor
como cualquier otro: se puede guardar en un nombre o dar a otra función.

```rythme
triple = fn(x) = x * 3
print(triple(5))
print([1, 2, 3] |> map(fn(x) = x + 10))
```

```sortie
15
[11, 12, 13]
```

`map` aplica la función a cada elemento de la lista. A menudo se escribe una
función anónima cuando solo sirve una vez, en ese lugar.

## Una función que recuerda: la closure

Una función anónima puede usar los nombres escritos antes que ella:

```rythme
factor = 3
print([1, 2, 3] |> map(fn(x) = x * factor))
```

```sortie
[3, 6, 9]
```

Una función puede incluso fabricar otra función. Esta se lleva consigo el
valor que recibió, como una mochila:

```rythme
fn sumador(n) = fn(x) = x + n

mas10 = sumador(10)
mas1 = sumador(1)
print(mas10(5), mas1(5))
```

```sortie
15 6
```

Una función así, que se lleva valores del lugar donde fue escrita, se llama
[closure](../debuter/glossaire.md#closure) (en español, un «cierre» o
«clausura»). Como los valores no cambian nunca, lo que se lleva tampoco
cambia.

## Las funciones son valores

Una función con nombre también se puede dar a otra función, por su nombre,
**sin paréntesis**:

```rythme
fn cuadrado(x) = x * x
fn aplicar_dos_veces(f, x) = f(f(x))

print(aplicar_dos_veces(cuadrado, 3), map([1, 2, 3], cuadrado))
```

```sortie
81 [1, 4, 9]
```

`cuadrado` es la función misma; `cuadrado(3)` es su resultado, 9.

## Errores frecuentes

### Escribir return

```rythme norun
fn doble(x)
  return x * 2
end
print(doble(4))
```

```text
error[E001]: `return` no se usa en Rythme
 --> main.ry:2:3
  |
2 |   return x * 2
  |   ^^^^^^ viene de otro lenguaje
  |
  = ayuda: no hace falta `return`: la última expresión de una función es su resultado.
      Para parar antes con un error, devuelve `Err(...)` y usa `?`.
  = más sobre este error: `rythme explain E001`
```

Quita `return`: la última línea, `x * 2`, ya es el resultado. O escríbelo
todo en una línea: `fn doble(x) = x * 2`.

### Olvidar un argumento

```rythme norun
fn area(ancho, alto) = ancho * alto
print(area(3))
```

```text
error[E004]: `area` espera 2 argumentos, pero recibió 1
 --> main.ry:2:7
  |
2 | print(area(3))
  |       ^^^^^^^ 1 argumento(s) aquí
  |
1 | fn area(ancho, alto) = ancho * alto
  |    ---- definida aquí
  |
  = ayuda: uso: `area(ancho, alto)`
  = más sobre este error: `rythme explain E004`
```

Rythme no tiene valores por defecto: da un argumento por parámetro,
`area(3, 4)`.

### Olvidar los paréntesis de una función sin parámetros

```rythme norun
fn hola = "hola"
print(hola())
```

```text
error[E001]: se esperaba `(` y los parámetros, pero se encontró `=`
 --> main.ry:1:9
  |
1 | fn hola = "hola"
  |         ^ se esperaba `(` y los parámetros aquí
  |
  = ayuda: incluso sin parámetros, escribe los paréntesis: `fn saludar() = print("hola")`
  = más sobre este error: `rythme explain E001`
```

Escribe `fn hola() = "hola"`.

### Olvidar el end de un bloque

```rythme norun
fn presentar(nombre)
  "Hola, {nombre}"

print(presentar("Ada"))
```

```text
error[E020]: falta `end` para cerrar este `fn`
 --> main.ry:1:1
  |
1 | fn presentar(nombre)
  | ^^ este `fn` nunca se cierra
  |
5 | 
  | - el código termina aquí
  |
  = ayuda: añade `end` en su propia línea después de la última línea del bloque
  = más sobre este error: `rythme explain E020`
```

Una función sin `=` es un bloque: ciérrala con `end`, debajo de su última
línea.

## Ponte a prueba

**Pregunta 1:** en `fn precio(cantidad, unitario) = cantidad * unitario` y
la llamada `precio(3, 2)`, ¿cuáles son los parámetros y cuáles los
argumentos?

> **Respuesta**
>
> Los parámetros son `cantidad` y `unitario`; los argumentos son `3` y `2`.
> El resultado es 6.

**Pregunta 2:** ¿qué muestra este programa?

```rythme norun
fn signo(n) if n < 0 = "negativo"
fn signo(0) = "cero"
fn signo(_) = "positivo"
print(signo(0), signo(-2), signo(5))
```

> **Respuesta**
>
> `cero negativo positivo`. Las definiciones se prueban en orden: `0` no es
> negativo, así que usa la segunda.

**Pregunta 3:** ¿cuánto vale `sumador(3)(4)`, con
`fn sumador(n) = fn(x) = x + n`?

> **Respuesta**
>
> `7`. `sumador(3)` da una función que suma 3; se llama enseguida con 4.

## En resumen

- `fn nombre(x) = ...` escribe una función en una línea; `fn nombre(x) ... end`
  en bloque, donde la última línea es el resultado.
- No hay `return`; hace falta un argumento por cada parámetro.
- Varias definiciones por casos se prueban en orden; una guarda `if` añade
  una condición.
- `valor.f(args)` es otra forma de escribir `f(valor, args)`.
- `fn(x) = ...` es una función sin nombre, que se puede dar a `map`; se
  lleva los nombres escritos antes que ella (una closure).

Siguiente: [trabajar con texto](texte.md). Compara con las funciones de
[Ruby](../ruby/fonctions.md) y de [Rust](../rust/fonctions.md).
