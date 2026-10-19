# Gestionar los errores

Esta página explica los dos tipos de errores de Rust: los errores fatales con `panic!`, y los errores previstos con `Result` y el operador `?`.

> **Objetivos:**
>
> - distinguir un error fatal (`panic!`) de un error previsto (`Result`);
> - devolver un `Result` y tratarlo con `match`;
> - pasar un error a quien llama con `?`;
> - elegir entre `unwrap`, `expect` y un valor por defecto;
> - crear tu propio tipo de error.

## Dos tipos de errores

Un [error](../debuter/glossaire.md#error) es cuando el programa no puede hacer lo que
se le pide. Rust distingue dos tipos, como con un coche: una avería del motor te detiene
en seco, un atasco solo te obliga a cambiar de ruta.

- Un error **irrecuperable**: un bug, una situación imposible. El programa se detiene
  con `panic!`.
- Un error **recuperable**: un archivo que falta, un texto mal escrito… La función
  devuelve un `Result` y quien llama decide qué hacer.

## Detener el programa con panic!

`panic!` detiene el programa y muestra un mensaje. Leer la casilla 10 de una lista de 3
elementos, por ejemplo, provoca un `panic!`:

```rust norun
fn main() {
    let v = vec![1, 2, 3];
    println!("{}", v[10]);
}
```

```text
thread 'main' panicked at src/main.rs:3:21:
index out of bounds: the len is 3 but the index is 10
```

También puedes provocar `panic!` tú mismo:

```rust norun
panic!("Esto nunca debería pasar");
```

> **Atención:** reserva `panic!` para los bugs de verdad. Para un error normal, usa
> `Result`.

## El tipo Result

`Result<T, E>` es una [enumeración](types.md) con dos variantes. `T` es el tipo del
valor en caso de éxito, `E` el del error. Un `Result` es o bien:

- `Ok(valor)`: todo ha ido bien;
- `Err(error)`: algo ha fallado.

```rust
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("división por cero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("{:?}", dividir(10.0, 4.0));
    println!("{:?}", dividir(1.0, 0.0));
}
```

```sortie
Ok(2.5)
Err("división por cero")
```

Piensa en el sobre con el resultado de un examen: contiene o bien el diploma, o bien una
carta que explica el suspenso. Tienes que abrirlo para saberlo.

## Tratar un Result con match

`match` abre el `Result` y trata los dos casos:

```rust
fn main() {
    let pruebas = ["42", "abc"];
    for texto in pruebas {
        match texto.parse::<i32>() {
            Ok(n) => println!("{texto} → el número {n}"),
            Err(e) => println!("{texto} → error: {e}"),
        }
    }
}
```

```sortie
42 → el número 42
abc → error: invalid digit found in string
```

## Propagar con ?

El operador `?` simplifica mucho el código. **Pasa** el error a la función que ha
llamado, como un empleado que devuelve un expediente a su jefe en lugar de tratarlo.
Colocado después de un `Result`:

- si es `Ok(v)`, da `v`;
- si es `Err(e)`, la función se detiene y devuelve `Err(e)`.

```rust
use std::num::ParseIntError;

fn suma(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    println!("{:?}", suma("2", "40"));
    println!("{}", suma("2", "cuarenta").is_err());
}
```

```sortie
Ok(42)
true
```

> **Recuerda:** `?` solo se usa en una función que a su vez devuelve un `Result` (o una
> `Option`).

`ParseIntError` es el tipo de error que devuelve `parse` para un entero. `main` también
puede devolver un `Result`. `Ok(())` significa «éxito, sin valor que dar»:

```rust
fn main() -> Result<(), std::num::ParseIntError> {
    let n: i32 = "7".parse()?;
    println!("{}", n * 6);
    Ok(())
}
```

```sortie
42
```

## unwrap y expect

`unwrap()` saca el valor de un `Ok` (o de un `Some`), y provoca un `panic!` si no lo hay.
`expect()` hace lo mismo, con tu propio mensaje:

```rust
let n: i32 = "12".parse().unwrap();
let m: i32 = "30".parse().expect("se esperaba un número");
println!("{}", n + m);
```

```sortie
42
```

Si falta el valor, el programa se detiene con tu mensaje:

```rust norun
fn main() {
    let n: i32 = "doce".parse().expect("se esperaba un número");
}
```

```text
thread 'main' panicked at src/main.rs:2:33:
se esperaba un número: ParseIntError { kind: InvalidDigit }
```

> **Consejo:** `unwrap` es práctico en las pruebas y los ejemplos. En un programa real,
> prefiere `match`, `?` o un valor por defecto.

## Valores por defecto y combinadores

Algunos métodos cortos evitan escribir un `match` completo. Funcionan con `Option` y con
`Result`:

```rust
let vacio: Option<i32> = None;
println!("{}", vacio.unwrap_or(0));
println!("{:?}", Some(5).map(|n| n * 2));
println!("{}", "abc".parse::<i32>().unwrap_or(-1));
```

```sortie
0
Some(10)
-1
```

| Método | Efecto |
|---------|-------|
| `unwrap_or(v)` | el valor, o `v` si falta |
| `map(f)` | transforma el valor si existe |
| `is_some()` / `is_ok()` | comprueba la presencia o el éxito |

## Tu propio tipo de error

Para un programa más grande, crea una [enumeración](types.md) que enumere los errores
posibles. Luego `match` te permite reaccionar a cada uno:

```rust
enum ErrorCuenta {
    SaldoInsuficiente,
    MontoCero,
}

fn retirar(saldo: u32, monto: u32) -> Result<u32, ErrorCuenta> {
    if monto == 0 {
        return Err(ErrorCuenta::MontoCero);
    }
    if monto > saldo {
        return Err(ErrorCuenta::SaldoInsuficiente);
    }
    Ok(saldo - monto)
}

fn main() {
    for monto in [30, 0, 80] {
        match retirar(50, monto) {
            Ok(resto) => println!("Retiro OK, quedan {resto} €"),
            Err(ErrorCuenta::MontoCero) => println!("Error: el monto debe ser positivo"),
            Err(ErrorCuenta::SaldoInsuficiente) => println!("Error: saldo insuficiente"),
        }
    }
}
```

```sortie
Retiro OK, quedan 20 €
Error: el monto debe ser positivo
Error: saldo insuficiente
```

## Errores frecuentes

### Usar ? en una función que no devuelve un Result

```rust norun
fn main() {
    let n: i32 = "7".parse()?;
    println!("{n}");
}
```

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:2:29
  |
1 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
2 |     let n: i32 = "7".parse()?;
  |                             ^ cannot use the `?` operator in a function that returns `()`
```

`?` tiene que poder devolver el error. Haz que la función devuelva un `Result`:
`fn main() -> Result<(), std::num::ParseIntError>`, con `Ok(())` al final.

### Tomar un Result por su valor

```rust norun
fn main() {
    let n: i32 = "42".parse::<i32>();
    println!("{n}");
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:2:18
  |
2 |     let n: i32 = "42".parse::<i32>();
  |            ---   ^^^^^^^^^^^^^^^^^^^ expected `i32`, found `Result<i32, ParseIntError>`
  |            |
  |            expected due to this
```

`parse` devuelve el sobre, no el número. Ábrelo: `match`, `?`, `unwrap_or(0)` o
`expect("…")`.

### Olvidar el caso Err

```rust norun
fn main() {
    match "42".parse::<i32>() {
        Ok(n) => println!("{n}"),
    }
}
```

```text
error[E0004]: non-exhaustive patterns: `Err(_)` not covered
 --> src/main.rs:2:11
  |
2 |     match "42".parse::<i32>() {
  |           ^^^^^^^^^^^^^^^^^^^ pattern `Err(_)` not covered
```

Rust te obliga a prever el fallo. Añade `Err(e) => println!("error: {e}"),`.

## Ponte a prueba

**1.** ¿Cuándo usar `panic!` en lugar de `Result`?

> **Respuesta**
>
> Para un bug, una situación que nunca debería pasar. Un error previsible (archivo que
> falta, entrada no válida) se trata con `Result`.

**2.** ¿Qué hace `?` después de un `Err(e)`?

> **Respuesta**
>
> Detiene la función enseguida y devuelve `Err(e)` a quien la llamó.

**3.** Lee una edad del texto `"abc"`, con `0` si la lectura falla.

> **Respuesta**
>
> ```rust
> let edad: u32 = "abc".parse().unwrap_or(0);
> println!("{edad}");
> ```
>
> ```sortie
> 0
> ```

## En resumen

- `panic!` detiene el programa: resérvalo para los bugs de verdad.
- `Result` es `Ok(valor)` o `Err(error)`.
- `match` trata los dos casos; `unwrap_or` da un valor por defecto.
- `?` devuelve el error a quien llama, en una función que devuelve un `Result`.
- `unwrap` y `expect` son prácticos para las pruebas, arriesgados en otros sitios.

Compara con [los errores en Ruby](../ruby/erreurs.md) y
[en Rythme](../rythme/erreurs.md).

Siguiente: [Las closures en detalle](closures.md)
