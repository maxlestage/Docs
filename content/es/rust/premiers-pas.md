# Primeros pasos

Esta página presenta las variables, los tipos básicos, cómo mostrar con `println!`, los comentarios y las constantes en Rust.

> **Objetivos:**
>
> - crear variables con `let`, y hacerlas modificables con `mut`;
> - elegir el tipo de número adecuado y convertir con `as`;
> - agrupar valores en una tupla;
> - mostrar valores con `println!` y `format!`;
> - escribir comentarios y constantes.

## Las variables con let

Una [variable](../debuter/glossaire.md#variable) es una etiqueta puesta sobre un
valor. Se crea con `let`. Por defecto, **no** puede cambiar: es
[inmutable](../debuter/glossaire.md#inmutable).

```rust
let ciudad = "Valencia";
println!("{ciudad}");
```

```sortie
Valencia
```

Si intentas modificarla, el compilador se niega (ver
[Errores frecuentes](#errores-frecuentes)).

## Las variables modificables con mut

Añade `mut` para hacer la variable [mutable](../debuter/glossaire.md#mutable),
es decir, modificable:

```rust
let mut contador = 0;
contador = contador + 1;
contador += 1;
println!("{contador}");
```

```sortie
2
```

`contador += 1` es un atajo de `contador = contador + 1`. También existen `-=`,
`*=` y `/=`.

> **Recuerda:** en Rust, todo es constante por defecto. Escribe `mut` solo cuando
> el valor tenga que cambiar.

## El sombreado

Puedes declarar una variable nueva con el mismo nombre. La anterior queda **sombreada**
(en inglés: *shadowing*): todavía existe, pero el nombre designa ahora la
nueva. El tipo puede incluso cambiar:

```rust
let texto = "42";
let texto: i32 = texto.parse().unwrap();
let texto = texto * 2;
println!("{texto}");
```

```sortie
84
```

Aquí, `texto` es primero un texto y luego un número. `parse` lee un número en un texto
(detalles en [Trabajar con texto](texte.md)).

> **Nota:** sombrear no es modificar. Con `let`, creas una **nueva**
> variable; con `mut`, cambias el valor de la misma variable.

## Los tipos simples

Un [tipo](../debuter/glossaire.md#tipo) dice qué clase de valor contiene una
variable. Rust suele adivinar el tipo solo. También puedes escribirlo después de `:`.

| Tipo | Contenido | Ejemplo |
|------|---------|---------|
| `i32` | [entero](../debuter/glossaire.md#entero) (positivo o negativo) | `-7` |
| `u32` | entero positivo | `42` |
| `f64` | número decimal ([flotante](../debuter/glossaire.md#flotante)) | `3.14` |
| `bool` | verdadero o falso ([booleano](../debuter/glossaire.md#booleano)) | `true` |
| `char` | un carácter | `'ñ'` |

```rust
let entero: i32 = -7;
let positivo: u32 = 42;
let precio: f64 = 9.99;
let activo: bool = true;
let letra: char = 'ñ';
println!("{entero} {positivo} {precio} {activo} {letra}");
println!("{}", 7 / 2);
println!("{}", 7.0 / 2.0);
println!("{}", 7 % 2);
```

```sortie
-7 42 9.99 true ñ
3
3.5
1
```

> **Atención:** una división entre enteros da un entero: `7 / 2` vale `3`. El
> `%` da el resto de la división: `7 % 2` vale `1`.

### Los tamaños de enteros

Un entero ocupa un espacio fijo en memoria. La letra dice si puede ser negativo (`i`
de *integer*, con signo) o no (`u` de *unsigned*, sin signo). El número da su
tamaño, en bits:

| Tipo | Valor más pequeño | Valor más grande |
|------|--------------------|--------------------|
| `u8` | 0 | 255 |
| `i32` | −2 147 483 648 | 2 147 483 647 |
| `u32` | 0 | 4 294 967 295 |
| `i64` | aproximadamente −9 × 10¹⁸ | aproximadamente 9 × 10¹⁸ |
| `usize` | 0 | depende de la máquina (las posiciones en una lista) |

```rust
println!("{} {}", u8::MIN, u8::MAX);
println!("{}", i32::MAX);
```

```sortie
0 255
2147483647
```

> **Consejo:** si no hay una razón especial, elige `i32` para un entero y `f64` para un
> número decimal. Son las opciones por defecto de Rust.

### El desbordamiento

¿Qué pasa si un cálculo se sale de los límites? Es un **desbordamiento** (en
inglés: *overflow*). Durante el desarrollo, el programa se detiene con un error.
Para elegir tú mismo el comportamiento, existen métodos:

```rust
let x: u8 = 250;
println!("{}", x.saturating_add(10));
println!("{}", x.wrapping_add(10));
```

```sortie
255
4
```

- `saturating_add` se queda en el máximo, como un indicador lleno;
- `wrapping_add` vuelve a empezar desde cero, como un cuentakilómetros que da la vuelta.

### Convertir con as

Rust nunca mezcla dos tipos de números por su cuenta. Para convertir, escribe `as`
seguido del tipo deseado:

```rust
let manzanas: i32 = 7;
let mitad = manzanas as f64 / 2.0;
println!("{mitad}");
let precio = 3.99;
println!("{}", precio as i32);
let grande: i32 = 300;
println!("{}", grande as u8);
```

```sortie
3.5
3
44
```

> **Atención:** `as` no comprueba nada. De `f64` a `i32`, corta los decimales
> (`3.99` se convierte en `3`, sin redondear). Hacia un tipo demasiado pequeño, el resultado es
> sorprendente: `300` convertido a `u8` vale `44`.

## Las tuplas

Una **tupla** agrupa unos pocos valores, cuyos tipos pueden ser diferentes, entre
paréntesis. Cada valor se lee con `.0`, `.1`, `.2`…:

```rust
let punto = (3, 4);
let alumna = ("Lucía", 17, true);
println!("{} {}", punto.0, punto.1);
println!("{:?}", alumna);
let (nombre, edad, _) = alumna;
println!("{nombre} tiene {edad} años");
```

```sortie
3 4
("Lucía", 17, true)
Lucía tiene 17 años
```

La línea `let (nombre, edad, _) = alumna;` **descompone** la tupla: cada valor va a
su variable. El `_` ignora el tercero.

## Los textos: String y &str

Rust tiene dos tipos de texto (de [cadena de
caracteres](../debuter/glossaire.md#cadena-de-caracteres)):

- `&str`: un texto fijo, escrito a menudo entre comillas en el código.
- `String`: un texto que te pertenece y que puedes hacer crecer.

```rust
let saludo: &str = "Hola";
let mut mensaje = String::from(saludo);
mensaje.push_str(", mundo");
mensaje.push('!');
println!("{mensaje}");
println!("{}", mensaje.len());
```

```sortie
Hola, mundo!
12
```

El texto tiene su propia página: [Trabajar con texto](texte.md).

## Mostrar con println! y format!

`println!` muestra, `format!` fabrica un `String` sin mostrarlo. Las `{}` son
huecos, que se reemplazan por los valores:

```rust
let nombre = "Lucía";
let nota = 17.456;
println!("{} sacó {}", nombre, nota);
println!("{nombre} sacó {nota:.1}");
println!("[{:>5}]", 42);
println!("[{:<5}]", 42);
println!("{:?}", "con comillas");
let frase = format!("¡{nombre}!");
println!("{frase}");
```

```sortie
Lucía sacó 17.456
Lucía sacó 17.5
[   42]
[42   ]
"con comillas"
¡Lucía!
```

| Hueco | Efecto |
|-------------|-------|
| `{}` | se muestra normal |
| `{nombre}` | muestra la variable `nombre` |
| `{:.2}` | dos cifras decimales |
| `{:>5}` | alineado a la derecha en 5 caracteres |
| `{:?}` | se muestra en modo depuración |

`print!` (sin `ln`) muestra sin pasar a la línea siguiente.

## Los comentarios

Un [comentario](../debuter/glossaire.md#comentario) es una nota para las personas:
el compilador lo ignora.

```rust
// Un comentario de una línea.
/* Un comentario
   de varias líneas. */
/// Un comentario de documentación (antes de una función, una estructura…).
fn doble(n: i32) -> i32 {
    n * 2
}

fn main() {
    println!("{}", doble(4)); // muestra 8
}
```

```sortie
8
```

## Las constantes

Una [constante](../debuter/glossaire.md#constante) es un valor fijado de una vez por
todas. Se escribe con `const`, en MAYÚSCULAS, y su tipo es obligatorio:

```rust
const TASA_IVA: f64 = 0.2;
const DIAS_POR_SEMANA: u32 = 7;

fn main() {
    println!("{}", 100.0 * TASA_IVA);
    println!("{}", DIAS_POR_SEMANA * 4);
}
```

```sortie
20
28
```

## Errores frecuentes

### Modificar una variable sin mut

```rust norun
fn main() {
    let puntos = 10;
    puntos = 20;
    println!("{puntos}");
}
```

```text
error[E0384]: cannot assign twice to immutable variable `puntos`
 --> src/main.rs:3:5
  |
2 |     let puntos = 10;
  |         ------ first assignment to `puntos`
3 |     puntos = 20;
  |     ^^^^^^^^^^^ cannot assign twice to immutable variable
help: consider making this binding mutable
```

Corrección: `let mut puntos = 10;`.

### Mezclar enteros y números decimales

```rust norun
fn main() {
    let total = 5 + 2.5;
    println!("{total}");
}
```

```text
error[E0277]: cannot add a float to an integer
 --> src/main.rs:2:19
  |
2 |     let total = 5 + 2.5;
  |                   ^ no implementation for `{integer} + {float}`
```

Corrección: escribe `5.0 + 2.5`, o convierte una variable con `as f64`.

### Un número demasiado grande para su tipo

```rust norun
fn main() {
    let pequeno: u8 = 300;
    println!("{pequeno}");
}
```

```text
error: literal out of range for `u8`
 --> src/main.rs:2:23
  |
2 |     let pequeno: u8 = 300;
  |                       ^^^
  |
  = note: the literal `300` does not fit into the type `u8` whose range is `0..=255`
```

Corrección: elige un tipo más grande, como `u32` o `i32`.

### Una constante sin tipo

```rust norun
const MAX = 10;

fn main() {
    println!("{MAX}");
}
```

```text
error: missing type for `const` item
 --> src/main.rs:1:10
  |
1 | const MAX = 10;
  |          ^ help: provide a type for the constant: `: i32`
```

Corrección: `const MAX: i32 = 10;`.

## Ponte a prueba

**1.** ¿Compila este código? Si no, ¿cómo corregirlo?

```rust norun
let puntos = 0;
puntos += 5;
```

> **Respuesta**
>
> No: `puntos` no es modificable. Escribe `let mut puntos = 0;`.

**2.** ¿Cuánto valen `9 / 4` y `9.0 / 4.0`?

> **Respuesta**
>
> `9 / 4` vale `2` (división entera) y `9.0 / 4.0` vale `2.25`.

**3.** ¿Cómo mostrar la media de `total: i32 = 7` entre `2` alumnos, con decimales?

> **Respuesta**
>
> Convierte con `as` antes de dividir:
>
> ```rust
> let total: i32 = 7;
> println!("{}", total as f64 / 2.0);
> ```
>
> ```sortie
> 3.5
> ```

## En resumen

- `let` crea una variable no modificable; `let mut` la hace modificable.
- Los tipos simples: `i32`, `u32`, `f64`, `bool`, `char`; `as` convierte un número.
- Cada entero tiene límites (`u8`: de 0 a 255); superarlos es un error.
- Una tupla `(a, b)` agrupa unos pocos valores, que se leen con `.0`, `.1`.
- `println!("{x}")` muestra, `format!` fabrica un `String`.
- `const` crea una constante, en MAYÚSCULAS, con su tipo.

El mismo capítulo existe para [Ruby](../ruby/premiers-pas.md) y
[Rythme](../rythme/premiers-pas.md).

Siguiente: [Números y cálculos](nombres.md)
