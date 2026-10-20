# Genéricos y traits

Esta página muestra cómo escribir una sola función para varios tipos gracias a los genéricos, y cómo describir un comportamiento común con los traits.

> **Objetivos:**
>
> - escribir una función y una estructura genéricas con `<T>`;
> - limitar un tipo genérico con una restricción (`T: Display`);
> - definir un trait e implementarlo para varios tipos;
> - guardar objetos de tipos diferentes con `Box<dyn Trait>`;
> - usar los traits habituales: `Debug`, `Clone`, `Display`, `From`.

## El problema: código repetido

Imagina dos funciones que hacen lo mismo, para dos tipos diferentes:

```rust
fn mayor_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn mayor_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", mayor_i32(3, 8));
    println!("{}", mayor_f64(2.5, 1.5));
}
```

```sortie
8
2.5
```

El código es idéntico, solo cambia el tipo. Los **genéricos** evitan esta repetición.

## Una función genérica

Una función genérica es como un molde para pasteles: la forma es siempre la misma, pero
eliges la masa cada vez. Se sustituye el tipo por una letra, a menudo `T` (de *tipo*),
escrita entre `<` y `>` después del nombre:

```rust
fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", mayor(3, 8));
    println!("{}", mayor(2.5, 1.5));
    println!("{}", mayor("pera", "kiwi"));
}
```

```sortie
8
2.5
pera
```

Lee `<T: PartialOrd>` así: «para cualquier tipo `T` que sepa compararse».

> **Recuerda:** `: PartialOrd` es una **restricción**. Sin ella, Rust rechaza `a > b`,
> porque no sabe si `T` se puede comparar.

## Varias restricciones

Una restricción es un **trait**: una capacidad que el tipo debe tener. Una oferta de
empleo funciona igual: «cualquier candidato que sepa conducir **y** hable inglés». Para
pedir varios traits, únelos con `+`. Aquí, `Display` permite mostrar con `{}`:

```rust
use std::fmt::Display;

fn mostrar_dos_veces<T: Display + Clone>(x: T) {
    let copia = x.clone();
    println!("{x} y {copia}");
}

fn main() {
    mostrar_dos_veces(7);
    mostrar_dos_veces("ok");
}
```

```sortie
7 y 7
ok y ok
```

## impl Trait como parámetro

Existe una forma más corta de escribirlo: `impl Trait` en lugar del tipo. Se lee
«cualquier tipo que tenga este trait»:

```rust
use std::fmt::Display;

fn enmarcar(x: impl Display) -> String {
    format!("[ {x} ]")
}

fn main() {
    println!("{}", enmarcar(42));
    println!("{}", enmarcar("hola"));
    println!("{}", enmarcar(3.5));
}
```

```sortie
[ 42 ]
[ hola ]
[ 3.5 ]
```

> **Consejo:** para empezar, `impl Trait` suele ser la forma más legible.

## Una estructura genérica

Una estructura también puede tener un tipo genérico. `Vec<T>` y `Option<T>` están
hechos así. Aquí tienes una caja que contiene un valor de cualquier tipo:

```rust
#[derive(Debug)]
struct Caja<T> {
    contenido: T,
}

impl<T> Caja<T> {
    fn abrir(self) -> T {
        self.contenido
    }
}

fn main() {
    let a = Caja { contenido: 42 };
    let b = Caja { contenido: "secreto" };
    println!("{:?}", a);
    println!("{}", b.abrir());
}
```

```sortie
Caja { contenido: 42 }
secreto
```

`impl<T> Caja<T>` se lee: «para cualquier tipo `T`, estos son los métodos de
`Caja<T>`».

## Definir tu propio trait

Un trait describe un comportamiento: una lista de métodos que un tipo promete tener. Es
un contrato: «todo animal sabe decir su nombre y hacer su sonido». Se escribe con
`trait`, y luego cada tipo lo firma con `impl Trait for Tipo`. Un método también puede
tener un comportamiento **por defecto**:

```rust
trait Animal {
    fn nombre(&self) -> String;
    fn sonido(&self) -> String;

    // Método por defecto: los tipos no necesitan escribirlo.
    fn presentar(&self) -> String {
        format!("{} hace {}", self.nombre(), self.sonido())
    }
}

struct Perro;

impl Animal for Perro {
    fn nombre(&self) -> String {
        String::from("El perro")
    }
    fn sonido(&self) -> String {
        String::from("guau")
    }
}

fn main() {
    let rex = Perro;
    println!("{}", rex.presentar());
}
```

