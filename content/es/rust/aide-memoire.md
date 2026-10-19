# Chuleta

Esta página resume, en tablas y ejemplos cortos, todo lo que debes recordar de la documentación de Rust.

## Herramientas

| Comando | Efecto |
|----------|-------|
| `rustup update` | actualiza Rust |
| `cargo new nombre` | crea un proyecto |
| `cargo run` | compila y ejecuta |
| `cargo check` | comprueba sin compilar |
| `cargo test` | ejecuta las pruebas |
| `cargo add crate` | añade una dependencia |
| `cargo fmt` / `cargo clippy` | da formato / aconseja |

Detalles: [introducción](introduction.md) y [organizar un proyecto con Cargo](projet.md).

## Variables y tipos

| Escritura | Significado |
|----------|------|
| `let x = 5;` | variable no modificable |
| `let mut x = 5;` | variable modificable |
| `let x: f64 = 2.5;` | tipo explícito |
| `const MAX: u32 = 10;` | constante |
| `i32`, `u32`, `f64`, `bool`, `char` | tipos simples |
| `&str` / `String` | texto prestado / texto poseído |

```rust
let mut n = 1;
n += 1;
let n = n * 10; // sombreado
let s = format!("n = {n:>4}");
println!("{s} {:?}", "texto");
```

```sortie
n =   20 "texto"
```

Detalles: [primeros pasos](premiers-pas.md).

## Condiciones y bucles

```rust
let x = 7;
let paridad = if x % 2 == 0 { "par" } else { "impar" };
let tamano = match x {
    0 => "cero",
    1..=9 => "pequeño",
    _ => "grande",
};
println!("{paridad} {tamano}");
for i in 0..3 {
    print!("{i}");
}
let mut k = 0;
while k < 2 {
    k += 1;
}
let v = loop {
    break k * 10;
};
println!(" {v}");
```

```sortie
impar pequeño
012 20
```

Detalles: [condiciones y bucles](conditions-boucles.md).

## Funciones y closures

```rust
fn sumar(a: i32, b: i32) -> i32 {
    a + b // sin ; : es el valor devuelto
}

fn main() {
    let triple = |x: i32| x * 3;
    println!("{} {}", sumar(1, 2), triple(4));
}
```

```sortie
3 12
```

Detalles: [las funciones](fonctions.md).

## Texto

| Escritura | Significado |
|----------|------|
| `String::from("a")` / `"a".to_string()` | crea un `String` |
| `s.push_str("…")` / `s.push('c')` | añade texto / un carácter |
| `s.chars().count()` | número de caracteres (`len()`: bytes) |
| `s.trim()` / `s.to_uppercase()` | limpia / pasa a mayúsculas |
| `s.contains("x")` / `s.starts_with("x")` | busca |
| `s.replace("a", "b")` | reemplaza |
| `s.split(',').collect::<Vec<_>>()` | divide |
| `s.parse::<i32>()` | texto → número (`Result`) |

```rust
let s = String::from(" Otoño, invierno ");
let palabras: Vec<&str> = s.trim().split(", ").collect();
println!("{:?} {}", palabras, s.trim().chars().count());
println!("{}", "21".parse::<i32>().unwrap() * 2);
```

```sortie
["Otoño", "invierno"] 15
42
```

Detalles: [trabajar con texto](texte.md).

## Colecciones

| Escritura | Significado |
|----------|------|
| `vec![1, 2]` | crea un `Vec` |
| `v.push(x)` / `v.pop()` | añade / quita al final |
| `v.get(i)` | `Option` sobre el elemento `i` |
| `&v[1..3]` | porción (slice) |
| `HashMap::new()` | tabla clave-valor |
| `m.insert(k, v)` / `m.get(&k)` | añade / lee |
| `*m.entry(k).or_insert(0) += 1` | cuenta |
| `BTreeMap` | tabla ordenada por clave |

```rust
let v = vec![1, 2, 3, 4];
let cuadrados: Vec<i32> = v.iter().map(|x| x * x).collect();
let suma_pares: i32 = v.iter().filter(|x| *x % 2 == 0).sum();
println!("{:?} {suma_pares} {:?}", cuadrados, &v[..2]);
```

```sortie
[1, 4, 9, 16] 6 [1, 2]
```

Detalles: [colecciones](collections.md).

## Propiedad y préstamo

| Escritura | Significado |
|----------|------|
| `let b = a;` | mueve `a` (copia para los tipos simples) |
| `a.clone()` | copia independiente |
| `&a` | préstamo de lectura (varios posibles) |
| `&mut a` | préstamo de escritura (uno solo a la vez) |
| `fn f<'a>(x: &'a str) -> &'a str` | tiempo de vida explícito |

