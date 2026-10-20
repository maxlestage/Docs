# Estructuras, enumeraciones y traits

Esta página muestra cómo crear tus propios tipos con `struct` y `enum`, añadirles métodos con `impl`, y usar `Option` para un valor que puede faltar.

> **Objetivos:**
>
> - agrupar datos en una estructura (`struct`);
> - añadirle métodos y funciones asociadas con `impl`;
> - describir un valor entre varias opciones con una enumeración (`enum`);
> - tratar un valor ausente con `Option`, `match` e `if let`;
> - obtener traits comunes con `#[derive(...)]`.

## Las estructuras

Una **estructura** (`struct`) agrupa varios valores con nombre, llamados **campos**.
Es como un formulario: cada casilla tiene un nombre y espera cierto tipo de respuesta.
El nombre de una estructura empieza por mayúscula.

```rust
struct Persona {
    nombre: String,
    edad: u32,
}

fn main() {
    let p = Persona {
        nombre: String::from("Alice"),
        edad: 30,
    };
    println!("{} tiene {} años", p.nombre, p.edad);
}
```

```sortie
Alice tiene 30 años
```

Para modificar un campo, la variable debe ser `mut`:

```rust
struct Contador {
    valor: u32,
}

fn main() {
    let mut c = Contador { valor: 0 };
    c.valor += 5;
    println!("{}", c.valor);
}
```

```sortie
5
```

## Los métodos con impl

