# Les itérateurs

Un itérateur donne les éléments d'une collection un par un : avec lui, vous transformez, filtrez et résumez une liste en une seule ligne claire.

> **Objectifs :**
>
> - parcourir une collection avec `iter`, `iter_mut` et `into_iter` ;
> - transformer et filtrer avec `map`, `filter`, `enumerate`, `zip`… ;
> - résumer avec `sum`, `count`, `max`, `find`… ;
> - rassembler le résultat avec `collect` ou `fold` ;
> - écrire votre propre itérateur.

## Qu'est-ce qu'un itérateur ?

Imaginez un distributeur de tickets : à chaque fois que vous appuyez, il donne le ticket
suivant. Quand il n'y en a plus, il ne donne plus rien.

Un itérateur fait la même chose avec les éléments d'une collection. Sa méthode `next()`
donne `Some(élément)`, puis `None` quand c'est fini :

```rust
let fruits = vec!["pomme", "poire"];
let mut distributeur = fruits.iter();

println!("{:?}", distributeur.next());
println!("{:?}", distributeur.next());
println!("{:?}", distributeur.next());
```

```sortie
Some("pomme")
Some("poire")
None
```

Vous n'appelez presque jamais `next()` vous-même : la boucle `for` le fait pour vous.

```rust
let fruits = vec!["pomme", "poire"];
for fruit in fruits.iter() {
    println!("J'aime la {fruit}");
}
```

```sortie
J'aime la pomme
J'aime la poire
```

## Trois façons de parcourir : iter, iter_mut et into_iter

Il existe trois méthodes, selon ce que vous voulez faire des éléments. Elles suivent les
règles de la [possession](possession.md).

| Méthode | Chaque élément est… | La collection après |
|---------|---------------------|---------------------|
| `v.iter()` | emprunté en lecture (`&T`) | toujours utilisable |
| `v.iter_mut()` | emprunté en écriture (`&mut T`) | utilisable, modifiée |
| `v.into_iter()` | donné (`T`) | n'existe plus |

### iter : regarder

```rust
let notes = vec![12, 15, 9];
for note in notes.iter() {
    println!("note : {note}");
}
println!("Il y a toujours {} notes", notes.len());
```

```sortie
note : 12
note : 15
note : 9
Il y a toujours 3 notes
```

### iter_mut : modifier sur place

Chaque élément est une référence modifiable : on écrit `*note` pour changer la valeur.

```rust
let mut notes = vec![12, 15, 9];
for note in notes.iter_mut() {
    *note += 1; // un point bonus pour tout le monde
}
println!("{:?}", notes);
```

```sortie
[13, 16, 10]
```

### into_iter : consommer

La collection est donnée à la boucle : vous récupérez les éléments eux-mêmes, mais la
collection n'est plus utilisable ensuite.

```rust
let noms = vec![String::from("Léa"), String::from("Tom")];
for nom in noms.into_iter() {
    println!("{nom} est à moi maintenant");
}
// ici, `noms` n'existe plus
```

```sortie
Léa est à moi maintenant
Tom est à moi maintenant
```

> **Astuce :** `for x in &v` est un raccourci pour `v.iter()`, `for x in &mut v` pour
> `v.iter_mut()`, et `for x in v` pour `v.into_iter()`.

## Transformer : map et filter

Les **adaptateurs** prennent un itérateur et en fabriquent un nouveau. Les deux plus
utiles sont :

- `map(|x| …)` : transforme chaque élément ;
- `filter(|x| …)` : garde seulement les éléments pour lesquels la condition est vraie.

```rust
let nombres = vec![1, 2, 3, 4, 5, 6];

let doubles: Vec<i32> = nombres.iter().map(|n| n * 2).collect();
let pairs: Vec<&i32> = nombres.iter().filter(|n| *n % 2 == 0).collect();

println!("{:?}", doubles);
println!("{:?}", pairs);
```

```sortie
[2, 4, 6, 8, 10, 12]
[2, 4, 6]
```

