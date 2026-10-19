# Rust: introducción e instalación

Rust es un lenguaje rápido y seguro: esta página explica lo que aporta, cómo instalarlo y cómo ejecutar tu primer programa.

> **Objetivos:**
>
> - saber qué es un lenguaje compilado y qué aporta Rust;
> - instalar Rust con `rustup`;
> - crear y ejecutar un proyecto con `cargo`;
> - leer un pequeño programa en Rust y modificarlo;
> - entender un primer mensaje de error del compilador.

## ¿Qué es Rust?

Rust es un lenguaje de programación **compilado**. Un
[compilador](../debuter/glossaire.md#compilador) es un programa que traduce tu
código a un archivo ejecutable, que el ordenador ejecuta directamente. La traducción se hace
una sola vez, antes de la ejecución: el programa obtenido es muy rápido.

Piensa en un libro traducido de antemano: el lector extranjero lo lee de un tirón, sin
esperar a un intérprete frase por frase.

Rust tiene tres grandes cualidades:

- **La velocidad**: un programa en Rust es tan rápido como un programa en C o C++.
- **La seguridad**: el compilador rechaza muchos errores incluso antes de la ejecución
  (memoria mal usada, valor ausente olvidado…).
- **Las herramientas**: `cargo` crea, compila, prueba y formatea tus proyectos.

Aquí tienes un programa en Rust muy pequeño:

```rust
fn main() {
    println!("¡Hola, Rust!");
}
```

```sortie
¡Hola, Rust!
```

> **Nota:** Rust es exigente al principio. El compilador se queja a menudo, pero sus
> mensajes de error son claros e incluso proponen correcciones.

## Instalar Rust

La herramienta oficial se llama `rustup`. Instala dos programas:

- `rustc`, el compilador;
- `cargo`, la herramienta que gestiona tus proyectos (llama a `rustc` por ti).

### En Linux y macOS

Abre una terminal y escribe el comando de abajo. Si no conoces la terminal,
lee primero [Usar la terminal](../debuter/terminal.md).

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Sigue las instrucciones y luego abre una terminal nueva.

### En Windows

Descarga `rustup-init.exe` desde el sitio [rustup.rs](https://rustup.rs) y ejecútalo.
El instalador puede pedirte que instales también las herramientas de compilación de
Visual Studio: acepta.

### Comprobar la instalación

```sh
rustc --version
cargo --version
```

Cada comando muestra un número de versión, por ejemplo `rustc 1.97.0`.

> **Consejo:** para pasar más adelante a la última versión de Rust, escribe
> `rustup update`.

## Crear un proyecto con cargo

`cargo new` crea una carpeta de proyecto lista para usar:

```sh
cargo new hola
cd hola
```

La carpeta contiene dos cosas:

```text
hola/
├── Cargo.toml      la descripción del proyecto (nombre, versión…)
└── src/
    └── main.rs     el código del programa
```

Para compilar y ejecutar el programa, un solo comando:

```sh
cargo run
```

Cargo compila el código y luego lo ejecuta. Verás aparecer `Hello, world!`.

> **Recuerda:** `cargo new` crea un proyecto, `cargo run` lo compila y lo ejecuta.

## Tu primer programa

Abre `src/main.rs` y reemplaza su contenido:

```rust
fn main() {
    let nombre = "Alicia";
    let edad = 30;
    println!("Me llamo {nombre} y tengo {edad} años.");
}
```

```sortie
Me llamo Alicia y tengo 30 años.
```

Leámoslo línea por línea:

- `fn main()` declara la [función](../debuter/glossaire.md#funcion) `main`. Es el
  punto de partida de todo programa en Rust: siempre empieza ahí.
- Las llaves `{ … }` rodean el **cuerpo** de la función: las instrucciones que se
  ejecutan, en orden.
- `let` crea una [variable](../debuter/glossaire.md#variable): un nombre puesto sobre un
  [valor](../debuter/glossaire.md#valor).
- `println!` muestra una línea. El `!` indica una **macro**, una especie de función
  especial. `{nombre}` se reemplaza por el valor de `nombre`.
- Cada instrucción termina con un punto y coma `;`.

## Compilar sin cargo

Para un solo archivo, también puedes usar `rustc` directamente. Crea un
ejecutable, que luego ejecutas:

```sh
rustc main.rs
./main
```

En un proyecto real, prefiere siempre `cargo`. Se presenta en detalle en
[Proyectos, crates y pruebas](projet.md).

## Errores frecuentes

### Olvidar el ! de println!

```rust norun
fn main() {
    println("¡Hola!");
}
```

```text
error[E0423]: expected function, found macro `println`
 --> src/main.rs:2:5
  |
2 |     println("¡Hola!");
  |     ^^^^^^^ not a function
help: use `!` to invoke the macro
```

`println!` es una macro: el `!` es obligatorio. Escribe `println!("¡Hola!");`.

### Olvidar un punto y coma

```rust norun
fn main() {
    let nombre = "Alicia"
    println!("¡Hola, {nombre}!");
}
```

```text
error: expected `;`, found `println`
 --> src/main.rs:2:26
  |
2 |     let nombre = "Alicia"
  |                          ^ help: add `;` here
3 |     println!("¡Hola, {nombre}!");
  |     ------- unexpected token
```

El compilador muestra dónde añadir el `;`: al final de la línea 2.

### Poner el texto entre comillas simples

```rust norun
fn main() {
    println!('¡Hola!');
}
```

```text
error: character literal may only contain one codepoint
 --> src/main.rs:2:14
  |
2 |     println!('¡Hola!');
  |              ^^^^^^^^
help: if you meant to write a string literal, use double quotes
```

En Rust, un texto se escribe entre comillas dobles: `"¡Hola!"`. Las comillas simples
`'a'` están reservadas para un solo carácter.

### Escribir mal main

```rust norun
fn Main() {
    println!("¡Hola!");
}
```

```text
error[E0601]: `main` function not found in crate `main`
 --> src/main.rs:3:2
  |
3 | }
  |  ^ consider adding a `main` function to `src/main.rs`
```

Rust distingue las mayúsculas de las minúsculas: la función de partida se llama `main`,
en minúsculas.

## Ponte a prueba

**1.** ¿Qué comando crea un proyecto llamado `prueba`? ¿Y cuál lo ejecuta?

> **Respuesta**
>
> `cargo new prueba` crea el proyecto. Después, dentro de la carpeta `prueba`, `cargo run` lo
> compila y lo ejecuta.

**2.** ¿Qué significa el `!` en `println!`?

> **Respuesta**
>
> Indica una **macro**, una especie de función especial. Sin el `!`, el programa
> no compila.

**3.** ¿Qué muestra este programa?

```rust norun
fn main() {
    let ciudad = "Sevilla";
    println!("Vivo en {ciudad}.");
}
```

> **Respuesta**
>
> ```sortie
> Vivo en Sevilla.
> ```

## En resumen

- Rust es un lenguaje compilado, rápido y seguro.
- `rustup` instala Rust; `rustup update` lo actualiza.
- `cargo new nombre` crea un proyecto, `cargo run` lo compila y lo ejecuta.
- Un programa siempre empieza por `fn main()`.
- `println!` muestra una línea de texto; cada instrucción termina con `;`.

Compara con [la introducción a Ruby](../ruby/introduction.md) o
[la introducción a Rythme](../rythme/introduction.md).

Siguiente: [Primeros pasos](premiers-pas.md).
