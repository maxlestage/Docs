# Ejercicios resueltos: segunda serie

Esta página propone doce ejercicios nuevos de Rust, del más fácil al más difícil, cada uno con una pista, una solución explicada y a menudo una tabla que sigue el programa paso a paso.

## Cómo trabajar

Estos ejercicios son **nuevos**: no repiten los de
[Ejercicios para empezar bien](exercices-faciles.md) ni los de
[Ejercicios resueltos](exercices.md). Hazlos en un proyecto creado con
`cargo new ejercicios2` y ejecuta tu código con `cargo run`.

1. Lee el enunciado y la salida esperada.
2. En papel, escribe los pasos en español. Haz una pequeña tabla de traza, como en
   [Seguir un programa paso a paso](pas-a-pas.md).
3. Escribe el código, ejecútalo y compara la salida carácter por carácter.
4. ¿Atascado? Lee la pista. Mira la solución al final.

Los niveles:

- ★☆☆: fácil, unas pocas líneas;
- ★★☆: medio, hay que combinar varias nociones;
- ★★★: más difícil, tómate tu tiempo y haz la tabla en papel.

> **Consejo:** tu solución puede ser distinta e igual de correcta. Si compila y
> muestra la salida correcta, lo has conseguido.

## Para calentar

### Ejercicio 1: La cuenta atrás

Nivel: ★☆☆

Muestra una cuenta atrás de 5 a 1 y luego `¡Despegue!`.

Salida esperada:

```text
5
4
3
2
1
¡Despegue!
```

> **Pista:** `(1..=5).rev()` da los números del 1 al 5 **al revés**: 5, 4, 3, 2, 1.
> No olvides los paréntesis alrededor de `1..=5`.

> **Solución**
>
> ```rust
> fn main() {
>     for n in (1..=5).rev() {
>         println!("{n}");
>     }
>     println!("¡Despegue!");
> }
> ```
>
> ```sortie
> 5
> 4
> 3
> 2
> 1
> ¡Despegue!
> ```
>
> | Vuelta | n | Muestra |
> |--------|---|---------|
> | 1 | 5 | 5 |
> | 2 | 4 | 4 |
> | 3 | 3 | 3 |
> | 4 | 2 | 2 |
> | 5 | 1 | 1 |
> | después | | ¡Despegue! |
>
> Por qué funciona: `.rev()` da la vuelta al intervalo, como cuando lees una lista
> empezando por abajo. El último `println!` está **después** del bucle: solo se
> ejecuta una vez.

### Ejercicio 2: El mayor de los tres

Nivel: ★☆☆

Escribe una función `mayor(a: i32, b: i32, c: i32) -> i32` que devuelva el mayor de
los tres números, **sin** usar `max`. Pruébala con `(4, 9, 2)` y `(7, 3, 7)`.

Salida esperada:

```text
9
7
```

> **Pista:** guarda un «campeón»: `let mut grande = a;`. Si `b` es mayor, `b` pasa a
> ser el campeón. Luego haz lo mismo con `c`.

> **Solución**
>
> ```rust
> fn mayor(a: i32, b: i32, c: i32) -> i32 {
>     let mut grande = a;
>     if b > grande {
>         grande = b;
>     }
>     if c > grande {
>         grande = c;
>     }
>     grande
> }
>
> fn main() {
>     println!("{}", mayor(4, 9, 2));
>     println!("{}", mayor(7, 3, 7));
> }
> ```
>
> ```sortie
> 9
> 7
> ```
>
> Por qué funciona: es como un torneo. El campeón inicial es `a`, y cada recién
> llegado lo desafía. Para `(4, 9, 2)`: `grande` vale 4, luego 9 (9 > 4), y luego
> sigue en 9 (2 no es mayor que 9).

### Ejercicio 3: Fizz y Buzz

Nivel: ★☆☆

Para cada número del 1 al 15, muestra `Fizz` si es divisible entre 3, `Buzz` si es
divisible entre 5, `FizzBuzz` si es divisible entre los dos, y si no, el número.
Ponlo todo en una sola línea, separado por espacios.

Salida esperada:

```text
1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
```

> **Pista:** «divisible entre 3» se escribe `n % 3 == 0`. Prueba **primero** el caso
> `n % 15 == 0`: si no, el 15 lo atraparía el caso «divisible entre 3».

> **Pista:** `print!` (sin `ln`) muestra sin pasar a la línea siguiente. Añade un
> `println!()` vacío al final.

