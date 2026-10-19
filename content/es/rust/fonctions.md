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

Línea por línea:

- `fn saludar() { … }` describe la función: su nombre, `saludar`, y lo que hace. En
  este momento todavía no se muestra nada;
- el programa siempre empieza por `main`;
- el primer `saludar();` llama a la función: Rust ejecuta su bloque, muestra
  «¡Hola!» y luego vuelve a `main`;
- el segundo `saludar();` hace exactamente lo mismo otra vez.

El orden no importa: una función puede escribirse antes o después de `main`.

## Los parámetros

Una función que siempre hace exactamente lo mismo sirve de poco. Los parámetros le dan
información distinta en cada llamada, como una receta de crepes que haces para 2 o
para 6 personas.

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

Línea por línea:

- `fn presentar(nombre: &str, edad: u32)` anuncia dos parámetros: un texto `nombre` y
  un número entero positivo `edad`;
- `presentar("Hugo", 12);` llama a la función: `nombre` vale `"Hugo"` y `edad` vale
  `12`, y luego `println!` muestra la frase;
- la segunda llamada vuelve a empezar con otros valores: `"Inés"` y `34`.

Aquí, `nombre` y `edad` son los parámetros; `"Hugo"` y `12` son los argumentos.

**Te toca:** escribe una función `mostrar_precio` que reciba el nombre de un
artículo (`&str`) y su precio en euros (`u32`), y que muestre por ejemplo «Pan: 2
euros». Llámala para el pan a 2 euros y para el queso a 5 euros.

> **Solución**
>
> ```rust
> fn mostrar_precio(articulo: &str, precio: u32) {
>     println!("{articulo}: {precio} euros");
> }
>
> fn main() {
>     mostrar_precio("Pan", 2);
>     mostrar_precio("Queso", 5);
> }
> ```
>
> ```sortie
> Pan: 2 euros
> Queso: 5 euros
> ```

## El tipo de retorno

Una función también puede hacer un cálculo y **devolverte** el resultado, como una
calculadora. Así puedes guardar ese resultado en una variable y volver a usarlo.

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

Línea por línea:

- `fn cuadrado(n: i32) -> i32`: la función recibe un entero `n` y promete devolver un
  entero;
- en `main`, `cuadrado(7)` llama a la función: `n` vale 7;
- la última línea de la función, `n * n`, calcula 49. No tiene `;`: es el valor
  devuelto;
- `let resultado = cuadrado(7);` guarda entonces 49 en `resultado`, que `println!`
  muestra.

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

**Te toca:** escribe una función `perimetro` que reciba el lado de un cuadrado
(`i32`) y devuelva su perímetro, es decir 4 veces el lado. Muestra el perímetro de un
cuadrado de lado 5.

> **Solución**
>
> ```rust
> fn perimetro(lado: i32) -> i32 {
>     4 * lado
> }
>
> fn main() {
>     println!("{}", perimetro(5));
> }
> ```
>
> ```sortie
> 20
> ```

## Expresiones e instrucciones

Esta diferencia explica una regla que acabas de ver: la última línea de una función no
lleva punto y coma cuando es ella la que da el valor devuelto.

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

Línea por línea:

- `let x = {` abre un bloque cuyo valor irá a `x`;
- `let a = 3;` y `let b = 4;` son instrucciones: terminan con `;` y no producen
  ningún valor;
- `a * b` no tiene `;`: es una expresión, que vale 12. Es el valor del bloque;
- `x` recibe entonces 12, y `println!` lo muestra.

> **Atención:** un punto y coma de más convierte la expresión en instrucción. La
> función ya no devuelve nada (ver [Errores frecuentes](#errores-frecuentes)).

## Una función que se llama a sí misma

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

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

Línea por línea:

- `factorial(5)` calcula `5 * factorial(4)`: primero necesita la respuesta para 4;
- del mismo modo, `factorial(4)` pide `factorial(3)`, luego 2, luego 1;
- `factorial(1)` es el caso base: devuelve 1 sin llamarse a sí misma;
- las respuestas suben: 2 × 1 = 2, luego 3 × 2 = 6, 4 × 6 = 24 y 5 × 24 = 120.

## Las closures

A veces necesitas una función muy pequeña, solo en un sitio: por ejemplo, para decirle
a otra función qué cálculo hacer. Escribirla aparte con `fn` sería pesado. Una closure
cabe en una línea, justo donde la necesitas.

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

Línea por línea:

- `let doble = |x: i32| x * 2;` guarda una closure en `doble`: recibe `x` y devuelve
  `x * 2`;
- `let sumar = |a, b| a + b;` recibe dos valores y devuelve su suma. Rust adivina su
  tipo;
- `doble(5)` se llama como una función y vale 10; `sumar(2, 3)` vale 5.

**Te toca:** escribe una closure `es_par` que reciba un `i32` y devuelva `true` si
el número es par (el resto de su división entre 2 es 0). Muestra el resultado para 4 y
luego para 7.

> **Solución**
>
> ```rust
> let es_par = |n: i32| n % 2 == 0;
> println!("{}", es_par(4));
> println!("{}", es_par(7));
> ```
>
> ```sortie
> true
> false
> ```

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