```rust
fn anadir_s(palabra: &mut String) {
    palabra.push('s');
}

fn main() {
    let mut gato = String::from("gato");
    let copia = gato.clone();
    anadir_s(&mut gato);
    println!("{copia} {gato}");
}
```

```sortie
gato gatos
```

Detalles: [propiedad y préstamo](possession.md).

## Estructuras, enumeraciones y traits

```rust
#[derive(Debug, Clone, PartialEq)]
struct Punto {
    x: i32,
    y: i32,
}

impl Punto {
    fn suma(&self) -> i32 {
        self.x + self.y
    }
}

enum Orden {
    Avanzar(i32),
    Parar,
}

trait Describir {
    fn describir(&self) -> String;
}

impl Describir for Punto {
    fn describir(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Punto { x: 2, y: 3 };
    println!("{:?} {} {}", p, p.suma(), p.describir());
    for o in [Orden::Avanzar(5), Orden::Parar] {
        match o {
            Orden::Avanzar(n) => println!("avanza {n}"),
            Orden::Parar => println!("parar"),
        }
    }
}
```

```sortie
Punto { x: 2, y: 3 } 5 (2, 3)
avanza 5
parar
```

Detalles: [estructuras, enumeraciones y traits](types.md).

## Genéricos y traits

| Escritura | Significado |
|----------|------|
| `fn f<T: Display>(x: T)` | función genérica con restricción |
| `fn f(x: impl Display)` | lo mismo, más corto |
| `trait T { fn m(&self); }` | define un trait |
| `impl T for Tipo { … }` | implementa un trait |
| `Vec<Box<dyn T>>` | lista de objetos de tipos distintos |
| `#[derive(Debug, Clone, PartialEq, Default)]` | traits escritos por ti |

```rust
use std::fmt::Display;

fn enmarcar(x: impl Display) -> String {
    format!("[{x}]")
}

fn main() {
    println!("{} {}", enmarcar(7), enmarcar("ok"));
}
```

```sortie
[7] [ok]
```

Detalles: [genéricos y traits](generiques.md).

## Option y Result

| Escritura | Significado |
|----------|------|
| `Some(v)` / `None` | valor presente / ausente |
| `Ok(v)` / `Err(e)` | éxito / fallo |
| `x?` | devuelve el error a quien llama |
| `x.unwrap()` / `x.expect("…")` | el valor, o `panic!` |
| `x.unwrap_or(v)` | el valor, o `v` |
| `if let Some(v) = x { … }` | actúa solo si está presente |

```rust
fn leer(t: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = t.parse()?;
    Ok(n * 2)
}

fn main() {
    println!("{:?}", leer("21"));
    println!("{}", leer("x").unwrap_or(0));
    if let Some(c) = "abc".chars().next() {
        println!("{c}");
    }
}
```

```sortie
Ok(42)
0
a
```

Detalles: [gestionar los errores](erreurs.md).

## Iteradores

| Escritura | Significado |
|----------|------|
| `v.iter()` / `v.iter_mut()` / `v.into_iter()` | leer / modificar / consumir |
| `map(f)` / `filter(f)` | transforma / conserva si es verdadero |
| `enumerate()` / `zip(otro)` | `(posición, x)` / pares |
| `rev()` / `take(n)` / `skip(n)` / `chain(otro)` | al revés / inicio / final / a continuación |
| `sum()` / `count()` / `min()` / `max()` | resume |
| `any(f)` / `all(f)` / `find(f)` / `position(f)` | hace una pregunta |
| `collect::<Vec<_>>()` | reúne (también `String`, `HashMap`) |
| `fold(inicio, \|acc, x\| …)` | acumula |
| `impl Iterator for T { type Item = …; fn next(&mut self) … }` | tu propio iterador |

```rust
let v = vec![3, 1, 4, 1, 5];
let grandes: Vec<i32> = v.iter().filter(|x| **x > 2).map(|x| x * 10).collect();
let total: i32 = v.iter().sum();
let producto = v.iter().fold(1, |acc, x| acc * x);
println!("{:?} {total} {producto}", grandes);
println!("{:?} {}", v.iter().position(|x| *x == 4), v.iter().any(|x| *x > 4));
```

```sortie
[30, 40, 50] 14 60
Some(2) true
```

> **Atención:** los iteradores son perezosos: `map` solo no hace nada sin
> `collect`, `sum`, `for`…

Detalles: [los iteradores](iterateurs.md).

## Módulos y crates

