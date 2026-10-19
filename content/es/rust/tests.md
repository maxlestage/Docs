# Probar tu código

Una prueba es un pequeño programa que comprueba que tu código hace lo que debe: Rust y Cargo las escriben y las ejecutan de forma muy sencilla.

> **Objetivos:**
>
> - comprobar un valor con `assert!`, `assert_eq!` y `assert_ne!`;
> - escribir una prueba con `#[test]` y ejecutarla con `cargo test`;
> - colocar las pruebas en un módulo `#[cfg(test)]`;
> - probar que una función se detiene de verdad con `#[should_panic]`;
> - escribir pruebas en la carpeta `tests/` y en la documentación.

## ¿Por qué probar?

Cuando modificas un programa, te arriesgas a romper lo que funcionaba. Comprobarlo todo
a mano, cada vez, es largo y se olvidan casos.

Una prueba hace esta comprobación **automáticamente**, con un solo comando. Es como una
lista de control antes del despegue de un avión: rápida, y tranquilizadora.

## Las macros de comprobación

Tres macros comprueban una condición. Si es falsa, provocan un `panic!` con un mensaje
claro.

| Macro | Comprueba que… |
|-------|----------------|
| `assert!(condición)` | la condición es verdadera |
| `assert_eq!(a, b)` | `a` es igual a `b` |
| `assert_ne!(a, b)` | `a` es distinto de `b` |

```rust
let total = 2 + 2;
assert!(total > 3);
assert_eq!(total, 4);
assert_ne!(total, 5);
println!("Todas las comprobaciones pasan");
```

```sortie
Todas las comprobaciones pasan
```

Puedes añadir un mensaje, escrito como en `println!`. Solo se muestra si la
comprobación falla:

```rust
let edad = 15;
assert!(edad >= 0, "la edad no puede ser negativa: {edad}");
println!("ok");
```

```sortie
ok
```

Cuando una comprobación falla, el programa se detiene y muestra los dos valores:

```rust norun
assert_eq!(2 + 2, 5);
```

```text
assertion `left == right` failed
  left: 4
 right: 5
```

> **Nota:** `left` es el primer valor, `right` el segundo. Por costumbre, se pone el
> resultado calculado a la izquierda y el valor esperado a la derecha.

## Escribir una prueba con #[test]

Una prueba es una función sin parámetros, precedida de `#[test]`. La prueba **pasa** si
llega hasta el final, y **falla** si provoca un `panic!` (por ejemplo con una macro
`assert`).

Las pruebas se colocan en un módulo `tests`, al final del archivo:

```rust
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suma_dos_numeros() {
        assert_eq!(sumar(2, 3), 5);
    }

    #[test]
    fn suma_un_negativo() {
        assert_eq!(sumar(2, -5), -3);
    }
}

fn main() {
    println!("{}", sumar(2, 3));
}
```

```sortie
5
```

Línea por línea:

- `#[cfg(test)]`: este módulo solo se compila para las pruebas. No hace más pesado tu
  programa final;
- `mod tests`: un [módulo](modules.md) como cualquier otro;
- `use super::*;`: importa todo lo que hay en el módulo padre (aquí, `sumar`);
- `#[test]`: marca cada función de prueba.

> **Consejo:** dales a tus pruebas nombres que describan lo que comprueban:
> `suma_un_negativo` dice más que `test2`.

## Ejecutar las pruebas: cargo test

En la carpeta del proyecto, escribe:

```sh
cargo test
```

Cargo compila el proyecto en modo prueba, ejecuta cada función `#[test]` y muestra un
resumen:

```text
running 2 tests
test tests::suma_dos_numeros ... ok
test tests::suma_un_negativo ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Si una prueba falla, Cargo indica cuál, con los dos valores comparados. Esto es lo que
se muestra si escribes por error `assert_eq!(sumar(2, -5), 3);`:

```text
running 2 tests
test tests::suma_dos_numeros ... ok
test tests::suma_un_negativo ... FAILED

failures:

---- tests::suma_un_negativo stdout ----

thread 'tests::suma_un_negativo' panicked at src/main.rs:16:9:
assertion `left == right` failed
  left: -3
 right: 3

failures:
    tests::suma_un_negativo

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Para ejecutar solo algunas pruebas, da un trozo de su nombre:

```sh
cargo test negativo   # ejecuta solo las pruebas cuyo nombre contiene «negativo»
```

> **Nota:** las pruebas se ejecutan en paralelo, así que el orden en que se muestran
> puede cambiar. Y lo que tus pruebas muestran con `println!` queda oculto, salvo si
> fallan.

## Probar una parada: #[should_panic]

A veces, el comportamiento correcto es **detenerse**. `#[should_panic]` indica que la
prueba debe provocar un `panic!` para pasar. Con `expected`, también se comprueba el
mensaje:

```rust
fn dividir(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("división por cero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_normalmente() {
        assert_eq!(dividir(10, 2), 5);
    }

    #[test]
    #[should_panic(expected = "división por cero")]
    fn rechaza_cero() {
        dividir(1, 0);
    }
}

fn main() {
    println!("{}", dividir(10, 2));
}
```

```sortie
5
```

En el resumen, esta prueba aparece así:

```text
test tests::rechaza_cero - should panic ... ok
```

## ¿Qué probar?

Para cada función, piensa en tres tipos de casos:

- un caso **normal**: `sumar(2, 3)`;
- un caso **límite**: una lista vacía, cero, un texto vacío;
- un caso **de error**: una entrada no válida debe dar un `Err` (o un `panic!`).