Ce qui est entre `| |` est une [closure](fonctions.md) : une petite fonction sans nom.
`collect()` rassemble le résultat dans un `Vec` (voir plus bas).

> **Note :** dans `filter`, la closure reçoit une référence vers l'élément (ici `&&i32`).
> L'étoile `*n` lit la valeur. Si le compilateur se plaint, essayez d'ajouter ou
> d'enlever une `*`.

On peut **enchaîner** les adaptateurs. Lisez la chaîne de haut en bas, comme une recette :

```rust
let nombres = vec![1, 2, 3, 4, 5, 6];
let resultat: Vec<i32> = nombres
    .iter()
    .filter(|n| *n % 2 == 0) // garde 2, 4, 6
    .map(|n| n * 10)         // donne 20, 40, 60
    .collect();
println!("{:?}", resultat);
```

```sortie
[20, 40, 60]
```

## Numéroter, associer, inverser…

### enumerate : avoir le numéro

`enumerate` donne des paires `(position, élément)`. La position commence à 0.

```rust
let podium = ["Léa", "Tom", "Inès"];
for (i, nom) in podium.iter().enumerate() {
    println!("{}. {nom}", i + 1);
}
```

```sortie
1. Léa
2. Tom
3. Inès
```

### zip : marcher à deux

`zip` avance dans deux itérateurs en même temps et donne des paires. Il s'arrête quand
le plus court est fini.

```rust
let eleves = ["Léa", "Tom", "Inès"];
let notes = [15, 12, 18];
for (eleve, note) in eleves.iter().zip(notes.iter()) {
    println!("{eleve} : {note}");
}
```

```sortie
Léa : 15
Tom : 12
Inès : 18
```

### rev, take, skip et chain

| Adaptateur | Effet |
|------------|-------|
| `rev()` | parcourt à l'envers |
| `take(n)` | garde les `n` premiers |
| `skip(n)` | saute les `n` premiers |
| `chain(autre)` | met un autre itérateur au bout |

```rust
let v = vec![1, 2, 3, 4, 5];

let envers: Vec<i32> = v.iter().rev().copied().collect();
let debut: Vec<i32> = v.iter().take(2).copied().collect();
let fin: Vec<i32> = v.iter().skip(3).copied().collect();
let tout: Vec<i32> = debut.iter().chain(fin.iter()).copied().collect();

println!("{:?}", envers);
println!("{:?} {:?}", debut, fin);
println!("{:?}", tout);
```

```sortie
[5, 4, 3, 2, 1]
[1, 2] [4, 5]
[1, 2, 4, 5]
```

`copied()` transforme les références `&i32` en valeurs `i32` : c'est pratique pour
obtenir un `Vec<i32>` plutôt qu'un `Vec<&i32>`.

Les intervalles comme `1..=5` sont aussi des itérateurs :

```rust
let compte_a_rebours: Vec<i32> = (1..=5).rev().collect();
println!("{:?}", compte_a_rebours);
```

```sortie
[5, 4, 3, 2, 1]
```

## Résumer : sum, count, min, max…

Les **consommateurs** parcourent tout l'itérateur et donnent un seul résultat.

```rust
let notes = vec![12, 15, 9, 18];

let total: i32 = notes.iter().sum();
let nombre = notes.iter().count();
let plus_haute = notes.iter().max();
let plus_basse = notes.iter().min();

println!("total {total}, {nombre} notes");
println!("max {:?}, min {:?}", plus_haute, plus_basse);
```

```sortie
total 54, 4 notes
max Some(18), min Some(9)
```

> **Note :** `max` et `min` renvoient une `Option`, car une liste vide n'a pas de
> maximum. `sum` a besoin qu'on lui donne le type du résultat (`let total: i32`).

### Poser une question : any, all, find, position

| Méthode | Question | Résultat |
|---------|----------|----------|
| `any(…)` | au moins un élément vérifie la condition ? | `bool` |
| `all(…)` | tous les éléments la vérifient ? | `bool` |
| `find(…)` | quel est le premier qui la vérifie ? | `Option<&T>` |
| `position(…)` | à quelle position est le premier ? | `Option<usize>` |

