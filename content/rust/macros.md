# Les macros

Une macro est du code qui écrit du code : vous en utilisez depuis le premier jour avec `println!`, et ce chapitre vous apprend à écrire les vôtres avec `macro_rules!`.

> **Objectifs :**
>
> - reconnaître et utiliser les macros courantes (`println!`, `vec!`, `format!`, `assert!`) ;
> - comprendre à quoi sert une macro, et en quoi elle diffère d'une fonction ;
> - écrire une macro simple avec `macro_rules!` ;
> - accepter un nombre variable d'arguments avec `$(…),*` ;
> - savoir ce que sont les macros `derive` et les attributs.

## Les macros que vous connaissez déjà

Une [macro](../debuter/glossaire.md#macro) se reconnaît à son point d'exclamation `!`.
Vous en utilisez beaucoup :

```rust
let nom = "Léa";
let notes = vec![12, 15, 9];               // fabrique un Vec
let phrase = format!("{nom} a {} notes", notes.len()); // fabrique une String
println!("{phrase}");                      // affiche avec un retour à la ligne
print!("sans retour… ");                   // affiche sans retour à la ligne
println!("suite");
assert!(notes.len() == 3);                 // arrête le programme si c'est faux
assert_eq!(notes[0], 12);                  // compare deux valeurs
println!("Les vérifications sont passées.");
```

```sortie
Léa a 3 notes
sans retour… suite
Les vérifications sont passées.
```

D'autres macros utiles : `eprintln!` (affiche sur la sortie d'erreur), `panic!` (arrête
le programme), `todo!` (« pas encore écrit »), `dbg!` (affiche une valeur pour
[déboguer](debogage.md)).

## Pourquoi des macros ?

Une macro est développée **avant la compilation** : le compilateur remplace l'appel par
le code qu'elle produit. C'est comme un tampon encreur : vous tamponnez une ligne, et
c'est tout un motif qui apparaît sur la page.

Elle permet deux choses qu'une fonction ne sait pas faire :

- **un nombre variable d'arguments** : `println!("{}", a)` et
  `println!("{} {} {}", a, b, c)` sont tous les deux valides. Une fonction Rust a
  toujours le même nombre de paramètres ;
- **générer du code** : `vec![1, 2, 3]` devient un `Vec` créé puis rempli, et
  `println!` vérifie **à la compilation** que le texte entre accolades correspond aux
  valeurs données.

> **Note :** préférez une fonction quand elle suffit. Une macro est plus difficile à
> lire et à déboguer. On l'écrit pour éviter de répéter du code qu'une fonction ne
> pourrait pas factoriser.

## Une première macro avec macro_rules!

`macro_rules!` définit une macro. Elle contient des **règles** : un motif à gauche de
`=>`, le code produit à droite. Voici une macro sans argument :

```rust
macro_rules! bonjour {
    () => {
        println!("Bonjour depuis une macro !");
    };
}

fn main() {
    bonjour!();
    bonjour!();
}
```

```sortie
Bonjour depuis une macro !
Bonjour depuis une macro !
```

`()` est le motif : « aucun argument ». Chaque `bonjour!()` est remplacé par le
`println!`.

> **Attention :** une macro doit être définie **avant** l'endroit où on l'utilise dans
> le fichier, contrairement aux fonctions.

## Des arguments : $x:expr

Dans le motif, un argument s'écrit `$nom:sorte`. La sorte la plus courante est `expr`
(une expression : `3`, `a + b`, `f(x)`…).

```rust
macro_rules! carre {
    ($x:expr) => {
        $x * $x
    };
}

fn main() {
    println!("{}", carre!(7));
    let a = 2;
    println!("{}", carre!(a + 1));
}
```

```sortie
49
9
```

`carre!(a + 1)` donne bien 9 : Rust garde `a + 1` comme un seul bloc, et calcule
`(a + 1) * (a + 1)`.

Voici les sortes d'arguments les plus utiles :

