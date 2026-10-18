# Nombres et calculs

Rust propose plusieurs types de nombres, entiers et à virgule : ce chapitre montre comment les choisir, calculer avec, les convertir et les afficher proprement.

> **Objectifs :**
>
> - choisir le bon type d'entier ou de nombre à virgule ;
> - calculer avec les opérateurs, la division entière et le reste `%` ;
> - éviter les dépassements avec `checked_`, `wrapping_` et `saturating_` ;
> - convertir un nombre avec `as`, `From`, `TryFrom` et `parse` ;
> - afficher un nombre avec des décimales, des espaces ou des zéros.

## Les entiers

Un [entier](../debuter/glossaire.md#entier) est un nombre sans virgule : `3`, `-12`,
`2026`. Rust a plusieurs types d'entiers. Leur nom dit deux choses :

- la lettre : `i` pour un entier **signé** (il peut être négatif), `u` pour un entier
  **non signé** (*unsigned* : jamais négatif) ;
- le nombre : la taille en bits. Plus il y a de bits, plus le nombre peut être grand.

| Type | Plus petite valeur | Plus grande valeur |
|------|--------------------|--------------------|
| `i8` | -128 | 127 |
| `u8` | 0 | 255 |
| `i16` | -32 768 | 32 767 |
| `u16` | 0 | 65 535 |
| `i32` | -2 147 483 648 | 2 147 483 647 |
| `u32` | 0 | 4 294 967 295 |
| `i64` | environ -9,2 × 10¹⁸ | environ 9,2 × 10¹⁸ |
| `u64` | 0 | environ 1,8 × 10¹⁹ |
| `i128`, `u128` | encore plus grands | (38 chiffres environ) |

Quand vous n'écrivez pas de type, Rust choisit `i32`. C'est un bon choix par défaut.

```rust
let annee = 2026;          // i32, choisi par Rust
let age: u8 = 12;          // un âge n'est jamais négatif ni très grand
let population: u64 = 8_100_000_000;
println!("{annee} {age} {population}");
```

```sortie
2026 12 8100000000
```

> **Astuce :** le tiret bas `_` sépare les chiffres pour la lecture : `8_100_000_000`
> est le même nombre que `8100000000`.

### usize : le type des positions

`usize` est un entier non signé dont la taille dépend de l'ordinateur (64 bits sur un
ordinateur récent). C'est le type des **positions** et des **longueurs** : `len()`
renvoie un `usize`, et un indice de vecteur doit être un `usize`.

```rust
let fruits = vec!["pomme", "kiwi", "poire"];
let n: usize = fruits.len();
let i: usize = 1;
println!("{n} fruits, le deuxième est {}", fruits[i]);
```

```sortie
3 fruits, le deuxième est kiwi
```

## Les nombres à virgule

