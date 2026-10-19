# Trabajar con texto

Esta página muestra cómo crear, transformar, cortar y recorrer texto en Rust, con los dos tipos `String` y `&str`.

> **Objetivos:**
>
> - elegir entre `String` y `&str`;
> - construir un texto con `push_str`, `format!` y `join`;
> - buscar, reemplazar y cortar un texto;
> - convertir un texto en número, y al revés;
> - recorrer los caracteres sin partir una letra con tilde.

## Dos tipos de texto: String y &str

Un texto, o [cadena de caracteres](../debuter/glossaire.md#cadena-de-caracteres), es una
sucesión de letras. Rust tiene dos tipos de texto. Para empezar, recuerda esto:

- `&str`: un texto que **miras**. Los textos entre comillas son `&str`.
- `String`: un texto que **posees** y que puedes modificar.

Piensa en un libro: `&str` es el libro de la biblioteca, que lees sin escribir en él;
`String` es tu propio cuaderno, donde puedes añadir páginas.

```rust
let mirado: &str = "hola";
let propio: String = String::from("hola");
let otro: String = "buenas".to_string();
println!("{mirado} {propio} {otro}");
```

```sortie
hola hola buenas
```

`String::from("…")` y `.to_string()` fabrican una `String` a partir de un `&str`.
En el otro sentido, para pasar de una `String` a un `&str`, añade `&` delante:

```rust
fn mostrar(t: &str) {
    println!("[{t}]");
}

fn main() {
    let nombre = String::from("Léa");
    mostrar(&nombre);
    mostrar("Tom");
}
```

```sortie
[Léa]
[Tom]
```

> **Recuerda:** un parámetro de función de tipo `&str` acepta las dos clases de texto.
> Es la opción más práctica.

## Hacer crecer una String

Una `String` declarada con `mut` puede crecer. `push_str` añade un texto, `push` añade
un solo carácter:

```rust
let mut frase = String::from("¡Hola");
frase.push_str(" a todos");
frase.push('!');
println!("{frase}");
```

```sortie
¡Hola a todos!
```

El operador `+` también pega dos textos, pero a la izquierda hace falta una `String`:

```rust
let inicio = String::from("Hola");
let saludo = inicio + " mundo";
println!("{saludo}");
```

```sortie
Hola mundo
```

## Montar un texto con format!

`format!` funciona como `println!`, pero **devuelve** una `String` en lugar de
mostrarla:

```rust
let nombre = "Alice";
let edad = 30;
let ficha = format!("{nombre} ({edad} años)");
println!("{ficha}");
```

```sortie
Alice (30 años)
```

> **Consejo:** `format!` es la forma más sencilla de pegar varios trozos de texto y
> números.

## La longitud de un texto

`len()` cuenta los **bytes**, no las letras. Un byte es la casilla más pequeña de la
memoria. Una letra simple ocupa un byte, una letra con tilde o una `ñ` ocupa dos.
Para contar los caracteres, usa `chars().count()`:

```rust
let palabra = "año";
println!("{}", palabra.len());
println!("{}", palabra.chars().count());
```

```sortie
4
3
```

> **Atención:** para un texto en español, prefiere `chars().count()`.

## Cambiar mayúsculas y minúsculas, y limpiar

```rust
let bruto = "  Hola Rust  ";
println!("[{}]", bruto.trim());
println!("{}", bruto.trim().to_uppercase());
println!("{}", bruto.trim().to_lowercase());
```

```sortie
[Hola Rust]
HOLA RUST
hola rust
```

- `trim()` quita los espacios del principio y del final;
- `to_uppercase()` pasa a mayúsculas;
- `to_lowercase()` pasa a minúsculas.

## Buscar en un texto

Estos métodos responden con `true` o `false`:

```rust
let frase = "El gato duerme";
println!("{}", frase.contains("gato"));
println!("{}", frase.starts_with("El"));
println!("{}", frase.ends_with("!"));
```

```sortie
true
true
false
```

## Reemplazar

`replace` devuelve una **nueva** `String`. El texto de partida no cambia:

```rust
let antes = "Me gusta el té";
let despues = antes.replace("té", "café");
println!("{antes}");
println!("{despues}");
```

```sortie
Me gusta el té
Me gusta el café
```

## Cortar con split

`split` corta el texto en cada separador. `collect` guarda los trozos en un `Vec`:

```rust
let lista = "manzana,pera,kiwi";
let frutas: Vec<&str> = lista.split(',').collect();
println!("{:?}", frutas);
println!("{}", frutas.len());
```

```sortie
["manzana", "pera", "kiwi"]
3
```

Para cortar en los espacios, `split_whitespace` es más práctico:

```rust
for palabra in "uno  dos tres".split_whitespace() {
    println!("{palabra}");
}
```

```sortie
uno
dos
tres
```

Para hacer lo contrario, `join` vuelve a pegar los trozos:

```rust
let palabras = vec!["a", "b", "c"];
println!("{}", palabras.join("-"));
```

```sortie
a-b-c
```

## Convertir un texto en número

`parse` lee un número en un texto. Indica el tipo que quieres con `::<i32>`. El
resultado es un `Result`, porque quizá el texto no sea un número:

```rust
let bueno = "42".parse::<i32>();
let malo = "abc".parse::<i32>();
println!("{:?}", bueno);
println!("{}", malo.is_err());
let n = "  7 ".trim().parse::<i32>().unwrap();
println!("{}", n * 2);
```

```sortie
Ok(42)
true
14
```

> **Consejo:** acuérdate de `trim()` antes de `parse`: un espacio o un salto de línea
> hace fallar la lectura. Los errores se explican en [Gestionar los errores](erreurs.md).

En el otro sentido, `to_string()` convierte un número en texto:

```rust
let texto = 2026.to_string();
println!("{}", texto.len());
```

```sortie
4
```

## Recorrer los caracteres

`chars()` da los caracteres uno a uno (valores de tipo `char`):

```rust
let palabra = "canción";
for c in palabra.chars() {
    print!("{c}.");
}
println!();
let vocales = palabra.chars().filter(|c| "aeiouó".contains(*c)).count();
println!("{vocales} vocales");
```

```sortie
c.a.n.c.i.ó.n.
3 vocales
```

Para dar la vuelta a un texto, combina `chars`, `rev` y `collect`:

```rust
let al_reves: String = "radar".chars().rev().collect();
println!("{al_reves}");
```

```sortie
radar
```

## Tomar un trozo con cuidado

Puedes tomar una parte de un texto con `&texto[inicio..fin]`. Los números son
**bytes**:

```rust
let saludo = "buenos días";
println!("{}", &saludo[0..6]);
println!("{}", &saludo[7..]);
```

```sortie
buenos
días
```

> **Atención:** si el corte cae en medio de una letra con tilde, el programa se detiene
> con un `panic!`.

```rust norun
let palabra = "árbol";
println!("{}", &palabra[0..1]); // pánico: end byte index 1 is not a char boundary
```

Para un texto con tildes, pasa por los caracteres:

```rust
let palabra = "árbol";
let inicio: String = palabra.chars().take(2).collect();
println!("{inicio}");
```

```sortie
ár
```

## Los textos en bruto

Un texto en bruto se escribe `r"…"`. Las barras invertidas `\` se quedan tal cual.
Con `r#"…"#`, puede incluso contener comillas:

```rust
let ruta = r"C:\carpeta\nuevo";
let cita = r#"Dijo "hola""#;
println!("{ruta}");
println!("{cita}");
```

```sortie
C:\carpeta\nuevo
Dijo "hola"
```

> **Nota:** en un texto normal, `\n` es un salto de línea. En un texto en bruto, es
> solo una barra y una `n`.

## Errores frecuentes

### Poner un &str en una String

```rust norun
fn main() {
    let nombre: String = "Léa";
    println!("{nombre}");
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:2:26
  |
2 |     let nombre: String = "Léa";
  |                 ------   ^^^^^ expected `String`, found `&str`
  |                 |
  |                 expected due to this
help: try using a conversion method
```

Un texto entre comillas es un `&str`. Conviértelo: `"Léa".to_string()` o
`String::from("Léa")`.

### Leer una letra con [0]

```rust norun
fn main() {
    let palabra = "hola";
    let primera = palabra[0];
    println!("{primera}");
}
```

```text
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> src/main.rs:3:27
  |
3 |     let primera = palabra[0];
  |                           ^ string indices are ranges of `usize`
  |
  = note: you can use `.chars().nth()` or `.bytes().nth()`
```

Una letra puede ocupar varios bytes: Rust se niega a adivinar. Escribe
`palabra.chars().next()` para la primera letra, o `palabra.chars().nth(2)` para la
tercera. Las dos devuelven un `Option` (ver [el tipo Option](types.md)).

### Sumar dos &str

```rust norun
fn main() {
    let frase = "Hola" + " mundo";
    println!("{frase}");
}
```

```text
error[E0369]: cannot add `&str` to `&str`
 --> src/main.rs:2:24
  |
2 |     let frase = "Hola" + " mundo";
  |                 ------ ^ -------- &str
  |                 |      |
  |                 |      `+` cannot be used to concatenate two `&str` strings
  |                 &str
  |
  = note: string concatenation requires an owned `String` on the left
```

A la izquierda del `+` hace falta una `String`: `String::from("Hola") + " mundo"`. Más
sencillo: `format!("{}{}", "Hola", " mundo")`.

### Hacer crecer una String sin mut

```rust norun
fn main() {
    let frase = String::from("¡Hola");
    frase.push_str("!");
    println!("{frase}");
}
```

```text
error[E0596]: cannot borrow `frase` as mutable, as it is not declared as mutable
 --> src/main.rs:3:5
  |
3 |     frase.push_str("!");
  |     ^^^^^ cannot borrow as mutable
```

Corrección: `let mut frase = String::from("¡Hola");`.

## Ponte a prueba

**1.** ¿Cuánto valen `"niño".len()` y `"niño".chars().count()`?

> **Respuesta**
>
> `5` y `4`: la `ñ` ocupa dos bytes, pero es un solo carácter.

**2.** ¿Cómo obtener `["a", "b", "c"]` a partir de `"a;b;c"`?

> **Respuesta**
>
> ```rust
> let trozos: Vec<&str> = "a;b;c".split(';').collect();
> println!("{:?}", trozos);
> ```
>
> ```sortie
> ["a", "b", "c"]
> ```

**3.** ¿Por qué escribir `.trim()` antes de `.parse()` con lo que se escribe en el
teclado?

> **Respuesta**
>
> Lo que se escribe termina con un salto de línea. `parse` falla con ese carácter;
> `trim()` lo quita.

## En resumen

- `&str` para mirar un texto, `String` para poseerlo y modificarlo.
- `push_str`, `format!` y `join` construyen texto.
- `len()` cuenta los bytes, `chars().count()` cuenta los caracteres.
- `trim`, `to_uppercase`, `replace`, `split` y `contains` transforman y buscan.
- `parse::<i32>()` lee un número y devuelve un `Result`.

Compara con [el texto en Ruby](../ruby/texte.md) y [en Rythme](../rythme/texte.md).

Siguiente: [Colecciones](collections.md).
