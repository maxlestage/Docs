# Bien écrire du Rust

Un code juste ne suffit pas : il doit aussi être facile à lire, pour les autres et pour vous dans six mois ; cette page donne les conventions et les tournures que tout développeur Rust reconnaît.

> **Objectifs :**
>
> - nommer variables, fonctions, types et constantes selon les conventions ;
> - mettre en forme le code avec `cargo fmt` et le relire avec `cargo clippy` ;
> - utiliser les tournures idiomatiques : itérateurs, `?`, `if let`, combinateurs
>   d'`Option`, retours anticipés ;
> - emprunter dans les paramètres (`&str`, `&[T]`) et écrire de petites fonctions ;
> - documenter avec `///`.

## Pourquoi un style commun ?

Un texte bien écrit suit des règles d'orthographe et de ponctuation : on le lit sans
effort. Le code, c'est pareil. Tous les projets Rust suivent les **mêmes** conventions,
et des outils les appliquent pour vous. Résultat : en ouvrant n'importe quel projet,
vous êtes tout de suite chez vous.

Un code est bien plus souvent **lu** qu'écrit. Chaque minute passée à le rendre clair
fait gagner des heures plus tard.

## Les conventions de nommage

| Élément | Style | Exemple |
|---------|-------|---------|
| variable, fonction, méthode | `snake_case` | `prix_total`, `calculer_moyenne` |
| module, fichier, crate | `snake_case` | `mod facturation;`, `mon_jeu` |
| type : `struct`, `enum`, `trait` | `UpperCamelCase` | `Panier`, `CouleurFeu`, `Affichable` |
| variante d'`enum` | `UpperCamelCase` | `CouleurFeu::Rouge` |
| constante, `static` | `SCREAMING_SNAKE_CASE` | `const TAUX_TVA: f64 = 0.2;` |
| paramètre générique | une majuscule | `T`, `K`, `V` |
| durée de vie | courte, en minuscules | `'a` |

Quelques habitudes en plus :

- une fonction qui renvoie un `bool` commence souvent par `est_`, `a_` ou `peut_` (en
  anglais : `is_`, `has_`, `can_`) : `est_vide`, `a_gagne` ;
- une conversion qui crée une nouvelle valeur s'appelle `to_…` (`to_string`), une
  qui consomme la valeur `into_…` (`into_iter`), une qui emprunte `as_…` (`as_str`) ;
- un constructeur s'appelle `new`.

```rust
const TAUX_TVA: f64 = 0.2;

struct Panier {
    total_ht: f64,
}

impl Panier {
    fn new() -> Panier {
        Panier { total_ht: 0.0 }
    }

    fn est_vide(&self) -> bool {
        self.total_ht == 0.0
    }

    fn total_ttc(&self) -> f64 {
        self.total_ht * (1.0 + TAUX_TVA)
    }
}

fn main() {
    let mut panier = Panier::new();
    println!("vide ? {}", panier.est_vide());
    panier.total_ht = 50.0;
    println!("TTC : {}", panier.total_ttc());
}
```

```sortie
vide ? true
TTC : 60
```

