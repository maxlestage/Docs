# Propiedad y préstamo

La propiedad es la idea central de Rust: cada valor tiene un solo propietario, y las demás partes del código solo pueden tomarlo prestado.

> **Objetivos:**
>
> - explicar las tres reglas de la propiedad;
> - reconocer un movimiento, y evitarlo con `clone` o un préstamo;
> - tomar prestado un valor para leerlo con `&` y para modificarlo con `&mut`;
> - entender los mensajes de error del verificador de préstamos;
> - leer un tiempo de vida como `'a`.

## ¿Por qué la propiedad?

Un programa guarda sus datos en la memoria. Algún día hay que liberar esa memoria.

- Algunos lenguajes (C) te piden que la liberes a mano: se te olvida, o la
  liberas dos veces.
- Otros (Ruby, Python) usan un recolector de basura que funciona durante la ejecución.

Rust elige un tercer camino: **reglas sencillas**, comprobadas por el compilador.
La memoria se libera automáticamente, en el momento justo, sin coste oculto.

Una imagen ayuda a entenderlo todo: un valor es un **objeto**, por ejemplo un libro.
Una variable que lo **posee** es su propietaria. Puedes regalar el libro
(moverlo), hacerle una fotocopia (clonarlo) o prestarlo (tomarlo prestado).

## Las tres reglas

> **Recuerda:**
>
> 1. Cada valor tiene **un propietario** (una variable).
> 2. Solo hay **un único** propietario a la vez.
> 3. Cuando el propietario desaparece (fin de su bloque `{ }`), el valor se libera.

