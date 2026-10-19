# Ejercicios resueltos

Esta página propone doce ejercicios, del más fácil al más difícil, para practicar todo el recorrido de Rust, cada uno con su solución.

## Cómo trabajar

Intenta cada ejercicio solo, en un proyecto creado con `cargo new ejercicios`. Mira la
pista si te atascas y luego la solución para comparar.

- ★☆☆: fácil, unas pocas líneas;
- ★★☆: medio, hay que combinar varias nociones;
- ★★★: más difícil, tómate tu tiempo.

> **Consejo:** tu solución puede ser distinta e igual de correcta. Si compila y
> muestra el resultado correcto, lo has conseguido.

## Lo básico

### Ejercicio 1: Presentarse

Nivel: ★☆☆

Crea dos variables, `nombre` y `edad`, y luego muestra la frase
`Me llamo Zoé y el año que viene tendré 26 años.` (calcula `26` a partir de `edad`).

> **Solución**
>
> ```rust
> fn main() {
>     let nombre = "Zoé";
>     let edad = 25;
>     println!("Me llamo {nombre} y el año que viene tendré {} años.", edad + 1);
> }
> ```
>
> ```sortie
> Me llamo Zoé y el año que viene tendré 26 años.
> ```

### Ejercicio 2: Par o impar

Nivel: ★☆☆

Para cada número del 1 al 5, muestra si es par o impar.

> **Pista:** un número es par si el resto de su división entre 2 vale 0: `n % 2 == 0`.

> **Solución**
>
> ```rust
> fn main() {
>     for n in 1..=5 {
>         if n % 2 == 0 {
>             println!("{n} es par");
>         } else {
>             println!("{n} es impar");
>         }
>     }
> }
> ```
>
> ```sortie
> 1 es impar
> 2 es par
> 3 es impar
> 4 es par
> 5 es impar
> ```

### Ejercicio 3: La tabla de multiplicar

Nivel: ★☆☆

Muestra la tabla del 7, de `7 x 1 = 7` a `7 x 5 = 35`.

