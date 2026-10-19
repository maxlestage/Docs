# Encontrar y corregir tus errores

Todo programa tiene bugs al principio: esta página te enseña a leer un mensaje de error, a mostrar valores para entender lo que pasa y a pedir ayuda de forma eficaz.

> **Objetivos:**
>
> - reconocer los tres tipos de errores: de sintaxis, de ejecución y de lógica;
> - leer un mensaje de error línea por línea;
> - mostrar valores para entender lo que hace el programa;
> - reducir un problema a unas pocas líneas;
> - buscar un mensaje de error en internet y pedir ayuda.

## Tres tipos de errores

### Los errores de sintaxis

El código está **mal escrito**: un paréntesis olvidado, un `end` que falta. El
programa ni siquiera arranca. Aquí, a la función le falta su `end`:

```ruby norun
def saludar(nombre)
  puts "¡Hola, #{nombre}!"

saludar("Ada")
```

```text
syntaxe.rb: --> syntaxe.rb
Unmatched keyword, missing `end' ?
> 1  def saludar(nombre)
> 4  saludar("Ada")
syntaxe.rb:4: syntax error, unexpected end-of-input, expecting `end' or dummy end (SyntaxError)
saludar("Ada")
              ^
```

Ruby lo ha adivinado bien: «missing `end' ?», falta un `end`.

### Los errores de ejecución

El código está bien escrito, arranca, pero **se detiene a mitad de camino** porque
una operación es imposible. Aquí, se lee el elemento número 3 de una lista
que solo tiene tres (numerados 0, 1 y 2):

```rust norun
fn main() {
    let notas = vec![12, 15, 9];
    println!("{}", notas[3]);
}
```

```text
thread 'main' (1707) panicked at indice.rs:3:25:
index out of bounds: the len is 3 but the index is 3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

En Rust, se dice que el programa **entra en pánico** (*panic*).

### Los errores de lógica

El programa funciona, **pero el resultado es incorrecto**. No hay ningún mensaje:
es el error más difícil de encontrar. Aquí, queremos la media de 10
y 20:

```ruby
puts 10 + 20 / 2
```

```sortie
20
```

Esperábamos 15. La división va antes que la suma, como en
matemáticas: el ordenador ha calculado `10 + 10`. Había que escribir
`(10 + 20) / 2`.

> **Recuerda:** un error de sintaxis impide arrancar, un error
> de ejecución detiene a mitad de camino, un error de lógica da un resultado
> incorrecto sin decir nada.

## Leer un mensaje de error

Un mensaje de error da miedo, pero siempre sigue el mismo esquema.
Responde a tres preguntas: **qué**, **dónde** y, a menudo, **cómo
corregirlo**.

### En Ruby

```ruby norun
precios = [12, 5, 8]
total = precios.sum
puts "Total: " + total
```

```text
precios.rb:3:in `+': no implicit conversion of Integer into String (TypeError)

puts "Total: " + total
                 ^^^^^
	from precios.rb:3:in `<main>'
```

Línea por línea:

1. `precios.rb:3`: el archivo y la **línea** del problema;
2. ``in `+'``: durante la operación `+`;
3. `no implicit conversion of Integer into String`: **qué**: Ruby no
   convierte por su cuenta un número (*Integer*) en texto (*String*);
4. `(TypeError)`: el **nombre** del error, útil para buscar en internet;
5. la línea de código, con `^^^^^` bajo la parte que falla;
6. `from …`: el camino que siguió el programa hasta el error.

Corrección: `puts "Total: #{total}"`.

### En Rust

```rust norun
fn main() {
    let edad: i32 = "36";
    println!("{edad}");
}
```

```text
error[E0308]: mismatched types
 --> edad.rs:2:21
  |
2 |     let edad: i32 = "36";
  |               ---   ^^^^ expected `i32`, found `&str`
  |               |
  |               expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

1. `error[E0308]: mismatched types`: **qué**: los tipos no
   coinciden; `E0308` es el código del error;
