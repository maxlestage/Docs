# Rust : introduction et installation

Rust est un langage rapide et sûr : cette page explique ce qu'il apporte, comment l'installer et comment lancer votre premier programme.

## Qu'est-ce que Rust ?

Rust est un langage de programmation **compilé**. Le compilateur transforme votre code en
un programme exécutable, très rapide.

Rust a trois grandes qualités :

- **La vitesse** : un programme Rust est aussi rapide qu'un programme C ou C++.
- **La sûreté** : le compilateur refuse beaucoup d'erreurs avant même l'exécution
  (mémoire mal utilisée, valeur absente oubliée…).
- **Les outils** : `cargo` crée, compile, teste et formate vos projets.

Voici un tout petit programme Rust :

```rust
fn main() {
    println!("Bonjour, Rust !");
}
```

```sortie
Bonjour, Rust !
```

> **Note :** Rust est exigeant au début. Le compilateur se plaint souvent, mais ses
> messages d'erreur sont clairs et proposent même des corrections.

## Installer Rust

L'outil officiel s'appelle `rustup`. Il installe le compilateur `rustc` et l'outil
`cargo`.

### Sous Linux et macOS

Ouvrez un terminal et tapez :

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Suivez les instructions, puis ouvrez un nouveau terminal.

### Sous Windows

Téléchargez `rustup-init.exe` sur le site [rustup.rs](https://rustup.rs) et lancez-le.
L'installateur peut vous demander d'installer aussi les outils de compilation de
Visual Studio : acceptez.

### Vérifier l'installation

```sh
rustc --version
cargo --version
```

Chaque commande affiche un numéro de version, par exemple `rustc 1.97.0`.

> **Astuce :** pour passer à la dernière version de Rust plus tard, tapez
> `rustup update`.

## Créer un projet avec cargo

`cargo new` crée un dossier de projet tout prêt :

```sh
cargo new bonjour
cd bonjour
```

Le dossier contient deux choses :

```text
bonjour/
├── Cargo.toml      la description du projet
└── src/
    └── main.rs     le code du programme
```

Pour compiler et lancer le programme, une seule commande :

```sh
cargo run
```

Cargo compile le code, puis l'exécute. Vous voyez `Hello, world!` s'afficher.

> **À retenir :** `cargo new` crée un projet, `cargo run` le compile et le lance.

## Votre premier programme

Ouvrez `src/main.rs` et remplacez son contenu :

```rust
fn main() {
    let prenom = "Alice";
    let age = 30;
    println!("Je m'appelle {prenom} et j'ai {age} ans.");
}
```

```sortie
Je m'appelle Alice et j'ai 30 ans.
```

Quelques remarques :

- `fn main()` est le point de départ de tout programme Rust.
- `let` crée une variable.
- `println!` affiche une ligne. Le `!` indique une **macro**, une sorte de fonction
  spéciale.
- Chaque instruction se termine par un point-virgule `;`.

## Compiler sans cargo

Pour un seul fichier, vous pouvez aussi utiliser `rustc` directement :

```sh
rustc main.rs
./main
```

Dans un vrai projet, préférez toujours `cargo`. Il est présenté en détail dans
[Projets, crates et tests](projet.md).

## Et ensuite ?

- Découvrez les variables et les types dans [Premiers pas](premiers-pas.md).
- Comparez avec [l'introduction à Ruby](../ruby/introduction.md) ou
  [l'introduction à Rythme](../rythme/introduction.md).
