# Condiciones y bucles

Esta página muestra cómo elegir entre varios caminos con `if` y `match`, y cómo repetir código con `loop`, `while` y `for`.

> **Objetivos:**
>
> - escribir una [condición](../debuter/glossaire.md#condicion) con `if`, `else if` y
>   `else`;
> - elegir entre varios casos con `match` y `matches!`;
> - repetir código con `loop`, `while` y `for`;
> - salir de un bucle, incluso anidado, con `break` y `continue`;
> - leer un valor opcional con `if let` y `while let`.

## La condición if

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

## if es una expresión

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

### Las guardas

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

## El bucle loop

Un [bucle](../debuter/glossaire.md#bucle) repite un bloque de código. `loop` repite
sin fin, hasta un `break`. El `break` puede devolver un valor:

```rust
let mut n = 1;
let resultado = loop {
    n = n * 2;
    if n > 100 {
        break n;
    }
};
println!("{resultado}");
```

```sortie
128
```

## El bucle while

`while` repite mientras la condición sea verdadera. Se comprueba antes de cada vuelta:

```rust
let mut cuenta = 3;
while cuenta > 0 {
    println!("{cuenta}...");
    cuenta -= 1;
}
println!("¡Ya!");
```

```sortie
3...
2...
1...
¡Ya!
```

## El bucle for

`for` recorre un rango o una colección, elemento por elemento. Es el bucle
más usado.

### Sobre un rango

`1..4` va de 1 a 3 (el 4 queda excluido). `1..=4` va de 1 a 4 incluido. `rev()` recorre
al revés.

```rust
for i in 1..4 {
    print!("{i} ");
}
println!();
for i in (1..=4).rev() {
    print!("{i} ");
}
println!();
```

```sortie
1 2 3 
4 3 2 1 
```

### Sobre una colección

```rust
let frutas = ["manzana", "pera", "kiwi"];
for fruta in frutas {
    println!("Me gusta: {fruta}");
}
for (i, fruta) in frutas.iter().enumerate() {
    println!("{i}: {fruta}");
}
```

```sortie
Me gusta: manzana
Me gusta: pera
Me gusta: kiwi
0: manzana
1: pera
2: kiwi
```

`enumerate()` da, además de cada elemento, su posición (a partir de 0).

## continue y break

`continue` pasa a la vuelta siguiente, `break` detiene el bucle:

```rust
for n in 1..10 {
    if n % 2 == 0 {
        continue;
    }
    if n > 7 {
        break;
    }
    print!("{n} ");
}
println!();
```

```sortie
1 3 5 7 
```

### Salir de dos bucles: las etiquetas

En dos bucles anidados, `break` solo detiene el bucle interior. Para detener
el de fuera, ponle una **etiqueta**: un nombre que empieza por un apóstrofo,
como `'cuadricula`.

```rust
'cuadricula: for fila in 1..=3 {
    for columna in 1..=3 {
        if fila * columna == 4 {
            println!("Encontrado en ({fila}, {columna})");
            break 'cuadricula;
        }
    }
}
println!("Fin");
```

```sortie
Encontrado en (2, 2)
Fin
```

`continue 'cuadricula` funciona de la misma forma: pasa a la vuelta siguiente del bucle
etiquetado.

## if let y while let

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

**1.** ¿Cuántas veces muestra `bip` este código: `for _ in 0..3 { println!("bip"); }`?

> **Respuesta**
>
> Tres veces: `0..3` da 0, 1 y 2 (el 3 queda excluido).

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

**3.** En dos bucles anidados, ¿cómo detener los dos de golpe?

> **Respuesta**
>
> Ponle una etiqueta al bucle de fuera (`'fuera: for …`) y escribe
> `break 'fuera;` en el bucle interior.

## En resumen

- `if … else` elige un camino, y puede devolver un valor.
- `match` compara un valor con patrones; `_` recoge el resto; `matches!` prueba un
  solo patrón.
- `loop` repite hasta `break`, `while` mientras una condición sea verdadera.
- `for x in 1..=5` o `for x in lista`: el bucle más habitual.
- `continue` pasa a la vuelta siguiente, `break` detiene el bucle; una etiqueta `'nombre`
  apunta a un bucle concreto.
- `if let Some(x) = …` y `while let` leen un valor opcional.

Compara con [Ruby](../ruby/conditions-boucles.md) y
[Rythme](../rythme/conditions-boucles.md).

Siguiente: [Las funciones](fonctions.md).
