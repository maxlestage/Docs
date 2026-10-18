# Aide-mémoire

Cette page résume en tableaux et en courts exemples tout ce qu'il faut retenir de la documentation Rust.

## Outils

| Commande | Effet |
|----------|-------|
| `rustup update` | met Rust à jour |
| `cargo new nom` | crée un projet |
| `cargo run` | compile et lance |
| `cargo check` | vérifie sans compiler |
| `cargo test` | lance les tests |
| `cargo add crate` | ajoute une dépendance |
| `cargo fmt` / `cargo clippy` | met en forme / conseille |

Détails : [introduction](introduction.md) et [projets](projet.md#les-commandes-cargo).

## Variables et types

| Écriture | Sens |
|----------|------|
| `let x = 5;` | variable non modifiable |
| `let mut x = 5;` | variable modifiable |
| `let x: f64 = 2.5;` | type explicite |
| `const MAX: u32 = 10;` | constante |
| `i32`, `u32`, `f64`, `bool`, `char` | types simples |
| `&str` / `String` | texte emprunté / texte possédé |

```rust
let mut n = 1;
n += 1;
let n = n * 10; // masquage
let s = format!("n = {n:>4}");
println!("{s} {:?}", "texte");
```

```sortie
n =   20 "texte"
```

Détails : [Premiers pas](premiers-pas.md).

## Conditions et boucles

```rust
let x = 7;
let parite = if x % 2 == 0 { "pair" } else { "impair" };
let taille = match x {
    0 => "nul",
    1..=9 => "petit",
    _ => "grand",
};
println!("{parite} {taille}");
for i in 0..3 {
    print!("{i}");
}
let mut k = 0;
while k < 2 {
    k += 1;
}
let v = loop {
    break k * 10;
};
println!(" {v}");
```

```sortie
impair petit
012 20
```

Détails : [Conditions et boucles](conditions-boucles.md).

## Fonctions et closures

```rust
fn ajouter(a: i32, b: i32) -> i32 {
    a + b // pas de ; : c'est la valeur renvoyée
}

fn main() {
    let triple = |x: i32| x * 3;
    println!("{} {}", ajouter(1, 2), triple(4));
}
```

```sortie
3 12
```

Détails : [Les fonctions](fonctions.md).

## Collections

| Écriture | Sens |
|----------|------|
| `vec![1, 2]` | crée un `Vec` |
| `v.push(x)` / `v.pop()` | ajoute / retire à la fin |
| `v.get(i)` | `Option` sur l'élément `i` |
| `&v[1..3]` | tranche |
| `HashMap::new()` | table clé-valeur |
| `m.insert(k, v)` / `m.get(&k)` | ajoute / lit |
| `*m.entry(k).or_insert(0) += 1` | compte |
| `BTreeMap` | table triée par clé |

```rust
let v = vec![1, 2, 3, 4];
let carres: Vec<i32> = v.iter().map(|x| x * x).collect();
let somme_pairs: i32 = v.iter().filter(|x| *x % 2 == 0).sum();
println!("{:?} {somme_pairs} {:?}", carres, &v[..2]);
```

```sortie
[1, 4, 9, 16] 6 [1, 2]
```

Détails : [Collections](collections.md).

## Possession et emprunt

| Écriture | Sens |
|----------|------|
| `let b = a;` | déplace `a` (copie pour les types simples) |
| `a.clone()` | copie indépendante |
| `&a` | emprunt en lecture (plusieurs possibles) |
| `&mut a` | emprunt en écriture (un seul à la fois) |
| `fn f<'a>(x: &'a str) -> &'a str` | durée de vie explicite |

```rust
fn ajouter_s(mot: &mut String) {
    mot.push('s');
}

fn main() {
    let mut chat = String::from("chat");
    let copie = chat.clone();
    ajouter_s(&mut chat);
    println!("{copie} {chat}");
}
```

```sortie
chat chats
```

Détails : [Possession et emprunt](possession.md).

## Structures, énumérations et traits

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn somme(&self) -> i32 {
        self.x + self.y
    }
}

enum Commande {
    Avancer(i32),
    Stop,
}

trait Decrire {
    fn decrire(&self) -> String;
}

impl Decrire for Point {
    fn decrire(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 2, y: 3 };
    println!("{:?} {} {}", p, p.somme(), p.decrire());
    for c in [Commande::Avancer(5), Commande::Stop] {
        match c {
            Commande::Avancer(n) => println!("avance de {n}"),
            Commande::Stop => println!("stop"),
        }
    }
}
```

```sortie
Point { x: 2, y: 3 } 5 (2, 3)
avance de 5
stop
```

Détails : [Structures, énumérations et traits](types.md).

## Option et Result

| Écriture | Sens |
|----------|------|
| `Some(v)` / `None` | valeur présente / absente |
| `Ok(v)` / `Err(e)` | réussite / échec |
| `x?` | renvoie l'erreur à l'appelant |
| `x.unwrap()` / `x.expect("…")` | la valeur, ou `panic!` |
| `x.unwrap_or(v)` | la valeur, ou `v` |
| `if let Some(v) = x { … }` | agit seulement si présent |

```rust
fn lire(t: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = t.parse()?;
    Ok(n * 2)
}

fn main() {
    println!("{:?}", lire("21"));
    println!("{}", lire("x").unwrap_or(0));
    if let Some(c) = "abc".chars().next() {
        println!("{c}");
    }
}
```

```sortie
Ok(42)
0
a
```

Détails : [Gérer les erreurs](erreurs.md).

## Modules et tests

```rust
mod outils {
    pub fn double(n: i32) -> i32 {
        n * 2
    }
}

use outils::double;

fn main() {
    assert_eq!(double(4), 8);
    println!("ok");
}
```

```sortie
ok
```

Détails : [Projets, crates et tests](projet.md#les-tests).

> **Astuce :** les aide-mémoire des autres langages : [Ruby](../ruby/aide-memoire.md) et
> [Rythme](../rythme/aide-memoire.md).
