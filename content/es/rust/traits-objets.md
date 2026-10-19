# Los objetos trait: dyn Trait

Este capítulo explica cómo manejar valores de tipos distintos que comparten un mismo trait, gracias a `dyn Trait`, y cuándo es mejor usar genéricos.

> **Objetivos:**
>
> - entender la diferencia entre un genérico (estático) y `dyn Trait` (dinámico);
> - usar `&dyn Trait` y `Box<dyn Trait>`;
> - guardar valores de tipos distintos en un `Vec<Box<dyn Trait>>`;
> - elegir entre genérico y `dyn`;
> - reconocer un trait que no puede convertirse en `dyn`;
> - devolver una closure con `Box<dyn Fn>`.

Este capítulo sigue a [Genéricos y traits](generiques.md). Vuelve a leerlo si la palabra
[trait](../debuter/glossaire.md#trait) no te queda clara.

## El punto de partida: unas formas y un trait

Tomemos tres formas: un cuadrado, un rectángulo y un círculo. Todas son distintas, pero
todas saben hacer dos cosas: dar su **nombre** y su **área** (su superficie). Ese punto
en común se convierte en un trait, `Forma`:

```rust
use std::f64::consts::PI;

trait Forma {
    fn nombre(&self) -> String;
    fn area(&self) -> f64;
}

struct Cuadrado {
    lado: f64,
}

struct Rectangulo {
    ancho: f64,
    alto: f64,
}

struct Circulo {
    radio: f64,
}

impl Forma for Cuadrado {
    fn nombre(&self) -> String {
        String::from("cuadrado")
    }
    fn area(&self) -> f64 {
        self.lado * self.lado
    }
}

impl Forma for Rectangulo {
    fn nombre(&self) -> String {
        String::from("rectángulo")
    }
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }
}

impl Forma for Circulo {
    fn nombre(&self) -> String {
        String::from("círculo")
    }
    fn area(&self) -> f64 {
        PI * self.radio * self.radio
    }
}

fn main() {
    let c = Cuadrado { lado: 2.0 };
    let r = Rectangulo { ancho: 2.0, alto: 3.0 };
    let o = Circulo { radio: 1.0 };
    println!("{}: {:.2}", c.nombre(), c.area());
    println!("{}: {:.2}", r.nombre(), r.area());
    println!("{}: {:.2}", o.nombre(), o.area());
}
```

```sortie
cuadrado: 4.00
rectángulo: 6.00
círculo: 3.14
```

`{:.2}` muestra dos cifras después del punto decimal. Para que los ejemplos sean
cortos, el resto de la página solo usa el cuadrado y el círculo.

## Primera forma: el genérico (estático)

Ya conoces `impl Forma` como parámetro. Quiere decir «cualquier tipo que tenga el
trait `Forma`»:

```rust
trait Forma {
    fn area(&self) -> f64;
}

struct Cuadrado(f64);
struct Circulo(f64);

impl Forma for Cuadrado {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

impl Forma for Circulo {
    fn area(&self) -> f64 {
        3.0 * self.0 * self.0
    }
}

fn describir(f: &impl Forma) {
    println!("área: {}", f.area());
}

fn main() {
    describir(&Cuadrado(2.0));
    describir(&Circulo(1.0));
}
```

```sortie
área: 4
área: 3
```

`struct Cuadrado(f64);` es una estructura con un solo campo sin nombre: se lee con
`self.0`. Aquí, el círculo toma 3 como valor de π, para tener números redondos.

¿Qué hace Rust entre bastidores? Durante la compilación, **copia** la función para cada
tipo usado. Es como un impresor que prepara un sello por forma:

```text
tu código:       describir(f: &impl Forma)

lo que Rust hace con él, al compilar:
                 describir_para_Cuadrado(f: &Cuadrado)  → llama a Cuadrado::area
                 describir_para_Circulo(f: &Circulo)    → llama a Circulo::area
```

Se dice que es **estático**: todo se decide antes de que el programa se ejecute.

> **Recuerda:** un genérico produce una copia de la función por tipo. Es muy rápido,
> pero cada copia solo conoce **un único** tipo.

## Segunda forma: dyn Trait (dinámico)

Escribe `&dyn Forma` en lugar de `&impl Forma`. El programa muestra lo mismo:

```rust
trait Forma {
    fn area(&self) -> f64;
}

struct Cuadrado(f64);
struct Circulo(f64);

impl Forma for Cuadrado {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

impl Forma for Circulo {
    fn area(&self) -> f64 {
        3.0 * self.0 * self.0
    }
}

fn describir(f: &dyn Forma) {
    println!("área: {}", f.area());
}

fn main() {
    describir(&Cuadrado(2.0));
    describir(&Circulo(1.0));
}
```

```sortie
área: 4
área: 3
```

Pero esta vez solo existe **una única** función `describir`. No sabe si recibe un
cuadrado o un círculo. La palabra `dyn` (de *dinámico*) quiere decir: «el tipo real se
descubrirá durante la ejecución».

Un valor `&dyn Forma` se llama un **objeto trait**.

## Cómo funciona: la tabla de métodos

¿Cómo encuentra `describir` el método `area` correcto? Un `&dyn Forma` no es una simple
dirección. Son **dos** datos pegados:

1. dónde está el valor (el propio cuadrado);
2. dónde está la **tabla de métodos** de su tipo, que se llama *vtable*.

```text
  &dyn Forma
  ┌────────────┬────────────┐
  │  datos   ──┼──► Cuadrado(2.0)
  │  tabla   ──┼──► vtable de Cuadrado
  └────────────┴────────────┘          ┌────────────────────────┐
                                       │ area → Cuadrado::area  │
                                       └────────────────────────┘
```

Piensa en una maleta con una etiqueta de instrucciones atada. Quien la recibe no abre la
maleta para adivinar qué contiene: lee la etiqueta, que dice qué método llamar.

Una llamada `f.area()` ocurre en dos pasos:

| Paso | Lo que hace el programa |
|------|-------------------------|
| 1 | lee la tabla atada a `f` |
| 2 | encuentra en ella la dirección del `area` correcto y lo llama |

Puedes comprobar que el objeto trait es el doble de grande que una referencia simple:

```rust
use std::mem::size_of;

trait Forma {
    fn area(&self) -> f64;
}

struct Cuadrado(f64);

fn main() {
    println!("{}", size_of::<&dyn Forma>() == 2 * size_of::<&Cuadrado>());
}
```

```sortie
true
```

> **Recuerda:** `dyn Trait` = una sola función, y un pequeño rodeo por la tabla en
> cada llamada. Es un poquito más lento, pero mucho más flexible.

## Box<dyn Trait>: poseer una forma

`&dyn Forma` **toma prestada** una forma que existe en otro sitio. Para **poseer** una
forma cuyo tipo exacto no conoces, la guardas en una `Box`:

```rust
trait Forma {
    fn area(&self) -> f64;
}

struct Cuadrado(f64);

impl Forma for Cuadrado {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

fn main() {
    let f: Box<dyn Forma> = Box::new(Cuadrado(3.0));
    println!("{}", f.area());
}
```

```sortie
9
```

`Box::new(x)` mete `x` en una caja, guardada en una zona de la memoria llamada el
montón (*heap*). La `Box` en sí siempre tiene el mismo tamaño, sea cual sea la forma
que contiene. Por eso Rust la acepta. Box se explica en detalle en
[Box, Rc y RefCell](pointeurs.md).

¿Por qué no simplemente `let f: dyn Forma = …`? Porque un cuadrado y un círculo no
tienen el mismo tamaño. Rust debe conocer el tamaño de cada variable antes de lanzar el
programa. Una caja, en cambio, siempre tiene el mismo tamaño.

Prueba: cambia `Cuadrado(3.0)` por otra forma que tenga el trait `Forma`. El tipo de
`f` no cambia: sigue siendo `Box<dyn Forma>`.

## Vec<Box<dyn Trait>>: una lista de formas mezcladas

Por fin, aquí está la respuesta al problema del principio: una lista que contiene
cuadrados **y** círculos.

```rust
trait Forma {
    fn nombre(&self) -> String;
    fn area(&self) -> f64;
}

struct Cuadrado(f64);
struct Circulo(f64);

impl Forma for Cuadrado {
    fn nombre(&self) -> String {
        String::from("cuadrado")
    }
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

impl Forma for Circulo {
    fn nombre(&self) -> String {
        String::from("círculo")
    }
    fn area(&self) -> f64 {
        3.0 * self.0 * self.0
    }
}

fn main() {
    let formas: Vec<Box<dyn Forma>> = vec![
        Box::new(Cuadrado(2.0)),
        Box::new(Circulo(1.0)),
        Box::new(Cuadrado(1.0)),
    ];

    let mut total = 0.0;
    for f in &formas {
        println!("{}: {}", f.nombre(), f.area());
        total += f.area();
    }
    println!("total: {total}");
}
```

```sortie
cuadrado: 4
círculo: 3
cuadrado: 1
total: 8
```

Sigamos el bucle paso a paso:

| Paso | f | tipo real | área | total |
|------|---|-----------|------|-------|
| inicio | – | – | – | 0 |
| 1 | `formas[0]` | `Cuadrado(2.0)` | 4 | 4 |
| 2 | `formas[1]` | `Circulo(1.0)` | 3 | 7 |
| 3 | `formas[2]` | `Cuadrado(1.0)` | 1 | 8 |

En la memoria, el `Vec` contiene tres cajas del mismo tamaño. Cada una apunta a su forma
y a su tabla:

```text
formas: [   caja    |   caja    |   caja    ]
              │           │           │
              ▼           ▼           ▼
          Cuadrado     Circulo     Cuadrado
          (tabla       (tabla      (tabla
          Cuadrado)    Circulo)    Cuadrado)
```

Prueba: añade `Box::new(Circulo(2.0))` a la lista. Aparece una línea `círculo: 12` y el
total pasa a 20.

### Fabricar una forma según una elección

Una función puede devolver una `Box<dyn Forma>` cuyo tipo depende de un valor que solo
se conoce durante la ejecución:

```rust
trait Forma {
    fn area(&self) -> f64;
}

struct Cuadrado(f64);
struct Circulo(f64);

impl Forma for Cuadrado {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

impl Forma for Circulo {
    fn area(&self) -> f64 {
        3.0 * self.0 * self.0
    }
}

fn fabricar(letra: char, tamano: f64) -> Box<dyn Forma> {
    if letra == 'c' {
        Box::new(Cuadrado(tamano))
    } else {
        Box::new(Circulo(tamano))
    }
}

fn main() {
    let pedido = "c2 o1 c3";
    let mut formas = Vec::new();
    for palabra in pedido.split(' ') {
        let letra = palabra.chars().next().unwrap();
        let tamano: f64 = palabra[1..].parse().unwrap();
        formas.push(fabricar(letra, tamano));
    }
    for f in &formas {
        println!("{}", f.area());
    }
}
```

```sortie
4
3
9
```

Con un genérico, sería imposible: las dos ramas del `if` deben tener el mismo tipo.
Aquí, ese tipo común es `Box<dyn Forma>`.

## Genérico o dyn: ¿cuál elegir?

| Pregunta | Genérico (`impl Trait`, `<T: Trait>`) | `dyn Trait` |
|----------|---------------------------------------|-------------|
| ¿Cuándo se elige el tipo? | al compilar | durante la ejecución |
| ¿Varios tipos en un mismo `Vec`? | no | sí, con `Box` |
| Velocidad de llamada | la más rápida | un pequeño rodeo por la tabla |
| Tamaño del programa | una copia por tipo | una sola función |
| ¿Todos los traits posibles? | sí | no, ver más abajo |

Una regla sencilla para empezar:

- **un solo tipo cada vez** (una función que recibe «una forma»): el genérico;
- **varios tipos mezclados** (una lista de formas, una elección hecha durante la
  ejecución): `dyn`.

> **Recuerda:** empieza por `impl Trait`. Pasa a `dyn Trait` cuando necesites
> **mezclar** tipos distintos.

## Devolver una closure con `Box<dyn Fn>`

Una [closure](fonctions.md#las-closures) es una pequeña función sin nombre. Cada
closure tiene su propio tipo, que nadie puede escribir. Para que una función devuelva
**una u otra** de dos closures, se usa el trait `Fn` en versión `dyn`:

```rust
fn ajuste(n: i32, doblar: bool) -> Box<dyn Fn(i32) -> i32> {
    if doblar {
        Box::new(move |x| x * n)
    } else {
        Box::new(move |x| x + n)
    }
}

fn main() {
    let f = ajuste(3, true);
    let g = ajuste(3, false);
    println!("{}", f(5));
    println!("{}", g(5));
}
```

```sortie
15
8
```

Lee `Box<dyn Fn(i32) -> i32>` así: «una caja que contiene una función, cualquiera, que
recibe un `i32` y devuelve un `i32`». `move` hace que la closure se lleve `n` consigo.

También se pueden guardar closures distintas en un `Vec`:

```rust
fn main() {
    let pasos: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 10),
        Box::new(|x| x - 3),
    ];
    let mut valor = 2;
    for paso in &pasos {
        valor = paso(valor);
        println!("{valor}");
    }
}
```

```sortie
3
30
27
```

| Paso | cálculo | valor |
|------|---------|-------|
| inicio | – | 2 |
| 1 | 2 + 1 | 3 |
| 2 | 3 × 10 | 30 |
| 3 | 30 − 3 | 27 |

Las closures tienen su propio capítulo completo: [Las closures en detalle](closures.md).

## Cuando un trait no puede convertirse en dyn

Algunos traits no pueden servir como objeto trait. Rust dice entonces que el trait no es
*dyn compatible* (antes se decía *object safe*).

La razón es sencilla. Un objeto trait solo tiene su **tabla de métodos**. Si un método
necesita conocer el tipo real para existir, no puede entrar en la tabla. Estos son los
dos casos más frecuentes:

| El trait contiene… | Por qué bloquea |
|--------------------|-----------------|
| una función sin `self` que devuelve `Self` (`fn nueva() -> Self`) | sin valor, es imposible leer una tabla; y `Self` no tiene un tamaño conocido |
| un método genérico (`fn comparar<T>(&self, x: T)`) | haría falta una casilla en la tabla para **cada** tipo `T` posible |

Este es el mensaje real:

```rust norun
trait Forma {
    fn nueva() -> Self;
    fn area(&self) -> f64;
}

fn mostrar(f: &dyn Forma) {
    println!("{}", f.area());
}

fn main() {}
```

```text
error[E0038]: the trait `Forma` is not dyn compatible
 --> src/main.rs:6:16
  |
6 | fn mostrar(f: &dyn Forma) {
  |                ^^^^^^^^^ `Forma` is not dyn compatible
  |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
 --> src/main.rs:2:8
  |
1 | trait Forma {
  |       ----- this trait is not dyn compatible...
2 |     fn nueva() -> Self;
  |        ^^^^^ ...because associated function `nueva` has no `self` parameter
help: consider turning `nueva` into a method by giving it a `&self` argument
  |
2 |     fn nueva(&self) -> Self;
  |              +++++
help: alternatively, consider constraining `nueva` so it does not apply to trait objects
  |
2 |     fn nueva() -> Self where Self: Sized;
  |                        +++++++++++++++++
```

El mensaje dice claramente «it needs to allow building a vtable»: hay que poder construir
la tabla. La segunda ayuda es la más práctica: `where Self: Sized` quita esa función de
la tabla. El resto del trait se puede usar con `dyn`:

```rust
trait Forma {
    fn nueva() -> Self
    where
        Self: Sized;
    fn area(&self) -> f64;
}

struct Cuadrado(f64);

impl Forma for Cuadrado {
    fn nueva() -> Self {
        Cuadrado(1.0)
    }
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

fn mostrar(f: &dyn Forma) {
    println!("{}", f.area());
}

fn main() {
    let c = Cuadrado::nueva();
    mostrar(&c);
}
```

```sortie
1
```

> **Nota:** el trait `Clone` está en este caso: `clone` devuelve `Self`. Por tanto, un
> trait que exige `Clone` (`trait Forma: Clone`) no puede convertirse en `dyn Forma`.

## Errores frecuentes

### Olvidar la palabra dyn

```rust norun
trait Forma {
    fn area(&self) -> f64;
}

struct Cuadrado(f64);

impl Forma for Cuadrado {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

fn main() {
    let f: Box<Forma> = Box::new(Cuadrado(2.0));
}
```

```text
error[E0782]: expected a type, found a trait
  --> src/main.rs:14:16
   |
14 |     let f: Box<Forma> = Box::new(Cuadrado(2.0));
   |                ^^^^^
   |
help: you can add the `dyn` keyword if you want a trait object
   |
14 |     let f: Box<dyn Forma> = Box::new(Cuadrado(2.0));
   |                +++
```

Un trait no es un tipo. Escribe `Box<dyn Forma>`.

### Un Vec<dyn Trait> sin Box

```rust norun
trait Forma {
    fn area(&self) -> f64;
}

fn main() {
    let formas: Vec<dyn Forma> = Vec::new();
}
```

```text
error[E0277]: the size for values of type `dyn Forma` cannot be known at compilation time
 --> src/main.rs:6:17
  |
6 |     let formas: Vec<dyn Forma> = Vec::new();
  |                 ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `dyn Forma`
```

Las formas no tienen todas el mismo tamaño. Mete cada una en una caja:
`Vec<Box<dyn Forma>>`.

### Un tipo que no tiene el trait

```rust norun
trait Forma {
    fn area(&self) -> f64;
}

struct Cuadrado(f64);
struct Circulo(f64);

impl Forma for Cuadrado {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

fn main() {
    let formas: Vec<Box<dyn Forma>> = vec![Box::new(Cuadrado(2.0)), Box::new(Circulo(1.0))];
    println!("{}", formas.len());
}
```

```text
error[E0277]: the trait bound `Circulo: Forma` is not satisfied
  --> src/main.rs:15:69
   |
15 |     let formas: Vec<Box<dyn Forma>> = vec![Box::new(Cuadrado(2.0)), Box::new(Circulo(1.0))];
   |                                                                     ^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
   |
help: the trait `Forma` is not implemented for `Circulo`
```

Solo pueden entrar los tipos que han firmado el contrato. Añade
`impl Forma for Circulo { … }`.

### Devolver dos closures con impl Fn

```rust norun
fn ajuste(n: i32, doblar: bool) -> impl Fn(i32) -> i32 {
    if doblar {
        move |x| x * n
    } else {
        move |x| x + n
    }
}

fn main() {
    println!("{}", ajuste(3, true)(5));
}
```

```text
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:5:9
  |
2 | /     if doblar {
3 | |         move |x| x * n
  | |         --------------
  | |         |
  | |         the expected closure
  | |         expected because of this
4 | |     } else {
5 | |         move |x| x + n
  | |         ^^^^^^^^^^^^^^ expected closure, found a different closure
6 | |     }
  | |_____- `if` and `else` have incompatible types
  |
  = note: expected closure `{closure@src/main.rs:3:9: 3:17}`
             found closure `{closure@src/main.rs:5:9: 5:17}`
  = note: no two closures, even if identical, have the same type
  = help: consider boxing your closure and/or using it as a trait object
```

`impl Fn` designa **un solo** tipo oculto. Dos closures tienen dos tipos distintos.
Devuelve `Box<dyn Fn(i32) -> i32>` y mete cada closure en `Box::new(…)`, como en
[Devolver una closure con Box<dyn Fn>](#devolver-una-closure-con-box-dyn-fn).

## Ponte a prueba

**1.** ¿Cuál es la diferencia entre `fn f(x: &impl Forma)` y `fn f(x: &dyn Forma)`?

> **Respuesta**
>
> Con `impl Forma`, Rust fabrica una copia de `f` para cada tipo, al compilar
> (estático). Con `dyn Forma`, solo hay una función; encuentra el método correcto
> durante la ejecución, gracias a la tabla de métodos (dinámico).

**2.** ¿Por qué se escribe `Vec<Box<dyn Forma>>` y no `Vec<dyn Forma>`?

> **Respuesta**
>
> Un cuadrado y un círculo no tienen el mismo tamaño. Un `Vec` exige elementos de
> tamaño conocido. Una `Box` siempre tiene el mismo tamaño, sea cual sea la forma que
> contiene.

**3.** Escribe un trait `Saludar` con `fn hola(&self) -> String`, dos estructuras `Es` y
`En`, y un bucle sobre un `Vec<Box<dyn Saludar>>` que muestre `hola` y luego `hello`.

> **Respuesta**
>
> ```rust
> trait Saludar {
>     fn hola(&self) -> String;
> }
>
> struct Es;
> struct En;
>
> impl Saludar for Es {
>     fn hola(&self) -> String {
>         String::from("hola")
>     }
> }
>
> impl Saludar for En {
>     fn hola(&self) -> String {
>         String::from("hello")
>     }
> }
>
> fn main() {
>     let lista: Vec<Box<dyn Saludar>> = vec![Box::new(Es), Box::new(En)];
>     for s in &lista {
>         println!("{}", s.hola());
>     }
> }
> ```
>
> ```sortie
> hola
> hello
> ```

## En resumen

- Un genérico (`impl Trait`, `<T: Trait>`) es estático: una copia por tipo, elegida al
  compilar.
- `dyn Trait` es dinámico: una sola función, que lee la tabla de métodos (vtable)
  durante la ejecución.
- `&dyn Trait` toma prestado, `Box<dyn Trait>` posee; `Vec<Box<dyn Trait>>` mezcla
  tipos.
- Empieza por los genéricos; pasa a `dyn` para mezclar tipos.
- Un trait con un método genérico o un `-> Self` sin `self` no es dyn compatible
  (salvo con `where Self: Sized`).
- `Box<dyn Fn(…) -> …>` devuelve o guarda closures distintas.

Siguiente: [Los traits de la biblioteca estándar](traits-standard.md)
