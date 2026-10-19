# Los bucles

Esta página muestra cómo repetir código con `loop`, `while` y `for`, y cómo cambiar el curso de un bucle con `break` y `continue`.

> **Objetivos:**
>
> - repetir código con `loop`, `while` y `for`;
> - recorrer un rango o una colección con `for`;
> - salir de un bucle, incluso anidado, con `break` y `continue`.

Este capítulo sigue a [las condiciones](conditions-boucles.md): un bucle usa una condición para saber cuándo detenerse.

## El bucle loop

Imagina que te piden mostrar los números del 1 al 100. Escribir 100 `println!` sería
largo y pesado. Un bucle hace este trabajo repetitivo por ti.

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

Línea por línea:

- `n` vale 1 al principio; `mut` permite cambiarlo después;
- en cada vuelta, `n` se multiplica por 2: vale 2, luego 4, 8, 16, 32, 64, 128;
- mientras `n` no pase de 100, el `if` no hace nada y el bucle vuelve a empezar;
- cuando `n` vale 128, `break n;` detiene el bucle y da 128, guardado en
  `resultado`.

## El bucle while

A veces no sabes de antemano cuántas vueltas harán falta. Repites mientras dura una
situación, como cuando remueves una sopa mientras no está caliente.

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

Línea por línea:

- `cuenta` vale 3 al principio;
- antes de cada vuelta, Rust hace la pregunta `cuenta > 0`. `3 > 0` es verdadero:
  entra en el bloque;
- el bloque muestra `cuenta` y luego le resta 1 (`-= 1`): vale 2, luego 1, luego 0;
- cuando `cuenta` vale 0, `0 > 0` es falso: Rust sale del bucle y muestra «¡Ya!».

**Te toca:** escribe un programa que muestre los números del 1 al 5 con un bucle
`while`, y luego «¡Listo!».

> **Solución**
>
> ```rust
> let mut n = 1;
> while n <= 5 {
>     println!("{n}");
>     n += 1;
> }
> println!("¡Listo!");
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> ¡Listo!
> ```

## El bucle for

Muy a menudo tienes una serie de cosas que tratar una por una: cada artículo de una
lista de la compra, cada número del 1 al 10. `for` está hecho para eso.

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

Línea por línea:

- `for i in 1..4` da a `i` el valor 1, luego 2, luego 3;
- `print!` muestra sin saltar de línea: los números quedan uno al lado del otro;
- `println!()`, vacío, solo sirve para saltar de línea;
- el segundo bucle hace lo mismo del 1 al 4, pero al revés gracias a `rev()`.

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

**Te toca:** escribe un programa que muestre la tabla del 7, de «7 x 1 = 7» a
«7 x 5 = 35», una línea por cálculo.

> **Solución**
>
> ```rust
> for i in 1..=5 {
>     let resultado = 7 * i;
>     println!("7 x {i} = {resultado}");
> }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

## continue y break

A veces quieres cambiar el curso de un bucle por el camino: dejar de buscar en cuanto
has encontrado algo, o ignorar algunos elementos. Para eso sirven estas dos palabras.

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

Línea por línea:

- `n` toma los valores 1, 2, 3… hasta 9;
- si `n` es par (el resto de la división entre 2 vale 0), `continue` pasa enseguida
  a la vuelta siguiente: no se llega al `print!`;
- si `n` pasa de 7, `break` detiene todo el bucle: el 9 nunca se muestra;
- si no, `print!` muestra el número.

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
## Errores frecuentes

### Olvidar mut en un bucle

```rust norun
fn main() {
    let cuenta = 3;
    while cuenta > 0 {
        println!("{cuenta}");
        cuenta -= 1;
    }
}
```

```text
error[E0384]: cannot assign twice to immutable variable `cuenta`
 --> src/main.rs:5:9
  |
2 |     let cuenta = 3;
  |         ------ first assignment to `cuenta`
...
5 |         cuenta -= 1;
  |         ^^^^^^^^^^^ cannot assign twice to immutable variable
help: consider making this binding mutable
```

El bucle cambia `cuenta` en cada vuelta. Declárala modificable:
`let mut cuenta = 3;`.

### Un break con valor en un for

```rust norun
fn main() {
    let numeros = [4, 8, 15];
    let encontrado = for n in numeros {
        if n > 5 {
            break n;
        }
    };
}
```

```text
error[E0571]: `break` with value from a `for` loop
 --> src/main.rs:5:13
  |
3 |     let encontrado = for n in numeros {
  |                      ---------------- you can't `break` with a value in a `for` loop
4 |         if n > 5 {
5 |             break n;
  |             ^^^^^^^ can only break with a value inside `loop` or breakable block
```

Solo `loop` puede dar un valor con `break`: un bucle `for` puede terminar sin llegar
nunca al `break`. Crea la variable antes del bucle (`let mut encontrado = 0;`) y
escribe `encontrado = n; break;` dentro del bucle.

## Ponte a prueba

**1.** ¿Cuántas veces muestra `bip` este código: `for _ in 0..3 { println!("bip"); }`?

> **Respuesta**
>
> Tres veces: `0..3` da 0, 1 y 2 (el 3 queda excluido).

**2.** ¿Qué muestra `for i in 1..=4 { if i == 2 { continue; } print!("{i} "); }`?

> **Respuesta**
>
> `1 3 4`: cuando `i` vale 2, `continue` pasa a la vuelta siguiente sin mostrar nada.

**3.** En dos bucles anidados, ¿cómo detener los dos de golpe?

> **Respuesta**
>
> Ponle una etiqueta al bucle de fuera (`'fuera: for …`) y escribe
> `break 'fuera;` en el bucle interior.

## En resumen

- `loop` repite hasta `break`, `while` mientras una condición sea verdadera.
- `for x in 1..=5` o `for x in lista`: el bucle más habitual.
- `continue` pasa a la vuelta siguiente, `break` detiene el bucle; una etiqueta `'nombre`
  apunta a un bucle concreto.
- Una variable que cambia dentro de un bucle se declara con `let mut`.

Siguiente: [Las funciones](fonctions.md).
