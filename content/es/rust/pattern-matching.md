# La coincidencia de patrones

La coincidencia de patrones compara un valor con formas posibles y extrae sus piezas: es una de las herramientas más potentes de Rust, con `match`, `if let`, `let else` y `while let`.

> **Objetivos:**
>
> - escribir un `match` completo con valores, rangos y alternativas;
> - extraer las piezas de una tupla, una estructura, una enumeración o un slice;
> - añadir una condición a un patrón con una guarda `if`;
> - elegir entre `match`, `if let`, `let else` y `while let`;
> - comprobar una forma rápidamente con `matches!`.

Un **patrón** (en inglés: *pattern*) es una forma. Piensa en el juego de encajar formas
para niños: el cubo pasa por el agujero cuadrado, el cilindro por el agujero redondo.
Rust prueba cada agujero en orden, y el valor pasa por el primero que le corresponde.

## match: todos los casos, en orden

Ya has visto [match sobre una enumeración](types.md#match-sobre-una-enumeracion).
`match` también funciona con números, caracteres o textos. Cada línea es un **brazo**:
un patrón, una flecha `=>` y luego el código que se ejecuta.

```rust
let dado = 3;
let texto = match dado {
    1 => "uno",
    2 => "dos",
    3 => "tres",
    _ => "muchos",
};
println!("{texto}");
```

```sortie
tres
```

Dos reglas que recordar:

- los brazos se prueban **de arriba abajo**; gana el primero que coincide;
- el `match` debe ser **exhaustivo**: todos los valores posibles deben estar cubiertos.
  El patrón `_` («cualquier otra cosa») sirve de red de seguridad.

> **Recuerda:** la exhaustividad es una seguridad. Si añades una variante a una
> enumeración, el compilador te muestra cada `match` que hay que completar.

`match` es una [expresión](../debuter/glossaire.md#expresion): produce un valor, aquí
guardado en `texto`. Por eso todos los brazos deben producir el mismo tipo.

## Varios valores: | y los rangos

La barra `|` significa «o». Un rango `a..=b` cubre todos los valores de `a` a `b`
incluido (mira los [rangos](../debuter/glossaire.md#rango)).

```rust
fn valoracion(nota: u32) -> &'static str {
    match nota {
        0..=7 => "insuficiente",
        8 | 9 => "casi",
        10..=15 => "bien",
        16..=20 => "muy bien",
        _ => "nota no válida",
    }
}

fn main() {
    for n in [5, 9, 12, 18, 25] {
        println!("{n}: {}", valoracion(n));
    }
}
```

```sortie
5: insuficiente
9: casi
12: bien
18: muy bien
25: nota no válida
```

Los rangos también funcionan con los caracteres:

```rust
for c in ['a', 'Z', '7', '?'] {
    let tipo = match c {
        'a'..='z' => "minúscula",
        'A'..='Z' => "mayúscula",
        '0'..='9' => "cifra",
        _ => "otro",
    };
    println!("{c}: {tipo}");
}
```

```sortie
a: minúscula
Z: mayúscula
7: cifra
?: otro
```

## Dar un nombre con @

Un patrón puede **comprobar** un valor y **nombrarlo** a la vez, con `nombre @ patrón`:

```rust
let edad = 15;
match edad {
    0 => println!("recién nacido"),
    n @ 1..=17 => println!("menor de {n} años"),
    n => println!("mayor de edad, {n} años"),
}
```

```sortie
menor de 15 años
```

El último brazo, `n`, es un nombre solo: acepta cualquier valor y lo guarda en `n`.
Es como `_`, pero conservando el valor.

## Descomponer una tupla

Un patrón puede tener la **forma** del valor y guardar cada pieza en una variable. Es
la **desestructuración** (en inglés: *destructuring*).

```rust
let punto = (3, -2);
match punto {
    (0, 0) => println!("origen"),
    (x, 0) => println!("en el eje horizontal, x = {x}"),
    (0, y) => println!("en el eje vertical, y = {y}"),
    (x, y) => println!("en otro sitio: ({x}, {y})"),
}
```

```sortie
en otro sitio: (3, -2)
```

La desestructuración también funciona fuera de un `match`, con `let` o en un `for`:

```rust
let (nombre, edad) = ("Zoé", 12);
println!("{nombre} tiene {edad} años");
for (i, fruta) in ["manzana", "kiwi"].iter().enumerate() {
    println!("{i}: {fruta}");
}
```

```sortie
Zoé tiene 12 años
0: manzana
1: kiwi
```

## Descomponer una estructura

Para una [estructura](types.md#las-estructuras), se escribe el nombre de los campos.
`..` ignora los campos restantes.

```rust
struct Jugador {
    nombre: String,
    puntos: u32,
    nivel: u32,
}

fn main() {
    let j = Jugador { nombre: String::from("Leo"), puntos: 120, nivel: 3 };

    let Jugador { nombre, puntos, .. } = &j;
    println!("{nombre}: {puntos} puntos");

    match j {
        Jugador { nivel: 1, .. } => println!("principiante"),
        Jugador { nivel: n, .. } => println!("nivel {n}"),
    }
}
```

```sortie
Leo: 120 puntos
nivel 3
```

`nivel: 1` comprueba el valor del campo. `nivel: n` guarda el campo en `n`.

## Descomponer una enumeración

Las variantes de una enumeración pueden llevar datos. El patrón los extrae:

```rust
enum Forma {
    Circulo(f64),
    Rectangulo { ancho: f64, alto: f64 },
    Punto,
}

fn area(f: &Forma) -> f64 {
    match f {
        Forma::Circulo(r) => 3.14 * r * r,
        Forma::Rectangulo { ancho, alto } => ancho * alto,
        Forma::Punto => 0.0,
    }
}

fn main() {
    let formas = [
        Forma::Circulo(1.0),
        Forma::Rectangulo { ancho: 2.0, alto: 3.0 },
        Forma::Punto,
    ];
    for f in &formas {
        println!("{}", area(f));
    }
}
```

```sortie
3.14
6
0
```

## Descomponer un slice

Un patrón entre corchetes describe el **contenido** de una lista. `..` sustituye a
«todos los elementos del medio», sea cual sea su número.

```rust
fn describir(v: &[i32]) -> String {
    match v {
        [] => String::from("vacío"),
        [solo] => format!("uno solo: {solo}"),
        [a, b] => format!("dos: {a} y {b}"),
        [primero, .., ultimo] => format!("de {primero} a {ultimo}"),
    }
}

fn main() {
    println!("{}", describir(&[]));
    println!("{}", describir(&[7]));
    println!("{}", describir(&[1, 2]));
    println!("{}", describir(&[1, 2, 3, 4, 5]));
}
```

```sortie
vacío
uno solo: 7
dos: 1 y 2
de 1 a 5
```

Con `resto @ ..`, puedes incluso nombrar el medio: `[primero, resto @ ..]` guarda todo
excepto el primero en `resto`.

## Añadir una condición: la guarda

Una **guarda** es un `if` colocado después del patrón. El brazo solo se elige si la
condición es verdadera:

```rust
let temperaturas = [-5, 0, 18, 35];
for t in temperaturas {
    let opinion = match t {
        t if t < 0 => "helada",
        0 => "cero exacto",
        t if t > 30 => "ola de calor",
        _ => "normal",
    };
    println!("{t}: {opinion}");
}
```

```sortie
-5: helada
0: cero exacto
18: normal
35: ola de calor
```

> **Atención:** el compilador no mira dentro de las guardas para comprobar la
> exhaustividad. Por eso a menudo hace falta un último brazo sin guarda (`_`).

## Patrones anidados

Los patrones se encajan unos dentro de otros como muñecas rusas. Aquí, una `Option`
contiene una tupla:

```rust
let pedidos = [Some(("café", 2)), None, Some(("té", 0)), Some(("zumo", 1))];
for p in pedidos {
    match p {
        None => println!("sin pedido"),
        Some((bebida, 0)) => println!("{bebida}: cancelado"),
        Some((bebida, 1)) => println!("un {bebida}"),
        Some((bebida, n)) => println!("{n} × {bebida}"),
    }
}
```

```sortie
2 × café
sin pedido
té: cancelado
un zumo
```

## if let: un solo caso interesante

Cuando solo te interesa un patrón, `match` es un poco largo. `if let` comprueba un
patrón y ejecuta el bloque si coincide. Se puede añadir un `else`:

```rust
let entrada = "42";
if let Ok(n) = entrada.parse::<i32>() {
    println!("el doble es {}", n * 2);
} else {
    println!("no es un número");
}

let config: Option<&str> = None;
if let Some(tema) = config {
    println!("tema {tema}");
}
println!("fin");
```

```sortie
el doble es 84
fin
```

## let else: salir pronto

`let … else` hace lo contrario: **descompone** el valor y, si el patrón no coincide,
ejecuta el bloque `else`, que debe **salir** de la función (`return`, `break`,
`continue` o `panic!`). Después, la variable está disponible sin anidar.

```rust
fn leer_edad(texto: &str) -> String {
    let Ok(edad) = texto.parse::<u32>() else {
        return String::from("edad no válida");
    };
    format!("tienes {edad} años")
}

fn main() {
    println!("{}", leer_edad("30"));
    println!("{}", leer_edad("treinta"));
}
```

```sortie
tienes 30 años
edad no válida
```

> **Consejo:** `let else` mantiene el caso normal pegado a la izquierda del código, y
> los casos de error resueltos arriba. El código queda plano y legible.

## while let: repetir mientras coincida

`while let` repite un bucle mientras el patrón coincida. Es perfecto para vaciar una
pila:

```rust
let mut pila = vec![1, 2, 3];
while let Some(cima) = pila.pop() {
    println!("quito {cima}");
}
println!("pila vacía");
```

```sortie
quito 3
quito 2
quito 1
pila vacía
```

`pop` devuelve `Some(…)` mientras queden elementos, y luego `None`: el bucle se
detiene.

## matches!: una pregunta de sí o no

La [macro](../debuter/glossaire.md#macro) `matches!` responde `true` o `false`: «¿tiene
este valor esta forma?». Acepta `|`, los rangos y las guardas.

```rust
let c = 'e';
println!("{}", matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y'));

let notas = [12, 4, 18, 9];
let buenas = notas.iter().filter(|n| matches!(n, 10..=20)).count();
println!("{buenas} notas buenas");

let resultado: Option<i32> = Some(7);
println!("{}", matches!(resultado, Some(x) if x > 5));
```

```sortie
true
2 notas buenas
true
```

## Errores frecuentes

### Un match que olvida valores

```rust norun
fn main() {
    let nota = 15;
    match nota {
        0..=9 => println!("insuficiente"),
        10..=20 => println!("bien"),
    }
}
```

```text
error[E0004]: non-exhaustive patterns: `i32::MIN..=-1_i32` and `21_i32..=i32::MAX` not covered
 --> src/main.rs:3:11
  |
3 |     match nota {
  |           ^^^^ patterns `i32::MIN..=-1_i32` and `21_i32..=i32::MAX` not covered
  |
  = note: the matched value is of type `i32`
```

Tú sabes que una nota está entre 0 y 20, pero el compilador no: un `i32` puede valer
-3 o 1000. Añade un último brazo `_ => println!("no válida"),`.

### Creer que un nombre compara con una variable

```rust norun
fn main() {
    let objetivo = 5;
    let intento = 3;
    match intento {
        objetivo => println!("¡ganado!"),
        _ => println!("perdido"),
    }
}
```

```text
warning: unreachable pattern
 --> src/main.rs:6:9
  |
5 |         objetivo => println!("¡ganado!"),
  |         -------- matches any value
6 |         _ => println!("perdido"),
  |         ^ no value can reach this
  |
note: there is a binding of the same name; if you meant to pattern match against the value of that binding, that is a feature of constants that is not available for `let` bindings
```

Solo es una advertencia, pero el programa muestra «¡ganado!». En un patrón, un nombre
solo **crea** una variable nueva que lo acepta todo. Para comparar, usa una guarda:
`n if n == objetivo => …`, o simplemente `if intento == objetivo`.

### Un let else que no sale

```rust norun
fn main() {
    let texto = "abc";
    let Ok(n) = texto.parse::<i32>() else {
        println!("no es un número");
    };
    println!("{n}");
}
```

```text
error[E0308]: `else` clause of `let...else` does not diverge
 --> src/main.rs:3:43
  |
3 |       let Ok(n) = texto.parse::<i32>() else {
  |  ___________________________________________^
4 | |         println!("no es un número");
5 | |     };
  | |_____^ expected `!`, found `()`
  |
  = note:   expected type `!`
          found unit type `()`
  = help: try adding a diverging expression, such as `return` or `panic!(..)`
  = help: ...or use `match` instead of `let...else`
```

Después del `let else`, `n` debe existir. Por eso el bloque `else` debe salir: añade
`return;` después del `println!`.

## Ponte a prueba

**1.** ¿Qué hace el patrón `_`? ¿Y el patrón `x` solo?

> **Respuesta**
>
> Los dos aceptan cualquier valor. `_` lo ignora; `x` lo guarda en una variable nueva
> `x`.

**2.** ¿Qué patrón acepta un slice de al menos dos elementos, nombrando el primero y
el último?

> **Respuesta**
>
> `[primero, .., ultimo]`. Acepta dos elementos o más.

**3.** Escribe con `matches!` una prueba que diga si un carácter es una cifra de 0 a 9.

> **Respuesta**
>
> ```rust
> let c = '4';
> println!("{}", matches!(c, '0'..='9'));
> println!("{}", matches!('x', '0'..='9'));
> ```
>
> ```sortie
> true
> false
> ```

## En resumen

- `match` prueba los brazos en orden y debe cubrir todos los casos; `_` sirve de red.
- `|` combina patrones, `a..=b` cubre un rango, `nombre @ patrón` conserva el valor.
- Un patrón puede descomponer una tupla, una estructura, una enumeración o un slice.
- Una guarda `if` añade una condición a un brazo.
- `if let` para un solo caso, `let else` para salir pronto, `while let` para repetir.
- `matches!` responde `true` o `false` a «¿tiene este valor esta forma?».

Siguiente: [genéricos y traits](generiques.md)
