# Exercices corrigés : deuxième série

Cette page propose douze nouveaux exercices Rust, du plus facile au plus difficile, chacun avec un indice, une solution expliquée et souvent un tableau qui suit le programme pas à pas.

## Comment travailler

Ces exercices sont **nouveaux** : ils ne reprennent pas ceux des
[Exercices pour bien démarrer](exercices-faciles.md) ni des
[Exercices corrigés](exercices.md). Faites-les dans un projet créé avec
`cargo new exercices2`, et lancez votre code avec `cargo run`.

1. Lisez l'énoncé et la sortie attendue.
2. Sur papier, écrivez les étapes en français. Faites un petit tableau de trace, comme
   dans [Suivre un programme pas à pas](pas-a-pas.md).
3. Écrivez le code, lancez-le, comparez la sortie caractère par caractère.
4. Bloqué ? Lisez l'indice. Regardez la solution en dernier.

Les niveaux :

- ★☆☆ : facile, quelques lignes ;
- ★★☆ : moyen, il faut combiner plusieurs notions ;
- ★★★ : plus dur, prenez votre temps et faites le tableau sur papier.

> **Astuce :** votre solution peut être différente et tout aussi juste. Si elle compile
> et affiche la bonne sortie, c'est gagné.

## Pour s'échauffer

### Exercice 1 : Le compte à rebours

Niveau : ★☆☆

Affichez un compte à rebours de 5 à 1, puis `Décollage !`.

Sortie attendue :

```text
5
4
3
2
1
Décollage !
```

> **Indice :** `(1..=5).rev()` donne les nombres de 1 à 5 **à l'envers** : 5, 4, 3, 2, 1.
> N'oubliez pas les parenthèses autour de `1..=5`.

> **Solution**
>
> ```rust
> fn main() {
>     for n in (1..=5).rev() {
>         println!("{n}");
>     }
>     println!("Décollage !");
> }
> ```
>
> ```sortie
> 5
> 4
> 3
> 2
> 1
> Décollage !
> ```
>
> | Tour | n | Affiche |
> |------|---|---------|
> | 1 | 5 | 5 |
> | 2 | 4 | 4 |
> | 3 | 3 | 3 |
> | 4 | 2 | 2 |
> | 5 | 1 | 1 |
> | après | | Décollage ! |
>
> Pourquoi ça marche : `.rev()` retourne l'intervalle, comme on lit une liste en
> partant du bas. Le dernier `println!` est **après** la boucle : il ne s'exécute
> qu'une fois.

### Exercice 2 : Le plus grand des trois

Niveau : ★☆☆

Écrivez une fonction `plus_grand(a: i32, b: i32, c: i32) -> i32` qui renvoie le plus
grand des trois nombres, **sans** utiliser `max`. Testez-la avec `(4, 9, 2)` et
`(7, 3, 7)`.

Sortie attendue :

```text
9
7
```

> **Indice :** gardez un « champion » : `let mut grand = a;`. Si `b` est plus grand,
> `b` devient le champion. Puis pareil avec `c`.