```rust
let notes = vec![12, 15, 9, 18];

println!("{}", notes.iter().any(|n| *n < 10));
println!("{}", notes.iter().all(|n| *n >= 10));
println!("{:?}", notes.iter().find(|n| **n > 13));
println!("{:?}", notes.iter().position(|n| *n == 9));
println!("{:?}", notes.iter().find(|n| **n > 20));
```

```sortie
true
false
Some(15)
Some(2)
None
```

## Rassembler avec collect

`collect()` range les éléments dans une collection. Vous choisissez laquelle en donnant
le type, à gauche :

```rust
use std::collections::HashMap;

fn main() {
    // dans un Vec
    let carres: Vec<i32> = (1..=4).map(|n| n * n).collect();

    // dans une String (à partir de caractères)
    let cri: String = "bonjour".chars().map(|c| c.to_ascii_uppercase()).collect();

    // dans une HashMap (à partir de paires)
    let ages: HashMap<&str, i32> = vec![("Léa", 12), ("Tom", 14)].into_iter().collect();

    println!("{:?}", carres);
    println!("{cri}");
    println!("Tom a {} ans", ages["Tom"]);
}
```

```sortie
[1, 4, 9, 16]
BONJOUR
Tom a 14 ans
```

> **Astuce :** écrivez `Vec<_>` : le `_` laisse Rust deviner le type des éléments.
> Par exemple `let v: Vec<_> = (1..4).collect();`.

## Accumuler avec fold

`fold` part d'une valeur de départ, puis la met à jour avec chaque élément. C'est une
boucle qui garde un « total » :

```rust
let nombres = vec![1, 2, 3, 4];

// départ : 0 ; à chaque tour : total + n
let somme = nombres.iter().fold(0, |total, n| total + n);

// départ : un texte vide ; à chaque tour : on ajoute le nombre
let texte = nombres.iter().fold(String::new(), |acc, n| acc + &n.to_string());

println!("{somme}");
println!("{texte}");
```

```sortie
10
1234
```

Voici ce qui se passe pour `somme` :

```text
total = 0
total = 0 + 1 = 1
total = 1 + 2 = 3
total = 3 + 3 = 6
total = 6 + 4 = 10   → résultat
```

## Les itérateurs sont paresseux

Un adaptateur comme `map` ou `filter` **ne fait rien tout seul**. Il prépare le travail,
comme une recette écrite qu'on n'a pas encore cuisinée. Le travail commence seulement
quand un consommateur (`for`, `collect`, `sum`…) demande les éléments.

```rust
let nombres = vec![1, 2, 3];

let prepare = nombres.iter().map(|n| {
    println!("je calcule {n} × 10");
    n * 10
});
println!("Rien n'est encore calculé.");

let resultats: Vec<i32> = prepare.collect();
println!("{:?}", resultats);
```

```sortie
Rien n'est encore calculé.
je calcule 1 × 10
je calcule 2 × 10
je calcule 3 × 10
[10, 20, 30]
```

Cette paresse a un avantage : Rust ne calcule que ce qui est nécessaire. Ici, `take(3)`
s'arrête après trois éléments, même si l'intervalle n'a pas de fin :

```rust
let premiers_carres: Vec<u64> = (1..).map(|n| n * n).take(3).collect();
println!("{:?}", premiers_carres);
```

```sortie
[1, 4, 9]
```

> **À retenir :** pour faire une action sur chaque élément (afficher…), utilisez une
> boucle `for` ou `for_each`, pas `map` seul.

## Écrire son propre itérateur

Pour que votre structure devienne un itérateur, implémentez le [trait](generiques.md)
`Iterator`. Il suffit d'écrire deux choses :

- `type Item` : le type des éléments donnés ;
- `fn next(&mut self)` : renvoie `Some(élément suivant)`, ou `None` quand c'est fini.

Voici un compte à rebours :

