# Rust : introduction et installation

Rust est un langage rapide et sûr : cette page explique ce qu'il apporte, comment l'installer et comment lancer votre premier programme.

> **Objectifs :**
>
> - savoir ce qu'est un langage compilé et ce que Rust apporte ;
> - installer Rust avec `rustup` ;
> - créer et lancer un projet avec `cargo` ;
> - lire un petit programme Rust et le modifier ;
> - comprendre un premier message d'erreur du compilateur.

## Qu'est-ce que Rust ?

Rust est un langage de programmation **compilé**. Un
[compilateur](../debuter/glossaire.md#compilateur) est un programme qui traduit votre
code en un fichier exécutable, que l'ordinateur lance directement. La traduction a lieu
une seule fois, avant l'exécution : le programme obtenu est très rapide.

Pensez à un livre traduit à l'avance : le lecteur étranger le lit d'une traite, sans
attendre un interprète phrase par phrase.

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

L'outil officiel s'appelle `rustup`. Il installe deux programmes :

- `rustc`, le compilateur ;
- `cargo`, l'outil qui gère vos projets (il appelle `rustc` pour vous).

### Sous Linux et macOS

Ouvrez un terminal et tapez la commande ci-dessous. Si le terminal vous est inconnu,
lisez d'abord [Utiliser le terminal](../debuter/terminal.md).

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
├── Cargo.toml      la description du projet (nom, version…)
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

Lisons-le ligne par ligne :

- `fn main()` déclare la [fonction](../debuter/glossaire.md#fonction) `main`. C'est le
  point de départ de tout programme Rust : il commence toujours là.
- Les accolades `{ … }` entourent le **corps** de la fonction : les instructions à
  exécuter, dans l'ordre.
- `let` crée une [variable](../debuter/glossaire.md#variable) : un nom posé sur une
  [valeur](../debuter/glossaire.md#valeur).
- `println!` affiche une ligne. Le `!` indique une **macro**, une sorte de fonction
  spéciale. `{prenom}` est remplacé par la valeur de `prenom`.
- Chaque instruction se termine par un point-virgule `;`.

## Compiler sans cargo

Pour un seul fichier, vous pouvez aussi utiliser `rustc` directement. Il crée un
exécutable, que vous lancez ensuite :

```sh
rustc main.rs
./main
```

Dans un vrai projet, préférez toujours `cargo`. Il est présenté en détail dans
[Projets, crates et tests](projet.md).

## Erreurs fréquentes

### Oublier le ! de println!

```rust norun
fn main() {
    println("Bonjour !");
}
```

```text
error[E0423]: expected function, found macro `println`
 --> src/main.rs:2:5
  |
2 |     println("Bonjour !");
  |     ^^^^^^^ not a function
help: use `!` to invoke the macro
```

`println!` est une macro : le `!` est obligatoire. Écrivez `println!("Bonjour !");`.

### Oublier un point-virgule

```rust norun
fn main() {
    let prenom = "Alice"
    println!("Bonjour, {prenom} !");
}
```

```text
error: expected `;`, found `println`
 --> src/main.rs:2:25
  |
2 |     let prenom = "Alice"
  |                         ^ help: add `;` here
3 |     println!("Bonjour, {prenom} !");
  |     ------- unexpected token
```

Le compilateur montre où ajouter le `;` : à la fin de la ligne 2.

### Mettre le texte entre apostrophes

```rust norun
fn main() {
    println!('Bonjour !');
}
```

```text
error[E0762]: unterminated character literal
 --> src/main.rs:2:24
  |
2 |     println!('Bonjour !');
  |                        ^^^
help: if you meant to write a string literal, use double quotes
```

En Rust, un texte s'écrit entre guillemets doubles : `"Bonjour !"`. Les apostrophes
`'a'` sont réservées à un seul caractère.

### Mal écrire main

```rust norun
fn Main() {
    println!("Bonjour !");
}
```

```text
error[E0601]: `main` function not found in crate `main`
 --> src/main.rs:3:2
  |
3 | }
  |  ^ consider adding a `main` function to `src/main.rs`
```

Rust distingue les majuscules des minuscules : la fonction de départ s'appelle `main`,
en minuscules.

## Testez-vous

**1.** Quelle commande crée un projet nommé `essai` ? Et laquelle le lance ?

> **Réponse**
>
> `cargo new essai` crée le projet. Ensuite, dans le dossier `essai`, `cargo run` le
> compile et le lance.

**2.** Que signifie le `!` dans `println!` ?

> **Réponse**
>
> Il indique une **macro**, une sorte de fonction spéciale. Sans le `!`, le programme
> ne compile pas.

**3.** Qu'affiche ce programme ?

```rust norun
fn main() {
    let ville = "Nantes";
    println!("J'habite à {ville}.");
}
```

> **Réponse**
>
> ```sortie
> J'habite à Nantes.
> ```

## En résumé

- Rust est un langage compilé, rapide et sûr.
- `rustup` installe Rust ; `rustup update` le met à jour.
- `cargo new nom` crée un projet, `cargo run` le compile et le lance.
- Un programme commence toujours par `fn main()`.
- `println!` affiche une ligne de texte ; chaque instruction finit par `;`.

Comparez avec [l'introduction à Ruby](../ruby/introduction.md) ou
[l'introduction à Rythme](../rythme/introduction.md).

Suite : [Premiers pas](premiers-pas.md).
