# Ejercicios para empezar bien

Esta página propone quince pequeños ejercicios muy guiados, todos fáciles, para practicar las bases de Rust: variables, números, condiciones, bucles, funciones, texto, listas y tablas.

## Cómo hacer estos ejercicios

Cada ejercicio es corto. Hazlos en orden: cada uno usa un poco más de cosas que el
anterior.

1. **Lee** el enunciado y la salida esperada.
2. **Prueba en papel**: escribe los pasos en español, línea por línea.
3. **Escribe el código** en un proyecto creado con `cargo new ejercicios`, y luego
   ejecútalo con `cargo run`.
4. **Compara** tu salida con la salida esperada, carácter por carácter.
5. ¿Atascado? Lee **la primera pista**, luego la segunda. Mira la solución al final.

> **Consejo:** si el compilador muestra un error, lee la línea `help:`. A menudo
> contiene la corrección. La página [Las trampas del principiante](pieges.md) explica
> los errores más comunes.

Tu solución puede ser distinta de la nuestra e igual de correcta. Si compila y muestra
la salida correcta, lo has conseguido.

## Mostrar y calcular

### Ejercicio 1: Decir hola

Nivel: ★☆☆

Escribe un programa que muestre dos líneas: `¡Hola!` y luego `Estoy aprendiendo Rust.`

Salida esperada:

```text
¡Hola!
Estoy aprendiendo Rust.
```

> **Pista:** una línea se muestra con `println!("…");`. No olvides el `!`.

> **Solución**
>
> ```rust
> fn main() {
>     println!("¡Hola!");
>     println!("Estoy aprendiendo Rust.");
> }
> ```
>
> ```sortie
> ¡Hola!
> Estoy aprendiendo Rust.
> ```
>
> Por qué funciona: cada `println!` muestra su texto y luego pasa a la línea siguiente.

### Ejercicio 2: Dentro de diez años

Nivel: ★☆☆

Crea una variable `nombre` que valga `"Léa"` y una variable `edad` que valga `14`.
Muestra su edad dentro de diez años.

Salida esperada:

```text
Léa tendrá 24 años dentro de diez años.
```

> **Pista:** crea las variables con `let nombre = "Léa";` y `let edad = 14;`.

> **Pista:** el cálculo puede hacerse en otra variable:
> `let mas_tarde = edad + 10;`. Luego muéstrala con `{mas_tarde}`.

> **Solución**
>
> ```rust
> fn main() {
>     let nombre = "Léa";
>     let edad = 14;
>     let mas_tarde = edad + 10;
>     println!("{nombre} tendrá {mas_tarde} años dentro de diez años.");
> }
> ```
>
> ```sortie
> Léa tendrá 24 años dentro de diez años.
> ```
>
> Por qué funciona: `{nombre}` y `{mas_tarde}` se sustituyen por el valor de las
> variables en el momento de mostrarlas.

### Ejercicio 3: Un contador que avanza

Nivel: ★☆☆

Crea una variable `paso` que valga 0. Súmale 100 tres veces, mostrando su valor
después de cada suma.

Salida esperada:

```text
100
200
300
```

> **Pista:** una variable que cambia debe declararse con `let mut`.

> **Pista:** para sumar 100, escribe `paso += 100;` (es lo mismo que
> `paso = paso + 100;`).

> **Solución**
>
> ```rust
> fn main() {
>     let mut paso = 0;
>     paso += 100;
>     println!("{paso}");
>     paso += 100;
>     println!("{paso}");
>     paso += 100;
>     println!("{paso}");
> }
> ```
>
> ```sortie
> 100
> 200
> 300
> ```
>
> Por qué funciona: `mut` permite que la variable cambie. Sin él, Rust rechaza la
> segunda línea.

### Ejercicio 4: Horas y minutos

Nivel: ★☆☆

Una película dura 135 minutos. Muestra su duración en horas y minutos.

Salida esperada:

```text
2 h 15 min
```

