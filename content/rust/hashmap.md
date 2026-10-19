# HashMap, HashSet et BTreeMap

Ce chapitre présente les collections qui rangent des valeurs par clé ou sans doublons : `HashMap`, `BTreeMap`, `HashSet`, ainsi que la file `VecDeque`.

> **Objectifs :**
>
> - ajouter, lire et retirer des paires clé-valeur dans une `HashMap` ;
> - compter des éléments avec `entry().or_insert()` ;
> - afficher une table dans un ordre stable, avec un tri ou une `BTreeMap` ;
> - comparer deux ensembles avec `HashSet` ;
> - choisir la bonne collection pour chaque problème.

Toutes ces collections se trouvent dans `std::collections`. Contrairement à `Vec`, il
faut les importer avec `use`.

## Qu'est-ce qu'une HashMap ?

Une `HashMap` associe des **clés** à des **valeurs**, comme un annuaire associe un nom
à un numéro de téléphone. On l'appelle aussi un
[dictionnaire](../debuter/glossaire.md#dictionnaire). Vous ne cherchez pas « la
troisième ligne » : vous cherchez directement « Zoé ».

```rust
use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::new();
    ages.insert("Zoé", 12);
    ages.insert("Léo", 9);
    println!("{}", ages.len());
    println!("{}", ages["Zoé"]);
}
```

```sortie
2
12
```

Toutes les clés ont le même type (ici `&str`), et toutes les valeurs aussi (ici `i32`).
Une clé n'apparaît qu'une fois : insérer une clé existante **remplace** sa valeur.

```rust
use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::new();
    ages.insert("Zoé", 12);
    let ancien = ages.insert("Zoé", 13);
    println!("{:?} {}", ancien, ages["Zoé"]);
}
```

```sortie
Some(12) 13
```

`insert` renvoie l'ancienne valeur dans une [Option](types.md#le-type-option) : `None`
si la clé était nouvelle.

## Lire, tester et retirer

`ages["Zoé"]` arrête le programme si la clé n'existe pas. Préférez `get`, qui renvoie
une `Option` :

```rust
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("pommes", 3);
    stock.insert("poires", 0);

    println!("{:?}", stock.get("pommes"));
    println!("{:?}", stock.get("kiwis"));
    println!("{}", stock.contains_key("poires"));

    let retire = stock.remove("poires");
    println!("{:?} {}", retire, stock.len());

    match stock.get("kiwis") {
        Some(n) => println!("{n} kiwis"),
        None => println!("pas de kiwis"),
    }
    let n = stock.get("kiwis").copied().unwrap_or(0);
    println!("{n}");
}
```

```sortie
Some(3)
None
true
Some(0) 1
pas de kiwis
0
```

> **Note :** `get` renvoie une **référence** (`Some(&3)`, affichée `Some(3)`).
> `.copied()` en fait une copie, et `unwrap_or(0)` donne une valeur par défaut.

Pour modifier une valeur en place, `get_mut` renvoie une référence `&mut` :

```rust
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("pommes", 3);
    if let Some(n) = stock.get_mut("pommes") {
        *n += 10;
    }
    println!("{}", stock["pommes"]);
}
```

```sortie
13
```

## Compter avec entry

`entry(clé)` donne accès à la case d'une clé, qu'elle existe ou non. `or_insert(0)`
la crée avec `0` si elle manque, puis renvoie une référence vers la valeur. C'est la
façon idéale de **compter** :

```rust
use std::collections::HashMap;

fn main() {
    let texte = "le chat et le chien et le lapin";
    let mut compte = HashMap::new();
    for mot in texte.split_whitespace() {
        *compte.entry(mot).or_insert(0) += 1;
    }
    println!("le : {}", compte["le"]);
    println!("et : {}", compte["et"]);
    println!("chat : {}", compte["chat"]);
}
```

```sortie
le : 3
et : 2
chat : 1
```

Lisez la ligne importante ainsi : « prends la case de `mot`, mets-y 0 si elle est vide,
puis ajoute 1 ». L'étoile `*` modifie la valeur pointée par la référence.

Le même outil sert à **regrouper** : ici, une liste de prénoms par initiale.

```rust
use std::collections::HashMap;

fn main() {
    let prenoms = ["Ali", "Bea", "Anna", "Bob", "Chloé"];
    let mut groupes: HashMap<char, Vec<&str>> = HashMap::new();
    for p in prenoms {
        let initiale = p.chars().next().unwrap();
        groupes.entry(initiale).or_insert_with(Vec::new).push(p);
    }
    println!("{:?}", groupes[&'A']);
    println!("{:?}", groupes[&'B']);
}
```

```sortie
["Ali", "Anna"]
["Bea", "Bob"]
```

## Parcourir une HashMap

Une boucle `for` sur `&table` donne des paires `(clé, valeur)` :

```rust
use std::collections::HashMap;

fn main() {
    let mut notes = HashMap::new();
    notes.insert("maths", 15);
    notes.insert("français", 12);
    notes.insert("anglais", 17);

    let mut total = 0;
    for (_matiere, note) in &notes {
        total += note;
    }
    println!("total : {total}");
}
```

```sortie
total : 44
```

> **Attention :** une `HashMap` ne garde **aucun ordre**. Deux exécutions du même
> programme peuvent afficher les paires dans un ordre différent. C'est voulu : cela
> protège contre certaines attaques.

Pour un affichage stable, rassemblez les paires dans un `Vec`, puis triez-le :

```rust
use std::collections::HashMap;

fn main() {
    let mut notes = HashMap::new();
    notes.insert("maths", 15);
    notes.insert("français", 12);
    notes.insert("anglais", 17);

    let mut paires: Vec<_> = notes.iter().collect();
    paires.sort();
    for (matiere, note) in paires {
        println!("{matiere} : {note}");
    }
}
```

```sortie
anglais : 17
français : 12
maths : 15
```

`keys()` et `values()` parcourent seulement les clés ou seulement les valeurs.

## BTreeMap : des clés toujours triées

Une `BTreeMap` s'utilise exactement comme une `HashMap`, mais elle garde ses clés
**triées**. Plus besoin de trier avant d'afficher :

```rust
use std::collections::BTreeMap;

fn main() {
    let mut scores = BTreeMap::new();
    scores.insert("Zoé", 30);
    scores.insert("Ali", 25);
    scores.insert("Léo", 40);
    for (nom, score) in &scores {
        println!("{nom} : {score}");
    }
    println!("{:?}", scores.first_key_value());
}
```

```sortie
Ali : 25
Léo : 40
Zoé : 30
Some(("Ali", 25))
```

> **Astuce :** une `HashMap` est un peu plus rapide ; une `BTreeMap` donne un résultat
> toujours identique. Pour débuter, et dès que vous affichez une table, la `BTreeMap`
> est souvent le choix le plus simple.

## HashSet : un ensemble sans doublons

Un `HashSet` est un sac où chaque valeur n'apparaît **qu'une fois**. Il n'y a pas de
valeur associée : seulement des éléments. `insert` renvoie `false` si l'élément était
déjà là.

```rust
use std::collections::HashSet;

fn main() {
    let mut vus = HashSet::new();
    println!("{}", vus.insert("rouge"));
    println!("{}", vus.insert("bleu"));
    println!("{}", vus.insert("rouge"));
    println!("{}", vus.len());
    println!("{}", vus.contains("bleu"));
}
```

```sortie
true
true
false
2
true
```

### Retirer les doublons d'une liste

```rust
use std::collections::HashSet;

fn main() {
    let villes = vec!["Lyon", "Paris", "Lyon", "Nice", "Paris"];
    let uniques: HashSet<&str> = villes.into_iter().collect();
    let mut liste: Vec<_> = uniques.into_iter().collect();
    liste.sort();
    println!("{:?}", liste);
}
```

```sortie
["Lyon", "Nice", "Paris"]
```

### Union, intersection et différence

Deux ensembles se comparent comme en mathématiques. Imaginez deux clubs et leurs
membres :

| Opération | Sens | Méthode |
|-----------|------|---------|
| union | membres d'au moins un club | `a.union(&b)` |
| intersection | membres des deux clubs | `a.intersection(&b)` |
| différence | membres de `a` mais pas de `b` | `a.difference(&b)` |

```rust
use std::collections::{BTreeSet, HashSet};

fn main() {
    let foot: HashSet<&str> = ["Ali", "Bea", "Chloé"].into_iter().collect();
    let echecs: HashSet<&str> = ["Bea", "Dan"].into_iter().collect();

    // On range chaque résultat dans un BTreeSet pour l'afficher trié.
    let union: BTreeSet<_> = foot.union(&echecs).collect();
    let commun: BTreeSet<_> = foot.intersection(&echecs).collect();
    let seulement_foot: BTreeSet<_> = foot.difference(&echecs).collect();
    println!("{:?}", union);
    println!("{:?}", commun);
    println!("{:?}", seulement_foot);
}
```

```sortie
{"Ali", "Bea", "Chloé", "Dan"}
{"Bea"}
{"Ali", "Chloé"}
```

Un `BTreeSet` est l'ensemble trié : il est à `HashSet` ce que `BTreeMap` est à
`HashMap`. Les accolades `{ }` de l'affichage rappellent qu'il s'agit d'un ensemble.

## VecDeque : une file d'attente

Un `Vec` ajoute et retire vite **à la fin**. Retirer **au début** est lent : tous les
éléments doivent se décaler. Une `VecDeque` (prononcez « vec-dèque ») est rapide aux
deux bouts. C'est la file d'attente à la boulangerie : on arrive derrière, on est
servi devant.

```rust
use std::collections::VecDeque;

fn main() {
    let mut file = VecDeque::new();
    file.push_back("Ali");
    file.push_back("Bea");
    file.push_back("Dan");
    file.push_front("Mamie"); // prioritaire

    while let Some(client) = file.pop_front() {
        println!("Je sers {client}");
    }
    println!("{}", file.is_empty());
}
```

```sortie
Je sers Mamie
Je sers Ali
Je sers Bea
Je sers Dan
true
```

## Quelle collection choisir ?

| Besoin | Collection |
|--------|------------|
| une liste ordonnée, accès par position | `Vec<T>` |
| ajouter ou retirer aux deux bouts (file) | `VecDeque<T>` |
| retrouver une valeur par sa clé, vite | `HashMap<K, V>` |
| la même chose, avec les clés triées | `BTreeMap<K, V>` |
| des valeurs uniques, tester l'appartenance | `HashSet<T>` |
| des valeurs uniques et triées | `BTreeSet<T>` |

> **À retenir :** commencez par `Vec`. Passez à une table quand vous cherchez « par
> nom », et à un ensemble quand les doublons vous gênent.

## Erreurs fréquentes

### Prendre le résultat de get pour la valeur

```rust norun
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("pommes", 3);
    let n: i32 = stock.get("pommes");
    println!("{n}");
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:6:18
  |
6 |     let n: i32 = stock.get("pommes");
  |            ---   ^^^^^^^^^^^^^^^^^^^ expected `i32`, found `Option<&{integer}>`
  |            |
  |            expected due to this
  |
  = note: expected type `i32`
             found enum `Option<&{integer}>`
```

`get` renvoie une `Option` : la clé peut manquer. Écrivez
`stock.get("pommes").copied().unwrap_or(0)`, ou traitez les deux cas avec `match`.

### Oublier mut

```rust norun
use std::collections::HashMap;

fn main() {
    let stock = HashMap::new();
    stock.insert("pommes", 3);
    println!("{:?}", stock);
}
```

```text
error[E0596]: cannot borrow `stock` as mutable, as it is not declared as mutable
 --> src/main.rs:5:5
  |
5 |     stock.insert("pommes", 3);
  |     ^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
4 |     let mut stock = HashMap::new();
  |         +++
```

Ajouter une paire modifie la table : déclarez-la avec `let mut`.

### Une structure comme clé, sans Hash ni Eq

```rust norun
use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut noms = HashMap::new();
    noms.insert(Point { x: 0, y: 0 }, "origine");
    println!("{:?}", noms);
}
```

```text
error[E0277]: the trait bound `Point: Eq` is not satisfied
  --> src/main.rs:11:10
   |
11 |     noms.insert(Point { x: 0, y: 0 }, "origine");
   |          ^^^^^^ the trait `Eq` is not implemented for `Point`
   |
help: consider annotating `Point` with `#[derive(Eq)]`
   |
 4 + #[derive(Eq)]
 5 | struct Point {
   |
```

Une clé doit pouvoir être comparée (`Eq`) et « hachée » (`Hash`). Ajoutez
`#[derive(Debug, PartialEq, Eq, Hash)]` au-dessus de la structure. Voir
[Les traits de la bibliothèque standard](traits-standard.md#hash-pour-servir-de-cle).

## Testez-vous

**1.** Pourquoi deux exécutions du même programme peuvent-elles afficher une
`HashMap` dans un ordre différent ? Comment obtenir un ordre stable ?

> **Réponse**
>
> Une `HashMap` ne garde aucun ordre. Triez les paires dans un `Vec` avant de les
> afficher, ou utilisez une `BTreeMap`.

**2.** Quelle ligne ajoute 1 au compteur de `mot`, en le créant à 0 s'il n'existe
pas ?

> **Réponse**
>
> `*compte.entry(mot).or_insert(0) += 1;`

**3.** Quelles lettres sont communes à « chat » et « tache » ? Répondez avec deux
`HashSet` et triez le résultat.

> **Réponse**
>
> ```rust
> use std::collections::HashSet;
>
> fn main() {
>     let a: HashSet<char> = "chat".chars().collect();
>     let b: HashSet<char> = "tache".chars().collect();
>     let mut communes: Vec<char> = a.intersection(&b).copied().collect();
>     communes.sort();
>     println!("{:?}", communes);
> }
> ```
>
> ```sortie
> ['a', 'c', 'h', 't']
> ```

## En résumé

- `HashMap` associe des clés à des valeurs : `insert`, `get`, `remove`, `contains_key`.
- `*table.entry(clé).or_insert(0) += 1` compte en une ligne.
- Une `HashMap` n'a pas d'ordre : triez avant d'afficher, ou prenez une `BTreeMap`.
- `HashSet` garde des valeurs uniques et offre `union`, `intersection`, `difference`.
- `VecDeque` est une file : `push_back` derrière, `pop_front` devant.

Suite : [Les pièges du débutant en Rust](pieges.md)