| Sorte | Ce qu'elle accepte | Exemple |
|-------|--------------------|---------|
| `expr` | une expression | `2 + 3`, `nom.len()` |
| `ident` | un nom (variable, fonction) | `total`, `ma_fonction` |
| `ty` | un type | `i32`, `Vec<String>` |
| `literal` | une valeur écrite telle quelle | `42`, `"texte"` |
| `tt` | un seul « morceau » de code, quel qu'il soit | `+`, `x`, `(a, b)` |

### Plusieurs règles

Une macro peut avoir plusieurs règles. Rust essaie la première, puis la suivante… et
prend la première qui correspond, comme un [`match`](pattern-matching.md) :

```rust
macro_rules! saluer {
    () => {
        println!("Bonjour, inconnu !")
    };
    ($nom:expr) => {
        println!("Bonjour, {} !", $nom)
    };
}

fn main() {
    saluer!();
    saluer!("Inès");
}
```

```sortie
Bonjour, inconnu !
Bonjour, Inès !
```

### Fabriquer une fonction avec ident

Avec `ident`, la macro peut même créer des fonctions :

```rust
macro_rules! fonction_qui_dit {
    ($nom:ident, $texte:literal) => {
        fn $nom() {
            println!($texte);
        }
    };
}

fonction_qui_dit!(miauler, "Miaou");
fonction_qui_dit!(aboyer, "Wouf");

fn main() {
    miauler();
    aboyer();
}
```

```sortie
Miaou
Wouf
```

## Répéter : $(…),*

Pour accepter **autant d'arguments qu'on veut**, entourez le motif de `$( … )`, suivi
d'un séparateur et d'un signe de répétition :

- `$( … ),*` : zéro ou plusieurs fois, séparés par des virgules ;
- `$( … ),+` : une ou plusieurs fois.

Dans le code produit, `$( … )*` répète le morceau une fois par argument reçu.

```rust
macro_rules! somme {
    ($($x:expr),*) => {
        0 $( + $x )*
    };
}

fn main() {
    println!("{}", somme!());
    println!("{}", somme!(1, 2, 3));
    println!("{}", somme!(10, 20, 30, 40));
}
```

```sortie
0
6
100
```

`somme!(1, 2, 3)` devient `0 + 1 + 2 + 3`.

## Exemple complet : une macro hashmap!

Rust a `vec![…]` pour créer un `Vec` rempli, mais rien de pareil pour une
[`HashMap`](hashmap.md). Écrivons-la :

```rust
use std::collections::HashMap;

macro_rules! hashmap {
    ($($cle:expr => $valeur:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $( map.insert($cle, $valeur); )*
        map
    }};
}

fn main() {
    let ages = hashmap! {
        "Léa" => 12,
        "Tom" => 14,
        "Inès" => 13, // la virgule finale est acceptée
    };
    println!("Tom a {} ans", ages["Tom"]);
    println!("{} personnes", ages.len());
}
```

```sortie
Tom a 14 ans
3 personnes
```

Décortiquons le motif `$($cle:expr => $valeur:expr),* $(,)?` :

- `$cle:expr => $valeur:expr` : une paire, avec une flèche `=>` au milieu ;
- `),*` : autant de paires qu'on veut, séparées par des virgules ;
- `$(,)?` : une virgule finale facultative (`?` veut dire « zéro ou une fois »).

Les **doubles** accolades `{{ … }}` produisent un bloc : la macro peut ainsi déclarer une
variable `map` et renvoyer sa valeur, comme une expression.

> **Astuce :** une macro s'appelle avec `( )`, `[ ]` ou `{ }`, au choix. Par habitude :
> `vec![…]` pour une liste, `{ }` pour un bloc, `( )` pour le reste.

## Voir le code produit : cargo expand

Quand une macro ne fait pas ce que vous voulez, regardez le code qu'elle produit avec
l'outil `cargo expand` :

```sh
cargo install cargo-expand
cargo expand
```

Pour l'exemple `hashmap!`, la partie intéressante ressemble à ceci :

```text
fn main() {
    let ages =
        {
            let mut map = HashMap::new();
            map.insert("Léa", 12);
            map.insert("Tom", 14);
            map.insert("Inès", 13);
            map
        };
```

On voit exactement ce que le compilateur reçoit : plus de macro, du code Rust normal.

