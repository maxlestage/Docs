# Les pièges du débutant en Rust

Cette page rassemble douze pièges où presque tous les débutants en Rust tombent, avec pour chacun ce qui se passe, pourquoi, et comment s'en sortir.

> **Objectifs :**
>
> - reconnaître les erreurs les plus courantes des premières semaines ;
> - lire le message du compilateur sans paniquer ;
> - savoir quelle petite correction appliquer à chaque fois.

## Avant de commencer

Rust a un [compilateur](../debuter/glossaire.md#compilateur) très strict. Il refuse
beaucoup de programmes. C'est frustrant au début. Mais c'est un ami : il vous arrête
**avant** que le programme ne fasse une bêtise.

Chaque piège suit le même chemin :

```text
ce que vous attendez  →  ce qui se passe  →  pourquoi  →  la correction
```

> **À retenir :** un message d'erreur de Rust n'est pas une punition. C'est une
> explication, souvent avec la correction en bas, après `help:`.

## Piège 1 : les variables ne changent pas

### Ce que vous attendez

Vous créez un score à 0, puis vous ajoutez 10. Vous pensez voir `10`.

```rust norun
fn main() {
    let score = 0;
    score = score + 10;
    println!("{score}");
}
```

### Ce qui se passe

```text
error[E0384]: cannot assign twice to immutable variable `score`
 --> src/main.rs:3:5
  |
2 |     let score = 0;
  |         ----- first assignment to `score`
3 |     score = score + 10;
  |     ^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut score = 0;
  |         +++
```

### Pourquoi

En Rust, une variable est [immuable](../debuter/glossaire.md#immuable) par défaut. Elle
ne change plus une fois créée. C'est une étiquette collée sur une boîte fermée.

### La correction

Ajoutez `mut` (pour *mutable*, « modifiable ») :

```rust
fn main() {
    let mut score = 0;
    score = score + 10;
    println!("{score}");
}
```

```sortie
10
```

Essayez : retirez `mut`. Le compilateur vous propose lui-même de le remettre, avec des
`+++` sous l'endroit à compléter.

## Piège 2 : String et &str, deux sortes de texte

### Ce que vous attendez

Vous écrivez un texte entre guillemets dans une variable de type `String`.

```rust norun
fn main() {
    let nom: String = "Zoé";
    println!("{nom}");
}
```

### Ce qui se passe

```text
error[E0308]: mismatched types
 --> src/main.rs:2:23
  |
2 |     let nom: String = "Zoé";
  |              ------   ^^^^^ expected `String`, found `&str`
  |              |
  |              expected due to this
  |
help: try using a conversion method
  |
2 |     let nom: String = "Zoé".to_string();
  |                            ++++++++++++
```

### Pourquoi

Un texte entre guillemets est un `&str` : un texte **écrit dans le programme**, qu'on
lit seulement. Une `String` est un texte **à vous**, qui peut grandir. C'est la
différence entre une affiche collée au mur et une feuille dans votre cahier.

| Type | Image | Peut grandir ? |
|------|-------|----------------|
| `&str` | une affiche qu'on regarde | non |
| `String` | une feuille à vous | oui |

### La correction

Transformez l'affiche en feuille avec `String::from` ou `.to_string()` :

```rust
fn main() {
    let nom: String = String::from("Zoé");
    let ville = "Lyon".to_string();
    println!("{nom} habite à {ville}");
}
```

```sortie
Zoé habite à Lyon
```

Tout est expliqué dans [Travailler avec du texte](texte.md).

## Piège 3 : utiliser une valeur après l'avoir donnée

### Ce que vous attendez

Vous copiez un texte dans une autre variable, puis vous affichez les deux.

```rust norun
fn main() {
    let a = String::from("bonjour");
    let b = a;
    println!("{a} {b}");
}
```

### Ce qui se passe

```text
error[E0382]: borrow of moved value: `a`
 --> src/main.rs:4:16
  |
2 |     let a = String::from("bonjour");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
3 |     let b = a;
  |             - value moved here
4 |     println!("{a} {b}");
  |                ^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let b = a.clone();
  |              ++++++++
```

### Pourquoi

`let b = a;` ne copie pas la `String`. Il la **donne** à `b`. C'est comme donner un
livre : vous ne l'avez plus. On dit que la valeur est « déplacée » (*moved*).

```text
avant :  a ──► "bonjour"
après :  a     (vide)
         b ──► "bonjour"
```

C'est la règle de [possession](../debuter/glossaire.md#possession). Pas de panique :
elle a son propre chapitre, [Possession et emprunt](possession.md). Pour l'instant,
retenez seulement la correction.

### La correction

Si vous voulez vraiment deux textes, faites une copie avec `.clone()` :

```rust
fn main() {
    let a = String::from("bonjour");
    let b = a.clone();
    println!("{a} {b}");
}
```

```sortie
bonjour bonjour
```

> **Note :** les nombres (`i32`, `f64`…) et `bool` ne sont pas concernés. Ils sont
> petits, Rust les copie tout seul. `let y = x;` garde `x` utilisable.

## Piège 4 : un nombre qui déborde

### Ce que vous attendez

Un `u8` va de 0 à 255. Vous partez de 250 et ajoutez 1, dix fois. Vous pensez voir
`260`.

```rust norun
fn main() {
    let mut niveau: u8 = 250;
    for _ in 0..10 {
        niveau += 1;
    }
    println!("{niveau}");
}
```

### Ce qui se passe

Le programme compile, puis s'arrête net pendant l'exécution :

```text
thread 'main' (2045) panicked at src/main.rs:4:9:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Pourquoi

Une variable `u8` est une boîte qui ne peut pas contenir plus que 255. Arrivé à 256,
le nombre « déborde » (*overflow*). En mode développement, Rust préfère s'arrêter : on
appelle cela une [panique](../debuter/glossaire.md#panique).

| Tour | niveau avant | niveau après |
|------|--------------|--------------|
| 1 | 250 | 251 |
| … | … | … |
| 5 | 254 | 255 |
| 6 | 255 | débordement : panique |

### La correction

Choisissez un type assez grand. `i32` va jusqu'à plus de deux milliards :

```rust
fn main() {
    let mut niveau: i32 = 250;
    for _ in 0..10 {
        niveau += 1;
    }
    println!("{niveau}");
}
```

```sortie
260
```

Pour plus de détails, voyez [Le dépassement de capacité](nombres.md#le-depassement-de-capacite).

## Piège 5 : 7 divisé par 2 donne 3

### Ce que vous attendez

Vous partagez 7 bonbons entre 2 enfants. Vous pensez voir `3.5`.

```rust
fn main() {
    let bonbons = 7;
    let enfants = 2;
    println!("{}", bonbons / enfants);
    println!("{}", bonbons % enfants);
}
```

```sortie
3
1
```

### Pourquoi

`7` et `2` sont des [entiers](../debuter/glossaire.md#entier). Entre deux entiers, `/`
fait une division **entière** : il garde la partie avant la virgule. `%` donne le
reste. C'est exactement le partage de bonbons : 3 chacun, il en reste 1.

### La correction

Pour un résultat à virgule, travaillez avec des `f64`, en écrivant `7.0` et `2.0` :

```rust
fn main() {
    let bonbons = 7.0;
    let enfants = 2.0;
    println!("{}", bonbons / enfants);
}
```

```sortie
3.5
```

Essayez : écrivez `7.0 / 2`. Rust refuse de mélanger un `f64` et un entier. Écrivez
`7.0 / 2.0`.

## Piège 6 : le point-virgule de trop

### Ce que vous attendez

Votre fonction calcule le double d'un nombre. Vous pensez voir `8`.

```rust norun
fn double(x: i32) -> i32 {
    x * 2;
}

fn main() {
    println!("{}", double(4));
}
```

### Ce qui se passe

```text
error[E0308]: mismatched types
 --> src/main.rs:1:22
  |
1 | fn double(x: i32) -> i32 {
  |    ------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     x * 2;
  |          - help: remove this semicolon to return this value
```

### Pourquoi

En Rust, la dernière ligne **sans** point-virgule est la valeur renvoyée. Avec un
point-virgule, la ligne devient une simple instruction : elle calcule, puis jette le
résultat. La fonction ne renvoie alors « rien », qui s'écrit `()`.

```text
x * 2     →  une valeur : 8        (renvoyée)
x * 2;    →  une instruction       (le 8 est jeté, il reste ())
```

### La correction

Retirez le point-virgule de la dernière ligne :

```rust
fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    println!("{}", double(4));
}
```

```sortie
8
```

Voir aussi [Expressions et instructions](fonctions.md#expressions-et-instructions).

## Piège 7 : lire une lettre avec [0]

### Ce que vous attendez

Vous voulez la première lettre de « chat ». Vous pensez voir `c`.

```rust norun
fn main() {
    let mot = String::from("chat");
    let premiere = mot[0];
    println!("{premiere}");
}
```

### Ce qui se passe

```text
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> src/main.rs:3:24
  |
3 |     let premiere = mot[0];
  |                        ^ string indices are ranges of `usize`
  |
  = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
  = note: you can use `.chars().nth()` or `.bytes().nth()`
```

### Pourquoi

Rust range le texte en [Unicode](../debuter/glossaire.md#unicode). Une lettre comme `é`
prend 2 cases en mémoire, `a` n'en prend qu'une. La case 0 n'est donc pas toujours une
lettre entière. Rust préfère refuser plutôt que de vous donner une demi-lettre.

### La correction

Parcourez les lettres avec `.chars()`, puis prenez celle qui vous intéresse :

```rust
fn main() {
    let mot = String::from("été");
    let premiere = mot.chars().next();
    let troisieme = mot.chars().nth(2);
    println!("{:?} {:?}", premiere, troisieme);
}
```

```sortie
Some('é') Some('é')
```

Le résultat est une [Option](../debuter/glossaire.md#option) : le mot pourrait être
trop court. Voir [Parcourir les caractères](texte.md#parcourir-les-caracteres).

## Piège 8 : comparer deux sortes d'entiers

### Ce que vous attendez

Vous comparez la longueur d'une liste avec un nombre. Vous pensez voir `grand`.

```rust norun
fn main() {
    let v = vec![10, 20, 30];
    let limite: i32 = 2;
    if v.len() > limite {
        println!("grand");
    }
}
```

### Ce qui se passe

```text
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     if v.len() > limite {
  |        -------   ^^^^^^ expected `usize`, found `i32`
  |        |
  |        expected because this is `usize`
```

### Pourquoi

`len()` renvoie un `usize` : le type des tailles et des positions. `limite` est un
`i32`. Pour Rust, ce sont deux [types](../debuter/glossaire.md#type) différents, même
s'ils contiennent tous les deux des nombres entiers. Il ne les mélange jamais tout
seul.

### La correction

Le plus simple : donnez le bon type à `limite` dès le départ.

```rust
fn main() {
    let v = vec![10, 20, 30];
    let limite: usize = 2;
    if v.len() > limite {
        println!("grand");
    }
}
```

```sortie
grand
```

> **Astuce :** une variable qui sert de position ou de taille dans une liste doit être
> un `usize`.

## Piège 9 : unwrap sur une boîte vide

### Ce que vous attendez

Vous demandez le premier élément d'une liste, et vous l'ouvrez avec `unwrap()`.

```rust norun
fn main() {
    let v: Vec<i32> = Vec::new();
    let premier = v.first().unwrap();
    println!("{premier}");
}
```

### Ce qui se passe

```text
thread 'main' (2158) panicked at src/main.rs:3:29:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Pourquoi

`first()` renvoie une `Option` : une boîte qui contient soit `Some(valeur)`, soit
`None` (rien). La liste est vide, donc la boîte aussi. `unwrap()` veut dire « ouvre la
boîte, et s'il n'y a rien, arrête tout ».

### La correction

Prévoyez le cas vide, avec `match` ou avec `unwrap_or` :

```rust
fn main() {
    let v: Vec<i32> = Vec::new();
    match v.first() {
        Some(x) => println!("premier : {x}"),
        None => println!("la liste est vide"),
    }
    let premier = v.first().copied().unwrap_or(0);
    println!("{premier}");
}
```

```sortie
la liste est vide
0
```

> **À retenir :** `unwrap()` est pratique pour essayer. Dans un vrai programme, chaque
> `unwrap()` est une question : « suis-je **sûr** que la boîte n'est pas vide ? »

## Piège 10 : masquer au lieu de modifier

### Ce que vous attendez

Vous ajoutez 1, 2 puis 3 au total dans une boucle. Vous pensez voir `6` à la fin.

```rust
fn main() {
    let total = 0;
    for n in 1..=3 {
        let total = total + n;
        println!("dans la boucle : {total}");
    }
    println!("à la fin : {total}");
}
```

```sortie
dans la boucle : 1
dans la boucle : 2
dans la boucle : 3
à la fin : 0
```

### Pourquoi

`let total = …` dans la boucle crée une **nouvelle** variable, qui porte le même nom.
On appelle cela le **masquage** (*shadowing*). Elle cache l'ancienne, puis disparaît à
la fin du tour. L'ancien `total` n'a jamais bougé.

| Tour | n | total de la boucle | total du dehors |
|------|---|--------------------|-----------------|
| 1 | 1 | 0 + 1 = 1 | 0 |
| 2 | 2 | 0 + 2 = 2 | 0 |
| 3 | 3 | 0 + 3 = 3 | 0 |

### La correction

Pour modifier la même variable : `let mut` une seule fois, puis **pas** de `let`
dans la boucle.

```rust
fn main() {
    let mut total = 0;
    for n in 1..=3 {
        total = total + n;
    }
    println!("à la fin : {total}");
}
```

```sortie
à la fin : 6
```

> **À retenir :** `let` crée une nouvelle variable. `x = …` (sans `let`) modifie
> celle qui existe.

## Piège 11 : une case qui n'existe pas

### Ce que vous attendez

Une liste a 3 notes. Vous voulez la troisième, alors vous écrivez `notes[3]`.

```rust norun
fn main() {
    let notes = vec![12, 15, 9];
    println!("{}", notes[3]);
}
```

### Ce qui se passe

```text
thread 'main' (2246) panicked at src/main.rs:3:25:
index out of bounds: the len is 3 but the index is 3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Pourquoi

Les positions (les [indices](../debuter/glossaire.md#indice)) commencent à **0**. Trois
cases ont les numéros 0, 1 et 2. La case 3 n'existe pas.

```text
position :   0    1    2
notes    : [ 12 | 15 |  9 ]
```

### La correction

La troisième note est `notes[2]`. Si vous n'êtes pas sûr, utilisez `get`, qui renvoie
une `Option` au lieu de s'arrêter :

```rust
fn main() {
    let notes = vec![12, 15, 9];
    println!("{}", notes[2]);
    println!("{:?}", notes.get(3));
}
```

```sortie
9
None
```

## Piège 12 : l'intervalle qui s'arrête trop tôt

### Ce que vous attendez

Vous comptez de 1 à 5 avec `1..5`. Vous pensez voir 5 nombres.

```rust
fn main() {
    for i in 1..5 {
        print!("{i} ");
    }
    println!();
}
```

```sortie
1 2 3 4 
```

### Pourquoi

Un [intervalle](../debuter/glossaire.md#intervalle) `1..5` s'arrête **avant** 5. C'est
pratique pour les positions : `0..v.len()` passe sur toutes les cases, sans déborder.

### La correction

Pour inclure la fin, ajoutez `=` : `1..=5`.

```rust
fn main() {
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();
}
```

```sortie
1 2 3 4 5 
```

| Écriture | Nombres obtenus |
|----------|-----------------|
| `1..5` | 1, 2, 3, 4 |
| `1..=5` | 1, 2, 3, 4, 5 |

## Erreurs fréquentes

Toute cette page parle d'erreurs. En voici encore deux, très courtes.

### Afficher une liste avec {}

```rust norun
fn main() {
    let v = vec![1, 2, 3];
    println!("{}", v);
}
```

```text
error[E0277]: `Vec<{integer}>` doesn't implement `std::fmt::Display`
 --> src/main.rs:3:20
  |
3 |     println!("{}", v);
  |               --   ^ `Vec<{integer}>` cannot be formatted with the default formatter
  |               |
  |               required by this formatting parameter
  |
  = help: the trait `std::fmt::Display` is not implemented for `Vec<{integer}>`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

`{}` est fait pour les humains, et Rust ne sait pas comment présenter une liste.
Utilisez `{:?}`, l'affichage « pour développeur » : `println!("{:?}", v);` affiche
`[1, 2, 3]`.

### parse sans dire le type voulu

```rust norun
fn main() {
    let n = "42".parse().unwrap();
    println!("{}", n + 1);
}
```

```text
error[E0284]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let n = "42".parse().unwrap();
  |         ^        ----- type must be known at this point
  |
  = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `n` an explicit type
  |
2 |     let n: /* Type */ = "42".parse().unwrap();
  |          ++++++++++++
```

`parse` peut fabriquer un entier, un nombre à virgule, un booléen… Dites lequel :
`let n: i32 = "42".parse().unwrap();`.

## Testez-vous

**1.** Qu'affiche ce programme : `let x = 10; let y = 4; println!("{}", x / y);` ?

> **Réponse**
>
> `2`. Ce sont deux entiers : la division garde seulement la partie entière. Pour
> obtenir `2.5`, écrivez `10.0 / 4.0`.

**2.** Pourquoi cette fonction ne compile-t-elle pas ?
`fn triple(x: i32) -> i32 { x * 3; }`

> **Réponse**
>
> À cause du point-virgule après `x * 3`. La ligne ne renvoie plus rien (`()`), alors
> que la fonction promet un `i32`. Écrivez `x * 3` sans point-virgule.

**3.** Après `let a = String::from("ok"); let b = a;`, peut-on encore afficher `a` ?
Comment garder les deux ?

> **Réponse**
>
> Non : la `String` a été donnée (déplacée) à `b`. Pour garder les deux, écrivez
> `let b = a.clone();`.

## En résumé

- Une variable est immuable par défaut : ajoutez `mut` pour la modifier.
- `"texte"` est un `&str` ; `String::from("texte")` donne une `String`.
- `let b = a;` donne une `String` à `b` : utilisez `.clone()` pour garder les deux.
- Entre deux entiers, `/` coupe la virgule ; un dépassement arrête le programme.
- Pas de point-virgule après la valeur renvoyée ; pas de `[0]` sur un texte.
- Les positions commencent à 0, `1..5` exclut 5, et `unwrap()` sur `None` arrête tout.

Suite : [Exercices pour bien démarrer](exercices-faciles.md)
