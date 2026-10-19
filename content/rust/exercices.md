# Exercices corrigés

Cette page propose douze exercices, du plus facile au plus difficile, pour pratiquer tout le parcours Rust, chacun avec sa solution.

## Comment travailler

Essayez chaque exercice seul, dans un projet créé avec `cargo new exercices`. Regardez
l'indice si vous bloquez, puis la solution pour comparer.

- ★☆☆ : facile, quelques lignes ;
- ★★☆ : moyen, il faut combiner plusieurs notions ;
- ★★★ : plus dur, prenez votre temps.

> **Astuce :** votre solution peut être différente et tout aussi juste. Si elle compile
> et affiche le bon résultat, c'est gagné.

## Les bases

### Exercice 1 : Se présenter

Niveau : ★☆☆

Créez deux variables, `prenom` et `age`, puis affichez la phrase
`Je m'appelle Zoé et j'aurai 26 ans l'an prochain.` (calculez `26` à partir de `age`).

> **Solution**
>
> ```rust
> fn main() {
>     let prenom = "Zoé";
>     let age = 25;
>     println!("Je m'appelle {prenom} et j'aurai {} ans l'an prochain.", age + 1);
> }
> ```
>
> ```sortie
> Je m'appelle Zoé et j'aurai 26 ans l'an prochain.
> ```

### Exercice 2 : Pair ou impair

Niveau : ★☆☆

Pour chaque nombre de 1 à 5, affichez s'il est pair ou impair.

> **Indice :** un nombre est pair si le reste de sa division par 2 vaut 0 : `n % 2 == 0`.

> **Solution**
>
> ```rust
> fn main() {
>     for n in 1..=5 {
>         if n % 2 == 0 {
>             println!("{n} est pair");
>         } else {
>             println!("{n} est impair");
>         }
>     }
> }
> ```
>
> ```sortie
> 1 est impair
> 2 est pair
> 3 est impair
> 4 est pair
> 5 est impair
> ```

### Exercice 3 : La table de multiplication

Niveau : ★☆☆

Affichez la table de 7, de `7 x 1 = 7` à `7 x 5 = 35`.