Un bloque `impl` (de *implementación*) añade funciones a una estructura. Un
[método](../debuter/glossaire.md#metodo) es una función unida a un tipo, que
se llama con un punto: `r.area()`. Su primer parámetro, `self`, designa
al propio objeto:

```rust
struct Rectangulo {
    ancho: f64,
    alto: f64,
}

impl Rectangulo {
    // Una función asociada: crea un Rectangulo.
    fn cuadrado(lado: f64) -> Rectangulo {
        Rectangulo { ancho: lado, alto: lado }
    }

    // Un método que lee el objeto.
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }

    // Un método que modifica el objeto.
    fn agrandar(&mut self, factor: f64) {
        self.ancho *= factor;
        self.alto *= factor;
    }
}

fn main() {
    let mut r = Rectangulo { ancho: 3.0, alto: 2.0 };
    println!("{}", r.area());
    r.agrandar(2.0);
    println!("{}", r.area());
    let c = Rectangulo::cuadrado(4.0);
    println!("{}", c.area());
}
```

```sortie
6
24
16
```

| Primer parámetro | Significado |
|------------------|-------------|
| `&self` | el método lee el objeto |
| `&mut self` | el método modifica el objeto |
| ninguno | función asociada, llamada con `Tipo::nombre()` |

### El constructor new

Por costumbre, la función asociada que crea un objeto se llama `new`. En un bloque
`impl`, `Self` es un atajo para el nombre del tipo:

```rust
struct Jugador {
    nombre: String,
    puntos: u32,
}

impl Jugador {
    fn new(nombre: &str) -> Self {
        Self { nombre: nombre.to_string(), puntos: 0 }
    }

    fn anotar(&mut self, n: u32) {
        self.puntos += n;
    }
}

fn main() {
    let mut j = Jugador::new("Sam");
    j.anotar(3);
    j.anotar(2);
    println!("{}: {} puntos", j.nombre, j.puntos);
}
```

```sortie
Sam: 5 puntos
```

`String::new()` y `Vec::new()`, que ya conoces, son funciones asociadas
del mismo tipo.

## Las enumeraciones

Una **enumeración** (`enum`) es un tipo que vale **un** valor entre una lista de
opciones, llamadas **variantes**. Un semáforo está en verde, amarillo **o** rojo, nunca
en dos a la vez. Una variante se escribe con `::`: `Semaforo::Amarillo`.

```rust
enum Semaforo {
    Verde,
    Amarillo,
    Rojo,
}

fn main() {
    let s = Semaforo::Amarillo;
    let accion = match s {
        Semaforo::Verde => "pasa",
        Semaforo::Amarillo => "frena",
        Semaforo::Rojo => "detente",
    };
    println!("{accion}");
}
```

```sortie
frena
```

### Variantes con datos

Cada variante puede llevar sus propios datos: un número para el círculo (su
radio), dos campos con nombre para el rectángulo, nada para el punto.

```rust
enum Forma {
    Circulo(f64),
    Rectangulo { ancho: f64, alto: f64 },
    Punto,
}
```

## match sobre una enumeración

`match` abre cada variante y recupera sus datos:

```rust
enum Forma {
    Circulo(f64),
    Rectangulo { ancho: f64, alto: f64 },
    Punto,
}

fn area(f: &Forma) -> f64 {
    match f {
        Forma::Circulo(radio) => 3.14 * radio * radio,
        Forma::Rectangulo { ancho, alto } => ancho * alto,
        Forma::Punto => 0.0,
    }
}

fn main() {
    let formas = vec![
        Forma::Circulo(1.0),
        Forma::Rectangulo { ancho: 2.0, alto: 5.0 },
        Forma::Punto,
    ];
    for f in &formas {
        println!("{}", area(f));
    }
}
```

```sortie
3.14
10
0
```

> **Recuerda:** si olvidas una variante en un `match`, el programa no
> compila. Añadir una variante te muestra todos los sitios que hay que actualizar.

## El tipo Option

Muchos lenguajes tienen un valor [null](../debuter/glossaire.md#null), que quiere decir
«nada», y que provoca errores cuando se olvida. Rust no tiene `null`. Un
valor que puede faltar tiene el tipo `Option<T>`: una caja que contiene un valor de
tipo `T`, o que está vacía.

- `Some(valor)`: hay un valor;
- `None`: no hay nada.

```rust
fn dividir(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    match dividir(10, 2) {
        Some(n) => println!("Resultado: {n}"),
        None => println!("División imposible"),
    }
    if let Some(n) = dividir(9, 3) {
        println!("9 / 3 = {n}");
    }
    println!("{:?}", dividir(1, 0));
}
```

```sortie
Resultado: 5
9 / 3 = 3
None
```

> **Consejo:** `if let` es un atajo para un `match` que solo se interesa por un
> caso.

### Los métodos útiles de Option

Para los casos sencillos, algunos métodos evitan escribir un `match`:

```rust
let presente: Option<i32> = Some(4);
let ausente: Option<i32> = None;
println!("{} {}", presente.is_some(), ausente.is_none());
println!("{}", ausente.unwrap_or(0));
println!("{:?}", presente.map(|n| n * 10));
```

```sortie
true true
0
Some(40)
```

| Método | Efecto |
|--------|--------|
| `is_some()` / `is_none()` | ¿hay un valor? |
| `unwrap_or(v)` | el valor, o `v` si la caja está vacía |
| `map(f)` | transforma el valor si existe |
| `unwrap()` | el valor, o detiene el programa si la caja está vacía |

## Los traits

Un **trait** describe un comportamiento: una lista de métodos que un tipo promete
tener. Es como un permiso: un tipo que tiene el trait `Clone` «tiene derecho» a ser
copiado con `.clone()`. Lo más sencillo es obtener los traits comunes con
`#[derive(...)]`, una línea colocada justo antes de la estructura:

| Trait | Lo que permite |
|-------|----------------|
| `Debug` | mostrar con `{:?}` |
| `Clone` | copiar con `.clone()` |
| `PartialEq` | comparar con `==` |

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

Escribir tus propios traits se explica en [Genéricos y traits](generiques.md).

## Errores frecuentes

### Olvidar un campo

```rust norun
struct Persona {
    nombre: String,
    edad: u32,
}

fn main() {
    let p = Persona { nombre: String::from("Alice") };
}
```

```text
error[E0063]: missing field `edad` in initializer of `Persona`
 --> src/main.rs:7:13
  |
7 |     let p = Persona { nombre: String::from("Alice") };
  |             ^^^^^^^ missing `edad`
```

Todos los campos deben recibir un valor: `Persona { nombre: …, edad: 30 }`.

### Mostrar una estructura sin Debug

```rust norun
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let p = Punto { x: 1, y: 2 };
    println!("{:?}", p);
}
```

```text
error[E0277]: `Punto` doesn't implement `Debug`
 --> src/main.rs:8:22
  |
8 |     println!("{:?}", p);
  |               ----   ^ `Punto` cannot be formatted using `{:?}` because it doesn't implement `Debug`
  |               |
  |               required by this formatting parameter
  |
  = note: add `#[derive(Debug)]` to `Punto` or manually `impl Debug for Punto`
```

Añade `#[derive(Debug)]` justo antes de `struct Punto`.

### Llamar a un método &mut self sobre una variable no mut

```rust norun
struct Contador {
    valor: u32,
}

impl Contador {
    fn aumentar(&mut self) {
        self.valor += 1;
    }
}

fn main() {
    let c = Contador { valor: 0 };
    c.aumentar();
}
```

```text
error[E0596]: cannot borrow `c` as mutable, as it is not declared as mutable
  --> src/main.rs:13:5
   |
13 |     c.aumentar();
   |     ^ cannot borrow as mutable
```

El método modifica el objeto: la variable debe ser `mut`. `let mut c = …`.

### Olvidar una variante en un match

```rust norun
enum Semaforo {
    Verde,
    Amarillo,
    Rojo,
}

fn main() {
    let s = Semaforo::Rojo;
    match s {
        Semaforo::Verde => println!("pasa"),
        Semaforo::Amarillo => println!("frena"),
    }
}
```

```text
error[E0004]: non-exhaustive patterns: `Semaforo::Rojo` not covered
  --> src/main.rs:9:11
   |
 9 |     match s {
   |           ^ pattern `Semaforo::Rojo` not covered
```

Añade la variante olvidada: `Semaforo::Rojo => println!("detente"),`.

## Ponte a prueba

**1.** ¿Qué diferencia hay entre `fn area(&self)` y `fn cuadrado(lado: f64)` en un bloque
`impl Rectangulo`?

> **Respuesta**
>
> `area` es un método: se llama sobre un objeto, `r.area()`. `cuadrado` es una
> función asociada, sin objeto: se llama con `Rectangulo::cuadrado(4.0)`.

**2.** Escribe una enumeración `Moneda` con las variantes `Cara` y `Cruz`.

> **Respuesta**
>
> ```rust
> #[derive(Debug)]
> enum Moneda {
>     Cara,
>     Cruz,
> }
>
> fn main() {
>     let m = Moneda::Cruz;
>     println!("{:?}", m);
> }
> ```
>
> ```sortie
> Cruz
> ```

**3.** ¿Cuánto vale `None.unwrap_or(7)`? ¿Y `Some(3).unwrap_or(7)`?

> **Respuesta**
>
> `7` y luego `3`: `unwrap_or` da el valor si existe, si no, el valor de
> reserva.

## En resumen

- `struct` agrupa campos con nombre; `impl` le añade métodos.
- `&self` lee el objeto, `&mut self` lo modifica; sin `self`, es una función
  asociada, como `Tipo::new()`.
- `enum` vale un valor entre una lista de variantes.
- `match` trata cada variante y obliga a no olvidar ninguna.
- `Option` vale `Some(valor)` o `None`: Rust no tiene `null`.
- `#[derive(Debug, Clone, PartialEq)]` da traits comunes a un tipo.

Compara con [los tipos en Ruby](../ruby/types.md) y [en Rythme](../rythme/types.md).

Siguiente: [la coincidencia de patrones](pattern-matching.md)
