# Organiser un projet avec Cargo

Cargo est l'outil qui crée, compile, lance et teste vos projets Rust : cette page montre comment un projet est organisé et quelles commandes utiliser au quotidien.

> **Objectifs :**
>
> - créer un projet avec `cargo new` et connaître ses fichiers ;
> - lire le fichier `Cargo.toml` ;
> - utiliser les commandes `cargo` du quotidien ;
> - ajouter une bibliothèque (une crate) ;
> - garder un code propre avec `cargo fmt`, `cargo clippy` et `cargo doc`.

## Créer un projet

Dans un terminal, placez-vous dans le dossier où ranger vos projets, puis tapez :

```sh
cargo new bonjour
cd bonjour
```

Cargo crée un dossier `bonjour` avec tout le nécessaire :

```text
bonjour/
├── Cargo.toml      ← la fiche d'identité du projet
├── .gitignore      ← les fichiers que Git doit ignorer
└── src/
    └── main.rs     ← votre programme
```

Le fichier `src/main.rs` contient déjà un programme qui marche :

```rust
fn main() {
    println!("Hello, world!");
}
```

```sortie
Hello, world!
```

Après une compilation, un dossier `target/` apparaît : il contient le programme compilé.
Vous n'avez pas besoin d'y toucher.

> **Astuce :** pour une bibliothèque plutôt qu'un programme, tapez
> `cargo new --lib nom`. La différence est expliquée dans
> [Modules et crates](modules.md#bibliotheque-ou-programme-lib-et-bin).

## Le fichier Cargo.toml

`Cargo.toml` décrit le projet :

```toml
[package]
name = "bonjour"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- `name` : le nom du projet ;
- `version` : sa version ;
- `edition` : l'édition de Rust utilisée (la plus récente est `2024`) ;
- `[dependencies]` : les bibliothèques dont le projet a besoin (vide pour l'instant).

> **Note :** en Rust, un paquet de code s'appelle une **crate**. Votre projet est une
> crate, et chaque bibliothèque aussi.

## Les commandes cargo

| Commande | Effet |
|----------|-------|
| `cargo new nom` | crée un projet |
| `cargo build` | compile |
| `cargo run` | compile et lance |
| `cargo check` | vérifie le code sans produire de programme (rapide) |
| `cargo build --release` | compile en mode optimisé |
| `cargo test` | lance les tests |
| `cargo add nom` | ajoute une dépendance |

Toutes ces commandes se tapent **dans le dossier du projet** (celui qui contient
`Cargo.toml`).

Pour donner des arguments à votre programme, placez-les après `--` :

```sh
cargo run -- ajouter Acheter du pain
```

> **Astuce :** pendant que vous écrivez, `cargo check` est bien plus rapide que
> `cargo build`.

### Mode debug et mode release

Par défaut, `cargo build` compile vite, mais le programme obtenu n'est pas optimisé
(mode *debug*, rangé dans `target/debug/`). Pour la version finale, utilisez
`cargo build --release` : la compilation est plus lente, mais le programme est souvent
beaucoup plus rapide (rangé dans `target/release/`).

## Ajouter une dépendance

Les bibliothèques publiques sont sur [crates.io](https://crates.io). Pour en ajouter
une, utilisez `cargo add` :

```sh
cargo add rand
```

La ligne apparaît dans `Cargo.toml`, et Cargo télécharge la bibliothèque au prochain
`cargo build` :

```toml
[dependencies]
rand = "0.9"
```

Tout le détail, avec un exemple, est dans
[Modules et crates](modules.md#ajouter-une-crate-de-crates-io).

## Ranger le code en modules

Quand `main.rs` devient trop long, on range le code dans des **modules**, souvent un
par fichier (`src/geometrie.rs`, `src/affichage.rs`…). Le mot `pub` choisit ce qui est
visible depuis les autres fichiers, et `use` crée des raccourcis.

Tout est expliqué pas à pas dans [Modules et crates](modules.md).

## Les tests

Cargo sait aussi lancer vos **tests** : de petites fonctions marquées `#[test]` qui
vérifient votre code avec `assert_eq!`. Une seule commande les lance tous :

```sh
cargo test
```

Pour écrire vos premiers tests, lisez [Tester son code](tests.md).

## Les outils de qualité

Rust fournit trois outils très pratiques :

```sh
cargo fmt          # met le code en forme, selon le style officiel
cargo clippy       # signale le code maladroit et propose mieux
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

## Erreurs fréquentes

### Lancer cargo hors du dossier du projet

```sh
cargo run
```

```text
error: could not find `Cargo.toml` in `/home/lea` or any parent directory
```

Cargo cherche `Cargo.toml` dans le dossier courant. Placez-vous d'abord dans le projet :
`cd bonjour`, puis relancez.

### Utiliser une crate sans l'ajouter

```rust norun
use rand::Rng;

fn main() {
    let de = rand::rng().random_range(1..=6);
    println!("{de}");
}
```

```text
error[E0432]: unresolved import `rand`
 --> src/main.rs:1:5
  |
1 | use rand::Rng;
  |     ^^^^ use of unresolved module or unlinked crate `rand`
  |
  = help: if you wanted to use a crate named `rand`, use `cargo add rand` to add it to your `Cargo.toml`
```

Le compilateur donne la solution : `cargo add rand`.

### Un nom de projet avec des espaces

```sh
cargo new "Mon Jeu"
```

```text
error: invalid character ` ` in package name: `Mon Jeu`, characters must be Unicode XID characters (numbers, `-`, `_`, or most letters)
```

Utilisez des minuscules et des tirets bas : `cargo new mon_jeu`.

## Testez-vous

1. Quel fichier décrit le nom du projet et ses dépendances ?

> **Réponse**
>
> `Cargo.toml`, à la racine du projet.

2. Quelle commande vérifie le plus vite que le code compile, sans produire de
   programme ?

> **Réponse**
>
> `cargo check`.

3. Comment ajouter la bibliothèque `rand` à votre projet ?

> **Réponse**
>
> Avec `cargo add rand`, dans le dossier du projet. La ligne `rand = "…"` apparaît
> dans la section `[dependencies]` de `Cargo.toml`.

## En résumé

- `cargo new nom` crée un projet : `Cargo.toml` et `src/main.rs`.
- `Cargo.toml` décrit le projet et ses dépendances.
- `cargo run`, `cargo check`, `cargo test` : les commandes du quotidien.
- `cargo add nom` ajoute une bibliothèque (une crate).
- `cargo fmt` et `cargo clippy` améliorent votre code ; `cargo doc` le documente.

Comparez avec [les projets en Ruby](../ruby/projet.md) et
[en Rythme](../rythme/projet.md). Suite : [Projets pas à pas](projets.md). Tout le
résumé est dans l'[aide-mémoire](aide-memoire.md).
