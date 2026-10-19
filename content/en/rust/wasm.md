# Rust in the browser with WebAssembly

WebAssembly lets you compile Rust into a `.wasm` file that the browser runs, and call your Rust functions from JavaScript like ordinary functions.

> **Goals:**
>
> - understand what WebAssembly is and the path Rust → `.wasm` → browser;
> - set up a project with `wasm-bindgen` and compile it with `wasm-pack`;
> - call a Rust function from JavaScript;
> - know what crosses the border: numbers, text, lists;
> - write the logic in plain Rust and test it with `cargo run` before exporting it.

## What is WebAssembly?

A browser normally understands only one programming language: JavaScript.
**WebAssembly** (*Wasm*) is a second language it understands. Nobody writes it by
hand: it is compact machine code, produced by a
[compiler](../debuter/glossaire.md#compiler), like the Rust one.

Think of a travel plug adapter. Your device (the Rust code) has a plug from home; the
country (the browser) has its own sockets. The adapter (`.wasm` and a little
JavaScript) lets you plug one into the other.

```text
  Your code              Compilation               The browser
 ┌───────────┐   cargo /     ┌───────────────┐   loads   ┌──────────────────┐
 │  lib.rs   │ ────────────► │ calc_bg.wasm  │ ────────► │ Chrome, Firefox, │
 │  (Rust)   │   wasm-pack   │ calc.js       │           │ Safari...        │
 └───────────┘               └───────────────┘           │  + your JS page  │
                                                         └──────────────────┘
```

- The `.wasm` file contains your compiled Rust code.
- The small `.js` file is the **glue**: it loads the `.wasm` and translates values
  between JavaScript and Rust.

What is it for? For the **heavy** parts of a page: a game, a photo editor, compressing
a file, calculations on lots of data. The rest of the page (the buttons, the text)
stays in JavaScript.

> **Remember:** WebAssembly is a second language of the browser; Rust can compile to
> it. The `.wasm` does the calculation, the JavaScript glue makes the connection.

## Start with the logic in plain Rust

Before thinking about the browser, write your functions as usual, and check them with
a `main`. This is the most important part: if it is right here, it will be right in the
browser.

Our example: a function `strength` that gives a password a score from 0 to 4. One
point per rule it follows.

```rust
fn strength(password: &str) -> u32 {
    let mut points = 0;
    if password.chars().count() >= 8 {
        points += 1;
    }
    if password.chars().any(|c| c.is_uppercase()) {
        points += 1;
    }
    if password.chars().any(|c| c.is_ascii_digit()) {
        points += 1;
    }
    if password.chars().any(|c| !c.is_alphanumeric()) {
        points += 1;
    }
    points
}

fn main() {
    println!("cat: {}", strength("cat"));
    println!("kitten2024: {}", strength("kitten2024"));
    println!("Kitten2024!: {}", strength("Kitten2024!"));
}
```

```sortie
cat: 0
kitten2024: 2
Kitten2024!: 4
```

### Step by step with "Kitten2024!"

| Step | Rule | True? | `points` after |
|------|------|-------|---------------:|
| start | | | 0 |
| 1 | at least 8 characters (there are 11) | yes | 1 |
| 2 | an uppercase letter (`K`) | yes | 2 |
| 3 | a digit (`2`) | yes | 3 |
| 4 | a character that is neither a letter nor a digit (`!`) | yes | 4 |

With `"kitten2024"`: 10 characters (+1), no uppercase letter, digits (+1), no symbol.
Total: 2.

- `chars().count()` counts the characters (see [Working with text](texte.md)).
- `any(|c| …)` answers `true` as soon as **one** character matches the
  [closure](../debuter/glossaire.md#closure) (see [Iterators](iterateurs.md)).

Try it: call `strength("Été2024!")`. The result is 4: `É` really is an uppercase
letter, because `is_uppercase` knows accented letters.

> **Remember:** write and test the logic in normal Rust, with `cargo run`. The browser
> comes later.

## Test before exporting

Let's add two more functions that the web page will use, and check everything with
`assert_eq!` (see [Testing your code](tests.md)):

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

fn average(grades: &[f64]) -> f64 {
    if grades.is_empty() {
        return 0.0;
    }
    grades.iter().sum::<f64>() / grades.len() as f64
}

fn main() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(greet("Léa"), "Hello, Léa!");
    assert_eq!(average(&[]), 0.0);
    println!("{}", average(&[12.0, 15.5, 9.0]));
    println!("All tests pass");
}
```

```sortie
12.166666666666666
All tests pass
```

The loop hidden in `average(&[12.0, 15.5, 9.0])`, step by step:

| Step | Grade read | Sum |
|------|-----------:|----:|
| start | | 0.0 |
| 1 | 12.0 | 12.0 |
| 2 | 15.5 | 27.5 |
| 3 | 9.0 | 36.5 |
| end | 36.5 / 3 | 12.166666666666666 |

The test `average(&[]) == 0.0` protects you from a trap: without the `if`, you would
divide 0.0 by 0, and the result would be `NaN` ("not a number").

Try it: remove the `if grades.is_empty()`. The program stops on
`assertion `left == right` failed` with `left: NaN` and `right: 0.0`.

> **Remember:** tests run on your computer, without a browser. A bug found here is ten
> times easier to fix than a bug in the page.

## Setting up the project with wasm-bindgen

The Rust is ready. Now you need to **show** it to JavaScript. That is the job of the
`wasm-bindgen` [crate](../debuter/glossaire.md#crate).

Create a library (not a program: there is no `main`):

```sh
cargo new --lib calc
cd calc
```

Then fill in `Cargo.toml`:

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

- `crate-type = ["cdylib"]` asks for a library that **another language** can load,
  here JavaScript.
- `wasm-bindgen` writes the glue between the two languages.

In `src/lib.rs`, we reuse **the same functions**. The only changes are `pub` (visible
from outside) and the `#[wasm_bindgen]` label ("export to JavaScript"):

```rust norun
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[wasm_bindgen]
pub fn strength(password: &str) -> u32 {
    let mut points = 0;
    if password.chars().count() >= 8 {
        points += 1;
    }
    if password.chars().any(|c| c.is_uppercase()) {
        points += 1;
    }
    if password.chars().any(|c| c.is_ascii_digit()) {
        points += 1;
    }
    if password.chars().any(|c| !c.is_alphanumeric()) {
        points += 1;
    }
    points
}

#[wasm_bindgen]
pub fn average(grades: &[f64]) -> f64 {
    if grades.is_empty() {
        return 0.0;
    }
    grades.iter().sum::<f64>() / grades.len() as f64
}
```

> **Note:** `#[wasm_bindgen]` is a [macro](../debuter/glossaire.md#macro): it writes
> for you the code that translates the arguments on the way in and the result on the
> way out.

> **Remember:** same code as before, plus `pub` and `#[wasm_bindgen]` in front of each
> function to export.

## Compiling with wasm-pack

`wasm-pack` does everything in one command: it compiles to the Wasm target, calls
`wasm-bindgen` and puts the result in a `pkg/` folder. You install it once:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

Then, in the project folder:

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

The folder then contains:

```text
calc/
├── Cargo.toml
├── src/
│   └── lib.rs
└── pkg/                    ← made by wasm-pack
    ├── calc_bg.wasm        ← your compiled Rust (about 28 KB)
    ├── calc.js             ← the JavaScript glue
    ├── calc.d.ts           ← the types, for TypeScript
    └── package.json
```

`--target web` produces a module that the browser loads directly. With
`--target nodejs`, the same code loads in [Node.js](../debuter/glossaire.md#node-js).

> **Note:** the `Optional fields missing` line is only advice, not an error.

> **Remember:** `wasm-pack build --target web` → a `pkg/` folder with the `.wasm` and
> its `.js` glue.

## Calling Rust from JavaScript

In the page, a `main.js` file imports the functions from `pkg/calc.js`. You must first
call `init()`: it downloads the `.wasm` and prepares it.

```javascript norun
import init, { add, greet, strength, average } from "./pkg/calc.js";

await init();

console.log(add(2, 3));
console.log(greet("Léa"));
console.log(strength("Kitten2024!"));
console.log(average(new Float64Array([12, 15.5, 9])));
```

The HTML page loads it with `<script type="module" src="main.js"></script>`. Open the
browser console (F12 key):

```text
5
Hello, Léa!
4
12.166666666666666
```

The same results as with `cargo run`: it is the same code.

To see the page, serve the folder with a small web server, for example
`python3 -m http.server`, then open `http://localhost:8000`. Double-clicking the HTML
file is not enough: the browser refuses to load a module from the disk.

To connect `strength` to a real input field, you read the typed text on every key
press (see [the DOM in JavaScript](../javascript/dom.md)):

```javascript norun
const field = document.querySelector("#password");
const score = document.querySelector("#score");

field.addEventListener("input", () => {
  score.textContent = `Strength: ${strength(field.value)} / 4`;
});
```

What happens when the user types `Cats1`:

| Key | `field.value` | `strength(…)` | Displayed |
|-----|---------------|--------------:|-----------|
| `C` | `C` | 1 | Strength: 1 / 4 |
| `a` | `Ca` | 1 | Strength: 1 / 4 |
| `t` | `Cat` | 1 | Strength: 1 / 4 |
| `s` | `Cats` | 1 | Strength: 1 / 4 |
| `1` | `Cats1` | 2 | Strength: 2 / 4 |

> **Remember:** `import init, { … }`, then `await init()`, then your Rust functions
> are called like JavaScript functions.

## What crosses the border

JavaScript and Wasm do not share the same values. Imagine customs at a border: every
value that goes through is checked and sometimes **transformed**.

### Numbers: they go through as they are

Wasm only knows numbers. An `i32`, a `u32`, an `f64` go through directly. The
`calc.d.ts` file shows it: they all become `number` in JavaScript.

```text
export function add(a: number, b: number): number;
export function strength(password: string): number;
export function greet(name: string): string;
export function average(grades: Float64Array): number;
```

But a JavaScript `number` is always an `f64`. When Rust expects an `i32`, the glue
**converts** it:

| JavaScript call | What Rust receives | Result |
|-----------------|--------------------|-------:|
| `add(2, 3)` | `a = 2`, `b = 3` | 5 |
| `add(2.7, 1)` | `a = 2` (decimals cut off) | 3 |
| `add(2147483647, 1)` | the largest `i32`, then + 1 | -2147483648 |

The last result is surprising. An `i32` cannot go above 2,147,483,647; in the `.wasm`
compiled in `release` mode, the calculation wraps around, like a car's odometer that
goes back to zero. Ordinary Rust shows the same wrap-around with `wrapping_add` (see
[Numbers](nombres.md)):

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

If very large numbers are possible, use `i64` or `f64`, or use `checked_add`, which
answers `None` instead of wrapping around.

### Text: copied into the Wasm memory

The `.wasm` has its own memory, like a closed box. To give it some text, the
JavaScript glue **copies** it into this box, in UTF-8, then gives Rust two numbers:
**where** it starts and **how many** [bytes](../debuter/glossaire.md#byte) long it
is. That is why `strength` receives `(a: number, b: number)` behind the scenes of
`calc.d.ts`.

```text
  JavaScript: "Léa"
        │  copy in UTF-8
        ▼
  Wasm memory:  ... │ L │ é │ é │ a │ ...
                      ▲   └─2 bytes──┘
                   position 1024, length 4
        │
        ▼
  Rust receives name: &str = "Léa"
```

Rust counts these bytes with `len()`, and the characters with `chars().count()`:

```rust
fn main() {
    let name = "Léa";
    println!("bytes: {}", name.len());
    println!("characters: {}", name.chars().count());
    for (i, byte) in name.bytes().enumerate() {
        println!("{i}: {byte}");
    }
}
```

```sortie
bytes: 4
characters: 3
0: 76
1: 195
2: 169
3: 97
```

The `é` takes up two bytes (195 and 169). For the way back (`greet` returns a
`String`), it is the opposite: the glue reads the bytes back from the box and builds a
JavaScript string.

> **Note:** every crossing copies the text. For a function called a million times, it
> is better to send all the work in one go.

### Lists of numbers: typed arrays

`&[f64]` becomes a `Float64Array` (a JavaScript array of `f64`), `&[i32]` an
`Int32Array`, `&[u8]` a `Uint8Array`. As with text, the array is copied into the Wasm
memory.

### Everything else: not directly

A `HashMap`, a `Vec<String>` as a return value, your own structs… do not cross on
their own. You have to annotate them with `#[wasm_bindgen]`, or turn them into JSON
with `serde` (see [JSON, CSV and TOML](formats.md)).

| Rust type | JavaScript type | How it crosses |
|-----------|-----------------|----------------|
| `i32`, `u32`, `f64` | `number` | directly |
| `bool` | `boolean` | directly |
| `&str`, `String` | `string` | copied in UTF-8 |
| `&[f64]`, `Vec<f64>` | `Float64Array` | copied |
| `HashMap`, struct | — | `#[wasm_bindgen]` or JSON |

> **Remember:** numbers go through as they are; text and lists are copied at every
> crossing; everything else takes some effort.

## Common mistakes

### Forgetting #[wasm_bindgen]

```rust norun
pub fn strength(password: &str) -> u32 {
    // ...
}
```

The project compiles, but the page crashes when it loads:

```text
SyntaxError: The requested module './pkg/calc.js' does not provide an export named 'strength'
```

Without the label, the function stays inside the Rust code: the glue does not export
it. Add `#[wasm_bindgen]` above it, then run `wasm-pack build --target web` again.

### Returning a type that cannot cross

```rust norun
use std::collections::HashMap;

#[wasm_bindgen]
pub fn count_words(words: Vec<String>) -> HashMap<String, u32> {
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

`IntoWasmAbi` means "knows how to cross the border". A `HashMap` does not. Return a
JSON string (`serde_json::to_string`), or a simpler type.

### A panic in the Wasm

```rust norun
#[wasm_bindgen]
pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}
```

```javascript norun
console.log(divide(10, 2));
console.log(divide(10, 0));
```

```text
5
RuntimeError: unreachable
```

Dividing by zero makes Rust [panic](../debuter/glossaire.md#panic). In the browser,
the panic becomes this `RuntimeError: unreachable`, which does not say much. Check the
dangerous cases in Rust and return a clear value, for example `Option<i32>`, which
becomes `number | undefined` in JavaScript.

### Sending a number where Rust expects text

```javascript norun
console.log(greet(42));
```

```text
RuntimeError: memory access out of bounds
```

The glue thinks it is receiving text and gets it wrong while copying it. JavaScript
does not check types: TypeScript, with the `calc.d.ts` file, would have reported the
error before running (see [the TypeScript track](../typescript/introduction.md)).

## Test yourself

1. What is the `calc.js` file produced by `wasm-pack` for?

> **Answer**
>
> It is the glue: it loads `calc_bg.wasm` and translates values between JavaScript and
> Rust (for example, it copies text into the Wasm memory).

2. Why test `strength` with a `main` before compiling to Wasm?

> **Answer**
>
> Because it is simpler and faster: `cargo run` shows the result right away, with clear
> error messages. In the browser, a panic becomes a `RuntimeError: unreachable` that is
> hard to understand.

3. What does the Rust function really receive when JavaScript passes it `"Léa"`?

> **Answer**
>
> A copy of the text in UTF-8 in the Wasm memory: a position and a length (4 bytes,
> because `é` takes two). Rust sees it as a `&str`.

## Summary

- WebAssembly is a second language of the browser; Rust compiles to `.wasm`.
- Write and test the logic in plain Rust, with `main` and `assert_eq!`.
- To export: `crate-type = ["cdylib"]`, the `wasm-bindgen` crate, `pub` and
  `#[wasm_bindgen]` in front of each function.
- `wasm-pack build --target web` builds `pkg/`; in JavaScript: `await init()`, then you
  call the functions.
- Numbers go through as they are, text and arrays are copied; a `HashMap` does not
  cross without help.

Next: [Writing fast code](performance.md)