> **Solution**
>
> ```rust
> fn main() {
>     let table = 7;
>     for i in 1..=5 {
>         println!("{table} x {i} = {}", table * i);
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

### Exercice 4 : Une fonction de conversion

Niveau : ★☆☆

Écrivez une fonction `celsius_vers_fahrenheit(c: f64) -> f64`. La formule est
`c * 9 / 5 + 32`. Affichez le résultat pour 0, 20 et 100 degrés.

> **Indice :** avec des `f64`, écrivez `9.0` et `5.0`, pas `9` et `5`.

> **Solution**
>
> ```rust
> fn celsius_vers_fahrenheit(c: f64) -> f64 {
>     c * 9.0 / 5.0 + 32.0
> }
>
> fn main() {
>     for c in [0.0, 20.0, 100.0] {
>         println!("{c} °C = {} °F", celsius_vers_fahrenheit(c));
>     }
> }
> ```
>
> ```sortie
> 0 °C = 32 °F
> 20 °C = 68 °F
> 100 °C = 212 °F
> ```

## Listes et textes

### Exercice 5 : La moyenne des notes

Niveau : ★★☆

Dans `let notes = vec![12, 15, 9, 18];`, calculez la note la plus haute et la moyenne
(un `f64`).

> **Indice :** `iter().max()` renvoie une `Option`. Pour la moyenne, convertissez avec
> `as f64`.

> **Solution**
>
> ```rust
> fn main() {
>     let notes = vec![12, 15, 9, 18];
>     let meilleure = notes.iter().max().unwrap();
>     let somme: i32 = notes.iter().sum();
>     let moyenne = somme as f64 / notes.len() as f64;
>     println!("Meilleure note : {meilleure}");
>     println!("Moyenne : {moyenne}");
> }
> ```
>
> ```sortie
> Meilleure note : 18
> Moyenne : 13.5
> ```

### Exercice 6 : Compter les voyelles

Niveau : ★★☆

Écrivez une fonction `voyelles(texte: &str) -> usize` qui compte les voyelles d'un
texte (sans accents, minuscules ou majuscules).

> **Indice :** parcourez `texte.to_lowercase().chars()` et testez si `"aeiouy"`
> contient le caractère.

> **Solution**
>
> ```rust
> fn voyelles(texte: &str) -> usize {
>     let mut total = 0;
>     for c in texte.to_lowercase().chars() {
>         if "aeiouy".contains(c) {
>             total += 1;
>         }
>     }
>     total
> }
>
> fn main() {
>     println!("{}", voyelles("Bonjour"));
>     println!("{}", voyelles("RUST"));
> }
> ```
>
> ```sortie
> 3
> 1
> ```

### Exercice 7 : Le palindrome

Niveau : ★★☆

Écrivez une fonction `est_palindrome(mot: &str) -> bool` : un palindrome se lit pareil
dans les deux sens, comme « kayak ». Ignorez les majuscules.

> **Indice :** retournez le texte avec `chars().rev().collect::<String>()`.

> **Solution**
>
> ```rust
> fn est_palindrome(mot: &str) -> bool {
>     let mot = mot.to_lowercase();
>     let envers: String = mot.chars().rev().collect();
>     mot == envers
> }
>
> fn main() {
>     for mot in ["Kayak", "Rust", "été"] {
>         println!("{mot} : {}", est_palindrome(mot));
>     }
> }
> ```
>
> ```sortie
> Kayak : true
> Rust : false
> été : true
> ```

### Exercice 8 : Compter les mots

Niveau : ★★☆

Comptez combien de fois chaque mot apparaît dans `"un deux un trois deux un"`.
Affichez les mots par ordre alphabétique.

> **Indice :** utilisez une `HashMap` et `entry(mot).or_insert(0)`. L'ordre d'une
> `HashMap` est aléatoire : rangez les paires dans un `Vec` et triez-le.

> **Solution**
>
> ```rust
> use std::collections::HashMap;
>
> fn main() {
>     let texte = "un deux un trois deux un";
>     let mut compte = HashMap::new();
>     for mot in texte.split_whitespace() {
>         *compte.entry(mot).or_insert(0) += 1;
>     }
>     let mut paires: Vec<_> = compte.into_iter().collect();
>     paires.sort();
>     for (mot, n) in paires {
>         println!("{mot} : {n}");
>     }
> }
> ```
>
> ```sortie
> deux : 2
> trois : 1
> un : 3
> ```

## Possession, types et erreurs

### Exercice 9 : Emprunter au lieu de prendre

Niveau : ★★☆

Ce programme ne compile pas, car `nom` est déplacé dans la fonction :

```rust norun
fn saluer(nom: String) {
    println!("Bonjour {nom}");
}