> **Solución**
>
> ```rust
> fn main() {
>     for n in 1..=15 {
>         if n % 15 == 0 {
>             print!("FizzBuzz");
>         } else if n % 3 == 0 {
>             print!("Fizz");
>         } else if n % 5 == 0 {
>             print!("Buzz");
>         } else {
>             print!("{n}");
>         }
>         if n < 15 {
>             print!(" ");
>         }
>     }
>     println!();
> }
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```
>
> Por qué funciona: un `if … else if` se detiene en la **primera** prueba verdadera.
> Por eso el caso más preciso (15) tiene que ir primero. La prueba `n < 15` evita un
> espacio de más al final de la línea.

### Ejercicio 4: Las iniciales

Nivel: ★☆☆

Escribe una función `iniciales(nombre: &str) -> String` que devuelva la primera letra
de cada palabra, seguida de un punto. Prueba con `"Ada Lovelace"` y
`"Grace Brewster Hopper"`.

Salida esperada:

```text
A.L.
G.B.H.
```

> **Pista:** `nombre.split_whitespace()` divide el texto en palabras.
> `palabra.chars().next()` da la primera letra, dentro de un `Option`.

> **Pista:** parte de `let mut resultado = String::new();` y luego, para cada
> palabra, `resultado.push(letra);` y `resultado.push('.');`.

> **Solución**
>
> ```rust
> fn iniciales(nombre: &str) -> String {
>     let mut resultado = String::new();
>     for palabra in nombre.split_whitespace() {
>         if let Some(letra) = palabra.chars().next() {
>             resultado.push(letra);
>             resultado.push('.');
>         }
>     }
>     resultado
> }
>
> fn main() {
>     println!("{}", iniciales("Ada Lovelace"));
>     println!("{}", iniciales("Grace Brewster Hopper"));
> }
> ```
>
> ```sortie
> A.L.
> G.B.H.
> ```
>
> Por qué funciona: `chars().next()` devuelve `Some(letra)` si hay una letra, y
> `None` si la palabra está vacía. `if let Some(letra)` solo se queda con el primer
> caso. `push` añade **un** carácter (entre `'…'`), `push_str` añade un texto.

## Combinar nociones

### Ejercicio 5: La suma de las cifras

Nivel: ★★☆

Escribe una función `suma_cifras(n: u32) -> u32` que sume las cifras de un número:
para 4721, eso da 4 + 7 + 2 + 1 = 14. No pases por texto.

Salida esperada:

```text
14
9
```

(la segunda línea es para `suma_cifras(900)`).

> **Pista:** `n % 10` da la **última** cifra (4721 % 10 = 1). `n / 10` quita esa
> última cifra (4721 / 10 = 472).

> **Pista:** repite con `while n > 0 { … }`. El parámetro tiene que cambiar: escribe
> `mut n: u32` en la firma.

> **Solución**
>
> ```rust
> fn suma_cifras(mut n: u32) -> u32 {
>     let mut suma = 0;
>     while n > 0 {
>         suma += n % 10;
>         n /= 10;
>     }
>     suma
> }
>
> fn main() {
>     println!("{}", suma_cifras(4721));
>     println!("{}", suma_cifras(900));
> }
> ```
>
> ```sortie
> 14
> 9
> ```
>
> | Vuelta | n al principio | n % 10 | suma | n al final |
> |--------|----------------|--------|------|------------|
> | 1 | 4721 | 1 | 1 | 472 |
> | 2 | 472 | 2 | 3 | 47 |
> | 3 | 47 | 7 | 10 | 4 |
> | 4 | 4 | 4 | 14 | 0 |
>
> Por qué funciona: nos «comemos» el número por la derecha, una cifra por vuelta.
> Cuando `n` vale 0, ya no queda nada que comer: el bucle se detiene.

### Ejercicio 6: Dar la vuelta a una lista a mano

Nivel: ★★☆

Parte de `let lista = vec![1, 2, 3, 4];`. Construye una lista **nueva** al revés, sin
usar `reverse` ni `rev`. Muestra las dos listas.

Salida esperada:

```text
[1, 2, 3, 4]
[4, 3, 2, 1]
```

> **Pista:** crea `let mut al_reves = Vec::new();`. Para cada elemento, insértalo **al
> principio** de `al_reves`: `al_reves.insert(0, x);`.