Una variable solo existe en el bloque donde se crea: es su
[ámbito](../debuter/glossaire.md#portee).

```rust
{
    let s = String::from("hola");
    println!("{s}");
} // aquí, s desaparece: la memoria se libera
println!("fin del bloque");
```

```sortie
hola
fin del bloque
```

## El movimiento

Cuando le das un `String` a otra variable, se **mueve** (en inglés:
*move*). Es como regalar tu libro: ya no lo tienes. La variable antigua
ya no se puede usar.

```text
let a = String::from("gato");
let b = a;

   a  ──╳                     (a ya no es válida)
   b  ──────►  [ "gato" ]     (b es el nuevo propietario)
```

El compilador rechaza usar `a` después de este movimiento. El mismo movimiento
ocurre cuando pasas un valor a una función: la función se convierte en propietaria.
Los dos casos se muestran en [Errores frecuentes](#errores-frecuentes).

### Los tipos copiados

Los tipos pequeños y sencillos (`i32`, `f64`, `bool`, `char`…) se **copian** en lugar
de moverse: copiarlos no cuesta casi nada. Se dice que tienen el trait `Copy`.
Las dos variables siguen siendo utilizables:

```rust
let x = 5;
let y = x;
println!("{x} {y}");
```

```sortie
5 5
```

## Clonar un valor

Para tener **dos** valores independientes, usa `clone()`. Es la fotocopia:
cada uno tiene su ejemplar.

```rust
let a = String::from("gato");
let mut b = a.clone();
b.push_str("s");
println!("{a} {b}");
```

```sortie
gato gatos
```

```text
   a  ──────►  [ "gato" ]
   b  ──────►  [ "gatos" ]    (una copia completa, modificada)
```

> **Consejo:** `clone()` es práctico para empezar. Pero copiar lleva tiempo:
> el préstamo, a continuación, suele ser mejor.

## Tomar prestado con &

Una **referencia** `&` permite usar un valor sin quedárselo. Se dice que lo
**tomas prestado**. Es el préstamo del libro: el otro lo lee y luego te lo devuelve. El
propietario sigue siendo el mismo.

```text
   nombre  ──────►  [ "Zoé" ]
                       ▲
   &nombre ────────────┘   (miramos, no nos lo quedamos)
```

```rust
fn longitud(s: &String) -> usize {
    s.len()
}

fn main() {
    let nombre = String::from("Zoé");
    let n = longitud(&nombre);
    println!("{nombre} tiene {n} bytes");
}
```

```sortie
Zoé tiene 4 bytes
```

> **Nota:** `é` ocupa dos bytes en memoria, de ahí el `4`.

En la práctica, para un parámetro de texto, se escribe `&str` en lugar de `&String`:
acepta los dos tipos de texto.

```rust
fn gritar(s: &str) -> String {
    s.to_uppercase()
}

fn main() {
    let a = String::from("hola");
    println!("{}", gritar(&a));
    println!("{}", gritar("buenas"));
}
```

```sortie
HOLA
BUENAS
```

## Tomar prestado para modificar con &mut

Una referencia `&mut` permite **modificar** el valor prestado: prestas el libro
con un lápiz. La propia variable debe ser `mut`:

```rust
fn agregar_punto(s: &mut String) {
    s.push('.');
}

fn main() {
    let mut frase = String::from("Fin");
    agregar_punto(&mut frase);
    agregar_punto(&mut frase);
    println!("{frase}");
}
```

```sortie
Fin..
```

## Las reglas del préstamo

En un momento dado, puedes tener **o bien**:

- tantas referencias `&` como quieras (varios lectores),
- **o bien** una sola referencia `&mut` (un solo escritor).

Nunca las dos a la vez. Piensa en una pizarra: varias personas pueden
leerla juntas, pero cuando alguien escribe en ella, nadie más debe leer ni
escribir al mismo tiempo.

La parte del compilador que comprueba estas reglas se llama **verificador de préstamos** (en
inglés: *borrow checker*).

```rust
let mut v = vec![1, 2, 3];
let a = &v;
let b = &v;
println!("{:?} {:?}", a, b); // varios lectores: permitido
let c = &mut v;
c.push(4); // a y b ya no se usan: se permite un escritor
println!("{:?}", v);
```

```sortie
[1, 2, 3] [1, 2, 3]
[1, 2, 3, 4]
```

Un préstamo dura hasta su último uso. Aquí, `a` y `b` ya no se usan después
del primer `println!`: por eso el préstamo `&mut` que sigue está permitido.

## Los tiempos de vida

Una referencia nunca debe vivir más que el valor que toma prestado. Uno no
se queda con un libro prestado después de que la biblioteca cierre para siempre. El
compilador lo comprueba con **tiempos de vida** (en inglés: *lifetimes*).

La mayoría de las veces, los adivina solo. A veces hay que escribirlos con un apóstrofo,
como `'a`:

```rust
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    println!("{}", mas_largo("manzana", "kiwi"));
}
```

```sortie
manzana
```

Lee `'a` así: «el resultado vive tanto como `x` e `y`». Para empezar, basta
con saber que existe.

Un texto escrito entre comillas en el código tiene el tiempo de vida `'static`: existe
durante toda la ejecución del programa.

## Errores frecuentes

### Usar un valor movido

```rust norun
fn main() {
    let a = String::from("gato");
    let b = a;
    println!("{a} {b}");
}
```

```text
error[E0382]: borrow of moved value: `a`
 --> src/main.rs:4:16
  |
2 |     let a = String::from("gato");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
3 |     let b = a;
  |             - value moved here
4 |     println!("{a} {b}");
  |                ^ value borrowed here after move
```

Después de `let b = a;`, solo `b` posee el texto. Para conservar los dos, escribe
`let b = a.clone();`, o toma prestado: `let b = &a;`.

### Dar un valor a una función, dos veces

```rust norun
fn mostrar(s: String) {
    println!("{s}");
}

fn main() {
    let nombre = String::from("Zoé");
    mostrar(nombre);
    mostrar(nombre);
}
```

```text
error[E0382]: use of moved value: `nombre`
 --> src/main.rs:8:13
  |
6 |     let nombre = String::from("Zoé");
  |         ------ move occurs because `nombre` has type `String`, which does not implement the `Copy` trait
7 |     mostrar(nombre);
  |             ------ value moved here
8 |     mostrar(nombre);
  |             ^^^^^^ value used here after move
```

La función solo necesita leer: haz que tome prestado el texto.
`fn mostrar(s: &str)`, y luego `mostrar(&nombre);`.

### Modificar durante una lectura

```rust norun
fn main() {
    let mut v = vec![1, 2, 3];
    let primero = &v[0];
    v.push(4);
    println!("{primero}");
}
```

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let primero = &v[0];
  |                    - immutable borrow occurs here
4 |     v.push(4);
  |     ^^^^^^^^^ mutable borrow occurs here
5 |     println!("{primero}");
  |                ------- immutable borrow later used here
```

Este error te protege: `push` puede mover el vector en memoria, y `primero`
apuntaría entonces a una zona liberada. Usa `primero` antes del `push`, o copia el
valor: `let primero = v[0];`.

### Devolver una referencia sin tiempo de vida

```rust norun
fn mas_largo(x: &str, y: &str) -> &str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    println!("{}", mas_largo("manzana", "kiwi"));
}
```

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:35
  |
1 | fn mas_largo(x: &str, y: &str) -> &str {
  |                 ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
```

El compilador no sabe si el resultado toma prestado `x` o `y`. Añade un tiempo de
vida, como en [Los tiempos de vida](#los-tiempos-de-vida).

## Ponte a prueba

**1.** Después de `let a = 5; let b = a;`, ¿se puede seguir usando `a`? ¿Y con
`String::from("5")`?

> **Respuesta**
>
> Con `5`, sí: un `i32` se copia. Con un `String`, no: se mueve a
> `b`.

**2.** ¿Qué firma eliges para una función que cuenta las vocales de un texto
sin modificarlo?

> **Respuesta**
>
> Un préstamo de lectura: `fn vocales(texto: &str) -> usize`.

**3.** Corrige este código para que añada `!` al texto:

```rust norun
fn gritar(s: String) {
    s.push('!');
}
```

> **Respuesta**
>
> Toma prestado para escribir:
>
> ```rust
> fn gritar(s: &mut String) {
>     s.push('!');
> }
>
> fn main() {
>     let mut palabra = String::from("Alto");
>     gritar(&mut palabra);
>     println!("{palabra}");
> }
> ```
>
> ```sortie
> Alto!
> ```

## En resumen

- Cada valor tiene un solo propietario, que se libera al final de su bloque.
- `let b = a;` mueve `a` a `b` (los tipos sencillos se copian).
- `a.clone()` hace una copia independiente.
- `&a` toma prestado para leer; se permiten varios lectores.
- `&mut a` toma prestado para escribir; solo uno a la vez.
- Una referencia nunca vive más que su valor: es su tiempo de vida.

Siguiente: [los tiempos de vida](durees-de-vie.md)
