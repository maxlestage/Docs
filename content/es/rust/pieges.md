# Las trampas del principiante en Rust

Esta página reúne doce trampas en las que caen casi todos los principiantes en Rust, con, para cada una, lo que pasa, por qué, y cómo salir de ella.

> **Objetivos:**
>
> - reconocer los errores más comunes de las primeras semanas;
> - leer el mensaje del compilador sin entrar en pánico;
> - saber qué pequeña corrección aplicar cada vez.

## Antes de empezar

Rust tiene un [compilador](../debuter/glossaire.md#compilador) muy estricto. Rechaza
muchos programas. Al principio es frustrante. Pero es un amigo: te detiene **antes** de
que el programa haga una tontería.

Cada trampa sigue el mismo camino:

```text
lo que esperas  →  lo que pasa  →  por qué  →  la corrección
```

> **Recuerda:** un mensaje de error de Rust no es un castigo. Es una explicación, a
> menudo con la corrección abajo, después de `help:`.

## Variables y texto

### Trampa 1: las variables no cambian

**Lo que esperas**

Creas una puntuación a 0 y luego le sumas 10. Esperas ver `10`.

```rust norun
fn main() {
    let puntos = 0;
    puntos = puntos + 10;
    println!("{puntos}");
}
```

**Lo que pasa**

```text
error[E0384]: cannot assign twice to immutable variable `puntos`
 --> src/main.rs:3:5
  |
2 |     let puntos = 0;
  |         ------ first assignment to `puntos`
3 |     puntos = puntos + 10;
  |     ^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut puntos = 0;
  |         +++
```

**Por qué**

En Rust, una variable es [inmutable](../debuter/glossaire.md#inmutable) por defecto. Ya
no cambia una vez creada. Es una etiqueta pegada en una caja cerrada.

**La corrección**

Añade `mut` (de *mutable*, «modificable»):

```rust
fn main() {
    let mut puntos = 0;
    puntos = puntos + 10;
    println!("{puntos}");
}
```

```sortie
10
```

Prueba: quita `mut`. El propio compilador te propone volver a ponerlo, con `+++` debajo
del sitio que hay que completar.

### Trampa 2: String y &str, dos tipos de texto

**Lo que esperas**

Escribes un texto entre comillas en una variable de tipo `String`.

```rust norun
fn main() {
    let nombre: String = "Zoé";
    println!("{nombre}");
}
```

**Lo que pasa**

```text
error[E0308]: mismatched types
 --> src/main.rs:2:26
  |
2 |     let nombre: String = "Zoé";
  |                 ------   ^^^^^ expected `String`, found `&str`
  |                 |
  |                 expected due to this
  |
help: try using a conversion method
  |
2 |     let nombre: String = "Zoé".to_string();
  |                               ++++++++++++
```

**Por qué**

Un texto entre comillas es un `&str`: un texto **escrito en el programa**, que solo se
lee. Una `String` es un texto **tuyo**, que puede crecer. Es la diferencia entre un
cartel pegado en la pared y una hoja de tu cuaderno.

| Tipo | Imagen | ¿Puede crecer? |
|------|--------|----------------|
| `&str` | un cartel que se mira | no |
| `String` | una hoja tuya | sí |

**La corrección**

Convierte el cartel en hoja con `String::from` o `.to_string()`:

```rust
fn main() {
    let nombre: String = String::from("Zoé");
    let ciudad = "Lyon".to_string();
    println!("{nombre} vive en {ciudad}");
}
```

```sortie
Zoé vive en Lyon
```

Todo se explica en [Trabajar con texto](texte.md).

### Trampa 3: usar un valor después de haberlo dado

**Lo que esperas**

Copias un texto en otra variable y luego muestras los dos.

```rust norun
fn main() {
    let a = String::from("hola");
    let b = a;
    println!("{a} {b}");
}
```

**Lo que pasa**

```text
error[E0382]: borrow of moved value: `a`
 --> src/main.rs:4:16
  |
2 |     let a = String::from("hola");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
3 |     let b = a;
  |             - value moved here
4 |     println!("{a} {b}");
  |                ^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let b = a.clone();
  |              ++++++++
```

**Por qué**

`let b = a;` no copia la `String`. Se la **da** a `b`. Es como regalar un libro: ya no
lo tienes. Se dice que el valor se ha «movido» (*moved*).

```text
antes:   a ──► "hola"
después: a     (vacía)
         b ──► "hola"
```

Es la regla de la [propiedad](../debuter/glossaire.md#propiedad). Que no cunda el
pánico: tiene su propio capítulo, [Propiedad y préstamo](possession.md). Por ahora,
recuerda solo la corrección.

**La corrección**

Si de verdad quieres dos textos, haz una copia con `.clone()`:

```rust
fn main() {
    let a = String::from("hola");
    let b = a.clone();
    println!("{a} {b}");
}
```

```sortie
hola hola
```

> **Nota:** los números (`i32`, `f64`…) y `bool` no se ven afectados. Son pequeños, y
> Rust los copia solo. `let y = x;` deja `x` utilizable.

## Números y funciones

### Trampa 4: un número que se desborda

**Lo que esperas**

Un `u8` va de 0 a 255. Empiezas en 250 y sumas 1, diez veces. Esperas ver `260`.

```rust norun
fn main() {
    let mut nivel: u8 = 250;
    for _ in 0..10 {
        nivel += 1;
    }
    println!("{nivel}");
}
```

**Lo que pasa**

El programa compila y luego se para en seco durante la ejecución:

```text
thread 'main' (10517) panicked at src/main.rs:4:9:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

**Por qué**

Una variable `u8` es una caja que no puede contener más de 255. Al llegar a 256, el
número «se desborda» (*overflow*). En modo desarrollo, Rust prefiere pararse: a esto se
le llama [pánico](../debuter/glossaire.md#panico).

| Vuelta | nivel antes | nivel después |
|--------|-------------|---------------|
| 1 | 250 | 251 |
| … | … | … |
| 5 | 254 | 255 |
| 6 | 255 | desbordamiento: pánico |

**La corrección**

Elige un tipo lo bastante grande. `i32` llega a más de dos mil millones:

```rust
fn main() {
    let mut nivel: i32 = 250;
    for _ in 0..10 {
        nivel += 1;
    }
    println!("{nivel}");
}
```

```sortie
260
```

Para más detalles, mira [El desbordamiento](nombres.md#el-desbordamiento).

### Trampa 5: 7 dividido entre 2 da 3

**Lo que esperas**

Repartes 7 caramelos entre 2 niños. Esperas ver `3.5`.

```rust
fn main() {
    let caramelos = 7;
    let ninos = 2;
    println!("{}", caramelos / ninos);
    println!("{}", caramelos % ninos);
}
```

```sortie
3
1
```

**Por qué**

`7` y `2` son [enteros](../debuter/glossaire.md#entero). Entre dos enteros, `/` hace una
división **entera**: se queda con la parte antes de la coma. `%` da el resto. Es
exactamente el reparto de caramelos: 3 para cada uno, y sobra 1.

**La corrección**

Para un resultado con decimales, trabaja con `f64`, escribiendo `7.0` y `2.0`:

```rust
fn main() {
    let caramelos = 7.0;
    let ninos = 2.0;
    println!("{}", caramelos / ninos);
}
```

```sortie
3.5
```

Prueba: escribe `7.0 / 2`. Rust se niega a mezclar un `f64` y un entero. Escribe
`7.0 / 2.0`.

### Trampa 6: el punto y coma de más

**Lo que esperas**

Tu función calcula el doble de un número. Esperas ver `8`.

```rust norun
fn doble(x: i32) -> i32 {
    x * 2;
}

fn main() {
    println!("{}", doble(4));
}
```

**Lo que pasa**

```text
error[E0308]: mismatched types
 --> src/main.rs:1:21
  |
1 | fn doble(x: i32) -> i32 {
  |    -----            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     x * 2;
  |          - help: remove this semicolon to return this value
```

**Por qué**

En Rust, la última línea **sin** punto y coma es el valor devuelto. Con un punto y
coma, la línea se convierte en una simple instrucción: calcula y luego tira el
resultado. La función devuelve entonces «nada», que se escribe `()`.

```text
x * 2     →  un valor: 8            (devuelto)
x * 2;    →  una instrucción        (el 8 se tira, queda ())
```

**La corrección**

Quita el punto y coma de la última línea:

```rust
fn doble(x: i32) -> i32 {
    x * 2
}

fn main() {
    println!("{}", doble(4));
}
```

```sortie
8
```

Mira también [Expresiones e instrucciones](fonctions.md#expresiones-e-instrucciones).

## Texto y tipos

### Trampa 7: leer una letra con [0]

**Lo que esperas**

Quieres la primera letra de «gato». Esperas ver `g`.

```rust norun
fn main() {
    let palabra = String::from("gato");
    let primera = palabra[0];
    println!("{primera}");
}
```

**Lo que pasa**

```text
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> src/main.rs:3:27
  |
3 |     let primera = palabra[0];
  |                           ^ string indices are ranges of `usize`
  |
  = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
  = note: you can use `.chars().nth()` or `.bytes().nth()`
```

**Por qué**

Rust guarda el texto en [Unicode](../debuter/glossaire.md#unicode). Una letra como `ñ`
ocupa 2 casillas en memoria, `a` solo ocupa una. Así que la casilla 0 no siempre es una
letra entera. Rust prefiere negarse antes que darte media letra.

**La corrección**

Recorre las letras con `.chars()` y luego toma la que te interesa:

```rust
fn main() {
    let palabra = String::from("ñandú");
    let primera = palabra.chars().next();
    let tercera = palabra.chars().nth(2);
    println!("{:?} {:?}", primera, tercera);
}
```

```sortie
Some('ñ') Some('n')
```

El resultado es una [Option](../debuter/glossaire.md#option): la palabra podría ser
demasiado corta. Mira [Recorrer los caracteres](texte.md#recorrer-los-caracteres).

### Trampa 8: comparar dos tipos de enteros

**Lo que esperas**

Comparas la longitud de una lista con un número. Esperas ver `grande`.

```rust norun
fn main() {
    let v = vec![10, 20, 30];
    let limite: i32 = 2;
    if v.len() > limite {
        println!("grande");
    }
}
```

**Lo que pasa**

```text
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     if v.len() > limite {
  |        -------   ^^^^^^ expected `usize`, found `i32`
  |        |
  |        expected because this is `usize`
```

**Por qué**

`len()` devuelve un `usize`: el tipo de los tamaños y las posiciones. `limite` es un
`i32`. Para Rust, son dos [tipos](../debuter/glossaire.md#tipo) distintos, aunque los
dos contengan números enteros. Nunca los mezcla por su cuenta.

**La corrección**

Lo más sencillo: da a `limite` el tipo correcto desde el principio.

```rust
fn main() {
    let v = vec![10, 20, 30];
    let limite: usize = 2;
    if v.len() > limite {
        println!("grande");
    }
}
```

```sortie
grande
```

> **Consejo:** una variable que sirve de posición o de tamaño en una lista debe ser un
> `usize`.

## Bucles, listas y Option

### Trampa 9: unwrap sobre una caja vacía

**Lo que esperas**

Pides el primer elemento de una lista y lo abres con `unwrap()`.

```rust norun
fn main() {
    let v: Vec<i32> = Vec::new();
    let primero = v.first().unwrap();
    println!("{primero}");
}
```

**Lo que pasa**

```text
thread 'main' (10595) panicked at src/main.rs:3:29:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

**Por qué**

`first()` devuelve una `Option`: una caja que contiene o bien `Some(valor)`, o bien
`None` (nada). La lista está vacía, así que la caja también. `unwrap()` quiere decir
«abre la caja, y si no hay nada, páralo todo».

**La corrección**

Prevé el caso vacío, con `match` o con `unwrap_or`:

```rust
fn main() {
    let v: Vec<i32> = Vec::new();
    match v.first() {
        Some(x) => println!("primero: {x}"),
        None => println!("la lista está vacía"),
    }
    let primero = v.first().copied().unwrap_or(0);
    println!("{primero}");
}
```

```sortie
la lista está vacía
0
```

> **Recuerda:** `unwrap()` es práctico para probar. En un programa de verdad, cada
> `unwrap()` es una pregunta: «¿estoy **seguro** de que la caja no está vacía?»

### Trampa 10: ocultar en vez de modificar

**Lo que esperas**

Sumas 1, 2 y luego 3 al total en un bucle. Esperas ver `6` al final.

```rust
fn main() {
    let total = 0;
    for n in 1..=3 {
        let total = total + n;
        println!("en el bucle: {total}");
    }
    println!("al final: {total}");
}
```

```sortie
en el bucle: 1
en el bucle: 2
en el bucle: 3
al final: 0
```

**Por qué**

`let total = …` dentro del bucle crea una variable **nueva**, que lleva el mismo
nombre. A esto se le llama **ocultación** (*shadowing*). Tapa a la antigua y luego
desaparece al final de la vuelta. El `total` antiguo nunca se movió.

| Vuelta | n | total del bucle | total de fuera |
|--------|---|-----------------|----------------|
| 1 | 1 | 0 + 1 = 1 | 0 |
| 2 | 2 | 0 + 2 = 2 | 0 |
| 3 | 3 | 0 + 3 = 3 | 0 |

**La corrección**

Para modificar la misma variable: `let mut` una sola vez y luego **ningún** `let`
dentro del bucle.

```rust
fn main() {
    let mut total = 0;
    for n in 1..=3 {
        total = total + n;
    }
    println!("al final: {total}");
}
```

```sortie
al final: 6
```

> **Recuerda:** `let` crea una variable nueva. `x = …` (sin `let`) modifica la que ya
> existe.

### Trampa 11: una casilla que no existe

**Lo que esperas**

Una lista tiene 3 notas. Quieres la tercera, así que escribes `notas[3]`.

```rust norun
fn main() {
    let notas = vec![12, 15, 9];
    println!("{}", notas[3]);
}
```

**Lo que pasa**

```text
thread 'main' (10625) panicked at src/main.rs:3:25:
index out of bounds: the len is 3 but the index is 3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

**Por qué**

Las posiciones (los [índices](../debuter/glossaire.md#indice)) empiezan en **0**. Tres
casillas tienen los números 0, 1 y 2. La casilla 3 no existe.

```text
posición:   0    1    2
notas   : [ 12 | 15 |  9 ]
```

**La corrección**

La tercera nota es `notas[2]`. Si no estás seguro, usa `get`, que devuelve una `Option`
en vez de pararse:

```rust
fn main() {
    let notas = vec![12, 15, 9];
    println!("{}", notas[2]);
    println!("{:?}", notas.get(3));
}
```

```sortie
9
None
```

### Trampa 12: el rango que se para demasiado pronto

**Lo que esperas**

Cuentas de 1 a 5 con `1..5`. Esperas ver 5 números.

```rust
fn main() {
    for i in 1..5 {
        print!("{i} ");
    }
    println!();
}
```

```sortie
1 2 3 4 
```

**Por qué**

Un [rango](../debuter/glossaire.md#rango) `1..5` se para **antes** del 5. Es
práctico para las posiciones: `0..v.len()` pasa por todas las casillas, sin salirse.

**La corrección**

Para incluir el final, añade `=`: `1..=5`.

```rust
fn main() {
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();
}
```

```sortie
1 2 3 4 5 
```

| Escritura | Números obtenidos |
|-----------|-------------------|
| `1..5` | 1, 2, 3, 4 |
| `1..=5` | 1, 2, 3, 4, 5 |

## Errores frecuentes

Toda esta página habla de errores. Aquí van dos más, muy cortos.

### Mostrar una lista con {}

```rust norun
fn main() {
    let v = vec![1, 2, 3];
    println!("{}", v);
}
```

```text
error[E0277]: `Vec<{integer}>` doesn't implement `std::fmt::Display`
 --> src/main.rs:3:20
  |
3 |     println!("{}", v);
  |               --   ^ `Vec<{integer}>` cannot be formatted with the default formatter
  |               |
  |               required by this formatting parameter
  |
  = help: the trait `std::fmt::Display` is not implemented for `Vec<{integer}>`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

`{}` está hecho para las personas, y Rust no sabe cómo presentar una lista. Usa `{:?}`,
la visualización «para desarrolladores»: `println!("{:?}", v);` muestra `[1, 2, 3]`.

### parse sin decir el tipo que quieres

```rust norun
fn main() {
    let n = "42".parse().unwrap();
    println!("{}", n + 1);
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

`parse` puede fabricar un entero, un número con decimales, un booleano… Di cuál:
`let n: i32 = "42".parse().unwrap();`.

## Ponte a prueba

**1.** ¿Qué muestra este programa: `let x = 10; let y = 4; println!("{}", x / y);`?

> **Respuesta**
>
> `2`. Son dos enteros: la división se queda solo con la parte entera. Para obtener
> `2.5`, escribe `10.0 / 4.0`.

**2.** ¿Por qué no compila esta función?
`fn triple(x: i32) -> i32 { x * 3; }`

> **Respuesta**
>
> Por el punto y coma después de `x * 3`. La línea ya no devuelve nada (`()`), mientras
> que la función promete un `i32`. Escribe `x * 3` sin punto y coma.

**3.** Después de `let a = String::from("ok"); let b = a;`, ¿se puede seguir mostrando
`a`? ¿Cómo conservar los dos?

> **Respuesta**
>
> No: la `String` se ha dado (movido) a `b`. Para conservar los dos, escribe
> `let b = a.clone();`.

## En resumen

- Una variable es inmutable por defecto: añade `mut` para modificarla.
- `"texto"` es un `&str`; `String::from("texto")` da una `String`.
- `let b = a;` da una `String` a `b`: usa `.clone()` para conservar los dos.
- Entre dos enteros, `/` corta los decimales; un desbordamiento para el programa.
- Nada de punto y coma después del valor devuelto; nada de `[0]` sobre un texto.
- Las posiciones empiezan en 0, `1..5` excluye el 5, y `unwrap()` sobre `None` lo para
  todo.

Siguiente: [Ejercicios para empezar bien](exercices-faciles.md)
