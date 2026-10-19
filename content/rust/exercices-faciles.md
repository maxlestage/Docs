# Exercices pour bien démarrer

Cette page propose quinze petits exercices très guidés, tous faciles, pour pratiquer les bases de Rust : variables, nombres, conditions, boucles, fonctions, texte, listes et tables.

## Comment faire ces exercices

Chaque exercice est court. Prenez-les dans l'ordre : chacun utilise un peu plus de
choses que le précédent.

1. **Lisez** l'énoncé et la sortie attendue.
2. **Essayez sur papier** : écrivez les étapes en français, ligne par ligne.
3. **Écrivez le code** dans un projet créé avec `cargo new exercices`, puis lancez-le
   avec `cargo run`.
4. **Comparez** votre sortie avec la sortie attendue, caractère par caractère.
5. Bloqué ? Lisez **le premier indice**, puis le second. Regardez la solution en
   dernier.

> **Astuce :** si le compilateur affiche une erreur, lisez la ligne `help:`. Elle
> contient souvent la correction. La page [Les pièges du débutant](pieges.md) explique
> les erreurs les plus courantes.

Votre solution peut être différente de la nôtre et tout aussi juste. Si elle compile
et affiche la bonne sortie, c'est gagné.

## Afficher et calculer

### Exercice 1 : Dire bonjour

Niveau : ★☆☆

Écrivez un programme qui affiche deux lignes : `Bonjour !` puis `J'apprends Rust.`

Sortie attendue :

```text
Bonjour !
J'apprends Rust.
```

> **Indice :** une ligne s'affiche avec `println!("…");`. N'oubliez pas le `!`.

> **Solution**
>
> ```rust
> fn main() {
>     println!("Bonjour !");
>     println!("J'apprends Rust.");
> }
> ```
>
> ```sortie
> Bonjour !
> J'apprends Rust.
> ```
>
> Pourquoi ça marche : chaque `println!` affiche son texte, puis passe à la ligne.

### Exercice 2 : Dans dix ans

Niveau : ★☆☆

Créez une variable `prenom` qui vaut `"Léa"` et une variable `age` qui vaut `14`.
Affichez son âge dans dix ans.

Sortie attendue :

```text
Léa aura 24 ans dans dix ans.
```

> **Indice :** créez les variables avec `let prenom = "Léa";` et `let age = 14;`.

> **Indice :** le calcul peut se faire dans une autre variable :
> `let plus_tard = age + 10;`. Puis affichez-la avec `{plus_tard}`.

> **Solution**
>
> ```rust
> fn main() {
>     let prenom = "Léa";
>     let age = 14;
>     let plus_tard = age + 10;
>     println!("{prenom} aura {plus_tard} ans dans dix ans.");
> }
> ```
>
> ```sortie
> Léa aura 24 ans dans dix ans.
> ```
>
> Pourquoi ça marche : `{prenom}` et `{plus_tard}` sont remplacés par la valeur des
> variables au moment de l'affichage.

### Exercice 3 : Un compteur qui avance

Niveau : ★☆☆

Créez une variable `pas` qui vaut 0. Ajoutez-lui 100 trois fois, en affichant sa
valeur après chaque ajout.

Sortie attendue :

```text
100
200
300
```

> **Indice :** une variable qui change doit être déclarée avec `let mut`.

> **Indice :** pour ajouter 100, écrivez `pas += 100;` (c'est la même chose que
> `pas = pas + 100;`).

> **Solution**
>
> ```rust
> fn main() {
>     let mut pas = 0;
>     pas += 100;
>     println!("{pas}");
>     pas += 100;
>     println!("{pas}");
>     pas += 100;
>     println!("{pas}");
> }
> ```
>
> ```sortie
> 100
> 200
> 300
> ```
>
> Pourquoi ça marche : `mut` autorise la variable à changer. Sans lui, Rust refuse
> la deuxième ligne.

### Exercice 4 : Heures et minutes

Niveau : ★☆☆

Un film dure 135 minutes. Affichez sa durée en heures et minutes.

Sortie attendue :

```text
2 h 15 min
```

