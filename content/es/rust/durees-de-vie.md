# Los tiempos de vida

Un tiempo de vida es el periodo durante el cual una referencia sigue siendo válida: este capítulo explica por qué Rust los comprueba, y cómo escribirlos cuando el compilador te lo pide.

> **Objetivos:**
>
> - entender qué es una referencia colgante, y por qué Rust la rechaza;
> - leer y escribir una anotación `'a` en una función;
> - saber cuándo puedes prescindir de la anotación;
> - guardar una referencia en una estructura;
> - saber cuándo usar simplemente un `String` en su lugar.

Este capítulo continúa [Propiedad y préstamo](possession.md). Vuelve a leerlo si las
palabras «tomar prestado» y «referencia» todavía no te resultan familiares.

## El problema: la referencia colgante

Una [referencia](../debuter/glossaire.md#prestamo) `&x` señala un valor que pertenece a
otra persona. ¿Qué pasa si ese valor desaparece mientras la referencia todavía existe?

Imagina un ticket de guardarropa. El ticket (la referencia) dice dónde está tu abrigo
(el valor). Si el guardarropa cierra y tira todos los abrigos, tu ticket señala un
gancho vacío o, peor aún, el abrigo de otra persona. Es una **referencia colgante**
(en inglés: *dangling reference*).

```rust norun
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    } // x desaparece aquí
    println!("{r}"); // r señalaría una casilla liberada
}
```

```text
error[E0597]: `x` does not live long enough
 --> src/main.rs:5:13
  |
4 |         let x = 5;
  |             - binding `x` declared here
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     } // x desaparece aquí
  |     - `x` dropped here while still borrowed
7 |     println!("{r}"); // r señalaría una casilla liberada
  |                - borrow later used here
```

En C, este programa compilaría y mostraría cualquier cosa. Rust lo rechaza **antes**
de ejecutarlo. El mensaje se lee así: «`x` no vive lo suficiente; se libera en la
línea 6 cuando todavía está prestado».

La corrección es sencilla: el valor debe vivir al menos tanto como la referencia.

```rust
let x = 5;
let r = &x;
println!("{r}");
```

```sortie
5
```

## El comprobador de préstamos

El **comprobador de préstamos** (*borrow checker*) es la parte del
[compilador](../debuter/glossaire.md#compilador) que hace esta comprobación. Piensa en
un bibliotecario muy estricto: anota la fecha de devolución de cada libro prestado, y
rechaza cualquier préstamo que durara más que el propio libro.

Para cada referencia, calcula dos zonas del código:

- la zona donde existe el **valor** (desde su creación hasta el final de su bloque);
- la zona donde se usa la **referencia** (desde su creación hasta su último uso).

La segunda debe caber por completo dentro de la primera. Esta zona se llama el **tiempo
de vida** de la referencia (en inglés: *lifetime*).

```text
let x = 5;           ──┐ x existe
let r = &x;          ──┼─┐ r se usa
println!("{r}");     ──┼─┘
                       │
}                    ──┘ x desaparece: todo va bien
```

> **Recuerda:** un tiempo de vida no es una duración en segundos. Es una porción del
> código fuente, comprobada al compilar. No cuesta nada durante la ejecución.

## Anotar una función con 'a

La mayoría de las veces, el compilador sigue los tiempos de vida él solo. Pero una
función que **devuelve una referencia** plantea una pregunta: ¿de qué parámetro viene?

```rust
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("albaricoque");
    let b = String::from("kiwi");
    println!("{}", mas_largo(&a, &b));
}
```

```sortie
albaricoque
```

Desglosemos la firma:

- `<'a>` declara un tiempo de vida llamado `'a` (un apóstrofo y luego un nombre corto);
- `x: &'a str` e `y: &'a str`: los dos textos viven al menos durante `'a`;
- `-> &'a str`: el resultado es válido durante `'a`, no más.

En pocas palabras: «el resultado vive tanto como **el más corto** de los dos
parámetros». La anotación no cambia el tiempo de vida de los valores. **Describe** un
vínculo, para que el compilador pueda comprobar cada llamada.

### El vínculo se comprueba en la llamada

Gracias a este vínculo, el compilador sabe que `resultado` depende de `a` **y** de `b`.
Aquí, todo se usa dentro del bloque de `b`: está permitido.

```rust
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("albaricoque");
    {
        let b = String::from("kiwi");
        let resultado = mas_largo(&a, &b);
        println!("{resultado}");
    }
}
```

```sortie
albaricoque
```

Usar `resultado` después del final del bloque de `b` se rechazaría, aunque el
resultado sea en realidad `a`. El compilador solo mira la firma (ver
[Errores frecuentes](#errores-frecuentes)).

### Un tiempo de vida para un solo parámetro

Si el resultado solo depende de un parámetro, vincula solo ese:

```rust
fn primera_palabra<'a>(texto: &'a str, _separador: &str) -> &'a str {
    texto.split(' ').next().unwrap_or("")
}

fn main() {
    println!("{}", primera_palabra("hola a todos", " "));
}
```

```sortie
hola
```

## ¿Cuándo no hace falta escribir nada?

Escribir `'a` por todas partes sería pesado. Por eso el compilador aplica tres reglas,
llamadas **reglas de elisión** (elidir = omitir). En versión sencilla:

1. Cada parámetro referencia recibe su propio tiempo de vida.
2. Si hay **un solo** parámetro referencia, el resultado toma su tiempo de vida.
3. En un método con `&self`, el resultado toma el tiempo de vida de `self`.

Si estas reglas bastan, no escribes nada. Si no, el compilador te pide una anotación.

```rust
// Regla 2: un solo parámetro referencia, nada que escribir.
fn sin_espacios(s: &str) -> &str {
    s.trim()
}

fn main() {
    println!("[{}]", sin_espacios("  gato  "));
}
```

```sortie
[gato]
```

El compilador lee esta función como `fn sin_espacios<'a>(s: &'a str) -> &'a str`. Con
dos parámetros referencia, como `mas_largo`, ninguna regla permite elegir: hay que
escribir `'a`.

## Una referencia en una estructura

Una estructura puede contener una referencia, por ejemplo un `&str` que toma prestado un
texto existente. Entonces la estructura necesita un tiempo de vida:

```rust
struct Extracto<'a> {
    texto: &'a str,
    linea: usize,
}

impl<'a> Extracto<'a> {
    fn mostrar(&self) {
        println!("línea {}: {}", self.linea, self.texto);
    }
}

fn main() {
    let libro = String::from("Érase una vez.\nUn gatito.");
    let segunda = libro.lines().nth(1).unwrap();
    let e = Extracto { texto: segunda, linea: 2 };
    e.mostrar();
}
```

```sortie
línea 2: Un gatito.
```

Lee `Extracto<'a>` así: «un `Extracto` no puede vivir más que el texto que toma
prestado». Aquí, `e` no vive más que `libro`: todo va bien.

> **Nota:** `impl<'a> Extracto<'a>` repite el tiempo de vida, como se repite un
> parámetro [genérico](generiques.md).

## El tiempo de vida 'static

`'static` es un tiempo de vida especial: «válido durante **toda** la ejecución del
programa». Los textos escritos entre comillas en el código son `&'static str`: se
guardan dentro del propio programa.

```rust
fn estacion(mes: u32) -> &'static str {
    match mes {
        12 | 1 | 2 => "invierno",
        3..=5 => "primavera",
        6..=8 => "verano",
        _ => "otoño",
    }
}

fn main() {
    println!("{}", estacion(7));
    println!("{}", estacion(1));
}
```

```sortie
verano
invierno
```

Aquí, `'static` es imprescindible: la función no tiene ningún parámetro referencia, así
que no se aplica ninguna regla de elisión.

> **Atención:** si el compilador te propone `'static` para hacer callar un error,
> desconfía. Casi nunca es la buena solución para un valor calculado durante la
> ejecución.

## O si no: usa String

Los tiempos de vida sirven para **tomar prestado** sin copiar. Pero no estás obligado a
tomar prestado. Una estructura que **posee** su texto, con un `String`, no necesita
ninguna anotación:

```rust
struct Libro {
    titulo: String,
    autor: String,
}

fn crear_libro(titulo: &str) -> Libro {
    Libro { titulo: titulo.to_string(), autor: String::from("desconocido") }
}

fn main() {
    let l = crear_libro("Dune");
    println!("{} ({})", l.titulo, l.autor);
}
```

```sortie
Dune (desconocido)
```

| Situación | Elección recomendada |
|-----------|----------------------|
| una estructura que guarda datos mucho tiempo | `String` (posee) |
| una función que crea un texto nuevo | devolver un `String` |
| un parámetro que solo se lee | `&str` (préstamo) |
| una pequeña vista temporal de un texto existente | `&'a str` en una estructura |

> **Consejo:** cuando empiezas, pon `String` en tus estructuras. Pasa a las referencias
> cuando te sientas cómodo, o cuando copiar cueste de verdad demasiado.

## Errores frecuentes

### Devolver una referencia a una variable local

```rust norun
fn saludar(nombre: &str) -> &str {
    let frase = format!("Hola {nombre}");
    &frase
}

fn main() {
    println!("{}", saludar("Zoé"));
}
```

```text
error[E0515]: cannot return reference to local variable `frase`
 --> src/main.rs:3:5
  |
3 |     &frase
  |     ^^^^^^ returns a reference to data owned by the current function
```

`frase` desaparece al final de la función: su referencia quedaría colgante. Devuelve el
propio `String`: `fn saludar(nombre: &str) -> String`, y luego `frase` sin `&`.

### Una referencia en una estructura, sin tiempo de vida

```rust norun
struct Libro {
    titulo: &str,
}

fn main() {
    let l = Libro { titulo: "Dune" };
    println!("{}", l.titulo);
}
```

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:13
  |
2 |     titulo: &str,
  |             ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct Libro<'a> {
2 ~     titulo: &'a str,
  |
```

Sigue la ayuda (`struct Libro<'a> { titulo: &'a str }`) o, más sencillo, usa
`titulo: String`.

### Usar el resultado después del final de uno de los parámetros

```rust norun
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("albaricoque");
    let resultado;
    {
        let b = String::from("kiwi");
        resultado = mas_largo(&a, &b);
    }
    println!("{resultado}");
}
```

```text
error[E0597]: `b` does not live long enough
  --> src/main.rs:10:35
   |
 9 |         let b = String::from("kiwi");
   |             - binding `b` declared here
10 |         resultado = mas_largo(&a, &b);
   |                                   ^^ borrowed value does not live long enough
11 |     }
   |     - `b` dropped here while still borrowed
12 |     println!("{resultado}");
   |                --------- borrow later used here
```

La firma dice que el resultado puede venir de `b`. Por lo tanto, no debe sobrevivir a
`b`. Usa `resultado` dentro del bloque, o declara `b` fuera del bloque.

## Ponte a prueba

**1.** ¿Qué es una referencia colgante?

> **Respuesta**
>
> Una referencia a un valor que ya se ha liberado. Rust la rechaza al compilar.

**2.** ¿Por qué `fn mayusculas(s: &str) -> &str` no necesita `'a`?

> **Respuesta**
>
> Solo tiene un parámetro referencia: la regla de elisión da su tiempo de vida al
> resultado.

**3.** ¿Qué tiempo de vida hay que escribir para una función sin parámetros que
devuelve `"hola"`?

> **Respuesta**
>
> `'static`, porque el texto está escrito en el programa:
>
> ```rust
> fn bienvenida() -> &'static str {
>     "hola"
> }
>
> fn main() {
>     println!("{}", bienvenida());
> }
> ```
>
> ```sortie
> hola
> ```

## En resumen

- Una referencia nunca debe vivir más que el valor que toma prestado.
- El comprobador de préstamos lo controla al compilar, sin coste en la ejecución.
- `'a` describe un vínculo entre los parámetros y el resultado; no cambia nada en los
  valores.
- Con un solo parámetro referencia, o `&self`, no se escribe nada: es la elisión.
- `'static` vale para todo el programa, como los textos entre comillas.
- En caso de duda, un `String` en la estructura evita cualquier anotación.

Siguiente: [estructuras, enumeraciones y traits](types.md)