> **Solución**
>
> ```rust
> fn main() {
>     let lista = vec![1, 2, 3, 4];
>     let mut al_reves = Vec::new();
>     for x in &lista {
>         al_reves.insert(0, *x);
>     }
>     println!("{lista:?}");
>     println!("{al_reves:?}");
> }
> ```
>
> ```sortie
> [1, 2, 3, 4]
> [4, 3, 2, 1]
> ```
>
> | Vuelta | x | al_reves después de la vuelta |
> |--------|---|-------------------------------|
> | 1 | 1 | [1] |
> | 2 | 2 | [2, 1] |
> | 3 | 3 | [3, 2, 1] |
> | 4 | 4 | [4, 3, 2, 1] |
>
> Por qué funciona: es como apilar platos. Cada plato nuevo pasa delante de los
> demás. `&lista` presta la lista al bucle: sigue siendo utilizable para el
> `println!`. `{:?}` muestra una lista entera.

### Ejercicio 7: La palabra más larga

Nivel: ★★☆

Escribe una función `mas_larga(frase: &str) -> &str` que devuelva la palabra más
larga de una frase. En caso de empate, quédate con la primera. Prueba con
`"mi perro come atún en el jardín"`.

Salida esperada:

```text
jardín (6 letras)
```

> **Pista:** guarda un campeón, como en el ejercicio 2: `let mut mejor = "";`. Compara
> las longitudes con `palabra.chars().count()`, no con `len()`: `len()` cuenta los
> bytes, y `í` ocupa 2.

> **Solución**
>
> ```rust
> fn mas_larga(frase: &str) -> &str {
>     let mut mejor = "";
>     for palabra in frase.split_whitespace() {
>         if palabra.chars().count() > mejor.chars().count() {
>             mejor = palabra;
>         }
>     }
>     mejor
> }
>
> fn main() {
>     let palabra = mas_larga("mi perro come atún en el jardín");
>     println!("{palabra} ({} letras)", palabra.chars().count());
> }
> ```
>
> ```sortie
> jardín (6 letras)
> ```
>
> Por qué funciona: la prueba es `>` y no `>=`, así que una palabra de la misma
> longitud no reemplaza al campeón: gana la primera. La palabra devuelta es un trozo
> de la frase (un `&str`), sin copia.
>
> Prueba: con `>=`, prueba `"gato mesa"`. Obtienes `mesa` en lugar de `gato`.

### Ejercicio 8: La sucesión de Fibonacci

Nivel: ★★☆

En la sucesión de Fibonacci, cada número es la suma de los dos anteriores:
0, 1, 1, 2, 3, 5… Muestra los 10 primeros números en una línea.

Salida esperada:

```text
0 1 1 2 3 5 8 13 21 34
```

> **Pista:** guarda dos variables `a` (el número actual) y `b` (el siguiente). En cada
> vuelta, muestra `a` y luego avanza: el nuevo `a` es `b`, el nuevo `b` es `a + b`.

> **Pista:** ¡cuidado con el orden! Calcula `let siguiente = a + b;` **antes** de
> modificar `a`.

> **Solución**
>
> ```rust
> fn main() {
>     let mut a: u64 = 0;
>     let mut b: u64 = 1;
>     let mut numeros = Vec::new();
>     for _ in 0..10 {
>         numeros.push(a.to_string());
>         let siguiente = a + b;
>         a = b;
>         b = siguiente;
>     }
>     println!("{}", numeros.join(" "));
> }
> ```
>
> ```sortie
> 0 1 1 2 3 5 8 13 21 34
> ```
>
> | Vuelta | a (mostrado) | b | siguiente = a + b | a después | b después |
> |--------|--------------|---|-------------------|-----------|-----------|
> | 1 | 0 | 1 | 1 | 1 | 1 |
> | 2 | 1 | 1 | 2 | 1 | 2 |
> | 3 | 1 | 2 | 3 | 2 | 3 |
> | 4 | 2 | 3 | 5 | 3 | 5 |
> | 5 | 3 | 5 | 8 | 5 | 8 |
>
> Por qué funciona: las dos variables avanzan juntas, como dos pies que caminan.
> `siguiente` guarda el cálculo mientras movemos `a`: sin él, perderíamos el valor
> antiguo de `a`. `join(" ")` pega los textos con un espacio entre ellos. El `_` de
> `for _ in` quiere decir «no uso el número de la vuelta».

### Ejercicio 9: Las calificaciones

Nivel: ★★☆

Escribe una función `calificacion(nota: u32) -> &'static str` que devuelva
`"insuficiente"` (0 a 9), `"bien"` (10 a 13), `"notable"` (14 a 15) o
`"sobresaliente"` (16 a 20), y `"nota no válida"` por encima. Usa `match` con
intervalos. Muestra la calificación de 8, 12, 15, 19 y 25.