> **Indice :** une heure, c'est 60 minutes. Combien de fois 60 entre dans 135 ?

> **Indice :** entre deux entiers, `/` donne le nombre entier de fois (`135 / 60`) et
> `%` donne ce qui reste (`135 % 60`).

> **Solution**
>
> ```rust
> fn main() {
>     let duree = 135;
>     let heures = duree / 60;
>     let minutes = duree % 60;
>     println!("{heures} h {minutes} min");
> }
> ```
>
> ```sortie
> 2 h 15 min
> ```
>
> Pourquoi ça marche : 60 entre 2 fois dans 135 (cela fait 120), et il reste
> 135 − 120 = 15 minutes.

### Exercice 5 : Celsius en Fahrenheit

Niveau : ★☆☆

Convertissez 20 °C en degrés Fahrenheit. La formule est : `F = C × 9 / 5 + 32`.

Sortie attendue :

```text
20 °C = 68 °F
```

> **Indice :** utilisez des nombres à virgule (`f64`) : écrivez `20.0`, `9.0`, `5.0`
> et `32.0`.

> **Solution**
>
> ```rust
> fn main() {
>     let celsius = 20.0;
>     let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
>     println!("{celsius} °C = {fahrenheit} °F");
> }
> ```
>
> ```sortie
> 20 °C = 68 °F
> ```
>
> Pourquoi ça marche : 20 × 9 = 180, 180 / 5 = 36, 36 + 32 = 68. Avec des `f64`, la
> division ne coupe pas la virgule. Rust affiche `20.0` sous la forme `20`.

## Décider et répéter

### Exercice 6 : Pair ou impair

Niveau : ★☆☆

Créez une variable `n` qui vaut 7. Affichez `7 est impair` (ou `pair` si vous changez
le nombre).

Sortie attendue :

```text
7 est impair
```

> **Indice :** un nombre est pair quand le reste de sa division par 2 vaut 0 :
> `n % 2 == 0`.

> **Indice :** utilisez `if … { … } else { … }`.

> **Solution**
>
> ```rust
> fn main() {
>     let n = 7;
>     if n % 2 == 0 {
>         println!("{n} est pair");
>     } else {
>         println!("{n} est impair");
>     }
> }
> ```
>
> ```sortie
> 7 est impair
> ```
>
> Pourquoi ça marche : `7 % 2` vaut 1, donc la condition est fausse et c'est le bloc
> `else` qui s'exécute.

### Exercice 7 : La table de 7

Niveau : ★☆☆

Affichez les cinq premières lignes de la table de multiplication de 7.

Sortie attendue :

```text
7 x 1 = 7
7 x 2 = 14
7 x 3 = 21
7 x 4 = 28
7 x 5 = 35
```

> **Indice :** une boucle `for i in 1..=5` fait prendre à `i` les valeurs 1, 2, 3, 4
> et 5.

> **Solution**
>
> ```rust
> fn main() {
>     for i in 1..=5 {
>         println!("7 x {i} = {}", 7 * i);
>     }
> }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```
>
> Pourquoi ça marche : le `=` de `1..=5` inclut le 5. Les `{}` vides sont remplacés
> par le calcul écrit après la virgule.

### Exercice 8 : La somme de 1 à 100

Niveau : ★☆☆

Calculez 1 + 2 + 3 + … + 100 avec une boucle, et affichez le résultat.

Sortie attendue :

```text
5050
```

> **Indice :** il faut une variable `total`, qui commence à 0 et qui change : `let mut`.

> **Indice :** dans la boucle `for n in 1..=100`, ajoutez `n` au total. Voici les
> premiers tours :
>
> | Tour | n | total après |
> |------|---|-------------|
> | 1 | 1 | 1 |
> | 2 | 2 | 3 |
> | 3 | 3 | 6 |

> **Solution**
>
> ```rust
> fn main() {
>     let mut total = 0;
>     for n in 1..=100 {
>         total += n;
>     }
>     println!("{total}");
> }
> ```
>
> ```sortie
> 5050
> ```
>
> Pourquoi ça marche : `total` est créé **avant** la boucle, une seule fois. Chaque
> tour lui ajoute le nombre suivant.

