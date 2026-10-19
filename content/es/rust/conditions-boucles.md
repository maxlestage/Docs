# Las condiciones

Esta página muestra cómo elegir entre varios caminos con `if` y `match`, y cómo leer un valor opcional con `if let`.

> **Objetivos:**
>
> - escribir una [condición](../debuter/glossaire.md#condicion) con `if`, `else if` y
>   `else`;
> - comparar valores con `==`, `<`, `&&`, `||` y `!`;
> - elegir entre varios casos con `match` y `matches!`;
> - leer un valor opcional con `if let` y `while let`.

## La condición if

Un programa a menudo tiene que decidir. Si hace calor, aconseja beber; si hace frío,
ponerse un abrigo. `if` («si») sirve para hacer esa elección.

`if` ejecuta un bloque solo si la condición es verdadera. No hacen falta paréntesis,
pero las llaves son obligatorias.

```rust
let temperatura = 25;
if temperatura > 30 {
    println!("Hace calor.");
} else if temperatura > 15 {
    println!("Hace buen tiempo.");
} else {
    println!("Hace fresco.");
}
```

```sortie
Hace buen tiempo.
```

Línea por línea:

- `let temperatura = 25;` guarda 25 en la variable `temperatura`;
- primera prueba: «¿25 es mayor que 30?». No: Rust se salta ese bloque;
- segunda prueba, con `else if`: «¿25 es mayor que 15?». Sí: Rust muestra
  «Hace buen tiempo.»;
- el bloque `else` no se ejecuta, porque una prueba ya ha tenido éxito.

Rust prueba las condiciones en orden y se detiene en la primera verdadera. `else`
recoge todos los demás casos.

La condición debe ser un verdadero [booleano](../debuter/glossaire.md#booleano) (`bool`):
`true` o `false`. Un número no basta (ver [Errores
frecuentes](#errores-frecuentes)).

### Los operadores de comparación

| Operador | Significado |
|-----------|------|
| `==` / `!=` | igual / distinto |
| `<` / `<=` | menor / menor o igual |
| `>` / `>=` | mayor / mayor o igual |
| `&&` / `\|\|` / `!` | y / o / no |

```rust
let edad = 15;
let acompanado = true;
if edad >= 18 || acompanado {
    println!("Entrada permitida");
}
if !(edad >= 18) && edad > 12 {
    println!("Adolescente");
}
```

```sortie
Entrada permitida
Adolescente
```

**Te toca:** escribe un programa que guarde `hora = 15` en una variable y luego
muestre «Buenos días» si la hora es antes de las 12, «Buenas tardes» si es antes de
las 18, y «Buenas noches» si no.

> **Solución**
>
> ```rust
> let hora = 15;
> if hora < 12 {
>     println!("Buenos días");
> } else if hora < 18 {
>     println!("Buenas tardes");
> } else {
>     println!("Buenas noches");
> }
> ```
>
> ```sortie
> Buenas tardes
> ```

## if es una expresión

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

En Rust, `if` **devuelve un valor**. Puedes guardarlo en una variable:

```rust
let edad = 20;
let estado = if edad >= 18 { "mayor" } else { "menor" };
println!("{estado}");
```

```sortie
mayor
```

> **Atención:** las dos ramas deben dar el mismo tipo.

## Elegir con match

Cuando un valor puede tomar muchas formas, una larga serie de `else if` se vuelve
difícil de leer. `match` ordena todos esos casos con claridad, uno debajo de otro.

`match` compara un valor con varios **patrones** (formas posibles). El primero
que coincide gana. Piensa en una clasificación de correo: cada carta va a la primera casilla
que le corresponde.

```rust
let dia = 6;
let nombre = match dia {
    1 => "lunes",
    2 | 3 | 4 => "mitad de semana",
    5 => "viernes",
    6 | 7 => "fin de semana",
    _ => "día desconocido",
};
println!("{nombre}");
```

```sortie
fin de semana
```

Línea por línea:

- `dia` vale 6;
- `match dia` compara 6 con cada patrón, de arriba abajo: ¿1? no. ¿2, 3 o 4? no.
  ¿5? no;
- `6 | 7` coincide: el `match` da el texto `"fin de semana"`, guardado en `nombre`;
- el patrón `_` no se usa aquí: solo cuenta si ningún otro patrón coincide.

`|` significa «o». Los patrones también pueden ser rangos:

```rust
let nota = 14;
match nota {
    0..=9 => println!("Insuficiente"),
    10..=15 => println!("Bien"),
    _ => println!("Muy bien"),
}
```

```sortie
Bien
```

> **Recuerda:** `match` debe cubrir **todos** los casos. El patrón `_` recoge todo lo
> demás.

**Te toca:** escribe un programa que guarde `monedas = 3` en una variable y luego
use `match` para mostrar «una» para 1, «dos» para 2, «tres» para 3, y «muchas» para
todo lo demás.

> **Solución**
>
> ```rust
> let monedas = 3;
> match monedas {
>     1 => println!("una"),
>     2 => println!("dos"),
>     3 => println!("tres"),
>     _ => println!("muchas"),
> }
> ```
>
> ```sortie
> tres
> ```

### Las guardas

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Un patrón puede ir seguido de `if` y una condición: es una **guarda**. El nombre `n`
recibe el valor probado:

```rust
let temperatura = -4;
let estado = match temperatura {
    n if n < 0 => "hielo",
    0..=99 => "líquido",
    _ => "vapor",
};
println!("{estado}");
```

```sortie
hielo
```

### Probar un solo patrón con matches!

`matches!(valor, patrón)` responde `true` o `false`. Es práctico dentro de un `if`:

```rust
let letra = 'e';
let vocal = matches!(letra, 'a' | 'e' | 'i' | 'o' | 'u');
println!("{vocal}");
println!("{}", matches!(42, 0..=9));
```

```sortie
true
false
```

`match` es todavía más útil con las enumeraciones: ver
[Estructuras, enumeraciones y traits](types.md).

## if let y while let

Piensa en un buzón: a veces hay una carta, a veces nada. Antes de leer la carta, hay
que comprobar que está ahí. `if let` y `while let` hacen esa comprobación por ti.

Algunos valores son **opcionales**: valen `Some(x)` si hay algo,
o `None` si no hay nada. Es el tipo `Option`, presentado en detalle en
[Estructuras, enumeraciones y traits](types.md).

`if let` ejecuta un bloque solo si el valor coincide con el patrón, y da un nombre a
lo que contiene:

```rust
let mut pila = vec![1, 2, 3];
if let Some(arriba) = pila.pop() {
    println!("Quitado: {arriba}");
}
```

```sortie
Quitado: 3
```

`pop()` quita el último elemento de una lista y devuelve `Some(elemento)`, o `None` si
la lista está vacía. `while let` repite mientras el patrón coincida:

```rust
let mut pila = vec![1, 2, 3];
while let Some(arriba) = pila.pop() {
    print!("{arriba} ");
}
println!();
```

```sortie
3 2 1 
```

> **Consejo:** `if let` es un `match` abreviado, para cuando solo te interesa un caso.
## Errores frecuentes

### Una condición que no es un booleano

```rust norun
fn main() {
    let n = 1;
    if n {
        println!("sí");
    }
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if n {
  |        ^ expected `bool`, found integer
```

Rust no convierte un número en booleano. Escribe la comparación: `if n != 0`.

### = en lugar de ==

```rust norun
fn main() {
    let x = 5;
    if x = 5 {
        println!("cinco");
    }
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if x = 5 {
  |        ^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
```

`=` guarda un valor, `==` compara. Escribe `if x == 5`.

### Un match que olvida casos

```rust norun
fn main() {
    let nota = 14;
    match nota {
        0..=9 => println!("Insuficiente"),
        10..=20 => println!("Bien"),
    }
}
```

```text
error[E0004]: non-exhaustive patterns: `i32::MIN..=-1_i32` and `21_i32..=i32::MAX` not covered
 --> src/main.rs:3:11
  |
3 |     match nota {
  |           ^^^^ patterns `i32::MIN..=-1_i32` and `21_i32..=i32::MAX` not covered
```

Un `i32` también puede valer `-5` o `300`. Añade una última línea
`_ => println!("Nota no válida"),`.

### Dos ramas de tipos distintos

```rust norun
fn main() {
    let edad = 20;
    let estado = if edad >= 18 { "mayor" } else { 0 };
}
```

```text
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:3:51
  |
3 |     let estado = if edad >= 18 { "mayor" } else { 0 };
  |                                  -------          ^ expected `&str`, found integer
  |                                  |
  |                                  expected because of this
```

Una variable solo tiene un tipo: las dos ramas deben dar texto, por ejemplo
`else { "menor" }`.

## Ponte a prueba

**1.** ¿Qué muestra este código?

```rust norun
let x = 7;
if x > 10 {
    println!("grande");
} else if x > 5 {
    println!("mediano");
} else {
    println!("pequeño");
}
```

> **Respuesta**
>
> `mediano`: 7 no es mayor que 10, pero sí es mayor que 5. Rust se detiene en la
> primera condición verdadera.

**2.** Escribe un `match` que muestre `pequeño` para 0 a 9, `mediano` para 10 a 99 y
`grande` en otro caso, para `n = 42`.

> **Respuesta**
>
> ```rust
> let n = 42;
> match n {
>     0..=9 => println!("pequeño"),
>     10..=99 => println!("mediano"),
>     _ => println!("grande"),
> }
> ```
>
> ```sortie
> mediano
> ```

**3.** ¿Qué hace `if let Some(cima) = pila.pop() { … }` cuando la pila está vacía?

> **Respuesta**
>
> `pop()` devuelve `None`, que no encaja con el patrón `Some(cima)`: el bloque no
> se ejecuta, y el programa sigue después.

## En resumen

- `if … else` elige un camino, y puede devolver un valor.
- `==`, `<`, `&&`, `||` y `!` construyen condiciones; una condición siempre es
  un `bool`.
- `match` compara un valor con patrones; `_` recoge el resto; `matches!` prueba un
  solo patrón.
- `if let Some(x) = …` y `while let` leen un valor opcional.

Compara con [Ruby](../ruby/conditions-boucles.md) y
[Rythme](../rythme/conditions-boucles.md).

Siguiente: [Los bucles](boucles.md).