> **Pista:** una hora son 60 minutos. ¿Cuántas veces cabe 60 en 135?

> **Pista:** entre dos enteros, `/` da el número entero de veces (`135 / 60`) y `%` da
> lo que sobra (`135 % 60`).

> **Solución**
>
> ```rust
> fn main() {
>     let duracion = 135;
>     let horas = duracion / 60;
>     let minutos = duracion % 60;
>     println!("{horas} h {minutos} min");
> }
> ```
>
> ```sortie
> 2 h 15 min
> ```
>
> Por qué funciona: 60 cabe 2 veces en 135 (eso hace 120), y sobran
> 135 − 120 = 15 minutos.

### Ejercicio 5: Celsius a Fahrenheit

Nivel: ★☆☆

Convierte 20 °C a grados Fahrenheit. La fórmula es: `F = C × 9 / 5 + 32`.

Salida esperada:

```text
20 °C = 68 °F
```

> **Pista:** usa números con decimales (`f64`): escribe `20.0`, `9.0`, `5.0` y `32.0`.

> **Solución**
>
> ```rust
> fn main() {
>     let celsius = 20.0;
>     let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
>     println!("{celsius} °C = {fahrenheit} °F");
> }
> ```
>
> ```sortie
> 20 °C = 68 °F
> ```
>
> Por qué funciona: 20 × 9 = 180, 180 / 5 = 36, 36 + 32 = 68. Con `f64`, la división
> no corta los decimales. Rust muestra `20.0` como `20`.

## Decidir y repetir

### Ejercicio 6: Par o impar

Nivel: ★☆☆

Crea una variable `n` que valga 7. Muestra `7 es impar` (o `par` si cambias el
número).

Salida esperada:

```text
7 es impar
```

> **Pista:** un número es par cuando el resto de su división entre 2 vale 0:
> `n % 2 == 0`.

> **Pista:** usa `if … { … } else { … }`.

> **Solución**
>
> ```rust
> fn main() {
>     let n = 7;
>     if n % 2 == 0 {
>         println!("{n} es par");
>     } else {
>         println!("{n} es impar");
>     }
> }
> ```
>
> ```sortie
> 7 es impar
> ```
>
> Por qué funciona: `7 % 2` vale 1, así que la condición es falsa y es el bloque
> `else` el que se ejecuta.

### Ejercicio 7: La tabla del 7

Nivel: ★☆☆

Muestra las cinco primeras líneas de la tabla de multiplicar del 7.

Salida esperada:

```text
7 x 1 = 7
7 x 2 = 14
7 x 3 = 21
7 x 4 = 28
7 x 5 = 35
```

> **Pista:** un bucle `for i in 1..=5` hace que `i` tome los valores 1, 2, 3, 4 y 5.