| Escritura | Significado |
|----------|------|
| `mod nombre { … }` | módulo en el mismo archivo |
| `mod nombre;` | módulo en `nombre.rs` |
| `pub fn` / `pub struct` / `pub campo` | hace público (privado por defecto) |
| `use a::b;` / `use a::{b, c};` | atajo |
| `crate::…` / `super::…` | desde la raíz / el padre |
| `src/main.rs` / `src/lib.rs` | programa / biblioteca |
| `cargo add nombre` | añade un crate de crates.io |

```rust
mod utiles {
    pub fn doble(n: i32) -> i32 {
        n * 2
    }

    pub mod texto {
        pub fn gritar(s: &str) -> String {
            format!("{}!", s.to_uppercase())
        }
    }
}

use utiles::{doble, texto};

fn main() {
    println!("{} {}", doble(4), texto::gritar("ok"));
}
```

```sortie
8 OK!
```

Detalles: [módulos y crates](modules.md).

## Teclado y archivos

| Escritura | Significado |
|----------|------|
| `io::stdin().read_line(&mut s)` | lee una línea del teclado |
| `env::args().collect::<Vec<String>>()` | argumentos del comando |
| `fs::write("f.txt", texto)?` | escribe (sobrescribe) un archivo |
| `fs::read_to_string("f.txt")?` | lee un archivo entero |
| `OpenOptions::new().append(true).open(…)?` | añade al final |
| `BufReader::new(archivo).lines()` | lee línea a línea |

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    fs::write("a.txt", "uno\ndos\n")?;
    for linea in fs::read_to_string("a.txt")?.lines() {
        println!("- {linea}");
    }
    Ok(())
}
```

```sortie
- uno
- dos
```

Detalles: [leer y escribir: teclado y archivos](fichiers.md).

## Box, Rc y RefCell

| Escritura | Significado |
|----------|------|
| `Box::new(x)` | valor en el montón, un solo propietario (tipos recursivos) |
| `Rc::new(x)` / `Rc::clone(&r)` | varios propietarios, solo lectura |
| `RefCell::new(x)` | `.borrow()` / `.borrow_mut()` comprobados en ejecución |
| `Rc<RefCell<T>>` | compartido y modificable (un solo hilo) |

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let caja = Box::new(5);
    let compartido = Rc::new(RefCell::new(vec![1]));
    let otro = Rc::clone(&compartido);
    otro.borrow_mut().push(2);
    println!("{} {:?} {}", *caja + 1, compartido.borrow(), Rc::strong_count(&compartido));
}
```

```sortie
6 [1, 2] 2
```

Detalles: [Box, Rc y RefCell](pointeurs.md).

## Hilos

| Escritura | Significado |
|----------|------|
| `thread::spawn(move \|\| …)` | lanza un hilo |
| `manejador.join().unwrap()` | espera a que termine, recupera su resultado |
| `let (tx, rx) = mpsc::channel();` | canal: `tx.send(x)`, `rx.recv()` |
| `Arc<Mutex<T>>` | valor compartido entre hilos: `*m.lock().unwrap() += 1` |
| `thread::scope(\|s\| { s.spawn(…); })` | hilos que toman prestado |

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let contador = Arc::new(Mutex::new(0));
    let manejadores: Vec<_> = (0..4)
        .map(|_| {
            let c = Arc::clone(&contador);
            thread::spawn(move || *c.lock().unwrap() += 1)
        })
        .collect();
    for m in manejadores {
        m.join().unwrap();
    }
    println!("{}", *contador.lock().unwrap());
}
```

```sortie
4
```

Detalles: [hacer varias cosas a la vez](concurrence.md).

## Pruebas

| Escritura | Significado |
|----------|------|
| `assert!(cond)` / `assert_eq!(a, b)` / `assert_ne!(a, b)` | comprueba, si no `panic!` |
| `#[test]` | marca una función de prueba |
| `#[cfg(test)] mod tests { use super::*; … }` | módulo de pruebas |
| `#[should_panic]` | la prueba debe provocar un `panic!` |
| `cargo test` / `cargo test nombre` | ejecuta todas las pruebas / algunas |
| `tests/*.rs` | pruebas de integración |

```rust
fn doble(n: i32) -> i32 {
    n * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doble_cuatro() {
        assert_eq!(doble(4), 8);
    }
}

fn main() {
    assert_eq!(doble(4), 8);
    println!("ok");
}
```

```sortie
ok
```

Detalles: [probar tu código](tests.md).

> **Consejo:** las chuletas de los otros lenguajes: [Ruby](../ruby/aide-memoire.md) y
> [Rythme](../rythme/aide-memoire.md).
