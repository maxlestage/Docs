# Colecciones

Esta página presenta las colecciones más útiles de Rust: los vectores `Vec`, las tablas `HashMap`, los iteradores y los slices.

> **Objetivos:**
>
> - elegir entre un array de tamaño fijo y un `Vec`;
> - añadir, leer y quitar elementos de un `Vec` sin riesgo;
> - asociar claves a valores con `HashMap` y `BTreeMap`;
> - transformar una colección con `map`, `filter` y `collect`;
> - tomar una parte de una lista con un slice.

Una **colección** es un valor que contiene varios otros, como una caja de
almacenaje.

## Los arrays de tamaño fijo

Para guardar los siete días de la semana, crear siete variables sería pesado. Un array
los guarda todos bajo un solo nombre, como una huevera: un número fijo de huecos, un
hueco por elemento.

Un array `[T; N]` contiene siempre el mismo número `N` de elementos, todos del mismo
tipo `T`. Las posiciones (los **índices**) empiezan en 0:

```rust
let dias = ["lun", "mar", "mié"];
println!("{}", dias[0]);
println!("{}", dias.len());
println!("{:?}", dias);
```

```sortie
lun
3
["lun", "mar", "mié"]
```

Línea por línea:

- `let dias = [...]` crea un array de tres textos, guardado en `dias`;
- `dias[0]` lee el hueco número 0, es decir, el **primero**: `"lun"`;
- `dias.len()` da el número de huecos: 3;
- `{:?}` muestra todo el array de una vez, con los corchetes.

Un array también puede llenarse con un mismo valor: `[0; 5]` da cinco ceros.

```rust
let ceros = [0; 5];
println!("{:?}", ceros);
```

```sortie
[0, 0, 0, 0, 0]
```

## Los vectores: Vec

A menudo no sabes de antemano cuántos elementos tendrás. Una lista de la compra crece
cuando piensas en un artículo, y se acorta cuando lo compras. Para eso sirve el `Vec`.

