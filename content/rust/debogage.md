# Déboguer en Rust

Déboguer, c'est trouver pourquoi un programme ne fait pas ce que vous attendez : en Rust, le compilateur, la macro dbg!, les traces de panique, clippy et un débogueur vous aident à chaque étape.

> **Objectifs :**
>
> - lire une erreur du compilateur de haut en bas, et obtenir son explication avec
>   `rustc --explain` ;
> - afficher des valeurs pendant l'exécution avec `dbg!` ;
> - remonter à l'origine d'une panique avec `RUST_BACKTRACE=1` ;
> - préférer `expect` à `unwrap`, et écouter les conseils de clippy ;
> - suivre un programme pas à pas dans VS Code.

Si le mot [débogage](../debuter/glossaire.md#debogage) est nouveau pour vous, lisez
d'abord [Déboguer : trouver et corriger les erreurs](../debuter/deboguer.md).

## Lire une erreur du compilateur

En Rust, beaucoup de [bugs](../debuter/glossaire.md#bug) sont arrêtés **avant**
l'exécution : le compilateur refuse le programme et explique pourquoi. Ses messages
sont longs, mais toujours construits de la même façon. Voici un programme avec une
faute de frappe :

```rust norun
fn main() {
    let notes = vec![12, 15, 9];
    let mut totla = 0;
    for n in &notes {
        total += n;
    }
    println!("{total}");
}
```

```text
error[E0425]: cannot find value `total` in this scope
 --> src/main.rs:5:9
  |
5 |         total += n;
  |         ^^^^^
  |
help: a local variable with a similar name exists
  |
5 -         total += n;
5 +         totla += n;
  |

error[E0425]: cannot find value `total` in this scope
 --> src/main.rs:7:16
  |
7 |     println!("{total}");
  |                ^^^^^

error: could not compile `notes` (bin "notes") due to 2 previous errors
```

Lisez chaque erreur dans cet ordre :

1. **la première ligne** : le code de l'erreur (`E0425`) et la phrase qui résume
   (« impossible de trouver la valeur `total` ») ;
2. **la flèche `-->`** : le fichier, la ligne et la colonne (`src/main.rs:5:9`) ;
3. **le dessin** : la ligne de code, avec `^^^^^` sous le morceau fautif ;
4. **`help:`** : souvent, la correction toute faite.

> **À retenir :** corrigez **la première erreur d'abord**, puis recompilez. Une seule
> cause produit souvent plusieurs erreurs : ici, deux erreurs, mais une seule faute
> (`totla` au lieu de `total`, ligne 3).

## Comprendre un code d'erreur : rustc --explain

Chaque code d'erreur a une explication détaillée, avec des exemples. Le compilateur le
rappelle à la fin de son message : `For more information about this error, try
rustc --explain E0382`. Par exemple, pour cette erreur de
[possession](../debuter/glossaire.md#possession) :

```text
error[E0382]: borrow of moved value: `noms`
 --> src/main.rs:4:20
  |
2 |     let noms = vec![String::from("Ada"), String::from("Alan")];
  |         ---- move occurs because `noms` has type `Vec<String>`, which does not implement the `Copy` trait
3 |     let copie = noms;
  |                 ---- value moved here
4 |     println!("{}", noms.len());
  |                    ^^^^ value borrowed here after move
```

```sh
rustc --explain E0382
```

```text
A variable was used after its contents have been moved elsewhere.

Erroneous code example:
...
Since `MyStruct` is a type that is not marked `Copy`, the data gets moved out
of `x` when we set `y`. This is fundamental to Rust's ownership system: outside
of workarounds like `Rc`, a value cannot be owned by more than one variable.
```

L'explication est en anglais, mais les exemples de code parlent d'eux-mêmes. Le même
texte est en ligne, dans l'index des erreurs de Rust (*Rust error codes index*).

## Afficher des valeurs avec dbg!

Quand le programme compile mais donne un mauvais résultat, il faut regarder ce que
valent les variables pendant l'exécution. La macro `dbg!` affiche le fichier, la ligne,
l'expression **et** sa valeur :

```rust norun
fn carre(x: i32) -> i32 {
    x * x
}

fn main() {
    let a = 4;
    let b = dbg!(carre(a)) + 1;
    dbg!(b);
    let liste = vec![1, 2, 3];
    dbg!(&liste);
    println!("fini : {b}");
}
```

```text
[src/main.rs:7:13] carre(a) = 16
[src/main.rs:8:5] b = 17
[src/main.rs:10:5] &liste = [
    1,
    2,
    3,
]
fini : 17
```

Trois choses à savoir :

- `dbg!` **renvoie** la valeur : on peut l'insérer au milieu d'un calcul
  (`dbg!(carre(a)) + 1`) sans rien changer au reste ;
- elle écrit sur la **sortie d'erreur** (*stderr*), pas sur la sortie normale : ses
  messages ne se mélangent pas aux résultats que votre programme doit produire ;
- elle a besoin du trait `Debug` (`#[derive(Debug)]` sur vos structures).

> **Astuce :** passez une référence (`dbg!(&liste)`) pour ne pas
> [déplacer](possession.md#le-deplacement) la valeur. Et enlevez vos `dbg!` une fois le
> bug trouvé : clippy peut vous le rappeler.

## Remonter une panique : RUST_BACKTRACE=1

Une [panique](../debuter/glossaire.md#panique) arrête le programme. Le message donne
la ligne où elle s'est produite :

```rust norun
fn troisieme(liste: &[i32]) -> i32 {
    liste[2]
}

fn calculer(liste: &[i32]) -> i32 {
    troisieme(liste) * 10
}

fn main() {
    let notes = vec![12, 15];
    println!("{}", calculer(&notes));
}
```

```text
thread 'main' (1710) panicked at src/main.rs:2:5:
index out of bounds: the len is 2 but the index is 2
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

La ligne 2 est dans `troisieme`. Mais **qui** a appelé `troisieme` avec une liste trop
courte ? La **trace d'appels** (*backtrace*) répond. Relancez avec la variable
d'environnement `RUST_BACKTRACE=1` :

```sh
RUST_BACKTRACE=1 cargo run
```

```text
thread 'main' (1711) panicked at src/main.rs:2:5:
index out of bounds: the len is 2 but the index is 2
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/2d8144b7880597b6e6d3dfd63a9a9efae3f533d3/library/std/src/panicking.rs:689:5
   1: core::panicking::panic_fmt
             at /rustc/2d8144b7880597b6e6d3dfd63a9a9efae3f533d3/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/2d8144b7880597b6e6d3dfd63a9a9efae3f533d3/library/core/src/panicking.rs:271:5
   3: notes::troisieme
             at ./src/main.rs:2:5
   4: notes::calculer
             at ./src/main.rs:6:5
   5: notes::main
             at ./src/main.rs:11:20
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /rustc/2d8144b7880597b6e6d3dfd63a9a9efae3f533d3/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

Cherchez les lignes qui portent le nom de **votre** projet (ici `notes::`) et ignorez
celles de Rust lui-même (`core::`, `std::`). On lit la pile du haut vers le bas :
`main` (ligne 11) a appelé `calculer` (ligne 6), qui a appelé `troisieme` (ligne 2).

> **Note :** sous Windows, dans PowerShell, écrivez
> `$env:RUST_BACKTRACE=1; cargo run`.

## unwrap ou expect : des messages utiles

`unwrap()` arrête le programme sur un `Err` ou un `None`, avec un message générique
(voir [Gérer les erreurs](erreurs.md#unwrap-et-expect)) :

```rust norun
use std::fs;

fn main() {
    let texte = fs::read_to_string("reglages.txt").unwrap();
    println!("{texte}");
}
```

```text
thread 'main' (2501) panicked at src/main.rs:4:52:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

`expect("…")` fait la même chose, mais commence par **votre** message. Six mois plus
tard, vous saurez tout de suite ce qui manquait :

```rust norun
let texte = fs::read_to_string("reglages.txt").expect("impossible de lire reglages.txt");
```

```text
thread 'main' (2543) panicked at src/main.rs:4:52:
impossible de lire reglages.txt: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

> **Astuce :** dans un vrai programme, préférez renvoyer l'erreur avec `?` plutôt que
> de paniquer. Gardez `unwrap` pour les essais rapides et les tests.

## Les conseils de clippy

**Clippy** est un relecteur automatique : il connaît des centaines de pièges et de
tournures maladroites. Il est installé avec Rust :

```sh
cargo clippy
```

```rust norun
fn main() {
    let notes = vec![12, 15, 9];
    if notes.len() == 0 {
        println!("aucune note");
    }
    let mut total = 0;
    for n in &notes {
        total = total + n;
    }
    println!("{total}");
}
```

```text
warning: length comparison to zero
 --> src/main.rs:3:8
  |
3 |     if notes.len() == 0 {
  |        ^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `notes.is_empty()`

warning: manual implementation of an assign operation
 --> src/main.rs:8:9
  |
8 |         total = total + n;
  |         ^^^^^^^^^^^^^^^^^ help: replace it with: `total += n`
```

Le programme était juste, mais clippy propose une écriture plus claire. Parfois, il
trouve de vrais bugs. `cargo clippy --fix` applique les corrections sûres tout seul.
Le chapitre [Bien écrire du Rust](style.md) en dit plus.

## Un débogueur dans VS Code

`dbg!` oblige à modifier le code et à relancer. Un **débogueur** permet de mettre le
programme en pause et d'inspecter toutes les variables, sans rien modifier. Dans
[VS Code](../debuter/editeur.md) :

1. installez les extensions **rust-analyzer** (l'aide à l'écriture) et **CodeLLDB**
   (le débogueur) ;
2. cliquez dans la marge, à gauche d'un numéro de ligne : un point rouge apparaît.
   C'est un **point d'arrêt** ;
3. cliquez sur **Debug**, au-dessus de `fn main` (un lien ajouté par rust-analyzer) ;
4. le programme s'arrête au point rouge. Le panneau de gauche montre chaque variable et
   sa valeur.

Ensuite, les boutons en haut de l'écran font avancer le programme :

| Bouton | Raccourci | Effet |
|--------|-----------|-------|
| Continuer | F5 | jusqu'au prochain point d'arrêt |
| Pas à pas principal | F10 | exécute la ligne, sans entrer dans les fonctions |
| Pas à pas détaillé | F11 | entre dans la fonction appelée |
| Arrêter | Maj+F5 | termine le programme |

C'est comme regarder un film image par image : vous voyez exactement à quel moment
l'histoire tourne mal.

## Une enquête complète

Voici une fonction qui calcule une moyenne. Elle compile sans erreur, mais le résultat
est faux :

```rust
fn moyenne(notes: &[u32]) -> f64 {
    let total: u32 = notes.iter().sum();
    let n = notes.len() as u32;
    (total / n) as f64
}

fn main() {
    println!("{}", moyenne(&[12, 15]));
}
```

```sortie
13
```

On attendait 13.5. **Étape 1 : observer.** On ajoute des `dbg!` pour voir chaque
valeur intermédiaire :

```rust norun
fn moyenne(notes: &[u32]) -> f64 {
    let total: u32 = notes.iter().sum();
    let n = notes.len() as u32;
    dbg!(total, n);
    dbg!(total / n) as f64
}
```

```text
[src/main.rs:4:5] total = 27
[src/main.rs:4:5] n = 2
[src/main.rs:5:5] total / n = 13
13
```

**Étape 2 : comprendre.** `total` et `n` sont justes. C'est la division qui perd la
virgule : entre deux entiers, `/` donne un entier (`27 / 2 = 13`). La conversion en
`f64` arrive trop tard.

**Étape 3 : corriger**, en convertissant **avant** de diviser, puis vérifier :

```rust
fn moyenne(notes: &[u32]) -> f64 {
    let total: u32 = notes.iter().sum();
    total as f64 / notes.len() as f64
}

fn main() {
    println!("{}", moyenne(&[12, 15]));
    println!("{}", moyenne(&[10, 10, 11]));
}
```

```sortie
13.5
10.333333333333334
```

**Étape 4 : protéger.** On ajoute un [test](tests.md) avec ce cas précis
(`assert_eq!(moyenne(&[12, 15]), 13.5);`) : si le bug revient un jour, `cargo test` le
dira. Et on enlève les `dbg!`.

> **À retenir :** observer, comprendre, corriger, protéger. Ne changez pas le code au
> hasard en espérant que ça marche : regardez d'abord les valeurs.

## Erreurs fréquentes

### Donner la valeur à dbg!

```rust norun
fn main() {
    let nom = String::from("Ada");
    dbg!(nom);
    println!("Bonjour {nom}");
}
```

```text
error[E0382]: borrow of moved value: `nom`
 --> src/main.rs:4:24
  |
2 |     let nom = String::from("Ada");
  |         --- move occurs because `nom` has type `String`, which does not implement the `Copy` trait
3 |     dbg!(nom);
  |     --------- value moved here
4 |     println!("Bonjour {nom}");
  |                        ^^^ value borrowed here after move
  |
help: consider borrowing instead of transferring ownership
  |
3 |     dbg!(&nom);
  |          +
```

`dbg!` prend la valeur, puis la renvoie : ici, on ne la récupère pas. Écrivez
`dbg!(&nom);`.

### Afficher une liste avec {}

```rust norun
fn main() {
    let notes = vec![12, 15, 9];
    println!("{}", notes);
}
```

```text
error[E0277]: `Vec<{integer}>` doesn't implement `std::fmt::Display`
 --> src/main.rs:3:20
  |
3 |     println!("{}", notes);
  |               --   ^^^^^ `Vec<{integer}>` cannot be formatted with the default formatter
  |
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

`{}` est l'affichage « pour l'utilisateur », qu'une liste n'a pas. Pour déboguer,
utilisez `{:?}` (ou `{:#?}`, sur plusieurs lignes).

### Une structure sans Debug

```rust norun
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    dbg!(&p);
}
```

```text
error[E0277]: `Point` doesn't implement `Debug`
 --> src/main.rs:8:5
  |
8 |     dbg!(&p);
  |     ^^^^^^^^ the trait `Debug` is not implemented for `Point`
  |
  = note: add `#[derive(Debug)]` to `Point` or manually `impl Debug for Point`
```

Ajoutez `#[derive(Debug)]` au-dessus de `struct Point`.

## Testez-vous

1. Le compilateur affiche cinq erreurs. Par laquelle commencer ?

> **Réponse**
>
> Par la première. Corrigez-la, puis recompilez : les suivantes en découlent souvent
> et disparaissent d'elles-mêmes.

2. Où `dbg!` écrit-il, et que renvoie-t-il ?

> **Réponse**
>
> Sur la sortie d'erreur (stderr). Il renvoie la valeur qu'on lui a donnée, ce qui
> permet de le placer au milieu d'une expression.

3. Comment savoir quelle fonction a appelé celle qui a paniqué ?

> **Réponse**
>
> En relançant avec `RUST_BACKTRACE=1 cargo run` : la trace d'appels montre la chaîne
> des fonctions, avec leurs lignes.

## En résumé

- Lisez les erreurs de haut en bas : code, position, dessin, `help:`. Corrigez la
  première d'abord ; `rustc --explain E0382` donne l'explication complète.
- `dbg!(&valeur)` affiche une valeur avec sa position, sur stderr.
- `RUST_BACKTRACE=1` montre la chaîne d'appels qui mène à une panique.
- `expect("message")` plutôt que `unwrap()` ; `cargo clippy` signale les maladresses.
- Avec CodeLLDB, un point d'arrêt met le programme en pause pour tout inspecter.

Suite : [Bien écrire du Rust](style.md)