```rust
fn promedio(notas: &[f64]) -> Option<f64> {
    if notas.is_empty() {
        return None;
    }
    Some(notas.iter().sum::<f64>() / notas.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promedio_normal() {
        assert_eq!(promedio(&[10.0, 20.0]), Some(15.0));
    }

    #[test]
    fn promedio_de_una_lista_vacia() {
        assert_eq!(promedio(&[]), None);
    }

    #[test]
    fn analisis_de_un_texto_no_valido() {
        assert!("doce".parse::<i32>().is_err());
    }
}

fn main() {
    println!("{:?} {:?}", promedio(&[10.0, 20.0]), promedio(&[]));
}
```

```sortie
Some(15.0) None
```

> **Atención:** para comparar dos números decimales calculados, la igualdad exacta puede
> fallar (`0.1 + 0.2` no vale exactamente `0.3`). Comprueba más bien que la diferencia
> es muy pequeña: `assert!((x - 0.3).abs() < 1e-9);`.

## Las pruebas de integración: la carpeta tests/

Las pruebas del módulo `tests` comprueban cada función desde dentro. Las **pruebas de
integración** usan tu código desde fuera, como lo haría otro programa. Necesitan una
biblioteca (`src/lib.rs`, consulta [Módulos y crates](modules.md)):

```text
calculo/
├── Cargo.toml
├── src/
│   └── lib.rs           ← pub fn sumar(…)
└── tests/
    └── calculos.rs      ← las pruebas de integración
```

Cada archivo de `tests/` se compila aparte. La biblioteca se importa por el nombre del
proyecto, y no hace falta `#[cfg(test)]`:

```rust norun
// tests/calculos.rs
use calculo::sumar;

#[test]
fn suma_desde_fuera() {
    assert_eq!(sumar(10, 32), 42);
}
```

`cargo test` ejecuta entonces los dos tipos de pruebas, uno después del otro:

```text
     Running unittests src/lib.rs
running 3 tests
...
     Running tests/calculos.rs
running 1 test
test suma_desde_fuera ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Las pruebas en la documentación

Los comentarios `///` documentan una función. Un ejemplo de código escrito dentro, entre
tres acentos graves, es **también una prueba**: `cargo test` lo compila y lo ejecuta. Así
tu documentación siempre sigue siendo correcta.

````rust norun
/// Suma dos números.
///
/// ```
/// let total = calculo::sumar(2, 3);
/// assert_eq!(total, 5);
/// ```
pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}
````

En el resumen de `cargo test`, estas pruebas aparecen al final:

```text
   Doc-tests calculo

running 1 test
test src/lib.rs - sumar (line 3) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

> **Nota:** las pruebas de documentación solo funcionan en una biblioteca
> (`src/lib.rs`).

## Errores frecuentes

### Olvidar use super::*

```rust norun
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    #[test]
    fn suma() {
        assert_eq!(sumar(2, 3), 5);
    }
}
```

```text
error[E0425]: cannot find function `sumar` in this scope
help: consider importing this function
  |
7 +     use crate::sumar;
```

El módulo `tests` es un cajón aparte: no ve las funciones del padre. Añade
`use super::*;` al principio del módulo.

### Comparar estructuras sin PartialEq ni Debug

```rust norun
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let a = Punto { x: 1, y: 2 };
    assert_eq!(a, Punto { x: 1, y: 2 });
}
```

```text
error[E0369]: binary operation `==` cannot be applied to type `Punto`
help: consider annotating `Punto` with `#[derive(PartialEq)]`

error[E0277]: `Punto` doesn't implement `Debug`
help: consider annotating `Punto` with `#[derive(Debug)]`
```

`assert_eq!` necesita comparar (`PartialEq`) y mostrar (`Debug`) los valores. Añade los
dos de una vez:

```rust
#[derive(Debug, PartialEq)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let a = Punto { x: 1, y: 2 };
    assert_eq!(a, Punto { x: 1, y: 2 });
    println!("iguales");
}
```

```sortie
iguales
```

### Una prueba should_panic que no se detiene

Si la función no provoca un `panic!`, la prueba `#[should_panic]` falla:

```text
test tests::rechaza_cero - should panic ... FAILED

---- tests::rechaza_cero stdout ----
note: test did not panic as expected
```

Comprueba que llamas a la función con un valor que debe detenerla (aquí,
`dividir(1, 0)` y no `dividir(4, 2)`).

## Ponte a prueba

1. ¿Cuál es la diferencia entre `assert!` y `assert_eq!`?

> **Respuesta**
>
> `assert!` comprueba que una condición es verdadera. `assert_eq!` comprueba que dos
> valores son iguales, y muestra los dos (`left` y `right`) si falla.

2. ¿Para qué sirve `#[cfg(test)]` delante de `mod tests`?

> **Respuesta**
>
> El módulo solo se compila para `cargo test`. No se incluye en el programa final.

3. Una prueba debe comprobar que una función se detiene con `panic!`. ¿Qué hay que añadir?

> **Respuesta**
>
> El atributo `#[should_panic]` debajo de `#[test]` (con `expected = "…"` para
> comprobar el mensaje).

## En resumen

- `assert!`, `assert_eq!` y `assert_ne!` comprueban una condición; si no, `panic!`.
- Una prueba es una función `#[test]`, colocada en un módulo `#[cfg(test)] mod tests`
  con `use super::*;`.
- `cargo test` ejecuta todas las pruebas y muestra un resumen.
- `#[should_panic]` prueba que una función se detiene de verdad.
- La carpeta `tests/` contiene las pruebas de integración; los ejemplos de los
  comentarios `///` también son pruebas.

Compara con [las pruebas en Ruby](../ruby/tests.md) y
[en Rythme](../rythme/tests.md). Siguiente:
[Organizar un proyecto con Cargo](projet.md).

Siguiente: [Depurar en Rust](debogage.md)