```sortie
El perro hace guau
```

`struct Perro;` es una estructura sin campos: no contiene nada, pero puede tener
métodos.

## Implementar un trait para dos estructuras

Cada estructura escribe su propia versión. Una función que recibe `&impl Animal`
acepta las dos:

```rust
trait Animal {
    fn nombre(&self) -> String;
    fn sonido(&self) -> String;
    fn presentar(&self) -> String {
        format!("{} hace {}", self.nombre(), self.sonido())
    }
}

struct Perro;
struct Gato {
    nombre_propio: String,
}

impl Animal for Perro {
    fn nombre(&self) -> String {
        String::from("El perro")
    }
    fn sonido(&self) -> String {
        String::from("guau")
    }
}

impl Animal for Gato {
    fn nombre(&self) -> String {
        self.nombre_propio.clone()
    }
    fn sonido(&self) -> String {
        String::from("miau")
    }
    // El gato sustituye el método por defecto.
    fn presentar(&self) -> String {
        format!("{} dice {} (bostezando)", self.nombre(), self.sonido())
    }
}

fn hacer_hablar(a: &impl Animal) {
    println!("{}", a.presentar());
}

fn main() {
    hacer_hablar(&Perro);
    hacer_hablar(&Gato { nombre_propio: String::from("Michi") });
}
```

```sortie
El perro hace guau
Michi dice miau (bostezando)
```

## Una lista de objetos diferentes

Un `Vec` solo contiene un tipo. Para guardar un perro **y** un gato en la misma lista,
se usa `Box<dyn Animal>`: «una caja que contiene un animal, da igual cuál».

```rust
trait Animal {
    fn sonido(&self) -> String;
}

struct Perro;
struct Gato;

impl Animal for Perro {
    fn sonido(&self) -> String {
        String::from("guau")
    }
}

impl Animal for Gato {
    fn sonido(&self) -> String {
        String::from("miau")
    }
}

fn main() {
    let animales: Vec<Box<dyn Animal>> = vec![Box::new(Perro), Box::new(Gato), Box::new(Perro)];
    for a in &animales {
        println!("{}", a.sonido());
    }
}
```

```sortie
guau
miau
guau
```

> **Nota:** `Box::new(x)` guarda `x` en una caja. La palabra `dyn` significa que el tipo
> real solo se conocerá durante la ejecución.

## Los traits habituales de la biblioteca

Rust ofrece traits ya hechos. Muchos se obtienen gratis con `#[derive(...)]`:

| Trait | Qué permite | ¿derive? |
|-------|-------------|----------|
| `Debug` | mostrar con `{:?}` | sí |
| `Clone` | copiar con `.clone()` | sí |
| `PartialEq` | comparar con `==` | sí |
| `Default` | crear un valor por defecto | sí |
| `Display` | mostrar con `{}` | no, hay que escribirlo |
| `From` | convertir desde otro tipo | no, hay que escribirlo |

### Debug, Clone, PartialEq y Default

```rust
#[derive(Debug, Clone, PartialEq, Default)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let a = Punto { x: 1, y: 2 };
    let b = a.clone();
    let cero = Punto::default();
    println!("{:?}", a);
    println!("{}", a == b);
    println!("{:?}", cero);
}
```

```sortie
Punto { x: 1, y: 2 }
true
Punto { x: 0, y: 0 }
```

### Display: una visualización personalizada

Para mostrar un tipo con `{}`, escribe tú mismo el trait `Display`. Copia este modelo,
solo cambia la línea `write!`:

```rust
use std::fmt;

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
    println!("Hace {t}");
}
```

```sortie
Hace 21.5 °C
```

### From: convertir un tipo en otro

`From` explica cómo fabricar tu tipo a partir de otro. Así obtienes `.into()` de regalo:

```rust
#[derive(Debug)]
struct Euros(f64);

impl From<f64> for Euros {
    fn from(importe: f64) -> Euros {
        Euros(importe)
    }
}

fn main() {
    let a = Euros::from(12.5);
    let precio: f64 = 3.0;
    let b: Euros = precio.into();
    println!("{:?} {:?}", a, b);
}
```