2. `--> edad.rs:2:21`: **dónde**: archivo, línea 2, columna 21;
3. la línea de código, con `^^^^` bajo el problema: se esperaba un
   número (`i32`) y se encontró un texto (`&str`);
4. `---` muestra la causa: es `i32` lo que pide un número;
5. la última línea da el comando para obtener una explicación completa.

Corrección: `let edad: i32 = 36;`, sin comillas.

### En Python

```python norun
precios = [12, 5, 8]
total = sum(precios)
print("Total: " + total)
```

```text
Traceback (most recent call last):
  File "precios.py", line 3, in <module>
    print("Total: " + total)
          ~~~~~~~~~~^~~~~~~
TypeError: can only concatenate str (not "int") to str
```

A diferencia de Ruby y Rust, Python pone lo esencial **abajo**: lee
primero la última línea.

1. `Traceback`: el camino que siguió el programa, de arriba abajo;
2. `File "precios.py", line 3`: **dónde**: el archivo y la línea;
3. la línea de código, con `^` bajo la operación que falla;
4. `TypeError: can only concatenate str (not "int") to str`: **qué**:
   solo se puede pegar (*concatenate*) un texto (`str`) a un texto, no
   un número (`int`).

Corrección: `print(f"Total: {total}")`.

### En JavaScript

```javascript norun
const alumno = { nombre: "Ada", notas: [12, 15] };
console.log(alumno.nota.length);
```

```text
file:///home/ada/notas.mjs:2
console.log(alumno.nota.length);
                        ^

TypeError: Cannot read properties of undefined (reading 'length')
    at file:///home/ada/notas.mjs:2:25
```

1. `notas.mjs:2`: **dónde**: el archivo y la línea;
2. la línea de código, con `^` bajo el lugar del problema;
3. `TypeError: Cannot read properties of undefined (reading 'length')`:
   **qué**: se pide `length` a algo que vale `undefined`
   («nada»);
4. `at …`: el camino que siguió el programa.

`alumno.nota` no existe: la propiedad se llama `notas`. JavaScript no
lo señala enseguida, devuelve `undefined`, y el error llega un
paso más tarde.

### En Rythme

```rythme norun
notas = [12, 15, 9]
print(len(nota))
```

```text
error[E003]: nombre desconocido `nota`
 --> nombre.ry:2:11
  |
2 | print(len(nota))
  |           ^^^^ no definido
  |
  = ayuda: ¿quisiste decir `notas`?
  = más sobre este error: `rythme explain E003`
```

1. `error[E003]: nombre desconocido`: **qué**, en español;
2. `--> nombre.ry:2:11`: **dónde**;
3. la línea, con `^^^^` bajo el nombre desconocido;
4. `ayuda`: **cómo corregirlo**: aquí, una simple errata;
5. `rythme explain E003`: una explicación detallada, con ejemplos.

> **Consejo:** cuando hay varios errores, corrige **el primero**
> y vuelve a ejecutar. Los siguientes suelen ser consecuencias del
> primero.

## Mostrar valores para entender

Cuando el resultado es incorrecto, mira lo que contienen tus variables.
Añade salidas por pantalla en medio del programa.

### En Ruby: puts y p

`puts` muestra un valor de forma sencilla. `p` lo muestra **tal como está
en el código**: se ven las comillas de un texto y los corchetes de una
lista. Para depurar, mejor `p`.

```ruby
precios = [12, 5, 8]
p precios
total = precios.sum
p total
puts "Media: #{total / precios.size}"
```

```sortie
[12, 5, 8]
25
Media: 8
```

```ruby
puts "42"
p "42"
puts nil
p nil
```

```sortie
42
"42"

nil
```

Con `puts`, no se ve si `42` es un número o un texto, ni si un
valor es `nil`. Con `p`, sí.

### En Rust: println! y dbg!

`println!("{:?}", x)` muestra un valor tal como es. `dbg!` hace
más: muestra el archivo, la línea, el cálculo y su resultado, y luego
**devuelve el valor**. Así que se puede meter en medio de un cálculo.