Salida esperada:

```text
8: insuficiente
12: bien
15: notable
19: sobresaliente
25: nota no válida
```

> **Pista:** un `match` acepta intervalos inclusivos: `0..=9 => "insuficiente",`.

> **Pista:** `&'static str` quiere decir «un texto escrito directamente en el
> programa», que existe todo el tiempo. Es el tipo de `"bien"`.

> **Solución**
>
> ```rust
> fn calificacion(nota: u32) -> &'static str {
>     match nota {
>         0..=9 => "insuficiente",
>         10..=13 => "bien",
>         14..=15 => "notable",
>         16..=20 => "sobresaliente",
>         _ => "nota no válida",
>     }
> }
>
> fn main() {
>     for nota in [8, 12, 15, 19, 25] {
>         println!("{nota}: {}", calificacion(nota));
>     }
> }
> ```
>
> ```sortie
> 8: insuficiente
> 12: bien
> 15: notable
> 19: sobresaliente
> 25: nota no válida
> ```
>
> Por qué funciona: `match` prueba los casos de arriba abajo, como buzones numerados.
> `_` atrapa todo lo demás: Rust lo exige, porque un `u32` puede ir mucho más allá
> de 20.

## Para ir más lejos

### Ejercicio 10: El cifrado César

Nivel: ★★★

El cifrado César desplaza cada letra 3 posiciones en el alfabeto: `a` se convierte en
`d`, `b` en `e`… y `x`, `y`, `z` vuelven al principio (`a`, `b`, `c`). Escribe
`cesar(texto: &str) -> String` para letras minúsculas sin acentos; los demás
caracteres (espacios…) no cambian.

Salida esperada:

```text
hola zoe -> krod crh
```

> **Pista:** un carácter puede convertirse en número: `c as u8`. `'a' as u8` vale 97.
> La posición en el alfabeto es entonces `c as u8 - b'a'` (0 para `a`, 25 para `z`).

> **Pista:** desplaza y vuelve al principio con `%`:
> `(posicion + 3) % 26`. Luego conviértelo otra vez en letra: `(b'a' + nueva) as char`.

> **Solución**
>
> ```rust
> fn cesar(texto: &str) -> String {
>     let mut resultado = String::new();
>     for c in texto.chars() {
>         if c.is_ascii_lowercase() {
>             let posicion = c as u8 - b'a';
>             let nueva = (posicion + 3) % 26;
>             resultado.push((b'a' + nueva) as char);
>         } else {
>             resultado.push(c);
>         }
>     }
>     resultado
> }
>
> fn main() {
>     let mensaje = "hola zoe";
>     println!("{mensaje} -> {}", cesar(mensaje));
> }
> ```
>
> ```sortie
> hola zoe -> krod crh
> ```
>
> | c | posicion | posicion + 3 | % 26 | letra |
> |---|----------|--------------|------|-------|
> | h | 7 | 10 | 10 | k |
> | o | 14 | 17 | 17 | r |
> | z | 25 | 28 | 2 | c |
> | (espacio) | | | | (espacio) |
>
> Por qué funciona: el alfabeto es como un reloj de 26 horas. Después de 25, se vuelve
> a 0: ese es el papel de `% 26`. `b'a'` es la letra `a` en forma de byte (`u8`),
> para poder calcular con ella.
>
> Prueba: desplaza 23 en lugar de 3. Aplicado al texto cifrado, lo descifra:
> recuperas `hola zoe`.

### Ejercicio 11: Los números primos

Nivel: ★★★

Un número primo solo es divisible entre 1 y entre sí mismo (2, 3, 5, 7…). Escribe
`es_primo(n: u32) -> bool` y luego muestra los números primos hasta 30.

Salida esperada:

```text
2 3 5 7 11 13 17 19 23 29
```

> **Pista:** 0 y 1 no son primos. Para los demás, prueba cada divisor `d` de 2 a
> `n - 1`: si `n % d == 0`, no es primo.

> **Pista:** en cuanto se encuentra un divisor, `return false;` sale de la función
> enseguida. Si el bucle termina sin encontrar nada, es primo.