Un `Vec` (un «vector») es una [lista](../debuter/glossaire.md#lista) que puede crecer
o encogerse. Es la colección más habitual. La macro `vec![…]` crea uno; `push` añade
al final, `pop` quita el último elemento.

```rust
let mut notas = vec![12, 15, 9];
notas.push(18);
println!("{:?}", notas);
println!("Primera: {}", notas[0]);
println!("Tamaño: {}", notas.len());
let ultima = notas.pop();
println!("{:?} {:?}", ultima, notas);
```

```sortie
[12, 15, 9, 18]
Primera: 12
Tamaño: 4
Some(18) [12, 15, 9]
```

Línea por línea:

- `let mut notas = vec![12, 15, 9];` crea un `Vec` de tres notas. `mut` permite
  modificarlo después;
- `notas.push(18)` añade 18 al final: ahora hay cuatro notas;
- `notas[0]` lee la primera nota (12) y `notas.len()` cuenta los elementos (4);
- `notas.pop()` quita la última nota y la guarda en `ultima`: la lista vuelve a tener
  tres elementos.

`pop` devuelve `Some(18)`: el valor quitado, dentro de un `Option`, porque la lista
podría haber estado vacía.

### ¿Array o Vec?

| | Array `[T; N]` | `Vec<T>` |
|-|----------------|----------|
| Tamaño | fijado al escribir el código | cambia durante la ejecución |
| Añadir / quitar | imposible | `push`, `pop`, `insert`, `remove` |
| Uso típico | los 7 días, los 12 meses | una lista de la compra, notas |

> **Consejo:** en caso de duda, elige un `Vec`.

### Leer sin riesgo con get

`notas[10]` detiene el programa si el elemento no existe. `get` devuelve un `Option`
(ver [Option](types.md)):

```rust
let notas = vec![12, 15, 9];
println!("{:?}", notas.get(1));
println!("{:?}", notas.get(10));
```

```sortie
Some(15)
None
```

### Recorrer un vector

```rust
let mut precios = vec![10, 20, 30];
for p in &precios {
    print!("{p} ");
}
println!();
for p in &mut precios {
    *p += 1;
}
println!("{:?}", precios);
```

```sortie
10 20 30 
[11, 21, 31]
```

> **Nota:** el `&` delante de `precios` toma prestado el vector en lugar de
> consumirlo. Todo se explica en [Propiedad y préstamo](possession.md).

**Te toca:** escribe un programa que cree un `Vec` con `"pan"` y `"leche"`, añada
`"huevos"` con `push` y luego muestre cada artículo en su propia línea con `for`.

> **Solución**
>
> ```rust
> let mut compra = vec!["pan", "leche"];
> compra.push("huevos");
> for articulo in &compra {
>     println!("{articulo}");
> }
> ```
>
> ```sortie
> pan
> leche
> huevos
> ```

## Las tablas: HashMap

Un `HashMap` es un [diccionario](../debuter/glossaire.md#diccionario): asocia
**claves** a **valores**, como una guía telefónica asocia un nombre a un número.
No forma parte de los tipos cargados por defecto: hay que importarlo con `use`.

Con un `Vec`, encuentras un elemento por su posición. Pero para saber la edad de
Alicia, buscar «Alicia» es mucho más sencillo que recordar «el hueco 0».

```rust
use std::collections::HashMap;

fn main() {
    let mut edades = HashMap::new();
    edades.insert("Alicia", 30);
    edades.insert("Bruno", 25);
    edades.insert("Alicia", 31);
    println!("{:?}", edades.get("Alicia"));
    println!("{}", edades.len());
    println!("{}", edades.contains_key("Zoe"));
}
```

```sortie
Some(31)
2
false
```

Línea por línea:

- `HashMap::new()` crea una tabla vacía, guardada en `edades`;
- los dos primeros `insert` asocian 30 a `"Alicia"` y 25 a `"Bruno"`;
- el tercer `insert` reutiliza la clave `"Alicia"`: el valor 31 **reemplaza** a 30;
- `get("Alicia")` da `Some(31)`, `len()` cuenta 2 claves, y `contains_key("Zoe")`
  responde `false`: Zoe no está en la tabla.

**Te toca:** escribe un programa que asocie `"Francia"` a `"París"` e `"Italia"` a
`"Roma"` en un `HashMap`, y luego muestre la capital de Italia con `get`.

> **Solución**
>
> ```rust
> use std::collections::HashMap;
>
> fn main() {
>     let mut capitales = HashMap::new();
>     capitales.insert("Francia", "París");
>     capitales.insert("Italia", "Roma");
>     println!("{:?}", capitales.get("Italia"));
> }
> ```
>
> ```sortie
> Some("Roma")
> ```

### Contar con entry

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

`entry(...).or_insert(...)` crea el valor si falta y luego da acceso al valor. La
estrella `*` permite modificar lo que designa:

```rust
use std::collections::HashMap;

fn main() {
    let texto = "el gato y el perro y el conejo";
    let mut cuenta = HashMap::new();
    for palabra in texto.split_whitespace() {
        *cuenta.entry(palabra).or_insert(0) += 1;
    }
    let mut pares: Vec<_> = cuenta.into_iter().collect();
    pares.sort();
    println!("{:?}", pares);
}
```

```sortie
[("conejo", 1), ("el", 3), ("gato", 1), ("perro", 1), ("y", 2)]
```

> **Atención:** el orden de recorrido de un `HashMap` es aleatorio. Para un orden
> ordenado, ordena el resultado como arriba, o usa `BTreeMap`.

### BTreeMap: una tabla ordenada

`BTreeMap` se usa como `HashMap`, pero mantiene las claves en orden:

```rust
use std::collections::BTreeMap;

fn main() {
    let mut existencias = BTreeMap::new();
    existencias.insert("peras", 4);
    existencias.insert("albaricoques", 10);
    existencias.insert("kiwis", 7);
    for (fruta, cantidad) in &existencias {
        println!("{fruta}: {cantidad}");
    }
}
```

```sortie
albaricoques: 10
kiwis: 7
peras: 4
```

## Los iteradores

Duplicar todas las notas, quedarse solo con los números pares, calcular un total:
estas tareas aparecen sin parar. Los iteradores permiten escribirlas en una línea, sin
tener que escribir tú mismo un bucle `for`.

`iter()` crea un **iterador**: un objeto que da los elementos uno a uno, como la cinta
transportadora de una caja de supermercado. Después se encadenan operaciones con
[closures](fonctions.md#las-closures). Los iteradores tienen su propio capítulo:
[Los iteradores](iterateurs.md).

| Método | Efecto |
|--------|--------|
| `map(f)` | transforma cada elemento |
| `filter(f)` | conserva los elementos que cumplen `f` |
| `collect()` | reúne el resultado (a menudo en un `Vec`) |
| `sum()` | suma |
| `count()` | cuenta |

```rust
let numeros = vec![1, 2, 3, 4, 5, 6];
let dobles: Vec<i32> = numeros.iter().map(|n| n * 2).collect();
let pares: Vec<&i32> = numeros.iter().filter(|n| *n % 2 == 0).collect();
let total: i32 = numeros.iter().sum();
let grandes = numeros.iter().filter(|n| **n > 3).count();
println!("{:?}", dobles);
println!("{:?}", pares);
println!("{total} {grandes}");
```

```sortie
[2, 4, 6, 8, 10, 12]
[2, 4, 6]
21 3
```

Línea por línea:

- `map(|n| n * 2)` toma cada número `n` y lo multiplica por 2; `collect()` guarda los
  resultados en el nuevo `Vec` `dobles`;
- `filter(|n| *n % 2 == 0)` conserva solo los números cuyo resto al dividir por 2 es
  0: los pares;
- `sum()` lo suma todo: 1 + 2 + 3 + 4 + 5 + 6 = 21;
- la última línea conserva los números mayores que 3 (4, 5 y 6) y los cuenta: 3.

El vector `numeros` no cambia: cada línea fabrica un resultado nuevo.

Se pueden encadenar tantos pasos como se quiera:

```rust
let palabras = vec!["rust", "es", "rápido"];
let resultado: Vec<String> = palabras
    .iter()
    .filter(|p| p.len() > 3)
    .map(|p| p.to_uppercase())
    .collect();
println!("{:?}", resultado);
```

```sortie
["RUST", "RÁPIDO"]
```

> **Consejo:** un iterador no hace nada mientras no lo consumas (con `collect`, `sum`,
> `for`…). Se dice que es **perezoso**.

**Te toca:** escribe un programa que parta de `vec![5, 12, 8, 20]` (unos precios),
guarde en un nuevo `Vec` los precios mayores que 10 y luego lo muestre.

> **Solución**
>
> ```rust
> let precios = vec![5, 12, 8, 20];
> let caros: Vec<&i32> = precios.iter().filter(|p| **p > 10).collect();
> println!("{:?}", caros);
> ```
>
> ```sortie
> [12, 20]
> ```

## Los slices

A veces solo quieres trabajar con un trozo de una lista: los tres primeros de una
clasificación, o las notas del segundo trimestre. Un slice muestra ese trozo, como una
ventana colocada sobre la lista.

Un **slice** (`&[T]`) es una vista sobre una parte de un array o de un vector, sin
copia:

```rust
let letras = vec!['a', 'b', 'c', 'd', 'e'];
let centro = &letras[1..4];
let inicio = &letras[..2];
println!("{:?} {:?}", centro, inicio);
```

```sortie
['b', 'c', 'd'] ['a', 'b']
```

Línea por línea:

- `letras` contiene cinco letras, en las posiciones 0 a 4;
- `&letras[1..4]` toma las posiciones 1, 2 y 3: la posición 4 **no** se incluye;
- `&letras[..2]` empieza al principio y se para antes de la posición 2: `'a'` y `'b'`;
- no se copia nada: `centro` e `inicio` miran directamente dentro de `letras`.

Una función que acepta `&[i32]` funciona tanto con un array como con un `Vec`:

```rust
fn media(valores: &[f64]) -> f64 {
    valores.iter().sum::<f64>() / valores.len() as f64
}

fn main() {
    let v = vec![10.0, 12.0, 17.0];
    println!("{}", media(&v));
    println!("{}", media(&[4.0, 6.0]));
}
```

```sortie
13
5
```

## Errores frecuentes

### Olvidar importar HashMap

```rust norun
fn main() {
    let mut edades = HashMap::new();
    edades.insert("Alicia", 30);
}
```

```text
error[E0433]: cannot find type `HashMap` in this scope
 --> src/main.rs:2:22
  |
2 |     let mut edades = HashMap::new();
  |                      ^^^^^^^ use of undeclared type `HashMap`
help: consider importing this struct
  |
1 + use std::collections::HashMap;
```

Añade `use std::collections::HashMap;` al principio del archivo.

### Mezclar tipos en un Vec

```rust norun
fn main() {
    let mezcla = vec![1, "dos", 3];
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:2:26
  |
2 |     let mezcla = vec![1, "dos", 3];
  |                          ^^^^^ expected integer, found `&str`
```

Todos los elementos de un `Vec` tienen el mismo tipo. Escribe `vec![1, 2, 3]`, o guarda
valores distintos en una [enumeración](types.md).

### Reutilizar un Vec después de un for

```rust norun
fn main() {
    let notas = vec![12, 15, 9];
    for n in notas {
        println!("{n}");
    }
    println!("{}", notas.len());
}
```

```text
error[E0382]: borrow of moved value: `notas`
 --> src/main.rs:6:20
  |
2 |     let notas = vec![12, 15, 9];
  |         ----- move occurs because `notas` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     for n in notas {
  |              ----- `notas` moved due to this implicit call to `.into_iter()`
...
6 |     println!("{}", notas.len());
  |                    ^^^^^ value borrowed here after move
```

`for n in notas` consume el vector. Para conservarlo, recórrelo por préstamo:
`for n in &notas`. La razón se explica en [Propiedad y préstamo](possession.md).

## Ponte a prueba

**1.** ¿Qué da `vec![3, 1, 2].get(5)`?

> **Respuesta**
>
> `None`: no hay ningún elemento en la posición 5. Con `[5]`, el programa se
> detendría.

**2.** ¿Cómo contar cuántas veces aparece cada letra en `"papa"`?

> **Respuesta**
>
> ```rust
> use std::collections::BTreeMap;
>
> fn main() {
>     let mut cuenta = BTreeMap::new();
>     for c in "papa".chars() {
>         *cuenta.entry(c).or_insert(0) += 1;
>     }
>     println!("{:?}", cuenta);
> }
> ```
>
> ```sortie
> {'a': 2, 'p': 2}
> ```
>
> Un `HashMap` también funciona, pero `BTreeMap` muestra las letras en orden.

**3.** Con `let v = vec![1, 2, 3, 4];`, ¿cómo obtener la suma de los números pares?

> **Respuesta**
>
> ```rust
> let v = vec![1, 2, 3, 4];
> let suma: i32 = v.iter().filter(|n| *n % 2 == 0).sum();
> println!("{suma}");
> ```
>
> ```sortie
> 6
> ```

## En resumen

- Un array `[T; N]` tiene un tamaño fijo; un `Vec` crece: `vec![…]`, `push`, `pop`,
  `get`.
- `HashMap` asocia claves a valores; su orden es aleatorio.
- `BTreeMap` mantiene las claves ordenadas.
- `iter()`, `map`, `filter` y `collect` transforman una colección.
- `&v[1..3]` es un slice: una vista sobre una parte de una lista.

Compara con [Ruby](../ruby/collections.md) y [Rythme](../rythme/collections.md).

Siguiente: [HashMap, HashSet y BTreeMap](hashmap.md)