```rust
fn main() {
    let precios = [12, 5, 8];
    let total: i32 = precios.iter().sum();
    println!("{:?} {total}", precios);
    let media = dbg!(total / 3);
    println!("Media: {media}");
}
```

En la terminal:

```text
[12, 5, 8] 25
[main.rs:5:17] total / 3 = 8
Media: 8
```

`dbg!` escribe en la **salida de errores**, aparte: no se mezcla con los
resultados reales del programa si los guardas en un archivo.

### En Rythme: print y debug

`print` muestra. `debug` muestra el valor con el archivo y la línea, y luego
lo devuelve, como `dbg!`:

```rythme
precios = [12, 5, 8]
total = debug(sum(precios))
print("Media: {total / len(precios)}")
```

En la terminal:

```text
[debug main.ry:2:9] 25
Media: 8.333333333333334
```

### En Python: print y el signo =

En un texto `f"…"`, escribir `{total=}` muestra el **nombre** de la variable
y su valor. `repr` muestra un valor tal como está en el código, con
las comillas de un texto:

```python
precios = [12, 5, 8]
total = sum(precios)
print(f"{precios=} {total=}")
print("42", repr("42"))
```

```sortie
precios=[12, 5, 8] total=25
42 '42'
```

### En JavaScript: console.log

Pon las variables entre llaves: `console.log` muestra entonces cada
nombre con su valor. `JSON.stringify` muestra las comillas de un texto:

```javascript
const precios = [12, 5, 8];
const total = precios.reduce((suma, p) => suma + p, 0);
console.log({ precios, total });
console.log("42", JSON.stringify("42"));
```

```sortie
{ precios: [ 12, 5, 8 ], total: 25 }
42 "42"
```

> **Atención:** una vez encontrado el bug, quita tus `p`, `dbg!`,
> `debug`, `print` y `console.log` de depuración. No deben quedarse
> en el programa final.

## El patito de goma

Un método muy serio, a pesar de su nombre: pon un patito de goma (o
cualquier objeto) al lado de la pantalla, y **explícale tu código,
línea por línea, en voz alta**.

«Aquí pongo el primer precio en `total`. Luego, para cada precio… ¡ah,
pero empiezo en 1, no en 0!»

Al explicarlo, te ves obligado a decir lo que el código **hace de verdad**, y
no lo que **crees** que hace. Muy a menudo, encuentras el bug antes
de terminar. También funciona con un amigo, aunque no programe.

## Reducir el problema

Un bug escondido en 200 líneas es difícil de encontrar. En 5 líneas,
salta a la vista.

1. **Copia** tu programa en un archivo nuevo.
2. **Quita** un trozo que no parezca relacionado con el problema.
3. **Vuelve a ejecutar**: ¿sigue estando el bug?
   - sí: sigue quitando;
   - no: el bug estaba en el trozo quitado. Vuelve a ponerlo y busca
     dentro.
4. Detente cuando solo queden **unas pocas líneas**.

Sustituye también los datos leídos de un archivo o del teclado por
valores escritos en el código: `notas = [12, 15, 9]`.

> **Consejo:** cambia **una sola cosa cada vez** y vuelve a ejecutar. Si
> cambias tres cosas, no sabrás cuál ha influido.

## Buscar el mensaje en internet

Seguramente no eres la primera persona que se encuentra con ese mensaje.
Cópialo en un buscador:

- conserva la parte **general**: `no implicit conversion of Integer into
  String`;
- quita lo que es propio de **tu** programa: nombres de archivo,
  números de línea, nombres de variables;
- añade el **nombre del lenguaje**: `python`, `javascript`, `go`, `ruby`,
  `rust`…

Los códigos de error se buscan muy bien: `rust E0308`. Y se
explican sin internet:

```sh
rustc --explain E0308
rythme explain E003
```

## Pedir ayuda

Si llevas mucho tiempo atascado, pregunta: en un foro, a un amigo, a un
asistente. Una buena pregunta recibe rápido una buena respuesta. Contiene:

1. **lo que quieres hacer**: «calcular la media de mis notas»;
2. **un ejemplo mínimo**: las pocas líneas que muestran el problema,
   en texto (no una foto de la pantalla);
3. **lo que esperabas**: «12»;
4. **lo que pasa**: el resultado obtenido, o el mensaje de error
   **completo**, copiado y pegado;
5. **lo que ya has probado**.

Por ejemplo:

```text
Quiero la media de 10 y 20 en Ruby. Mi código:

    puts 10 + 20 / 2

Espero 15, pero obtengo 20. He comprobado que 10 + 20 da 30.
¿Por qué?
```

A menudo, al preparar el ejemplo mínimo, encontrarás la respuesta
tú mismo.

## Errores frecuentes

### Leer solo la última línea del mensaje

En Ruby, el mensaje empieza arriba y el camino `from …` va después. En Rust,
suele haber varios errores seguidos:

```rust norun
fn main() {
    let total = 0;
    total = total + 5;
    println!("{total}");
}
```

```text
error[E0384]: cannot assign twice to immutable variable `total`
 --> mu.rs:3:5
  |
2 |     let total = 0;
  |         ----- first assignment to `total`
3 |     total = total + 5;
  |     ^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut total = 0;
  |         +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0384`.
```

La última línea casi no dice nada. Todo está **arriba**: el problema
(`cannot assign twice`) e incluso la corrección (`help: … let mut total`).

### Buscar en el sitio equivocado

La línea indicada no siempre es la que hay que corregir. Para un `end`
olvidado, Ruby se queja al **final** del archivo:

```text
syntaxe.rb:4: syntax error, unexpected end-of-input, expecting `end' or dummy end (SyntaxError)
```

El problema está más arriba, en la línea 1: la función que nunca se
cierra. Si la línea indicada parece correcta, mira las líneas **de encima**.

### No ver la diferencia entre un número y un texto

`puts` muestra `42` tanto para el número como para el texto `"42"`:

```ruby norun
edad = "36"
puts edad + 1
```

```text
main.rb:2:in `+': no implicit conversion of Integer into String (TypeError)

puts edad + 1
            ^
	from main.rb:2:in `<main>'
```

Un `p edad` habría mostrado `"36"`, con las comillas: es un texto.
Conviértelo con `edad.to_i`, o escribe `edad = 36`.

## Ponte a prueba

**1.** Tu programa muestra `20` en lugar de `15`, sin ningún mensaje. ¿Qué
tipo de error es?

> **Respuesta**
>
> Un error de lógica: el programa funciona, pero el resultado es incorrecto.
> Muestra los valores intermedios (`print` en Python, `console.log` en
> JavaScript, `p` en Ruby, `dbg!` en Rust, `debug` en Rythme) para encontrar
> el paso que falla.

**2.** En el mensaje `--> edad.rs:2:21`, ¿qué significan `2` y `21`?

> **Respuesta**
>
> El error está en el archivo `edad.rs`, en la línea 2, columna 21 (el
> carácter número 21 de la línea).

**3.** ¿Qué debe contener una buena petición de ayuda?

> **Respuesta**
>
> Lo que quieres hacer, un ejemplo mínimo en texto, el resultado
> esperado, el resultado obtenido (o el mensaje de error completo) y lo que
> ya has probado.

## En resumen

- Tres tipos de errores: de sintaxis (no arranca), de ejecución (se detiene a mitad de camino), de lógica (resultado incorrecto).
- Un mensaje de error dice qué, dónde (archivo, línea, columna) y a menudo cómo corregirlo: léelo entero.
- Muestra los valores: `print(f"{x=}")` en Python, `console.log({ x })` en JavaScript, `p` en Ruby, `dbg!` en Rust, `debug` en Rythme.
- Explícale tu código al patito y reduce el problema a unas pocas líneas.
- Busca en internet la parte general del mensaje; para pedir ayuda, da un ejemplo mínimo.

Siguiente: [guardar tu trabajo con Git](git.md).