### Exercice 9 : Le jour de la semaine

Niveau : ★☆☆

Créez une variable `jour` qui vaut 3. Avec `match`, affichez le nom du jour : 1 pour
lundi, 2 pour mardi, 3 pour mercredi. Pour tout autre nombre, affichez `autre jour`.

Sortie attendue :

```text
mercredi
```

> **Indice :** un `match` liste des cas : `1 => println!("lundi"),`.

> **Indice :** le cas `_ =>` attrape « tout le reste ». Rust l'exige ici, car `jour`
> pourrait valoir n'importe quel nombre.

> **Solution**
>
> ```rust
> fn main() {
>     let jour = 3;
>     match jour {
>         1 => println!("lundi"),
>         2 => println!("mardi"),
>         3 => println!("mercredi"),
>         _ => println!("autre jour"),
>     }
> }
> ```
>
> ```sortie
> mercredi
> ```
>
> Pourquoi ça marche : Rust essaie les cas de haut en bas et exécute le premier qui
> correspond.

## Écrire des fonctions

### Exercice 10 : Le carré

Niveau : ★☆☆

Écrivez une fonction `carre` qui reçoit un `i32` et renvoie ce nombre multiplié par
lui-même. Affichez `carre(4)` et `carre(9)`.

Sortie attendue :

```text
16
81
```

> **Indice :** la première ligne s'écrit `fn carre(x: i32) -> i32 {`.

> **Indice :** la dernière ligne de la fonction est la valeur renvoyée, **sans**
> point-virgule : `x * x`.

> **Solution**
>
> ```rust
> fn carre(x: i32) -> i32 {
>     x * x
> }
>
> fn main() {
>     println!("{}", carre(4));
>     println!("{}", carre(9));
> }
> ```
>
> ```sortie
> 16
> 81
> ```
>
> Pourquoi ça marche : `-> i32` annonce ce que la fonction renvoie, et `x * x` sans
> point-virgule est cette valeur.

### Exercice 11 : Majeur ou pas

Niveau : ★☆☆

Écrivez une fonction `est_majeur` qui reçoit un âge (`u32`) et renvoie `true` si
l'âge est 18 ou plus. Testez-la avec 15 et 18.

Sortie attendue :

```text
15 ans : false
18 ans : true
```

> **Indice :** la fonction renvoie un `bool` : `fn est_majeur(age: u32) -> bool`.

> **Indice :** une comparaison donne déjà `true` ou `false`. Le corps de la fonction
> tient en une ligne : `age >= 18`.

> **Solution**
>
> ```rust
> fn est_majeur(age: u32) -> bool {
>     age >= 18
> }
>
> fn main() {
>     println!("15 ans : {}", est_majeur(15));
>     println!("18 ans : {}", est_majeur(18));
> }
> ```
>
> ```sortie
> 15 ans : false
> 18 ans : true
> ```
>
> Pourquoi ça marche : `>=` veut dire « plus grand ou égal ». Pas besoin de `if` :
> la comparaison est déjà la réponse.

## Jouer avec le texte

### Exercice 12 : Crier un mot

Niveau : ★☆☆

Pour le mot `"bonjour"`, affichez son nombre de lettres, puis le mot en majuscules.

Sortie attendue :

```text
7 lettres
BONJOUR
```

> **Indice :** `.chars().count()` compte les lettres d'un texte.

> **Indice :** `.to_uppercase()` renvoie le texte en majuscules.

> **Solution**
>
> ```rust
> fn main() {
>     let mot = "bonjour";
>     println!("{} lettres", mot.chars().count());
>     println!("{}", mot.to_uppercase());
> }
> ```
>
> ```sortie
> 7 lettres
> BONJOUR
> ```
>
> Pourquoi ça marche : `.chars()` parcourt les lettres une par une, et `.count()` les
> compte. Avec `"été"`, `.chars().count()` donne bien 3.

### Exercice 13 : Construire une phrase

Niveau : ★☆☆

Partez d'une `String` vide. Ajoutez-lui `"Rust"`, puis `" est"`, puis `" rapide"`.
Affichez la phrase et sa longueur.