> **Solución**
>
> ```rust
> fn main() {
>     let tabla = 7;
>     for i in 1..=5 {
>         println!("{tabla} x {i} = {}", tabla * i);
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

### Ejercicio 4: Una función de conversión

Nivel: ★☆☆

Escribe una función `celsius_a_fahrenheit(c: f64) -> f64`. La fórmula es
`c * 9 / 5 + 32`. Muestra el resultado para 0, 20 y 100 grados.

> **Pista:** con `f64`, escribe `9.0` y `5.0`, no `9` y `5`.

> **Solución**
>
> ```rust
> fn celsius_a_fahrenheit(c: f64) -> f64 {
>     c * 9.0 / 5.0 + 32.0
> }
>
> fn main() {
>     for c in [0.0, 20.0, 100.0] {
>         println!("{c} °C = {} °F", celsius_a_fahrenheit(c));
>     }
> }
> ```
>
> ```sortie
> 0 °C = 32 °F
> 20 °C = 68 °F
> 100 °C = 212 °F
> ```

## Listas y textos

### Ejercicio 5: La media de las notas

Nivel: ★★☆

En `let notas = vec![12, 15, 9, 18];`, calcula la nota más alta y la media
(un `f64`).

> **Pista:** `iter().max()` devuelve un `Option`. Para la media, convierte con
> `as f64`.

> **Solución**
>
> ```rust
> fn main() {
>     let notas = vec![12, 15, 9, 18];
>     let mejor = notas.iter().max().unwrap();
>     let suma: i32 = notas.iter().sum();
>     let media = suma as f64 / notas.len() as f64;
>     println!("Mejor nota: {mejor}");
>     println!("Media: {media}");
> }
> ```
>
> ```sortie
> Mejor nota: 18
> Media: 13.5
> ```

### Ejercicio 6: Contar las vocales

Nivel: ★★☆

Escribe una función `vocales(texto: &str) -> usize` que cuente las vocales de un
texto (sin tildes, en minúsculas o mayúsculas).

> **Pista:** recorre `texto.to_lowercase().chars()` y comprueba si `"aeiou"`
> contiene el carácter.

> **Solución**
>
> ```rust
> fn vocales(texto: &str) -> usize {
>     let mut total = 0;
>     for c in texto.to_lowercase().chars() {
>         if "aeiou".contains(c) {
>             total += 1;
>         }
>     }
>     total
> }
>
> fn main() {
>     println!("{}", vocales("Hola mundo"));
>     println!("{}", vocales("RUST"));
> }
> ```
>
> ```sortie
> 4
> 1
> ```

### Ejercicio 7: El palíndromo

Nivel: ★★☆

Escribe una función `es_palindromo(palabra: &str) -> bool`: un palíndromo se lee igual
en los dos sentidos, como «kayak». Ignora las mayúsculas.

> **Pista:** invierte el texto con `chars().rev().collect::<String>()`.

> **Solución**
>
> ```rust
> fn es_palindromo(palabra: &str) -> bool {
>     let palabra = palabra.to_lowercase();
>     let al_reves: String = palabra.chars().rev().collect();
>     palabra == al_reves
> }
>
> fn main() {
>     for palabra in ["Kayak", "Rust", "oso"] {
>         println!("{palabra}: {}", es_palindromo(palabra));
>     }
> }
> ```
>
> ```sortie
> Kayak: true
> Rust: false
> oso: true
> ```

### Ejercicio 8: Contar las palabras

Nivel: ★★☆

Cuenta cuántas veces aparece cada palabra en `"uno dos uno tres dos uno"`.
Muestra las palabras en orden alfabético.

> **Pista:** usa un `HashMap` y `entry(palabra).or_insert(0)`. El orden de un
> `HashMap` es aleatorio: guarda los pares en un `Vec` y ordénalo.

> **Solución**
>
> ```rust
> use std::collections::HashMap;
>
> fn main() {
>     let texto = "uno dos uno tres dos uno";
>     let mut cuenta = HashMap::new();
>     for palabra in texto.split_whitespace() {
>         *cuenta.entry(palabra).or_insert(0) += 1;
>     }
>     let mut pares: Vec<_> = cuenta.into_iter().collect();
>     pares.sort();
>     for (palabra, n) in pares {
>         println!("{palabra}: {n}");
>     }
> }
> ```
>
> ```sortie
> dos: 2
> tres: 1
> uno: 3
> ```

## Propiedad, tipos y errores

### Ejercicio 9: Tomar prestado en lugar de quedárselo

Nivel: ★★☆

Este programa no compila, porque `nombre` se mueve a la función:

```rust norun
fn saludar(nombre: String) {
    println!("Hola {nombre}");
}