```sortie
Euros(12.5) Euros(3.0)
```

> **Consejo:** usas `From` desde el principio sin saberlo: `String::from("texto")`.

## Errores frecuentes

### Olvidar la restricción

```rust norun
fn mayor<T>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", mayor(3, 8));
}
```

```text
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:2:10
  |
2 |     if a > b { a } else { b }
  |        - ^ - T
  |        |
  |        T
help: consider restricting type parameter `T` with trait `PartialOrd`
```

Rust no sabe si cualquier `T` se puede comparar. Escribe `<T: PartialOrd>`.

### Olvidar un método del trait

```rust norun
trait Animal {
    fn nombre(&self) -> String;
    fn sonido(&self) -> String;
}

struct Perro;

impl Animal for Perro {
    fn nombre(&self) -> String {
        String::from("Rex")
    }
}

fn main() {}
```

```text
error[E0046]: not all trait items implemented, missing: `sonido`
 --> src/main.rs:8:1
  |
3 |     fn sonido(&self) -> String;
  |     --------------------------- `sonido` from trait
...
8 | impl Animal for Perro {
  | ^^^^^^^^^^^^^^^^^^^^^ missing `sonido` in implementation
```

Firmar el contrato obliga a escribir todos los métodos que no tienen versión por
defecto. Añade `fn sonido(&self) -> String { … }`.

### Poner dos tipos en el mismo Vec

```rust norun
struct Perro;
struct Gato;

fn main() {
    let animales = vec![Perro, Gato];
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:5:32
  |
5 |     let animales = vec![Perro, Gato];
  |                                ^^^^ expected `Perro`, found `Gato`
```

Dales un trait común y guarda cajas: `Vec<Box<dyn Animal>>` (consulta
[Una lista de objetos diferentes](#una-lista-de-objetos-diferentes)).

### Mostrar con {} sin Display

```rust norun
struct Euros(f64);

fn main() {
    let precio = Euros(12.5);
    println!("{precio}");
}
```

```text
error[E0277]: `Euros` doesn't implement `std::fmt::Display`
 --> src/main.rs:5:15
  |
5 |     println!("{precio}");
  |               ^^^^^^^^ `Euros` cannot be formatted with the default formatter
  |
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

Escribe `impl fmt::Display for Euros` (consulta
[Display](#display-una-visualizacion-personalizada)), o añade `#[derive(Debug)]` y
muestra con `{:?}`.

## Ponte a prueba

**1.** ¿Qué significa `fn f<T: Clone>(x: T)`?

> **Respuesta**
>
> `f` acepta un valor de cualquier tipo `T`, siempre que ese tipo sepa copiarse con
> `.clone()`.

**2.** Escribe un trait `Area` con un método `area(&self) -> f64`, e impleméntalo para
una estructura `Cuadrado { lado: f64 }`.

> **Respuesta**
>
> ```rust
> trait Area {
>     fn area(&self) -> f64;
> }
>
> struct Cuadrado {
>     lado: f64,
> }
>
> impl Area for Cuadrado {
>     fn area(&self) -> f64 {
>         self.lado * self.lado
>     }
> }
>
> fn main() {
>     println!("{}", Cuadrado { lado: 3.0 }.area());
> }
> ```
>
> ```sortie
> 9
> ```

**3.** ¿Por qué escribir `Box<dyn Animal>` en lugar de `Perro` en un `Vec`?

> **Respuesta**
>
> Para guardar animales de tipos diferentes en la misma lista. `dyn Animal` quiere
> decir «cualquier tipo que tenga el trait `Animal`».

## En resumen

- `<T>` hace genérica una función o una estructura: funciona para varios tipos.
- Una restricción como `T: Display` exige una capacidad al tipo.
- `impl Trait` como parámetro es una forma corta y legible.
- Un trait enumera métodos; puede ofrecer métodos por defecto.
- `Vec<Box<dyn Trait>>` guarda objetos de tipos diferentes.
- `#[derive(Debug, Clone, PartialEq, Default)]` escribe traits por ti.

Los tipos básicos están en [Estructuras, enumeraciones y traits](types.md).

Siguiente: [los objetos trait: dyn Trait](traits-objets.md)