## Les macros derive et les attributs

Il existe une autre famille : les macros **procédurales**. Vous les avez déjà vues, sous
deux formes :

- `#[derive(Debug, Clone)]` au-dessus d'une structure : la macro **écrit pour vous**
  l'implémentation des traits ;
- les **attributs**, comme `#[test]` ou `#[tokio::main]` (voir
  [l'asynchrone](async.md#un-runtime-tokio)), qui transforment la fonction placée
  dessous.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a.clone();           // grâce à derive(Clone)
    println!("{:?}", b);         // grâce à derive(Debug)
    println!("{}", a == b);      // grâce à derive(PartialEq)
}
```

```sortie
Point { x: 1, y: 2 }
true
```

Écrire une macro procédurale demande un projet à part et des crates comme `syn` et
`quote` : c'est un sujet avancé. Retenez qu'en utiliser une est aussi simple qu'écrire
`#[derive(…)]`.

## Erreurs fréquentes

### Oublier le point d'exclamation

```rust norun
println("Bonjour");
```

```text
error[E0423]: expected function, found macro `println`
 --> main.rs:3:1
  |
3 | println("Bonjour");
  | ^^^^^^^ not a function
  |
help: use `!` to invoke the macro
  |
3 | println!("Bonjour");
  |        +
```

Sans `!`, Rust cherche une fonction nommée `println`. Ajoutez le `!`.

### Utiliser une macro avant de la définir

```rust norun
fn main() {
    double!(4);
}

macro_rules! double {
    ($x:expr) => { $x * 2 };
}
```

```text
error: cannot find macro `double` in this scope
 --> main.rs:3:5
  |
3 |     double!(4);
  |     ^^^^^^ consider moving the definition of `double` before this call
  |
note: a macro with the same name exists, but it appears later
 --> main.rs:6:14
  |
6 | macro_rules! double {
  |              ^^^^^^
```

Les macros `macro_rules!` sont lues de haut en bas. Déplacez la définition au-dessus de
`main`.

### Appeler une macro avec un motif qu'elle ne connaît pas

```rust norun
macro_rules! carre {
    ($x:expr) => { $x * $x };
}

fn main() {
    let a = carre!();
}
```

```text
error: unexpected end of macro invocation
 --> main.rs:7:13
  |
2 | macro_rules! carre {
  | ------------------ when calling this macro
...
7 |     let a = carre!();
  |             ^^^^^^^^ missing tokens in macro arguments
  |
note: while trying to match meta-variable `$x:expr`
 --> main.rs:3:6
  |
3 |     ($x:expr) => { $x * $x };
  |      ^^^^^^^
```

Aucune règle n'accepte zéro argument. Donnez un argument, ou ajoutez une règle `() => …`.

## Testez-vous

1. Pourquoi `println!` est-il une macro et pas une fonction ?

> **Réponse**
>
> Parce qu'il accepte un nombre variable d'arguments, et qu'il vérifie à la compilation
> que les `{}` du texte correspondent aux valeurs données. Une fonction ne sait faire
> ni l'un ni l'autre.

2. Dans `macro_rules!`, que veut dire `$($x:expr),*` ?

> **Réponse**
>
> « Zéro, une ou plusieurs expressions, séparées par des virgules ». Chacune s'appelle
> `$x`, et `$( … )*` dans le code produit répète un morceau pour chacune.

3. Que fait `#[derive(Debug)]` au-dessus d'une structure ?

> **Réponse**
>
> C'est une macro : elle écrit à votre place l'implémentation du trait `Debug`, ce qui
> permet d'afficher la structure avec `{:?}`.

## En résumé

- Une macro se termine par `!` et produit du code avant la compilation.
- Elle accepte un nombre variable d'arguments et peut générer du code.
- `macro_rules!` associe des motifs (`$x:expr`, `$n:ident`…) à du code produit.
- `$( … ),*` répète un motif ; `$(,)?` accepte une virgule finale.
- `cargo expand` montre le code produit ; `#[derive]` et les attributs sont des macros
  procédurales.

Suite : [unsafe et les règles qu'on peut enfreindre](unsafe.md).
