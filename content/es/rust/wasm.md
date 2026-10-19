# Rust en el navegador con WebAssembly

WebAssembly te permite compilar Rust en un archivo `.wasm` que el navegador ejecuta, y llamar a tus funciones de Rust desde JavaScript como funciones normales.

> **Objetivos:**
>
> - entender qué es WebAssembly y el camino Rust → `.wasm` → navegador;
> - preparar un proyecto con `wasm-bindgen` y compilarlo con `wasm-pack`;
> - llamar a una función de Rust desde JavaScript;
> - saber qué cruza la frontera: números, textos, listas;
> - escribir la lógica en Rust puro y probarla con `cargo run` antes de exportarla.

## ¿Qué es WebAssembly?

Normalmente, un navegador entiende un solo lenguaje de programación: JavaScript.
**WebAssembly** (*Wasm*) es un segundo lenguaje que entiende. Nadie lo escribe a mano:
es un código máquina compacto, fabricado por un
[compilador](../debuter/glossaire.md#compilador), como el de Rust.

Piensa en un adaptador de enchufe de viaje. Tu aparato (el código Rust) tiene el
enchufe de tu país; el país que visitas (el navegador) tiene sus propias tomas. El
adaptador (`.wasm` y un poco de JavaScript) permite conectar uno con el otro.

```text
  Tu código              Compilación               El navegador
 ┌───────────┐   cargo /     ┌───────────────┐   carga   ┌──────────────────┐
 │  lib.rs   │ ────────────► │ calc_bg.wasm  │ ────────► │ Chrome, Firefox, │
 │  (Rust)   │   wasm-pack   │ calc.js       │           │ Safari...        │
 └───────────┘               └───────────────┘           │  + tu página JS  │
                                                         └──────────────────┘
```

- El archivo `.wasm` contiene tu código Rust compilado.
- El pequeño archivo `.js` es el **pegamento**: carga el `.wasm` y traduce los valores
  entre JavaScript y Rust.

¿Para qué sirve? Para las partes **pesadas** de una página: un juego, un editor de
fotos, la compresión de un archivo, cálculos sobre muchos datos. El resto de la página
(los botones, el texto) sigue en JavaScript.

> **Recuerda:** WebAssembly es un segundo lenguaje del navegador; Rust puede
> compilarse a él. El `.wasm` hace el cálculo, el pegamento JavaScript hace de enlace.

## Empezar por la lógica en Rust puro

Antes de pensar en el navegador, escribe tus funciones como siempre, y compruébalas con
un `main`. Es la parte más importante: si aquí está bien, estará bien en el navegador.

Nuestro ejemplo: una función `fuerza` que da a una contraseña una nota de 0 a 4. Un
punto por cada regla que cumple.

```rust
fn fuerza(clave: &str) -> u32 {
    let mut puntos = 0;
    if clave.chars().count() >= 8 {
        puntos += 1;
    }
    if clave.chars().any(|c| c.is_uppercase()) {
        puntos += 1;
    }
    if clave.chars().any(|c| c.is_ascii_digit()) {
        puntos += 1;
    }
    if clave.chars().any(|c| !c.is_alphanumeric()) {
        puntos += 1;
    }
    puntos
}

fn main() {
    println!("gato: {}", fuerza("gato"));
    println!("gatito2024: {}", fuerza("gatito2024"));
    println!("Gatito2024!: {}", fuerza("Gatito2024!"));
}
```

```sortie
gato: 0
gatito2024: 2
Gatito2024!: 4
```

### Paso a paso con «Gatito2024!»

| Paso | Regla | ¿Se cumple? | `puntos` después |
|------|-------|-------------|-----------------:|
| inicio | | | 0 |
| 1 | al menos 8 caracteres (hay 11) | sí | 1 |
| 2 | una mayúscula (`G`) | sí | 2 |
| 3 | una cifra (`2`) | sí | 3 |
| 4 | un carácter que no es ni letra ni cifra (`!`) | sí | 4 |

Con `"gatito2024"`: 10 caracteres (+1), ninguna mayúscula, cifras (+1), ningún
símbolo. Total: 2.

- `chars().count()` cuenta los caracteres (mira [Trabajar con texto](texte.md)).
- `any(|c| …)` responde `true` en cuanto **un** carácter cumple la
  [closure](../debuter/glossaire.md#closure) (mira [Los iteradores](iterateurs.md)).

Prueba: llama a `fuerza("Été2024!")`. El resultado es 4: `É` sí es una mayúscula,
porque `is_uppercase` conoce las letras con tilde.

> **Recuerda:** escribe y prueba la lógica en Rust normal, con `cargo run`. El
> navegador vendrá después.

## Probar antes de exportar

Añadamos otras dos funciones que usará la página web, y comprobemos todo con
`assert_eq!` (mira [Probar tu código](tests.md)):

```rust
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

fn saludar(nombre: &str) -> String {
    format!("¡Hola, {nombre}!")
}

fn promedio(notas: &[f64]) -> f64 {
    if notas.is_empty() {
        return 0.0;
    }
    notas.iter().sum::<f64>() / notas.len() as f64
}

fn main() {
    assert_eq!(sumar(2, 3), 5);
    assert_eq!(saludar("Léa"), "¡Hola, Léa!");
    assert_eq!(promedio(&[]), 0.0);
    println!("{}", promedio(&[12.0, 15.5, 9.0]));
    println!("Todas las pruebas pasan");
}
```

```sortie
12.166666666666666
Todas las pruebas pasan
```

El bucle escondido en `promedio(&[12.0, 15.5, 9.0])`, paso a paso:

| Paso | Nota leída | Suma |
|------|-----------:|-----:|
| inicio | | 0.0 |
| 1 | 12.0 | 12.0 |
| 2 | 15.5 | 27.5 |
| 3 | 9.0 | 36.5 |
| fin | 36.5 / 3 | 12.166666666666666 |

La prueba `promedio(&[]) == 0.0` te protege de una trampa: sin el `if`, dividirías 0.0
entre 0, y el resultado sería `NaN` («no es un número»).

Prueba: borra el `if notas.is_empty()`. El programa se detiene en
`assertion `left == right` failed` con `left: NaN` y `right: 0.0`.

> **Recuerda:** las pruebas se ejecutan en tu ordenador, sin navegador. Un error
> encontrado aquí es diez veces más fácil de corregir que un error en la página.

## Preparar el proyecto con wasm-bindgen

El Rust está listo. Ahora hay que **mostrárselo** a JavaScript. Ese es el trabajo del
[crate](../debuter/glossaire.md#crate) `wasm-bindgen`.

Crea una biblioteca (no un programa: no hay `main`):

```sh
cargo new --lib calc
cd calc
```

Luego completa `Cargo.toml`:

```toml
[package]
name = "calc"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

- `crate-type = ["cdylib"]` pide una biblioteca que **otro lenguaje** puede cargar,
  aquí JavaScript.
- `wasm-bindgen` escribe el pegamento entre los dos lenguajes.

En `src/lib.rs`, retomamos **las mismas funciones**. Solo cambian `pub` (visible desde
fuera) y la etiqueta `#[wasm_bindgen]` («exportar a JavaScript»):

```rust norun
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn saludar(nombre: &str) -> String {
    format!("¡Hola, {nombre}!")
}

#[wasm_bindgen]
pub fn fuerza(clave: &str) -> u32 {
    let mut puntos = 0;
    if clave.chars().count() >= 8 {
        puntos += 1;
    }
    if clave.chars().any(|c| c.is_uppercase()) {
        puntos += 1;
    }
    if clave.chars().any(|c| c.is_ascii_digit()) {
        puntos += 1;
    }
    if clave.chars().any(|c| !c.is_alphanumeric()) {
        puntos += 1;
    }
    puntos
}

#[wasm_bindgen]
pub fn promedio(notas: &[f64]) -> f64 {
    if notas.is_empty() {
        return 0.0;
    }
    notas.iter().sum::<f64>() / notas.len() as f64
}
```

> **Nota:** `#[wasm_bindgen]` es una [macro](../debuter/glossaire.md#macro): escribe
> por ti el código que traduce los argumentos a la entrada y el resultado a la salida.

> **Recuerda:** el mismo código que antes, más `pub` y `#[wasm_bindgen]` delante de
> cada función que quieras exportar.

## Compilar con wasm-pack

`wasm-pack` lo hace todo en un solo comando: compila hacia el destino Wasm, llama a
`wasm-bindgen` y guarda el resultado en una carpeta `pkg/`. Se instala una vez:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

Luego, en la carpeta del proyecto:

```sh
wasm-pack build --target web
```

```text
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
   Compiling wasm-bindgen v0.2.129
   Compiling calc v0.1.0 (/home/lea/calc)
    Finished `release` profile [optimized] target(s) in 11.33s
[INFO]: Installing wasm-bindgen...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: :-) Done in 11.72s
[INFO]: :-) Your wasm pkg is ready to publish at /home/lea/calc/pkg.
```

La carpeta contiene entonces:

```text
calc/
├── Cargo.toml
├── src/
│   └── lib.rs
└── pkg/                    ← fabricado por wasm-pack
    ├── calc_bg.wasm        ← tu Rust compilado (unos 28 KB)
    ├── calc.js             ← el pegamento JavaScript
    ├── calc.d.ts           ← los tipos, para TypeScript
    └── package.json
```

`--target web` produce un módulo que el navegador carga directamente. Con
`--target nodejs`, el mismo código se carga en [Node.js](../debuter/glossaire.md#node-js).

> **Nota:** la línea `Optional fields missing` es solo un consejo, no un error.

> **Recuerda:** `wasm-pack build --target web` → una carpeta `pkg/` con el `.wasm` y
> su pegamento `.js`.

## Llamar a Rust desde JavaScript

En la página, un archivo `main.js` importa las funciones de `pkg/calc.js`. Primero hay
que llamar a `init()`: descarga el `.wasm` y lo prepara.

```javascript norun
import init, { sumar, saludar, fuerza, promedio } from "./pkg/calc.js";

await init();

console.log(sumar(2, 3));
console.log(saludar("Léa"));
console.log(fuerza("Gatito2024!"));
console.log(promedio(new Float64Array([12, 15.5, 9])));
```

La página HTML lo carga con `<script type="module" src="main.js"></script>`. Abre la
consola del navegador (tecla F12):

```text
5
¡Hola, Léa!
4
12.166666666666666
```

Los mismos resultados que con `cargo run`: es el mismo código.

Para ver la página, sirve la carpeta con un pequeño servidor web, por ejemplo
`python3 -m http.server`, y luego abre `http://localhost:8000`. Hacer doble clic en el
archivo HTML no basta: el navegador se niega a cargar un módulo desde el disco.

Para conectar `fuerza` a un campo de texto de verdad, leemos el texto escrito con cada
tecla (mira [el DOM en JavaScript](../javascript/dom.md)):

```javascript norun
const campo = document.querySelector("#clave");
const nota = document.querySelector("#nota");

campo.addEventListener("input", () => {
  nota.textContent = `Fuerza: ${fuerza(campo.value)} / 4`;
});
```

Lo que pasa cuando el usuario escribe `Gato1`:

| Tecla | `campo.value` | `fuerza(…)` | Se muestra |
|-------|---------------|------------:|------------|
| `G` | `G` | 1 | Fuerza: 1 / 4 |
| `a` | `Ga` | 1 | Fuerza: 1 / 4 |
| `t` | `Gat` | 1 | Fuerza: 1 / 4 |
| `o` | `Gato` | 1 | Fuerza: 1 / 4 |
| `1` | `Gato1` | 2 | Fuerza: 2 / 4 |

> **Recuerda:** `import init, { … }`, luego `await init()`, y después tus funciones de
> Rust se llaman como funciones de JavaScript.

## Lo que cruza la frontera

JavaScript y Wasm no comparten los mismos valores. Imagina una aduana: cada valor que
pasa se controla y a veces se **transforma**.

### Los números: pasan tal cual

Wasm solo conoce números. Un `i32`, un `u32`, un `f64` pasan directamente. El archivo
`calc.d.ts` lo muestra: todos se convierten en `number` en JavaScript.

```text
export function sumar(a: number, b: number): number;
export function fuerza(clave: string): number;
export function saludar(nombre: string): string;
export function promedio(notas: Float64Array): number;
```

Pero un `number` de JavaScript siempre es un `f64`. Cuando Rust espera un `i32`, el
pegamento lo **convierte**:

| Llamada en JavaScript | Lo que recibe Rust | Resultado |
|-----------------------|--------------------|----------:|
| `sumar(2, 3)` | `a = 2`, `b = 3` | 5 |
| `sumar(2.7, 1)` | `a = 2` (decimales cortados) | 3 |
| `sumar(2147483647, 1)` | el mayor `i32`, y luego + 1 | -2147483648 |

El último resultado sorprende. Un `i32` no pasa de 2 147 483 647; en el `.wasm`
compilado en modo `release`, el cálculo da la vuelta, como un cuentakilómetros que
vuelve a cero. El Rust normal muestra la misma vuelta con `wrapping_add` (mira
[Los números](nombres.md)):

```rust
fn main() {
    let max = i32::MAX;
    println!("{max}");
    println!("{}", max.wrapping_add(1));
    println!("{:?}", max.checked_add(1));
}
```

```sortie
2147483647
-2147483648
None
```

Si puede haber números muy grandes, usa `i64` o `f64`, o usa `checked_add`, que
responde `None` en vez de dar la vuelta.

### Los textos: copiados en la memoria del Wasm

El `.wasm` tiene su propia memoria, como una caja cerrada. Para darle un texto, el
pegamento JavaScript lo **copia** en esa caja, en UTF-8, y luego le da a Rust dos
números: **dónde** empieza y **cuántos** [bytes](../debuter/glossaire.md#byte) ocupa.
Por eso `fuerza` recibe `(a: number, b: number)` entre bastidores en `calc.d.ts`.

```text
  JavaScript: "Léa"
        │  copia en UTF-8
        ▼
  memoria del Wasm:  ... │ L │ é │ é │ a │ ...
                           ▲   └─2 bytes──┘
                        posición 1024, longitud 4
        │
        ▼
  Rust recibe nombre: &str = "Léa"
```

Rust cuenta esos bytes con `len()`, y los caracteres con `chars().count()`:

```rust
fn main() {
    let nombre = "Léa";
    println!("bytes: {}", nombre.len());
    println!("caracteres: {}", nombre.chars().count());
    for (i, byte) in nombre.bytes().enumerate() {
        println!("{i}: {byte}");
    }
}
```

```sortie
bytes: 4
caracteres: 3
0: 76
1: 195
2: 169
3: 97
```

La `é` ocupa dos bytes (195 y 169). Para la vuelta (`saludar` devuelve un `String`),
es al revés: el pegamento vuelve a leer los bytes de la caja y fabrica un texto de
JavaScript.

> **Nota:** cada cruce copia el texto. Para una función llamada un millón de veces, es
> mejor enviar todo el trabajo de una sola vez.

### Las listas de números: arrays tipados

`&[f64]` se convierte en un `Float64Array` (un array de JavaScript de `f64`), `&[i32]`
en un `Int32Array`, `&[u8]` en un `Uint8Array`. Como con los textos, el array se copia
en la memoria del Wasm.

### El resto: no directamente

Un `HashMap`, un `Vec<String>` como valor de retorno, tus propias estructuras… no
cruzan solos. Hay que anotarlos con `#[wasm_bindgen]`, o convertirlos en JSON con
`serde` (mira [JSON, CSV y TOML](formats.md)).

| Tipo Rust | Tipo JavaScript | Cómo pasa |
|-----------|-----------------|-----------|
| `i32`, `u32`, `f64` | `number` | directamente |
| `bool` | `boolean` | directamente |
| `&str`, `String` | `string` | copia en UTF-8 |
| `&[f64]`, `Vec<f64>` | `Float64Array` | copia |
| `HashMap`, estructura | — | `#[wasm_bindgen]` o JSON |

> **Recuerda:** los números pasan tal cual; los textos y las listas se copian en cada
> cruce; el resto requiere un esfuerzo.

## Errores frecuentes

### Olvidar #[wasm_bindgen]

```rust norun
pub fn fuerza(clave: &str) -> u32 {
    // ...
}
```

El proyecto compila, pero la página falla al cargar:

```text
SyntaxError: The requested module './pkg/calc.js' does not provide an export named 'fuerza'
```

Sin la etiqueta, la función se queda dentro del Rust: el pegamento no la exporta.
Añade `#[wasm_bindgen]` encima y vuelve a ejecutar `wasm-pack build --target web`.

### Devolver un tipo que no cruza

```rust norun
use std::collections::HashMap;

#[wasm_bindgen]
pub fn contar(palabras: Vec<String>) -> HashMap<String, u32> {
    // ...
}
```

```text
error[E0277]: the trait bound `HashMap<String, u32>: IntoWasmAbi` is not satisfied
  --> src/lib.rs:38:1
   |
38 | #[wasm_bindgen]
   | ^^^^^^^^^^^^^^^ the trait `IntoWasmAbi` is not implemented for `HashMap<String, u32>`
```

`IntoWasmAbi` quiere decir «sabe cruzar la frontera». Un `HashMap` no sabe. Devuelve un
texto JSON (`serde_json::to_string`), o un tipo más simple.

### Un pánico en el Wasm

```rust norun
#[wasm_bindgen]
pub fn dividir(a: i32, b: i32) -> i32 {
    a / b
}
```

```javascript norun
console.log(dividir(10, 2));
console.log(dividir(10, 0));
```

```text
5
RuntimeError: unreachable
```

La división entre cero hace [entrar en pánico](../debuter/glossaire.md#panico) a Rust.
En el navegador, el pánico se convierte en este `RuntimeError: unreachable`, que no dice
mucho. Comprueba los casos peligrosos en Rust y devuelve un valor claro, por ejemplo
`Option<i32>`, que se convierte en `number | undefined` en JavaScript.

### Enviar un número donde Rust espera un texto

```javascript norun
console.log(saludar(42));
```

```text
RuntimeError: memory access out of bounds
```

El pegamento cree recibir un texto y se equivoca al copiarlo. JavaScript no comprueba
los tipos: TypeScript, con el archivo `calc.d.ts`, habría señalado el error antes de
ejecutar (mira [el recorrido de TypeScript](../typescript/introduction.md)).

## Ponte a prueba

1. ¿Para qué sirve el archivo `calc.js` que produce `wasm-pack`?

> **Respuesta**
>
> Es el pegamento: carga `calc_bg.wasm` y traduce los valores entre JavaScript y Rust
> (por ejemplo, copia los textos en la memoria del Wasm).

2. ¿Por qué probar `fuerza` con un `main` antes de compilar a Wasm?

> **Respuesta**
>
> Porque es más simple y más rápido: `cargo run` muestra el resultado enseguida, con
> mensajes de error claros. En el navegador, un pánico se convierte en un
> `RuntimeError: unreachable` difícil de entender.

3. ¿Qué recibe realmente la función de Rust cuando JavaScript le pasa `"Léa"`?

> **Respuesta**
>
> Una copia del texto en UTF-8 en la memoria del Wasm: una posición y una longitud
> (4 bytes, porque `é` ocupa dos). Rust la ve como un `&str`.

## En resumen

- WebAssembly es un segundo lenguaje del navegador; Rust se compila a `.wasm`.
- Escribe y prueba la lógica en Rust puro, con `main` y `assert_eq!`.
- Para exportar: `crate-type = ["cdylib"]`, el crate `wasm-bindgen`, `pub` y
  `#[wasm_bindgen]` delante de cada función.
- `wasm-pack build --target web` fabrica `pkg/`; en JavaScript: `await init()`, y
  luego se llaman las funciones.
- Los números pasan tal cual, los textos y los arrays se copian; un `HashMap` no cruza
  sin ayuda.

Siguiente: [Escribir código rápido](performance.md)