fn main() {
    let nombre = String::from("Inès");
    saludar(nombre);
    saludar(nombre); // error: use of moved value
}
```

Corrígelo **tomando prestado** el texto, sin usar `clone`.

> **Pista:** el parámetro pasa a ser `&str`, y la llamada `saludar(&nombre)`.

> **Solución**
>
> ```rust
> fn saludar(nombre: &str) {
>     println!("Hola {nombre}");
> }
>
> fn main() {
>     let nombre = String::from("Inès");
>     saludar(&nombre);
>     saludar(&nombre);
> }
> ```
>
> ```sortie
> Hola Inès
> Hola Inès
> ```

### Ejercicio 10: Una cuenta bancaria

Nivel: ★★☆

Crea una estructura `Cuenta` con un campo `saldo: i32`, y dos métodos:
`depositar(&mut self, cantidad: i32)` y `mostrar(&self)`. Deposita 50 y luego 20 en
una cuenta que empieza en 0.

> **Solución**
>
> ```rust
> struct Cuenta {
>     saldo: i32,
> }
>
> impl Cuenta {
>     fn depositar(&mut self, cantidad: i32) {
>         self.saldo += cantidad;
>     }
>
>     fn mostrar(&self) {
>         println!("Saldo: {} €", self.saldo);
>     }
> }
>
> fn main() {
>     let mut cuenta = Cuenta { saldo: 0 };
>     cuenta.depositar(50);
>     cuenta.depositar(20);
>     cuenta.mostrar();
> }
> ```
>
> ```sortie
> Saldo: 70 €
> ```

### Ejercicio 11: Una división segura

Nivel: ★★★

Escribe `dividir(a: i32, b: i32) -> Option<i32>` que devuelva `None` si `b` vale 0.
Luego escribe `leer_numero(texto: &str) -> Result<i32, String>` que devuelva un mensaje
de error claro si el texto no es un número. Prueba las dos.

> **Pista:** para `leer_numero`, usa `match texto.parse::<i32>()` con un caso
> `Ok(n)` y un caso `Err(_)`.

> **Solución**
>
> ```rust
> fn dividir(a: i32, b: i32) -> Option<i32> {
>     if b == 0 { None } else { Some(a / b) }
> }
>
> fn leer_numero(texto: &str) -> Result<i32, String> {
>     match texto.parse::<i32>() {
>         Ok(n) => Ok(n),
>         Err(_) => Err(format!("«{texto}» no es un número")),
>     }
> }
>
> fn main() {
>     println!("{:?}", dividir(10, 2));
>     println!("{:?}", dividir(1, 0));
>     match leer_numero("12") {
>         Ok(n) => println!("Leído: {n}"),
>         Err(e) => println!("Error: {e}"),
>     }
>     match leer_numero("doce") {
>         Ok(n) => println!("Leído: {n}"),
>         Err(e) => println!("Error: {e}"),
>     }
> }
> ```
>
> ```sortie
> Some(5)
> None
> Leído: 12
> Error: «doce» no es un número
> ```

### Ejercicio 12: Formas con un trait

Nivel: ★★★

Crea un trait `Forma` con un método `area(&self) -> f64`. Impleméntalo para
`Cuadrado { lado: f64 }` y `Rectangulo { ancho: f64, alto: f64 }`. Guarda un cuadrado
de lado 3 y un rectángulo de 2 por 5 en un `Vec<Box<dyn Forma>>`, y luego muestra la
suma de las áreas.

> **Pista:** vuelve a leer [Una lista de objetos diferentes](generiques.md).

> **Solución**
>
> ```rust
> trait Forma {
>     fn area(&self) -> f64;
> }
>
> struct Cuadrado {
>     lado: f64,
> }
>
> struct Rectangulo {
>     ancho: f64,
>     alto: f64,
> }
>
> impl Forma for Cuadrado {
>     fn area(&self) -> f64 {
>         self.lado * self.lado
>     }
> }
>
> impl Forma for Rectangulo {
>     fn area(&self) -> f64 {
>         self.ancho * self.alto
>     }
> }
>
> fn main() {
>     let formas: Vec<Box<dyn Forma>> = vec![
>         Box::new(Cuadrado { lado: 3.0 }),
>         Box::new(Rectangulo { ancho: 2.0, alto: 5.0 }),
>     ];
>     let mut total = 0.0;
>     for f in &formas {
>         println!("Área: {}", f.area());
>         total += f.area();
>     }
>     println!("Total: {total}");
> }
> ```
>
> ```sortie
> Área: 9
> Área: 10
> Total: 19
> ```

## En resumen

- Empieza por los ejercicios ★☆☆ y luego sube de dificultad.
- Lee los mensajes del compilador: a menudo indican la corrección.
- Prueba tu código con varios valores, incluidos los casos límite (0, texto vacío).
- Para repasar una noción, la [chuleta](aide-memoire.md) resume todo el recorrido.

Para seguir, prueba los mismos ejercicios en [Ruby](../ruby/exercices.md) o en
[Rythme](../rythme/exercices.md).

Siguiente: [ejercicios resueltos: segunda serie](exercices-2.md)