> **Solution**
>
> ```rust
> fn plus_grand(a: i32, b: i32, c: i32) -> i32 {
>     let mut grand = a;
>     if b > grand {
>         grand = b;
>     }
>     if c > grand {
>         grand = c;
>     }
>     grand
> }
>
> fn main() {
>     println!("{}", plus_grand(4, 9, 2));
>     println!("{}", plus_grand(7, 3, 7));
> }
> ```
>
> ```sortie
> 9
> 7
> ```
>
> Pourquoi ça marche : c'est comme un tournoi. Le champion de départ est `a`, et
> chaque nouveau venu le défie. Pour `(4, 9, 2)` : `grand` vaut 4, puis 9 (9 > 4),
> puis reste 9 (2 n'est pas plus grand que 9).

### Exercice 3 : Fizz et Buzz

Niveau : ★☆☆

Pour chaque nombre de 1 à 15, affichez `Fizz` s'il est divisible par 3, `Buzz` s'il
est divisible par 5, `FizzBuzz` s'il est divisible par les deux, et sinon le nombre.
Mettez tout sur une seule ligne, séparé par des espaces.

Sortie attendue :

```text
1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
```

> **Indice :** « divisible par 3 » s'écrit `n % 3 == 0`. Testez **d'abord** le cas
> `n % 15 == 0` : sinon, 15 serait attrapé par le cas « divisible par 3 ».

> **Indice :** `print!` (sans `ln`) affiche sans passer à la ligne. Ajoutez un
> `println!()` vide à la fin.

> **Solution**
>
> ```rust
> fn main() {
>     for n in 1..=15 {
>         if n % 15 == 0 {
>             print!("FizzBuzz");
>         } else if n % 3 == 0 {
>             print!("Fizz");
>         } else if n % 5 == 0 {
>             print!("Buzz");
>         } else {
>             print!("{n}");
>         }
>         if n < 15 {
>             print!(" ");
>         }
>     }
>     println!();
> }
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```
>
> Pourquoi ça marche : un `if … else if` s'arrête au **premier** test vrai. Le cas le
> plus précis (15) doit donc venir en premier. Le test `n < 15` évite un espace en
> trop à la fin de la ligne.

### Exercice 4 : Les initiales

Niveau : ★☆☆

Écrivez une fonction `initiales(nom: &str) -> String` qui renvoie la première lettre de
chaque mot, suivie d'un point. Testez avec `"Ada Lovelace"` et
`"Grace Brewster Hopper"`.

Sortie attendue :

```text
A.L.
G.B.H.
```

> **Indice :** `nom.split_whitespace()` découpe le texte en mots.
> `mot.chars().next()` donne la première lettre, dans une `Option`.

> **Indice :** partez de `let mut resultat = String::new();`, puis, pour chaque mot,
> `resultat.push(lettre);` et `resultat.push('.');`.

> **Solution**
>
> ```rust
> fn initiales(nom: &str) -> String {
>     let mut resultat = String::new();
>     for mot in nom.split_whitespace() {
>         if let Some(lettre) = mot.chars().next() {
>             resultat.push(lettre);
>             resultat.push('.');
>         }
>     }
>     resultat
> }
>
> fn main() {
>     println!("{}", initiales("Ada Lovelace"));
>     println!("{}", initiales("Grace Brewster Hopper"));
> }
> ```
>
> ```sortie
> A.L.
> G.B.H.
> ```
>
> Pourquoi ça marche : `chars().next()` renvoie `Some(lettre)` s'il y a une lettre,
> `None` si le mot est vide. `if let Some(lettre)` ne garde que le premier cas.
> `push` ajoute **un** caractère (entre `'…'`), `push_str` ajoute un texte.

## Combiner les notions

### Exercice 5 : La somme des chiffres

Niveau : ★★☆

Écrivez une fonction `somme_chiffres(n: u32) -> u32` qui additionne les chiffres d'un
nombre : pour 4721, cela fait 4 + 7 + 2 + 1 = 14. Ne passez pas par du texte.

Sortie attendue :

```text
14
9
```

(la deuxième ligne est pour `somme_chiffres(900)`).

> **Indice :** `n % 10` donne le **dernier** chiffre (4721 % 10 = 1). `n / 10` enlève
> ce dernier chiffre (4721 / 10 = 472).

> **Indice :** répétez avec `while n > 0 { … }`. Le paramètre doit changer : écrivez
> `mut n: u32` dans la signature.

> **Solution**
>
> ```rust
> fn somme_chiffres(mut n: u32) -> u32 {
>     let mut somme = 0;
>     while n > 0 {
>         somme += n % 10;
>         n /= 10;
>     }
>     somme
> }
>
> fn main() {
>     println!("{}", somme_chiffres(4721));
>     println!("{}", somme_chiffres(900));
> }
> ```
>
> ```sortie
> 14
> 9
> ```
>
> | Tour | n au début | n % 10 | somme | n à la fin |
> |------|-----------|--------|-------|------------|
> | 1 | 4721 | 1 | 1 | 472 |
> | 2 | 472 | 2 | 3 | 47 |
> | 3 | 47 | 7 | 10 | 4 |
> | 4 | 4 | 4 | 14 | 0 |
>
> Pourquoi ça marche : on « mange » le nombre par la droite, un chiffre par tour.
> Quand `n` vaut 0, il n'y a plus rien à manger : la boucle s'arrête.

### Exercice 6 : Retourner une liste à la main

Niveau : ★★☆

Partez de `let liste = vec![1, 2, 3, 4];`. Construisez une **nouvelle** liste à
l'envers, sans utiliser `reverse` ni `rev`. Affichez les deux listes.

Sortie attendue :

```text
[1, 2, 3, 4]
[4, 3, 2, 1]
```

> **Indice :** créez `let mut envers = Vec::new();`. Pour chaque élément, insérez-le
> **au début** de `envers` : `envers.insert(0, x);`.

> **Solution**
>
> ```rust
> fn main() {
>     let liste = vec![1, 2, 3, 4];
>     let mut envers = Vec::new();
>     for x in &liste {
>         envers.insert(0, *x);
>     }
>     println!("{liste:?}");
>     println!("{envers:?}");
> }
> ```
>
> ```sortie
> [1, 2, 3, 4]
> [4, 3, 2, 1]
> ```
>
> | Tour | x | envers après le tour |
> |------|---|----------------------|
> | 1 | 1 | [1] |
> | 2 | 2 | [2, 1] |
> | 3 | 3 | [3, 2, 1] |
> | 4 | 4 | [4, 3, 2, 1] |
>
> Pourquoi ça marche : c'est comme empiler des assiettes. Chaque nouvelle assiette
> passe devant les autres. `&liste` prête la liste à la boucle : elle reste
> utilisable pour le `println!`. `{:?}` affiche une liste entière.

### Exercice 7 : Le mot le plus long

Niveau : ★★☆

Écrivez une fonction `plus_long(phrase: &str) -> &str` qui renvoie le mot le plus long
d'une phrase. En cas d'égalité, gardez le premier. Testez avec
`"le chat dort sur le canapé"`.

Sortie attendue :

```text
canapé (6 lettres)
```

> **Indice :** gardez un champion, comme à l'exercice 2 : `let mut meilleur = "";`.
> Comparez les longueurs avec `mot.chars().count()`, pas `len()` : `len()` compte les
> octets, et `é` en prend 2.

> **Solution**
>
> ```rust
> fn plus_long(phrase: &str) -> &str {
>     let mut meilleur = "";
>     for mot in phrase.split_whitespace() {
>         if mot.chars().count() > meilleur.chars().count() {
>             meilleur = mot;
>         }
>     }
>     meilleur
> }
>
> fn main() {
>     let mot = plus_long("le chat dort sur le canapé");
>     println!("{mot} ({} lettres)", mot.chars().count());
> }
> ```
>
> ```sortie
> canapé (6 lettres)
> ```
>
> Pourquoi ça marche : le test est `>` et non `>=`, donc un mot de même longueur ne
> remplace pas le champion : le premier gagne. Le mot renvoyé est un morceau de la
> phrase (un `&str`), sans copie.
>
> Essayez : avec `>=`, testez `"chat dort"`. Vous obtenez `dort` au lieu de `chat`.

### Exercice 8 : La suite de Fibonacci

Niveau : ★★☆

Dans la suite de Fibonacci, chaque nombre est la somme des deux précédents :
0, 1, 1, 2, 3, 5… Affichez les 10 premiers nombres sur une ligne.

Sortie attendue :

```text
0 1 1 2 3 5 8 13 21 34
```

> **Indice :** gardez deux variables `a` (le nombre actuel) et `b` (le suivant). À
> chaque tour, affichez `a`, puis avancez : le nouveau `a` est `b`, le nouveau `b` est
> `a + b`.

> **Indice :** attention à l'ordre ! Calculez `let suivant = a + b;` **avant** de
> modifier `a`.

> **Solution**
>
> ```rust
> fn main() {
>     let mut a: u64 = 0;
>     let mut b: u64 = 1;
>     let mut nombres = Vec::new();
>     for _ in 0..10 {
>         nombres.push(a.to_string());
>         let suivant = a + b;
>         a = b;
>         b = suivant;
>     }
>     println!("{}", nombres.join(" "));
> }
> ```
>
> ```sortie
> 0 1 1 2 3 5 8 13 21 34
> ```
>
> | Tour | a (affiché) | b | suivant = a + b | a après | b après |
> |------|-------------|---|-----------------|---------|---------|
> | 1 | 0 | 1 | 1 | 1 | 1 |
> | 2 | 1 | 1 | 2 | 1 | 2 |
> | 3 | 1 | 2 | 3 | 2 | 3 |
> | 4 | 2 | 3 | 5 | 3 | 5 |
> | 5 | 3 | 5 | 8 | 5 | 8 |
>
> Pourquoi ça marche : les deux variables avancent ensemble, comme deux pieds qui
> marchent. `suivant` garde le calcul pendant qu'on déplace `a` : sans lui, on
> perdrait l'ancienne valeur de `a`. `join(" ")` colle les textes avec un espace
> entre eux. Le `_` de `for _ in` veut dire « je n'utilise pas le numéro du tour ».

### Exercice 9 : Les mentions

Niveau : ★★☆

Écrivez une fonction `mention(note: u32) -> &'static str` qui renvoie
`"insuffisant"` (0 à 9), `"assez bien"` (10 à 13), `"bien"` (14 à 15) ou
`"très bien"` (16 à 20), et `"note invalide"` au-delà. Utilisez `match` avec des
intervalles. Affichez la mention de 8, 12, 15, 19 et 25.

Sortie attendue :

```text
8 : insuffisant
12 : assez bien
15 : bien
19 : très bien
25 : note invalide
```

> **Indice :** un `match` accepte des intervalles inclus : `0..=9 => "insuffisant",`.

> **Indice :** `&'static str` veut dire « un texte écrit directement dans le
> programme », qui existe tout le temps. C'est le type de `"bien"`.

> **Solution**
>
> ```rust
> fn mention(note: u32) -> &'static str {
>     match note {
>         0..=9 => "insuffisant",
>         10..=13 => "assez bien",
>         14..=15 => "bien",
>         16..=20 => "très bien",
>         _ => "note invalide",
>     }
> }
>
> fn main() {
>     for note in [8, 12, 15, 19, 25] {
>         println!("{note} : {}", mention(note));
>     }
> }
> ```
>
> ```sortie
> 8 : insuffisant
> 12 : assez bien
> 15 : bien
> 19 : très bien
> 25 : note invalide
> ```
>
> Pourquoi ça marche : `match` essaie les cas de haut en bas, comme des boîtes aux
> lettres numérotées. `_` attrape tout le reste : Rust l'exige, car un `u32` peut
> aller bien au-delà de 20.

## Pour aller plus loin

### Exercice 10 : Le code de César

Niveau : ★★★

Le code de César décale chaque lettre de 3 dans l'alphabet : `a` devient `d`, `b`
devient `e`… et `x`, `y`, `z` reviennent au début (`a`, `b`, `c`). Écrivez
`cesar(texte: &str) -> String` pour des lettres minuscules sans accents ; les autres
caractères (espaces…) ne changent pas.

Sortie attendue :

```text
bonjour zoe -> erqmrxu crh
```

> **Indice :** un caractère peut devenir un nombre : `c as u8`. `'a' as u8` vaut 97.
> La position dans l'alphabet est donc `c as u8 - b'a'` (0 pour `a`, 25 pour `z`).

> **Indice :** décalez puis revenez au début avec `%` :
> `(position + 3) % 26`. Puis remettez en lettre : `(b'a' + nouvelle) as char`.

> **Solution**
>
> ```rust
> fn cesar(texte: &str) -> String {
>     let mut resultat = String::new();
>     for c in texte.chars() {
>         if c.is_ascii_lowercase() {
>             let position = c as u8 - b'a';
>             let nouvelle = (position + 3) % 26;
>             resultat.push((b'a' + nouvelle) as char);
>         } else {
>             resultat.push(c);
>         }
>     }
>     resultat
> }
>
> fn main() {
>     let message = "bonjour zoe";
>     println!("{message} -> {}", cesar(message));
> }
> ```
>
> ```sortie
> bonjour zoe -> erqmrxu crh
> ```
>
> | c | position | position + 3 | % 26 | lettre |
> |---|----------|--------------|------|--------|
> | b | 1 | 4 | 4 | e |
> | o | 14 | 17 | 17 | r |
> | z | 25 | 28 | 2 | c |
> | (espace) | | | | (espace) |
>
> Pourquoi ça marche : l'alphabet est comme une horloge à 26 heures. Après 25, on
> repart à 0 : c'est le rôle de `% 26`. `b'a'` est la lettre `a` sous forme d'octet
> (`u8`), pour calculer avec.
>
> Essayez : décalez de 23 au lieu de 3. Appliqué au texte codé, cela le décode : vous
> retrouvez `bonjour zoe`.

### Exercice 11 : Les nombres premiers

Niveau : ★★★

Un nombre premier n'est divisible que par 1 et par lui-même (2, 3, 5, 7…). Écrivez
`est_premier(n: u32) -> bool`, puis affichez les nombres premiers jusqu'à 30.

Sortie attendue :

```text
2 3 5 7 11 13 17 19 23 29
```

> **Indice :** 0 et 1 ne sont pas premiers. Pour les autres, essayez chaque diviseur
> `d` de 2 à `n - 1` : si `n % d == 0`, ce n'est pas premier.

> **Indice :** dès qu'un diviseur est trouvé, `return false;` sort tout de suite de la
> fonction. Si la boucle se termine sans rien trouver, c'est premier.

> **Solution**
>
> ```rust
> fn est_premier(n: u32) -> bool {
>     if n < 2 {
>         return false;
>     }
>     let mut d = 2;
>     while d * d <= n {
>         if n % d == 0 {
>             return false;
>         }
>         d += 1;
>     }
>     true
> }
>
> fn main() {
>     let mut premiers = Vec::new();
>     for n in 1..=30 {
>         if est_premier(n) {
>             premiers.push(n.to_string());
>         }
>     }
>     println!("{}", premiers.join(" "));
> }
> ```
>
> ```sortie
> 2 3 5 7 11 13 17 19 23 29
> ```
>
> Trace de `est_premier(9)` puis de `est_premier(11)` :
>
> | Appel | d | d × d <= n ? | n % d | Résultat |
> |-------|---|--------------|-------|----------|
> | 9 | 2 | 4 <= 9 : oui | 1 | on continue |
> | 9 | 3 | 9 <= 9 : oui | **0** | `false` |
> | 11 | 2 | 4 <= 11 : oui | 1 | on continue |
> | 11 | 3 | 9 <= 11 : oui | 2 | on continue |
> | 11 | 4 | 16 <= 11 : **non** | | `true` |
>
> Pourquoi ça marche : la solution s'arrête quand `d × d` dépasse `n`. Si `n` avait un
> diviseur plus grand, il aurait aussi un « partenaire » plus petit, déjà testé
> (pour 30 : 10 × 3, et 3 a été vu avant 10). Votre version jusqu'à `n - 1` est juste
> aussi, simplement plus lente.

### Exercice 12 : Compresser un texte

Niveau : ★★★

Écrivez `compresser(texte: &str) -> String` qui remplace chaque suite de lettres
identiques par la lettre suivie du nombre de répétitions : `"aaabccdddd"` devient
`"a3b1c2d4"`.

Sortie attendue :

```text
aaabccdddd -> a3b1c2d4
zzz -> z3
```

> **Indice :** gardez la lettre en cours (`courante`) et son compteur (`compte`). Si la
> lettre suivante est la même, `compte += 1`. Sinon, écrivez la lettre et son compte,
> puis recommencez avec la nouvelle lettre.

> **Indice :** après la boucle, il reste la **dernière** suite à écrire. C'est l'oubli
> le plus courant !

> **Solution**
>
> ```rust
> fn compresser(texte: &str) -> String {
>     let mut resultat = String::new();
>     let mut lettres = texte.chars();
>     let Some(mut courante) = lettres.next() else {
>         return resultat;
>     };
>     let mut compte = 1;
>     for c in lettres {
>         if c == courante {
>             compte += 1;
>         } else {
>             resultat.push_str(&format!("{courante}{compte}"));
>             courante = c;
>             compte = 1;
>         }
>     }
>     resultat.push_str(&format!("{courante}{compte}"));
>     resultat
> }
>
> fn main() {
>     for texte in ["aaabccdddd", "zzz"] {
>         println!("{texte} -> {}", compresser(texte));
>     }
> }
> ```
>
> ```sortie
> aaabccdddd -> a3b1c2d4
> zzz -> z3
> ```
>
> | c | c == courante ? | courante | compte | resultat |
> |---|-----------------|----------|--------|----------|
> | (départ) | | a | 1 | |
> | a | oui | a | 2 | |
> | a | oui | a | 3 | |
> | b | non | b | 1 | a3 |
> | c | non | c | 1 | a3b1 |
> | c | oui | c | 2 | a3b1 |
> | d | non | d | 1 | a3b1c2 |
> | d | oui | d | 2 | a3b1c2 |
> | d | oui | d | 3 | a3b1c2 |
> | d | oui | d | 4 | a3b1c2 |
> | (après la boucle) | | | | a3b1c2d4 |
>
> Pourquoi ça marche : la première lettre est lue à part avec `lettres.next()`. Si le
> texte est vide, `let … else` renvoie tout de suite un texte vide. Ensuite, une suite
> n'est écrite que quand une **autre** lettre arrive : la dernière suite, qui n'est
> suivie de rien, est écrite après la boucle.
>
> Essayez : supprimez la ligne `resultat.push_str(…)` placée après la boucle. Vous
> obtenez `a3b1c2` et un texte vide pour `zzz` : la dernière suite est perdue.

## En résumé

- Faites un tableau de trace sur papier avant d'écrire le code : il montre les étapes.
- Le « champion » (exercices 2 et 7) et l'accumulateur (exercices 5 et 8) sont deux
  idées qui reviennent partout.
- Pensez aux cas limites : texte vide, égalité, dernière suite, nombre 0 ou 1.
- Pour revoir une notion, l'[aide-mémoire](aide-memoire.md) résume tout le parcours.

Suite : [Aide-mémoire](aide-memoire.md)
