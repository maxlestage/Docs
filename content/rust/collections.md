# Collections

Cette page présente les collections les plus utiles de Rust : les vecteurs `Vec`, les tables `HashMap`, les itérateurs et les tranches.

> **Objectifs :**
>
> - choisir entre un tableau de taille fixe et un `Vec` ;
> - ajouter, lire et retirer des éléments d'un `Vec` sans risque ;
> - associer des clés à des valeurs avec `HashMap` et `BTreeMap` ;
> - transformer une collection avec `map`, `filter` et `collect` ;
> - prendre une partie d'une liste avec une tranche.

Une **collection** est une valeur qui en contient plusieurs autres, comme une boîte de
rangement.

## Les tableaux de taille fixe

Un tableau `[T; N]` contient toujours le même nombre `N` d'éléments, tous du même type
`T`. Les positions (les **indices**) commencent à 0 :

```rust
let jours = ["lun", "mar", "mer"];
println!("{}", jours[0]);
println!("{}", jours.len());
println!("{:?}", jours);
```

```sortie
lun
3
["lun", "mar", "mer"]
```

Un tableau peut aussi être rempli d'une même valeur : `[0; 5]` donne cinq zéros.

```rust
let zeros = [0; 5];
println!("{:?}", zeros);
```

```sortie
[0, 0, 0, 0, 0]
```

## Les vecteurs : Vec