```rust
struct CompteARebours {
    reste: u32,
}

impl Iterator for CompteARebours {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.reste == 0 {
            None // c'est fini
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
    println!("Décollage !");

    // toutes les méthodes des itérateurs marchent aussi :
    let total: u32 = CompteARebours { reste: 4 }.sum();
    println!("total : {total}");
}
```

```sortie
3…
2…
1…
Décollage !
total : 10
```

Dès que `next` est écrit, vous avez gratuitement `map`, `filter`, `sum`, `collect`… et
toutes les autres méthodes de cette page.

## Erreurs fréquentes

### Oublier le type de collect

```rust norun
let nombres = vec![1, 2, 3];
let doubles = nombres.iter().map(|n| n * 2).collect();
```

```text
error[E0283]: type annotations needed
help: consider giving `doubles` an explicit type
  |
3 |     let doubles: Vec<_> = nombres.iter().map(|n| n * 2).collect();
```

`collect` peut fabriquer beaucoup de collections différentes : dites-lui laquelle.

```rust
let nombres = vec![1, 2, 3];
let doubles: Vec<i32> = nombres.iter().map(|n| n * 2).collect();
println!("{:?}", doubles);
```

```sortie
[2, 4, 6]
```

La même erreur arrive avec `sum` : écrivez `let total: i32 = ….sum();`.

### Utiliser map sans consommer

```rust norun
let nombres = vec![1, 2, 3];
nombres.iter().map(|n| println!("{n}"));
```

```text
warning: unused `Map` that must be used
  = note: iterators are lazy and do nothing unless consumed
```

Rien ne s'affiche : l'itérateur est paresseux. Utilisez `for_each` (ou une boucle `for`) :

```rust
let nombres = vec![1, 2, 3];
nombres.iter().for_each(|n| println!("{n}"));
```

```sortie
1
2
3
```

### Réutiliser une collection consommée

```rust norun
let noms = vec![String::from("Léa"), String::from("Tom")];
for nom in noms {
    println!("{nom}");
}
println!("{}", noms.len());
```

```text
error[E0382]: borrow of moved value: `noms`
help: consider iterating over a slice of the `Vec<String>`'s content to avoid moving into the `for` loop
  |
3 |     for nom in &noms {
```

`for nom in noms` appelle `into_iter()` : la liste est donnée à la boucle. Parcourez
plutôt une référence, avec `&noms` :

```rust
let noms = vec![String::from("Léa"), String::from("Tom")];
for nom in &noms {
    println!("{nom}");
}
println!("{}", noms.len());
```

```sortie
Léa
Tom
2
```

## Testez-vous

1. Que vaut `(1..=4).filter(|n| n % 2 == 1).count()` ?

> **Réponse**
>
> `2` : les nombres impairs de 1 à 4 sont 1 et 3.

2. Quelle méthode choisir pour ajouter 1 à chaque élément d'un `Vec`, sur place ?

> **Réponse**
>
> `iter_mut()` : `for x in v.iter_mut() { *x += 1; }`. `iter()` ne permet que de lire,
> et `into_iter()` consomme la liste.

3. Pourquoi `v.iter().map(|x| x * 2);` seul ne fait-il rien ?

> **Réponse**
>
> Parce que les itérateurs sont paresseux : `map` prépare le calcul, mais rien ne se
> passe tant qu'un consommateur (`collect`, `sum`, `for`…) ne demande pas les éléments.

## En résumé

- `iter()` lit, `iter_mut()` modifie, `into_iter()` consomme la collection.
- Les adaptateurs (`map`, `filter`, `enumerate`, `zip`, `rev`, `take`, `skip`, `chain`)
  fabriquent un nouvel itérateur.
- Les consommateurs (`sum`, `count`, `max`, `any`, `find`, `collect`, `fold`…) donnent
  le résultat.
- Les itérateurs sont paresseux : rien ne se passe sans consommateur.
- Implémentez `Iterator` (avec `next`) pour rendre votre structure parcourable.

Comparez avec [les blocs en Ruby](../ruby/blocs.md) et
[le style fonctionnel en Rythme](../rythme/fonctionnel.md). Suite :
[Modules et crates](modules.md).
