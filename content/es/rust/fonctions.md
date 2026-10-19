# Las funciones

Esta página explica cómo escribir funciones con `fn`, darles parámetros y un tipo de retorno, y usar las closures.

> **Objetivos:**
>
> - declarar una función con `fn` y llamarla;
> - darle parámetros con tipo y un tipo de retorno;
> - entender la diferencia entre expresión e instrucción;
> - devolver varios valores con una tupla;
> - escribir una closure y pasarla a otra función.

## Declarar una función

Una [función](../debuter/glossaire.md#funcion) es un trozo de código que tiene un
nombre. La escribes una vez y luego la **llamas** tantas veces como quieras,
como una receta de cocina que se vuelve a hacer.

Una función se declara con `fn`, un nombre en `minusculas_con_guiones_bajos`,
paréntesis y un bloque:

```rust
fn saludar() {
    println!("¡Hola!");
}

fn main() {
    saludar();
    saludar();
}
```

```sortie
¡Hola!
¡Hola!
```

El orden no importa: una función puede escribirse antes o después de `main`.

## Los parámetros

Un [parámetro](../debuter/glossaire.md#parametro) es una variable que la función
recibe. El valor dado en el momento de la llamada se llama
[argumento](../debuter/glossaire.md#argumento).

Cada parámetro tiene un nombre **y un tipo**. El tipo es obligatorio.

```rust
fn presentar(nombre: &str, edad: u32) {
    println!("{nombre} tiene {edad} años.");
}

fn main() {
    presentar("Hugo", 12);
    presentar("Inés", 34);
}
```

```sortie
Hugo tiene 12 años.
Inés tiene 34 años.
```

Aquí, `nombre` y `edad` son los parámetros; `"Hugo"` y `12` son los argumentos.

## El tipo de retorno

El tipo devuelto se escribe después de una flecha `->`. La **última expresión**, sin
punto y coma, es el valor devuelto:

```rust
fn cuadrado(n: i32) -> i32 {
    n * n
}

fn main() {
    let resultado = cuadrado(7);
    println!("{resultado}");
}
```

```sortie
49
```

Para salir antes, usa `return`:

```rust
fn signo(n: i32) -> &'static str {
    if n < 0 {
        return "negativo";
    }
    if n == 0 {
        return "cero";
    }
    "positivo"
}

fn main() {
    println!("{} {} {}", signo(-3), signo(0), signo(8));
}
```

```sortie
negativo cero positivo
```

> **Nota:** `&'static str` designa un texto escrito directamente en el programa. El
> `'static` es un tiempo de vida, presentado en
> [Propiedad y préstamo](possession.md).

Una función sin `->` devuelve `()`, que se pronuncia «unit»: un valor vacío, que quiere
decir «nada».

### Devolver varios valores

Una **tupla** agrupa varios valores (ver [Primeros pasos](premiers-pas.md#las-tuplas)):

```rust
fn division(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn main() {
    let (cociente, resto) = division(17, 5);
    println!("{cociente} resto {resto}");
}
```

```sortie
3 resto 2
```

## Expresiones e instrucciones

Es una idea importante en Rust:

- una **expresión** produce un valor: `5`, `a + b`, `if … { … } else { … }`, un
  bloque `{ … }`;
- una **instrucción** hace una acción y no produce nada. Termina con `;`.

Un bloque entre llaves es una expresión. Su valor es su última línea:

```rust
let x = {
    let a = 3;
    let b = 4;
    a * b
};
println!("{x}");
```

```sortie
12
```

> **Atención:** un punto y coma de más convierte la expresión en instrucción. La
> función ya no devuelve nada (ver [Errores frecuentes](#errores-frecuentes)).

## Una función que se llama a sí misma

Una función puede llamarse a sí misma: es la
[recursión](../debuter/glossaire.md#recursion). Siempre hace falta un caso base, si no
nunca se detiene:

```rust
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!("{}", factorial(5));
}
```

```sortie
120
```

## Las closures

Una [closure](../debuter/glossaire.md#closure) es una pequeña función sin nombre, que
se guarda en una variable. Sus parámetros se escriben entre barras `|…|`. Los tipos
se suelen adivinar:

```rust
let doble = |x: i32| x * 2;
let sumar = |a, b| a + b;
println!("{}", doble(5));
println!("{}", sumar(2, 3));
```

```sortie
10
5
```

### Capturar el entorno

Una closure puede usar las variables que la rodean. Las **captura**, como
una mochila que lleva consigo:

```rust
let bonus = 10;
let con_bonus = |nota: i32| nota + bonus;
println!("{}", con_bonus(12));
```

```sortie
22
```

Una closure que modifica una variable capturada debe guardarse en una variable `mut`:

```rust
let mut total = 0;
let mut sumar = |n: i32| total += n;
sumar(5);
sumar(7);
println!("{total}");
```

```sortie
12
```

### Pasar una closure a una función

Las closures sirven sobre todo para personalizar el trabajo de otra función. Aquí, el
parámetro `f` acepta cualquier función que reciba un `i32` y devuelva un `i32`:

```rust
fn aplicar_dos_veces(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    println!("{}", aplicar_dos_veces(|n| n + 3, 1));
    println!("{}", aplicar_dos_veces(|n| n * n, 3));
}
```

```sortie
7
81
```

Lee `impl Fn(i32) -> i32` así: «cualquier función que reciba un `i32` y
devuelva un `i32`». Aparecen por todas partes con los iteradores: ver
[Colecciones](collections.md).

## Errores frecuentes

### Olvidar el tipo de un parámetro

```rust norun
fn doble(x) -> i32 {
    x * 2
}

fn main() {
    println!("{}", doble(4));
}
```

```text
error: expected one of `:`, `@`, or `|`, found `)`
 --> src/main.rs:1:11
  |
1 | fn doble(x) -> i32 {
  |           ^ expected one of `:`, `@`, or `|`
help: if this is a parameter name, give it a type
```

Corrección: `fn doble(x: i32) -> i32`.

### Un punto y coma después del valor devuelto

```rust norun
fn cuadrado(n: i32) -> i32 {
    n * n;
}

fn main() {
    println!("{}", cuadrado(3));
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:1:24
  |
1 | fn cuadrado(n: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     n * n;
  |          - help: remove this semicolon to return this value
```

Corrección: quita el `;` después de `n * n`.

### Olvidar el tipo de retorno

```rust norun
fn cuadrado(n: i32) {
    n * n
}

fn main() {
    println!("{:?}", cuadrado(3));
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:2:5
  |
1 | fn cuadrado(n: i32) {
  |                    - help: try adding a return type: `-> i32`
2 |     n * n
  |     ^^^^^ expected `()`, found `i32`
```

Sin `->`, la función no devuelve nada. Añade `-> i32`.

### Olvidar un argumento

```rust norun
fn presentar(nombre: &str, edad: u32) {
    println!("{nombre} tiene {edad} años.");
}

fn main() {
    presentar("Hugo");
}
```

```text
error[E0061]: this function takes 2 arguments but 1 argument was supplied
 --> src/main.rs:6:5
  |
6 |     presentar("Hugo");
  |     ^^^^^^^^^-------- argument #2 of type `u32` is missing
```

Da un argumento por cada parámetro, en orden: `presentar("Hugo", 12);`.

## Ponte a prueba

**1.** ¿Qué devuelve `fn f() -> i32 { 1; 2 }`?

> **Respuesta**
>
> `2`: es la última expresión, sin punto y coma.

**2.** Escribe una función `media(a: f64, b: f64) -> f64`.

> **Respuesta**
>
> ```rust
> fn media(a: f64, b: f64) -> f64 {
>     (a + b) / 2.0
> }
>
> fn main() {
>     println!("{}", media(12.0, 15.0));
> }
> ```
>
> ```sortie
> 13.5
> ```

**3.** Escribe una closure `triple` que multiplique un número por 3.

> **Respuesta**
>
> ```rust
> let triple = |x: i32| x * 3;
> println!("{}", triple(4));
> ```
>
> ```sortie
> 12
> ```

## En resumen

- `fn nombre(param: Tipo) -> Retorno { … }` declara una función.
- El tipo de cada parámetro es obligatorio.
- La última expresión, sin `;`, es el valor devuelto; `return` sale antes.
- Una tupla `(a, b)` devuelve varios valores.
- `|x| x * 2` es una closure: una pequeña función sin nombre, que puede capturar las
  variables que la rodean.

Compara con [las funciones en Ruby](../ruby/fonctions.md) y
[en Rythme](../rythme/fonctions.md).

Siguiente: [Trabajar con texto](texte.md).