Un [flottant](../debuter/glossaire.md#flottant) est un nombre à virgule. En Rust, on
écrit un **point** : `3.14`. Il en existe deux types :

- `f64` : précis (environ 15 chiffres), choisi par défaut ;
- `f32` : moins précis (environ 7 chiffres), mais deux fois plus petit en mémoire.

```rust
let prix = 19.99;          // f64
let taille: f32 = 1.75;
println!("{prix} {taille}");
```

```sortie
19.99 1.75
```

### Les flottants ne sont pas exacts

L'ordinateur range les nombres en binaire. Beaucoup de nombres décimaux, comme `0.1`,
n'ont pas d'écriture binaire exacte : c'est comme `1/3 = 0,333…` en décimal. Il reste
une toute petite erreur :

```rust
let somme: f64 = 0.1 + 0.2;
println!("{somme}");
println!("{}", somme == 0.3);
println!("{}", (somme - 0.3).abs() < 1e-9);
```

```sortie
0.30000000000000004
false
true
```

> **À retenir :** ne comparez jamais deux flottants avec `==`. Vérifiez plutôt que leur
> écart est très petit. Pour de l'argent, comptez en centimes avec un entier.

## Les opérateurs

Les [opérateurs](../debuter/glossaire.md#operateur) habituels fonctionnent comme en
mathématiques :

| Opérateur | Sens | Exemple | Résultat |
|-----------|------|---------|----------|
| `+` | addition | `7 + 2` | `9` |
| `-` | soustraction | `7 - 2` | `5` |
| `*` | multiplication | `7 * 2` | `14` |
| `/` | division | `7.0 / 2.0` | `3.5` |
| `%` | reste de la division | `7 % 2` | `1` |

Les formes courtes `+=`, `-=`, `*=`, `/=` modifient une variable `mut` :

```rust
let mut score = 10;
score += 5;
score *= 2;
println!("{score}");
```

```sortie
30
```

### La division entière et le reste

Entre deux entiers, `/` donne une **division entière** : la partie après la virgule
est jetée. `%` donne le **reste**. Pensez à 7 bonbons partagés entre 2 enfants :
chacun en reçoit 3, et il en reste 1.

```rust
println!("{}", 7 / 2);
println!("{}", 7 % 2);
println!("{}", 7.0 / 2.0);
let minutes = 135;
println!("{} h {} min", minutes / 60, minutes % 60);
```

```sortie
3
1
3.5
2 h 15 min
```

> **Astuce :** `n % 2 == 0` est la façon classique de tester qu'un nombre est pair.

## Le dépassement de capacité

Un `u8` ne dépasse pas 255. Que se passe-t-il si un calcul va plus loin ? Cela
s'appelle un **dépassement** (en anglais : *overflow*).

- En mode débogage (`cargo run`), le programme **s'arrête** avec une
  [panique](../debuter/glossaire.md#panique) : `attempt to add with overflow`.
- En mode optimisé (`cargo run --release`), le nombre **fait le tour** : 255 + 1
  donne 0, comme un compteur kilométrique.

Pour ne pas dépendre du mode, dites vous-même ce que vous voulez avec trois familles de
méthodes :

| Méthode | Si ça dépasse… | `250u8 + 10` |
|---------|----------------|--------------|
| `checked_add` | renvoie `None` | `None` |
| `wrapping_add` | fait le tour | `4` |
| `saturating_add` | s'arrête au maximum | `255` |

```rust
let x: u8 = 250;
println!("{:?}", x.checked_add(10));
println!("{:?}", x.checked_add(3));
println!("{}", x.wrapping_add(10));
println!("{}", x.saturating_add(10));
let vies: u8 = 1;
println!("{}", vies.saturating_sub(5));
```

```sortie
None
Some(253)
4
255
0
```

Les mêmes méthodes existent pour `sub`, `mul` et `div` : `checked_mul`,
`saturating_sub`… `checked_` est la plus sûre : elle renvoie une
[Option](types.md#le-type-option) que vous devez traiter.

## Convertir un nombre

Rust ne convertit **jamais** un nombre tout seul. Pour additionner un `i32` et un `f64`,
il faut en convertir un.

### Avec as : rapide, mais sans filet

`as` convertit toujours, quitte à perdre de l'information : la partie décimale est
coupée, et un nombre trop grand est déformé.

```rust
let n: i32 = 7;
let x = n as f64 / 2.0;
println!("{x}");
println!("{}", 3.99_f64 as i32);
println!("{}", 300_i32 as u8);
println!("{}", 'A' as u32);
```

```sortie
3.5
3
44
65
```

`300 as u8` donne `44` (300 - 256) sans prévenir : utilisez `as` seulement quand vous
êtes sûr que le nombre tient.

### Avec From et TryFrom : sans surprise

`From` convertit quand c'est **toujours possible** (d'un petit type vers un plus
grand). `TryFrom` convertit quand c'est **peut-être possible** : il renvoie un
[Result](../debuter/glossaire.md#result).

```rust
let petit: u8 = 200;
let grand = i64::from(petit);
println!("{grand}");

let ok = u8::try_from(100_i32);
let trop = u8::try_from(300_i32);
println!("{:?}", ok);
println!("{:?}", trop.is_err());
```

```sortie
200
Ok(100)
true
```

### Depuis du texte avec parse

`parse` transforme un texte en nombre. Il faut dire quel type on veut, et traiter
l'échec possible (voir [Transformer un texte en nombre](texte.md#transformer-un-texte-en-nombre)) :

```rust
let age: u32 = "42".parse().unwrap();
let taille = "1.80".parse::<f64>().unwrap();
println!("{age} {taille}");
match "douze".parse::<i32>() {
    Ok(n) => println!("nombre : {n}"),
    Err(e) => println!("erreur : {e}"),
}
```

```sortie
42 1.8
erreur : invalid digit found in string
```

## Les méthodes mathématiques

Les nombres ont des méthodes. On les appelle avec un point :

```rust
println!("{}", 2_i32.pow(10));
println!("{}", 16.0_f64.sqrt());
println!("{}", (-5_i32).abs());
println!("{}", 3.max(8));
println!("{}", 3.min(8));
println!("{}", 15.clamp(0, 10));
println!("{}", 2.6_f64.round());
println!("{}", 2.6_f64.floor());
println!("{}", 2.1_f64.ceil());
```

```sortie
1024
4
5
8
3
10
3
2
3
```

- `pow` : puissance (entiers) ; pour les flottants, `powi` ou `powf`.
- `sqrt` : racine carrée (flottants seulement).
- `clamp(min, max)` : garde le nombre entre deux bornes, comme un volume entre 0 et 10.
- `round`, `floor`, `ceil` : arrondi au plus proche, vers le bas, vers le haut.

> **Attention :** écrivez `(-5_i32).abs()` avec des parenthèses. Sans elles,
> `-5_i32.abs()` se lit `-(5.abs())` et donne `-5`.

## Les constantes utiles

Chaque type connaît ses limites, et `std::f64::consts` contient les constantes
mathématiques :

```rust
use std::f64::consts::PI;

println!("{}", i32::MAX);
println!("{}", u8::MAX);
println!("{}", i64::MIN);
let rayon = 2.0;
println!("{}", PI * rayon * rayon);
```

```sortie
2147483647
255
-9223372036854775808
12.566370614359172
```

## Afficher un nombre joliment

Entre les accolades de `println!`, après deux-points `:`, vous pouvez régler
l'affichage :

| Format | Effet | Exemple | Résultat |
|--------|-------|---------|----------|
| `{:.2}` | 2 chiffres après la virgule | `3.14159` | `3.14` |
| `{:>8}` | aligné à droite sur 8 caractères | `42` | `      42` |
| `{:<8}` | aligné à gauche sur 8 caractères | `42` | `42      ` |
| `{:08.3}` | 8 caractères, zéros devant, 3 décimales | `3.14159` | `0003.142` |
| `{:e}` | notation scientifique | `1500.0` | `1.5e3` |

```rust
let pi = 3.14159;
println!("[{:.2}]", pi);
println!("[{:>8}]", 42);
println!("[{:<8}]", 42);
println!("[{:08.3}]", pi);
println!("[{:e}]", 1500.0);
println!("[{pi:>10.1}]");
```

```sortie
[3.14]
[      42]
[42      ]
[0003.142]
[1.5e3]
[       3.1]
```

Ce réglage est pratique pour aligner un tableau de prix :

```rust
let articles = [("pain", 1.2), ("fromage", 4.75), ("pommes", 12.0)];
for (nom, prix) in articles {
    println!("{nom:<10}{prix:>7.2} €");
}
```

```sortie
pain         1.20 €
fromage      4.75 €
pommes      12.00 €
```

## Erreurs fréquentes

### Mélanger un entier et un flottant

```rust norun
fn main() {
    let quantite: i32 = 3;
    let prix = 2.5;
    let total = quantite * prix;
    println!("{total}");
}
```

```text
error[E0277]: cannot multiply `i32` by `{float}`
 --> src/main.rs:4:26
  |
4 |     let total = quantite * prix;
  |                          ^ no implementation for `i32 * {float}`
  |
  = help: the trait `Mul<{float}>` is not implemented for `i32`
```

Rust ne convertit pas tout seul. Écrivez `quantite as f64 * prix`.

### Un nombre trop grand pour son type

```rust norun
fn main() {
    let age: u8 = 300;
    println!("{age}");
}
```

```text
error: literal out of range for `u8`
 --> src/main.rs:2:19
  |
2 |     let age: u8 = 300;
  |                   ^^^
  |
  = note: the literal `300` does not fit into the type `u8` whose range is `0..=255`
  = note: `#[deny(overflowing_literals)]` on by default
```

Un `u8` s'arrête à 255. Choisissez un type plus grand : `u16` ou `i32`.

### Un calcul qui dépasse

```rust norun
fn main() {
    let x: u8 = 255;
    let y = x + 1;
    println!("{y}");
}
```

```text
error: this arithmetic operation will overflow
 --> src/main.rs:3:13
  |
3 |     let y = x + 1;
  |             ^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
  |
  = note: `#[deny(arithmetic_overflow)]` on by default
```

Ici, le compilateur voit le problème d'avance. Choisissez un type plus grand, ou dites
ce que vous voulez : `x.checked_add(1)` ou `x.saturating_add(1)`.

### parse sans type

```rust norun
fn main() {
    let n = "42".parse().unwrap();
    println!("{n}");
}
```

```text
error[E0284]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let n = "42".parse().unwrap();
  |         ^        ----- type must be known at this point
  |
  = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `n` an explicit type
  |
2 |     let n: /* Type */ = "42".parse().unwrap();
  |          ++++++++++++
```

`parse` peut produire un `i32`, un `u8`, un `f64`… Dites lequel : `let n: i32 = …` ou
`"42".parse::<i32>()`.

## Testez-vous

**1.** Que valent `17 / 5` et `17 % 5` ?

> **Réponse**
>
> `3` et `2` : 17 = 5 × 3 + 2. Entre deux entiers, `/` jette la partie décimale.

**2.** Quel type choisir pour l'indice d'un vecteur ? Et pour une température
avec des décimales ?

> **Réponse**
>
> `usize` pour un indice ; `f64` pour une température.

**3.** Comment afficher `2.0 / 3.0` avec exactement deux décimales ?

> **Réponse**
>
> Avec le format `{:.2}` :
>
> ```rust
> println!("{:.2}", 2.0 / 3.0);
> ```
>
> ```sortie
> 0.67
> ```

## En résumé

- `i32` et `f64` sont les types par défaut ; `usize` sert aux positions et longueurs.
- Entre entiers, `/` est une division entière et `%` donne le reste.
- Les flottants ne sont pas exacts : ne les comparez pas avec `==`.
- `checked_`, `wrapping_` et `saturating_` disent quoi faire en cas de dépassement.
- `as` convertit sans filet ; `From`, `TryFrom` et `parse` sont plus sûrs.
- `{:.2}`, `{:>8}` et `{:08.3}` règlent l'affichage d'un nombre.

Suite : [Conditions et boucles](conditions-boucles.md)
