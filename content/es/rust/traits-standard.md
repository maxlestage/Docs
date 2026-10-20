# Los traits de la biblioteca estándar

La biblioteca estándar de Rust define un puñado de traits que encontrarás en todas partes: este capítulo muestra para qué sirve cada uno, y cuándo obtenerlo con `derive` o escribirlo a mano.

> **Objetivos:**
>
> - mostrar tus tipos con `Debug` y `Display`;
> - copiar, comparar y ordenar tus estructuras con `Clone`, `Copy`, `PartialEq` y `Ord`;
> - usar una estructura como clave de `HashMap` con `Hash`;
> - convertir con `From` e `Into`, y crear un valor por defecto con `Default`;
> - escribir un iterador y sobrecargar el operador `+`.

Un [trait](../debuter/glossaire.md#trait) es una lista de capacidades que un tipo
promete tener, como un carné: el carné de conducir dice «esta persona sabe conducir».
El trait `Display` dice «este tipo sabe mostrarse». Ya viste cómo definir tus propios
traits en [Genéricos y traits](generiques.md); aquí miramos los que Rust proporciona.

## derive: traits de regalo

Muchos traits estándar se pueden **derivar**: escribes `#[derive(…)]` encima del tipo,
y el compilador escribe el código por ti.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let a = Punto { x: 1, y: 2 };
    let b = a.clone();
    println!("{:?}", a);
    println!("{}", a == b);
}
```

```sortie
Punto { x: 1, y: 2 }
true
```

`derive` solo funciona si **todos los campos** ya tienen ese trait. Un `i32` sabe
clonarse y compararse: así que `Punto` también.

## Debug y Display: mostrarse

Hay dos maneras de mostrar un valor:

| Trait | Formato | ¿Para quién? | Cómo obtenerlo |
|-------|---------|--------------|----------------|
| `Debug` | `{:?}` o `{:#?}` | el programador | `#[derive(Debug)]` |
| `Display` | `{}` | el usuario | escribirlo a mano |

`Display` no se puede derivar: Rust no puede adivinar cómo quieres presentar tu tipo.
Se escribe con `impl` y un método `fmt`:

```rust
use std::fmt;

#[derive(Debug)]
struct Temperatura {
    grados: f64,
}

impl fmt::Display for Temperatura {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} °C", self.grados)
    }
}

fn main() {
    let t = Temperatura { grados: 21.456 };
    println!("{}", t);
    println!("{:?}", t);
    let texto = t.to_string();
    println!("{}", texto.len());
}
```

```sortie
21.5 °C
Temperatura { grados: 21.456 }
8
```

`write!` funciona como `format!`, pero escribe en `f`. Como extra, todo tipo que tiene
`Display` recibe el método `to_string()`.

> **Consejo:** `{:#?}` muestra una estructura en varias líneas, con sangría. Es
> práctico para las estructuras grandes.

## Clone y Copy: duplicar

- `Clone` da el método `.clone()`: una copia **explícita**, quizás costosa
  (una `String` copia todo su texto).
- `Copy` hace que la copia sea **automática**: `let b = a;` copia en lugar de
  [mover](possession.md#el-movimiento). Reservado a los tipos pequeños y simples.

```rust
#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let rojo = Color { r: 255, g: 0, b: 0 };
    let otro = rojo; // copiado, no movido
    println!("{:?}", rojo);
    println!("{:?}", otro);
}
```

```sortie
Color { r: 255, g: 0, b: 0 }
Color { r: 255, g: 0, b: 0 }
```

`Copy` también exige `Clone`. Una estructura que contiene una `String` o un `Vec` no
puede ser `Copy`: solo tendrá `Clone`.

## PartialEq y Eq: comparar con ==

`PartialEq` da los operadores `==` y `!=`. El `derive` compara **todos los campos**
uno por uno.

`Eq` es una promesa más: «todo valor es igual a sí mismo». Es cierto para los enteros
y los textos, pero no para los flotantes: `f64::NAN == f64::NAN` es falso. Así que un
tipo con un campo `f64` solo puede tener `PartialEq`.

```rust
#[derive(Debug, PartialEq, Eq)]
struct Carta {
    valor: u8,
    palo: char,
}

fn main() {
    let a = Carta { valor: 10, palo: '♥' };
    let b = Carta { valor: 10, palo: '♠' };
    println!("{}", a == b);
    println!("{}", a != b);
    println!("{}", f64::NAN == f64::NAN);
}
```

```sortie
false
true
false
```

## PartialOrd y Ord: ordenar

`PartialOrd` da `<`, `>`, `<=`, `>=`. `Ord` da un orden **total**, necesario para
`sort()`, `max()` o un `BTreeMap`. Exige `Eq` y `PartialOrd`.

El `derive` compara los campos **en el orden en que están escritos**, como un
diccionario compara la primera letra, luego la segunda:

```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Alumno {
    nota: u32,
    nombre: String,
}

fn main() {
    let mut clase = vec![
        Alumno { nota: 15, nombre: String::from("Zoé") },
        Alumno { nota: 12, nombre: String::from("Léo") },
        Alumno { nota: 15, nombre: String::from("Ali") },
    ];
    clase.sort();
    for a in &clase {
        println!("{} {}", a.nota, a.nombre);
    }
}
```

```sortie
12 Léo
15 Ali
15 Zoé
```

Los alumnos se ordenan por nota, y luego por nombre en caso de empate, porque `nota`
está escrito primero.

### Ordenar de otra manera, sin tocar el tipo

A menudo, el orden depende de la situación. `sort_by_key` y `sort_by` eligen el
criterio en el momento de ordenar, sin ningún `derive`:

```rust
struct Alumno {
    nombre: String,
    nota: u32,
}

fn main() {
    let mut clase = vec![
        Alumno { nombre: String::from("Zoé"), nota: 15 },
        Alumno { nombre: String::from("Léo"), nota: 12 },
        Alumno { nombre: String::from("Ali"), nota: 18 },
    ];
    clase.sort_by_key(|a| a.nota);
    println!("{}", clase[0].nombre);

    clase.sort_by(|a, b| b.nota.cmp(&a.nota)); // descendente
    println!("{}", clase[0].nombre);

    clase.sort_by(|a, b| a.nombre.cmp(&b.nombre));
    println!("{}", clase[0].nombre);
}
```

```sortie
Léo
Ali
Ali
```

`cmp` compara dos valores y devuelve un `Ordering`: `Less`, `Equal` o `Greater`.
Intercambiar `a` y `b` invierte el orden.

## Hash: para servir de clave

Una clave de `HashMap` o un elemento de `HashSet` debe tener `Hash` (saber resumirse en
un número) y `Eq`. Se derivan juntos (ver [HashMap](hashmap.md)):

```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Casilla {
    fila: u8,
    columna: u8,
}

fn main() {
    let mut tablero = HashMap::new();
    tablero.insert(Casilla { fila: 0, columna: 0 }, "torre");
    tablero.insert(Casilla { fila: 0, columna: 4 }, "rey");
    println!("{}", tablero[&Casilla { fila: 0, columna: 4 }]);
}
```

```sortie
rey
```

## Default: un valor inicial

`Default` da un valor «vacío»: `0` para los números, `""` para los textos, `false`
para los booleanos. Con `..Default::default()`, solo rellenas los campos que te
interesan:

```rust
#[derive(Debug, Default)]
struct Ajustes {
    volumen: u8,
    pantalla_completa: bool,
    apodo: String,
}

fn main() {
    let a = Ajustes::default();
    println!("{:?}", a);
    let a2 = Ajustes { volumen: 7, ..Default::default() };
    println!("{:?}", a2);
}
```

```sortie
Ajustes { volumen: 0, pantalla_completa: false, apodo: "" }
Ajustes { volumen: 7, pantalla_completa: false, apodo: "" }
```

Para otros valores iniciales, escribe `impl Default` a mano:

```rust
struct Partida {
    vidas: u32,
    nivel: u32,
}

impl Default for Partida {
    fn default() -> Self {
        Partida { vidas: 3, nivel: 1 }
    }
}

fn main() {
    let p = Partida::default();
    println!("{} vidas, nivel {}", p.vidas, p.nivel);
}
```

```sortie
3 vidas, nivel 1
```

## From e Into: convertir

`From` describe una conversión de un tipo hacia el tuyo. Al escribirlo, recibes `Into`
**gratis**, en el otro sentido:

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

fn main() {
    let f = Fahrenheit::from(Celsius(100.0));
    println!("{}", f.0);

    let g: Fahrenheit = Celsius(0.0).into();
    println!("{}", g.0);
}
```

```sortie
212
32
```

Ya conoces `String::from("…")`: es el mismo trait. `From` también se usa mucho para
los errores, con `?` (ver [Gestionar los errores](erreurs.md)).

> **Recuerda:** implementa `From`, nunca `Into`: `Into` viene solo.

## Iterator: un iterador hecho a mano

Un tipo se convierte en un [iterador](../debuter/glossaire.md#iterador) al
implementar un solo método: `next`, que devuelve `Some(valor)`, o `None` cuando se
acaba. Todos los demás métodos (`map`, `filter`, `sum`…) vienen con él.

```rust
struct CuentaAtras {
    resto: u32,
}

impl Iterator for CuentaAtras {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.resto == 0 {
            None
        } else {
            self.resto -= 1;
            Some(self.resto + 1)
        }
    }
}

fn main() {
    for n in (CuentaAtras { resto: 3 }) {
        println!("{n}…");
    }
    let total: u32 = CuentaAtras { resto: 4 }.sum();
    println!("total: {total}");
}
```

```sortie
3…
2…
1…
total: 10
```

`type Item = u32;` dice qué tipo de valor produce el iterador. El capítulo
[Iteradores](iterateurs.md) va más lejos.

## AsRef<str>: aceptar varias clases de texto

`AsRef<str>` quiere decir «puedo verme como un `&str`». Una función que lo acepta toma
tanto un `&str` como una `String`:

```rust
fn saludar<S: AsRef<str>>(nombre: S) {
    println!("¡Hola, {}!", nombre.as_ref());
}

fn main() {
    saludar("Zoé");
    saludar(String::from("Léo"));
}
```

```sortie
¡Hola, Zoé!
¡Hola, Léo!
```

> **Nota:** para empezar, un parámetro `&str` basta casi siempre. Te encontrarás
> `AsRef` sobre todo en la biblioteca estándar, por ejemplo
> `AsRef<Path>` para los [archivos](fichiers.md).

## Add: sobrecargar el operador +

Los operadores también son traits, en `std::ops`. Implementa `Add` y tu tipo se puede
sumar con `+`:

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector {
    x: i32,
    y: i32,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, otro: Vector) -> Vector {
        Vector { x: self.x + otro.x, y: self.y + otro.y }
    }
}

fn main() {
    let a = Vector { x: 1, y: 2 };
    let b = Vector { x: 10, y: 20 };
    println!("{:?}", a + b);
    println!("{}", a + b == Vector { x: 11, y: 22 });
}
```

```sortie
Vector { x: 11, y: 22 }
true
```

`Sub` (`-`), `Mul` (`*`) y `Neg` (el `-` delante) siguen el mismo modelo.

## ¿derive o impl a mano?

| Trait | Sirve para | ¿`derive`? | A mano cuando… |
|-------|------------|------------|----------------|
| `Debug` | `{:?}` | sí | rara vez |
| `Display` | `{}`, `to_string()` | **no** | siempre |
| `Clone` | `.clone()` | sí | rara vez |
| `Copy` | copia automática | sí | nunca |
| `PartialEq`, `Eq` | `==`, `!=` | sí | algunos campos no cuentan |
| `PartialOrd`, `Ord` | `<`, `sort()` | sí | el orden no es el de los campos |
| `Hash` | clave de `HashMap` | sí | rara vez |
| `Default` | `::default()` | sí | los valores iniciales no son 0 o vacío |
| `From` | conversiones | **no** | siempre |
| `Iterator` | `for`, `map`, `sum`… | **no** | siempre |
| `Add` | `+` | **no** | siempre |

> **Consejo:** una buena costumbre para tus estructuras de datos simples:
> `#[derive(Debug, Clone, PartialEq)]`. Añade el resto cuando el compilador lo pida.

## Errores frecuentes

### Ordenar una estructura sin Ord

```rust norun
#[derive(Debug)]
struct Alumno {
    nombre: String,
    nota: u32,
}

fn main() {
    let mut clase = vec![
        Alumno { nombre: String::from("Zoé"), nota: 15 },
        Alumno { nombre: String::from("Léo"), nota: 12 },
    ];
    clase.sort();
    println!("{:?}", clase);
}
```

```text
error[E0277]: the trait bound `Alumno: Ord` is not satisfied
  --> src/main.rs:12:11
   |
12 |     clase.sort();
   |           ^^^^ the trait `Ord` is not implemented for `Alumno`
   |
help: consider annotating `Alumno` with `#[derive(Ord)]`
   |
 2 + #[derive(Ord)]
 3 | struct Alumno {
   |
```

Rust no sabe cómo poner en orden a dos alumnos. Deriva
`PartialEq, Eq, PartialOrd, Ord`, o más sencillo, elige el criterio:
`clase.sort_by_key(|a| a.nota);`.

### Copy en una estructura que contiene una String

```rust norun
#[derive(Debug, Clone, Copy)]
struct Jugador {
    nombre: String,
    puntos: u32,
}

fn main() {
    let j = Jugador { nombre: String::from("Zoé"), puntos: 3 };
    let k = j;
    println!("{:?} {:?}", j, k);
}
```

```text
error[E0204]: the trait `Copy` cannot be implemented for this type
 --> src/main.rs:2:8
  |
1 | #[derive(Debug, Clone, Copy)]
  |                        ---- in this derive macro expansion
2 | struct Jugador {
  |        ^^^^^^^
3 |     nombre: String,
  |     -------------- this field does not implement `Copy`
```

Una `String` no se puede copiar automáticamente. Quita `Copy` y escribe
`let k = j.clone();`.

### Eq en una estructura con un f64

```rust norun
#[derive(Debug, PartialEq, Eq)]
struct Producto {
    nombre: String,
    precio: f64,
}

fn main() {
    let p = Producto { nombre: String::from("pan"), precio: 1.2 };
    println!("{:?}", p);
}
```

```text
error[E0277]: the trait bound `f64: Eq` is not satisfied
 --> src/main.rs:4:5
  |
1 | #[derive(Debug, PartialEq, Eq)]
  |                            -- in this derive macro expansion
...
4 |     precio: f64,
  |     ^^^^^^^^^^^ the trait `Eq` is not implemented for `f64`
```

Los flotantes solo tienen `PartialEq`, por culpa de `NAN`. Quita `Eq`, o guarda el
precio en céntimos en un `u32`.

## Ponte a prueba

**1.** ¿Qué diferencia hay entre `Debug` y `Display`?

> **Respuesta**
>
> `Debug` (`{:?}`) es una visualización técnica para el programador, y se deriva.
> `Display` (`{}`) es la visualización para el usuario, y se escribe a mano.

**2.** Implementas `From<Celsius> for Fahrenheit`. ¿Puedes escribir
`let f: Fahrenheit = c.into();`?

> **Respuesta**
>
> Sí: implementar `From` da `Into` automáticamente en el otro sentido.

**3.** Escribe `Display` para una estructura `Fraccion { arriba: i32, abajo: i32 }`,
que se muestre `3/4`.

> **Respuesta**
>
> ```rust
> use std::fmt;
>
> struct Fraccion {
>     arriba: i32,
>     abajo: i32,
> }
>
> impl fmt::Display for Fraccion {
>     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
>         write!(f, "{}/{}", self.arriba, self.abajo)
>     }
> }
>
> fn main() {
>     println!("{}", Fraccion { arriba: 3, abajo: 4 });
> }
> ```
>
> ```sortie
> 3/4
> ```

## En resumen

- `#[derive(…)]` escribe por ti `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`,
  `PartialOrd`, `Ord`, `Hash` y `Default`.
- `Display` se escribe a mano y da `{}` y `to_string()`.
- `Ord` permite `sort()`; `sort_by_key` ordena según un criterio sin derivar nada.
- Una clave de `HashMap` necesita `Eq` y `Hash`.
- Implementa `From` y recibes `Into`; implementa `next` y recibes todo `Iterator`.
- `Add` y sus primos de `std::ops` dan sentido a `+`, `-` y `*`.

Siguiente: [gestionar los errores](erreurs.md).