> **Solución**
>
> ```rust
> fn es_primo(n: u32) -> bool {
>     if n < 2 {
>         return false;
>     }
>     let mut d = 2;
>     while d * d <= n {
>         if n % d == 0 {
>             return false;
>         }
>         d += 1;
>     }
>     true
> }
>
> fn main() {
>     let mut primos = Vec::new();
>     for n in 1..=30 {
>         if es_primo(n) {
>             primos.push(n.to_string());
>         }
>     }
>     println!("{}", primos.join(" "));
> }
> ```
>
> ```sortie
> 2 3 5 7 11 13 17 19 23 29
> ```
>
> Traza de `es_primo(9)` y luego de `es_primo(11)`:
>
> | Llamada | d | ¿d × d <= n? | n % d | Resultado |
> |---------|---|--------------|-------|-----------|
> | 9 | 2 | 4 <= 9: sí | 1 | seguimos |
> | 9 | 3 | 9 <= 9: sí | **0** | `false` |
> | 11 | 2 | 4 <= 11: sí | 1 | seguimos |
> | 11 | 3 | 9 <= 11: sí | 2 | seguimos |
> | 11 | 4 | 16 <= 11: **no** | | `true` |
>
> Por qué funciona: la solución se detiene cuando `d × d` supera `n`. Si `n` tuviera
> un divisor más grande, también tendría un «compañero» más pequeño, ya probado
> (para 30: 10 × 3, y el 3 se vio antes que el 10). Tu versión hasta `n - 1` también
> es correcta, solo que más lenta.

### Ejercicio 12: Comprimir un texto

Nivel: ★★★

Escribe `comprimir(texto: &str) -> String`, que reemplace cada serie de letras
idénticas por la letra seguida del número de repeticiones: `"aaabccdddd"` se
convierte en `"a3b1c2d4"`.

Salida esperada:

```text
aaabccdddd -> a3b1c2d4
zzz -> z3
```

> **Pista:** guarda la letra en curso (`actual`) y su contador (`cuenta`). Si la letra
> siguiente es la misma, `cuenta += 1`. Si no, escribe la letra y su cuenta, y vuelve
> a empezar con la nueva letra.

> **Pista:** después del bucle, todavía queda por escribir la **última** serie. ¡Es el
> olvido más frecuente!

> **Solución**
>
> ```rust
> fn comprimir(texto: &str) -> String {
>     let mut resultado = String::new();
>     let mut letras = texto.chars();
>     let Some(mut actual) = letras.next() else {
>         return resultado;
>     };
>     let mut cuenta = 1;
>     for c in letras {
>         if c == actual {
>             cuenta += 1;
>         } else {
>             resultado.push_str(&format!("{actual}{cuenta}"));
>             actual = c;
>             cuenta = 1;
>         }
>     }
>     resultado.push_str(&format!("{actual}{cuenta}"));
>     resultado
> }
>
> fn main() {
>     for texto in ["aaabccdddd", "zzz"] {
>         println!("{texto} -> {}", comprimir(texto));
>     }
> }
> ```
>
> ```sortie
> aaabccdddd -> a3b1c2d4
> zzz -> z3
> ```
>
> | c | ¿c == actual? | actual | cuenta | resultado |
> |---|---------------|--------|--------|-----------|
> | (inicio) | | a | 1 | |
> | a | sí | a | 2 | |
> | a | sí | a | 3 | |
> | b | no | b | 1 | a3 |
> | c | no | c | 1 | a3b1 |
> | c | sí | c | 2 | a3b1 |
> | d | no | d | 1 | a3b1c2 |
> | d | sí | d | 2 | a3b1c2 |
> | d | sí | d | 3 | a3b1c2 |
> | d | sí | d | 4 | a3b1c2 |
> | (después del bucle) | | | | a3b1c2d4 |
>
> Por qué funciona: la primera letra se lee aparte con `letras.next()`. Si el texto
> está vacío, `let … else` devuelve enseguida un texto vacío. Después, una serie solo
> se escribe cuando llega **otra** letra: la última serie, a la que no sigue nada, se
> escribe después del bucle.
>
> Prueba: borra la línea `resultado.push_str(…)` colocada después del bucle. Obtienes
> `a3b1c2` y un texto vacío para `zzz`: la última serie se pierde.

## En resumen

- Haz una tabla de traza en papel antes de escribir el código: muestra los pasos.
- El «campeón» (ejercicios 2 y 7) y el acumulador (ejercicios 5 y 8) son dos ideas
  que aparecen en todas partes.
- Piensa en los casos límite: texto vacío, empate, última serie, número 0 o 1.
- Para repasar una noción, la [chuleta](aide-memoire.md) resume todo el recorrido.

Siguiente: [chuleta](aide-memoire.md)
