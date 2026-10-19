# HashMap, HashSet y BTreeMap

Este capítulo presenta las colecciones que guardan valores por clave o sin duplicados: `HashMap`, `BTreeMap`, `HashSet`, y también la cola `VecDeque`.

> **Objetivos:**
>
> - añadir, leer y quitar pares clave-valor en un `HashMap`;
> - contar elementos con `entry().or_insert()`;
> - mostrar una tabla en un orden estable, con una ordenación o un `BTreeMap`;
> - comparar dos conjuntos con `HashSet`;
> - elegir la colección adecuada para cada problema.

Todas estas colecciones están en `std::collections`. A diferencia de `Vec`, tienes que
importarlas con `use`.

## ¿Qué es un HashMap?

Un `HashMap` asocia **claves** a **valores**, como una agenda asocia un nombre a un
número de teléfono. También se le llama
[diccionario](../debuter/glossaire.md#diccionario). No buscas «la tercera línea»:
buscas directamente «Zoé».

```rust
use std::collections::HashMap;

fn main() {
    let mut edades = HashMap::new();
    edades.insert("Zoé", 12);
    edades.insert("Léo", 9);
    println!("{}", edades.len());
    println!("{}", edades["Zoé"]);
}
```

```sortie
2
12
```

Línea por línea:

- `use std::collections::HashMap;` hace que `HashMap` esté disponible en el programa;
- `HashMap::new()` crea una tabla vacía. El `mut` permite luego llenarla;
- los dos `insert` añaden dos pares: «Zoé» con 12, y luego «Léo» con 9;
- `edades.len()` cuenta los pares: 2. `edades["Zoé"]` busca el valor guardado bajo la
  clave «Zoé»: 12.

Todas las claves tienen el mismo tipo (aquí `&str`), y todos los valores también (aquí
`i32`). Una clave aparece una sola vez: insertar una clave existente **reemplaza** su
valor.

```rust
use std::collections::HashMap;

fn main() {
    let mut edades = HashMap::new();
    edades.insert("Zoé", 12);
    let anterior = edades.insert("Zoé", 13);
    println!("{:?} {}", anterior, edades["Zoé"]);
}
```

```sortie
Some(12) 13
```

`insert` devuelve el valor anterior en un [Option](types.md): `None` si la clave era
nueva.

**Te toca:** escribe un programa que guarde dos países y su capital en un `HashMap`:
«Francia» con «París», «Italia» con «Roma». Muestra el número de pares, y luego la
capital de Italia.

> **Solución**
>
> ```rust
> use std::collections::HashMap;
>
> fn main() {
>     let mut capitales = HashMap::new();
>     capitales.insert("Francia", "París");
>     capitales.insert("Italia", "Roma");
>     println!("{}", capitales.len());
>     println!("{}", capitales["Italia"]);
> }
> ```
>
> ```sortie
> 2
> Roma
> ```

## Leer, comprobar y quitar

Una tabla sirve para encontrar una información. Pero puedes pedir una clave que no está,
como buscar en la agenda un nombre que no aparece. Hay que prever ese caso.

`edades["Zoé"]` detiene el programa si la clave no existe. Mejor usa `get`, que
devuelve un `Option`:

```rust
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("manzanas", 3);
    stock.insert("peras", 0);

    println!("{:?}", stock.get("manzanas"));
    println!("{:?}", stock.get("kiwis"));
    println!("{}", stock.contains_key("peras"));

    let quitado = stock.remove("peras");
    println!("{:?} {}", quitado, stock.len());

    match stock.get("kiwis") {
        Some(n) => println!("{n} kiwis"),
        None => println!("no hay kiwis"),
    }
    let n = stock.get("kiwis").copied().unwrap_or(0);
    println!("{n}");
}
```

```sortie
Some(3)
None
true
Some(0) 1
no hay kiwis
0
```

Línea por línea:

- `stock.get("manzanas")` encuentra la clave y devuelve `Some(3)`. La clave `"kiwis"`
  no existe: `get` devuelve `None`;
- `contains_key("peras")` solo responde sí o no: `true`;
- `remove("peras")` quita el par y devuelve su valor, `Some(0)`. Queda 1 par;
- el `match` trata los dos casos. Aquí es `None`: muestra «no hay kiwis»;
- la última línea da 0 cuando falta la clave, sin escribir un `match`.

> **Nota:** `get` devuelve una **referencia** (`Some(&3)`, que se muestra `Some(3)`).
> `.copied()` hace una copia, y `unwrap_or(0)` da un valor por defecto.

Para modificar un valor en su sitio, `get_mut` devuelve una referencia `&mut`:

```rust
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("manzanas", 3);
    if let Some(n) = stock.get_mut("manzanas") {
        *n += 10;
    }
    println!("{}", stock["manzanas"]);
}
```

```sortie
13
```

## Contar con entry

Contar es muy habitual: los votos de una elección, las palabras de un texto, los
artículos de una cesta. En papel, escribirías cada nombre con palitos al lado. Un
`HashMap` hace lo mismo: el nombre es la clave, el número de palitos es el valor.

`entry(clave)` da acceso a la casilla de una clave, exista o no. `or_insert(0)` la crea
con `0` si falta, y luego devuelve una referencia al valor. Es la forma ideal de
**contar**:

```rust
use std::collections::HashMap;

fn main() {
    let texto = "el gato y el perro y el conejo";
    let mut cuenta = HashMap::new();
    for palabra in texto.split_whitespace() {
        *cuenta.entry(palabra).or_insert(0) += 1;
    }
    println!("el: {}", cuenta["el"]);
    println!("y: {}", cuenta["y"]);
    println!("gato: {}", cuenta["gato"]);
}
```

```sortie
el: 3
y: 2
gato: 1
```

Línea por línea:

- `cuenta` empieza vacío;
- `split_whitespace()` corta el texto en palabras, y el bucle las toma una a una;
- con el primer «el», la casilla no existe: `or_insert(0)` la crea con 0, y luego
  `+= 1` la pasa a 1;
- con el segundo «el», la casilla ya existe: pasa a 2, y luego a 3 con el tercero.

Lee la línea importante así: «toma la casilla de `palabra`, pon 0 si está vacía, y
luego suma 1». La estrella `*` modifica el valor al que apunta la referencia.

La misma herramienta sirve para **agrupar**: aquí, una lista de nombres por inicial.

```rust
use std::collections::HashMap;

fn main() {
    let nombres = ["Ali", "Bea", "Anna", "Bob", "Chloé"];
    let mut grupos: HashMap<char, Vec<&str>> = HashMap::new();
    for n in nombres {
        let inicial = n.chars().next().unwrap();
        grupos.entry(inicial).or_insert_with(Vec::new).push(n);
    }
    println!("{:?}", grupos[&'A']);
    println!("{:?}", grupos[&'B']);
}
```

```sortie
["Ali", "Anna"]
["Bea", "Bob"]
```

**Te toca:** escribe un programa que cuente los votos de `"sí no sí sí no sí"`, y luego
muestre el número de «sí» y el número de «no».

> **Solución**
>
> ```rust
> use std::collections::HashMap;
>
> fn main() {
>     let votos = "sí no sí sí no sí";
>     let mut cuenta = HashMap::new();
>     for voto in votos.split_whitespace() {
>         *cuenta.entry(voto).or_insert(0) += 1;
>     }
>     println!("sí: {}", cuenta["sí"]);
>     println!("no: {}", cuenta["no"]);
> }
> ```
>
> ```sortie
> sí: 4
> no: 2
> ```

## Recorrer un HashMap

A veces quieres todo el contenido de la tabla, no un solo valor: para mostrarlo todo, o
para calcular un total. Es como releer un boletín de notas de arriba abajo.

Un bucle `for` sobre `&tabla` da pares `(clave, valor)`:

```rust
use std::collections::HashMap;

fn main() {
    let mut notas = HashMap::new();
    notas.insert("matemáticas", 15);
    notas.insert("francés", 12);
    notas.insert("inglés", 17);

    let mut total = 0;
    for (_asignatura, nota) in &notas {
        total += nota;
    }
    println!("total: {total}");
}
```

```sortie
total: 44
```

Línea por línea:

- la tabla contiene tres pares: una asignatura y su nota;
- `total` empieza en 0;
- en cada vuelta, el bucle da un par: la asignatura va a `_asignatura`, la nota a
  `nota`. El `_` al principio del nombre quiere decir «no uso esta variable»;
- cada nota se suma al total: 15 + 12 + 17 = 44, sea cual sea el orden de las vueltas.

> **Atención:** un `HashMap` no guarda **ningún orden**. Dos ejecuciones del mismo
> programa pueden mostrar los pares en un orden distinto. Es a propósito: protege
> contra ciertos ataques.

Para mostrarlo de forma estable, reúne los pares en un `Vec` y luego ordénalo:

```rust
use std::collections::HashMap;

fn main() {
    let mut notas = HashMap::new();
    notas.insert("matemáticas", 15);
    notas.insert("francés", 12);
    notas.insert("inglés", 17);

    let mut pares: Vec<_> = notas.iter().collect();
    pares.sort();
    for (asignatura, nota) in pares {
        println!("{asignatura}: {nota}");
    }
}
```

```sortie
francés: 12
inglés: 17
matemáticas: 15
```

`keys()` y `values()` recorren solo las claves o solo los valores.

## BTreeMap: claves siempre ordenadas

Un diccionario de papel guarda sus palabras en orden alfabético: eso es lo que lo hace
fácil de leer. Un `BTreeMap` hace lo mismo con sus claves.

Un `BTreeMap` se usa exactamente como un `HashMap`, pero mantiene sus claves
**ordenadas**. Ya no hace falta ordenar antes de mostrar:

```rust
use std::collections::BTreeMap;

fn main() {
    let mut puntos = BTreeMap::new();
    puntos.insert("Zoé", 30);
    puntos.insert("Ali", 25);
    puntos.insert("Léo", 40);
    for (nombre, p) in &puntos {
        println!("{nombre}: {p}");
    }
    println!("{:?}", puntos.first_key_value());
}
```

```sortie
Ali: 25
Léo: 40
Zoé: 30
Some(("Ali", 25))
```

Línea por línea:

- los tres nombres se insertan desordenados: Zoé, Ali y luego Léo;
- la tabla los ordena enseguida, por orden alfabético;
- así que el bucle `for` los muestra en orden: Ali, Léo, Zoé;
- `first_key_value()` da el par de la clave más pequeña: `("Ali", 25)`.

> **Consejo:** un `HashMap` es un poco más rápido; un `BTreeMap` da siempre el mismo
> resultado. Para empezar, y en cuanto muestres una tabla, el `BTreeMap` suele ser la
> opción más sencilla.

**Te toca:** escribe un programa que guarde tres frutas y su precio en un `BTreeMap`:
«pera» a 3, «plátano» a 2, «albaricoque» a 4. Muéstralas en orden alfabético, una por
línea, con la forma «albaricoque: 4».

> **Solución**
>
> ```rust
> use std::collections::BTreeMap;
>
> fn main() {
>     let mut precios = BTreeMap::new();
>     precios.insert("pera", 3);
>     precios.insert("plátano", 2);
>     precios.insert("albaricoque", 4);
>     for (fruta, p) in &precios {
>         println!("{fruta}: {p}");
>     }
> }
> ```
>
> ```sortie
> albaricoque: 4
> pera: 3
> plátano: 2
> ```

## HashSet: un conjunto sin duplicados

Piensa en una lista de invitados: escribir dos veces el mismo nombre no sirve de nada.
Un conjunto guarda cada valor una sola vez, y responde rápido a la pregunta «¿está
dentro?».

Un `HashSet` es una bolsa donde cada valor aparece **una sola vez**. No hay valor
asociado: solo elementos. `insert` devuelve `false` si el elemento ya estaba.

```rust
use std::collections::HashSet;

fn main() {
    let mut vistos = HashSet::new();
    println!("{}", vistos.insert("rojo"));
    println!("{}", vistos.insert("azul"));
    println!("{}", vistos.insert("rojo"));
    println!("{}", vistos.len());
    println!("{}", vistos.contains("azul"));
}
```

```sortie
true
true
false
2
true
```

Línea por línea:

- `vistos` empieza vacío;
- «rojo» es nuevo: `insert` lo añade y devuelve `true`. Lo mismo para «azul»;
- el segundo «rojo» ya está: no se añade nada, e `insert` devuelve `false`;
- así que el conjunto tiene 2 elementos, y «azul» es uno de ellos.

### Quitar los duplicados de una lista

```rust
use std::collections::HashSet;

fn main() {
    let ciudades = vec!["Lyon", "Paris", "Lyon", "Nice", "Paris"];
    let unicas: HashSet<&str> = ciudades.into_iter().collect();
    let mut lista: Vec<_> = unicas.into_iter().collect();
    lista.sort();
    println!("{:?}", lista);
}
```

```sortie
["Lyon", "Nice", "Paris"]
```

**Te toca:** unos visitantes firman un libro de visitas:
`["Ali", "Bea", "Ali", "Dan", "Bea"]`. Escribe un programa que muestre cuántas personas
**distintas** vinieron, con un `HashSet`.

> **Solución**
>
> ```rust
> use std::collections::HashSet;
>
> fn main() {
>     let visitantes = ["Ali", "Bea", "Ali", "Dan", "Bea"];
>     let mut distintos = HashSet::new();
>     for v in visitantes {
>         distintos.insert(v);
>     }
>     println!("{} personas distintas", distintos.len());
> }
> ```
>
> ```sortie
> 3 personas distintas
> ```

### Unión, intersección y diferencia

Dos conjuntos se comparan como en matemáticas. Imagina dos clubes y sus miembros:

| Operación | Significado | Método |
|-----------|-------------|--------|
| unión | miembros de al menos un club | `a.union(&b)` |
| intersección | miembros de los dos clubes | `a.intersection(&b)` |
| diferencia | miembros de `a` pero no de `b` | `a.difference(&b)` |

```rust
use std::collections::{BTreeSet, HashSet};

fn main() {
    let futbol: HashSet<&str> = ["Ali", "Bea", "Chloé"].into_iter().collect();
    let ajedrez: HashSet<&str> = ["Bea", "Dan"].into_iter().collect();

    // Guardamos cada resultado en un BTreeSet para mostrarlo ordenado.
    let union: BTreeSet<_> = futbol.union(&ajedrez).collect();
    let comunes: BTreeSet<_> = futbol.intersection(&ajedrez).collect();
    let solo_futbol: BTreeSet<_> = futbol.difference(&ajedrez).collect();
    println!("{:?}", union);
    println!("{:?}", comunes);
    println!("{:?}", solo_futbol);
}
```

```sortie
{"Ali", "Bea", "Chloé", "Dan"}
{"Bea"}
{"Ali", "Chloé"}
```

Un `BTreeSet` es el conjunto ordenado: es a `HashSet` lo que `BTreeMap` es a
`HashMap`. Las llaves `{ }` de la salida recuerdan que se trata de un conjunto.

## VecDeque: una cola

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Un `Vec` añade y quita rápido **al final**. Quitar **al principio** es lento: todos
los elementos tienen que desplazarse. Un `VecDeque` (se pronuncia «vec-dek») es rápido
en los dos extremos. Es la cola de la panadería: llegas por detrás y te atienden por
delante.

```rust
use std::collections::VecDeque;

fn main() {
    let mut cola = VecDeque::new();
    cola.push_back("Ali");
    cola.push_back("Bea");
    cola.push_back("Dan");
    cola.push_front("Abuela"); // tiene prioridad

    while let Some(cliente) = cola.pop_front() {
        println!("Atiendo a {cliente}");
    }
    println!("{}", cola.is_empty());
}
```

```sortie
Atiendo a Abuela
Atiendo a Ali
Atiendo a Bea
Atiendo a Dan
true
```

Línea por línea:

- tres clientes llegan por detrás con `push_back`: Ali, Bea y luego Dan;
- la abuela pasa delante con `push_front`: la cola queda Abuela, Ali, Bea, Dan;
- `while let Some(cliente) = cola.pop_front()` atiende a la primera persona de la cola,
  vuelta tras vuelta, mientras quede alguien;
- cuando la cola está vacía, `pop_front` devuelve `None`: el bucle se detiene, e
  `is_empty()` muestra `true`.

## ¿Qué colección elegir?

Cada colección es una forma distinta de guardar cosas, como una estantería, un
archivador o una caja de clasificar. Elegir bien hace tu código más sencillo.

| Necesidad | Colección |
|-----------|-----------|
| una lista ordenada, acceso por posición | `Vec<T>` |
| añadir o quitar en los dos extremos (cola) | `VecDeque<T>` |
| encontrar un valor por su clave, rápido | `HashMap<K, V>` |
| lo mismo, con las claves ordenadas | `BTreeMap<K, V>` |
| valores únicos, comprobar si un elemento está | `HashSet<T>` |
| valores únicos y ordenados | `BTreeSet<T>` |

> **Recuerda:** empieza por `Vec`. Pasa a una tabla cuando busques «por nombre», y a un
> conjunto cuando te molesten los duplicados.

## Errores frecuentes

### Tomar el resultado de get por el valor

```rust norun
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("manzanas", 3);
    let n: i32 = stock.get("manzanas");
    println!("{n}");
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:6:18
  |
6 |     let n: i32 = stock.get("manzanas");
  |            ---   ^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `Option<&{integer}>`
  |            |
  |            expected due to this
  |
  = note: expected type `i32`
             found enum `Option<&{integer}>`
```

`get` devuelve un `Option`: la clave puede faltar. Escribe
`stock.get("manzanas").copied().unwrap_or(0)`, o trata los dos casos con `match`.

### Olvidar mut

```rust norun
use std::collections::HashMap;

fn main() {
    let stock = HashMap::new();
    stock.insert("manzanas", 3);
    println!("{:?}", stock);
}
```

```text
error[E0596]: cannot borrow `stock` as mutable, as it is not declared as mutable
 --> src/main.rs:5:5
  |
5 |     stock.insert("manzanas", 3);
  |     ^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
4 |     let mut stock = HashMap::new();
  |         +++
```

Añadir un par modifica la tabla: declárala con `let mut`.

### Una estructura como clave, sin Hash ni Eq

```rust norun
use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut nombres = HashMap::new();
    nombres.insert(Point { x: 0, y: 0 }, "origen");
    println!("{:?}", nombres);
}
```

```text
error[E0277]: the trait bound `Point: Eq` is not satisfied
  --> src/main.rs:11:13
   |
11 |     nombres.insert(Point { x: 0, y: 0 }, "origen");
   |             ^^^^^^ the trait `Eq` is not implemented for `Point`
   |
help: consider annotating `Point` with `#[derive(Eq)]`
   |
 4 + #[derive(Eq)]
 5 | struct Point {
   |
```

Una clave tiene que poder compararse (`Eq`) y «hashearse» (`Hash`). Añade
`#[derive(Debug, PartialEq, Eq, Hash)]` encima de la estructura. Mira
[Los traits de la biblioteca estándar](traits-standard.md).

## Ponte a prueba

**1.** ¿Por qué dos ejecuciones del mismo programa pueden mostrar un `HashMap` en un
orden distinto? ¿Cómo conseguir un orden estable?

> **Respuesta**
>
> Un `HashMap` no guarda ningún orden. Ordena los pares en un `Vec` antes de
> mostrarlos, o usa un `BTreeMap`.

**2.** ¿Qué línea suma 1 al contador de `palabra`, creándolo a 0 si no existe?

> **Respuesta**
>
> `*cuenta.entry(palabra).or_insert(0) += 1;`

**3.** ¿Qué letras tienen en común «queso» y «beso»? Responde con dos `HashSet` y
ordena el resultado.

> **Respuesta**
>
> ```rust
> use std::collections::HashSet;
>
> fn main() {
>     let a: HashSet<char> = "queso".chars().collect();
>     let b: HashSet<char> = "beso".chars().collect();
>     let mut comunes: Vec<char> = a.intersection(&b).copied().collect();
>     comunes.sort();
>     println!("{:?}", comunes);
> }
> ```
>
> ```sortie
> ['e', 'o', 's']
> ```

## En resumen

- `HashMap` asocia claves a valores: `insert`, `get`, `remove`, `contains_key`.
- `*tabla.entry(clave).or_insert(0) += 1` cuenta en una línea.
- Un `HashMap` no tiene orden: ordena antes de mostrar, o usa un `BTreeMap`.
- `HashSet` guarda valores únicos y ofrece `union`, `intersection`, `difference`.
- `VecDeque` es una cola: `push_back` por detrás, `pop_front` por delante.

Siguiente: [seguir un programa paso a paso](pas-a-pas.md).