> **Solución**
>
> ```rust
> fn main() {
>     for i in 1..=5 {
>         println!("7 x {i} = {}", 7 * i);
>     }
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
>
> Por qué funciona: el `=` de `1..=5` incluye el 5. Las `{}` vacías se sustituyen por
> el cálculo escrito después de la coma.

### Ejercicio 8: La suma de 1 a 100

Nivel: ★☆☆

Calcula 1 + 2 + 3 + … + 100 con un bucle, y muestra el resultado.

Salida esperada:

```text
5050
```

> **Pista:** necesitas una variable `total`, que empieza en 0 y que cambia: `let mut`.

> **Pista:** en el bucle `for n in 1..=100`, suma `n` al total. Estas son las
> primeras vueltas:
>
> | Vuelta | n | total después |
> |--------|---|---------------|
> | 1 | 1 | 1 |
> | 2 | 2 | 3 |
> | 3 | 3 | 6 |

> **Solución**
>
> ```rust
> fn main() {
>     let mut total = 0;
>     for n in 1..=100 {
>         total += n;
>     }
>     println!("{total}");
> }
> ```
>
> ```sortie
> 5050
> ```
>
> Por qué funciona: `total` se crea **antes** del bucle, una sola vez. Cada vuelta le
> suma el número siguiente.

### Ejercicio 9: El día de la semana

Nivel: ★☆☆

Crea una variable `dia` que valga 3. Con `match`, muestra el nombre del día: 1 para
lunes, 2 para martes, 3 para miércoles. Para cualquier otro número, muestra
`otro día`.

Salida esperada:

```text
miércoles
```

> **Pista:** un `match` enumera casos: `1 => println!("lunes"),`.

> **Pista:** el caso `_ =>` atrapa «todo lo demás». Rust lo exige aquí, porque `dia`
> podría valer cualquier número.

> **Solución**
>
> ```rust
> fn main() {
>     let dia = 3;
>     match dia {
>         1 => println!("lunes"),
>         2 => println!("martes"),
>         3 => println!("miércoles"),
>         _ => println!("otro día"),
>     }
> }
> ```
>
> ```sortie
> miércoles
> ```
>
> Por qué funciona: Rust prueba los casos de arriba abajo y ejecuta el primero que
> coincide.

## Escribir funciones

### Ejercicio 10: El cuadrado

Nivel: ★☆☆

Escribe una función `cuadrado` que reciba un `i32` y devuelva ese número multiplicado
por sí mismo. Muestra `cuadrado(4)` y `cuadrado(9)`.

Salida esperada:

```text
16
81
```

> **Pista:** la primera línea se escribe `fn cuadrado(x: i32) -> i32 {`.

> **Pista:** la última línea de la función es el valor devuelto, **sin** punto y
> coma: `x * x`.

> **Solución**
>
> ```rust
> fn cuadrado(x: i32) -> i32 {
>     x * x
> }
>
> fn main() {
>     println!("{}", cuadrado(4));
>     println!("{}", cuadrado(9));
> }
> ```
>
> ```sortie
> 16
> 81
> ```
>
> Por qué funciona: `-> i32` anuncia lo que la función devuelve, y `x * x` sin punto y
> coma es ese valor.

### Ejercicio 11: Mayor de edad o no

Nivel: ★☆☆

Escribe una función `es_mayor` que reciba una edad (`u32`) y devuelva `true` si la
edad es 18 o más. Pruébala con 15 y 18.

Salida esperada:

```text
15 años: false
18 años: true
```

> **Pista:** la función devuelve un `bool`: `fn es_mayor(edad: u32) -> bool`.

> **Pista:** una comparación ya da `true` o `false`. El cuerpo de la función cabe en
> una línea: `edad >= 18`.

> **Solución**
>
> ```rust
> fn es_mayor(edad: u32) -> bool {
>     edad >= 18
> }
>
> fn main() {
>     println!("15 años: {}", es_mayor(15));
>     println!("18 años: {}", es_mayor(18));
> }
> ```
>
> ```sortie
> 15 años: false
> 18 años: true
> ```
>
> Por qué funciona: `>=` quiere decir «mayor o igual que». No hace falta `if`: la
> comparación ya es la respuesta.

## Jugar con el texto

### Ejercicio 12: Gritar una palabra

Nivel: ★☆☆

Para la palabra `"hola"`, muestra su número de letras y luego la palabra en
mayúsculas.

Salida esperada:

```text
4 letras
HOLA
```

> **Pista:** `.chars().count()` cuenta las letras de un texto.

> **Pista:** `.to_uppercase()` devuelve el texto en mayúsculas.

> **Solución**
>
> ```rust
> fn main() {
>     let palabra = "hola";
>     println!("{} letras", palabra.chars().count());
>     println!("{}", palabra.to_uppercase());
> }
> ```
>
> ```sortie
> 4 letras
> HOLA
> ```
>
> Por qué funciona: `.chars()` recorre las letras una a una, y `.count()` las cuenta.
> Con `"año"`, `.chars().count()` da 3, como debe ser.

### Ejercicio 13: Construir una frase

Nivel: ★☆☆

Empieza con una `String` vacía. Añádele `"Rust"`, luego `" es"`, luego `" veloz"`.
Muestra la frase y su longitud.

Salida esperada:

```text
Rust es veloz
13
```

> **Pista:** una `String` vacía se crea con `String::new()`. Va a cambiar, así que:
> `let mut frase = String::new();`.

> **Pista:** `frase.push_str("Rust");` añade texto al final.

> **Solución**
>
> ```rust
> fn main() {
>     let mut frase = String::new();
>     frase.push_str("Rust");
>     frase.push_str(" es");
>     frase.push_str(" veloz");
>     println!("{frase}");
>     println!("{}", frase.len());
> }
> ```
>
> ```sortie
> Rust es veloz
> 13
> ```
>
> Por qué funciona: una `String` es un texto tuyo, que puede crecer. Los espacios
> forman parte de los textos añadidos: cuentan en la longitud.

## Listas y tablas

### Ejercicio 14: La mejor nota

Nivel: ★☆☆

Aquí tienes unas notas: `vec![12, 17, 9, 15]`. Muestra su suma y luego la más alta.

Salida esperada:

```text
suma: 53
mejor: 17
```

> **Pista:** recorre la lista con `for n in &notas`. Guarda dos variables `mut`:
> `suma` (en 0) y `mejor` (en 0).

> **Pista:** en el bucle: `suma += n;`, y luego `if *n > mejor { mejor = *n; }`. La
> estrella `*` lee el número a través del `&`.
>
> | Vuelta | n | suma | mejor |
> |--------|---|------|-------|
> | 1 | 12 | 12 | 12 |
> | 2 | 17 | 29 | 17 |
> | 3 | 9 | 38 | 17 |
> | 4 | 15 | 53 | 17 |

> **Solución**
>
> ```rust
> fn main() {
>     let notas = vec![12, 17, 9, 15];
>     let mut suma = 0;
>     let mut mejor = 0;
>     for n in &notas {
>         suma += n;
>         if *n > mejor {
>             mejor = *n;
>         }
>     }
>     println!("suma: {suma}");
>     println!("mejor: {mejor}");
> }
> ```
>
> ```sortie
> suma: 53
> mejor: 17
> ```
>
> Por qué funciona: `mejor` recuerda la nota más alta vista hasta ahora. Solo cambia
> si la nota de la vuelta es más alta. Rust también sabe hacerlo solo:
> `notas.iter().sum::<i32>()` y `notas.iter().max()`.

### Ejercicio 15: Contar las frutas

Nivel: ★☆☆

Aquí tienes una cesta: `["manzana", "kiwi", "manzana", "pera", "manzana"]`. Cuenta
cada fruta con un `HashMap` y luego muestra el número de manzanas y de kiwis.

Salida esperada:

```text
manzanas: 3
kiwis: 1
```

> **Pista:** empieza con `use std::collections::HashMap;` y
> `let mut cuenta = HashMap::new();`.

> **Pista:** en el bucle, esta línea suma 1 a la fruta (y la crea en 0 si no existe):
> `*cuenta.entry(fruta).or_insert(0) += 1;`.

> **Solución**
>
> ```rust
> use std::collections::HashMap;
>
> fn main() {
>     let cesta = ["manzana", "kiwi", "manzana", "pera", "manzana"];
>     let mut cuenta = HashMap::new();
>     for fruta in cesta {
>         *cuenta.entry(fruta).or_insert(0) += 1;
>     }
>     println!("manzanas: {}", cuenta["manzana"]);
>     println!("kiwis: {}", cuenta["kiwi"]);
> }
> ```
>
> ```sortie
> manzanas: 3
> kiwis: 1
> ```
>
> Por qué funciona: la tabla asocia cada fruta a un contador. Solo mostramos claves
> concretas: por eso el orden (imprevisible) de un `HashMap` no molesta. Más
> detalles en [HashMap, HashSet y BTreeMap](hashmap.md).

Para ejercicios más difíciles, sobre todo el recorrido, consulta los
[Ejercicios resueltos](exercices.md).

Siguiente: [Propiedad y préstamo](possession.md)
