# Projets, crates et tests

Cette page montre comment organiser un vrai projet Rust avec cargo : le fichier `Cargo.toml`, les dépendances, les modules, les tests et les outils de qualité.

## Le fichier Cargo.toml

Chaque projet créé par `cargo new` contient un fichier `Cargo.toml`. Il décrit le
projet :

```toml
[package]
name = "bonjour"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- `name` : le nom du projet ;
- `version` : sa version ;
- `edition` : l'édition de Rust utilisée ;
- `[dependencies]` : les bibliothèques dont le projet a besoin.

> **Note :** en Rust, un paquet de code s'appelle une **crate**. Votre projet est une
> crate, et chaque bibliothèque aussi.

## Ajouter une dépendance

Les bibliothèques publiques sont sur [crates.io](https://crates.io). Pour en ajouter
une, utilisez `cargo add` :

```sh
cargo add rand
```

La ligne apparaît dans `Cargo.toml` :

```toml
[dependencies]
rand = "0.9"
```

Vous pouvez ensuite l'utiliser dans le code. Cargo la télécharge au prochain
`cargo build` ou `cargo run` :

```rust norun
use rand::Rng;

fn main() {
    let de = rand::rng().random_range(1..=6);
    println!("Le dé affiche {de}");
}
```

## Les commandes cargo

| Commande | Effet |
|----------|-------|
| `cargo new nom` | crée un projet |
| `cargo build` | compile |
| `cargo run` | compile et lance |
| `cargo check` | vérifie le code sans produire de programme (rapide) |
| `cargo build --release` | compile en mode optimisé |
| `cargo test` | lance les tests |

> **Astuce :** pendant que vous écrivez, `cargo check` est bien plus rapide que
> `cargo build`.

## Les modules

Un **module** (`mod`) range le code en groupes. Par défaut, tout ce qui est dans un
module est **privé**. Le mot `pub` le rend public.

```rust
mod geometrie {
    pub fn aire_carre(cote: f64) -> f64 {
        cote * cote
    }

    pub mod cercle {
        pub fn aire(rayon: f64) -> f64 {
            3.14 * rayon * rayon
        }
    }
}

use geometrie::cercle;

fn main() {
    println!("{}", geometrie::aire_carre(3.0));
    println!("{}", cercle::aire(2.0));
}
```

```sortie
9
12.56
```

`use` crée un raccourci : on écrit `cercle::aire` au lieu de `geometrie::cercle::aire`.

### Un module par fichier

Dans un projet, chaque module va souvent dans son propre fichier :

```text
src/
├── main.rs
└── geometrie.rs
```

Dans `main.rs`, une seule ligne suffit pour charger `geometrie.rs` :

```rust norun
mod geometrie;

fn main() {
    println!("{}", geometrie::aire_carre(3.0));
}
```

> **Attention :** une fonction sans `pub` n'est pas visible hors de son module. Le
> compilateur vous le dira avec l'erreur `function is private`.

## Les tests

Un test est une fonction marquée `#[test]`. Elle échoue si elle provoque un `panic!`,
par exemple avec `assert_eq!`. On range les tests dans un module `tests` :

```rust
fn ajouter(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ajoute_deux_nombres() {
        assert_eq!(ajouter(2, 3), 5);
    }

    #[test]
    fn ajoute_un_negatif() {
        assert_eq!(ajouter(2, -5), -3);
    }
}

fn main() {
    println!("{}", ajouter(2, 3));
}
```

```sortie
5
```

- `#[cfg(test)]` : ce module n'est compilé que pour les tests ;
- `use super::*;` : importe tout ce qui est dans le module parent.

Lancez les tests avec :

```sh
cargo test
```

Cargo affiche un résumé :

```text
running 2 tests
test tests::ajoute_deux_nombres ... ok
test tests::ajoute_un_negatif ... ok

test result: ok. 2 passed; 0 failed
```

### Les macros de vérification

| Macro | Vérifie que… |
|-------|--------------|
| `assert!(cond)` | la condition est vraie |
| `assert_eq!(a, b)` | `a == b` |
| `assert_ne!(a, b)` | `a != b` |

Elles marchent aussi en dehors des tests :

```rust
let total = 2 + 2;
assert_eq!(total, 4);
assert!(total > 3, "le total est trop petit");
println!("Tout va bien");
```

```sortie
Tout va bien
```

## Les outils de qualité

Rust fournit trois outils très pratiques :

```sh
cargo fmt      # met le code en forme, selon le style officiel
cargo clippy   # signale le code maladroit et propose mieux
cargo doc --open   # génère la documentation HTML et l'ouvre
```

`cargo doc` utilise les commentaires `///` placés avant vos fonctions :

```rust
/// Renvoie le double de `n`.
///
/// Exemple : `double(4)` vaut `8`.
pub fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    println!("{}", double(4));
}
```

```sortie
8
```

> **À retenir :** prenez l'habitude de lancer `cargo fmt` et `cargo clippy` avant de
> partager votre code.

## En résumé

- `Cargo.toml` décrit le projet et ses dépendances.
- `cargo add nom` ajoute une bibliothèque (une crate).
- `mod` range le code ; `pub` le rend visible ailleurs.
- `#[test]` et `assert_eq!` écrivent des tests, lancés par `cargo test`.
- `cargo fmt` et `cargo clippy` améliorent votre code.

Comparez avec [les projets en Ruby](../ruby/projet.md) et
[en Rythme](../rythme/projet.md). Suite : [Exercices corrigés](exercices.md). Tout le
résumé est dans l'[aide-mémoire](aide-memoire.md).
