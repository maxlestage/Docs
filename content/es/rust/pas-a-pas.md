# Seguir un programa paso a paso

Esta página te enseña a ejecutar un pequeño programa Rust en tu cabeza, línea por línea, con una tabla que sigue el valor de cada variable.

> **Objetivos:**
>
> - leer un programa línea por línea, en el orden en que el ordenador lo ejecuta;
> - rellenar una tabla de traza: una columna por variable, una fila por paso;
> - seguir una condición, un bucle y una llamada a función;
> - adivinar la salida de un programa **antes** de ejecutarlo;
> - hacer que el programa muestre sus pasos con `println!` y `dbg!`.

## Por qué seguir un programa a mano

Un ordenador no adivina nada. Lee tus líneas **una a una**, de arriba abajo, y hace
exactamente lo que está escrito. Para entender un programa, lo mejor es hacer lo
mismo: jugar a ser el ordenador, con un lápiz.

Imagina una receta de cocina. No lees toda la receta de golpe: haces el paso 1, luego
el paso 2, y miras lo que hay en el bol después de cada paso. Un programa es igual. El
bol son las [variables](../debuter/glossaire.md#variable).

Saber hacer esto te servirá todo el tiempo:

- para **entender** un ejemplo que lees;
- para **encontrar un bug**: el lugar donde tu tabla y la realidad dejan de estar de
  acuerdo;
- para **resolver los ejercicios** sin probarlo todo al azar.

> **Recuerda:** seguir un programa es hacer como el ordenador: una línea cada vez,
> anotando lo que vale cada variable después de cada línea.

## El método de la tabla de traza

Una **tabla de traza** tiene una columna por variable y una fila por paso. Por cada
línea de código ejecutada, anotas los nuevos valores. Una casilla vacía quiere decir:
«todavía no creada».

```text
   el código                  la tabla
 ┌──────────────────┐     ┌──────┬────┬────┐
 │ let a = 2;       │ ──▶ │ Paso │ a  │ b  │
 │ let b = a + 1;   │     ├──────┼────┼────┤
 │ ...              │     │  1   │ 2  │    │
 └──────────────────┘     │  2   │ 2  │ 3  │
                          └──────┴────┴────┘
```

Tres reglas sencillas:

1. **Una línea de código cada vez.** Nunca dos.
2. **Lee siempre el valor actual** en la tabla: la fila de arriba.
3. **Una variable solo cambia si una línea la modifica** (con `=` o `+=`). Si no,
   copia su valor.

Para cada programa de esta página, haz esto:

1. lee el código;
2. **adivina** lo que se muestra, y escríbelo;
3. rellena la tabla en papel;
4. compara con nuestra tabla, y luego con la salida real.

> **Recuerda:** una columna por variable, una fila por paso. El valor que hay que usar
> es siempre el más reciente.

## Programa 1: variables que cambian

Aquí tienes un pequeño truco de magia con dos variables.

```rust
fn main() {
    let mut a = 3;
    let mut b = 5;
    a = a + b;
    b = a - b;
    a = a - b;
    println!("a = {a}, b = {b}");
}
```

Antes de ejecutarlo, adivina lo que se muestra. Escribe tu respuesta.

La palabra `mut` quiere decir que la variable tiene derecho a cambiar
([mutable](../debuter/glossaire.md#mutable)). Sigamos cada línea:

| Paso | Línea | Cálculo | a | b |
|------|-------|---------|---|---|
| 1 | `let mut a = 3;` | | 3 | |
| 2 | `let mut b = 5;` | | 3 | 5 |
| 3 | `a = a + b;` | 3 + 5 = 8 | **8** | 5 |
| 4 | `b = a - b;` | 8 − 5 = 3 | 8 | **3** |
| 5 | `a = a - b;` | 8 − 3 = 5 | **5** | 3 |

```sortie
a = 5, b = 3
```

Los dos valores se han **intercambiado**. Mira el paso 4: `a` ya vale 8, no 3. Siempre
se usa el valor **más reciente**, el de la fila de arriba.

Es como dos vasos: para intercambiar su contenido sin un tercer vaso, lo viertes todo
en el primero y luego recuperas la cantidad correcta.

> **Prueba:** sustituye `let mut b = 5;` por `let mut b = 10;`. Rehaz la tabla.
> Debes obtener `a = 10, b = 3`.

> **Recuerda:** en cada paso, se leen los valores **actuales** de las variables, no sus
> valores iniciales.

## Programa 2: una condición

Una [condición](../debuter/glossaire.md#condition) elige **un solo camino**. Los demás
bloques se saltan, como un cambio de agujas del tren.

```rust
fn main() {
    let temperatura = 14;
    let mut ropa = "camiseta";
    if temperatura < 10 {
        ropa = "abrigo";
    } else if temperatura < 20 {
        ropa = "jersey";
    }
    println!("{temperatura} grados: {ropa}");
}
```

Antes de ejecutarlo, adivina lo que se muestra.

Para una condición, añade una columna «Prueba»: la pregunta planteada y su respuesta
(`true` o `false`).

| Paso | Línea | Prueba | temperatura | ropa |
|------|-------|--------|-------------|------|
| 1 | `let temperatura = 14;` | | 14 | |
| 2 | `let mut ropa = "camiseta";` | | 14 | camiseta |
| 3 | `if temperatura < 10` | ¿14 < 10? **false** | 14 | camiseta |
| 4 | `else if temperatura < 20` | ¿14 < 20? **true** | 14 | camiseta |
| 5 | `ropa = "jersey";` | | 14 | **jersey** |
| 6 | `println!` | | 14 | jersey |

```sortie
14 grados: jersey
```

La línea `ropa = "abrigo";` **no** aparece en la tabla: su prueba era falsa, Rust salta
ese bloque.

```text
          ¿temperatura < 10?
           /            \
         sí              no
         |                |
     "abrigo"      ¿temperatura < 20?
                     /          \
                   sí            no
                   |              |
               "jersey"     (nada: se queda "camiseta")
```

> **Prueba:** pon `let temperatura = 25;`. Las dos pruebas son falsas: no se ejecuta
> ningún bloque, y el programa muestra `25 grados: camiseta`.

> **Recuerda:** en una condición, anota la prueba y su respuesta. Solo el bloque cuya
> prueba es verdadera entra en la tabla.

## Programa 3: un bucle que acumula

Un [bucle](../debuter/glossaire.md#boucle) repite un bloque. En la tabla, cada
**vuelta** del bucle se convierte en una o varias filas.

```rust
fn main() {
    let mut total = 0;
    for i in 1..5 {
        total += i * 2;
    }
    println!("total = {total}");
}
```

Antes de ejecutarlo, adivina lo que se muestra. ¡Cuidado con `1..5`!

El intervalo `1..5` da 1, 2, 3 y 4. El 5 **no** está incluido (para incluirlo, hace
falta `1..=5`).

| Vuelta | i | Cálculo | total tras la vuelta |
|--------|---|---------|----------------------|
| antes | | | 0 |
| 1 | 1 | 0 + 1 × 2 | 2 |
| 2 | 2 | 2 + 2 × 2 | 6 |
| 3 | 3 | 6 + 3 × 2 | 12 |
| 4 | 4 | 12 + 4 × 2 | 20 |
| fin | | `i` valdría 5: el bucle se detiene | 20 |

```sortie
total = 20
```

`total` es como una hucha: se crea **antes** del bucle, una sola vez, y cada vuelta
mete una moneda. Si la crearas dentro del bucle, se vaciaría en cada vuelta.

```text
 hucha:  [0] ──+2──▶ [2] ──+4──▶ [6] ──+6──▶ [12] ──+8──▶ [20]
 vuelta:        1           2           3             4
```

> **Prueba:** sustituye `1..5` por `1..=5`. Hay una vuelta más (i = 5, +10): el
> programa muestra `total = 30`.

> **Recuerda:** para un bucle, una fila por vuelta. Anota el valor de la variable del
> bucle **y** el del acumulador al final de la vuelta.

## Programa 4: una función llamada dos veces

Una [función](../debuter/glossaire.md#fonction) es una pequeña máquina: le das un
valor, trabaja en **su propio rincón** y luego devuelve un resultado.

```rust
fn precio_con_iva(base: i32) -> i32 {
    let iva = base / 5;
    base + iva
}

fn main() {
    let a = precio_con_iva(100);
    let b = precio_con_iva(30);
    println!("{a} + {b} = {}", a + b);
}
```

Antes de ejecutarlo, adivina lo que se muestra.

El programa empieza **siempre** por `main`. Cuando `main` llama a `precio_con_iva`, se
pone en pausa y espera la respuesta. Este es el dibujo de las llamadas:

```text
 main                                precio_con_iva
 ────                                ──────────────
 a = precio_con_iva(100) ──100──▶   base = 100, iva = 20
         ◀────────────────120─────  devuelve 100 + 20
 a = 120

 b = precio_con_iva(30)  ──30───▶   base = 30, iva = 6
         ◀────────────────36──────  devuelve 30 + 6
 b = 36

 println! muestra 120 + 36 = 156
```

En la tabla, una columna «¿Dónde?» dice en qué función estamos:

| Paso | ¿Dónde? | Línea | base | iva | a | b |
|------|---------|-------|------|-----|---|---|
| 1 | main | `precio_con_iva(100)`: llamada | | | | |
| 2 | precio_con_iva | `base` recibe 100 | 100 | | | |
| 3 | precio_con_iva | `let iva = base / 5;` | 100 | 20 | | |
| 4 | precio_con_iva | `base + iva`: devuelve 120 | 100 | 20 | | |
| 5 | main | `let a = …` | | | 120 | |
| 6 | main | `precio_con_iva(30)`: llamada | | | 120 | |
| 7 | precio_con_iva | `base` recibe 30 | 30 | | 120 | |
| 8 | precio_con_iva | `let iva = base / 5;` | 30 | 6 | 120 | |
| 9 | precio_con_iva | `base + iva`: devuelve 36 | 30 | 6 | 120 | |
| 10 | main | `let b = …` | | | 120 | 36 |

```sortie
120 + 36 = 156
```

El `println!` muestra `{a}` y `{b}`, o sea los precios **con** el IVA (120 y 36), no
100 y 30. Si habías adivinado `100 + 30 = …`, leíste los argumentos en lugar de los
resultados.

Dos cosas que observar:

- `base` e `iva` **desaparecen** cuando la función termina (casillas vacías en los
  pasos 5 y 10). Solo existen durante la llamada;
- en la segunda llamada, `base` e `iva` empiezan de cero: la función no se acuerda de
  la primera llamada.

> **Prueba:** añade `let c = precio_con_iva(a);` antes del `println!`, y muestra `c`.
> La función recibe 120, el IVA vale 24: `c` vale 144.

> **Recuerda:** en cada llamada, se «entra» en la función con el valor dado, se sigue
> hasta el resultado y luego se vuelve al lugar exacto de la llamada.

## Programa 5: una lista y una condición en el bucle

Último programa: un bucle sobre una [lista](../debuter/glossaire.md#liste) (un `Vec`),
con una condición **dentro** del bucle. Contamos los gastos grandes (más de 10 €) y los
sumamos.

```rust
fn main() {
    let gastos = vec![4, 25, 12, 7, 30];
    let mut grandes = 0;
    let mut suma = 0;
    for g in &gastos {
        if *g > 10 {
            grandes += 1;
            suma += g;
        }
    }
    println!("{grandes} gastos grandes, total {suma}");
}
```

Antes de ejecutarlo, adivina lo que se muestra.

`for g in &gastos` toma las casillas **una a una**, de izquierda a derecha. El `&`
quiere decir «mirar sin tomar»: la lista sigue siendo utilizable después. El asterisco
de `*g` lee el número a través de ese `&`.

```text
 gastos:  [ 4 | 25 | 12 |  7 | 30 ]
            ▲    ▲    ▲    ▲    ▲
 vuelta:    1    2    3    4    5
```

En cada vuelta hay una prueba. Añade una columna para ella:

| Vuelta | g | ¿g > 10? | grandes | suma |
|--------|---|----------|---------|------|
| antes | | | 0 | 0 |
| 1 | 4 | false | 0 | 0 |
| 2 | 25 | **true** | 1 | 25 |
| 3 | 12 | **true** | 2 | 37 |
| 4 | 7 | false | 2 | 37 |
| 5 | 30 | **true** | 3 | 67 |

```sortie
3 gastos grandes, total 67
```

En las vueltas 1 y 4, la prueba es falsa: `grandes` y `suma` no se mueven.
Simplemente **copias** sus valores en la tabla.

> **Prueba:** sustituye `*g > 10` por `*g >= 12`. El 12 sigue contando, pero no el 4
> ni el 7: la salida no cambia. Con `*g > 12`, en cambio, obtienes
> `2 gastos grandes, total 55`.

> **Recuerda:** cuando la prueba de una vuelta es falsa, las variables conservan su
> valor. Cópialo: no te saltes la fila de la tabla.

## Hacer que el programa muestre sus pasos

¿Es correcta tu tabla? El propio programa te lo puede decir: basta con hacer que
muestre sus variables mientras trabaja. Es como pedirle a un cocinero que diga en voz
alta cada gesto que hace.

### Con println! y una etiqueta

Añade un `println!` en el bucle, con una **etiqueta** para saber de dónde viene cada
línea:

```rust
fn main() {
    let mut total = 0;
    for i in 1..5 {
        total += i * 2;
        println!("[vuelta {i}] total = {total}");
    }
    println!("total = {total}");
}
```

```sortie
[vuelta 1] total = 2
[vuelta 2] total = 6
[vuelta 3] total = 12
[vuelta 4] total = 20
total = 20
```

Son exactamente las filas de nuestra tabla del programa 3. Sin la etiqueta
`[vuelta …]`, verías solo números, sin saber cuál es cuál.

### Con dbg!

Rust tiene una [macro](../debuter/glossaire.md#macro) hecha para esto: `dbg!`. Muestra
el nombre del archivo, el número de línea, el nombre de la variable **y** su valor. No
tienes que escribir ninguna etiqueta.

```rust
fn main() {
    let mut total = 0;
    for i in 1..5 {
        total += i * 2;
        dbg!(i, total);
    }
    println!("total = {total}");
}
```

```sortie
total = 20
```

¡La salida de arriba solo tiene una línea! Es normal: `dbg!` **no** escribe en la
salida normal (*stdout*), sino en la **salida de errores** (*stderr*). En tu terminal,
con `cargo run` (el archivo es entonces `src/main.rs`), ves las dos mezcladas:

```text
[src/main.rs:5:9] i = 1
[src/main.rs:5:9] total = 2
[src/main.rs:5:9] i = 2
[src/main.rs:5:9] total = 6
[src/main.rs:5:9] i = 3
[src/main.rs:5:9] total = 12
[src/main.rs:5:9] i = 4
[src/main.rs:5:9] total = 20
total = 20
```

`[src/main.rs:5:9]` quiere decir: archivo `src/main.rs`, línea 5, columna 9.

¿Por qué dos salidas? Imagina dos tuberías que salen del programa. La primera lleva los
**resultados**, la segunda los **mensajes para ti**. Si envías los resultados a un
archivo (`cargo run > resultado.txt`), los mensajes de `dbg!` se quedan en la pantalla
y no ensucian el archivo.

```text
              ┌─────────────┐  stdout (println!) ──▶ resultados
 programa ──▶ │             │
              └─────────────┘  stderr (dbg!)     ──▶ mensajes de depuración
```

Extra: `dbg!` **devuelve** el valor que le das. Puedes ponerlo alrededor de un cálculo
sin romper nada:

```rust
fn precio_con_iva(base: i32) -> i32 {
    let iva = base / 5;
    dbg!(base + iva)
}

fn main() {
    let a = precio_con_iva(100);
    println!("a = {a}");
}
```

```sortie
a = 120
```

Y en la salida de errores, además:

```text
[src/main.rs:3:5] base + iva = 120
```

> **Prueba:** en el programa 5, añade `dbg!(g, grandes, suma);` al final del bloque
> del bucle. Ejecútalo con `cargo run`: obtienes tu tabla de traza, escrita por el
> propio programa.

> **Recuerda:** `println!` con una etiqueta, o `dbg!(variable)`, convierten el programa
> en una tabla de traza. `dbg!` escribe en *stderr*: acuérdate de quitarlo cuando hayas
> encontrado el problema.

## ¿Y el depurador?

Un [depurador](../debuter/glossaire.md#debogage) pone el programa en pausa en una línea
y te muestra todas las variables, sin añadir nada al código. Hace tu tabla de traza por
ti, una línea cada vez: mira la sección sobre un depurador en VS Code, en el capítulo
[Depurar en Rust](debogage.md).

## Errores frecuentes

### Olvidar un paso

En el programa 5, a menudo se saltan las vueltas en las que «no pasa nada». La tabla
se vuelve entonces incorrecta, porque ya no sabes por dónde vas en la lista.

```text
Tabla incompleta (mal):          Tabla completa (bien):
| g  | grandes | suma |          | g  | grandes | suma |
| 25 | 1       | 25   |          | 4  | 0       | 0    |
| 12 | 2       | 37   |          | 25 | 1       | 25   |
| 30 | 3       | 67   |          | 12 | 2       | 37   |
                                 | 7  | 2       | 37   |
                                 | 30 | 3       | 67   |
```

Corrección: **una fila por vuelta**, aunque nada cambie. Copia los valores.

### Reutilizar un valor antiguo

En el programa 1, en la línea `b = a - b;`, muchos calculan 3 − 5 con la `a` del
principio. Pero `a` ya vale 8:

```text
Mal:  b = 3 - 5 = -2     (la "a" del paso 1)
Bien: b = 8 - 5 = 3      (la "a" del paso 3, la fila justo de arriba)
```

Corrección: lee siempre el valor en la **última** fila de la tabla.

### Una vuelta de más (o de menos)

`for i in 1..5` solo da **4** vueltas. Contar hasta 5 da un total equivocado (30 en
lugar de 20):

```text
1..5   → 1, 2, 3, 4        (el 5 queda excluido)
1..=5  → 1, 2, 3, 4, 5     (el 5 está incluido)
```

Corrección: antes de rellenar la tabla, escribe la lista de valores que tomará la
variable del bucle.

### Olvidar mut en una variable que cambia

Al copiar un programa para seguirlo, a veces se olvida `mut`. Rust se niega entonces a
compilarlo:

```rust norun
fn main() {
    let total = 0;
    for i in 1..5 {
        total += i * 2;
    }
    println!("total = {total}");
}
```

```text
error[E0384]: cannot assign twice to immutable variable `total`
 --> src/main.rs:4:9
  |
2 |     let total = 0;
  |         ----- first assignment to `total`
3 |     for i in 1..5 {
4 |         total += i * 2;
  |         ^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut total = 0;
  |         +++
```

Corrección: `let mut total = 0;`. En tu tabla también es una buena referencia: solo las
variables `mut` pueden cambiar de una fila a otra.

### Dar la lista a dbg! en lugar de prestarla

`dbg!(gastos)` **toma** la lista (se dice que se [mueve](possession.md)). Después, ya
no se puede usar:

```rust norun
fn main() {
    let gastos = vec![4, 25, 12];
    dbg!(gastos);
    println!("{}", gastos.len());
}
```

```text
error[E0382]: borrow of moved value: `gastos`
 --> src/main.rs:4:20
  |
2 |     let gastos = vec![4, 25, 12];
  |         ------ move occurs because `gastos` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     dbg!(gastos);
  |     ------------ value moved here
4 |     println!("{}", gastos.len());
  |                    ^^^^^^ value borrowed here after move
  |
help: consider borrowing instead of transferring ownership
  |
3 |     dbg!(&gastos);
  |          +
```

Corrección: préstala con `&`: `dbg!(&gastos);`.

## Ponte a prueba

**1.** ¿Cuánto vale `x` al final de estas tres líneas?
`let mut x = 2; x = x * 3; x = x - 1;`

> **Respuesta**
>
> | Paso | x |
> |------|---|
> | 1 | 2 |
> | 2 | 2 × 3 = 6 |
> | 3 | 6 − 1 = 5 |
>
> `x` vale **5**: cada línea parte del valor de la línea anterior.

**2.** ¿Cuántas vueltas da `for i in 0..3`, y qué valores toma `i`?

> **Respuesta**
>
> Tres vueltas: `i` vale 0, luego 1, luego 2. El 3 queda excluido.

**3.** ¿Qué muestra este programa? Haz la tabla antes de mirar la respuesta.

```rust norun
fn main() {
    let mut cuenta = 0;
    for n in [3, 8, 5, 10] {
        if n % 2 == 0 {
            cuenta += n;
        }
    }
    println!("{cuenta}");
}
```

> **Respuesta**
>
> | Vuelta | n | ¿n par? | cuenta |
> |--------|---|---------|--------|
> | 1 | 3 | false | 0 |
> | 2 | 8 | true | 8 |
> | 3 | 5 | false | 8 |
> | 4 | 10 | true | 18 |
>
> El programa suma los números pares. Aquí está con su salida:
>
> ```rust
> fn main() {
>     let mut cuenta = 0;
>     for n in [3, 8, 5, 10] {
>         if n % 2 == 0 {
>             cuenta += n;
>         }
>     }
>     println!("{cuenta}");
> }
> ```
>
> ```sortie
> 18
> ```

## En resumen

- Seguir un programa es jugar a ser el ordenador: una línea cada vez, de arriba abajo.
- La tabla de traza tiene una columna por variable y una fila por paso (o por vuelta).
- Usa siempre el valor **más reciente**; copia los valores que no cambian.
- Para una función, entra en ella con el valor recibido, síguela y luego vuelve al
  punto de llamada con el resultado.
- `println!` con una etiqueta o `dbg!` hacen que el programa muestre sus pasos;
  `dbg!` escribe en la salida de errores (*stderr*).

Siguiente: [Las trampas del principiante en Rust](pieges.md)