Sortie attendue :

```text
Rust est rapide
15
```

> **Indice :** une `String` vide se crée avec `String::new()`. Elle va changer, donc :
> `let mut phrase = String::new();`.

> **Indice :** `phrase.push_str("Rust");` ajoute du texte à la fin.

> **Solution**
>
> ```rust
> fn main() {
>     let mut phrase = String::new();
>     phrase.push_str("Rust");
>     phrase.push_str(" est");
>     phrase.push_str(" rapide");
>     println!("{phrase}");
>     println!("{}", phrase.len());
> }
> ```
>
> ```sortie
> Rust est rapide
> 15
> ```
>
> Pourquoi ça marche : une `String` est un texte à vous, qui peut grandir. Les
> espaces font partie des textes ajoutés : ils comptent dans la longueur.

## Listes et tables

### Exercice 14 : La meilleure note

Niveau : ★☆☆

Voici des notes : `vec![12, 17, 9, 15]`. Affichez leur somme, puis la plus grande.

Sortie attendue :

```text
somme : 53
meilleure : 17
```

> **Indice :** parcourez la liste avec `for n in &notes`. Gardez deux variables
> `mut` : `somme` (à 0) et `meilleure` (à 0).

> **Indice :** dans la boucle : `somme += n;`, puis
> `if *n > meilleure { meilleure = *n; }`. L'étoile `*` lit le nombre à travers le `&`.
>
> | Tour | n | somme | meilleure |
> |------|---|-------|-----------|
> | 1 | 12 | 12 | 12 |
> | 2 | 17 | 29 | 17 |
> | 3 | 9 | 38 | 17 |
> | 4 | 15 | 53 | 17 |

> **Solution**
>
> ```rust
> fn main() {
>     let notes = vec![12, 17, 9, 15];
>     let mut somme = 0;
>     let mut meilleure = 0;
>     for n in &notes {
>         somme += n;
>         if *n > meilleure {
>             meilleure = *n;
>         }
>     }
>     println!("somme : {somme}");
>     println!("meilleure : {meilleure}");
> }
> ```
>
> ```sortie
> somme : 53
> meilleure : 17
> ```
>
> Pourquoi ça marche : `meilleure` retient la plus grande note vue jusqu'ici. Elle
> ne change que si la note du tour est plus grande. Rust sait aussi le faire seul :
> `notes.iter().sum::<i32>()` et `notes.iter().max()`.

### Exercice 15 : Compter les fruits

Niveau : ★☆☆

Voici un panier : `["pomme", "kiwi", "pomme", "poire", "pomme"]`. Comptez chaque
fruit avec une `HashMap`, puis affichez le nombre de pommes et de kiwis.

Sortie attendue :

```text
pommes : 3
kiwis : 1
```

> **Indice :** commencez par `use std::collections::HashMap;` et
> `let mut compte = HashMap::new();`.

> **Indice :** dans la boucle, cette ligne ajoute 1 au fruit (et le crée à 0 s'il
> n'existe pas) : `*compte.entry(fruit).or_insert(0) += 1;`.

> **Solution**
>
> ```rust
> use std::collections::HashMap;
>
> fn main() {
>     let panier = ["pomme", "kiwi", "pomme", "poire", "pomme"];
>     let mut compte = HashMap::new();
>     for fruit in panier {
>         *compte.entry(fruit).or_insert(0) += 1;
>     }
>     println!("pommes : {}", compte["pomme"]);
>     println!("kiwis : {}", compte["kiwi"]);
> }
> ```
>
> ```sortie
> pommes : 3
> kiwis : 1
> ```
>
> Pourquoi ça marche : la table associe chaque fruit à un compteur. On n'affiche que
> des clés précises : l'ordre (imprévisible) d'une `HashMap` ne gêne donc pas. Plus
> de détails dans [HashMap, HashSet et BTreeMap](hashmap.md).

Pour des exercices plus difficiles, sur tout le parcours, voyez les
[Exercices corrigés](exercices.md).

Suite : [Possession et emprunt](possession.md)
