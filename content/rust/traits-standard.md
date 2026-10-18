# Les traits de la bibliothèque standard

La bibliothèque standard de Rust définit une poignée de traits que l'on retrouve partout : ce chapitre montre à quoi sert chacun, et quand l'obtenir avec `derive` ou l'écrire à la main.

> **Objectifs :**
>
> - afficher vos types avec `Debug` et `Display` ;
> - copier, comparer et trier vos structures avec `Clone`, `Copy`, `PartialEq` et `Ord` ;
> - utiliser une structure comme clé de `HashMap` avec `Hash` ;
> - convertir avec `From` et `Into`, et créer une valeur par défaut avec `Default` ;
> - écrire un itérateur et surcharger l'opérateur `+`.

Un [trait](../debuter/glossaire.md#trait) est une liste de capacités qu'un type promet
d'avoir, comme un permis : le permis de conduire dit « cette personne sait conduire ».
Le trait `Display` dit « ce type sait s'afficher ». Vous avez vu comment définir vos
propres traits dans [Génériques et traits](generiques.md) ; ici, on regarde ceux que
Rust fournit.

## derive : des traits offerts

Beaucoup de traits standards peuvent être **dérivés** : vous écrivez
`#[derive(…)]` au-dessus du type, et le compilateur écrit le code pour vous.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a.clone();
    println!("{:?}", a);
    println!("{}", a == b);
}
```

```sortie
Point { x: 1, y: 2 }
true
```

`derive` ne marche que si **tous les champs** ont déjà ce trait. Un `i32` sait se
cloner et se comparer : `Point` aussi, donc.

## Debug et Display : s'afficher

Il y a deux façons d'afficher une valeur :

| Trait | Format | Pour qui ? | Comment l'obtenir |
|-------|--------|------------|-------------------|
| `Debug` | `{:?}` ou `{:#?}` | le programmeur | `#[derive(Debug)]` |
| `Display` | `{}` | l'utilisateur | à écrire à la main |

`Display` ne peut pas être dérivé : Rust ne peut pas deviner comment vous voulez
présenter votre type. On l'écrit avec `impl` et une méthode `fmt` :

```rust
use std::fmt;

#[derive(Debug)]
struct Temperature {
    degres: f64,
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} °C", self.degres)
    }
}

fn main() {
    let t = Temperature { degres: 21.456 };
    println!("{}", t);
    println!("{:?}", t);
    let texte = t.to_string();
    println!("{}", texte.len());
}
```

```sortie
21.5 °C
Temperature { degres: 21.456 }
8
```

`write!` fonctionne comme `format!`, mais écrit dans `f`. En bonus, tout type qui a
`Display` reçoit la méthode `to_string()`.

> **Astuce :** `{:#?}` affiche une structure sur plusieurs lignes, indentée. C'est
> pratique pour les grosses structures.

## Clone et Copy : dupliquer

- `Clone` donne la méthode `.clone()` : une copie **explicite**, peut-être coûteuse
  (une `String` copie tout son texte).
- `Copy` rend la copie **automatique** : `let b = a;` copie au lieu de
  [déplacer](possession.md#le-deplacement). Réservé aux petits types simples.

```rust
#[derive(Debug, Clone, Copy)]
struct Couleur {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let rouge = Couleur { r: 255, g: 0, b: 0 };
    let autre = rouge; // copiée, pas déplacée
    println!("{:?}", rouge);
    println!("{:?}", autre);
}
```

```sortie
Couleur { r: 255, g: 0, b: 0 }
Couleur { r: 255, g: 0, b: 0 }
```

`Copy` exige aussi `Clone`. Une structure qui contient une `String` ou un `Vec` ne
peut pas être `Copy` : elle n'aura que `Clone`.

## PartialEq et Eq : comparer avec ==

`PartialEq` donne les opérateurs `==` et `!=`. Le `derive` compare **tous les champs**
un par un.

`Eq` est une promesse en plus : « toute valeur est égale à elle-même ». C'est vrai pour
les entiers et les textes, mais pas pour les flottants : `f64::NAN == f64::NAN` est
faux. Un type avec un champ `f64` ne peut donc avoir que `PartialEq`.

```rust
#[derive(Debug, PartialEq, Eq)]
struct Carte {
    valeur: u8,
    couleur: char,
}

fn main() {
    let a = Carte { valeur: 10, couleur: '♥' };
    let b = Carte { valeur: 10, couleur: '♠' };
    println!("{}", a == b);
    println!("{}", a != b);
    println!("{}", f64::NAN == f64::NAN);
}
```

```sortie
false
true
false
```

## PartialOrd et Ord : trier

`PartialOrd` donne `<`, `>`, `<=`, `>=`. `Ord` donne un ordre **total**, nécessaire
pour `sort()`, `max()` ou une `BTreeMap`. Il exige `Eq` et `PartialOrd`.

Le `derive` compare les champs **dans l'ordre où ils sont écrits**, comme un
dictionnaire compare la première lettre, puis la deuxième :

```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Eleve {
    note: u32,
    nom: String,
}

fn main() {
    let mut classe = vec![
        Eleve { note: 15, nom: String::from("Zoé") },
        Eleve { note: 12, nom: String::from("Léo") },
        Eleve { note: 15, nom: String::from("Ali") },
    ];
    classe.sort();
    for e in &classe {
        println!("{} {}", e.note, e.nom);
    }
}
```

```sortie
12 Léo
15 Ali
15 Zoé
```

Les élèves sont triés par note, puis par nom en cas d'égalité, car `note` est écrit
en premier.

### Trier autrement, sans toucher au type

Souvent, l'ordre dépend de la situation. `sort_by_key` et `sort_by` choisissent le
critère au moment du tri, sans aucun `derive` :

```rust
struct Eleve {
    nom: String,
    note: u32,
}

fn main() {
    let mut classe = vec![
        Eleve { nom: String::from("Zoé"), note: 15 },
        Eleve { nom: String::from("Léo"), note: 12 },
        Eleve { nom: String::from("Ali"), note: 18 },
    ];
    classe.sort_by_key(|e| e.note);
    println!("{}", classe[0].nom);

    classe.sort_by(|a, b| b.note.cmp(&a.note)); // décroissant
    println!("{}", classe[0].nom);

    classe.sort_by(|a, b| a.nom.cmp(&b.nom));
    println!("{}", classe[0].nom);
}
```

```sortie
Léo
Ali
Ali
```

`cmp` compare deux valeurs et renvoie un `Ordering` : `Less`, `Equal` ou `Greater`.
Inverser `a` et `b` inverse l'ordre.

## Hash : pour servir de clé

Une clé de `HashMap` ou un élément de `HashSet` doit avoir `Hash` (savoir se résumer en
un nombre) et `Eq`. On les dérive ensemble (voir [HashMap](hashmap.md)) :

```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Case {
    ligne: u8,
    colonne: u8,
}

fn main() {
    let mut plateau = HashMap::new();
    plateau.insert(Case { ligne: 0, colonne: 0 }, "tour");
    plateau.insert(Case { ligne: 0, colonne: 4 }, "roi");
    println!("{}", plateau[&Case { ligne: 0, colonne: 4 }]);
}
```

```sortie
roi
```

## Default : une valeur de départ

`Default` donne une valeur « vide » : `0` pour les nombres, `""` pour les textes,
`false` pour les booléens. Avec `..Default::default()`, vous ne remplissez que les
champs qui vous intéressent :

```rust
#[derive(Debug, Default)]
struct Reglages {
    volume: u8,
    plein_ecran: bool,
    pseudo: String,
}

fn main() {
    let r = Reglages::default();
    println!("{:?}", r);
    let r2 = Reglages { volume: 7, ..Default::default() };
    println!("{:?}", r2);
}
```

```sortie
Reglages { volume: 0, plein_ecran: false, pseudo: "" }
Reglages { volume: 7, plein_ecran: false, pseudo: "" }
```

Pour d'autres valeurs de départ, écrivez `impl Default` à la main :

```rust
struct Partie {
    vies: u32,
    niveau: u32,
}

impl Default for Partie {
    fn default() -> Self {
        Partie { vies: 3, niveau: 1 }
    }
}

fn main() {
    let p = Partie::default();
    println!("{} vies, niveau {}", p.vies, p.niveau);
}
```

```sortie
3 vies, niveau 1
```

## From et Into : convertir

`From` décrit une conversion d'un type vers le vôtre. En l'écrivant, vous recevez
`Into` **gratuitement**, dans l'autre sens :

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

fn main() {
    let f = Fahrenheit::from(Celsius(100.0));
    println!("{}", f.0);

    let g: Fahrenheit = Celsius(0.0).into();
    println!("{}", g.0);
}
```

```sortie
212
32
```

Vous connaissez déjà `String::from("…")` : c'est le même trait. `From` sert aussi
beaucoup pour les erreurs, avec `?` (voir [Gérer les erreurs](erreurs.md)).

> **À retenir :** implémentez `From`, jamais `Into` : `Into` vient tout seul.

## Iterator : un itérateur fait main

Un type devient un [itérateur](../debuter/glossaire.md#iterateur) en implémentant une
seule méthode : `next`, qui renvoie `Some(valeur)` ou `None` quand c'est fini. Toutes
les autres méthodes (`map`, `filter`, `sum`…) viennent avec.

```rust
struct CompteARebours {
    reste: u32,
}

impl Iterator for CompteARebours {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.reste == 0 {
            None
        } else {
            self.reste -= 1;
            Some(self.reste + 1)
        }
    }
}

fn main() {
    for n in (CompteARebours { reste: 3 }) {
        println!("{n}…");
    }
    let total: u32 = CompteARebours { reste: 4 }.sum();
    println!("total : {total}");
}
```

```sortie
3…
2…
1…
total : 10
```

`type Item = u32;` dit quel type de valeur l'itérateur produit. Le chapitre
[Itérateurs](iterateurs.md#ecrire-son-propre-iterateur) va plus loin.

## AsRef<str> : accepter plusieurs sortes de texte

`AsRef<str>` veut dire « je peux être vu comme un `&str` ». Une fonction qui l'accepte
prend aussi bien un `&str` qu'une `String` :

```rust
fn saluer<S: AsRef<str>>(nom: S) {
    println!("Bonjour {} !", nom.as_ref());
}

fn main() {
    saluer("Zoé");
    saluer(String::from("Léo"));
}
```

```sortie
Bonjour Zoé !
Bonjour Léo !
```

> **Note :** pour débuter, un paramètre `&str` suffit presque toujours. Vous croiserez
> surtout `AsRef` dans la bibliothèque standard, par exemple
> `AsRef<Path>` pour les [fichiers](fichiers.md).

## Add : surcharger l'opérateur +

Les opérateurs sont aussi des traits, dans `std::ops`. Implémentez `Add` et votre type
s'additionne avec `+` :

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vecteur {
    x: i32,
    y: i32,
}

impl Add for Vecteur {
    type Output = Vecteur;

    fn add(self, autre: Vecteur) -> Vecteur {
        Vecteur { x: self.x + autre.x, y: self.y + autre.y }
    }
}

fn main() {
    let a = Vecteur { x: 1, y: 2 };
    let b = Vecteur { x: 10, y: 20 };
    println!("{:?}", a + b);
    println!("{}", a + b == Vecteur { x: 11, y: 22 });
}
```

```sortie
Vecteur { x: 11, y: 22 }
true
```

`Sub` (`-`), `Mul` (`*`) et `Neg` (le `-` devant) suivent le même modèle.

## derive ou impl à la main ?

| Trait | Sert à | `derive` ? | À la main quand… |
|-------|--------|------------|------------------|
| `Debug` | `{:?}` | oui | rarement |
| `Display` | `{}`, `to_string()` | **non** | toujours |
| `Clone` | `.clone()` | oui | rarement |
| `Copy` | copie automatique | oui | jamais |
| `PartialEq`, `Eq` | `==`, `!=` | oui | certains champs ne comptent pas |
| `PartialOrd`, `Ord` | `<`, `sort()` | oui | l'ordre n'est pas celui des champs |
| `Hash` | clé de `HashMap` | oui | rarement |
| `Default` | `::default()` | oui | les valeurs de départ ne sont pas 0 ou vide |
| `From` | conversions | **non** | toujours |
| `Iterator` | `for`, `map`, `sum`… | **non** | toujours |
| `Add` | `+` | **non** | toujours |

> **Astuce :** une bonne habitude pour vos structures de données simples :
> `#[derive(Debug, Clone, PartialEq)]`. Ajoutez le reste quand le compilateur le
> demande.

## Erreurs fréquentes

### Trier une structure sans Ord

```rust norun
#[derive(Debug)]
struct Eleve {
    nom: String,
    note: u32,
}

fn main() {
    let mut classe = vec![
        Eleve { nom: String::from("Zoé"), note: 15 },
        Eleve { nom: String::from("Léo"), note: 12 },
    ];
    classe.sort();
    println!("{:?}", classe);
}
```

```text
error[E0277]: the trait bound `Eleve: Ord` is not satisfied
  --> src/main.rs:12:12
   |
12 |     classe.sort();
   |            ^^^^ the trait `Ord` is not implemented for `Eleve`
   |
help: consider annotating `Eleve` with `#[derive(Ord)]`
   |
 2 + #[derive(Ord)]
 3 | struct Eleve {
   |
```

Rust ne sait pas comment ranger deux élèves. Dérivez
`PartialEq, Eq, PartialOrd, Ord`, ou plus simplement, choisissez le critère :
`classe.sort_by_key(|e| e.note);`.

### Copy sur une structure qui contient une String

```rust norun
#[derive(Debug, Clone, Copy)]
struct Joueur {
    nom: String,
    points: u32,
}

fn main() {
    let j = Joueur { nom: String::from("Zoé"), points: 3 };
    let k = j;
    println!("{:?} {:?}", j, k);
}
```

```text
error[E0204]: the trait `Copy` cannot be implemented for this type
 --> src/main.rs:2:8
  |
1 | #[derive(Debug, Clone, Copy)]
  |                        ---- in this derive macro expansion
2 | struct Joueur {
  |        ^^^^^^
3 |     nom: String,
  |     ----------- this field does not implement `Copy`
```

Une `String` ne peut pas être copiée automatiquement. Retirez `Copy` et écrivez
`let k = j.clone();`.

### Eq sur une structure avec un f64

```rust norun
#[derive(Debug, PartialEq, Eq)]
struct Produit {
    nom: String,
    prix: f64,
}

fn main() {
    let p = Produit { nom: String::from("pain"), prix: 1.2 };
    println!("{:?}", p);
}
```

```text
error[E0277]: the trait bound `f64: Eq` is not satisfied
 --> src/main.rs:4:5
  |
1 | #[derive(Debug, PartialEq, Eq)]
  |                            -- in this derive macro expansion
...
4 |     prix: f64,
  |     ^^^^^^^^^ the trait `Eq` is not implemented for `f64`
```

Les flottants n'ont que `PartialEq`, à cause de `NAN`. Retirez `Eq`, ou rangez le prix
en centimes dans un `u32`.

## Testez-vous

**1.** Quelle différence entre `Debug` et `Display` ?

> **Réponse**
>
> `Debug` (`{:?}`) est un affichage technique pour le programmeur, et se dérive.
> `Display` (`{}`) est l'affichage pour l'utilisateur, et s'écrit à la main.

**2.** Vous implémentez `From<Celsius> for Fahrenheit`. Pouvez-vous écrire
`let f: Fahrenheit = c.into();` ?

> **Réponse**
>
> Oui : implémenter `From` donne `Into` automatiquement dans l'autre sens.

**3.** Écrivez `Display` pour une structure `Fraction { haut: i32, bas: i32 }`, qui
s'affiche `3/4`.

> **Réponse**
>
> ```rust
> use std::fmt;
>
> struct Fraction {
>     haut: i32,
>     bas: i32,
> }
>
> impl fmt::Display for Fraction {
>     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
>         write!(f, "{}/{}", self.haut, self.bas)
>     }
> }
>
> fn main() {
>     println!("{}", Fraction { haut: 3, bas: 4 });
> }
> ```
>
> ```sortie
> 3/4
> ```

## En résumé

- `#[derive(…)]` écrit pour vous `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`,
  `PartialOrd`, `Ord`, `Hash` et `Default`.
- `Display` s'écrit à la main et donne `{}` et `to_string()`.
- `Ord` permet `sort()` ; `sort_by_key` trie selon un critère sans rien dériver.
- Une clé de `HashMap` a besoin de `Eq` et `Hash`.
- Implémentez `From` et vous recevez `Into` ; implémentez `next` et vous recevez tout
  `Iterator`.
- `Add` et ses cousins de `std::ops` donnent un sens à `+`, `-` et `*`.

Suite : [Gérer les erreurs](erreurs.md).