Un `Vec` (un « vecteur ») est une [liste](../debuter/glossaire.md#liste) qui peut
grandir ou rétrécir. C'est la collection la plus courante. La macro `vec![…]` en crée
un ; `push` ajoute à la fin, `pop` retire le dernier élément.

```rust
let mut notes = vec![12, 15, 9];
notes.push(18);
println!("{:?}", notes);
println!("Premier : {}", notes[0]);
println!("Taille : {}", notes.len());
let dernier = notes.pop();
println!("{:?} {:?}", dernier, notes);
```

```sortie
[12, 15, 9, 18]
Premier : 12
Taille : 4
Some(18) [12, 15, 9]
```

`pop` renvoie `Some(18)` : la valeur retirée, dans une `Option`, car la liste aurait pu
être vide.

### Tableau ou Vec ?

| | Tableau `[T; N]` | `Vec<T>` |
|-|------------------|----------|
| Taille | fixée à l'écriture du code | change pendant l'exécution |
| Ajouter / retirer | impossible | `push`, `pop`, `insert`, `remove` |
| Usage typique | les 7 jours, les 12 mois | une liste de courses, des notes |

> **Astuce :** dans le doute, prenez un `Vec`.

### Lire sans risque avec get

`notes[10]` arrête le programme si l'élément n'existe pas. `get` renvoie une `Option`
(voir [Option](types.md#le-type-option)) :

```rust
let notes = vec![12, 15, 9];
println!("{:?}", notes.get(1));
println!("{:?}", notes.get(10));
```

```sortie
Some(15)
None
```

### Parcourir un vecteur

```rust
let mut prix = vec![10, 20, 30];
for p in &prix {
    print!("{p} ");
}
println!();
for p in &mut prix {
    *p += 1;
}
println!("{:?}", prix);
```

```sortie
10 20 30 
[11, 21, 31]
```

> **Note :** le `&` devant `prix` emprunte le vecteur au lieu de le consommer. Tout est
> expliqué dans [Possession et emprunt](possession.md).

## Les tables : HashMap

Une `HashMap` est un [dictionnaire](../debuter/glossaire.md#dictionnaire) : elle
associe des **clés** à des **valeurs**, comme un annuaire associe un nom à un numéro.
Elle ne fait pas partie des types chargés d'office : il faut l'importer avec `use`.

```rust
use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::new();
    ages.insert("Alice", 30);
    ages.insert("Bob", 25);
    ages.insert("Alice", 31);
    println!("{:?}", ages.get("Alice"));
    println!("{}", ages.len());
    println!("{}", ages.contains_key("Zoé"));
}
```

```sortie
Some(31)
2
false
```

### Compter avec entry

`entry(...).or_insert(...)` crée la valeur si elle manque, puis donne accès à la
valeur. L'étoile `*` permet de modifier ce qu'elle désigne :

```rust
use std::collections::HashMap;

fn main() {
    let texte = "le chat et le chien et le lapin";
    let mut compte = HashMap::new();
    for mot in texte.split_whitespace() {
        *compte.entry(mot).or_insert(0) += 1;
    }
    let mut paires: Vec<_> = compte.into_iter().collect();
    paires.sort();
    println!("{:?}", paires);
}
```

```sortie
[("chat", 1), ("chien", 1), ("et", 2), ("lapin", 1), ("le", 3)]
```

> **Attention :** l'ordre de parcours d'une `HashMap` est aléatoire. Pour un ordre
> trié, triez le résultat comme ci-dessus, ou utilisez `BTreeMap`.

### BTreeMap : une table triée

`BTreeMap` s'utilise comme `HashMap`, mais garde les clés dans l'ordre :

```rust
use std::collections::BTreeMap;

fn main() {
    let mut stock = BTreeMap::new();
    stock.insert("poires", 4);
    stock.insert("abricots", 10);
    stock.insert("kiwis", 7);
    for (fruit, quantite) in &stock {
        println!("{fruit} : {quantite}");
    }
}
```

```sortie
abricots : 10
kiwis : 7
poires : 4
```

## Les itérateurs

`iter()` crée un **itérateur** : un objet qui donne les éléments un par un, comme un
tapis roulant de caisse. On enchaîne ensuite des opérations avec des
[closures](fonctions.md#les-closures). Les itérateurs ont leur propre chapitre :
[Les itérateurs](iterateurs.md).

| Méthode | Effet |
|---------|-------|
| `map(f)` | transforme chaque élément |
| `filter(f)` | garde les éléments qui vérifient `f` |
| `collect()` | rassemble le résultat (souvent dans un `Vec`) |
| `sum()` | additionne |
| `count()` | compte |

```rust
let nombres = vec![1, 2, 3, 4, 5, 6];
let doubles: Vec<i32> = nombres.iter().map(|n| n * 2).collect();
let pairs: Vec<&i32> = nombres.iter().filter(|n| *n % 2 == 0).collect();
let total: i32 = nombres.iter().sum();
let grands = nombres.iter().filter(|n| **n > 3).count();
println!("{:?}", doubles);
println!("{:?}", pairs);
println!("{total} {grands}");
```

```sortie
[2, 4, 6, 8, 10, 12]
[2, 4, 6]
21 3
```

On peut enchaîner autant d'étapes que l'on veut :

```rust
let mots = vec!["rust", "est", "rapide"];
let resultat: Vec<String> = mots
    .iter()
    .filter(|m| m.len() > 3)
    .map(|m| m.to_uppercase())
    .collect();
println!("{:?}", resultat);
```

```sortie
["RUST", "RAPIDE"]
```

> **Astuce :** un itérateur ne fait rien tant qu'on ne le consomme pas (avec
> `collect`, `sum`, `for`…). On dit qu'il est **paresseux**.

## Les tranches

Une **tranche** (`&[T]`) est une vue sur une partie d'un tableau ou d'un vecteur, sans
copie :

```rust
let lettres = vec!['a', 'b', 'c', 'd', 'e'];
let milieu = &lettres[1..4];
let debut = &lettres[..2];
println!("{:?} {:?}", milieu, debut);
```

```sortie
['b', 'c', 'd'] ['a', 'b']
```

Une fonction qui accepte `&[i32]` fonctionne avec un tableau comme avec un `Vec` :

```rust
fn moyenne(valeurs: &[f64]) -> f64 {
    valeurs.iter().sum::<f64>() / valeurs.len() as f64
}

fn main() {
    let v = vec![10.0, 12.0, 17.0];
    println!("{}", moyenne(&v));
    println!("{}", moyenne(&[4.0, 6.0]));
}
```

```sortie
13
5
```

## Erreurs fréquentes

### Oublier d'importer HashMap

```rust norun
fn main() {
    let mut ages = HashMap::new();
    ages.insert("Alice", 30);
}
```

```text
error[E0433]: cannot find type `HashMap` in this scope
 --> src/main.rs:2:20
  |
2 |     let mut ages = HashMap::new();
  |                    ^^^^^^^ use of undeclared type `HashMap`
help: consider importing this struct
  |
1 + use std::collections::HashMap;
```

Ajoutez `use std::collections::HashMap;` en haut du fichier.

### Mélanger les types dans un Vec

```rust norun
fn main() {
    let melange = vec![1, "deux", 3];
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:2:27
  |
2 |     let melange = vec![1, "deux", 3];
  |                           ^^^^^^ expected integer, found `&str`
```

Tous les éléments d'un `Vec` ont le même type. Écrivez `vec![1, 2, 3]`, ou rangez des
valeurs différentes dans une [énumération](types.md#les-enumerations).

### Réutiliser un Vec après un for

```rust norun
fn main() {
    let notes = vec![12, 15, 9];
    for n in notes {
        println!("{n}");
    }
    println!("{}", notes.len());
}
```

```text
error[E0382]: borrow of moved value: `notes`
 --> src/main.rs:6:20
  |
2 |     let notes = vec![12, 15, 9];
  |         ----- move occurs because `notes` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     for n in notes {
  |              ----- `notes` moved due to this implicit call to `.into_iter()`
...
6 |     println!("{}", notes.len());
  |                    ^^^^^ value borrowed here after move
```

`for n in notes` consomme le vecteur. Pour le garder, parcourez-le par emprunt :
`for n in &notes`. La raison est expliquée dans [Possession et emprunt](possession.md).

## Testez-vous

**1.** Que donne `vec![3, 1, 2].get(5)` ?

> **Réponse**
>
> `None` : il n'y a pas d'élément à la position 5. Avec `[5]`, le programme
> s'arrêterait.

**2.** Comment compter combien de fois chaque lettre apparaît dans `"papa"` ?

> **Réponse**
>
> ```rust
> use std::collections::BTreeMap;
>
> fn main() {
>     let mut compte = BTreeMap::new();
>     for c in "papa".chars() {
>         *compte.entry(c).or_insert(0) += 1;
>     }
>     println!("{:?}", compte);
> }
> ```
>
> ```sortie
> {'a': 2, 'p': 2}
> ```
>
> Une `HashMap` marche aussi, mais `BTreeMap` affiche les lettres dans l'ordre.

**3.** Avec `let v = vec![1, 2, 3, 4];`, comment obtenir la somme des nombres pairs ?

> **Réponse**
>
> ```rust
> let v = vec![1, 2, 3, 4];
> let somme: i32 = v.iter().filter(|n| *n % 2 == 0).sum();
> println!("{somme}");
> ```
>
> ```sortie
> 6
> ```

## En résumé

- Un tableau `[T; N]` a une taille fixe ; un `Vec` grandit : `vec![…]`, `push`, `pop`,
  `get`.
- `HashMap` associe des clés à des valeurs ; son ordre est aléatoire.
- `BTreeMap` garde les clés triées.
- `iter()`, `map`, `filter` et `collect` transforment une collection.
- `&v[1..3]` est une tranche : une vue sur une partie d'une liste.

Comparez avec [Ruby](../ruby/collections.md) et [Rythme](../rythme/collections.md).

Suite : [Possession et emprunt](possession.md).