fn main() {
    let nom = String::from("Inès");
    saluer(nom);
    saluer(nom); // erreur : use of moved value
}
```

Corrigez-le en **empruntant** le texte, sans utiliser `clone`.

> **Indice :** le paramètre devient `&str`, et l'appel `saluer(&nom)`.

> **Solution**
>
> ```rust
> fn saluer(nom: &str) {
>     println!("Bonjour {nom}");
> }
>
> fn main() {
>     let nom = String::from("Inès");
>     saluer(&nom);
>     saluer(&nom);
> }
> ```
>
> ```sortie
> Bonjour Inès
> Bonjour Inès
> ```

### Exercice 10 : Un compte bancaire

Niveau : ★★☆

Créez une structure `Compte` avec un champ `solde: i32`, et deux méthodes :
`deposer(&mut self, montant: i32)` et `afficher(&self)`. Déposez 50 puis 20 sur un
compte qui commence à 0.

> **Solution**
>
> ```rust
> struct Compte {
>     solde: i32,
> }
>
> impl Compte {
>     fn deposer(&mut self, montant: i32) {
>         self.solde += montant;
>     }
>
>     fn afficher(&self) {
>         println!("Solde : {} €", self.solde);
>     }
> }
>
> fn main() {
>     let mut compte = Compte { solde: 0 };
>     compte.deposer(50);
>     compte.deposer(20);
>     compte.afficher();
> }
> ```
>
> ```sortie
> Solde : 70 €
> ```

### Exercice 11 : Une division sûre

Niveau : ★★★

Écrivez `diviser(a: i32, b: i32) -> Option<i32>` qui renvoie `None` si `b` vaut 0.
Puis écrivez `lire_nombre(texte: &str) -> Result<i32, String>` qui renvoie un message
d'erreur clair si le texte n'est pas un nombre. Testez les deux.

> **Indice :** pour `lire_nombre`, utilisez `match texte.parse::<i32>()` avec un cas
> `Ok(n)` et un cas `Err(_)`.

> **Solution**
>
> ```rust
> fn diviser(a: i32, b: i32) -> Option<i32> {
>     if b == 0 { None } else { Some(a / b) }
> }
>
> fn lire_nombre(texte: &str) -> Result<i32, String> {
>     match texte.parse::<i32>() {
>         Ok(n) => Ok(n),
>         Err(_) => Err(format!("« {texte} » n'est pas un nombre")),
>     }
> }
>
> fn main() {
>     println!("{:?}", diviser(10, 2));
>     println!("{:?}", diviser(1, 0));
>     match lire_nombre("12") {
>         Ok(n) => println!("Lu : {n}"),
>         Err(e) => println!("Erreur : {e}"),
>     }
>     match lire_nombre("douze") {
>         Ok(n) => println!("Lu : {n}"),
>         Err(e) => println!("Erreur : {e}"),
>     }
> }
> ```
>
> ```sortie
> Some(5)
> None
> Lu : 12
> Erreur : « douze » n'est pas un nombre
> ```

### Exercice 12 : Des formes avec un trait

Niveau : ★★★

Créez un trait `Forme` avec une méthode `aire(&self) -> f64`. Implémentez-le pour
`Carre { cote: f64 }` et `Rectangle { largeur: f64, hauteur: f64 }`. Rangez un carré
de côté 3 et un rectangle de 2 sur 5 dans un `Vec<Box<dyn Forme>>`, puis affichez la
somme des aires.

> **Indice :** relisez [Une liste d'objets différents](generiques.md#une-liste-d-objets-differents).

> **Solution**
>
> ```rust
> trait Forme {
>     fn aire(&self) -> f64;
> }
>
> struct Carre {
>     cote: f64,
> }
>
> struct Rectangle {
>     largeur: f64,
>     hauteur: f64,
> }
>
> impl Forme for Carre {
>     fn aire(&self) -> f64 {
>         self.cote * self.cote
>     }
> }
>
> impl Forme for Rectangle {
>     fn aire(&self) -> f64 {
>         self.largeur * self.hauteur
>     }
> }
>
> fn main() {
>     let formes: Vec<Box<dyn Forme>> = vec![
>         Box::new(Carre { cote: 3.0 }),
>         Box::new(Rectangle { largeur: 2.0, hauteur: 5.0 }),
>     ];
>     let mut total = 0.0;
>     for f in &formes {
>         println!("Aire : {}", f.aire());
>         total += f.aire();
>     }
>     println!("Total : {total}");
> }
> ```
>
> ```sortie
> Aire : 9
> Aire : 10
> Total : 19
> ```

## En résumé

- Commencez par les exercices ★☆☆, puis montez en difficulté.
- Lisez les messages du compilateur : ils indiquent souvent la correction.
- Testez votre code avec plusieurs valeurs, y compris les cas limites (0, texte vide).
- Pour revoir une notion, l'[aide-mémoire](aide-memoire.md) résume tout le parcours.

Pour continuer, essayez les mêmes exercices en [Ruby](../ruby/exercices.md) ou en
[Rythme](../rythme/exercices.md).

Suite : [Exercices corrigés : deuxième série](exercices-2.md)
