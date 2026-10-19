# Números y cálculos

Rust ofrece varios tipos de números, enteros y decimales: este capítulo muestra cómo elegirlos, calcular con ellos, convertirlos y mostrarlos bien.

> **Objetivos:**
>
> - elegir el tipo adecuado de entero o de número decimal;
> - calcular con los operadores, la división entera y el resto `%`;
> - evitar los desbordamientos con `checked_`, `wrapping_` y `saturating_`;
> - convertir un número con `as`, `From`, `TryFrom` y `parse`;
> - mostrar un número con decimales, espacios o ceros.

## Los enteros

Un programa cuenta sin parar: una edad, una puntuación, los artículos de una cesta.
Rust quiere saber de antemano cuánto sitio reservar para cada número, igual que eliges
una caja del tamaño adecuado para guardar un objeto.

Un [entero](../debuter/glossaire.md#entero) es un número sin decimales: `3`, `-12`,
`2026`. Rust tiene varios tipos de enteros. Su nombre dice dos cosas:

- la letra: `i` para un entero **con signo** (puede ser negativo), `u` para un entero
  **sin signo** (*unsigned*: nunca negativo);
- el número: el tamaño en bits. Cuantos más bits, más grande puede ser el número.

| Tipo | Valor más pequeño | Valor más grande |
|------|-------------------|------------------|
| `i8` | -128 | 127 |
| `u8` | 0 | 255 |
| `i16` | -32 768 | 32 767 |
| `u16` | 0 | 65 535 |
| `i32` | -2 147 483 648 | 2 147 483 647 |
| `u32` | 0 | 4 294 967 295 |
| `i64` | unos -9,2 × 10¹⁸ | unos 9,2 × 10¹⁸ |
| `u64` | 0 | unos 1,8 × 10¹⁹ |
| `i128`, `u128` | aún más grandes | (unas 38 cifras) |

Cuando no escribes un tipo, Rust elige `i32`. Es una buena elección por defecto.

```rust
let anio = 2026;           // i32, elegido por Rust
let edad: u8 = 12;         // una edad nunca es negativa ni muy grande
let poblacion: u64 = 8_100_000_000;
println!("{anio} {edad} {poblacion}");
```

```sortie
2026 12 8100000000
```

Línea por línea:

- `let anio = 2026;` crea la variable `anio`, que vale 2026. No se escribe ningún tipo:
  Rust elige `i32`;
- `let edad: u8 = 12;`: el `: u8` después del nombre fija el tipo. 12 cabe sin problema
  entre 0 y 255;
- `poblacion` es un `u64`: 8 mil millones no cabrían en un `i32`, que se detiene en
  unos 2100 millones;
- `println!` sustituye cada `{nombre}` por el valor de la variable y lo muestra todo.

> **Consejo:** el guion bajo `_` separa las cifras para leerlas mejor: `8_100_000_000`
> es el mismo número que `8100000000`.

### usize: el tipo de las posiciones

`usize` es un entero sin signo cuyo tamaño depende del ordenador (64 bits en un
ordenador reciente). Es el tipo de las **posiciones** y de las **longitudes**: `len()`
devuelve un `usize`, y un índice de vector debe ser un `usize`.

```rust
let frutas = vec!["manzana", "kiwi", "pera"];
let n: usize = frutas.len();
let i: usize = 1;
println!("{n} frutas, la segunda es {}", frutas[i]);
```

```sortie
3 frutas, la segunda es kiwi
```

## Los números decimales

Los enteros no bastan para un precio de 19,99 €, una altura de 1,75 m o una media de
13,5. Para estos valores con una parte después de la coma, hace falta otro tipo.

Un [flotante](../debuter/glossaire.md#flotante) es un número con decimales. En Rust, se
escribe un **punto**: `3.14`. Hay dos tipos:

- `f64`: preciso (unas 15 cifras), elegido por defecto;
- `f32`: menos preciso (unas 7 cifras), pero ocupa la mitad de memoria.

```rust
let precio = 19.99;        // f64
let altura: f32 = 1.75;
println!("{precio} {altura}");
```

```sortie
19.99 1.75
```

### Los flotantes no son exactos

El ordenador guarda los números en binario. Muchos números decimales, como `0.1`, no
tienen una escritura binaria exacta: es como `1/3 = 0,333…` en decimal. Queda un error
muy pequeño:

```rust
let suma: f64 = 0.1 + 0.2;
println!("{suma}");
println!("{}", suma == 0.3);
println!("{}", (suma - 0.3).abs() < 1e-9);
```

```sortie
0.30000000000000004
false
true
```

> **Recuerda:** nunca compares dos flotantes con `==`. Comprueba más bien que su
> diferencia es muy pequeña. Para el dinero, cuenta en céntimos con un entero.

**Te toca:** escribe un programa que guarde un número de días, `7`, y una
temperatura, `21.5`, en dos variables, y luego muestre «7 días a 21.5 grados».

> **Solución**
>
> ```rust
> let dias = 7;
> let temperatura = 21.5;
> println!("{dias} días a {temperatura} grados");
> ```
>
> ```sortie
> 7 días a 21.5 grados
> ```

## Los operadores

Calcular es el primer trabajo de un ordenador: el total de una cesta, una media, el
tiempo que queda para el final de una película. Los operadores son los signos que
escriben estos cálculos.

Los [operadores](../debuter/glossaire.md#operador) habituales funcionan como en
matemáticas:

| Operador | Significado | Ejemplo | Resultado |
|----------|-------------|---------|-----------|
| `+` | suma | `7 + 2` | `9` |
| `-` | resta | `7 - 2` | `5` |
| `*` | multiplicación | `7 * 2` | `14` |
| `/` | división | `7.0 / 2.0` | `3.5` |
| `%` | resto de la división | `7 % 2` | `1` |

Las formas cortas `+=`, `-=`, `*=`, `/=` modifican una variable `mut`:

```rust
let mut puntos = 10;
puntos += 5;
puntos *= 2;
println!("{puntos}");
```

```sortie
30
```

Línea por línea:

- `let mut puntos = 10;` crea `puntos`, que vale 10. `mut` le da derecho a cambiar;
- `puntos += 5;` es una forma corta de `puntos = puntos + 5`: `puntos` vale ahora 15;
- `puntos *= 2;` multiplica por 2: 15 × 2 da 30;
- `println!` muestra el valor actual de `puntos`: 30.

### La división entera y el resto

Entre dos enteros, `/` da una **división entera**: la parte decimal se descarta. `%`
da el **resto**. Piensa en 7 caramelos repartidos entre 2 niños: cada uno recibe 3, y
sobra 1.

```rust
println!("{}", 7 / 2);
println!("{}", 7 % 2);
println!("{}", 7.0 / 2.0);
let minutos = 135;
println!("{} h {} min", minutos / 60, minutos % 60);
```

```sortie
3
1
3.5
2 h 15 min
```

> **Consejo:** `n % 2 == 0` es la forma clásica de comprobar que un número es par.

**Te toca:** escribe un programa que guarde `200` segundos en una variable, y luego
muestre esa duración en minutos y segundos: «3 min 20 s».

> **Solución**
>
> ```rust
> let segundos = 200;
> println!("{} min {} s", segundos / 60, segundos % 60);
> ```
>
> ```sortie
> 3 min 20 s
> ```

## El desbordamiento

Un tipo de número se parece a una caja de tamaño fijo: si metes demasiado, se
desborda. Esta sección muestra cómo evitar que un cálculo dé un resultado falso o
detenga tu programa.

Un `u8` no pasa de 255. ¿Qué ocurre si un cálculo va más allá? Esto se llama un
**desbordamiento** (en inglés: *overflow*).

- En modo depuración (`cargo run`), el programa **se detiene** con un
  [pánico](../debuter/glossaire.md#panico): `attempt to add with overflow`.
- En modo optimizado (`cargo run --release`), el número **da la vuelta**: 255 + 1
  da 0, como un cuentakilómetros.

Para no depender del modo, di tú mismo lo que quieres con tres familias de métodos:

| Método | Si se desborda… | `250u8 + 10` |
|--------|-----------------|--------------|
| `checked_add` | devuelve `None` | `None` |
| `wrapping_add` | da la vuelta | `4` |
| `saturating_add` | se queda en el máximo | `255` |

```rust
let x: u8 = 250;
println!("{:?}", x.checked_add(10));
println!("{:?}", x.checked_add(3));
println!("{}", x.wrapping_add(10));
println!("{}", x.saturating_add(10));
let vidas: u8 = 1;
println!("{}", vidas.saturating_sub(5));
```

```sortie
None
Some(253)
4
255
0
```

Línea por línea:

- `x` vale 250, y un `u8` no pasa de 255;
- `checked_add(10)`: 260 no cabe, el método devuelve `None` («nada»).
  `checked_add(3)`: 253 cabe, devuelve `Some(253)`. `{:?}` sirve para mostrar estos
  valores;
- `wrapping_add(10)` da la vuelta: después de 255 vuelve a empezar en 0, y 260 - 256
  da 4;
- `saturating_add(10)` se queda bloqueado en el máximo, 255;
- `vidas` vale 1. Quitar 5 daría -4, imposible para un `u8`: `saturating_sub` se
  detiene en 0.

Los mismos métodos existen para `sub`, `mul` y `div`: `checked_mul`,
`saturating_sub`… `checked_` es el más seguro: devuelve un
[Option](types.md) que debes tratar.

**Te toca:** escribe un programa que guarde `3` en una variable `vidas` de tipo `u8`.
Muestra lo que queda después de perder 5 vidas, sin bajar de cero, y luego el resultado
de `vidas.checked_sub(5)`.

> **Solución**
>
> ```rust
> let vidas: u8 = 3;
> println!("{}", vidas.saturating_sub(5));
> println!("{:?}", vidas.checked_sub(5));
> ```
>
> ```sortie
> 0
> None
> ```

## Convertir un número

Rust **nunca** convierte un número por sí solo. Para sumar un `i32` y un `f64`, hay que
convertir uno de los dos.

### Con as: rápido, pero sin red

`as` siempre convierte, aunque pierda información: la parte decimal se corta, y un
número demasiado grande se deforma.

```rust
let n: i32 = 7;
let x = n as f64 / 2.0;
println!("{x}");
println!("{}", 3.99_f64 as i32);
println!("{}", 300_i32 as u8);
println!("{}", 'A' as u32);
```

```sortie
3.5
3
44
65
```

Línea por línea:

- `n as f64` convierte 7 en 7.0: la división se hace entre dos flotantes y da 3.5;
- `3.99_f64` es el número 3.99 de tipo `f64` (el sufijo pega el tipo al número).
  `as i32` corta la parte decimal, sin redondear: 3;
- `300_i32 as u8`: 300 no cabe en un `u8`, y `as` lo deforma en 44;
- `'A' as u32` da el número de la letra A en la tabla de caracteres: 65.

`300 as u8` da `44` (300 - 256) sin avisar: usa `as` solo cuando estés seguro de que
el número cabe.

### Con From y TryFrom: sin sorpresas

`From` convierte cuando es **siempre posible** (de un tipo pequeño a uno más grande).
`TryFrom` convierte cuando es **quizás posible**: devuelve un
[Result](../debuter/glossaire.md#result).

```rust
let pequeno: u8 = 200;
let grande = i64::from(pequeno);
println!("{grande}");

let ok = u8::try_from(100_i32);
let demasiado = u8::try_from(300_i32);
println!("{:?}", ok);
println!("{:?}", demasiado.is_err());
```

```sortie
200
Ok(100)
true
```

### Desde un texto con parse

`parse` transforma un texto en número. Hay que decir qué tipo se quiere, y tratar el
posible fallo (ver [Convertir un texto en número](texte.md#convertir-un-texto-en-numero)):

```rust
let edad: u32 = "42".parse().unwrap();
let altura = "1.80".parse::<f64>().unwrap();
println!("{edad} {altura}");
match "doce".parse::<i32>() {
    Ok(n) => println!("número: {n}"),
    Err(e) => println!("error: {e}"),
}
```

```sortie
42 1.8
error: invalid digit found in string
```

## Los métodos matemáticos

Redondear un precio, calcular una potencia, mantener una nota entre 0 y 10: estos
cálculos aparecen a menudo. Rust te los da ya hechos, no hace falta escribirlos tú.

Los números tienen métodos. Se llaman con un punto:

```rust
println!("{}", 2_i32.pow(10));
println!("{}", 16.0_f64.sqrt());
println!("{}", (-5_i32).abs());
println!("{}", 3.max(8));
println!("{}", 3.min(8));
println!("{}", 15.clamp(0, 10));
println!("{}", 2.6_f64.round());
println!("{}", 2.6_f64.floor());
println!("{}", 2.1_f64.ceil());
```

```sortie
1024
4
5
8
3
10
3
2
3
```

- `pow`: potencia (enteros); para los flotantes, `powi` o `powf`.
- `sqrt`: raíz cuadrada (solo flotantes).
- `clamp(min, max)`: mantiene el número entre dos límites, como un volumen entre 0 y 10.
- `round`, `floor`, `ceil`: redondeo al más cercano, hacia abajo, hacia arriba.

> **Atención:** escribe `(-5_i32).abs()` con paréntesis. Sin ellos,
> `-5_i32.abs()` se lee `-(5.abs())` y da `-5`.

## Las constantes útiles

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

En lugar de memorizar 2147483647 o 3,14159…, puedes pedírselos a Rust. El nombre es más
claro, y no hay riesgo de equivocarse al teclear.

Cada tipo conoce sus límites, y `std::f64::consts` contiene las constantes
matemáticas:

```rust
use std::f64::consts::PI;

println!("{}", i32::MAX);
println!("{}", u8::MAX);
println!("{}", i64::MIN);
let radio = 2.0;
println!("{}", PI * radio * radio);
```

```sortie
2147483647
255
-9223372036854775808
12.566370614359172
```

Línea por línea:

- `use std::f64::consts::PI;` hace que la constante `PI` esté disponible con este
  nombre corto;
- `i32::MAX`, `u8::MAX` e `i64::MIN` dan el valor más grande o más pequeño de cada
  tipo;
- la última línea calcula el área de un círculo de radio 2: π × 2 × 2.

## Mostrar un número bonito

Un resultado como `12.666666666666666` es difícil de leer. En un tique de compra,
quieres `12.67`, y precios bien alineados en columna. Los formatos sirven para eso.

Entre las llaves de `println!`, después de dos puntos `:`, puedes ajustar cómo se
muestra:

| Formato | Efecto | Ejemplo | Resultado |
|---------|--------|---------|-----------|
| `{:.2}` | 2 cifras después del punto decimal | `3.14159` | `3.14` |
| `{:>8}` | alineado a la derecha en 8 caracteres | `42` | `      42` |
| `{:<8}` | alineado a la izquierda en 8 caracteres | `42` | `42      ` |
| `{:08.3}` | 8 caracteres, ceros delante, 3 decimales | `3.14159` | `0003.142` |
| `{:e}` | notación científica | `1500.0` | `1.5e3` |

```rust
let pi = 3.14159;
println!("[{:.2}]", pi);
println!("[{:>8}]", 42);
println!("[{:<8}]", 42);
println!("[{:08.3}]", pi);
println!("[{:e}]", 1500.0);
println!("[{pi:>10.1}]");
```

```sortie
[3.14]
[      42]
[42      ]
[0003.142]
[1.5e3]
[       3.1]
```

Línea por línea:

- `{:.2}` deja 2 cifras después del punto decimal, redondeando: 3.14;
- `{:>8}` reserva 8 casillas y empuja 42 a la derecha; `{:<8}` lo empuja a la
  izquierda. Los corchetes `[ ]` están ahí para ver los espacios;
- `{:08.3}`: 3 decimales, 8 caracteres en total, completados con ceros delante;
- en la última línea, el nombre de la variable va antes de los dos puntos:
  `{pi:>10.1}`.

Este ajuste es práctico para alinear una lista de precios:

```rust
let articulos = [("pan", 1.2), ("queso", 4.75), ("manzanas", 12.0)];
for (nombre, precio) in articulos {
    println!("{nombre:<10}{precio:>7.2} €");
}
```

```sortie
pan          1.20 €
queso        4.75 €
manzanas    12.00 €
```

**Te toca:** escribe un programa que guarde un total de puntos, `38`, y un número de
notas, `3`, en dos enteros. Muestra la media con dos decimales: «Media: 12.67».

> **Solución**
>
> ```rust
> let total = 38;
> let cantidad = 3;
> let media = total as f64 / cantidad as f64;
> println!("Media: {media:.2}");
> ```
>
> ```sortie
> Media: 12.67
> ```

## Errores frecuentes

### Mezclar un entero y un flotante

```rust norun
fn main() {
    let cantidad: i32 = 3;
    let precio = 2.5;
    let total = cantidad * precio;
    println!("{total}");
}
```

```text
error[E0277]: cannot multiply `i32` by `{float}`
 --> src/main.rs:4:26
  |
4 |     let total = cantidad * precio;
  |                          ^ no implementation for `i32 * {float}`
  |
  = help: the trait `Mul<{float}>` is not implemented for `i32`
```

Rust no convierte por sí solo. Escribe `cantidad as f64 * precio`.

### Un número demasiado grande para su tipo

```rust norun
fn main() {
    let edad: u8 = 300;
    println!("{edad}");
}
```

```text
error: literal out of range for `u8`
 --> src/main.rs:2:20
  |
2 |     let edad: u8 = 300;
  |                    ^^^
  |
  = note: the literal `300` does not fit into the type `u8` whose range is `0..=255`
  = note: `#[deny(overflowing_literals)]` on by default
```

Un `u8` se detiene en 255. Elige un tipo más grande: `u16` o `i32`.

### Un cálculo que se desborda

```rust norun
fn main() {
    let x: u8 = 255;
    let y = x + 1;
    println!("{y}");
}
```

```text
error: this arithmetic operation will overflow
 --> src/main.rs:3:13
  |
3 |     let y = x + 1;
  |             ^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
  |
  = note: `#[deny(arithmetic_overflow)]` on by default
```

Aquí, el compilador ve el problema por adelantado. Elige un tipo más grande, o di lo
que quieres: `x.checked_add(1)` o `x.saturating_add(1)`.

### parse sin tipo

```rust norun
fn main() {
    let n = "42".parse().unwrap();
    println!("{n}");
}
```

```text
error[E0284]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let n = "42".parse().unwrap();
  |         ^        ----- type must be known at this point
  |
  = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `n` an explicit type
  |
2 |     let n: /* Type */ = "42".parse().unwrap();
  |          ++++++++++++
```

`parse` puede producir un `i32`, un `u8`, un `f64`… Di cuál: `let n: i32 = …` o
`"42".parse::<i32>()`.

## Ponte a prueba

**1.** ¿Cuánto valen `17 / 5` y `17 % 5`?

> **Respuesta**
>
> `3` y `2`: 17 = 5 × 3 + 2. Entre dos enteros, `/` descarta la parte decimal.

**2.** ¿Qué tipo eliges para el índice de un vector? ¿Y para una temperatura
con decimales?

> **Respuesta**
>
> `usize` para un índice; `f64` para una temperatura.

**3.** ¿Cómo muestras `2.0 / 3.0` con exactamente dos decimales?

> **Respuesta**
>
> Con el formato `{:.2}`:
>
> ```rust
> println!("{:.2}", 2.0 / 3.0);
> ```
>
> ```sortie
> 0.67
> ```

## En resumen

- `i32` y `f64` son los tipos por defecto; `usize` sirve para posiciones y longitudes.
- Entre enteros, `/` es una división entera y `%` da el resto.
- Los flotantes no son exactos: no los compares con `==`.
- `checked_`, `wrapping_` y `saturating_` dicen qué hacer en caso de desbordamiento.
- `as` convierte sin red; `From`, `TryFrom` y `parse` son más seguros.
- `{:.2}`, `{:>8}` y `{:08.3}` ajustan cómo se muestra un número.

Siguiente: [Condiciones y bucles](conditions-boucles.md)