Le compilateur vérifie ces conventions : un nom mal écrit donne un avertissement (voir
[Erreurs fréquentes](#erreurs-frequentes)).

## La mise en forme : cargo fmt

Où mettre les espaces, les accolades, les retours à la ligne ? Inutile d'en débattre :
l'outil **rustfmt** décide pour tout le monde. Ce code marche, mais il est pénible à lire :

```rust norun
fn aire(largeur:f64,hauteur:f64)->f64{largeur*hauteur}
fn main(){
let pieces=vec![("salon",5.0,4.0),("chambre",3.5,3.0)];
    for (nom,l,h) in &pieces { println!("{nom} : {} m²",aire(*l,*h)); }
}
```

Une seule commande, dans le dossier du projet :

```sh
cargo fmt
```

Et le fichier devient :

```rust
fn aire(largeur: f64, hauteur: f64) -> f64 {
    largeur * hauteur
}
fn main() {
    let pieces = vec![("salon", 5.0, 4.0), ("chambre", 3.5, 3.0)];
    for (nom, l, h) in &pieces {
        println!("{nom} : {} m²", aire(*l, *h));
    }
}
```

```sortie
salon : 20 m²
chambre : 10.5 m²
```

Indentation de 4 espaces, une espace après les virgules et autour des opérateurs, une
instruction par ligne. Tous les exemples de ce site suivent ce style.

> **Astuce :** dans VS Code avec rust-analyzer, activez « Format On Save » : le fichier
> est mis en forme à chaque enregistrement.

## Le relecteur : cargo clippy

`cargo clippy` va plus loin que la forme : il repère les tournures maladroites et les
pièges (voir aussi [Déboguer en Rust](debogage.md#les-conseils-de-clippy)). Sur ce
programme, qui donne pourtant le bon résultat :

```rust
fn somme(nombres: &Vec<i32>) -> i32 {
    let mut total = 0;
    for i in 0..nombres.len() {
        total = total + nombres[i];
    }
    return total;
}

fn main() {
    let notes = vec![12, 15, 9];
    if notes.len() == 0 {
        println!("aucune note");
    }
    println!("{}", somme(&notes));
}
```

```sortie
36
```

clippy donne cinq conseils (extraits) :

```text
warning: unneeded `return` statement
 --> src/main.rs:6:5
  |
6 |     return total;
  |     ^^^^^^^^^^^^
help: remove `return`

warning: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
 --> src/main.rs:1:19
  |
1 | fn somme(nombres: &Vec<i32>) -> i32 {
  |                   ^^^^^^^^^
help: change this to
  |
1 + fn somme(nombres: &[i32]) -> i32 {

warning: the loop variable `i` is only used to index `nombres`
 --> src/main.rs:3:14
  |
3 |     for i in 0..nombres.len() {
  |              ^^^^^^^^^^^^^^^^
help: consider using an iterator

warning: manual implementation of an assign operation
 --> src/main.rs:4:9
  |
4 |         total = total + nombres[i];
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace it with: `total += nombres[i]`

warning: length comparison to zero
  --> src/main.rs:11:8
   |
11 |     if notes.len() == 0 {
   |        ^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `notes.is_empty()`
```

Une fois les conseils suivis, le code est deux fois plus court :

```rust
fn somme(nombres: &[i32]) -> i32 {
    nombres.iter().sum()
}

fn main() {
    let notes = vec![12, 15, 9];
    if notes.is_empty() {
        println!("aucune note");
    }
    println!("{}", somme(&notes));
}
```

```sortie
36
```

> **À retenir :** prenez l'habitude de lancer `cargo fmt` puis `cargo clippy` avant
> chaque commit. Ce sont vos deux relecteurs gratuits.

## Les itérateurs plutôt que les boucles à indices

Une boucle avec un compteur dit **comment** faire. Une chaîne d'itérateurs dit **ce
que** l'on veut : « les notes au-dessus de 10, doublées ». Avant :

```rust
let notes = vec![8, 12, 15, 9, 18];
let mut bonnes = Vec::new();
for i in 0..notes.len() {
    if notes[i] >= 10 {
        bonnes.push(notes[i] * 2);
    }
}
println!("{bonnes:?}");
```

```sortie
[24, 30, 36]
```

Après :

```rust
let notes = vec![8, 12, 15, 9, 18];
let bonnes: Vec<i32> = notes.iter().filter(|&&n| n >= 10).map(|n| n * 2).collect();
println!("{bonnes:?}");
```

```sortie
[24, 30, 36]
```

Plus de variable `mut`, plus d'indice qui peut déborder. Voir
[Les itérateurs](iterateurs.md).

## Propager les erreurs avec ?

Un `match` sur chaque `Result` alourdit vite le code. L'opérateur `?` fait la même
chose en un caractère (voir [Propager avec ?](erreurs.md#propager-avec)). Avant :

```rust
use std::num::ParseIntError;

fn additionner(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = match a.parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let y = match b.parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    Ok(x + y)
}

fn main() {
    println!("{:?}", additionner("2", "40"));
    println!("{:?}", additionner("2", "dix"));
}
```

```sortie
Ok(42)
Err(ParseIntError { kind: InvalidDigit })
```

Après :

```rust
use std::num::ParseIntError;

fn additionner(a: &str, b: &str) -> Result<i32, ParseIntError> {
    Ok(a.parse::<i32>()? + b.parse::<i32>()?)
}

fn main() {
    println!("{:?}", additionner("2", "40"));
    println!("{:?}", additionner("2", "dix"));
}
```

```sortie
Ok(42)
Err(ParseIntError { kind: InvalidDigit })
```

## if let et les combinateurs d'Option

Un `match` qui ne s'intéresse qu'à **un** cas s'écrit plus simplement avec `if let`.
Avant :

```rust
let premier = vec!["Ada", "Alan"].first().copied();
match premier {
    Some(nom) => println!("Premier : {nom}"),
    None => {}
}
```

```sortie
Premier : Ada
```

Après :

```rust
let premier = vec!["Ada", "Alan"].first().copied();
if let Some(nom) = premier {
    println!("Premier : {nom}");
}
```

```sortie
Premier : Ada
```

Pour transformer une [Option](../debuter/glossaire.md#option) sans l'ouvrir, ses
méthodes (on dit des **combinateurs**) évitent aussi le `match` :

| Méthode | Effet |
|---------|-------|
| `map(f)` | applique `f` à la valeur s'il y en a une |
| `unwrap_or(v)` | la valeur, ou `v` si c'est `None` |
| `and_then(f)` | enchaîne avec une fonction qui renvoie elle-même une `Option` |
| `is_some()`, `is_none()` | répond par `true` ou `false` |

```rust
let mots = vec!["rouge", "vert"];

let longueur = mots.first().map(|m| m.len()).unwrap_or(0);
println!("{longueur}");

let initiale = mots.get(5).and_then(|m| m.chars().next());
println!("{initiale:?}");
```

```sortie
5
None
```

## Sortir tôt : les retours anticipés

Des `if` imbriqués les uns dans les autres forment une « pyramide » difficile à suivre.
Traitez d'abord les cas à écarter, avec `return`, puis écrivez le cas normal sans
indentation. Avant :

```rust
fn reduction(age: u32, etudiant: bool) -> u32 {
    if age < 120 {
        if age < 18 {
            50
        } else {
            if etudiant {
                30
            } else {
                0
            }
        }
    } else {
        0
    }
}

fn main() {
    println!("{} {} {}", reduction(12, false), reduction(20, true), reduction(40, false));
}
```

```sortie
50 30 0
```

Après :

```rust
fn reduction(age: u32, etudiant: bool) -> u32 {
    if age >= 120 {
        return 0; // âge invalide
    }
    if age < 18 {
        return 50;
    }
    if etudiant { 30 } else { 0 }
}

fn main() {
    println!("{} {} {}", reduction(12, false), reduction(20, true), reduction(40, false));
}
```

```sortie
50 30 0
```

Chaque cas se lit sur une ligne, dans l'ordre.

## Emprunter dans les paramètres

Une fonction qui ne fait que **lire** prend une référence, et la plus générale
possible :

| Plutôt que… | Écrivez… | Parce que… |
|-------------|----------|------------|
| `nom: String` | `nom: &str` | accepte une `String` empruntée **et** un texte `"…"` |
| `nom: &String` | `nom: &str` | même raison |
| `liste: Vec<T>` | `liste: &[T]` | ne prend pas possession de la liste |
| `liste: &Vec<T>` | `liste: &[T]` | accepte aussi un tableau ou un morceau de liste |

```rust
fn initiales(noms: &[&str]) -> String {
    noms.iter().filter_map(|n| n.chars().next()).collect()
}

fn main() {
    let equipe = vec!["Ada", "Brendan", "Grace"];
    println!("{}", initiales(&equipe));      // un Vec emprunté
    println!("{}", initiales(&equipe[1..])); // un morceau
    println!("{}", initiales(&["Linus"]));   // un tableau
}
```

```sortie
ABG
BG
L
```

Gardez `String` ou `Vec<T>` pour les fonctions qui doivent vraiment **garder** la
valeur (la ranger dans une structure, par exemple).

## De petites fonctions bien nommées

Une fonction fait **une** chose, et son nom dit laquelle. Si vous devez écrire un
commentaire pour expliquer un bloc de code, c'est souvent le signe qu'il mérite sa
propre fonction :

```rust
fn lire_notes(texte: &str) -> Vec<u32> {
    texte.split(',').filter_map(|n| n.trim().parse().ok()).collect()
}

fn moyenne(notes: &[u32]) -> Option<f64> {
    if notes.is_empty() {
        return None;
    }
    Some(notes.iter().sum::<u32>() as f64 / notes.len() as f64)
}

fn mention(moyenne: f64) -> &'static str {
    if moyenne >= 16.0 {
        "très bien"
    } else if moyenne >= 12.0 {
        "bien"
    } else {
        "passable"
    }
}

fn main() {
    let notes = lire_notes("14, 17, 12, abc");
    match moyenne(&notes) {
        Some(m) => println!("{m:.1} : {}", mention(m)),
        None => println!("aucune note"),
    }
}
```

```sortie
14.3 : bien
```

`main` se lit maintenant comme une phrase : lire les notes, calculer la moyenne,
donner la mention. Et chaque petite fonction se [teste](tests.md) à part.

## Documenter avec ///

Un commentaire `//` s'adresse à qui lit le code. Un commentaire `///`, placé juste
**au-dessus** d'une fonction, d'une structure ou d'un module, est sa
**documentation** : rust-analyzer l'affiche quand on survole le nom, et `cargo doc
--open` en fait un site web.

On y écrit, en Markdown : ce que fait la fonction, ses cas particuliers, et un exemple.

```rust
/// Calcule la moyenne d'une liste de notes.
///
/// Renvoie `None` si la liste est vide.
///
/// # Exemple
///
/// ```
/// assert_eq!(moyenne(&[12, 15]), Some(13.5));
/// ```
fn moyenne(notes: &[u32]) -> Option<f64> {
    if notes.is_empty() {
        return None;
    }
    Some(notes.iter().sum::<u32>() as f64 / notes.len() as f64)
}

fn main() {
    println!("{:?}", moyenne(&[12, 15]));
}
```

```sortie
Some(13.5)
```

Dans une bibliothèque, `cargo test` exécute aussi les exemples de la documentation
(voir [Les tests dans la documentation](tests.md#les-tests-dans-la-documentation)).
`//!`, en haut d'un fichier, documente le module entier.

## Erreurs fréquentes

### Ne pas suivre les conventions de nommage

```rust norun
const tauxTva: f64 = 0.2;

struct panier {
    Total: f64,
}

fn CalculerPrix(prix: f64) -> f64 {
    prix * (1.0 + tauxTva)
}
```

```text
warning: type `panier` should have an upper camel case name
 --> src/main.rs:3:8
  |
3 | struct panier {
  |        ^^^^^^ help: convert the identifier to upper camel case: `Panier`

warning: constant `tauxTva` should have an upper case name
 --> src/main.rs:1:7
  |
1 | const tauxTva: f64 = 0.2;
  |       ^^^^^^^

warning: structure field `Total` should have a snake case name
 --> src/main.rs:4:5
  |
4 |     Total: f64,
  |     ^^^^^ help: convert the identifier to snake case: `total`

warning: function `CalculerPrix` should have a snake case name
 --> src/main.rs:7:4
  |
7 | fn CalculerPrix(prix: f64) -> f64 {
  |    ^^^^^^^^^^^^ help: convert the identifier to snake case: `calculer_prix`
```

Ce ne sont que des avertissements, mais ne les ignorez pas : renommez en `TAUX_TVA`,
`Panier`, `total` et `calculer_prix`. rust-analyzer sait renommer partout d'un coup
(touche F2).

### Un paramètre &String

```rust norun
fn saluer(nom: &String) {
    println!("Bonjour {nom}");
}

fn main() {
    saluer("Ada");
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:6:12
  |
6 |     saluer("Ada");
  |     ------ ^^^^^ expected `&String`, found `&str`
  |     |
  |     arguments to this function are incorrect
```

Un texte entre guillemets est un `&str`, pas une `String`. Écrivez
`fn saluer(nom: &str)` : la fonction accepte alors les deux.

### Un commentaire /// mal placé

```rust norun
fn main() {
    let x = 5;
    println!("{x}");
    /// Affiche x.
}
```

```text
error[E0585]: found a documentation comment that doesn't document anything
 --> src/main.rs:4:5
  |
4 |     /// Affiche x.
  |     ^^^^^^^^^^^^^^
  |
  = help: doc comments must come before what they document, if a comment was intended use `//`
```

`///` doit être juste **au-dessus** de ce qu'il documente. Pour un simple commentaire
dans le code, utilisez `//`.

## Testez-vous

1. Comment s'écrivent une fonction, une structure et une constante en Rust ?

> **Réponse**
>
> Une fonction en `snake_case` (`calculer_total`), une structure en `UpperCamelCase`
> (`Panier`), une constante en `SCREAMING_SNAKE_CASE` (`TAUX_TVA`).

2. Quelle est la différence entre `cargo fmt` et `cargo clippy` ?

> **Réponse**
>
> `cargo fmt` met le code en forme (espaces, retours à la ligne) sans rien changer à ce
> qu'il fait. `cargo clippy` le relit et conseille de meilleures tournures, ou signale
> des bugs probables.

3. Pourquoi préférer `fn f(liste: &[i32])` à `fn f(liste: Vec<i32>)` ?

> **Réponse**
>
> La fonction emprunte la liste au lieu d'en prendre possession : l'appelant la garde,
> sans clone. Et elle accepte aussi un tableau ou un morceau de liste.

## En résumé

- `snake_case` pour les fonctions et variables, `UpperCamelCase` pour les types,
  `SCREAMING_SNAKE_CASE` pour les constantes.
- `cargo fmt` met en forme, `cargo clippy` relit : lancez-les avant chaque commit.
- Itérateurs, `?`, `if let`, `map` et `unwrap_or` remplacent les boucles à indices et
  les `match` verbeux.
- Sortez tôt avec `return`, empruntez dans les paramètres (`&str`, `&[T]`), et écrivez
  de petites fonctions bien nommées.
- Documentez avec `///`, avec un exemple.

Suite : [Organiser un projet avec Cargo](projet.md)
